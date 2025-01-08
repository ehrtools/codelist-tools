//! This file contains the Code Entry model
//!
//! A code entry is a pair of code and term. For example, in the ICD-10 codelist, the code
//! is the ICD-10 code and the term is the description of the code. like 'B29.0' and
//! 'Acute viral hepatitis C'.

use csv::DeserializeErrorKind::ParseFloat;
// External imports
use serde::{Deserialize, Serialize};

// Internal imports
use crate::errors::CodeListError;

/// Struct to represent a code entry
///
/// This is not specific to any codelist, but is a general representation
/// of a code entry.
///
/// Fields:
/// * `code` - The code
/// * `term` - The term
/// * `comment` - An optional comment
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq, Hash)]
pub struct CodeEntry {
    pub code: String,
    pub term: String,
    pub comment: Option<String>,
}


impl CodeEntry {
    /// Create a new code entry
    ///
    /// # Arguments
    /// * `code` - The code
    /// * `term` - The term
    /// * `comment` - An optional comment
    ///
    /// # Returns
    /// * `CodeEntry` - The code entry or a CodeListError
    ///
    /// # Errors
    /// * `CodeListError::EmptyCode` - If the code is an empty string
    /// * `CodeListError::EmptyTerm` - If the term is an empty string
    
    pub fn new<T: Into<String>>(code: T, term: String, comment: Option<String>) -> Result<CodeEntry, CodeListError> {
        let code = code.into();

        if code.trim().is_empty() {
            return Err(CodeListError::empty_code());
        }
        if term.trim().is_empty() {
            return Err(CodeListError::empty_term());
        }

        Ok(CodeEntry {
            code,
            term,
            comment,
        })
    }

    /// Add a comment to the code entry
    ///
    /// # Arguments
    /// * `comment` - The comment to add
    ///
    /// # Returns
    /// * `Result<(), CodeListError>`
    ///
    /// # Errors
    /// * `CodeListError::CodeEntryCommentAlreadyExists` - If the comment already exists
    pub fn add_comment(&mut self, comment: String) -> Result<(), CodeListError> {
        if self.comment.is_none() {
            self.comment = Some(comment);
            Ok(())
        } else {
            Err(CodeListError::code_entry_comment_already_exists(&self.code, &self.term))
        }
    }

    /// Update the comment for the code entry
    ///
    /// # Arguments
    /// * `comment` - The comment to update
    ///
    /// # Returns
    /// * `Result<(), CodeListError>`
    ///
    /// # Errors
    /// * `CodeListError::CodeEntryCommentDoesNotExist` - If the comment does not exist
    pub fn update_comment(&mut self, comment: String) -> Result<(), CodeListError> {
        if let Some(_x) = &self.comment {
            self.comment = Some(comment);
            Ok(())
        } else {
            Err(CodeListError::code_entry_comment_does_not_exist(&self.code, &self.term))
        }
    }

    /// Remove the comment for the code entry
    ///
    /// # Returns
    /// * `Result<(), CodeListError>`
    ///
    /// # Errors
    /// * `CodeListError::CodeEntryCommentDoesNotExist` - If the comment does not exist
    pub fn remove_comment(&mut self) -> Result<(), CodeListError> {
        if let Some(_x) = &self.comment {
            self.comment = None;
            Ok(())
        } else {
            Err(CodeListError::code_entry_comment_does_not_exist(&self.code, &self.term))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_code_entry_no_comment() -> Result<(), CodeListError> {
        let entry = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string(), None)?;
        assert_eq!(entry.code, "R65.2");
        assert_eq!(entry.term, "Severe sepsis");
        assert_eq!(entry.comment, None);
        Ok(())
    }

    #[test]
    fn test_creating_code_entry_with_comment() -> Result<(), CodeListError> {
        let entry = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string(), Some("Test comment".to_string()))?;
        assert_eq!(entry.code, "R65.2");
        assert_eq!(entry.term, "Severe sepsis");
        assert_eq!(entry.comment, Some("Test comment".to_string()));
        Ok(())
    }


    #[test]
    fn test_empty_code_returns_error() -> Result<(), CodeListError> {
        let error = CodeEntry::new("".to_string(), "Severe sepsis".to_string(), None).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(&error_string, "Empty code supplied");
        Ok(())
    }

    #[test]
    fn test_empty_term_returns_error() -> Result<(), CodeListError> {
        let error = CodeEntry::new("R65.2".to_string(), "".to_string(), None).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(&error_string, "Empty term supplied");
        Ok(())
    }

    #[test]
    fn test_whitespace_only_code_returns_error() -> Result<(), CodeListError> {
        let error = CodeEntry::new("   ".to_string(), "Some term".to_string(), None).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(&error_string, "Empty code supplied");
        Ok(())
    }

    #[test]
    fn test_whitespace_only_term_returns_error() -> Result<(), CodeListError> {
        let error = CodeEntry::new("R65.2".to_string(), "  \n\t  ".to_string(), None).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(&error_string, "Empty term supplied");
        Ok(())
    }

    #[test]
    fn test_add_comment_when_comment_does_not_exist() -> Result<(), CodeListError> {
        let mut entry = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string(), None)?;
        let comment = "Test comment";
        assert!(entry.add_comment(comment.to_string()).is_ok());
        assert_eq!(entry.comment, Some("Test comment".to_string()));
        Ok(())
    }

    #[test]
    fn test_add_comment_when_comment_already_exists() -> Result<(), CodeListError> {
        let mut entry = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string(), Some("test".to_string()))?;
        let comment = "Test comment";
        let error = entry.add_comment(comment.to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Comment for CodeEntry with code R65.2 and term Severe sepsis already exists. Please update comment instead.");

        Ok(())
    }

    #[test]
    fn test_update_comment_when_comment_already_exists() -> Result<(), CodeListError> {
        let mut entry = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string(), Some("test".to_string()))?;
        let comment = "Test comment";
        let result = entry.update_comment(comment.to_string());
        assert!(result.is_ok());
        assert_eq!(entry.comment, Some("Test comment".to_string()));
        Ok(())
    }

    #[test]
    fn test_update_comment_when_comment_does_not_already_exist() -> Result<(), CodeListError> {
        let mut entry = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string(), None)?;
        let comment = "Test comment";
        let error = entry.update_comment(comment.to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Comment for CodeEntry with code R65.2 and term Severe sepsis does not exist. Please use add comment instead if you are trying to add a comment.");
        Ok(())
    }

    #[test]
    fn test_remove_comment_when_comment_exists() -> Result<(), CodeListError> {
        let mut entry = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string(), Some("test".to_string()))?;
        let result = entry.remove_comment();
        assert!(result.is_ok());
        assert_eq!(entry.comment, None);
        Ok(())
    }

    #[test]
    fn test_remove_comment_when_comment_does_not_already_exist() -> Result<(), CodeListError> {
        let mut entry = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string(), None)?;
        let error = entry.remove_comment().unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Comment for CodeEntry with code R65.2 and term Severe sepsis does not exist. Please use add comment instead if you are trying to add a comment.");
        Ok(())
    }
}