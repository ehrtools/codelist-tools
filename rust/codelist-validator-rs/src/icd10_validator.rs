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

/// Implementation of ICD10Validator trait for the CodeList struct
impl ICD10Validator for CodeList {
    /// Validate a single ICD10 code
    /// 
    /// # Arguments
    /// 
    /// * `code`: the code to validate
    /// 
    /// # Returns
    /// 
    /// * `Result<(), String>`: unit type if the code is valid, otherwise an error containing the code and the reason the code is invalid
    fn validate_code(&self, code: &str) -> Result<(), CodeListValidatorError> {
        if code.len() > 7 {
            return Err(CodeListValidatorError::invalid_code_length(code, "ICD10 code is not greater than 7 characters in length"));
        }

        let re = &*REGEX;

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
    use codelist_rs::codelist::{ CodeList, CodeListType };
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
        let mut codelist = CodeList::new(CodeListType::ICD10, create_test_metadata(), None);
        codelist.add_entry("R65.2".to_string(), "Severe sepsis".to_string())?;
        codelist.add_entry("A48.51".to_string(), "Infant botulism".to_string())?;
        
        Ok(codelist)
    }


    #[test]
    fn test_validate_code_with_valid_code() {;

    }

    #[test]
    fn test_validate_code_with_invalid_code_length() {;

    }

    #[test]
    fn test_validate_code_with_invalid_code_content() {;

    }

    #[test]
    fn test_validate_all_code_valid_codelist() {}
}