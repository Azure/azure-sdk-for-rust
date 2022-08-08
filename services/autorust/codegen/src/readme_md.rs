use crate::Result;
use crate::{config_parser::Tag, jinja};
use askama::Template;
use camino::Utf8Path;
use std::collections::HashMap;

// https://djc.github.io/askama/

#[derive(Template)]
#[template(path = "readme.md.jinja")]
pub struct ReadmeMd<'a> {
    pub package_name: &'a str,
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
        jinja::render(self, path)
    }
}

pub fn url(path: &str) -> String {
    let url = path.replace('\\', "/");
    url.replace(
        "../../../azure-rest-api-specs/",
        "https://github.com/Azure/azure-rest-api-specs/blob/main/",
    )
}
