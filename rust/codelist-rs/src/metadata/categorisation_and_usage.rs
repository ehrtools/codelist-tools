//! This file contains the categorisation and usage struct and its implementation

// External imports
use std::collections::HashSet;

// Internal imports
use crate::errors::CodeListError;

pub struct CategorisationAndUsage {
    pub tags: HashSet<String>,
    pub usage: HashSet<String>,
    pub license: Option<String>,
}

impl CategorisationAndUsage {
    /// Create new CategorisationAndUsage
    ///
    /// # Arguments
    /// * `tags` - The tags of the codelist
    /// * `usage` - The usage of the codelist
    /// * `license` - The license of the codelist
    ///
    /// # Returns
    /// * `CategorisationAndUsage` - The new CategorisationAndUsage
    pub fn new(tags: Option<HashSet<String>>, usage: Option<HashSet<String>>, license: Option<String>) -> Self {
        Self {
            tags: tags.unwrap_or_default(),
            usage: usage.unwrap_or_default(),
            license,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a test categorisation and usage
    fn test_categorisation_and_usage() -> CategorisationAndUsage {
        CategorisationAndUsage::new(None, Some(HashSet::from(["usage1".to_string(), "usage2".to_string()])), Some("license1".to_string()))
    }

    #[test] 
    fn test_new() -> Result<(), CodeListError> {
        let categorisation_and_usage = test_categorisation_and_usage();
        assert_eq!(categorisation_and_usage.tags, HashSet::new());
        assert_eq!(categorisation_and_usage.usage, HashSet::from(["usage1".to_string(), "usage2".to_string()]));
        assert_eq!(categorisation_and_usage.license, Some("license1".to_string()));
        Ok(())
    }
}