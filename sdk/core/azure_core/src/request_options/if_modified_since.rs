// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    date,
    headers::{self, Header},
};
use typespec_client_core::date::OffsetDateTime;

#[derive(Debug, Clone, Copy)]
pub struct IfModifiedSince(OffsetDateTime);

impl IfModifiedSince {
    pub fn new(time: OffsetDateTime) -> Self {
        Self(time)
    }
}

impl Header for IfModifiedSince {
    fn name(&self) -> headers::HeaderName {
        headers::IF_MODIFIED_SINCE
    }

    fn value(&self) -> headers::HeaderValue {
        date::to_rfc1123(&self.0).into()
    }
}

impl From<OffsetDateTime> for IfModifiedSince {
    fn from(time: OffsetDateTime) -> Self {
        Self::new(time)
    }
}
