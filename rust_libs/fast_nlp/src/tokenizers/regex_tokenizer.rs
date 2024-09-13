use pyo3::prelude::*;

use nlp_rust::tokenizers::{RegexTokenizer};

#[pyclass]
pub struct FastRegexTokenizer {
    tokenizer: RegexTokenizer,
}

#[pymethods]
impl FastRegexTokenizer {
    #[new]
    fn new(patterns: Option<Vec<(&str, &str)>>) -> Self {
        FastRegexTokenizer {
            tokenizer: RegexTokenizer::new(patterns),
        }
    }
    
    fn compile(&mut self, flags: Option<&str>, sort: Option<bool>) {
        self.tokenizer.compile(flags, sort.unwrap_or(false))
    }

    fn tokenize(&mut self, text: &str, merge: Option<bool>) -> Vec<(String, Option<String>, (usize, usize))> {
        self.tokenizer.tokenize(text, merge.unwrap_or(false))
    }

    fn add_pattern(&mut self, entity: &str, pattern: &str) {
        self.tokenizer.add_pattern(entity, pattern);
    }

    fn clear_patterns(&mut self) {
        self.tokenizer.clear_patterns();
    }

    fn set_patterns(&mut self, patterns: Vec<(&str, &str)>, compile: Option<bool>) {
        self.tokenizer.set_patterns(patterns, compile.unwrap_or(true));
    }
    
    fn get_entities(&self) -> Vec<String> {
        self.tokenizer.get_entities()
    }
    
    fn get_entity_count(&self) -> usize {
        self.tokenizer.get_entity_count()
    }
}
