//! This file contains the purpose and context struct and its implementation

// Internal imports
use crate::errors::CodeListError;

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
    pub fn new(purpose: Option<String>, target_audience: Option<String>, use_context: Option<String>) -> Self {
        PurposeAndContext {
            purpose,
            target_audience,
            use_context,
        }
    }

    /// Add a purpose to the PurposeAndContext
    ///
    /// # Arguments
    /// * `purpose` - The purpose to add
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type if successful or error if purpose already exists
    pub fn add_purpose(&mut self, purpose: String) -> Result<(), CodeListError> {
        if self.purpose.is_none() {
            self.purpose = Some(purpose);
        } else {
            return Err(CodeListError::purpose_already_exists());
        }
        Ok(())
    }

    /// Update the purpose of the PurposeAndContext
    ///
    /// # Arguments
    /// * `purpose` - The purpose to update
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type if successful or error if purpose does not exist
    pub fn update_purpose(&mut self, purpose: String) -> Result<(), CodeListError> {
        if self.purpose.is_some() {
            self.purpose = Some(purpose);
        } else {
            return Err(CodeListError::purpose_does_not_exist());
        }
        Ok(())
    }

    /// Add a target audience to the PurposeAndContext
    ///
    /// # Arguments
    /// * `target_audience` - The target audience to add
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type if successful or error if target audience already exists
    pub fn add_target_audience(&mut self, target_audience: String) -> Result<(), CodeListError> {
        if self.target_audience.is_none() {
            self.target_audience = Some(target_audience);
        } else {
            return Err(CodeListError::target_audience_already_exists());
        }
        Ok(())
    }

    /// Update the target audience of the PurposeAndContext
    ///
    /// # Arguments
    /// * `target_audience` - The target audience to update
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type if successful or error if target audience does not exist
    pub fn update_target_audience(&mut self, target_audience: String) -> Result<(), CodeListError> {
        if self.target_audience.is_some() {
            self.target_audience = Some(target_audience);
        } else {
            return Err(CodeListError::target_audience_does_not_exist());
        }
        Ok(())
    }

    /// Add a use context to the PurposeAndContext
    ///
    /// # Arguments
    /// * `use_context` - The use context to add
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type if successful or error if use context already exists
    pub fn add_use_context(&mut self, use_context: String) -> Result<(), CodeListError> {
        if self.use_context.is_none() {
            self.use_context = Some(use_context);
        } else {
            return Err(CodeListError::use_context_already_exists());
        }
        Ok(())
    }

    /// Update the use context of the PurposeAndContext
    ///
    /// # Arguments
    /// * `use_context` - The use context to update
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type if successful or error if use context does not exist
    pub fn update_use_context(&mut self, use_context: String) -> Result<(), CodeListError> {
        if self.use_context.is_some() {
            self.use_context = Some(use_context);
        } else {
            return Err(CodeListError::use_context_does_not_exist());
        }
        Ok(())
    }
}

// add target audience
// update target audience
// add use context
// update use context

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let purpose_and_context = PurposeAndContext::new(Some("Purpose".to_string()), Some("Target Audience".to_string()), Some("Use Context".to_string()));
        assert_eq!(purpose_and_context.purpose, Some("Purpose".to_string()));
        assert_eq!(purpose_and_context.target_audience, Some("Target Audience".to_string()));
        assert_eq!(purpose_and_context.use_context, Some("Use Context".to_string()));
    }

    #[test]
    fn test_add_purpose() -> Result<(), CodeListError> {
        let mut purpose_and_context = PurposeAndContext::new(None, None, None);
        purpose_and_context.add_purpose("Purpose".to_string())?;
        assert_eq!(purpose_and_context.purpose, Some("Purpose".to_string()));
        Ok(())
    }

    #[test]
    fn test_add_purpose_already_exists() -> Result<(), CodeListError> {
        let mut purpose_and_context = PurposeAndContext::new(Some("Purpose".to_string()), None, None);
        let error = purpose_and_context.add_purpose("Purpose".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Purpose already exists. Please use update purpose instead.");
        Ok(())
    }

    #[test]
    fn test_update_purpose() -> Result<(), CodeListError> {
        let mut purpose_and_context = PurposeAndContext::new(Some("Purpose".to_string()), None, None);
        purpose_and_context.update_purpose("New Purpose".to_string())?;
        assert_eq!(purpose_and_context.purpose, Some("New Purpose".to_string()));
        Ok(())
    }

    #[test]
    fn test_update_purpose_does_not_exist() -> Result<(), CodeListError> {
        let mut purpose_and_context = PurposeAndContext::new(None, None, None);
        let error = purpose_and_context.update_purpose("Purpose".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Purpose does not exist. Please use add purpose instead.");
        Ok(())
    }

    #[test]
    fn test_add_target_audience() -> Result<(), CodeListError> {
        let mut purpose_and_context = PurposeAndContext::new(None, None, None);
        purpose_and_context.add_target_audience("Target Audience".to_string())?;
        assert_eq!(purpose_and_context.target_audience, Some("Target Audience".to_string()));
        Ok(())
    }

    #[test]
    fn test_add_target_audience_already_exists() -> Result<(), CodeListError> {
        let mut purpose_and_context = PurposeAndContext::new(None, Some("Target Audience".to_string()), None);
        let error = purpose_and_context.add_target_audience("Target Audience".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Target audience already exists. Please use update target audience instead.");
        Ok(())
    }

    #[test]
    fn test_update_target_audience() -> Result<(), CodeListError> {
        let mut purpose_and_context = PurposeAndContext::new(None, Some("Target Audience".to_string()), None);
        purpose_and_context.update_target_audience("New Target Audience".to_string())?;
        assert_eq!(purpose_and_context.target_audience, Some("New Target Audience".to_string()));
        Ok(())
    }

    #[test]
    fn test_update_target_audience_does_not_exist() -> Result<(), CodeListError> {
        let mut purpose_and_context = PurposeAndContext::new(None, None, None);
        let error = purpose_and_context.update_target_audience("Target Audience".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Target audience does not exist. Please use add target audience instead.");
        Ok(())
    }

    #[test]
    fn test_add_use_context() -> Result<(), CodeListError> {
        let mut purpose_and_context = PurposeAndContext::new(None, None, None);
        purpose_and_context.add_use_context("Use Context".to_string())?;
        assert_eq!(purpose_and_context.use_context, Some("Use Context".to_string()));
        Ok(())
    }

    #[test]
    fn test_add_use_context_already_exists() -> Result<(), CodeListError> {
        let mut purpose_and_context = PurposeAndContext::new(None, None, Some("Use Context".to_string()));
        let error = purpose_and_context.add_use_context("Use Context".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Use context already exists. Please use update use context instead.");
        Ok(())
    }

    #[test]
    fn test_update_use_context() -> Result<(), CodeListError> {
        let mut purpose_and_context = PurposeAndContext::new(None, None, Some("Use Context".to_string()));
        purpose_and_context.update_use_context("New Use Context".to_string())?;
        assert_eq!(purpose_and_context.use_context, Some("New Use Context".to_string()));
        Ok(())
    }

    #[test]
    fn test_update_use_context_does_not_exist() -> Result<(), CodeListError> {
        let mut purpose_and_context = PurposeAndContext::new(None, None, None);
        let error = purpose_and_context.update_use_context("Use Context".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Use context does not exist. Please use add use context instead.");
        Ok(())
    }
}