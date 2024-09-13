use pyo3::prelude::*;
use std::collections::HashMap;

use nlp_rust::parsers::number::*;

#[pyclass]
#[derive(Clone)]
struct FastConfig {
    pub config: Config
}

#[pymethods]
impl FastConfig {
    #[new]
    pub fn new(
        language: Option<String>,
        signs_allowed: Option<bool>,
        parse_complex: Option<bool>,
        bounded_numbers: Option<bool>,
        mixed_nums: Option<bool>,
        merge: Option<bool>,
        merge_multiples: Option<bool>,
        merge_implied: Option<bool>,
        merge_points: Option<bool>,
        merge_informals: Option<bool>,
        exclude_separators: Option<Vec<String>>,
        exclude_suffixes: Option<Vec<String>>,
    ) -> Self {
        let config = Config {
            language: Some(language.unwrap_or("en".to_string())),
            signs_allowed: Some(signs_allowed.unwrap_or(false)),
            parse_complex: Some(parse_complex.unwrap_or(false)),
            bounded_numbers: Some(bounded_numbers.unwrap_or(false)),
            mixed_nums: Some(mixed_nums.unwrap_or(true)),
            merge: Some(merge.unwrap_or(true)),
            merge_multiples: Some(merge_multiples.unwrap_or(true)),
            merge_implied: Some(merge_implied.unwrap_or(false)),
            merge_points: Some(merge_points.unwrap_or(false)),
            merge_informals: Some(merge_informals.unwrap_or(true)),
            exclude_separators: exclude_separators,
            exclude_suffixes: Some(exclude_suffixes.unwrap_or(vec!["m".to_string(), "y".to_string()].into())),
        };
        FastConfig { config: config }
    }
    
    #[getter]
    pub fn language(&mut self) -> String {
        self.config.language.clone().unwrap()
    }
    #[getter]
    pub fn signs_allowed(&mut self) -> bool {
        self.config.signs_allowed.clone().unwrap()
    }
    
    #[getter]
    pub fn parse_complex(&mut self) -> bool {
        self.config.parse_complex.clone().unwrap()
    }
    
    #[getter]
    pub fn bounded_numbers(&mut self) -> bool {
        self.config.bounded_numbers.clone().unwrap()
    }
    
    #[getter]
    pub fn mixed_nums(&mut self) -> bool {
        self.config.mixed_nums.clone().unwrap()
    }
    
    #[getter]
    pub fn merge(&mut self) -> bool {
        self.config.merge.clone().unwrap()
    }
    
    #[getter]
    pub fn merge_multiples(&mut self) -> bool {
        self.config.merge_implied.clone().unwrap()
    }
    
    #[getter]
    pub fn merge_implied(&mut self) -> bool {
        self.config.merge_implied.clone().unwrap()
    }
    
    #[getter]
    pub fn merge_points(&mut self) -> bool {
        self.config.merge_points.clone().unwrap()
    }
    
    #[getter]
    pub fn merge_informals(&mut self) -> bool {
        self.config.merge_informals.clone().unwrap()
    }
    
    #[getter]
    pub fn exclude_separators(&mut self) -> Option<Vec<String>> {
        self.config.exclude_separators.clone()
    }
    
    #[getter]
    pub fn exclude_suffixes(&mut self) -> Option<Vec<String>> {
        self.config.exclude_suffixes.clone()
    }
    
}

#[pyclass]
#[derive(Clone)]
pub struct FastData {
    pub data: Data,
    pub config: FastConfig,
}

#[pymethods]
impl FastData {
    #[new]
    pub fn new(config: FastConfig) -> Self {
        let mut data = Data::new(config.config.clone());
        FastData {
            data: data,
            config: config
        }
    }
    
    #[getter]
    pub fn config(&mut self) -> FastConfig {
        self.config.clone()
    }
    
    #[getter]
    pub fn DEFAULT_RE_FLAGS(&mut self) -> String {
        self.data.default_re_flags()
    }

    #[getter]
    pub fn A(&mut self) -> Vec<String> {
        self.data.a()
    }

    #[getter]
    pub fn ANDS(&mut self) -> Vec<String> {
        self.data.ands()
    }

    #[getter]
    pub fn POINTS(&mut self) -> Vec<String> {
       self.data.points()
    }

    #[getter]
    pub fn NEGATIVES(&mut self) -> Vec<String> {
        self.data.negatives()
    }
    
    #[getter]
    pub fn ZEROS(&mut self) -> Vec<String> {
        self.data.zeros()
    }
    
    #[getter]
    pub fn ONES(&mut self) -> HashMap<String, f64> {
        self.data.ones()
    }
    
    #[getter]
    pub fn ORDINAL_ONES(&mut self) -> HashMap<String, f64> {
        self.data.ordinal_ones()
    }
    
    #[getter]
    pub fn MULTIPLES(&mut self) -> HashMap<String, f64> {
        self.data.multiples()
    }
    
    #[getter]
    pub fn TEENS_AND_TEN(&mut self) -> HashMap<String, f64> {
       self.data.teens_and_ten()
    }
    
    #[getter]
    pub fn ORDINAL_TEENS_AND_TEN(&mut self) -> HashMap<String, f64> {
        self.data.ordinal_teens_and_ten()
    }
    
    #[getter]
    pub fn ORDINAL_TENS(&mut self) -> HashMap<String, f64> {
       self.data.ordinal_tens()
    }
    
    #[getter]
    pub fn TENS(&mut self) -> HashMap<String, f64> {
        self.data.tens()
    }
    
    #[getter]
    pub fn ORDINAL_MULTIPLES(&mut self) -> HashMap<String, f64> {
        self.data.ordinal_ones()
    }
    
    #[getter]
    pub fn SUFFIXES(&mut self) -> HashMap<String, f64> {
        self.data.suffixes()
    }
    
    #[getter]
    pub fn SUFFIXES_BY_NAME(&mut self) -> HashMap<String, f64> {
        self.data.suffixes_by_name()
    }
    
    #[getter]
    pub fn INFORMAL_EXACT(&mut self) -> HashMap<String, f64> {
        self.data.informal_exact()
    }
    
    #[getter]
    pub fn INFORMALS_MULTIPLYABLE(&mut self) -> HashMap<String, f64> {
        self.data.informals_multiplyable()
    }
    
    #[getter]
    pub fn SUPERSCRIPT_ONES(&mut self) -> HashMap<char, f64> {
        self.data.superscript_ones()
    }
    
    #[getter]
    pub fn SUPERSCRIPT_ONES_REGEX(&mut self) -> String {
        self.data.superscript_ones_regex()
    }
    
    #[getter]
    pub fn SUBSCRIPT_ONES(&mut self) -> HashMap<char, f64> {
        self.data.subscript_ones()
    }
    
    #[getter]
    pub fn SUBSCRIPT_ONES_REGEX(&mut self) -> String {
        self.data.subscript_ones_regex()
    }
    
    #[getter]
    pub fn SUPERSCRIPT_FRACTIONS(&mut self) -> HashMap<char, f64> {
        self.data.superscript_fractions()
    }
    
    #[getter]
    pub fn SUPERSCRIPT_FRACTIONS_REGEX(&mut self) -> String {
        self.data.superscript_fractions_regex()
    }
    
    #[getter]
    pub fn ORDINAL_SUFFIXES(&mut self) -> Vec<String> {
       self.data.ordinal_suffixes()
    }
    
    #[getter]
    pub fn ORDINALS(&mut self) -> HashMap<String, f64> {
        self.data.ordinals()
    }
    
    #[getter]
    pub fn INFORMAL_ALL(&mut self) -> HashMap<String, f64> {
        self.data.informal_all()
    }
    
    #[getter]
    pub fn ALL_NUMS(&mut self) -> HashMap<String, f64> {
        self.data.all_nums()
    }
    
    #[getter]
    pub fn _tens(&mut self) -> String {
        self.data._tens()
    }
    
    #[getter]
    pub fn _ones(&mut self) -> String {
       self.data._ones()
    }
    
    #[getter]
    pub fn _ordinal_ones(&mut self) -> String {
        self.data._ordinal_ones()
    }
    
    #[getter]
    pub fn _teens(&mut self) -> String {
        self.data._teens()
    }
    
    #[getter]
    pub fn _multiples(&mut self) -> String {
        self.data._multiples()
    }
    
    #[getter]
    pub fn _suffixes(&mut self) -> String {
        self.data._suffixes()
    }
    
    #[getter]
    pub fn _negs(&mut self) -> String {
        self.data._negs()
    }
    
    #[getter]
    pub fn _points(&mut self) -> String {
        self.data._points()
    }
    
    #[getter]
    pub fn HYPHEN(&mut self) -> String {
        self.data.hyphen()
    }
    
    #[getter]
    pub fn INTEGER_REGEX(&mut self) -> String {
       self.data.integer_regex()
    }
    
    #[getter]
    pub fn FLOAT_REGEX(&mut self) -> String {
        self.data.float_regex()
    }
    
    #[getter]
    pub fn ANY_NUMBER_REGEX(&mut self) -> String {
        self.data.any_number_regex()
    }
    
    #[getter]
    pub fn COMPLEX_NUMBER_REGEX(&mut self) -> String {
       self.data.complex_number_regex()
    }
    
    #[getter]
    pub fn BINARY_REGEX(&mut self) -> String {
        self.data.binary_regex()
    }
    
    #[getter]
    pub fn HEX_REGEX(&mut self) -> String {
        self.data.hex_regex()
    }
    
    #[getter]
    pub fn OCT_REGEX(&mut self) -> String {
        self.data.oct_regex()
    }
    
    #[getter]
    pub fn _all_ones(&mut self) -> Vec<String> {
        self.data._all_ones()
    }
    
    #[getter]
    pub fn NUMBER_FOLLOWED_BY_POWER_REGEX(&mut self) -> String {
        self.data.number_followed_by_power_regex()
    }
    
    #[getter]
    pub fn SUFFIX_NAME_REGEX(&mut self) -> String {
        self.data.suffix_name_regex()
    }
    
    #[getter]
    pub fn NUMBER_FOLLOWED_BY_SUFFIX_REGEX(&mut self) -> String {
        self.data.number_followed_by_suffix_regex()
    }
    
    #[getter]
    pub fn INFORMALS_EXACT_REGEX(&mut self) -> String {
        self.data.informals_exact_regex()
    }
    
    #[getter]
    pub fn INFORMALS_MULTIPLYABLE_REGEX(&mut self) -> String {
        self.data.informals_multiplyable_regex()
    }

    #[getter]
    pub fn ORDINAL_NUMERAL_REGEX(&mut self) -> String {
        self.data.ordinal_numeral_regex()
    }
    
    #[getter]
    pub fn FIRST_EXTRACTION_REGEXES(&mut self) -> Vec<String> {
        self.data.first_extraction_regexes()
    }
    
     #[getter] 
    pub fn LAST_EXTRACTION_REGEXES(&mut self) -> Vec<String> {
        self.data.last_extraction_regexes()
    }
    
    pub fn get_suffix_value(&mut self, suffix: &str) -> Option<f64> {
        self.data.get_suffix_value(suffix)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct FastPipe;

#[pymethods]
impl FastPipe {
    
    #[new]
    pub fn new() -> Self {
        FastPipe
    }
    
    pub fn normalize(&mut self, text: String, data: &mut FastData) -> String {
        Pipe::normalize(text, &mut data.data)
    }
    
}

#[pyfunction]
pub fn fast_normalize_and(numbers: Vec<Vec<String>>, data: &mut FastData) -> Vec<Vec<String>> {
    normalize_and(numbers, &mut data.data)
}

#[pyfunction]
pub fn fast_check_valid(text: String, data: &mut FastData) -> bool {
    check_valid(text, &mut data.data)
}

#[pyfunction]
pub fn fast_recover_real_indices_and_match(
    text: String,
    nums: Vec<Vec<String>>,
    data: &mut FastData,
) -> (Vec<(String, (usize, usize))>, String) {
    recover_real_indices_and_match(text, nums, &mut data.data)
}

#[pyfunction]
pub fn fast_tokenize(sentence: String) -> Vec<String> {
    tokenize(sentence)
}


pub fn register_mod(parent_module: &PyModule) -> PyResult<()> {
    let number_mod = PyModule::new(parent_module.py(), "number")?;
    number_mod.add_class::<FastConfig>()?;
    number_mod.add_class::<FastData>()?;
    number_mod.add_class::<FastPipe>()?;
    number_mod.add_function(wrap_pyfunction!(fast_normalize_and, number_mod)?);
    number_mod.add_function(wrap_pyfunction!(fast_check_valid, number_mod)?);
    number_mod.add_function(wrap_pyfunction!(fast_recover_real_indices_and_match, number_mod)?);
    number_mod.add_function(wrap_pyfunction!(fast_tokenize, number_mod)?);
    parent_module.add_submodule(&number_mod)
}

