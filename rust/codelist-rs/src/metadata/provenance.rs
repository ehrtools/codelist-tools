//! This file contains the provenance struct and its implementation
//!
//! Note: Contributors are maintained in their original insertion order using
//! IndexSet.

// External imports
use chrono::Utc;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

// Internal imports
use crate::errors::CodeListError;
use crate::metadata::metadata_source::Source;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Provenance {
    pub source: Source,
    pub created_date: chrono::DateTime<Utc>,
    pub last_modified_date: chrono::DateTime<Utc>,
    pub contributors: IndexSet<String>,
}

impl Default for Provenance {
    fn default() -> Self {
        Provenance::new(Source::ManuallyCreated, None)
    }
}

impl Provenance {
    /// Create a new provenance
    ///
    /// # Arguments
    /// * `source` - The source of the codelist
    pub fn new(source: Source, contributors: Option<IndexSet<String>>) -> Provenance {
        Self {
            source,
            created_date: Utc::now(),
            last_modified_date: Utc::now(),
            contributors: contributors.unwrap_or_default(),
        }
    }

    /// Update the last modified date
    ///
    /// # Arguments
    /// * `self` - The provenance to update
    pub fn update_last_modified_date(&mut self) {
        self.last_modified_date = Utc::now();
    }

    /// Add a contributor to the provenance
    ///
    /// # Arguments
    /// * `self` - The provenance to update
    /// * `contributor` - The contributor to add
    pub fn add_contributor(&mut self, contributor: String) {
        self.contributors.insert(contributor);
    }

    /// Remove a contributor from the provenance
    ///
    /// # Arguments
    /// * `self` - The provenance to update
    /// * `contributor` - The contributor to remove
    pub fn remove_contributor(&mut self, contributor: String) -> Result<(), CodeListError> {
        if self.contributors.shift_remove(&contributor) {
            Ok(())
        } else {
            Err(CodeListError::contributor_not_found(contributor))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // helper function to get the time difference between the current time and the
    // given date
    fn get_time_difference(date: chrono::DateTime<Utc>) -> i64 {
        let now = chrono::Utc::now();
        (date - now).num_milliseconds().abs()
    }

    fn create_test_provenance_no_contributors() -> Provenance {
        Provenance::new(Source::LoadedFromFile, None)
    }

    fn create_test_provenance_with_contributors() -> Provenance {
        let mut contributors = IndexSet::new();
        contributors.insert("Example Contributor".to_string());
        Provenance::new(Source::LoadedFromFile, Some(contributors))
    }

    #[test]
    fn test_new_provenance_no_contributors() {
        let provenance = create_test_provenance_no_contributors();
        assert_eq!(provenance.source, Source::LoadedFromFile);
        let time_difference = get_time_difference(provenance.created_date);
        assert!(time_difference < 1000);
        let time_difference = get_time_difference(provenance.last_modified_date);
        assert!(time_difference < 1000);
        assert_eq!(provenance.contributors, IndexSet::new());
    }

    #[test]
    fn test_new_provenance_with_contributors() {
        let provenance = create_test_provenance_with_contributors();
        assert_eq!(provenance.source, Source::LoadedFromFile);
        assert_eq!(provenance.contributors, IndexSet::from(["Example Contributor".to_string()]));
        let time_difference = get_time_difference(provenance.created_date);
        assert!(time_difference < 1000);
        let time_difference = get_time_difference(provenance.last_modified_date);
        assert!(time_difference < 1000);
    }

    #[test]
    fn test_update_last_modified_date() {
        let mut provenance = create_test_provenance_no_contributors();
        provenance.update_last_modified_date();
        let time_difference = get_time_difference(provenance.last_modified_date);
        assert!(time_difference < 1000);
    }

    #[test]
    fn test_add_contributor() {
        let mut provenance = create_test_provenance_no_contributors();
        provenance.add_contributor("Example Contributor".to_string());
        assert_eq!(provenance.contributors, IndexSet::from(["Example Contributor".to_string()]));
    }

    #[test]
    fn test_remove_contributor() -> Result<(), CodeListError> {
        let mut provenance = create_test_provenance_with_contributors();
        provenance.add_contributor("Example Contributor".to_string());
        provenance.remove_contributor("Example Contributor".to_string())?;
        assert_eq!(provenance.contributors, IndexSet::new());
        Ok(())
    }

    #[test]
    fn test_remove_contributor_not_found() {
        let mut provenance = create_test_provenance_no_contributors();
        let error = provenance.remove_contributor("Example Contributor".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Contributor Example Contributor not found");
    }

    #[test]
    fn test_contributors_order_is_maintained() -> Result<(), CodeListError> {
        let mut provenance = create_test_provenance_no_contributors();

        provenance.add_contributor("Example1".to_string());
        {
            let mut iter = provenance.contributors.iter();
            assert_eq!(iter.next(), Some(&"Example1".to_string()));
            assert_eq!(iter.next(), None);
        }

        provenance.add_contributor("Example2".to_string());
        {
            let mut iter = provenance.contributors.iter();
            assert_eq!(iter.next(), Some(&"Example1".to_string()));
            assert_eq!(iter.next(), Some(&"Example2".to_string()));
            assert_eq!(iter.next(), None);
        }

        provenance.add_contributor("Example3".to_string());
        {
            let mut iter = provenance.contributors.iter();
            assert_eq!(iter.next(), Some(&"Example1".to_string()));
            assert_eq!(iter.next(), Some(&"Example2".to_string()));
            assert_eq!(iter.next(), Some(&"Example3".to_string()));
            assert_eq!(iter.next(), None);
        }

        provenance.remove_contributor("Example2".to_string())?;
        {
            let mut iter = provenance.contributors.iter();
            assert_eq!(iter.next(), Some(&"Example1".to_string()));
            assert_eq!(iter.next(), Some(&"Example3".to_string()));
            assert_eq!(iter.next(), None);
        }

        Ok(())
    }
}
