# Azure Storage Stress Framework

This library contains the core runner and supporting types for Azure Storage stress tests.

## Test Design

This framework runs a `StressTest` for a given amount of time. A `StressTest` is primarily a factory for `StressTestOperation` objects. These operations are produced and run at a configurable parallelism level for the full duration of the stress test.

To stress test a crate, a new command-line program is written with a single implementation of `StressTestFactory`, a `clap::Subcommand` which will produce the correct `StressTest` based on the results of `clap::Parser::parse()`.

## Options

This framework uses `clap` for argument parsing. The core runner has several configurations defined in `src/args/rs`. All of these are visible through `--help` in the consuming commandline executable (in `clap` terms, all all core runner args are all `global`).

### Test Options

- `--parallel` (default value: 1)  
  How many operations to run in parallel at a given time. These will be run as separately spawned async tasks. Note that multipart transfers will have separate concurrency controls independent of this one.
- `--duration` (default value: 10)  
  Duration of the stress test, in seconds.
- `--setup-timeout`, `--operation-timeout`, `--cleanup-timeout`  
  Optional timeouts in seconds for global setup, individual operations, and global cleanup.

### Output Options

- `--log-pretty`  
  Flag to write logs more easily human-readable. In practice, this is currently limited to json output indentation.
- `--results-log-frequency` (default value: 100)  
  How often to log a running tally of results, in overall tests run. 0 disables these logs. This setting does not affect logs on failed operations.

### Fault Injection Options

Fault injection requires the `http-fault-injector` server to be running locally on the standard port.

- `--fault-standard`  
  Flag for whether to use a standard set of hardcoded, non-zero probabilities for fault injection. Incompatible with `--fault-injection-file`.
- `--fault-injection-file`  
  Path to a json file containing probabilities for fault injection. Incompatible with `--fault-standard`

Additionally, there is a `--fault-<code>` override argument for every individual fault code supported by the fault injector. E.g. `--fault-injection-file foo.json --fault-pa 0.1` will use the fault probabilities from the file `foo.json`, replacing the `pa` (partial response, then abort) probability from that file with 0.1.
