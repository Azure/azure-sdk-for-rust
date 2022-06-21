use super::headers::COPY_ID;
use azure_core::error::{Error, ErrorKind, ResultExt};
use azure_core::headers::Headers;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopyId(uuid::Uuid);

pub fn copy_id_from_headers(headers: &Headers) -> azure_core::Result<CopyId> {
    let copy_id = headers
        .get(COPY_ID)
        .ok_or_else(|| Error::message(ErrorKind::Other, "failed to get copy id from headers"))?;
    Ok(CopyId(uuid::Uuid::parse_str(copy_id.as_str()).context(
        ErrorKind::DataConversion,
        "failed to parse uuid from copy_id",
    )?))
}

impl From<uuid::Uuid> for CopyId {
    fn from(t: uuid::Uuid) -> Self {
        Self(t)
    }
}

impl TryFrom<&str> for CopyId {
    type Error = Error;

    fn try_from(s: &str) -> azure_core::Result<Self> {
        Ok(Self(
            s.parse().with_context(ErrorKind::DataConversion, || {
                format!("failed to parse CopyId from {s}")
            })?,
        ))
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
        let s = String::deserialize(deserializer)?;
        let uuid: uuid::Uuid = s
            .parse()
            .map_err(|e: uuid::Error| serde::de::Error::custom(e.to_string()))?;
        Ok(Self(uuid))
    }
}
