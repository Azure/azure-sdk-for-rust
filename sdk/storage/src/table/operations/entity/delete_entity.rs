use std::str::FromStr;

use crate::table::{
    operations::{header_time_value, header_value},
    prelude::*,
};
use azure_core::{AppendToUrlQuery, Error, Request};
use chrono::Utc;

pub struct DeleteEntityOptions {
    etag: Option<ETag>,
    timeout: Option<Timeout>,
    api_version: Option<ApiVersion>,
}

impl Default for DeleteEntityOptions {
    fn default() -> Self {
        Self {
            etag: Some(ETag::default()),
            timeout: Default::default(),
            api_version: Some(ApiVersion::default()),
        }
    }
}

impl DeleteEntityOptions {
    setters! {
        etag: ETag => Some(etag),
        timeout: Timeout => Some(timeout),
        api_version: ApiVersion => Some(api_version),
    }

    pub fn decorate_request(&self, request: &mut Request) -> Result<(), Error> {
        let headers = request.headers_mut();
        headers.append("If-Match", header_value::<ETag>(&self.etag)?);
        headers.append("x-ms-date", header_time_value(Utc::now())?);
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
