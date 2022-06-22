use crate::headers::{self, Header};
use http::header;

#[derive(Debug, Clone, Copy)]
pub struct UserAgent<'a>(&'a str);

impl<'a> UserAgent<'a> {
    #[must_use]
    pub fn new(agent: &'a str) -> Self {
        Self(agent)
    }
}

impl<'a> Header for UserAgent<'a> {
    fn name(&self) -> headers::HeaderName {
        header::USER_AGENT.into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
