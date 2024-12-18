use std::collections::HashSet;
use std::str::FromStr;
use crate::errors::CodeListValidatorError;
use crate::validation::{
    icd10_validator::ICD10Validator,
    opcs_validator::OPCSValidator,
    snomed_validator::SNOMEDValidator
};
use crate::codelist_type::CodeListType;

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

impl CodeList {
    pub fn new(codelist_type: &str, code_column: String, term_column: String, file_path: String) -> Result<CodeList, CodeListValidatorError> {
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

    // this is a temporary function and will be pulled into a CodeListFactory method
    pub fn load_codelist(&mut self, file_path: &str) {
        //TODO
        // read data from csv, for each row validate code based on codelist type using validate_codelist
        // store validated code/term pairs in entries within codelist
        // will need to pick up the errors
    }

    pub fn validate_format(&self) {
        match self.codelist_type {
            CodeListType::ICD10 => {
                ICD10Validator::validate_all_code(self);
            }
            CodeListType::SNOMED => {
                SNOMEDValidator::validate_all_code(self);
            }
            CodeListType::OPCS => {
                OPCSValidator::validate_all_code(self);
            }
        }
    }
}

//TODO:
// several options of making codelist, e.g. excel, txt file, csv, hashset - codelistfactory handles this
// pub struct CodeListFactory {
//     input_directory: String,
//     output_directory: String,
// }
// impl CodeListFactory {
//     pub fn generate_codelist() {
//         // method for taking in data and outputting result of codelist or error
//     }
// also need save to format function (e.g. to csv) - code/term columns, all valid
// }