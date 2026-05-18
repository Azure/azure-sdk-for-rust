// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Gateway 2.0 operation eligibility filter.

use crate::models::{OperationType, ResourceType};

/// Returns `true` when the resource and operation pair is eligible for Gateway 2.0.
///
/// `ResourceType::Document` operations (CRUD, query, batch, read-feed) and
/// `ResourceType::StoredProcedure` with `OperationType::Execute` are eligible.
/// Stored-procedure CRUD on the definitions themselves and every other
/// resource type fall back to standard Gateway via the eligibility-fallback
/// path. This matches .NET's `ThinClientStoreClient` which routes both
/// document operations and stored-procedure execution through Gateway 2.0.
///
/// `OperationType::Patch` is not currently a variant on the Rust enum and is
/// therefore not handled here. When the variant is added in a future slice,
/// this match must be updated.
pub(crate) fn is_operation_supported_by_gateway20(
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
            OperationType::Head | OperationType::HeadFeed | OperationType::Execute => false,
        },
        ResourceType::StoredProcedure => match operation_type {
            OperationType::Execute => true,
            OperationType::Create
            | OperationType::Read
            | OperationType::Replace
            | OperationType::Upsert
            | OperationType::Delete
            | OperationType::Query
            | OperationType::SqlQuery
            | OperationType::QueryPlan
            | OperationType::ReadFeed
            | OperationType::Batch
            | OperationType::Head
            | OperationType::HeadFeed => false,
        },
        ResourceType::DatabaseAccount
        | ResourceType::Database
        | ResourceType::DocumentCollection
        | ResourceType::Trigger
        | ResourceType::UserDefinedFunction
        | ResourceType::PartitionKeyRange
        | ResourceType::Offer => false,
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

    fn all_operation_types() -> [OperationType; 13] {
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
        ]
    }

    fn expected_gateway20_eligibility(
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
                OperationType::Head | OperationType::HeadFeed | OperationType::Execute => false,
            },
            ResourceType::StoredProcedure => matches!(operation_type, OperationType::Execute),
            ResourceType::DatabaseAccount
            | ResourceType::Database
            | ResourceType::DocumentCollection
            | ResourceType::Trigger
            | ResourceType::UserDefinedFunction
            | ResourceType::PartitionKeyRange
            | ResourceType::Offer => false,
        }
    }

    #[test]
    fn gateway20_eligibility_matrix_is_exhaustive() {
        for resource_type in all_resource_types() {
            for operation_type in all_operation_types() {
                assert_eq!(
                    is_operation_supported_by_gateway20(resource_type, operation_type),
                    expected_gateway20_eligibility(resource_type, operation_type),
                    "unexpected Gateway 2.0 eligibility for {resource_type:?} {operation_type:?}"
                );
            }
        }
    }

    #[test]
    fn stored_procedure_execute_is_gateway20_eligible() {
        // StoredProcedure::Execute routes via Gateway 2.0 (matches .NET
        // `ThinClientStoreClient` which forwards ExecuteJavaScript through
        // the thin client). Document::Execute is not a real operation in
        // Cosmos and remains ineligible.
        assert!(is_operation_supported_by_gateway20(
            ResourceType::StoredProcedure,
            OperationType::Execute
        ));
        assert!(!is_operation_supported_by_gateway20(
            ResourceType::Document,
            OperationType::Execute
        ));
    }

    #[test]
    fn stored_procedure_definition_crud_is_gateway1() {
        // StoredProcedure CRUD operates on the SP definition resource and
        // is not supported by Gateway 2.0; it falls through to standard
        // Gateway. Only `Execute` of an existing SP is G2-eligible.
        for op in [
            OperationType::Create,
            OperationType::Read,
            OperationType::Replace,
            OperationType::Upsert,
            OperationType::Delete,
        ] {
            assert!(
                !is_operation_supported_by_gateway20(ResourceType::StoredProcedure, op),
                "StoredProcedure {op:?} must NOT be Gateway 2.0 eligible"
            );
        }
    }
}
