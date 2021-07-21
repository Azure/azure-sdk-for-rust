use crate::headers::*;
use crate::AddAsHeader;
use http::request::Builder;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum LeaseDuration {
    Infinite,
    Seconds(u8),
}

impl AddAsHeader for LeaseDuration {
    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            LeaseDuration::Infinite => builder.header(LEASE_DURATION, "-1"),
            LeaseDuration::Seconds(seconds) => {
                builder.header(LEASE_DURATION, &format!("{}", seconds))
            }
        }
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HTTPHeaderError> {
        let (header_name, header_value) = match self {
            LeaseDuration::Infinite => (LEASE_DURATION, -1),
            LeaseDuration::Seconds(seconds) => (LEASE_DURATION, *seconds as i32),
        };

        request
            .headers_mut()
            .append(header_name, http::HeaderValue::from(header_value));

        Ok(())
    }
}

impl From<Duration> for LeaseDuration {
    fn from(d: Duration) -> Self {
        LeaseDuration::Seconds(d.as_secs() as u8)
    }
}
