// cargo run --example strip_bom -- ../../../vsts-rest-api-specs/specification
// Specification are not supposed to have byte order marks.
// This removes the byte order marks.

use std::{
    io::Write,
    path::{Path, PathBuf},
};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let dir = std::env::args().nth(1).expect("please specify root directory");
    // for_each_json(dir, |path: PathBuf| Ok(println!("{:?}", path)))?;
    for_each_json(dir, strip_bom)?;
    Ok(())
}

fn for_each_json<P, F>(dir: P, func: F) -> Result<()>
where
    P: AsRef<Path>,
    F: Fn(PathBuf) -> Result<()>,
{
    let paths = walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| match entry {
            Ok(entry) => match entry.file_name().to_str() {
                Some(path) => {
                    if path.ends_with(".json") {
                        Some(entry)
                    } else {
                        None
                    }
                }
                None => None,
            },
            Err(_) => None,
        })
        .map(|entry| entry.into_path());

    for path in paths {
        func(path)?
    }
    Ok(())
}

fn strip_bom<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut file = std::fs::File::open(&path)?;
    let bom = unicode_bom::Bom::from(&mut file);
    match bom {
        unicode_bom::Bom::Null => (),
        _ => {
            let bytes = std::fs::read(&path)?;
            if let Some(bytes) = bytes.as_slice().get(bom.len()..) {
                let mut file = std::fs::File::create(&path)?;
                file.write_all(bytes)?;
            }
        }
    }
    Ok(())
}
