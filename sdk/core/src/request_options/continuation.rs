use crate::{headers, Header};

#[derive(Debug, Clone)]
pub struct Continuation(String);

impl Continuation {
    pub fn new(c: String) -> Self {
        Self(c)
    }

    pub fn into_raw(self) -> String {
        self.0
    }
}

impl Header for Continuation {
    fn name(&self) -> &'static str {
        headers::CONTINUATION
    }

    fn value(&self) -> String {
        self.0.to_owned()
    }
}
