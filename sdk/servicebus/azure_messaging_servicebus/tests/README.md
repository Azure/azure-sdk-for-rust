# Azure Service Bus Live Tests

This directory contains live integration tests for the Azure Service Bus SDK for Rust. These tests validate the SDK functionality against a real Azure Service Bus namespace.

## Prerequisites

1. **Azure Service Bus Namespace**: You need an active Azure Service Bus namespace with at least one queue configured.

2. **Authentication**: You can use either connection string authentication or TokenCredential-based authentication.

3. **Test Resources**: The tests require specific Azure Service Bus entities to be created beforehand.

## Required Azure Resources

### For Connection String Tests (`servicebus_live_tests.rs`)

Create the following resources in your Service Bus namespace:

- **Queue**: A standard queue for basic send/receive operations
- **Topic**: A topic for publish/subscribe scenarios (optional)
- **Subscription**: A subscription on the topic (optional, only needed for topic tests)

### For TokenCredential Tests (`servicebus_token_credential_tests.rs`)

- **Queue**: Same queue as above
- **Azure AD App Registration** (for ClientSecretCredential tests): Optional, but recommended for comprehensive testing

## Environment Variables

### Connection String Authentication

Set these environment variables for connection string-based tests:

```bash
# Required for all connection string tests
export SERVICEBUS_CONNECTION_STRING="Endpoint=sb://your-namespace.servicebus.windows.net/;SharedAccessKeyName=RootManageSharedAccessKey;SharedAccessKey=your-key"
export SERVICEBUS_QUEUE_NAME="test-queue"

# Optional for topic/subscription tests
export SERVICEBUS_TOPIC_NAME="test-topic"
export SERVICEBUS_SUBSCRIPTION_NAME="test-subscription"
```

### TokenCredential Authentication

Set these environment variables for TokenCredential-based tests:

```bash
# Required for all TokenCredential tests
export SERVICEBUS_NAMESPACE="your-namespace.servicebus.windows.net"
export SERVICEBUS_QUEUE_NAME="test-queue"

# For DeveloperToolsCredential (one of these approaches):

# Option 1: Service Principal with Client Secret
export AZURE_TENANT_ID="your-tenant-id"
export AZURE_CLIENT_ID="your-client-id"
export AZURE_CLIENT_SECRET="your-client-secret"

# Option 2: Use Azure CLI (run 'az login' first)
# No additional environment variables needed

# Option 3: Use Managed Identity (when running on Azure)
# No additional environment variables needed
```

## Required Permissions

For TokenCredential authentication, ensure your identity has the following RBAC roles:

- **Azure Service Bus Data Owner** or **Azure Service Bus Data Sender** and **Azure Service Bus Data Receiver**
- Scope: Service Bus namespace or specific queue/topic

## Running the Tests

### All Tests (Unit + Live)

```bash
cargo test
```

### Only Live Tests

```bash
# Connection string tests
cargo test servicebus_live_tests

# TokenCredential tests
cargo test servicebus_token_credential_tests

# All live tests
cargo test --test '*'
```

### Individual Test Cases

```bash
# Test basic send/receive
cargo test test_send_receive_queue_message

# Test batch operations
cargo test test_send_multiple_messages

# Test message properties
cargo test test_message_properties

# Test PeekLock operations
cargo test test_peek_lock_operations

# Test TokenCredential authentication
cargo test test_default_azure_credential
```

### Running with Logging

```bash
RUST_LOG=debug cargo test test_send_receive_queue_message
```

## Test Coverage

### Connection String Tests (`servicebus_live_tests.rs`)

- **Basic Send/Receive**: Send a message to a queue and receive it back
- **Batch Operations**: Send multiple messages and receive them
- **Message Properties**: Test standard and custom message properties
- **PeekLock Mode**: Test complete and abandon operations
- **ReceiveAndDelete Mode**: Test automatic message deletion
- **Topic/Subscription**: Test publish/subscribe messaging (optional)
- **Scheduled Messages**: Test delayed message delivery

### TokenCredential Tests (`servicebus_token_credential_tests.rs`)

- **DeveloperToolsCredential**: Test with default credential chain
- **ClientSecretCredential**: Test with service principal authentication
- **AzureCliCredential**: Test with Azure CLI credentials
- **Batch Operations**: Test multiple messages with TokenCredential
- **Message Properties**: Test comprehensive property handling

## Test Environment Setup

### Azure Portal Setup

1. **Create Service Bus Namespace**:
   - Go to Azure Portal → Create a resource → Service Bus
   - Choose pricing tier (Standard or Premium for topics)
   - Create the namespace

2. **Create Queue**:
   - In your Service Bus namespace → Queues → Add
   - Name: Use the value you set in `SERVICEBUS_QUEUE_NAME`
   - Configure as needed (defaults are fine for testing)

3. **Create Topic and Subscription** (Optional):
   - In your Service Bus namespace → Topics → Add
   - Name: Use the value you set in `SERVICEBUS_TOPIC_NAME`
   - Create a subscription under the topic
   - Name: Use the value you set in `SERVICEBUS_SUBSCRIPTION_NAME`

4. **Get Connection String**:
   - In your Service Bus namespace → Shared access policies
   - Select "RootManageSharedAccessKey"
   - Copy the "Primary Connection String"

### Azure AD Setup (for TokenCredential)

1. **Create App Registration**:
   - Go to Azure AD → App registrations → New registration
   - Note down Application (client) ID and Directory (tenant) ID

2. **Create Client Secret**:
   - In your app registration → Certificates & secrets
   - Create a new client secret and copy the value

3. **Assign Permissions**:
   - Go to your Service Bus namespace → Access control (IAM)
   - Add role assignment → Azure Service Bus Data Owner
   - Assign to your app registration

## Common Issues

### Authentication Errors

- **Connection String**: Verify the connection string is correct and has proper permissions
- **TokenCredential**: Ensure proper RBAC roles are assigned
- **Azure CLI**: Run `az login` and verify you're logged into the correct tenant

### Missing Resources

- **Queue Not Found**: Verify the queue exists and the name matches `SERVICEBUS_QUEUE_NAME`
- **Topic/Subscription**: These tests are optional and will be skipped if not configured

### Network Issues

- **Firewall**: Ensure your Service Bus namespace allows connections from your IP
- **Corporate Networks**: Some corporate networks may block AMQP connections

## Debugging

Enable verbose logging to troubleshoot issues:

```bash
RUST_LOG=azure_messaging_servicebus=debug,azure_core_amqp=debug cargo test
```

For even more detailed AMQP protocol debugging:

```bash
RUST_LOG=trace cargo test test_send_receive_queue_message
```

## Performance Considerations

Live tests create real network connections and send actual messages, so:

- Tests may take 10-30 seconds to complete
- Consider running against a dedicated test namespace
- Clean up test messages if needed (tests attempt to complete/delete sent messages)
- Some tests include delays for scheduled message testing

## Contributing

When adding new live tests:

1. Follow the existing naming convention: `test_<functionality>`
2. Use the `#[recorded::test(live)]` attribute for live tests
3. Clean up resources (complete/delete messages) in tests
4. Add appropriate assertions to validate functionality
5. Document any new environment variables or setup requirements
