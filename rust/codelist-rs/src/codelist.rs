//! This file contains the core functionality for the codelist

// External imports
use std::{collections::HashSet, io::Write};

use csv::Writer;
use serde::{Deserialize, Serialize};

// Internal imports
use crate::code_entry::CodeEntry;
use crate::{
    codelist_options::CodeListOptions, errors::CodeListError, metadata::Metadata,
    types::CodeListType,
};

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
    pub name: String,
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
    /// * `entries` - The set of code entries
    /// * `codelist_type` - The type of codelist
    /// * `metadata` - Metadata describing the codelist
    /// * `logs` - Logs of anything that happened during the codelist creation
    /// * `options` - Customisable options for the codelist
    ///
    /// # Returns
    /// * `CodeList` - The new CodeList
    pub fn new(
        name: String,
        codelist_type: CodeListType,
        metadata: Metadata,
        options: Option<CodeListOptions>,
    ) -> Self {
        CodeList {
            name,
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
    pub fn add_entry(
        &mut self,
        code: String,
        term: String,
        comment: Option<String>,
    ) -> Result<(), CodeListError> {
        let entry = CodeEntry::new(code, term, comment)?;
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
    /// * `CodeListError::EntryNotFound` - If the entry to be removed is not
    ///   found
    pub fn remove_entry(&mut self, code: &str, term: &str) -> Result<(), CodeListError> {
        let removed =
            self.entries.remove(&CodeEntry::new(code.to_string(), term.to_string(), None)?);
        if removed {
            Ok(())
        } else {
            Err(CodeListError::entry_not_found(code))
        }
    }

    /// Get the full entries of the codelist, including code, term and optional
    /// comment
    ///
    /// # Returns
    /// * `&HashSet<CodeEntry>` - The entries of the codelist
    pub fn full_entries(&self) -> &HashSet<CodeEntry> {
        &self.entries
    }

    /// Get the code and term of the codelist
    ///
    /// # Returns
    /// * `HashSet<(&String, &String)>` - The codes and terms of the codelist
    pub fn code_term_entries(&self) -> HashSet<(&String, &String)> {
        self.entries.iter().map(|entry| (&entry.code, &entry.term)).collect()
    }

    /// Get the codes of the codelist
    ///
    /// # Returns
    /// * `HashSet<&String>` - The codes of the codelist
    pub fn codes(&self) -> HashSet<&String> {
        self.entries.iter().map(|entry| &entry.code).collect()
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
        wtr.write_record([
            &self.codelist_options.code_field_name,
            &self.codelist_options.term_field_name,
        ])?;
        for entry in self.entries.iter() {
            wtr.write_record([&entry.code, &entry.term])?;
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
            writeln!(file, "{log}")?;
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
    use chrono::Utc;
    use tempfile::TempDir;

    use super::*;
    use crate::metadata::{
        CategorisationAndUsage, Provenance, PurposeAndContext, Source, ValidationAndReview,
    };

    // Helper function to create test metadata
    fn create_test_metadata() -> Metadata {
        Metadata {
            provenance: Provenance::new(Source::ManuallyCreated, None),
            categorisation_and_usage: CategorisationAndUsage::new(None, None, None),
            purpose_and_context: PurposeAndContext::new(None, None, None),
            validation_and_review: ValidationAndReview::new(None, None, None, None, None),
        }
    }

    // Helper function to create a test codelist with two entries, default options
    // and test metadata
    fn create_test_codelist() -> Result<CodeList, CodeListError> {
        let mut codelist = CodeList::new(
            "test_codelist".to_string(),
            CodeListType::ICD10,
            create_test_metadata(),
            None,
        );
        codelist.add_entry("R65.2".to_string(), "Severe sepsis".to_string(), None)?;
        codelist.add_entry(
            "A48.51".to_string(),
            "Infant botulism".to_string(),
            Some("test comment".to_string()),
        )?;

        Ok(codelist)
    }

    // helper function to get the time difference between the current time and the
    // given date
    fn get_time_difference(date: chrono::DateTime<Utc>) -> i64 {
        let now = chrono::Utc::now();
        (date - now).num_milliseconds().abs()
    }

    #[test]
    fn test_create_codelist_default_options() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;

        assert_eq!(codelist.codelist_type(), &CodeListType::ICD10);
        assert_eq!(codelist.full_entries().len(), 2);
        assert_eq!(codelist.logs.len(), 0);
        assert_eq!(&codelist.codelist_options, &CodeListOptions::default());

        assert_eq!(codelist.metadata().provenance.source, Source::ManuallyCreated);
        let time_difference = get_time_difference(codelist.metadata().provenance.created_date);
        assert!(time_difference < 1000);
        let time_difference =
            get_time_difference(codelist.metadata().provenance.last_modified_date);
        assert!(time_difference < 1000);
        assert_eq!(codelist.metadata().provenance.contributors, HashSet::new());

        assert_eq!(codelist.metadata().categorisation_and_usage.tags, HashSet::new());
        assert_eq!(codelist.metadata().categorisation_and_usage.usage, HashSet::new());
        assert_eq!(codelist.metadata().categorisation_and_usage.license, None);

        assert_eq!(codelist.metadata().purpose_and_context.purpose, None);
        assert_eq!(codelist.metadata().purpose_and_context.target_audience, None);
        assert_eq!(codelist.metadata().purpose_and_context.use_context, None);

        assert!(!codelist.metadata().validation_and_review.reviewed);
        assert_eq!(codelist.metadata().validation_and_review.reviewer, None);
        assert_eq!(codelist.metadata().validation_and_review.review_date, None);
        assert_eq!(codelist.metadata().validation_and_review.status, None);
        assert_eq!(codelist.metadata().validation_and_review.validation_notes, None);

        Ok(())
    }

    #[test]
    fn test_create_codelist_custom_options() -> Result<(), CodeListError> {
        let metadata = create_test_metadata();

        let codelist_options = CodeListOptions {
            allow_duplicates: true,
            truncate_to_3_digits: true,
            add_x_codes: true,
            code_column_name: "test_code".to_string(),
            term_column_name: "test_term".to_string(),
            code_field_name: "test_code".to_string(),
            term_field_name: "test_term".to_string(),
        };

        let codelist = CodeList::new(
            "test_codelist".to_string(),
            CodeListType::ICD10,
            metadata,
            Some(codelist_options),
        );

        assert!(codelist.codelist_options.allow_duplicates);
        assert!(codelist.codelist_options.truncate_to_3_digits);
        assert!(codelist.codelist_options.add_x_codes);
        assert_eq!(codelist.codelist_options.code_field_name, "test_code".to_string());
        assert_eq!(codelist.codelist_options.term_field_name, "test_term".to_string());
        assert_eq!(codelist.codelist_options.code_column_name, "test_code".to_string());
        assert_eq!(codelist.codelist_options.term_column_name, "test_term".to_string());

        assert_eq!(codelist.metadata().provenance.source, Source::ManuallyCreated);
        let time_difference = get_time_difference(codelist.metadata().provenance.created_date);
        assert!(time_difference < 1000);
        let time_difference =
            get_time_difference(codelist.metadata().provenance.last_modified_date);
        assert!(time_difference < 1000);
        assert_eq!(codelist.metadata().provenance.contributors, HashSet::new());

        assert_eq!(codelist.metadata().categorisation_and_usage.tags, HashSet::new());
        assert_eq!(codelist.metadata().categorisation_and_usage.usage, HashSet::new());
        assert_eq!(codelist.metadata().categorisation_and_usage.license, None);

        assert_eq!(codelist.metadata().purpose_and_context.purpose, None);
        assert_eq!(codelist.metadata().purpose_and_context.target_audience, None);
        assert_eq!(codelist.metadata().purpose_and_context.use_context, None);

        assert!(!codelist.metadata().validation_and_review.reviewed);
        assert_eq!(codelist.metadata().validation_and_review.reviewer, None);
        assert_eq!(codelist.metadata().validation_and_review.review_date, None);
        assert_eq!(codelist.metadata().validation_and_review.status, None);
        assert_eq!(codelist.metadata().validation_and_review.validation_notes, None);

        assert_eq!(codelist.codelist_type(), &CodeListType::ICD10);
        assert_eq!(codelist.full_entries().len(), 0);
        assert_eq!(codelist.logs.len(), 0);

        Ok(())
    }

    #[test]
    fn test_duplicate_entries() -> Result<(), CodeListError> {
        let mut codelist = CodeList::new(
            "test_codelist".to_string(),
            CodeListType::ICD10,
            create_test_metadata(),
            None,
        );
        codelist.add_entry("R65.2".to_string(), "Severe sepsis".to_string(), None)?;
        codelist.add_entry("R65.2".to_string(), "Severe sepsis".to_string(), None)?;

        assert_eq!(codelist.full_entries().len(), 1);

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
        let entry1 = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string(), None)?;
        let entry2 = CodeEntry::new(
            "A48.51".to_string(),
            "Infant botulism".to_string(),
            Some("test comment".to_string()),
        )?;

        assert_eq!(codelist.full_entries().len(), 2);
        assert!(codelist.full_entries().contains(&entry1));
        assert!(codelist.full_entries().contains(&entry2));

        Ok(())
    }

    #[test]
    fn test_remove_entry_that_exists() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.remove_entry("R65.2", "Severe sepsis")?;
        let entry = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string(), None)?;

        assert_eq!(codelist.full_entries().len(), 1);
        assert!(!codelist.full_entries().contains(&entry));

        Ok(())
    }

    #[test]
    fn test_remove_entry_that_doesnt_exist() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let error = codelist.remove_entry("A48.52", "Infant botulism").unwrap_err();

        assert!(matches!(error, CodeListError::EntryNotFound { code } if code == "A48.52"));
        assert_eq!(codelist.full_entries().len(), 2);

        Ok(())
    }

    #[test]
    fn test_get_full_entries() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let entries = codelist.full_entries();
        let test_entry_1 = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string(), None)?;
        let test_entry_2 = CodeEntry::new(
            "A48.51".to_string(),
            "Infant botulism".to_string(),
            Some("test comment".to_string()),
        )?;

        assert_eq!(entries.len(), 2);
        assert!(entries.contains(&test_entry_1));
        assert!(entries.contains(&test_entry_2));

        Ok(())
    }

    #[test]
    fn test_get_code_term_entries() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let entries = codelist.code_term_entries();

        let test_entry_1 = (&"R65.2".to_string(), &"Severe sepsis".to_string());
        let test_entry_2 = (&"A48.51".to_string(), &"Infant botulism".to_string());

        assert_eq!(entries.len(), 2);
        assert!(entries.contains(&test_entry_1));
        assert!(entries.contains(&test_entry_2));

        Ok(())
    }

    #[test]
    fn test_get_codes() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let codes = codelist.codes();

        let test_code_1 = "R65.2".to_string();
        let test_code_2 = "A48.51".to_string();

        assert_eq!(codes.len(), 2);
        assert!(codes.contains(&test_code_1));
        assert!(codes.contains(&test_code_2));

        Ok(())
    }

    #[test]
    fn test_save_to_csv() -> Result<(), CodeListError> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.csv");
        let file_path_str = file_path
            .to_str()
            .ok_or(CodeListError::invalid_file_path("Path contains invalid Unicode characters"))?;
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
        let file_path_str = file_path
            .to_str()
            .ok_or(CodeListError::invalid_file_path("Path contains invalid Unicode characters"))?;

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
        let file_path_str = file_path
            .to_str()
            .ok_or(CodeListError::invalid_file_path("Path contains invalid Unicode characters"))?;

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
        let codelist =
            CodeList::new("test".to_string(), CodeListType::ICD10, metadata.clone(), None);

        assert_eq!(codelist.metadata(), &metadata);
    }
}

// add tests for get code, get code term entries
