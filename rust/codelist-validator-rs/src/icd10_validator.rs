use codelist_rs::codelist::CodeList;
use regex::Regex;
use std::sync::LazyLock;
use crate::errors::CodeListValidatorError;

/// ICD10 code regex pattern
static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[A-Z]\d{2}(X|(\.\d{1,3})?|\d{1,4})?$").expect("Unable to create regex")
});

/// ICD10 validator trait
/// 
/// `validate_code`: validates a single ICD10 code
/// `validate_all_code`: validates all ICD10 codes in the codelist
pub trait ICD10Validator {
    fn validate_code(&self, code: &str) -> Result<(), CodeListValidatorError>; // for 1 code
    fn validate_all_code(&self) -> Result<(), CodeListValidatorError>;
}

impl ICD10Validator for CodeList {
    /// Validate the form of a single ICD10 code
    ///
    /// The rules are:
    ///        - The code must be 7 characters or less
    ///        - The first character must be a letter
    ///        - The second and third characters must be numbers
    ///        - The fourth character must be a dot, or a number or X
    ///        - If the fourth character is a dot, there must be at least 1 number after the dot
    ///        - If the fourth character is a X, there are no further characters
    ///        - The fifth to seventh characters must be numbers if present
    /// 
    /// # Arguments
    /// 
    /// * `code`: the code to validate
    /// 
    /// # Returns
    /// 
    /// * `Result<(), CodeListValidatorError>`: unit type if the code is valid, otherwise an error containing the code and the reason the code is invalid
    fn validate_code(&self, code: &str) -> Result<(), CodeListValidatorError> {
        if code.len() > 7 {
            return Err(CodeListValidatorError::invalid_code_length(code, "ICD10 code is greater than 7 characters in length"));
        }

        let re = &REGEX;

        if !re.is_match(code) {
            return Err(CodeListValidatorError::invalid_code_contents(
                code,
                format!("ICD10 code {} does not match the expected format", code), // Corrected string interpolation
            ));
        }
        Ok(())
    }

    /// Validate all ICD10 codes in the codelist
    /// 
    /// # Returns
    /// 
    /// * `Result<(), CodeListValidatorError>`: unit type if all codes are valid in the codelist, otherwise an error containing all invalid codes and the reason for being invalid
    fn validate_all_code(&self) -> Result<(), CodeListValidatorError> {
        let mut invalid_codes = Vec::new();

        for code_entry in self.entries.iter() {
            let code = &code_entry.code;
            if let Err(err) = self.validate_code(code) {
                let error_reason = format!("{}", err);
                invalid_codes.push((code.clone(), error_reason));
            }
        }

        if invalid_codes.is_empty() {
            Ok(())
        } else {
            Err(CodeListValidatorError::invalid_codelist(invalid_codes))
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
        Metadata {
            source: MetadataSource::ManuallyCreated,
            authors: Some(vec!["Caroline Morton".to_string()]),
            version: Some("2024-12-19".to_string()),
            description: Some("A test codelist".to_string()),
        }
    }

    // Helper function to create a test codelist with two entries, default options and test metadata
    fn create_test_codelist() -> Result<CodeList, CodeListError> {
        let codelist = CodeList::new(CodeListType::ICD10, create_test_metadata(), None);
        Ok(codelist)
    }

    #[test]
    fn test_validate_code_with_valid_code() -> Result<(), CodeListError> {;
        let codelist = create_test_codelist()?;
        let code = "A009";
        assert!(codelist.validate_code(code).is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "A009000000";
        let error = codelist.validate_code(code).unwrap_err();
        assert!(matches!(error, CodeListValidatorError::InvalidCodeLength{code: c, reason: r} if c == code && r == "ICD10 code is greater than 7 characters in length"));
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_first_character_not_a_letter() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "1009";
        let error = codelist.validate_code(code).unwrap_err();
        assert!(matches!(error, CodeListValidatorError::InvalidCodeContents{code: c, reason: r} if c == code && r == "ICD10 code 1009 does not match the expected format"));
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_second_character_not_a_number() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "AA09";
        let error = codelist.validate_code(code).unwrap_err();
        assert!(matches!(error, CodeListValidatorError::InvalidCodeContents{code: c, reason: r} if c == code && r == "ICD10 code AA09 does not match the expected format"));
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_third_character_not_a_number() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "A0A9";
        let error = codelist.validate_code(code).unwrap_err();
        assert!(matches!(error, CodeListValidatorError::InvalidCodeContents{code: c, reason: r} if c == code && r == "ICD10 code A0A9 does not match the expected format"));
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_fourth_character_not_a_dot_number_or_x() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "A00A";
        let error = codelist.validate_code(code).unwrap_err();
        assert!(matches!(error, CodeListValidatorError::InvalidCodeContents{code: c, reason: r} if c == code && r == "ICD10 code A00A does not match the expected format"));
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_no_number_after_fourth_character_dot() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "A00.A";
        let error = codelist.validate_code(code).unwrap_err();
        assert!(matches!(error, CodeListValidatorError::InvalidCodeContents{code: c, reason: r} if c == code && r == "ICD10 code A00.A does not match the expected format"));
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_characters_after_fourth_character_x() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "A00X12";
        let error = codelist.validate_code(code).unwrap_err();
        assert!(matches!(error, CodeListValidatorError::InvalidCodeContents{code: c, reason: r} if c == code && r == "ICD10 code A00X12 does not match the expected format"));
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_fifth_to_seventh_characters_not_numbers() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "A00.4AA";
        let error = codelist.validate_code(code).unwrap_err();
        assert!(matches!(error, CodeListValidatorError::InvalidCodeContents{code: c, reason: r} if c == code && r == "ICD10 code A00.4AA does not match the expected format"));
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("A54".to_string(), "Gonorrhoea".to_string())?;
        codelist.add_entry("A37".to_string(), "Pertussis".to_string())?;
        codelist.add_entry("A05".to_string(), "Measles".to_string())?;
        codelist.add_entry("B74.0".to_string(), "Lymphatic filariasis".to_string())?;
        codelist.add_entry("N40".to_string(), "Benign prostatic hypertrophy".to_string())?;
        codelist.add_entry("M10".to_string(), "Gout".to_string())?;
        codelist.add_entry("Q90".to_string(), "Down Syndrome".to_string())?;
        codelist.add_entry("K02".to_string(), "Dental caries".to_string())?;
        assert!(codelist.validate_all_code().is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_all_invalid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("A009000000".to_string(), "Gonorrhoea".to_string())?;
        codelist.add_entry("1009".to_string(), "Pertussis".to_string())?;
        codelist.add_entry("AA09".to_string(), "Measles".to_string())?;
        codelist.add_entry("A0A9".to_string(), "Lymphatic filariasis".to_string())?;
        codelist.add_entry("A00A".to_string(), "Benign prostatic hypertrophy".to_string())?;
        codelist.add_entry("A00.A".to_string(), "Gout".to_string())?;
        codelist.add_entry("A00X12".to_string(), "Down Syndrome".to_string())?;
        codelist.add_entry("A00.4AA".to_string(), "Dental caries".to_string())?;
        let error = codelist.validate_all_code().unwrap_err();
        let error_reason = format!("{}", error);
        
        assert!(error_reason.contains("A009000000") && 
                error_reason.contains("Code A009000000 is an invalid length") &&
                error_reason.contains("ICD10 code is greater than 7 characters in length"));

        assert!(error_reason.contains("1009") && 
                error_reason.contains("Code 1009 contents is invalid") &&
                error_reason.contains("ICD10 code 1009 does not match the expected format"));

        assert!(error_reason.contains("AA09") && 
                error_reason.contains("Code AA09 contents is invalid") &&
                error_reason.contains("ICD10 code AA09 does not match the expected format")); 

        assert!(error_reason.contains("A0A9") && 
                error_reason.contains("Code A0A9 contents is invalid") &&
                error_reason.contains("ICD10 code A0A9 does not match the expected format"));

        assert!(error_reason.contains("A00A") && 
                error_reason.contains("Code A00A contents is invalid") &&
                error_reason.contains("ICD10 code A00A does not match the expected format"));

        assert!(error_reason.contains("A00.A") && 
                error_reason.contains("Code A00.A contents is invalid") &&
                error_reason.contains("ICD10 code A00.A does not match the expected format"));

        assert!(error_reason.contains("A00X12") && 
                error_reason.contains("Code A00X12 contents is invalid") &&
                error_reason.contains("ICD10 code A00X12 does not match the expected format"));

        assert!(error_reason.contains("A00.4AA") && 
                error_reason.contains("Code A00.4AA contents is invalid") &&
                error_reason.contains("ICD10 code A00.4AA does not match the expected format"));

        assert!(matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 8));
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_mixed_invalid_and_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("A54".to_string(), "Gonorrhoea".to_string())?;
        codelist.add_entry("1009".to_string(), "Pertussis".to_string())?;
        codelist.add_entry("A05".to_string(), "Measles".to_string())?;
        codelist.add_entry("A0A9".to_string(), "Lymphatic filariasis".to_string())?;
        codelist.add_entry("N40".to_string(), "Benign prostatic hypertrophy".to_string())?;
        codelist.add_entry("A00.A".to_string(), "Gout".to_string())?;
        codelist.add_entry("Q90".to_string(), "Down Syndrome".to_string())?;
        codelist.add_entry("A00.4AA".to_string(), "Dental caries".to_string())?;
        let error = codelist.validate_all_code().unwrap_err();
        let error_reason = format!("{}", error);

       assert!(error_reason.contains("1009") && 
               error_reason.contains("Code 1009 contents is invalid") &&
               error_reason.contains("ICD10 code 1009 does not match the expected format"));

       assert!(error_reason.contains("A0A9") && 
               error_reason.contains("Code A0A9 contents is invalid") &&
               error_reason.contains("ICD10 code A0A9 does not match the expected format"));

       assert!(error_reason.contains("A00.A") && 
               error_reason.contains("Code A00.A contents is invalid") &&
               error_reason.contains("ICD10 code A00.A does not match the expected format"));

       assert!(error_reason.contains("A00.4AA") && 
               error_reason.contains("Code A00.4AA contents is invalid") &&
               error_reason.contains("ICD10 code A00.4AA does not match the expected format"));

       assert!(matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 4));
       Ok(())
    }
}

