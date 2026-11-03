// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::serde::skip_serializing_read_only;
use azure_security_keyvault_secrets::models::{
    BackupSecretResult, DeletedSecret, DeletedSecretProperties, KeyVaultError,
    ListDeletedSecretPropertiesResult, ListSecretPropertiesResult, Secret, SecretAttributes,
    SecretProperties,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

fn round_trip_read_only<T: DeserializeOwned + Serialize>(value: Value) {
    let model: T = serde_json::from_value(value.clone()).expect("valid response model");
    let actual = serde_json::to_value(model).expect("serializable response model");
    let expected = if skip_serializing_read_only(&value) {
        json!({})
    } else {
        value
    };
    assert_eq!(actual, expected);
}

#[test]
fn read_only_fields_follow_fake_features_in_consumer_builds() {
    round_trip_read_only::<BackupSecretResult>(json!({"value": "_w"}));
    round_trip_read_only::<Secret>(json!({"kid": "key-id", "managed": true}));
    round_trip_read_only::<DeletedSecret>(json!({
        "kid": "key-id", "managed": true, "deletedDate": 123, "scheduledPurgeDate": 456
    }));
    round_trip_read_only::<DeletedSecretProperties>(json!({
        "managed": true, "deletedDate": 123, "scheduledPurgeDate": 456
    }));
    round_trip_read_only::<SecretProperties>(json!({"managed": true}));
    round_trip_read_only::<SecretAttributes>(json!({
        "created": 123, "updated": 456, "recoverableDays": 7, "recoveryLevel": "Recoverable"
    }));
    round_trip_read_only::<KeyVaultError>(json!({
        "error": {"code": "Outer", "message": "outer error", "innererror": {
            "code": "Inner", "message": "inner error"
        }}
    }));
    round_trip_read_only::<ListSecretPropertiesResult>(json!({
        "nextLink": "/opaque?cursor=next",
        "value": [{"id": "secret-id", "managed": true, "attributes": {"created": 123}}]
    }));
    round_trip_read_only::<ListDeletedSecretPropertiesResult>(json!({
        "nextLink": "/opaque?cursor=next",
        "value": [{"id": "deleted-id", "managed": true, "deletedDate": 123}]
    }));
}

#[test]
fn absent_optional_read_only_fields_are_omitted() {
    assert_eq!(serde_json::to_value(Secret::default()).unwrap(), json!({}));
    assert_eq!(
        serde_json::to_value(SecretAttributes::default()).unwrap(),
        json!({})
    );
    assert_eq!(
        serde_json::to_value(BackupSecretResult::default()).unwrap(),
        json!({})
    );
}

#[test]
fn empty_read_only_lists_follow_fake_features() {
    round_trip_read_only::<ListSecretPropertiesResult>(json!({"value": []}));
    round_trip_read_only::<ListDeletedSecretPropertiesResult>(json!({"value": []}));
}
