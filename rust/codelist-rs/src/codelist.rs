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

/// Struct to represent a codelist options
///
/// # Fields
/// * `allow_duplicates` - Whether to allow duplicates in the codelist
/// * `truncate_to_3_digits` - Whether to truncate the code to 3 digits
/// * `add_x_codes` - Whether to add x codes to the codelist
/// * `code_column_name` - The name of the code column
/// * `term_column_name` - The name of the term column
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CodeListOptions {
    pub allow_duplicates: bool,
    pub truncate_to_3_digits: bool,  // ICD10 specific only
    pub add_x_codes: bool,           // ICD10 specific only
    pub code_column_name: String,
    pub term_column_name: String,
}

impl Default for CodeListOptions {
    /// Default implementation for CodeListOptions
    ///
    /// # Returns
    /// * `CodeListOptions` - The default CodeListOptions
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
    pub entries: HashSet<CodeEntry>,
    pub codelist_type: CodeListType,
    pub metadata: Metadata,
    pub logs: Vec<String>, // We will want to make this a struct with more info at some point
    pub codelist_options: CodeListOptions,
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
    pub fn add_entry(&mut self, code: String, term: String) -> Result<(), CodeListError> {
        let entry = CodeEntry::new(code, term)?;
        self.entries.insert(entry);
        Ok(())
    }

    /// Remove an entry from the codelist
    ///
    /// # Arguments
    /// * `code` - The code to remove
    /// * `term` - The term to remove
    ///
    /// # Errors
    /// * `CodeListError::EntryNotFound` - If the entry to be removed is not found
    pub fn remove_entry(&mut self, code: &str, term: &str) -> Result<(), CodeListError> {
        let removed = self.entries.remove(&CodeEntry::new(code.to_string(), term.to_string())?);
        if removed {
            Ok(())
        } else {
            Err(CodeListError::EntryNotFound(code.to_string()))
        }
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
    /// * `CodeListError::IOError` - If an error occurs when writing to the file
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
    /// * `CodeListError::IOError` - If an error occurs when writing to the file
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
    /// * `CodeListError::IOError` - If an error occurs when writing to the file
    pub fn save_log(&self, file_path: &str) -> std::result::Result<(), CodeListError> {
        let mut file = std::fs::File::create(file_path)?;
        for log in &self.logs {
            writeln!(file, "{}", log)?;
        }
        Ok(())
    }

    /// Add a log message to the codelist
    ///
    /// # Arguments
    /// * `message` - The message to add to the log
    pub fn add_log(&mut self, message: String) {
        self.logs.push(message);
    }

    /// Get the metadata
    ///
    /// # Returns
    /// * `&Metadata` - The metadata
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::MetadataSource;
    use tempfile::TempDir;

    // Helper function to create test metadata
    fn create_test_metadata() -> Metadata {
        Metadata {
            source: MetadataSource::ManuallyCreated,
            authors: Some(vec!["Caroline Morton".to_string()]),
            version: Some("2024-12-19".to_string()),
            description: Some("A test codelist".to_string()),
        }
    }

    // Helper function to create a test codelist with two entries, default options and test metadata
    fn create_test_codelist() -> Result<CodeList, CodeListError> {
        let mut codelist = CodeList::new(CodeListType::ICD10, create_test_metadata(), None);
        codelist.add_entry("R65.2".to_string(), "Severe sepsis".to_string())?;
        codelist.add_entry("A48.51".to_string(), "Infant botulism".to_string())?;
        
        Ok(codelist)
    }

    #[test]
    fn test_create_codelist_default_options() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;

        assert_eq!(codelist.metadata().source, MetadataSource::ManuallyCreated);
        assert_eq!(codelist.metadata().authors, Some(vec!["Caroline Morton".to_string()]));
        assert_eq!(codelist.metadata().version, Some("2024-12-19".to_string()));
        assert_eq!(codelist.metadata().description, Some("A test codelist".to_string()));
        assert_eq!(codelist.codelist_type(), &CodeListType::ICD10);
        assert_eq!(codelist.entries().len(), 2);
        assert_eq!(codelist.logs.len(), 0);
        assert_eq!(&codelist.codelist_options, &CodeListOptions::default());

        Ok(())
    }

    #[test]
    fn test_create_codelist_custom_options() {
        let metadata = create_test_metadata();

        let codelist_options = CodeListOptions {
            allow_duplicates: true,
            truncate_to_3_digits: true,
            add_x_codes: true,
            code_column_name: "test_code".to_string(),
            term_column_name: "test_term".to_string(),
        };
        
        let codelist = CodeList::new(CodeListType::ICD10, metadata, Some(codelist_options));

        assert_eq!(codelist.codelist_options.allow_duplicates, true);
        assert_eq!(codelist.codelist_options.truncate_to_3_digits, true);
        assert_eq!(codelist.codelist_options.add_x_codes, true);
        assert_eq!(codelist.codelist_options.code_column_name, "test_code".to_string());
        assert_eq!(codelist.codelist_options.term_column_name, "test_term".to_string());

        assert_eq!(codelist.metadata().source, MetadataSource::ManuallyCreated);
        assert_eq!(codelist.metadata().authors, Some(vec!["Caroline Morton".to_string()]));
        assert_eq!(codelist.metadata().version, Some("2024-12-19".to_string()));
        assert_eq!(codelist.metadata().description, Some("A test codelist".to_string()));
        assert_eq!(codelist.codelist_type(), &CodeListType::ICD10);
        assert_eq!(codelist.entries().len(), 0);
        assert_eq!(codelist.logs.len(), 0);

    }

    #[test]
    fn test_duplicate_entries() -> Result<(), CodeListError> {
        let mut codelist = CodeList::new(CodeListType::ICD10, create_test_metadata(), None);
        codelist.add_entry("R65.2".to_string(), "Severe sepsis".to_string())?;
        codelist.add_entry("R65.2".to_string(), "Severe sepsis".to_string())?;

        assert_eq!(codelist.entries().len(), 1);

        Ok(())
    }

    #[test]
    fn test_get_codelist_type() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;

        assert_eq!(codelist.codelist_type(), &CodeListType::ICD10);

        Ok(())
    }

    #[test]
    fn test_add_entry() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let entry1 = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string())?;
        let entry2 = CodeEntry::new("A48.51".to_string(), "Infant botulism".to_string())?;
    
        assert_eq!(codelist.entries().len(), 2);
        assert!(codelist.entries().contains(&entry1));
        assert!(codelist.entries().contains(&entry2));

        Ok(())
    }

    #[test]
    fn test_remove_entry_that_exists() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.remove_entry("R65.2", "Severe sepsis")?;
        let entry = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string())?;

        assert_eq!(codelist.entries().len(), 1);
        assert!(!codelist.entries().contains(&entry));

        Ok(())
    }

    #[test]
    fn test_remove_entry_that_doesnt_exist() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let error = codelist.remove_entry("A48.52", "Infant botulism").unwrap_err();

        assert!(matches!(error, CodeListError::EntryNotFound(code) if code == "A48.52"));
        assert_eq!(codelist.entries().len(), 2);

        Ok(())
    }

    #[test]
    fn test_get_entries() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let entries = codelist.entries();
        let test_entry_1 = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string())?;
        let test_entry_2 = CodeEntry::new("A48.51".to_string(), "Infant botulism".to_string())?;

        assert_eq!(entries.len(), 2);
        assert!(entries.contains(&test_entry_1));
        assert!(entries.contains(&test_entry_2));

        Ok(())
    }

    #[test]
    fn test_save_to_csv() -> Result<(), CodeListError> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.csv");
        let file_path_str = file_path.to_str().ok_or(CodeListError::InvalidFilePath)?;
        let codelist = create_test_codelist()?;
        codelist.save_to_csv(file_path_str)?;
        let content = std::fs::read_to_string(file_path_str)?;
        let lines: Vec<&str> = content.lines().collect();
        let mut data_lines = lines[1..].to_vec();
        data_lines.sort();

        assert_eq!(lines[0], "code,term");
        assert_eq!(data_lines, vec!["A48.51,Infant botulism", "R65.2,Severe sepsis"]);

        Ok(())
    }
    
    #[test]
    fn test_save_to_json() -> Result<(), CodeListError> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test_codelist.json");
        let file_path_str = file_path.to_str().ok_or(CodeListError::InvalidFilePath)?;

        let original_codelist = create_test_codelist()?;
        original_codelist.save_to_json(file_path_str)?;
        let json_content = std::fs::read_to_string(file_path_str)?;
        let loaded_codelist: CodeList = serde_json::from_str(&json_content)?;

        assert_eq!(original_codelist, loaded_codelist);

        Ok(())
    }

    #[test]
    fn test_add_to_log() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_log("Test log message".to_string());

        assert_eq!(codelist.logs.len(), 1);
        assert_eq!(codelist.logs[0], "Test log message".to_string());

        Ok(())
    }   

    #[test]
    fn test_save_log() -> Result<(), CodeListError> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.log");
        let file_path_str = file_path.to_str().ok_or(CodeListError::InvalidFilePath)?;

        let mut codelist = create_test_codelist()?;
        codelist.add_log("Test log message".to_string());
        codelist.save_log(file_path_str)?;
        let content = std::fs::read_to_string(file_path_str)?;

        assert_eq!(content, "Test log message\n");

        Ok(())
    }

#[test]
    fn test_get_metadata() {
        let metadata = create_test_metadata();
        let codelist = CodeList::new(CodeListType::ICD10, metadata.clone(), None);

        assert_eq!(codelist.metadata(), &metadata);
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