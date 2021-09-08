use super::{header_time_value, header_value, ApiVersion, ETag};
use azure_core::{Error, Request};
use chrono::{Duration, Utc};

pub struct DeleteEntityOptions {
    etag: Option<ETag>,
    timeout: Option<Duration>,
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
        timeout: Duration => Some(timeout),
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
        if self.timeout.is_some() {
            // add timeout header;
        }
        Ok(())
    }
}
