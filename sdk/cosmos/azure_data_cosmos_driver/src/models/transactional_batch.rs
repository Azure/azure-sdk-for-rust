// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Validation for the service-defined transactional batch envelope.

use crate::models::cosmos_resource_reference::validate_resource_name;
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransactionalBatchOperation<'a> {
    #[serde(borrow)]
    operation_type: Option<Cow<'a, str>>,
    #[serde(borrow)]
    id: Option<Cow<'a, str>>,
    #[serde(borrow)]
    resource_body: Option<TransactionalBatchResourceBody<'a>>,
}

#[derive(Deserialize)]
struct TransactionalBatchResourceBody<'a> {
    #[serde(borrow)]
    id: Option<Cow<'a, str>>,
}

pub(crate) fn validate_resource_names(body: Option<&[u8]>) -> crate::error::Result<()> {
    let Some(body) = body else {
        return Ok(());
    };
    let operations =
        serde_json::from_slice::<Vec<TransactionalBatchOperation<'_>>>(body).map_err(|error| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::status_codes::SERIALIZATION_REQUEST_BODY_INVALID)
                .with_message("transactional batch body must be a valid JSON operation array")
                .with_source(error)
                .build()
        })?;

    for operation in operations {
        match operation.operation_type.as_deref() {
            Some("Create" | "Upsert") => {
                validate_id(operation.id.as_deref())?;
                validate_id(
                    operation
                        .resource_body
                        .as_ref()
                        .and_then(|resource_body| resource_body.id.as_deref()),
                )?;
            }
            Some("Replace" | "Read" | "Delete") => validate_id(operation.id.as_deref())?,
            _ => {}
        }
    }

    Ok(())
}

fn validate_id(id: Option<&str>) -> crate::error::Result<()> {
    if let Some(id) = id {
        validate_resource_name("item", id)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unsafe_transactional_batch_ids() {
        for operation in [
            serde_json::json!({
                "operationType": "Create",
                "resourceBody": { "id": "../other-container" }
            }),
            serde_json::json!({
                "operationType": "Upsert",
                "id": "item?query",
                "resourceBody": { "id": "safe-item" }
            }),
            serde_json::json!({
                "operationType": "Replace",
                "id": "item/name",
                "resourceBody": { "id": "item/name" }
            }),
            serde_json::json!({
                "operationType": "Read",
                "id": ".\t."
            }),
            serde_json::json!({
                "operationType": "Delete",
                "id": ""
            }),
        ] {
            let body = serde_json::to_vec(&[operation]).unwrap();
            let error = validate_resource_names(Some(&body))
                .expect_err("unsafe transactional batch item names must be rejected");
            assert_eq!(
                error.status(),
                crate::error::status_codes::CLIENT_BAD_REQUEST
            );
        }
    }

    #[test]
    fn accepts_safe_and_unknown_transactional_batch_operations() {
        for body in [
            br#"[{"operationType":"Create","resourceBody":{"id":"safe-item"}}]"#.as_slice(),
            br#"[{"operationType":"Read","id":"safe-item"}]"#.as_slice(),
            br#"[{"operationType":"FutureOperation","id":"../opaque"}]"#.as_slice(),
        ] {
            validate_resource_names(Some(body))
                .expect("safe and unknown batch operations must pass driver validation");
        }
    }

    #[test]
    fn rejects_malformed_transactional_batch_bodies() {
        for body in [
            br#"{"not":"an array"}"#.as_slice(),
            b"not-json".as_slice(),
            br#"[{"operationType":"Read","id":"safe","id":"../unsafe"}]"#.as_slice(),
            br#"[{"operationType":"Create","resourceBody":{"id":42}}]"#.as_slice(),
        ] {
            let error = validate_resource_names(Some(body))
                .expect_err("malformed batch envelopes must be rejected");
            assert_eq!(
                error.status(),
                crate::error::status_codes::SERIALIZATION_REQUEST_BODY_INVALID
            );
        }
    }
}
