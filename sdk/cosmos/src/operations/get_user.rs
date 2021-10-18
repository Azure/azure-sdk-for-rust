use crate::prelude::*;
use azure_core::Request as HttpRequest;

#[derive(Debug, Clone, Default)]
pub struct GetUserOptions {
    consistency_level: Option<ConsistencyLevel>,
}

impl GetUserOptions {
    pub fn new() -> Self {
        Self {
            consistency_level: None,
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }

    pub(crate) fn decorate_request(&self, request: &mut HttpRequest) -> Result<(), crate::Error> {
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;
        request.set_body(bytes::Bytes::from_static(&[]).into());

        Ok(())
    }
}
