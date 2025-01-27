use crate::models::Crate;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

mod models;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the package name from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "--package" {
        eprintln!("Usage: {} --package <package_name>", args[0]);
        std::process::exit(1);
    }
    let package_name = &args[2];
    let path_str = format!("./target/doc/{}.json", package_name);
    let path = Path::new(&path_str);

    // Call cargo +nightly rustdoc to generate the JSON file
    let output = Command::new("cargo")
        .arg("+nightly")
        .arg("rustdoc")
        .arg("-Z")
        .arg("unstable-options")
        .arg("--output-format")
        .arg("json")
        .arg("--package")
        .arg(package_name)
        .arg("--all-features")
        .output()?;

    if !output.status.success() {
        eprintln!(
            "Failed to generate JSON file: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // -------- Prettifying package_name.json - starts -----
    // Parse the JSON to ensure it's valid
    let json_value: serde_json::Value = serde_json::from_str(&contents)?;

    // Write the pretty-printed JSON back to the file
    let mut file = File::create(path)?;
    serde_json::to_writer_pretty(&mut file, &json_value)?;
    // -------- Prettifying package_name.json - ends -----

    let mut root: Crate = serde_json::from_str(&contents)?;

    // Remove items
    // 1. with item.inner.impl.is_synthetic set to true - [auto traits]
    // 2. with item.inner.impl.blanket_impl is not null - [blanket impls]
    root.index.retain(|_id, item| {
        if let Some(inner_impl) = item.inner.get("impl") {
            if let Some(is_synthetic) = inner_impl.get("is_synthetic") {
                if is_synthetic.as_bool().unwrap_or(false) {
                    return false;
                }
            }
            if let Some(blanket_impl) = inner_impl.get("blanket_impl") {
                if !blanket_impl.is_null() {
                    return false;
                }
            }
        }
        true
    });

    let output_path_str = format!("./target/doc/{}.rust.json", package_name);
    let output_path = Path::new(&output_path_str);
    let mut output_file = File::create(output_path)?;
    serde_json::to_writer_pretty(&mut output_file, &root)?;

    println!("File has been generated at: {}", output_path_str);

    Ok(())
}
