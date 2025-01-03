use core::num;

/// Enum to represent the different types of errors that can occur in the codelist-validator library
///

#[derive(Debug, thiserror::Error, thiserror_ext::Construct)]
pub enum CodeListValidatorError {
    #[error("Invalid SNOMED code: {code}")]
    InvalidSnomedCode { code: String, reason: String },

    #[error("Code is not numerical: {0}")]
    #[construct(skip)]
    ParseIntError(#[from] num::ParseIntError),

    // #[error("Entry not found: {code}")]
    // EntryNotFound { code: String },
    //
    // #[error("Invalid file path: {msg}")]
    // InvalidFilePath { msg: String },
    //
    // #[error("Invalid input: {msg}")]
    // InvalidInput { msg: String },
    //
    // #[error("Invalid code field: {msg}")]
    // InvalidCodeField { msg: String },
    //
    // #[error("Invalid term field: {msg}")]
    // InvalidTermField { msg: String },
    //
    // #[error("Empty code: {msg}")]
    // EmptyCode { msg: String },
    //
    // #[error("Empty term: {msg}")]
    // EmptyTerm { msg: String },
    //
    // #[error("Column index out of bounds: {msg}")]
    // ColumnIndexOutOfBounds { msg: String },
    //
    // #[error("Invalid code type: {msg}")]
    // InvalidCodeType { msg: String },
    //
    // #[error("Invalid term type: {msg}")]
    // InvalidTermType { msg: String },
    //
    // #[error("JSON error: {0}")]
    // #[construct(skip)]
    // JSONError(#[from] serde_json::Error),
    //
    // #[error("IO error: {0}")]
    // #[construct(skip)]
    // IOError(#[from] io::Error),
    //
    // #[error("CSV error: {0}")]
    // #[construct(skip)]
    // CSVError(#[from] csv::Error),
}
