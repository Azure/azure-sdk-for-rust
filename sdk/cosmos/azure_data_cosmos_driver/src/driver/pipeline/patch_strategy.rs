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
}

/// Resolves `requested` against the supplied `instructions`.
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
/// deliberately does *not* fall back on either condition: an over-long list
/// surfaces the service's own `400` rather than silently changing execution
/// mode, and an unsafe list is sent with `retry_safe: false` so the retry layers
/// stop rather than repeat a mutation.
pub(crate) fn resolve_patch_strategy(
    requested: PatchStrategy,
    instructions: &PatchInstructions,
) -> PatchExecution {
    match requested {
        PatchStrategy::ClientSide => PatchExecution::ClientSide,
        PatchStrategy::ServerSide => PatchExecution::ServerSide {
            retry_safe: instructions.is_retry_safe(),
        },
        PatchStrategy::Auto => {
            let fits = instructions.operations.len() <= MAX_SERVER_SIDE_PATCH_OPERATIONS;
            if instructions.is_retry_safe() && fits {
                PatchExecution::ServerSide { retry_safe: true }
            } else {
                PatchExecution::ClientSide
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PatchOperation;
    use serde_json::json;

    fn safe_ops(count: usize) -> PatchInstructions {
        PatchInstructions::from(
            (0..count)
                .map(|i| PatchOperation::set(format!("/f{i}"), json!(i)))
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
                resolve_patch_strategy(PatchStrategy::ClientSide, &instructions),
                PatchExecution::ClientSide
            );
        }
    }

    #[test]
    fn auto_prefers_the_service_for_safe_operations() {
        assert_eq!(
            resolve_patch_strategy(PatchStrategy::Auto, &safe_ops(1)),
            PatchExecution::ServerSide { retry_safe: true }
        );
    }

    /// `Auto` never emits an unsafe server-side patch — that combination only
    /// arises when the caller asks for it explicitly.
    #[test]
    fn auto_falls_back_to_client_side_for_unsafe_operations() {
        assert_eq!(
            resolve_patch_strategy(PatchStrategy::Auto, &unsafe_ops()),
            PatchExecution::ClientSide
        );
    }

    #[test]
    fn auto_falls_back_when_the_list_exceeds_the_service_limit() {
        assert_eq!(
            resolve_patch_strategy(
                PatchStrategy::Auto,
                &safe_ops(MAX_SERVER_SIDE_PATCH_OPERATIONS)
            ),
            PatchExecution::ServerSide { retry_safe: true },
            "the limit itself is still accepted"
        );
        assert_eq!(
            resolve_patch_strategy(
                PatchStrategy::Auto,
                &safe_ops(MAX_SERVER_SIDE_PATCH_OPERATIONS + 1)
            ),
            PatchExecution::ClientSide
        );
    }

    /// An explicit `ServerSide` request is never rewritten. The over-long list
    /// goes to the service so the caller sees its `400`, and the unsafe list
    /// goes with retries disabled rather than silently switching to RMW.
    #[test]
    fn server_side_is_honored_without_fallback() {
        assert_eq!(
            resolve_patch_strategy(
                PatchStrategy::ServerSide,
                &safe_ops(MAX_SERVER_SIDE_PATCH_OPERATIONS + 1)
            ),
            PatchExecution::ServerSide { retry_safe: true }
        );
        assert_eq!(
            resolve_patch_strategy(PatchStrategy::ServerSide, &unsafe_ops()),
            PatchExecution::ServerSide { retry_safe: false }
        );
    }
}
