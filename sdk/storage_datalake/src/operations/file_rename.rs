use azure_core::headers::{etag_from_headers, last_modified_from_headers};
use azure_core::prelude::*;
use azure_storage::core::headers::CommonStorageResponseHeaders;
use chrono::{DateTime, Utc};
use std::convert::TryInto;

use azure_core::{HttpHeaderError, Request as HttpRequest, Response as HttpResponse};

#[derive(Debug, Clone, Default)]
pub struct FileRenameOptions<'a> {
    if_match_condition: Option<IfMatchCondition<'a>>,
}

impl<'a> FileRenameOptions<'a> {
    pub fn new() -> Self {
        Self {
            if_match_condition: None,
        }
    }

    setters! {
        if_match_condition: IfMatchCondition<'a> => Some(if_match_condition),
    }

    pub(crate) fn decorate_request(
        &self,
        req: &mut HttpRequest,
        rename_source: &str,
    ) -> Result<(), crate::Error> {
        azure_core::headers::add_optional_header2(&self.if_match_condition, req)?;
        azure_core::headers::add_mandatory_header2(&ContentLength::new(0), req)?; // Length is required for renaming files
        req.headers_mut().append(
            "x-ms-rename-source",
            http::HeaderValue::from_str(rename_source)
                .map_err(HttpHeaderError::InvalidHeaderValue)?,
        );

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FileRenameResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: String,
    pub last_modified: DateTime<Utc>,
}

impl FileRenameResponse {
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        let common_storage_response_headers = (&headers).try_into()?;
        let etag = etag_from_headers(&headers)?;
        let last_modified = last_modified_from_headers(&headers)?;

        Ok(Self {
            common_storage_response_headers,
            etag,
            last_modified,
        })
    }
}
