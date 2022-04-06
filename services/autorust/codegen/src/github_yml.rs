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

#[derive(Template)]
#[template(path = "publish-services.yml.jinja")]
pub struct PublishServicesYml<'a> {
    pub packages: &'a Vec<&'a str>,
}

impl<'a> PublishServicesYml<'a> {
    pub fn create(&self, path: impl AsRef<Utf8Path>) -> Result<()> {
        let md = self.render()?;
        let mut file = File::create(path.as_ref())?;
        write!(file, "{}", md)?;
        Ok(())
    }
}

#[derive(Template)]
#[template(path = "check-all-services.yml.jinja")]
pub struct CheckAllServicesYml<'a> {
    pub packages: &'a Vec<&'a str>,
}

impl<'a> CheckAllServicesYml<'a> {
    pub fn create(&self, path: impl AsRef<Utf8Path>) -> Result<()> {
        let md = self.render()?;
        let mut file = File::create(path.as_ref())?;
        write!(file, "{}", md)?;
        Ok(())
    }
}
