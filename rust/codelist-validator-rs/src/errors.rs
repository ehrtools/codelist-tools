/// Enum to represent the different types of errors that can occur in the
/// codelist-validator library
///
/// * `InvalidCodeLength` - An error that occurs when the code is not the
///   correct length
/// * `ParseIntError` - An error that occurs when the code is not composed of
///   numerical characters
/// * `InvalidCodeContents` - An error that occurs when the code does not match
///   the expected format
/// * `InvalidCodelist` - An error that occurs when the codelist is invalid
/// * `UnsupportedCodeType` - An error that occurs when the code type is not
///   supported
#[derive(Debug, thiserror::Error, thiserror_ext::Construct, Clone)]
pub enum CodeListValidatorError {
    #[error("Code {code} is an invalid length for type {codelist_type}. Reason: {reason}")]
    InvalidCodeLength { code: String, reason: String, codelist_type: String },

    #[error("Code {code} is not composed of all numerical characters for type {codelist_type}. Reason: {reason}")]
    #[construct(skip)]
    ParseIntError { code: String, reason: String, codelist_type: String },

    #[error("Code {code} contents is invalid for type {codelist_type}. Reason: {reason}")]
    InvalidCodeContents { code: String, reason: String, codelist_type: String },

    #[error("Some codes in the list are invalid. Details: {}", reasons.join(", "))]
    InvalidCodelist { reasons: Vec<String> },

    #[error("CodeType {code_type} is not supported")]
    UnsupportedCodeType { code_type: String },

    #[error("Custom validation failed. Reason: {reason}")]
    CustomValidationFailed { reason: String },

    #[error("Invalid custom regex pattern: {0}")]
    #[construct(skip)]
    InvalidRegexPattern(#[from] regex::Error),
}
