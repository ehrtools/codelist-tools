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
            Some(regex) => custom_validate_all_code(self, &regex),
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
        codelist::CodeList,
        codelist_options::CodeListOptions,
        errors::CodeListError,
        metadata::Metadata,
        types::CodeListType,
    };

    use super::*;
    use crate::validator::Validator;
    use regex::Regex;
    use std::sync::LazyLock;

    static TEST_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^[A-Z]{3}[!]{1}$").expect("Failed to compile test regex")
    });

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
        codelist.add_entry("ABC!".to_string(), None, None)?;
        assert!(codelist.validate_codes(Some(&TEST_REGEX)).is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_too_long() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist();
        codelist.add_entry("ABC!L".to_string(), None, None)?;
        let error = codelist.validate_codes(Some(&TEST_REGEX)).unwrap_err().to_string();
        assert!(error.contains("Some codes in the list are invalid. Details:"));
        assert!(error.contains("Code ABC!L contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_invalid_contents() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist();
        codelist.add_entry("100!".to_string(), None, None)?;
        let error = codelist.validate_codes(Some(&TEST_REGEX)).unwrap_err().to_string();
        assert!(error.contains("Some codes in the list are invalid. Details:"));
        assert!(error.contains("Code 100! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist();
        codelist.add_entry("ABC!".to_string(), None, None)?;
        codelist.add_entry("CDE!".to_string(), None, None)?;
        codelist.add_entry("ZOE!".to_string(), None, None)?;
        codelist.add_entry("FQH!".to_string(), None, None)?;
        codelist.add_entry("OKL!".to_string(), None, None)?;
        codelist.add_entry("MYP!".to_string(), None, None)?;
        codelist.add_entry("QNM!".to_string(), None, None)?;
        codelist.add_entry("KPL!".to_string(), None, None)?;
        assert!(codelist.validate_codes(Some(&TEST_REGEX)).is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_all_invalid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist();
        codelist.add_entry("A0P!".to_string(), Some("Gonorrhoea".to_string()), None)?;
        codelist.add_entry("AaB!".to_string(), Some("Pertussis".to_string()), None)?;
        codelist.add_entry("AAAAAAA!".to_string(), Some("Measles".to_string()), None)?;
        codelist.add_entry("AB".to_string(), Some("Lymphatic filariasis".to_string()), None)?;
        codelist.add_entry("abcd".to_string(), None, None)?;
        codelist.add_entry("abC!".to_string(), Some("Gout".to_string()), None)?;
        codelist.add_entry("OPP!!".to_string(), Some("Down Syndrome".to_string()), None)?;
        codelist.add_entry("!!PP".to_string(), Some("Dental caries".to_string()), None)?;
        let error = codelist.validate_codes(Some(&TEST_REGEX)).unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code A0P! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code AaB! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code AAAAAAA! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code AB contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code abcd contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code abC! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code OPP!! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code !!PP contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));

        assert!(
            matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 8)
        );
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_mixed_invalid_and_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist();
        codelist.add_entry("A54!p".to_string(), None, None)?;
        codelist.add_entry("1009!".to_string(), None, None)?;
        codelist.add_entry("A0p5!".to_string(), None, None)?;
        codelist.add_entry("aab!".to_string(), None, None)?;
        codelist.add_entry("ABC!".to_string(), None, None)?;
        codelist.add_entry("LPK!".to_string(), None, None)?;
        codelist.add_entry("FLP!".to_string(), None, None)?;
        codelist.add_entry("GVM!".to_string(), None, None)?;
        let error = codelist.validate_codes(Some(&TEST_REGEX)).unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code A54!p contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code 1009! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code A0p5! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code aab! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));

        assert!(
            matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 4)
        );
        Ok(())
    }
}

