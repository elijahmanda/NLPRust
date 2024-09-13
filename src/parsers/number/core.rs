# coding: utf-8
# cython: embedsignature=True
# cython: language_level=3
# cython: profile=False

import regex as re
from .ejtoken import tokenize
from .normalize cimport (
    Pipe,
    recover_real_indices_and_match,
    normalize_and,
)
from .utils import text_span_replace, count_spaces, HashableDict
from .words2num import _words2num
from .constants import (
    FIRST_EXTRACTION_REGEXES,
    LAST_EXTRACTION_REGEXES,
    FLAGS,
    _REPLACEMENT,
)
cimport cython
from .logic cimport Logic
from .classes cimport NumberInfo


cdef _get_idxs_from_bool(bool_container):
    built_idxs = []
    cdef int i
    cdef bint truth
    for i, truth in enumerate(bool_container):
        if not truth:
            built_idxs.append(i)
    return built_idxs

@cython.boundscheck(False)
@cython.wraparound(False)
cdef _get_numbers_from_idxs(numbers, idxs):
    cdef int prev_idx, end
    prev_idx = 0
    nums = []
    for end in idxs:
        nums.append(numbers[prev_idx: end + 1])
        prev_idx = end + 1
    return nums

cdef _check_and_point(numbers, data):
    logic = Logic(numbers, data)
    bool_container = logic.apply_sequence_logic() 
    return bool_container

cdef first_extraction(str text, data):
    """ extract direct numbers like:
            -6.7 4'444 1e-35 23.8k' """
    rreturn = []
    regexes = getattr(data, FIRST_EXTRACTION_REGEXES, [])
    text = _replace(text, regexes, rreturn, data=data)
    return (text, # we pass the text to the next Pipeline
    rreturn)

@cython.wraparound(False)
cdef _replace(str text, regexes, rreturn, data):
    flags = data.DEFAULT_RE_FLAGS
    flags_dict = getattr(data, FLAGS, {})
    cdef int i, lc, rc, start, end
    for i, regex in enumerate(regexes):
        try:
            matches = re.finditer(
            regex,
            text,
            flags_dict.get(regex) or flags
            )
        except Exception as e:
            print(regex, "->", i)
            raise e
        if matches:
            for match in matches:
                if match.group():
                    lc,rc = count_spaces(match.group())
                    start, end = (
                        match.span()[0] + lc,
                        match.span()[1] - rc
                    )
                    rreturn.append(
                    (
                        match.group().strip(),
                        (start, end)
                    ))
                    # we replace the found number with `$` to avoid the next Pipeline extracting the same number again
                    text = text_span_replace(text, _REPLACEMENT * (end - start), (start, end))
    return text

def info_gen(
    str num_string,
    num_val,
    spans,
    data,
):
    d = HashableDict()
    cleaned = Pipe(data=data)(num_string)
    tokens = tokenize(cleaned, data=data)
    info_generator = NumberInfo(data)
    info = info_generator.generate(
        num_string,
        num_val,
        tokens,
    )
    d.update(info)
    d["span"] = spans
    d["value"] = num_val
    return d
    
cpdef parse(str text, data):
    
    # extract numbers 1
    remaining_words, matches = first_extraction(text, data)
    
    cleaned = Pipe(data=data)(remaining_words)
    tokens = tokenize(cleaned, data=data)
    bools = _check_and_point(tokens, data)
    end_idxs = _get_idxs_from_bool(bools)
    nums = _get_numbers_from_idxs(tokens, end_idxs)
    norm_nums = nums
    norm_nums = normalize_and(norm_nums, data=data)
    # get real indices
    real, text_repl = recover_real_indices_and_match(remaining_words, norm_nums, data)
    real.extend(matches)
    # extract remaining numbers 3
    last_extraction_regexes = getattr(data, LAST_EXTRACTION_REGEXES, [])
    
    text_repl = _replace(text_repl, last_extraction_regexes, real, data)
    rt_final = []
    spans = set()
    for n in real:
        num_string = n[0]
        span = n[1]
        info = gen_info(num_string, span, data=data)
        if info[2].get("value") is None or span[1] in spans:
            continue
        spans.add(span[1])
        rt_final.append(info)
    
    #sort by span
    rt_final.sort(key=__key1)
    rt_final = list(map(__map, rt_final))
    return rt_final
    
def __key1(n):
    return n[2]["span"]

def __map(d):
    return {"text": d[0], "value": d[1], **d[2]}

cdef gen_info(str num_string, span, data):
    value = _words2num(num_string, data)
    return num_string, value, info_gen(num_string, value, span, data)