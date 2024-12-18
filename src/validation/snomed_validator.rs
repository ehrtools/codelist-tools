use crate::codelist::CodeList;

pub trait SNOMEDValidator {
    fn validate(&self, code: &str) -> bool;
}

impl SNOMEDValidator for CodeList {
    fn validate(&self, code: &str) -> bool {
        //TODO
        true
    }
}