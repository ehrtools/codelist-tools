use std::sync::LazyLock;

use codelist_rs::codelist::CodeList;
use regex::Regex;

use crate::{errors::CodeListValidatorError, validator::CodeValidator};

pub struct OpcsValidator<'a>(pub &'a CodeList);

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[A-Z]\d{2}(\.\d{1,2}|\d{1,2})?$").expect("Unable to create regex")
});

impl CodeValidator for OpcsValidator<'_> {
    fn validate_code(&self, code: &str) -> Result<(), CodeListValidatorError> {
        if code.len() > 5 {
            return Err(CodeListValidatorError::invalid_code_length(
                code,
                "Code is greater than 5 characters in length",
                self.0.codelist_type.to_string(),
            ));
        }

        if code.len() < 3 {
            return Err(CodeListValidatorError::invalid_code_length(
                code,
                "Code is less than 3 characters in length",
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
            CodeListType::OPCS,
            create_test_metadata(),
            None,
        )?;
        Ok(codelist)
    }

    #[test]
    fn test_validate_code_with_valid_code() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        assert!(codelist.validate_codes().is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_less_than_3_characters(
    ) -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = OpcsValidator(&codelist);
        let code = "A0";
        let error = validator.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code A0 is an invalid length for type OPCS. Reason: Code is less than 3 characters in length");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_greater_than_5_characters(
    ) -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = OpcsValidator(&codelist);
        let code = "A01000";
        let error = validator.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code A01000 is an invalid length for type OPCS. Reason: Code is greater than 5 characters in length");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_first_character_not_a_letter(
    ) -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = OpcsValidator(&codelist);
        let code = "101";
        let error = validator.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code 101 contents is invalid for type OPCS. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_second_character_not_a_number(
    ) -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = OpcsValidator(&codelist);
        let code = "AA1";
        let error = validator.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code AA1 contents is invalid for type OPCS. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_third_character_not_a_number(
    ) -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = OpcsValidator(&codelist);
        let code = "A0A";
        let error = validator.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code A0A contents is invalid for type OPCS. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_no_fifth_character_after_dot(
    ) -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = OpcsValidator(&codelist);
        let code = "A01.";
        let error = validator.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code A01. contents is invalid for type OPCS. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_fifth_character_after_dot_not_a_number(
    ) -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = OpcsValidator(&codelist);
        let code = "A01.A";
        let error = validator.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code A01.A contents is invalid for type OPCS. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_fifth_character_not_a_number(
    ) -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = OpcsValidator(&codelist);
        let code = "A010A";
        let error = validator.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code A010A contents is invalid for type OPCS. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("C01".to_string(), Some("Excision of eye".to_string()), None)?;
        codelist.add_entry(
            "C02".to_string(),
            Some("Extirpation of lesion of orbit".to_string()),
            None,
        )?;
        codelist.add_entry(
            "C03".to_string(),
            Some("Insertion of prosthesis of eye".to_string()),
            None,
        )?;
        codelist.add_entry(
            "C04".to_string(),
            Some("Attention to prosthesis of eye".to_string()),
            None,
        )?;
        codelist.add_entry(
            "C05".to_string(),
            Some("Plastic repair of orbit ".to_string()),
            None,
        )?;
        codelist.add_entry(
            "L31.4".to_string(),
            Some("Insertion Artery Carotid Stent Transluminal Percutaneous".to_string()),
            None,
        )?;
        codelist.add_entry(
            "L35.3".to_string(),
            Some("Insertion Artery Cerebral Stent Transluminal Percutaneous".to_string()),
            None,
        )?;
        codelist.add_entry(
            "L47.4".to_string(),
            Some("Insertion Artery Coeliac Stent Transluminal Percutaneous".to_string()),
            None,
        )?;
        assert!(codelist.validate_codes().is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_all_invalid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("A0".to_string(), Some("Excision of eye".to_string()), None)?;
        codelist.add_entry(
            "A01000".to_string(),
            Some("Extirpation of lesion of orbit".to_string()),
            None,
        )?;
        codelist.add_entry(
            "101".to_string(),
            Some("Insertion of prosthesis of eye".to_string()),
            None,
        )?;
        codelist.add_entry(
            "AA1".to_string(),
            Some("Attention to prosthesis of eye".to_string()),
            None,
        )?;
        codelist.add_entry(
            "A0A".to_string(),
            Some("Plastic repair of orbit ".to_string()),
            None,
        )?;
        codelist.add_entry(
            "A01.".to_string(),
            Some("Insertion Artery Carotid Stent Transluminal Percutaneous".to_string()),
            None,
        )?;
        codelist.add_entry(
            "A01.A".to_string(),
            Some("Insertion Artery Cerebral Stent Transluminal Percutaneous".to_string()),
            None,
        )?;
        codelist.add_entry(
            "A010A".to_string(),
            Some("Insertion Artery Coeliac Stent Transluminal Percutaneous".to_string()),
            None,
        )?;
        let error = codelist.validate_codes().unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code A0 is an invalid length for type OPCS. Reason: Code is less than 3 characters in length"));
        assert!(error_string.contains("Code A01000 is an invalid length for type OPCS. Reason: Code is greater than 5 characters in length"));
        assert!(error_string.contains("Code 101 contents is invalid for type OPCS. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code AA1 contents is invalid for type OPCS. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A0A contents is invalid for type OPCS. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A01. contents is invalid for type OPCS. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A01.A contents is invalid for type OPCS. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A010A contents is invalid for type OPCS. Reason: Code does not match the expected format"));

        assert!(
            matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 8)
        );
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_mixed_invalid_and_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("C01".to_string(), Some("Excision of eye".to_string()), None)?;
        codelist.add_entry(
            "A01000".to_string(),
            Some("Extirpation of lesion of orbit".to_string()),
            None,
        )?;
        codelist.add_entry(
            "C03".to_string(),
            Some("Insertion of prosthesis of eye".to_string()),
            None,
        )?;
        codelist.add_entry(
            "AA1".to_string(),
            Some("Attention to prosthesis of eye".to_string()),
            None,
        )?;
        codelist.add_entry(
            "C05".to_string(),
            Some("Plastic repair of orbit ".to_string()),
            None,
        )?;
        codelist.add_entry(
            "A01.".to_string(),
            Some("Insertion Artery Carotid Stent Transluminal Percutaneous".to_string()),
            None,
        )?;
        codelist.add_entry(
            "L35.3".to_string(),
            Some("Insertion Artery Cerebral Stent Transluminal Percutaneous".to_string()),
            None,
        )?;
        codelist.add_entry(
            "A010A".to_string(),
            Some("Insertion Artery Coeliac Stent Transluminal Percutaneous".to_string()),
            None,
        )?;
        let error = codelist.validate_codes().unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code A01000 is an invalid length for type OPCS. Reason: Code is greater than 5 characters in length"));
        assert!(error_string.contains("Code AA1 contents is invalid for type OPCS. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A01. contents is invalid for type OPCS. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A010A contents is invalid for type OPCS. Reason: Code does not match the expected format"));

        assert!(
            matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 4)
        );
        Ok(())
    }
}
