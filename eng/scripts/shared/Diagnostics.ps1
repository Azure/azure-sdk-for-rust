# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

function ConvertTo-AzDevOpsLoggingValue(
  [string]$Value,
  [switch]$Property
) {
  $escaped = $Value.Replace('%', '%AZP25').Replace("`r", '%0D').Replace("`n", '%0A')
  if ($Property) {
    $escaped = $escaped.Replace(';', '%3B').Replace(']', '%5D')
  }
  return $escaped
}

function ConvertTo-GitHubLoggingValue(
  [string]$Value,
  [switch]$Property
) {
  $escaped = $Value.Replace('%', '%25').Replace("`r", '%0D').Replace("`n", '%0A')
  if ($Property) {
    $escaped = $escaped.Replace(':', '%3A').Replace(',', '%2C')
  }
  return $escaped
}

function Get-RepositoryRelativePath([string]$Path) {
  if (!$Path) {
    return $null
  }

  try {
    $fullPath = if ([System.IO.Path]::IsPathRooted($Path)) {
      [System.IO.Path]::GetFullPath($Path)
    }
    else {
      [System.IO.Path]::GetFullPath(([System.IO.Path]::Combine($RepoRoot, $Path)))
    }
    $relativePath = [System.IO.Path]::GetRelativePath(
      [System.IO.Path]::GetFullPath($RepoRoot),
      $fullPath
    )

    if ([System.IO.Path]::IsPathRooted($relativePath) -or $relativePath -eq '..' -or $relativePath.StartsWith("../") -or $relativePath.StartsWith("..\"))
    {
      return $null
    }

    return $relativePath.Replace('\', '/')
  }
  catch {
    return $null
  }
}

function ConvertFrom-CSpellIssue([string]$Line) {
  if ($Line -notmatch '^(?<file>.+):(?<line>\d+):(?<column>\d+)\s+-\s+(?<message>.+)$') {
    return $null
  }

  return [pscustomobject]@{
    SourcePath = $Matches.file
    LineNumber = [int]$Matches.line
    ColumnNumber = [int]$Matches.column
    Message = $Matches.message
  }
}

function Write-PipelineIssue(
  [ValidateSet('error', 'warning')]
  [string]$Type,
  [string]$Message,
  [string]$SourcePath,
  [int]$LineNumber,
  [int]$ColumnNumber,
  [string]$Code
) {
  $relativePath = Get-RepositoryRelativePath $SourcePath

  if (Test-SupportsDevOpsLogging) {
    $properties = "type=$Type;"
    if ($relativePath) {
      $properties += "sourcepath=$(ConvertTo-AzDevOpsLoggingValue $relativePath -Property);"
      if ($LineNumber -gt 0) {
        $properties += "linenumber=$LineNumber;"
      }
      if ($ColumnNumber -gt 0) {
        $properties += "columnnumber=$ColumnNumber;"
      }
    }
    if ($Code) {
      $properties += "code=$(ConvertTo-AzDevOpsLoggingValue $Code -Property);"
    }

    Write-Host "##vso[task.logissue $properties]$(ConvertTo-AzDevOpsLoggingValue $Message)"
  }
  elseif (Test-SupportsGitHubLogging) {
    $properties = @()
    if ($relativePath) {
      $properties += "file=$(ConvertTo-GitHubLoggingValue $relativePath -Property)"
      if ($LineNumber -gt 0) {
        $properties += "line=$LineNumber"
      }
      if ($ColumnNumber -gt 0) {
        $properties += "col=$ColumnNumber"
      }
    }
    if ($Code) {
      $properties += "title=$(ConvertTo-GitHubLoggingValue $Code -Property)"
    }

    $propertyText = if ($properties.Count -gt 0) { " $($properties -join ',')" } else { '' }
    Write-Host "::$Type$propertyText::$(ConvertTo-GitHubLoggingValue $Message)"
  }
  elseif ($Type -eq 'error') {
    if ($relativePath) {
      Write-Host "[$relativePath`:$LineNumber`:$ColumnNumber] $Message" -ForegroundColor Red
    }
    else {
      Write-Host $Message -ForegroundColor Red
    }
  }
  else {
    if ($relativePath) {
      Write-Host "[$relativePath`:$LineNumber`:$ColumnNumber] $Message" -ForegroundColor Yellow
    }
    else {
      Write-Host $Message -ForegroundColor Yellow
    }
  }
}

function New-PipelineIssueBudget([int]$Maximum = 50) {
  return [pscustomobject]@{
    Maximum = $Maximum
    Emitted = 0
    Suppressed = 0
  }
}

function Write-BudgetedPipelineIssue(
  $Budget,
  [ValidateSet('error', 'warning')]
  [string]$Type,
  [string]$Message,
  [string]$SourcePath,
  [int]$LineNumber,
  [int]$ColumnNumber,
  [string]$Code
) {
  if ($Budget.Emitted -ge $Budget.Maximum) {
    $Budget.Suppressed++
    return
  }

  Write-PipelineIssue `
    -Type $Type `
    -Message $Message `
    -SourcePath $SourcePath `
    -LineNumber $LineNumber `
    -ColumnNumber $ColumnNumber `
    -Code $Code
  $Budget.Emitted++
}

function Complete-PipelineIssueBudget($Budget) {
  if ($Budget.Suppressed -gt 0) {
    Write-Host "Suppressed $($Budget.Suppressed) additional pipeline issue(s) after reaching the $($Budget.Maximum)-issue limit. See the task log for complete output."
  }
}
