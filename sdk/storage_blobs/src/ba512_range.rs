use crate::{AddAsHeader, Not512ByteAlignedError, Parse512AlignedError};
use azure_core::prelude::Range;
use azure_core::{HttpHeaderError, Request};
use http::request::Builder;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BA512Range {
    start: u64,
    end: u64,
}

impl BA512Range {
    pub fn start(&self) -> u64 {
        self.start
    }
    pub fn end(&self) -> u64 {
        self.end
    }

    pub fn new(start: u64, end: u64) -> Result<Self, Not512ByteAlignedError> {
        if start % 512 != 0 {
            return Err(Not512ByteAlignedError::StartRange(start));
        }
        if (end + 1) % 512 != 0 {
            return Err(Not512ByteAlignedError::EndRange(end));
        }

        Ok(Self { start, end })
    }

    #[inline]
    pub fn size(&self) -> u64 {
        self.end - self.start + 1
    }
}

impl From<BA512Range> for Range {
    fn from(range: BA512Range) -> Self {
        Self {
            start: range.start(),
            end: range.end(),
        }
    }
}

impl TryFrom<Range> for BA512Range {
    type Error = Not512ByteAlignedError;

    fn try_from(r: Range) -> Result<Self, Self::Error> {
        BA512Range::new(r.start, r.end)
    }
}

impl TryFrom<(u64, u64)> for BA512Range {
    type Error = Not512ByteAlignedError;

    fn try_from((start, end): (u64, u64)) -> Result<Self, Self::Error> {
        BA512Range::new(start, end)
    }
}

impl AddAsHeader for BA512Range {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(http::header::RANGE, &format!("{}", self))
    }

    fn add_as_header2(&self, request: &mut Request) -> Result<(), HttpHeaderError> {
        request.headers_mut().append(
            http::header::RANGE,
            http::HeaderValue::from_str(&self.to_string())?,
        );

        Ok(())
    }
}

impl FromStr for BA512Range {
    type Err = Parse512AlignedError;
    fn from_str(s: &str) -> Result<BA512Range, Self::Err> {
        let v = s.split('/').collect::<Vec<&str>>();
        if v.len() != 2 {
            return Err(Parse512AlignedError::SplitNotFound);
        }

        let cp_start = v[0].parse::<u64>()?;
        let cp_end = v[1].parse::<u64>()?;

        Ok(BA512Range::new(cp_start, cp_end)?)
    }
}

impl fmt::Display for BA512Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bytes={}-{}", self.start, self.end)
    }
}

impl<'a> From<&'a BA512Range> for Range {
    fn from(ba: &'a BA512Range) -> Range {
        Range {
            start: ba.start(),
            end: ba.end(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Not512ByteAlignedError::{EndRange, StartRange};

    #[test]
    fn test_512range_parse() {
        let range = "0/511".parse::<BA512Range>().unwrap();

        assert_eq!(range.start, 0);
        assert_eq!(range.end, 511);
    }

    #[test]
    fn test_512range_parse_panic_1() {
        let err = "abba/2000".parse::<BA512Range>().unwrap_err();
        assert!(matches!(err, Parse512AlignedError::ParseIntError(_)));
    }

    #[test]
    fn test_512range_parse_panic_2() {
        let err = "1000-2000".parse::<BA512Range>().unwrap_err();
        assert_eq!(err, Parse512AlignedError::SplitNotFound);
    }

    #[test]
    fn test_512range_invalid_start_range() {
        let err = "7/511".parse::<BA512Range>().unwrap_err();
        assert_eq!(
            err,
            Parse512AlignedError::Not512ByteAlignedError(StartRange(7))
        );
    }

    #[test]
    fn test_512range_invalid_end_range() {
        let err = "0/100".parse::<BA512Range>().unwrap_err();
        assert_eq!(
            err,
            Parse512AlignedError::Not512ByteAlignedError(EndRange(100))
        );
    }

    #[test]
    fn test_512range_display() {
        let range = BA512Range { start: 0, end: 511 };

        let txt = format!("{}", range);

        assert_eq!(txt, "bytes=0-511");
    }
}
