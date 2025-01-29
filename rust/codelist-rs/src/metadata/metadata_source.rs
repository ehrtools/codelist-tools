//! This file contains the metadata source enum and its implementation

// External imports
use serde::{Deserialize, Serialize};

// Internal imports
use crate::errors::CodeListError;

/// Enum to represent the different sources of the codelist
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MetadataSource {
    LoadedFromFile,
    MappedFromAnotherCodelist,
    ManuallyCreated,
}

impl MetadataSource {
    /// Converts the metadata source to a string
    /// 
    /// # Arguments
    /// * `self` - The metadata source to convert to a string
    /// 
    /// # Returns
    /// * `String` - The string representation of the metadata source
    pub fn to_string(&self) -> String {
        match self {
            MetadataSource::LoadedFromFile => "Loaded from file".to_string(),
            MetadataSource::MappedFromAnotherCodelist => "Mapped from another codelist".to_string(),
            MetadataSource::ManuallyCreated => "Manually created".to_string(),
        }
    }

    /// Converts a string to a metadata source
    /// 
    /// # Arguments
    /// * `s` - The string to convert to a metadata source
    /// 
    /// # Returns
    /// * `MetadataSource` - The metadata source
    pub fn from_string(s: &str) -> Result<MetadataSource, CodeListError> {
        Ok(match s {
            "Loaded from file" => MetadataSource::LoadedFromFile,
            "Mapped from another codelist" => MetadataSource::MappedFromAnotherCodelist,
            "Manually created" => MetadataSource::ManuallyCreated,
            _ => return Err(CodeListError::invalid_metadata_source(s)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_source_to_string() {
        assert_eq!(MetadataSource::LoadedFromFile.to_string(), "Loaded from file");
        assert_eq!(MetadataSource::MappedFromAnotherCodelist.to_string(), "Mapped from another codelist");
        assert_eq!(MetadataSource::ManuallyCreated.to_string(), "Manually created");
    }

    #[test]
    fn test_metadata_source_from_string() -> Result<(), CodeListError> {
        assert_eq!(MetadataSource::from_string("Loaded from file")?, MetadataSource::LoadedFromFile);
        assert_eq!(MetadataSource::from_string("Mapped from another codelist")?, MetadataSource::MappedFromAnotherCodelist);
        assert_eq!(MetadataSource::from_string("Manually created")?, MetadataSource::ManuallyCreated);
        let error = MetadataSource::from_string("Metadata source").unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Invalid metadata source: Metadata source");
        Ok(())
    }
}
