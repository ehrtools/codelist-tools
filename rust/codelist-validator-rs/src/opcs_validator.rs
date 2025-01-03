use codelist_rs::codelist::CodeList;
use regex::Regex;
use std::sync::LazyLock;
use crate::errors::CodeListValidatorError;

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[A-Z]\d{2}(\.\d{1,2}|\d{1,2})?$").expect("Unable to create regex")
});

pub trait OPCSValidator {
    fn validate_code(&self, code: &str) -> Result<(), CodeListValidatorError>; // for 1 code
    fn validate_all_code(&self) -> Result<(), CodeListValidatorError>;
}

// Implement the `OPCSValidator` trait for `CodeList`
impl OPCSValidator for CodeList {
    fn validate_code(&self, code: &str) -> Result<(), CodeListValidatorError> {
        if code.len() > 5 {
            return Err(CodeListValidatorError::invalid_code_length(code, "OPCS code is not greater than 5 in length"))
        }

        let re = &REGEX;

        if !re.is_match(code) {
            return Err(CodeListValidatorError::invalid_code_contents(
                code,
                format!("OPCS code {} does not match the expected format", code), // Corrected string interpolation
            ));
        }
        Ok(())
    }

    fn validate_all_code(&self) -> Result<(), CodeListValidatorError> {
        for code_entry in self.entries.iter() {
            let code = &code_entry.code;
            self.validate_code(code)?;
        }
        Ok(())
    }
}