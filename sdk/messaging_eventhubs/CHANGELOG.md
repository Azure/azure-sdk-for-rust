# Change log

## 0.14.1

- Renamed the following constructor methods and marked the old methods as deprecated
  - `from_connection_string()` -> `new_from_connection_string()`
  - `from_namespace_and_credential()` -> `new_from_credential()`
  - `from_namespace_and_named_key_credential()` -> `new_from_named_key_credential()`
  - `from_namespace_and_sas_credential()` -> `new_from_sas_credential()`
- Removed generic parameter `C` from `EventHubConnection`

## 0.14.0

- Fixed problem with `azure_identity` credentials
- Added example showcasing how to work with `azure_identity` credentials

## 0.14.0-alpha

- Changed version number to follow that of `azure_core`
- Changing visibility of the following to public
  - `BasicRetryPolicy`,
  - `mod authorization`,
  - `EventHubTokenCredential`
  - `SharedAccessCredential`
  - `AzureNamedKeyCredential`
  - `AzureSasCredential`
- Added helper function `crate::authorization::build_connection_signature_authorization_resource()`
- Added following convenience constructor methods to `EventHubConnection`, `EventHubProducerClient`, and `EventHubConsumerClient`
  - `from_namespace_and_named_key_credential()`
  - `from_namespace_and_sas_credential()`

## 0.1.2

### 0.1.2-beta

- Changed visibility of `IntoAzureCoreError` to `pub(crate)` and restricted its impl to only foreign
  error types. All other error type natively implements `Into<azure_core::error::Error>`

### 0.1.2-alpha

- Fixed a bug where `EventStream` is not `Send` because `ClosingBoxedFuture` misses `Send` in its
  trait bounds
- Changed visibility of struct `EventStream` to public
- Changed visibility of trait `IntoAzureCoreError` to public
- Updated dependency `azure_core` to `"0.13"`
- Updated dependency `time`'s version to `"<=0.3.23"`, which is the latest version that supports
  rust version 1.65

## 0.1.1

- Fixed wrong crate name in the example

## 0.1.0

- Initial release
