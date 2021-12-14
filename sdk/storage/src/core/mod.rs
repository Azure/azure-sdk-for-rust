pub mod clients;
mod connection_string;
mod connection_string_builder;
mod copy_id;
mod copy_progress;
mod errors;
mod into_azure_path;
mod macros;
pub mod prelude;
pub mod shared_access_signature;

pub use self::connection_string::{ConnectionString, EndpointProtocol};
pub use self::connection_string_builder::ConnectionStringBuilder;
pub use self::into_azure_path::IntoAzurePath;
pub(crate) mod headers;
pub use copy_id::{copy_id_from_headers, CopyId};
pub use copy_progress::CopyProgress;
pub(crate) mod parsing_xml;
mod stored_access_policy;
pub use errors::{Error, Result};
pub(crate) mod util;
pub mod xml;

#[derive(Debug, Clone, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub struct Yes;
#[derive(Debug, Clone, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub struct No;

pub trait ToAssign: std::fmt::Debug {}
pub trait Assigned: ToAssign {}
pub trait NotAssigned: ToAssign {}

impl ToAssign for Yes {}
impl ToAssign for No {}

impl Assigned for Yes {}
impl NotAssigned for No {}

#[derive(Debug, Clone, PartialEq)]
pub struct IPRange {
    pub start: std::net::IpAddr,
    pub end: std::net::IpAddr,
}

pub use stored_access_policy::{StoredAccessPolicy, StoredAccessPolicyList};

pub use consistency::{ConsistencyCRC64, ConsistencyMD5};

mod consistency {
    use crate::Error;
    use bytes::Bytes;
    use serde::{Deserialize, Deserializer};
    use std::convert::TryInto;

    #[derive(Debug, Clone, PartialEq)]
    pub struct ConsistencyCRC64(Bytes);

    const CRC64_BYTE_LENGTH: usize = 8;

    impl ConsistencyCRC64 {
        /// Decodes from base64 encoded input
        pub fn decode(input: impl AsRef<[u8]>) -> crate::Result<Self> {
            let bytes = base64::decode(input).map_err(Error::Base64DecodeError)?;
            let bytes = Bytes::from(bytes);
            match bytes.len() {
                CRC64_BYTE_LENGTH => Ok(Self(bytes)),
                len => Err(Error::CRC64Not8BytesLong(len)),
            }
        }
        pub fn bytes(&self) -> &Bytes {
            &self.0
        }
        pub fn as_slice(&self) -> &[u8; CRC64_BYTE_LENGTH] {
            // we check the length when decoding, so this unwrap is safe
            self.0.as_ref().try_into().unwrap()
        }
    }

    impl AsRef<[u8; CRC64_BYTE_LENGTH]> for ConsistencyCRC64 {
        fn as_ref(&self) -> &[u8; CRC64_BYTE_LENGTH] {
            self.as_slice()
        }
    }

    impl<'de> Deserialize<'de> for ConsistencyCRC64 {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de>,
        {
            let bytes = String::deserialize(deserializer)?;
            ConsistencyCRC64::decode(bytes).map_err(serde::de::Error::custom)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct ConsistencyMD5(Bytes);

    const MD5_BYTE_LENGTH: usize = 16;

    impl ConsistencyMD5 {}

    impl ConsistencyMD5 {
        /// Decodes from base64 encoded input
        pub fn decode(input: impl AsRef<[u8]>) -> Result<Self, Error> {
            let bytes = base64::decode(input).map_err(Error::Base64DecodeError)?;
            let bytes = Bytes::from(bytes);
            match bytes.len() {
                MD5_BYTE_LENGTH => Ok(Self(bytes)),
                len => Err(Error::DigestNot16BytesLong(len)),
            }
        }
        pub fn bytes(&self) -> &Bytes {
            &self.0
        }
        pub fn as_slice(&self) -> &[u8; MD5_BYTE_LENGTH] {
            // we check the length when decoding, so this unwrap is safe
            self.0.as_ref().try_into().unwrap()
        }
    }

    impl AsRef<[u8; MD5_BYTE_LENGTH]> for ConsistencyMD5 {
        fn as_ref(&self) -> &[u8; MD5_BYTE_LENGTH] {
            self.as_slice()
        }
    }

    impl<'de> Deserialize<'de> for ConsistencyMD5 {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de>,
        {
            let bytes = String::deserialize(deserializer)?;
            ConsistencyMD5::decode(bytes).map_err(serde::de::Error::custom)
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use serde::de::value::{Error, StringDeserializer};
        use serde::de::IntoDeserializer;

        #[test]
        fn should_deserialize_consistency_crc64() {
            let input = base64::encode([1, 2, 4, 8, 16, 32, 64, 128]);
            let deserializer: StringDeserializer<Error> = input.into_deserializer();
            let content_crc64 = ConsistencyCRC64::deserialize(deserializer).unwrap();
            assert_eq!(
                content_crc64,
                ConsistencyCRC64(Bytes::from_static(&[1, 2, 4, 8, 16, 32, 64, 128]))
            );
        }

        #[test]
        fn should_deserialize_consistency_md5() {
            let input = base64::encode([1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128]);
            let deserializer: StringDeserializer<Error> = input.into_deserializer();
            let content_md5 = ConsistencyMD5::deserialize(deserializer).unwrap();
            assert_eq!(
                content_md5,
                ConsistencyMD5(Bytes::from_static(&[
                    1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128
                ]))
            );
        }
    }
}
