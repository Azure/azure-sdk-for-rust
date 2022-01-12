use azure_core::AddAsHeader;
use http::request::Builder;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct SourceContentMD5([u8; 16]);

impl From<md5::Digest> for SourceContentMD5 {
    fn from(md5: md5::Digest) -> Self {
        Self(md5.0)
    }
}

impl AddAsHeader for SourceContentMD5 {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header("x-ms-source-content-md5", base64::encode(self.0))
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HttpHeaderError> {
        request.headers_mut().append(
            "x-ms-source-content-md5",
            http::header::HeaderValue::from_str(&base64::encode(self.0))?,
        );

        Ok(())
    }
}
