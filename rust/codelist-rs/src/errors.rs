use std::fmt;
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

#[derive(Debug)]
pub enum CodeListError {
    InvalidCodeListType(String),
    JSONError(serde_json::Error),
    IOError(io::Error),
    EntryNotFound(String),
    CSVError(csv::Error),
    InvalidFilePath,
    InvalidInput(String),

    InvalidCodeField(String),
    InvalidTermField(String),
    EmptyCode(String),
    EmptyTerm(String),
    InvalidCode(String),
    InvalidTerm(String),
    ColumnIndexOutOfBounds(String),
}

impl From<io::Error> for CodeListError {
    fn from(err: io::Error) -> Self {
        CodeListError::IOError(err)
    }
}

impl From<serde_json::Error> for CodeListError {
    fn from(err: serde_json::Error) -> Self {
        CodeListError::JSONError(err)
    }
}

impl From<csv::Error> for CodeListError {
    fn from(err: csv::Error) -> Self {
        CodeListError::CSVError(err)
    }
}

impl fmt::Display for CodeListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCodeListType(invalid_type) => write!(f, "Invalid codelist type provided: {}", invalid_type),
            Self::JSONError(err) => write!(f, "JSON error: {}", err),
            Self::IOError(err) => write!(f, "IO error: {}", err),
            Self::EntryNotFound(code) => write!(f, "Entry not found: {}", code),
            Self::CSVError(err) => write!(f, "CSV error: {}", err),
            Self::EmptyCode(msg) => write!(f, "Empty code: {}", msg),
            Self::EmptyTerm(msg) => write!(f, "Empty term: {}", msg),
            Self::InvalidFilePath => write!(f, "Invalid file path"),
            Self::InvalidInput(err) => write!(f, "Invalid input: {}", err),
            Self::InvalidCodeField(err) => write!(f, "Invalid column field: {}", err),
            Self::InvalidTermField(err) => write!(f, "Invalid term field: {}", err),
            Self::InvalidCode(err) => write!(f, "Invalid code: {}", err),
            Self::InvalidTerm(err) => write!(f, "Invalid term: {}", err),
            Self::ColumnIndexOutOfBounds(err) => write!(f, "Column index out of bounds: {}", err),
        }
    }
}