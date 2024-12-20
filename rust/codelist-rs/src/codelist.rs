//! This file contains the core functionality for the codelist

// External imports
use std::collections::HashSet;
use std::io::Write;
use serde::{Serialize, Deserialize};
use csv::Writer;

// Internal imports
use crate::types::CodeListType;
use crate::code_entry::CodeEntry;
use crate::metadata::Metadata;
use crate::errors::CodeListError;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeList {
    entries: HashSet<CodeEntry>,
    codelist_type: CodeListType,
    metadata: Metadata,
    logs: Vec<String>, // We will want to make this a struct with more info at some point
    codelist_options: CodeListOptions,
}


impl CodeList {
    /// Create a new CodeList
    ///
    /// # Arguments
    /// * `codelist_type` - The type of codelist
    /// * `metadata` - Metadata describing the codelist
    /// * `options` - Customisable options for the codelist
    ///
    /// # Returns
    /// * `CodeList` - The new CodeList
    pub fn new(codelist_type: CodeListType, metadata: Metadata, options: Option<CodeListOptions>) -> Self {
        CodeList {
            entries: HashSet::new(),
            codelist_type,
            metadata,
            logs: Vec::new(),
            codelist_options: options.unwrap_or_default(),
        }
    }

    /// Get the type of the codelist
    ///
    /// # Returns
    /// * `&CodeListType` - The type of the codelist
    pub fn codelist_type(&self) -> &CodeListType {
        &self.codelist_type
    }

    /// Add an entry to the codelist
    ///
    /// # Arguments
    /// * `code` - The code to add
    /// * `term` - The term to add
    pub fn add_entry(&mut self, code: String, term: String) {
        let entry = CodeEntry::new(code, term);
        self.entries.insert(entry);
    }

    /// Remove an entry from the codelist
    ///
    /// # Arguments
    /// * `code` - The code to remove
    ///
    /// # Errors
    /// * `CodeListError::EntryNotFound` - If the entry to be removed is not found
    pub fn remove_entry(&mut self, code: &str) -> Result<(), CodeListError> {
        let initial_size = self.entries.len();
        self.entries.retain(|entry| entry.code != code);
        let final_size = self.entries.len();
        if initial_size == final_size {
            return Err(CodeListError::EntryNotFound(code.to_string()));
        }
        Ok(())
    }

    /// Get the entries of the codelist
    ///
    /// # Returns
    /// * `&HashSet<CodeEntry>` - The entries of the codelist
    pub fn entries(&self) -> &HashSet<CodeEntry> {
        &self.entries
    }

    /// Save the codelist entries to a CSV file
    ///
    /// # Arguments
    /// * `file_path` - The path to the file to save the codelist entries to
    ///
    /// # Errors
    /// * `CodeListError::IOError` - If the file cannot be written to
    pub fn save_to_csv(&self, file_path: &str) -> std::result::Result<(), CodeListError> {
        let mut wtr = Writer::from_path(file_path)?;
        // use column names from options
        wtr.write_record(&[&self.codelist_options.code_column_name, &self.codelist_options.term_column_name])?;
        for entry in self.entries.iter() {
            wtr.write_record(&[&entry.code, &entry.term])?;
        }
        wtr.flush()?;
        Ok(())
    }

    /// Save the codelist struct to a JSON file
    ///
    /// # Arguments
    /// * `file_path` - The path to the file to save the codelist struct to
    ///
    /// # Errors
    /// * `CodeListError::IOError` - If the file cannot be written to
    pub fn save_to_json(&self, file_path: &str) -> std::result::Result<(), CodeListError> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(file_path, json)?;
        Ok(())
    }

    /// Save the logs to a file
    ///
    /// # Arguments
    /// * `file_path` - The path to the file to save the logs to
    ///
    /// # Errors
    /// * `CodeListError::IOError` - If the file cannot be written to
    pub fn save_log(&self, file_path: &str) -> std::result::Result<(), CodeListError> {
        let mut file = std::fs::File::create(file_path)?;
        for log in &self.logs {
            writeln!(file, "{}", log)?;
        }
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codelist() {

    }
}

// 



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