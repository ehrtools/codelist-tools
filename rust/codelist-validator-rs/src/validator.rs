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

/// Custom validator trait for validating a codelist with a custom regex pattern defined in the CodelistOptions
///
/// `custom_validate_all_code`: validates all codes in the codelist with the custom regex pattern
pub(crate) trait CustomCodeValidator {
    fn custom_validate_all_code(&self) -> Result<(), CodeListValidatorError>;
}
/// Validator trait
pub trait Validator {
    fn validate_codes(&self) -> Result<(), CodeListValidatorError>;
}

impl Validator for CodeList {
    fn validate_codes(&self) -> Result<(), CodeListValidatorError> {
        match &self.codelist_options.custom_regex {
            Some(_) => self.custom_validate_all_code(),
            None => match self.codelist_type {
                CodeListType::ICD10 => IcdValidator(self).validate_all_code(),
                CodeListType::SNOMED => SnomedValidator(self).validate_all_code(),
                CodeListType::OPCS => OpcsValidator(self).validate_all_code(),
                CodeListType::CTV3 => Ctv3Validator(self).validate_all_code(),
            },
        }
    }
}
