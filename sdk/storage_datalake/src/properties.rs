use azure_core::Header;
use http::HeaderMap;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Properties(BTreeMap<Cow<'static, str>, Cow<'static, str>>);

const HEADER: &str = "x-ms-properties";

impl Default for Properties {
    fn default() -> Self {
        Self::new()
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
    fn name(&self) -> &'static str {
        HEADER
    }

    fn value(&self) -> String {
        // the header is a comma separated list of key=base64(value) see
        // [https://docs.microsoft.com/rest/api/storageservices/datalakestoragegen2/filesystem/create#request-headers](https://docs.microsoft.com/rest/api/storageservices/datalakestoragegen2/filesystem/create#request-headers)
        self.0
            .iter()
            .map(|(k, v)| format!("{}={}", k.as_ref(), base64::encode(v.as_ref())))
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl TryFrom<&HeaderMap> for Properties {
    type Error = crate::Error;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        let mut properties = Self::new();

        let header_value = headers
            .get(HEADER)
            .ok_or_else(|| crate::Error::HeaderNotFound(HEADER.to_owned()))?
            .to_str()?;
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
                    .ok_or_else(|| crate::Error::GenericErrorWithText("missing key".to_owned()))?;
                let value = key_and_value.next().ok_or_else(|| {
                    crate::Error::GenericErrorWithText("missing value".to_owned())
                })?;

                // we do not check if there are more entries. We just ignore them.
                Ok((key, value))
            })
            .collect::<crate::Result<Vec<(&str, &str)>>>()? // if we have an error, return error
            .into_iter()
            .map(|(key, value)| {
                let value = std::str::from_utf8(&base64::decode(value)?)?.to_owned(); // the value is base64 encoded se we decode it
                Ok((key, value))
            })
            .collect::<crate::Result<Vec<(&str, String)>>>()? // if we have an error, return error
            .into_iter()
            .for_each(|(key, value)| {
                properties.insert(key.to_owned(), value); // finally store the key and value into the properties
            });

        Ok(properties)
    }
}
