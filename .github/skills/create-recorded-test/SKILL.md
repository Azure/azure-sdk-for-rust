---
name: create-recorded-test
description: Generate a new recorded integration test
disable-model-invocation: true
agent: "agent"
---

# Generate a new recorded integration test

You will generate one or more recorded integration tests under the `sdk/{service-directory}/{crate-name}/tests` directory in `${input:testFile:test_file}.rs` where:

- `{repo-root}` is the root directory containing `Cargo.toml`.
- `{service-directory}` is the directory name under [sdk](../../sdk) for the current ${file} e.g., `keyvault`.
- `{crate-name}` is the crate directory name under `sdk/{service-directory}` for the current ${file} e.g., `azure_security_keyvault_secrets`.

## Set up

These instructions only need to be done once. If changes described are already present, do not make the changes again.

- Recorded tests must always be integration tests under the `sdk/{service-directory}/{crate-name}/tests` directory. Do not add recorded tests to `src/`, `examples/`, or `README.md`.
- If PowerShell is not installed, stop and ask the user to install it first using [PowerShell installation instructions](https://learn.microsoft.com/powershell/scripting/install/installing-powershell).
- Provision test resources using the full script path:

  ```bash
  {repo-root}/eng/common/TestResources/New-TestResources.ps1 -ServiceDirectory {service-directory}
  ```

- To run recorded tests, rely on the existing test infrastructure. Test Proxy is acquired automatically for test runs when needed, but not for `test-proxy push -a sdk/{service-directory}/assets.json`.
- For manual Test Proxy usage or asset publishing, see [CONTRIBUTING.md](../../CONTRIBUTING.md) and the [Test Proxy documentation](https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md).

## Adding a new recorded test

For each new recorded test:

- In `sdk/{service-directory}/{crate-name}/tests/${input:testFile}.rs`, add or update async integration tests attributed with `#[recorded::test]`.
- Use the signature `async fn ${input:testName}(ctx: TestContext) -> Result<()>`.
- Start each test with:
  - `let recording = ctx.recording();`
  - `let mut options = {Client}Options::default();`
  - `recording.instrument(&mut options.client_options);`
- Construct clients with `recording.var("{ENV_VAR}", None).as_str()`, `recording.credential()`, and `Some(options)`.
- Use [sdk/keyvault/azure_security_keyvault_secrets/tests/secret_client.rs](../../sdk/keyvault/azure_security_keyvault_secrets/tests/secret_client.rs) as the example for function signatures, `TestContext`, and required `ClientOptions` instrumentation.
- Generate one test per client method or scenario the user wants to cover. If multiple methods share setup and belong together, keep them in the same integration test file as separate `#[recorded::test]` functions.
- Prefer asserting on returned models or observable state instead of only checking success.
- Reuse existing crate patterns for imports, helper types, resource naming, and environment variables.

## Recording and playback

- Record or update sessions against live resources:

  ```bash
  AZURE_TEST_MODE=record cargo test -p {crate-name} --test ${input:testFile}
  ```

- Run playback to make sure tests were properly recorded:

  ```bash
  cargo test -p {crate-name} --test ${input:testFile}
  ```

- After recording and playback succeeds, publish updated recordings only if `test-proxy` is already installed and available:

  ```bash
  test-proxy push -a sdk/{service-directory}/assets.json
  ```

## Tear down

- Remove provisioned test resources using the full script path when they are no longer needed:

  ```bash
  {repo-root}/eng/common/TestResources/Remove-TestResources.ps1 -ServiceDirectory {service-directory}
  ```
