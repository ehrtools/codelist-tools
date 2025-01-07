use serde::{Serialize, Deserialize};

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
    pub add_x_codes: bool,
    pub code_column_name: String, // for csv files
    pub term_column_name: String, // for csv files          
    pub code_field_name: String, // for json files
    pub term_field_name: String, // for json files

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
            code_field_name: "code".to_string(),
            term_field_name: "term".to_string(),
        }
    }
}