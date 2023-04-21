use azure_core::headers::{self, Header};

#[derive(Debug, Clone)]
pub struct EncryptionScope(String);

impl Header for EncryptionScope {
    fn name(&self) -> headers::HeaderName {
        "x-ms-encryption-scope".into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.clone().into()
    }
}

impl From<String> for EncryptionScope {
    fn from(s: String) -> Self {
        Self(s)
    }
}
