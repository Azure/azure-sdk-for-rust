# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Load common ES scripts
. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

# Install and launch the Cosmos emulator
try {
  # Clean up env vars
  $env:AZURE_TEST_MODE = $env:_COSMOS_OLD_TEST_MODE
  Remove-Item env:\AZURE_COSMOS_CONNECTION_STRING

  # Stop the emulator
  if ($IsWindows) {
    throw "Windows not yet supported"
    exit 1
  }
  elseif (Get-Command "docker" -ErrorAction SilentlyContinue) {
    docker rm -f cosmos-emulator-tests
  }
  else {
    Write-Error "Cosmos emulator is not supported on this platform."
    exit 1
  }
}
finally {
  Pop-Location
}
