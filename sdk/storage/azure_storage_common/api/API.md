# azure_storage_common

- **Description**: Common types shared across Azure Storage client libraries.
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`

```rust
pub mod models {
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct UserDelegationKey {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_delegated_user_tid: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::rfc3339::option")]
        pub signed_expiry: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_oid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_service: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::rfc3339::option")]
        pub signed_start: Option<azure_core::time::OffsetDateTime>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_tid: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub signed_version: Option<String>,
        #[serde(default, deserialize_with = "base64::option::deserialize", serialize_with = "base64::option::serialize", skip_serializing_if = "Option::is_none")]
        pub value: Option<Vec<u8>>,
    }
}
```
