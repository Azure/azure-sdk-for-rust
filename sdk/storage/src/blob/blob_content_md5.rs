use azure_core::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlobContentMD5([u8; 16]);

impl AddAsHeader for BlobContentMD5 {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header("x-ms-blob-content-md5", base64::encode(self.0))
    }
}

impl From<md5::Digest> for BlobContentMD5 {
    fn from(md5: md5::Digest) -> Self {
        BlobContentMD5(md5.0)
    }
}
