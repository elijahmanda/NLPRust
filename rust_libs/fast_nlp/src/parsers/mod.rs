use pyo3::prelude::*;

pub mod number;

pub fn register_mod(parent_module: &PyModule) -> PyResult<()> {
    let parsers_mod = PyModule::new(parent_module.py(), "parsers")?;
    number::register_mod(parsers_mod)?;
    parent_module.add_submodule(&parsers_mod)
}
