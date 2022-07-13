use azure_core::{
    error::Error,
    headers::{Header, HeaderName, HeaderValue, TAGS},
};
use std::{collections::HashMap, str::FromStr};
use url::form_urlencoded;

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Tags {
    pub tag_set: Option<TagSet>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TagSet {
    pub tag: Vec<Tag>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    pub key: String,
    pub value: String,
}

impl Tags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<S>(&mut self, key: S, value: S)
    where
        S: Into<String>,
    {
        if self.tag_set.is_none() {
            self.tag_set = Some(TagSet { tag: vec![] });
        }

        let tag_set = self.tag_set.as_mut().unwrap();
        tag_set.tag.push(Tag {
            key: key.into(),
            value: value.into(),
        });
    }

    pub fn to_xml(&self) -> String {
        let mut s = String::new();
        s.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<Tags>\n<TagSet>\n");
        if let Some(tag_set) = &self.tag_set {
            for tag in &tag_set.tag {
                let node = format!(
                    "\t<Tag><Key>{}</Key><Value>{}</Value></Tag>\n",
                    tag.key, tag.value
                );
                s.push_str(&node);
            }
        }

        s.push_str("</TagSet></Tags>");
        s
    }
}

impl From<HashMap<String, String>> for Tags {
    fn from(map: HashMap<String, String>) -> Self {
        let mut tags = Self::new();
        for (key, value) in map {
            tags.insert(key, value);
        }
        tags
    }
}

impl From<HashMap<&str, &str>> for Tags {
    fn from(map: HashMap<&str, &str>) -> Self {
        let mut tags = Self::new();
        for (key, value) in map {
            tags.insert(key, value);
        }
        tags
    }
}

impl From<Tags> for HashMap<String, String> {
    fn from(tags: Tags) -> Self {
        let mut map = Self::new();
        if let Some(tag_set) = &tags.tag_set {
            for tag in &tag_set.tag {
                map.insert(tag.key.clone(), tag.value.clone());
            }
        }
        map
    }
}

impl FromStr for Tags {
    type Err = Error;
    fn from_str(value: &str) -> azure_core::Result<Tags> {
        let mut tags = Self::new();
        let pairs = form_urlencoded::parse(value.as_bytes());
        for (key, value) in pairs {
            tags.insert(key.to_owned(), value.to_owned());
        }
        Ok(tags)
    }
}

impl Header for Tags {
    fn name(&self) -> HeaderName {
        TAGS
    }

    fn value(&self) -> HeaderValue {
        let mut encoded = form_urlencoded::Serializer::new(String::new());
        let encoder = &mut encoded;
        if let Some(tag_set) = &self.tag_set {
            for tag in &tag_set.tag {
                encoder.append_pair(&tag.key, &tag.value);
            }
        }
        let encoded_tags = encoded.finish();
        encoded_tags.into()
    }
}
