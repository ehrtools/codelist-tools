use crate::codelist::CodeList;

pub trait OPCSValidator {
    fn validate(&self, code: &str) -> bool;
}

// Implement the `OPCSValidator` trait for `CodeList`
impl OPCSValidator for CodeList {
    fn validate(&self, code: &str) -> bool {
        //TODO
        true
    }
}