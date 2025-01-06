extern crate syn;
extern crate quote;
extern crate serde;
extern crate serde_json;
extern crate glob;

use quote::ToTokens;
use syn::{File, Item};
use serde::Serialize;
use std::fs::{self, File as StdFile};
use std::io::Write;
use glob::glob;

#[derive(Serialize)]
struct SerializableFile {
    items: Vec<SerializableItem>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum SerializableItem {
    Struct(SerializableStruct),
    Enum(SerializableEnum),
    Function(SerializableFunction),
    Other(SerializableOther),
}

#[derive(Serialize)]
struct SerializableStruct {
    ident: String,
    fields: Vec<String>,
}

#[derive(Serialize)]
struct SerializableEnum {
    ident: String,
    variants: Vec<String>,
}

#[derive(Serialize)]
struct SerializableFunction {
    ident: String,
    inputs: Vec<String>,
    output: Option<String>,
}
#[derive(Serialize)]
struct SerializableOther {
    kind: String,
    content: String,
}
fn parse_rust_file(file_path: &str) -> File {
    let content = fs::read_to_string(file_path).expect("Unable to read file");
    syn::parse_file(&content).expect("Unable to parse file")
}

fn main() {
    let mut all_items = Vec::new();

    for entry in glob("/workspaces/azure-sdk-for-rust/sdk/core/azure_core/src/**/*.rs").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let ast = parse_rust_file(path.to_str().unwrap());
                all_items.extend(ast.items);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    process_ast(&all_items);
    write_ast_to_json(&all_items, "ast_core.json");
}

fn process_ast(items: &[Item]) {
    for item in items {
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

fn write_ast_to_json(items: &[Item], output_path: &str) {
    let serializable_file = SerializableFile {
        items: items.iter().map(|item| {
            match item {
                Item::Struct(struct_item) => SerializableItem::Struct(SerializableStruct {
                    ident: struct_item.ident.to_string(),
                    fields: struct_item.fields.to_token_stream().to_string().split_whitespace().map(|s| s.to_string()).collect(),
                }),
                Item::Enum(enum_item) => SerializableItem::Enum(SerializableEnum {
                    ident: enum_item.ident.to_string(),
                    variants: enum_item.variants.iter().map(|v| v.ident.to_string()).collect(),
                }),
                Item::Fn(fn_item) => SerializableItem::Function(SerializableFunction {
                    ident: fn_item.sig.ident.to_string(),
                    inputs: fn_item.sig.inputs.iter().map(|i| match i {
                        syn::FnArg::Receiver(_) => "self".to_string(),
                        syn::FnArg::Typed(pat_type) => format!("{}: {}", quote::quote!(#pat_type.pat), quote::quote!(#pat_type.ty)),
                    }).collect(),
                    output: match &fn_item.sig.output {
                        syn::ReturnType::Default => None,
                        syn::ReturnType::Type(_, ty) => Some(quote::quote!(#ty).to_string()),
                    },
                }),
                _ => SerializableItem::Other(SerializableOther {
                    kind: "unknown".to_string(),
                    content: item.to_token_stream().to_string(),
                }),
            }
        }).collect(),
    };

    let serialized_ast = serde_json::to_string_pretty(&serializable_file).expect("Failed to serialize AST");
    let mut file = StdFile::create(output_path).expect("Unable to create file");
    file.write_all(serialized_ast.as_bytes()).expect("Unable to write data");
}
