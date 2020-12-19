use azure_core::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConditionMaxSize(u64);

impl ConditionMaxSize {
    pub fn new(max_size: u64) -> Self {
        Self(max_size)
    }
}

impl AddAsHeader for ConditionMaxSize {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header("x-ms-blob-condition-maxsize", &format!("{}", self.0))
    }
}

impl From<u64> for ConditionMaxSize {
    fn from(n: u64) -> Self {
        Self(n)
    }
}
