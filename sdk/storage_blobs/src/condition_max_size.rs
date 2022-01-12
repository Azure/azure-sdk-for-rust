use azure_core::AddAsHeader;
use http::request::Builder;

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

impl AddAsHeader for ConditionMaxSize {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header("x-ms-blob-condition-maxsize", &format!("{}", self.0))
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HttpHeaderError> {
        request.headers_mut().append(
            "x-ms-blob-condition-maxsize",
            http::header::HeaderValue::from(self.0),
        );

        Ok(())
    }
}
