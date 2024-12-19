/// This module defines the different types of codelists that can be used

/// External imports
use std::str::FromStr;
use serde::{Serialize, Deserialize};

/// Internal imports
use crate::errors::CodeListValidatorError;


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
    type Err = CodeListValidatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "icd10" => Ok(CodeListType::ICD10),
            "snomed" => Ok(CodeListType::SNOMED),
            "opcs" => Ok(CodeListType::OPCS),
            invalid_code => Err(CodeListValidatorError::InvalidCodeListType(invalid_code.to_string())),
        }
    }

    // TODO fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

// TODO: tests to ensure correct string conversion
