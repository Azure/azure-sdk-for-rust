# Requirements for performance tests

Each performance test consists of three phases:

1) Warmup
1) Test operation
1) Cleanup

## Common test inputs

* Duration of the test in seconds
* Number of iterations of the main test loop
* Parallel - number of operations to execute in parallel
* Disable test cleanup
* Test Proxy servers.
* Results file - location to write test outputs
* Warmup - Duration of the warmup in seconds.
* TLS
  * Allow untrusted TLS certificates
* Advanced options
  * Print job statistics (?)
  * Track latency and print per-operation latency statistics
  * Target throughput (operations/second) (?)
* Language specific options
  * Max I/O completion threads
  * Minimum number of asynchronous I/O threads in the thread pool
  * Minimum number of worker threads the thread pool creates on demand
  * Sync - run a synchronous version of the test

## Expected test outputs

Each test is expected to generate the following elements:

* Package Versions - a set of packages tested and their versions.
* Operations per second - Double precision float
* Standard Output of the test
* Standard Error of the test
* Exception - Text of any exceptions thrown during the test.
* Average CPU Use during the test - Double precision float.
* Average memory use during the test - Double precision float.

## Perf Test Harness

Each performance test defines a `get_metadata()` function which returns a `TestMetadata` structure.

A `TestMetadata` structure contains the following fields

```rust
pub struct TestMetadata {
    name: &'static str
    description: &'static str
    options: &'static[&'static TestOption]
}
```

A `TestOptions` defines a set of options for the test which will be merged with the common test inputs to define the command line for the performance test.

```rust
pub struct TestOption {
    /// The name of the test option. This is used as the key in the `TestArguments` map.
    name: &'static str,

    long_activator: &str,

    short_activator:&str,

    /// Display message - displayed in the --help message.
    display_message: &[str],

    /// Expected argument count
    expected_args_len: u16,

    /// Required
    mandatory: bool,

    /// Argument value is sensitive and should be sanitized.
    sensitive: bool,
}
```
