//! This file contains the categorisation and usage struct and its
//! implementation

// External imports
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

// Internal imports
use crate::errors::CodeListError;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
    pub fn new(
        tags: Option<HashSet<String>>,
        usage: Option<HashSet<String>>,
        license: Option<String>,
    ) -> Self {
        Self { tags: tags.unwrap_or_default(), usage: usage.unwrap_or_default(), license }
    }

    /// Add a tag to the categorisation and usage
    ///
    /// # Arguments
    /// * `self` - The categorisation and usage to update
    /// * `tag` - The tag to add
    pub fn add_tag(&mut self, tag: String) -> Result<(), CodeListError> {
        if self.tags.insert(tag.clone()) {
            Ok(())
        } else {
            Err(CodeListError::tag_already_exists(format!(
                "Unable to add tag {tag}. Tag already exists.",
            )))
        }
    }

    /// Remove a tag from the categorisation and usage
    ///
    /// # Arguments
    /// * `self` - The categorisation and usage to update
    /// * `tag` - The tag to remove
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - Unit type if successful, or an error if
    ///   the tag does not exist
    pub fn remove_tag(&mut self, tag: String) -> Result<(), CodeListError> {
        if self.tags.remove(&tag) {
            Ok(())
        } else {
            Err(CodeListError::tag_does_not_exist(format!("Unable to remove tag {tag}",)))
        }
    }

    /// Add a usage to the categorisation and usage
    ///
    /// # Arguments
    /// * `self` - The categorisation and usage to update
    /// * `usage` - The usage to add
    pub fn add_usage(&mut self, usage: String) {
        self.usage.insert(usage);
    }

    /// Remove a usage from the categorisation and usage
    ///
    /// # Arguments
    /// * `self` - The categorisation and usage to update
    /// * `usage` - The usage to remove
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - The result of the operation, or an error
    ///   if the usage does not exist
    pub fn remove_usage(&mut self, usage: String) -> Result<(), CodeListError> {
        if self.usage.remove(&usage) {
            Ok(())
        } else {
            Err(CodeListError::usage_does_not_exist(format!("Unable to remove usage {usage}",)))
        }
    }

    /// Add a license to the categorisation and usage
    ///
    /// # Arguments
    /// * `self` - The categorisation and usage to update
    /// * `license` - The license to add
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - Unit type if successful, or an error if
    ///   the license already exists
    pub fn add_license(&mut self, license: String) -> Result<(), CodeListError> {
        if self.license.is_some() {
            Err(CodeListError::license_already_exists(format!(
                "Unable to add license {license}. Please use update license instead.",
            )))
        } else {
            self.license = Some(license);
            Ok(())
        }
    }

    /// Update the license of the categorisation and usage
    ///
    /// # Arguments
    /// * `self` - The categorisation and usage to update
    /// * `license` - The license to update to
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - Unit type if successful, or an error if
    ///   the license does not exist
    pub fn update_license(&mut self, license: String) -> Result<(), CodeListError> {
        if self.license.is_some() {
            self.license = Some(license);
            Ok(())
        } else {
            Err(CodeListError::license_does_not_exist(format!(
                "Unable to update license {license}. Please use add license instead.",
            )))
        }
    }

    /// Remove the license of the categorisation and usage
    ///
    /// # Arguments
    /// * `self` - The categorisation and usage to update
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - Unit type if successful, or an error if
    ///   the license does not exist
    pub fn remove_license(&mut self) -> Result<(), CodeListError> {
        if self.license.is_some() {
            self.license = None;
            Ok(())
        } else {
            Err(CodeListError::license_does_not_exist(
                "Unable to remove license. Please use add license instead.".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a test categorisation and usage with all parameters
    // set to None
    fn test_categorisation_and_usage_all_none() -> CategorisationAndUsage {
        CategorisationAndUsage::new(None, None, None)
    }

    /// Helper function to create a test categorisation and usage with all
    /// paramaters set to some value
    fn test_categorisation_and_usage_all_some() -> CategorisationAndUsage {
        let mut tags = HashSet::new();
        tags.insert("tag1".to_string());
        tags.insert("tag2".to_string());
        let mut usage = HashSet::new();
        usage.insert("usage1".to_string());
        usage.insert("usage2".to_string());
        CategorisationAndUsage::new(Some(tags), Some(usage), Some("license1".to_string()))
    }

    #[test]
    fn test_new() -> Result<(), CodeListError> {
        let categorisation_and_usage = test_categorisation_and_usage_all_some();
        assert_eq!(
            categorisation_and_usage.tags,
            HashSet::from(["tag1".to_string(), "tag2".to_string()])
        );
        assert_eq!(
            categorisation_and_usage.usage,
            HashSet::from(["usage1".to_string(), "usage2".to_string()])
        );
        assert_eq!(categorisation_and_usage.license, Some("license1".to_string()));
        Ok(())
    }

    #[test]
    fn test_add_tag() -> Result<(), CodeListError> {
        let mut categorisation_and_usage = test_categorisation_and_usage_all_some();
        let _ = categorisation_and_usage.add_tag("tag3".to_string());
        assert_eq!(
            categorisation_and_usage.tags,
            HashSet::from(["tag1".to_string(), "tag2".to_string(), "tag3".to_string()])
        );
        Ok(())
    }

    #[test]
    fn test_remove_tag() -> Result<(), CodeListError> {
        let mut categorisation_and_usage = test_categorisation_and_usage_all_some();
        categorisation_and_usage.remove_tag("tag1".to_string())?;
        assert_eq!(categorisation_and_usage.tags, HashSet::from(["tag2".to_string()]));
        Ok(())
    }

    #[test]
    fn test_remove_tag_does_not_exist() -> Result<(), CodeListError> {
        let mut categorisation_and_usage = test_categorisation_and_usage_all_none();
        let error = categorisation_and_usage.remove_tag("example".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Tag does not exist: Unable to remove tag example");
        Ok(())
    }

    #[test]
    fn test_add_usage() -> Result<(), CodeListError> {
        let mut categorisation_and_usage = test_categorisation_and_usage_all_none();
        categorisation_and_usage.add_usage("usage3".to_string());
        let mut expected = HashSet::new();
        expected.insert("usage3".to_string());
        assert_eq!(categorisation_and_usage.usage, expected);
        Ok(())
    }

    #[test]
    fn test_remove_usage() -> Result<(), CodeListError> {
        let mut categorisation_and_usage = test_categorisation_and_usage_all_some();
        categorisation_and_usage.remove_usage("usage1".to_string())?;
        let mut expected = HashSet::new();
        expected.insert("usage2".to_string());
        assert_eq!(categorisation_and_usage.usage, expected);
        Ok(())
    }

    #[test]
    fn test_remove_usage_does_not_exist() -> Result<(), CodeListError> {
        let mut categorisation_and_usage = test_categorisation_and_usage_all_none();
        let error = categorisation_and_usage.remove_usage("example".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Usage does not exist: Unable to remove usage example");
        Ok(())
    }

    #[test]
    fn test_add_license() -> Result<(), CodeListError> {
        let mut categorisation_and_usage = test_categorisation_and_usage_all_none();
        categorisation_and_usage.add_license("license2".to_string())?;
        assert_eq!(categorisation_and_usage.license, Some("license2".to_string()));
        Ok(())
    }

    #[test]
    fn test_add_license_already_exists() -> Result<(), CodeListError> {
        let mut categorisation_and_usage = test_categorisation_and_usage_all_some();
        let error = categorisation_and_usage.add_license("license1".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "License already exists: Unable to add license license1. Please use update license instead.");
        Ok(())
    }

    #[test]
    fn test_update_license() -> Result<(), CodeListError> {
        let mut categorisation_and_usage = test_categorisation_and_usage_all_some();
        assert_eq!(categorisation_and_usage.license, Some("license1".to_string()));
        categorisation_and_usage.update_license("license2".to_string())?;
        assert_eq!(categorisation_and_usage.license, Some("license2".to_string()));
        Ok(())
    }

    #[test]
    fn test_update_license_does_not_exist() -> Result<(), CodeListError> {
        let mut categorisation_and_usage = test_categorisation_and_usage_all_none();
        let error = categorisation_and_usage.update_license("example".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "License does not exist: Unable to update license example. Please use add license instead.");
        Ok(())
    }

    #[test]
    fn test_remove_license() -> Result<(), CodeListError> {
        let mut categorisation_and_usage = test_categorisation_and_usage_all_some();
        assert_eq!(categorisation_and_usage.license, Some("license1".to_string()));
        categorisation_and_usage.remove_license()?;
        assert_eq!(categorisation_and_usage.license, None);
        Ok(())
    }

    #[test]
    fn test_remove_license_does_not_exist() -> Result<(), CodeListError> {
        let mut categorisation_and_usage = test_categorisation_and_usage_all_none();
        let error = categorisation_and_usage.remove_license().unwrap_err();
        let error_string = error.to_string();
        assert_eq!(
            error_string,
            "License does not exist: Unable to remove license. Please use add license instead."
        );
        Ok(())
    }
}
