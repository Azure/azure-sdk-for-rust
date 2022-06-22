use crate::{headers, Header};

#[derive(Debug, Clone)]
pub struct Continuation(String);

impl Continuation {
    #[must_use]
    pub fn new(c: String) -> Self {
        Self(c)
    }

    #[must_use]
    pub fn into_raw(self) -> String {
        self.0
    }
}

impl Header for Continuation {
    fn name(&self) -> headers::HeaderName {
        headers::CONTINUATION.into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_owned().into()
    }
}
