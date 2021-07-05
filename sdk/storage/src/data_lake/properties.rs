use azure_core::AddAsHeader;
use http::request::Builder;
use http::HeaderMap;
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Properties<'a, 'b>(HashMap<Cow<'a, str>, Cow<'b, str>>);

const HEADER: &str = "x-ms-properties";

impl<'a, 'b> Default for Properties<'a, 'b> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, 'b> Properties<'a, 'b> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert<K: Into<Cow<'a, str>>, V: Into<Cow<'b, str>>>(
        &mut self,
        k: K,
        v: V,
    ) -> Option<Cow<'b, str>> {
        self.0.insert(k.into(), v.into())
    }

    pub fn hash_map(&self) -> &HashMap<Cow<'a, str>, Cow<'b, str>> {
        &self.0
    }
}

impl<'a, 'b> AddAsHeader for Properties<'a, 'b> {
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
    ) -> Result<(), http::header::InvalidHeaderValue> {
        // the header is a comma separated list of key=base64(value) see
        // [https://docs.microsoft.com/rest/api/storageservices/datalakestoragegen2/filesystem/create#request-headers](https://docs.microsoft.com/rest/api/storageservices/datalakestoragegen2/filesystem/create#request-headers)
        let mut s = String::new();
        self.0.iter().for_each(|(k, v)| {
            s.push_str(&format!("{}={},", k.as_ref(), base64::encode(v.as_ref())));
        });

        // since we added a comma to the last entry, we will strip it to the exported header (this
        // is safe since we know that comma is 1 byte in UTF8):
        request.headers_mut().append(
            HEADER,
            http::header::HeaderValue::from_str(&s[..s.len() - 1])?,
        );

        Ok(())
    }
}

impl TryFrom<&HeaderMap> for Properties<'static, 'static> {
    type Error = crate::Error;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        let mut properties = Self::new();

        // this is probably too complicated. Should we split
        // it in more maneageable code blocks?
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
            .collect::<Result<Vec<(&str, &str)>, crate::Error>>()? // if we have an error, return error
            .into_iter()
            .map(|(key, value)| {
                let value = std::str::from_utf8(&base64::decode(value)?)?.to_owned(); // the value is base64 encoded se we decode it
                Ok((key, value))
            })
            .collect::<Result<Vec<(&str, String)>, crate::Error>>()? // if we have an error, return error
            .into_iter()
            .for_each(|(key, value)| {
                properties.insert(key.to_owned(), value); // finally store the key and value into the properties
            });

        Ok(properties)
    }
}
