use crate::AddAsHeader;
use bytes::Bytes;
use http::request::Builder;
use http::HeaderMap;
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
}

impl AddAsHeader for &Metadata {
    fn add_as_header(&self, builder: Builder) -> Builder {
        let mut builder = builder;

        for (key, val) in self.0.iter() {
            builder = builder.header(&format!("x-ms-meta-{}", key), val.as_ref());
        }

        builder
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), http::header::InvalidHeaderValue> {
        for (key, value) in self.0.iter() {
            let header_name =
                http::header::HeaderName::from_bytes(format!("x-ms-meta-{}", key).as_bytes())
                    .unwrap();
            let header_value = http::header::HeaderValue::from_bytes(value)?;

            request.headers_mut().append(header_name, header_value);
        }

        Ok(())
    }
}

impl From<&HeaderMap> for Metadata {
    fn from(header_map: &HeaderMap) -> Self {
        let mut metadata = Metadata::new();
        header_map
            .iter()
            .map(|header| (header.0.as_str(), header.1.as_bytes()))
            .filter(|(key, _)| key.starts_with("x-ms-meta-"))
            .for_each(|(key, value)| {
                metadata.insert(
                    key.strip_prefix("x-ms-meta-").unwrap().to_owned(),
                    value.to_owned(),
                );
            });

        metadata
    }
}
