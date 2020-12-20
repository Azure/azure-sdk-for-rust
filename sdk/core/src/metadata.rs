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

    pub fn insert<K, V>(&mut self, k: K, v: V) -> Option<String>
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.0.insert(k.into(), v.into())
    }
}

impl AddAsHeader for &Metadata {
    fn add_as_header(&self, builder: Builder) -> Builder {
        let mut builder = builder;

        for (key, val) in self.0.iter() {
            builder = builder.header(&format!("x-ms-meta-{}", key), val);
        }

        builder
    }
}
