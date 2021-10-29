use super::TableEntity;
use crate::table::prelude::{header_time_value, header_value, ApiVersion};
use azure_core::{Error, Request};
use chrono::Utc;
use http::HeaderValue;

pub struct InsertOrMergeEntityOptions {
    // timeout: Option<Duration>,
    api_version: Option<ApiVersion>,
}

impl Default for InsertOrMergeEntityOptions {
    fn default() -> Self {
        Self {
            // timeout: Default::default(),
            api_version: Some(ApiVersion::default()),
        }
    }
}

impl InsertOrMergeEntityOptions {
    setters! {
        // timeout: Duration => Some(timeout),
        api_version: ApiVersion => Some(api_version),
    }

    pub fn decorate_request<'b, ENTITY: serde::Serialize + TableEntity<'b>>(
        &self,
        request: &mut Request,
        entity: &ENTITY,
    ) -> Result<(), Error> {
        let headers = request.headers_mut();
        headers.append("x-ms-date", header_time_value(Utc::now())?);
        headers.append("Content-Type", HeaderValue::from_static("application/json"));
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
