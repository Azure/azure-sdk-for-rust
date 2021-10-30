use crate::table::{
    operations::{header_time_value, header_value},
    prelude::*,
};
use azure_core::{AppendToUrlQuery, Error, Request, Response};
use chrono::Utc;
use http::Uri;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct QueryTablesOptions<'a> {
    top: Option<Top>,
    timeout: Option<Timeout>,
    filter: Option<Filter<'a>>,
    api_version: Option<ApiVersion>,
    odata_metadata_level: Option<OdataMetadataLevel>,
}

impl Default for QueryTablesOptions<'_> {
    fn default() -> Self {
        Self {
            top: Default::default(),
            filter: Default::default(),
            timeout: Default::default(),
            api_version: Some(ApiVersion::default()),
            odata_metadata_level: Some(OdataMetadataLevel::FullMetadata),
        }
    }
}

impl<'a> QueryTablesOptions<'a> {
    setters! {
        top: Top => Some(top),
        timeout: Timeout => Some(timeout),
        filter: Filter<'a> => Some(filter),
        api_version: ApiVersion => Some(api_version),
        odata_metadata_level: OdataMetadataLevel  => Some(odata_metadata_level),
    }

    pub(crate) fn base_uri_path(&self) -> &str {
        if self.top.is_none() && self.filter.is_none() {
            "Tables"
        } else {
            "Tables()"
        }
    }

    pub(crate) fn decorate_request(&self, request: &mut Request) -> Result<(), Error> {
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
        *request.uri_mut() = self.append_query_parameters(request);
        Ok(())
    }

    fn append_query_parameters(&self, request: &mut Request) -> Uri {
        let mut url = url::Url::from_str(request.uri().to_string().as_str()).unwrap();
        if let Some(top) = self.top.as_ref() {
            top.append_to_url_query(&mut url);
        };
        if let Some(filter) = self.filter.as_ref() {
            filter.append_to_url_query(&mut url);
        }
        if let Some(timeout) = self.timeout.as_ref() {
            timeout.append_to_url_query(&mut url);
        };
        http::Uri::from_str(url.to_string().as_str()).unwrap()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryTablesResponse {
    #[serde(rename = "value")]
    pub tables: Vec<super::Table>,
}

impl QueryTablesResponse {
    pub(crate) async fn try_from(response: Response) -> Result<Self, Error> {
        let body = azure_core::collect_pinned_stream(response.deconstruct().2).await?;
        let response = serde_json::from_slice(&body)?;
        Ok(response)
    }
}
