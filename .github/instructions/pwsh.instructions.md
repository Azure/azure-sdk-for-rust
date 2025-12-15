---
applyTo: "eng/**/*.ps1"
---

# PowerShell Script Instructions

## Path Handling

- **Always use `[System.IO.Path]::Combine` instead of `Join-Path`**
  - Use the full form: `([System.IO.Path]::Combine($path1, $path2, ...))`
  - Example: `([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))`
  - This ensures consistent path handling across all platforms

## Command Invocation

- **Use `Invoke-LoggedCommand` for invoking external commands**
  - Commands should be logged unless they output to stderr that needs to be processed
  - Use `-GroupOutput` to collapse output in CI logs
  - Use `-DoNotExitOnFailedExitCode` when you need to handle exit codes manually
  - Example: `Invoke-LoggedCommand "cargo build --keep-going" -GroupOutput`
  - Example with error handling: `Invoke-LoggedCommand "cargo test ..." -GroupOutput -DoNotExitOnFailedExitCode`

## Output Grouping

- **Use `LogGroupStart` and `LogGroupEnd` to collapse verbose output**
  - Use for output that is verbose or not relevant in most cases (e.g., large JSON dumps, detailed test output)
  - Example:
    ```powershell
    LogGroupStart "Raw JSON Output"
    Get-Content $jsonFile | ForEach-Object { Write-Host $_ }
    LogGroupEnd
    ```

## Console Output

- **Do not use emojis in console output**
  - Use plain text instead
  - Bad: `Write-Host "✅ Tests passed"`
  - Good: `Write-Host "Tests passed"`

## Code Organization

- **Extract common code patterns into functions**
  - If you see similar code blocks repeated, extract them into a reusable function
  - Functions should have clear parameters and return values
  - Example: Extract test execution logic into a shared function rather than duplicating it

## Parameter Defaults

- **Use default parameter values when sensible defaults are available**
  - Set default values directly in the parameter declaration instead of checking and setting in the script body
  - Example:
    ```powershell
    param(
      [string]$OutputDirectory = ([System.IO.Path]::Combine($PSScriptRoot, '..', 'output'))
    )
    ```

## Error Handling

- **Use `LogError` for error messages if common.ps1 is imported**
  - If you've imported common.ps1, use `LogError` followed by `exit 1`
  - If you're not sure if common.ps1 is imported, use `Write-Host` with red color
  - Never use `Write-Error`
  - Example with common.ps1: 
    ```powershell
    if ($exitCode -ne 0) {
      LogError "Operation failed"
      exit 1
    }
    ```
  - Example without common.ps1:
    ```powershell
    if ($exitCode -ne 0) {
      Write-Host "Operation failed" -ForegroundColor Red
      exit 1
    }
    ```
- **Use `LogWarning` for warning messages if common.ps1 is imported**
  - If you've imported common.ps1, use `LogWarning` instead of `Write-Warning`
  - Example: `LogWarning "Test results directory not found"`
- **Fail fast on errors**
  - Exit immediately when a critical operation fails
  - Unless otherwise specified, don't accumulate errors and summarize at the end
