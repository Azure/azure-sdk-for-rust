use crate::AddAsHeader;
use http::request::Builder;

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

impl AddAsHeader for SequenceNumber {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(crate::BLOB_SEQUENCE_NUMBER, &format!("{}", self.0))
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HTTPHeaderError> {
        request.headers_mut().append(
            crate::BLOB_SEQUENCE_NUMBER,
            http::HeaderValue::from_str(&format!("{}", self.0))?,
        );

        Ok(())
    }
}
