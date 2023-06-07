use azure_core::{
    base64,
    headers::{self, Header, CONTENT_MD5},
};
use azure_storage::headers::CONTENT_CRC64;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Hash {
    MD5([u8; 16]),
    CRC64(u64),
}

impl Header for Hash {
    fn name(&self) -> headers::HeaderName {
        match self {
            Hash::MD5(_) => CONTENT_MD5,
            Hash::CRC64(_) => CONTENT_CRC64,
        }
    }

    fn value(&self) -> headers::HeaderValue {
        match self {
            Hash::MD5(md5) => base64::encode(md5),
            Hash::CRC64(crc64) => format!("{crc64}"),
        }
        .into()
    }
}

impl From<[u8; 16]> for Hash {
    fn from(md5: [u8; 16]) -> Self {
        Hash::MD5(md5)
    }
}

#[cfg(feature = "md5")]
impl From<md5::Digest> for Hash {
    fn from(md5: md5::Digest) -> Self {
        Hash::MD5(md5.0)
    }
}
