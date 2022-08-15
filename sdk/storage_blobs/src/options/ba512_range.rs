use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    headers::{self, Header},
    prelude::Range,
};
use std::{convert::TryFrom, fmt, str::FromStr};

/// A 512 byte aligned byte range
///
/// [Read more here](https://docs.microsoft.com/en-us/rest/api/storageservices/specifying-the-range-header-for-blob-service-operations#format-2-bytesstartbyte-endbyte).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

    pub fn new(start: u64, end: u64) -> azure_core::Result<Self> {
        if start % 512 != 0 {
            return Err(Error::with_message(ErrorKind::Other, || {
                format!("start range not 512-byte aligned: {}", start)
            }));
        }
        if (end + 1) % 512 != 0 {
            return Err(Error::with_message(ErrorKind::Other, || {
                format!("end range not 512-byte aligned: {}", end)
            }));
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
    type Error = Error;

    fn try_from(r: Range) -> azure_core::Result<Self> {
        BA512Range::new(r.start, r.end)
    }
}

impl TryFrom<(u64, u64)> for BA512Range {
    type Error = Error;

    fn try_from((start, end): (u64, u64)) -> azure_core::Result<Self> {
        BA512Range::new(start, end)
    }
}

impl Header for BA512Range {
    fn name(&self) -> headers::HeaderName {
        headers::RANGE
    }

    fn value(&self) -> headers::HeaderValue {
        self.to_string().into()
    }
}

impl FromStr for BA512Range {
    type Err = Error;
    fn from_str(s: &str) -> azure_core::Result<BA512Range> {
        let v = s.split('/').collect::<Vec<&str>>();
        if v.len() != 2 {
            return Err(Error::message(ErrorKind::Other, "split not found"));
        }

        let cp_start = v[0]
            .parse::<u64>()
            .with_context(ErrorKind::DataConversion, || {
                format!("error parsing '{}' into u64", v[0])
            })?;
        let cp_end = v[1]
            .parse::<u64>()
            .with_context(ErrorKind::DataConversion, || {
                format!("error parsing '{}' into u64", v[1])
            })?;

        BA512Range::new(cp_start, cp_end)
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
    use azure_core::error::ErrorKind;

    #[test]
    fn test_512range_parse() {
        let range = "0/511".parse::<BA512Range>().unwrap();

        assert_eq!(range.start, 0);
        assert_eq!(range.end, 511);
    }

    #[test]
    fn test_512range_parse_panic_1() {
        let err = "abba/2000".parse::<BA512Range>().unwrap_err();
        assert!(matches!(err.kind(), ErrorKind::DataConversion));
    }

    #[test]
    fn test_512range_parse_panic_2() {
        let err = "1000-2000".parse::<BA512Range>().unwrap_err();
        assert!(matches!(err.kind(), ErrorKind::Other));
    }

    #[test]
    fn test_512range_invalid_start_range() {
        let err = "7/511".parse::<BA512Range>().unwrap_err();
        assert!(matches!(err.kind(), ErrorKind::Other));
    }

    #[test]
    fn test_512range_invalid_end_range() {
        let err = "0/100".parse::<BA512Range>().unwrap_err();
        assert!(matches!(err.kind(), ErrorKind::Other));
    }

    #[test]
    fn test_512range_display() {
        let range = BA512Range { start: 0, end: 511 };

        let txt = format!("{}", range);

        assert_eq!(txt, "bytes=0-511");
    }
}
