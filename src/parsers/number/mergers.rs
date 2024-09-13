from __future__ import annotations

from math import log10
from typing import Iterator, List, Optional

from more_itertools import windowed, padded, chunked

from nlp.tokens import Token
from nlp.constants import NUMBER
from .core import info_gen
from .ejtoken import tokenize
from .normalize import Pipe


def _tok(text, data):
    return tokenize(Pipe(data)(text))


class Merger:
    
    def __init__(
        self,
        data,
        exceptions: List[str] = None,
        **kwargs,
    ) -> None:
        self.data = data
        self.config = data.config
        if exceptions is None:
            exceptions = ("hundred",)
        self.exceptions = set(exceptions)
    
    def merge_multiples(self, tokens: List[Token], og_text: str) -> Iterator[Token]:
        # num < 1000 -> multiple thousand, million, billion, ...
        
        multiples = filter(
            lambda x: x not in self.exceptions,
            self.data.MULTIPLES.keys()
        )
        iterator = list(windowed(tokens, 2))
        i = 0
        last_start = None
        while True:
            try:
                t1, t2 = iterator[i]
                last_start = t2.span[0]
                value1 = t1.metadata["value"]
                value2 = t2.metadata["value"]
            except (IndexError, KeyError):
                break
            if t2 is None:
                if t1.span[0] != last_start:
                    yield t1
                break
            if _tok(t2.text.lower(), self.data)[0] not in multiples: #or value1 > 1000:
                if t1.span[0] != last_start:
                    
                    yield t1
                if (i + 1) == len(iterator) and t2 is not None:
                    yield t2
            else:
                span = (t1.span[0], t2.span[1])
                text = og_text[span[0]: span[1]]
                exponent = int(log10(value2))
                value = (value1 * pow(10, exponent + 1)) + (value2 - pow(10, exponent))
                info = info_gen(text, value, span, self.data)
                info.pop("span", None)
                yield Token(text, span=span, entity=NUMBER, metadata=info)
            i +=1
        
            
    def merge_points(self, tokens: List[Token], og_text: str) -> Iterator[Token]:
        return tokens
    
    def merge_informals(self, tokens: List[Token], og_text: str) -> Iterator[Token]:
        """
        5 and a quarter
        """

        i = 0
        iterator = chunked(tokens, 2)
        iterator = list(map(lambda x: padded(x, n=2), iterator))
        iterator = [tuple(x) for x in iterator]
        while True:
            try:
                yielded = False
                t1, t2 = iterator[i]
            except Exception:
                break
            if t2 is None:
                yield t1
                return
            t_and = og_text[t1.span[1]: t2.span[0]].strip().lower()
            _and = _tok(t2.text.lower(), self.data)[0] == "and" and t_and == "" or t_and == "and"
            if _and:
                toks = _tok(t2.text, self.data)
                last = toks[-1].lower()
                    
                if last in [*self.data.INFORMAL_EXACT, *self.data.INFORMALS_MULTIPLYABLE]:
                    span = t1.span[0], t2.span[1]
                    text = og_text[span[0]: span[1]]
                    value = t1.metadata["value"] + t2.metadata["value"]
                    metadata = {
                        "value": value,
                        "number_type": "spoken",
                        "value_type": "integer" if isinstance(value, int) else "float"
                    }
                    yielded = True
                    yield Token(text, span=span, metadata=metadata, entity=NUMBER)
            if not yielded:
                yield t1
                if t2 is not None:
                    yield t2
                else:
                    break
                    
            i += 1
        
    
    def merge(self, tokens: List[Token], og_text: str) -> List[Token]:
        if len(tokens) <= 1:
            return tokens
        new_tokens = tokens
        if self.config.merge_multiples:
            new_tokens = list(self.merge_multiples(tokens, og_text)) or new_tokens
        if self.config.merge_points:
            new_tokens = list(self.merge_points(new_tokens, og_text)) or new_tokens
        if self.config.merge_informals:
            new_tokens = list(self.merge_informals(new_tokens, og_text)) or new_tokens
        return new_tokens
        