use crate::config_parser::Tag;
use askama::Template;
use camino::Utf8Path;
use std::{collections::HashMap, fs::File, io::Write};

pub type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(crate::io::Error),
    #[error(transparent)]
    Askama(#[from] askama::Error),
}
impl<T: Into<crate::io::Error>> From<T> for Error {
    fn from(error: T) -> Self {
        Self::Io(error.into())
    }
}

// https://djc.github.io/askama/

#[derive(Template)]
#[template(path = "readme.md.jinja")]
pub struct ReadmeMd<'a> {
    pub crate_name: &'a str,
    pub readme_url: String,
    pub tags: &'a Vec<&'a Tag>,
    pub default_tag: &'a Tag,
    pub operation_totals: HashMap<&'a str, usize>,
    pub api_version_totals: HashMap<&'a str, usize>,
    pub api_versions: HashMap<&'a str, String>,
}

impl<'a> ReadmeMd<'a> {
    pub fn operation_total(&self, tag: &'a Tag) -> &usize {
        self.operation_totals.get(tag.name()).unwrap_or(&0)
    }
    pub fn api_version_total(&self, tag: &'a Tag) -> &usize {
        self.api_version_totals.get(tag.name()).unwrap_or(&0)
    }
    pub fn api_versions(&self, tag: &'a Tag) -> &str {
        self.api_versions.get(tag.name()).map(String::as_str).unwrap_or_default()
    }
}

impl<'a> ReadmeMd<'a> {
    pub fn create(&self, path: impl AsRef<Utf8Path>) -> Result<()> {
        let md = self.render()?;
        let mut file = File::create(path.as_ref())?;
        write!(file, "{}", md)?;
        Ok(())
    }
}

pub fn url(path: &str) -> String {
    let url = path.replace('\\', "/");
    url.replace(
        "../../../azure-rest-api-specs/",
        "https://github.com/Azure/azure-rest-api-specs/blob/main/",
    )
}
