use crate::Header;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy)]
pub struct IfModifiedSince(DateTime<Utc>);

impl IfModifiedSince {
    pub fn new(time: DateTime<Utc>) -> Self {
        Self(time)
    }
}

impl Header for IfModifiedSince {
    fn name(&self) -> &'static str {
        http::header::IF_MODIFIED_SINCE.as_str()
    }

    fn value(&self) -> String {
        self.0.to_rfc2822()
    }
}
