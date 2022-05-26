// cargo run --example list_crates
// This list all crates that are generated.

// cargo run --example list_crates -- 0.1.0
// If a version is passed in as an option, it will filter out any that
// are already published to crates.io.
// An updated clone of crates.io-index is required.
// git clone https://github.com/rust-lang/crates.io-index

use autorust_codegen::crates::has_version;
use autorust_codegen::crates::list_crate_names;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let version = std::env::args().nth(1);
    let names = list_crate_names()?;
    match &version {
        Some(version) => {
            for name in names.iter() {
                if !has_version(name, version)? {
                    println!("{}", name);
                    // println!("cargo owner --add github:Azure:azure-sdk-publish-rust -- {}", name);
                }
            }
        }
        None => {
            for (i, name) in names.iter().enumerate() {
                println!("{} {}", i + 1, name);
            }
        }
    }
    Ok(())
}
