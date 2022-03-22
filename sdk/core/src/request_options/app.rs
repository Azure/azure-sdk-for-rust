use crate::headers::{self, Header};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App(String);

impl App {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

impl<S> From<S> for App
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl Header for App {
    fn name(&self) -> headers::HeaderName {
        headers::APP.into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
