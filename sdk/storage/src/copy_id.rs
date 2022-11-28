use super::headers::COPY_ID;
use azure_core::error::{Error, ErrorKind, ResultExt};
use azure_core::headers::Headers;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopyId(uuid::Uuid);

pub fn copy_id_from_headers(headers: &Headers) -> azure_core::Result<CopyId> {
    headers.get_as(&COPY_ID)
}

impl From<uuid::Uuid> for CopyId {
    fn from(t: uuid::Uuid) -> Self {
        Self(t)
    }
}

impl FromStr for CopyId {
    type Err = Error;

    fn from_str(s: &str) -> azure_core::Result<Self> {
        let uuid = s
            .parse()
            .with_context(ErrorKind::DataConversion, || format!("malformed UUID: {s}"))?;
        Ok(Self(uuid))
    }
}

impl fmt::Display for CopyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for CopyId {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for CopyId {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: Error| serde::de::Error::custom(e.to_string()))
    }
}
