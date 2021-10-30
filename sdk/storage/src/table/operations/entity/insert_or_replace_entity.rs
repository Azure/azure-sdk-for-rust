use crate::table::{
    operations::{header_time_value, header_value},
    prelude::*,
};
use azure_core::{AppendToUrlQuery, Error, Request};
use chrono::Utc;
use http::HeaderValue;
use std::str::FromStr;

pub struct InsertOrReplaceEntityOptions {
    timeout: Option<Timeout>,
    api_version: Option<ApiVersion>,
}

impl Default for InsertOrReplaceEntityOptions {
    fn default() -> Self {
        Self {
            timeout: Default::default(),
            api_version: Some(ApiVersion::default()),
        }
    }
}

impl InsertOrReplaceEntityOptions {
    setters! {
        timeout: Timeout => Some(timeout),
        api_version: ApiVersion => Some(api_version),
    }

    pub fn decorate_request<'b, ENTITY: serde::Serialize + TableEntity<'b>>(
        &self,
        entity: &ENTITY,
        request: &mut Request,
    ) -> Result<(), Error> {
        let headers = request.headers_mut();
        headers.append("Content-Type", HeaderValue::from_static("application/json"));
        headers.append("x-ms-date", header_time_value(Utc::now())?);
        headers.append(
            "x-ms-version",
            header_value::<ApiVersion>(&self.api_version)?,
        );

        let serialized = serde_json::to_string(&entity)?;
        headers.append(
            "Content-Length",
            HeaderValue::from(serialized.as_bytes().len()),
        );
        request.set_body(bytes::Bytes::from(serialized).into());

        if let Some(timeout) = self.timeout.as_ref() {
            let mut url = url::Url::from_str(request.uri().to_string().as_str()).unwrap();
            timeout.append_to_url_query(&mut url);
            *request.uri_mut() = http::Uri::from_str(url.to_string().as_str()).unwrap();
        };

        Ok(())
    }
}
