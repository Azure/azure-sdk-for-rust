# Contributing

## Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install)

  When you run `cargo build`, toolchain version [1.85](https://releases.rs/docs/1.85.0/) and necessary components will be installed automatically.

- (Recommended) If you use [Visual Studio Code], install recommended extensions to improve your development experience.

## Finding Issues to Work On

If you'd like to contribute to the Azure SDK for Rust, we recommend looking for issues with the following labels:

- [good first issue](https://github.com/Azure/azure-sdk-for-rust/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) - These issues are a good starting point for new contributors since they should be relatively straightforward to address.

- [help wanted](https://github.com/Azure/azure-sdk-for-rust/issues?q=is%3Aopen+is%3Aissue+label%3A%22help+wanted%22) - These issues are areas where we're actively seeking community contributions.

Further discussion on or pull requests for these issues is highly valued, and we encourage you to participate in the discussions or take on the tasks described in these issues.

## Using Copilot

This repository is [configured](https://code.visualstudio.com/docs/copilot/copilot-customization) to facilitate Copilot.
In addition to [general instructions](https://github.com/Azure/azure-sdk-for-rust/blob/main/AGENTS.md), you can find additional skills in [.github/skills] or use `/skill` in Copilot; and prompts in [.github/prompts] or type `#prompt` in Copilot.

To generate a new performance test, for example, you might prompt with:

```text
Using #perf-test.prompt.md generate a perf test for SecretClient::get_secret.
```

For comprehensive guidance on how AI agents should interact with this repository, including workflows, automation boundaries, and safety guidelines, see [AGENTS.md](https://github.com/Azure/azure-sdk-for-rust/blob/main/AGENTS.md).

## Generated code

If you want to contribute to a file that is generated (the file is located in a `generated` subdirectory), the best approach is to open a PR on the TypeSpec specification since we cannot replace generated code that will be replaced when regenerated.
Please visit the [Azure/azure-rest-api-specs repo](https://github.com/Azure/azure-rest-api-specs/) to view and make changes to Azure service API specifications.

Once changes are merged,

1. Change directories to the crate you want to regenerate:

   ```sh
   cd sdk/keyvault/azure_security_keyvault_secrets
   ```

2. Update `tsp-location.yaml` with the commit in the `Azure/azure-rest-api-specs` repository.
3. Run `tsp-client update`.

### Emitter updates

If you require a change in the emitter, after a new version has been published:

1. Update `eng/emitter-package.json` with the new version of `@azure-tools/typespec-rust`.
2. Update any dependencies to match versions specified in the emitter's [package.json](https://github.com/Azure/typespec-rust/blob/main/packages/typespec-rust/package.json) file.
3. Run `tsp-client update` in your crate directory or, to update all crates, run:

   ```bash
   # bash
   find sdk -name tsp-location.yaml -execdir tsp-client update \;
   ```

   ```powershell
   # powershell
   Get-ChildItem sdk -Filter tsp-location.yaml -Recurse | ForEach-Object -Begin { Push-Location } -Process { Set-Location $_.DirectoryName; tsp-client update } -End { Pop-Location }
   ```

## Coding

We welcome contributions! But before you start coding, please read our [Rust Guidelines] including [implementation details](https://azure.github.io/azure-sdk/rust_implementation.html) for contributors.

Most of our code is generated from [TypeSpec] and, more specifically for Azure services, [TypeSpec Azure] libraries. Changes to any services - including documentation improvements - need to made in TypeSpec specifications
found in <https://github.com/Azure/azure-rest-api-specs>. Only TypeSpec is supported at this time, so any changes to OpenAPI v2 a.k.a., "swagger", specifications will not be used.

Crates containing the majority of written code include `azure_core` and its dependencies. Some crates like `azure_security_keyvault_secrets` might also have some written code to improve usability.

## Building

To build any crate in the Azure SDK for Rust navigate to the crate's project folder and run `cargo build`.
Alternatively, you can build any one or more crates by passing their crate names to `--package` (short: `-p`) e.g., `cargo build -p azure_security_keyvault_secrets`.

You can also build the entire workspace by either building from the root source directory or running `cargo build --workspace`, but unless you're making changes to `azure_core`
or its dependencies, this is generally unnecessary nor recommended. It will take considerable time and drive space.

### Building on Windows

Developers on Windows must enable Developer Mode to support symlinks used in this repository. See [Developer Mode for Windows](https://learn.microsoft.com/windows/advanced-settings/developer-mode) for instructions on enabling Developer Mode.

After enabling Developer Mode, you must also enable symlinks for Git in this repository:

```pwsh
git config core.symlinks true
```

Alternatively, you can enable symlinks globally for all Git repositories:

```pwsh
git config --global core.symlinks true
```

By default we use the [`openssl`](https://crates.io/crates/openssl) crate and, indirectly, the [`openssl-sys`](https://crates.io/crates/openssl-sys) crate. On Windows, you may need to download and build openssl before you can successfully compile.
Since `openssl-sys` supports [vcpkg](https://learn.microsoft.com/vcpkg/), you can bootstrap OpenSSL:

1. Clone `vcpkg` somewhere in your development environment:

   ```pwsh
   git clone --depth=1 https://github.com/microsoft/vcpkg.git
   ```

2. Run the bootstrap script to download a prebuilt binary:

   ```pwsh
   cd vcpkg
   .\bootstrap-vcpkg.bat
   ```

3. Set up environment variables:

   ```pwsh
   $env:VCPKG_ROOT = "C:\path\to\vcpkg" # from step 1
   $env:PATH = "${env:VCPKG_ROOT};${env:PATH}"
   ```

   To persist these variables for future sessions, remember to set them in the Windows System Environment Variables panel.

4. Change directories into the `eng/` folder in this repo and run:

   ```pwsh
   cd eng
   vcpkg install

   $env:OPENSSL_DIR = "$PWD\vcpkg_installed\x64-windows"
   ```

### Linting

You can run `cargo clippy` to check for common issues. Like `cargo build`, you can pass one or more crate names to `--package`.
Before create a pull request (PR), it's a good practice to build and lint your project to avoid a lot of commits that can make the review process tedious for reviewers.

## Testing

To test any crate in the Azure SDK for Rust navigate to the crate's project folder and run `cargo test`.
Alternatively, you can test any one or more crates by passing their crate names to `--package` (short: `-p`) e.g., `cargo test -p azure_security_keyvault_secrets`.

This command will run all tests in the selected packages, including unit tests, integration tests, any tests within examples, and doc tests.
To learn more about the different styles of tests and where they are located in a project, see [Tests in The Cargo Book](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#tests).

### Integration Tests

We use integration tests - tests defined under a crate's `tests/` directory - for testing against provisioned resources.
Most crates use recorded tests to record (and sanitize) or play back HTTP traffic during test execution by attributing tests with `#[recorded::test]`.

If your crate does not communicate over HTTP or provisioning resources cannot be fully automated, you can also mark tests as `#[recorded::test(live)]`.

For more details about recorded tests, please refer to the [`azure_core_test` crate](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core_test/README.md).

### Documentation Tests

Though you should use integration tests for testing client methods, or create examples under `examples/` that show customers how to do common tasks,
[documentation tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html) can be helpful to show customers examples of calling your code
e.g., utility code to parse some resource identifier, that are also runnable (or just compilable) and will fail `cargo test` if the test assertions fail.

Each crate should define document tests to highlight common scenarios. Unlike markdown comments in Rust `.rs` source code, however, you need to define the code fence
using the `rust no_run` declaration e.g.,

````markdown
```rust no_run
#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}
```
````

This is because when GitHub renders the `README.md` file, it needs to know the syntax. The `no_run` tells `cargo test` to only compile the test - to make sure it indeed compiles - but not to run it,
which is important if it shows an example running against live resources that likely include placeholder values e.g., `https://my-vault.vault.azure.net`.

More guidance for writing documentation tests can be found in our [Rust Guidelines].

### Debugging with Visual Studio Code

[Visual Studio Code] with recommended extensions installed can be used to run and debug tests for a module or individual tests.
In most cases you can click **Run Test** or **Debug** found right above the test function.
For integration tests - those tests found under the `tests/` directory - this will run or debug the test using a previously recorded session, if one exists.

If a recording for an integration test does not exist or needs to be updated, along with the LLDB extension installed
you can create or update a `.vscode/launch.json` file to debug recording a session e.g.:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Record secret_roundtrip",
      "cargo": {
        "args": [
          "test",
          "--no-run",
          "--test=secret_client",
          "--package=azure_security_keyvault_secrets",
          "secret_roundtrip"
        ],
        "filter": {
          "name": "secret_client",
          "kind": "test"
        },
        "env": {
          "AZURE_TEST_MODE": "record"
        }
      },
      "cwd": "${workspaceFolder}",
      "env": {
        "AZURE_KEYVAULT_URL": "https://my-vault.vault.azure.net/",
        "RUST_LOG": "trace"
      }
    }
  ]
}
```

If you don't need to debug recording the test or running live, you can record from the command line e.g.:

```sh
AZURE_KEYVAULT_URL=https://my-vault.vault.azure.net AZURE_TEST_MODE=record cargo test -p azure_security_keyvault_secrets --test secret_client
```

In either case above, the [Test Proxy] will be started automatically if installed.

#### Running Test Proxy Manually

You can also start the [Test Proxy] manually, which may provide additional diagnostics.

If you're using a `.vscode/launch.json` file, in the outer `env` above add `"PROXY_MANUAL_START": "true"`.

Similarly, if running on the command line pass `PROXY_MANUAL_START=true`.

#### Enabling Trace Logging

To log tracing information to the terminal, you can add the `RUST_LOG` environment variable as shown above using the [same format supported by `env_logger`](https://docs.rs/env_logger/latest/env_logger/#enabling-logging).
The targets are the crate names if you want to trace more or less for specific targets e.g., `RUST_LOG=info,azure_core=trace` to trace information messages by default but detailed traces for the `azure_core` crate.

To log traces from the [Test Proxy] itself, pass `test-proxy=trace` to `RUST_LOG` e.g.,

```sh
RUST_LOG=info,azure_core=debug,test-proxy=trace cargo test -p azure_security_keyvault_secrets --test secret_client
```

#### Debugging in Windows

Using the recommended [CodeLLDB] Visual Studio Code extension on Windows will stop at breakpoints but may not pretty print variables e.g.,
for a `String` or `str` slice you may only see the length of UTF-8 bytes. See <https://github.com/vadimcn/codelldb/wiki/Windows> for suggestions.

Alternatively, you can install the [C/C++] extension and update configuration as described below.
`String`, `str` slices, enums and more should pretty print as expected, but some types e.g., `PathBuf` may not; however,
you can click the memory button next to a variable to see the contents in memory.

##### Debugging from within the Editor

To support debugging tests from within the editor, you can change your Visual Studio Code user settings as shown below:

```json
{
  "rust-analyzer.debug.engine": "ms-vscode.cpptools",
}
```

##### Run and Debug

For debugging examples from the **Run and Debug** view in Visual Studio Code, add a target to your `.vscode/launch.json` file like so:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "cppvsdbg",
      "request": "launch",
      "name": "Launch test_proxy",
      "program": "${workspaceRoot}/target/debug/examples/test_proxy.exe",
      "args": [
        "--auto"
      ],
      "cwd": "${workspaceFolder}",
      "environment": [
        {
          "name": "RUST_LOG",
          "value": "debug,test-proxy=trace"
        }
      ],
      "preLaunchTask": "Build test_proxy"
    }
  ]
}
```

And update `.vscode/tasks.json` to add the `preLaunchTask` if desired:

```json
{
  "tasks": [
    {
      "label": "Build test_proxy",
      "command": "cargo",
      "args": [
        "build",
        "--package",
        "azure_core_test",
        "--example",
        "test_proxy"
      ],
      "problemMatcher": [
        "$rustc"
      ]
    }
  ]
}
```

## Code Review Process

Before a pull request will be considered by the Azure SDK team, the following requirements must be met:

- Prior to issuing the pull request:
  - All code must have completed any necessary legal sign-off for being publicly viewable (Patent review, JSR review, etc.)
  - The changes cannot break any existing functional/unit tests that are part of the central repository.
    - This includes all tests, even those not associated with the given feature area
  - Code submitted must have basic unit test coverage, and have all unit tests pass. Testing is the full responsibility of person submitting the code review.
    - Functional tests are encouraged, and provide teams with a way to mitigate regressions cause by other code contributions.
  - Code should be commented.
  - Code should be fully code reviewed.
  - Code should be able to merge without any conflicts into the dev branch being targeted.
  - Code should pass all relevant static checks and coding guidelines set forth by the specific repository.
  - All build warnings and code analysis warnings should be fixed prior to submission.
- As part of the pull request (aka, in the text box on GitHub as part of submitting the pull request):
  - Proof of completion of the code review and test passes requirements above.
  - Identity of QA responsible for feature testing (can be conducted post-merging of the pull request).
  - Short description of the payload of pull request.
- After the pull request is submitted:
  - When your PR is submitted someone on our team will be auto assigned the PR for review. No need to email us.

Once all of the above steps are met, the following process will be followed:

- A member of the Azure SDK team will review the pull request on GitHub.
- If the pull request meets the repository's requirements, the individual will approve the pull request, merging the code into the appropriate branch of the source repository.
- If the request does not meet any of the requirements, the pull request will not be merged, and the necessary fixes for acceptance will be communicated back to the person who opened the PR.

### Pull Request Etiquette and Best Practices

#### Reviewers

- If you disagree with the overall approach of the PR, comment on the general PR rather than individual lines of code.
- Leaving [suggested changes](https://docs.github.com/pull-requests/collaborating-with-pull-requests/reviewing-changes-in-pull-requests/commenting-on-a-pull-request#adding-line-comments-to-a-pull-request) is welcomed, but please never commit them for a PR you did not create.
- When you are seeking to understand something rather than request corrections, it is suggested to use language such as "I'm curious ..." as a prefix to comments.
- For comments that are just optional suggestions or are explicitly non-blocking, prefix them with "nit: " or "non-blocking: ".
- Avoid marking a PR as "Request Changes" ![Request Change Icon](https://user-images.githubusercontent.com/1279263/151379844-b9babb22-b0fe-4b9c-b749-eb7488a38d84.png) unless you have serious concerns that should block the PR from merging.
- When to mark a PR as "Approved"
  - You feel confident that the code meets a high quality bar, has adequate test coverage, is ready to merge.
  - You have left comments that are uncontroversial and there is a shared understanding with the author that the comments can be addressed or resolved prior to being merged without significant discussion or significant change to the design or approach.
- When to leave comments without approval
  - You do not feel confident that your review alone is sufficient to call the PR ready to merge.
  - You have feedback that may require detailed discussion or may indicate a need to change the current design or approach in a non-trivial way.
- When to mark a PR as "Request Changes"
  - You have significant concerns that must be addressed before this PR should be merged such as unintentional breaking changes, security issues, or potential data loss.

#### Pull Request Authors

- If you add significant changes to a PR after it has been marked approved, please confirm with reviewers that they still approve before merging.
- Please ensure that you have obtained an approval from at least one of the code owners before merging.
- If a reviewer marks your PR as approved along with specific comments, it is expected that those comments will be addressed or resolved prior to merging.
  - One exception is when a comment clearly states that the feedback is optional or just a nit
  - When in doubt, reach out to the commenter to confirm that they have no concerns with you merging without addressing a comment.

### Performance Testing

Performance testing is supported via [criterion](https://bheisler.github.io/criterion.rs/book/criterion_rs.html)
There are samples of performance tests under `sdk/core/azure_core/benches` folder.
To execute the performance tests in `azure_core` folder you can run `cargo bench` in the `sdk/core/azure_core` folder.
The output of the tests will be presented in the command line as well as saved under the `target/criterion` folder.

## Releases

### Versions

To provide a helpful versioning experience, the Azure SDK for Rust libraries follow conventions similar to other Azure SDKs.

Release builds will fail if a library depends on another Azure SDK for Rust library which has not been released and is not included in the current release build.

#### Workspace dependencies

The root `Cargo.toml` file represents released versions of crates which can be used by other Azure SDK for Rust libraries. To use a released version of a library, use `workspace = true` in your library's `Cargo.toml`.

```toml
azure_core = { workspace = true }
```

If an SDK library depends on an unreleased SDK library, specify that dependency using a path-based dependency (`version` is required for the library to release):

```toml
azure_core = { path = "../../core/azure_core", version = "0.31.0" }
```

#### Version increment on release

When a release to crates.io completes, the Engineering System opens a pull request to increment versions of released packages. This PR should be merged as soon as possible to put the `main` branch in a "releasable" state and to provide context that code at the HEAD of `main` may not reflect code in a previously released version of a library.

The incremented version will be a "beta" of an incrementally higher release. This incremented version in the PR is a placeholder. The version can be updated in another PR to reflect the intended release version.

## Samples

### Third-party dependencies

Third party libraries should only be included in samples when necessary to demonstrate usage of an Azure SDK package; they should not be suggested or endorsed as alternatives to the Azure SDK.

When code samples take dependencies, readers should be able to use the material without significant license burden or research on terms. This goal requires restricting dependencies to certain types of open source or commercial licenses.

Samples may take the following categories of dependencies:

- **Open-source** : Open source offerings that use an [Open Source Initiative (OSI) approved license](https://opensource.org/licenses). Any component whose license isn't OSI-approved is considered a commercial offering. Prefer OSS projects that are members of any of the [OSS foundations that Microsoft is part of](https://opensource.microsoft.com/ecosystem/). Prefer permissive licenses for libraries, like [MIT](https://opensource.org/licenses/MIT) and [Apache 2](https://opensource.org/licenses/Apache-2.0). Copy-left licenses like [GPL](https://opensource.org/licenses/gpl-license) are acceptable for tools, and OSs. [Kubernetes](https://github.com/kubernetes/kubernetes), [Linux](https://github.com/torvalds/linux), and [Newtonsoft.Json](https://github.com/JamesNK/Newtonsoft.Json) are examples of this license type. Links to open source components should be to where the source is hosted, including any applicable license, such as a GitHub repository (or similar).

- **Commercial**: Commercial offerings that enable readers to learn from our content without unnecessary extra costs. Typically, the offering has some form of a community edition, or a free trial sufficient for its use in content. A commercial license may be a form of dual-license, or tiered license. Links to commercial components should be to the commercial site for the software, even if the source software is hosted publicly on GitHub (or similar).

- **Dual licensed**: Commercial offerings that enable readers to choose either license based on their needs. For example, if the offering has an OSS and commercial license, readers can  choose between them. [MySql](https://github.com/mysql/mysql-server) is an example of this license type.

- **Tiered licensed**: Offerings that enable readers to use the license tier that corresponds to their characteristics. For example, tiers may be available for students, hobbyists, or companies with defined revenue  thresholds. For offerings with tiered licenses, strive to limit our use in tutorials to the features available in the lowest tier. This policy enables the widest audience for the article. [Docker](https://www.docker.com/), [IdentityServer](https://duendesoftware.com/products/identityserver), [ImageSharp](https://sixlabors.com/products/imagesharp/), and [Visual Studio](https://visualstudio.com) are examples of this license type.

In general, we prefer taking dependencies on licensed components in the order of the listed categories. In cases where the category may not be well known, we'll document the category so that readers understand the choice that they're making by using that dependency.

## Troubleshooting

### Proc-macro map is missing error entry

If Visual Studio Code with the rust-analyzer extension active is showing an error for all code in the editor with an error similar to:

> tracing::function: internal error: proc-macro map is missing error entry for crate Crate(Id(6560))

This could be a discrepancy between your active Rust toolchain and the internal `rust-analyzer` the rust-analyzer extension distributes and uses by default.
Because we do not currently mandate a specific version of the Rust toolchain in our `rust-toolchain.toml` (as long as it supports the MSRV in `Cargo.toml`),
the version of `rust-analyzer` you have installed should match the active toolchain.

In your Visual Studio Code settings, add:

```json
{
    "rust-analyzer.server.path": "${userHome}/.cargo/bin/rust-analyzer"
}
```

Set this to an appropriate path for local development. If you're running Visual Studio Code with a remote session e.g., WSL or SSH,
you'll need to [open remote settings](https://code.visualstudio.com/docs/remote/troubleshooting#_local-absolute-path-settings-fail-when-applied-remotely) and add the configuration above.

1. Press `F1`.
2. Type "open remote settings (json)" and press `Enter`.
3. Add the above setting to your remote settings. If you use multiple remote sessions you'll need to do this for each one.

[C/C++]: https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools
[CodeLLDB]: https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb
[Rust Guidelines]: https://azure.github.io/azure-sdk/rust_introduction.html
[Test Proxy]: https://github.com/Azure/azure-sdk-tools/blob/main/tools/test-proxy/Azure.Sdk.Tools.TestProxy/README.md
[TypeSpec]: https://aka.ms/typespec
[TypeSpec Azure]: https://aka.ms/typespec/azure
[Visual Studio Code]: https://code.visualstudio.com
