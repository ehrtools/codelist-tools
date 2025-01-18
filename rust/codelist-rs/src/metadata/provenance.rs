//! This file contains the provenance struct and its implementation

// External imports
use chrono::Utc;

// Internal imports
use crate::metadata::metadata_source::MetadataSource;
use crate::errors::CodeListError;

pub struct Provenance {
    pub source: MetadataSource,          
    pub created_date: chrono::DateTime<Utc>,
    pub last_modified_date: chrono::DateTime<Utc>,
    pub contributors: Option<Vec<String>>, 
}

impl Provenance {
    /// Create a new provenance
    ///
    /// # Arguments
    /// * `source` - The source of the codelist
    pub fn new(source: MetadataSource, contributors: Option<Vec<String>>) -> Provenance {
        Provenance {
            source,
            created_date: chrono::Utc::now(),
            last_modified_date: chrono::Utc::now(),
            contributors: contributors,
        }
    }

    /// Update the last modified date
    ///
    /// # Arguments
    /// * `self` - The provenance to update
    pub fn update_last_modified_date(&mut self) {
        self.last_modified_date = chrono::Utc::now();
    }

    /// Add a contributor to the provenance
    ///
    /// # Arguments
    /// * `self` - The provenance to update
    /// * `contributor` - The contributor to add
    pub fn add_contributor(&mut self, contributor: String) {
        if let Some(contributors) = &mut self.contributors {
            contributors.push(contributor);
        } else {
            self.contributors = Some(vec![contributor]);
        }
    }

    /// Remove a contributor from the provenance
    ///
    /// # Arguments
    /// * `self` - The provenance to update
    /// * `contributor` - The contributor to remove
    pub fn remove_contributor(&mut self, contributor: String) -> Result<(), CodeListError> {
        if let Some(contributors) = &mut self.contributors {
            contributors.retain(|c| c != &contributor);
        } else {
            return Err(CodeListError::contributor_not_found(contributor));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
