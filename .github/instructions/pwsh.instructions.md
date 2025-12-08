# PowerShell Script Instructions

These rules apply to all PowerShell scripts (`**/*.ps1`) in the repository.

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
  - Use for summaries or output that's not relevant in most cases
  - Example:
    ```powershell
    LogGroupStart "Test Summary"
    Write-Host "Passed: $passed"
    Write-Host "Failed: $failed"
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

## Error Handling

- **Fail fast on errors**
  - Exit immediately when a critical operation fails
  - Don't accumulate errors and summarize at the end
  - Use `Write-Error` and `exit` for fatal errors
  - Example: 
    ```powershell
    if ($exitCode -ne 0) {
      Write-Error "Operation failed"
      exit $exitCode
    }
    ```

## Script Scoping

- **Remove unnecessary conditional logic**
  - If a script only runs in one context (e.g., CI only), remove switches and conditionals for other contexts
  - Keep scripts focused on a single purpose
  - Example: Don't add a `-CI` switch if the script only runs in CI environments
