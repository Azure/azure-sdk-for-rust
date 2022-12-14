#[derive(Debug)]
pub struct AzureSasCredential(String);

impl AzureSasCredential {
    pub fn new(signature: impl Into<String>) -> Self {
        Self(signature.into())
    }

    pub fn signature(&self) -> &str {
        &self.0
    }

    pub fn update(&mut self, signature: impl Into<String>) {
        self.0 = signature.into();
    }
}
