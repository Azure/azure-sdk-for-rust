# azure_core_amqp

Azure AMQP crate for consumption of AMQP based packages in the Azure SDK for C++.

NOTE: THIS IS NOT A GENERAL PURPOSE AMQP LIBRARY AND SHOULD NOT BE USED AS SUCH.

This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).

## Testing the AMQP Client.

The AMQP package is tested using the standard `cargo test` command line:

```bash
cargo test --package azure_core_amqp --all-features
```

Certain functionality of the test requires that the azure-amqp TestAmqpBroker be running at the time of the test.

To enable this functionality, there are two ways of installing and running the TestAmqpBroker, Scripted and Manual.

### Scripted Broker Install
For Scripted testing, simply run the powershell script in the sdk/core/azure_core_amqp directory:

```powershell
PS> .\sdk\core\azure_core_amqp\Test-Setup.ps1
```

This will download the TestAmqpBroker, build it and run the executable.

Note that this requires that you have the [.Net SDK](https://dot.net/download) installed on your machine.

You can then run the azure_core_amqp package tests.

Once you have finished running your tests, you run:

```powershell
PS> .\sdk\core\azure_core_amqp\Test-Cleanup.ps1
```

which will terminate the test broker.

### Manual Broker Install
For Manual testing, first clone the azure-amqp repository to a local directory:

```bash
cd <Test Working Directory>
git clone https://github.com/Azure/azure-amqp
```

Alternately, you can clone to a specific release in the azure-amqp repository:

```
git clone https://github.com/Azure/azure-amqp.git --branch hotfix
```

Set an environment variable the test AMQP broker should listen on:

```powershell
  $env:TEST_BROKER_ADDRESS = 'amqp://127.0.0.1:25672'
```

And launch the test broker:

```powershell
cd azure-amqp/test/TestAmqpBroker
dotnet run -- $env:TEST_BROKER_ADDRESS
```

Now, when you run the cargo tests, the networking functionality of the AMQP APIs will be executed.

License: MIT
