// cargo run --example list_crates

use std::fs;
use camino::Utf8Path;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut names = Vec::new();
    let paths = fs::read_dir("../mgmt")?;
    for path in paths {
        match path {
            Ok(path) => {
                match Utf8Path::from_path(&path.path()){
                    Some(path) => {
                        if path.is_dir() {
                            match path.file_name() {
                                Some(file_name) => {
                                    // println!("path {}", file_name);
                                    names.push(file_name.to_string())
                                },
                                None => (),
                            }
                        }
                    },
                    None => (),
                }
            },
            Err(_) => todo!(),
        }
    }

    names.sort();
    for name in names {
        println!("cargo publish -p azure_mgmt_{}", name);
    }

    Ok(())
}
