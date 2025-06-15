// We are importing the CodeList struct from the codelist_rs crate
use codelist_rs::{
    codelist::CodeList, errors::CodeListError, metadata::Metadata, types::CodeListType,
};

fn main() -> Result<(), CodeListError> {
    // Create a new codelist
    let mut codelist =
        CodeList::new("test_codelist".to_string(), CodeListType::ICD10, Metadata::default(), None)?;

    codelist.add_entry("A00".to_string(), Some("Cholera".to_string()), None)?;
    codelist.add_entry(
        "A01".to_string(),
        Some("Typhoid and paratyphoid fevers".to_string()),
        None,
    )?;

    println!("{codelist:#?}");

    Ok(())
}
