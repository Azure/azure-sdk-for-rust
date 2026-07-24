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
/// `OperationType::Patch` always returns `false`: the driver short-circuits
/// `Patch` operations to the driver-side `patch_handler` pipeline stage in
/// `CosmosDriver::execute_operation` before transport selection ever runs,
/// so the outer `Patch` operation never reaches Gateway 2.0 transport. The
/// sub-operations the patch handler issues (`Read` / `Replace`) are each
/// evaluated on their own merits.
pub(crate) fn is_operation_supported_by_gateway_v2(
    resource_type: ResourceType,
    operation_type: OperationType,
) -> bool {
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
            | OperationType::Batch => true,
            OperationType::Head
            | OperationType::HeadFeed
            | OperationType::Execute
            | OperationType::Patch => false,
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
                | OperationType::Batch => true,
                OperationType::Head
                | OperationType::HeadFeed
                | OperationType::Execute
                | OperationType::Patch => false,
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
                    is_operation_supported_by_gateway_v2(resource_type, operation_type),
                    expected_gateway_v2_eligibility(resource_type, operation_type),
                    "unexpected Gateway 2.0 eligibility for {resource_type:?} {operation_type:?}"
                );
            }
        }
    }
}
