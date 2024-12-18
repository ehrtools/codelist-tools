use std::collections::HashSet;
use std::str::FromStr;
use crate::validation::{
    icd10_validator::ICD10Validator,
    opcs_validator::OPCSValidator,
    snomed_validator::SNOMEDValidator
};

pub enum CodeListType {
    ICD10,
    SNOMED,
    OPCS,
}

impl FromStr for CodeListType {
    type Err = String; // TODO: add custom error handling

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "icd10" => Ok(CodeListType::ICD10),
            "snomed" => Ok(CodeListType::SNOMED),
            "opcs" => Ok(CodeListType::OPCS),
            _ => Err(format!("Invalid codelist type: {}", s)),
        }
    }
}

pub struct CodeEntry {
    code: String,
    term: String,
}

pub struct CodeList {
    file_path: String,
    codelist_type: CodeListType,
    code_column: String,
    term_column: String,
    entries: HashSet<CodeEntry>,
}

// TODO: add custom error handling
impl CodeList {
    pub fn new(codelist_type: &str, code_column: String, term_column: String, file_path: String) -> Result<CodeList, String> {
        let codelist_type = CodeListType::from_str(codelist_type)?;
        let mut codelist = CodeList {
            file_path: file_path.clone(),
            codelist_type,
            code_column,
            term_column,
            entries: HashSet::new(),
        };
        
        // Load the codelist entries
        codelist.load_codelist(&file_path);
        
        Ok(codelist)
    }

    // function to add term/code entries to codelist struct for validating
    pub fn load_codelist(&mut self, file_path: &str) {
        //TODO
        // read data from csv, for each row validate code based on codelist type using validate_codelist
        // store validated code/term pairs in entries within codelist
    }

    pub fn validate_codelist(&self, code: &str) -> bool {
        match self.codelist_type {
            CodeListType::ICD10 => ICD10Validator::validate(self, code),
            CodeListType::SNOMED => SNOMEDValidator::validate(self, code),
            CodeListType::OPCS => OPCSValidator::validate(self, code),
        }
    }
}
