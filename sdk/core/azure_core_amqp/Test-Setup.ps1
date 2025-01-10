# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cspell: ignore JOBID

param (
  [string]$PackageName
)
. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

$WorkingDirectory = ([System.IO.Path]::Combine($RepoRoot, "../TestArtifacts"))

if (-not $PackageName) {
  Write-Host "PackageName parameter not provided."
  exit 1
}

Write-Host Using Working Directory $WorkingDirectory

if (-not (Test-Path $WorkingDirectory)) {
  Write-Host "Working directory does not exist, creating working directory: $WorkingDirectory"
  New-Item -ItemType Directory -Path $WorkingDirectory
}


Write-Host "Setting current directory to working directory: $WorkingDirectory"
Push-Location -Path $WorkingDirectory
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

  Invoke-LoggedCommand "dotnet publish --framework net6.0"

  if ($IsLinux -or $IsMacOS) {
    Write-Host "Setting execute permission for TestAmqpBroker..."
    Invoke-LoggedCommand "chmod +x $workingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net462/TestAmqpBroker.exe"
    if (!$? -ne 0) {
      Write-Error "Failed to set execute permission for TestAmqpBroker."
      exit 1
    }
  }

  Write-Host "Test broker built successfully."

  Write-Host "Listing files in TestAmqpBroker publish directory..."
  Get-ChildItem -Path $WorkingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net6.0/publish

  # now that the Test broker has been built, launch the broker on a local address.
  $env:TEST_BROKER_ADDRESS = 'amqp://127.0.0.1:25672'

  Write-Host "Starting test broker listening on " $env:TEST_BROKER_ADDRESS "..."

  Set-Location -Path $WorkingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net6.0/publish
  $job = TestAmqpBroker $env:TEST_BROKER_ADDRESS /headless &
  Receive-Job -Job $job

  Write-Host Broker job is ($($job).Id)
  $env:TEST_BROKER_JOBID = $($job).Id

  Write-Host "Test broker started with JOB ID: $env:TEST_BROKER_JOBID"
}
finally {
  Pop-Location
}
