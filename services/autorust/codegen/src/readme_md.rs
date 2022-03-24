use crate::config_parser::Tag;
use askama::Template;
use camino::Utf8Path;
use std::{fs::File, io::Write};

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
