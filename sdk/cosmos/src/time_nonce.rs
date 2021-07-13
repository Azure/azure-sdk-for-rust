use chrono::{DateTime, Utc};

const TIME_FORMAT: &str = "%a, %d %h %Y %T GMT";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TimeNonce {
    time: DateTime<Utc>,
}

impl TimeNonce {
    pub fn new() -> Self {
        Self { time: Utc::now() }
    }
}

impl From<DateTime<Utc>> for TimeNonce {
    fn from(time: DateTime<Utc>) -> Self {
        Self { time }
    }
}

impl Default for TimeNonce {
    fn default() -> Self {
        Self::new()
    }
}

impl ToString for TimeNonce {
    fn to_string(&self) -> String {
        self.time.format(TIME_FORMAT).to_string()
    }
}
