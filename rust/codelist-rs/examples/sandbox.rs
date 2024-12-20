
// We are importing the CodeList struct from the codelist_rs crate
use codelist_rs::codelist::CodeList;
use codelist_rs::metadata::{Metadata, MetadataSource};
use codelist_rs::types::CodeListType;
use codelist_rs::errors::CodeListError;

fn main() -> Result<(), CodeListError> {

    // Metadata for the codelist
    let metadata = Metadata {
        source: MetadataSource::ManuallyCreated,
        authors: Some(vec!["Caroline Morton".to_string()]),
        version: Some("2024-12-19".to_string()),
        description: Some("A test codelist".to_string()),
    };

    // Create a new codelist
    let mut codelist = CodeList::new(
        CodeListType::ICD10,
        metadata,
        None,
    );

    codelist.add_entry("A00".to_string(), "Cholera".to_string())?;
    codelist.add_entry("A01".to_string(), "Typhoid and paratyphoid fevers".to_string())?;

    println!("{:?}", codelist);

    Ok(())
}
