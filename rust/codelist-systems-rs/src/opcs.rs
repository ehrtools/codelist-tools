//! OPCS.
//!
//! Rules (preserved from the legacy `opcs_validator`):
//! 1. Length 3–5.
//! 2. First character is a letter (upper case after normalisation).
//! 3. Second and third characters are digits.
//! 4. Fourth character, if present, is `.` or a digit.
//! 5. If fourth is `.`, a digit must follow.
//! 6. Fifth character, if present, is a digit.

use std::sync::LazyLock;

use codelist_rs::types::{Code, CodeSystemId, NormalizedCode};
use regex::Regex;

use crate::{
    core::CodingSystem,
    errors::{SystemError, ValidationError},
};

/// OPCS coding system marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Opcs;

static OPCS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[A-Z]\d{2}(\.\d{1,2}|\d{1,2})?$").expect("OPCS regex compiles")
});

impl CodingSystem for Opcs {
    const ID: CodeSystemId = CodeSystemId("OPCS");

    fn normalize(code: &Code) -> Result<NormalizedCode, SystemError> {
        let s = code.as_str().trim().to_ascii_uppercase();
        if s.is_empty() {
            return Err(SystemError::normalisation("OPCS", "empty after trim"));
        }
        Ok(NormalizedCode::from(s))
    }

    fn validate_syntax(code: &NormalizedCode) -> Result<(), ValidationError> {
        let len = code.as_str().len();
        if !(3..=5).contains(&len) {
            return Err(ValidationError::invalid_length(
                code.as_str().to_string(),
                "OPCS".to_string(),
                format!("length must be between 3 and 5 (got {len})"),
            ));
        }
        if !OPCS_REGEX.is_match(code.as_str()) {
            return Err(ValidationError::invalid_contents(
                code.as_str().to_string(),
                "OPCS".to_string(),
                "does not match expected format".to_string(),
            ));
        }
        Ok(())
    }
}
