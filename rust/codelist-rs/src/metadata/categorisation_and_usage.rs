//! This file contains the categorisation and usage struct and its implementation

pub struct CategorisationAndUsage {
    pub tags: Option<Vec<String>>,
    pub usage: Option<Vec<String>>,
    pub license: Option<String>,
}

// add tag
// remove tag
// add license
// add license
// remove license

#[cfg(test)]
mod tests {
    use super::*;
}