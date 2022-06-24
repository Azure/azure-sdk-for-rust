use crate::{headers, request_options::NextMarker, Header};
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Continuation {
    String(String),
    Range(Range<u64>),
}

impl Into<Continuation> for NextMarker {
    fn into(self) -> Continuation {
        Continuation::String(self.as_str().to_string())
    }
}

impl Into<Continuation> for &str {
    fn into(self) -> Continuation {
        Continuation::String(self.to_owned())
    }
}

impl Into<Continuation> for String {
    fn into(self) -> Continuation {
        Continuation::String(self)
    }
}

impl Into<Continuation> for Range<u64> {
    fn into(self) -> Continuation {
        Continuation::Range(self)
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
