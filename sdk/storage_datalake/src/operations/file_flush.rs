use azure_core::prelude::ContentLength;
use azure_core::prelude::IfMatchCondition;
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

use azure_core::{Request as HttpRequest, Response as HttpResponse};

#[derive(Debug, Clone, Default)]
pub struct FileFlushOptions {
    if_match_condition: Option<IfMatchCondition>,
}

impl FileFlushOptions {
    pub fn new() -> Self {
        Self {
            if_match_condition: None,
        }
    }

    setters! {
        if_match_condition: IfMatchCondition => Some(if_match_condition),
    }

    pub(crate) fn decorate_request(&self, req: &mut HttpRequest) -> Result<(), crate::Error> {
        azure_core::headers::add_optional_header2(&self.if_match_condition, req)?;
        azure_core::headers::add_mandatory_header2(&ContentLength::new(0), req)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FileFlushResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl FileFlushResponse {
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        let common_storage_response_headers = (&headers).try_into()?;

        Ok(Self {
            common_storage_response_headers,
        })
    }
}
