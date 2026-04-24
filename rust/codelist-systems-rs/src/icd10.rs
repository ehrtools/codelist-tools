//! ICD10.
//!
//! Rules (preserved from the legacy `icd10_validator`):
//! 1. Length ≤ 7.
//! 2. First character is a letter (upper case after normalisation).
//! 3. Second and third characters are digits.
//! 4. Fourth character is `.`, a digit, or `X`.
//! 5. If fourth is `.`, at least one digit follows.
//! 6. If fourth is `X`, no further characters.
//! 7. Fifth–seventh characters are digits if present.

use std::sync::LazyLock;

use codelist_rs::types::{Code, CodeSystemId, NormalizedCode};
use regex::Regex;

use crate::{
    capabilities::{Truncatable, XExtensible},
    core::CodingSystem,
    errors::{SystemError, ValidationError},
};

/// ICD10 coding system marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Icd10;

static ICD10_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[A-Z]\d{2}(X|(\.\d{1,3})?|\d{1,4})?$").expect("ICD10 regex compiles")
});

impl CodingSystem for Icd10 {
    const ID: CodeSystemId = CodeSystemId("ICD10");

    fn normalize(code: &Code) -> Result<NormalizedCode, SystemError> {
        let s = code.as_str().trim().to_ascii_uppercase();
        if s.is_empty() {
            return Err(SystemError::normalisation("ICD10", "empty after trim"));
        }
        Ok(NormalizedCode::from(s))
    }

    fn validate_syntax(code: &NormalizedCode) -> Result<(), ValidationError> {
        if code.as_str().len() > 7 {
            return Err(ValidationError::invalid_length(
                code.as_str().to_string(),
                "ICD10".to_string(),
                "length > 7".to_string(),
            ));
        }
        if !ICD10_REGEX.is_match(code.as_str()) {
            return Err(ValidationError::invalid_contents(
                code.as_str().to_string(),
                "ICD10".to_string(),
                "does not match expected format".to_string(),
            ));
        }
        Ok(())
    }
}

impl Truncatable for Icd10 {
    fn is_truncatable(code: &NormalizedCode) -> bool {
        code.as_str().len() > 3
    }

    fn truncate(code: &NormalizedCode) -> NormalizedCode {
        let s: String = code.as_str().chars().take(3).collect();
        NormalizedCode::from(s)
    }
}

impl XExtensible for Icd10 {
    fn is_x_addable(code: &NormalizedCode) -> bool {
        code.as_str().len() == 3
    }

    fn add_x(code: &NormalizedCode) -> NormalizedCode {
        NormalizedCode::from(format!("{}X", code.as_str()))
    }
}
