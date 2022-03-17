use crate::headers::*;
use crate::Header;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LeaseId(Uuid);

impl std::fmt::Display for LeaseId {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.0.fmt(fmt)
    }
}

impl std::str::FromStr for LeaseId {
    type Err = <Uuid as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::from_str(s)?))
    }
}

impl Header for LeaseId {
    fn name(&self) -> &'static str {
        LEASE_ID
    }

    fn value(&self) -> String {
        format!("{}", self.0)
    }
}
