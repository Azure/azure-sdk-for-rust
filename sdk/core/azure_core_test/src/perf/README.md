# Performance Tests

The Azure SDK defines a standardized set of performance tests which use a test framework defined by the [PerfAutomation tool](https://github.com/Azure/azure-sdk-tools/tree/main/tools/perf-automation).

Performance tests are defined in a "perf" directory under the package root.

By convention, all performance tests are named "perf" and are invoked via:

```bash
cargo bench --package <package name> --bench perf -- {perf test name} {perf test arguments}
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

***NOTE: Performance Tests are "recorded" tests***

This means that they follow the same rules as tests annotated with the `#[recorded::test]` attribute. There is one difference between perf tests and tests with the `recorded::test` attribute: perf tests default to `live` mode, and normal `recorded::test` tests default to `playback` mode.

To configure the tests for record mode tests, set `AZURE_TEST_MODE` to `record` before running your performance tests, and to run your tests using the test proxy, set `AZURE_TEST_MODE` to `playback`

## Test authoring

Performance tests have three phases:

1. Setup - Establish any resources needed to run the test.
2. Run - Actually perform the test.
3. Cleanup - Cleanup any resources used by the test.

Each is defined by functions on the `PerfTest` trait.

### Test Metadata

Tests are defined by an instance of a `PerfTestMetadata` structure, which defines the name of the test, and other information about the test.

A perf test has a name (`get_secret`, `list_blobs`, `upload_blob`, etc), a short description, a set of test options, and a pointer to a function which returns an instance of the test.

Each perf test also has a set of command line options that are specific to the individual test, these are defined by a `PerfTestOptions` structure. It contains fields like help text for the option, activators

An example of perf test metadata [can be found here](https://github.com/Azure/azure-sdk-for-rust/blob/e47a38f93e7ac2797754c103da7fe8b177e46365/sdk/keyvault/azure_security_keyvault_keys/perf/create_key.rs#L26C1-L41C1)

This defines a test named `create_key` with a single required "vault_url" option.

An example of the `create_new_test` function [can be found here](https://github.com/Azure/azure-sdk-for-rust/blob/e47a38f93e7ac2797754c103da7fe8b177e46365/sdk/keyvault/azure_security_keyvault_keys/perf/get_key.rs#L42-L58)

### Test invocation

The final piece of code which is necessary to run the performance tests is logic to hook up the tests with a test runner.

An example of this, from the Keyvault Keys performance tests [can be found here](https://github.com/Azure/azure-sdk-for-rust/blob/e47a38f93e7ac2797754c103da7fe8b177e46365/sdk/keyvault/azure_security_keyvault_keys/perf/perf_tests.rs#L24-L35)

This declares a perf test runner with a set of defined test metadata and runs the performance test. If your performance test suite has more than one performance test, then it should be added to the final parameter to the `PerfRunner::new()` function.

### Declaring Tests

The process of authoring tests starts with the cargo.toml file for your package.

Add the following to the `cargo.toml` file:

```toml
[[bench]]
name = "perf"
path = "perf/get_secret.rs"
harness = false
```

This declares a test named `perf` (which is required for the perf automation tests) located in a directory named `perf` in a module named `get_secret.rs`. It also declares the test as *not* requiring the standard test harness - that's because the test defines its own test harness.

After this, to invoke your perf test, you simply use:

```bash
cargo bench --package azure_storage_blob --bench perf -- {performance test command line}
```

For example,

```bash
cargo bench --package azure_storage_blob --bench perf -- list_blob --help
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

### Declaring a test pipeline

Before you can declare your test pipeline, you need to create some infrastructure for your tests.

#### Test Pipeline Yaml Configuration

Test pipelines are defined using a [`perf.yml`](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/storage/azure_storage_blob/perf.yml) file declared in the package directory.

For example (from the `storage/azure_storage_blob` package):

```yml
trigger: none

pr: none

# Schedule the pipeline to run at UTC+7 Hours (Midnight Pacific time)
schedules:
- cron: "0 7 * * *"
  displayName: Daily midnight run.
  branches:
    include:
    - main
  always: true

parameters:
- name: PackageVersions
  displayName: PackageVersions (regex of package versions to run)
  type: string
  default: '12|source'
- name: Tests
  displayName: Tests (regex of tests to run)
  type: string
  default: '^(download|upload|list-blobs)$'
- name: Arguments
  displayName: Arguments (regex of arguments to run)
  type: string
  default: '(10240)|(10485760)|(1073741824)|(5 )|(500 )|(50000 )'
- name: Iterations
  displayName: Iterations (times to run each test)
  type: number
  default: '5'
- name: Profile
  type: boolean
  default: false
- name: AdditionalArguments
  displayName: AdditionalArguments (passed to PerfAutomation)
  type: string
  default: ' '

extends:
  template: /eng/pipelines/templates/jobs/perf.yml
  parameters:
    ServiceDirectory: storage/azure_storage_blob
    PackageVersions: ${{ parameters.PackageVersions }}
    Tests: ${{ parameters.Tests }}
    Arguments: ${{ parameters.Arguments }}
    Iterations: ${{ parameters.Iterations }}
    AdditionalArguments: ${{ parameters.AdditionalArguments }}
    Profile: ${{ parameters.Profile }}
```

You'll want to configure the `ServiceDirectory` field to match the location of your package, and tweak the default values for the variables to match your performance tests.

#### Performance Test Yaml Configuration

Once you've created a `perf.yml` file, you need to create a `perf-tests.yml` file in the same directory. This file is used during the performance automation to configure the performance tests which are run.

```yml
Service: storage-blob

Project: azure-storage-blobs-perf

PrimaryPackage: azure_storage_blob

PackageVersions:
- azure_storage_blob: source
  azure_core: source

Tests:
- Test: download
  Class: DownloadBlob
  Arguments:
  - --size 10240 --parallel 64
  - --size 10485760 --parallel 32
  - --size 1073741824 --parallel 1 --warmup 60 --duration 60
  - --size 1073741824 --parallel 8 --warmup 60 --duration 60

- Test: upload
  Class: UploadBlob
  Arguments:
  - --size 10240 --parallel 64
  - --size 10485760 --parallel 32
  - --size 1073741824 --parallel 1 --warmup 60 --duration 60
  - --size 1073741824 --parallel 8 --warmup 60 --duration 60

- Test: list-blobs
  Class: list_blob
  Arguments:
  - --count 5 --parallel 64
  - --count 500 --parallel 32
  - --count 50000 --parallel 32 --warmup 60 --duration 60
```

This example (from Azure Storage Blobs) defines a service and project, and specifies the package versions which are going to be tested (this option currently is unsupported for Rust, so leave this as 'source').

The key part is the "Tests" node - that defines the test parameters which will be run for each performance test.

And FINALLY, you need to create a pull request containing this file and find the SHA for the commit containing your changes. This will be important for the next steps.

#### Creating the performance pipeline

Once the pull request you created above has been committed to the branch, you can start to create the performance pipelines (note: DO NOT ATTEMPT TO CREATE THE PIPELINE UNTIL THE `perf.yml` file mentioned above is in `main` - if you don't, you are highly likely to disrupt all operations in the repository).

Navigate to the `azure-sdk` Azure DevOps instance, and select the `internal` project.

Within the `internal` project, select `Pipelines`, select "All" from the right hand pane. This will show a tree structured hierarchy of pipelines.

Navigate to the `perf` part of the hierarchy and you'll see a list of languages (`cpp`, `java`, `net`, `rust`).

Open the `rust` node and you'll see the defined Rust performance test pipelines. Click on the `rust` node to select just the `rust` pipeline container.

Click on the `New pipeline` button on the top right of the window.

Select `GitHub` and then select the 'All Repositories` combo on the right.

Next select `Azure/azure-sdk-for-rust` to specify the Rust SDK and configure your pipeline with an `Existing Azure Pipelines YAML file`.

Select your pipeline file from the main branch of the repository and you're almost done.

The next thing you want to do is to "save" the new pipeline. This will cause your pipeline to be created. You can also attempt to `run` the pipeline at this point but it is likely to fail.

You now need to set the required variables or the pipeline. Performance pipelines require the `Secrets for Resource Provisioner` variable group added to the pipeline. To add this, select the newly created pipeline, and click on `Edit`. Navigate to the `...` menu and select `Triggers`. This brings up the `Yaml`, `Variables`, `Triggers` and `History` edit. Make sure that all the triggers (included scheduled triggers) are cleared from the `Triggers` - Rust performance pipeline triggers are managed by the pipeline yaml file, rather than in the Azure DevOps user interface.

Select `Variables`, which allows you to add variables to the pipeline. You want to select `Variable groups` in the left hand column and select `Link variable group` in the right hand column.

That will bring up a pane on the right with a number of variable groups. You want to select the `Secrets for Resource Provisioner` variable group and click the `Link` button.

Once you've saved these changes, your pipeline should be ready to run.

You may need to ask for help from the Azure SDK Engineering Systems team to enable access to test resources for your pipeline.

## Running the test automation locally

It is possible to run the performance test automation locally, that is often helpful when debugging performance tests.

Running the performance tests locally requires a clone of the `Azure/azure-sdk-tools` repo.

Start at the root of the `azure-sdk-tools` repo and navigate to the `tools/perf-automation/Azure.Sdk.Tools/PerfAutomation` directory:

```bash
cd tools/perf-automation/Azure.Sdk.Tools/PerfAutomation
```

Then run the perf automation tool, replacing your repo on the command line:

```bash
 dotnet run . -- -l rust --language-version N/A --tests-file {Path to perf-tests.yml file in your repo} --repo-root {Path to the root of your repo}
 ```

 This will run your performance tests and save the results in the `results` directory locally.
