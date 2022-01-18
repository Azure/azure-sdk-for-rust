use crate::prelude::*;

use azure_core::Request as HttpRequest;

#[derive(Debug, Clone)]
pub struct GetPermissionOptions {
    consistency_level: Option<ConsistencyLevel>,
}

impl GetPermissionOptions {
    pub fn new() -> Self {
        Self {
            consistency_level: None,
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }

    pub(crate) fn decorate_request(&self, request: &mut HttpRequest) -> crate::Result<()> {
        azure_core::headers::add_optional_header2(&self.consistency_level, request)?;

        Ok(())
    }
}
