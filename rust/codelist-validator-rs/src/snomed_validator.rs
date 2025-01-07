use codelist_rs::codelist::CodeList;
use crate::errors::CodeListValidatorError;

const MAX_LENGTH: u32 = 18;
const MIN_LENGTH: u32 = 6;

pub trait SNOMEDValidator {
    fn validate_code(&self, code: &str, min_length: u32, max_length: u32) -> Result<(), CodeListValidatorError>; // for 1 code
    fn validate_all_code(&self, min_length: Option<u32>, max_length: Option<u32>) -> Result<(), CodeListValidatorError>;
}

impl SNOMEDValidator for CodeList {
    /// Validate the form of a single SNOMED code
    ///
    /// Rules:
    ///     - The code must be a number
    ///     - The code must be between the minimum and maximum length (the default is minimum length 6 and maximum length 18)
    /// 
    /// # Arguments
    /// 
    /// * `code`: the code to validate
    /// * `min_length`: the minimum length of the code (default is 6)
    /// * `max_length`: the maximum length of the code (default is 18)
    /// 
    /// # Returns
    /// 
    /// * `Result<(), CodeListValidatorError>`: unit type if the code is valid, otherwise an error containing the code and the reason the code is invalid
    fn validate_code(&self, code: &str, min_length: u32, max_length: u32) -> Result<(), CodeListValidatorError> {
        code.trim().parse::<u128>().map_err(|e| CodeListValidatorError::ParseIntError {
            code: code.to_string(),
            reason: e.to_string(),
            codelist_type: self.codelist_type.to_string(),
        })?;
        let length = code.len() as u32;
        if length < min_length || length > max_length {
            return Err(CodeListValidatorError::invalid_code_length(code, format!("Code is not between {} and {} numbers in length", min_length, max_length), self.codelist_type.to_string()));
        }
        Ok(())
    }

    /// Validate all SNOMED codes in the codelist
    /// 
    /// # Arguments
    /// 
    /// * `min_length`: optional minimum length of the code (default is 6)
    /// * `max_length`: optional maximum length of the code (default is 18)
    /// 
    /// # Returns
    /// 
    /// * `Result<(), CodeListValidatorError>`: unit type if all codes are valid in the codelist, otherwise an error containing a vector of all invalid codes and the reason for being invalid
    fn validate_all_code(&self, min_length: Option<u32>, max_length: Option<u32>) -> Result<(), CodeListValidatorError> {
        let mut reasons = Vec::new();
        let min_length = min_length.unwrap_or(MIN_LENGTH);
        let max_length = max_length.unwrap_or(MAX_LENGTH);

        for code_entry in self.entries.iter() {
            let code = &code_entry.code;
            if let Err(err) = self.validate_code(code, min_length, max_length) {
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
        Metadata {
            source: MetadataSource::ManuallyCreated,
            authors: Some(vec!["Caroline Morton".to_string()]),
            version: Some("2024-12-19".to_string()),
            description: Some("A test codelist".to_string()),
        }
    }

    // Helper function to create a test codelist with two entries, default options and test metadata
    fn create_test_codelist() -> Result<CodeList, CodeListError> {
        let codelist = CodeList::new(CodeListType::SNOMED, create_test_metadata(), None);
        Ok(codelist)
    }

    #[test]
    fn test_validate_code_with_valid_code_default_max_min_lengths() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "204351007";
        assert!(codelist.validate_code(code, MIN_LENGTH, MAX_LENGTH).is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_code_with_valid_code_custom_max_min_lengths() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "1111111111111111111";
        assert!(codelist.validate_code(code, 18, 20).is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_not_all_numbers() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "11A6BB789A";
        let error = codelist.validate_code(code, MIN_LENGTH, MAX_LENGTH).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code 11A6BB789A is not composed of all numerical characters for type SNOMED. Reason: invalid digit found in string");
        Ok(())
    }


    #[test]
    fn test_validate_code_with_invalid_code_length_less_than_min_length_of_3() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "11";
        let error = codelist.validate_code(code, 3, 5).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code 11 is an invalid length for type SNOMED. Reason: Code is not between 3 and 5 numbers in length");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_greater_than_max_length_of_5() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "1111111";
        let error = codelist.validate_code(code, 3, 5).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code 1111111 is an invalid length for type SNOMED. Reason: Code is not between 3 and 5 numbers in length");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_less_than_default_min_length() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "2043";
        let error = codelist.validate_code(code, MIN_LENGTH, MAX_LENGTH).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code 2043 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_greater_than_default_max_length() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let code = "2043510071234567890";
        let error = codelist.validate_code(code, MIN_LENGTH, MAX_LENGTH).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code 2043510071234567890 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length");
        Ok(())
    }

    #[test]
    fn test_validate_all_code_with_valid_codelist() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("204351007".to_string(), "Fallot's trilogy (disorder)".to_string())?;
        codelist.add_entry("405752007".to_string(), "Congenital atrial septal defect (disorder)".to_string())?;
        codelist.add_entry("405752008".to_string(), "Congenital atrial septal defect with patent foramen ovale (disorder)".to_string())?;
        codelist.add_entry("77480004".to_string(), "Congenital biliary atresia (disorder)".to_string())?;
        codelist.add_entry("34000006".to_string(), "Crohn's disease (disorder)".to_string())?;
        codelist.add_entry("417357006".to_string(), "Sickling disorder due to hemoglobin S (disorder)".to_string())?;
        codelist.add_entry("24700007".to_string(), "Multiple sclerosis (disorder)".to_string())?;
        codelist.add_entry("398254007".to_string(), "Pre-eclampsia (disorder)".to_string())?;

        assert!(codelist.validate_all_code(None, None).is_ok());

        Ok(())
    }
    
    #[test]
    fn test_validate_codelist_with_all_invalid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("11".to_string(), "Fallot's trilogy (disorder)".to_string())?;
        codelist.add_entry("111111111111111111111111111111".to_string(), "Fallot's trilogy (disorder)".to_string())?;
        codelist.add_entry("AA090".to_string(), "Fallot's trilogy (disorder)".to_string())?;
        codelist.add_entry("BB".to_string(), "Fallot's trilogy (disorder)".to_string())?;
        codelist.add_entry("1".to_string(), "Fallot's trilogy (disorder)".to_string())?;
        codelist.add_entry("1111111111111111111AAAAAAAAAA".to_string(), "Fallot's trilogy (disorder)".to_string())?;
        codelist.add_entry("11111".to_string(), "Fallot's trilogy (disorder)".to_string())?;
        
        let error = codelist.validate_all_code(None, None).unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code 11 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length"));
        assert!(error_string.contains("Code 111111111111111111111111111111 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length"));
        assert!(error_string.contains("Code AA090 is not composed of all numerical characters for type SNOMED. Reason: invalid digit found in string"));
        assert!(error_string.contains("Code BB is not composed of all numerical characters for type SNOMED. Reason: invalid digit found in string"));
        assert!(error_string.contains("Code 1111111111111111111AAAAAAAAAA is not composed of all numerical characters for type SNOMED. Reason: invalid digit found in string"));
        assert!(error_string.contains("Code 11111 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length"));
        
        Ok(())
    }

    #[test]
    fn test_validate_all_code_with_mixed_valid_and_invalid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("11".to_string(), "Fallot's trilogy (disorder)".to_string())?;
        codelist.add_entry("405752007".to_string(), "Congenital atrial septal defect (disorder)".to_string())?;
        codelist.add_entry("AA090".to_string(), "Congenital atrial septal defect with patent foramen ovale (disorder)".to_string())?;
        codelist.add_entry("77480004".to_string(), "Congenital biliary atresia (disorder)".to_string())?;
        codelist.add_entry("34000006".to_string(), "Crohn's disease (disorder)".to_string())?;
        codelist.add_entry("417357006".to_string(), "Sickling disorder due to hemoglobin S (disorder)".to_string())?;
        codelist.add_entry("24700007".to_string(), "Multiple sclerosis (disorder)".to_string())?;
        codelist.add_entry("11111".to_string(), "Pre-eclampsia (disorder)".to_string())?;

        let error = codelist.validate_all_code(None, None).unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code 11 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length"));
        assert!(error_string.contains("Code AA090 is not composed of all numerical characters for type SNOMED. Reason: invalid digit found in string"));
        assert!(error_string.contains("Code 11111 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length"));
        
        Ok(())
    }
}