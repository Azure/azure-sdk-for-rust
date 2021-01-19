use crate::AddAsHeader;
use bytes::Bytes;
use http::request::Builder;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Metadata(HashMap<String, Bytes>);

impl Metadata {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn as_mut(&mut self) -> &mut HashMap<String, Bytes> {
        &mut self.0
    }

    pub fn insert<K, V>(&mut self, k: K, v: V) -> Option<Bytes>
    where
        K: Into<String>,
        V: Into<Bytes>,
    {
        self.0.insert(k.into(), v.into())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, k: &str) -> Option<Bytes> {
        self.0.get(k).map(|b| b.clone())
    }
}

impl AddAsHeader for &Metadata {
    fn add_as_header(&self, builder: Builder) -> Builder {
        let mut builder = builder;

        for (key, val) in self.0.iter() {
            builder = builder.header(&format!("x-ms-meta-{}", key), val.as_ref());
        }

        builder
    }
}
