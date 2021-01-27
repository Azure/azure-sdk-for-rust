use crate::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct SourceContentMD5([u8; 16]);

impl AddAsHeader for SourceContentMD5 {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(crate::SOURCE_CONTENT_MD5, base64::encode(self.0))
    }
}

impl From<md5::Digest> for SourceContentMD5 {
    fn from(md5: md5::Digest) -> Self {
        Self(md5.0)
    }
}
