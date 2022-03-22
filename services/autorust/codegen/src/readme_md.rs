use askama::Template;
use std::{fs::File, io::Write, path::Path};

pub type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(crate::IoError),
    #[error(transparent)]
    Askama(#[from] askama::Error),
}
impl<T: Into<crate::IoError>> From<T> for Error {
    fn from(error: T) -> Self {
        Self::Io(error.into())
    }
}

// https://djc.github.io/askama/

#[derive(Template)]
#[template(path = "readme.md.jinja")]
pub struct ReadmeMd<'a> {
    pub crate_name: &'a str,
}

impl<'a> ReadmeMd<'a> {
    pub fn create(&self, path: impl AsRef<Path>) -> Result<()> {
        let md = self.render()?;
        let mut file = File::create(path)?;
        write!(file, "{}", md)?;
        Ok(())
    }
}
