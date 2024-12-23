use crate::errors::CodeListError;
use crate::codelist::CodeList;
use crate::codelist::CodeListOptions;
use crate::metadata::{Metadata, MetadataSource};
use crate::types::CodeListType;

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
    /// * `CodeListError::CSVError` - If there is an error reading the CSV file
    /// * `CodeListError::InvalidCodeColumnName` - If the provided code column name in codelist_options is invalid
    /// * `CodeListError::InvalidTermColumnName` - If the provided term column name in codelist_options is invalid
    /// * `CodeListError::CodeNotFound` - If the code is not found in the row
    /// * `CodeListError::TermNotFound` - If the term is not found in the row
    /// * `CodeListError::EmptyCode` - If the code is an empty string
    /// * `CodeListError::EmptyTerm` - If the term is an empty string
    pub fn load_codelist_from_csv_file(&self, file_path: &str) -> Result<CodeList, CodeListError> {
        let mut rdr = csv::Reader::from_path(file_path)?;
        let headers = rdr.headers()?;
        let mut codelist = CodeList::new(self.codelist_type.clone(), self.metadata.clone(), Some(self.codelist_options.clone()));
        
        // Check for duplicate headers
        let code_column: Vec<_> = headers.iter()
            .enumerate()
            .filter(|(_, h)| *h == self.codelist_options.code_column_name)
            .collect();
        let term_column: Vec<_> = headers.iter()
            .enumerate()
            .filter(|(_, h)| *h == self.codelist_options.term_column_name)
            .collect();
        
        if code_column.len() > 1 {
            return Err(CodeListError::InvalidCodeColumnName(format!("Multiple columns found with the header: {}", self.codelist_options.code_column_name)));
        }
        if term_column.len() > 1 {
            return Err(CodeListError::InvalidTermColumnName(format!("Multiple columns found with the header: {}", self.codelist_options.term_column_name)));
        }

        // Get the index of the code column
        let code_idx = code_column.first()
            .map(|(idx, _)| *idx)
            .ok_or_else(|| CodeListError::InvalidCodeColumnName(format!("Column not found with the header: {}", self.codelist_options.code_column_name)))?;

        // Get the index of the term column
        let term_idx = term_column.first()
            .map(|(idx, _)| *idx)
            .ok_or_else(|| CodeListError::InvalidTermColumnName(format!("Column not found with the header: {}", self.codelist_options.term_column_name)))?;

        // Iterate over the records in the code and term columns and add the code and term to the codelist
        for (row_num, result) in rdr.records().enumerate() {
            let record = result?;
            let code = record.get(code_idx)
                .ok_or_else(|| CodeListError::CodeNotFound(format!("Code not found in row: {}", row_num + 1)))?
                .trim();
            if code.is_empty() {
                return Err(CodeListError::CodeNotFound(format!("Empty code field in row: {}", row_num + 1)));
            }
            let term = record.get(term_idx)
                .ok_or_else(|| CodeListError::TermNotFound(format!("Term not found in row: {}", row_num + 1)))?
                .trim();
            if term.is_empty() {
                return Err(CodeListError::TermNotFound(format!("Empty term field in row: {}", row_num + 1)));
            }
            codelist.add_entry(code.to_string(), term.to_string())?;
        }
        
        Ok(codelist)
    }

    /// Load a codelist from a json file
    ///
    /// # Arguments
    /// * `file_path` - The path to the json file
    /// 
    /// # Returns
    /// * `Result<CodeList, CodeListError>` - The codelist or an error
    /// 
    /// # Errors
    pub fn load_codelist_from_json_file(&self, file_path: &str) -> Result<CodeList, CodeListError> {
        let mut codelist = CodeList::new(self.codelist_type.clone(), self.metadata.clone(), Some(self.codelist_options.clone()));

        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        let json_data: serde_json::Value = serde_json::from_reader(reader)?;

        if let Some(entries) = json_data.as_array() {
            for (index, entry) in entries.iter().enumerate() {
                let code = entry[&self.codelist_options.code_column_name]
                    .as_str()
                    .ok_or_else(|| CodeListError::CodeNotFound(format!("Code not found in json file at index: {}", index + 1)))?
                    .trim();
                
                let term = entry[&self.codelist_options.term_column_name]
                    .as_str()
                    .ok_or_else(|| CodeListError::TermNotFound(format!("Term not found in json file at index: {}", index + 1)))?
                    .trim();

                if code.is_empty() {
                    return Err(CodeListError::CodeNotFound(format!("Empty code field in json file at index: {}", index + 1)));
                }
                if term.is_empty() {
                    return Err(CodeListError::TermNotFound(format!("Empty term field in json file at index: {}", index + 1)));
                }
                codelist.add_entry(code.to_string(), term.to_string())?;
            }
        } else {
            return Err(CodeListError::InvalidInput("JSON must be an array of objects".to_string()));
        }
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
            let path_str = path.to_str()
                .ok_or_else(|| CodeListError::InvalidFilePath)?;
            let codelist = self.load_codelist_from_file(path_str)?;
            codelists.push(codelist);
        }
        Ok(codelists)
    }

    pub fn load_codelists(&self, codelists: Option<Vec<CodeList>>, path: Option<String>) -> Result<Vec<CodeList>, CodeListError> {
        match (codelists, path) {
            (Some(codelist), None) => Ok(codelist),
            (None, Some(folder_path)) => self.load_codelists_from_folder(folder_path),
            (None, None) => Err(CodeListError::InvalidInput("Codelist vector or path must be provided".to_string())),
            (Some(_), Some(_)) => Err(CodeListError::InvalidInput("Both codelist vector and path cannot be provided".to_string())),
        }
    }

    pub fn process_codelists(&self) {
        println!("We will process the codelists here.")
    }

    // currently saving files as numbers
    pub fn save_codelists_to_json(&self, folder_path: &str, codelists: Vec<CodeList>) -> Result<(), CodeListError> {
        for (index, codelist) in codelists.iter().enumerate() {
            let filename = format!("{}.json", index + 1);
            let full_path = std::path::Path::new(folder_path).join(filename);
            let path_str = full_path.to_str()
                .ok_or_else(|| CodeListError::InvalidFilePath)?;
            codelist.save_to_json(path_str)?;
        }
        Ok(())
    }

    pub fn save_codelists_to_csv(&self, folder_path: &str, codelists: Vec<CodeList>) -> Result<(), CodeListError> {
        for (index, codelist) in codelists.iter().enumerate() {
            let filename = format!("{}.csv", index + 1);
            let full_path = std::path::Path::new(folder_path).join(filename);
            let path_str = full_path.to_str()
                .ok_or_else(|| CodeListError::InvalidFilePath)?;
            codelist.save_to_csv(path_str)?;
        }
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    // Helper function to create test metadata
    fn create_test_metadata() -> Metadata {
        Metadata {
            source: MetadataSource::ManuallyCreated,
            authors: Some(vec!["Caroline Morton".to_string()]),
            version: Some("2024-12-19".to_string()),
            description: Some("A test codelist".to_string()),
        }
    }

    fn create_test_codelist_factory() -> CodeListFactory {
        let metadata = create_test_metadata();
        let codelist_type = CodeListType::ICD10;
        let codelist_options = CodeListOptions::default();
        CodeListFactory::new(codelist_options, metadata, codelist_type)
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

    #[test]
    fn test_load_codelist_from_csv_file() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_codelist.csv");
        let file_path_str = file_path.to_str()
            .ok_or_else(|| CodeListError::InvalidFilePath)?;

        // Create test CSV content
        let csv_content = "\
code,term,description
A01,Test Disease 1,Description 1
B02,Test Disease 2,Description 2
C03,Test Disease 3,Description 3";

        fs::write(&file_path, csv_content)?;
        let factory = create_test_codelist_factory();
        
        let result = factory.load_codelist_from_csv_file(file_path_str);
        assert!(result.is_ok());
        let codelist = result?;
        assert_eq!(codelist.entries.len(), 3);
        
        // Test individual entries exist
        assert_eq!(codelist.entries.iter().find(|e| e.code == "A01" && e.term == "Test Disease 1").is_some(), true);
        assert_eq!(codelist.entries.iter().find(|e| e.code == "B02" && e.term == "Test Disease 2").is_some(), true);
        assert_eq!(codelist.entries.iter().find(|e| e.code == "C03" && e.term == "Test Disease 3").is_some(), true);
        
        assert_eq!(codelist.codelist_options.allow_duplicates, false);
        assert_eq!(codelist.codelist_options.truncate_to_3_digits, false);
        assert_eq!(codelist.codelist_options.add_x_codes, false);
        assert_eq!(codelist.codelist_options.code_column_name, "code".to_string());
        assert_eq!(codelist.codelist_options.term_column_name, "term".to_string());
        assert_eq!(codelist.metadata.source, MetadataSource::ManuallyCreated);
        assert_eq!(codelist.metadata.authors, Some(vec!["Caroline Morton".to_string()]));
        assert_eq!(codelist.metadata.version, Some("2024-12-19".to_string()));
        assert_eq!(codelist.metadata.description, Some("A test codelist".to_string()));
        assert_eq!(codelist.codelist_type, CodeListType::ICD10);

        Ok(())
    }

    #[test]
    fn test_load_codelist_from_csv_file_invalid_term_column_name() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_codelist.json");
        let file_path_str = file_path.to_str()
            .ok_or_else(|| CodeListError::InvalidFilePath)?;

        // Create test CSV content
        let csv_content = "\
code,term_test,description_test
A01,Test Disease 1,Description 1
B02,Test Disease 2,Description 2
C03,Test Disease 3,Description 3";

        fs::write(&file_path, csv_content)?;
        let factory = create_test_codelist_factory();
        
        let error = factory.load_codelist_from_csv_file(file_path_str).unwrap_err();

        assert!(matches!(error, CodeListError::InvalidTermColumnName(msg) if msg == format!("Column not found with the header: {}", factory.codelist_options.term_column_name)));

        Ok(())
    }

    #[test]
    fn test_load_codelist_from_csv_file_invalid_code_column_name() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_codelist.json");
        let file_path_str = file_path.to_str()
            .ok_or_else(|| CodeListError::InvalidFilePath)?;

        // Create test CSV content
        let csv_content = "\
code_test,term,description_test
A01,Test Disease 1,Description 1
B02,Test Disease 2,Description 2
C03,Test Disease 3,Description 3";

        fs::write(&file_path, csv_content)?;
        let factory = create_test_codelist_factory();
        
        let error = factory.load_codelist_from_csv_file(file_path_str).unwrap_err();

        assert!(matches!(error, CodeListError::InvalidCodeColumnName(msg) if msg == format!("Column not found with the header: {}", factory.codelist_options.code_column_name)));

        Ok(())
    }

    #[test]
    fn test_load_codelist_from_csv_file_empty_code() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_codelist.csv");
        let file_path_str = file_path.to_str()
            .ok_or_else(|| CodeListError::InvalidFilePath)?;

        // Create CSV with empty code
        let csv_content = "\
code,term,description
,Test Disease 1,Description 1
B02,Test Disease 2,Description 2";

        fs::write(&file_path, csv_content)?;
        let factory = create_test_codelist_factory();
        
        let error = factory.load_codelist_from_csv_file(file_path_str).unwrap_err();
        assert!(matches!(error, CodeListError::CodeNotFound(msg) if msg.contains("Empty code field in row: 1")));

        Ok(())
    }

    #[test]
    fn test_load_codelist_from_csv_file_empty_term() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_codelist.csv");
        let file_path_str = file_path.to_str()
            .ok_or_else(|| CodeListError::InvalidFilePath)?;

        // Create CSV with empty term
        let csv_content = "\
code,term,description
A01,,Description 1
B02,Test Disease 2,Description 2";

        fs::write(&file_path, csv_content)?;
        let factory = create_test_codelist_factory();
        
        let error = factory.load_codelist_from_csv_file(file_path_str).unwrap_err();
        assert!(matches!(error, CodeListError::TermNotFound(msg) if msg.contains("Empty term field in row: 1")));

        Ok(())
    }

    #[test]
    fn test_load_codelist_from_csv_file_missing_code() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_codelist.csv");
        let file_path_str = file_path.to_str()
            .ok_or_else(|| CodeListError::InvalidFilePath)?;

        // Create CSV with missing code column
        let csv_content = "\
term,description
Test Disease 1,Description 1
Test Disease 2,Description 2";

        fs::write(&file_path, csv_content)?;
        let factory = create_test_codelist_factory();
        
        let error = factory.load_codelist_from_csv_file(file_path_str).unwrap_err();
        assert!(matches!(error, CodeListError::InvalidCodeColumnName(msg) if msg.contains("Column not found with the header: code")));

        Ok(())
    }

    #[test]
    fn test_load_codelist_from_csv_file_missing_term() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_codelist.csv");
        let file_path_str = file_path.to_str()
            .ok_or_else(|| CodeListError::InvalidFilePath)?;

        // Create CSV with missing term column
        let csv_content = "\
code,description
A01,Description 1
B02,Description 2";

        fs::write(&file_path, csv_content)?;
        let factory = create_test_codelist_factory();
        
        let error = factory.load_codelist_from_csv_file(file_path_str).unwrap_err();
        assert!(matches!(error, CodeListError::InvalidTermColumnName(msg) if msg.contains("Column not found with the header: term")));

        Ok(())
    }
}
