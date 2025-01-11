/// This file contains the python bindings for the codelist-rs library's CodeList struct
/// This should only contain the python bindings for the CodeList struct.

// External imports
use pyo3::prelude::*;
use pyo3::{PyResult, PyErr};
use pyo3::types::PyDict;

// Internal imports
use codelist_rs::codelist::CodeList;
use codelist_rs::codelist_options::CodeListOptions;
use codelist_rs::types::CodeListType;
use codelist_rs::metadata::{Metadata, MetadataSource};


/// Python wrapper for the CodeList struct
///
/// This struct is a python wrapper for the CodeList struct in the codelist-rs library.
/// It allows us to create a new CodeList object from python and interact with it.
#[pyclass(name = "CodeList")]
pub struct PyCodeList {
    inner: CodeList,
}

/// Python methods for the PyCodeList struct
#[pymethods]
impl PyCodeList {
    #[new]
    #[pyo3(signature = (codelist_type, source, authors=None, version=None, description=None, options=None))]
    fn new(
        codelist_type: &str,
        source: &str,
        authors: Option<Vec<String>>,
        version: Option<String>,
        description: Option<String>,
        options: Option<&PyDict>,
    ) -> PyResult<Self> {
        // Convert string to CodeListType
        let codelist_type = match codelist_type.to_uppercase().as_str() {
            "ICD10" => CodeListType::ICD10,
            "SNOMED" => CodeListType::SNOMED,
            "OPCS" => CodeListType::OPCS,
            _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Invalid codelist type: {}", codelist_type),
            )),
        };

        // Create metadata
        let metadata = Metadata::new(
            MetadataSource::ManuallyCreated,
            authors,
            version,
            description,
        );

        // Parse CodeListOptions from PyDict
        let codelist_options = CodeListOptions::default();

        // Create codelist
        let codelist = CodeList::new(codelist_type, metadata, Some(codelist_options));
        Ok(PyCodeList { inner: codelist })
    }

    /// Add an entry to the codelist
    #[pyo3(text_signature = "($self, code, term)")]
    fn add_entry(&mut self, code: String, term: String) -> PyResult<()> {
        self.inner.add_entry(code, term);
        Ok(())
    }

    /// Get all entries in the codelist
    fn entries(&self) -> Vec<(String, String)> {
        self.inner.entries()
            .iter()
            .map(|entry| (entry.code.clone(), entry.term.clone()))
            .collect()
    }
}