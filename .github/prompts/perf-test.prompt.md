---
agent: "agent"
description: "Generate a new performance test"
---

# Generate a new performance test

You will generate a new performance (perf) test under the `sdk/{service-directory}/{crate-name}/perf` directory as `${input:testName:perf_test_name}.rs` where:

- `{service-directory}` is the directory name under [sdk](../../sdk) for the current ${file} e.g., `core`.
- `{crate-name}` is the crate directory name under `sdk/{service-directory}` for the current ${file} e.g., `azure_core`.

## Set up

These instructions only need to be done once. If changes described are already present, do not make the changes again.

- Under the `{crate-name}` directory, in `Cargo.toml` add the following section:

  ```toml
  [[bench]]
  name = "perf"
  path = "perf/perf_tests.rs"
  harness = false
  ```

- Under the `{crate-name}` directory, add a `perf/perf_tests.rs` file that uses `azure_core_test::perf::PerfRunner` to register and run the crate's perf tests.
- Under the `{crate-name}` directory, copy [sdk/core/perf.yml](../../sdk/core/perf.yml) and change _only_ the following contents:
  - Change `ServiceDirectory` to match the `{service-directory}`.
- Under the `{crate-name}` directory, copy [sdk/core/perf-tests.yml](../../sdk/core/perf-tests.yml) and change _only_ the following contents:
  - Change `Service` to match the `{service-directory}`.
  - Change `Project` to match the `{crate-name}`.
  - Change the `PackageVersions` to replace `azure_core` with the `{crate-name}`.

## Adding a new perf test

For each new perf test:

- In `sdk/{service-directory}/{crate-name}/perf/${input:testName}.rs`:
  - Add a type that implements `azure_core_test::perf::PerfTest`.
  - Generate a perf test for the currently selected function in ${file} or specified client method name, if any.
  - Use [sdk/keyvault/azure_security_keyvault_keys/perf/get_key.rs](../../sdk/keyvault/azure_security_keyvault_keys/perf/get_key.rs) or [sdk/storage/azure_storage_blob/perf/list_blob_test.rs](../../sdk/storage/azure_storage_blob/perf/list_blob_test.rs) as examples.
  - Define a `test_metadata()` function returning `PerfTestMetadata`, including any required command-line options.
  - Separate setup, the measured operation, and cleanup in the `PerfTest` implementation.
  - If credentials are needed, prefer the crate's existing perf-test pattern (for example `context.recording().credential()` or an existing helper) instead of introducing a new authentication flow.
  - If the client has options, instrument them with `recording.instrument_perf(&mut options.client_options)?`.
- In `sdk/{service-directory}/{crate-name}/perf/perf_tests.rs`:
  - Add `mod ${input:testName};`.
  - Register the new test metadata with `PerfRunner::new(...)`.
- In the `{crate-name}/perf-tests.yml` file:
  - Add a top-level `Tests:` property if it does not already exist.
  - Add the following YAML under the `Tests:` property as a separate YAML array object:

    ```yaml
    - Test: ${input:testName}
      Class: ${input:testName}
      Arguments:
        - --sync
    ```

- Match existing crate patterns for client construction, shared setup state, and environment variable names.
