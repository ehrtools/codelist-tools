// SNOMED validator for validating SNOMED codes in a codelist
use codelist_rs::codelist::CodeList;

use crate::{errors::CodeListValidatorError, validator::CodeValidator};

pub struct SnomedValidator<'a>(pub &'a CodeList);

const MAX_LENGTH: u32 = 18;
const MIN_LENGTH: u32 = 6;

impl CodeValidator for SnomedValidator<'_> {
    fn validate_code(&self, code: &str) -> Result<(), CodeListValidatorError> {
        code.trim().parse::<u128>().map_err(|e| CodeListValidatorError::ParseIntError {
            code: code.to_string(),
            reason: e.to_string(),
            codelist_type: self.0.codelist_type.to_string(),
        })?;
        let length = code.len() as u32;
        if !(MIN_LENGTH..=MAX_LENGTH).contains(&length) {
            return Err(CodeListValidatorError::invalid_code_length(
                code,
                format!("Code is not between {MIN_LENGTH} and {MAX_LENGTH} numbers in length",),
                self.0.codelist_type.to_string(),
            ));
        }
        Ok(())
    }

    fn validate_all_code(&self) -> Result<(), CodeListValidatorError> {
        let mut reasons = Vec::new();

        for (code, _) in self.0.entries.iter() {
            if let Err(err) = self.validate_code(code) {
                reasons.push(err.to_string());
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
        let codelist = CodeList::new(
            "test_codelist".to_string(),
            CodeListType::SNOMED,
            create_test_metadata(),
            None,
        )?;
        Ok(codelist)
    }

    #[test]
    fn test_validate_code_with_valid_code_default_max_min_lengths() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        assert!(codelist.validate_codes().is_ok());
        Ok(())
    }
    #[test]
    fn test_validate_code_with_invalid_code_not_all_numbers() -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = SnomedValidator(&codelist);
        let code = "11A6BB789A";
        let error = validator.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code 11A6BB789A is not composed of all numerical characters for type SNOMED. Reason: invalid digit found in string");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_less_than_min_length_of_3(
    ) -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = SnomedValidator(&codelist);
        let code = "11";
        let error = validator.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code 11 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_greater_than_max_length_of_18(
    ) -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = SnomedValidator(&codelist);
        let code = "1111111111111111111111111111";
        let error = validator.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code 1111111111111111111111111111 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_less_than_default_min_length(
    ) -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = SnomedValidator(&codelist);
        let code = "2043";
        let error = validator.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code 2043 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length");
        Ok(())
    }

    #[test]
    fn test_validate_code_with_invalid_code_length_greater_than_default_max_length(
    ) -> Result<(), CodeListError> {
        let codelist = create_test_codelist()?;
        let validator = SnomedValidator(&codelist);
        let code = "2043510071234567890";
        let error = validator.validate_code(code).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Code 2043510071234567890 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length");
        Ok(())
    }

    #[test]
    fn test_validate_all_code_with_valid_codelist() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry(
            "204351007".to_string(),
            Some("Fallot's trilogy (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "405752007".to_string(),
            Some("Congenital atrial septal defect (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "405752008".to_string(),
            Some(
                "Congenital atrial septal defect with patent foramen ovale (disorder)".to_string(),
            ),
            None,
        )?;
        codelist.add_entry(
            "77480004".to_string(),
            Some("Congenital biliary atresia (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "34000006".to_string(),
            Some("Crohn's disease (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "417357006".to_string(),
            Some("Sickling disorder due to hemoglobin S (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "24700007".to_string(),
            Some("Multiple sclerosis (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "398254007".to_string(),
            Some("Pre-eclampsia (disorder)".to_string()),
            None,
        )?;

        assert!(codelist.validate_codes().is_ok());

        Ok(())
    }

    #[test]
    fn test_validate_codelist_with_all_invalid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry(
            "11".to_string(),
            Some("Fallot's trilogy (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "111111111111111111111111111111".to_string(),
            Some("Fallot's trilogy (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "AA090".to_string(),
            Some("Fallot's trilogy (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "BB".to_string(),
            Some("Fallot's trilogy (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "1".to_string(),
            Some("Fallot's trilogy (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "1111111111111111111AAAAAAAAAA".to_string(),
            Some("Fallot's trilogy (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "11111".to_string(),
            Some("Fallot's trilogy (disorder)".to_string()),
            None,
        )?;

        let error = codelist.validate_codes().unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code 11 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length"));
        assert!(error_string.contains("Code 111111111111111111111111111111 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length"));
        assert!(error_string.contains("Code AA090 is not composed of all numerical characters for type SNOMED. Reason: invalid digit found in string"));
        assert!(error_string.contains("Code BB is not composed of all numerical characters for type SNOMED. Reason: invalid digit found in string"));
        assert!(error_string.contains("Code 1111111111111111111AAAAAAAAAA is not composed of all numerical characters for type SNOMED. Reason: invalid digit found in string"));
        assert!(error_string.contains("Code 11111 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length"));

        Ok(())
    }

    #[test]
    fn test_validate_all_code_with_mixed_valid_and_invalid_codes() -> Result<(), CodeListError> {
        let mut codelist = create_test_codelist()?;
        codelist.add_entry(
            "11".to_string(),
            Some("Fallot's trilogy (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "405752007".to_string(),
            Some("Congenital atrial septal defect (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "AA090".to_string(),
            Some(
                "Congenital atrial septal defect with patent foramen ovale (disorder)".to_string(),
            ),
            None,
        )?;
        codelist.add_entry(
            "77480004".to_string(),
            Some("Congenital biliary atresia (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "34000006".to_string(),
            Some("Crohn's disease (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "417357006".to_string(),
            Some("Sickling disorder due to hemoglobin S (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "24700007".to_string(),
            Some("Multiple sclerosis (disorder)".to_string()),
            None,
        )?;
        codelist.add_entry(
            "11111".to_string(),
            Some("Pre-eclampsia (disorder)".to_string()),
            None,
        )?;

        let error = codelist.validate_codes().unwrap_err();
        let error_string = error.to_string();

        assert!(error_string.contains("Some codes in the list are invalid. Details:"));
        assert!(error_string.contains("Code 11 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length"));
        assert!(error_string.contains("Code AA090 is not composed of all numerical characters for type SNOMED. Reason: invalid digit found in string"));
        assert!(error_string.contains("Code 11111 is an invalid length for type SNOMED. Reason: Code is not between 6 and 18 numbers in length"));

        Ok(())
    }
}
