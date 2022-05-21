use crate::config_parser::Tag;
use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;
use std::{collections::HashSet, fs};

#[derive(Deserialize, Debug, Default)]
pub struct PackageConfig {
    #[serde(default)]
    pub tags: Option<Tags>,
}

const NO_LIMIT: i32 = -1;

fn no_limit() -> i32 {
    NO_LIMIT
}

#[derive(Deserialize, Debug, Default)]
pub struct Tags {
    #[serde(default)]
    pub allow: Vec<String>,
    #[serde(default)]
    pub deny: Vec<String>,
    #[serde(default)]
    pub deny_contains: Vec<String>,
    #[serde(default = "no_limit")]
    pub limit: i32,
}

impl<'a> PackageConfig {
    /// Filter the tags based on the configuration
    pub fn filter_tags(&self, tags: Vec<&'a Tag>) -> Vec<&'a Tag> {
        let mut tags = tags.clone();
        if let Some(config) = &self.tags {
            if !config.allow.is_empty() {
                let allow: HashSet<&str> = config.allow.iter().map(String::as_str).collect();
                tags = tags.into_iter().filter(|tag| allow.contains(tag.name())).collect();
            }
            if !config.deny.is_empty() {
                let deny: HashSet<&str> = config.deny.iter().map(String::as_str).collect();
                tags = tags.into_iter().filter(|tag| !deny.contains(tag.name())).collect();
            }
            if !config.deny_contains.is_empty() {
                tags = tags
                    .into_iter()
                    .filter(|tag| !config.deny_contains.iter().any(|deny| tag.name().contains(deny)))
                    .collect();
            }
            if config.limit > NO_LIMIT {
                tags.truncate(config.limit as usize);
            }
        }
        tags
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to deserialize autorust.toml")]
    Deserialize(#[from] toml::de::Error),
    #[error(transparent)]
    Io(#[from] crate::io::Error),
}

/// Deserializes the autorust.toml into a PackageConfig
/// If the file does not exist, then returns a default instance
pub fn read(path: &Utf8Path) -> Result<PackageConfig> {
    if path.exists() {
        let bytes = fs::read(path).map_err(|source| crate::io::Error::ReadFile {
            source,
            file: Utf8PathBuf::from(path),
        })?;
        Ok(toml::from_slice(&bytes)?)
    } else {
        Ok(PackageConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn readme_tags() -> Vec<Tag> {
        let tags = vec![
            "package-2022-02-preview",
            "package-2021-09",
            "package-2021-08",
            "package-2021-05",
            "package-2021-04",
            "package-2021-02-preview-only",
            "package-2021-02",
        ];
        let tags: Vec<_> = tags.into_iter().map(Tag::new).collect();
        tags
    }

    #[test]
    fn empty_config() -> Result<(), Error> {
        let tags = readme_tags();
        let len = tags.len();
        let tags = tags.iter().collect();

        let config: PackageConfig = toml::from_str(
            r#"
            "#,
        )?;

        let tags = config.filter_tags(tags);
        assert_eq!(len, tags.len());
        Ok(())
    }

    #[test]
    fn allow() -> Result<(), Error> {
        let tags = readme_tags();
        let tags = tags.iter().collect();

        let config: PackageConfig = toml::from_str(
            r#"
            [tags]
            allow = ["package-2021-08", "package-2021-05"]
            "#,
        )?;
        let tags = config.filter_tags(tags);
        assert_eq!(2, tags.len());
        Ok(())
    }

    #[test]
    fn deny() -> Result<(), Error> {
        let tags = readme_tags();
        let len = tags.len();
        let tags = tags.iter().collect();

        let config: PackageConfig = toml::from_str(
            r#"
            [tags]
            deny = ["package-2021-08", "package-2021-05"]
            "#,
        )?;
        let tags = config.filter_tags(tags);
        assert_eq!(len - 2, tags.len());
        Ok(())
    }

    #[test]
    fn deny_contains() -> Result<(), Error> {
        let tags = readme_tags();
        let len = tags.len();
        let tags = tags.iter().collect();

        let config: PackageConfig = toml::from_str(
            r#"
            [tags]
            deny_contains = ["only"]
            "#,
        )?;
        let tags = config.filter_tags(tags);
        assert!(len > tags.len());
        Ok(())
    }

    #[test]
    fn limit() -> Result<(), Error> {
        let tags = readme_tags();
        let tags = tags.iter().collect();

        let config: PackageConfig = toml::from_str(
            r#"
            [tags]
            limit = 3
            "#,
        )?;
        let tags = config.filter_tags(tags);
        assert_eq!(3, tags.len());

        Ok(())
    }

    #[test]
    fn no_limit() -> Result<(), Error> {
        let tags = readme_tags();
        let len = tags.len();
        let tags = tags.iter().collect();

        let config: PackageConfig = toml::from_str(
            r#"
            [tags]
            limit = -1
            "#,
        )?;
        let tags = config.filter_tags(tags);
        assert_eq!(len, tags.len());

        Ok(())
    }
}
