use azure_core::AddAsHeader;
use http::request::Builder;

use crate::headers::{CONTENT_CRC64, CONTENT_MD5};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Hash {
    MD5([u8; 16]),
    CRC64(u64),
}

impl AddAsHeader for Hash {
    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            Hash::MD5(md5) => builder.header(CONTENT_MD5, base64::encode(md5)),
            Hash::CRC64(crc64) => builder.header(CONTENT_CRC64, &format!("{}", crc64)),
        }
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), http::header::InvalidHeaderValue> {
        let (header_name, header_value) = match self {
            Hash::MD5(md5) => (CONTENT_MD5, base64::encode(md5)),
            Hash::CRC64(crc64) => (CONTENT_CRC64, crc64.to_string()),
        };

        request.headers_mut().append(
            header_name,
            http::header::HeaderValue::from_str(&header_value)?,
        );

        Ok(())
    }
}

impl From<md5::Digest> for Hash {
    fn from(md5: md5::Digest) -> Self {
        Hash::MD5(md5.0)
    }
}
