use crate::{ErrorKind, Result, ResultExt};
use askama::Template;
use camino::Utf8Path;
use std::{fs::File, io::Write};

#[derive(Template)]
#[template(path = "publish-services.yml.jinja")]
pub struct PublishServicesYml<'a> {
    pub packages: &'a Vec<&'a str>,
}

pub fn render<T: Template>(template: &T, path: impl AsRef<Utf8Path>) -> Result<()> {
    let rendered = template.render().with_context(ErrorKind::Io, || "render {path}")?;
    let mut file = File::create(path.as_ref())?;
    write!(file, "{}", rendered)?;
    Ok(())
}

impl<'a> PublishServicesYml<'a> {
    pub fn create(&self, path: impl AsRef<Utf8Path>) -> Result<()> {
        render(self, path)
    }
}

#[derive(Template)]
#[template(path = "publish-sdks.yml.jinja")]
pub struct PublishSdksYml<'a> {
    pub packages: &'a Vec<&'a str>,
}

impl<'a> PublishSdksYml<'a> {
    pub fn create(&self, path: impl AsRef<Utf8Path>) -> Result<()> {
        render(self, path)
    }
}

#[derive(Template)]
#[template(path = "check-all-services.yml.jinja")]
pub struct CheckAllServicesYml<'a> {
    pub packages: &'a Vec<&'a str>,
}

impl<'a> CheckAllServicesYml<'a> {
    pub fn create(&self, path: impl AsRef<Utf8Path>) -> Result<()> {
        render(self, path)
    }
}

#[derive(Template)]
#[template(path = "Cargo.toml.jinja")]
pub struct CargoToml<'a> {
    pub package_name: &'a str,
    pub default_feature: &'a str,
    pub features: Vec<String>,
    pub azure_core_features: Vec<&'a str>,
}

impl<'a> CargoToml<'a> {
    pub fn create(&self, path: impl AsRef<Utf8Path>) -> Result<()> {
        render(self, path)
    }
}

#[derive(Template)]
#[template(path = "WorkspaceCargo.toml.jinja")]
pub struct WorkspaceCargoToml {
    pub dirs: Vec<String>,
}

impl WorkspaceCargoToml {
    pub fn create(&self, path: impl AsRef<Utf8Path>) -> Result<()> {
        render(self, path)
    }
}
