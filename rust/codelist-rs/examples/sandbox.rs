
// We are importing the CodeList struct from the codelist_rs crate
use codelist_rs::codelist::CodeList;


fn main() {
    // Create a new codelist
    let mut codelist = CodeList::new(
        "ICD10",
        "code".to_string(),
        "term".to_string(),
    );

    println!("{:?}", codelist);
}