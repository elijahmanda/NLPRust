# coding: utf-8
# cython: embedsignature=True
# cython: language_level=3
# cython: profile=False


from functools import cached_property
from itertools import chain
import typing as t

import regex as re


from nlp.parsers.number.constants import (
    NUMBER_TYPE,
    VALUE_TYPE,
    NumberType,
)
from nlp.parsers.number.ejtoken import tokenize
from nlp.parsers.number.normalize import Pipe
from nlp.utils.sequences import flatten_sequences
from nlp.utils.strings import CaseLessString
from .words2num import convert_suffixes



def map_str_all(*iterables):
    """
    Return a flattened sequence of `iterables`
    converted to strings
    """
    iterable = flatten_sequences(iterables)
    return set(chain(map(str, iterable)))

def bool_str_set(val, *others):
    return bool(
        {val, CaseLessString(val)} \
        & \
        set(flatten_sequences(others))
    )


class DataAttrGetter:
    def __init__(self, val):
        self.val = val
        self._data = None
        self._is_ordinal = False
        self._is_suffix = False
        self._informal_exact = False
        self._informal_multiplyable = False
    
    @property
    def data(self):
        return self._data
    
    def set_data(self, data):
        self._data = data
    
    def string(self, strg) -> None:
        strg = CaseLessString(strg)
        self._is_ordinal = strg in self.data.ORDINALS
        self._is_suffix = strg in self.data.SUFFIXES_BY_NAME
        self._informal_exact = bool(strg in self.data.INFORMAL_EXACT)
        self._informal_multiplyable = bool(strg in self.data.INFORMALS_MULTIPLYABLE)
    
    @cached_property
    def ones(self):
        return bool_str_set(
            self.val,
            map_str_all(
                self.data.ONES.values(),
                self.data.ONES.keys()
            )
        )
    
    @cached_property
    def tens(self):
        return bool_str_set(
            self.val,
            map_str_all(
                self.data.TENS.values(),
                self.data.TENS.keys()
            )
        )
    
    @cached_property
    def teens(self):
        return bool_str_set(
            self.val,
            map_str_all(
                self.data.TEENS_AND_TEN.values(),
                self.data.TEENS_AND_TEN.keys()
            )
        )
        
    @cached_property
    def multiple(self):
        return self.val \
        and self.val != 100 \
        and bool_str_set(
            self.val,
            map_str_all(
                self.data.MULTIPLES.values(),
                self.data.MULTIPLES.keys()
            )
        )
    
    @cached_property
    def hundred(self):
        return bool_str_set(
            self.val,
            map_str_all(
                [100, "hundred"]
            )
        )

    @cached_property
    def is_point(self):
        return bool_str_set(self.val, self.data.POINTS)

    @cached_property
    def is_num_word(self):
        return bool_str_set(self.val, self.data.ALL_VALID, self.data.INFORMAL_ALL)
    
    @cached_property
    def informal_exact(self):
        return self._informal_exact
    
    @cached_property
    def informal_multiplyable(self):
        return self._informal_multiplyable

    @cached_property
    def is_and(self):
        return bool_str_set(self.val, self.data.ANDS)
    
    @cached_property
    def is_a(self):
        return bool_str_set(self.val, self.data.A)

    @cached_property
    def is_ordinal(self):
        return self._is_ordinal or self.is_suffix
    
    @cached_property
    def is_suffix(self):
        return self._is_suffix
        
    def __lt__(self, val):
        return False
    
    __gt__ = __lt__
    __ge__ = __lt__
    __le__ = __lt__

    def __eq__(self, val):
        return str(self.val).lower() == str(val).lower()
    

class CompStr(DataAttrGetter):

    def __hash__(self):
        return hash((self.val, self.is_num_word))


class ModInt(int, DataAttrGetter):
    
    def __init__(self, val):
        super().__init__(int(val))

    @property
    def is_num_word(self):
        return True

    @property
    def is_ordinal(self):
        return self._is_ordinal
        
    @is_ordinal.setter
    def is_ordinal(self, val: bool):
        self._is_ordinal = val


class ModFloat(float, DataAttrGetter):
    
    def __init__(self, val):
        super().__init__(float(val))

    @property
    def is_num_word(self):
        return True

    @property
    def is_ordinal(self):
        return False

cdef get_suffix(n, data):
    v = ""
    suffixes = tuple(data.SUFFIXES.keys())
    max_len = len(max(suffixes, key=len))
    if not n.lower().endswith(suffixes):
        return v
    for ch in n[::-1]:
        if ch.isalpha():
            v += ch
        else:
            break
    if len(v) > max_len:
        return ""
    return v[::-1]

cdef match(regex, text, data):
    res = re.search(
        f"(?:{regex})",
        text,
        data.FLAGS.get(regex) or data.DEFAULT_RE_FLAGS,
    )
    return res and res.group() == text


def convert_match(n, converter, data):
    n = converter([n], data)
    if len(n) != 1:
        return 
    return n[0]


cdef class NumberInfo:
    
    def __cinit__(self, data):
        self.data = data
    
    cpdef generate(
        self,
        str num_string,
        num_val,
        tokens,
    ):
        d = {}
        data = self.data
        if isinstance(num_val, complex):
            d[NUMBER_TYPE] = NumberType.COMPLEX
        elif set(num_string) & {*data.SUPERSCRIPT_ONES, *data.SUPERSCRIPT_FRACTIONS}:
            d[NUMBER_TYPE] = NumberType.SUPERSCRIPT
        elif self.get_ordinal_suffix(num_string):
            suffix = self.get_ordinal_suffix(num_string)
            d[NUMBER_TYPE] = NumberType.ORDINAL
            d["suffix"] = suffix
            
        elif match(data.BINARY_REGEX, num_string, data):
            d[NUMBER_TYPE] = NumberType.BINARY
            
        elif match(self.data.HEX_REGEX, num_string, data):
            d[NUMBER_TYPE] = NumberType.HEX
            
        elif match(self.data.OCT_REGEX, num_string, data):
            d[NUMBER_TYPE] = NumberType.OCTAL
            
        elif self.is_spoken(num_string):
            d[NUMBER_TYPE] = NumberType.SPOKEN
            
        elif self.is_integer(num_string):
            d[NUMBER_TYPE] = NumberType.INTEGER
            
        elif self.is_float(num_string):
            d[NUMBER_TYPE] = NumberType.FLOAT
        
        d[VALUE_TYPE] = NumberType.INTEGER
        if isinstance(num_val, float):
            d[VALUE_TYPE] = NumberType.FLOAT
        if isinstance(num_val, complex):
            d[VALUE_TYPE] = NumberType.COMPLEX
            d[NUMBER_TYPE] = NumberType.COMPLEX
        
        return d

    cdef get_ordinal_suffix(self, str num_string):
        s = num_string.lower()
        if s.endswith(tuple(self.data.ORDINAL_SUFFIXES)):
            return num_string[-2:]

    cdef is_spoken(self, str s):
        tks = tokenize(Pipe(data=self.data)(s), data=self.data)
        if len(tks) > 1:
            return True
        return tks[0].isalpha()
    
    cdef is_integer(self, str s):
        if self.is_spoken(s):
            return False
        return not bool(set("e.") & set(s.lower()))

    cdef is_float(self, str s):
        if self.is_spoken(s) or self.is_integer(s):
            return False
        return True