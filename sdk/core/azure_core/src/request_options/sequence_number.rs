use crate::headers::{self, Header};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct SequenceNumber(u64);

impl SequenceNumber {
    pub fn new(max_results: u64) -> Self {
        Self(max_results)
    }
}

impl From<u64> for SequenceNumber {
    fn from(max_results: u64) -> Self {
        Self::new(max_results)
    }
}

impl Header for SequenceNumber {
    fn name(&self) -> headers::HeaderName {
        headers::BLOB_SEQUENCE_NUMBER
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_string().into()
    }
}
