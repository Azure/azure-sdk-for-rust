use azure_core::{Error, Request, Response};
use chrono::Utc;
use http::HeaderValue;

use super::{ApiVersion, OdataMetadataLevel};

#[derive(Debug, Clone)]
pub struct ListTablesOptions {
    top: Option<i32>,
    filter: Option<String>,
    api_version: Option<ApiVersion>,
    odata_metadata_level: Option<OdataMetadataLevel>,
}

impl Default for ListTablesOptions {
    fn default() -> Self {
        Self {
            top: Default::default(),
            filter: Default::default(),
            api_version: Some(ApiVersion::default()),
            odata_metadata_level: Some(OdataMetadataLevel::FullMetadata),
        }
    }
}

impl ListTablesOptions {
    setters! {
        top: i32 => Some(top),
        filter: String => Some(filter),
        api_version: ApiVersion => Some(api_version),
        odata_metadata_level: OdataMetadataLevel  => Some(odata_metadata_level),
    }

    pub fn decorate_request(&self, request: &mut Request) -> Result<(), Error> {
        let headers = request.headers_mut();
        headers.append(
            "x-ms-version",
            HeaderValue::from_str(self.api_version.as_ref().unwrap().as_ref()).unwrap(),
        );
        headers.append(
            "Accept",
            HeaderValue::from_str(self.odata_metadata_level.as_ref().unwrap().as_ref()).unwrap(),
        );
        headers.append(
            "x-ms-date",
            HeaderValue::from_str(
                Utc::now()
                    .format("%a, %d %h %Y %T GMT")
                    .to_string()
                    .as_str(),
            )
            .unwrap(),
        );
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    #[serde(rename = "odata.type")]
    pub odata_type: Option<String>,
    #[serde(rename = "odata.id")]
    pub odata_id: Option<String>,
    #[serde(rename = "odata.editLink")]
    pub odata_link: Option<String>,
    #[serde(rename = "TableName")]
    pub table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTablesResponse {
    #[serde(rename = "odata.metadata")]
    pub odata_metadata: Option<String>,
    #[serde(rename = "value")]
    pub tables: Vec<Table>,
}

impl ListTablesResponse {
    pub(crate) async fn try_from(response: Response) -> Result<Self, Error> {
        let body = azure_core::collect_pinned_stream(response.deconstruct().2).await?;
        let response = serde_json::from_slice(&body)?;
        Ok(response)
    }
}
