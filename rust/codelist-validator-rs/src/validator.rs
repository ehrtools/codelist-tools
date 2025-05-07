//! Generic trait for validating a codelist
use crate::errors::{CodeListValidatorError};
use codelist_rs::codelist::CodeList;
use codelist_rs::types::CodeListType;
use crate::opcs_validator::OpcsValidator;
use crate::snomed_validator::SnomedValidator;
use crate::icd10_validator::IcdValidator;

/// Validator trait for validating a codelist.
///
/// `validate_code`: validates a single OPCS code
/// `validate_all_code`: validates all OPCS codes in the codelist
pub(crate) trait CodeValidator {
    fn validate_code(&self, code: &str) -> Result<(), CodeListValidatorError>; // for 1 code
    fn validate_all_code(&self) -> Result<(), CodeListValidatorError>;
}


/// Validator trait
pub trait Validator {
    fn validate_codes(&self) -> Result<(), CodeListValidatorError>;
}

impl Validator for CodeList {
    fn validate_codes(&self) -> Result<(), CodeListValidatorError> {
        match self.codelist_type {
            CodeListType::ICD10 => IcdValidator(self).validate_all_code(),
            CodeListType::SNOMED => SnomedValidator(self).validate_all_code(),
            CodeListType::OPCS => OpcsValidator(self).validate_all_code(),
            _ => Err(CodeListValidatorError::UnsupportedCodeType{code_type: self.codelist_type.to_string()}),
        }
    }
}
