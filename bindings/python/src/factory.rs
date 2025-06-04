#![allow(non_local_definitions)]
use codelist_rs::{
    codelist_factory::CodeListFactory,
    codelist_options::CodeListOptions,
    metadata::{
        CategorisationAndUsage, Metadata, Provenance, PurposeAndContext, Source,
        ValidationAndReview,
    },
    types::CodeListType,
};
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::codelist::PyCodeList;

#[pyclass(name = "CodeListFactory")]
pub struct PyCodeListFactory {
    inner: CodeListFactory,
}

#[pymethods]
impl PyCodeListFactory {
    #[new]
    #[pyo3(signature = (codelist_type))]
    fn new(codelist_type: &str) -> PyResult<Self> {
        let codelist_type = match codelist_type.to_uppercase().as_str() {
            "ICD10" => CodeListType::ICD10,
            "SNOMED" => CodeListType::SNOMED,
            "OPCS" => CodeListType::OPCS,
            _ => {
                return Err(PyValueError::new_err(format!(
                    "Invalid codelist type: {codelist_type}"
                )))
            }
        };

        let metadata = Metadata::new(
            Provenance::new(Source::ManuallyCreated, None),
            CategorisationAndUsage::new(None, None, None),
            PurposeAndContext::new(None, None, None),
            ValidationAndReview::new(None, None, None, None, None),
        );

        let options = CodeListOptions::default();
        let factory = CodeListFactory::new(options, metadata, codelist_type);

        Ok(Self { inner: factory })
    }

    #[pyo3(signature = (name, path))]
    fn load_from_file(&self, name: String, path: String) -> PyResult<PyCodeList> {
        self.inner
            .load_codelist_from_file(name, &path)
            .map(|codelist| PyCodeList { inner: codelist })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
