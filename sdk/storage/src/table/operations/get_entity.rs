use std::str::FromStr;

use azure_core::{AppendToUrlQuery, Error, HTTPHeaderError, Request, Response};
use chrono::Utc;
use http::HeaderValue;
use reqwest::Url;

use super::{header_value, ApiVersion, OdataMetadataLevel, TableEntity};

#[derive(Debug, Clone)]
pub struct GetEntityOptions<'a> {
    api_version: Option<ApiVersion>,
    search_by: Option<SearchOption<'a>>,
    odata_metadata_level: Option<OdataMetadataLevel>,
}

#[derive(Debug, Clone)]
pub enum SearchOption<'a> {
    /// (partition_key, row_key)
    Identity(&'a str, &'a str),
    /// odata filter query
    Query {
        top: Option<crate::Top>,
        select: Option<crate::Select<'a>>,
        filter: Option<crate::Filter<'a>>,
    },
}

impl Default for GetEntityOptions<'_> {
    fn default() -> Self {
        Self {
            search_by: Default::default(),
            api_version: Some(ApiVersion::default()),
            odata_metadata_level: Some(OdataMetadataLevel::FullMetadata),
        }
    }
}

impl<'a> GetEntityOptions<'a> {
    setters! {
        api_version: ApiVersion => Some(api_version),
        search_by: SearchOption<'a> => Some(search_by),
        odata_metadata_level: OdataMetadataLevel => Some(odata_metadata_level),
    }

    //TODO: create HTTPUrlError or HTTPQueryError
    pub fn decorate_request_url(&self, request: &mut Request) -> Result<(), Error> {
        let mut url = Url::from_str(request.uri_mut().to_string().as_str())?;
        match self.search_by {
            Some(option) => match option {
                SearchOption::Identity(partition_key, row_key) => {
                    url.path_segments_mut()?.push(
                        format!("(PartitionKey='{}',RowKey='{}')", partition_key, row_key).as_str(),
                    );
                    Ok(())
                }
                SearchOption::Query {
                    top,
                    select,
                    filter,
                } => {
                    filter.append_to_url_query(&mut url);
                    select.append_to_url_query(&mut url);
                    top.append_to_url_query(&mut url);
                    Ok(())
                }
            },
            None => Error::UrlQueryParameterNotFound {
                expected_parameter: "partition and row key or odata filter condition".to_string(),
                url,
            },
        }
    }
    pub fn decorate_request_headers(&self, request: &mut Request) -> Result<(), HTTPHeaderError> {
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
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetEntityResponse<ENTITY> {
    #[serde(flatten)]
    pub model: ENTITY,
}
