use super::header_time_value;
use super::{header_value, ApiVersion, OdataMetadataLevel};
use azure_core::HTTPHeaderError;
use azure_core::Request;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct QueryEntitiesOptions {
    api_version: Option<ApiVersion>,
    odata_metadata_level: Option<OdataMetadataLevel>,
}

impl Default for QueryEntitiesOptions {
    fn default() -> Self {
        Self {
            api_version: Some(ApiVersion::default()),
            odata_metadata_level: Some(OdataMetadataLevel::FullMetadata),
        }
    }
}

impl QueryEntitiesOptions {
    setters! {
        api_version: ApiVersion => Some(api_version),
        odata_metadata_level: OdataMetadataLevel => Some(odata_metadata_level),
    }

    pub fn decorate_request_headers(&self, request: &mut Request) -> Result<(), HTTPHeaderError> {
        let headers = request.headers_mut();
        headers.append("x-ms-date", header_time_value(Utc::now())?);
        headers.append(
            "Accept",
            header_value::<OdataMetadataLevel>(&self.odata_metadata_level)?,
        );
        headers.append(
            "x-ms-version",
            header_value::<ApiVersion>(&self.api_version)?,
        );
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEntityResponse<ENTITY> {
    /// odata_metadata fields
    #[serde(rename = "odata.type")]
    pub odata_type: Option<String>,
    #[serde(rename = "odata.id")]
    pub odata_id: Option<String>,
    #[serde(rename = "odata.etag")]
    pub odata_etag: Option<String>,
    #[serde(rename = "odata.editLink")]
    pub odata_edit_link: Option<String>,
    #[serde(rename = "Timestamp@odata.type")]
    pub timestamp_odata_type: Option<String>,
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<String>,

    #[serde(flatten)]
    pub model: ENTITY,
}
