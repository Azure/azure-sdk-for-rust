# Mock testing framework

## How to use

### Writing examples or test cases

If you want to write a test case or an example that supports the mock testing framework all you have to do is create a `Context` value with a transaction name that matches a collection of transactions found in the "tests/transactions" directory at the root of the SDK workspace.

```rust
let context = Context::new("create_database_and_collection");
```

**NOTE**: This requires that the `mock_transport_framework` feature is enabled. Otherwise the above code will not compile.

The above code expects there to be a `create_database_and_collection` subdirectory in the `tests/transactions` directory. This directory will contain all the outgoing requests and incoming responses for the example or test.

### Recording an execution

Instead of writing the request and response json files by hand, you can simply record a run of the example or test against live services. This ensures that the requests and responses are valid, working requests and responses. You can do this like so:

1. Execute the example/test with the `mock_transport_framework` feature enabled.
2. Export the environmental variable `TESTING_MODE` with the value of `RECORD`.

For example:

```bash
TESTING_MODE=RECORD cargo run --example create_database_and_collection --features=mock_transport_framework
```

### Replaying an execution

Once there are request and response json files in the correct transaction folder, you can easily run the example or test against the locally stored requests and responses (instead of a live Azure cloud) like so:

1. Execute the example/test with the `mock_transport_framework` feature enabled.
2. Make sure the environmental variable `TESTING_MODE` is either not set or set with the value of `REPLAY`.

For example:

```bash
TESTING_MODE=REPLAY cargo run --example create_database_and_collection --features=mock_transport_framework
```

#### Limitations

The mock testing framework supports the pipeline architecture **only**. For this reason we cannot migrate the E2E test cases until the pipeline migration has been completed.

## Notes

Most available E2E test can and will support the new testing framework. While the mock testing framework will never replace proper testing against Azure, the mock framework will enable quick, reproducible tests.
