use azure_core::headers::CommonStorageResponseHeaders;
use azure_core::prelude::IfMatchCondition;
use azure_core::prelude::{ContentLength, ContentType};
use bytes::Bytes;
use std::convert::TryInto;

use azure_core::{Request as HttpRequest, Response as HttpResponse};

#[derive(Debug, Clone, Default)]
pub struct FileAppendOptions<'a> {
    if_match_condition: Option<IfMatchCondition<'a>>,
}

impl<'a> FileAppendOptions<'a> {
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
        bytes: Bytes,
    ) -> Result<(), crate::Error> {
        azure_core::headers::add_optional_header2(&self.if_match_condition, req)?;
        azure_core::headers::add_mandatory_header2(
            &ContentType::new("application/octet-stream"),
            req,
        )?;
        azure_core::headers::add_mandatory_header2(&ContentLength::new(bytes.len() as i32), req)?;
        req.set_body(bytes.into());

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FileAppendResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl FileAppendResponse {
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        let common_storage_response_headers = (&headers).try_into()?;

        Ok(Self {
            common_storage_response_headers,
        })
    }
}
