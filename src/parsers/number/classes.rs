use std::collections::HashMap;
use std::borrow::Cow;
use std::hash::{Hash, Hasher};

use crate::parsers::number::{DataHolder, Data};

pub struct CompStr {
    pub val: DataHolder,
    pub data: Data,
    _is_ordinal: bool,
    _is_suffix: bool,
    _informal_exact: bool,
    _informal_multiplyable: bool
}

impl CompStr {
    
    pub fn new(val: DataHolder, data: Data) -> Self {
        CompStr { 
            val: val,
            data: data,
            _is_ordinal: false,
            _is_suffix: false,
            _informal_exact: false,
            _informal_multiplyable: false,
        }
    }
    
    pub fn string(&mut self, strg: String) {
        self._is_ordinal = self.data.ordinals().contains_key(&strg);
        self._is_suffix = self.data.suffixes_by_name().contains_key(&strg);
        self._informal_exact = self.data.informal_exact().contains_key(&strg);
        self._informal_multiplyable = self.data.informals_multiplyable().contains_key(&strg);
    }
    
    pub fn ones(&mut self) -> bool {
        let string = match self.val.type_ {
            "text" => self.val.text.clone().unwrap(),
            "float" => self.val.float.clone().unwrap().to_string(),
            _ => self.val.int.clone().unwrap().to_string(),
        };
        for (k, v) in &self.data.ones() {
            if string == k.to_string() || string == v.to_string() {
                return true;
            }
        }
        false
    }
    
    pub fn tens(&mut self) -> bool {
        let string = match self.val.type_ {
            "text" => self.val.text.clone().unwrap(),
            "float" => self.val.float.clone().unwrap().to_string(),
            _ => self.val.int.clone().unwrap().to_string(),
        };
        for (k, v) in &self.data.tens() {
            if string == k.to_string() || string == v.to_string() {
                return true;
            }
        }
        false
    }
    
    pub fn teens(&mut self) -> bool {
        let string = match self.val.type_ {
            "text" => self.val.text.clone().unwrap(),
            "float" => self.val.float.clone().unwrap().to_string(),
            _ => self.val.int.clone().unwrap().to_string(),
        };
        for (k, v) in &self.data.teens_and_ten() {
            if string == k.to_string() || string == v.to_string() {
                return true;
            }
        }
        false
    }
        
    pub fn multiples(&mut self) -> bool {
        let string = match self.val.type_ {
            "text" => self.val.text.clone().unwrap(),
            "float" => self.val.float.clone().unwrap().to_string(),
            _ => self.val.int.clone().unwrap().to_string(),
        };
        for (k, v) in &self.data.multiples() {
            if string == k.to_string() || string == v.to_string() {
                return true;
            }
        }
        false
    }
    
    pub fn hundred(&mut self) -> bool {
        let string = match self.val.type_ {
            "text" => self.val.text.clone().unwrap(),
            "float" => self.val.float.clone().unwrap().to_string(),
            _ => self.val.int.clone().unwrap().to_string(),
        };
        let hundreds = vec!["100", "hundred"];
        for k in &hundreds {
            if string == k.to_string() {
                return true;
            }
        }
        false
    }

    pub fn is_point(&mut self) -> bool {
        let string = match self.val.type_ {
            "text" => self.val.text.clone().unwrap(),
            "float" => self.val.float.clone().unwrap().to_string(),
            _ => self.val.int.clone().unwrap().to_string(),
        };
        self.data.points().contains(&string)
    }

    pub fn is_num_word(&mut self) -> bool {
        let string = match self.val.type_ {
            "text" => self.val.text.clone().unwrap(),
            "float" => self.val.float.clone().unwrap().to_string(),
            _ => self.val.int.clone().unwrap().to_string(),
        };
        let mut all_: HashMap<String, f64> = HashMap::new();
        all_.extend(self.data.all_valid());
        all_.extend(self.data.informal_all());
        for (k, _) in &all_ {
            if string == k.to_string() {
                return true;
            }
        }
        false
    }
    
    pub fn informal_exact(&self) -> bool {
        self._informal_exact
    }
    
    pub fn informal_multiplyable(&self) -> bool {
        self._informal_multiplyable
    }

    pub fn is_and(&mut self) -> bool {
        let string = match self.val.type_ {
            "text" => self.val.text.clone().unwrap(),
            "float" => self.val.float.clone().unwrap().to_string(),
            _ => self.val.int.clone().unwrap().to_string(),
        };
        self.data.ands().contains(&string)
    }
    
    pub fn is_a(&mut self) -> bool {
        let string = match self.val.type_ {
            "text" => self.val.text.clone().unwrap(),
            "float" => self.val.float.clone().unwrap().to_string(),
            _ => self.val.int.clone().unwrap().to_string(),
        };
        self.data.a().contains(&string)
    }

    pub fn is_ordinal(&self) -> bool {
        self._is_ordinal || self.is_suffix()
    }
    
    pub fn is_suffix(&self) -> bool {
        self._is_suffix
    }
    
    pub fn __lt__(&self, _other: DataHolder) -> bool {
        false
    }
    pub fn __gt__(&self, _other: DataHolder) -> bool {
        false
    }
    pub fn __le__(&self, _other: DataHolder) -> bool {
        false
    }
    pub fn __ge__(&self, _other: DataHolder) -> bool {
        false
    }

    pub fn __eq__(&mut self, other: DataHolder) -> bool {
        let string = match self.val.type_ {
            "text" => self.val.text.clone().unwrap(),
            "float" => self.val.float.clone().unwrap().to_string(),
            _ => self.val.int.clone().unwrap().to_string(),
        };
        let other_string = match other.type_ {
            "text" => other.text.clone().unwrap(),
            "float" => other.float.clone().unwrap().to_string(),
            _ => other.int.clone().unwrap().to_string(),
        };
        string.to_lowercase() == other_string.to_lowercase()
    }

    pub fn __hash__(&mut self) -> isize {
        let self_str: Cow<str> = self.val.text.clone().unwrap().into();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self_str.hash(&mut hasher);
        hasher.finish() as isize
    }
}

/*
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
*/