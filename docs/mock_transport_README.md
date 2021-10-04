# Mock testing framework

## How to use

### Writing examples or test cases

If you want to write a test case or an example that supports the mock testing framework all you have to do is to call the `start_mock_transaction` function of the `Context` before passing it to the pipeline. 

```rust
let mut context = Context::new();
context.start_mock_transaction("create_database_and_collection");
```

The function expects a transaction name that must be unique. 
This call initializes the mock framework and it's all you need to add to your code (*note*: this call resolves in a noop if you do not enable the mock testing framework).

#### Limitations

 The mock testing framework supports the pipeline architecture **only**. For this reason we cannot migrate the E2E test cases until the pipeline migration has been completed.

### Recording an execution 

1. Execute the example/test with the `mock_transport_framework` feature enabled.
2. Export the environmental variable `TESTING_MODE` with the value of `RECORD`.

For example: 

```bash
TESTING_MODE=PLAY cargo run --example create_database_and_collection --features=mock_transport_framework
```

### Replaying an execution

1. Execute the example/test with the `mock_transport_framework` feature enabled.
2. Export the environmental variable `TESTING_MODE` with the value of `PLAY`.

For example: 

```bash
TESTING_MODE=RECORD cargo run --example create_database_and_collection --features=mock_transport_framework
```

### Executing the recorded tests locally

Under the folder `SessionRecords` you will find recorded examples and test cases. You can reply them locally to make sure your code changes did non behave unexpectedly. The existing test cases will be checked by GitHub Actions (in the future). If you think your new code is correct even though it breaks the existing recorded tests, please include the new recordings in your PR. 
## Notes

Most available E2E test can and will support the new testing framework. While the mock testing framework will never replace proper testing against Azure, the mock framework will enable quick, reproducible tests.
