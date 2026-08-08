# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cspell: ignore JOBID cfsclean configfile depsfile


# Load common ES scripts
. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

function Wait-TestBroker {
  param(
    [int]$JobId,
    [string]$HostName,
    [int]$Port,
    [int]$TimeoutSeconds = 30
  )

  $deadline = (Get-Date).AddSeconds($TimeoutSeconds)
  while ((Get-Date) -lt $deadline) {
    $job = Get-Job -Id $JobId -ErrorAction SilentlyContinue
    if (!$job -or $job.State -ne "Running") {
      return $false
    }

    $client = [System.Net.Sockets.TcpClient]::new()
    try {
      $client.Connect($HostName, $Port)
      return $true
    }
    catch {
      Start-Sleep -Milliseconds 500
    }
    finally {
      $client.Dispose()
    }
  }

  return $false
}

function Stop-TestBrokerJob {
  param([int]$JobId)

  $job = Get-Job -Id $JobId -ErrorAction SilentlyContinue
  if (!$job) {
    return
  }

  if ($job.State -eq "Running") {
    Stop-Job -Id $JobId
  }
  Remove-Job -Id $JobId
}

function Write-TestBrokerOutput {
  param([int]$JobId)

  $job = Get-Job -Id $JobId -ErrorAction SilentlyContinue
  if ($job) {
    Write-Host "Test broker job state: $($job.State)"
    Receive-Job -Id $JobId -Keep
  }
}

if ($IsMacOS) {
  Write-Host "AMQP tests are not supported on macOS. Skipping test setup."
  exit 0
}

# Create the test binary *outside* the repo root to avoid polluting the repo.
$WorkingDirectory = [System.IO.Path]::Combine($RepoRoot, "../TestArtifacts")

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

  $repositoryDir = [System.IO.Path]::Combine($WorkingDirectory, "azure-amqp")
  if (Test-Path $repositoryDir) {
    Write-Host "Removing previously cloned repository: $repositoryDir"
    Remove-Item $repositoryDir -Force -Recurse | Out-Null
  }

  $repositoryUrl = "https://github.com/Azure/azure-amqp.git"
  $repositoryHash = "239aff0d87b2c19e1fa91636e0fc0f6ee6e9999a"
  $cloneCommand = "git clone --revision $repositoryHash --depth=1 $repositoryUrl `"$repositoryDir`""

  Write-Host "Cloning repository from $repositoryUrl..."
  Invoke-LoggedCommand $cloneCommand

  $repositoryHead = Invoke-LoggedCommand "git -C `"$repositoryDir`" rev-parse HEAD"
  if ($repositoryHead.Trim() -ne $repositoryHash) {
    LogError "Expected azure-amqp commit $repositoryHash, but cloned $repositoryHead."
    exit 1
  }

  $brokerProject = [System.IO.Path]::Combine($repositoryDir, "test", "TestAmqpBroker", "TestAmqpBroker.csproj")
  $nugetConfig = [System.IO.Path]::Combine($repositoryDir, "nuget.cfsclean.config")
  if (!(Test-Path $nugetConfig)) {
    LogError "The pinned azure-amqp commit does not contain $nugetConfig."
    exit 1
  }

  Invoke-LoggedCommand `
    "dotnet restore `"$brokerProject`" --configfile `"$nugetConfig`"" `
    -GroupOutput
  Invoke-LoggedCommand `
    "dotnet build `"$brokerProject`" --configuration Debug --framework net10.0 --no-restore" `
    -GroupOutput

  Write-Host "Test broker built successfully."

  $brokerHost = "127.0.0.1"
  $brokerPort = 25672
  $env:TEST_BROKER_ADDRESS = "amqp://${brokerHost}:${brokerPort}"

  Write-Host "Starting test broker listening on ${env:TEST_BROKER_ADDRESS} ..."

  # Note that we cannot use `dotnet run -f` here because the TestAmqpBroker relies on args[0] being the broker address.
  # If we use `dotnet run -f`, the first argument is the csproj file.
  # Instead, we use `dotnet exec` to run the compiled DLL directly.
  # This allows us to pass the broker address as the first argument.
  $brokerOutputDirectory = [System.IO.Path]::Combine(
    $repositoryDir,
    "bin",
    "Debug",
    "TestAmqpBroker",
    "net10.0"
  )
  $brokerAssembly = [System.IO.Path]::Combine($brokerOutputDirectory, "TestAmqpBroker.dll")
  Set-Location -Path $brokerOutputDirectory
  $job = dotnet exec $brokerAssembly ${env:TEST_BROKER_ADDRESS} /headless &

  $env:TEST_BROKER_JOBID = $job.Id

  Write-Host "Waiting up to 30 seconds for the test broker to accept connections..."
  if (!(Wait-TestBroker -JobId $job.Id -HostName $brokerHost -Port $brokerPort)) {
    Write-TestBrokerOutput -JobId $job.Id
    Stop-TestBrokerJob -JobId $job.Id
    LogError "Test broker did not become ready at ${env:TEST_BROKER_ADDRESS}."
    exit 1
  }

  Write-TestBrokerOutput -JobId $job.Id
  Write-Host "Test broker is ready."

  $repositoryStatus = @(
    Invoke-LoggedCommand "git -C `"$repositoryDir`" status --porcelain --untracked-files=all"
  )
  if ($repositoryStatus.Count -ne 0) {
    Write-Host "Files changed in the azure-amqp clone:"
    $repositoryStatus | ForEach-Object { Write-Host $_ }
    Stop-TestBrokerJob -JobId $job.Id
    LogError "Test broker setup changed files in the azure-amqp clone."
    exit 1
  }

  Write-Host "The azure-amqp clone is clean after setup."
}
finally {
  Pop-Location
}
