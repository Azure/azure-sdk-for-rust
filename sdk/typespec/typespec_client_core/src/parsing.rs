// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Parser helper utilities.

use crate::date;
use typespec::error::{Error, ErrorKind, ResultExt};

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
        date::parse_rfc1123(s).with_context(ErrorKind::DataConversion, || {
            format!("error parsing date time '{s}'")
        })
    }
}
