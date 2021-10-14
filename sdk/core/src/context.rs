use chrono::{DateTime, Utc};

/// Pipeline execution context.
#[derive(Clone, Debug, Default)]
pub struct Context {
    timeout: Option<DateTime<Utc>>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_timeout_date(timeout: DateTime<Utc>) -> Self {
        Self {
            timeout: Some(timeout),
        }
    }

    pub fn with_timeout(maximum_duration: impl Into<chrono::Duration>) -> Self {
        Self {
            timeout: Some(Utc::now() + maximum_duration.into()),
        }
    }

    pub fn timeout(&self) -> Option<DateTime<Utc>> {
        self.timeout
    }
}
