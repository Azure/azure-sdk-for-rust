#!/usr/bin/env pwsh
# Serialize test execution to rule out concurrency-related hangs in CI.
# Remove this file once the root cause is identified.
$env:RUST_TEST_THREADS = '1'
Write-Host "RUST_TEST_THREADS set to $env:RUST_TEST_THREADS"
exit 0
