//! This file contains the Code Entry model
//!
//! A code entry is a pair of code and term. For example, in the ICD-10 codelist, the code
//! is the ICD-10 code and the term is the description of the code. like 'B29.0' and
//! 'Acute viral hepatitis C'.


/// Struct to represent a code entry
///
/// This is not specific to any codelist, but is a general representation
/// of a code entry.
///
/// Fields:
/// * `code` - The code
/// * `term` - The term
pub struct CodeEntry {
    code: String,
    term: String,
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