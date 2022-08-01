use crate::headers::{self, Header};
use time::Duration;

#[derive(Debug, Clone)]
pub enum LeaseDuration {
    Infinite,
    Seconds(u8),
}

impl Header for LeaseDuration {
    fn name(&self) -> headers::HeaderName {
        headers::LEASE_DURATION
    }

    fn value(&self) -> headers::HeaderValue {
        match self {
            LeaseDuration::Infinite => "-1".to_owned(),
            LeaseDuration::Seconds(seconds) => {
                format!("{}", seconds)
            }
        }
        .into()
    }
}

impl From<Duration> for LeaseDuration {
    fn from(d: Duration) -> Self {
        LeaseDuration::Seconds(d.whole_seconds() as u8)
    }
}
