use codelist_rs::types::Code;
use codelist_systems_rs::{CodingSystem, opcs::Opcs};

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
            matches!(err, codelist_systems_rs::ValidationError::InvalidLength { .. }),
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
            matches!(err, codelist_systems_rs::ValidationError::InvalidLength { .. }),
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
            matches!(err, codelist_systems_rs::ValidationError::InvalidContents { .. }),
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
