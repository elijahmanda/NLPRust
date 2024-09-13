use std::collections::HashSet;
use std::str;
use std::hash::{Hash, Hasher};
use std::borrow::Cow;
use std::fmt;

const DEFAULT_ENCODING: &str = "utf-8";

fn encode(text: &str) -> Vec<u8> {
    Ok(text.as_bytes().to_vec())
}

fn decode(byte_string: Vec<u8>) -> String {
    match str::from_utf8(&byte_string) {
        Ok(s) => Ok(s.to_string()),
        Err(_) => Err("Failed to decode byte string."),
    }
}

fn text_span_replace(text: &str, replacement: &str, start: usize, end: usize) -> String {
    let mut tmp = text.as_bytes().to_vec();
    let replacement_bytes = replacement.as_bytes();
    tmp.splice(start..end, replacement_bytes.iter().cloned());
    decode(tmp)
}

fn count_spaces(text: &str) -> (usize, usize) {
    let text_len = text.len();
    let left = text_len - text.trim_start().len();
    let right = text_len - text.trim_end().len();
    (left, right)
}

fn get_text_chunks(text: &str, span: (usize, usize)) -> (String, String, String) {
    let (start, end) = span;
    let left_chunk = text[0..start].to_string();
    let middle_chunk = text[start..end].to_string();
    let right_chunk = text[end..].to_string();
    (left_chunk, middle_chunk, right_chunk)
}

fn has_punct(text: &str) -> bool {
    let punctuation: HashSet<_> = string::PUNCTUATION.chars().collect();
    text.chars().any(|c| punctuation.contains(&c))
}

fn has_space(text: &str) -> bool {
    text.contains(' ')
}

fn remove_spaces(text: &str) -> String {
    text.split_whitespace().collect()
}

fn remove_multiple_spaces(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[pyclass(extends=str)]
#[derive(Debug)]
struct CaseLessString;

#[pymethods]
impl CaseLessString {
    #[new]
    fn new(obj: &PyAny) -> PyResult<Self> {
        if obj.is_instance_of::<str>() {
            Ok(CaseLessString)
        } else {
            Err(PyValueError::new_err("Expected a string instance."))
        }
    }

    fn __eq__(&self, other: &PyAny) -> PyResult<bool> {
        if let Ok(other_str) = other.extract::<String>() {
            let self_str: Cow<str> = self.into();
            Ok(self_str.to_lowercase() == other_str.to_lowercase())
        } else {
            Ok(false)
        }
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        let self_str: Cow<str> = self.into();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self_str.hash(&mut hasher);
        Ok(hasher.finish() as isize)
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        let self_str: Cow<str> = self.into();
        Ok(format!("CaseLessString({})", self_str))
    }
}

#[pymodule]
fn my_rust_utils(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    m.add_function(wrap_pyfunction!(text_span_replace, m)?)?;
    m.add_function(wrap_pyfunction!(count_spaces, m)?)?;
    m.add_function(wrap_pyfunction!(get_text_chunks, m)?)?;
    m.add_function(wrap_pyfunction!(has_punct, m)?)?;
    m.add_function(wrap_pyfunction!(has_space, m)?)?;
    m.add_function(wrap_pyfunction!(remove_spaces, m)?)?;
    m.add_function(wrap_pyfunction!(remove_multiple_spaces, m)?)?;
    m.add_class::<CaseLessString>()?;
    Ok(())
}
