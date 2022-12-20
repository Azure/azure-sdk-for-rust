/// TODO: move to azure_core?
#[derive(Debug)]
pub struct AzureNamedKeyCredential {
    name: String,
    key: String,
}

impl AzureNamedKeyCredential {
    /// Create a new instance of `AzureNamedKeyCredential`
    pub fn new(name: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            key: key.into(),
        }
    }

    /// Update the name and key
    pub fn update(&mut self, name: impl Into<String>, key: impl Into<String>) {
        self.name = name.into();
        self.key = key.into();
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the key
    pub fn key(&self) -> &str {
        &self.key
    }
}
