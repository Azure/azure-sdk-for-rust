use azure_core::headers::{Header, HeaderName, HeaderValue};

#[derive(Debug, Clone)]
pub struct IfTagsCondition(String);

impl IfTagsCondition {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl From<String> for IfTagsCondition {
    fn from(n: String) -> Self {
        Self(n)
    }
}

impl Header for IfTagsCondition {
    fn name(&self) -> HeaderName {
        "x-ms-if-tags".into()
    }

    fn value(&self) -> HeaderValue {
        self.0.clone().into()
    }
}
