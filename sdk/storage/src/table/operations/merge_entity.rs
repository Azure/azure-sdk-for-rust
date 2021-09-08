use azure_core::{Error, Request};
use chrono::{Duration, Utc};
use http::HeaderValue;

use super::{header_time_value, header_value, ApiVersion, ETag, TableEntity};

pub struct MergeEntityOptions {
    etag: Option<ETag>,
    timeout: Option<Duration>,
    api_version: Option<ApiVersion>,
}

impl Default for MergeEntityOptions {
    fn default() -> Self {
        Self {
            timeout: Default::default(),
            etag: Some(ETag::default()),
            api_version: Some(ApiVersion::default()),
        }
    }
}

impl MergeEntityOptions {
    setters! {
        etag: ETag => Some(etag),
        timeout: Duration => Some(timeout),
        api_version: ApiVersion => Some(api_version),
    }

    pub fn decorate_request<'b, ENTITY: serde::Serialize + TableEntity<'b>>(
        &self,
        request: &mut Request,
        entity: &ENTITY,
    ) -> Result<(), Error> {
        let headers = request.headers_mut();
        headers.append("Content-Type", HeaderValue::from_static("application/json"));
        headers.append("If-Match", header_value::<ETag>(&self.etag)?);
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
        Ok(())
    }
}
