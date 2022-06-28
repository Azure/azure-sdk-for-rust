use azure_core::headers::{self, Header};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlobContentDisposition(std::borrow::Cow<'static, str>);

impl BlobContentDisposition {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    pub const fn from_static(s: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(s))
    }
}

impl From<&'static str> for BlobContentDisposition {
    fn from(s: &'static str) -> Self {
        Self::from_static(s)
    }
}

impl From<String> for BlobContentDisposition {
    fn from(s: String) -> Self {
        Self(std::borrow::Cow::Owned(s))
    }
}

impl From<&String> for BlobContentDisposition {
    fn from(s: &String) -> Self {
        Self(std::borrow::Cow::Owned(s.clone()))
    }
}

impl Header for BlobContentDisposition {
    fn name(&self) -> headers::HeaderName {
        "x-ms-blob-content-disposition".into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_string().into()
    }
}
