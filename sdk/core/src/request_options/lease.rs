use crate::headers::{self, Header};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn name(&self) -> headers::HeaderName {
        headers::LEASE_ID
    }

    fn value(&self) -> headers::HeaderValue {
        format!("{}", self.0).into()
    }
}
