use regex::Regex;
use codelist_rs::codelist::CodeList;
use crate::errors::CodeListValidatorError;
use crate::validator::CustomCodeValidator;

impl CustomCodeValidator for CodeList {
    fn custom_validate_all_code(&self) -> Result<(), CodeListValidatorError> {
        let mut reasons = Vec::new();

        let re_str = self.codelist_options.custom_regex.as_ref()
            .ok_or_else(|| CodeListValidatorError::custom_validation_failed("Custom regex pattern not provided"))?;

        // regex is compiled once when this method is called and used for validation of all codes
        let re = Regex::new(re_str)?;

        for (code, _) in self.entries.iter() {
            if !re.is_match(code) {
                reasons.push(
                    CodeListValidatorError::invalid_code_contents(
                        code,
                        "Code does not match the custom regex pattern",
                        self.codelist_type.to_string(),
                    )
                    .to_string(),
                );
            }
        }

        if reasons.is_empty() {
            Ok(())
        } else {
            Err(CodeListValidatorError::invalid_codelist(reasons))
        }
    }
}

#[cfg(test)]
mod tests {
    use codelist_rs::{
        codelist::CodeList,
        errors::CodeListError,
        metadata::{
            categorisation_and_usage::CategorisationAndUsage, metadata_source::Source,
            provenance::Provenance, purpose_and_context::PurposeAndContext,
            validation_and_review::ValidationAndReview, Metadata,
        },
        types::CodeListType,
        codelist_options::CodeListOptions,
    };

    use super::*;
    use crate::validator::Validator;

    // Helper function to create test metadata
    fn create_test_metadata() -> Metadata {
        Metadata::new(
            Provenance::new(Source::ManuallyCreated, None),
            CategorisationAndUsage::new(None, None, None),
            PurposeAndContext::new(None, None, None),
            ValidationAndReview::new(None, None, None, None, None),
        )
    }

    // Helper function to create a test codelist with two entries, default options
    // and test metadata
    fn create_test_codelist() -> Result<CodeList, CodeListError> {
        let options = CodeListOptions {
            allow_duplicates: true,
            code_column_name: "test_code".to_string(),
            term_column_name: "test_term".to_string(),
            code_field_name: "test_code".to_string(),
            term_field_name: "test_term".to_string(),
            custom_regex: Some("^[A-Z]{3}[!]{1}$".to_string()),
        };

        let codelist = CodeList::new(
            "test_codelist".to_string(),
            CodeListType::ICD10,
            create_test_metadata(),
            Some(options),
        )?;
        Ok(codelist)
    }

    #[test]
    fn test_validate_code_with_valid_code() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("ABC!".to_string(), None, None)?;
        assert!(codelist.validate_codes().is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_too_long() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("ABC!L".to_string(), None, None)?;
        let error = codelist.validate_codes().unwrap_err().to_string();
        assert_eq!(error, "Some codes in the list are invalid. Details: Code ABC!L contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern");
        Ok(())
    }

    #[test]
    fn test_validate_invalid_code_invalid_contents() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("100!".to_string(), None, None)?;
        let error = codelist.validate_codes().unwrap_err().to_string();
        assert_eq!(error, "Some codes in the list are invalid. Details: Code 100! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern");
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("ABC!".to_string(), None, None)?;
        codelist.add_entry("CDE!".to_string(), None, None)?;
        codelist.add_entry("ZOE!".to_string(), None, None)?;
        codelist.add_entry("FQH!".to_string(), None, None)?;
        codelist.add_entry("OKL!".to_string(), None, None)?;
        codelist.add_entry("MYP!".to_string(), None, None)?;
        codelist.add_entry("QNM!".to_string(), None, None)?;
        codelist.add_entry("KPL!".to_string(), None, None)?;
        assert!(codelist.validate_codes().is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_all_invalid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("A0P!".to_string(), Some("Gonorrhoea".to_string()), None)?;
        codelist.add_entry("AaB!".to_string(), Some("Pertussis".to_string()), None)?;
        codelist.add_entry("AAAAAAA!".to_string(), Some("Measles".to_string()), None)?;
        codelist.add_entry("AB".to_string(), Some("Lymphatic filariasis".to_string()), None)?;
        codelist.add_entry("abcd".to_string(), None, None)?;
        codelist.add_entry("abC!".to_string(), Some("Gout".to_string()), None)?;
        codelist.add_entry("OPP!!".to_string(), Some("Down Syndrome".to_string()), None)?;
        codelist.add_entry("!!PP".to_string(), Some("Dental caries".to_string()), None)?;
        let error = codelist.validate_codes().unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code A0P! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code AaB! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code AAAAAAA! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code AB contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code abcd contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code abC! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code OPP!! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code !!PP contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));

        assert!(
            matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 8)
        );
        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_mixed_invalid_and_valid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry("A54!p".to_string(), None, None)?;
        codelist.add_entry("1009!".to_string(), None, None)?;
        codelist.add_entry("A0p5!".to_string(), None, None)?;
        codelist.add_entry("aab!".to_string(), None, None)?;
        codelist.add_entry("ABC!".to_string(), None, None)?;
        codelist.add_entry("LPK!".to_string(), None, None)?;
        codelist.add_entry("FLP!".to_string(), None, None)?;
        codelist.add_entry("GVM!".to_string(), None, None)?;
        let error = codelist.validate_codes().unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code A54!p contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code 1009! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code A0p5! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));
        assert!(error_string.contains("Code aab! contents is invalid for type ICD10. Reason: Code does not match the custom regex pattern"));

        assert!(
            matches!(error, CodeListValidatorError::InvalidCodelist { reasons } if reasons.len() == 4)
        );
        Ok(())
    }
}