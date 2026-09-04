// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Gateway 2.0 operation eligibility filter.

use crate::models::{OperationType, ResourceType};

/// Returns `true` when the resource and operation pair is eligible for Gateway 2.0.
///
/// Only `ResourceType::Document` is currently eligible. Stored-procedure
/// execution is explicitly out of
/// scope for Rust SDK GA; every non-Document resource type falls back to
/// standard Gateway via the eligibility-fallback path.
///
/// A server-side `OperationType::Patch` on a document is eligible and uses
/// RNTBD opcode `0x0002`. Client-side PATCH never reaches transport selection
/// as a PATCH; its helper Read and Replace are evaluated independently.
///
/// `is_rid_addressed` operations always return `false`. Gateway 2.0 derives its
/// `DatabaseName` / `CollectionName` routing tokens by parsing the signing link
/// (`gateway_v2_dispatch::parse_resource_names`), but a RID-addressed feed signs
/// over a bare lowercased RID that carries no `dbs`/`colls` segments, so the
/// wrap would fail locally with `CLIENT_BAD_REQUEST`. Those requests fall back
/// to standard Gateway, which routes raw RID paths natively. See
/// <https://github.com/Azure/azure-sdk-for-rust/issues/4921> for adding native
/// RID support to the Gateway 2.0 metadata path.
pub(crate) fn is_operation_supported_by_gateway_v2(
    resource_type: ResourceType,
    operation_type: OperationType,
    is_full_fidelity_change_feed: bool,
    is_rid_addressed: bool,
) -> bool {
    if is_full_fidelity_change_feed {
        // Excluded by the Gateway 2.0 contract; see docs/GATEWAY_V2_SPEC.md.
        return false;
    }
    if is_rid_addressed {
        return false;
    }
    // Both arms of this match are intentionally exhaustive (no wildcard `_` arm) so
    // that adding a new variant to either enum is a compile-time error, forcing an
    // explicit eligibility decision rather than a silent fail-closed default.
    match resource_type {
        ResourceType::Document => match operation_type {
            OperationType::Create
            | OperationType::Read
            | OperationType::Replace
            | OperationType::Upsert
            | OperationType::Delete
            | OperationType::Query
            | OperationType::SqlQuery
            | OperationType::QueryPlan
            | OperationType::ReadFeed
            | OperationType::Patch
            | OperationType::Batch => true,
            OperationType::Head | OperationType::HeadFeed | OperationType::Execute => false,
            // Distributed transactions route through the standard gateway
            // coordinator, never the thin-client Gateway 2.0 path.
            #[cfg(feature = "preview_dtx")]
            OperationType::CommitDistributedTransaction
            | OperationType::ReadDistributedTransaction => false,
        },
        ResourceType::DatabaseAccount
        | ResourceType::Database
        | ResourceType::DocumentCollection
        | ResourceType::StoredProcedure
        | ResourceType::Trigger
        | ResourceType::UserDefinedFunction
        | ResourceType::PartitionKeyRange
        | ResourceType::Offer => false,
        #[cfg(feature = "preview_dtx")]
        ResourceType::DistributedTransactionBatch => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_resource_types() -> [ResourceType; 9] {
        [
            ResourceType::DatabaseAccount,
            ResourceType::Database,
            ResourceType::DocumentCollection,
            ResourceType::Document,
            ResourceType::StoredProcedure,
            ResourceType::Trigger,
            ResourceType::UserDefinedFunction,
            ResourceType::PartitionKeyRange,
            ResourceType::Offer,
        ]
    }

    fn all_operation_types() -> [OperationType; 14] {
        [
            OperationType::Create,
            OperationType::Read,
            OperationType::ReadFeed,
            OperationType::Replace,
            OperationType::Delete,
            OperationType::Upsert,
            OperationType::Query,
            OperationType::SqlQuery,
            OperationType::QueryPlan,
            OperationType::Batch,
            OperationType::Head,
            OperationType::HeadFeed,
            OperationType::Execute,
            OperationType::Patch,
        ]
    }

    fn expected_gateway_v2_eligibility(
        resource_type: ResourceType,
        operation_type: OperationType,
    ) -> bool {
        match resource_type {
            ResourceType::Document => match operation_type {
                OperationType::Create
                | OperationType::Read
                | OperationType::Replace
                | OperationType::Upsert
                | OperationType::Delete
                | OperationType::Query
                | OperationType::SqlQuery
                | OperationType::QueryPlan
                | OperationType::ReadFeed
                | OperationType::Patch
                | OperationType::Batch => true,
                OperationType::Head | OperationType::HeadFeed | OperationType::Execute => false,
                #[cfg(feature = "preview_dtx")]
                OperationType::CommitDistributedTransaction
                | OperationType::ReadDistributedTransaction => false,
            },
            ResourceType::DatabaseAccount
            | ResourceType::Database
            | ResourceType::DocumentCollection
            | ResourceType::StoredProcedure
            | ResourceType::Trigger
            | ResourceType::UserDefinedFunction
            | ResourceType::PartitionKeyRange
            | ResourceType::Offer => false,
            #[cfg(feature = "preview_dtx")]
            ResourceType::DistributedTransactionBatch => false,
        }
    }

    #[test]
    fn gateway_v2_eligibility_matrix_is_exhaustive() {
        for resource_type in all_resource_types() {
            for operation_type in all_operation_types() {
                assert_eq!(
                    is_operation_supported_by_gateway_v2(
                        resource_type,
                        operation_type,
                        false,
                        false
                    ),
                    expected_gateway_v2_eligibility(resource_type, operation_type),
                    "unexpected Gateway 2.0 eligibility for {resource_type:?} {operation_type:?}"
                );
            }
        }
    }

    #[test]
    fn full_fidelity_change_feed_read_feed_is_ineligible() {
        // A `Document`/`ReadFeed` is otherwise eligible, but a full-fidelity
        // (AllVersionsAndDeletes) change feed must route through the standard
        // gateway because Gateway 2.0 does not forward the `A-IM` header.
        assert!(is_operation_supported_by_gateway_v2(
            ResourceType::Document,
            OperationType::ReadFeed,
            false,
            false,
        ));
        assert!(!is_operation_supported_by_gateway_v2(
            ResourceType::Document,
            OperationType::ReadFeed,
            true,
            false,
        ));
    }

    #[test]
    fn rid_addressed_operations_are_ineligible() {
        // Gateway 2.0 derives its DatabaseName/CollectionName routing tokens by
        // parsing the signing link. A RID-addressed feed signs over a bare
        // lowercased RID with no `dbs`/`colls` segments, so wrapping fails
        // locally with CLIENT_BAD_REQUEST before the request is ever sent.
        // Every otherwise-eligible operation must fall back to standard Gateway
        // when addressed by RID.
        for operation_type in all_operation_types() {
            if !is_operation_supported_by_gateway_v2(
                ResourceType::Document,
                operation_type,
                false,
                false,
            ) {
                continue;
            }
            assert!(
                !is_operation_supported_by_gateway_v2(
                    ResourceType::Document,
                    operation_type,
                    false,
                    true
                ),
                "RID-addressed {operation_type:?} must not route through Gateway 2.0"
            );
        }
    }
}
