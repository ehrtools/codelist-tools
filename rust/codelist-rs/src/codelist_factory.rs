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
    /// * `CodeListError::IOError` - If there is an error reading the file
    /// * `CodeListError::CSVError` - If there is an error parsing the CSV file
    /// * `CodeListError::InvalidCodeField` - If the code field is missing from the JSON object
    /// * `CodeListError::InvalidTermField` - If the term field is missing from the JSON object
    /// * `CodeListError::InvalidCode` - If the code value is not a valid string
    /// * `CodeListError::InvalidTerm` - If the term value is not a valid string
    /// * `CodeListError::EmptyCode` - If the code value is an empty string
    /// * `CodeListError::EmptyTerm` - If the term value is an empty string
    pub fn load_codelist_from_csv_file(&self, file_path: &str) -> Result<CodeList, CodeListError> {
        let mut rdr = csv::Reader::from_path(file_path)?;
        let headers = rdr.headers()?;
        let mut codelist = CodeList::new(self.codelist_type.clone(), self.metadata.clone(), Some(self.codelist_options.clone()));
        
        let code_column: Vec<_> = headers.iter()
            .enumerate()
            .filter(|(_, h)| *h == self.codelist_options.code_field_name)
            .collect();
        let term_column: Vec<_> = headers.iter()
            .enumerate()
            .filter(|(_, h)| *h == self.codelist_options.term_field_name)
            .collect();
        
        if code_column.len() > 1 {
            return Err(CodeListError::InvalidCodeField(format!("Multiple columns found with the header: {}", self.codelist_options.code_field_name)));
        }
        if term_column.len() > 1 {
            return Err(CodeListError::InvalidTermField(format!("Multiple columns found with the header: {}", self.codelist_options.term_field_name)));
        }

        let code_idx = code_column.first()
            .map(|(idx, _)| *idx)
            .ok_or_else(|| CodeListError::InvalidCodeField(format!("Column not found with the header: {}", self.codelist_options.code_field_name)))?;

        let term_idx = term_column.first()
            .map(|(idx, _)| *idx)
            .ok_or_else(|| CodeListError::InvalidTermField(format!("Column not found with the header: {}", self.codelist_options.term_field_name)))?;

        for (row_num, result) in rdr.records().enumerate() {
            let record = result?;
            let code = record.get(code_idx)
                .ok_or_else(|| CodeListError::ColumnIndexOutOfBounds(
                    format!("Row {}: Cannot access column at index {}.", row_num + 2, code_idx)
                ))?
                .trim();
            if code.is_empty() {
                return Err(CodeListError::EmptyCode(format!("Empty code field in row: {}", row_num + 2)));
            }
            let term = record.get(term_idx)
                .ok_or_else(|| CodeListError::ColumnIndexOutOfBounds(
                    format!("Row {}: Cannot access column at index {}.", row_num + 2, term_idx)
                ))?
                .trim();
            if term.is_empty() {
                return Err(CodeListError::EmptyTerm(format!("Empty term field in row: {}", row_num + 2)));
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
    /// * `CodeListError::IOError` - If there is an error reading the json file
    /// * `CodeListError::JSONError` - If there is an error parsing the json file
    /// * `CodeListError::InvalidCodeField` - If the code field is missing from the JSON object
    /// * `CodeListError::InvalidTermField` - If the term field is missing from the JSON object
    /// * `CodeListError::EmptyCode` - If the code value is an empty string
    /// * `CodeListError::EmptyTerm` - If the term value is an empty string
    /// * `CodeListError::InvalidCodeType` - If the code value is neither a string nor a number, or if a string code contains invalid UTF-8 characters
    /// * `CodeListError::InvalidTermType` - If the term value is not a string, or if a string term contains invalid UTF-8 characters
    /// * `CodeListError::InvalidInput` - If the JSON is not an array of objects
    /// 
    /// * Assumes that the json file is an array of objects with "code" and "term" fields 
    pub fn load_codelist_from_json_file(&self, file_path: &str) -> Result<CodeList, CodeListError> {
        let mut codelist = CodeList::new(self.codelist_type.clone(), self.metadata.clone(), Some(self.codelist_options.clone()));

        let file = std::fs::File::open(file_path)?;
        let reader = std::io::BufReader::new(file);
        let json_data: serde_json::Value = serde_json::from_reader(reader)?;

        if let Some(entries) = json_data.as_array() {
            for (index, entry) in entries.iter().enumerate() {

                let code_value = entry.get("code")
                    .ok_or_else(|| CodeListError::InvalidCodeField(
                        format!("No {} field found in json file at index: {}", self.codelist_options.code_field_name, index)
                    ))?;

                let code = if code_value.is_number() {
                    code_value.to_string().trim().to_string()
                } else if code_value.is_string() {
                    let code_str = code_value.as_str()
                        .ok_or_else(|| CodeListError::InvalidCodeType(format!("Expected string value for code at index {}, but found invalid UTF-8 string", index))
                        )?
                        .trim();
                    
                    if code_str.is_empty() {
                        return Err(CodeListError::EmptyCode(format!("Empty code at index: {}", index)));
                    }
                    
                    code_str.to_string()
                } else {
                    return Err(CodeListError::InvalidCodeType(format!("Code at index {} must be a string or number", index)));
                };

                let term_value = entry.get("term")
                    .ok_or_else(|| CodeListError::InvalidTermField(
                        format!("No {} field found in json file at index: {}", self.codelist_options.term_field_name, index)
                    ))?;

                let term = if term_value.is_string() {
                    let term_str = term_value.as_str()
                        .ok_or_else(|| CodeListError::InvalidTermType(format!("Expected string value for term at index {}, but found invalid UTF-8 string", index)))?
                        .trim();
                    
                    if term_str.is_empty() {
                        return Err(CodeListError::EmptyTerm(format!("Empty term at index: {}", index)));
                    }
                    
                    term_str.to_string()
                } else {
                    return Err(CodeListError::InvalidTermType(format!("Term at index {} must be a string", index)));
                };

                codelist.add_entry(code, term)?;
            }
        } else {
            return Err(CodeListError::InvalidInput("JSON must be an array of objects".to_string()));
        }
        Ok(codelist)
    }

    /// Load a codelist from a file
    ///
    /// # Arguments
    /// * `file_path` - The path to the file
    /// 
    /// # Returns
    /// * `Result<CodeList, CodeListError>` - The codelist or an error
    /// 
    /// # Errors
    /// * `CodeListError::InvalidFilePath` - If the file path is not a csv or json file
    pub fn load_codelist_from_file(&self, file_path: &str) -> Result<CodeList, CodeListError> {
        match std::path::Path::new(file_path).extension() {
            Some(ext) if ext == "csv" => self.load_codelist_from_csv_file(file_path),
            Some(ext) if ext == "json" => self.load_codelist_from_json_file(file_path),
            _ => Err(CodeListError::InvalidFilePath(format!("File path {} is not a csv or json file", file_path))),
        }
    }

    /// Load codelists from a folder
    ///
    /// # Arguments
    /// * `folder_path` - The path to the folder
    /// 
    /// # Returns
    /// * `Result<Vec<CodeList>, CodeListError>` - The codelists or an error
    /// 
    /// # Errors
    /// * `CodeListError::IOError` - If there is an error reading the folder
    pub fn load_codelists_from_folder(&self, folder_path: &str) -> Result<Vec<CodeList>, CodeListError> {
        let dir = std::fs::read_dir(folder_path)?;
        let mut codelists: Vec<CodeList> = Vec::new();

        for entry in dir {
            let entry = entry?;
            let path = entry.path();

            // Skips if not csv/json
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if ext == "csv" || ext == "json" {
                    if let Some(path_str) = path.to_str() {
                        if let Ok(codelist) = self.load_codelist_from_file(path_str) {
                            codelists.push(codelist);
                        }
                    }
                }
            }
        }
        Ok(codelists)
    }

    /// Load codelists from a folder or a vector of codelists
    ///
    /// # Arguments
    /// * `codelists` - The vector of codelists
    /// * `path` - The path to the folder
    /// 
    /// # Returns
    /// * `Result<Vec<CodeList>, CodeListError>` - The codelists or an error
    /// 
    /// # Errors
    /// * `CodeListError::InvalidInput` - If the codelist vector and path are both provided, or neither are provided
    pub fn load_codelists(&self, codelists: Option<Vec<CodeList>>, path: Option<&str>) -> Result<Vec<CodeList>, CodeListError> {
        match (codelists, path) {
            (Some(codelist), None) => Ok(codelist),
            (None, Some(folder_path)) => self.load_codelists_from_folder(folder_path),
            (None, None) => Err(CodeListError::InvalidInput("Codelist vector or path must be provided".to_string())),
            (Some(_), Some(_)) => Err(CodeListError::InvalidInput("Either codelist vector or path must be provided, not both".to_string())),
        }
    }

    /// Process the codelists
    /// 
    /// # Arguments
    /// * `codelists` - The vector of codelists
    /// 
    /// # Returns
    /// * `Result<Vec<CodeList>, CodeListError>` - The codelists or an error
    /// 
    /// * To be developed in the future
    pub fn process_codelists(&self, codelists: Vec<CodeList>) {
        println!("We will process the codelists here.")
    }

    /// Save the codelists to a json file
    /// 
    /// # Arguments
    /// * `folder_path` - The path to the folder
    /// * `codelists` - The vector of codelists
    /// 
    /// # Returns
    /// * `Result<(), CodeListError>` - The result of the operation
    /// 
    /// # Errors
    /// * `CodeListError::InvalidFilePath` - If the file path contains invalid unicode characters
    /// 
    /// * Currently saving files as numbers
    pub fn save_codelists_to_json(&self, folder_path: &str, codelists: Vec<CodeList>) -> Result<(), CodeListError> {
        for (index, codelist) in codelists.iter().enumerate() {
            let filename = format!("{}.json", index + 1);
            let full_path = std::path::Path::new(folder_path).join(filename);
            let path_str = full_path.to_str()
                .ok_or_else(|| CodeListError::InvalidFilePath(
                    "Path contains invalid Unicode characters".to_string()
                ))?;
            codelist.save_to_json(path_str)?;
        }
        Ok(())
    }

    /// Save the codelists to a csv file
    /// 
    /// # Arguments
    /// * `folder_path` - The path to the folder
    /// * `codelists` - The vector of codelists
    /// 
    /// # Returns
    /// * `Result<(), CodeListError>` - The result of the operation
    /// 
    /// # Errors
    /// * `CodeListError::InvalidFilePath` - If the file path contains invalid unicode characters
    /// 
    /// * Currently saving files as numbers
    pub fn save_codelists_to_csv(&self, folder_path: &str, codelists: Vec<CodeList>) -> Result<(), CodeListError> {
        for (index, codelist) in codelists.iter().enumerate() {
            let filename = format!("{}.csv", index + 1);
            let full_path = std::path::Path::new(folder_path).join(filename);
            let path_str = full_path.to_str()
                .ok_or_else(|| CodeListError::InvalidFilePath(format!("Path contains invalid Unicode characters")))?;
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

    fn create_test_codelists(factory: &CodeListFactory) -> Result<Vec<CodeList>, CodeListError> {
        let codelist1 = CodeList::new(CodeListType::ICD10, factory.metadata.clone(), Some(factory.codelist_options.clone()));
        let codelist2 = CodeList::new(CodeListType::ICD10, factory.metadata.clone(), Some(factory.codelist_options.clone()));
        let codelists = factory.load_codelists(Some(vec![codelist1, codelist2]), None)?;
        Ok(codelists)
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
            .ok_or_else(|| CodeListError::InvalidFilePath(
                "Path contains invalid Unicode characters".to_string()
            ))?;

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
            .ok_or_else(|| CodeListError::InvalidFilePath(format!("Path contains invalid Unicode characters")))?;

        // Create test CSV content
        let csv_content = "\
code,term_test,description_test
A01,Test Disease 1,Description 1
B02,Test Disease 2,Description 2
C03,Test Disease 3,Description 3";

        fs::write(&file_path, csv_content)?;
        let factory = create_test_codelist_factory();
        
        let error = factory.load_codelist_from_csv_file(file_path_str).unwrap_err();

        assert!(matches!(error, CodeListError::InvalidTermField(msg) if msg == format!("Column not found with the header: {}", factory.codelist_options.term_column_name)));

        Ok(())
    }

    #[test]
    fn test_load_codelist_from_csv_file_invalid_code_column_name() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_codelist.json");
        let file_path_str = file_path.to_str()
            .ok_or_else(|| CodeListError::InvalidFilePath(format!("Path contains invalid Unicode characters")))?;

        // Create test CSV content
        let csv_content = "\
code_test,term,description_test
A01,Test Disease 1,Description 1
B02,Test Disease 2,Description 2
C03,Test Disease 3,Description 3";

        fs::write(&file_path, csv_content)?;
        let factory = create_test_codelist_factory();
        
        let error = factory.load_codelist_from_csv_file(file_path_str).unwrap_err();

        assert!(matches!(error, CodeListError::InvalidCodeField(msg) if msg == format!("Column not found with the header: {}", factory.codelist_options.code_column_name)));

        Ok(())
    }

    #[test]
    fn test_load_codelist_from_csv_file_empty_code() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_codelist.csv");
        let file_path_str = file_path.to_str()
            .ok_or_else(|| CodeListError::InvalidFilePath(format!("Path contains invalid Unicode characters")))?;

        // Create CSV with empty code
        let csv_content = "\
code,term,description
,Test Disease 1,Description 1
B02,Test Disease 2,Description 2";

        fs::write(&file_path, csv_content)?;
        let factory = create_test_codelist_factory();
        
        let error = factory.load_codelist_from_csv_file(file_path_str).unwrap_err();
        assert!(matches!(error, CodeListError::EmptyCode(msg) if msg.contains("Empty code field in row: 2")));

        Ok(())
    }

    #[test]
    fn test_load_codelist_from_csv_file_empty_term() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_codelist.csv");
        let file_path_str = file_path.to_str()
            .ok_or_else(|| CodeListError::InvalidFilePath(format!("Path contains invalid Unicode characters")))?;

        // Create CSV with empty term
        let csv_content = "\
code,term,description
A01,,Description 1
B02,Test Disease 2,Description 2";

        fs::write(&file_path, csv_content)?;
        let factory = create_test_codelist_factory();
        
        let error = factory.load_codelist_from_csv_file(file_path_str).unwrap_err();
        assert!(matches!(error, CodeListError::EmptyTerm(msg) if msg.contains("Empty term field in row: 2")));

        Ok(())
    }

    #[test]
    fn test_load_codelist_from_csv_file_duplicate_code_column() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_codelist.csv");
        let file_path_str = file_path.to_str()
            .ok_or_else(|| CodeListError::InvalidFilePath(format!("Path contains invalid Unicode characters")))?;

        // CSV with duplicate code columns
        let csv_content = "\
code,code,term
A01,A01,Test Disease 1";

        fs::write(&file_path, csv_content)?;
        let factory = create_test_codelist_factory();
        
        let error = factory.load_codelist_from_csv_file(file_path_str).unwrap_err();
        assert!(matches!(error, CodeListError::InvalidCodeField(msg) if msg.contains("Multiple columns found with the header: code")));

        Ok(())
    }

    #[test]
    fn test_load_codelist_from_csv_file_duplicate_term_column() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_codelist.csv");
        let file_path_str = file_path.to_str()
            .ok_or_else(|| CodeListError::InvalidFilePath(format!("Path contains invalid Unicode characters")))?;

        // CSV with duplicate term columns
        let csv_content = "\
code,term,term
A01,Test Disease 1,Test Disease 1";

        fs::write(&file_path, csv_content)?;
        let factory = create_test_codelist_factory();
        
        let error = factory.load_codelist_from_csv_file(file_path_str).unwrap_err();
        assert!(matches!(error, CodeListError::InvalidTermField(msg) if msg.contains("Multiple columns found with the header: term")));

        Ok(())
    }

    #[test]
    fn test_load_codelist_from_csv_file_unequal_columns() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_codelist.csv");
        let file_path_str = file_path.to_str()
            .ok_or_else(|| CodeListError::InvalidFilePath(format!("Path contains invalid Unicode characters")))?;

        // CSV with a row that has fewer columns than the header
        let csv_content = "\
code,term,description
A01";  // Missing columns

        fs::write(&file_path, csv_content)?;
        let factory = create_test_codelist_factory();
        
        let error = factory.load_codelist_from_csv_file(file_path_str).unwrap_err();
        assert!(matches!(error, CodeListError::CSVError(_)));

        Ok(())
    }
    
    #[test]
    fn test_load_codelist_from_json_file() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_codelist.json");
        let file_path_str = file_path.to_str()
            .ok_or_else(|| CodeListError::InvalidFilePath(format!("Path contains invalid Unicode characters"))  )?;

        // Create JSON with valid data
        let json_content = r#"[
            {"code": "A01", "term": "Test Disease 1"},
            {"code": "B02", "term": "Test Disease 2"},
            {"code": "C03", "term": "Test Disease 3"}
        ]"#;

        fs::write(&file_path, json_content)?;
        let factory = create_test_codelist_factory();
        
        let result = factory.load_codelist_from_json_file(file_path_str);
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
    fn test_load_codelist_from_json_file_invalid_code_field() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let factory = create_test_codelist_factory();

        let file_path = temp_dir.path().join("missing_code.json");
        let file_path_str = file_path.to_str().unwrap();
        let json_content = r#"[
            {"wrong_code": "A01", "term": "Test Disease 1"}
        ]"#;
        fs::write(&file_path, json_content)?;
        
        let error = factory.load_codelist_from_json_file(file_path_str).unwrap_err();
        assert!(matches!(error, CodeListError::InvalidCodeField(msg) if msg.contains(format!("No {} field found in json file at index: 0", factory.codelist_options.code_field_name).as_str())));

        Ok(())
    }

    #[test]
    fn test_load_codelist_from_json_file_invalid_term_field() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let factory = create_test_codelist_factory();

        let file_path = temp_dir.path().join("missing_term.json");
        let file_path_str = file_path.to_str().unwrap();
        let json_content = r#"[
            {"code": "A01", "wrong_term": "Test Disease 1"}
        ]"#;
        fs::write(&file_path, json_content)?;

        let error = factory.load_codelist_from_json_file(file_path_str).unwrap_err();
        assert!(matches!(error, CodeListError::InvalidTermField(msg) if msg.contains(format!("No {} field found in json file at index: 0", factory.codelist_options.term_field_name).as_str())));

        Ok(())
    }    

    #[test]
    fn test_load_codelist_from_json_file_empty_code() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let factory = create_test_codelist_factory();

        let file_path = temp_dir.path().join("empty_code.json");
        let file_path_str = file_path.to_str().unwrap();
        let json_content = r#"[
            {"code": "", "term": "Test Disease 1"},
            {"code": "B02", "term": "Test Disease 2"},
            {"code": "C03", "term": "Test Disease 3"}
        ]"#;
        fs::write(&file_path, json_content)?;

        let error = factory.load_codelist_from_json_file(file_path_str).unwrap_err();
        assert!(matches!(error, CodeListError::EmptyCode(msg) if msg.contains("Empty code at index: 0")));

        Ok(())
    }    

    #[test]
    fn test_load_codelist_from_json_file_empty_term() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let factory = create_test_codelist_factory();

        let file_path = temp_dir.path().join("empty_term.json");
        let file_path_str = file_path.to_str().unwrap();
        let json_content = r#"[
            {"code": "A01", "term": ""},
            {"code": "B02", "term": "Test Disease 2"},
            {"code": "C03", "term": "Test Disease 3"}
        ]"#;
        fs::write(&file_path, json_content)?;

        let error = factory.load_codelist_from_json_file(file_path_str).unwrap_err();
        assert!(matches!(error, CodeListError::EmptyTerm(msg) if msg.contains("Empty term at index: 0")));

        Ok(())
    }    

    #[test]
    fn test_load_codelist_from_json_file_invalid_code_type() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let factory = create_test_codelist_factory();

        let file_path = temp_dir.path().join("invalid_code_type.json");
        let file_path_str = file_path.to_str().unwrap();
        let json_content = r#"[
            {"code": true, "term": "Test Disease 1"},
            {"code": "B02", "term": "Test Disease 2"},
            {"code": "C03", "term": "Test Disease 3"}
        ]"#;
        fs::write(&file_path, json_content)?;

        let error = factory.load_codelist_from_json_file(file_path_str).unwrap_err();
        assert!(matches!(error, CodeListError::InvalidCodeType(msg) if msg.contains("Code at index 0 must be a string or number")));

        Ok(())
    }
    
    #[test]
    fn test_load_codelist_from_json_file_invalid_term_type() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let factory = create_test_codelist_factory();

        let file_path = temp_dir.path().join("invalid_term_type.json");
        let file_path_str = file_path.to_str().unwrap();
        let json_content = r#"[
            {"code": "A01", "term": 123},
            {"code": "B02", "term": "Test Disease 2"},
            {"code": "C03", "term": "Test Disease 3"}
        ]"#;
        fs::write(&file_path, json_content)?;

        let error = factory.load_codelist_from_json_file(file_path_str).unwrap_err();
        assert!(matches!(error, CodeListError::InvalidTermType(msg) if msg.contains("Term at index 0 must be a string")));

        Ok(())
    }
    
    #[test]
    fn test_load_codelist_from_json_file_invalid_input() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let factory = create_test_codelist_factory();

        let file_path = temp_dir.path().join("invalid_input.json");
        let file_path_str = file_path.to_str().unwrap();
        let json_content = r#"{"code": "A01", "term": "Test Disease 1"}"#;
        fs::write(&file_path, json_content)?;

        let error = factory.load_codelist_from_json_file(file_path_str).unwrap_err();
        println!("Error: {}", error);
        assert!(matches!(error, CodeListError::InvalidInput(msg) if msg.contains("JSON must be an array of objects")));

        Ok(())
    }

    #[test]
    fn test_load_codelist_from_file_invalid_file_path() -> Result<(), CodeListError> {
        let factory = create_test_codelist_factory();
        let error = factory.load_codelist_from_file("invalid_file_path").unwrap_err();
        assert!(matches!(error, CodeListError::InvalidFilePath(msg) if msg.contains("File path invalid_file_path is not a csv or json file")));
        Ok(())
    }

    #[test]
    fn test_load_codelist_from_file() -> Result<(), CodeListError> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test_codelist.csv");
        let file_path_str = file_path.to_str()
            .ok_or_else(|| CodeListError::InvalidFilePath(format!("Path contains invalid Unicode characters")))?;

        // Create test CSV content
        let csv_content = "\
code,term,description
A01,Test Disease 1,Description 1
B02,Test Disease 2,Description 2";

        fs::write(&file_path, csv_content)?;
        let factory = create_test_codelist_factory();
        
        let result = factory.load_codelist_from_file(file_path_str);
        assert!(result.is_ok());
        let codelist = result?;
        assert_eq!(codelist.entries.len(), 2);

        Ok(())
    }

    #[test]
    fn test_load_codelists_from_folder() -> Result<(), CodeListError> {
        let factory = create_test_codelist_factory();
        let temp_dir = tempdir()?;
        let temp_dir_path = temp_dir.path();
        let temp_dir_str = temp_dir_path.to_str()
            .ok_or(CodeListError::InvalidFilePath(
                "Path contains invalid Unicode characters".to_string()
            ))?;

        // Create test CSV content
        let csv_content = "\
code,term,description
A01,Test Disease 1,Description 1
B02,Test Disease 2,Description 2";

        let csv_path = temp_dir_path.join("test_codelist.csv");
        fs::write(&csv_path, csv_content)?;

        // Create test JSON content
        let json_content = r#"[
            {"code": "A01", "term": "Test Disease 1"},
            {"code": "B02", "term": "Test Disease 2"}
        ]"#;
        let json_path = temp_dir_path.join("test_codelist.json");
        fs::write(&json_path, json_content)?;

        let codelists = factory.load_codelists_from_folder(temp_dir_str)?;
        assert_eq!(codelists.len(), 2);
        Ok(())
    }

    #[test]
    fn test_load_codelists_with_codelists() -> Result<(), CodeListError> {
        let factory = create_test_codelist_factory();
        let codelists = create_test_codelists(&factory)?;
        assert_eq!(codelists.len(), 2);
        Ok(())
    }

    #[test]
    fn test_load_codelists_with_folder() -> Result<(), CodeListError> {
        let factory = create_test_codelist_factory();
        let temp_dir = tempdir()?;
        let temp_dir_path = temp_dir.path();
        
        let temp_dir_str = temp_dir_path.to_str()
            .ok_or(CodeListError::InvalidFilePath(
                "Path contains invalid Unicode characters".to_string()
            ))?;

        // Create test CSV content
        let csv_content = "\
code,term,description
A01,Test Disease 1,Description 1
B02,Test Disease 2,Description 2";

        let csv_path = temp_dir_path.join("test_codelist.csv");
        fs::write(&csv_path, csv_content)?;

        // Create test JSON content
        let json_content = r#"[
            {"code": "A01", "term": "Test Disease 1"},
            {"code": "B02", "term": "Test Disease 2"}
        ]"#;
        let json_path = temp_dir_path.join("test_codelist.json");
        fs::write(&json_path, json_content)?;

        let codelists = factory.load_codelists(None, Some(temp_dir_str))?;
        assert_eq!(codelists.len(), 2);
        Ok(())
    }

    #[test]
    fn test_load_codelists_no_input() -> Result<(), CodeListError> {
        let factory = create_test_codelist_factory();
        let error = factory.load_codelists(None, None).unwrap_err();
        assert!(matches!(error, CodeListError::InvalidInput(msg) if msg.contains("Codelist vector or path must be provided")));
        Ok(())
    }

    #[test]
    fn test_load_codelists_both_input() -> Result<(), CodeListError> {
        // Create test folder
        let factory = create_test_codelist_factory();
        let temp_dir = tempdir()?;
        let temp_dir_path = temp_dir.path();
        
        let temp_dir_str = temp_dir_path.to_str()
            .ok_or(CodeListError::InvalidFilePath(
                "Path contains invalid Unicode characters".to_string()
            ))?;

        // Create test CSV content
        let csv_content = "\
code,term,description
A01,Test Disease 1,Description 1
B02,Test Disease 2,Description 2";

        let csv_path = temp_dir_path.join("test_codelist.csv");
        fs::write(&csv_path, csv_content)?;

        // Create test JSON content
        let json_content = r#"[
            {"code": "A01", "term": "Test Disease 1"},
            {"code": "B02", "term": "Test Disease 2"}
        ]"#;
        let json_path = temp_dir_path.join("test_codelist.json");
        fs::write(&json_path, json_content)?;

        // create test codelists
        let codelist1 = CodeList::new(CodeListType::ICD10, factory.metadata.clone(), Some(factory.codelist_options.clone()));
        let codelist2 = CodeList::new(CodeListType::ICD10, factory.metadata.clone(), Some(factory.codelist_options.clone()));
        let codelists = factory.load_codelists(Some(vec![codelist1, codelist2]), None)?;

        // load codelists from folder
        let codelists = factory.load_codelists(Some(codelists), Some(temp_dir_str)).unwrap_err();
        assert!(matches!(codelists, CodeListError::InvalidInput(msg) if msg.contains("Either codelist vector or path must be provided, not both")));
        Ok(())
    }

    #[test]
    fn test_save_codelists_to_csv() -> Result<(), CodeListError> {
        let factory = create_test_codelist_factory();
        let codelists = create_test_codelists(&factory)?;
        let temp_dir = tempdir()?;
        let temp_dir_path = temp_dir.path();
        let temp_dir_str = temp_dir_path.to_str()
            .ok_or(CodeListError::InvalidFilePath(
                "Path contains invalid Unicode characters".to_string()
            ))?;
        let csv_path1 = temp_dir_path.join("1.csv");
        let csv_path2 = temp_dir_path.join("2.csv");
        let result = factory.save_codelists_to_csv(temp_dir_str, codelists);
        assert!(result.is_ok());
        assert!(csv_path1.exists());
        assert!(csv_path2.exists());
        Ok(())
    }

    #[test]
    fn test_save_codelists_to_json() -> Result<(), CodeListError> {
        let factory = create_test_codelist_factory();
        let codelists = create_test_codelists(&factory)?;
        let temp_dir = tempdir()?;
        let temp_dir_path = temp_dir.path();
        let temp_dir_str = temp_dir_path.to_str()
            .ok_or(CodeListError::InvalidFilePath(
                "Path contains invalid Unicode characters".to_string()
            ))?;
        let json_path1 = temp_dir_path.join("1.json");
        let json_path2 = temp_dir_path.join("2.json");
        let result = factory.save_codelists_to_json(temp_dir_str, codelists);
        assert!(result.is_ok());
        assert!(json_path1.exists());
        assert!(json_path2.exists());
        Ok(())
    }
}