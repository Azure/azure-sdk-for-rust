use std::str::FromStr;

use crate::table::{
    operations::{header_time_value, header_value},
    prelude::*,
};
use azure_core::{AppendToUrlQuery, HTTPHeaderError, Request};
use chrono::Utc;
use http::HeaderValue;

#[derive(Debug, Clone)]
pub struct DeleteTableOptions {
    timeout: Option<Timeout>,
    api_version: Option<ApiVersion>,
}

impl Default for DeleteTableOptions {
    fn default() -> Self {
        Self {
            timeout: Default::default(),
            api_version: Some(ApiVersion::default()),
        }
    }
}

impl DeleteTableOptions {
    setters! {
        timeout: Timeout => Some(timeout),
        api_version: ApiVersion => Some(api_version),
    }

    pub fn decorate_request(&self, request: &mut Request) -> Result<(), HTTPHeaderError> {
        let headers = request.headers_mut();
        headers.append("x-ms-date", header_time_value(Utc::now())?);
        headers.append("Content-Type", HeaderValue::from_static("application/json"));
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteTableResponse {
    // do we want to return some of the headers to the client? x-ms-request-id, x-ms-version, Date or x-ms-client-request-id
}
