use crate::prelude::*;
use azure_core::Request as HttpRequest;

#[derive(Debug, Clone, Default)]
pub struct CreateUserOptions {
    consistency_level: Option<ConsistencyLevel>,
}

impl CreateUserOptions {
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
        let body = CreateUserBody { id: user_name };
        request.set_body(bytes::Bytes::from(serde_json::to_string(&body)?).into());
        Ok(())
    }
}

#[derive(Serialize, Debug)]
struct CreateUserBody<'a> {
    id: &'a str,
}
