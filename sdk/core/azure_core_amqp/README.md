# Azure AMQP library for Rust

Azure AMQP crate for consumption of AMQP based packages in the Azure SDK for Rust and C++.

> NOTE: THIS IS NOT A GENERAL PURPOSE AMQP LIBRARY AND SHOULD NOT BE USED AS SUCH.

This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).

## Testing the AMQP Client

The AMQP package is tested using the standard `cargo test` command line:

```pwsh
cargo test --package azure_core_amqp --all-features
```

Certain AMQP tests require a running AMQP broker. The tests without a broker still run, and the broker-dependent tests are skipped.

One existing AMQP broker is the "TestAMQPBroker" from the azure-amqp GitHub repository.

The broker can be installed and run through the setup script or through the manual steps below.

### Scripted broker install

Install [PowerShell](https://learn.microsoft.com/powershell/scripting/install/installing-powershell) and the [.NET 10 SDK](https://dot.net/download). Run the setup script from the repository root.

```pwsh
./sdk/core/azure_core_amqp/Test-Setup.ps1
```

The script clones Azure/azure-amqp at commit [`239aff0d87b2c19e1fa91636e0fc0f6ee6e9999a`](https://github.com/Azure/azure-amqp/commit/239aff0d87b2c19e1fa91636e0fc0f6ee6e9999a), restores through the broker repository's `nuget.cfsclean.config`, builds `TestAmqpBroker` for .NET 10, and launches it in the background. The config was added in [Azure/azure-amqp#318](https://github.com/Azure/azure-amqp/pull/318).

Run the package tests in the same PowerShell process so `TEST_BROKER_ADDRESS` remains available.

```pwsh
cargo test --package azure_core_amqp --all-features
```

Stop the broker after the tests finish.

```pwsh
./sdk/core/azure_core_amqp/Test-Cleanup.ps1
```

#### Updating the broker pin

Update the pin only to an azure-amqp commit that contains `nuget.cfsclean.config` and builds `TestAmqpBroker` for `net10.0`. Change `$repositoryHash` in `Test-Setup.ps1`, run the setup and cleanup scripts, and confirm that setup reports a clean azure-amqp clone.

### Manual broker install

Clone the pinned azure-amqp commit to a local directory.

```pwsh
cd <Test Working Directory>
git clone https://github.com/Azure/azure-amqp --revision 239aff0d87b2c19e1fa91636e0fc0f6ee6e9999a
```

Normal external developer builds use the repository's standard NuGet configuration.

```pwsh
cd azure-amqp
dotnet build .\test\TestAmqpBroker\TestAmqpBroker.csproj --configuration Debug --framework net10.0
```

CFSClean builds must authenticate to the `azure-sdk-for-net` Azure Artifacts feed, then run this restore and build sequence from the clone root.

```pwsh
dotnet restore .\test\TestAmqpBroker\TestAmqpBroker.csproj --configfile .\nuget.cfsclean.config
dotnet build .\test\TestAmqpBroker\TestAmqpBroker.csproj --configuration Debug --framework net10.0 --no-restore
```

Set the broker address and launch the built assembly.

```pwsh
$env:TEST_BROKER_ADDRESS = 'amqp://127.0.0.1:25672'
dotnet exec .\bin\Debug\TestAmqpBroker\net10.0\TestAmqpBroker.dll $env:TEST_BROKER_ADDRESS /headless
```

Now, when you run the cargo tests, the networking functionality of the AMQP APIs will be executed.

License: MIT
