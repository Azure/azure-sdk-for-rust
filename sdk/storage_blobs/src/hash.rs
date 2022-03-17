use azure_core::Header;

use azure_storage::core::headers::{CONTENT_CRC64, CONTENT_MD5};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Hash {
    MD5([u8; 16]),
    CRC64(u64),
}

impl Header for Hash {
    fn name(&self) -> &'static str {
        match self {
            Hash::MD5(_) => CONTENT_MD5,
            Hash::CRC64(_) => CONTENT_CRC64,
        }
    }

    fn value(&self) -> String {
        match self {
            Hash::MD5(md5) => base64::encode(md5),
            Hash::CRC64(crc64) => format!("{}", crc64),
        }
    }
}

impl From<md5::Digest> for Hash {
    fn from(md5: md5::Digest) -> Self {
        Hash::MD5(md5.0)
    }
}
