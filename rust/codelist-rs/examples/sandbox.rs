// We are importing the CodeList struct from the codelist_rs crate
use codelist_rs::{codelist::CodeList, errors::CodeListError, types::CodeListType};

use std::default::Default;

fn main() -> Result<(), CodeListError> {
    // Metadata for the codelist
    let metadata = Default::default();

    // Create a new codelist
    let mut codelist =
        CodeList::new("test_codelist".to_string(), CodeListType::ICD10, metadata, None);

    codelist.add_entry("A00".to_string(), "Cholera".to_string(), None)?;
    codelist.add_entry("A01".to_string(), "Typhoid and paratyphoid fevers".to_string(), None)?;

    println!("{codelist:#?}");

    Ok(())
}
