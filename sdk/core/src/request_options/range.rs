use crate::headers::{AsHeaders, HeaderName, HeaderValue};
use crate::ParseError;
use std::convert::From;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {
    pub fn new(start: u64, end: u64) -> Range {
        Range { start, end }
    }

    pub fn len(&self) -> u64 {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.end == self.start
    }
}

impl AsHeaders for Range {
    type Iter = std::vec::IntoIter<(HeaderName, HeaderValue)>;

    fn as_headers(&self) -> Self::Iter {
        let mut headers = vec![("x-ms-range".into(), format!("{}", self).into())];
        if self.len() < 1024 * 1024 * 4 {
            headers.push(("x-ms-range-get-content-crc64".into(), "true".into()));
        }
        headers.into_iter()
    }
}

impl From<std::ops::Range<u64>> for Range {
    fn from(r: std::ops::Range<u64>) -> Self {
        Self {
            start: r.start,
            end: r.end,
        }
    }
}

impl From<std::ops::Range<i32>> for Range {
    fn from(r: std::ops::Range<i32>) -> Self {
        Self {
            start: r.start as u64,
            end: r.end as u64,
        }
    }
}

impl From<std::ops::Range<usize>> for Range {
    fn from(r: std::ops::Range<usize>) -> Self {
        Self {
            start: r.start as u64,
            end: r.end as u64,
        }
    }
}

impl FromStr for Range {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Range, Self::Err> {
        let v = s.split('/').collect::<Vec<&str>>();
        if v.len() != 2 {
            return Err(ParseError::TokenNotFound {
                item: "Range",
                token: "/".to_owned(),
                full: s.to_owned(),
            });
        }

        let cp_start = v[0].parse::<u64>()?;
        let cp_end = v[1].parse::<u64>()? + 1;

        Ok(Range {
            start: cp_start,
            end: cp_end,
        })
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bytes={}-{}", self.start, self.end - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_parse() {
        let range = "1000/2000".parse::<Range>().unwrap();

        assert_eq!(range.start, 1000);
        assert_eq!(range.end, 2001);
    }

    #[test]
    fn test_range_parse_panic_1() {
        let err = "abba/2000".parse::<Range>().unwrap_err();
        assert!(matches!(err, ParseError::Int(_)));
    }

    #[test]
    fn test_range_parse_panic_2() {
        let err = "1000-2000".parse::<Range>().unwrap_err();
        assert_eq!(
            err,
            ParseError::TokenNotFound {
                item: "Range",
                token: "/".to_string(),
                full: "1000-2000".to_string()
            }
        );
    }

    #[test]
    fn test_range_display() {
        let range = Range {
            start: 100,
            end: 501,
        };

        let txt = format!("{}", range);

        assert_eq!(txt, "bytes=100-500");
    }
}
