use pyo3::prelude::*;

pub mod regex_tokenizer;

pub use regex_tokenizer::FastRegexTokenizer;


pub fn register_mod(parent_module: &PyModule) -> PyResult<()> {
    let tokenizers_mod = PyModule::new(parent_module.py(), "tokenizers")?;
    tokenizers_mod.add_class::<FastRegexTokenizer>()?;
    parent_module.add_submodule(&tokenizers_mod)
}