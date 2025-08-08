use codelist_builder_rs::errors::CodeListBuilderError;
use codelist_builder_rs::snomed_usage_data::SnomedUsageData;
use codelist_builder_rs::usage_year::UsageYear;

#[tokio::test]
async fn test_download_usage() -> Result<(), CodeListBuilderError> {
    let base_url = "https://files.digital.nhs.uk";

    let result_2011_12 = SnomedUsageData::download_usage(base_url, UsageYear::Y2011_12).await?;
    let result_2012_13 = SnomedUsageData::download_usage(base_url, UsageYear::Y2012_13).await?;
    let result_2013_14 = SnomedUsageData::download_usage(base_url, UsageYear::Y2013_14).await?;
    let result_2014_15 = SnomedUsageData::download_usage(base_url, UsageYear::Y2014_15).await?;
    let result_2015_16 = SnomedUsageData::download_usage(base_url, UsageYear::Y2015_16).await?;
    let result_2016_17 = SnomedUsageData::download_usage(base_url, UsageYear::Y2016_17).await?;
    let result_2017_18 = SnomedUsageData::download_usage(base_url, UsageYear::Y2017_18).await?;
    let result_2018_19 = SnomedUsageData::download_usage(base_url, UsageYear::Y2018_19).await?;
    let result_2019_20 = SnomedUsageData::download_usage(base_url, UsageYear::Y2019_20).await?;
    let result_2020_21 = SnomedUsageData::download_usage(base_url, UsageYear::Y2020_21).await?;
    let result_2021_22 = SnomedUsageData::download_usage(base_url, UsageYear::Y2021_22).await?;
    let result_2022_23 = SnomedUsageData::download_usage(base_url, UsageYear::Y2022_23).await?;
    let result_2023_24 = SnomedUsageData::download_usage(base_url, UsageYear::Y2023_24).await?;

    assert!(!result_2011_12.usage_data.is_empty());
    assert!(!result_2012_13.usage_data.is_empty());
    assert!(!result_2013_14.usage_data.is_empty());
    assert!(!result_2014_15.usage_data.is_empty());
    assert!(!result_2015_16.usage_data.is_empty());
    assert!(!result_2016_17.usage_data.is_empty());
    assert!(!result_2017_18.usage_data.is_empty());
    assert!(!result_2018_19.usage_data.is_empty());
    assert!(!result_2019_20.usage_data.is_empty());
    assert!(!result_2020_21.usage_data.is_empty());
    assert!(!result_2021_22.usage_data.is_empty());
    assert!(!result_2022_23.usage_data.is_empty());
    assert!(!result_2023_24.usage_data.is_empty());

    Ok(())
}
