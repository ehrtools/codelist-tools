use codelist_rs::types::Code;
use codelist_systems_rs::{CodingSystem, ctv3::Ctv3};

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
            matches!(err, codelist_systems_rs::ValidationError::InvalidLength { .. }),
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
            matches!(err, codelist_systems_rs::ValidationError::InvalidLength { .. }),
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
            matches!(err, codelist_systems_rs::ValidationError::InvalidContents { .. }),
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
