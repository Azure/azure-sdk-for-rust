use crate::headers::{self, Header};

#[derive(Debug, Clone, Copy)]
pub struct UserAgent<'a>(&'a str);

impl<'a> UserAgent<'a> {
    pub fn new(agent: &'a str) -> Self {
        Self(agent)
    }
}

impl<'a> Header for UserAgent<'a> {
    fn name(&self) -> headers::HeaderName {
        headers::USER_AGENT
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
