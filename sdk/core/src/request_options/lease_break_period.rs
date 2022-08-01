use crate::headers::{self, Header};
use time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct LeaseBreakPeriod(Duration);

impl From<Duration> for LeaseBreakPeriod {
    fn from(duration: Duration) -> Self {
        Self(duration)
    }
}

impl Header for LeaseBreakPeriod {
    fn name(&self) -> headers::HeaderName {
        headers::LEASE_BREAK_PERIOD
    }

    fn value(&self) -> headers::HeaderValue {
        format!("{}", self.0.whole_seconds()).into()
    }
}
