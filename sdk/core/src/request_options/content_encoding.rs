use crate::Header;

#[derive(Debug, Clone, Copy)]
pub struct ContentEncoding<'a>(&'a str);

impl<'a, S> From<S> for ContentEncoding<'a>
where
    S: Into<&'a str>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl<'a> Header for ContentEncoding<'a> {
    fn name(&self) -> &'static str {
        http::header::CONTENT_ENCODING.as_str()
    }

    fn value(&self) -> String {
        self.0.to_owned()
    }
}
