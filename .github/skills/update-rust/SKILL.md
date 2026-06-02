---
name: update-rust
description: Update the pinned stable and nightly Rust toolchain versions used by the repository
---

Run `eng/scripts/Update-Rust.ps1` to update the pinned Rust toolchain versions.

## What the script does

- Fetches the latest stable Rust version from GitHub (if `-Version` not specified)
- Detects a working nightly toolchain date from `rustc --version` (if `-NightlyVersion` not specified), advancing day-by-day until `rustup` can install it
- Updates `rust-toolchain.toml`, `eng/scripts/Language-Settings.ps1`, and `eng/scripts/*.rs` shebang lines

## Parameters

- `-Version` — stable channel version (e.g., `"1.96.0"`); omit to auto-detect from GitHub
- `-NightlyVersion` — nightly date (e.g., `"2026-05-01"` or `"nightly-2026-05-01"`); omit to auto-detect from `rustc --version`

## Examples

Update both channels automatically:

```powershell
eng/scripts/Update-Rust.ps1
```

Update to a specific stable version with auto-detected nightly:

```powershell
eng/scripts/Update-Rust.ps1 -Version 1.96.0
```

Update both channels explicitly:

```powershell
eng/scripts/Update-Rust.ps1 -Version 1.96.0 -NightlyVersion 2026-05-01
```
