/// This file contains the python bindings for the codelist-rs library's CodeList struct
/// This should only contain the python bindings for the CodeList struct.

// External imports
use pyo3::prelude::*;
use pyo3::{PyResult, PyErr};
use pyo3::types::PyDict;

// Internal imports
use codelist_rs::codelist::CodeList;
use codelist_rs::codelist_options::CodeListOptions;
use codelist_rs::metadata::{
    CategorisationAndUsage, Metadata, Provenance, PurposeAndContext, Source, ValidationAndReview,
};
use codelist_rs::types::CodeListType;
use codelist_rs::metadata::{Metadata, MetadataSource, Provenance, CategorisationAndUsage, PurposeAndContext, ValidationAndReview};


/// Python wrapper for the CodeList struct
///
/// This struct is a python wrapper for the CodeList struct in the codelist-rs library.
/// It allows us to create a new CodeList object from python and interact with it.
#[pyclass(name = "CodeList")]
pub struct PyCodeList {
    pub inner: CodeList,
}

/// Python methods for the PyCodeList struct
#[pymethods]
impl PyCodeList {
    #[new]
    #[pyo3(signature = (name, codelist_type, source, authors=None))]
    fn new(
        name: String,
        codelist_type: &str,
        source: &str,
        authors: Option<Vec<String>>,
    ) -> PyResult<Self> {
        // Convert string to CodeListType
        let codelist_type = match codelist_type.to_uppercase().as_str() {
            "ICD10" => CodeListType::ICD10,
            "ICD" => CodeListType::ICD10,
            "SNOMED" => CodeListType::SNOMED,
            "SNOMEDCT" => CodeListType::SNOMED,
            "OPCS" => CodeListType::OPCS,
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid codelist type: {}",
                    codelist_type
                )))
            }
        };

        // Create metadata
        let source = Source::from_string(source).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid source: {}", source))
        })?;
        // convert authors vec to HashSet
        let authors_set = authors
            .map(|authors| {
                authors
                    .into_iter()
                    .collect::<std::collections::HashSet<String>>()
            })
            .unwrap_or_default();
        let provenance = Provenance::new(source, Some(authors_set));
        let categorisation_and_usage = CategorisationAndUsage::new(None, None, None);
        let purpose_and_context = PurposeAndContext::new(None, None, None);
        let validation_and_review =
            ValidationAndReview::new(Some(false), None, None, Some("started".to_string()), None);
        let metadata = Metadata::new(
            provenance,
            categorisation_and_usage,
            purpose_and_context,
            validation_and_review,
        );

        // Parse CodeListOptions from PyDict
        let codelist_options = CodeListOptions::default();

        // Create codelist
        let codelist = CodeList::new(name, codelist_type, metadata, Some(codelist_options));
        Ok(PyCodeList { inner: codelist })
    }

    /// Get the name of the codelist
    #[getter]
    fn name(&self) -> String {
        self.inner.name.to_string()
    }

    /// Add an entry to the codelist
    #[pyo3(text_signature = "($self, code, term, comment=None)")]
    fn add_entry(&mut self, code: String, term: String, comment: Option<String>) -> PyResult<()> {
        let _ = self.inner.add_entry(code, term, comment);
        Ok(())
    }

    /// Get all entries in the codelist
    fn entries(&self) -> Vec<(String, String)> {
        self.inner
            .full_entries()
            .iter()
            .map(|entry| (entry.code.clone(), entry.term.clone()))
            .collect()
    }
}