//! This file contains the metadata struct and its implementation

// External imports
use std::collections::HashSet;
use chrono::Utc;
use serde::{Deserialize, Serialize};

// Internal imports
use crate::metadata::provenance::Provenance;
use crate::metadata::categorisation_and_usage::CategorisationAndUsage;
use crate::metadata::metadata_source::MetadataSource;
use crate::metadata::purpose_and_context::PurposeAndContext;
use crate::metadata::validation_and_review::ValidationAndReview;
use crate::errors::CodeListError;


/// Metadata struct
/// 
/// This struct contains the metadata of a codelist
/// 
/// # Fields
/// * `provenance` - The provenance of the codelist
/// * `categorisation_and_usage` - The categorisation and usage of the codelist
/// * `metadata_source` - The source of the codelist
/// * `purpose_and_context` - The purpose and context of the codelist
/// * `validation_and_review` - The validation and review of the codelist

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub provenance: Provenance,
    pub categorisation_and_usage: CategorisationAndUsage,
    pub purpose_and_context: PurposeAndContext,
    pub validation_and_review: ValidationAndReview,
}

impl Metadata {
    /// Create new Metadata
    /// 
    /// # Arguments
    /// * `provenance` - The provenance of the codelist
    /// * `categorisation_and_usage` - The categorisation and usage of the codelist
    /// * `metadata_source` - The source of the codelist
    /// * `purpose_and_context` - The purpose and context of the codelist
    /// * `validation_and_review` - The validation and review of the codelist
    /// 
    /// # Returns
    /// * `Metadata` - The new Metadata
    pub fn new(provenance: Provenance, categorisation_and_usage: CategorisationAndUsage, purpose_and_context: PurposeAndContext, validation_and_review: ValidationAndReview) -> Self {
        Self { provenance, categorisation_and_usage, purpose_and_context, validation_and_review }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // helper function to get the time difference between the current time and the given date
    fn get_time_difference(date: chrono::DateTime<Utc>) -> i64 {
        let now = chrono::Utc::now();
        (date - now).num_milliseconds().abs()
    }

    #[test]
    fn test_new() -> Result<(), CodeListError> {
        let provenance = Provenance::new(MetadataSource::ManuallyCreated, Some(HashSet::from(["Test".to_string()])));
        let categorisation_and_usage = CategorisationAndUsage::new(Some(HashSet::from(["tag1".to_string()])), Some(HashSet::from(["usage1".to_string()])), Some("license1".to_string()));
        let purpose_and_context = PurposeAndContext::new(Some("purpose1".to_string()), Some("target_audience1".to_string()), Some("use_context1".to_string()));
        let validation_and_review = ValidationAndReview::new(Some(true), Some("reviewer1".to_string()), Some(chrono::Utc::now()), Some("status1".to_string()), Some("validation_notes1".to_string()));
        let metadata = Metadata::new(provenance, categorisation_and_usage, purpose_and_context, validation_and_review);

        assert_eq!(metadata.provenance.source, MetadataSource::ManuallyCreated);
        let time_difference = get_time_difference(metadata.provenance.created_date);
        assert!(time_difference < 1000);
        let time_difference = get_time_difference(metadata.provenance.last_modified_date);
        assert!(time_difference < 1000);
        assert_eq!(metadata.provenance.contributors, HashSet::from(["Test".to_string()]));

        assert_eq!(metadata.categorisation_and_usage.tags, HashSet::from(["tag1".to_string()]));
        assert_eq!(metadata.categorisation_and_usage.usage, HashSet::from(["usage1".to_string()]));
        assert_eq!(metadata.categorisation_and_usage.license, Some("license1".to_string()));

        assert_eq!(metadata.purpose_and_context.purpose, Some("purpose1".to_string()));
        assert_eq!(metadata.purpose_and_context.target_audience, Some("target_audience1".to_string()));
        assert_eq!(metadata.purpose_and_context.use_context, Some("use_context1".to_string()));

        assert_eq!(metadata.validation_and_review.reviewed, true);
        assert_eq!(metadata.validation_and_review.reviewer, Some("reviewer1".to_string()));
        let time_difference = get_time_difference(metadata.validation_and_review.review_date
            .ok_or(CodeListError::ReviewDateIsNone)?);
        assert!(time_difference < 1000);
        assert_eq!(metadata.validation_and_review.status, Some("status1".to_string()));
        assert_eq!(metadata.validation_and_review.validation_notes, Some("validation_notes1".to_string()));

        Ok(())
    }
}