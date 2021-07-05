use azure_core::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConditionAppendPosition(u64);

impl ConditionAppendPosition {
    pub fn new(max_size: u64) -> Self {
        Self(max_size)
    }
}

impl From<u64> for ConditionAppendPosition {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl AddAsHeader for ConditionAppendPosition {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header("x-ms-blob-condition-appendpos", &format!("{}", self.0))
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), http::header::InvalidHeaderValue> {
        request.headers_mut().append(
            "x-ms-blob-condition-appendpos",
            http::header::HeaderValue::from_str(&self.0.to_string())?,
        );

        Ok(())
    }
}
