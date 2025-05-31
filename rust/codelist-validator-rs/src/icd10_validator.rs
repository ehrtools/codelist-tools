use std::sync::LazyLock;

use codelist_rs::codelist::CodeList;
use regex::Regex;

use crate::{errors::CodeListValidatorError, validator::CodeValidator};

pub struct IcdValidator<'a>(pub &'a CodeList);

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[A-Z]\d{2}(X|(\.\d{1,3})?|\d{1,4})?$").expect("Unable to create regex")
});

impl CodeValidator for IcdValidator<'_> {
    fn validate_code(&self, code: &str) -> Result<(), CodeListValidatorError> {
        if code.len() > 7 {
            return Err(CodeListValidatorError::invalid_code_length(
                code,
                "Code is greater than 7 characters in length",
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
            CodeListType::ICD10,
            create_test_metadata(),
            None,
        );
        Ok(codelist)
    }

    #[test]
    fn test_validate_code_with_valid_code() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let _ = codelist.add_entry("A100".to_string(), Some("test".to_string()), None);
        assert!(codelist.validate_codes().is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_too_long() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = IcdValidator(&codelist);
        let code = "A009000000";
        let error = validator.validate_code(code).unwrap_err().to_string();
        assert_eq!(error, "Code A009000000 is an invalid length for type ICD10. Reason: Code is greater than 7 characters in length");
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_first_character_not_a_letter() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = IcdValidator(&codelist);
        let code = "1009";
        let error = validator.validate_code(code).unwrap_err().to_string();
        assert_eq!(error, "Code 1009 contents is invalid for type ICD10. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_second_character_not_a_number() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = IcdValidator(&codelist);
        let code = "AA09";
        let error = validator.validate_code(code).unwrap_err().to_string();
        assert_eq!(error, "Code AA09 contents is invalid for type ICD10. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_third_character_not_a_number() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = IcdValidator(&codelist);
        let code = "A0A9";
        let error = validator.validate_code(code).unwrap_err().to_string();
        assert_eq!(error, "Code A0A9 contents is invalid for type ICD10. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_fourth_character_not_a_dot_number_or_x(
    ) -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = IcdValidator(&codelist);
        let code = "A00A";
        let error = validator.validate_code(code).unwrap_err().to_string();
        assert_eq!(error, "Code A00A contents is invalid for type ICD10. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_no_number_after_fourth_character_dot() -> Result<(), CodeListError>
    {
        let codelist = create_test_codelist()?;
        let validator = IcdValidator(&codelist);
        let code = "A00.A";
        let error = validator.validate_code(code).unwrap_err().to_string();
        assert_eq!(error, "Code A00.A contents is invalid for type ICD10. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_characters_after_fourth_character_x() -> Result<(), CodeListError>
    {
        let codelist = create_test_codelist()?;
        let validator = IcdValidator(&codelist);
        let code = "A00X12";
        let error = validator.validate_code(code).unwrap_err().to_string();
        assert_eq!(error, "Code A00X12 contents is invalid for type ICD10. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_fifth_to_seventh_characters_not_numbers(
    ) -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = IcdValidator(&codelist);
        let code = "A00.4AA";
        let error = validator.validate_code(code).unwrap_err().to_string();
        assert_eq!(error, "Code A00.4AA contents is invalid for type ICD10. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("A54".to_string(), Some("Gonorrhoea".to_string()), None)?;
        codelist.add_entry("A37".to_string(), Some("Pertussis".to_string()), None)?;
        codelist.add_entry("A05".to_string(), Some("Measles".to_string()), None)?;
        codelist.add_entry("B74.0".to_string(), Some("Lymphatic filariasis".to_string()), None)?;
        codelist.add_entry(
            "N40".to_string(),
            Some("Benign prostatic hypertrophy".to_string()),
            None,
        )?;
        codelist.add_entry("M10".to_string(), Some("Gout".to_string()), None)?;
        codelist.add_entry("Q90".to_string(), Some("Down Syndrome".to_string()), None)?;
        codelist.add_entry("K02".to_string(), Some("Dental caries".to_string()), None)?;
        assert!(codelist.validate_codes().is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_all_invalid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("A009000000".to_string(), Some("Gonorrhoea".to_string()), None)?;
        codelist.add_entry("1009".to_string(), Some("Pertussis".to_string()), None)?;
        codelist.add_entry("AA09".to_string(), Some("Measles".to_string()), None)?;
        codelist.add_entry("A0A9".to_string(), Some("Lymphatic filariasis".to_string()), None)?;
        codelist.add_entry(
            "A00A".to_string(),
            Some("Benign prostatic hypertrophy".to_string()),
            None,
        )?;
        codelist.add_entry("A00.A".to_string(), Some("Gout".to_string()), None)?;
        codelist.add_entry("A00X12".to_string(), Some("Down Syndrome".to_string()), None)?;
        codelist.add_entry("A00.4AA".to_string(), Some("Dental caries".to_string()), None)?;
        let error = codelist.validate_codes().unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code A009000000 is an invalid length for type ICD10. Reason: Code is greater than 7 characters in length"));
        assert!(error_string.contains("Code 1009 contents is invalid for type ICD10. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code AA09 contents is invalid for type ICD10. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A0A9 contents is invalid for type ICD10. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A00A contents is invalid for type ICD10. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A00.A contents is invalid for type ICD10. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A00X12 contents is invalid for type ICD10. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A00.4AA contents is invalid for type ICD10. Reason: Code does not match the expected format"));

        assert!(
            matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 8)
        );
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_mixed_invalid_and_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("A54".to_string(), Some("Gonorrhoea".to_string()), None)?;
        codelist.add_entry("1009".to_string(), Some("Pertussis".to_string()), None)?;
        codelist.add_entry("A05".to_string(), Some("Measles".to_string()), None)?;
        codelist.add_entry("A0A9".to_string(), Some("Lymphatic filariasis".to_string()), None)?;
        codelist.add_entry(
            "N40".to_string(),
            Some("Benign prostatic hypertrophy".to_string()),
            None,
        )?;
        codelist.add_entry("A00.A".to_string(), Some("Gout".to_string()), None)?;
        codelist.add_entry("Q90".to_string(), Some("Down Syndrome".to_string()), None)?;
        codelist.add_entry("A00.4AA".to_string(), Some("Dental caries".to_string()), None)?;
        let error = codelist.validate_codes().unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code 1009 contents is invalid for type ICD10. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A0A9 contents is invalid for type ICD10. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A00.A contents is invalid for type ICD10. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A00.4AA contents is invalid for type ICD10. Reason: Code does not match the expected format"));

        assert!(
            matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 4)
        );
        Ok(())
    }
}
