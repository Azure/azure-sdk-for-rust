// cargo run --example list_crates

use std::fs;
use camino::Utf8Path;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let paths = fs::read_dir("../svc")?;
    for path in paths {
        match path {
            Ok(path) => {
                // path.metadata().
                match Utf8Path::from_path(&path.path()){
                    Some(path) => {
                        println!("path {}", path);
                    },
                    None => (),
                }
            },
            Err(_) => todo!(),
        }
    }

    Ok(())
}
