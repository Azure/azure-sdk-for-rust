use crate::Header;

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
    fn name(&self) -> &'static str {
        http::header::CONTENT_DISPOSITION.as_str()
    }

    fn value(&self) -> String {
        self.0.to_owned()
    }
}
