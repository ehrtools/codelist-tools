use codelist_rs::codelist::CodeList;
use regex::Regex;
use std::sync::LazyLock;
use crate::errors::CodeListValidatorError;

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[A-Z]\d{2}(X|(\.\d{1,3})?|\d{1,4})?$").expect("Unable to create regex")
});

pub trait ICD10Validator {
    fn validate_code(&self, code: &str) -> Result<(), CodeListValidatorError>; // for 1 code
    fn validate_all_code(&self) -> Result<(), CodeListValidatorError>;
}

// Implement the `OPCSValidator` trait for `CodeList`
impl ICD10Validator for CodeList {
    fn validate_code(&self, code: &str) -> Result<(), CodeListValidatorError> {
        if code.len() > 7 {
            return Err(CodeListValidatorError::invalid_code_length(code, "ICD10 code is not greater than 7 in length"))
        }

        let re = &REGEX;

        if !re.is_match(code) {
            return Err(CodeListValidatorError::invalid_code_contents(
                code,
                format!("ICD10 code {} does not match the expected format", code), // Corrected string interpolation
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

// def validate_icd10_code(code: str) -> bool:
// """
//         Validate the form of an ICD-10 code.
//
//         The rules are:
//         - The code must be 7 characters or less
//         - The first character must be a letter
//         - The second and third characters must be numbers
//         - The fourth character must be a dot, or a number or X
//         - If the fourth character is a dot, there must be at least 1 number after the dot
//         - If the fourth character is a X, there are no further characters
//         - The fifth to seventh characters must be numbers if present
//
//         Args:
//             code (str): The code to validate.
//
//         Returns:
//             bool: True if the code is valid, False otherwise.
//         """


//TODO: tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//
// }
//