# cspell: ignore LASTEXITCODE
param (
  [string]$packageName,
  [string]$workingDirectory
)

if (-not $packageName) {
  Write-Host "packageName parameter not provided."
  exit 1
}

if (-not $workingDirectory) {
  Write-Host "workingDirectory parameter not provided."
  exit 1
}

if ($packageName -eq "azure_core_amqp") {
  # Test setup for the azure_core_amqp package.
  if (Test-Path $workingDirectory) {
    Write-Host "Removing existing working directory..."
    Remove-Item -Recurse -Force $workingDirectory
  }

  New-Item -ItemType Directory -Path $workingDirectory

  Write-Host "Setting current directory to working directory: $workingDirectory"
  Push-Location -Path $workingDirectory
  try {

    $repositoryUrl = "https://github.com/Azure/azure-amqp.git"
    $cloneCommand = "git clone $repositoryUrl"

    Write-Host "Cloning repository from $repositoryUrl..."
    Invoke-LoggedCommand $cloneCommand

    Push-Location -Path "./azure-amqp/test/TestAmqpBroker"
    try {
      Invoke-LoggedCommand "dotnet restore"
      if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to restore dependencies for TestAmqpBroker."
        exit 1
      }

      Invoke-LoggedCommand "dotnet build"
      if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to build TestAmqpBroker."
        exit 1
      }

      if ($IsLinux) {
        Write-Host "Setting execute permission for TestAmqpBroker..."
        Invoke-LoggedCommand "chmod +x $workingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net462/TestAmqpBroker.exe"
        if ($LASTEXITCODE -ne 0) {
          Write-Error "Failed to set execute permission for TestAmqpBroker."
          exit 1
        }
      }
    }
    finally {
      Pop-Location
    }

    # now that the Test broker has been built, launch the broker on a local address.
    $env:TEST_BROKER_ADDRESS = 'amqp://127.0.0.1:25672'

    $remoteJob = Start-Process $workingDirectory"/azure-amqp/bin/Debug/TestAmqpBroker/net462/TestAmqpBroker.exe" -ArgumentList { "$env:TEST_BROKER_ADDRESS, /headless" }
    Write-Host "Starting test broker..." $remoteJob
    $env:TEST_BROKER_PID = (Get-Process -Name "TestAmqpBroker").Id
    Write-Host "Test broker started with PID: $env:TEST_BROKER_PID"
  }
  finally {
    Pop-Location
  }
}
else {
  # Other packages do not need any special setup.
  Write-Host "Skipping test setup for package $packageName."
  exit 0
}
