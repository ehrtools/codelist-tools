/// This module defines the different types of codelists that can be used

/// External imports
use std::str::FromStr;
use serde::{Serialize, Deserialize};

/// Internal imports
use crate::errors::CodeListError;


/// Enum to represent the different types of codelists
///
/// # Variants
/// * `ICD10` - The ICD10 codelist
/// * `SNOMED` - The SNOMED codelist
/// * `OPCS` - The OPCS codelist
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum CodeListType {
    ICD10,
    SNOMED,
    OPCS,
}


impl FromStr for CodeListType {
    type Err = CodeListError;
    /// Convert a string to a CodeListType
    ///
    /// # Arguments
    /// * `s` - The string to convert to a CodeListType
    ///
    /// # Returns
    /// * `Result<CodeListType, CodeListError>` - The CodeListType or a CodeListError
    ///
    /// # Errors
    /// * `CodeListError::InvalidCodeListType` - If the string is not a valid CodeListType
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "icd10" => Ok(CodeListType::ICD10),
            "snomed" => Ok(CodeListType::SNOMED),
            "opcs" => Ok(CodeListType::OPCS),
            invalid_string => Err(CodeListError::invalid_code_list_type(invalid_string)),
        }
    }
}

impl ToString for CodeListType {
    /// Convert a CodeListType to a string
    ///
    /// # Returns
    /// * `String` - The string representation of the CodeListType
    fn to_string(&self) -> String {
        match self {
            CodeListType::ICD10 => "ICD10".to_string(),
            CodeListType::SNOMED => "SNOMED".to_string(),
            CodeListType::OPCS => "OPCS".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert!(matches!(CodeListType::from_str("icd10"), Ok(CodeListType::ICD10)));
        assert!(matches!(CodeListType::from_str("snomed"), Ok(CodeListType::SNOMED)));
        assert!(matches!(CodeListType::from_str("opcs"), Ok(CodeListType::OPCS)));
        assert!(matches!(CodeListType::from_str("invalid"), 
            Err(CodeListError::InvalidCodeListType { name }) if name == "invalid"));
    }

    #[test]
    fn test_from_str_case_insensitive() {
        assert!(matches!(CodeListType::from_str("ICD10"), Ok(CodeListType::ICD10)));
        assert!(matches!(CodeListType::from_str("SNOMED"), Ok(CodeListType::SNOMED)));
        assert!(matches!(CodeListType::from_str("OPCS"), Ok(CodeListType::OPCS)));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(CodeListType::ICD10.to_string(), "icd10");
        assert_eq!(CodeListType::SNOMED.to_string(), "snomed");
        assert_eq!(CodeListType::OPCS.to_string(), "opcs");
    }
}

