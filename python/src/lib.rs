// extern crate pyo3;
// extern crate codelist_rs;
//
// use pyo3::prelude::*;
// use pyo3::{PyResult, PyErr};
// use pyo3::types::PyDict;
// use codelist_rs::codelist::{CodeList, CodeListOptions};
// use codelist_rs::types::CodeListType;
// use codelist_rs::metadata::{Metadata, MetadataSource};
//
// #[pyclass(name = "CodeList")]
// pub struct PyCodeList {
//     inner: CodeList,
// }
//
// #[pymethods]
// impl PyCodeList {
//     #[new]
//     #[pyo3(signature = (codelist_type, source, authors=None, version=None, description=None, options=None))]
//     fn new(
//         codelist_type: &str,
//         source: &str,
//         authors: Option<Vec<String>>,
//         version: Option<String>,
//         description: Option<String>,
//         options: Option<&PyDict>,
//     ) -> PyResult<Self> {
//         // Convert string to CodeListType
//         let codelist_type = match codelist_type.to_uppercase().as_str() {
//             "ICD10" => CodeListType::ICD10,
//             "SNOMED" => CodeListType::SNOMED,
//             "OPCS" => CodeListType::OPCS,
//             _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
//                 format!("Invalid codelist type: {}", codelist_type),
//             )),
//         };
//
//         // Create metadata
//         let metadata = Metadata::new(
//             MetadataSource::ManuallyCreated,
//             authors,
//             version,
//             description,
//         );
//
//         // Parse CodeListOptions from PyDict
//         let codelist_options = CodeListOptions::default();
//
//         // Create codelist
//         let codelist = CodeList::new(codelist_type, metadata, Some(codelist_options));
//         Ok(PyCodeList { inner: codelist })
//     }
// }
//
// #[pymodule]
// fn codelist_py(py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_class::<PyCodeList>()?;
//     Ok(())
// }
// extern crate pyo3;
use pyo3::prelude::*;

use pyo3::types::PyModule;


#[pyclass]
struct Dummy {
    value: i32,
}

#[pymethods]
impl Dummy {
    #[new]
    fn new(value: i32) -> Self {
        Dummy { value }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

#[pymodule]
fn codelist_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Dummy>()?;
    Ok(())
}
