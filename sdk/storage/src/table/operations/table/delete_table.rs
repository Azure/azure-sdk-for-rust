use azure_core::{HTTPHeaderError, Request};
use chrono::Utc;
use http::HeaderValue;

use crate::operations::{header_time_value, header_value, ApiVersion};

#[derive(Debug, Clone)]
pub struct DeleteTableOptions {
    api_version: Option<ApiVersion>,
}

impl Default for DeleteTableOptions {
    fn default() -> Self {
        Self {
            api_version: Some(ApiVersion::default()),
        }
    }
}

impl DeleteTableOptions {
    setters! {
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
        Ok(())
    }
}

pub struct DeleteTableResponse {
    // do we want to return some of the headers to the client? x-ms-request-id, x-ms-version, Date or x-ms-client-request-id
}
