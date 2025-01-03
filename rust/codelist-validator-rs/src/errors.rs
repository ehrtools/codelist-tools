use core::num;

/// Enum to represent the different types of errors that can occur in the codelist-validator library
///

#[derive(Debug, thiserror::Error, thiserror_ext::Construct, Clone)]
pub enum CodeListValidatorError {
    #[error("Code {code} is an invalid length. Reason: {reason}")]
    InvalidCodeLength { code: String, reason: String },

    #[error("Code is not numerical: {0}")]
    #[construct(skip)]
    ParseIntError(#[from] num::ParseIntError),

    #[error("Error initialising REGEX: {0}")]
    #[construct(skip)]
    RegexInitError(#[from] regex::Error),

    #[error("Code {code} contents is invalid. Reason: {reason}")]
    InvalidCodeContents { code: String, reason: String },
}
