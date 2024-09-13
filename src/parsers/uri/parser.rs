use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3::exceptions::PyValueError;
use regex::Regex;
use url::Url;
use std::collections::HashMap;

#[pyfunction]
fn parse(text: &str) -> PyResult<Vec<HashMap<String, PyObject>>> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let url_pattern = r"(?i)((?:(https?|ftp|git|ssh|ws):(?:/{1,3}|[a-z0-9%])|[a-z0-9.\-]+[.](?:com|net|org|edu|gov|mil|aero|asia|biz|cat|coop|info
                      |int|jobs|mobi|museum|name|post|pro|tel|travel|xxx|ac|ad|ae|af|ag|ai|al|am|an|ao|aq|ar|as|at|au|aw|ax|az|ba|
                      bb|bd|be|bf|bg|bh|bi|bj|bm|bn|bo|br|bs|bt|bv|bw|by|bz|ca|cc|cd|cf|cg|ch|ci|ck|cl|cm|cn|co|cr|cs|cu|cv|cx|cy|
                      cz|dd|de|dj|dk|dm|do|dz|ec|ee|eg|eh|er|es|et|eu|fi|fj|fk|fm|fo|fr|ga|gb|gd|ge|gf|gg|gh|gi|gl|gm|gn|gp|gq|gr|
                      gs|gt|gu|gw|gy|hk|hm|hn|hr|ht|hu|id|ie|il|im|in|io|iq|ir|is|it|je|jm|jo|jp|ke|kg|kh|ki|km|kn|kp|kr|kw|ky|kz|
                      la|lb|lc|li|lk|lr|ls|lt|lu|lv|ly|ma|mc|md|me|mg|mh|mk|ml|mm|mn|mo|mp|mq|mr|ms|mt|mu|mv|mw|mx|my|mz|na|nc|ne|
                      nf|ng|ni|nl|no|np|nr|nu|nz|om|pa|pe|pf|pg|ph|pk|pl|pm|pn|pr|ps|pt|pw|py|qa|re|ro|rs|ru|rw|sa|sb|sc|sd|se|sg|
                      sh|si|sj|Ja|sk|sl|sm|sn|so|sr|ss|st|su|sv|sx|sy|sz|tc|td|tf|tg|th|tj|tk|tl|tm|tn|to|tp|tr|tt|tv|tw|tz|ua|ug|
                      uk|us|uy|uz|va|vc|ve|vg|vi|vn|vu|wf|ws|ye|yt|yu|za|zm|zw)/)(?:[^\s()<>{}\[\]]+|\([^\s()]*?\([^\s()]+\)[^\s()]
                      *?\)|\([^\s]+?\))+(?:\([^\s()]*?\([^\s()]+\)[^\s()]*?\)|\([^\s]+?\)|[^\s`!()\[\]{};:'\".,<>?«»“”‘’])|(?:(?<!@)
                      [a-z0-9]+(?:[.\-][a-z0-9]+)*[.](?:com|net|org|edu|gov|mil|aero|asia|biz|cat|coop|info|int|jobs|mobi|museum|name
                      |post|pro|tel|travel|xxx|ac|ad|ae|af|ag|ai|al|am|an|ao|aq|ar|as|at|au|aw|ax|az|ba|bb|bd|be|bf|bg|bh|bi|bj|bm|bn
                      |bo|br|bs|bt|bv|bw|by|bz|ca|cc|cd|cf|cg|ch|ci|ck|cl|cm|cn|co|cr|cs|cu|cv|cx|cy|cz|dd|de|dj|dk|dm|do|dz|ec|ee|eg
                      |eh|er|es|et|eu|fi|fj|fk|fm|fo|fr|ga|gb|gd|ge|gf|gg|gh|gi|gl|gm|gn|gp|gq|gr|gs|gt|gu|'gw|gy|hk|hm|hn|hr|ht|hu|id
                      |ie|il|im|in|io|iq|ir|is|it|je|jm|jo|jp|ke|kg|kh|ki|km|kn|kp|kr|kw|ky|kz|la|lb|lc|li|lk|lr|ls|lt|lu|lv|ly|ma|mc|
                      md|me|mg|mh|mk|ml|mm|mn|mo|mp|mq|mr|ms|mt|mu|mv|mw|mx|my|mz|na|nc|ne|nf|ng|ni|nl|no|np|nr|nu|nz|om|pa|pe|pf|pg|
                      ph|pk|pl|pm|pn|pr|ps|pt|pw|py|qa|re|ro|rs|ru|rw|sa|sb|sc|sd|se|sg|sh|si|sj|Ja|sk|sl|sm|sn|so|sr|ss|st|su|sv|sx|
                      sy|sz|tc|td|tf|tg|th|tj|tk|tl|tm|tn|to|tp|tr|tt|tv|tw|tz|ua|ug|uk|us|uy|uz|va|vc|ve|vg|vi|vn|vu|wf|ws|ye|yt|yu|
                      za|zm|zw)\b/?(?!@)))";

    let url_regex = Regex::new(url_pattern).map_err(|e| PyValueError::new_err(e.to_string()))?;

    let mut results = Vec::new();

    for caps in url_regex.captures_iter(text) {
        let url = caps.get(0).map_or("", |m| m.as_str());
        match Url::parse(url) {
            Ok(parsed_url) => {
                let mut metadata = HashMap::new();
                metadata.insert("scheme".to_string(), py.None());
                metadata.insert("netloc".to_string(), py.None());
                metadata.insert("path".to_string(), py.None());
                metadata.insert("params".to_string(), py.None());
                metadata.insert("query".to_string(), py.None());
                metadata.insert("fragment".to_string(), py.None());

                if let Some(scheme) = parsed_url.scheme() {
                    metadata.insert("scheme".to_string(), PyDict::new(py).to_object(py));
                }
                if let Some(host) = parsed_url.host_str() {
                    metadata.insert("netloc".to_string(), PyDict::new(py).to_object(py));
                }
                if !parsed_url.path().is_empty() {
                    metadata.insert("path".to_string(), PyDict::new(py).to_object(py));
                }
                if let Some(query) = parsed_url.query() {
                    metadata.insert("query".to_string(), PyDict::new(py).to_object(py));
                }
                if let Some(fragment) = parsed_url.fragment() {
                    metadata.insert("fragment".to_string(), PyDict::new(py).to_object(py));
                }

                let result = PyDict::new(py);
                result.set_item("TEXT", url).unwrap();
                result.set_item("SPAN", (caps.get(0).unwrap().start(), caps.get(0).unwrap().end()))
                      .unwrap();
                result.set_item("METADATA", metadata).unwrap();
                results.push(result.extract().unwrap());
            }
            Err(_) => continue,
        }
    }

    Ok(results)
}

/*
#[pymodule]
fn my_rust_utils(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
*/
