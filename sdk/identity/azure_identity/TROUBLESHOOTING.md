# Troubleshoot Azure Identity authentication issues

This troubleshooting guide covers failure investigation techniques, common errors for the credential types in the `azure_identity` crate, and mitigation steps to resolve these errors.

## Table of contents

- [Handle azure_identity errors](#handle-azure_identity-errors)
  - [Permission issues](#permission-issues)
- [Find relevant information in errors](#find-relevant-information-in-errors)
- [Troubleshoot AzureCliCredential authentication issues](#troubleshoot-azureclicredential-authentication-issues)
- [Troubleshoot AzureDeveloperCliCredential authentication issues](#troubleshoot-azuredeveloperclicredential-authentication-issues)
- [Troubleshoot AzurePipelinesCredential authentication issues](#troubleshoot-azurepipelinescredential-authentication-issues)
- [Troubleshoot ClientCertificateCredential authentication issues](#troubleshoot-clientcertificatecredential-authentication-issues)
- [Troubleshoot ClientSecretCredential authentication issues](#troubleshoot-clientsecretcredential-authentication-issues)
- [Troubleshoot ManagedIdentityCredential authentication issues](#troubleshoot-managedidentitycredential-authentication-issues)
  - [Azure App Service and Azure Functions managed identity](#azure-app-service-and-azure-functions-managed-identity)
  - [Azure Virtual Machine managed identity](#azure-virtual-machine-managed-identity)
- [Troubleshoot WorkloadIdentityCredential authentication issues](#troubleshoot-workloadidentitycredential-authentication-issues)
- [Get additional help](#get-additional-help)

## Handle azure_identity errors

Any service client method that makes a request to the service may return an error due to authentication failure. This is because the credential authenticates on the first call to the service and on any subsequent call that needs to refresh an access token. Authentication errors include a description of the failure and possibly an error message from Microsoft Entra ID. Depending on the application, these errors may or may not be recoverable.

### Permission issues

Service client errors with a status code of 401 or 403 often indicate that authentication succeeded but the caller doesn't have permission to access the specified API. Check the service documentation to determine which RBAC roles are needed for the request, and ensure the authenticated user or service principal has the appropriate role assignments.

## Find relevant information in errors

Authentication errors can include responses from Microsoft Entra ID and often contain information helpful in diagnosis. Consider the following error message:

```
Error: ClientSecretCredential authentication failed. AADSTS7000215: Invalid client secret provided. Ensure the secret being sent in the request is the client secret value, not the client secret ID, for a secret added to app '<client ID>'. Trace ID: d05377ee-e3ab-4d37-bab7-d46f530b7401 Correlation ID: 1ef06246-3841-470a-860f-ab1b78f325a5 Timestamp: 2025-06-03 16:13:57Z
```

This error contains several pieces of information:

- __Failing Credential Type__: The type of credential that failed to authenticate.

- __Microsoft Entra ID Error Code and Message__: This can give insight into the specific reason the request failed. For instance, in this case authentication failed because the provided client secret is incorrect. [Microsoft Entra ID documentation](https://learn.microsoft.com/entra/identity-platform/reference-error-codes#aadsts-error-codes) has more information on AADSTS error codes.

- __Correlation ID and Timestamp__: The correlation ID and timestamp identify the request in server-side logs. This information can be useful to support engineers diagnosing unexpected Microsoft Entra ID failures.

Many credential errors also carry the HTTP response that caused them. This can help in advanced debugging scenarios, for example when you want to check header values that aren't represented in the error message. The example below demonstrates how to access that response in such a case.

```rust
use azure_core::error::ErrorKind;

let result = client.method().await;
if let Err(err) = result {
    match err.kind() {
        // ErrorKind::Credential indicates an authentication problem
        ErrorKind::Credential => {
            // a credential error may wrap another error having an HTTP response
            if let Some(inner) = err.downcast_ref::<azure_core::Error>() {
                if let ErrorKind::HttpResponse {
                    raw_response: Some(response),
                    ..
                } = inner.kind()
                {
                    let headers = response.headers();
                    for (name, value) in headers.iter() {
                        println!("{}: {}", name.as_str(), value.as_str());
                    }
                }
            }
        }
        _ => {
            todo!("handle other kinds of errors")
        }
    }
}
```

<a id="client-secret"></a>
## Troubleshoot ClientSecretCredential authentication issues

| Error Code | Issue | Mitigation |
|---|---|---|
|AADSTS7000215|An invalid client secret was provided.|Ensure the secret provided to the credential constructor is valid. If unsure, create a new client secret using the Azure portal. Details on creating a new client secret are in [Microsoft Entra ID documentation](https://learn.microsoft.com/entra/identity-platform/howto-create-service-principal-portal#option-2-create-a-new-application-secret).|
|AADSTS7000222|An expired client secret was provided.|Create a new client secret using the Azure portal. Details on creating a new client secret are in [Microsoft Entra ID documentation](https://learn.microsoft.com/entra/identity-platform/howto-create-service-principal-portal#option-2-create-a-new-application-secret).|
|AADSTS700016|The specified application wasn't found in the specified tenant.|Ensure the client and tenant IDs provided to the credential constructor are correct for your application registration. For multi-tenant apps, ensure the application has been added to the desired tenant by a tenant admin. To add a new application in the desired tenant, follow the [Microsoft Entra ID instructions](https://learn.microsoft.com/entra/identity-platform/howto-create-service-principal-portal).|

<a id="client-cert"></a>
## Troubleshoot ClientCertificateCredential authentication issues

| Error Code | Description | Mitigation |
|---|---|---|
|AADSTS700027|Client assertion contains an invalid signature.|Ensure the specified certificate has been uploaded to the application registration as described in [Microsoft Entra ID documentation](https://learn.microsoft.com/entra/identity-platform/howto-create-service-principal-portal#option-1-upload-a-certificate).|
|AADSTS700016|The specified application wasn't found in the specified tenant.|Ensure the client and tenant IDs provided to the credential constructor are correct for your application registration. For multi-tenant apps, ensure the application has been added to the desired tenant by a tenant admin. To add a new application in the desired tenant, follow the [Microsoft Entra ID instructions](https://learn.microsoft.com/entra/identity-platform/howto-create-service-principal-portal).|

<a id="managed-id"></a>
## Troubleshoot ManagedIdentityCredential authentication issues

`ManagedIdentityCredential` is designed to work on a variety of Azure hosts support managed identity. Configuration and troubleshooting vary from host to host. The below table lists the Azure hosts that can be assigned a managed identity and are supported by `ManagedIdentityCredential`.

|Host Environment| | |
|---|---|---|
|Azure Virtual Machines and Scale Sets|[Configuration](https://learn.microsoft.com/entra/identity/managed-identities-azure-resources/qs-configure-portal-windows-vm)|[Troubleshooting](#azure-virtual-machine-managed-identity)|
|Azure App Service and Azure Functions|[Configuration](https://learn.microsoft.com/azure/app-service/overview-managed-identity)|[Troubleshooting](#azure-app-service-and-azure-functions-managed-identity)|

### Azure Virtual Machine managed identity

| Error Message |Description| Mitigation |
|---|---|---|
|The requested identity hasn’t been assigned to this resource.|The IMDS endpoint responded with a status code of 400, indicating the requested identity isn’t assigned to the VM.|If using a user assigned identity, ensure the specified ID is correct.<p/><p/>If using a system assigned identity, make sure it has been enabled as described in [managed identity documentation](https://learn.microsoft.com/entra/identity/managed-identities-azure-resources/qs-configure-portal-windows-vm#enable-system-assigned-managed-identity-on-an-existing-vm).|
|The request failed due to a gateway error.|The request to the IMDS endpoint failed due to a gateway error, 502 or 504 status code.|IMDS doesn't support requests via proxy or gateway. Disable proxies or gateways running on the VM for requests to the IMDS endpoint `http://169.254.169.254`|
|Connection timed out|No response was received for the request to IMDS or the request timed out.|<ul><li>Ensure the VM is configured for managed identity as described in [managed identity documentation](https://learn.microsoft.com/entra/identity/managed-identities-azure-resources/qs-configure-portal-windows-vm).</li><li>Verify the IMDS endpoint is reachable on the VM. See [below](#verify-imds-is-available-on-the-vm) for instructions.</li></ul>|

#### Verify IMDS is available on the VM

If you have access to the VM, you can use `curl` to verify the managed identity endpoint is available.

```sh
curl 'http://169.254.169.254/metadata/identity/oauth2/token?resource=https://management.core.windows.net&api-version=2018-02-01' -H "Metadata: true"
```

> If successful, this command's output will contain an access token that SHOULD NOT BE SHARED, to avoid compromising account security.

### Azure App Service and Azure Functions managed identity

| Error Message |Description| Mitigation |
|---|---|---|
|Connection timed out with url: "http://169.254.169.254/..."|The App Service host hasn't set environment variables for managed identity configuration.|<ul><li>Ensure the App Service is configured for managed identity as described in [App Service documentation](https://learn.microsoft.com/azure/app-service/overview-managed-identity).</li><li>Verify the App Service environment is properly configured and the managed identity endpoint is available. See [below](#verify-the-app-service-managed-identity-endpoint-is-available) for instructions.</li></ul>|

#### Verify the App Service managed identity endpoint is available

If you can SSH into the App Service, you can verify managed identity is available in the environment. First ensure the environment variables `IDENTITY_ENDPOINT` and `IDENTITY_SECRET` are set. Then you can verify the managed identity endpoint is available using `curl`.

```sh
curl "$IDENTITY_ENDPOINT?resource=https://management.core.windows.net&api-version=2019-08-01" -H "X-IDENTITY-HEADER: $IDENTITY_HEADER"
```

> If successful, this command's output will contain an access token that SHOULD NOT BE SHARED, to avoid compromising account security.

<a id="azure-cli"></a>
## Troubleshoot AzureCLICredential authentication issues

|Error Message|Description|Mitigation|
|---|---|---|
|az not found on PATH|The Azure CLI isn’t installed or isn't on the application's path.|<ul><li>Ensure the Azure CLI is installed as described in [Azure CLI documentation](https://learn.microsoft.com/cli/azure/install-azure-cli).</li><li>Validate the installation location is in the application's `PATH` environment variable.</li></ul>|
|ERROR: Please run 'az login' to setup account|No account is currently logged into the Azure CLI, or the login has expired.|<ul><li>Run `az login` to log into the Azure CLI. More information about Azure CLI authentication is available in the [Azure CLI documentation](https://learn.microsoft.com/cli/azure/authenticate-azure-cli).</li><li>Verify that the Azure CLI can obtain tokens. See [below](#verify-the-azure-cli-can-obtain-tokens) for instructions.</li></ul>|
|invalid subscription [your subscription]. If this is the name of a subscription, use its ID instead|The subscription name contains a character that may not be safe in a command line.|Use the subscription's ID instead of its name. You can get this from the Azure CLI: `az account show --name "[your subscription]" --query "id"`

#### Verify the Azure CLI can obtain tokens

You can manually verify that the Azure CLI can authenticate and obtain tokens. First, use the `account` command to verify the logged in account.

```azurecli
az account show
```

Once you've verified the Azure CLI is using the correct account, you can validate that it's able to obtain tokens for that account.

```azurecli
az account get-access-token --output json --scope https://management.core.windows.net/.default
```

> This command's output will contain an access token and SHOULD NOT BE SHARED, to avoid compromising account security.

<a id="azd"></a>
## Troubleshoot AzureDeveloperCliCredential authentication issues

| Error Message |Description| Mitigation |
|---|---|---|
|azd not found on PATH|The Azure Developer CLI isn't installed or couldn't be found.|<ul><li>Ensure the Azure Developer CLI is properly installed. See the installation instructions at [Install or update the Azure Developer CLI](https://learn.microsoft.com/azure/developer/azure-developer-cli/install-azd).</li><li>Validate the installation location has been added to the `PATH` environment variable.</li></ul>|
|please run `azd auth login` from a command prompt before using this credential|No account is logged into the Azure Developer CLI, or the login has expired.|<ul><li>Log in to the Azure Developer CLI using the `azd login` command.</li><li>Validate that the Azure Developer CLI can obtain tokens. For instructions, see [Verify the Azure Developer CLI can obtain tokens](#verify-the-azure-developer-cli-can-obtain-tokens).</li></ul>|

#### Verify the Azure Developer CLI can obtain tokens

You can manually verify that the Azure Developer CLI is properly authenticated and can obtain tokens. Execute the command corresponding to your CLI version to verify the account currently logged in.

- In Azure Developer CLI versions >= 1.23.0:

    ```sh
    azd auth status
    ```

- In Azure Developer CLI versions < 1.23.0:

    ```sh
    azd config list
    ```

Once you've verified the Azure Developer CLI is using the correct account, you can validate that it's able to obtain tokens for this account.

```sh
azd auth token --output json --scope https://management.core.windows.net/.default
```

> This command's output will contain an access token and SHOULD NOT BE SHARED, to avoid compromising account security.

<a id="workload"></a>
## Troubleshoot `WorkloadIdentityCredential` authentication issues

| Error Message |Description| Mitigation |
|---|---|---|
|no client ID/tenant ID/token file specified|Incomplete configuration|In most cases these values are provided via environment variables set by Azure Workload Identity.<ul><li>If your application runs on Azure Kubernetes Service (AKS) or a cluster that has deployed the Azure Workload Identity admission webhook, check pod labels and service account configuration. See the [AKS documentation](https://learn.microsoft.com/azure/aks/workload-identity-deploy-cluster#disable-workload-identity) and [Azure Workload Identity troubleshooting guide](https://azure.github.io/azure-workload-identity/docs/troubleshooting.html) for more details.<li>If your application isn't running on AKS or your cluster hasn't deployed the Workload Identity admission webhook, set these values in `WorkloadIdentityCredentialOptions`

<a id="apc"></a>
## Troubleshoot `AzurePipelinesCredential` authentication issues

| Error Message |Description| Mitigation |
|---|---|---|
| AADSTS900023: Specified tenant identifier 'some tenant ID' is neither a valid DNS name, nor a valid external domain.|The `tenant_id` argument to `AzurePipelinesCredential::new()` is incorrect| Verify the tenant ID. It must identify the tenant of the user-assigned managed identity or service principal configured for the service connection.|
| No service connection found with identifier |The `service_connection_id` argument to `AzurePipelinesCredential::new()` is incorrect| Verify the service connection ID. This parameter refers to the `resourceId` of the Azure Service Connection. It can also be found in the query string of the service connection's configuration in Azure DevOps. [Azure Pipelines documentation](https://learn.microsoft.com/azure/devops/pipelines/library/service-endpoints?view=azure-devops&tabs=yaml) has more information about service connections.|
|401 (Unauthorized) response from OIDC endpoint|The `system_access_token` argument to `AzurePipelinesCredential::new()` is incorrect|Check pipeline configuration. This value comes from the predefined variable `System.AccessToken` [as described in Azure Pipelines documentation](https://learn.microsoft.com/azure/devops/pipelines/build/variables?view=azure-devops&tabs=yaml#systemaccesstoken).|

## Get additional help

Additional information on ways to reach out for support can be found in [SUPPORT.md](https://github.com/Azure/azure-sdk-for-rust/blob/main/SUPPORT.md).
