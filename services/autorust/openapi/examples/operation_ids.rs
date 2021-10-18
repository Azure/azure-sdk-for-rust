// Print the operation IDs alphabetically
// cargo run --example operation_ids -- ../azure-rest-api-specs/specification/vmware/resource-manager/Microsoft.AVS/stable/2020-03-20/vmware.json

use autorust_openapi::*;
use std::{
    fs::{self},
    path::Path,
    process::exit,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match std::env::args().nth(1) {
        None => {
            eprintln!("Please pass in the spec path.");
            exit(1);
        }
        Some(file_in) => {
            let file_in = Path::new(&file_in);
            let bytes = fs::read(file_in)?;
            let api: OpenAPI = serde_json::from_slice(&bytes)?;

            let mut operation_ids = Vec::new();
            for (_path, item) in api.paths() {
                match item {
                    ReferenceOr::Reference { .. } => (),
                    ReferenceOr::Item(item) => {
                        for op in item.operations() {
                            if let Some(operation_id) = &op.operation_id {
                                operation_ids.push(operation_id);
                            }
                        }
                    }
                }
            }

            operation_ids.sort();
            for operation_id in operation_ids {
                println!("{}", operation_id);
            }
        }
    }
    Ok(())
}
