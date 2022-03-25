use azure_core::headers::{self, Header};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConditionMaxSize(u64);

impl ConditionMaxSize {
    pub fn new(max_size: u64) -> Self {
        Self(max_size)
    }
}

impl From<u64> for ConditionMaxSize {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl Header for ConditionMaxSize {
    fn name(&self) -> headers::HeaderName {
        "x-ms-blob-condition-maxsize".into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_string().into()
    }
}
