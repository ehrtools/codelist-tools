//! This file contains the codelist options for the codelist

use serde::{Deserialize, Serialize};

/// Struct to represent a codelist options
///
/// # Fields
/// * `allow_duplicates` - Whether to allow duplicates in the codelist
/// * `code_column_name` - The name of the code column
/// * `term_column_name` - The name of the term column
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CodeListOptions {
    pub allow_duplicates: bool,
    pub code_column_name: String, // for csv files
    pub term_column_name: String, // for csv files
    pub code_field_name: String,  // for json files
    pub term_field_name: String,
    pub custom_regex: Option<String>, // for custom validation
}

impl Default for CodeListOptions {
    /// Default implementation for CodeListOptions
    ///
    /// # Returns
    /// * `CodeListOptions` - The default CodeListOptions
    fn default() -> Self {
        Self {
            allow_duplicates: false,
            code_column_name: "code".to_string(),
            term_column_name: "term".to_string(),
            code_field_name: "code".to_string(),
            term_field_name: "term".to_string(),
            custom_regex: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let options = CodeListOptions::default();
        assert!(!options.allow_duplicates);
        assert_eq!(options.code_column_name, "code");
        assert_eq!(options.term_column_name, "term");
        assert_eq!(options.code_field_name, "code");
        assert_eq!(options.term_field_name, "term");
        assert_eq!(options.custom_regex, None);
    }
}
