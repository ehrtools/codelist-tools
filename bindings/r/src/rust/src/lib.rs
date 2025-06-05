#![allow(clippy::incompatible_msrv)]
use codelist_rs::{codelist::CodeList as BaseCodelist, metadata::Metadata, types::CodeListType};
use extendr_api::prelude::*;

#[extendr]
fn hello() -> &'static str {
    println!("hello function called");
    "hello there!"
}

#[extendr]
struct Codelist {
    name: String,
    inner: BaseCodelist,
}

#[extendr]
impl Codelist {
    fn new(name: String) -> Self {
        let mut codelist = BaseCodelist::new(
            "test_codelist".to_string(),
            CodeListType::ICD10,
            Metadata::default(),
            None,
        );
        codelist.add_entry("R65.2".to_string(), None, None).unwrap();

        codelist
            .add_entry(
                "A48.51".to_string(),
                Some("Infant botulism".to_string()),
                Some("test comment".to_string()),
            )
            .unwrap();
        let inner = codelist;

        Codelist { name, inner }
    }

    fn set_name(&mut self, new_name: String) -> &mut Self {
        self.name = new_name;
        self
    }

    fn get_entries(&self) -> List {
        let entries: Vec<List> = self
            .inner
            .entries
            .iter()
            .map(|(code, (description, comment))| {
                list!(
                    code = code.clone(),
                    description = description.clone().unwrap_or_default(),
                    comment = comment.clone().unwrap_or_default()
                )
            })
            .collect();

        List::from_values(entries)
    }
}

extendr_module! {
    mod codelist;
    impl Codelist;
    fn hello;
}
