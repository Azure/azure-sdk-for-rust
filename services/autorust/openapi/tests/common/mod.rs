use assert_json_diff::assert_json_eq;
use autorust_openapi::OpenAPI;
use serde_json::Value;
use std::{fs, path::Path};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn assert_roundtrip_eq<P: AsRef<Path> + std::fmt::Debug>(paths: &[P]) -> Result<()> {
    for path in paths {
        println!("  test {:?}", path);
        let bytes = fs::read(path)?;
        let spec: OpenAPI = serde_json::from_slice(&bytes)?;
        let a = serde_json::to_value(spec)?;
        let b: Value = serde_json::from_slice(&bytes)?;
        assert_json_eq!(a, b);
    }
    Ok(())
}
