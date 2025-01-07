//! This file contains custom errors for the codelist library

use std::io;
use serde_json;
use csv;

/// Enum to represent the different types of errors that can occur in the codelist library
/// 
/// * `InvalidCodeListType` - An error that occurs when an invalid code list type is provided
/// * `JSONError` - An error that occurs when there is an error serializing or deserializing JSON
/// * `IOError` - An error that occurs when there is an error reading or writing to a file
/// * `EntryNotFound` - An error that occurs when an entry is not found in the codelist
/// * `CSVError` - An error that occurs when there is an error serializing or deserializing CSV

#[derive(Debug, thiserror::Error, thiserror_ext::Construct)]
pub enum CodeListError {
    #[error("Invalid codelist type: {name}")]
    InvalidCodeListType { name: String },
    
    #[error("Entry not found: {code}")]
    EntryNotFound { code: String },
    
    #[error("Invalid file path: {msg}")]
    InvalidFilePath { msg: String },
    
    #[error("Invalid input: {msg}")]
    InvalidInput { msg: String },
    
    #[error("Invalid code field: {msg}")]
    InvalidCodeField { msg: String },
    
    #[error("Invalid term field: {msg}")]
    InvalidTermField { msg: String },
    
    #[error("Empty code: {msg}")]
    EmptyCode { msg: String },
    
    #[error("Empty term: {msg}")]
    EmptyTerm { msg: String },
    
    #[error("Column index out of bounds: {msg}")]
    ColumnIndexOutOfBounds { msg: String },
    
    #[error("Invalid code type: {msg}")]
    InvalidCodeType { msg: String },
    
    #[error("Invalid term type: {msg}")]
    InvalidTermType { msg: String },

    #[error("JSON error: {0}")]
    #[construct(skip)]
    JSONError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    #[construct(skip)]
    IOError(#[from] io::Error),
    
    #[error("CSV error: {0}")]
    #[construct(skip)]
    CSVError(#[from] csv::Error),
}