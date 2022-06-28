use azure_core::headers::{self, Header};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlobCacheControl(std::borrow::Cow<'static, str>);

impl BlobCacheControl {
    pub const fn from_static(s: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(s))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<&'static str> for BlobCacheControl {
    fn from(s: &'static str) -> Self {
        Self::from_static(s)
    }
}

impl From<String> for BlobCacheControl {
    fn from(s: String) -> Self {
        Self(std::borrow::Cow::Owned(s))
    }
}

impl From<&String> for BlobCacheControl {
    fn from(s: &String) -> Self {
        Self(std::borrow::Cow::Owned(s.clone()))
    }
}

impl Header for BlobCacheControl {
    fn name(&self) -> headers::HeaderName {
        azure_core::headers::BLOB_CACHE_CONTROL
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_string().into()
    }
}
