//! This file contains custom errors for the codelist library

use std::io;

use csv;
use regex;
use serde_json;

/// Enum to represent the different types of errors that can occur in the
/// codelist library

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

    #[error("Column index out of bounds: {msg}")]
    ColumnIndexOutOfBounds { msg: String },

    #[error("Invalid code type: {msg}")]
    InvalidCodeType { msg: String },

    #[error("Invalid term type: {msg}")]
    InvalidTermType { msg: String },

    #[error("Comment for entry with code {code} already exists. {msg}.")]
    CodeEntryCommentAlreadyExists { code: String, msg: String },

    #[error("Term for entry with code {code} already exists. {msg}.")]
    CodeEntryTermAlreadyExists { code: String, msg: String },

    #[error("Comment for entry with code {code} does not exist. {msg}.")]
    CodeEntryCommentDoesNotExist { code: String, msg: String },

    #[error("Term for entry with code {code} does not exist. {msg}.")]
    CodeEntryTermDoesNotExist { code: String, msg: String },

    #[error("Contributor {contributor} not found")]
    ContributorNotFound { contributor: String },

    #[error("Invalid metadata source: {source_string}")]
    InvalidMetadataSource { source_string: String },

    #[error("Purpose already exists: {msg}")]
    PurposeAlreadyExists { msg: String },

    #[error("Purpose does not exist: {msg}")]
    PurposeDoesNotExist { msg: String },

    #[error("Target audience already exists: {msg}")]
    TargetAudienceAlreadyExists { msg: String },

    #[error("Target audience does not exist: {msg}")]
    TargetAudienceDoesNotExist { msg: String },

    #[error("Use context already exists: {msg}")]
    UseContextAlreadyExists { msg: String },

    #[error("Use context does not exist: {msg}")]
    UseContextDoesNotExist { msg: String },

    #[error("Reviewer already exists: {msg}")]
    ReviewerAlreadyExists { msg: String },

    #[error("Reviewer does not exist: {msg}")]
    ReviewerDoesNotExist { msg: String },

    #[error("Review date already exists: {msg}")]
    ReviewDateAlreadyExists { msg: String },

    #[error("Review date does not exist: {msg}")]
    ReviewDateDoesNotExist { msg: String },

    #[error("Status already exists: {msg}")]
    StatusAlreadyExists { msg: String },

    #[error("Status does not exist: {msg}")]
    StatusDoesNotExist { msg: String },

    #[error("Validation notes already exist: {msg}")]
    ValidationNotesAlreadyExist { msg: String },

    #[error("Validation notes do not exist: {msg}")]
    ValidationNotesDoNotExist { msg: String },

    #[error("Review date is none.")]
    ReviewDateIsNone,

    #[error("Tag does not exist: {msg}")]
    TagDoesNotExist { msg: String },

    #[error("Tag already exists: {msg}")]
    TagAlreadyExists { msg: String },

    #[error("Usage does not exist: {msg}")]
    UsageDoesNotExist { msg: String },

    #[error("License already exists: {msg}")]
    LicenseAlreadyExists { msg: String },

    #[error("License does not exist: {msg}")]
    LicenseDoesNotExist { msg: String },

    #[error("JSON error: {0}")]
    #[construct(skip)]
    JSONError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    #[construct(skip)]
    IOError(#[from] io::Error),

    #[error("CSV error: {0}")]
    #[construct(skip)]
    CSVError(#[from] csv::Error),

    #[error("{codelist_type} cannot be truncated to 3 digits.")]
    CodeListNotTruncatable { codelist_type: String },

    #[error("{term_management} is not known. Valid values are 'first', 'drop_term'")]
    TermManagementNotKnown { term_management: String },

    #[error("{codelist_type} cannot be transformed by having X added to the end of it")]
    CodeListNotXAddable { codelist_type: String },

    #[error("Invalid custom regex pattern: {0}")]
    #[construct(skip)]
    InvalidRegexPattern(#[from] regex::Error),
}
