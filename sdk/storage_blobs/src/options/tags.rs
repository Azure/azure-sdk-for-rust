use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    headers::{Header, HeaderName, HeaderValue, TAGS},
};
use std::{
    collections::HashMap,
    fmt::Write,
    iter::{Extend, IntoIterator},
    str::FromStr,
};
use url::form_urlencoded;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
/// User-defined tags for specified blobs made up of one or more key-value
/// pairs.
///
/// The following limits apply to blob index tags:
/// * Each blob can have up to 10 tags
/// * Tag keys must be between one and 128 characters
/// * Tag values must be between zero and 256 characters
/// * Tag keys and values are case-sensitive
/// * Tag keys and values only support string data types. Any numbers, dates, times, or special characters are saved as strings
/// * Tag keys and values must adhere to the following naming rules:
///      * Alphanumeric characters:
///           * a through z (lowercase letters)
///           * A through Z (uppercase letters)
///           * 0 through 9 (numbers)
///           * Valid special characters: space, plus, minus, period, colon, equals, underscore, forward slash ( +-.:=_/)
///
/// Ref: https://docs.microsoft.com/en-us/azure/storage/blobs/storage-manage-find-blobs
pub struct Tags {
    pub tag_set: TagSet,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
pub struct TagSet {
    #[serde(default, rename = "Tag")]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    pub key: String,
    pub value: String,
}

impl Tags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.tag_set.tags.push(Tag {
            key: key.into(),
            value: value.into(),
        });
    }

    pub fn to_xml(&self) -> azure_core::Result<String> {
        let mut s = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?><Tags><TagSet>");
        for tag in &self.tag_set.tags {
            write!(
                &mut s,
                "<Tag><Key>{}</Key><Value>{}</Value></Tag>",
                tag.key, tag.value
            )
            .context(ErrorKind::DataConversion, "failed to write Tags xml")?;
        }
        s.push_str("</TagSet></Tags>");
        Ok(s)
    }
}

impl<K, V> Extend<(K, V)> for Tags
where
    K: Into<String>,
    V: Into<String>,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (K, V)>,
    {
        for (key, value) in iter {
            self.insert(key.into(), value.into());
        }
    }
}

impl IntoIterator for Tags {
    type Item = (String, String);
    type IntoIter = std::iter::Map<std::vec::IntoIter<Tag>, fn(Tag) -> (String, String)>;
    fn into_iter(self) -> Self::IntoIter {
        self.tag_set
            .tags
            .into_iter()
            .map(|tag| (tag.key, tag.value))
    }
}

impl From<HashMap<String, String>> for Tags {
    fn from(map: HashMap<String, String>) -> Self {
        let mut tags = Self::new();
        tags.extend(map);
        tags
    }
}

impl From<HashMap<&str, &str>> for Tags {
    fn from(map: HashMap<&str, &str>) -> Self {
        let mut tags = Self::new();
        tags.extend(map);
        tags
    }
}

impl From<Tags> for HashMap<String, String> {
    fn from(tags: Tags) -> Self {
        let mut map = Self::new();
        map.extend(tags);
        map
    }
}

impl FromStr for Tags {
    type Err = Error;
    fn from_str(value: &str) -> azure_core::Result<Tags> {
        let mut tags = Self::new();
        let pairs = form_urlencoded::parse(value.as_bytes());
        tags.extend(pairs);
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
        encoder.extend_pairs(self.clone());
        let encoded_tags = encoded.finish();
        encoded_tags.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_storage::xml::read_xml;

    #[test]
    fn parse_tags_xml() -> azure_core::Result<()> {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?><Tags><TagSet><Tag><Key>tag-name-1</Key><Value>tag-value-1</Value></Tag><Tag><Key>tag-name-2</Key><Value>tag-value-2</Value></Tag></TagSet></Tags>"#;
        let tags: Tags = read_xml(xml.as_bytes())?;
        assert_eq!(tags.tag_set.tags.len(), 2);
        assert_eq!(tags.tag_set.tags[0].key, "tag-name-1");
        assert_eq!(tags.tag_set.tags[0].value, "tag-value-1");
        assert_eq!(tags.tag_set.tags[1].key, "tag-name-2");
        assert_eq!(tags.tag_set.tags[1].value, "tag-value-2");
        let as_xml = tags.to_xml()?;
        assert_eq!(as_xml, xml);

        let empty = r#"<?xml version="1.0" encoding="utf-8"?><Tags><TagSet></TagSet></Tags>"#;
        let tags: Tags = read_xml(empty.as_bytes())?;
        assert_eq!(tags.tag_set.tags.len(), 0);
        let empty_as_xml = tags.to_xml()?;
        assert_eq!(empty_as_xml, empty);

        // verify parsing of self closing tags
        let empty = r#"<?xml version="1.0" encoding="utf-8"?><Tags><TagSet/></Tags>"#;
        let tags: Tags = read_xml(empty.as_bytes())?;
        assert_eq!(tags.tag_set.tags.len(), 0);

        Ok(())
    }
}
