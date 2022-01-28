// cargo run --example list_crates

// This list all crates that are generated.
// If a version is passed in as an option, it will filter out any that
// are already published to crates.io.
// An updated clone of crates.io-index is required.
// git clone https://github.com/rust-lang/crates.io-index

use camino::{Utf8Path, Utf8PathBuf};
use std::fs;
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

fn main() -> Result<()> {
    let version = std::env::args().nth(1);

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
    for name in names {
        println!("{}", name);
    }

    Ok(())
}
