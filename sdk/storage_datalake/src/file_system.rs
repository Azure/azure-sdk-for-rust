use azure_core::Etag;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer};
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileSystem {
    pub name: String,
    #[serde(with = "azure_core::parsing::rfc2822_time_format")]
    pub last_modified: DateTime<Utc>,
    pub etag: Etag,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FileSystemList {
    #[serde(rename = "filesystems")]
    pub file_systems: Vec<FileSystem>,
}

impl TryFrom<Bytes> for FileSystemList {
    type Error = crate::Error;

    fn try_from(response: Bytes) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice::<FileSystemList>(response.as_ref())?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    #[serde(deserialize_with = "deserialize_i64")]
    pub content_length: i64,
    pub etag: Etag,
    pub group: String,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub is_directory: bool,
    #[serde(with = "azure_core::parsing::rfc2822_time_format")]
    pub last_modified: DateTime<Utc>,
    pub name: String,
    pub owner: String,
    pub permissions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathList {
    pub paths: Vec<Path>,
}

impl TryFrom<Bytes> for PathList {
    type Error = azure_core::error::Error;

    fn try_from(response: Bytes) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(response.as_ref())?)
    }
}

fn deserialize_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = serde_json::Value::deserialize(deserializer)?;
    match s {
        serde_json::Value::String(str_val) => str_val.parse().map_err(serde::de::Error::custom),
        serde_json::Value::Number(num_val) => match num_val.as_i64() {
            Some(val) => Ok(val),
            None => Err(serde::de::Error::custom(format!(
                "could not convert {:?} to i64",
                num_val
            ))),
        },
        other => Err(serde::de::Error::custom(format!(
            "unexpected data format - expected string or number, got: {:?}",
            other
        ))),
    }
}

pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = serde_json::Value::deserialize(deserializer)?;
    match s {
        serde_json::Value::String(str_val) => str_val.parse().map_err(serde::de::Error::custom),
        serde_json::Value::Bool(bool_val) => Ok(bool_val),
        other => Err(serde::de::Error::custom(format!(
            "unexpected data format - expected string or bool, got: {:?}",
            other
        ))),
    }
}
