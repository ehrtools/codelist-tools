//! This file contains the Code Entry model
//!
//! A code entry is a pair of code and term. For example, in the ICD-10 codelist, the code
//! is the ICD-10 code and the term is the description of the code. like 'B29.0' and
//! 'Acute viral hepatitis C'.

use serde::{Deserialize, Serialize};

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
    /// * `CodeEntry` - The code entry
    pub fn new(code: String, term: String) -> CodeEntry {
        CodeEntry {
            code,
            term,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_code_entry() {
        let entry = CodeEntry::new("R65.2".to_string(), "Severe sepsis".to_string());
        assert_eq!(entry.code, "R65.2".to_string());
        assert_eq!(entry.term, "Severe sepsis".to_string());
    }
}