# Performance Tests

The Azure SDK defines a standardized set of performance tests which use a test framework defined by the [PerfAutomation tool](https://github.com/Azure/azure-sdk-tools/tree/main/tools/perf-automation).

Performance tests are defined in a "perf" directory under the package root.

By convention, all performance tests are named "perf" and are invoked via:

```bash
cargo test --package <package name> --test perf -- {perf test name} {perf test arguments}
```

where `package name` is the name of the rust package, `perf test name` is the name of the test you want to run, and `perf test arguments` is the arguments to that test.

Each performance test has the following standardized parameters:

* `--iterations <count>` - the number of iterations to run the test for. Default: 1
* `--sync` - Run only synchronous tests. (ignored)
* `--parallel <count>` - the number of concurrent tasks to use when running each test. Default: 1
* `--no-progress` - disable the once per second progress report.
* `--duration <seconds>` - the duration of each test in seconds. Default: 30
* `--warmup <seconds>` - the duration of the warmup period in seconds. Default: 5
* `--test-results <file>` - the file to write test results to (Default: tests/results.json)
* `--help` - show help.

Each test has its own set of parameters which are specific to the test.

## Test authoring

Performance tests have three phases:

1.. Setup - Establish any resources needed to run the test.
2.. Run - Actually perform the test.
3.. Cleanup - Cleanup any resources used by the test.

Each is defined by functions on the `PerfTest` trait.

### Test Metadata

Tests are defined by an instance of a `PerfTestMetadata` structure, which defines the name of the test, and other information about the test.

A perf test has a name (`get_secret`, `list_blobs`, `upload_blob`, etc), a short description, a set of test options, and a pointer to a function which returns an instance of the test.

Each perf test also has a set of command line options that are specific to the individual test, these are defined by a `PerfTestOptions` structure. It contains fields like help text for the option, activators

Here is an example of test metadata for a performance test:

```rust
PerfTestMetadata {
    name: "get_secret",
    description: "Get a secret from Key Vault",
    options: vec![PerfTestOption {
        name: "vault_url",
        display_message: "The URL of the Key Vault to use in the test",
        mandatory: true,
        short_activator: 'u',
        long_activator: "vault-url",
        expected_args_len: 1,
        ..Default::default()
    }],
    create_test: Self::create_new_test,
}
```

This defines a test named `get_secret` with a single required "vault_url" option.

For this test, the `create_new_test` function looks like:

```rust
fn create_new_test(runner: PerfRunner) -> CreatePerfTestReturn {
    async move {
        let vault_url_ref: Option<&String> = runner.try_get_test_arg("vault_url")?;
        let vault_url = vault_url_ref
            .expect("vault_url argument is mandatory")
            .clone();
        Ok(Box::new(GetSecrets {
            vault_url,
            random_key_name: OnceLock::new(),
            client: OnceLock::new(),
        }) as Box<dyn PerfTest>)
    }
    .boxed()
}
```

### Test invocation

The final piece of code which is necessary to run the performance tests is logic to hook up the tests with a test runner.

```rust
#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let runner = PerfRunner::new(
        env!("CARGO_MANIFEST_DIR"),
        file!(),
        vec![GetSecrets::test_metadata()],
    )?;

    runner.run().await?;

    Ok(())
}
```

This declares a perf test runner with a set of defined test metadata and runs the performance test. If your performance test suite has more than one performance test, then it should be added to the final parameter to the `PerfRunner::new()` function.

### Declaring Tests

The process of authoring tests starts with the cargo.toml file for your package.

Add the following to the `cargo.toml` file:

```toml
[[test]]
name = "perf"
path = "perf/get_secret.rs"
harness = false
```

This declares a test named `perf` (which is required for the perf automation tests) located in a directory named `perf` in a module named `get_secret.rs`. It also declares the test as *not* requiring the standard test harness - that's because the test defines its own test harness.

After this, to invoke your perf test, you simply use:

```bash
cargo test --package azure_storage_blob --test perf -- <performance test command line>
```

For example,

```bash
cargo test --package azure_storage_blob --test perf -- list_blob --help
```

returns the help text for the `list_blob`test:

```text
List blobs in a container

Usage: perf-070114707c71388a.exe list_blob [OPTIONS] --count <count>

Options:
  -c, --count <count>        The number of blobs to list
  -e, --endpoint <endpoint>  The endpoint of the blob storage
      --sync
      --parallel <COUNT>     The number of concurrent tasks to use when running each test [default: 1]
      --duration <SECONDS>   The duration of each test in seconds [default: 30]
      --warmup <SECONDS>     The duration of the warmup period in seconds [default: 5]
      --no-cleanup           Disable test cleanup
  -h, --help                 Print help
```

Note that some of these test options are not specific to the `list_blobs` test. This is to allow test options to be provided in any order in the command line.
