//! This file contains the validation and review struct and its implementation

// External imports
use serde::{Deserialize, Serialize};

// Internal imports
use crate::errors::CodeListError;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationAndReview {
    pub reviewed: bool,
    pub reviewer: Option<String>,
    pub review_date: Option<DateTime<Utc>>,
    pub status: Option<String>, // TODO: make this an enum
    pub validation_notes: Option<String>,
}

impl ValidationAndReview {
    /// Create new ValidationAndReview
    ///
    /// # Arguments
    /// * `reviewed` - Whether the codelist has been reviewed (default is false if value of None is provided)
    /// * `reviewer` - The reviewer of the codelist
    /// * `review_date` - The date of the review
    /// * `status` - The status of the codelist
    /// * `validation_notes` - The notes of the validation
    ///
    /// # Returns
    /// * `ValidationAndReview` - The new ValidationAndReview
    pub fn new(reviewed: Option<bool>, reviewer: Option<String>, review_date: Option<DateTime<Utc>>, status: Option<String>, validation_notes: Option<String>) -> Self {
        Self {
            reviewed: reviewed.unwrap_or(false),
            reviewer,
            review_date,
            status,
            validation_notes,
        }
    }

    /// Update the reviewed field
    ///
    /// # Arguments
    /// * `reviewed` - Whether the codelist has been reviewed
    pub fn update_reviewed(&mut self, reviewed: bool) {
        self.reviewed = reviewed;
    }

    /// Add a reviewer
    ///
    /// # Arguments
    /// * `reviewer` - The reviewer of the codelist
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type or error if reviewer already exists
    pub fn add_reviewer(&mut self, reviewer: String) -> Result<(), CodeListError> {
        if self.reviewer.is_none() {
            self.reviewer = Some(reviewer);
        } else {
            return Err(CodeListError::reviewer_already_exists("Unable to add reviewer. Please use update reviewer instead."));
        }
        Ok(())
    }

    /// Update the reviewer
    ///
    /// # Arguments
    /// * `reviewer` - The reviewer of the codelist
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type or error if reviewer does not exist
    pub fn update_reviewer(&mut self, reviewer: String) -> Result<(), CodeListError> {
        if self.reviewer.is_some() {
            self.reviewer = Some(reviewer);
        } else {
            return Err(CodeListError::reviewer_does_not_exist("Unable to update reviewer. Please use add reviewer instead."));
        }
        Ok(())
    }

    /// Remove the reviewer
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type or error if reviewer does not exist
    pub fn remove_reviewer(&mut self) -> Result<(), CodeListError> {
        if self.reviewer.is_some() {
            self.reviewer = None;
        } else {
            return Err(CodeListError::reviewer_does_not_exist("Unable to remove reviewer."));
        }
        Ok(())
    }

    /// Add a review date
    ///
    /// # Arguments
    /// * `review_date` - The date of the review
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type or error if review date already exists
    pub fn add_review_date(&mut self, review_date: DateTime<Utc>) -> Result<(), CodeListError> {
        if self.review_date.is_none() {
            self.review_date = Some(review_date);
        } else {
            return Err(CodeListError::review_date_already_exists("Unable to add review date. Please use update review date instead."));
        }
        Ok(())
    }

    /// Update the review date
    ///
    /// # Arguments
    /// * `review_date` - The date of the review
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type or error if review date does not exist
    pub fn update_review_date(&mut self, review_date: DateTime<Utc>) -> Result<(), CodeListError> {
        if self.review_date.is_some() {
            self.review_date = Some(review_date);
        } else {
            return Err(CodeListError::review_date_does_not_exist("Unable to update review date. Please use add review date instead."));
        }
        Ok(())
    }

    /// Remove the review date
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type or error if review date does not exist
    pub fn remove_review_date(&mut self) -> Result<(), CodeListError> {
        if self.review_date.is_some() {
            self.review_date = None;
        } else {
            return Err(CodeListError::review_date_does_not_exist("Unable to remove review date."));
        }
        Ok(())
    }

    /// Add a status
    ///
    /// # Arguments
    /// * `status` - The status of the codelist
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type or error if status already exists
    pub fn add_status(&mut self, status: String) -> Result<(), CodeListError> {
        if self.status.is_none() {
            self.status = Some(status);
        } else {
            return Err(CodeListError::status_already_exists("Unable to add status. Please use update status instead."));
        }
        Ok(())
    }

    /// Update the status
    ///
    /// # Arguments
    /// * `status` - The status of the codelist
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type or error if status does not exist
    pub fn update_status(&mut self, status: String) -> Result<(), CodeListError> {
        if self.status.is_some() {
            self.status = Some(status);
        } else {
            return Err(CodeListError::status_does_not_exist("Unable to update status. Please use add status instead."));
        }
        Ok(())
    }

    /// Remove the status
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type or error if status does not exist
    pub fn remove_status(&mut self) -> Result<(), CodeListError> {
        if self.status.is_some() {
            self.status = None;
        } else {
            return Err(CodeListError::status_does_not_exist("Unable to remove status."));
        }
        Ok(())
    }

    /// Get the validation notes
    pub fn get_validation_notes(&self) -> Option<String> {
        self.validation_notes.clone()
    }

    /// Add validation notes
    ///
    /// # Arguments
    /// * `validation_notes` - The notes of the validation
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type or error if validation notes already exist
    pub fn add_validation_notes(&mut self, validation_notes: String) -> Result<(), CodeListError> {
        if self.validation_notes.is_none() {
            self.validation_notes = Some(validation_notes);
        } else {
            return Err(CodeListError::validation_notes_already_exist("Unable to add validation notes. Please use update validation notes instead."));
        }
        Ok(())
    }

    /// Update the validation notes
    ///
    /// # Arguments
    /// * `validation_notes` - The notes of the validation
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type or error if validation notes does not exist
    pub fn update_validation_notes(&mut self, validation_notes: String) -> Result<(), CodeListError> {
        if let Some(existing) = &mut self.validation_notes {
            existing.push('\n');
            existing.push_str(&validation_notes);
            Ok(())
        } else {
            Err(CodeListError::validation_notes_do_not_exist(
                "Unable to update validation notes. Please use add validation notes instead.",
            ))
        }
    }


    /// Remove the validation notes
    ///
    /// # Returns
    /// * `Result<(), CodeListError>` - unit type or error if validation notes does not exist
    pub fn remove_validation_notes(&mut self) -> Result<(), CodeListError> {
        if self.validation_notes.is_some() {
            self.validation_notes = None;
        } else {
            return Err(CodeListError::validation_notes_do_not_exist("Unable to remove validation notes."));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // helper function to create a test validation and review
    fn test_validation_and_review_all_params_are_some_or_true() -> ValidationAndReview {
        ValidationAndReview::new(Some(true), Some("Reviewer".to_string()), Some(chrono::Utc::now()), Some("Status".to_string()), Some("Validation Notes".to_string()))
    }

    fn test_validation_and_review_all_params_are_none() -> ValidationAndReview {
        ValidationAndReview::new(None, None, None, None, None)
    }

    // helper function to get the time difference between the current time and the given date
    fn get_time_difference(date: chrono::DateTime<Utc>) -> i64 {
        let now = chrono::Utc::now();
        (date - now).num_milliseconds().abs()
    }

    #[test]
    fn test_new() {
        let validation_and_review = test_validation_and_review_all_params_are_some_or_true();
        assert_eq!(validation_and_review.reviewed, true);
        assert_eq!(validation_and_review.reviewer, Some("Reviewer".to_string()));
        let time_difference = get_time_difference(validation_and_review.review_date.unwrap());
        assert!(time_difference < 1000);
        assert_eq!(validation_and_review.status, Some("Status".to_string()));
        assert_eq!(validation_and_review.validation_notes, Some("Validation Notes".to_string()));
    }

    #[test]
    fn test_update_reviewed() {
        let mut validation_and_review = test_validation_and_review_all_params_are_some_or_true();
        assert_eq!(validation_and_review.reviewed, true);
        validation_and_review.update_reviewed(false);
        assert_eq!(validation_and_review.reviewed, false);
    }

    #[test]
    fn test_add_reviewer() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_none();
        assert_eq!(validation_and_review.reviewer, None);
        validation_and_review.add_reviewer("Reviewer".to_string())?;
        assert_eq!(validation_and_review.reviewer, Some("Reviewer".to_string()));
        Ok(())
    }

    #[test]
    fn test_add_reviewer_already_exists() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_some_or_true();
        let error = validation_and_review.add_reviewer("Reviewer".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Reviewer already exists: Unable to add reviewer. Please use update reviewer instead.");
        Ok(())
    }

    #[test]
    fn test_update_reviewer() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_some_or_true();
        assert_eq!(validation_and_review.reviewer, Some("Reviewer".to_string()));
        validation_and_review.update_reviewer("Reviewer 2".to_string())?;
        assert_eq!(validation_and_review.reviewer, Some("Reviewer 2".to_string()));
        Ok(())
    }

    #[test]
    fn test_update_reviewer_does_not_exist() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_none();
        let error = validation_and_review.update_reviewer("Reviewer".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Reviewer does not exist: Unable to update reviewer. Please use add reviewer instead.");
        Ok(())
    }

    #[test]
    fn test_remove_reviewer() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_some_or_true();
        assert_eq!(validation_and_review.reviewer, Some("Reviewer".to_string()));
        validation_and_review.remove_reviewer()?;
        assert_eq!(validation_and_review.reviewer, None);
        Ok(())
    }

    #[test]
    fn test_remove_reviewer_does_not_exist() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_none();
        let error = validation_and_review.remove_reviewer().unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Reviewer does not exist: Unable to remove reviewer.");
        Ok(())
    }

    #[test]
    fn test_add_review_date() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_none();
        assert_eq!(validation_and_review.review_date, None);
        validation_and_review.add_review_date(chrono::Utc::now())?;
        let time_difference = get_time_difference(
            validation_and_review.review_date
                .ok_or(CodeListError::review_date_is_none())?
        );
        assert!(time_difference < 1000);
        Ok(())
    }

    #[test]
    fn test_add_review_date_already_exists() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_some_or_true();
        let error = validation_and_review.add_review_date(chrono::Utc::now()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Review date already exists: Unable to add review date. Please use update review date instead.");
        Ok(())
    }

    #[test]
    fn test_update_review_date() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_some_or_true();
        std::thread::sleep(std::time::Duration::from_secs(10));
        validation_and_review.update_review_date(chrono::Utc::now())?;
        let time_difference = get_time_difference(validation_and_review.review_date.unwrap());
        assert!(time_difference < 1000);
        Ok(())
    }

    #[test]
    fn test_update_review_date_does_not_exist() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_none();
        let error = validation_and_review.update_review_date(chrono::Utc::now()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Review date does not exist: Unable to update review date. Please use add review date instead.");
        Ok(())
    }

    #[test]
    fn test_remove_review_date() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_none();
        validation_and_review.add_review_date(chrono::Utc::now())?;
        let difference = get_time_difference(validation_and_review.review_date.unwrap());
        assert!(difference < 1000);
        validation_and_review.remove_review_date()?;
        assert_eq!(validation_and_review.review_date, None);
        Ok(())
    }

    #[test]
    fn test_remove_review_date_does_not_exist() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_none();
        let error = validation_and_review.remove_review_date().unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Review date does not exist: Unable to remove review date.");
        Ok(())
    }

    #[test]
    fn test_add_status() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_none();
        assert_eq!(validation_and_review.status, None);
        validation_and_review.add_status("Status".to_string())?;
        assert_eq!(validation_and_review.status, Some("Status".to_string()));
        Ok(())
    }

    #[test]
    fn test_add_status_already_exists() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_some_or_true();
        let error = validation_and_review.add_status("Status".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Status already exists: Unable to add status. Please use update status instead.");
        Ok(())
    }

    #[test]
    fn test_update_status() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_some_or_true();
        assert_eq!(validation_and_review.status, Some("Status".to_string()));
        validation_and_review.update_status("Status 2".to_string())?;
        assert_eq!(validation_and_review.status, Some("Status 2".to_string()));
        Ok(())
    }

    #[test]
    fn test_update_status_does_not_exist() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_none();
        let error = validation_and_review.update_status("Status".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Status does not exist: Unable to update status. Please use add status instead.");
        Ok(())
    }

    #[test]
    fn test_remove_status() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_some_or_true();
        assert_eq!(validation_and_review.status, Some("Status".to_string()));
        validation_and_review.remove_status()?;
        assert_eq!(validation_and_review.status, None);
        Ok(())
    }

    #[test]
    fn test_remove_status_does_not_exist() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_none();
        let error = validation_and_review.remove_status().unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Status does not exist: Unable to remove status.");
        Ok(())
    }

    #[test]
    fn test_add_validation_notes() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_none();
        assert_eq!(validation_and_review.validation_notes, None);
        validation_and_review.add_validation_notes("Validation Notes".to_string())?;
        assert_eq!(validation_and_review.validation_notes, Some("Validation Notes".to_string()));
        Ok(())
    }

    #[test]
    fn test_add_validation_notes_already_exists() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_some_or_true();
        let error = validation_and_review.add_validation_notes("Validation Notes".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Validation notes already exist: Unable to add validation notes. Please use update validation notes instead.");
        Ok(())
    }

    #[test]
    fn test_update_validation_notes() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_some_or_true();
        assert_eq!(validation_and_review.validation_notes, Some("Validation Notes".to_string()));
        validation_and_review.update_validation_notes("Validation Notes 2".to_string())?;
        assert_eq!(validation_and_review.validation_notes, Some("Validation Notes\nValidation Notes 2".to_string()));
        Ok(())
    }

    #[test]
    fn test_update_validation_notes_does_not_exist() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_none();
        let error = validation_and_review.update_validation_notes("Validation Notes".to_string()).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Validation notes do not exist: Unable to update validation notes. Please use add validation notes instead.");
        Ok(())
    }

    #[test]
    fn test_remove_validation_notes() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_some_or_true();
        assert_eq!(validation_and_review.validation_notes, Some("Validation Notes".to_string()));
        validation_and_review.remove_validation_notes()?;
        assert_eq!(validation_and_review.validation_notes, None);
        Ok(())
    }

    #[test]
    fn test_remove_validation_notes_does_not_exist() -> Result<(), CodeListError> {
        let mut validation_and_review = test_validation_and_review_all_params_are_none();
        let error = validation_and_review.remove_validation_notes().unwrap_err();
        let error_string = error.to_string();
        assert_eq!(error_string, "Validation notes do not exist: Unable to remove validation notes.");
        Ok(())
    }
}