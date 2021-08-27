use std::str::FromStr;

use crate::{Filter, Top};

use super::{header_value, ApiVersion, OdataMetadataLevel};
use azure_core::{AppendToUrlQuery, Error, HTTPHeaderError, Request, Response};
use chrono::Utc;
use http::HeaderValue;

#[derive(Debug, Clone)]
pub struct ListTablesOptions<'a> {
    top: Option<Top>,
    filter: Option<Filter<'a>>,
    api_version: Option<ApiVersion>,
    odata_metadata_level: Option<OdataMetadataLevel>,
}

impl Default for ListTablesOptions<'_> {
    fn default() -> Self {
        Self {
            top: Default::default(),
            filter: Default::default(),
            api_version: Some(ApiVersion::default()),
            odata_metadata_level: Some(OdataMetadataLevel::FullMetadata),
        }
    }
}

impl<'a> ListTablesOptions<'a> {
    setters! {
        top: Top => Some(top),
        filter: Filter<'a> => Some(filter),
        api_version: ApiVersion => Some(api_version),
        odata_metadata_level: OdataMetadataLevel  => Some(odata_metadata_level),
    }

    pub fn decorate_request(&self, request: &mut Request) -> Result<(), HTTPHeaderError> {
        let headers = request.headers_mut();

        //if &self.odata_metadata_level. == OdataMetadataLevel::NoMetadata {
        //  return Err(HTTPHeaderError::HeaderValidationError(
        //       "List table operation can not include NoMetadata as the Accept header value".into(),
        //    ));
        //}
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

        if let Some(top) = self.top.as_ref() {
            let mut url = url::Url::from_str(request.uri().to_string().as_str()).unwrap();
            top.append_to_url_query(&mut url);
            let url_as_string = url.to_string();
            let uri = http::Uri::from_str(url_as_string.as_str()).unwrap();
            request.set_uri(uri);
        };

        if let Some(filter) = self.filter.as_ref() {
            let mut url = url::Url::from_str(request.uri().to_string().as_str()).unwrap();
            filter.append_to_url_query(&mut url);
            let url_as_string = url.to_string();
            let uri = http::Uri::from_str(url_as_string.as_str()).unwrap();
            request.set_uri(uri);
        }

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
