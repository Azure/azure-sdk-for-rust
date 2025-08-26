# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cspell: ignore JOBID depsfile


# Load common ES scripts
. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

if ($IsMacOS) {
  Write-Host "AMQP tests are not supported on macOS. Skipping test setup."
  exit 0
}

# Create the test binary *outside* the repo root to avoid polluting the repo.
$WorkingDirectory = ([System.IO.Path]::Combine($RepoRoot, "../TestArtifacts"))

# Create the working directory if it does not exist.
Write-Host "Using Working Directory $WorkingDirectory"

if (-not (Test-Path $WorkingDirectory)) {
  Write-Host "Working directory does not exist, creating working directory: $WorkingDirectory"
  New-Item -ItemType Directory -Path $WorkingDirectory
}

Write-Host "Setting current directory to working directory: $WorkingDirectory"
Push-Location -Path $WorkingDirectory

# Clone and build the Test Amqp Broker.
try {

  $repositoryUrl = "https://github.com/Azure/azure-amqp.git"
  $repositoryHash = "d82a86455c3459c5628bc95b25511f6e8a065598"
  $cloneCommand = "git clone $repositoryUrl --revision $repositoryHash"


  Write-Host "Cloning repository from $repositoryUrl..."
  Invoke-LoggedCommand $cloneCommand

  Set-Location -Path "./azure-amqp/test/TestAmqpBroker"

  Invoke-LoggedCommand "dotnet build --framework net8.0"
  if (-not $?) {
    Write-Error "Failed to build TestAmqpBroker."
    exit 1
  }

  Write-Host "Test broker built successfully."

  # now that the Test broker has been built, launch the broker on a local address.
  $env:TEST_BROKER_ADDRESS = 'amqp://127.0.0.1:25672'

  Write-Host "Starting test broker listening on ${env:TEST_BROKER_ADDRESS} ..."

  # Note that we cannot use `dotnet run -f` here because the TestAmqpBroker relies on args[0] being the broker address.
  # If we use `dotnet run -f`, the first argument is the csproj file.
  # Instead, we use `dotnet exec` to run the compiled DLL directly.
  # This allows us to pass the broker address as the first argument.
  Set-Location -Path $WorkingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net8.0
  $job = dotnet exec ./TestAmqpBroker.dll ${env:TEST_BROKER_ADDRESS} /headless &

  $env:TEST_BROKER_JOBID = $job.Id

  Write-Host "Waiting for test broker to start..."
  Start-Sleep -Seconds 3

  Write-Host "Job Output after wait:"
  Receive-Job $job.Id

  $job = Get-Job -Id $env:TEST_BROKER_JOBID
  if ($job.State -ne "Running") {
    Write-Host "Test broker failed to start."
    exit 1
  }
}
finally {
  Pop-Location
}
