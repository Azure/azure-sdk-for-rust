# Storage Queue Examples

This directory contains a set of examples for the use of the Storage Queue clients.

## Examples

| File | Description |
| --- | --- |
| `queue_hello_world.rs` | Getting started: create a queue, send and receive messages |
| `queue_client.rs` | Queue-level operations: metadata, send/peek/receive/delete, time-to-live/visibility options |
| `queue_service_client.rs` | Service-level operations: list queues, service properties, statistics |
| `access_policy.rs` | Set and get queue access policies (stored access policies for SAS) |

For logging and OpenTelemetry distributed tracing, see the
[`samples/storage_blob_logging`](https://github.com/Azure/azure-sdk-for-rust/tree/main/samples/storage_blob_logging) sample.
Logging and distributed tracing works the same across all Azure SDK for Rust client libraries.

## Setup

The following environment variables need to be set:

- `AZURE_QUEUE_STORAGE_ACCOUNT_NAME=<storage_account_name>`
