# Mock testing framework

## How to use

### Writing examples or test cases

If you want to write a test case or an example that supports the mock testing framework you must create a client with a a mock transport policy provided by the `mock_transport` utility crate located in the `eng/test` directory.

For instance in the `cosmos` crate, we do the following:

```rust
let transaction_name = "create_database_and_collection";
let transport_options = TransportOptions::new_custom_policy( // Build `TransportOptions` using a custom policy
    mock_transport::new_mock_transport(transaction_name.into()), // Use `mock_transport` to build the mock policy
);
let client = CosmosClient::builder(account_name, authorization_token)
    .transport(transport_options)
    .build(); // Build a `CosmosClient` using the custom `TransportOptions`
```

The transaction name must match a collection of transactions found in the "tests/transactions" directory at the root of the SDK workspace.

The above code expects there to be a `create_database_and_collection` subdirectory in the `tests/transactions` directory. This directory will contain all the outgoing requests and incoming responses for the example or test.

### Recording an execution

Instead of writing the request and response json files by hand, you can simply record a run of the example or test against live services. This ensures that the requests and responses are valid, working requests and responses. You can do this by executing the example/test in question with `TESTING_MODE` set to `RECORD`.

For example:

```bash
TESTING_MODE=RECORD cargo test create_database_and_collection
```

### Replaying an execution

Once there are request and response json files in the correct transaction folder, you can easily run the example or test against the locally stored requests and responses (instead of a live Azure cloud) by running the test again but this time with `TESTING_MODE` set to `REPLAY`.

For example:

```bash
TESTING_MODE=REPLAY cargo test create_database_and_collection
```

#### Limitations

The mock testing framework supports the pipeline architecture **only**. For this reason we cannot migrate the E2E test cases until the pipeline migration has been completed.

## Notes

Most available E2E test can and will support the new testing framework. While the mock testing framework will never replace proper testing against Azure, the mock framework will enable quick, reproducible tests.
