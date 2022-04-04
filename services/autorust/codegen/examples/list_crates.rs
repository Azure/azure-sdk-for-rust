// cargo run --example list_crates
// This list all crates that are generated.

// cargo run --example list_crates -- 0.1.0
// If a version is passed in as an option, it will filter out any that
// are already published to crates.io.
// An updated clone of crates.io-index is required.
// git clone https://github.com/rust-lang/crates.io-index

use autorust_codegen::github_yml::CheckAllServicesYml;
use autorust_codegen::github_yml::PublishServicesYml;
use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;
use std::io::BufRead;
use std::{
    fs::{self, File},
    io::BufReader,
    str::FromStr,
};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Get all directories below the given directory.
fn get_dirs(dir: impl AsRef<Utf8Path>) -> Result<Vec<Utf8PathBuf>> {
    let mut dirs = Vec::new();
    let paths = fs::read_dir(dir.as_ref())?;
    for path in paths {
        match path {
            Ok(path) => match Utf8Path::from_path(&path.path()) {
                Some(path) => {
                    if path.is_dir() {
                        dirs.push(path.to_path_buf());
                    }
                }
                None => (),
            },
            Err(_) => (),
        }
    }
    Ok(dirs)
}

fn list_crate_names() -> Result<Vec<String>> {
    let mut names: Vec<_> = get_dirs("../mgmt")?
        .into_iter()
        .filter_map(|d| d.file_name().map(|d| format!("azure_mgmt_{}", d)))
        .collect();
    names.extend(
        get_dirs("../svc")?
            .into_iter()
            .filter_map(|d| d.file_name().map(|d| format!("azure_svc_{}", d))),
    );
    names.sort();
    Ok(names)
}

fn main() -> Result<()> {
    let version = std::env::args().nth(1);

    let names = list_crate_names()?;
    // match &version {
    //     Some(version) => {
    //         for name in names.iter() {
    //             if !has_version(name, version)? {
    //                 println!("{}", name);
    //                 // println!("cargo publish -p {}", name);
    //                 // println!("Start-Sleep -Seconds 420");
    //                 // println!("cargo owner --add github:Azure:azure-sdk-publish-rust -- {}", name);
    //             }
    //         }
    //     }
    //     None => {
    //         for (i, name) in names.iter().enumerate() {
    //             println!("{} {}", i + 1, name);
    //         }
    //     }
    // }

    // let yml = PublishServicesYml {
    //     packages: &names.iter().map(String::as_str).collect(),
    // };
    // yml.create("../../.github/workflows/publish-services.yml")?;

    let yml = CheckAllServicesYml {
        packages: &names.iter().map(String::as_str).collect(),
    };
    yml.create("../../.github/workflows/check-all-services.yml")?;

    Ok(())
}

fn has_version(name: &str, version: &str) -> Result<bool> {
    Ok(get_versions(name)?.iter().any(|v| v.vers.as_str() == version))
}

fn get_versions(name: &str) -> Result<Vec<CrateVersion>> {
    let path = format!("../../../crates.io-index/az/ur/{}", name);
    let path = Utf8PathBuf::from_str(&path)?;
    let mut versions = Vec::new();
    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let version: CrateVersion = serde_json::from_str(&line?)?;
            versions.push(version);
        }
    }
    Ok(versions)
}

#[derive(Debug, Deserialize)]
pub struct CrateVersion {
    pub name: String,
    pub vers: String,
}
