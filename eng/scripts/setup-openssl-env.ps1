# Setup script for OpenSSL environment to enable workspace-wide clippy and builds
# Run this script before running cargo clippy --workspace or similar commands

Write-Host "Setting up OpenSSL environment for Rust builds..." -ForegroundColor Green

# Set VCPKG environment variables
$env:VCPKG_ROOT = "C:\vcpkg"
$env:OPENSSL_DIR = "C:\vcpkg\installed\x64-windows-static-md"
$env:CMAKE_TOOLCHAIN_FILE = "C:\vcpkg\scripts\buildsystems\vcpkg.cmake"

Write-Host "Environment variables set:" -ForegroundColor Yellow
Write-Host "  VCPKG_ROOT = $env:VCPKG_ROOT"
Write-Host "  OPENSSL_DIR = $env:OPENSSL_DIR"
Write-Host "  CMAKE_TOOLCHAIN_FILE = $env:CMAKE_TOOLCHAIN_FILE"

# Check if vcpkg and OpenSSL are installed
if (-not (Test-Path "C:\vcpkg\vcpkg.exe")) {
  Write-Host "WARNING: vcpkg not found at C:\vcpkg\vcpkg.exe" -ForegroundColor Red
  Write-Host "Please run the following commands to install vcpkg and OpenSSL:" -ForegroundColor Yellow
  Write-Host "  git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg"
  Write-Host "  C:\vcpkg\bootstrap-vcpkg.bat"
  Write-Host "  C:\vcpkg\vcpkg.exe integrate install"
  Write-Host "  C:\vcpkg\vcpkg.exe install openssl:x64-windows-static-md"
}
elseif (-not (Test-Path "C:\vcpkg\installed\x64-windows-static-md\lib\libssl.lib")) {
  Write-Host "WARNING: OpenSSL not found in vcpkg installation" -ForegroundColor Red
  Write-Host "Please run: C:\vcpkg\vcpkg.exe install openssl:x64-windows-static-md"
}
else {
  Write-Host "✓ vcpkg and OpenSSL are properly installed" -ForegroundColor Green
  Write-Host ""
  Write-Host "You can now run:" -ForegroundColor Cyan
  Write-Host "  cargo clippy --workspace --all-features --all-targets --keep-going --no-deps"
  Write-Host "  cargo build --workspace --all-features --all-targets"
  Write-Host "  cargo test --workspace"
}
