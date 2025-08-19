# Running Azure Service Bus Live Tests

This guide explains how to set up and run live integration tests for the Azure Service Bus SDK for Rust using the Azure SDK test infrastructure.

## Prerequisites

1. **Azure Subscription**: Access to the targeted subscription
2. **Azure CLI**: Install and configure Azure CLI (`az login`)
3. **Azure PowerShell**: Install PowerShell modules (`Install-Module Az -Force`)
4. **Bicep**: For infrastructure deployment (`az bicep install`)
5. **Permissions**: Contributor access to create resources in your subscription

## Automated Test Infrastructure Setup

The Azure SDK provides a standardized test infrastructure using the `New-TestResources.ps1` script with Bicep templates to automatically create and configure all required Azure Service Bus resources.

### Deploy Test Resources

From the repository root, run:

```powershell
# Set subscription
az account set --subscription "<Your Subscription Name>"

# Deploy test infrastructure
./eng/common/TestResources/New-TestResources.ps1 servicebus/azure_messaging_servicebus -SubscriptionId <subscription ID>
```

This will automatically create:

-   Azure Service Bus namespace with Standard tier
-   Test queue named `testqueue`
-   Test topic named `testtopic` with subscription `testsubscription`
-   RBAC permissions for TokenCredential testing
-   All required environment variables

The script will output PowerShell commands to set environment variables.

### Environment Variables

After deployment completes successfully, the script will output commands like:

```powershell
$env:SERVICEBUS_NAMESPACE = "sb-your-deployment-name.servicebus.windows.net"
$env:SERVICEBUS_QUEUE_NAME = "testqueue"
$env:SERVICEBUS_TOPIC_NAME = "testtopic"
$env:SERVICEBUS_SUBSCRIPTION_NAME = "testsubscription"
```

## Run Live Tests

Once the environment variables are set, run the tests:

**All Live Tests:**

```powershell
cd sdk/servicebus/azure_messaging_servicebus
cargo test
```

## Debugging

Enable debug logging for detailed information:

```powershell
$env:RUST_LOG = "debug"
cargo test
```

For detailed AMQP protocol debugging:

```powershell
$env:RUST_LOG = "azure_messaging_servicebus=debug,azure_core_amqp=debug"
cargo test
```

## Troubleshooting

### Deployment Issues

-   Ensure you're logged into Azure CLI: `az login`
-   Verify you have Contributor permissions on the subscription
-   Check that the resource group name doesn't already exist
-   Ensure Service Bus is available in your selected region

### Authentication Issues

-   For TokenCredential tests, ensure you're logged in via Azure CLI
-   RBAC permissions are automatically configured during deployment
-   Service principal credentials can be set via environment variables if needed

### Test Execution Issues

-   Verify all environment variables are set correctly
-   Check that Service Bus resources are in "Active" status
-   Ensure network connectivity to Azure Service Bus
-   Review test output for specific error messages

## Resource Cleanup

When you're finished testing, clean up the resources:

```powershell
# Clean up test resources
./eng/common/TestResources/Remove-TestResources.ps1 servicebus/azure_messaging_servicebus
```

## Performance Notes

-   Live tests create real network connections
-   Messages are sent to actual Azure Service Bus queues
-   Consider using a dedicated test namespace
-   Tests include cleanup operations (complete/delete messages)
-   Some tests may have delays for timing validation
