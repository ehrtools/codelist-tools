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

#[cfg(test)]
mod tests {
    use super::*;
    use codelist_rs::types::Code;

    #[test]
    fn valid_opcs_codes_pass_syntax() {
        for ok in ["C01", "L31.4", "L35.3", "L47.4", "A01", "Z94.2", "K40.1", "B201"] {
            let c = Code::from(ok);
            let n = Opcs::normalize(&c).unwrap();
            Opcs::validate_syntax(&n).unwrap_or_else(|_| panic!("{ok} should pass"));
        }
    }

    #[test]
    fn too_short_opcs_codes_fail_with_invalid_length() {
        for bad in ["A0", "A", "B"] {
            let c = Code::from(bad);
            let n = Opcs::normalize(&c).unwrap();
            let err = Opcs::validate_syntax(&n).unwrap_err();
            assert!(
                matches!(err, crate::errors::ValidationError::InvalidLength { .. }),
                "{bad} should fail with InvalidLength"
            );
        }
    }

    #[test]
    fn too_long_opcs_codes_fail_with_invalid_length() {
        for bad in ["A01000", "B123456"] {
            let c = Code::from(bad);
            let n = Opcs::normalize(&c).unwrap();
            let err = Opcs::validate_syntax(&n).unwrap_err();
            assert!(
                matches!(err, crate::errors::ValidationError::InvalidLength { .. }),
                "{bad} should fail with InvalidLength"
            );
        }
    }

    #[test]
    fn bad_content_opcs_codes_fail_with_invalid_contents() {
        for bad in ["101", "AA1", "A0A", "A01.", "A01.A", "A010A"] {
            let c = Code::from(bad);
            let n = Opcs::normalize(&c).unwrap();
            let err = Opcs::validate_syntax(&n).unwrap_err();
            assert!(
                matches!(err, crate::errors::ValidationError::InvalidContents { .. }),
                "{bad} should fail with InvalidContents"
            );
        }
    }

    #[test]
    fn opcs_normalize_trims_whitespace_and_uppercases() {
        // Intentional behaviour change over the legacy validator: lowercase input
        // is accepted and normalised to uppercase, so " a01 " becomes "A01".
        let c = Code::from(" a01 ");
        let n = Opcs::normalize(&c).unwrap();
        assert_eq!(n.as_str(), "A01");
        Opcs::validate_syntax(&n).unwrap();
    }

    #[test]
    fn opcs_normalize_rejects_empty_after_trim() {
        let c = Code::from("   ");
        assert!(Opcs::normalize(&c).is_err());
    }
}
