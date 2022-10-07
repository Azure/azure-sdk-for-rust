//! An assortment of helper utilities.

use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize,
};

pub fn case_insensitive_deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: DeserializeOwned + std::fmt::Debug,
    D: Deserializer<'de>,
{
    let v = String::deserialize(deserializer)?;
    T::deserialize(serde_json::Value::String(v.clone()))
        .or_else(|_| T::deserialize(serde_json::Value::String(v.to_lowercase())))
        .map_err(de::Error::custom)
}

/// Deserialize JSON null as default
/// https://github.com/serde-rs/serde/issues/1098
pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
