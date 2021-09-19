#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MockTransaction {
    pub(crate) name: String,
    pub(crate) number: u32,
}

#[cfg(feature = "mock_transport_framework")]
impl MockTransaction {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            number: 0,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn number(&self) -> u32 {
        self.number
    }
}
