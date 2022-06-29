use crate::{headers, Header};

#[derive(Clone, Debug)]
pub struct Continuation(String);

impl From<String> for Continuation {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Header for Continuation {
    fn name(&self) -> headers::HeaderName {
        headers::CONTINUATION
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.clone().into()
    }
}
