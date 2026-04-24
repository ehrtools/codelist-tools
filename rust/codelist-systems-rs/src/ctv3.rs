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
    use codelist_rs::types::Code;
    use proptest::prelude::*;

    use super::*;

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

    fn valid_ctv3() -> impl Strategy<Value = String> {
        (0u32..=5).prop_flat_map(|n| {
            proptest::string::string_regex(&format!("[a-zA-Z0-9]{{{n}}}"))
                .unwrap()
                .prop_map(move |s| format!("{s}{}", ".".repeat((5 - n) as usize)))
        })
    }

    proptest! {
        #[test]
        fn valid_shape_ctv3_validates_ok(s in valid_ctv3()) {
            let c = Code::from(s.as_str());
            let n = Ctv3::normalize(&c).unwrap();
            prop_assert!(Ctv3::validate_syntax(&n).is_ok());
        }

        #[test]
        fn ctv3_disallowed_chars_fail_invalid_contents(
            illegal in r"[!@#$%]",
            suffix in r"[a-zA-Z0-9.]{4}",
        ) {
            let s = format!("{illegal}{suffix}");
            let c = Code::from(s.as_str());
            let n = Ctv3::normalize(&c).unwrap();
            let err = Ctv3::validate_syntax(&n).unwrap_err();
            let is_invalid_contents = matches!(err, ValidationError::InvalidContents { .. });
            prop_assert!(is_invalid_contents);
        }

        #[test]
        fn ctv3_out_of_range_length_fails_invalid_length(
            s in prop_oneof![r"[a-zA-Z0-9.]{1,4}", r"[a-zA-Z0-9.]{6,12}"],
        ) {
            let c = Code::from(s.as_str());
            let n = Ctv3::normalize(&c).unwrap();
            let err = Ctv3::validate_syntax(&n).unwrap_err();
            let is_invalid_length = matches!(err, ValidationError::InvalidLength { .. });
            prop_assert!(is_invalid_length);
        }

        #[test]
        fn ctv3_trim_idempotent(
            s in valid_ctv3(),
            left in 0usize..5,
            right in 0usize..5,
        ) {
            let padded = format!("{}{s}{}", " ".repeat(left), " ".repeat(right));
            let base = Ctv3::normalize(&Code::from(s.as_str())).unwrap();
            let pad = Ctv3::normalize(&Code::from(padded.as_str())).unwrap();
            prop_assert_eq!(base.as_str(), pad.as_str());
        }
    }
}
