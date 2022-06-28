use crate::{headers, request_options::NextMarker, request_options::Range, Header};
use std::ops::Range as StdRange;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Continuation {
    String(String),
    Paired(String, Option<String>),
    Range(StdRange<u64>),
}

impl From<NextMarker> for Continuation {
    fn from(next_marker: NextMarker) -> Self {
        Continuation::String(next_marker.as_str().to_string())
    }
}

impl From<&str> for Continuation {
    fn from(value: &str) -> Self {
        Continuation::String(value.to_string())
    }
}

impl From<(String, Option<String>)> for Continuation {
    fn from(value: (String, Option<String>)) -> Self {
        Continuation::Paired(value.0, value.1)
    }
}

impl From<String> for Continuation {
    fn from(value: String) -> Self {
        Continuation::String(value)
    }
}

impl From<StdRange<u64>> for Continuation {
    fn from(value: StdRange<u64>) -> Self {
        Continuation::Range(value)
    }
}

impl From<Range> for Continuation {
    fn from(value: Range) -> Self {
        Continuation::Range(value.start..value.end)
    }
}

impl Continuation {
    pub fn as_string(&self) -> String {
        match self {
            Self::String(c) => c.clone(),
            _ => {
                panic!("unsupported coontinuation type")
            }
        }
    }
}

impl Header for Continuation {
    fn name(&self) -> headers::HeaderName {
        headers::CONTINUATION
    }

    fn value(&self) -> headers::HeaderValue {
        self.as_string().into()
    }
}
