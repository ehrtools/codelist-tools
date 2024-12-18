use crate::codelist::CodeList;

pub trait SNOMEDValidator {
    fn validate_code(&self, code: &str) -> bool; // for 1 code
    fn validate_all_code(&self);
        // TODO for vec of codelist entries, call validate code (above) for each entry, return result - codelist error or String ("codelist validated")
}

impl SNOMEDValidator for CodeList {
    fn validate_code(&self, code: &str) -> bool {
        //TODO
        true
    }

    fn validate_all_code(&self) {

    }
}

//TODO: tests