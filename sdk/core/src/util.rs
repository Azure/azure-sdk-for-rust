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
/// <https://github.com/serde-rs/serde/issues/1098>
pub fn deserialize_null_as_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct SiteConfig {
        #[serde(
            rename = "appSettings",
            default,
            deserialize_with = "deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub app_settings: Vec<NameValuePair>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct NameValuePair {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    #[test]
    fn deserialize_empty() -> crate::Result<()> {
        let bytes = br#"{}"#;
        let site_config: SiteConfig = serde_json::from_slice(bytes)?;
        assert_eq!(Vec::<NameValuePair>::default(), site_config.app_settings);
        Ok(())
    }

    #[test]
    fn deserialize_null() -> crate::Result<()> {
        let bytes = br#"{ "appSettings": null }"#;
        let site_config: SiteConfig = serde_json::from_slice(bytes)?;
        assert_eq!(Vec::<NameValuePair>::default(), site_config.app_settings);
        Ok(())
    }
}
