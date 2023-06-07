use azure_core::{
    base64,
    headers::{self, Header},
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct SourceContentMD5(pub [u8; 16]);

#[cfg(feature = "md5")]
impl From<md5::Digest> for SourceContentMD5 {
    fn from(md5: md5::Digest) -> Self {
        Self(md5.0)
    }
}

impl From<[u8; 16]> for SourceContentMD5 {
    fn from(md5: [u8; 16]) -> Self {
        SourceContentMD5(md5)
    }
}

impl Header for SourceContentMD5 {
    fn name(&self) -> headers::HeaderName {
        "x-ms-source-content-md5".into()
    }

    fn value(&self) -> headers::HeaderValue {
        base64::encode(self.0).into()
    }
}
