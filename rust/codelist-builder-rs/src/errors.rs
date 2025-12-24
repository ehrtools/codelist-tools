//! This file contains custom errors for the codelist-builder library

/// Enum to represent the different types of errors that can occur in the
/// codelist-builder library
use std::io;

#[derive(Debug, thiserror::Error, thiserror_ext::Construct)]
pub enum CodeListBuilderError {
    #[error("Invalid usage year: {name}")]
    InvalidUsageYear { name: String },

    #[error("Invalid usage data: {name}")]
    InvalidUsageData { name: String },

    #[error("HTTP error code: {code}: {body}")]
    HttpErrorCode { code: String, body: String },

    #[error("URL not found in config for year {year}")]
    UrlNotFound { year: String },

    #[error("HTTP request error: {0}")]
    #[construct(skip)]
    ReqwestError(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    #[construct(skip)]
    IOError(#[from] io::Error),

    #[error("JSON error: {0}")]
    #[construct(skip)]
    JSONError(#[from] serde_json::Error),

    #[error("CSV error: {0}")]
    #[construct(skip)]
    CSVError(#[from] csv::Error),
}
