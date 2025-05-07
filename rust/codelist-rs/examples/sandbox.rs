// We are importing the CodeList struct from the codelist_rs crate
use codelist_rs::{
    codelist::CodeList,
    errors::CodeListError,
    metadata::{
        categorisation_and_usage::CategorisationAndUsage, metadata::Metadata,
        metadata_source::Source, provenance::Provenance, purpose_and_context::PurposeAndContext,
        validation_and_review::ValidationAndReview,
    },
    types::CodeListType,
};

fn main() -> Result<(), CodeListError> {
    // Metadata for the codelist
    let metadata = Metadata::new(
        Provenance::new(Source::ManuallyCreated, None),
        CategorisationAndUsage::new(None, None, None),
        PurposeAndContext::new(None, None, None),
        ValidationAndReview::new(None, None, None, None, None),
    );

    // Create a new codelist
    let mut codelist =
        CodeList::new("test_codelist".to_string(), CodeListType::ICD10, metadata, None);

    codelist.add_entry("A00".to_string(), "Cholera".to_string(), None)?;
    codelist.add_entry("A01".to_string(), "Typhoid and paratyphoid fevers".to_string(), None)?;

    println!("{codelist:?}");

    Ok(())
}
