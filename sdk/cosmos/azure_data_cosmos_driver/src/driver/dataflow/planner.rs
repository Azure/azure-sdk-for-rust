// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pipeline planner for Cosmos DB operations.
//!
//! The planner validates an operation's target against its resource type and
//! constructs the appropriate dataflow [`Pipeline`].

use crate::models::{CosmosOperation, OperationTarget};

use super::{Pipeline, Request, RequestTarget};

/// Validates and builds a [`Pipeline`] for the given operation.
///
/// This is the "Planning" phase of operation execution. It:
/// 1. Validates that the operation's target is compatible with its resource type.
/// 2. Maps the operation target to a pipeline node tree (currently a single
///    [`Request`] leaf node for point and single-partition operations).
pub(crate) fn plan_pipeline(operation: &CosmosOperation) -> azure_core::Result<Pipeline> {
    let resource_type = operation.resource_type();
    let target = operation.target();

    if !resource_type.is_valid_target(target) {
        return Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            format!(
                "operation target {target_desc} is not valid for resource type {resource_type}",
                target_desc = target_description(target),
            ),
        ));
    }

    let request_target = match target {
        OperationTarget::None => RequestTarget::NonPartitioned,
        OperationTarget::PartitionKey(pk) => RequestTarget::LogicalPartitionKey(pk.clone()),
        OperationTarget::FeedRange(_) => {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "FeedRange targeting is not yet implemented; \
                 fan-out pipeline planning requires partition resolution",
            ));
        }
    };

    let root = Request::new(operation.clone(), request_target);
    Ok(Pipeline::new(Box::new(root)))
}

fn target_description(target: &OperationTarget) -> &'static str {
    match target {
        OperationTarget::None => "None",
        OperationTarget::PartitionKey(_) => "PartitionKey",
        OperationTarget::FeedRange(_) => "FeedRange",
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;
    use crate::models::{
        AccountReference, ContainerProperties, ContainerReference, DatabaseReference,
        ItemReference, OperationType, PartitionKey, PartitionKeyDefinition, ResourceType,
        SystemProperties,
    };

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        )
    }

    fn test_database() -> DatabaseReference {
        DatabaseReference::from_name(test_account(), "db".to_owned())
    }

    fn test_partition_key_definition() -> PartitionKeyDefinition {
        serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap()
    }

    fn test_container_props() -> ContainerProperties {
        ContainerProperties {
            id: Cow::Owned("coll".into()),
            partition_key: test_partition_key_definition(),
            system_properties: SystemProperties::default(),
        }
    }

    fn test_container() -> ContainerReference {
        ContainerReference::new(
            test_account(),
            "db",
            "db_rid",
            "coll",
            "coll_rid",
            &test_container_props(),
        )
    }

    // --- plan_pipeline tests ---

    #[test]
    fn plans_non_partitioned_pipeline_for_database_read() {
        let op = CosmosOperation::read_database(test_database());
        let pipeline = plan_pipeline(&op).unwrap();

        let request = pipeline.root().downcast_ref::<Request>().unwrap();
        assert_eq!(*request.target(), RequestTarget::NonPartitioned);
        assert_eq!(request.operation().operation_type(), OperationType::Read);
        assert_eq!(request.operation().resource_type(), ResourceType::Database);
    }

    #[test]
    fn plans_logical_partition_pipeline_for_item_read() {
        let pk = PartitionKey::from("pk-value");
        let item = ItemReference::from_name(&test_container(), pk.clone(), "doc1");
        let op = CosmosOperation::read_item(item);
        let pipeline = plan_pipeline(&op).unwrap();

        let request = pipeline.root().downcast_ref::<Request>().unwrap();
        assert_eq!(
            *request.target(),
            RequestTarget::LogicalPartitionKey(pk.clone())
        );
        assert_eq!(request.operation().operation_type(), OperationType::Read);
        assert_eq!(request.operation().resource_type(), ResourceType::Document);
    }

    #[test]
    fn rejects_feed_range_target() {
        let op = CosmosOperation::read_all_items_cross_partition(test_container());
        let result = plan_pipeline(&op);

        let err = result.err().expect("expected error for FeedRange target");
        assert!(
            err.to_string().contains("FeedRange"),
            "expected FeedRange error, got: {err}"
        );
    }
}
