# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

function Get-RustupCargoHome() {
  if ($env:CARGO_HOME) {
    return $env:CARGO_HOME
  }

  if (!$HOME) {
    throw 'HOME is not set and CARGO_HOME is not configured.'
  }

  return [System.IO.Path]::Combine($HOME, '.cargo')
}

function Get-RustupBinDirectory() {
  return [System.IO.Path]::Combine((Get-RustupCargoHome), 'bin')
}

function Add-RustupBinDirectoryToPath() {
  $cargoBin = Get-RustupBinDirectory
  $pathEntries = @($env:PATH -split [System.IO.Path]::PathSeparator)
  if ($pathEntries -notcontains $cargoBin) {
    $env:PATH = "$cargoBin$([System.IO.Path]::PathSeparator)$env:PATH"
  }

  return $cargoBin
}

function Test-RustupInstalled() {
  return $null -ne (Get-Command 'rustup' -ErrorAction SilentlyContinue)
}

function Get-RustupBootstrapInstaller() {
  if ($IsWindows) {
    return [pscustomobject]@{
      FileName = 'rustup-init.exe'
      Uri = 'https://win.rustup.rs'
      Command = $null
    }
  }

  $shell = Get-Command 'bash' -ErrorAction SilentlyContinue
  if (!$shell) {
    $shell = Get-Command 'sh' -ErrorAction SilentlyContinue
  }
  if (!$shell) {
    throw 'Neither bash nor sh was found to bootstrap rustup.'
  }

  return [pscustomobject]@{
    FileName = 'rustup-init.sh'
    Uri = 'https://sh.rustup.rs'
    Command = $shell.Source
  }
}

function New-RustupBootstrapPath(
  [Parameter(Mandatory = $true)]
  [string] $FileName
) {
  $tempPath = [System.IO.Path]::GetTempPath()
  while ($true) {
    $tempDirectory = [System.IO.Path]::Combine(
      $tempPath,
      "rustup-$([System.IO.Path]::GetRandomFileName())"
    )

    try {
      New-Item -ItemType Directory -Path $tempDirectory -ErrorAction Stop | Out-Null
      return [pscustomobject]@{
        DirectoryPath = $tempDirectory
        InstallerPath = [System.IO.Path]::Combine($tempDirectory, $FileName)
      }
    }
    catch [System.IO.IOException] {
    }
  }
}

function Ensure-RustupInstalled() {
  if (Test-RustupInstalled) {
    return $false
  }

  Add-RustupBinDirectoryToPath | Out-Null
  if (Test-RustupInstalled) {
    return $false
  }

  $installer = Get-RustupBootstrapInstaller
  $bootstrapPath = New-RustupBootstrapPath -FileName $installer.FileName
  $installerPath = $bootstrapPath.InstallerPath

  try {
    Invoke-WebRequest -Uri $installer.Uri -OutFile $installerPath
    if (!$IsWindows) {
      Invoke-LoggedCommand "chmod 755 `"$installerPath`"" -GroupOutput
      Start-PipedProcess `
        -FilePath $installer.Command `
        -ArgumentList @($installerPath, '--default-toolchain', 'none', '-y') `
        -GroupOutput
    }
    else {
      Start-PipedProcess `
        -FilePath $installerPath `
        -ArgumentList @('--default-toolchain', 'none', '-y') `
        -GroupOutput
    }
  }
  finally {
    if (Test-Path -LiteralPath $bootstrapPath.DirectoryPath) {
      Remove-Item -LiteralPath $bootstrapPath.DirectoryPath -Recurse -Force
    }
  }

  Add-RustupBinDirectoryToPath | Out-Null
  if (!(Test-RustupInstalled)) {
    throw 'rustup was installed but is still unavailable on PATH.'
  }

  return $true
}
