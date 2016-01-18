use std::str::FromStr;
use std::string::ParseError;
use std::fmt::{Display, Formatter};
use std::fmt;
use azure::core::enumerations;
use azure::core::errors::TraversingError;
use azure::core::parsing::FromStringOptional;

create_enum!(LeaseStatus,
                            (Locked,        "locked"),
                            (Unlocked,      "unlocked")
);

create_enum!(LeaseState,
                            (Available,     "available"),
                            (Leased,        "leased"),
                            (Expired,       "expired"),
                            (Breaking,      "breaking"),
                            (Broken,        "broken")
);

create_enum!(LeaseDuration,
                            (Infinite,      "infinite"),
                            (Fixed,         "fixed")
);

create_enum!(LeaseAction,
                            (Acquire,      "acquire"),
                            (Renew,         "renew "),
                            (Change,        "change"),
                            (Release,        "release "),
                            (Break,         "break")
);


#[derive(Debug, Clone, PartialEq)]
pub struct LeaseId {
    id: String,
}

impl LeaseId {
    pub fn new(s: &str) -> LeaseId {
        LeaseId { id: s.to_owned() }
    }
}

impl FromStr for LeaseId {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<LeaseId, ParseError> {
        Ok(LeaseId { id: s.to_owned() })
    }
}

impl Display for LeaseId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl LeaseId {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_owned();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lease_parse() {
        let lease = "id".parse::<LeaseId>().unwrap();
        assert_eq!(lease.id(), "id");
    }

    #[test]
    fn test_lease_display() {
        let lease = "id".parse::<LeaseId>().unwrap();
        let r = format!("{}", lease);
        assert_eq!(r, "id");
    }
}
