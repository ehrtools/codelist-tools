pub mod metadata_source;
pub mod provenance;
pub mod purpose_and_context;
pub mod validation_and_review;
pub mod categorisation_and_usage;
pub mod metadata;

// Re-export the structs to make them available at the metadata module level
pub use metadata::Metadata;
pub use provenance::Provenance;
pub use categorisation_and_usage::CategorisationAndUsage;
pub use metadata_source::Source;
pub use purpose_and_context::PurposeAndContext;
pub use validation_and_review::ValidationAndReview;

