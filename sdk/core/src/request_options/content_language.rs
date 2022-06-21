use crate::headers::{self, Header};

#[derive(Debug, Clone, Copy)]
pub struct ContentLanguage<'a>(&'a str);

impl<'a, S> From<S> for ContentLanguage<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl<'a> Header for ContentLanguage<'a> {
    fn name(&self) -> headers::HeaderName {
        headers::CONTENT_LANGUAGE
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
