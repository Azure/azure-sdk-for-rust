use crate::headers::*;
use crate::Header;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum LeaseDuration {
    Infinite,
    Seconds(u8),
}

impl Header for LeaseDuration {
    fn name(&self) -> &'static str {
        LEASE_DURATION
    }

    fn value(&self) -> String {
        match self {
            LeaseDuration::Infinite => "-1".to_owned(),
            LeaseDuration::Seconds(seconds) => {
                format!("{}", seconds)
            }
        }
    }
}

impl From<Duration> for LeaseDuration {
    fn from(d: Duration) -> Self {
        LeaseDuration::Seconds(d.as_secs() as u8)
    }
}
