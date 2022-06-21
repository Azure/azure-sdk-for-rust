use azure_core::headers::{self, Header};

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnEntity(bool);

impl ReturnEntity {
    pub fn new(s: impl Into<bool>) -> Self {
        Self(s.into())
    }
}

impl Header for ReturnEntity {
    fn name(&self) -> headers::HeaderName {
        "Prefer".into()
    }

    fn value(&self) -> headers::HeaderValue {
        match self.0 {
            true => "return-content",
            false => "return-no-content",
        }
        .into()
    }
}

impl From<bool> for ReturnEntity {
    fn from(s: bool) -> Self {
        Self::new(s)
    }
}
