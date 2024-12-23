use crate::errors::CodeListError;
use crate::codelist::CodeList;
use crate::codelist::CodeListOptions;
use crate::metadata::{Metadata, MetadataSource};
use crate::types::CodeListType;
use csv::Reader;
use std::fs::File;
use serde::Deserialize;

/// Struct to represent a codelist factory, which is used to load codelists from a directory and make sure all codelists are created following the same rules
///
/// # Fields
/// * `codelist_options` - The options for the codelist
/// * `metadata` - The metadata for the codelist
/// * `codelist_type` - The type of codelist
pub struct CodeListFactory {
    pub codelist_options: CodeListOptions,
    pub metadata: Metadata,
    pub codelist_type: CodeListType,
}

impl CodeListFactory {
    /// Create a new codelist factory
    ///
    /// # Arguments
    /// * `codelist_options` - The options for the codelist
    /// * `metadata` - The metadata for the codelist
    /// * `codelist_type` - The type of codelist
    pub fn new(codelist_options: CodeListOptions, metadata: Metadata, codelist_type: CodeListType) -> Self {
        CodeListFactory {
            codelist_options: codelist_options.clone(),
            metadata: metadata.clone(),
            codelist_type: codelist_type.clone(),
        }
    }

    /// Load a codelist from a csv file
    ///
    /// # Arguments
    /// * `file_path` - The path to the csv file
    /// 
    /// # Returns
    /// * `Result<CodeList, CodeListError>` - The codelist or an error
    /// 
    /// # Errors
    /// * `CodeListError::InvalidFilePath` - If the file path is not a valid file
    pub fn load_codelist_from_csv_file(&self, file_path: &str) -> Result<CodeList, CodeListError> {
        let codelist = CodeList::new(self.codelist_type.clone(), self.metadata.clone(), Some(self.codelist_options.clone()));
        Ok(codelist)
    }

    pub fn load_codelist_from_json_file(&self, file_path: &str) -> Result<CodeList, CodeListError> {
        let codelist = CodeList::new(self.codelist_type.clone(), self.metadata.clone(), Some(self.codelist_options.clone()));
        Ok(codelist)
    }

    pub fn load_codelist_from_file(&self, file_path: &str) -> Result<CodeList, CodeListError> {
        match std::path::Path::new(file_path).extension() {
            Some(ext) if ext == "csv" => self.load_codelist_from_csv_file(file_path),
            Some(ext) if ext == "json" => self.load_codelist_from_json_file(file_path),
            _ => Err(CodeListError::InvalidFilePath),
        }
    }

    pub fn load_codelists_from_folder(&self, folder_path: String) -> Result<Vec<CodeList>, CodeListError> {
        let dir = std::fs::read_dir(folder_path)?;
        let mut codelists: Vec<CodeList> = Vec::new();

        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            if let Some(path_str) = path.to_str() {
                if let Ok(codelist) = self.load_codelist_from_file(path_str) {
                    codelists.push(codelist);
                }
            }
        }
        Ok(codelists)
    }

    pub fn load_codelists(&self, codelists: Option<Vec<CodeList>>, path: Option<String>) -> Result<Vec<CodeList>, CodeListError> {
        match (codelists, path) {
            (Some(list), None) => Ok(list),  // Direct loading of provided codelists
            (None, Some(folder_path)) => self.load_codelists_from_folder(folder_path),
            (None, None) => Ok(Vec::new()),
            (Some(_), Some(_)) => Err(CodeListError::InvalidInput("Both codelist vector and path cannot be provided".to_string())),
        }
    }

    pub fn process_codelists(&self) {
        println!("We will process the codelists here.")
    }

    pub fn save_codelists_to_json(&self, path: String) {

    }

    pub fn save_codelists_to_csv(&self, path: String) {

    }

    pub fn save_codelists_to_folder(&self, path: String) {

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create test metadata
    fn create_test_metadata() -> Metadata {
        Metadata {
            source: MetadataSource::ManuallyCreated,
            authors: Some(vec!["Caroline Morton".to_string()]),
            version: Some("2024-12-19".to_string()),
            description: Some("A test codelist".to_string()),
        }
    }

    #[test]
    fn test_new_codelist_factory() {
        let metadata = create_test_metadata();
        let codelist_type = CodeListType::ICD10;
        let codelist_options = CodeListOptions::default();
        let codelist_factory = CodeListFactory::new(codelist_options, metadata, codelist_type);
    
        assert_eq!(codelist_factory.codelist_options.allow_duplicates, false);
        assert_eq!(codelist_factory.codelist_options.truncate_to_3_digits, false);
        assert_eq!(codelist_factory.codelist_options.add_x_codes, false);
        assert_eq!(codelist_factory.codelist_options.code_column_name, "code".to_string());
        assert_eq!(codelist_factory.codelist_options.term_column_name, "term".to_string());

        assert_eq!(codelist_factory.metadata.source, MetadataSource::ManuallyCreated);
        assert_eq!(codelist_factory.metadata.authors, Some(vec!["Caroline Morton".to_string()]));
        assert_eq!(codelist_factory.metadata.version, Some("2024-12-19".to_string()));
        assert_eq!(codelist_factory.metadata.description, Some("A test codelist".to_string()));
        assert_eq!(codelist_factory.codelist_type, CodeListType::ICD10);
    }
}


