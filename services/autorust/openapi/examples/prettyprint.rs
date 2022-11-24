use autorust_openapi::OpenAPI;
use camino::{Utf8Path, Utf8PathBuf};
use std::{
    fs::{self, create_dir_all, File},
    io::Write,
    process::exit,
};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// Pass in a output directory as the second parameter if you want to write the file instead of printing it to stdout.
// cargo run --example prettyprint -- ../azure-rest-api-specs/specification/security/resource-manager/common/v1/types.json
// cargo run --example prettyprint -- ../OpenAPI-Specification/examples/v2.0/json/petstore.json tmp
fn main() -> Result<()> {
    match std::env::args().nth(1) {
        None => {
            eprintln!("Please pass in the spec path.");
            exit(1);
        }
        Some(file_in) => {
            let file_in = Utf8Path::new(&file_in);
            // reading the whole file upfront is much faster than using a BufReader
            // https://github.com/serde-rs/json/issues/160
            let bytes = fs::read(file_in)?;
            let spec: OpenAPI = serde_json::from_slice(&bytes)?;
            let json = serde_json::to_string_pretty(&spec)?;

            match std::env::args().nth(2) {
                Some(dir_out) => {
                    create_dir_all(&dir_out)?;
                    let mut file_out = Utf8PathBuf::new();
                    file_out.push(&dir_out);
                    file_out.push(file_in.file_name().unwrap());
                    let mut file = File::create(file_out)?;
                    file.write_all(json.as_bytes())?;
                }
                None => {
                    println!("{}", json);
                }
            }
        }
    }
    Ok(())
}
