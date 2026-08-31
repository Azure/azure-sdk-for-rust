// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Selects how a patch operation is executed against Azure Cosmos DB.
///
/// A patch can run two ways:
///
/// - **Server-side** sends the instruction list to Cosmos DB in one request.
/// - **Client-side** reads the item, applies the instructions locally, and
///   replaces it under an ETag precondition.
///
/// The service accepts at most 10 instructions in one server-side PATCH.
/// [`Auto`](Self::Auto) also keeps non-retry-safe instructions on the tracked
/// client-side path so ambiguous failures cannot apply them twice.
///
/// [`PatchInstructions::is_retry_safe`]: crate::models::PatchInstructions::is_retry_safe
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PatchStrategy {
    /// Let the driver choose (default).
    ///
    /// Uses server-side PATCH for retry-safe lists containing at most 10
    /// instructions, and client-side PATCH otherwise. Settings that apply only
    /// to client-side RMW do not influence this selection.
    #[default]
    Auto,

    /// Always use the client-side read-modify-write loop.
    ClientSide,

    /// Always send the PATCH to the service.
    ///
    /// Lists exceeding 10 instructions fail instead of falling back. For an
    /// unsafe list, ambiguous-outcome retries are disabled to avoid applying
    /// an instruction twice. A caller-supplied tracking ID does not override
    /// this strategy and provides no marker-backed duplicate suppression.
    ServerSide,
}

impl PatchStrategy {
    fn parse(value: &str) -> Option<Self> {
        if value.eq_ignore_ascii_case("Auto") {
            Some(Self::Auto)
        } else if value.eq_ignore_ascii_case("ClientSide") {
            Some(Self::ClientSide)
        } else if value.eq_ignore_ascii_case("ServerSide") {
            Some(Self::ServerSide)
        } else {
            None
        }
    }

    /// Returns the canonical string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::ClientSide => "ClientSide",
            Self::ServerSide => "ServerSide",
        }
    }
}

impl std::fmt::Display for PatchStrategy {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl std::str::FromStr for PatchStrategy {
    type Err = crate::error::CosmosError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value).ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "'{value}' is not a valid patch strategy; expected one of Auto, ClientSide, ServerSide"
                ))
                .build()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn auto_is_the_default() {
        assert_eq!(PatchStrategy::default(), PatchStrategy::Auto);
    }

    #[test]
    fn round_trips_through_as_str() {
        for strategy in [
            PatchStrategy::Auto,
            PatchStrategy::ClientSide,
            PatchStrategy::ServerSide,
        ] {
            assert_eq!(
                PatchStrategy::from_str(strategy.as_str()).unwrap(),
                strategy
            );
            assert_eq!(strategy.to_string(), strategy.as_str());
        }
    }

    #[test]
    fn parses_case_insensitively() {
        assert_eq!(
            PatchStrategy::from_str("serverSIDE").unwrap(),
            PatchStrategy::ServerSide
        );
    }

    #[test]
    fn rejects_unknown_values_with_bad_request() {
        let error = PatchStrategy::from_str("Sideways").unwrap_err();
        assert_eq!(
            error.status().status_code(),
            azure_core::http::StatusCode::BadRequest
        );
    }
}
