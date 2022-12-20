/// Azure Shared Access Signature (SAS) credential
#[derive(Debug)]
pub struct AzureSasCredential(String);

impl AzureSasCredential {
    /// Create a new instance of `AzureSasCredential`
    pub fn new(signature: impl Into<String>) -> Self {
        Self(signature.into())
    }

    /// Get the signature
    pub fn signature(&self) -> &str {
        &self.0
    }

    /// Update the signature
    pub fn update(&mut self, signature: impl Into<String>) {
        self.0 = signature.into();
    }
}
