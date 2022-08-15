pub mod clients;
mod connection_string;
mod connection_string_builder;
mod copy_id;
mod copy_progress;
pub mod hmac;
mod macros;
pub mod prelude;
pub mod shared_access_signature;

pub use self::connection_string::{ConnectionString, EndpointProtocol};
pub use self::connection_string_builder::ConnectionStringBuilder;
pub mod headers;
pub use copy_id::{copy_id_from_headers, CopyId};
pub use copy_progress::CopyProgress;
pub mod parsing_xml;
pub mod storage_shared_key_credential;
mod stored_access_policy;
pub use azure_core::error::{Error, ErrorKind, ResultExt};
pub mod xml;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IPRange {
    pub start: std::net::IpAddr,
    pub end: std::net::IpAddr,
}

pub use stored_access_policy::{StoredAccessPolicy, StoredAccessPolicyList};

pub use consistency::{ConsistencyCRC64, ConsistencyMD5};

mod consistency {
    use azure_core::error::{Error, ErrorKind, ResultExt};
    use bytes::Bytes;
    use serde::{Deserialize, Deserializer};
    use std::{convert::TryInto, str::FromStr};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ConsistencyCRC64(Bytes);

    const CRC64_BYTE_LENGTH: usize = 8;

    impl ConsistencyCRC64 {
        /// Decodes from base64 encoded input
        pub fn decode(input: impl AsRef<[u8]>) -> azure_core::Result<Self> {
            let bytes = base64::decode(input).context(
                ErrorKind::DataConversion,
                "ConsistencyCRC64 failed base64 decode",
            )?;
            let bytes = Bytes::from(bytes);
            match bytes.len() {
                CRC64_BYTE_LENGTH => Ok(Self(bytes)),
                len => Err(Error::with_message(ErrorKind::Other, || {
                    format!("CRC64 not 8 bytes long. len: {len}")
                })),
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
        fn deserialize<D>(
            deserializer: D,
        ) -> std::result::Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de>,
        {
            let bytes = String::deserialize(deserializer)?;
            ConsistencyCRC64::decode(bytes).map_err(serde::de::Error::custom)
        }
    }

    impl FromStr for ConsistencyCRC64 {
        type Err = azure_core::error::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::decode(s)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ConsistencyMD5(Bytes);

    const MD5_BYTE_LENGTH: usize = 16;

    impl ConsistencyMD5 {
        /// Decodes from base64 encoded input
        pub fn decode(input: impl AsRef<[u8]>) -> azure_core::Result<Self> {
            let bytes = base64::decode(input)
                .context(ErrorKind::DataConversion, "ConsistencyMD5 failed decode")?;
            let bytes = Bytes::from(bytes);
            match bytes.len() {
                MD5_BYTE_LENGTH => Ok(Self(bytes)),
                len => Err(Error::with_message(ErrorKind::Other, || {
                    format!("MD5 digest not 16 bytes long. len: {len}")
                })),
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
        fn deserialize<D>(
            deserializer: D,
        ) -> std::result::Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de>,
        {
            let bytes = String::deserialize(deserializer)?;
            ConsistencyMD5::decode(bytes).map_err(serde::de::Error::custom)
        }
    }

    impl FromStr for ConsistencyMD5 {
        type Err = azure_core::error::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::decode(s)
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
