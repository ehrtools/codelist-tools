use crate::codelist::CodeList;

pub trait ICD10Validator {
    fn validate(&self, code: &str) -> bool;
}

impl ICD10Validator for CodeList {
    fn validate(&self, code: &str) -> bool {
        //TODO
        true
    }
}