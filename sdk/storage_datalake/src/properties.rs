use azure_core::AddAsHeader;
use http::request::Builder;
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

impl AddAsHeader for Properties {
    fn add_as_header(&self, builder: Builder) -> Builder {
        // the header is a comma separated list of key=base64(value) see
        // [https://docs.microsoft.com/rest/api/storageservices/datalakestoragegen2/filesystem/create#request-headers](https://docs.microsoft.com/rest/api/storageservices/datalakestoragegen2/filesystem/create#request-headers)
        let mut s = String::new();
        self.0.iter().for_each(|(k, v)| {
            s.push_str(&format!("{}={},", k.as_ref(), base64::encode(v.as_ref())));
        });

        // since we added a comma to the last entry, we will strip it to the exported header (this
        // is safe since we know that comma is 1 byte in UTF8):
        builder.header(HEADER, &s[..s.len() - 1])
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HttpHeaderError> {
        // the header is a comma separated list of key=base64(value) see
        // [https://docs.microsoft.com/rest/api/storageservices/datalakestoragegen2/filesystem/create#request-headers](https://docs.microsoft.com/rest/api/storageservices/datalakestoragegen2/filesystem/create#request-headers)

        let s = self
            .0
            .iter()
            .map(|(k, v)| format!("{}={}", k.as_ref(), base64::encode(v.as_ref())))
            .collect::<Vec<_>>()
            .join(",");

        request
            .headers_mut()
            .append(HEADER, http::header::HeaderValue::from_str(&s)?);

        Ok(())
    }
}

impl TryFrom<&HeaderMap> for Properties {
    type Error = crate::Error;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        let mut properties = Self::new();

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
        headers
            .get(HEADER)
            .ok_or_else(|| crate::Error::HeaderNotFound(HEADER.to_owned()))? // HEADER must exists or we return Err
            .to_str()?
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
