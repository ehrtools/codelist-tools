//! This file contains custom errors for the codelist-builder library

/// Enum to represent the different types of errors that can occur in the
/// codelist-builder library

#[derive(Debug, thiserror::Error, thiserror_ext::Construct)]
pub enum CodeListBuilderError {
    #[error("Invalid usage year: {name}")]
    InvalidUsageYear { name: String },

    #[error("Invalid usage data: {name}")]
    InvalidUsageData { name: String },

    #[error("HTTP request error: {0}")]
    #[construct(skip)]
    ReqwestError(#[from] reqwest::Error),

    #[error("CSV error: {0}")]
    #[construct(skip)]
    CSVError(#[from] csv::Error),
}
