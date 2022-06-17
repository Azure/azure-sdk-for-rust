use azure_core::error::{Error, ErrorKind, ResultExt};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopyProgress {
    pub bytes_copied: u64,
    pub bytes_total: u64,
}

impl fmt::Display for CopyProgress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.bytes_copied, self.bytes_total)
    }
}

impl FromStr for CopyProgress {
    type Err = Error;

    fn from_str(s: &str) -> azure_core::Result<Self> {
        let tokens = s.split('/').collect::<Vec<&str>>();
        if tokens.len() < 2 {
            return Err(Error::with_message(ErrorKind::DataConversion, || {
                format!("copy progress has insufficient tokens: {s}")
            }));
        }

        Ok(Self {
            bytes_copied: tokens[0]
                .parse()
                .with_context(ErrorKind::DataConversion, || {
                    format!("failed to parse bytes_copied from copy progress: {}", s)
                })?,
            bytes_total: tokens[1]
                .parse()
                .with_context(ErrorKind::DataConversion, || {
                    format!("failed to parse bytes_total from copy progress: {}", s)
                })?,
        })
    }
}

impl Serialize for CopyProgress {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for CopyProgress {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        s.parse()
            .map_err(|_| serde::de::Error::custom("Failed to deserialize CopyProgress"))
    }
}
