use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

pub mod parser;

use parser::parse;

#[pyclass]
pub struct FastURIParser;

#[pymethods]
impl FastURIParser {
    #[new]
    fn new() -> Self {
        FastURIParser
    }

    fn parse(&self, py: Python, text: &str) -> PyResult<Vec<HashMap<String, PyObject>>> {
        let parse_res = parse(text)?;
        let mut results = Vec::new();

        for res in parse_res {
            let mut token_metadata = HashMap::new();
            token_metadata.insert("text".to_string(), py.eval(&format!("'{}'", res["text"]), None, None)?.to_object(py));
            token_metadata.insert("span".to_string(), PyTuple::new(py, &[res["span"].0.to_object(py), res["span"].1.to_object(py)]).to_object(py));
            token_metadata.insert("entity".to_string(), py.eval(&format!("'{}'", "URI"), None, None)?.to_object(py));
            token_metadata.insert("metadata".to_string(), res["metadata"].to_object(py));

            results.push(token_metadata);
        }

        Ok(results)
    }
}

