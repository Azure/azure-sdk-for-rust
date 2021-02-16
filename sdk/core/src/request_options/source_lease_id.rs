use crate::headers::*;
use crate::AddAsHeader;
use http::request::Builder;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SourceLeaseId(Uuid);

impl std::fmt::Display for SourceLeaseId {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.0.fmt(fmt)
    }
}

impl std::str::FromStr for SourceLeaseId {
    type Err = <Uuid as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::from_str(s)?))
    }
}

impl AddAsHeader for SourceLeaseId {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(SOURCE_LEASE_ID, &format!("{}", self.0))
    }
}
