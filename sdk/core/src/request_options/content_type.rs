use crate::Header;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContentType<'a>(&'a str);

impl<'a> ContentType<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}

impl<'a, S> From<S> for ContentType<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl<'a> Header for ContentType<'a> {
    fn name(&self) -> &'static str {
        http::header::CONTENT_TYPE.as_str()
    }

    fn value(&self) -> String {
        self.0.to_owned()
    }
}
