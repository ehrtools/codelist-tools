//! The file contains the metadata for the codelist

// External imports
use serde::{Deserialize, Serialize};
use chrono::Utc;


/// Metadata Source Enum
///
/// This enum represents the different sources of the codelist
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MetadataSource {
    LoadedFromFile,
    MappedFromAnotherCodelist,
    ManuallyCreated,
}

/// Metadata Source
impl MetadataSource {
    /// Convert the metadata source to a string
    pub fn to_string(&self) -> String {
        match self {
            MetadataSource::LoadedFromFile => "Loaded from file".to_string(),
            MetadataSource::MappedFromAnotherCodelist => "Mapped from another codelist".to_string(),
            MetadataSource::ManuallyCreated => "Manually created".to_string(),
        }
    }
}

pub struct Provenance {
    pub source: MetadataSource,          
    pub created_date: chrono::DateTime<Utc>,
    pub last_modified_date: chrono::DateTime<Utc>,
    pub contributors: Option<Vec<String>>, 
    pub license: Option<String>
}

pub struct PurposeAndContext {
    pub purpose: Option<String>,
    pub target_audience: String,
    pub use_context: String,
}

pub struct ValidationAndReview {
    pub reviewed: Option<bool>,
    pub reviewer: Option<String>,
    pub review_date: Option<String>,
    pub status: Option<String>,
    pub validation_notes: Option<String>,
}

pub struct CategorisationAndUsage {
    pub tags: Option<Vec<String>>,
    pub license: Option<String>,
    pub usage: Option<Vec<String>>,
}

/// Struct to represent the metadata of a codelist
///
/// # Fields
/// * `source` - The source of the codelist
/// * `authors` - The authors of the codelist
/// * `version` - The version of the codelist
/// * `description` - The description of the codelist
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Metadata {
    pub source: MetadataSource,
    pub authors: Option<Vec<String>>,
    pub version: Option<String>, /// @emma we can enforce this to be something with a date format
    pub description: Option<String>,
}

impl Metadata {
    /// Create a new metadata
    ///
    /// # Arguments
    /// * `source` - The source of the codelist
    /// * `authors` - The authors of the codelist
    /// * `version` - The version of the codelist
    /// * `description` - The description of the codelist
    pub fn new(source: MetadataSource, authors: Option<Vec<String>>, version: Option<String>, description: Option<String>) -> Metadata {
        Metadata {
            source,
            authors,
            version,
            description,
        }
    }

    /// Add an author to the metadata
    ///
    /// # Arguments
    /// * `author` - The author to add
    pub fn add_author(&mut self, author: String) {
        if let Some(authors) = &mut self.authors {
            authors.push(author);
        } else {
            self.authors = Some(vec![author]);
        }
    }

    /// Remove an author from the metadata
    ///
    /// # Arguments
    /// * `author` - The author to remove
    pub fn remove_author(&mut self, author: String) {
        if let Some(authors) = &mut self.authors {
            let index = authors.iter().position(|x| x == &author);
            if let Some(index) = index {
                authors.remove(index);
            }
        }
    }

    /// Add a description to the metadata
    ///
    /// # Arguments
    /// * `description` - The description to add
    pub fn add_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Remove the description from the metadata
    ///
    /// # Arguments
    /// * `description` - The description to remove
    pub fn remove_description(&mut self) {
        self.description = None;
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
    fn test_new_metadata() {
        let metadata = Metadata {
            source: MetadataSource::LoadedFromFile,
            authors: Some(vec!["Author 1".to_string(), "Author 2".to_string()]),
            version: Some("1.0.0".to_string()),
            description: Some("This is a codelist".to_string()),
        };

        assert_eq!(metadata.source, MetadataSource::LoadedFromFile);
        assert_eq!(metadata.authors, Some(vec!["Author 1".to_string(), "Author 2".to_string()]));
        assert_eq!(metadata.version, Some("1.0.0".to_string()));
        assert_eq!(metadata.description, Some("This is a codelist".to_string()));
    }

    #[test]
    fn test_metadata_with_no_authors() {
        let metadata = Metadata {
            source: MetadataSource::LoadedFromFile,
            authors: None,
            version: Some("1.0.0".to_string()),
            description: Some("This is a codelist".to_string()),
        };
    }

    #[test]
    fn test_metadata_with_no_version() {
        let metadata = Metadata {
            source: MetadataSource::LoadedFromFile,
            authors: Some(vec!["Author 1".to_string(), "Author 2".to_string()]),
            version: None,
            description: Some("This is a codelist".to_string()),
        };
    }

    #[test]
    fn test_metadata_with_no_description() {
        let metadata = Metadata {
            source: MetadataSource::LoadedFromFile,
            authors: Some(vec!["Author 1".to_string(), "Author 2".to_string()]),
            version: Some("1.0.0".to_string()),
            description: None,
        };
    }

    #[test]
    fn test_add_author() {
        let mut metadata = Metadata {
            source: MetadataSource::LoadedFromFile,
            authors: Some(vec!["Author 1".to_string()]),
            version: Some("1.0.0".to_string()),
            description: Some("This is a codelist".to_string()),
        };

        metadata.add_author("Author 2".to_string());

        assert_eq!(metadata.authors, Some(vec!["Author 1".to_string(), "Author 2".to_string()]));
    }

    #[test]
    fn test_remove_author() {
        let mut metadata = Metadata {
            source: MetadataSource::LoadedFromFile,
            authors: Some(vec!["Author 1".to_string(), "Author 2".to_string()]),
            version: Some("1.0.0".to_string()),
            description: Some("This is a codelist".to_string()),
        };

        metadata.remove_author("Author 2".to_string());
        assert_eq!(metadata.authors, Some(vec!["Author 1".to_string()]));
    }

    #[test]
    fn test_add_description() {
        let mut metadata = Metadata {
            source: MetadataSource::LoadedFromFile,
            authors: Some(vec!["Author 1".to_string()]),
            version: Some("1.0.0".to_string()),
            description: Some("This is a codelist".to_string()),
        };

        metadata.add_description("This is a new description".to_string());
        assert_eq!(metadata.description, Some("This is a new description".to_string()));
    }

    #[test]
    fn test_remove_description() {
        let mut metadata = Metadata {
            source: MetadataSource::LoadedFromFile,
            authors: Some(vec!["Author 1".to_string()]),
            version: Some("1.0.0".to_string()),
            description: Some("This is a codelist".to_string()),
        };

        metadata.remove_description();
        assert_eq!(metadata.description, None);
    }



}