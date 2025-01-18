//! This file contains the metadata struct and its implementation

// Internal imports
use crate::metadata::provenance::Provenance;
use crate::metadata::categorisation_and_usage::CategorisationAndUsage;
use crate::metadata::metadata_source::MetadataSource;
use crate::metadata::purpose_and_context::PurposeAndContext;
use crate::metadata::validation_and_review::ValidationAndReview;

pub struct Metadata {
    pub provenance: Provenance,
    pub categorisation_and_usage: CategorisationAndUsage,
    pub metadata_source: MetadataSource,
    pub purpose_and_context: PurposeAndContext,
    pub validation_and_review: ValidationAndReview,
}

impl Metadata {
    // new
    // add each field?
}

#[cfg(test)]
mod tests {
    use super::*;
}