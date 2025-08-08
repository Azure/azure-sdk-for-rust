# Running Azure Service Bus Live Tests

This guide explains how to set up and run live integration tests for the Azure Service Bus SDK for Rust using the Azure SDK test infrastructure.

## Prerequisites

1. **Azure Subscription**: Access to Azure SDK Developer Playground subscription or your own Azure subscription
2. **Azure CLI**: Install and configure Azure CLI (`az login`)  
3. **Azure PowerShell**: Install PowerShell modules (`Install-Module Az -Force`)
4. **Bicep**: For infrastructure deployment (`az bicep install`)
5. **Permissions**: Contributor access to create resources in your subscription

## Automated Test Infrastructure Setup

The Azure SDK provides a standardized test infrastructure using the `New-TestResources.ps1` script with Bicep templates to automatically create and configure all required Azure Service Bus resources.

### Deploy Test Resources

From the repository root, run:

```powershell
# Set subscription (use Azure SDK Developer Playground or your own)
az account set --subscription "Azure SDK Developer Playground"

# Deploy test infrastructure
.\eng\common\TestResources\New-TestResources.ps1 servicebus/azure_messaging_servicebus -SubscriptionId faa080af-c1d8-40ad-9cce-e1a450ca5b57
```

This will automatically create:

- Azure Service Bus namespace with Standard tier
- Test queue named `testqueue`  
- Test topic named `testtopic` with subscription `testsubscription`
- Shared access keys for testing (RootManageSharedAccessKey, ListenOnly, SendOnly)
- RBAC permissions for TokenCredential testing
- All required environment variables

The script will output PowerShell commands to set environment variables.

### Environment Variables

After deployment completes successfully, the script will output commands like:

```powershell
$env:SERVICEBUS_NAMESPACE = "sb-your-deployment-name.servicebus.windows.net"
$env:SERVICEBUS_QUEUE_NAME = "testqueue"
$env:SERVICEBUS_TOPIC_NAME = "testtopic"
$env:SERVICEBUS_SUBSCRIPTION_NAME = "testsubscription"
$env:SERVICEBUS_CONNECTION_STRING = "Endpoint=sb://sb-your-deployment-name.servicebus.windows.net/;SharedAccessKeyName=RootManageSharedAccessKey;SharedAccessKey=..."
```

## Run Live Tests

Once the environment variables are set, run the tests:

**All Live Tests:**

```powershell
cd sdk\servicebus\azure_messaging_servicebus
cargo test --features test
```

**Specific Test Categories:**

```powershell
# Authentication tests
cargo test --test servicebus_authentication --features test

# Basic messaging tests  
cargo test --test servicebus_round_trip --features test

# Batch operations tests
cargo test --test servicebus_batching --features test

# Sender tests
cargo test --test servicebus_sender --features test

# Receiver tests
cargo test --test servicebus_receiver --features test

# Client lifecycle tests
cargo test --test servicebus_client --features test

# Topic/subscription tests
cargo test --test servicebus_topic_subscription --features test
```

**Individual Tests:**

```powershell
# DefaultAzureCredential test
cargo test test_default_azure_credential --features test

# Basic round-trip test
cargo test test_basic_send_receive_round_trip --features test

# Message properties test
cargo test test_message_properties_round_trip --features test
```

## Test Coverage

### ✅ Connection String Tests

- **Basic Send/Receive**: Validates fundamental message operations
- **Multiple Messages**: Tests batch operations with multiple messages  
- **PeekLock Operations**: Tests complete and abandon operations
- **ReceiveAndDelete Mode**: Tests automatic message deletion

### ✅ TokenCredential Tests

- **DefaultAzureCredential**: Tests authentication using Azure identity chain

## Debugging

Enable debug logging for detailed information:

```powershell
$env:RUST_LOG = "debug"
cargo test test_basic_send_receive_connection_string --features test
```

For detailed AMQP protocol debugging:

```powershell
$env:RUST_LOG = "azure_messaging_servicebus=debug,azure_core_amqp=debug"
cargo test test_basic_send_receive_connection_string --features test
```

## Expected Test Behavior

- **Connection String Tests**: Should all pass if environment variables are set correctly
- **TokenCredential Tests**: Will skip if not properly configured (not a failure)
- **Test Duration**: Each test typically takes 10-30 seconds due to network operations
- **Resource Cleanup**: Tests automatically complete/delete sent messages

## Troubleshooting

### Deployment Issues

- Ensure you're logged into Azure CLI: `az login`
- Verify you have Contributor permissions on the subscription
- Check that the resource group name doesn't already exist
- Ensure Service Bus is available in your selected region

### Authentication Issues

- For TokenCredential tests, ensure you're logged in via Azure CLI
- RBAC permissions are automatically configured during deployment
- Service principal credentials can be set via environment variables if needed

### Test Execution Issues

- Verify all environment variables are set correctly
- Check that Service Bus resources are in "Active" status
- Ensure network connectivity to Azure Service Bus
- Review test output for specific error messages

## Resource Cleanup

When you're finished testing, clean up the resources:

```powershell
# Clean up test resources  
.\eng\common\TestResources\Remove-TestResources.ps1 servicebus/azure_messaging_servicebus
```

## Performance Notes

- Live tests create real network connections
- Messages are sent to actual Azure Service Bus queues  
- Consider using a dedicated test namespace
- Tests include cleanup operations (complete/delete messages)
- Some tests may have delays for timing validation

## Success Indicators

✅ **Tests Pass**: All assertions succeed  
✅ **Messages Flow**: Send and receive operations complete  
✅ **Authentication Works**: TokenCredential tests authenticate successfully  
✅ **Cleanup Complete**: No test messages left in queues

## Next Steps

After live tests pass:

1. Review test output for any warnings
2. Consider running against different queue configurations  
3. Test with different message sizes and types
4. Validate performance characteristics with your workload
