---
agent: "agent"
description: "Generate a new performance test"
---

You will generate a new performance (perf) test under the `sdk/{service-directory}/{crate-name}/benches` directory named ${input:benchName:bench_name} where:

-   `{service-directory}` is the directory under [sdk] directory for the current ${file} e.g., `sdk/core`.
-   `{crate-name}` is the directory under `{service-directory}` for the current ${file} e.g., `sdk/core/azure_core`.

## Set up

These instructions only need to be done once. If changes described are already present, do not make the changes again.

-   Under the `{crate-name}` directory, in `Cargo.toml` add `criterion.workspace = true` to the `[dev-dependencies]` section.
-   Under the `{crate-name}` directory, in `Cargo.toml` add the following section:

    ```toml
    [[bench]]
    name = "benchmarks"
    harness = false
    ```

-   Under the `{crate-name}` directory, add a `benches/benchmarks.rs` file.
-   Under the `{crate-name}` directory, copy [sdk/core/perf.yml] and change _only_ the following contents:
    -   Change `ServiceDirectory` to match the `{service-directory}`.
-   Under the `{crate-name}` directory, copy [sdk/core/perf-tests.yml] and change _only_ the following contents:
    -   Change `Service` to match the `{service-directory}`.
    -   Change `Project` to match the `{crate-name}`.
    -   Change the `PackageVersions` to replace `azure_core` with the `{crate-name}`.

## Adding a new perf test

For each new perf test:

-   In the `{crate-name}/benches/benchmarks.rs` file:
    -   Add a new function named ${input:benchName}. It should take a single parameter `c: &mut Criterion` and return nothing.
    -   Generate a benchmark using the currently selected function in ${file} or specified client method name, if any.
    -   Use [sdk/keyvault/azure_security_keyvault_keys/benches/benchmarks.rs] as an example.
    -   If credentials are needed, import the `azure_core_test` crate and use `azure_core_test::credentials::from_env(None)` to get a `TokenCredential` for the client's `new` function.
    -   Separate client initialization from the client method being run in a benchmark.
    -   If `criterion_group` already exists, add the new perf test function to the end of the list of parameters or to the `target` parameter, if present.
-   In the `{crate-name}/perf-tests.yml` file:

    -   Add a top-level `Tests:` property if it does not already exist.
    -   Add the following YAML under the `Tests:` property as a separate YAML array object:

    ```yaml
    - Test: ${input:benchName}
        Class: ${input:benchName}
        Arguments:
        - --sync
    ```

-   Client constructors should already return a `Result<Arc<Self>>` so call `expect("expected new client")` on the return.
-   Always pass `None` for the options parameter on a client constructor.
