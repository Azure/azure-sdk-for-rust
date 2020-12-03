use crate::AddAsHeader;
use http::request::Builder;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Metadata(HashMap<String, String>);

impl Metadata {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn as_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.0
    }
}

impl From<HashMap<String, String>> for Metadata {
    fn from(metadata: HashMap<String, String>) -> Self {
        Self(metadata)
    }
}

impl AddAsHeader for &Metadata {
    fn add_as_header(&self, builder: Builder) -> Builder {
        let mut builder = builder;

        for (key, val) in self.0.iter() {
            builder = builder.header(&format!("x-ms-meta-{}", key) as &str, val as &str);
        }

        builder
    }
}
