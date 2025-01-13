# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cspell: ignore JOBID runtimeconfig

param (
  [string]$PackageName
)

# Load common ES scripts
. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

$WorkingDirectory = ([System.IO.Path]::Combine($RepoRoot, "../TestArtifacts"))

if (-not $PackageName) {
  Write-Host "PackageName parameter not provided."
  exit 1
}

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
  # $repositoryRelease = "v2.6.9"
  # $cloneCommand = "git clone $repositoryUrl --branch $repositoryRelease"
  $cloneCommand = "git clone $repositoryUrl"

  Write-Host "Cloning repository from $repositoryUrl..."
  Invoke-LoggedCommand $cloneCommand

  Set-Location -Path "./azure-amqp/test/TestAmqpBroker"

  #  Invoke-LoggedCommand "dotnet publish --self-contained --framework net6.0"
  Invoke-LoggedCommand "dotnet build --framework net6.0"
  if (!$? -ne 0) {
    Write-Error "Failed to build TestAmqpBroker."
    exit 1
  }

  Write-Host "Test broker built successfully."

  # now that the Test broker has been built, launch the broker on a local address.
  $env:TEST_BROKER_ADDRESS = 'amqp://127.0.0.1:25672'

  Write-Host "Starting test broker listening on $env:TEST_BROKER_ADDRESS ..."

  Set-Location -Path $WorkingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net6.0

  #  if ($IsLinux) {
  #    Set-Location -Path $WorkingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net6.0/linux-x64/publish
  #  }
  #  elseif ($IsMacOS) {
  #
  #    Set-Location -Path $WorkingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net6.0/osx-x64/publish
  #  }
  #  else {
  #    Set-Location -Path $WorkingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net6.0/win-x64/publish
  #  }

  #  $job = ./TestAmqpBroker $($env:TEST_BROKER_ADDRESS) /headless &
  Get-ChildItem -filter TestAmqpBroker*

  #  $job = dotnet --runtimeconfig ./TestAmqpBroker.runtimeconfig.json ./TestAmqpBroker.dll $($env:TEST_BROKER_ADDRESS) /headless &
  $job = dotnet ./TestAmqpBroker.dll $($env:TEST_BROKER_ADDRESS) /headless &

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
