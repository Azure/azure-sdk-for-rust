use azure_core::{
    base64,
    error::{Error, ErrorKind},
    headers::{self, Header, Headers, PROPERTIES},
};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Properties(BTreeMap<Cow<'static, str>, Cow<'static, str>>);

impl Default for Properties {
    fn default() -> Self {
        Self::new()
    }
}

impl From<BTreeMap<Cow<'static, str>, Cow<'static, str>>> for Properties {
    fn from(value: BTreeMap<Cow<'static, str>, Cow<'static, str>>) -> Self {
        Self(value)
    }
}

impl Properties {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert<K: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>>(
        &mut self,
        k: K,
        v: V,
    ) -> Option<Cow<'static, str>> {
        self.0.insert(k.into(), v.into())
    }

    pub fn get(&self, key: &str) -> std::option::Option<&Cow<'_, str>> {
        self.0.get(key)
    }
}

impl Header for Properties {
    fn name(&self) -> headers::HeaderName {
        PROPERTIES
    }

    fn value(&self) -> headers::HeaderValue {
        // the header is a comma separated list of key=base64(value) see
        // [https://docs.microsoft.com/rest/api/storageservices/datalakestoragegen2/filesystem/create#request-headers](https://docs.microsoft.com/rest/api/storageservices/datalakestoragegen2/filesystem/create#request-headers)
        self.0
            .iter()
            .map(|(k, v)| format!("{}={}", k.as_ref(), base64::encode(v.as_ref())))
            .collect::<Vec<_>>()
            .join(",")
            .into()
    }
}

impl TryFrom<&Headers> for Properties {
    type Error = azure_core::Error;

    fn try_from(headers: &Headers) -> Result<Self, Self::Error> {
        let header_value = headers.get_str(&PROPERTIES)?;
        Properties::try_from(header_value)
    }
}

impl TryFrom<&str> for Properties {
    type Error = azure_core::Error;

    fn try_from(header_value: &str) -> Result<Self, Self::Error> {
        let mut properties = Self::new();

        if header_value.is_empty() {
            return Ok(properties);
        }

        // this is probably too complicated. Should we split
        // it in more manageable code blocks?
        // The logic is this:
        // 1. Look for the header. If not found return error
        // 2. Split the header value by comma
        // 3. For each comma separated value:
        //      4. Split by equals. If we do not have at least 2 entries, return error.
        //      5. For each pair:
        //          6. Base64 decode the second entry (value). If error, return error.
        //          7. Insert the key value pair in the returned struct.
        header_value
            .split(',') // The list is a CSV so we split by comma
            .map(|key_value_pair| {
                let mut key_and_value = key_value_pair.split('='); // Each entry is key and value separated by =

                // we must have a key and a value (so two entries)
                let key = key_and_value
                    .next()
                    .ok_or_else(|| Error::message(ErrorKind::Other, "missing key"))?;
                let value = key_and_value
                    .next()
                    .ok_or_else(|| Error::message(ErrorKind::Other, "missing value"))?;

                // we do not check if there are more entries. We just ignore them.
                Ok((key, value))
            })
            .collect::<azure_core::Result<Vec<(&str, &str)>>>()? // if we have an error, return error
            .into_iter()
            .map(|(key, value)| {
                // the value is base64 encoded se we decode it
                let value = String::from_utf8(base64::decode(value)?)?;
                Ok((key, value))
            })
            .collect::<azure_core::Result<Vec<(&str, String)>>>()? // if we have an error, return error
            .into_iter()
            .for_each(|(key, value)| {
                properties.insert(key.to_owned(), value); // finally store the key and value into the properties
            });

        Ok(properties)
    }
}

impl FromStr for Properties {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}
