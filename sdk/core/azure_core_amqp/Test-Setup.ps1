# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cspell: ignore JOBID depsfile


# Load common ES scripts
. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

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
  # We would like to use the "hotfix" branch because that is current, but unfortunately it references System.Net.Security version 4.0.0
  $repositoryBranch = "master"
  $cloneCommand = "git clone $repositoryUrl --branch $repositoryBranch"

  Write-Host "Cloning repository from $repositoryUrl..."
  Invoke-LoggedCommand $cloneCommand

  Set-Location -Path "./azure-amqp/test/TestAmqpBroker"

  Invoke-LoggedCommand "dotnet build -p RollForward=LatestMajor --framework net6.0"
  if (!$? -ne 0) {
    Write-Error "Failed to build TestAmqpBroker."
    exit 1
  }

  Write-Host "Test broker built successfully."

  # now that the Test broker has been built, launch the broker on a local address.
  $env:TEST_BROKER_ADDRESS = 'amqp://127.0.0.1:25672'

  Write-Host "Starting test broker listening on ${env:TEST_BROKER_ADDRESS} ..."

  Set-Location -Path $WorkingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net6.0

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
