use std::str::FromStr;
use crate::errors::CodeListValidatorError;

pub enum CodeListType {
    ICD10,
    SNOMED,
    OPCS,
}

impl FromStr for CodeListType {
    type Err = CodeListValidatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "icd10" => Ok(CodeListType::ICD10),
            "snomed" => Ok(CodeListType::SNOMED),
            "opcs" => Ok(CodeListType::OPCS),
            invalid_code => Err(CodeListValidatorError::InvalidCodeListType(invalid_code.to_string())),
        }
    }

    // TODO fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

// TODO: tests to ensure correct string conversion
