use codelist_rs::types::Code;
use codelist_systems_rs::{CodingSystem, snomed::Snomed};

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
            matches!(err, codelist_systems_rs::ValidationError::InvalidLength { .. }),
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
            matches!(err, codelist_systems_rs::ValidationError::InvalidLength { .. }),
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
            matches!(err, codelist_systems_rs::ValidationError::InvalidContents { .. }),
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
