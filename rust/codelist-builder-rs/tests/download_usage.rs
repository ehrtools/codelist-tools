use codelist_builder_rs::errors::CodeListBuilderError;
use codelist_builder_rs::snomed_usage_data::SnomedUsageData;

#[tokio::test]
async fn test_download_usage() -> Result<(), CodeListBuilderError> {
    let result_2011_12 = SnomedUsageData::download_usage("2011-12", None).await?;
    let result_2012_13 = SnomedUsageData::download_usage("2012-13", None).await?;
    let result_2013_14 = SnomedUsageData::download_usage("2013-14", None).await?;
    let result_2014_15 = SnomedUsageData::download_usage("2014-15", None).await?;
    let result_2015_16 = SnomedUsageData::download_usage("2015-16", None).await?;
    let result_2016_17 = SnomedUsageData::download_usage("2016-17", None).await?;
    let result_2017_18 = SnomedUsageData::download_usage("2017-18", None).await?;
    let result_2018_19 = SnomedUsageData::download_usage("2018-19", None).await?;
    let result_2019_20 = SnomedUsageData::download_usage("2019-20", None).await?;
    let result_2020_21 = SnomedUsageData::download_usage("2020-21", None).await?;
    let result_2021_22 = SnomedUsageData::download_usage("2021-22", None).await?;
    let result_2022_23 = SnomedUsageData::download_usage("2022-23", None).await?;
    let result_2023_24 = SnomedUsageData::download_usage("2023-24", None).await?;

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
