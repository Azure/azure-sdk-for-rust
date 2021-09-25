use azure_core::prelude::IfMatchCondition;
use azure_core::prelude::ContentLength;
use http::StatusCode;

use azure_core::{Request as HttpRequest, Response as HttpResponse};

#[derive(Debug, Clone, Default)]
pub struct CreatePathOptions<'a> {
    if_match_condition: Option<IfMatchCondition<'a>>,
}

impl<'a> CreatePathOptions<'a> {
    pub fn new() -> Self {
        Self {
            if_match_condition: None,
        }
    }

    setters! {
        if_match_condition: IfMatchCondition<'a> => Some(if_match_condition),
    }

    pub(crate) fn decorate_request(&self, req: &mut HttpRequest) -> Result<(), crate::Error> {
        azure_core::headers::add_optional_header2(&self.if_match_condition, req)?;
        azure_core::headers::add_mandatory_header2(&ContentLength::new(0), req)?; // Length is required for creating files

        req.set_body(bytes::Bytes::new().into()); // TODO: needed?

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CreatePathResponse {
    pub status_code: StatusCode,
}

impl CreatePathResponse {
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let (status_code, _headers, _pinned_stream) = response.deconstruct();

        Ok(CreatePathResponse { status_code })
    }
}
