Write-Host "Testing C bindings..."

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
