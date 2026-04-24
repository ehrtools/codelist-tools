//! CTV3.
//!
//! Rules (preserved from the legacy `ctv3_validator`):
//! 1. Exactly 5 characters.
//! 2. Allowed characters: `a-z`, `A-Z`, `0-9`, and `.`.
//! 3. N alphanumeric characters followed by `5 - N` trailing dots (N = 0..=5).
//!
//! CTV3 is case-sensitive, so `normalize` trims whitespace only —
//! it does not fold case the way ICD10 and OPCS do.

use std::sync::LazyLock;

use codelist_rs::types::{Code, CodeSystemId, NormalizedCode};
use regex::Regex;

use crate::{
    core::CodingSystem,
    errors::{SystemError, ValidationError},
};

/// CTV3 coding system marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ctv3;

static CTV3_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?:[a-zA-Z0-9]{5}|[a-zA-Z0-9]{4}\.|[a-zA-Z0-9]{3}\.{2}|[a-zA-Z0-9]{2}\.{3}|[a-zA-Z0-9]\.{4}|\.{5})$",
    )
    .expect("CTV3 regex compiles")
});

impl CodingSystem for Ctv3 {
    const ID: CodeSystemId = CodeSystemId("CTV3");

    fn normalize(code: &Code) -> Result<NormalizedCode, SystemError> {
        // CTV3 is case-sensitive — no case folding, trim only.
        let s = code.as_str().trim().to_string();
        if s.is_empty() {
            return Err(SystemError::normalisation("CTV3", "empty after trim"));
        }
        Ok(NormalizedCode::from(s))
    }

    fn validate_syntax(code: &NormalizedCode) -> Result<(), ValidationError> {
        let len = code.as_str().len();
        if len != 5 {
            return Err(ValidationError::invalid_length(
                code.as_str().to_string(),
                "CTV3".to_string(),
                format!("length must be exactly 5 (got {len})"),
            ));
        }
        if !CTV3_REGEX.is_match(code.as_str()) {
            return Err(ValidationError::invalid_contents(
                code.as_str().to_string(),
                "CTV3".to_string(),
                "does not match expected format".to_string(),
            ));
        }
        Ok(())
    }
}
