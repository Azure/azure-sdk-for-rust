use crate::headers::{self, Header};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Accept<'a>(&'a str);

impl<'a> Accept<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }
}

impl<'a, S> From<S> for Accept<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl<'a> Header for Accept<'a> {
    fn name(&self) -> headers::HeaderName {
        http::header::ACCEPT.into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
