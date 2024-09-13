use pyo3::prelude::*;

pub mod tokenizers;
pub mod parsers;
pub mod utils;


#[pymodule]
fn fast_nlp(_py: Python, m: &PyModule) -> PyResult<()> {
    tokenizers::register_mod(m)?;
    parsers::register_mod(m)?;
    utils::register_mod(m)?;
    Ok(())
}


