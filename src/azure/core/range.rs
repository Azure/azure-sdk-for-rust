use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

impl FromStr for Range {
    type Err = ParseError;
    fn from_str(s : &str) -> Result<Range, ParseError> {
            let v = s.split("/").collect::<Vec<&str>>();
            if v.len() != 2 {
                return Err(ParseError);
            }

            let cp_start = try!(v[0].parse::<u64>());
            let cp_end = try!(v[1].parse::<u64>());

            Ok(Range {
                start: cp_start,
                end: cp_end,
            });
        }
    }
