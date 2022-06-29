use crate::{headers, Header};

#[derive(Clone, Debug)]
pub struct Continuation(std::borrow::Cow<'static, str>);

impl From<String> for Continuation {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

impl From<&'static str> for Continuation {
    fn from(s: &'static str) -> Self {
        Self(s.into())
    }
}

impl Header for Continuation {
    fn name(&self) -> headers::HeaderName {
        headers::CONTINUATION
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.clone().into_owned().into()
    }
}
