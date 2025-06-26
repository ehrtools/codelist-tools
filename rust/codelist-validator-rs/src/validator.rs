//! Generic trait for validating a codelist
use regex::Regex;

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
    fn validate_codes(&self, custom_regex: Option<&Regex>) -> Result<(), CodeListValidatorError>;
}

impl Validator for CodeList {
    fn validate_codes(&self, custom_regex: Option<&Regex>) -> Result<(), CodeListValidatorError> {
        match custom_regex {
            Some(regex) => custom_validate_all_code(self, regex),
            None => match self.codelist_type {
                CodeListType::ICD10 => IcdValidator(self).validate_all_code(),
                CodeListType::SNOMED => SnomedValidator(self).validate_all_code(),
                CodeListType::OPCS => OpcsValidator(self).validate_all_code(),
                CodeListType::CTV3 => Ctv3Validator(self).validate_all_code(),
            },
        }
    }
}

/// Validate all codes in the codelist using a custom regex
///
/// # Arguments
/// * `codelist` - The codelist to validate
/// * `regex` - The regex to use to validate the codes
///
/// # Returns
/// * `Result<(), CodeListValidatorError>` - Ok(()) if all codes match the custom regex pattern, Err(CodeListValidatorError) otherwise
fn custom_validate_all_code(codelist: &CodeList, re: &Regex) -> Result<(), CodeListValidatorError> {
    let mut reasons = Vec::new();
    for (code, _) in codelist.entries.iter() {
        if !re.is_match(code) {
            reasons.push(
                CodeListValidatorError::invalid_code_contents(
                    code,
                    "Code does not match the custom regex pattern",
                    codelist.codelist_type.to_string(),
                )
                .to_string(),
            );
        }
    }

    if reasons.is_empty() {
        Ok(())
    } else {
        Err(CodeListValidatorError::invalid_codelist(reasons))
    }
}

#[cfg(test)]
mod tests {
    use codelist_rs::{
        codelist::CodeList, codelist_options::CodeListOptions, errors::CodeListError,
        metadata::Metadata, types::CodeListType,
    };

    use super::*;
    use crate::validator::Validator;
    use regex::Regex;
    use std::sync::LazyLock;

    static TEST_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^B\d{2}$").expect("Failed to compile test regex"));

    // Helper function to create a test codelist with two entries, default options
    // and test metadata
    fn create_test_codelist() -> CodeList {
        let options = CodeListOptions {
            allow_duplicates: true,
            code_column_name: "test_code".to_string(),
            term_column_name: "test_term".to_string(),
            code_field_name: "test_code".to_string(),
            term_field_name: "test_term".to_string(),
        };

        CodeList::new(
            "test_codelist".to_string(),
            CodeListType::ICD10,
            Metadata::default(),
            Some(options),
        )
    }

    #[test]
    fn test_validate_code_with_valid_code() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist();
        codelist.add_entry("B11".to_string(), None, None)?;
        assert!(codelist.validate_codes(Some(&TEST_REGEX)).is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_too_long() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist();
        codelist.add_entry("B1123".to_string(), None, None)?;
        let error = codelist.validate_codes(Some(&TEST_REGEX)).unwrap_err().to_string();
        assert!(error.contains("Some codes in the list are invalid. Details:"));
        assert!(error.contains("Code B1123 contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_invalid_contents() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist();
        codelist.add_entry("!!!".to_string(), None, None)?;
        let error = codelist.validate_codes(Some(&TEST_REGEX)).unwrap_err().to_string();
        assert!(error.contains("Some codes in the list are invalid. Details:"));
        assert!(error.contains("Code !!! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist();
        codelist.add_entry("B01".to_string(), None, None)?;
        codelist.add_entry("B02".to_string(), None, None)?;
        codelist.add_entry("B03".to_string(), None, None)?;
        codelist.add_entry("B04".to_string(), None, None)?;
        codelist.add_entry("B05".to_string(), None, None)?;
        codelist.add_entry("B06".to_string(), None, None)?;
        codelist.add_entry("B07".to_string(), None, None)?;
        codelist.add_entry("B08".to_string(), None, None)?;
        assert!(codelist.validate_codes(Some(&TEST_REGEX)).is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_all_invalid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist();
        codelist.add_entry("B011!".to_string(), None, None)?;
        codelist.add_entry("B0A".to_string(), None, None)?;
        codelist.add_entry("A03".to_string(), None, None)?;
        codelist.add_entry("BK4".to_string(), None, None)?;
        codelist.add_entry("B".to_string(), None, None)?;
        codelist.add_entry("BA907".to_string(), None, None)?;
        codelist.add_entry("B07x".to_string(), None, None)?;
        codelist.add_entry("b08".to_string(), None, None)?;
        let error = codelist.validate_codes(Some(&TEST_REGEX)).unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code B011! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code B0A contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code A03 contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code BK4 contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code B contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code BA907 contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code B07x contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code b08 contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));

        assert!(
            matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 8)
        );
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_mixed_invalid_and_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist();
        codelist.add_entry("B01".to_string(), None, None)?;
        codelist.add_entry("B02".to_string(), None, None)?;
        codelist.add_entry("B03".to_string(), None, None)?;
        codelist.add_entry("B04".to_string(), None, None)?;
        codelist.add_entry("B".to_string(), None, None)?;
        codelist.add_entry("BA907".to_string(), None, None)?;
        codelist.add_entry("B07x".to_string(), None, None)?;
        codelist.add_entry("b08".to_string(), None, None)?;
        let error = codelist.validate_codes(Some(&TEST_REGEX)).unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code B contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code BA907 contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code B07x contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code b08 contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));

        assert!(
            matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 4)
        );
        Ok(())
    }
}
