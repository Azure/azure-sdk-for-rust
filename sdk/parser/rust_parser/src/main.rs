extern crate syn;
use syn::{File, Item};
use std::fs;

fn parse_rust_file(file_path: &str) -> File {
    let content = fs::read_to_string(file_path).expect("Unable to read file");
    syn::parse_file(&content).expect("Unable to parse file")
}

fn main() {
    let ast = parse_rust_file("/workspaces/azure-sdk-for-rust/sdk/temp-project/docs/src/lib.rs");
    process_ast(&ast);
}
// Example of processing the AST
fn process_ast(ast: &File) {
    for item in &ast.items {
        match item {
            Item::Struct(struct_item) => {
                println!("Found struct: {}", struct_item.ident);
            }
            Item::Enum(enum_item) => {
                println!("Found enum: {}", enum_item.ident);
            }
            Item::Fn(fn_item) => {
                println!("Found function: {}", fn_item.sig.ident);
            }
            _ => {}
        }
    }
}
