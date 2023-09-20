use azure_core::headers::{Header, HeaderName, HeaderValue, PREFER};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnEntity(bool);

impl ReturnEntity {
    pub fn new(s: impl Into<bool>) -> Self {
        Self(s.into())
    }
}

impl Header for ReturnEntity {
    fn name(&self) -> HeaderName {
        PREFER
    }

    fn value(&self) -> HeaderValue {
        if self.0 {
            "return-content"
        } else {
            "return-no-content"
        }
        .into()
    }
}

impl From<bool> for ReturnEntity {
    fn from(s: bool) -> Self {
        Self::new(s)
    }
}
