// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::error::{Error, ErrorKind, ResultExt};
use azure_core::http::headers::{Header, HeaderName, HeaderValue};
use std::fmt;
use std::ops::{Range, RangeFrom};
use std::str::FromStr;

const PREFIX: &str = "bytes ";
const WILDCARD: &str = "*";
const CONTENT_RANGE_ID: HeaderName = HeaderName::from_static("content-range");

type Result<T> = azure_core::Result<T>;

/// Trait to convert a value into an HTTP Range header.
/// Implemented on `Range<>` and `RangeFrom<>`.
/// Note that `Range<>` uses an exclusive end value while
/// HTTP uses an inclusive end value.
pub(crate) trait IntoRangeHeader {
    fn as_range_header(&self) -> String;
}

impl IntoRangeHeader for Range<usize> {
    fn as_range_header(&self) -> String {
        format!("bytes={}-{}", self.start, self.end - 1)
    }
}

impl IntoRangeHeader for RangeFrom<usize> {
    fn as_range_header(&self) -> String {
        format!("bytes={}-", self.start)
    }
}

/// Represents the `Content-Range` HTTP response header.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct ContentRange {
    /// Inclusive start and exclusive end of the range.
    pub range: Option<(usize, usize)>,
    /// Total length of the remote resource.
    pub total_len: Option<usize>,
}

impl Header for ContentRange {
    fn name(&self) -> azure_core::http::headers::HeaderName {
        CONTENT_RANGE_ID
    }

    fn value(&self) -> azure_core::http::headers::HeaderValue {
        let range_str = match self.range {
            Some(range) => format!("{}-{}", range.0, range.1),
            None => WILDCARD.to_string(),
        };
        let len_str = match self.total_len {
            Some(len) => len.to_string(),
            None => WILDCARD.to_string(),
        };
        format!("{}{}/{}", PREFIX, range_str, len_str).into()
    }
}

impl FromStr for ContentRange {
    type Err = Error;
    fn from_str(s: &str) -> Result<ContentRange> {
        let remaining = s.strip_prefix(PREFIX).ok_or_else(|| {
            Error::with_message_fn(ErrorKind::Other, || {
                format!(
                    "expected token \"{PREFIX}\" not found when parsing ContentRange from \"{s}\""
                )
            })
        })?;

        let mut split_at_slash = remaining.split('/');

        let range = parse_range(split_at_slash.next().ok_or_else(|| {
            Error::with_message(ErrorKind::Other, "Unexpected end of Content-Range.")
        })?)?;

        let total_len = parse_total_length(split_at_slash.next().ok_or_else(|| {
            Error::with_message_fn(ErrorKind::Other, || {
                format!(
                    "expected token \"{}\" not found when parsing ContentRange from \"{}\"",
                    "/", s
                )
            })
        })?)?;

        Ok(ContentRange { range, total_len })
    }
}

/// Parses the range portion of the Content-Range header: `<unit> <range>/<size>`.
/// The range portion can be of the format `<start>-<end>` or a wildcard `*`.
/// `start` and `end` are both serialized as inclusive values, but we return a
/// half-open range (inclusive start, exclusive end).
fn parse_range(s: &str) -> Result<Option<(usize, usize)>> {
    let s = s.trim();
    if s == WILDCARD {
        return Ok(None);
    }

    let mut split_at_dash = s.split('-');
    let start = split_at_dash
        .next()
        .ok_or_else(|| Error::with_message(ErrorKind::Other, "Unexpected end of Content-Range."))?
        .parse::<usize>()
        .with_kind(ErrorKind::DataConversion)?;
    let end = split_at_dash
        .next()
        .ok_or_else(|| {
            Error::with_message_fn(ErrorKind::Other, || {
                format!(
                    "expected token \"{}\" not found when parsing ContentRange from \"{}\"",
                    "-", s
                )
            })
        })?
        .parse::<usize>()
        .with_kind(ErrorKind::DataConversion)?;

    Ok(Some((start, end + 1)))
}

fn parse_total_length(s: &str) -> Result<Option<usize>> {
    let s = s.trim();
    if s == WILDCARD {
        return Ok(None);
    }
    Ok(Some(s.parse().with_kind(ErrorKind::DataConversion)?))
}

impl fmt::Display for ContentRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}/{}",
            PREFIX,
            self.range
                .map(|range| format!("{}-{}", range.0, range.1 - 1))
                .unwrap_or(WILDCARD.into()),
            self.total_len
                .map(|len| len.to_string())
                .unwrap_or(WILDCARD.into()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let range = "bytes 172032-172489/172490"
            .parse::<ContentRange>()
            .unwrap();

        assert_eq!(range.range.unwrap().0, 172032);
        assert_eq!(range.range.unwrap().1, 172490);
        assert_eq!(range.total_len.unwrap(), 172490);
    }

    #[test]
    fn parse_no_starting_token() {
        "something else".parse::<ContentRange>().unwrap_err();
    }

    #[test]
    fn parse_no_dash() {
        "bytes 100".parse::<ContentRange>().unwrap_err();
    }

    #[test]
    fn parse_no_slash() {
        "bytes 100-500".parse::<ContentRange>().unwrap_err();
    }

    #[test]
    fn display() {
        let range = ContentRange {
            range: Some((100, 500)),
            total_len: Some(5000),
        };

        let txt = format!("{range}");

        assert_eq!(txt, "bytes 100-499/5000");
    }
}

/// Represents an HTTP Range header value for blob operations.
///
/// Defines a range of bytes within an HTTP resource, starting at an offset and
/// ending at `offset + length - 1` inclusively. This matches the semantics of .NET's
/// `Azure.HttpRange`.
///
/// # Examples
///
/// ```
/// use azure_storage_blob::models::HttpRange;
///
/// // Range of 512 bytes starting at offset 0: bytes=0-511
/// let range = HttpRange::new(0, 512);
/// assert_eq!(range.to_string(), "bytes=0-511");
///
/// // Open-ended range starting at offset 255: bytes=255-
/// let range = HttpRange::from_offset(255);
/// assert_eq!(range.to_string(), "bytes=255-");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRange {
    /// The starting byte offset.
    offset: u64,
    /// The length of the range. If `None`, the range extends to the end of the resource.
    length: Option<u64>,
}

impl HttpRange {
    /// Creates a new `HttpRange` with the specified offset and length.
    ///
    /// The range will cover bytes from `offset` to `offset + length - 1` inclusive.
    ///
    /// # Arguments
    ///
    /// * `offset` - The starting byte offset.
    /// * `length` - The number of bytes in the range.
    pub fn new(offset: u64, length: u64) -> Self {
        Self {
            offset,
            length: Some(length),
        }
    }

    /// Creates a new `HttpRange` that starts at the specified offset and extends to the end.
    ///
    /// # Arguments
    ///
    /// * `offset` - The starting byte offset.
    pub fn from_offset(offset: u64) -> Self {
        Self {
            offset,
            length: None,
        }
    }
}

impl fmt::Display for HttpRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.length {
            Some(length) => write!(
                f,
                "bytes={}-{}",
                self.offset,
                self.offset.saturating_add(length).saturating_sub(1)
            ),
            None => write!(f, "bytes={}-", self.offset),
        }
    }
}

impl From<HttpRange> for HeaderValue {
    fn from(range: HttpRange) -> Self {
        HeaderValue::from(range.to_string())
    }
}

#[cfg(test)]
mod http_range_tests {
    use super::*;

    #[test]
    fn new_creates_bounded_range() {
        let range = HttpRange::new(0, 512);
        assert_eq!(range.to_string(), "bytes=0-511");
    }

    #[test]
    fn from_offset_creates_open_ended_range() {
        let range = HttpRange::from_offset(255);
        assert_eq!(range.to_string(), "bytes=255-");
    }

    #[test]
    fn display_bounded_range() {
        let range = HttpRange::new(0, 512);
        assert_eq!(range.to_string(), "bytes=0-511");
    }

    #[test]
    fn display_open_ended_range() {
        let range = HttpRange::from_offset(255);
        assert_eq!(range.to_string(), "bytes=255-");
    }

    #[test]
    fn to_string_bounded_range() {
        let range = HttpRange::new(100, 101);
        assert_eq!(range.to_string(), "bytes=100-200");
    }

    #[test]
    fn into_header_value() {
        let range = HttpRange::new(0, 512);
        let header_value: HeaderValue = range.into();
        assert_eq!(header_value.as_str(), "bytes=0-511");
    }

    #[test]
    fn display_zero_length_does_not_panic() {
        // length == 0 would underflow without saturating arithmetic; must not panic
        let range = HttpRange::new(0, 0);
        // saturating_add(0).saturating_sub(1) on offset 0 saturates to 0
        let _ = range.to_string();
    }

    #[test]
    fn display_overflow_does_not_panic() {
        // offset + length would overflow u64 without saturating arithmetic; must not panic
        let range = HttpRange::new(u64::MAX, u64::MAX);
        let _ = range.to_string();
    }
}
