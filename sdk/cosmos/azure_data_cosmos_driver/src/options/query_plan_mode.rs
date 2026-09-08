// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Controls how cross-partition query plans are resolved.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum QueryPlanMode {
    /// Prefer local planning and fall back to the Gateway before execution.
    #[default]
    LocalPreferred,
    /// Always request query plans from the Gateway.
    ///
    /// Use this as a compatibility or livesite fallback to temporarily bypass
    /// local planning while diagnosing or mitigating a local-planner issue.
    GatewayOnly,
}

impl QueryPlanMode {
    /// Returns the canonical string representation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LocalPreferred => "LocalPreferred",
            Self::GatewayOnly => "GatewayOnly",
        }
    }
}

impl std::fmt::Display for QueryPlanMode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl std::str::FromStr for QueryPlanMode {
    type Err = crate::error::CosmosError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim();
        if value.eq_ignore_ascii_case("LocalPreferred")
            || value.eq_ignore_ascii_case("local_preferred")
        {
            Ok(Self::LocalPreferred)
        } else if value.eq_ignore_ascii_case("GatewayOnly")
            || value.eq_ignore_ascii_case("gateway_only")
            || value.eq_ignore_ascii_case("gateway")
        {
            Ok(Self::GatewayOnly)
        } else {
            Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "'{value}' is not a valid query plan mode; expected LocalPreferred or GatewayOnly"
                ))
                .build())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_values() {
        assert_eq!(
            "LocalPreferred".parse::<QueryPlanMode>().unwrap(),
            QueryPlanMode::LocalPreferred
        );
        assert_eq!(
            "gateway".parse::<QueryPlanMode>().unwrap(),
            QueryPlanMode::GatewayOnly
        );
        assert_eq!(
            "gateway_only".parse::<QueryPlanMode>().unwrap(),
            QueryPlanMode::GatewayOnly
        );
    }

    #[test]
    fn rejects_unknown_values() {
        assert!("automatic".parse::<QueryPlanMode>().is_err());
    }
}
