use crate::headers::*;
use crate::Header;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct LeaseBreakPeriod(Duration);

impl From<Duration> for LeaseBreakPeriod {
    fn from(duration: Duration) -> Self {
        Self(duration)
    }
}

impl Header for LeaseBreakPeriod {
    fn name(&self) -> &'static str {
        LEASE_BREAK_PERIOD
    }

    fn value(&self) -> String {
        format!("{}", self.0.as_secs())
    }
}
