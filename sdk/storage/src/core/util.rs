use http::header::{AsHeaderName, HeaderMap, HeaderValue};
use std::str::FromStr;

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
