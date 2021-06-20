use bytes::Bytes;
use http::header::{AsHeaderName, HeaderMap, HeaderName, HeaderValue};
use http::{self, request::Builder};
use std::{convert::TryFrom, fmt::Display, str::FromStr};

pub fn format_header_value<D: Display>(value: D) -> Result<HeaderValue, http::Error> {
    let value: &str = &format(value);
    Ok(HeaderValue::try_from(value)?)
}

pub trait HeaderMapExt {
    fn get_header<K: AsHeaderName>(&self, key: K) -> Option<&HeaderValue>;

    fn get_as_str<K: AsHeaderName>(&self, key: K) -> Option<&str> {
        self.get_header(key).and_then(|v| v.to_str().ok())
    }

    fn get_as_string<K: AsHeaderName>(&self, key: K) -> Option<String> {
        self.get_as_str(key).map(|s| s.to_owned())
    }

    fn get_as_u64<K: AsHeaderName>(&self, key: K) -> Option<u64> {
        self.get_as_str(key).and_then(|s| s.parse::<u64>().ok())
    }

    fn get_as_enum<K: AsHeaderName, V: FromStr<Err = E>, E>(&self, key: K) -> Result<Option<V>, E> {
        if let Some(s) = self.get_as_str(key) {
            return Ok(Some(s.parse::<V>()?));
        }
        Ok(None)
    }
}

impl HeaderMapExt for HeaderMap {
    fn get_header<K: AsHeaderName>(&self, key: K) -> Option<&HeaderValue> {
        self.get(key)
    }
}

pub trait RequestBuilderExt {
    fn set_header<K, V>(self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
        Self: Sized;

    fn header_formatted<K, D: Display>(self, key: K, value: D) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        Self: Sized,
    {
        self.set_header(key, &format(value))
    }

    fn header_static<K>(self, key: K, value: &'static str) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        Self: Sized,
    {
        self.set_header(key, HeaderValue::from_static(value))
    }

    fn header_bytes<K, B: Into<Bytes>>(self, key: K, value: B) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        Self: Sized,
    {
        self.set_header(key, &value.into() as &[u8])
    }
}

impl RequestBuilderExt for Builder {
    fn set_header<K, V>(self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
        Self: Sized,
    {
        self.header(key, value)
    }
}

fn format<D: Display>(value: D) -> String {
    format!("{}", value)
}

const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

/// Returns Bytes without the UTF-8 BOM.
pub fn slice_bom(bytes: &Bytes) -> Bytes {
    if bytes.len() > 3 && bytes.slice(0..3).as_ref() == &UTF8_BOM {
        bytes.slice(3..)
    } else {
        bytes.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::util::*;
    use bytes::Bytes;

    #[test]
    fn test_strip_bom() {
        let bytes = Bytes::from_static(&[0xEF, 0xBB, 0xBF, 7]);
        assert_eq!(Bytes::from_static(&[7]), slice_bom(&bytes));

        let bytes = Bytes::from_static(&[8]);
        assert_eq!(Bytes::from_static(&[8]), slice_bom(&bytes));
    }
}
