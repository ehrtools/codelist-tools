use crate::codelist::CodeList;

pub trait ICD10Validator {
    fn validate_code(&self, code: &str) -> bool; // for 1 code
    fn validate_all_code(&self);
}

impl ICD10Validator for CodeList {
    fn validate_code(&self, code: &str) -> bool {
        //TODO
        true
    }

    fn validate_all_code(&self) {

    }


}

//TODO: tests
// cargo test --test icd10_validator