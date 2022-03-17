use azure_core::Header;

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
    fn name(&self) -> &'static str {
        "x-ms-blob-condition-maxsize"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}
