use crate::headers::{self, Header};

#[derive(Debug, Clone, Copy)]
pub struct ContentDisposition<'a>(&'a str);

impl<'a, S> From<S> for ContentDisposition<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl<'a> Header for ContentDisposition<'a> {
    fn name(&self) -> headers::HeaderName {
        headers::CONTENT_DISPOSITION
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
