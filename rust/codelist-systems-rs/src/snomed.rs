//! SNOMED CT coding system.
//!
//! Rules (preserved from the legacy `snomed_validator`):
//! 1. The code consists of digits only.
//! 2. The code must be between 6 and 18 digits in length (inclusive).

use codelist_rs::types::{Code, CodeSystemId, NormalizedCode};

use crate::{
    core::CodingSystem,
    errors::{SystemError, ValidationError},
};

/// SNOMED CT coding system marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Snomed;

impl CodingSystem for Snomed {
    const ID: CodeSystemId = CodeSystemId("SNOMED");

    fn normalize(code: &Code) -> Result<NormalizedCode, SystemError> {
        let s = code.as_str().trim().to_string();
        if s.is_empty() {
            return Err(SystemError::normalisation("SNOMED", "empty after trim"));
        }
        Ok(NormalizedCode::from(s))
    }

    fn validate_syntax(code: &NormalizedCode) -> Result<(), ValidationError> {
        let s = code.as_str();
        if !s.chars().all(|c| c.is_ascii_digit()) {
            return Err(ValidationError::invalid_contents(
                s.to_string(),
                "SNOMED".to_string(),
                "code must consist of digits only".to_string(),
            ));
        }
        let len = s.len();
        if !(6..=18).contains(&len) {
            return Err(ValidationError::invalid_length(
                s.to_string(),
                "SNOMED".to_string(),
                "length must be between 6 and 18 digits".to_string(),
            ));
        }
        Ok(())
    }
}
