//! Trait for custom validation of a codelist

use codelist_rs::{codelist::CodeList, types::CodeListType};

use crate::{
    errors::CodeListValidatorError, icd10_validator::IcdValidator, opcs_validator::OpcsValidator,
    snomed_validator::SnomedValidator,
};

pub(crate) trait CustomCodeValidator {
    fn custom_validate_code(&self, code: &str) -> Result<(), CodeListValidatorError>; // for 1 code
    fn custom_validate_all_code(&self) -> Result<(), CodeListValidatorError>;
}

impl CustomCodeValidator for CodeList {
    fn custom_validate_code(&self, code: &str) -> Result<(), CodeListValidatorError> {
        Ok(())
        //TODO
    }

    fn custom_validate_all_code(&self) -> Result<(), CodeListValidatorError> {
        Ok(())
        //TODO
    }
}

/// Custom validator trait
pub trait CustomValidator {
    fn custom_validate_codes(&self) -> Result<(), CodeListValidatorError>;
}

impl CustomValidator for CodeList {
    fn custom_validate_codes(&self) -> Result<(), CodeListValidatorError> {
        match self.codelist_options.custom_regex {
            Some(_) => {
                self.custom_validate_all_code()
            }
            None => {
                Err(CodeListValidatorError::CustomValidationFailed(format!("No regex provided for custom validation")))
            }
        }
    }
}



// fn custom_validate_codes(&self) -> Result<(), CodeListValidatorError> {
    //     let mut reasons = Vec::new();

    //     for (code, _) in self.entries.iter() {
    //         if let Err(err) = self.custom_validate_code(code) {
    //             reasons.push(err.to_string());
    //         }
    //     }

    //     if reasons.is_empty() {
    //         Ok(())
    //     } else {
    //         Err(CodeListValidatorError::invalid_codelist(reasons))
    //     } // for (code, _) in self.entries.iter() {
    //     //     if re.is_match(code) {
    //     //         return Ok(());
    //     //     }
    //     // }
    //     // Err(CodeListValidatorError::CustomValidationFailed(format!("No codes matched the regex: {}", re.to_string())))
    // }

    // fn custom_validate_code(&self, code: &str, re: Regex) -> Result<(), CodeListValidatorError> {
    //     if !re.is_match(code) {
    //         return Err(CodeListValidatorError::invalid_code_contents(
    //             code,
    //             "Code does not match the inputted regex",
    //             self.codelist_type.to_string(),
    //         ));
    //     }

    //     Ok(())
    // }