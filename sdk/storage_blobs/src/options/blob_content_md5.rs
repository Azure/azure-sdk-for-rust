use azure_core::{
    base64,
    headers::{self, Header},
};
use azure_storage::ConsistencyMD5;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlobContentMD5(pub [u8; 16]);

#[cfg(feature = "md5")]
impl From<md5::Digest> for BlobContentMD5 {
    fn from(md5: md5::Digest) -> Self {
        BlobContentMD5(md5.0)
    }
}

impl From<[u8; 16]> for BlobContentMD5 {
    fn from(md5: [u8; 16]) -> Self {
        BlobContentMD5(md5)
    }
}

impl From<ConsistencyMD5> for BlobContentMD5 {
    fn from(md5: ConsistencyMD5) -> Self {
        BlobContentMD5(*md5.as_slice())
    }
}

impl Header for BlobContentMD5 {
    fn name(&self) -> headers::HeaderName {
        "x-ms-blob-content-md5".into()
    }

    fn value(&self) -> headers::HeaderValue {
        base64::encode(self.0).into()
    }
}
