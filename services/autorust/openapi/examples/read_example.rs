use autorust_openapi::example::Example;
use std::{fs, process::exit};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

// cargo run --example read_example -- ../azure-rest-api-specs/specification/vmware/resource-manager/Microsoft.AVS/stable/2020-03-20/examples/PrivateClouds_List.json
fn main() -> Result<()> {
    match std::env::args().nth(1) {
        None => {
            eprintln!("Please pass in the spec path.");
            exit(1);
        }
        Some(path) => {
            let bytes = fs::read(path)?;
            let example: Example = serde_json::from_slice(&bytes)?;
            println!("# of parameters: {}", example.parameters.len());
            println!("# of responses: {}", example.responses.len());
            for (code, response) in &example.responses {
                match (code, &response.body) {
                    (code, Some(body)) => {
                        let body = serde_json::to_vec(body)?;
                        println!("  {}: {} bytes", code, body.len());
                    }
                    (code, None) => println!("  {}: 0 bytes", code),
                }
            }
        }
    }
    Ok(())
}
