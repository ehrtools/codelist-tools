use codelist_rs::codelist::CodeList;
use crate::errors::CodeListValidatorError;

const MAX_LENGTH: u32 = 18;
const MIN_LENGTH: u32 = 6;

pub trait SNOMEDValidator {
    fn validate_code(&self, code: &str, min_length: u32, max_length: u32) -> bool; // for 1 code
    fn validate_all_code(&self, min_length: Option<u32>, max_length: Option<u32>);
}

impl SNOMEDValidator for CodeList {
    fn validate_code(&self, code: &str, min_length: u32, max_length: u32) -> Result<bool, CodeListValidatorError> {
        // check code is numerical
        code.trim().parse::<u64>()?;
        let length = code.len() as u32;
        let result = min_length <= length && length <= max_length;
        if !result {
            return Err(CodeListValidatorError::invalid_snomed_code(code, ));
        }
        Ok(result)
    }

    fn validate_all_code(&self, min_length: Option<u32>, max_length: Option<u32>) -> Result<bool, CodeListValidatorError> {
        let min_length = min_length.unwrap_or(MIN_LENGTH);
        let max_length = max_length.unwrap_or(MAX_LENGTH);
        for code_entry in self.entries {
            let code = &code_entry.code;
            let result = self.validate_code(code, min_length, max_length);
            if !result {
                return Err(CodeListValidatorError::invalid_snomed_code(code));
            }
        }
        Ok(true)
    }

    // TODO for vec of codelist entries, call validate code (above) for each entry, return result - codelist error or String ("codelist validated")
}


// @staticmethod
// def validate_snomed_code(code: str, min_length: int = 6, max_length: int = 18) -> bool:
// """
//         Validate the form of a SNOMED CT code.
//
//         Args:
//             code (str): The code to validate.
//             min_length (int, optional): The minimum length of the code. Defaults to 6.
//             max_length (int, optional): The maximum length of the code. Defaults to 18.
//
//         Returns:
//             bool: True if the code is valid, False otherwise.
//         """
// if not code.isdigit():
// return False
// if not min_length <= len(code) <= max_length:
// return False
// return True

//TODO: tests