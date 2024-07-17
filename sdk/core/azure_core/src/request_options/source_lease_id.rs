use crate::headers::{self, Header};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Header for SourceLeaseId {
    fn name(&self) -> headers::HeaderName {
        headers::SOURCE_LEASE_ID
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_string().into()
    }
}
