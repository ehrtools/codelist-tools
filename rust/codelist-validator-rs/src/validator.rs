//! Generic trait for validating a codelist
use codelist_rs::{codelist::CodeList, types::CodeListType};

use crate::{
    ctv3_validator::Ctv3Validator, errors::CodeListValidatorError, icd10_validator::IcdValidator,
    opcs_validator::OpcsValidator, snomed_validator::SnomedValidator,
};

/// Validator trait for validating a codelist.
///
/// `validate_code`: validates a single code
/// `validate_all_code`: validates all codes in the codelist
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
            CodeListType::CTV3 => Ctv3Validator(self).validate_all_code(),
        }
    }
}
