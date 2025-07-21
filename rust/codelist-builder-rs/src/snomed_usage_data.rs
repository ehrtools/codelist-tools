//! This file contains the snomed usage data struct and its implementation

// Internal imports
use crate::errors::CodeListBuilderError;
use crate::usage_year::UsageYear;

// External imports
use csv;
use reqwest;
use serde::{Deserialize, Serialize};

/// Struct to represent a snomed usage data entry
///
/// # Fields
/// * `snomed_concept_id` - The snomed concept id
/// * `description` - The description
/// * `usage` - The usage
/// * `active_at_start` - Whether the concept was active at the sstart of the usage period
/// * `active_at_end` - Whether the concept was active at the end of the usage period
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SnomedUsageDataEntry {
    pub snomed_concept_id: String,
    pub description: String,
    pub usage: String, // allows for * for count of 1-4
    pub active_at_start: bool,
    pub active_at_end: bool,
}

/// Struct to represent snomed usage data
///
/// # Fields
/// * `usage_data` - The usage data
/// * `usage_year` - The usage year
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SnomedUsageData {
    pub usage_data: Vec<SnomedUsageDataEntry>,
    pub usage_year: UsageYear,
}

impl SnomedUsageData {
    /// Download snomed usage data from a url
    ///
    /// # Arguments
    /// * `base_url` - The base url
    /// * `usage_year` - The usage year
    ///
    /// # Returns
    /// Self or an error if the download fails
    pub async fn download_usage(
        base_url: &str,
        usage_year: UsageYear,
    ) -> Result<Self, CodeListBuilderError> {
        let url = format!(
            "{}/{}",
            base_url.trim_end_matches('/'),
            usage_year.path().trim_start_matches('/')
        );

        let body = reqwest::get(&url).await?.text().await?;

        let usage_data = Self::parse_from_string(&body)?;

        Ok(SnomedUsageData { usage_data, usage_year })
    }

    /// Parse snomed usage data from a string
    ///
    /// # Arguments
    /// * `data` - The data to parse
    ///
    /// # Returns
    /// * The parsed usage data or an error
    pub fn parse_from_string(
        data: &str,
    ) -> Result<Vec<SnomedUsageDataEntry>, CodeListBuilderError> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b'\t')
            .from_reader(data.as_bytes());

        let mut usage_data = Vec::new();

        for (row_idx, result) in rdr.records().enumerate() {
            let record = result?;

            if record.len() != 5 {
                return Err(CodeListBuilderError::invalid_usage_data(format!(
                    "Invalid number of columns in record ({}) at row {}",
                    record.len(),
                    row_idx + 1
                )));
            }

            if let Some((col_idx, _)) =
                record.iter().enumerate().find(|(_, field)| field.trim().is_empty())
            {
                return Err(CodeListBuilderError::invalid_usage_data(format!(
                    "Empty value found in record at row {}, column {}",
                    row_idx + 1,
                    col_idx
                )));
            }

            let entry = SnomedUsageDataEntry {
                snomed_concept_id: record[0].to_string(),
                description: record[1].to_string(),
                usage: record[2].to_string(),
                active_at_start: record[3] == *"1",
                active_at_end: record[4] == *"1",
            };

            usage_data.push(entry);
        }
        Ok(usage_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::CodeListBuilderError;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const LONG_TEST_DATA: &str = "SNOMED_Concept_ID	Description	Usage	Active_at_Start	Active_at_End
279991000000102	Short message service text message sent to patient (procedure)	122292090	1	1
163030003	On examination - Systolic blood pressure reading (finding)	59227180	1	1
163031004	On examination - Diastolic blood pressure reading (finding)	59184050	1	1
163020007	On examination - blood pressure reading (finding)	37837700	1	1
1000731000000107	Serum creatinine level (observable entity)	33211250	1	1
1000661000000107	Serum sodium level (observable entity)	31630420	1	1
1000651000000109	Serum potassium level (observable entity)	31542470	1	1
162763007	On examination - weight (finding)	30836800	1	1
1022431000000105	Haemoglobin estimation (observable entity)	29864410	1	1
4468401000001106	Triptorelin 3.75mg injection (pdr for recon)+solvent prefilled syringe (product)	80	0	0";

    const SINGLE_ENTRY_TEST_DATA: &str =
        "SNOMED_Concept_ID	Description	Usage	Active_at_Start	Active_at_End
279991000000102	Short message service text message sent to patient (procedure)	122292090	1	1";

    #[test]
    fn test_parse_from_string_single_entry() -> Result<(), CodeListBuilderError> {
        let test_data = SINGLE_ENTRY_TEST_DATA;

        let entries = SnomedUsageData::parse_from_string(test_data)?;

        assert_eq!(entries.len(), 1);

        let entry = &entries[0];
        assert_eq!(entry.snomed_concept_id, "279991000000102");
        assert_eq!(
            entry.description,
            "Short message service text message sent to patient (procedure)"
        );
        assert_eq!(entry.usage, "122292090");
        assert!(entry.active_at_start);
        assert!(entry.active_at_end);

        Ok(())
    }

    #[test]
    fn test_parse_from_string_multiple_entries() -> Result<(), CodeListBuilderError> {
        let test_data = LONG_TEST_DATA;

        let entries = SnomedUsageData::parse_from_string(test_data)?;

        assert_eq!(entries.len(), 10);

        assert_eq!(entries[0].snomed_concept_id, "279991000000102");
        assert_eq!(
            entries[0].description,
            "Short message service text message sent to patient (procedure)"
        );
        assert_eq!(entries[0].usage, "122292090");
        assert!(entries[0].active_at_start);
        assert!(entries[0].active_at_end);

        assert_eq!(entries[1].snomed_concept_id, "163030003");
        assert_eq!(
            entries[1].description,
            "On examination - Systolic blood pressure reading (finding)"
        );
        assert_eq!(entries[1].usage, "59227180");
        assert!(entries[1].active_at_start);
        assert!(entries[1].active_at_end);

        assert_eq!(entries[2].snomed_concept_id, "163031004");
        assert_eq!(
            entries[2].description,
            "On examination - Diastolic blood pressure reading (finding)"
        );
        assert_eq!(entries[2].usage, "59184050");
        assert!(entries[2].active_at_start);
        assert!(entries[2].active_at_end);

        assert_eq!(entries[3].snomed_concept_id, "163020007");
        assert_eq!(entries[3].description, "On examination - blood pressure reading (finding)");
        assert_eq!(entries[3].usage, "37837700");
        assert!(entries[3].active_at_start);
        assert!(entries[3].active_at_end);

        assert_eq!(entries[4].snomed_concept_id, "1000731000000107");
        assert_eq!(entries[4].description, "Serum creatinine level (observable entity)");
        assert_eq!(entries[4].usage, "33211250");
        assert!(entries[4].active_at_start);
        assert!(entries[4].active_at_end);

        assert_eq!(entries[5].snomed_concept_id, "1000661000000107");
        assert_eq!(entries[5].description, "Serum sodium level (observable entity)");
        assert_eq!(entries[5].usage, "31630420");
        assert!(entries[5].active_at_start);
        assert!(entries[5].active_at_end);

        assert_eq!(entries[6].snomed_concept_id, "1000651000000109");
        assert_eq!(entries[6].description, "Serum potassium level (observable entity)");
        assert_eq!(entries[6].usage, "31542470");
        assert!(entries[6].active_at_start);
        assert!(entries[6].active_at_end);

        assert_eq!(entries[7].snomed_concept_id, "162763007");
        assert_eq!(entries[7].description, "On examination - weight (finding)");
        assert_eq!(entries[7].usage, "30836800");
        assert!(entries[7].active_at_start);
        assert!(entries[7].active_at_end);

        assert_eq!(entries[8].snomed_concept_id, "1022431000000105");
        assert_eq!(entries[8].description, "Haemoglobin estimation (observable entity)");
        assert_eq!(entries[8].usage, "29864410");
        assert!(entries[8].active_at_start);
        assert!(entries[8].active_at_end);

        assert_eq!(entries[9].snomed_concept_id, "4468401000001106");
        assert_eq!(
            entries[9].description,
            "Triptorelin 3.75mg injection (pdr for recon)+solvent prefilled syringe (product)"
        );
        assert_eq!(entries[9].usage, "80");
        assert!(!entries[9].active_at_start);
        assert!(!entries[9].active_at_end);

        Ok(())
    }

    #[test]
    fn test_parse_from_string_empty_data() -> Result<(), CodeListBuilderError> {
        let test_data = "SNOMED_Concept_ID	Description	Usage	Active_at_Start	Active_at_End";
        let entries = SnomedUsageData::parse_from_string(test_data)?;
        assert_eq!(entries.len(), 0);
        Ok(())
    }

    #[test]
    fn test_parse_from_string_column_count_too_small() -> Result<(), CodeListBuilderError> {
        let test_data = "SNOMED_Concept_ID	Description	Usage	Active_at_Start
279991000000102	Short message service text message sent to patient (procedure)	122292090	1";
        let error = SnomedUsageData::parse_from_string(test_data).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(
            &error_string,
            "Invalid usage data: Invalid number of columns in record (4) at row 1"
        );
        Ok(())
    }

    #[test]
    fn test_parse_from_string_column_count_too_big() -> Result<(), CodeListBuilderError> {
        let test_data =
            "SNOMED_Concept_ID	Description	Usage	Active_at_Start	Active_at_End	Active_at_End
279991000000102	Short message service text message sent to patient (procedure)	122292090	1	1	1";
        let error = SnomedUsageData::parse_from_string(test_data).unwrap_err();
        let error_string = error.to_string();
        assert_eq!(
            &error_string,
            "Invalid usage data: Invalid number of columns in record (6) at row 1"
        );
        Ok(())
    }

    #[test]
    fn test_parse_from_string_unequal_column_count() -> Result<(), CodeListBuilderError> {
        let test_data = "SNOMED_Concept_ID	Description	Usage	Active_at_Start	Active_at_End
279991000000102	Short message service text message sent to patient (procedure)	122292090	1
163030003	On examination - Systolic blood pressure reading (finding)	59227180	1
163031004	On examination - Diastolic blood pressure reading (finding)	59184050	1	1
163020007	On examination - blood pressure reading (finding)	37837700	1	1
1000731000000107	Serum creatinine level (observable entity)	33211250	1
1000661000000107	Serum sodium level (observable entity)	31630420	1	1";
        let error = SnomedUsageData::parse_from_string(test_data).unwrap_err();
        let error_string = error.to_string();
        assert!(error_string.contains("CSV error:"));
        Ok(())
    }

    #[tokio::test]
    async fn test_download_usage_from_url() -> Result<(), CodeListBuilderError> {
        let mock_server = MockServer::start().await;
        let usage_year = UsageYear::Y2020_21;

        let test_data = LONG_TEST_DATA;

        Mock::given(method("GET"))
            .and(path(usage_year.path()))
            .respond_with(ResponseTemplate::new(200).set_body_string(test_data))
            .mount(&mock_server)
            .await;

        let result = SnomedUsageData::download_usage(&mock_server.uri(), usage_year).await?;

        let usage_data = result.usage_data;
        let usage_year = result.usage_year;

        assert_eq!(usage_data.len(), 10);

        assert_eq!(usage_data[0].snomed_concept_id, "279991000000102");
        assert_eq!(
            usage_data[0].description,
            "Short message service text message sent to patient (procedure)"
        );
        assert_eq!(usage_data[0].usage, "122292090");
        assert!(usage_data[0].active_at_start);
        assert!(usage_data[0].active_at_end);

        assert_eq!(usage_data[1].snomed_concept_id, "163030003");
        assert_eq!(
            usage_data[1].description,
            "On examination - Systolic blood pressure reading (finding)"
        );
        assert_eq!(usage_data[1].usage, "59227180");
        assert!(usage_data[1].active_at_start);
        assert!(usage_data[1].active_at_end);

        assert_eq!(usage_data[2].snomed_concept_id, "163031004");
        assert_eq!(
            usage_data[2].description,
            "On examination - Diastolic blood pressure reading (finding)"
        );
        assert_eq!(usage_data[2].usage, "59184050");
        assert!(usage_data[2].active_at_start);
        assert!(usage_data[2].active_at_end);

        assert_eq!(usage_data[3].snomed_concept_id, "163020007");
        assert_eq!(usage_data[3].description, "On examination - blood pressure reading (finding)");
        assert_eq!(usage_data[3].usage, "37837700");
        assert!(usage_data[3].active_at_start);
        assert!(usage_data[3].active_at_end);

        assert_eq!(usage_data[4].snomed_concept_id, "1000731000000107");
        assert_eq!(usage_data[4].description, "Serum creatinine level (observable entity)");
        assert_eq!(usage_data[4].usage, "33211250");
        assert!(usage_data[4].active_at_start);
        assert!(usage_data[4].active_at_end);

        assert_eq!(usage_data[5].snomed_concept_id, "1000661000000107");
        assert_eq!(usage_data[5].description, "Serum sodium level (observable entity)");
        assert_eq!(usage_data[5].usage, "31630420");
        assert!(usage_data[5].active_at_start);
        assert!(usage_data[5].active_at_end);

        assert_eq!(usage_data[6].snomed_concept_id, "1000651000000109");
        assert_eq!(usage_data[6].description, "Serum potassium level (observable entity)");
        assert_eq!(usage_data[6].usage, "31542470");
        assert!(usage_data[6].active_at_start);
        assert!(usage_data[6].active_at_end);

        assert_eq!(usage_data[7].snomed_concept_id, "162763007");
        assert_eq!(usage_data[7].description, "On examination - weight (finding)");
        assert_eq!(usage_data[7].usage, "30836800");
        assert!(usage_data[7].active_at_start);
        assert!(usage_data[7].active_at_end);

        assert_eq!(usage_data[8].snomed_concept_id, "1022431000000105");
        assert_eq!(usage_data[8].description, "Haemoglobin estimation (observable entity)");
        assert_eq!(usage_data[8].usage, "29864410");
        assert!(usage_data[8].active_at_start);
        assert!(usage_data[8].active_at_end);

        assert_eq!(usage_data[9].snomed_concept_id, "4468401000001106");
        assert_eq!(
            usage_data[9].description,
            "Triptorelin 3.75mg injection (pdr for recon)+solvent prefilled syringe (product)"
        );
        assert_eq!(usage_data[9].usage, "80");
        assert!(!usage_data[9].active_at_start);
        assert!(!usage_data[9].active_at_end);

        assert_eq!(usage_year, UsageYear::Y2020_21);

        Ok(())
    }
}
