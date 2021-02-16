use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopyProgress {
    pub bytes_copied: u64,
    pub bytes_total: u64,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CopyProgressParseError {
    #[error("number parse error")]
    Disconnect(#[from] std::num::ParseIntError),
    #[error("isufficient tokens error")]
    InsufficientTokens(),
}

impl fmt::Display for CopyProgress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.bytes_copied, self.bytes_total)
    }
}

impl FromStr for CopyProgress {
    type Err = CopyProgressParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split('/').collect::<Vec<&str>>();
        if tokens.len() < 2 {
            return Err(CopyProgressParseError::InsufficientTokens());
        }

        Ok(Self {
            bytes_copied: tokens[0].parse()?,
            bytes_total: tokens[1].parse()?,
        })
    }
}

impl Serialize for CopyProgress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for CopyProgress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        s.parse()
            .map_err(|e: CopyProgressParseError| serde::de::Error::custom(e.to_string()))
    }
}
