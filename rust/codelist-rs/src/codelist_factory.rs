/// Struct to represent a codelist factory, which is used to load codelists from a directory and make sure all codelists are created following the same rules
///
/// # Fields
/// * `input_dir` - The path to the directory to load the codelists from
/// * `output_dir` - The path to the directory to save the codelists to
pub struct CodeListFactory {
    input_dir: Option<String>,
    output_dir: Option<String>,
}

impl CodeListFactory {
    pub fn new(input_dir: Option<String>, output_dir: Option<String>) -> Self {
        CodeListFactory {
            input_dir: input_dir.or(Some(".".to_string())),
            output_dir: output_dir.or(Some(".".to_string())),
        }
    }

    pub fn load_codelist_from_csv(&self, file_path: &str) -> Result<CodeList, CodeListError> {
        
    }

    pub fn load_codelist_from_json(&self, file_path: &str) -> Result<CodeList, CodeListError> {
        
    }

    pub fn process_directory(&self, directory: &str) -> Result<(), CodeListError> {
        // call either load_codelist_from_csv or load_codelist_from_json
    }

    // pub fn process_icd_codelists(&self, directory: &str) -> Result<(), CodeListError> {
        
    // }

    // pub fn process_snomed_codelists(&self, directory: &str) -> Result<(), CodeListError> {
        
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_codelist_factory_with_two_default_dirs() {
        let codelist_factory = CodeListFactory::new(None, None);
        assert_eq!(codelist_factory.input_dir, Some(".".to_string()));
        assert_eq!(codelist_factory.output_dir, Some(".".to_string()));
    }

    #[test]
    fn test_new_codelist_factory_with_one_default_dir() {
        let codelist_factory = CodeListFactory::new(Some("input".to_string()), None);
        assert_eq!(codelist_factory.input_dir, Some("input".to_string()));
        assert_eq!(codelist_factory.output_dir, Some(".".to_string()));
    }

    #[test]
    fn test_new_codelist_factory_with_two_specified_dirs() {
        let codelist_factory = CodeListFactory::new(Some("input".to_string()), Some("output".to_string()));
        assert_eq!(codelist_factory.input_dir, Some("input".to_string()));
        assert_eq!(codelist_factory.output_dir, Some("output".to_string()));
    }
}
