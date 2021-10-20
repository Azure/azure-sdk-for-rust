use crate::AddAsHeader;
use http::request::Builder;
use http::HeaderMap;
use std::collections::HashMap;
use url::form_urlencoded;

#[derive(Debug, Clone)]
pub struct Tags(HashMap<String, String>);

impl Default for Tags {
    fn default() -> Self {
        Self::new()
    }
}

impl AsMut<HashMap<String, String>> for Tags {
    fn as_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.0
    }
}

impl Tags {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert<K, V>(&mut self, k: K, v: V) -> Option<String>
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.0.insert(k.into(), v.into())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, k: &str) -> Option<String> {
        self.0.get(k).cloned()
    }
}

impl AddAsHeader for &Tags {
    fn add_as_header(&self, builder: Builder) -> Builder {
        let mut encoded = form_urlencoded::Serializer::new(String::new());
        let mut encoder = &mut encoded;
        for (key, value) in self.0.iter() {
            encoder = encoder.append_pair(key, value);
        }
        builder.header("x-ms-tags", encoded.finish().to_string())
    }

    fn add_as_header2(
        &self,
        request: &mut crate::Request,
    ) -> Result<(), crate::errors::HTTPHeaderError> {
        let mut encoded = form_urlencoded::Serializer::new(String::new());
        let mut encoder = &mut encoded;
        for (key, value) in self.0.iter() {
            encoder = encoder.append_pair(key, value);
        }
        let header_name = http::header::HeaderName::from_bytes("x-ms-tags".as_bytes())?;
        let encoded_tags = encoded.finish();
        let header_value = http::header::HeaderValue::from_bytes(encoded_tags.as_bytes())?;
        request.headers_mut().append(header_name, header_value);

        Ok(())
    }
}

impl From<&HeaderMap> for Tags {
    fn from(header_map: &HeaderMap) -> Self {
        let mut tags = Tags::new();
        header_map
            .iter()
            .map(|header| (header.0.as_str(), header.1.as_bytes()))
            .filter(|(key, _)| key.starts_with("x-ms-tags"))
            .for_each(|(_key, value)| {
                let x: String = String::from_utf8(value.to_vec()).unwrap();
                let key_value_pair: Vec<&str> = x.split("=").collect::<Vec<_>>();
                tags.insert(
                    key_value_pair.get(0).unwrap().to_string(),
                    key_value_pair.get(1).unwrap().to_string(),
                );
            });

        tags
    }
}
