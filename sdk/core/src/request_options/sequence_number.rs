use crate::headers;
use crate::Header;

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
    fn name(&self) -> &'static str {
        headers::BLOB_SEQUENCE_NUMBER
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}
