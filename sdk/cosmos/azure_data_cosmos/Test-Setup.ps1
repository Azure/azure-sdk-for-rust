# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.


# Load common ES scripts
. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

# Install and launch the Cosmos emulator
try {

  if ($IsWindows) {
    throw "Windows not yet supported"
    exit 1
  }
  elseif (Get-Command "docker" -ErrorAction SilentlyContinue) {
    $parameters = @(
      "--publish", "8081:8081"
      "--publish", "10250-10255:10250-10255"
      "--name", "cosmos-emulator-tests"
      "--detach",
      "--env", "AZURE_COSMOS_EMULATOR_ARGS='/noexplorer /noui /enablepreview /disableratelimiting /enableaadauthentication'"
    )
    docker run @parameters mcr.microsoft.com/cosmosdb/linux/azure-cosmos-emulator:latest
  }
  else {
    Write-Error "Cosmos emulator is not supported on this platform."
    exit 1
  }

  # Wait for the emulator to start
  while (-not (docker logs cosmos-emulator-tests | Select-String -Pattern "^Started\s*$")) {
    Start-Sleep -Seconds 1
  }

  Write-Host "Emulator is running"

  # With the emulator online, we can run the tests against the emulator
  $env:AZURE_TEST_MODE = "live"
  $env:AZURE_COSMOS_CONNECTION_STRING = "emulator"
}
finally {
  Pop-Location
}
