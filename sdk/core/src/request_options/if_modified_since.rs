use crate::headers::{self, Header};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy)]
pub struct IfModifiedSince(DateTime<Utc>);

impl IfModifiedSince {
    pub fn new(time: DateTime<Utc>) -> Self {
        Self(time)
    }
}

impl Header for IfModifiedSince {
    fn name(&self) -> headers::HeaderName {
        http::header::IF_MODIFIED_SINCE.into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_rfc2822().into()
    }
}
