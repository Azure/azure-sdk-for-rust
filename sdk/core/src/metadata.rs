use crate::AddAsHeader;
use http::request::Builder;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Metadata(HashMap<String, String>);

impl<'a> Metadata {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn as_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.0
    }
}

impl<'a> From<HashMap<String, String>> for Metadata {
    fn from(metadata: HashMap<String, String>) -> Self {
        Self(metadata)
    }
}

impl<'a> AddAsHeader for Metadata {
    fn add_as_header(&self, builder: Builder) -> Builder {
        let mut builder = builder;

        for (key, val) in self.0.iter() {
            builder = builder.header(&format!("x-ms-meta-{}", key) as &str, val as &str);
        }

        builder
    }
}

impl<'a> AddAsHeader for Option<Metadata> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        if let Some(item) = self {
            AddAsHeader::add_as_header(item, builder)
        } else {
            builder
        }
    }
}
