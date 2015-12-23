use std::str::FromStr;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    SplitNotFound,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(pie : ParseIntError) -> ParseError {
        ParseError::ParseIntError(pie)
    }
}

impl FromStr for Range {
    type Err = ParseError;
    fn from_str(s : &str) -> Result<Range, ParseError> {
            let v = s.split("/").collect::<Vec<&str>>();
            if v.len() != 2 {
                return Err(ParseError::SplitNotFound);
            }

            let cp_start = try!(v[0].parse::<u64>());
            let cp_end = try!(v[1].parse::<u64>());

            Ok(Range {
                start: cp_start,
                end: cp_end,
            })
        }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.start, self.end)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_parse() {
        let range = "1000/2000".parse::<Range>().unwrap();

        assert_eq!(range.start, 1000);
        assert_eq!(range.end, 2000);
    }

    #[test]
    #[should_panic(expected = "ParseIntError(ParseIntError { kind: InvalidDigit })")]
    fn test_range_parse_panic_1() {
        "abba/2000".parse::<Range>().unwrap();
    }

    #[test]
    #[should_panic(expected = "SplitNotFound")]
    fn test_range_parse_panic_2() {
        "1000-2000".parse::<Range>().unwrap();
    }

    #[test]
    fn test_range_display() {
        let range = Range { start: 100, end: 500};

        let txt = format!("{}", range);

        assert_eq!(txt, "100/500");
    }
}
