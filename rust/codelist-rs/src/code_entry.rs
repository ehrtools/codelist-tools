//! This file contains the Code Entry model
//!
//! A code entry is a pair of code and term. For example, in the ICD-10 codelist, the code
//! is the ICD-10 code and the term is the description of the code. like 'B29.0' and
//! 'Acute viral hepatitis C'.

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
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq, Hash)]
pub struct CodeEntry {
    pub code: String,
    pub term: String,
}


impl CodeEntry {
    /// Create a new code entry
    ///
    /// # Arguments
    /// * `code` - The code
    /// * `term` - The term
    ///
    /// # Returns
    /// * `CodeEntry` - The code entry or a CodeListError
    ///
    /// # Errors
    /// * `CodeListError::EmptyCode` - If the code is an empty string
    /// * `CodeListError::EmptyTerm` - If the term is an empty string
    
    pub fn new<T: Into<String>>(code: T, term: String) -> Result<CodeEntry, CodeListError> {
        let code = code.into();

        if code.trim().is_empty() {
            return Err(CodeListError::empty_code("Empty code supplied"));
        }
        if term.trim().is_empty() {
            return Err(CodeListError::empty_term("Empty term supplied"));
        }

        Ok(CodeEntry {
            code,
            term,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_code_entry() -> Result<(), CodeListError> {
        let entry = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string())?;
        assert_eq!(entry.code, "R65.2");
        assert_eq!(entry.term, "Severe sepsis");
        Ok(())
    }

    #[test]
    fn test_empty_code_returns_error() -> Result<(), CodeListError> {
        let error = CodeEntry::new("".to_string(), "Severe sepsis".to_string()).unwrap_err();
        assert!(matches!(error, CodeListError::EmptyCode { msg } if msg == "Empty code supplied"));
        Ok(())
    }

    #[test]
    fn test_empty_term_returns_error() -> Result<(), CodeListError> {
        let error = CodeEntry::new("R65.2".to_string(), "".to_string()).unwrap_err();
        assert!(matches!(error, CodeListError::EmptyTerm { msg } if msg == "Empty term supplied"));
        Ok(())
    }

    #[test]
    fn test_whitespace_only_code_returns_error() -> Result<(), CodeListError> {
        let error = CodeEntry::new("   ".to_string(), "Some term".to_string()).unwrap_err();
        assert!(matches!(error, CodeListError::EmptyCode { msg } if msg == "Empty code supplied"));
        Ok(())
    }

    #[test]
    fn test_whitespace_only_term_returns_error() -> Result<(), CodeListError> {
        let error = CodeEntry::new("R65.2".to_string(), "  \n\t  ".to_string()).unwrap_err();
        assert!(matches!(error, CodeListError::EmptyTerm { msg } if msg == "Empty term supplied"));
        Ok(())
    }

}
