use crate::{core::COPY_ID, AzureStorageError};
use http::HeaderMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopyId(uuid::Uuid);

pub fn copy_id_from_headers(headers: &HeaderMap) -> Result<CopyId, AzureStorageError> {
    let copy_id = headers
        .get(COPY_ID)
        .ok_or_else(|| AzureStorageError::HeaderNotFound(COPY_ID.to_owned()))?;
    Ok(CopyId(uuid::Uuid::parse_str(copy_id.to_str()?)?))
}

impl From<uuid::Uuid> for CopyId {
    fn from(t: uuid::Uuid) -> Self {
        Self(t)
    }
}

impl TryFrom<&str> for CopyId {
    type Error = uuid::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(s.parse()?))
    }
}

impl fmt::Display for CopyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for CopyId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for CopyId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let uuid: uuid::Uuid = s
            .parse()
            .map_err(|e: uuid::Error| serde::de::Error::custom(e.to_string()))?;
        Ok(Self(uuid))
    }
}
