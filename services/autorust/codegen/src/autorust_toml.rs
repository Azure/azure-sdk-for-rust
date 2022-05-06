use crate::config_parser::Tag;
use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;
use std::{collections::HashSet, fs};

#[derive(Deserialize, Debug, Default)]
pub struct PackageConfig {
    pub tags_allow: Vec<String>,
}
impl<'a> PackageConfig {
    pub fn tags(&self, tags: Vec<&'a Tag>) -> Vec<&'a Tag> {
        if self.tags_allow.len() > 0 {
            let tags_allow: HashSet<&str> = self.tags_allow.iter().map(String::as_str).collect();
            tags.into_iter().filter(|tag| tags_allow.contains(tag.name())).collect()
        } else {
            tags
        }
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

    #[test]
    fn test_tags_allow() -> Result<(), Error> {
        let s = r#"tags_allow = ["package-2021-08", "package-2021-05"]"#;
        let config: PackageConfig = toml::from_str(s)?;
        assert_eq!(2, config.tags_allow.len());
        assert_eq!("package-2021-08", config.tags_allow[0]);
        assert_eq!("package-2021-05", config.tags_allow[1]);
        Ok(())
    }
}
