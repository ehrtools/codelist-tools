//! This file contains the core functionality for the codelist

// External imports
use std::collections::HashSet;
use std::str::FromStr;
use serde::{Serialize, Deserialize};

// Internal imports
use crate::types::CodeListType;
use crate::code_entry::CodeEntry;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    source: String, /// @emma this could be an enum at some point .e.g mapped from another codelist, or laoded from file
    authors: Option<Vec<String>>,
    version: Option<String>, /// @emma we can enforce this to be something with a date format
    description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeListOptions {
    allow_duplicates: bool,
    truncate_to_3_digits: bool,  // ICD10 specific only
    add_x_codes: bool,           // ICD10 specific only
    code_column_name: String,
    term_column_name: String,
}


/// Codelist Options
impl Default for CodeListOptions {
    fn default() -> Self {
        Self {
            allow_duplicates: false,
            truncate_to_3_digits: false,
            add_x_codes: false,
            code_column_name: "code".to_string(),
            term_column_name: "term".to_string(),
        }
    }
}


/// Struct to represent a codelist
///
/// # Fields
/// * `entries` - The set of code entries
/// * `codelist_type` - The type of codelist
/// * `metadata` - Metadata about the codelist
/// * `logs` - Logs of anything that happened during the codelist creation
/// * `codelist_options` - Options for the codelist
#[derive(Debug)]
pub struct CodeList {
    entries: HashSet<CodeEntry>,
    codelist_type: CodeListType,
    metadata: Metadata,
    logs: Vec<String>, // We will want to make this a struct with more info at some point
    codelist_options: CodeListOptions,
}


impl CodeList {
    pub fn new(codelist_type: CodeListType, metadata: Metadata, options: Option<CodeListOptions>) -> Self {
        CodeList {
            entries: HashSet::new(),
            codelist_type,
            metadata,
            logs: Vec::new(),
            codelist_options: options.unwrap_or_default(),
        }
    }

    pub fn codelist_type(&self) -> &CodeListType {
        &self.codelist_type
    }

    pub fn add_entry(&mut self, code: String, term: String) {
        let entry = CodeEntry::new(code, term);
        self.entries.insert(entry);
    }

    pub fn remove_entry(&mut self, code: &str) {
        // Some implementation
    }

    pub fn entries(&self) -> &HashSet<CodeEntry> {
        &self.entries
    }

    pub fn save_to_csv(&self, file_path: &str) {
        // Some implementation
    }

    pub fn save_to_json(&self, file_path: &str) {
        // Some implementation
    }

    pub fn save_log(&self, file_path: &str) {
        // Some implementation
    }

}




//TODO:
// several options of making codelist, e.g. excel, txt file, csv, hashset - codelistfactory handles this
// pub struct CodeListFactory {
//     input_directory: String,
//     output_directory: String,
// }
// impl CodeListFactory {
//     pub fn generate_codelist() {
//         // method for taking in data and outputting result of codelist or error
//     }
// also need save to format function (e.g. to csv) - code/term columns, all valid
// }