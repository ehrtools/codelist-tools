//! This file contains the core functionality for the codelist

// External imports
use std::{
    collections::{BTreeMap, HashSet},
    io::Write,
    str::FromStr,
};

use csv::Writer;
use serde::{Deserialize, Serialize};

// Internal imports
use crate::{
    codelist_options::CodeListOptions, errors::CodeListError, metadata::Metadata,
    types::CodeListType,
};

/// Struct to represent a codelist
///
/// # Fields
/// * `name` - The name of the codelist
/// * `entries` - The set of code entries
/// * `codelist_type` - The type of codelist
/// * `metadata` - Metadata about the codelist
/// * `logs` - Logs of anything that happened during the codelist creation
/// * `codelist_options` - Options for the codelist
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeList {
    pub name: String,
    pub entries: BTreeMap<String, (Option<String>, Option<String>)>,
    pub codelist_type: CodeListType,
    pub metadata: Metadata,
    pub logs: Vec<String>, // We will want to make this a struct with more info at some point
    pub codelist_options: CodeListOptions,
}

impl CodeList {
    /// Create a new CodeList
    ///
    /// # Arguments
    /// * `name` - The name of the codelist
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
            entries: BTreeMap::new(),
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
    /// * `term` - The optional term to add
    /// * `comment` - The optional comment to add
    pub fn add_entry(
        &mut self,
        code: String,
        term: Option<String>,
        comment: Option<String>,
    ) -> Result<(), CodeListError> {
        if code.is_empty() {
            return Err(CodeListError::empty_code("Empty code supplied"));
        }
        self.entries.insert(code, (term, comment));
        Ok(())
    }

    /// Remove an entry from the codelist
    ///
    /// # Arguments
    /// * `code` - The code to remove
    ///
    /// # Errors
    /// * `CodeListError::EntryNotFound` - If the entry to be removed is not
    ///   found
    pub fn remove_entry(&mut self, code: &str) -> Result<(), CodeListError> {
        let removed = self.entries.remove(code);
        if removed.is_some() {
            Ok(())
        } else {
            Err(CodeListError::entry_not_found(code))
        }
    }

    /// Get the full entries of the codelist, including code, optional term and
    /// optional comment
    ///
    /// # Returns
    /// * `&BTreeMap<String, (Option<String>, Option<String>)` - The entries of
    ///   the codelist
    pub fn full_entries(&self) -> &BTreeMap<String, (Option<String>, Option<String>)> {
        &self.entries
    }

    /// Get the code and term of the codelist
    ///
    /// # Returns
    /// * `BTreeMap<&String, Option<&String>>` - The codes and terms of the
    ///   codelist
    pub fn code_term_entries(&self) -> BTreeMap<&String, Option<&String>> {
        self.entries.iter().map(|(code, (term_opt, _))| (code, term_opt.as_ref())).collect()
    }

    /// Get the codes of the codelist
    ///
    /// # Returns
    /// * `HashSet<&String>` - The codes of the codelist
    pub fn codes(&self) -> HashSet<&String> {
        self.entries.keys().collect()
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
        for (code, (term, _)) in self.entries.iter() {
            wtr.write_record([code, term.as_deref().unwrap_or("")])?;
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
    pub fn save_to_json(&self, file_path: &str) -> Result<(), CodeListError> {
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

    /// Add a comment to a code entry
    ///
    /// # Arguments
    /// * `code` - The code of the entry to add the comment to
    /// * `comment` - The comment to add
    ///
    /// # Returns
    /// * `Result<(), CodeListError>`
    ///
    /// # Errors
    /// * `CodeListError::CodeEntryCommentAlreadyExists` - If the comment
    ///   already exists
    pub fn add_comment(&mut self, code: String, comment: String) -> Result<(), CodeListError> {
        match self.entries.get_mut(&code) {
            Some((_, comment_opt)) => {
                if comment_opt.is_some() {
                    Err(CodeListError::code_entry_comment_already_exists(
                        code,
                        "Please use update comment instead",
                    ))
                } else {
                    *comment_opt = Some(comment);
                    Ok(())
                }
            }
            None => Err(CodeListError::entry_not_found(&code)),
        }
    }

    /// Update the comment for the code entry
    ///
    /// # Arguments
    /// * `code` - The code of the entry to update the comment for
    /// * `comment` - The comment to update
    ///
    /// # Returns
    /// * `Result<(), CodeListError>`
    ///
    /// # Errors
    /// * `CodeListError::CodeEntryCommentDoesNotExist` - If the comment does
    ///   not exist
    pub fn update_comment(&mut self, code: String, comment: String) -> Result<(), CodeListError> {
        match self.entries.get_mut(&code) {
            Some((_, comment_opt)) => {
                if comment_opt.is_some() {
                    *comment_opt = Some(comment);
                    Ok(())
                } else {
                    Err(CodeListError::code_entry_comment_does_not_exist(
                        code,
                        "Please use add comment instead",
                    ))
                }
            }
            None => Err(CodeListError::entry_not_found(&code)),
        }
    }

    /// Remove the comment from the code entry
    ///
    /// # Arguments
    /// * `code` - The code of the entry to remove the comment from
    ///
    /// # Returns
    /// * `Result<(), CodeListError>`
    ///
    /// # Errors
    /// * `CodeListError::CodeEntryCommentDoesNotExist` - If there is no comment
    ///   to remove
    pub fn remove_comment(&mut self, code: String) -> Result<(), CodeListError> {
        match self.entries.get_mut(&code) {
            Some((_, comment_opt)) => {
                if comment_opt.is_some() {
                    *comment_opt = None;
                    Ok(())
                } else {
                    Err(CodeListError::code_entry_comment_does_not_exist(
                        code,
                        "Unable to remove comment",
                    ))
                }
            }
            None => Err(CodeListError::entry_not_found(&code)),
        }
    }

    /// Add a term to the code entry
    ///
    /// # Arguments
    /// * `code` - The code of the entry to add the term to
    /// * `term` - The term to add
    ///
    /// # Errors
    /// * `CodeListError::CodeEntryTermAlreadyExists` - If the term already
    ///   exists
    pub fn add_term(&mut self, code: String, term: String) -> Result<(), CodeListError> {
        match self.entries.get_mut(&code) {
            Some((term_opt, _)) => {
                if term_opt.is_some() {
                    Err(CodeListError::code_entry_term_already_exists(
                        code,
                        "Please use update term instead",
                    ))
                } else {
                    *term_opt = Some(term);
                    Ok(())
                }
            }
            None => Err(CodeListError::entry_not_found(&code)),
        }
    }

    /// Update the term for the code entry
    ///
    /// # Arguments
    /// * `code` - The code of the entry to update the term for
    /// * `term` - The term to update
    ///
    /// # Errors
    /// * `CodeListError::CodeEntryTermDoesNotExist` - If the term does not
    ///   exist
    pub fn update_term(&mut self, code: String, term: String) -> Result<(), CodeListError> {
        match self.entries.get_mut(&code) {
            Some((term_opt, _)) => {
                if term_opt.is_some() {
                    *term_opt = Some(term);
                    Ok(())
                } else {
                    Err(CodeListError::code_entry_term_does_not_exist(
                        code,
                        "Please use add term instead",
                    ))
                }
            }
            None => Err(CodeListError::entry_not_found(&code)),
        }
    }

    /// Remove the term from the code entry
    ///
    /// # Arguments
    /// * `code` - The code of the entry to remove the term from
    ///
    /// # Errors
    /// * `CodeListError::CodeEntryTermDoesNotExist` - If there is no term to
    ///   remove
    pub fn remove_term(&mut self, code: String) -> Result<(), CodeListError> {
        match self.entries.get_mut(&code) {
            Some((term_opt, _)) => {
                if term_opt.is_some() {
                    *term_opt = None;
                    Ok(())
                } else {
                    Err(CodeListError::code_entry_term_does_not_exist(
                        code,
                        "Unable to remove term",
                    ))
                }
            }
            None => Err(CodeListError::entry_not_found(&code)),
        }
    }

    /// Truncate codelist entries to 3 digits
    ///
    /// # Arguments
    /// * `term_management` - How to handle ambiguous terms
    ///
    /// # Errors
    /// * `CodeListError::CodeListNotTruncatable` - If the codelist is not ICD10
    pub fn truncate_to_3_digits(
        &mut self,
        term_management: TermManagement,
    ) -> Result<(), CodeListError> {
        if !self.codelist_type.is_truncatable() {
            return Err(CodeListError::CodeListNotTruncatable {
                codelist_type: self.codelist_type.to_string(),
            });
        }

        // Keep track of all the three-digit codes
        let mut threes = self
            .entries
            .keys()
            .filter(|code| code.len() == 3)
            .cloned()
            .collect::<HashSet<String>>();

        let mut adds = vec![];
        let mut removes = vec![];

        for (code, (term, _)) in self.entries.iter() {
            // Codes of 3 or less will not be truncated
            if code.len() <= 3 {
                continue;
            }

            // We'll remove this one later
            removes.push(code.clone());

            // truncate the entry's code
            let truncated_code = code[..3].to_string();

            // If we already have this one, then go on to the next one
            if threes.contains(&truncated_code) {
                continue;
            }

            // Note that we've seen it
            threes.insert(truncated_code.clone());

            // The term and comment that goes with it to make the
            // entry depends on the term_management
            let (term, comment) = match term_management {
                TermManagement::DropTerm => {
                    (None, Some("Truncated to 3 digits, term discarded".to_string()))
                }

                TermManagement::First => (
                    term.clone(),
                    Some(format!("{code} truncated to 3 digits, term first encountered")),
                ),
            };

            // We'll add this one later
            adds.push((truncated_code, term, comment));
        }

        // Add the new three-digit codes
        for (code, term, comment) in &adds {
            self.add_entry(code.clone(), term.clone(), comment.clone())?;
        }

        // Remove the longer codes
        for code in &removes {
            self.remove_entry(code)?;
        }

        Ok(())
    }

    /// Add X to codelist entries that are 3 digits
    ///
    /// # Errors
    /// * `CodeListError::CodeListNotXAddable` - If the codelist is not ICD10
    pub fn add_x_codes(&mut self) -> Result<(), CodeListError> {
        if !self.codelist_type.is_x_addable() {
            return Err(CodeListError::CodeListNotXAddable {
                codelist_type: self.codelist_type.to_string(),
            });
        }

        // Keep track of all the four-digit codes ending in X
        let mut exes = self
            .entries
            .keys()
            .filter(|code| code.len() == 4 && code.ends_with("X"))
            .cloned()
            .collect::<HashSet<String>>();

        let mut adds = vec![];

        for (code, (term, comment)) in &self.entries {
            if code.len() == 3 {
                let mut new_code = code.clone();
                new_code.push('X');

                if exes.contains(&new_code) {
                    continue;
                }

                exes.insert(new_code.clone());

                adds.push((new_code, term.clone(), comment.clone()));
            }
        }

        for (code, term, comment) in &adds {
            self.add_entry(code.clone(), term.clone(), comment.clone())?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TermManagement {
    DropTerm,
    First,
}

/// Map Term Management from string
impl FromStr for TermManagement {
    type Err = CodeListError;
    /// Map TermManagement from a string
    fn from_str(s: &str) -> Result<Self, CodeListError> {
        match s.to_lowercase().as_str() {
            "drop_term" => Ok(TermManagement::DropTerm),
            "first" => Ok(TermManagement::First),
            _ => Err(CodeListError::TermManagementNotKnown { term_management: s.to_string() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use indexmap::IndexSet;
    use tempfile::TempDir;

    use super::*;
    use crate::metadata::{Metadata, Source};

    // Helper function to create a test codelist with two entries, default options
    // and test metadata
    fn create_test_codelist() -> Result<CodeList, CodeListError> {
        let mut codelist = CodeList::new(
            "test_codelist".to_string(),
            CodeListType::ICD10,
            Metadata::default(),
            None,
        );
        codelist.add_entry("R65.2".to_string(), None, None)?;

        codelist.add_entry(
            "A48.51".to_string(),
            Some("Infant botulism".to_string()),
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
        assert_eq!(codelist.entries.len(), 2);
        assert_eq!(codelist.logs.len(), 0);
        assert_eq!(&codelist.codelist_options, &CodeListOptions::default());

        assert_eq!(codelist.metadata().provenance.source, Source::ManuallyCreated);
        let time_difference = get_time_difference(codelist.metadata().provenance.created_date);
        assert!(time_difference < 1000);
        let time_difference =
            get_time_difference(codelist.metadata().provenance.last_modified_date);
        assert!(time_difference < 1000);
        assert_eq!(codelist.metadata().provenance.contributors, IndexSet::new());

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
        let codelist_options = CodeListOptions {
            allow_duplicates: true,
            code_column_name: "test_code".to_string(),
            term_column_name: "test_term".to_string(),
            code_field_name: "test_code".to_string(),
            term_field_name: "test_term".to_string(),
        };

        let codelist = CodeList::new(
            "test_codelist".to_string(),
            CodeListType::ICD10,
            Default::default(),
            Some(codelist_options),
        );

        assert!(codelist.codelist_options.allow_duplicates);
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
        assert_eq!(codelist.metadata().provenance.contributors, IndexSet::new());

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
        assert_eq!(codelist.entries.len(), 0);
        assert_eq!(codelist.logs.len(), 0);

        Ok(())
    }

    #[test]
    fn test_duplicate_entries() -> Result<(), CodeListError> {
        let mut codelist = CodeList::new(
            "test_codelist".to_string(),
            CodeListType::ICD10,
            Default::default(),
            None,
        );
        codelist.add_entry("R65.2".to_string(), Some("Severe sepsis".to_string()), None)?;
        codelist.add_entry("R65.2".to_string(), Some("Severe sepsis".to_string()), None)?;

        assert_eq!(codelist.entries.len(), 1);

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
        let code1 = "R65.2".to_string();
        let code2 = "A48.51".to_string();

        let first_entry = codelist.entries.get(&code1);
        let (term1, comment1) =
            first_entry.ok_or_else(|| CodeListError::entry_not_found(&code1))?;
        let second_entry = codelist.entries.get(&code2);
        let (term2, comment2) =
            second_entry.ok_or_else(|| CodeListError::entry_not_found(&code1))?;

        assert!(first_entry.is_some());
        assert!(comment1.is_none());
        assert_eq!(term1.as_deref(), None);

        assert!(second_entry.is_some());
        assert_eq!(comment2.as_deref(), Some("test comment"));
        assert_eq!(term2.as_deref(), Some("Infant botulism"));

        Ok(())
    }

    #[test]
    fn test_add_entry_with_empty_code_returns_error() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let error = codelist.add_entry("".to_string(), None, None).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(&error_string, "Empty code: Empty code supplied");
        Ok(())
    }

    #[test]
    fn test_remove_entry_that_exists() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.remove_entry("R65.2")?;
        let entry = codelist.entries.get("A48.51");
        let (term, comment) = entry.ok_or_else(|| CodeListError::entry_not_found("A48.51"))?;

        assert_eq!(codelist.entries.len(), 1);
        assert!(entry.is_some());
        assert_eq!(comment.as_deref(), Some("test comment"));
        assert_eq!(term.as_deref(), Some("Infant botulism"));

        Ok(())
    }

    #[test]
    fn test_remove_entry_that_doesnt_exist() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let error = codelist.remove_entry("A48.52").unwrap_err();

        assert!(matches!(error, CodeListError::EntryNotFound { code } if code == "A48.52"));
        assert_eq!(codelist.entries.len(), 2);

        Ok(())
    }

    #[test]
    fn test_get_code_term_entries() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let entries = codelist.code_term_entries();
        let expected_term = "Infant botulism".to_string();
        let key1 = "R65.2".to_string();
        let key2 = "A48.51".to_string();

        assert_eq!(entries.len(), 2);
        assert_eq!(entries.get(&key1), Some(&None));
        assert_eq!(entries.get(&key2), Some(&Some(&expected_term)));

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
        assert_eq!(data_lines, vec!["A48.51,Infant botulism", "R65.2,"]);

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
        let metadata: Metadata = Default::default();
        let codelist =
            CodeList::new("test".to_string(), CodeListType::ICD10, metadata.clone(), None);

        assert_eq!(codelist.metadata(), &metadata);
    }

    #[test]
    fn test_add_comment_when_comment_does_not_exist() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let expected_comment = "test comment";
        let key = "R65.2";
        codelist.add_comment(key.to_string(), expected_comment.to_string())?;

        let entry = codelist.entries.get(key);

        let (_, actual_comment) = entry.ok_or_else(|| CodeListError::entry_not_found(key))?;

        assert_eq!(actual_comment.as_deref(), Some(expected_comment));

        Ok(())
    }

    #[test]
    fn test_add_comment_when_comment_already_exists() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let key = "A48.51";
        let error = codelist.add_comment(key.to_string(), "test".to_string()).unwrap_err();
        let error_string = error.to_string();
        let entry = codelist.entries.get(key);
        let (_, actual_comment) = entry.ok_or_else(|| CodeListError::entry_not_found(key))?;

        assert_eq!(actual_comment.as_deref(), Some("test comment"));
        assert_eq!(
            &error_string,
            "Comment for entry with code A48.51 already exists. Please use update comment instead."
        );

        Ok(())
    }

    #[test]
    fn test_update_comment_when_comment_already_exists() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let expected_comment = "new test comment";
        let key = "A48.51";
        codelist.update_comment(key.to_string(), expected_comment.to_string())?;

        let entry = codelist.entries.get(key);

        let (_, actual_comment) = entry.ok_or_else(|| CodeListError::entry_not_found(key))?;

        assert_eq!(actual_comment.as_deref(), Some(expected_comment));

        Ok(())
    }

    #[test]
    fn test_update_comment_when_comment_does_not_already_exist() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let key = "R65.2";
        let error = codelist.update_comment(key.to_string(), "test".to_string()).unwrap_err();
        let error_string = error.to_string();
        let entry = codelist.entries.get(key);
        let (_, actual_comment) = entry.ok_or_else(|| CodeListError::entry_not_found(key))?;

        assert_eq!(actual_comment.as_deref(), None);
        assert_eq!(
            &error_string,
            "Comment for entry with code R65.2 does not exist. Please use add comment instead."
        );

        Ok(())
    }

    #[test]
    fn test_remove_comment_when_comment_exists() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let key = "A48.51";
        codelist.remove_comment(key.to_string())?;
        let entry = codelist.entries.get(key);
        let (_, comment) = entry.ok_or_else(|| CodeListError::entry_not_found(key))?;

        assert_eq!(comment.as_deref(), None);

        Ok(())
    }

    #[test]
    fn test_remove_comment_when_comment_does_not_already_exist() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let key = "R65.2";
        let error = codelist.remove_comment(key.to_string()).unwrap_err();
        let error_string = error.to_string();
        let entry = codelist.entries.get(key);
        let (_, actual_comment) = entry.ok_or_else(|| CodeListError::entry_not_found(key))?;

        assert_eq!(actual_comment.as_deref(), None);
        assert_eq!(
            &error_string,
            "Comment for entry with code R65.2 does not exist. Unable to remove comment."
        );

        Ok(())
    }

    #[test]
    fn test_add_term_when_term_does_not_exist() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let expected_term = "test term";
        let key = "R65.2";
        codelist.add_term(key.to_string(), expected_term.to_string())?;

        let entry = codelist.entries.get(key);

        let (actual_term, _) = entry.ok_or_else(|| CodeListError::entry_not_found(key))?;

        assert_eq!(actual_term.as_deref(), Some(expected_term));

        Ok(())
    }

    #[test]
    fn test_add_term_when_term_already_exists() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let key = "A48.51";
        let error = codelist.add_term(key.to_string(), "test".to_string()).unwrap_err();
        let error_string = error.to_string();
        let entry = codelist.entries.get(key);
        let (actual_term, _) = entry.ok_or_else(|| CodeListError::entry_not_found(key))?;

        assert_eq!(actual_term.as_deref(), Some("Infant botulism"));
        assert_eq!(
            &error_string,
            "Term for entry with code A48.51 already exists. Please use update term instead."
        );

        Ok(())
    }

    #[test]
    fn test_update_term_when_term_already_exists() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let expected_term = "new test term";
        let key = "A48.51";
        codelist.update_term(key.to_string(), expected_term.to_string())?;

        let entry = codelist.entries.get(key);

        let (actual_term, _) = entry.ok_or_else(|| CodeListError::entry_not_found(key))?;

        assert_eq!(actual_term.as_deref(), Some(expected_term));

        Ok(())
    }

    #[test]
    fn test_update_term_when_term_does_not_already_exist() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let key = "R65.2";
        let error = codelist.update_term(key.to_string(), "test".to_string()).unwrap_err();
        let error_string = error.to_string();
        let entry = codelist.entries.get(key);
        let (actual_term, _) = entry.ok_or_else(|| CodeListError::entry_not_found(key))?;

        assert_eq!(actual_term.as_deref(), None);
        assert_eq!(
            &error_string,
            "Term for entry with code R65.2 does not exist. Please use add term instead."
        );

        Ok(())
    }

    #[test]
    fn test_truncate_to_3_digits_snomed() -> Result<(), CodeListError> {
        let mut snomed_codelist = CodeList::new(
            "test_codelist".to_string(),
            CodeListType::SNOMED,
            Default::default(),
            None,
        );

        // A SNOMED list is not truncatable
        assert!(snomed_codelist.truncate_to_3_digits(TermManagement::First).is_err());

        Ok(())
    }

    #[test]
    fn test_truncate_to_3_digits_icd10_4_digits_drop_term() -> Result<(), CodeListError> {
        let metadata: Metadata = Default::default();

        let mut expected_codelist =
            CodeList::new("test_codelist".to_string(), CodeListType::ICD10, metadata.clone(), None);
        expected_codelist.add_entry(
            "B01".to_string(),
            None,
            Some("Truncated to 3 digits, term discarded".to_string()),
        )?;

        let mut observed_codelist =
            CodeList::new("test_codelist".to_string(), CodeListType::ICD10, metadata, None);

        observed_codelist.add_entry(
            "B012".to_string(),
            Some("Varicella pneumonia".to_string()),
            None,
        )?;

        observed_codelist.truncate_to_3_digits(TermManagement::DropTerm)?;

        assert_eq!(observed_codelist, expected_codelist);

        Ok(())
    }

    #[test]
    fn test_truncate_to_3_digits_3_and_4_digits_drop_term() -> Result<(), CodeListError> {
        let metadata: Metadata = Default::default();

        let mut expected_codelist =
            CodeList::new("test_codelist".to_string(), CodeListType::ICD10, metadata.clone(), None);
        expected_codelist.add_entry(
            "B01".to_string(),
            Some("Varicella [chickenpox]".to_string()),
            None,
        )?;

        let mut observed_codelist =
            CodeList::new("test_codelist".to_string(), CodeListType::ICD10, metadata, None);

        observed_codelist.add_entry(
            "B01".to_string(),
            Some("Varicella [chickenpox]".to_string()),
            None,
        )?;
        observed_codelist.add_entry(
            "B012".to_string(),
            Some("Varicella pneumonia".to_string()),
            None,
        )?;

        observed_codelist.truncate_to_3_digits(TermManagement::DropTerm)?;

        assert_eq!(observed_codelist, expected_codelist);

        Ok(())
    }

    #[test]
    fn test_truncate_to_3_digits_icd10_4_digits_first() -> Result<(), CodeListError> {
        let metadata: Metadata = Default::default();

        let mut expected_codelist =
            CodeList::new("test_codelist".to_string(), CodeListType::ICD10, metadata.clone(), None);
        expected_codelist.add_entry(
            "B01".to_string(),
            Some("Varicella pneumonia".to_string()),
            Some("B012 truncated to 3 digits, term first encountered".to_string()),
        )?;

        let mut observed_codelist =
            CodeList::new("test_codelist".to_string(), CodeListType::ICD10, metadata, None);

        observed_codelist.add_entry(
            "B012".to_string(),
            Some("Varicella pneumonia".to_string()),
            None,
        )?;

        observed_codelist.truncate_to_3_digits(TermManagement::First)?;

        assert_eq!(observed_codelist, expected_codelist);

        Ok(())
    }

    #[test]
    fn test_truncate_to_3_digits_3_and_4_digits_first() -> Result<(), CodeListError> {
        let metadata: Metadata = Default::default();

        let mut expected_codelist =
            CodeList::new("test_codelist".to_string(), CodeListType::ICD10, metadata.clone(), None);
        expected_codelist.add_entry(
            "B01".to_string(),
            Some("Varicella [chickenpox]".to_string()),
            None,
        )?;

        let mut observed_codelist =
            CodeList::new("test_codelist".to_string(), CodeListType::ICD10, metadata, None);

        observed_codelist.add_entry(
            "B01".to_string(),
            Some("Varicella [chickenpox]".to_string()),
            None,
        )?;
        observed_codelist.add_entry(
            "B012".to_string(),
            Some("Varicella pneumonia".to_string()),
            None,
        )?;

        observed_codelist.truncate_to_3_digits(TermManagement::First)?;

        assert_eq!(observed_codelist, expected_codelist);

        Ok(())
    }

    #[test]
    fn test_add_x_codes_icd10() -> Result<(), CodeListError> {
        let mut expected_codelist = CodeList::new(
            "test_codelist".to_string(),
            CodeListType::ICD10,
            Default::default(),
            None,
        );
        expected_codelist.add_entry("A10".to_string(), Some("Cholera".to_string()), None)?;

        expected_codelist.add_entry(
            "B01".to_string(),
            Some("Typhoid and paratyphoid fevers".to_string()),
            None,
        )?;

        expected_codelist.add_entry("B0111".to_string(), Some("TB".to_string()), None)?;

        let mut observed_codelist = expected_codelist.clone();

        expected_codelist.add_entry("A10X".to_string(), Some("Cholera".to_string()), None)?;

        expected_codelist.add_entry(
            "B01X".to_string(),
            Some("Typhoid and paratyphoid fevers".to_string()),
            None,
        )?;

        observed_codelist.add_x_codes()?;

        assert_eq!(observed_codelist, expected_codelist);

        Ok(())
    }

    #[test]
    fn test_add_x_codes_icd10_exists() -> Result<(), CodeListError> {
        let mut expected_codelist = CodeList::new(
            "test_codelist".to_string(),
            CodeListType::ICD10,
            Default::default(),
            None,
        );
        expected_codelist.add_entry("A10".to_string(), Some("Cholera".to_string()), None)?;

        expected_codelist.add_entry(
            "B01".to_string(),
            Some("Typhoid and paratyphoid fevers".to_string()),
            None,
        )?;

        expected_codelist.add_entry(
            "B01X".to_string(),
            Some("Varicella [chickenpox]".to_string()),
            None,
        )?;

        expected_codelist.add_entry("B0111".to_string(), Some("TB".to_string()), None)?;

        let mut observed_codelist = expected_codelist.clone();

        expected_codelist.add_entry("A10X".to_string(), Some("Cholera".to_string()), None)?;

        observed_codelist.add_x_codes()?;

        assert_eq!(observed_codelist, expected_codelist);

        Ok(())
    }

    #[test]
    fn test_add_x_codes_snomed() -> Result<(), CodeListError> {
        let mut snomed_codelist = CodeList::new(
            "test_codelist".to_string(),
            CodeListType::SNOMED,
            Default::default(),
            None,
        );

        // A SNOMED list is not x_appendable
        assert!(snomed_codelist.add_x_codes().is_err());

        Ok(())
    }

    #[test]
    fn test_remove_term_when_term_exists() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let key = "A48.51";
        codelist.remove_term(key.to_string())?;
        let entry = codelist.entries.get(key);
        let (term, _) = entry.ok_or_else(|| CodeListError::entry_not_found(key))?;

        assert_eq!(term.as_deref(), None);

        Ok(())
    }

    #[test]
    fn test_remove_term_when_term_does_not_already_exist() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        let key = "R65.2";
        let error = codelist.remove_term(key.to_string()).unwrap_err();
        let error_string = error.to_string();
        let entry = codelist.entries.get(key);
        let (actual_term, _) = entry.ok_or_else(|| CodeListError::entry_not_found(key))?;

        assert_eq!(actual_term.as_deref(), None);
        assert_eq!(
            &error_string,
            "Term for entry with code R65.2 does not exist. Unable to remove term."
        );

        Ok(())
    }

    #[test]
    fn get_full_entries() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let entries = codelist.full_entries();
        let expected_term1 = None;
        let expected_comment1 = None;
        let expected_term2 = "Infant botulism";
        let expected_comment2 = "test comment";
        let key1 = "R65.2".to_string();
        let key2 = "A48.51".to_string();

        let entry1 = entries.get(&key1);
        let entry2 = entries.get(&key2);

        let (term1, comment1) = entry1.ok_or_else(|| CodeListError::entry_not_found(&key1))?;
        let (term2, comment2) = entry2.ok_or_else(|| CodeListError::entry_not_found(&key2))?;

        assert_eq!(entries.len(), 2);
        assert_eq!(term1.as_deref(), expected_term1);
        assert_eq!(comment1.as_deref(), expected_comment1);
        assert_eq!(term2.as_deref(), Some(expected_term2));
        assert_eq!(comment2.as_deref(), Some(expected_comment2));

        Ok(())
    }
}
