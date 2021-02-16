use crate::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct SequenceNumber(u64);

impl SequenceNumber {
    pub fn new(max_results: u64) -> Self {
        Self(max_results)
    }
}

impl AddAsHeader for SequenceNumber {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(crate::BLOB_SEQUENCE_NUMBER, &format!("{}", self.0))
    }
}

impl From<u64> for SequenceNumber {
    fn from(max_results: u64) -> Self {
        Self::new(max_results)
    }
}
