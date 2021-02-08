use crate::headers::*;
use crate::AddAsHeader;
use http::request::Builder;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct LeaseBreakPeriod(Duration);

impl AddAsHeader for LeaseBreakPeriod {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(LEASE_BREAK_PERIOD, &format!("{}", self.0.as_secs()))
    }
}

impl From<Duration> for LeaseBreakPeriod {
    fn from(duration: Duration) -> Self {
        Self(duration)
    }
}
