// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(doc)]
use crate::http::request::Request;
use crate::{
    http::headers::{self, Header},
    time::{self, OffsetDateTime},
};

/// Sets the "if-modified-since" header in a [`Request`].
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
        time::to_rfc7231(&self.0).into()
    }
}

impl From<OffsetDateTime> for IfModifiedSince {
    fn from(time: OffsetDateTime) -> Self {
        Self::new(time)
    }
}
