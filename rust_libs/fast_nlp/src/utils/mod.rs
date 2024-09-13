use pyo3::prelude::*;

pub fn register_mod(parent_module: &PyModule) -> PyResult<()> {
    let utils_mod = PyModule::new(parent_module.py(), "utils")?;
    parent_module.add_submodule(&utils_mod)
}