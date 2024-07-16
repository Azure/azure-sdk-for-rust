use crate::headers;
use crate::headers::Headers;
use crate::Header;
use bytes::Bytes;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Metadata(HashMap<String, Bytes>);

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}

impl AsMut<HashMap<String, Bytes>> for Metadata {
    fn as_mut(&mut self) -> &mut HashMap<String, Bytes> {
        &mut self.0
    }
}

impl Metadata {
    pub fn new() -> Self {
        Self(HashMap::new())
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

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, k: &str) -> Option<Bytes> {
        self.0.get(k).cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item = Metadatum> + '_ {
        self.0.iter().map(|(key, value)| {
            Metadatum(
                key.clone(),
                std::str::from_utf8(value)
                    .expect("non-utf8 header value")
                    .into(),
            )
        })
    }
}

#[derive(Debug)]
pub struct Metadatum(String, String);

impl Header for Metadatum {
    fn name(&self) -> headers::HeaderName {
        format!("x-ms-meta-{}", self.0).into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.1.clone().into()
    }
}

impl From<&Headers> for Metadata {
    fn from(header_map: &Headers) -> Self {
        let mut metadata = Metadata::new();
        header_map.iter().for_each(|(name, value)| {
            let name = name.as_str();
            let value = value.as_str();
            if let Some(name) = name.strip_prefix("x-ms-meta-") {
                metadata.insert(name.to_owned(), value.to_owned());
            }
        });

        metadata
    }
}
