use azure_core::headers::{self, Header};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlobContentEncoding(std::borrow::Cow<'static, str>);

impl BlobContentEncoding {
    pub const fn from_static(s: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(s))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<&'static str> for BlobContentEncoding {
    fn from(s: &'static str) -> Self {
        Self::from_static(s)
    }
}

impl From<String> for BlobContentEncoding {
    fn from(s: String) -> Self {
        Self(std::borrow::Cow::Owned(s))
    }
}

impl From<&String> for BlobContentEncoding {
    fn from(s: &String) -> Self {
        Self(std::borrow::Cow::Owned(s.clone()))
    }
}

impl Header for BlobContentEncoding {
    fn name(&self) -> headers::HeaderName {
        "x-ms-blob-content-encoding".into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_string().into()
    }
}
