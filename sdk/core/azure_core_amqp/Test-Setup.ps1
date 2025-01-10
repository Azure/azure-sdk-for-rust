# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cspell: ignore JOBID

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
Write-Host Using Working Directory $WorkingDirectory

if (-not (Test-Path $WorkingDirectory)) {
  Write-Host "Working directory does not exist, creating working directory: $WorkingDirectory"
  New-Item -ItemType Directory -Path $WorkingDirectory
}

Write-Host "Setting current directory to working directory: $WorkingDirectory"
Push-Location -Path $WorkingDirectory

# Clone and build the Test Amqp Broker.
try {

  $repositoryUrl = "https://github.com/Azure/azure-amqp.git"
  $cloneCommand = "git clone $repositoryUrl"

  Write-Host "Cloning repository from $repositoryUrl..."
  Invoke-LoggedCommand $cloneCommand

  Set-Location -Path "./azure-amqp/test/TestAmqpBroker"
  Invoke-LoggedCommand "dotnet restore"
  if (!$? -ne 0) {
    Write-Error "Failed to restore dependencies for TestAmqpBroker."
    exit 1
  }

  Invoke-LoggedCommand "dotnet build"
  if (!$? -ne 0) {
    Write-Error "Failed to build TestAmqpBroker."
    exit 1
  }

  Invoke-LoggedCommand "dotnet publish --self-contained --framework net6.0"

  Write-Host "Test broker built successfully."

  # now that the Test broker has been built, launch the broker on a local address.
  $env:TEST_BROKER_ADDRESS = 'amqp://127.0.0.1:25672'

  Write-Host "Starting test broker listening on " $env:TEST_BROKER_ADDRESS "..."

  if ($IsLinux) {
    Set-Location -Path $WorkingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net6.0/linux-x64/publish
  }
  elseif ($IsMacOS) {
    Set-Location -Path $WorkingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net6.0/osx-x64/publish
  }
  else {
    Set-Location -Path $WorkingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net6.0/win-x64/publish
  }

  #  Set-Location -Path $WorkingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net6.0
  Get-ChildItem -Filter TestAmqpBroker*
  $job = ./TestAmqpBroker $($env:TEST_BROKER_ADDRESS) /headless &

  Write-Host Broker job is ($($job).Id)

  Write-Host Job State:
  Get-Job -Id $($job).Id

  $env:TEST_BROKER_JOBID = $($job).Id

  Write-Host "Waiting for test broker to start..."
  Start-Sleep -Seconds 5

  Write-Host Job Output after 5 seconds:
  Receive-Job $($job).Id

  $job = Get-Job -Id $env:TEST_BROKER_JOBID
  if (-not(($($job).State) -eq "Running")) {
    Write-Host "Test broker failed to start."
    exit 1
  }

  Write-Host "Test broker started with JOB ID: $env:TEST_BROKER_JOBID"
}
finally {
  Pop-Location
}
