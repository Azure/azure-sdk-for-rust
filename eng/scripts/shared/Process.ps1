# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

function Start-PipedProcess
{
  [CmdletBinding()]
  param
  (
    [Parameter(Mandatory = $true)]
    [string] $FilePath,
    [string[]] $ArgumentList = @(),
    [string] $WorkingDirectory,
    [hashtable] $Environment,
    [switch] $GroupOutput,
    [int[]] $AllowedExitCodes = @(0),
    [switch] $DoNotExitOnFailedExitCode
  )

  $startTime = Get-Date
  $Command = "$FilePath $($ArgumentList -join ' ')".Trim()

  if ($GroupOutput) {
    LogGroupStart $Command
  }
  else {
    Write-Host "> $Command"
  }

  $processStartInfo = [System.Diagnostics.ProcessStartInfo]::new()
  $processStartInfo.FileName = $FilePath
  $processStartInfo.UseShellExecute = $false
  $processStartInfo.CreateNoWindow = $true
  $processStartInfo.RedirectStandardOutput = $true
  $processStartInfo.RedirectStandardError = $true

  if ($WorkingDirectory) {
    $processStartInfo.WorkingDirectory = $WorkingDirectory
  }

  foreach ($argument in $ArgumentList) {
    [void] $processStartInfo.ArgumentList.Add($argument)
  }

  if ($Environment) {
    foreach ($entry in $Environment.GetEnumerator()) {
      if ($null -eq $entry.Value) {
        [void] $processStartInfo.Environment.Remove([string] $entry.Key)
      }
      else {
        $processStartInfo.Environment[[string] $entry.Key] = [string] $entry.Value
      }
    }
  }

  $process = [System.Diagnostics.Process]::new()
  $process.StartInfo = $processStartInfo

  if (!$process.Start()) {
    throw "Failed to start process: $FilePath"
  }

  $stdoutStream = [Console]::OpenStandardOutput()
  $stderrStream = [Console]::OpenStandardError()
  $stdoutTask = $process.StandardOutput.BaseStream.CopyToAsync($stdoutStream)
  $stderrTask = $process.StandardError.BaseStream.CopyToAsync($stderrStream)

  try {
    $process.WaitForExit()
    [System.Threading.Tasks.Task]::WaitAll(@($stdoutTask, $stderrTask))
    $stdoutStream.Flush()
    $stderrStream.Flush()
    $exitCode = $process.ExitCode
    $duration = (Get-Date) - $startTime

    if ($GroupOutput) {
      LogGroupEnd
    }

    if ($exitCode -notin $AllowedExitCodes) {
      LogError "Command failed to execute ($duration): $Command`n"
      if (!$DoNotExitOnFailedExitCode) {
        exit $exitCode
      }
    }
    else {
      Write-Host "Command succeeded ($duration)`n"
    }
  }
  finally {
    $process.Dispose()
  }

  return [pscustomobject]@{
    ExitCode = $exitCode
  }
}
