use azure_core::Header;

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

impl Header for ConditionAppendPosition {
    fn name(&self) -> &'static str {
        "x-ms-blob-condition-appendpos"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}
