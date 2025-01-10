param (
  [string]$PackageName,
  [string]$WorkingDirectory
)

if (-not $PackageName) {
  Write-Host "Please provide a package name."
  exit 1
}

if (-not $WorkingDirectory) {
  Write-Host "Please provide a working directory."
  exit 1
}

if (-not($PackageName -eq "azure_core_amqp")) {
  Write-Host "Skipping test setup for package $PackageName."
  exit 0
}

# Kill the test broker process started in Test-Setup.ps1
Write-Host "Stopping test broker with PID: $env:TEST_BROKER_PID"
Stop-Process -Id $env:TEST_BROKER_PID -Force
Write-Host "Test broker stopped."
