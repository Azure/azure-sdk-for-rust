// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::error::{Error, ErrorKind, ResultExt};
use std::fmt;
use std::str::FromStr;

const PREFIX: &str = "bytes ";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct ContentRange {
    start: u64,
    end: u64,
    total_length: u64,
}

impl ContentRange {
    pub fn new(start: u64, end: u64, total_length: u64) -> ContentRange {
        ContentRange {
            start,
            end,
            total_length,
        }
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn end(&self) -> u64 {
        self.end
    }

    pub fn total_length(&self) -> u64 {
        self.total_length
    }
}

impl FromStr for ContentRange {
    type Err = Error;
    fn from_str(s: &str) -> azure_core::Result<ContentRange> {
        let remaining = s.strip_prefix(PREFIX).ok_or_else(|| {
            Error::with_message_fn(ErrorKind::Other, || {
                format!(
                    "expected token \"{PREFIX}\" not found when parsing ContentRange from \"{s}\""
                )
            })
        })?;

        let mut split_at_dash = remaining.split('-');
        let start = split_at_dash
            .next()
            .ok_or_else(|| {
                Error::with_message_fn(ErrorKind::Other, || {
                    format!(
                        "expected token \"{}\" not found when parsing ContentRange from \"{}\"",
                        "-", s
                    )
                })
            })?
            .parse()
            .with_kind(ErrorKind::DataConversion)?;

        let mut split_at_slash = split_at_dash
            .next()
            .ok_or_else(|| {
                Error::with_message_fn(ErrorKind::Other, || {
                    format!(
                        "expected token \"{}\" not found when parsing ContentRange from \"{}\"",
                        "-", s
                    )
                })
            })?
            .split('/');

        let end = split_at_slash
            .next()
            .ok_or_else(|| {
                Error::with_message_fn(ErrorKind::Other, || {
                    format!(
                        "expected token \"{}\" not found when parsing ContentRange from \"{}\"",
                        "/", s
                    )
                })
            })?
            .parse()
            .with_kind(ErrorKind::DataConversion)?;

        let total_length = split_at_slash
            .next()
            .ok_or_else(|| {
                Error::with_message_fn(ErrorKind::Other, || {
                    format!(
                        "expected token \"{}\" not found when parsing ContentRange from \"{}\"",
                        "/", s
                    )
                })
            })?
            .parse()
            .with_kind(ErrorKind::DataConversion)?;

        Ok(ContentRange {
            start,
            end,
            total_length,
        })
    }
}

impl fmt::Display for ContentRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}-{}/{}",
            PREFIX,
            self.start(),
            self.end(),
            self.total_length()
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

        assert_eq!(range.start(), 172032);
        assert_eq!(range.end(), 172489);
        assert_eq!(range.total_length(), 172490);
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
            start: 100,
            end: 501,
            total_length: 5000,
        };

        let txt = format!("{range}");

        assert_eq!(txt, "bytes 100-501/5000");
    }
}
