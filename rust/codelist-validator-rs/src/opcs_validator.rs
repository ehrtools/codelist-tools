use codelist_rs::codelist::CodeList;
use regex::Regex;
use std::sync::LazyLock;
use crate::errors::CodeListValidatorError;
use codelist_rs::metadata::{ Metadata, Provenance, CategorisationAndUsage, PurposeAndContext, ValidationAndReview };

/// OPCS code regex pattern
static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[A-Z]\d{2}(\.\d{1,2}|\d{1,2})?$").expect("Unable to create regex")
});

/// OPCS validator trait
/// 
/// `validate_code`: validates a single OPCS code
/// `validate_all_code`: validates all OPCS codes in the codelist
pub trait OPCSValidator {
    fn validate_code(&self, code: &str) -> Result<(), CodeListValidatorError>; // for 1 code
    fn validate_all_code(&self) -> Result<(), CodeListValidatorError>;
}

impl OPCSValidator for CodeList {
    /// Validate the form of a single OPCS code
    ///
    /// Rules:
    ///     - The code must be 3-5 characters long
    ///     - The first character must be a letter
    ///     - The second and third characters must be numbers
    ///     - If there is a fourth character and it is a dot, there must be a number after the dot
    ///     - The fifth character, if present, is a number
    /// 
    /// # Arguments
    /// 
    /// * `code`: the code to validate
    /// 
    /// # Returns
    /// 
    /// * `Result<(), >`: unit type if the code is valid, otherwise an error containing the code and the reason the code is invalid
    fn validate_code(&self, code: &str) -> Result<(), CodeListValidatorError> {
        if code.len() > 5 {
            return Err(CodeListValidatorError::invalid_code_length(code, "Code is greater than 5 characters in length", self.codelist_type.to_string()))
        }

        if code.len() < 3 {
            return Err(CodeListValidatorError::invalid_code_length(code, "Code is less than 3 characters in length", self.codelist_type.to_string()))
        }

        let re = &REGEX;

        if !re.is_match(code) {
            return Err(CodeListValidatorError::invalid_code_contents(code, "Code does not match the expected format", self.codelist_type.to_string()))
        }
        Ok(())
    }

    /// Validate all OPCS codes in the codelist
    /// 
    /// # Returns
    /// 
    /// * `Result<(), CodeListValidatorError>`: unit type if all codes are valid in the codelist, otherwise an error containing a vector of all invalid codes and the reason for being invalid
    fn validate_all_code(&self) -> Result<(), CodeListValidatorError> {
        let mut reasons  = Vec::new();

        for code_entry in self.entries.iter() {
            let code = &code_entry.code;
            if let Err(err) = self.validate_code(code) {
                let error_reason = err.to_string();
                reasons.push(error_reason);
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
    use super::*;
    use codelist_rs::metadata::{ Metadata, MetadataSource };
    use codelist_rs::codelist::CodeList;
    use codelist_rs::types::CodeListType;
    use codelist_rs::errors::CodeListError;

    // Helper function to create test metadata
    fn create_test_metadata() -> Metadata {
        Metadata::new(
            Provenance::new(MetadataSource::ManuallyCreated, None),
            CategorisationAndUsage::new(None, None, None),
            PurposeAndContext::new(None, None, None),
            ValidationAndReview::new(None, None, None, None, None),
        )
    }

    // Helper function to create a test codelist with two entries, default options and test metadata
    fn create_test_codelist() -> Result<CodeList, CodeListError> {
        let codelist = CodeList::new(CodeListType::OPCS, create_test_metadata(), None);
        Ok(codelist)
    }

    #[test]
    fn test_validate_code_with_valid_code() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "A01";
        assert!(codelist.validate_code(code).is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_less_than_3_characters() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "A0";
        let error = codelist.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code A0 is an invalid length for type OPCS. Reason: Code is less than 3 characters in length");
        Ok(())
    }


    #[test]
    fn test_validate_code_with_invalid_code_length_greater_than_5_characters() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "A01000";    
        let error = codelist.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code A01000 is an invalid length for type OPCS. Reason: Code is greater than 5 characters in length");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_first_character_not_a_letter() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "101";
        let error = codelist.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code 101 contents is invalid for type OPCS. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_second_character_not_a_number() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "AA1";
        let error = codelist.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code AA1 contents is invalid for type OPCS. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_third_character_not_a_number() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "A0A";
        let error = codelist.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code A0A contents is invalid for type OPCS. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_no_fifth_character_after_dot() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "A01.";
        let error = codelist.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code A01. contents is invalid for type OPCS. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_fifth_character_after_dot_not_a_number() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "A01.A";
        let error = codelist.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code A01.A contents is invalid for type OPCS. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_fifth_character_not_a_number() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "A010A";
        let error = codelist.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code A010A contents is invalid for type OPCS. Reason: Code does not match the expected format");
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("C01".to_string(), "Excision of eye".to_string(), None)?;
        codelist.add_entry("C02".to_string(), "Extirpation of lesion of orbit".to_string(), None)?;
        codelist.add_entry("C03".to_string(), "Insertion of prosthesis of eye".to_string(), None)?;
        codelist.add_entry("C04".to_string(), "Attention to prosthesis of eye".to_string(), None)?;
        codelist.add_entry("C05".to_string(), "Plastic repair of orbit ".to_string(), None)?;
        codelist.add_entry("L31.4".to_string(), "Insertion Artery Carotid Stent Transluminal Percutaneous".to_string(), None)?;
        codelist.add_entry("L35.3".to_string(), "Insertion Artery Cerebral Stent Transluminal Percutaneous".to_string(), None)?;
        codelist.add_entry("L47.4".to_string(), "Insertion Artery Coeliac Stent Transluminal Percutaneous".to_string(), None)?;
        assert!(codelist.validate_all_code().is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_all_invalid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("A0".to_string(), "Excision of eye".to_string(), None)?;
        codelist.add_entry("A01000".to_string(), "Extirpation of lesion of orbit".to_string(), None)?;
        codelist.add_entry("101".to_string(), "Insertion of prosthesis of eye".to_string(), None)?;
        codelist.add_entry("AA1".to_string(), "Attention to prosthesis of eye".to_string(), None)?;
        codelist.add_entry("A0A".to_string(), "Plastic repair of orbit ".to_string(), None)?;
        codelist.add_entry("A01.".to_string(), "Insertion Artery Carotid Stent Transluminal Percutaneous".to_string(), None)?;
        codelist.add_entry("A01.A".to_string(), "Insertion Artery Cerebral Stent Transluminal Percutaneous".to_string(), None)?;
        codelist.add_entry("A010A".to_string(), "Insertion Artery Coeliac Stent Transluminal Percutaneous".to_string(), None)?;
        let error = codelist.validate_all_code().unwrap_err();
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

        assert!(matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 8));
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_mixed_invalid_and_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("C01".to_string(), "Excision of eye".to_string(), None)?;
        codelist.add_entry("A01000".to_string(), "Extirpation of lesion of orbit".to_string(), None)?;
        codelist.add_entry("C03".to_string(), "Insertion of prosthesis of eye".to_string(), None)?;
        codelist.add_entry("AA1".to_string(), "Attention to prosthesis of eye".to_string(), None)?;
        codelist.add_entry("C05".to_string(), "Plastic repair of orbit ".to_string(), None)?;
        codelist.add_entry("A01.".to_string(), "Insertion Artery Carotid Stent Transluminal Percutaneous".to_string(), None)?;
        codelist.add_entry("L35.3".to_string(), "Insertion Artery Cerebral Stent Transluminal Percutaneous".to_string(), None)?;
        codelist.add_entry("A010A".to_string(), "Insertion Artery Coeliac Stent Transluminal Percutaneous".to_string(), None)?;
        let error = codelist.validate_all_code().unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code A01000 is an invalid length for type OPCS. Reason: Code is greater than 5 characters in length"));
        assert!(error_string.contains("Code AA1 contents is invalid for type OPCS. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A01. contents is invalid for type OPCS. Reason: Code does not match the expected format"));
        assert!(error_string.contains("Code A010A contents is invalid for type OPCS. Reason: Code does not match the expected format"));

        assert!(matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 4));
        Ok(())
    }

} 