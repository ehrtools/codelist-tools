use codelist_rs::codelist::CodeList;
use crate::errors::CodeListValidatorError;

const MAX_LENGTH: u32 = 18;
const MIN_LENGTH: u32 = 6;

pub trait SNOMEDValidator {
    fn validate_code(&self, code: &str, min_length: u32, max_length: u32) -> Result<(), CodeListValidatorError>; // for 1 code
    fn validate_all_code(&self, min_length: Option<u32>, max_length: Option<u32>) -> Result<(), CodeListValidatorError>;
}

impl SNOMEDValidator for CodeList {
    fn validate_code(&self, code: &str, min_length: u32, max_length: u32) -> Result<(), CodeListValidatorError> {
        // check code is numerical, positive
        code.trim().parse::<u64>()?;
        let length = code.len() as u32;
        let result = min_length <= length && length <= max_length;
        if !result {
            return Err(CodeListValidatorError::invalid_code_length(
                code,
                format!("SNOMED code is not between {} and {} in length", min_length, max_length),
            ));
        }
        Ok(())
    }

    fn validate_all_code(&self, min_length: Option<u32>, max_length: Option<u32>) -> Result<(), CodeListValidatorError> {
        let min_length = min_length.unwrap_or(MIN_LENGTH);
        let max_length = max_length.unwrap_or(MAX_LENGTH);
        for code_entry in self.entries.iter() {
            let code = &code_entry.code;
            self.validate_code(code, min_length, max_length)?;
        }
        Ok(())
    }
}

//TODO: tests