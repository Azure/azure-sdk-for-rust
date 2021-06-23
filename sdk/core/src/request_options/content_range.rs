use crate::ParsingError;
use std::fmt;
use std::str::FromStr;

const PREFIX: &str = "bytes ";

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ContentRange {
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

    pub fn is_empty(&self) -> bool {
        self.end == self.start
    }
}

impl FromStr for ContentRange {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<ContentRange, Self::Err> {
        let remaining = s
            .strip_prefix(PREFIX)
            .ok_or_else(|| ParsingError::TokenNotFound {
                item: "ContentRange",
                token: PREFIX.to_owned(),
                full: s.into(),
            })?;

        let mut split_at_dash = remaining.split('-');
        let start = split_at_dash.next().unwrap().parse()?;

        let mut split_at_slash = split_at_dash
            .next()
            .ok_or_else(|| ParsingError::TokenNotFound {
                item: "ContentRange",
                token: "-".to_owned(),
                full: s.into(),
            })?
            .split('/');

        let end = split_at_slash.next().unwrap().parse()?;
        let total_length = split_at_slash
            .next()
            .ok_or_else(|| ParsingError::TokenNotFound {
                item: "ContentRange",
                token: "/".to_owned(),
                full: s.into(),
            })?
            .parse()?;

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
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let range = "bytes 172032-172489/172490"
            .parse::<ContentRange>()
            .unwrap();

        assert_eq!(range.start(), 172032);
        assert_eq!(range.end(), 172489);
        assert_eq!(range.total_length(), 172490);
    }

    #[test]
    #[should_panic(
        expected = "TokenNotFound { item: \"ContentRange\", token: \"bytes \", full: \"something else\" }"
    )]
    fn test_parse_no_starting_token() {
        "something else".parse::<ContentRange>().unwrap();
    }

    #[test]
    #[should_panic(
        expected = "TokenNotFound { item: \"ContentRange\", token: \"-\", full: \"bytes 100\""
    )]
    fn test_parse_no_dash() {
        "bytes 100".parse::<ContentRange>().unwrap();
    }

    #[test]
    #[should_panic(
        expected = "TokenNotFound { item: \"ContentRange\", token: \"/\", full: \"bytes 100-500\""
    )]
    fn test_parse_no_slash() {
        "bytes 100-500".parse::<ContentRange>().unwrap();
    }

    #[test]
    fn test_display() {
        let range = ContentRange {
            start: 100,
            end: 501,
            total_length: 5000,
        };

        let txt = format!("{}", range);

        assert_eq!(txt, "bytes 100-501/5000");
    }
}
