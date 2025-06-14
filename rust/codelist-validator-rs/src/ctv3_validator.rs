//! CTV3 validator for validating CTV3 codes in a codelist
//! 
//! Validation Rules
//! 1. The code must be exactly 5 characters in length.
//! 2. Only alphanumeric characters (a-z, A-Z, 0-9) and dots (.) are allowed.
//! 3. The code must match one of these patterns:
//!    - 5 alphanumeric characters (e.g. "Af918")
//!    - 4 alphanumeric characters followed by a dot (e.g. "ABb1.")
//!    - 3 alphanumeric characters followed by two dots (e.g. "Me4..")
//!    - 2 alphanumeric characters followed by three dots (e.g. "Fb...")
//!    - 1 alphanumeric character followed by four dots (e.g. "F....")
use std::sync::LazyLock;

use codelist_rs::codelist::CodeList;
use regex::Regex;

use crate::{errors::CodeListValidatorError, validator::CodeValidator};

pub struct Ctv3Validator<'a>(pub &'a CodeList);

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:[a-zA-Z0-9]{5}|[a-zA-Z0-9]{4}\.|[a-zA-Z0-9]{3}\.\.|[a-zA-Z0-9]{2}\.\.\.|[a-zA-Z0-9]{1}\.\.\.\.)$").expect("Unable to create regex")
});

impl CodeValidator for Ctv3Validator<'_> {
    fn validate_code(&self, code: &str) -> Result<(), CodeListValidatorError> {
        if code.len() > 5 {
            return Err(CodeListValidatorError::invalid_code_length(
                code,
                "Code is greater than 5 characters in length",
                self.0.codelist_type.to_string(),
            ));
        }

        if code.len() < 5 {
            return Err(CodeListValidatorError::invalid_code_length(
                code,
                "Code is less than 5 characters in length",
                self.0.codelist_type.to_string(),
            ));
        }

        if !REGEX.is_match(code) {
            return Err(CodeListValidatorError::invalid_code_contents(
                code,
                "Code does not match the expected format",
                self.0.codelist_type.to_string(),
            ));
        }

        Ok(())
    }

    fn validate_all_code(&self) -> Result<(), CodeListValidatorError> {
        let mut reasons = Vec::new();

        for (code, _) in self.0.entries.iter() {
            if let Err(err) = self.validate_code(code) {
                reasons.push(err.to_string());
            }
        }

        if reasons.is_empty() {
            Ok(())
        } else {
            Err(CodeListValidatorError::invalid_codelist(reasons))
        }
    }
}

#[cfg(test)]
mod tests {
    use codelist_rs::{
        codelist::CodeList,
        errors::CodeListError,
        metadata::{
            categorisation_and_usage::CategorisationAndUsage, metadata_source::Source,
            provenance::Provenance, purpose_and_context::PurposeAndContext,
            validation_and_review::ValidationAndReview, Metadata,
        },
        types::CodeListType,
    };

    use super::*;
    use crate::validator::Validator;

    // Helper function to create test metadata
    fn create_test_metadata() -> Metadata {
        Metadata::new(
            Provenance::new(Source::ManuallyCreated, None),
            CategorisationAndUsage::new(None, None, None),
            PurposeAndContext::new(None, None, None),
            ValidationAndReview::new(None, None, None, None, None),
        )
    }

    // Helper function to create a test codelist with two entries, default options
    // and test metadata
    fn create_test_codelist() -> Result<CodeList, CodeListError> {
        let codelist = CodeList::new(
            "test_codelist".to_string(),
            CodeListType::CTV3,
            create_test_metadata(),
            None,
        );
        Ok(codelist)
    }

    #[test]
    fn test_validate_codelist_with_valid_code() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let _ = codelist.add_entry("A9f..".to_string(), None, None);
        assert!(codelist.validate_codes().is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_too_long() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = Ctv3Validator(&codelist);
        let code: &'static str = "A009000000";
        let error = validator.validate_code(code).unwrap_err().to_string();
        assert_eq!(error, "Code A009000000 is an invalid length for type CTV3. Reason: Code is greater than 5 characters in length");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_too_short() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = Ctv3Validator(&codelist);
        let code = "Af.";
        let error = validator.validate_code(code).unwrap_err().to_string();
        assert_eq!(error, "Code Af. is an invalid length for type CTV3. Reason: Code is less than 5 characters in length");
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_dot_first_character() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = Ctv3Validator(&codelist);
        let code = ".a009";
        let error = validator.validate_code(code).unwrap_err().to_string();
        assert_eq!(error, "Code .a009 contents is invalid for type CTV3. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_dot_middle_character_between_letters() -> Result<(), CodeListError>
    {
        let codelist = create_test_codelist()?;
        let validator = Ctv3Validator(&codelist);
        let code = "10a.f";
        let error = validator.validate_code(code).unwrap_err().to_string();
        assert_eq!(error, "Code 10a.f contents is invalid for type CTV3. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_invalid_characters() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = Ctv3Validator(&codelist);
        let code = "Af!!!";
        let error = validator.validate_code(code).unwrap_err().to_string();
        assert_eq!(error, "Code Af!!! contents is invalid for type CTV3. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("Af918".to_string(), None, None)?;
        codelist.add_entry("ABb..".to_string(), None, None)?;
        codelist.add_entry("alkif".to_string(), None, None)?;
        codelist.add_entry("F....".to_string(), None, None)?;
        codelist.add_entry("bn89.".to_string(), None, None)?;
        codelist.add_entry("Me...".to_string(), None, None)?;
        codelist.add_entry("99999".to_string(), None, None)?;
        codelist.add_entry("kk98.".to_string(), None, None)?;
        assert!(codelist.validate_codes().is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_all_invalid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("A00900000".to_string(), None, None)?;
        codelist.add_entry("10".to_string(), None, None)?;
        codelist.add_entry("a.9jb".to_string(), None, None)?;
        codelist.add_entry("..9jJ".to_string(), None, None)?;
        codelist.add_entry("A00A".to_string(), None, None)?;
        codelist.add_entry("*unf.".to_string(), None, None)?;
        codelist.add_entry("..j..".to_string(), None, None)?;
        codelist.add_entry("9874ji".to_string(), None, None)?;
        let error = codelist.validate_codes().unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code A00900000 is an invalid length for type CTV3. Reason: Code is greater than 5 characters in length"));
        assert!(error_string.contains("Code 10 is an invalid length for type CTV3. Reason: Code is less than 5 characters in length"));
        assert!(error_string.contains("Code a.9jb contents is invalid for type CTV3. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code ..9jJ contents is invalid for type CTV3. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A00A is an invalid length for type CTV3. Reason: Code is less than 5 characters in length"));
        assert!(error_string.contains("Code *unf. contents is invalid for type CTV3. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code ..j.. contents is invalid for type CTV3. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code 9874ji is an invalid length for type CTV3. Reason: Code is greater than 5 characters in length"));

        assert!(
            matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 8)
        );
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_mixed_invalid_and_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("A54..".to_string(), None, None)?;
        codelist.add_entry("1009.".to_string(), None, None)?;
        codelist.add_entry("jk90L".to_string(), None, None)?;
        codelist.add_entry("LK...".to_string(), None, None)?;
        codelist.add_entry("N40".to_string(), None, None)?;
        codelist.add_entry("A00.l".to_string(), None, None)?;
        codelist.add_entry("Q90.....".to_string(), None, None)?;
        codelist.add_entry("A..9k".to_string(), None, None)?;
        let error = codelist.validate_codes().unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code N40 is an invalid length for type CTV3. Reason: Code is less than 5 characters in length"));
        assert!(error_string.contains("Code A00.l contents is invalid for type CTV3. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code Q90..... is an invalid length for type CTV3. Reason: Code is greater than 5 characters in length"));
        assert!(error_string.contains("Code A..9k contents is invalid for type CTV3. Reason: Code does not match the expected format"));

        assert!(
            matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 4)
        );
        Ok(())
    }
}
