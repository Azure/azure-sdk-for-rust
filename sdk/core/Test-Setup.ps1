# cspell: ignore LASTEXITCODE
param (
  [string]$PackageName,
  [string]$WorkingDirectory
)
. "$PSScriptRoot\..\..\eng\common\scripts\common.ps1"

if (-not $PackageName) {
  Write-Host "PackageName parameter not provided."
  exit 1
}

if (-not $WorkingDirectory) {
  Write-Host "WorkingDirectory parameter not provided."
  exit 1
}

if ($PackageName -eq "azure_core_amqp") {
  # Test setup for the azure_core_amqp package.

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

    if ($IsLinux -or $IsMacOS) {
      Write-Host "Setting execute permission for TestAmqpBroker..."
      Invoke-LoggedCommand "chmod +x $workingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net462/TestAmqpBroker.exe"
      if (!$? -ne 0) {
        Write-Error "Failed to set execute permission for TestAmqpBroker."
        exit 1
      }
    }

    # now that the Test broker has been built, launch the broker on a local address.
    $env:TEST_BROKER_ADDRESS = 'amqp://127.0.0.1:25672'

    Write-Host "Looking for test amqp broker executable..."
    Get-ChildItem -Path $WorkingDirectory/azure-amqp/bin/Debug/TestAmqpBroker -Recurse -Filter TestAmqpBroker*

    Write-Host "Test broker is: $brokerExecutable"
    Write-Host "Starting test broker listening on " $env:TEST_BROKER_ADDRESS "..."

    if ($IsLinux -or $IsMacOS) {
      $job = dotnet run -framework net60 TestAmqpBroker.dll $env:TEST_BROKER_ADDRESS /headless &
      Write-Host Broker job is $job
    }
    else {
      Set-Location $WorkingDirectory/azure-amqp/bin/Debug/TestAmqpBroker/net462

      Start-Process TestAmqpBroker.exe -ArgumentList { ${env:TEST_BROKER_ADDRESS}, "/headless" }
      $env:TEST_BROKER_PID = (Get-Process -Name "TestAmqpBroker").Id
    }
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
