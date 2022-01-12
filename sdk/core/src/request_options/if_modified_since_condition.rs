use crate::AddAsHeader;
use chrono::{DateTime, Utc};
use http::header::{IF_MODIFIED_SINCE, IF_UNMODIFIED_SINCE};
use http::request::Builder;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IfModifiedSinceCondition {
    Modified(DateTime<Utc>),
    Unmodified(DateTime<Utc>),
}

impl AddAsHeader for IfModifiedSinceCondition {
    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            IfModifiedSinceCondition::Modified(date) => {
                builder.header(IF_MODIFIED_SINCE, &date.to_rfc2822() as &str)
            }
            IfModifiedSinceCondition::Unmodified(date) => {
                builder.header(IF_UNMODIFIED_SINCE, &date.to_rfc2822() as &str)
            }
        }
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HttpHeaderError> {
        match self {
            IfModifiedSinceCondition::Modified(date) => {
                request.headers_mut().append(
                    IF_MODIFIED_SINCE,
                    http::HeaderValue::from_str(&date.to_rfc2822() as &str)?,
                );
            }
            IfModifiedSinceCondition::Unmodified(date) => {
                request.headers_mut().append(
                    IF_UNMODIFIED_SINCE,
                    http::HeaderValue::from_str(&date.to_rfc2822() as &str)?,
                );
            }
        }

        Ok(())
    }
}
