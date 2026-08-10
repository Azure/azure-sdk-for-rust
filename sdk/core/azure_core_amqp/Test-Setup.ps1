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

    # Bound each attempt. The blocking Connect() method takes no timeout, so a
    # firewall that drops the SYN makes it wait for the operating system
    # default. That default is longer than $TimeoutSeconds on Linux.
    $remainingMilliseconds = [int][Math]::Min(1000, ($deadline - (Get-Date)).TotalMilliseconds)
    if ($remainingMilliseconds -le 0) {
      break
    }

    $client = [System.Net.Sockets.TcpClient]::new()
    try {
      $connectTask = $client.ConnectAsync($HostName, $Port)
      if ($connectTask.Wait($remainingMilliseconds) -and $client.Connected) {
        return $true
      }
    }
    catch {
      # The connection failed. Try again until the deadline.
      Write-Debug "Connection to ${HostName}:${Port} failed: $_"
    }
    finally {
      $client.Dispose()
    }

    Start-Sleep -Milliseconds 500
  }

  return $false
}

function Test-EnvironmentFlag {
  param([string]$Name)

  $value = [System.Environment]::GetEnvironmentVariable($Name)
  if ([string]::IsNullOrWhiteSpace($value)) {
    return $false
  }

  $value = $value.Trim()
  return -not (($value -eq "0") -or ($value -ieq "false"))
}

function Test-BrokerPinReachable {
  param(
    [string]$Repository,
    [string]$CommitHash,
    [string]$BranchName = "master",
    [int]$TimeoutSeconds = 15
  )

  # `git merge-base --is-ancestor` cannot answer this question. The broker
  # clone is shallow, so it holds one commit and no parents, and every commit
  # looks unreachable. The GitHub compare API answers it in one call, and that
  # call works without a token on a public repository.
  #
  # The rule is `ahead_by -eq 0`. Do not read the status string: a reachable
  # commit reports "identical" when it is the head of the branch, and "behind"
  # when it is older.
  #
  # The function returns $true, $false, or $null when the check did not run.
  # The unauthenticated rate limit is 60 requests each hour for each IP
  # address, and CI agents share an address, so a failed call must never fail
  # the build. Every failure returns $null.
  $compareUri = "https://api.github.com/repos/$Repository/compare/$BranchName...$CommitHash"
  $headers = @{
    "Accept"               = "application/vnd.github+json"
    "X-GitHub-Api-Version" = "2022-11-28"
  }

  try {
    $comparison = Invoke-RestMethod `
      -Uri $compareUri `
      -Method Get `
      -Headers $headers `
      -TimeoutSec $TimeoutSeconds
  }
  catch {
    LogWarning "The request to $compareUri failed: $_"
    return $null
  }

  if ($null -eq $comparison -or $null -eq $comparison.ahead_by) {
    LogWarning "The response from $compareUri does not hold an ahead_by field."
    return $null
  }

  # Coerce after the absence test, never instead of it. `$null -as [int]` gives 0, so a
  # coercion on its own would turn a missing field into 0 and read as reachable. A
  # non-numeric ahead_by counts as "the check did not run", because a build that sets
  # TEST_BROKER_REQUIRE_MERGED must not fail on a malformed answer.
  $aheadBy = $comparison.ahead_by -as [int]
  if ($null -eq $aheadBy) {
    LogWarning "The response from $compareUri holds a non-numeric ahead_by field."
    return $null
  }

  return [bool]($aheadBy -eq 0)
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

# The identifier of the broker job, and the flag that tells the finally block
# whether setup finished. The finally block stops the broker on every path
# that does not finish, because Invoke-LoggedCommand calls `exit` from inside
# itself when a command fails.
$brokerJobId = $null
$setupSucceeded = $false

# Clone and build the Test Amqp Broker.
try {

  $repositoryDir = [System.IO.Path]::Combine($WorkingDirectory, "azure-amqp")
  if (Test-Path $repositoryDir) {
    Write-Host "Removing previously cloned repository: $repositoryDir"
    Remove-Item $repositoryDir -Force -Recurse | Out-Null
  }

  $repositoryName = "Azure/azure-amqp"
  $repositoryUrl = "https://github.com/$repositoryName.git"

  # The pinned azure-amqp commit, as a full 40-character SHA, so that the
  # broker build stays reproducible. A tag is not an option here, because
  # azure-amqp uses lightweight tags and has no tag ruleset, so a maintainer
  # can move a tag to a different commit without a trace.
  #
  # This SHA is the head of master in Azure/azure-amqp. The reachability check
  # below stays quiet while the pin sits on master.
  #
  # To update the pin:
  #   1. Pick an azure-amqp commit that contains nuget.cfsclean.config and
  #      builds TestAmqpBroker for net10.0.
  #   2. Put the full 40-character SHA of that commit below, and update the
  #      comment above with the ref that the SHA comes from. For a merged
  #      pull request, use the merge_commit_sha, not the head SHA.
  #   3. Update the same SHA in sdk/core/azure_core_amqp/README.md.
  #   4. Run this script and then Test-Cleanup.ps1. Make sure that setup
  #      reports a clean azure-amqp clone.
  #
  # Set TEST_BROKER_COMMIT to point the broker at a different commit without a
  # code change.
  $repositoryHash = "111de654e170de3ab6cefe150043458c67b6660d"
  if (-not [string]::IsNullOrWhiteSpace($env:TEST_BROKER_COMMIT)) {
    $repositoryHash = $env:TEST_BROKER_COMMIT.Trim()
    Write-Host "TEST_BROKER_COMMIT overrides the pinned azure-amqp commit: $repositoryHash"
  }

  if ($repositoryHash -notmatch '^[0-9a-fA-F]{40}$') {
    LogError "The azure-amqp pin must be a full 40-character commit SHA, but it is '$repositoryHash'."
    exit 1
  }

  $cloneCommand = "git clone --revision $repositoryHash --depth=1 $repositoryUrl `"$repositoryDir`""

  Write-Host "Cloning repository from $repositoryUrl..."
  Invoke-LoggedCommand $cloneCommand

  # Take the last line only. Invoke-LoggedCommand returns every output line, and an array
  # here would turn the comparison below into a filter.
  #
  # Keep this as two statements. Wrapping the call in "$( ... )" breaks the argument, because
  # the outer double-quoted string consumes the escaped quotes first. The command then splits
  # into three arguments, the second one binds to -ExecutePath, and git runs with a bare -C.
  $repositoryHeadLines = Invoke-LoggedCommand "git -C `"$repositoryDir`" rev-parse HEAD"
  $repositoryHead = [string]($repositoryHeadLines | Select-Object -Last 1)
  if ($repositoryHead.Trim() -ne $repositoryHash) {
    LogError "Expected azure-amqp commit $repositoryHash, but cloned $repositoryHead."
    exit 1
  }

  # A reachable pin says nothing. Only the other two outcomes write a message.
  $pinIsOnMaster = Test-BrokerPinReachable `
    -Repository $repositoryName `
    -CommitHash $repositoryHash
  if ($null -eq $pinIsOnMaster) {
    # The check did not run. Continue always, even when
    # TEST_BROKER_REQUIRE_MERGED is set. A rate limit or a network error is
    # not evidence that the pin is bad.
    LogWarning "The reachability check for azure-amqp commit $repositoryHash did not run. The pin is unchanged."
  }
  elseif (-not $pinIsOnMaster) {
    $pinMessage = @(
      "The azure-amqp commit $repositoryHash is not reachable from master."
      "If the source pull request has merged, this is expected: azure-amqp squash-merges, so the head commit of a pull request never lands on master."
      "Update the pin to the squash commit on master, which is the merge_commit_sha of the merged pull request. Do not use the merge_commit_sha of an open pull request, because that is a throwaway test-merge commit that disappears."
    ) -join "`n"

    if (Test-EnvironmentFlag "TEST_BROKER_REQUIRE_MERGED") {
      LogError "$pinMessage`nTEST_BROKER_REQUIRE_MERGED is set, so this is an error."
      exit 1
    }
    LogWarning $pinMessage
  }

  # The dotnet arguments below are relative to the clone root. Keep the
  # absolute forms only for the checks.
  $brokerProjectRelative = [System.IO.Path]::Combine("test", "TestAmqpBroker", "TestAmqpBroker.csproj")
  $brokerProject = [System.IO.Path]::Combine($repositoryDir, $brokerProjectRelative)
  if (!(Test-Path $brokerProject)) {
    LogError "The pinned azure-amqp commit does not contain $brokerProject."
    exit 1
  }

  # The restore config belongs to this repository, and not to the broker clone. The restricted
  # feed policy is this pipeline's requirement, so the file that satisfies it sits next to this
  # script. Pass an absolute path, because the dotnet calls run from the clone root.
  $nugetConfig = [System.IO.Path]::Combine($PSScriptRoot, "nuget.cfsclean.config")
  if (!(Test-Path $nugetConfig)) {
    LogError "This repository does not contain $nugetConfig."
    exit 1
  }

  # Run the restore and the build from the clone root. This Push-Location is
  # load-bearing: the dotnet command reads global.json from the current
  # directory and not from the project directory, and the arguments above are
  # relative to the clone root. Without it, the SDK version in the
  # global.json of azure-amqp is never applied.
  Push-Location -Path $repositoryDir
  try {
    Invoke-LoggedCommand `
      "dotnet restore `"$brokerProjectRelative`" --configfile `"$nugetConfig`"" `
      -GroupOutput
    Invoke-LoggedCommand `
      "dotnet build `"$brokerProjectRelative`" --configuration Debug --framework net10.0 --no-restore" `
      -GroupOutput
  }
  finally {
    Pop-Location
  }

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

  $brokerJobId = $job.Id
  $env:TEST_BROKER_JOBID = $job.Id

  Write-Host "Waiting up to 30 seconds for the test broker to accept connections..."
  if (!(Wait-TestBroker -JobId $job.Id -HostName $brokerHost -Port $brokerPort)) {
    Write-TestBrokerOutput -JobId $job.Id
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
    LogError "Test broker setup changed files in the azure-amqp clone."
    exit 1
  }

  Write-Host "The azure-amqp clone is clean after setup."
  $setupSucceeded = $true
}
finally {
  # Stop the broker on every path that does not finish setup. A broker that
  # stays alive holds port 25672 and breaks the next run.
  if (-not $setupSucceeded -and $null -ne $brokerJobId) {
    Write-Host "Setup did not finish. Stopping the test broker."
    Stop-TestBrokerJob -JobId $brokerJobId
    $env:TEST_BROKER_JOBID = $null
    $env:TEST_BROKER_ADDRESS = $null
  }

  Pop-Location
}
