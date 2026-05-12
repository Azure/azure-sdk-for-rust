# Storage Blob Examples

This directory contains a set of examples for the use of the Blob Storage clients.

## Examples

| File | Description |
| --- | --- |
| `blob_hello_world.rs` | Getting started: create a container, upload and download a blob |
| `blob_client.rs` | Blob-level operations: exists, metadata, index tags, access tier |
| `blob_container_client.rs` | Container-level operations: metadata, list blobs with continuation, access policies |
| `blob_service_client.rs` | Service-level operations: list containers, service properties (CORS), find blobs by tags |
| `block_blob_client.rs` | Block blob operations: staged block upload, copy from URL |
| `append_blob_client.rs` | Append blob operations: create, append blocks, seal |
| `page_blob_client.rs` | Page blob operations: create, upload/clear pages, list page ranges, resize |
| `blob_storage_logging.rs` | Logging and OpenTelemetry distributed tracing |
| `blob_storage_upload_file.rs` | Upload a local file with streaming support for large files |
| `storage_error.rs` | Structured error handling with `StorageError` |

## Setup

The following environment variables need to be set:

- `AZURE_STORAGE_ACCOUNT_NAME=<storage_account_name>`
