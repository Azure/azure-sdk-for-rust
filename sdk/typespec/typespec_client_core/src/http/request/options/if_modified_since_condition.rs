// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(doc)]
use crate::http::request::Request;
use crate::{
    time::{self, OffsetDateTime},
    http::headers::{self, Header, HeaderName},
};

/// Sets the "if-modified-since" or "if-unmodified-since" headers in a [`Request`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfModifiedSinceCondition {
    Modified(OffsetDateTime),
    Unmodified(OffsetDateTime),
}

impl Header for IfModifiedSinceCondition {
    fn name(&self) -> HeaderName {
        match self {
            IfModifiedSinceCondition::Modified(_) => headers::IF_MODIFIED_SINCE,
            IfModifiedSinceCondition::Unmodified(_) => headers::IF_UNMODIFIED_SINCE,
        }
    }

    fn value(&self) -> headers::HeaderValue {
        match self {
            IfModifiedSinceCondition::Modified(date)
            | IfModifiedSinceCondition::Unmodified(date) => time::to_rfc7231(date),
        }
        .into()
    }
}
