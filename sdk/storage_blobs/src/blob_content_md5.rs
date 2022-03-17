use azure_core::Header;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlobContentMD5([u8; 16]);

impl From<md5::Digest> for BlobContentMD5 {
    fn from(md5: md5::Digest) -> Self {
        BlobContentMD5(md5.0)
    }
}

impl Header for BlobContentMD5 {
    fn name(&self) -> &'static str {
        "x-ms-blob-content-md5"
    }

    fn value(&self) -> String {
        base64::encode(self.0)
    }
}
