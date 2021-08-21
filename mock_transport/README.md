# Mock testing framework

## How to use

### Writing examples or test cases

If you want to write a test case or an example that supports the mock testing framework all you have to do is to call the `start_transaction` function before interacting with Azure. The function expects a transaction name that must be unique. For example:

```rust
azure_core::mock_transport::start_transaction("create_database_and_collection");
```

This call initializes the mock framework and it's all you need to add to your code (*note*: this call resolves in a noop if you do not enable the mock testing framework).

#### Limitations

* The mock testing framework supports async and multithreading but the call order **must** be deterministic. Also, for this same reason, tests cannot be run concurrently (with `cargo test -- --test-threads=1`).
* The mock testing framework supports the pipeline architecture **only**. For this reason we cannot migrate the E2E test cases until the pipeline migration has been completed.

### Recording an execution 

1. Make sure the `AzureSDKforRustDumpPathRoot` environmental variable is set to a valid, writable path. This is where the requests/responses will be stored (into a subfolder named as the transaction name defined in the code).
2. Execute the example/test with the `mock_transport_generate` feature enabled.

For example: 

```bash
cargo run --example create_database_and_collection --features mock_transport_generate
```

### Replaying an execution

1. Make sure the `AzureSDKforRustDumpPathRoot` environmental variable is set to a valid, readable path. This is where the requests/responses will be read from (from a subfolder named as the transaction name defined in the code).
2. Execute the example/test with the `mock_transport_consume` feature enabled.

For example: 

```bash
cargo run --example create_database_and_collection --features mock_transport_consume
```

### Executing the recorded tests locally

Under the folder `mock_transport` you will find recorded examples and test cases. You can reply them locally to make sure your code changes did non behave unexpectedly. The existing test cases will be checked by GitHub Actions (in the future). If you think your new code is correct even though it breaks the existing recorded tests, please include the new recordings in your PR. 
## Notes

Most available E2E test can and will support the new testing framework. While the mock testing framework will never replace proper testing against Azure, the mock framework will enable quick, reproducible tests.
