use crate::errors::CodeListError;
use crate::codelist::CodeList;
use crate::codelist::CodeListOptions;
use crate::metadata::Metadata;
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
            codelist_options,
            metadata,
            codelist_type,
        }
    }
}

// function wise- 

// i think lets separate them:
// •⁠  ⁠load_codelist_from_file
// •⁠  ⁠⁠load_all_codelists_from_folder (whcih calls load_codelist_from_file in a loop)
// •⁠  ⁠⁠load_codelist_directly
// •⁠  ⁠⁠load_codelists (which would do some sort of logic that if it received a path, woudl call load_all_codelists_from_folder, and if it received Vec<Codelists>) it would just laod them
// hen we want process codelists (one method)
// then saving fns

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
}

// python flow
// options =  {“allow_duplicates”: False, code_column_name: “code”, code_column_term “term”}
// metadata={“authors”: [“Caroline”, “Emma”]}
// factory = CodelistFactory(options=options, metadata=metadata)
// factory.load_codelists(path=‘a_path’)
// factory.process()
// factory.output_codelists(path=‘a_path’)
