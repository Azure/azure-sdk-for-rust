<!-- cspell:ignore cfsclean configfile -->

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

Set `TEST_BROKER_REQUIRED` to make a missing broker an error instead of a skip. The broker-dependent tests then fail when `TEST_BROKER_ADDRESS` is absent. The CI pipeline sets `TEST_BROKER_REQUIRED`, so a broker that stops running turns the build red.

One existing AMQP broker is the "TestAMQPBroker" from the azure-amqp GitHub repository.

The broker can be installed and run through the setup script or through the manual steps below.

### Scripted broker install

Install [PowerShell](https://learn.microsoft.com/powershell/scripting/install/installing-powershell), the [.NET 10 SDK](https://dot.net/download), and [Git](https://git-scm.com/downloads) 2.49 or later. Git 2.49 added the `git clone --revision` option that the setup script uses. Run the setup script from the repository root.

```pwsh
./sdk/core/azure_core_amqp/Test-Setup.ps1
```

The script clones Azure/azure-amqp at commit [`111de654e170de3ab6cefe150043458c67b6660d`](https://github.com/Azure/azure-amqp/commit/111de654e170de3ab6cefe150043458c67b6660d), restores through the `nuget.cfsclean.config` in this package, builds `TestAmqpBroker` for .NET 10, and launches it in the background. Run the package tests in the same PowerShell process so `TEST_BROKER_ADDRESS` remains available.

```pwsh
cargo test --package azure_core_amqp --all-features
```

Stop the broker after the tests finish.

```pwsh
./sdk/core/azure_core_amqp/Test-Cleanup.ps1
```

#### Updating the broker pin

Update the pin only to an azure-amqp commit that contains `nuget.cfsclean.config` and builds `TestAmqpBroker` for `net10.0`. Change `$repositoryHash` in `Test-Setup.ps1` to the full 40-character SHA, update the same SHA in this file, run the setup and cleanup scripts, and make sure that setup reports a clean azure-amqp clone. The pin stays a bare SHA. A tag is not safe here, because azure-amqp uses lightweight tags and has no tag ruleset, so a maintainer can move a tag to a different commit without a trace.

Set `TEST_BROKER_COMMIT` to try a different commit without a code change.

```pwsh
$env:TEST_BROKER_COMMIT = '<full 40-character SHA>'
```

Setup also asks the GitHub compare API whether the pinned commit is reachable from azure-amqp `master`. A reachable pin says nothing. An unreachable pin writes a warning, and `TEST_BROKER_REQUIRE_MERGED` turns that warning into an error. A check that could not run writes a warning and always continues, because the unauthenticated rate limit is 60 requests each hour for each IP address.

The current pin is `111de654e170de3ab6cefe150043458c67b6660d`, the head of `master` in Azure/azure-amqp, so no warning appears. Azure/azure-amqp squash-merges its pull requests, so the commit that lands on `master` is the `merge_commit_sha` of a merged pull request and never the head commit of that pull request.

### Manual broker install

Clone the pinned azure-amqp commit to a local directory.

```pwsh
cd <Test Working Directory>
git clone https://github.com/Azure/azure-amqp --revision 111de654e170de3ab6cefe150043458c67b6660d
```

Normal external developer builds use the repository's standard NuGet configuration.

```pwsh
cd azure-amqp
dotnet build .\test\TestAmqpBroker\TestAmqpBroker.csproj --configuration Debug --framework net10.0
```

CFSClean builds restore from the `azure-sdk-for-net` Azure Artifacts feed. The feed is public and answers anonymous reads, so an external developer needs no credentials to restore a package that the feed has already cached. The CFSClean environment supplies credentials because a cache miss makes the feed fetch the package from upstream, and that fetch needs an authenticated caller. Run this restore and build sequence from the clone root.

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
