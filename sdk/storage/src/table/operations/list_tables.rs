use super::{header_time_value, header_value, ApiVersion, OdataMetadataLevel};
use crate::{Filter, Top};
use azure_core::{AppendToUrlQuery, Error, Request, Response};
use chrono::Utc;
use http::Uri;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ListTablesOptions<'a> {
    top: Option<Top>,
    filter: Option<Filter<'a>>,
    api_version: Option<ApiVersion>,
    next_table_name: Option<String>, // TODO: try use the next_table_name option
    odata_metadata_level: Option<OdataMetadataLevel>,
}

impl Default for ListTablesOptions<'_> {
    fn default() -> Self {
        Self {
            top: Default::default(),
            filter: Default::default(),
            next_table_name: Default::default(),
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

    pub(crate) fn query_parameters(&self) -> bool {
        self.top.is_some() || self.filter.is_some()
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

        if let Some(top) = self.top.as_ref() {
            let mut url = url::Url::from_str(request.uri().to_string().as_str()).unwrap();
            top.append_to_url_query(&mut url);
            *request.uri_mut() = http::Uri::from_str(url.to_string().as_str()).unwrap();
        };

        if let Some(filter) = self.filter.as_ref() {
            let mut url = url::Url::from_str(request.uri().to_string().as_str()).unwrap();
            filter.append_to_url_query(&mut url);
            *request.uri_mut() = Uri::from_str(url.to_string().as_str()).unwrap();
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListTablesResponse {
    ///If the number of tables to be returned exceeds 1,000 or the query does not complete within the timeout interval,
    ///next_table_name will containe the hash of the name of the next table in the list.
    pub next_table_name: Option<String>,
    pub body: ListTablesResponseBody,
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
pub struct ListTablesResponseBody {
    #[serde(rename = "odata.metadata")]
    pub odata_metadata: Option<String>,
    #[serde(rename = "value")]
    pub tables: Vec<Table>,
}

impl ListTablesResponseBody {
    pub(crate) async fn try_from(response: Response) -> Result<Self, Error> {
        let body = azure_core::collect_pinned_stream(response.deconstruct().2).await?;
        let response = serde_json::from_slice(&body)?;
        Ok(response)
    }
}
