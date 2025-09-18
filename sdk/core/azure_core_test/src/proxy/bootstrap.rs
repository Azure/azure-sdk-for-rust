// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Bootstrap acquisition and startup of the test-proxy.

pub use super::*;
pub use azure_core::{test::TestMode, Result};
pub use serde_json::json;
pub use std::{env, io, path::Path, process::Stdio, time::Duration};
use std::{
    io::{Seek, Write},
    path::PathBuf,
};
pub use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
    process::{ChildStdout, Command},
};
pub use tracing::Level;
use tracing::Span;

// cspell:ignore aspnetcore devcert teamprojectid testproxy
pub const KESTREL_CERT_PATH_ENV: &str = "ASPNETCORE_Kestrel__Certificates__Default__Path";
pub const KESTREL_CERT_PASSWORD_ENV: &str = "ASPNETCORE_Kestrel__Certificates__Default__Password";
pub const KESTREL_CERT_PASSWORD: &str = "password";
pub const MIN_VERSION: Version = Version {
    major: 20241213,
    minor: 1,
    metadata: None,
};
const PROXY_MANUAL_START: &str = "PROXY_MANUAL_START";
const SYSTEM_TEAMPROJECTID: &str = "SYSTEM_TEAMPROJECTID";

/// Starts the test-proxy.
///
/// This is intended for internal use only and should not be called directly in tests.
#[tracing::instrument(level = "debug", fields(crate_dir = ?crate_dir.as_ref(), ?options), err)]
pub async fn start(
    test_mode: Option<TestMode>,
    crate_dir: impl AsRef<Path>,
    options: Option<ProxyOptions>,
) -> Result<Proxy> {
    if env::var(PROXY_MANUAL_START).is_ok_and(|v| v.eq_ignore_ascii_case("true")) {
        tracing::warn!(
            "environment variable {PROXY_MANUAL_START} is 'true'; not starting test-proxy"
        );
        let proxy = Proxy::existing()?;

        // Set up default matchers and sanitizers.
        proxy.initialize().await?;

        return Ok(proxy);
    }

    // Find root of git repo or work tree: a ".git" directory or file will exist either way.
    let git_dir = crate::find_ancestor_file(crate_dir.as_ref(), ".git")?;
    let git_dir = git_dir.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "parent git repository not found")
    })?;

    let executable_file_path = ensure_test_proxy(git_dir).await?;

    // Create an assets.json file in the crate_dir if a parent doesn't already exist.
    ensure_assets_file(test_mode, crate_dir).await?;

    // Construct the command line arguments and start the test-proxy service.
    let mut args: Vec<String> = Vec::new();
    args.extend_from_slice(&[
        "start".into(),
        "--storage-location".into(),
        git_dir
            .to_str()
            .ok_or_else(|| ErrorKind::Other.into_error())?
            .into(),
        // Write exceptions to stdout; otherwise,
        // reading from stderr on a separate thread hangs the process on Windows.
        "--universal".into(),
    ]);
    options.unwrap_or_default().copy_to(&mut args);
    tracing::debug!(
        "starting test-proxy with storage location {git_dir}",
        git_dir = git_dir.display()
    );

    let mut proxy = Proxy::default();
    let max_seconds = Duration::from_secs(env::var(SYSTEM_TEAMPROJECTID).map_or(5, |_| 20));
    tokio::select! {
        result = proxy.start(git_dir, &executable_file_path, args.into_iter()) => {
            result?;
            proxy.endpoint()
        }
        _ = tokio::time::sleep(max_seconds) => {
            proxy.stop().await?;
            return Err(azure_core::Error::with_message(ErrorKind::Other, "timed out waiting for test-proxy to start"));
        },
    };

    // Set up default matchers and sanitizers.
    proxy.initialize().await?;

    Ok(proxy)
}

async fn ensure_assets_file(
    test_mode: Option<TestMode>,
    crate_dir: impl AsRef<Path>,
) -> Result<()> {
    if test_mode == Some(TestMode::Record)
        && matches!(crate::find_ancestor_file(crate_dir.as_ref(), "assets.json"), Err(err) if err.kind() == &ErrorKind::Io)
    {
        let assets_file = crate_dir.as_ref().join("assets.json");
        tracing::debug!("creating {path}", path = assets_file.display());

        let assets_dir = assets_file
            .parent()
            .and_then(Path::file_name)
            .map(|dir| dir.to_ascii_lowercase())
            .ok_or_else(|| {
                azure_core::Error::with_message(
                    ErrorKind::Io,
                    "failed to get assets.json parent directory name",
                )
            })?;
        let assets_dir = assets_dir.to_string_lossy();
        let assets_content = json!({
            "AssetsRepo": "Azure/azure-sdk-assets",
            "AssetsRepoPrefixPath": "rust",
            "TagPrefix": format!("rust/{assets_dir}"),
            "Tag": "",
        });
        let file = File::create_new(assets_file).await?;
        return serde_json::to_writer_pretty(file.into_std().await, &assets_content)
            .map_err(azure_core::Error::from);
    }

    Ok(())
}

pub fn trace_line(level: Level, line: &str) {
    if !line.starts_with('[') {
        let line = line.trim();
        if line.is_empty() {
            return;
        }

        // tracing::*!() macros require constant Level, so we have to use a match here.
        match level {
            Level::ERROR => tracing::error!(target: "test-proxy", "{line}"),
            _ => tracing::trace!(target: "test-proxy", "{line}"),
        }
    }
}

async fn ensure_test_proxy(git_dir: &Path) -> Result<PathBuf> {
    let output_dir = git_dir.join(".proxy");
    let mut executable_file_path = output_dir.join("Azure.Sdk.Tools.TestProxy");
    if cfg!(windows) {
        let path = executable_file_path.as_mut_os_string();
        path.push(".exe");
    }

    // TODO: Create a lock file lock? https://github.com/Azure/azure-sdk-for-rust/issues/2299

    let required_version = required_proxy_version(git_dir).await?;
    if let Ok(output) = Command::new(&executable_file_path)
        .arg("--version")
        .output()
        .await
    {
        if let Ok(version) = String::from_utf8(output.stdout) {
            let installed_version = String::from("1.0.0-dev.") + version.trim();
            tracing::trace!("requires test-proxy {required_version}; found {installed_version}");

            if installed_version == required_version {
                return Ok(executable_file_path);
            }
        }
    }

    download_test_proxy(&required_version, &executable_file_path, &output_dir).await?;

    Ok(executable_file_path)
}

async fn required_proxy_version(git_dir: &Path) -> Result<String> {
    let mut path = git_dir.join("eng/target_proxy_version.txt");
    if !path.exists() {
        path = git_dir.join("eng/common/testproxy/target_version.txt");
    }

    Ok(tokio::fs::read_to_string(path).await?.trim().to_string())
}

fn download_file_name() -> Option<&'static str> {
    if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        Some("test-proxy-standalone-linux-x64.tar.gz")
    } else if cfg!(all(target_os = "linux", target_arch = "aarch64")) {
        Some("test-proxy-standalone-linux-arm64.tar.gz")
    } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
        Some("test-proxy-standalone-osx-x64.zip")
    } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        Some("test-proxy-standalone-osx-arm64.zip")
    } else if cfg!(all(target_os = "windows", not(target_arch = "x86"))) {
        Some("test-proxy-standalone-win-x64.zip")
    } else {
        None
    }
}

#[tracing::instrument(level = "debug", fields(url), err)]
async fn download_test_proxy(
    version: &str,
    executable_file_path: &Path,
    output_dir: &Path,
) -> Result<()> {
    let download_file_name = download_file_name().ok_or_else(|| {
        azure_core::Error::with_message(
            ErrorKind::Other,
            "test-proxy not supported on current platform",
        )
    })?;
    let url: url::Url = format!("https://github.com/Azure/azure-sdk-tools/releases/download/Azure.Sdk.Tools.TestProxy_{}/{}", version, download_file_name).parse()?;
    Span::current().record("url", url.as_str());

    let map_reqwest_err = |err: reqwest::Error| {
        let url = err.url().cloned().unwrap();
        azure_core::Error::with_error(ErrorKind::Other, err, format!("failed to download {url}"))
    };
    let archive = reqwest::get(url)
        .await
        .map_err(map_reqwest_err)?
        .bytes()
        .await
        .map_err(map_reqwest_err)?;
    let archive_file_path = output_dir.join(download_file_name);
    tokio::fs::create_dir_all(&output_dir).await?;

    // Using a std::fs::File to archives because I found no async zip implementation that instills a lot of confidence.
    // At this point we should be running in a separate task already, though, and its only done rarely during tests anyway.
    let mut archive_file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(&archive_file_path)?;
    archive_file.write_all(&archive)?;
    archive_file.flush()?;

    // Seek to the beginning and extract.
    archive_file.seek(io::SeekFrom::Start(0))?;
    extract_test_proxy(&archive_file, &archive_file_path, output_dir)?;

    #[cfg(unix)]
    {
        use std::{fs::Permissions, os::unix::fs::PermissionsExt as _};
        tokio::fs::set_permissions(executable_file_path, Permissions::from_mode(0o755)).await?;
    }

    Ok(())
}

fn extract_test_proxy(
    archive_file: &std::fs::File,
    archive_file_path: &Path,
    output_dir: &Path,
) -> Result<()> {
    tracing::trace!(
        "extracting {} to {}",
        archive_file_path.display(),
        output_dir.display()
    );
    match archive_file_path.extension() {
        Some(ext) if ext == "gz" => untar(archive_file, output_dir),
        Some(ext) if ext == "zip" => unzip(archive_file, output_dir),
        _ => {
            return Err(azure_core::Error::with_message(
                ErrorKind::Io,
                format!("unsupported archive {}", archive_file_path.display()),
            ))
        }
    }
    .map_err(|err| {
        let message = format!(
            "failed to extract {}: {:?}",
            archive_file_path.display(),
            &err
        );
        azure_core::Error::with_error(ErrorKind::Io, err, message)
    })
}

// cspell:ignore bufread untar
fn untar(
    archive_file: &std::fs::File,
    output_dir: &Path,
) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let reader = std::io::BufReader::new(archive_file);
    let decoder = flate2::bufread::GzDecoder::new(reader);
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(output_dir)?;

    Ok(())
}

fn unzip(
    archive_file: &std::fs::File,
    output_dir: &Path,
) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut zip = zip::ZipArchive::new(archive_file)?;
    zip.extract(output_dir)?;

    Ok(())
}
