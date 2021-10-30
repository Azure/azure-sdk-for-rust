use crate::table::{
    operations::{header_time_value, header_value},
    prelude::*,
};
use azure_core::{AppendToUrlQuery, HTTPHeaderError, Request};
use chrono::Utc;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct QueryEntityOptions {
    timeout: Option<Timeout>,
    api_version: Option<ApiVersion>,
    odata_metadata_level: Option<OdataMetadataLevel>,
}

impl Default for QueryEntityOptions {
    fn default() -> Self {
        Self {
            timeout: Default::default(),
            api_version: Some(ApiVersion::default()),
            odata_metadata_level: Some(OdataMetadataLevel::FullMetadata),
        }
    }
}

impl QueryEntityOptions {
    setters! {
        timeout: Timeout => Some(timeout),
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

        if let Some(timeout) = self.timeout.as_ref() {
            let mut url = url::Url::from_str(request.uri().to_string().as_str()).unwrap();
            timeout.append_to_url_query(&mut url);
            *request.uri_mut() = http::Uri::from_str(url.to_string().as_str()).unwrap()
        }

        Ok(())
    }
}
