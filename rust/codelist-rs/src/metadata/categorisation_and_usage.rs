//! This file contains the categorisation and usage struct and its implementation

// External imports
use std::collections::HashSet;

// Internal imports
use crate::errors::CodeListError;

pub struct CategorisationAndUsage {
    pub tags: HashSet<String>,
    pub usage: HashSet<String>,
    pub license: Option<String>,
}

impl CategorisationAndUsage {
    pub fn new(tags: Option<HashSet<String>>, usage: Option<HashSet<String>>, license: Option<String>) -> Self {
        Self {
            tags: tags.unwrap_or_default(),
            usage: usage.unwrap_or_default(),
            license,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
}