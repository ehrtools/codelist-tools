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

#[cfg(test)]
mod tests {
    use super::*;
    use codelist_rs::types::Code;

    #[test]
    fn valid_ctv3_codes_pass_syntax() {
        for ok in ["Af918", "ABb..", "alkif", "F....", "bn89.", "Me...", "99999", "....."] {
            let c = Code::from(ok);
            let n = Ctv3::normalize(&c).unwrap();
            Ctv3::validate_syntax(&n).unwrap_or_else(|_| panic!("{ok} should pass"));
        }
    }

    #[test]
    fn too_short_ctv3_codes_fail_with_invalid_length() {
        for bad in ["Af.", "A00A", "10"] {
            let c = Code::from(bad);
            let n = Ctv3::normalize(&c).unwrap();
            let err = Ctv3::validate_syntax(&n).unwrap_err();
            assert!(
                matches!(err, crate::errors::ValidationError::InvalidLength { .. }),
                "{bad} should fail with InvalidLength"
            );
        }
    }

    #[test]
    fn too_long_ctv3_codes_fail_with_invalid_length() {
        for bad in ["A009000000", "9874ji", "Q90....."] {
            let c = Code::from(bad);
            let n = Ctv3::normalize(&c).unwrap();
            let err = Ctv3::validate_syntax(&n).unwrap_err();
            assert!(
                matches!(err, crate::errors::ValidationError::InvalidLength { .. }),
                "{bad} should fail with InvalidLength"
            );
        }
    }

    #[test]
    fn bad_content_ctv3_codes_fail_with_invalid_contents() {
        for bad in [".a009", "10a.f", "Af!!!", "A..9k", "..9jJ", "A00.l"] {
            let c = Code::from(bad);
            let n = Ctv3::normalize(&c).unwrap();
            let err = Ctv3::validate_syntax(&n).unwrap_err();
            assert!(
                matches!(err, crate::errors::ValidationError::InvalidContents { .. }),
                "{bad} should fail with InvalidContents"
            );
        }
    }

    #[test]
    fn ctv3_normalize_preserves_case() {
        // CTV3 is case-sensitive: normalize must not fold to uppercase.
        // "Af918" must remain "Af918", not become "AF918" as ICD10/OPCS would.
        let c = Code::from("Af918");
        let n = Ctv3::normalize(&c).unwrap();
        assert_eq!(n.as_str(), "Af918");
    }

    #[test]
    fn ctv3_normalize_trims_whitespace() {
        let c = Code::from(" Af918 ");
        let n = Ctv3::normalize(&c).unwrap();
        assert_eq!(n.as_str(), "Af918");
        Ctv3::validate_syntax(&n).unwrap();
    }

    #[test]
    fn ctv3_normalize_rejects_empty_after_trim() {
        let c = Code::from("   ");
        assert!(Ctv3::normalize(&c).is_err());
    }
}
