use crate::headers::{self, Header};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentDisposition(std::borrow::Cow<'static, str>);

impl ContentDisposition {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    pub const fn from_static(s: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(s))
    }
}

impl From<&'static str> for ContentDisposition {
    fn from(s: &'static str) -> Self {
        Self::from_static(s)
    }
}

impl From<String> for ContentDisposition {
    fn from(s: String) -> Self {
        Self(std::borrow::Cow::Owned(s))
    }
}

impl From<&String> for ContentDisposition {
    fn from(s: &String) -> Self {
        Self(std::borrow::Cow::Owned(s.clone()))
    }
}

impl Header for ContentDisposition {
    fn name(&self) -> headers::HeaderName {
        headers::CONTENT_DISPOSITION
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_string().into()
    }
}
