/// This contains the bindings for the codelist-rs library at the level of the
/// Python module.

// External imports
use pyo3::prelude::*;

// Internal imports
pub mod codelists;
use codelists::codelist::PyCodeList;


/// Python module for the codelist-rs library
#[pymodule]
fn codelist(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CodeList>()?;
    Ok(())
}
