// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Chooses between server-side and client-side execution for a patch.

use crate::models::{PatchInstructions, MAX_SERVER_SIDE_PATCH_OPERATIONS};
use crate::options::PatchStrategy;

/// How a patch will actually be executed, after resolving the caller's
/// [`PatchStrategy`] against the operations they supplied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PatchExecution {
    /// Read the item, apply the operations locally, write it back under an
    /// ETag precondition.
    ClientSide,
    /// Send the operations to the service as a single `PATCH` request.
    ServerSide {
        /// Whether the request may be resent after an ambiguous failure.
        /// Threaded onto the operation so the retry layers can honor it.
        retry_safe: bool,
    },
}

impl PatchExecution {
    /// Short label for tracing and diagnostics.
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::ClientSide => "ClientSide",
            Self::ServerSide { .. } => "ServerSide",
        }
    }

    /// Whether the resolved execution may be resent after an ambiguous failure.
    /// The client-side loop re-reads before every attempt, so it always may.
    pub(crate) fn retry_safe(&self) -> bool {
        match self {
            Self::ClientSide => true,
            Self::ServerSide { retry_safe } => *retry_safe,
        }
    }
}

/// Resolves `requested` against the parsed `instructions`.
///
/// `None` means the body was missing or could not be parsed. `Auto` and
/// `ClientSide` leave that error to the RMW handler; explicit `ServerSide`
/// still honors the requested path and sends the body with ambiguous retries
/// disabled because its safety cannot be established.
///
/// [`PatchStrategy::Auto`] prefers the service, because one round trip beats
/// two and a multi-write-region account resolves concurrent patches at the path
/// level instead of losing one of them. It steps back to the client-side loop
/// only where the server-side path cannot deliver the same result:
///
/// - the operation list is not retry-safe, so resending after an ambiguous
///   failure could double-apply it, or
/// - the list exceeds the service's per-document operation limit, which would
///   simply be rejected.
///
/// The explicit strategies are honored as written. [`PatchStrategy::ServerSide`]
/// deliberately does *not* fall back: an over-long list
/// surfaces the service's own `400` rather than silently changing execution
/// mode, and an unsafe list is sent with `retry_safe: false` so the retry layers
/// stop rather than repeat a mutation. A tracking ID does not override explicit
/// server execution; the service does not persist the client's tracking marker,
/// so the ID provides no duplicate suppression on that path.
pub(crate) fn resolve_patch_strategy(
    requested: PatchStrategy,
    instructions: Option<&PatchInstructions>,
) -> crate::error::Result<PatchExecution> {
    Ok(match requested {
        PatchStrategy::ClientSide => PatchExecution::ClientSide,
        PatchStrategy::ServerSide => PatchExecution::ServerSide {
            retry_safe: instructions.is_some_and(PatchInstructions::is_retry_safe),
        },
        PatchStrategy::Auto => {
            let server_eligible = instructions.is_some_and(|instructions| {
                instructions.is_retry_safe()
                    && instructions.operations.len() <= MAX_SERVER_SIDE_PATCH_OPERATIONS
            });
            if server_eligible {
                PatchExecution::ServerSide { retry_safe: true }
            } else {
                PatchExecution::ClientSide
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PatchOperation;
    use serde_json::json;

    fn safe_ops(count: usize) -> PatchInstructions {
        PatchInstructions::from(
            (0..count)
                .map(|index| PatchOperation::set(format!("/f{index}"), json!(index)))
                .collect::<Vec<_>>(),
        )
    }

    fn unsafe_ops() -> PatchInstructions {
        PatchInstructions::from(vec![PatchOperation::increment("/visits", 1i64)])
    }

    #[test]
    fn client_side_is_honored_whatever_the_operations() {
        for instructions in [safe_ops(1), unsafe_ops(), safe_ops(50)] {
            assert_eq!(
                resolve_patch_strategy(PatchStrategy::ClientSide, Some(&instructions)).unwrap(),
                PatchExecution::ClientSide
            );
        }
    }

    #[test]
    fn auto_prefers_the_service_for_safe_operations() {
        assert_eq!(
            resolve_patch_strategy(PatchStrategy::Auto, Some(&safe_ops(1))).unwrap(),
            PatchExecution::ServerSide { retry_safe: true }
        );
    }

    #[test]
    fn auto_falls_back_to_client_side_for_unsafe_operations() {
        assert_eq!(
            resolve_patch_strategy(PatchStrategy::Auto, Some(&unsafe_ops())).unwrap(),
            PatchExecution::ClientSide
        );
    }

    #[test]
    fn auto_falls_back_when_the_list_exceeds_the_service_limit() {
        assert_eq!(
            resolve_patch_strategy(
                PatchStrategy::Auto,
                Some(&safe_ops(MAX_SERVER_SIDE_PATCH_OPERATIONS)),
            )
            .unwrap(),
            PatchExecution::ServerSide { retry_safe: true },
            "the limit itself is still accepted"
        );
        assert_eq!(
            resolve_patch_strategy(
                PatchStrategy::Auto,
                Some(&safe_ops(MAX_SERVER_SIDE_PATCH_OPERATIONS + 1)),
            )
            .unwrap(),
            PatchExecution::ClientSide
        );
    }

    #[test]
    fn server_side_is_honored_without_fallback() {
        assert_eq!(
            resolve_patch_strategy(
                PatchStrategy::ServerSide,
                Some(&safe_ops(MAX_SERVER_SIDE_PATCH_OPERATIONS + 1)),
            )
            .unwrap(),
            PatchExecution::ServerSide { retry_safe: true }
        );
        assert_eq!(
            resolve_patch_strategy(PatchStrategy::ServerSide, Some(&unsafe_ops())).unwrap(),
            PatchExecution::ServerSide { retry_safe: false }
        );
    }

    #[test]
    fn unknown_server_side_payload_is_sent_fail_closed() {
        assert_eq!(
            resolve_patch_strategy(PatchStrategy::ServerSide, None).unwrap(),
            PatchExecution::ServerSide { retry_safe: false }
        );
        assert_eq!(
            resolve_patch_strategy(PatchStrategy::Auto, None).unwrap(),
            PatchExecution::ClientSide
        );
    }

    #[test]
    fn explicit_rmw_options_do_not_influence_strategy_resolution() {
        assert_eq!(
            resolve_patch_strategy(PatchStrategy::Auto, Some(&safe_ops(1))).unwrap(),
            PatchExecution::ServerSide { retry_safe: true }
        );
    }
}
