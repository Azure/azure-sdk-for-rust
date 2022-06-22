use azure_core::headers::{self, Header};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConditionAppendPosition(u64);

impl ConditionAppendPosition {
    #[must_use]
    pub fn new(max_size: u64) -> Self {
        Self(max_size)
    }
}

impl From<u64> for ConditionAppendPosition {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl Header for ConditionAppendPosition {
    fn name(&self) -> headers::HeaderName {
        "x-ms-blob-condition-appendpos".into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_string().into()
    }
}
