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

#[cfg(test)]
mod tests {
    use codelist_rs::types::Code;
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn valid_snomed_codes_pass_syntax() {
        for ok in ["204351007", "405752007", "77480004", "34000006", "24700007", "398254007"] {
            let c = Code::from(ok);
            let n = Snomed::normalize(&c).unwrap();
            Snomed::validate_syntax(&n).unwrap_or_else(|_| panic!("{ok} should pass"));
        }
    }

    #[test]
    fn too_short_snomed_codes_fail_syntax() {
        for bad in ["11", "11111", "2043"] {
            let c = Code::from(bad);
            let n = Snomed::normalize(&c).unwrap();
            let err = Snomed::validate_syntax(&n).unwrap_err();
            assert!(
                matches!(err, crate::errors::ValidationError::InvalidLength { .. }),
                "{bad} should fail with InvalidLength"
            );
        }
    }

    #[test]
    fn too_long_snomed_codes_fail_syntax() {
        for bad in ["1111111111111111111111111111", "9999999999999999999"] {
            let c = Code::from(bad);
            let n = Snomed::normalize(&c).unwrap();
            let err = Snomed::validate_syntax(&n).unwrap_err();
            assert!(
                matches!(err, crate::errors::ValidationError::InvalidLength { .. }),
                "{bad} should fail with InvalidLength"
            );
        }
    }

    #[test]
    fn non_numeric_snomed_codes_fail_syntax() {
        for bad in ["AA0901", "11A6BB789A", "ABC123DEF"] {
            let c = Code::from(bad);
            let n = Snomed::normalize(&c).unwrap();
            let err = Snomed::validate_syntax(&n).unwrap_err();
            assert!(
                matches!(err, crate::errors::ValidationError::InvalidContents { .. }),
                "{bad} should fail with InvalidContents"
            );
        }
    }

    #[test]
    fn snomed_normalize_trims_whitespace() {
        let c = Code::from("  204351007 ");
        let n = Snomed::normalize(&c).unwrap();
        assert_eq!(n.as_str(), "204351007");
        Snomed::validate_syntax(&n).unwrap();
    }

    #[test]
    fn snomed_normalize_rejects_empty_code() {
        let c = Code::from("   ");
        assert!(Snomed::normalize(&c).is_err());
    }

    proptest! {
        #[test]
        fn valid_shape_snomed_validates_ok(s in r"[0-9]{6,18}") {
            let c = Code::from(s.as_str());
            let n = Snomed::normalize(&c).unwrap();
            prop_assert!(Snomed::validate_syntax(&n).is_ok());
        }

        #[test]
        fn snomed_disallowed_chars_fail_invalid_contents(
            a in r"[0-9]*",
            b in r"[!@#$%a-zA-Z]+",
            c in r"[0-9]*",
        ) {
            let s = format!("{a}{b}{c}");
            let code = Code::from(s.as_str());
            let n = Snomed::normalize(&code).unwrap();
            let err = Snomed::validate_syntax(&n).unwrap_err();
            let is_invalid_contents = matches!(err, ValidationError::InvalidContents { .. });
            prop_assert!(is_invalid_contents);
        }

        #[test]
        fn snomed_out_of_range_length_digits_fail_invalid_length(
            s in prop_oneof![r"[0-9]{1,5}", r"[0-9]{19,25}"],
        ) {
            let code = Code::from(s.as_str());
            let n = Snomed::normalize(&code).unwrap();
            let err = Snomed::validate_syntax(&n).unwrap_err();
            let is_invalid_length = matches!(err, ValidationError::InvalidLength { .. });
            prop_assert!(is_invalid_length);
        }

        #[test]
        fn snomed_trim_idempotent(
            s in r"[0-9]{6,18}",
            left in 0usize..5,
            right in 0usize..5,
        ) {
            let padded = format!("{}{s}{}", " ".repeat(left), " ".repeat(right));
            let base = Snomed::normalize(&Code::from(s.as_str())).unwrap();
            let pad = Snomed::normalize(&Code::from(padded.as_str())).unwrap();
            prop_assert_eq!(base.as_str(), pad.as_str());
        }
    }
}
