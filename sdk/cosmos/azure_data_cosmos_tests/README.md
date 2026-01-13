# azure_data_cosmos_tests

This crate contains integration tests for `azure_data_cosmos`.

It is **not published** (`publish = false`) and exists to keep test-only feature flags and dependencies out of the customer-facing `azure_data_cosmos` crate.

## Running tests

Run all tests that require key auth:

```bash
cargo test -p azure_data_cosmos_tests --features key_auth
```

Run tests that also exercise the preview query engine:

```bash
cargo test -p azure_data_cosmos_tests --features key_auth,preview_query_engine
```

## Environment

Tests are driven by `tests/framework/test_client.rs`.

- `AZURE_COSMOS_CONNECTION_STRING`
  - Set to a Cosmos DB connection string.
  - Special value: `emulator` uses the well-known local emulator connection string.
- `AZURE_COSMOS_TEST_MODE`: `required` | `allowed` | `skipped` (default: `allowed`)
- `AZURE_COSMOS_ALLOW_INVALID_CERT`: `true` to allow invalid TLS certs (useful for emulator)

