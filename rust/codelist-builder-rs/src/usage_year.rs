//! This file contains the usage year enum and its implementation

// Internal imports
use crate::errors::CodeListBuilderError;

// External imports
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Enum to represent usage year
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UsageYear {
    Y2011_12,
    Y2012_13,
    Y2013_14,
    Y2014_15,
    Y2015_16,
    Y2016_17,
    Y2017_18,
    Y2018_19,
    Y2019_20,
    Y2020_21,
    Y2021_22,
    Y2022_23,
    Y2023_24,
}

impl FromStr for UsageYear {
    type Err = CodeListBuilderError;

    /// Convert a string to a usage year
    ///
    /// # Arguments
    /// * `s` - The string to convert
    ///
    /// # Returns Self or an error if the string is not a valid usage year
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "2011-12" => Ok(UsageYear::Y2011_12),
            "2012-13" => Ok(UsageYear::Y2012_13),
            "2013-14" => Ok(UsageYear::Y2013_14),
            "2014-15" => Ok(UsageYear::Y2014_15),
            "2015-16" => Ok(UsageYear::Y2015_16),
            "2016-17" => Ok(UsageYear::Y2016_17),
            "2017-18" => Ok(UsageYear::Y2017_18),
            "2018-19" => Ok(UsageYear::Y2018_19),
            "2019-20" => Ok(UsageYear::Y2019_20),
            "2020-21" => Ok(UsageYear::Y2020_21),
            "2021-22" => Ok(UsageYear::Y2021_22),
            "2022-23" => Ok(UsageYear::Y2022_23),
            "2023-24" => Ok(UsageYear::Y2023_24),
            invalid_string => Err(CodeListBuilderError::invalid_usage_year(invalid_string)),
        }
    }
}

impl UsageYear {
    /// Get the URL for the usage year
    ///
    /// # Returns
    /// * `String` - The path for the usage year
    pub fn path(&self) -> String {
        match self {
            UsageYear::Y2011_12 => "/53/C8F877/SNOMED_code_usage_2011-12.txt".to_string(),
            UsageYear::Y2012_13 => "/69/866A44/SNOMED_code_usage_2012-13.txt".to_string(),
            UsageYear::Y2013_14 => "/82/40F702/SNOMED_code_usage_2013-14.txt".to_string(),
            UsageYear::Y2014_15 => "/BB/47E566/SNOMED_code_usage_2014-15.txt".to_string(),
            UsageYear::Y2015_16 => "/8B/15EAA1/SNOMED_code_usage_2015-16.txt".to_string(),
            UsageYear::Y2016_17 => "/E2/79561E/SNOMED_code_usage_2016-17.txt".to_string(),
            UsageYear::Y2017_18 => "/9F/024949/SNOMED_code_usage_2017-18.txt".to_string(),
            UsageYear::Y2018_19 => "/13/F2956B/SNOMED_code_usage_2018-19.txt".to_string(),
            UsageYear::Y2019_20 => "/8F/882EB3/SNOMED_code_usage_2019-20.txt".to_string(),
            UsageYear::Y2020_21 => "/8A/09BBE6/SNOMED_code_usage_2020-21.txt".to_string(),
            UsageYear::Y2021_22 => "/71/6C02F5/SNOMED_code_usage_2021-22.txt".to_string(),
            UsageYear::Y2022_23 => "/09/E1218D/SNOMED_code_usage_2022-23.txt".to_string(),
            UsageYear::Y2023_24 => "/B8/7D8335/SNOMED_code_usage_2023-24.txt".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str() -> Result<(), CodeListBuilderError> {
        let usage_year = UsageYear::from_str("2011-12")?;
        assert_eq!(usage_year, UsageYear::Y2011_12);
        Ok(())
    }

    #[test]
    fn test_path() {
        let usage_year = UsageYear::Y2015_16;
        let url = usage_year.path();
        let expected_url = "/8B/15EAA1/SNOMED_code_usage_2015-16.txt".to_string();
        assert_eq!(url, expected_url);
    }
}
