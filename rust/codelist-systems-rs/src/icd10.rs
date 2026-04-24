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

#[cfg(test)]
mod tests {
    use codelist_rs::types::Code;
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn valid_icd10_codes_pass_syntax() {
        for ok in ["A54", "A37", "A05", "B74.0", "N40", "M10", "Q90", "K02"] {
            let c = Code::from(ok);
            let n = Icd10::normalize(&c).unwrap();
            Icd10::validate_syntax(&n).unwrap_or_else(|_| panic!("{ok} should pass"));
        }
    }

    #[test]
    fn invalid_icd10_codes_fail_syntax() {
        // Previously-invalid codes from the existing ICD10 validator test-suite.
        // Note: "a54" was invalid in the old validator (rejected lowercase), but is
        // now valid because `normalize` uppercases before `validate_syntax` runs.
        // That's the intended design change. Hence it's not in this list.
        for bad in ["A009000000", "1009", "AA09", "A0A9", "A00A", "A00.A", "A00X12", "A00.4AA"] {
            let c = Code::from(bad);
            let n = Icd10::normalize(&c).unwrap();
            assert!(Icd10::validate_syntax(&n).is_err(), "{bad} should fail");
        }
    }

    #[test]
    fn too_long_icd10_codes_report_length_error() {
        let c = Code::from("A009000000");
        let n = Icd10::normalize(&c).unwrap();
        let err = Icd10::validate_syntax(&n).unwrap_err();
        assert!(matches!(err, crate::errors::ValidationError::InvalidLength { .. }));
    }

    #[test]
    fn icd10_normalize_upper_cases_and_trims() {
        let c = Code::from("  a54 ");
        let n = Icd10::normalize(&c).unwrap();
        assert_eq!(n.as_str(), "A54");
        Icd10::validate_syntax(&n).unwrap();
    }

    #[test]
    fn icd10_normalize_rejects_empty_code() {
        let c = Code::from("   ");
        assert!(Icd10::normalize(&c).is_err());
    }

    fn valid_icd10() -> &'static str {
        r"[A-Z][0-9]{2}(X|(\.[0-9]{1,3})?|[0-9]{1,4})?"
    }

    proptest! {
        #[test]
        fn arbitrary_strings_match_regex_iff_validate_ok(s in "[A-Za-z0-9. X]{0,10}") {
            let c = Code::from(s.as_str());
            let Ok(n) = Icd10::normalize(&c) else { return Ok(()); };
            let regex_ok = regex::Regex::new(r"^[A-Z]\d{2}(X|(\.\d{1,3})?|\d{1,4})?$")
                .unwrap()
                .is_match(n.as_str()) && n.as_str().len() <= 7;
            let validate_ok = Icd10::validate_syntax(&n).is_ok();
            prop_assert_eq!(regex_ok, validate_ok);
        }

        #[test]
        fn valid_shape_icd10_validates_ok(s in valid_icd10()) {
            let c = Code::from(s.as_str());
            let n = Icd10::normalize(&c).unwrap();
            prop_assert!(Icd10::validate_syntax(&n).is_ok());
        }

        #[test]
        fn icd10_disallowed_chars_fail_invalid_contents(
            prefix in r"[A-Z][0-9]{2}",
            illegal in r"[!@#$%]",
        ) {
            let s = format!("{prefix}{illegal}");
            let c = Code::from(s.as_str());
            let n = Icd10::normalize(&c).unwrap();
            let err = Icd10::validate_syntax(&n).unwrap_err();
            let is_invalid_contents = matches!(err, ValidationError::InvalidContents { .. });
            prop_assert!(is_invalid_contents);
        }

        #[test]
        fn icd10_out_of_range_length_fails_invalid_length(s in r"[A-Z0-9.X]{8,15}") {
            let c = Code::from(s.as_str());
            let n = Icd10::normalize(&c).unwrap();
            let err = Icd10::validate_syntax(&n).unwrap_err();
            let is_invalid_length = matches!(err, ValidationError::InvalidLength { .. });
            prop_assert!(is_invalid_length);
        }

        #[test]
        fn icd10_trim_idempotent(
            s in valid_icd10(),
            left in 0usize..5,
            right in 0usize..5,
        ) {
            let padded = format!("{}{s}{}", " ".repeat(left), " ".repeat(right));
            let base = Icd10::normalize(&Code::from(s.as_str())).unwrap();
            let pad = Icd10::normalize(&Code::from(padded.as_str())).unwrap();
            prop_assert_eq!(base.as_str(), pad.as_str());
        }
    }

    #[test]
    fn icd10_is_truncatable_when_longer_than_three() {
        let n = Icd10::normalize(&Code::from("A00.4")).unwrap();
        assert!(Icd10::is_truncatable(&n));
        let short = Icd10::normalize(&Code::from("A00")).unwrap();
        assert!(!Icd10::is_truncatable(&short));
    }

    #[test]
    fn icd10_truncate_to_three_chars() {
        let n = Icd10::normalize(&Code::from("A00.4")).unwrap();
        let t = Icd10::truncate(&n);
        assert_eq!(t.as_str(), "A00");
    }

    #[test]
    fn icd10_is_x_addable_for_three_char_codes() {
        let n = Icd10::normalize(&Code::from("A00")).unwrap();
        assert!(Icd10::is_x_addable(&n));
    }

    #[test]
    fn icd10_add_x_appends_x() {
        let n = Icd10::normalize(&Code::from("A00")).unwrap();
        assert_eq!(Icd10::add_x(&n).as_str(), "A00X");
    }
}
