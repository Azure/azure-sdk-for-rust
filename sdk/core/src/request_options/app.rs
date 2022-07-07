use crate::headers::{self, Header};

/// The (friendly) name of the application making the request
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
        headers::APP
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.clone().into()
    }
}
