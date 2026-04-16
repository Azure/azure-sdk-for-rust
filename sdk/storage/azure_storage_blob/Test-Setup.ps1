#!/usr/bin/env pwsh
# Serialize test execution to rule out concurrency-related hangs in CI.
# Remove this file once the root cause is identified.
$env:RUST_TEST_THREADS = '1'
Write-Host "RUST_TEST_THREADS set to $env:RUST_TEST_THREADS"

# Set $LASTEXITCODE so Invoke-LoggedCommand in CI doesn't fail with
# "'$LASTEXITCODE' cannot be retrieved because it has not been set."
$global:LASTEXITCODE = 0
