use crate::prelude::*;
use azure_core::Request as HttpRequest;

#[derive(Debug, Clone, Default)]
pub struct ReplaceUserOptions {
    consistency_level: Option<ConsistencyLevel>,
}

impl ReplaceUserOptions {
    pub fn new() -> Self {
        Self {
            consistency_level: None,
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }

    pub(crate) fn decorate_request(
        &self,
        request: &mut HttpRequest,
        user_name: &str,
    ) -> Result<(), crate::Error> {
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;
        let body = ReplaceUserBody { id: user_name };
        request.set_body(bytes::Bytes::from(serde_json::to_string(&body)?).into());

        Ok(())
    }
}

#[derive(Serialize)]
struct ReplaceUserBody<'a> {
    id: &'a str,
}
