use std::str::FromStr;
use std::fmt;
use std::num::ParseIntError;
use azure::core::range::Range;
use std::convert::Into;

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

    pub fn new(start: u64, end: u64) -> Result<BA512Range, Not512ByteAlignedError> {
        if start % 512 != 0 {
            return Err(Not512ByteAlignedError::StartRange(start));
        }
        if (end + 1) % 512 != 0 {
            return Err(Not512ByteAlignedError::EndRange(end));
        }

        Ok(BA512Range {
            start: start,
            end: end,
        })
    }

    #[inline]
    pub fn size(&self) -> u64 {
        self.end - self.start + 1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Not512ByteAlignedError {
    StartRange(u64),
    EndRange(u64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    SplitNotFound,
    ParseIntError(ParseIntError),
    Not512ByteAlignedError(Not512ByteAlignedError),
}

impl From<ParseIntError> for ParseError {
    fn from(pie: ParseIntError) -> ParseError {
        ParseError::ParseIntError(pie)
    }
}

impl From<Not512ByteAlignedError> for ParseError {
    fn from(nae: Not512ByteAlignedError) -> ParseError {
        ParseError::Not512ByteAlignedError(nae)
    }
}

impl Into<Range> for BA512Range {
    fn into(self) -> Range {
        Range {
            start: self.start(),
            end: self.end(),
        }
    }
}

impl FromStr for BA512Range {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<BA512Range, ParseError> {
        let v = s.split("/").collect::<Vec<&str>>();
        if v.len() != 2 {
            return Err(ParseError::SplitNotFound);
        }

        let cp_start = try!(v[0].parse::<u64>());
        let cp_end = try!(v[1].parse::<u64>());



        Ok(try!(BA512Range::new(cp_start, cp_end)))
    }
}

impl fmt::Display for BA512Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bytes={}-{}", self.start, self.end)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_512range_parse() {
        let range = "0/511".parse::<BA512Range>().unwrap();

        assert_eq!(range.start, 0);
        assert_eq!(range.end, 511);
    }

    #[test]
    #[should_panic(expected = "ParseIntError(ParseIntError { kind: InvalidDigit })")]
    fn test_512range_parse_panic_1() {
        "abba/2000".parse::<BA512Range>().unwrap();
    }

    #[test]
    #[should_panic(expected = "SplitNotFound")]
    fn test_512range_parse_panic_2() {
        "1000-2000".parse::<BA512Range>().unwrap();
    }

    #[test]
    #[should_panic(expected = "Not512ByteAlignedError(StartRange(7))")]
    fn test_512range_invalid_start_range() {
        "7/511".parse::<BA512Range>().unwrap();
    }

    #[test]
    #[should_panic(expected = "Not512ByteAlignedError(EndRange(100))")]
    fn test_512range_invalid_end_range() {
        "0/100".parse::<BA512Range>().unwrap();
    }

    #[test]
    fn test_512range_display() {
        let range = BA512Range {
            start: 0,
            end: 511,
        };

        let txt = format!("{}", range);

        assert_eq!(txt, "bytes=0-511");
    }
}
