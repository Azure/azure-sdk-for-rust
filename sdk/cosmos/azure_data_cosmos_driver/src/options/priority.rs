// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Priority level types for priority-based execution.

use std::fmt::{self, Display};

/// Request priority for priority-based execution.
///
/// When enabled at the account level, low priority requests are throttled
/// before high priority requests once provisioned throughput is exhausted.
///
/// See [Priority-based execution](https://learn.microsoft.com/azure/cosmos-db/priority-based-execution)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PriorityLevel {
    /// High priority request (default behavior).
    #[default]
    High,
    /// Low priority request (throttled first when at capacity).
    Low,
}

impl PriorityLevel {
    /// Returns the wire format representation of this priority level.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::High => "High",
            Self::Low => "Low",
        }
    }
}

impl Display for PriorityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for PriorityLevel {
    type Err = azure_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "High" => Ok(Self::High),
            "Low" => Ok(Self::Low),
            _ => Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                format!("Unknown priority level: {s}"),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::error::ErrorKind;

    #[test]
    fn parses_valid_priority_levels() {
        let high: PriorityLevel = "High".parse().expect("parse High");
        let low: PriorityLevel = "Low".parse().expect("parse Low");
        assert_eq!(high, PriorityLevel::High);
        assert_eq!(low, PriorityLevel::Low);
    }

    #[test]
    fn parsing_invalid_priority_returns_data_conversion_error() {
        let err = "Medium"
            .parse::<PriorityLevel>()
            .expect_err("expected error for invalid priority");
        assert_eq!(*err.kind(), ErrorKind::DataConversion);
        assert!(
            err.to_string().contains("Unknown priority level: Medium"),
            "unexpected error message: {err}"
        );
    }

    #[test]
    fn display_roundtrips_through_from_str() {
        for level in [PriorityLevel::High, PriorityLevel::Low] {
            let s = level.to_string();
            assert_eq!(s, level.as_str());
            let parsed: PriorityLevel = s.parse().expect("roundtrip parse failed");
            assert_eq!(parsed, level);
        }
    }
}
