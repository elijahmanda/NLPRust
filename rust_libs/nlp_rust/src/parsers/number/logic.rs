# coding: utf-8
# cython: embedsignature=True
# cython: language_level=3
# cython: profile=False

cimport cython
from functools import lru_cache
from more_itertools import stagger
from .classes import (
    CompStr,
    ModFloat,
    ModInt,
)
from .data import Data


cdef class Logic:
    
    def __cinit__(self, numbers, data):
        self.numbers = numbers
        self.config = data.config
        self.data = data
        self.bool_container = []
        self.idx_container = []
        self.prev_multiple = CompStr("always-big")
        self.prev_multiple.set_data(self.data)
        self.prev_point = False
        self.and_sep = False
        self.prev_false_truth = True
        self.beginning = True
        self.index = 0
   
    @cython.wraparound(False) 
    cdef get_num(self, num_, str string):
        if int(num_) == num_:
            modint = ModInt(int(num_))
            modint.set_data(self.data)
            modint.string(string)
            return modint
        elif isinstance(num_, float):
            modfloat = ModFloat(num_)
            modfloat.set_data(self.data)
            modfloat.string(string)
            return modfloat
        raise TypeError("Expected an integer or float got: %s" % num_)
    
    @cython.wraparound(False)
    cdef make_order(self, val):
        num_ = self.data.ALL_NUMS.get(val)
        if num_ is None:
            comp = CompStr(val)
            comp.set_data(self.data)
            return comp
        else:
            return self.get_num(num_, val)
            
    @cython.wraparound(False)
    cdef add_to_con(self, bint truth, object idx = None):
        if truth:
            self.prev_false_truth = False
            self.beginning = False
        else:
            self.prev_multiple = CompStr("always-big")
            self.prev_multiple.set_data(self.data)
            self.prev_point = False
            self.and_sep = False
            self.prev_false_truth = True
            self.beginning =True
        self.bool_container.append(truth)
        self.idx_container.append((self.index, idx))
        self.index = idx
    
    @cython.wraparound(False)
    @cython.boundscheck(False)
    cpdef apply_sequence_logic(self):
        fillvalue = CompStr("false")
        fillvalue.set_data(self.data)
        iterator = stagger(
            self.numbers,
            offsets=(-2, -1, 0, 1, 2),
            longest=True,
            fillvalue=fillvalue,
        )
        cdef int i
        for i, item in enumerate(iterator):
            # [pp, p, num, n, nn]
            if isinstance(item[2], CompStr):
                # Item is not a number
                continue
            next_num = self.make_order(item[3])
            nnext_num = self.make_order(item[4])
            prev_num = self.make_order(item[1])
            pprev = self.make_order(item[0])
            num = self.make_order(item[2])
            
            if not num.is_num_word:
                # Is num a word (non number)
                self.add_to_con(False, i)
                
            elif str(num) in self.data.ALL_NUMS and \
                not self.prev_point and \
                (
                    (
                        next_num.is_and and \
                        nnext_num.informal_exact
                    ) or \
                    (
                        next_num.informal_exact and \
                        num in (0, 1) or \
                        next_num.informal_multiplyable and num > 1
                    )
                ):
                self.add_to_con(True)
            # nothing can come after an ordinal or suffix
            # eg: twenty third million -> [twenty third, million]
            elif num.is_ordinal: # or num.is_suffix:
                self.add_to_con(False, i)
            # nothing can come after an informal
            # eg: two halves million -> [two halves, million]
            elif num.informal_exact or num.informal_multiplyable:
                
                self.add_to_con(False, i)
        
            elif not self.prev_point and next_num == 0:
                # if the previous token is not a point and
                # the next number is zero
                # eg: two million zero -> [two million, zero]
                self.add_to_con(False, i)
            
            elif num.is_a:
                # `a` followed by informal exact
                # a quarter -> [a quarter]
                if next_num.informal_exact:
                    self.add_to_con(True)
                else:
                    self.add_to_con(False, i)
            # if the next char is a point and what follows after the point isn't a ones
            # eg: twenty three point million ->
            # [twenty three point, million]
            # the `point` is stripped out in this case when
            # normalizing
            elif next_num.is_point and not nnext_num.ones:
                self.add_to_con(False, i)
        
            elif num in self.data.NEGATIVES:
                # if num is a word that represents negative
                # eg: negative, minus, neg
                if (next_num in self.data.ALL_NUMS.values() or next_num.informal_exact) and self.beginning:
                    self.add_to_con(True)
                else:
                    self.add_to_con(False)
        
            elif prev_num.hundred and next_num.hundred and not num.multiple:
                # hundred any hundred ->
                # [hundred, any, hundred]
                self.add_to_con(False, i)
        
            elif num.multiple and next_num >= num:
                # thousand thousand ->
                # [thousand, thousand]
                # thousand hundred ->
                # [thousand, hundred]
                self.add_to_con(False, i)
        
            elif num.is_and:
                self.and_sep = True
                if (
                    not prev_num.is_num_word or \
                    prev_num.is_and or \
                    prev_num.is_point or \
                    prev_num.informal_exact
                    ):
                    self.add_to_con(False, i)
                elif (
                     not self.prev_point and next_num.informal_exact or \
                     next_num.ones or \
                     next_num.teens or \
                     next_num.tens
                     ):
                    self.add_to_con(True)
                elif not self.prev_point and next_num.is_a and nnext_num.informal_exact:
                    self.add_to_con(True)
                else:
                    self.add_to_con(False, i)
                    self.and_sep = False

            elif num.is_point:
                # if the num represents a point and the
                # preceding tokens are ones (0, 1, 3, ..., 9)
                if next_num.ones:
                    self.add_to_con(True)
                    self.prev_point = True
                else:
                    self.add_to_con(False, i)
        
            elif num >= self.prev_multiple or next_num >= self.prev_multiple:
                # A number is constructed with decreasing multiple eg: billion -> million -> thousand
                # if a number in the chain is greater than
                # the previous multiple in the chain cut the chain
                # eg: billion million billion ->
                # [billion million, billion]
                self.add_to_con(False, i)
            
            elif num.ones:
                if next_num.teens:
                    self.add_to_con(False, i)
                elif self.prev_point:
                    if next_num.ones:
                        self.add_to_con(True)
                    elif next_num.multiple:
                        self.add_to_con(True)
                    else:
                        self.add_to_con(False, i)
                elif pprev.hundred and prev_num.is_and and next_num.hundred:
                    self.add_to_con(False)
                elif not self.prev_point and next_num.is_point:
                    if nnext_num.ones:
                        self.add_to_con(True)
                    else:
                        self.add_to_con(False, i)
                elif not self.prev_point and next_num.informal_exact and num in (0,1):
                    self.add_to_con(True)
                elif not self.prev_point and next_num.informal_multiplyable and num>1:
                    self.add_to_con(True)
                elif num == 0:
                    if next_num.ones or self.prev_point:
                        self.add_to_con(True)
                    else:
                        self.add_to_con(False, i)
                elif prev_num.tens and next_num.hundred:
                    self.add_to_con(False)
                elif next_num.hundred:
                    self.add_to_con(True)
                elif next_num.multiple:
                    self.add_to_con(True)
                elif next_num.is_and:
                    self.add_to_con(False, i)
                else:
                    self.add_to_con(False, i)
                
            elif num.teens:
                if (
                    next_num.multiple or
                    next_num.is_point and nnext_num.ones or \
                    next_num.informal_multiplyable
                    ):
                    self.add_to_con(True)
                else:
                    self.add_to_con(False, i)
                
            elif num.tens:
                if (
                    next_num.ones or \
                    self.data.ORDINAL_ONES.get(next_num) is not None or \
                    next_num.multiple or \
                    next_num.is_point and nnext_num.ones or \
                    next_num.informal_multiplyable
                    ):
                    self.add_to_con(True)
                else:
                    self.add_to_con(False, i)
              
            elif num.multiple:
                self.prev_multiple = num
                if self.prev_point:
                    self.add_to_con(False, i)
                elif next_num.is_point:
                    if nnext_num.ones:
                        self.add_to_con(True)
                    else:
                        self.add_to_con(False, i)
                elif next_num.is_and or num > next_num:
                    self.add_to_con(True)
                elif next_num.informal_multiplyable:
                    self.add_to_con(True)
                else:
                    self.add_to_con(False, i)
                
            elif num.hundred:
                if (
                    next_num.is_and or \
                    next_num.is_point or \
                    next_num.ones or \
                    next_num.teens or \
                    next_num.tens or \
                    next_num.multiple or \
                    next_num.informal_multiplyable
                    ):
                    self.add_to_con(True)
                else:
                    self.add_to_con(False, i)
        return self.bool_container