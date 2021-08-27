use azure_core::{HTTPHeaderError, Request};
use chrono::Utc;
use http::HeaderValue;

use super::{header_value, ApiVersion};

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
        headers.append("Content-Type", HeaderValue::from_static("application/json"));
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
