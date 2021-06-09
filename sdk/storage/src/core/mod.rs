pub mod clients;
mod connection_string;
mod connection_string_builder;
mod copy_id;
mod copy_progress;
mod errors;
mod into_azure_path;
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
pub use errors::Error;

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

use serde::{Deserialize, Deserializer};
pub use stored_access_policy::{StoredAccessPolicy, StoredAccessPolicyList};

#[derive(Debug, Clone, PartialEq)]
pub struct ConsistencyCRC64(pub [u8; 8]);

impl<'de> Deserialize<'de> for ConsistencyCRC64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)
            .and_then(|str| base64::decode(str).map_err(serde::de::Error::custom))
            .and_then(|content_crc64_vec| {
                if content_crc64_vec.len() != 8 {
                    return Err(serde::de::Error::custom(crate::Error::CRC64Not8BytesLong(
                        content_crc64_vec.len() as u64,
                    )));
                }
                let mut content_crc64 = [0; 8];
                content_crc64.copy_from_slice(&content_crc64_vec[0..8]);
                Ok(Self(content_crc64))
            })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsistencyMD5(pub [u8; 16]);

impl<'de> Deserialize<'de> for ConsistencyMD5 {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)
            .and_then(|str| base64::decode(str).map_err(serde::de::Error::custom))
            .and_then(|content_md5_vec| {
                if content_md5_vec.len() != 16 {
                    return Err(serde::de::Error::custom(
                        crate::Error::DigestNot16BytesLong(content_md5_vec.len() as u64),
                    ));
                }
                let mut content_md5 = [0; 16];
                content_md5.copy_from_slice(&content_md5_vec[0..16]);
                Ok(Self(content_md5))
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::de::value::{Error, StringDeserializer};
    use serde::de::IntoDeserializer;

    #[test]
    fn should_deserialize_consistency_crc64() {
        let deserializer: StringDeserializer<Error> =
            base64::encode([1, 2, 4, 8, 16, 32, 64, 128]).into_deserializer();
        let content_crc64 = ConsistencyCRC64::deserialize(deserializer).unwrap();
        assert_eq!(
            content_crc64,
            ConsistencyCRC64([1, 2, 4, 8, 16, 32, 64, 128])
        );
    }

    #[test]
    fn should_deserialize_consistency_md5() {
        let deserializer: StringDeserializer<Error> =
            base64::encode([1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128])
                .into_deserializer();
        let content_md5 = ConsistencyMD5::deserialize(deserializer).unwrap();
        assert_eq!(
            content_md5,
            ConsistencyMD5([1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128])
        );
    }
}
