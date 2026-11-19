# Contributing to Azure Identity for Rust

Start with the [repository contribution guide](https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md).

## Prerequisites

Offline checks require the repository's Rust toolchain. Testing the `client_certificate` feature also requires OpenSSL development libraries.

Live integration tests additionally require:

- An Azure subscription and permission to create resources and role assignments.
- Azure CLI, authenticated with `az login`, and Azure PowerShell, authenticated with `Connect-AzAccount`.
- PowerShell, Bicep, Docker, `kubectl`, and `ssh-keygen` for the identity provisioning scripts.
- For identity bindings, permission to create federated identity credentials and update the AKS cluster, plus Kubernetes cluster administrator permissions.
  See the [identity bindings setup guide](https://learn.microsoft.com/azure/aks/identity-bindings) for current requirements and limitations.

## Build and offline checks

Run these commands from the repository root:

```sh
cargo build -p azure_identity --all-features
cargo test -p azure_identity --all-features
cargo test -p azure_identity --no-default-features
cargo fmt -p azure_identity --check
cargo clippy -p azure_identity --all-features --all-targets -- -D warnings
```

With `AZURE_TEST_MODE` unset or set to `playback`, non-ignored tests do not require Azure resources, CLI sign-in, a test proxy, or network access.
Credential HTTP requests use mock transports, and CLI credentials use mock process executors.
Token proxy tests verify request rewriting, custom CA configuration, SNI, address overrides, and CA rotation without connecting to a token service.
CA plumbing and cache tests use synthetic non-certificate bytes and injected mock factories; they do not validate certificate trust or TLS handshakes.
These custom `TokenProxyClient` tests also run with no default features and with only `reqwest` enabled; they do not require `reqwest_rustls`.
Malformed-input tests construct real HTTP clients but do not need a valid CA or send requests.
Real CA/TLS tests are explicitly ignored. Do not generate or check in test CAs.
Host names such as `proxy.contoso.com` in these tests are fixtures, not provisioned endpoints.

The README and migration guide test targets compile examples but intentionally ignore executing them.
Do not use an unfiltered `--include-ignored` or `--ignored` command: those examples are not live test scenarios.

## Integration tests

The ordinary ACI and AKS workload identity tests use `#[recorded::test(live)]` because the identity provisioning scripts create their required environments.
The macro ignores them unless `AZURE_TEST_MODE=live` is set when compiling the tests.
Do not force them to run with `--ignored` in playback or record mode.
When switching modes, run `cargo clean -p azure_identity` before compiling again so the macro uses the new mode.
Missing required environment variables fail the test rather than silently skipping it.
Tests requiring real CA/TLS configuration are explicitly ignored with reasons linking here, even when `AZURE_TEST_MODE=live`.
The IMDS live test runs only in live mode and returns without contacting IMDS when `IDENTITY_IMDS_AVAILABLE` is absent.

| Test | Selection | Required environment |
| --- | --- | --- |
| `managed_identity_credential::tests::aci_user_assigned_live` | Live mode | Provisioned ACI test app, user-assigned identity, and storage account |
| `workload_identity_credential::tests::live` | Live mode | Provisioned AKS test app with ordinary workload identity and storage access |
| `managed_identity_credential::tests::imds_system_assigned_live` | Live mode, with `IDENTITY_IMDS_AVAILABLE` set | Azure host with a system-assigned managed identity and reachable IMDS |
| `custom_token_proxy::tests::accepts_inline_and_file_ca` | Explicit `--ignored --exact` | Real CA configuration supplied by an AKS identity binding environment |
| `custom_token_proxy::tests::sends_requests_with_custom_ca_and_sni` | Explicit `--ignored --exact` | AKS identity binding pod with a token proxy, custom CA, and SNI |

### Provisioning live resources

From the repository root, use the shared provisioning entry point, not the pre- or post-provisioning scripts directly:

```powershell
./eng/common/TestResources/New-TestResources.ps1 `
    -ServiceDirectory identity `
    -SubscriptionId "<subscription-id>" `
    -ResourceGroupName "<resource-group>" `
    -AdditionalParameters @{ deployResources = $true }
```

The [Bicep template](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/identity/test-resources.bicep) defaults `deployResources` to `false`.
Enabling it creates two storage accounts, one user-assigned identity, one container registry, and one AKS cluster with a single node.
The [post-provisioning script](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/identity/test-resources-post.ps1) builds and pushes the test app, creates an ACI instance, and deploys an AKS pod and public load balancer.
It uses the template's existing `IDENTITY_ACR_NAME` output to detect the effective deployment setting, whether passed through `AdditionalParameters` or `ArmTemplateParameters`.
Disabled deployments skip post-provisioning; enabled deployments require complete resource outputs.
Before publishing endpoints, the script waits for addresses, pod readiness, and successful authentication and storage access through each deployed app.
Pod and IP address readiness use `ReadinessTimeoutSeconds` (180 seconds by default).
Authentication and storage access use a separate `EndpointReadinessTimeoutSeconds` budget (900 seconds by default) to allow for identity and role propagation.
Within that endpoint budget, the script checks TCP connectivity from the agent and unauthenticated HTTP reachability before testing authentication and storage access.
A response such as HTTP 404 proves HTTP reachability, not successful authentication; the `/api` check must still return HTTP 200.
TCP probes wait at most 5 seconds and HTTP reachability probes wait at most 10 seconds, both capped by the remaining endpoint budget.
Each HTTP request waits at most 60 seconds or the remaining endpoint budget, whichever is shorter.
Timeout errors retain the last HTTP error status and response body, when available.
If AKS endpoint validation fails, the script collects pod logs, pod and service status, service endpoints, and recent pod and service events before cleaning up its temporary Kubernetes configuration.
These resources incur charges until removed.

The identity weekly pipeline allows 120 minutes per test job, including provisioning, application setup, tests, and cleanup.
This job ceiling is separate from the pod/IP and endpoint readiness limits; non-weekly pipelines retain their existing timeout.
On failure or cancellation, a two-minute diagnostic step attempts to collect deployment state, failed deployment operations, recent failed Azure Activity Log entries, and AKS worker provisioning state before cleanup.
The [diagnostic script](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/identity/eng/scripts/Get-DeploymentDiagnostics.ps1) selects resources by the current build, job, and service tags.
Cancellation diagnostics are best effort and cannot run after the agent terminates or its cancellation grace expires.
Azure Activity Log can still be inspected independently of the agent.

Use the deployment outputs and post-provisioning output to set the environment variables in the shell running the tests.
Endpoint variables are set in the provisioning PowerShell process and published as pipeline variables in CI.
For another shell, including a parent shell that launched `pwsh`, use the printed PowerShell assignments or `sh` export commands.
The script restores the caller's directory and removes its temporary manifest, Dockerfile, and Kubernetes configuration.

For ACI, set `IDENTITY_ACI_IP_USER_ASSIGNED`, `IDENTITY_STORAGE_NAME_USER_ASSIGNED`, and `IDENTITY_USER_ASSIGNED_IDENTITY_CLIENT_ID`:

```sh
AZURE_TEST_MODE=live cargo test -p azure_identity --all-features --lib \
  managed_identity_credential::tests::aci_user_assigned_live -- --exact
```

For ordinary AKS workload identity, set `IDENTITY_AKS_IP` and `IDENTITY_STORAGE_NAME_USER_ASSIGNED`:

```sh
AZURE_TEST_MODE=live cargo test -p azure_identity --all-features --lib \
  workload_identity_credential::tests::live -- --exact
```

For IMDS, run on an Azure host with a system-assigned managed identity.
Set `IDENTITY_IMDS_AVAILABLE=1` only on that host.
Ensure other managed identity environment variables do not select a different credential source.
The existing template does not provision this host, and setting `IDENTITY_IMDS_AVAILABLE` does not itself configure a managed identity.
The weekly `msi_image` matrix supplies this opt-in for its identity-enabled host.
Locally, run the test only on a host with a configured system-assigned identity:

```sh
AZURE_TEST_MODE=live IDENTITY_IMDS_AVAILABLE=1 \
  cargo test -p azure_identity --all-features --lib \
  managed_identity_credential::tests::imds_system_assigned_live -- --exact
```

### AKS identity binding token proxy

The existing AKS resource enables OIDC and workload identity, but the post-provisioning script configures ordinary federated workload identity.
That configuration alone does not create an identity binding or inject token proxy settings.
No additional Azure resources are needed for the mocked tests.
The real CA and token proxy tests are explicitly ignored because their prerequisites are not part of the standard deployment and their TLS behavior cannot be validated by HTTP playback.

To run the end-to-end token proxy test:

1. Reuse the provisioned AKS cluster and user-assigned identity.
   Follow the [identity bindings setup guide](https://learn.microsoft.com/azure/aks/identity-bindings) to create the binding, for example:

   ```sh
   az aks identity-binding create --resource-group "<resource-group>" \
     --cluster-name "<cluster-name>" --name "<binding-name>" \
     --managed-identity-resource-id "<identity-resource-id>"
   ```

2. Apply the guide's `ClusterRole` and `ClusterRoleBinding` to authorize the test pod's service account to use that identity.
3. Create a test pod with the `azure.workload.identity/use: "true"` label and the `azure.workload.identity/use-identity-binding: "true"` annotation.
   Use a development image containing this checkout, Rust, and the build dependencies.
   The existing deployed test app exposes ordinary workload identity, not this token proxy test.
4. Verify that the workload identity webhook injected `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_FEDERATED_TOKEN_FILE`, `AZURE_KUBERNETES_TOKEN_PROXY`, `AZURE_KUBERNETES_SNI_NAME`, and `AZURE_KUBERNETES_CA_FILE`.
   The token file and CA file must be readable inside the pod.
   Alternatively, set `AZURE_KUBERNETES_CA_DATA` to PEM data, but do not set both CA variables.
5. From the checkout inside that pod, run:

   ```sh
   AZURE_TEST_MODE=live cargo test -p azure_identity --lib \
     custom_token_proxy::tests::sends_requests_with_custom_ca_and_sni -- --ignored --exact
   ```

The test requests real tokens using both inline and file-based CA configuration.
It requires the default `reqwest_rustls` feature, but not `client_certificate`.
To validate loading the provided CA without sending token requests, use the same pod environment and select:

```sh
cargo test -p azure_identity --lib \
  custom_token_proxy::tests::accepts_inline_and_file_ca -- --ignored --exact
```

This parsing check does not validate trust against a server; that requires the end-to-end TLS test.
Both tests read real CA material from `AZURE_KUBERNETES_CA_FILE` or `AZURE_KUBERNETES_CA_DATA`.
Do not generate test CAs or copy CAs, projected tokens, or credentials into source files, images, or recordings.

### GitHub Actions credential

Provision the identity resources with GitHub OIDC trust for the repository and ref that will run the workflow:

```powershell
./eng/common/TestResources/New-TestResources.ps1 `
    -ServiceDirectory identity `
    -SubscriptionId "<subscription-id>" `
    -ResourceGroupName "<resource-group>" `
    -AdditionalParameters @{
        deployResources = $true
        githubRepository = 'Azure/azure-sdk-for-rust'
        githubRef = 'refs/heads/main'
    }
```

Use the deployment's `AZURE_TENANT_ID` and `AZURE_CLIENT_ID` outputs to dispatch the [credential test workflow](https://github.com/Azure/azure-sdk-for-rust/blob/main/.github/workflows/github-actions-credential-test.yml):

```sh
gh workflow run github-actions-credential-test.yml --repo Azure/azure-sdk-for-rust \
  --ref main -f tenant-id="<AZURE_TENANT_ID>" -f client-id="<AZURE_CLIENT_ID>" \
  -f scope="https://management.azure.com/.default"
```

For a fork or another branch, update both the provisioning parameters and the workflow repository/ref.
The workflow sets `AZURE_TEST_MODE=live` and `TEST_SCOPE`, then runs `cargo test --manifest-path sdk/identity/azure_identity/Cargo.toml --test github_actions_credential --all-features`.
The `#[recorded::test(live)]` test requests a token using `GitHubActionsCredential`; it needs GitHub's runner-provided OIDC environment variables and cannot authenticate from a local shell.
No recordings are created.

### Recording and playback

There are currently no recorded identity tests or identity recording assets to publish.
Ordinary OAuth HTTP exchanges can be recorded by instrumenting the credential's `ClientOptions` and sanitizing assertions, secrets, and returned tokens.
However, `Recording::credential()` deliberately substitutes a mock credential in playback; it does not replay the identity credential's authentication exchange.
CLI subprocesses and tokens obtained inside a deployed test app are also outside the caller's recording pipeline.

Recording the token proxy exchange would not validate its actual TLS handshake, custom trust roots, SNI, or AKS identity binding authorization.
The token proxy transport rewrites request destinations itself, so attaching recording policies alone is not sufficient to make that end-to-end test replayable.
Use mocks for deterministic transport configuration and request tests, and the explicitly ignored test for real TLS and AKS validation.
`AZURE_TEST_MODE=record` does not enable live-only tests or explicitly ignored tests.
No recordings are needed for these mocked and live-only scenarios.

If adding replayable tests later, follow the shared [recorded testing and asset publication workflow](https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md#integration-tests).
Add recording metadata, sanitize all sensitive values, verify playback without Azure access, and publish recordings before updating the asset pointer in a PR.

### Cleanup

Remove the dedicated test resource group after testing:

```powershell
./eng/common/TestResources/Remove-TestResources.ps1 `
    -ServiceDirectory identity `
    -ResourceGroupName "<resource-group>"
```

If reusing resources outside that group, separately remove the test pod, service, service account, Kubernetes role bindings, and identity binding.
Do not delete an identity's automatically created federated credential while other identity bindings still use it.

## Troubleshooting

- **Live test is ignored:** Set `AZURE_TEST_MODE=live` before compiling the tests; clean the package first if switching modes. Set the variables listed for that scenario before running it.
- **CA/TLS test stays ignored in live mode:** These tests require separate manual setup and explicit `--ignored --exact` selection as described above.
- **IMDS live test returns without authenticating:** Set `IDENTITY_IMDS_AVAILABLE=1` only on an Azure host with a configured system-assigned managed identity.
- **Provisioning readiness times out:** Check the reported TCP, HTTP reachability, or authentication/storage phase, last HTTP error, AKS diagnostics, and identity role assignments. Endpoint validation allows 900 seconds by default; pod and IP readiness allow 180 seconds. The script does not publish an endpoint before its authentication/storage check succeeds.
- **Weekly job times out during Azure deployment:** Check the deployment diagnostic step or Azure Activity Log for provider error codes and correlation IDs. Increasing endpoint readiness limits does not change the weekly job's 120-minute ceiling.
- **Token proxy settings are missing:** Ordinary workload identity is insufficient; recreate the pod with the identity binding annotation after configuring the binding and Kubernetes authorization.
- **CA or SNI failures:** Use the webhook-provided values inside the pod, and configure exactly one CA source. HTTP recordings cannot diagnose the live TLS handshake.
- **Storage authorization failures:** Check the test identity's role assignments and allow time for role propagation.
- **Offline test tries to connect:** Inject a mock HTTP transport or process executor. Do not rely on an unresolvable fixture hostname or a failed connection as the mock.
