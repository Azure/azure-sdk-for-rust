# Azure AMQP library for Rust

Azure AMQP crate for consumption of AMQP based packages in the Azure SDK for Rust and C++.

> NOTE: THIS IS NOT A GENERAL PURPOSE AMQP LIBRARY AND SHOULD NOT BE USED AS SUCH.

This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).

## Features

The default features enable AMQP over TCP and secure WebSockets with rustls and the aws-lc-rs provider. The TCP TLS connector reads the trust store of the operating system, including roots installed by an operator.

- `fe2o3_amqp` (default): enables AMQP over TCP.
- `fe2o3_amqp_rustls` (default): enables rustls for AMQP over TCP.
- `fe2o3_amqp_ws` (default): enables AMQP over secure WebSockets.
- `fe2o3_amqp_ws_rustls` (default): enables rustls for AMQP over secure WebSockets.

### Selecting a TLS stack

The crate does not provide a `native-tls` feature. To use another TLS stack, disable the default features, enable the base transport features, and add direct dependencies on the AMQP crates with the TLS features you need. Cargo unifies the dependency features. A TLS stack must be selected somewhere in the dependency graph. If no stack is selected, the crate compiles but opening a connection returns an error.

For AMQP over TCP with `native-tls`:

```toml
[dependencies]
azure_core_amqp = { version = "1.2.0-beta.1", default-features = false, features = [
    "fe2o3_amqp",
] }
fe2o3-amqp = { version = "0.16", features = ["native-tls"] }
```

For AMQP over secure WebSockets with `native-tls`, also enable `fe2o3_amqp_ws` and add `fe2o3-amqp-ws`:

```toml
[dependencies]
azure_core_amqp = { version = "1.2.0-beta.1", default-features = false, features = [
    "fe2o3_amqp_ws",
] }
fe2o3-amqp = { version = "0.16", features = ["native-tls"] }
fe2o3-amqp-ws = { version = "0.16", features = ["native-tls"] }
```

## Testing the AMQP Client

The AMQP package is tested using the standard `cargo test` command line:

```pwsh
cargo test --package azure_core_amqp --all-features
```

Certain AMQP tests requires that there be a running AMQP broker on the machine at the time of the test (the tests will run without the broker, the relevant tests will just be skipped).

One existing AMQP broker is the "TestAMQPBroker" from the azure-amqp GitHub repository.

To launch the TestAMQPBroker, there are two ways of installing and running the TestAmqpBroker, Scripted and Manual.

### Scripted Broker Install

Running the broker from a script requires that you first [install Powershell](https://learn.microsoft.com/powershell/scripting/install/installing-powershell?view=powershell-7.4).
From a running powershell instance,  run the powershell script in the sdk/core/azure_core_amqp directory:

```pwsh
./sdk/core/azure_core_amqp/Test-Setup.ps1
```

This will download the TestAmqpBroker, build it and launch the executable in the background.

Note that this requires that you have the [.NET SDK](https://dot.net/download) installed on your machine.

You can then run the azure_core_amqp package tests.

Once you have finished running your tests, you run:

```pwsh
./sdk/core/azure_core_amqp/Test-Cleanup.ps1
```

which will terminate the test broker.

### Manual Broker Install

For Manual testing, first clone the azure-amqp repository to a local directory:

```pwsh
cd <Test Working Directory>
git clone https://github.com/Azure/azure-amqp
```

Alternately, you can clone to a specific release in the azure-amqp repository:

```pwsh
git clone https://github.com/Azure/azure-amqp.git --branch hotfix
```

Set an environment variable the test AMQP broker should listen on:

```pwsh
$env:TEST_BROKER_ADDRESS = 'amqp://127.0.0.1:25672'
```

And launch the test broker:

```pwsh
cd azure-amqp/test/TestAmqpBroker
dotnet run -- $env:TEST_BROKER_ADDRESS
```

Now, when you run the cargo tests, the networking functionality of the AMQP APIs will be executed.

License: MIT
