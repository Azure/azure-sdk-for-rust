use crate::AddAsHeader;
use http::request::Builder;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Metadata<'a>(HashMap<Cow<'a, str>, Cow<'a, str>>);

impl<'a> Metadata<'a> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn as_mut(&mut self) -> &mut HashMap<Cow<'a, str>, Cow<'a, str>> {
        &mut self.0
    }

    pub fn insert<K, V>(&mut self, k: K, v: V) -> Option<Cow<'a, str>>
    where
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.0.insert(k.into(), v.into())
    }
}

impl From<HashMap<String, String>> for Metadata<'static> {
    fn from(metadata: HashMap<String, String>) -> Self {
        let mut s = Self(HashMap::with_capacity(metadata.capacity()));
        metadata.into_iter().for_each(|(k, v)| {
            s.insert(Cow::Owned(k), Cow::Owned(v));
        });
        s
    }
}

impl<'a> AddAsHeader for &Metadata<'a> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        let mut builder = builder;

        for (key, val) in self.0.iter() {
            builder = builder.header(&format!("x-ms-meta-{}", key) as &str, val as &str);
        }

        builder
    }
}
