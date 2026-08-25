// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Selects how a patch operation is executed against Azure Cosmos DB.
///
/// A patch can run two ways, and they are not interchangeable:
///
/// - **Server-side** — the operation list is sent to the service as a single
///   `PATCH` request. One round trip, and on a multi-write-region account the
///   service resolves conflicts at the *path* level, so concurrent patches to
///   different properties of the same item both survive.
/// - **Client-side** — the driver reads the item, applies the operations
///   locally, and writes it back with an ETag precondition, retrying on `412`.
///   Two round trips minimum, and conflict resolution is document-level
///   last-writer-wins, so a concurrent write to an unrelated property is lost.
///
/// Server-side is therefore preferable whenever it is safe. It is not always
/// safe: the service caps a single-document patch at 10 operations, and if a
/// request fails after it may already have been received, re-sending it can
/// double-apply operations that are not idempotent (see
/// [`PatchInstructions::is_retry_safe`]). [`Auto`](Self::Auto) encodes that
/// trade-off so callers do not have to.
///
/// [`PatchInstructions::is_retry_safe`]: crate::models::PatchInstructions::is_retry_safe
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PatchStrategy {
    /// Let the driver choose (default).
    ///
    /// Runs server-side when that is safe — every operation is retry-safe and
    /// the list fits within the service's 10-operation limit — and falls back
    /// to client-side otherwise. The resolved path is logged at `DEBUG` under
    /// the `patch_execution` field.
    #[default]
    Auto,

    /// Always use the client-side read-modify-write loop.
    ///
    /// Costs an extra round trip and gives up path-level conflict resolution,
    /// but never re-applies an operation after an ambiguous failure.
    ClientSide,

    /// Always send the patch to the service.
    ///
    /// Overrides the safety check that [`Auto`](Self::Auto) applies. When the
    /// operation list is not retry-safe and carries no ETag precondition, the
    /// driver stops retrying the request after an ambiguous failure rather
    /// than risk double-applying it, and surfaces the underlying error.
    ServerSide,
}

impl PatchStrategy {
    /// Parses a patch strategy from its wire format representation.
    ///
    /// Parsing is case-sensitive for exact matches, with case-insensitive fallback.
    ///
    /// Returns `None` if the string does not match any known strategy.
    fn parse(s: &str) -> Option<Self> {
        match s {
            "Auto" => Some(Self::Auto),
            "ClientSide" => Some(Self::ClientSide),
            "ServerSide" => Some(Self::ServerSide),
            _ => {
                if s.eq_ignore_ascii_case("Auto") {
                    Some(Self::Auto)
                } else if s.eq_ignore_ascii_case("ClientSide") {
                    Some(Self::ClientSide)
                } else if s.eq_ignore_ascii_case("ServerSide") {
                    Some(Self::ServerSide)
                } else {
                    None
                }
            }
        }
    }

    /// Returns the wire format representation of this patch strategy.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::ClientSide => "ClientSide",
            Self::ServerSide => "ServerSide",
        }
    }
}

impl std::fmt::Display for PatchStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for PatchStrategy {
    type Err = crate::error::CosmosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s).ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "'{s}' is not a valid patch strategy; expected one of \
                     Auto, ClientSide, ServerSide"
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
        for strategy in [
            PatchStrategy::Auto,
            PatchStrategy::ClientSide,
            PatchStrategy::ServerSide,
        ] {
            let canonical = strategy.as_str();
            assert_eq!(
                PatchStrategy::from_str(&canonical.to_lowercase()).unwrap(),
                strategy
            );
            assert_eq!(
                PatchStrategy::from_str(&canonical.to_uppercase()).unwrap(),
                strategy
            );
        }
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
