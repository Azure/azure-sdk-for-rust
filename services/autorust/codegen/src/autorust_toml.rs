use crate::config_parser::Tag;
use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;
use std::{collections::HashSet, fs};

/// `autorust.toml` files are used to configure code generation for a crate
#[derive(Deserialize, Debug, Default)]
pub struct PackageConfig {
    /// A section for configuring which tags are selected for code generation
    #[serde(default)]
    pub tags: Tags,

    /// A section for workarounds that apply to properties
    #[serde(default)]
    pub properties: Properties,
}

const NO_LIMIT: i32 = -1;

#[derive(Deserialize, Debug, Default)]
pub struct Tags {
    /// A list of tag names to filter for
    #[serde(default)]
    pub allow: Vec<String>,

    /// A list of tag names to filter out
    #[serde(default)]
    pub deny: Vec<String>,

    /// A list of strings for filtering out tag names
    /// If the tag names contains any of the strings, it is filtered out
    #[serde(default)]
    pub deny_contains: Vec<String>,

    /// Filter out any tag names that contain `preview`
    pub deny_contains_preview: Option<bool>,

    /// Filter out any tag names that contain `only`
    pub deny_contains_only: Option<bool>,

    /// Limit the number of tags for code generation
    pub limit: Option<i32>,

    /// Sort the tags alphabetically
    pub sort: Option<bool>,

    /// Choose which tag should be the default
    /// This overrides a default value set in the readme.md
    pub default: Option<String>,
}

impl Tags {
    pub fn new(
        allow: Vec<String>,
        deny: Vec<String>,
        deny_contains: Vec<String>,
        deny_contains_preview: Option<bool>,
        deny_contains_only: Option<bool>,
        limit: Option<i32>,
        sort: Option<bool>,
        default: Option<String>,
    ) -> Self {
        Self {
            allow,
            deny,
            deny_contains,
            deny_contains_preview,
            deny_contains_only,
            limit,
            sort,
            default,
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct Properties {
    /// Some properties need to have `Box<_>` added to avoid recursive types
    /// This is common in error definitions
    #[serde(default)]
    pub boxed: Vec<Vec<String>>,

    /// Some properties need to have `Option<_>` added
    #[serde(default)]
    pub optional: Vec<Vec<String>>,

    /// Enumeration cases are case sensitive by default
    /// This allows an enum type to be case insensitive
    #[serde(default)]
    pub fix_case: Vec<String>,

    /// Some properties need to be left as `serde_json::Value`
    #[serde(default)]
    pub invalid_type: Vec<Vec<String>>,
}

impl<'a> PackageConfig {
    pub fn default_tag(&self) -> Option<&str> {
        self.tags.default.as_deref()
    }

    /// Filter the tags based on the configuration
    pub fn filter_tags(&self, tags: Vec<&'a Tag>) -> Vec<&'a Tag> {
        let mut tags = tags.clone();
        if self.tags.sort.unwrap_or_default() {
            tags.sort_by_key(|tag| tag.name());
            tags.reverse();
        }
        if !self.tags.allow.is_empty() {
            let allow: HashSet<&str> = self.tags.allow.iter().map(String::as_str).collect();
            tags = tags.into_iter().filter(|tag| allow.contains(tag.name())).collect();
        }
        if !self.tags.deny.is_empty() {
            let deny: HashSet<&str> = self.tags.deny.iter().map(String::as_str).collect();
            tags = tags.into_iter().filter(|tag| !deny.contains(tag.name())).collect();
        }
        let mut deny_contains: Vec<&str> = self.tags.deny_contains.iter().map(String::as_str).collect();
        if self.tags.deny_contains_preview.unwrap_or_default() {
            deny_contains.push("preview");
        }
        if self.tags.deny_contains_only.unwrap_or_default() {
            deny_contains.push("only");
        }
        if !deny_contains.is_empty() {
            tags = tags
                .into_iter()
                .filter(|tag| !deny_contains.iter().any(|deny| tag.name().contains(deny)))
                .collect();
        }
        if let Some(limit) = self.tags.limit {
            if limit > NO_LIMIT {
                tags.truncate(limit as usize);
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
            "package-2022-03",
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
    fn deny_contains_only() -> Result<(), Error> {
        let tags = readme_tags();
        let len = tags.len();
        let tags = tags.iter().collect();

        let config: PackageConfig = toml::from_str(
            r#"
            [tags]
            deny_contains_only = true
            "#,
        )?;
        let tags = config.filter_tags(tags);
        assert!(len > tags.len());
        Ok(())
    }

    #[test]
    fn deny_contains_preview() -> Result<(), Error> {
        let tags = readme_tags();
        let len = tags.len();
        let tags = tags.iter().collect();

        let config: PackageConfig = toml::from_str(
            r#"
            [tags]
            deny_contains_preview = true
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

    #[test]
    fn sort() -> Result<(), Error> {
        let tags = readme_tags();
        let len = tags.len();
        let tags = tags.iter().collect();

        let config: PackageConfig = toml::from_str(
            r#"
            [tags]
            sort = true
            "#,
        )?;
        let tags = config.filter_tags(tags);
        assert_eq!(len, tags.len());
        assert_eq!("package-2022-03", tags[0].name());
        Ok(())
    }

    #[test]
    fn default() -> Result<(), Error> {
        let config: PackageConfig = toml::from_str(
            r#"
            [tags]
            default = "package-resources-2021-04"
            "#,
        )?;
        assert_eq!(Some("package-resources-2021-04".to_string()), config.tags.default);
        Ok(())
    }

    #[test]
    fn boxed() -> Result<(), Error> {
        let config: PackageConfig = toml::from_str(
            r#"
            [properties]
            boxed = [
                ["../../../azure-rest-api-specs/specification/applicationinsights/data-plane/Microsoft.Insights/preview/v1/AppInsights.json", "errorInfo", "innererror"]
              ]
            "#,
        )?;
        assert_eq!(1, config.properties.boxed.len());
        assert_eq!("errorInfo", config.properties.boxed[0][1]);
        assert_eq!("innererror", config.properties.boxed[0][2]);
        Ok(())
    }
}
