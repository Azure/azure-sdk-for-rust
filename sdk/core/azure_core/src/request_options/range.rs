use crate::error::{Error, ErrorKind, ResultExt};
use crate::headers::{self, AsHeaders, HeaderName, HeaderValue};
use std::fmt;
use std::ops::{Range as StdRange, RangeFrom};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Range {
    Range(StdRange<u64>),
    RangeFrom(RangeFrom<u64>),
}

impl Range {
    pub fn new(start: u64, end: u64) -> Range {
        (start..end).into()
    }

    fn optional_len(&self) -> Option<u64> {
        match self {
            Range::Range(r) => Some(r.end - r.start),
            Range::RangeFrom(_) => None,
        }
    }
}

impl From<StdRange<u64>> for Range {
    fn from(r: StdRange<u64>) -> Self {
        Self::Range(r)
    }
}

impl From<RangeFrom<u64>> for Range {
    fn from(r: RangeFrom<u64>) -> Self {
        Self::RangeFrom(r)
    }
}

impl From<StdRange<usize>> for Range {
    fn from(r: StdRange<usize>) -> Self {
        (r.start as u64..r.end as u64).into()
    }
}

impl From<RangeFrom<usize>> for Range {
    fn from(r: RangeFrom<usize>) -> Self {
        (r.start as u64..).into()
    }
}

impl AsHeaders for Range {
    type Iter = std::vec::IntoIter<(HeaderName, HeaderValue)>;

    fn as_headers(&self) -> Self::Iter {
        let mut headers = vec![(headers::MS_RANGE, format!("{self}").into())];
        if let Some(len) = self.optional_len() {
            if len < 1024 * 1024 * 4 {
                headers.push((
                    headers::RANGE_GET_CONTENT_CRC64,
                    HeaderValue::from_static("true"),
                ));
            }
        }
        headers.into_iter()
    }
}

impl FromStr for Range {
    type Err = Error;
    fn from_str(s: &str) -> crate::Result<Range> {
        let v = s.split('/').collect::<Vec<&str>>();
        if v.len() != 2 {
            return Err(Error::with_message(ErrorKind::Other, || {
                format!(
                    "expected token \"{}\" not found when parsing Range from \"{}\"",
                    "/", s
                )
            }));
        }

        let cp_start = v[0].parse::<u64>().map_kind(ErrorKind::DataConversion)?;
        let cp_end = v[1].parse::<u64>().map_kind(ErrorKind::DataConversion)? + 1;

        Ok((cp_start..cp_end).into())
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Range::Range(r) => write!(f, "bytes={}-{}", r.start, r.end - 1),
            Range::RangeFrom(r) => write!(f, "bytes={}-", r.start),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_parse() {
        let range = "1000/2000".parse::<Range>().unwrap();
        assert_eq!(range, Range::new(1000, 2001));
    }

    #[test]
    fn test_range_parse_panic_1() {
        "abba/2000".parse::<Range>().unwrap_err();
    }

    #[test]
    fn test_range_parse_panic_2() {
        "1000-2000".parse::<Range>().unwrap_err();
    }

    #[test]
    fn test_range_display() {
        let range = Range::new(100, 501);
        let txt = format!("{range}");
        assert_eq!(txt, "bytes=100-500");
    }
}
