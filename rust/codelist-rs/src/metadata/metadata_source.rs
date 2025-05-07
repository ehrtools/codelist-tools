//! This file contains the metadata source enum and its implementation

// External imports
use std::fmt;

use serde::{Deserialize, Serialize};

// Internal imports
use crate::errors::CodeListError;

/// Enum to represent the different sources of the codelist
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Source {
    LoadedFromFile,
    MappedFromAnotherCodelist,
    ManuallyCreated,
}

impl Source {
    /// Converts a string to a metadata source
    ///
    /// # Arguments
    /// * `s` - The string to convert to a metadata source
    ///
    /// # Returns
    /// * `Source` - The metadata source
    pub fn from_string(s: &str) -> Result<Source, CodeListError> {
        Ok(match s {
            "Loaded from file" => Source::LoadedFromFile,
            "Mapped from another codelist" => Source::MappedFromAnotherCodelist,
            "Manually created" => Source::ManuallyCreated,
            _ => return Err(CodeListError::invalid_metadata_source(s)),
        })
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Source::LoadedFromFile => "Loaded from file",
            Source::MappedFromAnotherCodelist => "Mapped from another codelist",
            Source::ManuallyCreated => "Manually created",
        };
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_source_to_string() {
        assert_eq!(Source::LoadedFromFile.to_string(), "Loaded from file");
        assert_eq!(Source::MappedFromAnotherCodelist.to_string(), "Mapped from another codelist");
        assert_eq!(Source::ManuallyCreated.to_string(), "Manually created");
    }

    #[test]
    fn test_metadata_source_from_string() -> Result<(), CodeListError> {
        assert_eq!(Source::from_string("Loaded from file")?, Source::LoadedFromFile);
        assert_eq!(
            Source::from_string("Mapped from another codelist")?,
            Source::MappedFromAnotherCodelist
        );
        assert_eq!(Source::from_string("Manually created")?, Source::ManuallyCreated);
        let error = Source::from_string("Metadata source").unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Invalid metadata source: Metadata source");
        Ok(())
    }
}
