// We are importing the CodeList struct from the codelist_rs crate
use codelist_rs::codelist::CodeList;
use codelist_rs::metadata::metadata::Metadata;
use codelist_rs::metadata::provenance::Provenance;
use codelist_rs::metadata::categorisation_and_usage::CategorisationAndUsage;
use codelist_rs::metadata::purpose_and_context::PurposeAndContext;
use codelist_rs::metadata::validation_and_review::ValidationAndReview;
use codelist_rs::metadata::metadata_source::MetadataSource;
use codelist_rs::types::CodeListType;
use codelist_rs::errors::CodeListError;

fn main() -> Result<(), CodeListError> {

    // Metadata for the codelist
    let metadata = Metadata::new(
        Provenance::new(MetadataSource::ManuallyCreated, None),
        CategorisationAndUsage::new(None, None, None),
        PurposeAndContext::new(None, None, None),
        ValidationAndReview::new(None, None, None, None, None),
    );

    // Create a new codelist
    let mut codelist = CodeList::new(
        CodeListType::ICD10,
        metadata,
        None,
    );

    codelist.add_entry("A00".to_string(), "Cholera".to_string(), None)?;
    codelist.add_entry("A01".to_string(), "Typhoid and paratyphoid fevers".to_string(), None)?;

    println!("{:?}", codelist);

    Ok(())
}
