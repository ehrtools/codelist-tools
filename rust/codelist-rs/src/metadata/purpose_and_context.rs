//! This file contains the purpose and context struct and its implementation

// External imports
use serde::{Deserialize, Serialize};

// Internal imports
use crate::errors::CodeListError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurposeAndContext {
    pub purpose: Option<String>,
    pub target_audience: Option<String>,
    pub use_context: Option<String>,
}

impl PurposeAndContext {
    /// Create a new PurposeAndContext
    ///
    /// # Arguments
    /// * `purpose` - The purpose of the codelist
    /// * `target_audience` - The target audience of the codelist
    /// * `use_context` - The use context of the codelist
    ///
    /// # Returns
    /// * `PurposeAndContext` - The new PurposeAndContext
    pub fn new(
        purpose: Option<String>,
        target_audience: Option<String>,
        use_context: Option<String>,
    ) -> Self {
        Self { purpose, target_audience, use_context }
    }

    /// Add a purpose to the PurposeAndContext
    ///
    /// # Arguments
    /// * `purpose` - The purpose to add
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type if successful or error if
    ///   purpose already exists
    pub fn add_purpose(&mut self, purpose: String) -> Result<(), CodeListError> {
        if self.purpose.is_none() {
            self.purpose = Some(purpose);
        } else {
            return Err(CodeListError::purpose_already_exists(
                "Unable to add purpose. Please use update purpose instead.",
            ));
        }
        Ok(())
    }

    /// Update the purpose of the PurposeAndContext
    ///
    /// # Arguments
    /// * `purpose` - The purpose to update
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type if successful or error if
    ///   purpose does not exist
    pub fn update_purpose(&mut self, purpose: String) -> Result<(), CodeListError> {
        if self.purpose.is_some() {
            self.purpose = Some(purpose);
        } else {
            return Err(CodeListError::purpose_does_not_exist(
                "Unable to update purpose. Please use add purpose instead.",
            ));
        }
        Ok(())
    }

    /// Remove the purpose of the PurposeAndContext
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type if successful or error if
    ///   purpose does not exist
    pub fn remove_purpose(&mut self) -> Result<(), CodeListError> {
        if self.purpose.is_some() {
            self.purpose = None;
        } else {
            return Err(CodeListError::purpose_does_not_exist("Unable to remove purpose."));
        }
        Ok(())
    }

    /// Add a target audience to the PurposeAndContext
    ///
    /// # Arguments
    /// * `target_audience` - The target audience to add
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type if successful or error if
    ///   target audience already exists
    pub fn add_target_audience(&mut self, target_audience: String) -> Result<(), CodeListError> {
        if self.target_audience.is_none() {
            self.target_audience = Some(target_audience);
        } else {
            return Err(CodeListError::target_audience_already_exists(
                "Unable to add target audience. Please use update target audience instead.",
            ));
        }
        Ok(())
    }

    /// Update the target audience of the PurposeAndContext
    ///
    /// # Arguments
    /// * `target_audience` - The target audience to update
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type if successful or error if
    ///   target audience does not exist
    pub fn update_target_audience(&mut self, target_audience: String) -> Result<(), CodeListError> {
        if self.target_audience.is_some() {
            self.target_audience = Some(target_audience);
        } else {
            return Err(CodeListError::target_audience_does_not_exist(
                "Unable to update target audience. Please use add target audience instead.",
            ));
        }
        Ok(())
    }

    /// Remove the target audience of the PurposeAndContext
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type if successful or error if
    ///   target audience does not exist
    pub fn remove_target_audience(&mut self) -> Result<(), CodeListError> {
        if self.target_audience.is_some() {
            self.target_audience = None;
        } else {
            return Err(CodeListError::target_audience_does_not_exist(
                "Unable to remove target audience.",
            ));
        }
        Ok(())
    }

    /// Add a use context to the PurposeAndContext
    ///
    /// # Arguments
    /// * `use_context` - The use context to add
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type if successful or error if use
    ///   context already exists
    pub fn add_use_context(&mut self, use_context: String) -> Result<(), CodeListError> {
        if self.use_context.is_none() {
            self.use_context = Some(use_context);
        } else {
            return Err(CodeListError::use_context_already_exists(
                "Unable to add use context. Please use update use context instead.",
            ));
        }
        Ok(())
    }

    /// Update the use context of the PurposeAndContext
    ///
    /// # Arguments
    /// * `use_context` - The use context to update
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type if successful or error if use
    ///   context does not exist
    pub fn update_use_context(&mut self, use_context: String) -> Result<(), CodeListError> {
        if self.use_context.is_some() {
            self.use_context = Some(use_context);
        } else {
            return Err(CodeListError::use_context_does_not_exist(
                "Unable to update use context. Please use add use context instead.",
            ));
        }
        Ok(())
    }

    /// Remove the use context of the PurposeAndContext
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type if successful or error if use
    ///   context does not exist
    pub fn remove_use_context(&mut self) -> Result<(), CodeListError> {
        if self.use_context.is_some() {
            self.use_context = None;
        } else {
            return Err(CodeListError::use_context_does_not_exist("Unable to remove use context."));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_purpose_and_context_all_params_are_some() -> PurposeAndContext {
        PurposeAndContext::new(
            Some("Purpose".to_string()),
            Some("Target Audience".to_string()),
            Some("Use Context".to_string()),
        )
    }

    fn create_test_purpose_and_context_all_params_are_none() -> PurposeAndContext {
        PurposeAndContext::new(None, None, None)
    }

    #[test]
    fn test_new() {
        let purpose_and_context = create_test_purpose_and_context_all_params_are_some();
        assert_eq!(purpose_and_context.purpose, Some("Purpose".to_string()));
        assert_eq!(purpose_and_context.target_audience, Some("Target Audience".to_string()));
        assert_eq!(purpose_and_context.use_context, Some("Use Context".to_string()));
    }

    #[test]
    fn test_add_purpose() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_none();
        purpose_and_context.add_purpose("Purpose".to_string())?;
        assert_eq!(purpose_and_context.purpose, Some("Purpose".to_string()));
        Ok(())
    }

    #[test]
    fn test_add_purpose_already_exists() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_some();
        let error = purpose_and_context.add_purpose("Purpose".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(
            error_string,
            "Purpose already exists: Unable to add purpose. Please use update purpose instead."
        );
        Ok(())
    }

    #[test]
    fn test_update_purpose() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_some();
        purpose_and_context.update_purpose("New Purpose".to_string())?;
        assert_eq!(purpose_and_context.purpose, Some("New Purpose".to_string()));
        Ok(())
    }

    #[test]
    fn test_update_purpose_does_not_exist() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_none();
        let error = purpose_and_context.update_purpose("Purpose".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(
            error_string,
            "Purpose does not exist: Unable to update purpose. Please use add purpose instead."
        );
        Ok(())
    }

    #[test]
    fn test_remove_purpose() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_some();
        purpose_and_context.remove_purpose()?;
        assert_eq!(purpose_and_context.purpose, None);
        Ok(())
    }

    #[test]
    fn test_remove_purpose_does_not_exist() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_none();
        let error = purpose_and_context.remove_purpose().unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Purpose does not exist: Unable to remove purpose.");
        Ok(())
    }

    #[test]
    fn test_add_target_audience() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_none();
        purpose_and_context.add_target_audience("Target Audience".to_string())?;
        assert_eq!(purpose_and_context.target_audience, Some("Target Audience".to_string()));
        Ok(())
    }

    #[test]
    fn test_add_target_audience_already_exists() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_some();
        let error =
            purpose_and_context.add_target_audience("Target Audience".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Target audience already exists: Unable to add target audience. Please use update target audience instead.");
        Ok(())
    }

    #[test]
    fn test_update_target_audience() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_some();
        purpose_and_context.update_target_audience("New Target Audience".to_string())?;
        assert_eq!(purpose_and_context.target_audience, Some("New Target Audience".to_string()));
        Ok(())
    }

    #[test]
    fn test_update_target_audience_does_not_exist() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_none();
        let error =
            purpose_and_context.update_target_audience("Target Audience".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Target audience does not exist: Unable to update target audience. Please use add target audience instead.");
        Ok(())
    }

    #[test]
    fn test_remove_target_audience() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_some();
        purpose_and_context.remove_target_audience()?;
        assert_eq!(purpose_and_context.target_audience, None);
        Ok(())
    }

    #[test]
    fn test_remove_target_audience_does_not_exist() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_none();
        let error = purpose_and_context.remove_target_audience().unwrap_err();
        let error_string = error.to_string();
        assert_eq!(
            error_string,
            "Target audience does not exist: Unable to remove target audience."
        );
        Ok(())
    }

    #[test]
    fn test_add_use_context() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_none();
        purpose_and_context.add_use_context("Use Context".to_string())?;
        assert_eq!(purpose_and_context.use_context, Some("Use Context".to_string()));
        Ok(())
    }

    #[test]
    fn test_add_use_context_already_exists() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_some();
        let error = purpose_and_context.add_use_context("Use Context".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Use context already exists: Unable to add use context. Please use update use context instead.");
        Ok(())
    }

    #[test]
    fn test_update_use_context() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_some();
        purpose_and_context.update_use_context("New Use Context".to_string())?;
        assert_eq!(purpose_and_context.use_context, Some("New Use Context".to_string()));
        Ok(())
    }

    #[test]
    fn test_update_use_context_does_not_exist() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_none();
        let error = purpose_and_context.update_use_context("Use Context".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Use context does not exist: Unable to update use context. Please use add use context instead.");
        Ok(())
    }

    #[test]
    fn test_remove_use_context() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_some();
        purpose_and_context.remove_use_context()?;
        assert_eq!(purpose_and_context.use_context, None);
        Ok(())
    }

    #[test]
    fn test_remove_use_context_does_not_exist() -> Result<(), CodeListError> {
        let mut purpose_and_context = create_test_purpose_and_context_all_params_are_none();
        let error = purpose_and_context.remove_use_context().unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Use context does not exist: Unable to remove use context.");
        Ok(())
    }
}
