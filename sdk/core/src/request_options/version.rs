use crate::headers::{self, Header};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version(String);

impl Version {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

impl<S> From<S> for Version
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl Header for Version {
    fn name(&self) -> headers::HeaderName {
        headers::VERSION
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
