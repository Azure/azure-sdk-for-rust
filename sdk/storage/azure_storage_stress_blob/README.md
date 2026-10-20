# Azure Blob Storage Stress Tests

This executable runs stress tests on the `azure_storage_blob` crate, using the `azure_storage_stress` framework.

## Usage

Stress tests may be run through the executable produced from compiling this crate. Be sure to compile a release build.

From the repository root, you may run the following on a Windows machine:
```
cargo build --release --package azure_storage_stress_blob
.\target\release\azure_storage_stress_blob.exe
```

And on a Linux machine:
```
cargo build --release --package azure_storage_stress_blob
target/release/azure_storage_stress_blob
```

The above will print the `--help` text. To run an actual test, supply the correct arguments, including the subcommand (test ID) and any arguments to configure the test. Available subcommands are printed out in the help text.

Currently, there is only one test defined: `roundtrip`. Using the minimum possible arguments, the following will repeatedly, sequentially, roundtrip a 10 KiB blob for ten seconds:
```
.\target\release\azure_storage_stress_blob.exe roundtrip direct-memory --data-len 10240
```

A more complex invocation below. This test will run for an hour, transferring 1 GiB streamed blobs roundtrip. It will run 32 of these roundtrip operations at a time, with each of the partitioned transfer operations transferring 64 MiB blocks sequentially. It will use hardcoded fault-simulation probabilities, which means the machine must be running `http-fault-injector` locally or else the tests will fail. The global test setup (creating a container) will timeout after a minute, cancelling the whole test with it. All output will be prettified for improved human readability.
```
.\target\release\azure_storage_stress_blob.exe roundtrip generated-stream --duration 3600 --parallel 32 --setup-timeout 60 --data-len 1073741824 --block-len 67108864 --concurrency 1 --fault-standard --log-pretty
```

Note that framework options are intermixed with test-specific options. This is due to marking all framework options as `global` through `clap`. Those options can go before even the subcommand (e.g. `--duration 3600 roundtrip generated-stream --log-pretty`), though the test options *must* come after the subcommand and it's positional args. For simplicity, it is easiest to put all options at the end of the command.

## Tests

In addition to each test's arguments, global arguments for the runner are also defined. See `azure_storage_stress` documentation for more info.

### Roundtrip

An operation in this test is to generate a unique blob, upload its contents, then download and ensure no corruption. Download bytes are streamed through a checksum calculation for integrity checks and then immediately dropped.

Currently, upload and download mechanisms are not configurable. The operation uses the partitioned upload and download methods.

#### Positional Arguments

1. Data source (**required**)  
   The type of the data source. Valid values include `generated-stream` and `direct-memory`.  
   A "generated stream" is a custom type to stream over pre-generated random bytes.  
   "Direct memory" is a `Vec<u8>` of the data length containing pre-generated random bytes.  
   To speed up generation, these data sources only generate a little under 10 KiB and then cycle over that generated sequence. The value is of odd length to avoid any errors which may be hidden through block alignment.

#### Options

- `--data-len` (**required**)  
  The length of the blob to test.
- `--concurrency` (default value: 2)  
  The value to use for the parallel value of the partitioned transfers.
- `--block-len` (default value: 4 MiB)  
  The value to use for the partition size in partitioned transfers.
