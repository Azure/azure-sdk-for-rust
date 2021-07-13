use azure_core::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BlobContentMD5([u8; 16]);

impl From<md5::Digest> for BlobContentMD5 {
    fn from(md5: md5::Digest) -> Self {
        BlobContentMD5(md5.0)
    }
}

impl AddAsHeader for BlobContentMD5 {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header("x-ms-blob-content-md5", base64::encode(self.0))
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HTTPHeaderError> {
        request.headers_mut().append(
            "x-ms-blob-content-md5",
            http::header::HeaderValue::from_str(&base64::encode(self.0))?,
        );

        Ok(())
    }
}
