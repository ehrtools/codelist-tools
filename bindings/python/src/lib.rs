extern crate core;

use pyo3::prelude::*;

// Internal imports
pub mod codelist;
pub mod factory;

use codelist::PyCodeList;
use factory::PyCodeListFactory;


/// Top-level Python module `codelists_rs`
#[pymodule]
fn codelists_rs(py: Python, m: &PyModule) -> PyResult<()> {
    // Add codelist submodule
    let codelist_module = PyModule::new(py, "codelist")?;
    codelist_module.add_class::<PyCodeList>()?;
    m.add_submodule(codelist_module)?;

    // Register it globally under the full dotted path
    py.import("sys")?
        .getattr("modules")?
        .set_item("codelists_rs.codelist", codelist_module)?;

    // Add factory submodule
    let factory_module = PyModule::new(py, "factory")?;
    factory_module.add_class::<PyCodeListFactory>()?;
    m.add_submodule(factory_module)?;

    // Register it globally under the full dotted path
    py.import("sys")?
        .getattr("modules")?
        .set_item("codelists_rs.factory", factory_module)?;

    Ok(())
}
