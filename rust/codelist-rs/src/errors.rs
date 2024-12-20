use std::fmt;
use std::io;
use serde_json;
use csv;

#[derive(Debug)]
pub enum CodeListError {
    InvalidCodeListType(String),
    JSONSerializationError(serde_json::Error),
    IOError(io::Error),
    EntryNotFound(String),
    CSVSerializationError(csv::Error),
}

impl From<io::Error> for CodeListError {
    fn from(err: io::Error) -> Self {
        CodeListError::IOError(err)
    }
}

impl From<serde_json::Error> for CodeListError {
    fn from(err: serde_json::Error) -> Self {
        CodeListError::JSONSerializationError(err)
    }
}

impl From<csv::Error> for CodeListError {
    fn from(err: csv::Error) -> Self {
        CodeListError::CSVSerializationError(err)
    }
}

impl fmt::Display for CodeListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCodeListType(invalid_type) => write!(f, "Invalid codelist type provided: {}", invalid_type),
            Self::JSONSerializationError(err) => write!(f, "Serialization error: {}", err),
            Self::IOError(err) => write!(f, "IO error: {}", err),
            Self::EntryNotFound(code) => write!(f, "Entry not found: {}", code),
            Self::CSVSerializationError(err) => write!(f, "CSV serialization error: {}", err),
        }
    }
}
