use core::num;

/// Enum to represent the different types of errors that can occur in the codelist-validator library
///
/// * `InvalidCodeLength` - An error that occurs when the code is not the correct length
/// * `ParseIntError` - An error that occurs when the code is not composed of numerical characters
/// * `InvalidCodeContents` - An error that occurs when the code does not match the expected format

#[derive(Debug, thiserror::Error, thiserror_ext::Construct, Clone)]
pub enum CodeListValidatorError {
    #[error("Code {code} is an invalid length. Reason: {reason}")]
    InvalidCodeLength { code: String, reason: String },

    #[error("Code is not composed of numerical characters: {0}")]
    #[construct(skip)]
    ParseIntError(#[from] num::ParseIntError),

    #[error("Code {code} contents is invalid. Reason: {reason}")]
    InvalidCodeContents { code: String, reason: String },

    #[error("Some codes in the list are invalid. Details: {reasons:?}")]
    InvalidCodelist { reasons: Vec<(String, String)> },
}
