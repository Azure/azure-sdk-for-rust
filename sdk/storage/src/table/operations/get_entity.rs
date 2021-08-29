use std::str::FromStr;

use azure_core::{AppendToUrlQuery, Error, HTTPHeaderError, Request, Response};
use chrono::Utc;
use http::HeaderValue;

use crate::Select;

use super::{header_value, ApiVersion, OdataMetadataLevel};

#[derive(Debug, Clone)]
pub struct GetEntityOptions<'a> {
    select: Option<Select<'a>>,
    api_version: Option<ApiVersion>,
    odata_metadata_level: Option<OdataMetadataLevel>,
}

impl Default for GetEntityOptions<'_> {
    fn default() -> Self {
        Self {
            select: Default::default(),
            api_version: Some(ApiVersion::default()),
            odata_metadata_level: Some(OdataMetadataLevel::FullMetadata),
        }
    }
}

impl<'a> GetEntityOptions<'a> {
    setters! {
        select: Select<'a> => Some(select),
        api_version: ApiVersion => Some(api_version),
        odata_metadata_level: OdataMetadataLevel => Some(odata_metadata_level),
    }

    pub fn decorate_request(&self, request: &mut Request) -> Result<(), HTTPHeaderError> {
        let headers = request.headers_mut();
        headers.append(
            "Accept",
            header_value::<OdataMetadataLevel>(&self.odata_metadata_level)?,
        );
        headers.append(
            "x-ms-version",
            header_value::<ApiVersion>(&self.api_version)?,
        );

        headers.append(
            "x-ms-date",
            HeaderValue::from_str(
                Utc::now()
                    .format("%a, %d %h %Y %T GMT")
                    .to_string()
                    .as_str(),
            )?,
        );

        if let Some(select) = self.select.as_ref() {
            let mut url = url::Url::from_str(request.uri().to_string().as_str()).unwrap();
            select.append_to_url_query(&mut url);
            let url_as_string = url.to_string();
            let uri = http::Uri::from_str(url_as_string.as_str()).unwrap();
            request.set_uri(uri);
        };

        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableEntity {
    #[serde(rename = "PartitionKey")]
    partition_key: String,
    #[serde(rename = "RowKey")]
    row_key: String,
    #[serde(rename = "Timestamp")]
    timestamp: String,
    #[serde(rename = "to")]
    to: String,
    #[serde(rename = "header")]
    header: String,
    #[serde(rename = "content")]
    content: String,
}

impl TableEntity {
    pub(crate) async fn try_from(response: Response) -> Result<Self, Error> {
        let body = azure_core::collect_pinned_stream(response.deconstruct().2).await?;
        let response = serde_json::from_slice(&body)?;
        Ok(response)
    }
}
