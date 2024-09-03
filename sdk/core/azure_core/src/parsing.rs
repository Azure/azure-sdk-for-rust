// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Parser helper utilities.

use crate::error::{Error, ErrorKind, ResultExt};
use typespec_client_core::date;

pub trait FromStringOptional<T> {
    fn from_str_optional(s: &str) -> crate::Result<T>;
}

impl FromStringOptional<u64> for u64 {
    fn from_str_optional(s: &str) -> crate::Result<u64> {
        s.parse::<u64>().map_kind(ErrorKind::DataConversion)
    }
}

impl FromStringOptional<String> for String {
    fn from_str_optional(s: &str) -> crate::Result<String> {
        Ok(s.to_owned())
    }
}

impl FromStringOptional<bool> for bool {
    fn from_str_optional(s: &str) -> crate::Result<bool> {
        match s {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(Error::with_message(ErrorKind::DataConversion, || {
                "error parsing bool '{s}'"
            })),
        }
    }
}

impl FromStringOptional<date::OffsetDateTime> for date::OffsetDateTime {
    fn from_str_optional(s: &str) -> crate::Result<date::OffsetDateTime> {
        from_azure_time(s).with_context(ErrorKind::DataConversion, || {
            format!("error parsing date time '{s}'")
        })
    }
}

#[cfg(not(feature = "azurite_workaround"))]
pub fn from_azure_time(s: &str) -> crate::Result<date::OffsetDateTime> {
    date::parse_rfc1123(s)
}

#[cfg(feature = "azurite_workaround")]
pub fn from_azure_time(s: &str) -> crate::Result<date::OffsetDateTime> {
    if let Ok(dt) = date::parse_rfc1123(s) {
        Ok(dt)
    } else {
        tracing::warn!("Received an invalid date: {}, returning now()", s);
        Ok(date::OffsetDateTime::now_utc())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_from_azure_time() {
        let t = super::from_azure_time("Sun, 27 Sep 2009 17:26:40 GMT").unwrap();

        assert_eq!(t.day(), 27);
        assert_eq!(t.month(), time::Month::September);
        assert_eq!(t.hour(), 17);
        assert_eq!(t.second(), 40);
    }
}
