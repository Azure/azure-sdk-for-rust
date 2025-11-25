$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0
. "$PSScriptRoot/../../../eng/common/scripts/common.ps1"

LogGroupStart "Testing azure_data_cosmos_native C bindings"
$BuildDir = Join-Path $PSScriptRoot "build"
if (Test-Path $BuildDir) {
  Remove-Item -Recurse -Force $BuildDir
}
mkdir $BuildDir
Push-Location $BuildDir
try {
  cmake ..
  make
  make test
}
finally {
  Pop-Location
}
LogGroupEnd
