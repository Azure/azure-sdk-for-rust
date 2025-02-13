# Azure client library test utilities

The types and functions in this crate help test client libraries built on `azure_core`.

## Client methods

To test client methods using our [Test Proxy] or run against live resources, you can attribute asynchronous tests
using the `#[recorded::test]` attribute:

```rust no_run
use azure_core::Result;
use azure_core_test::{recorded, TestContext};
use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};

#[recorded::test]
async fn get_secret(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    // Instrument your client options to hook up the test-proxy pipeline policy.
    let mut options = SecretClientOptions::default();
    recording.instrument(&mut options.client_options);

    // Get variables, credentials, and pass the instrumented client options to the client to begin.
    let client = SecretClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    todo!()
}
```

The `TestContext` parameter is required unless your test function is attributed as `#[recorded::test(live)]` (live-only).
You can name the parameter whatever you want.
The `TestContext` parameter is used to initialize an HTTP client to play back or record tests
and provides other information to test functions that may be useful.

These tests must also return a `std::result::Result<T, E>`, which can be redefined e.g., `azure_core::Result<T>`.

### Recording features

Besides instrumenting your client and getting credentials - real credentials when running live or recording,
but mock credentials when playing back - there are a number of other helpful features on the `Recording` object returned above:

* `add_sanitizer` will add custom sanitizers. There are many pre-configured by the [Test Proxy] as well.
* `remove_sanitizers` will remove named sanitizers, like `AZSDK3430` that sanitizes all `$..id` fields and may cause playback to fail.
* `add_matcher` adds a custom matcher to match headers, path segments, and or body content.
* `random` gets random data (numbers, arrays, etc.) that is initializes from the OS when running live or recording,
  but the seed is saved with the recording and used during play back so that sequential generation of random data is deterministic.
  ChaCha20 is used to provide a deterministic, portable sequence of seeded random data.
* `var` gets a required variable with optional `ValueOptions` you can use to sanitize values.
  This function will err if the variable is not set in the environment when running live or recording, or available when playing back.
* `var_opt` gets optional variables and will not err in the aforementioned cases.

## Record tests

Like with all our other Azure SDK languages, we use a common system for provisioning resources named [Test Resources].
This uses a `test-resources.json` or `test-resources.bicep` file in the crate or service directory.

When you run this, it will output some environment variables you can set in your shell, or pass on the command line if your shell supports it.

```bash
pwsh ./eng/common/TestResources/New-TestResources.ps1 -ServiceDirectory keyvault
```

In this example, it will output a number of environment variables including `AZURE_KEYVAULT_URL`. We'll pass that in the command line for bash below.

Before you can record, though, you need to make sure you're authenticated. The script above will authenticate you in PowerShell, but the
`AzurePowerShellCredential` is not yet implemented; however, the `AzureCliCredential` is so log in with `az`:

```bash
az login
```

No you can record your tests and, after they pass successfully, check in the test recordings. You need to pass `AZURE_TEST_MODE=record` to record,
along with any other environment variables output by the [Test Resources] scripts.

```bash
AZURE_TEST_MODE=record AZURE_KEYVAULT_URL=https://my-vault.vault.azure.net cargo test -p azure_security_keyvault_secrets
test-proxy push -a sdk/keyvault/assets.json
```

Once your assets are checked in by [Test Proxy], you can commit your local changes and create a PR.

## Playing back tests

To play back tests, just run `cargo test`:

```bash
cargo test
```

If you get errors, they could indicate regressions in your tests or perhaps variables or random data wasn't saved correctly.
Review any data you generate or use not coming from the service.

[Test Proxy]: https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md
[Test Resources]: https://github.com/Azure/azure-sdk-tools/blob/main/eng/common/TestResources/README.md
