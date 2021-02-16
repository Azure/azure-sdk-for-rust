use crate::headers::*;
use crate::AddAsHeader;
use http::request::Builder;
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

impl AddAsHeader for LeaseId {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(LEASE_ID, &format!("{}", self.0))
    }
}
