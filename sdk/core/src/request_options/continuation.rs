use crate::{headers, request_options::NextMarker, Header};
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Continuation {
    String(String),
    Range(Range<u64>),
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

impl From<String> for Continuation {
    fn from(value: String) -> Self {
        Continuation::String(value)
    }
}

impl From<Range<u64>> for Continuation {
    fn from(value: Range<u64>) -> Self {
        Continuation::Range(value)
    }
}

impl Continuation {
    pub fn as_string(&self) -> String {
        match self {
            Self::String(c) => c.clone(),
            Self::Range(_) => {
                panic!("unable to convert Continuation::Range to string")
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
