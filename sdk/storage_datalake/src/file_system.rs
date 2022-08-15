use azure_core::Etag;
use bytes::Bytes;
use serde::{self, Deserialize, Deserializer};
use std::convert::TryFrom;
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileSystem {
    pub name: String,
    #[serde(with = "azure_core::date::rfc1123")]
    pub last_modified: OffsetDateTime,
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
    #[serde(with = "azure_core::date::rfc1123")]
    pub last_modified: OffsetDateTime,
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

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_path_serialization() {
        let payload = json!({
            "contentLength": 100_i64,
            "etag": "etag",
            "group": "group",
            "isDirectory": true,
            "lastModified": "Thu, 16 Jun 2022 10:22:59 GMT",
            "name": "name",
            "permissions": "permissions",
            "owner": "owner"
        });

        let path: Path = serde_json::from_slice(payload.to_string().as_ref()).unwrap();
        assert_eq!(path.content_length, 100);
        assert!(path.is_directory);

        let payload_str = json!({
            "contentLength": "100",
            "etag": "etag",
            "group": "group",
            "isDirectory": "true",
            "lastModified": "Thu, 16 Jun 2022 10:22:59 GMT",
            "name": "name",
            "permissions": "permissions",
            "owner": "owner"
        });

        let path: Path = serde_json::from_slice(payload_str.to_string().as_ref()).unwrap();
        assert_eq!(path.content_length, 100);
        assert!(path.is_directory);
    }
}
