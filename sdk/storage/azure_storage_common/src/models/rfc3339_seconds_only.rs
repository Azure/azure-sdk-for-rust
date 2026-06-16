// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Serde serialization helpers that format [`OffsetDateTime`](azure_core::time::OffsetDateTime)
//! as RFC 3339 truncated to whole seconds (e.g. `2022-08-12T20:55:02Z`).
//!
//! Some Azure Storage service endpoints reject timestamps with fractional seconds.
//! This module provides a custom `serialize` function (and an `option` sub-module
//! for `Option<OffsetDateTime>`) that strips fractional seconds while leaving
//! deserialization to the standard RFC 3339 parser which accepts any valid precision.

use azure_core::time::OffsetDateTime;
use serde::Serializer;
use time::format_description::FormatItem;

/// A compile-time format description for `YYYY-MM-DDTHH:MM:SSZ`.
const FORMAT: &[FormatItem<'static>] =
    time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z");

/// Serializes an [`OffsetDateTime`] as an RFC 3339 string with no fractional seconds.
pub fn serialize<S>(value: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = value
        .to_offset(time::UtcOffset::UTC)
        .format(FORMAT)
        .map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&s)
}

/// Serde helpers for `Option<OffsetDateTime>` fields.
pub mod option {
    use azure_core::time::OffsetDateTime;
    use serde::Serializer;

    /// Serializes an `Option<OffsetDateTime>` as an RFC 3339 string with no fractional seconds,
    /// or skips if `None`.
    pub fn serialize<S>(value: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(dt) => super::serialize(dt, serializer),
            None => serializer.serialize_none(),
        }
    }
}
