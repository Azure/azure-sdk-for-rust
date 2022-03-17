use crate::Header;

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
    fn name(&self) -> &'static str {
        http::header::CONTENT_LANGUAGE.as_str()
    }

    fn value(&self) -> String {
        self.0.to_owned()
    }
}
