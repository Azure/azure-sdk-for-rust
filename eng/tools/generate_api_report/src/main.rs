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

    // Prettifying the JSON
    // Parse the JSON to ensure it's valid
    let json_value: serde_json::Value = serde_json::from_str(&contents)?;

    // Write the pretty-printed JSON back to the file
    let mut file = File::create(path)?;
    serde_json::to_writer_pretty(&mut file, &json_value)?;

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
    
    // Navigate to Cargo.toml and get the path for the package
    let cargo_toml_path = Path::new("Cargo.toml");
    let cargo_toml_content = std::fs::read_to_string(cargo_toml_path)?;
    let cargo_toml: toml::Value = toml::from_str(&cargo_toml_content)?;

    let package_path = cargo_toml
        .get("workspace")
        .and_then(|ws| ws.get("members"))
        .and_then(|members| members.as_array())
        .and_then(|members| {
            members.iter().find_map(|member| {
                if member.as_str()?.ends_with(package_name) {
                    Some(member.as_str()?.to_string())
                } else {
                    None
                }
            })
        })
        .ok_or("Package path not found in Cargo.toml")?;

    // Create the review/ folder under the obtained path if it doesn't exist
    let review_folder_path = Path::new(&package_path).join("review");
    if !review_folder_path.exists() {
        std::fs::create_dir_all(&review_folder_path)?;
    }

    // Create the package_name.rust.json in the review/ folder
    let output_path_str = review_folder_path.join(format!("{}.rust.json", package_name));
    let output_path = Path::new(&output_path_str);
    let mut output_file = File::create(output_path)?;
    serde_json::to_writer_pretty(&mut output_file, &root)?;

    println!("File has been generated at: {}", output_path_str.display());

    Ok(())
}
