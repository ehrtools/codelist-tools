//! Errors produced by the CodingSystem trait and its implementors.

use thiserror::Error;
use thiserror_ext::Construct;

#[derive(Debug, Error, Construct, Clone)]
pub enum SystemError {
    #[error("{system}: {msg}")]
    Normalisation { system: String, msg: String },
}

#[derive(Debug, Error, Construct, Clone)]
pub enum ValidationError {
    #[error("Code {code} has invalid length for {system}: {reason}")]
    InvalidLength { code: String, system: String, reason: String },

    #[error("Code {code} has invalid contents for {system}: {reason}")]
    InvalidContents { code: String, system: String, reason: String },
}
