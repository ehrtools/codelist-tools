#![allow(non_local_definitions)]
//! This file contains the python bindings for the codelist-rs library's
//! CodeList struct This should only contain the python bindings for the
//! CodeList struct.

use codelist_rs::{
    codelist::{CodeList, TermManagement},
    codelist_options::CodeListOptions,
    metadata::{
        CategorisationAndUsage, Metadata, Provenance, PurposeAndContext, Source,
        ValidationAndReview,
    },
    types::CodeListType,
};
use codelist_validator_rs::validator::Validator;
use indexmap::IndexSet;
use regex::Regex;
use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyDict, PySet},
    PyErr, PyResult,
};

/// Python wrapper for the CodeList struct
///
/// This struct is a python wrapper for the CodeList struct in the codelist-rs
/// library. It allows us to create a new CodeList object from python and
/// interact with it.
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
                    "Invalid codelist type: {codelist_type}"
                )))
            }
        };

        // Create metadata
        let source = Source::from_string(source).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid source: {source}"))
        })?;
        // convert authors vec to IndexSet
        let authors_set = authors
            .map(|authors| authors.into_iter().collect::<IndexSet<String>>())
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
    #[pyo3(signature = (code, term=None, comment=None))]
    fn add_entry(
        &mut self,
        code: String,
        term: Option<String>,
        comment: Option<String>,
    ) -> PyResult<()> {
        let _ = self.inner.add_entry(code, term, comment);
        Ok(())
    }

    /// Get all entries in the codelist
    fn entries(&self) -> Vec<(String, Option<String>, Option<String>)> {
        self.inner
            .full_entries()
            .iter()
            .map(|(code, (term, comment))| (code.clone(), term.clone(), comment.clone()))
            .collect()
    }

    /// Add a contributor to the codelist's provenance
    fn add_contributor(&mut self, contributor: String) -> PyResult<()> {
        self.inner.metadata.provenance.add_contributor(contributor);
        Ok(())
    }

    /// Remove a contributor from the codelist's provenance
    fn remove_contributor(&mut self, contributor: String) -> PyResult<()> {
        self.inner
            .metadata
            .provenance
            .remove_contributor(contributor)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    #[getter]
    fn contributors(&self, py: Python) -> PyResult<PyObject> {
        let py_set = PySet::new(py, &[] as &[String])?;
        for contributor in &self.inner.metadata.provenance.contributors {
            py_set.add(contributor)?;
        }
        Ok(py_set.into())
    }

    /// Get date created and last modified date as dict
    fn get_dates(&self, py: Python) -> PyResult<PyObject> {
        let date_created = self.inner.metadata.provenance.created_date;
        let last_modified_date = self.inner.metadata.provenance.last_modified_date;

        let dict = PyDict::new(py);
        dict.set_item("date_created", date_created.to_string())?;
        dict.set_item("last_modified_date", last_modified_date.to_string())?;

        Ok(dict.into())
    }

    /// Get tag information
    fn get_tags(&self, py: Python) -> PyResult<PyObject> {
        let tags = self.inner.metadata.categorisation_and_usage.tags.clone();
        let py_set = PySet::new(py, &[] as &[String])?;
        for tag in tags {
            py_set.add(tag)?;
        }
        Ok(py_set.into())
    }

    /// Add a tag to the codelist
    fn add_tag(&mut self, tag: String) -> PyResult<()> {
        self.inner
            .metadata
            .categorisation_and_usage
            .add_tag(tag)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Remove a tag from the codelist
    fn remove_tag(&mut self, tag: String) -> PyResult<()> {
        self.inner
            .metadata
            .categorisation_and_usage
            .remove_tag(tag)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Get usage information
    fn get_usage(&self, py: Python) -> PyResult<PyObject> {
        let usage = self.inner.metadata.categorisation_and_usage.usage.clone();
        let py_set = PySet::new(py, &[] as &[String])?;
        for usage_item in usage {
            py_set.add(usage_item)?;
        }
        Ok(py_set.into())
    }

    /// Add usage information to the codelist
    fn add_usage(&mut self, usage: String) -> PyResult<()> {
        self.inner.metadata.categorisation_and_usage.add_usage(usage);
        Ok(())
    }

    /// Remove usage information from the codelist
    fn remove_usage(&mut self, usage: String) -> PyResult<()> {
        self.inner
            .metadata
            .categorisation_and_usage
            .remove_usage(usage)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Get license information (with is a OPtional String)
    fn get_license_info(&self) -> Option<String> {
        self.inner.metadata.categorisation_and_usage.license.clone()
    }

    /// Add license information to the codelist
    fn add_license(&mut self, license: String) -> PyResult<()> {
        self.inner
            .metadata
            .categorisation_and_usage
            .add_license(license)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Remove license information from the codelist
    fn remove_license(&mut self) -> PyResult<()> {
        self.inner
            .metadata
            .categorisation_and_usage
            .remove_license()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Update the license information
    fn update_license(&mut self, license: String) -> PyResult<()> {
        self.inner
            .metadata
            .categorisation_and_usage
            .update_license(license)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Get the purpose of the codelist
    fn get_purpose(&self) -> Option<String> {
        self.inner.metadata.purpose_and_context.purpose.clone()
    }

    /// Add a purpose to the codelist
    fn add_purpose(&mut self, purpose: String) -> PyResult<()> {
        self.inner
            .metadata
            .purpose_and_context
            .add_purpose(purpose)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Update the purpose of the codelist
    fn update_purpose(&mut self, purpose: String) -> PyResult<()> {
        self.inner
            .metadata
            .purpose_and_context
            .update_purpose(purpose)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Remove a purpose from the codelist
    fn remove_purpose(&mut self) -> PyResult<()> {
        self.inner
            .metadata
            .purpose_and_context
            .remove_purpose()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Get the target audience of the codelist
    fn get_audience(&self) -> Option<String> {
        self.inner.metadata.purpose_and_context.target_audience.clone()
    }

    /// Add a target audience to the codelist
    fn add_audience(&mut self, target_audience: String) -> PyResult<()> {
        self.inner
            .metadata
            .purpose_and_context
            .add_target_audience(target_audience)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Update the target audience of the codelist
    fn update_audience(&mut self, target_audience: String) -> PyResult<()> {
        self.inner
            .metadata
            .purpose_and_context
            .update_target_audience(target_audience)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Remove a target audience from the codelist
    fn remove_audience(&mut self) -> PyResult<()> {
        self.inner
            .metadata
            .purpose_and_context
            .remove_target_audience()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Get the use context of the codelist
    fn get_use_context(&self) -> Option<String> {
        self.inner.metadata.purpose_and_context.use_context.clone()
    }

    /// Add a use context to the codelist
    fn add_use_context(&mut self, use_context: String) -> PyResult<()> {
        self.inner
            .metadata
            .purpose_and_context
            .add_use_context(use_context)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Update the use context of the codelist
    fn update_use_context(&mut self, use_context: String) -> PyResult<()> {
        self.inner
            .metadata
            .purpose_and_context
            .update_use_context(use_context)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Remove a use context from the codelist
    fn remove_use_context(&mut self) -> PyResult<()> {
        self.inner
            .metadata
            .purpose_and_context
            .remove_use_context()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Truncate to 3 digits if ICD10
    fn truncate_to_3_digits(&mut self, term_management: String) -> PyResult<()> {
        // Term management as string to TermManagement enum
        let term_management = term_management
            .parse::<TermManagement>()
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        self.inner
            .truncate_to_3_digits(term_management)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Add X Codes to the codelist. This is a convenient way to add X to 3
    /// digit ICD10 codes.
    fn add_x_codes(&mut self) -> PyResult<()> {
        self.inner.add_x_codes().map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// See if the codelist is validated
    fn is_validated(&self) -> bool {
        self.inner.metadata.validation_and_review.reviewed
    }

    /// Add Validation Information to the codelist
    #[pyo3(signature = (reviewer, status=None, notes=None))]
    fn add_validation_info(
        &mut self,
        reviewer: String,
        status: Option<String>,
        notes: Option<String>,
    ) -> PyResult<()> {
        // Add reviewer
        self.inner
            .metadata
            .validation_and_review
            .add_reviewer(reviewer)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        // Add review date // TODO: Sort out datetime with pyclass
        self.inner
            .metadata
            .validation_and_review
            .add_review_date(chrono::Utc::now())
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        // Add status
        if let Some(status) = status {
            self.inner
                .metadata
                .validation_and_review
                .update_status(status)
                .map_err(|e| PyValueError::new_err(e.to_string()))?;
        }

        // Add validation notes
        if let Some(validation_notes) = notes {
            // if existing notes, append to them, otherwise just set them
            if let Some(_existing_notes) =
                &self.inner.metadata.validation_and_review.get_validation_notes()
            {
                self.inner
                    .metadata
                    .validation_and_review
                    .update_validation_notes(validation_notes)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
            } else {
                self.inner
                    .metadata
                    .validation_and_review
                    .add_validation_notes(validation_notes)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
            }
        }

        // Update reviewed status
        self.inner.metadata.validation_and_review.update_reviewed(true);

        Ok(())
    }

    /// Update the validaation notes
    fn update_validation_notes(&mut self, notes: String) -> PyResult<()> {
        self.inner
            .metadata
            .validation_and_review
            .update_validation_notes(notes)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Get the validation status of the codelist
    fn get_validation_status(&self) -> Option<String> {
        self.inner.metadata.validation_and_review.status.clone()
    }

    /// Get the validation notes of the codelist
    fn get_validation_notes(&self) -> Option<String> {
        self.inner.metadata.validation_and_review.validation_notes.clone()
    }

    /// Get the reviewer of the codelist
    fn get_reviewer(&self) -> Option<String> {
        self.inner.metadata.validation_and_review.reviewer.clone()
    }

    /// Validate the codelist based on the codelist type
    #[pyo3(signature = (custom_regex=None))]
    fn validate_codes(&self, custom_regex: Option<String>) -> PyResult<()> {
        match custom_regex {
            Some(regex_str) => {
                let regex = Regex::new(&regex_str)
                    .map_err(|e| PyValueError::new_err(format!("Invalid regex: {}", e)))?;
                self.inner.validate_codes(Some(&regex))
                    .map_err(|e| PyValueError::new_err(e.to_string()))?
            }
            None => self.inner.validate_codes(None)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        }
        Ok(())
    }

    /// Add a comment to the codelist
    fn add_comment(&mut self, code: String, comment: String) -> PyResult<()> {
        self.inner.add_comment(code, comment).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Update a comment in the codelist
    fn update_comment(&mut self, code: String, comment: String) -> PyResult<()> {
        self.inner
            .update_comment(code, comment)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Remove a comment from the codelist
    fn remove_comment(&mut self, code: String) -> PyResult<()> {
        self.inner.remove_comment(code).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Add a term to the codelist
    fn add_term(&mut self, code: String, term: String) -> PyResult<()> {
        self.inner.add_term(code, term).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Update a term in the codelist
    fn update_term(&mut self, code: String, term: String) -> PyResult<()> {
        self.inner.update_term(code, term).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }

    /// Remove a term from the codelist
    fn remove_term(&mut self, code: String) -> PyResult<()> {
        self.inner.remove_term(code).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(())
    }
}
