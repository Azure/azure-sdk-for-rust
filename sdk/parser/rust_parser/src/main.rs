extern crate syn;
extern crate quote;
extern crate serde;
extern crate serde_json;
extern crate glob;

use quote::ToTokens;
use syn::{File, Item, Attribute, Visibility, Generics, Type, Expr, Ident};
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
    Const(SerializableConst),
    Enum(SerializableEnum),
    ExternCrate(SerializableExternCrate),
    Fn(SerializableFunction),
    ForeignMod(SerializableForeignMod),
    Impl(SerializableImpl),
    Macro(SerializableMacro),
    Mod(SerializableMod),
    Static(SerializableStatic),
    Struct(SerializableStruct),
    Trait(SerializableTrait),
    TraitAlias(SerializableTraitAlias),
    Type(SerializableType),
    Union(SerializableUnion),
    Use(SerializableUse),
    Verbatim(SerializableVerbatim),
    Other(SerializableOther),
}

#[derive(Serialize)]
struct SerializableConst {
    attrs: Vec<String>,
    vis: String,
    ident: String,
    generics: String,
    ty: String,
    expr: String,
}

#[derive(Serialize)]
struct SerializableEnum {
    attrs: Vec<String>,
    vis: String,
    ident: String,
    variants: Vec<String>,
}

#[derive(Serialize)]
struct SerializableExternCrate {
    attrs: Vec<String>,
    vis: String,
    ident: String,
}

#[derive(Serialize)]
struct SerializableFunction {
    attrs: Vec<String>,
    vis: String,
    ident: String,
    inputs: Vec<String>,
    output: Option<String>,
    block: String,
}

#[derive(Serialize)]
struct SerializableForeignMod {
    attrs: Vec<String>,
    abi: String,
    items: Vec<String>,
}

#[derive(Serialize)]
struct SerializableImpl {
    attrs: Vec<String>,
    trait_: Option<String>,
    self_ty: String,
    items: Vec<String>,
}

#[derive(Serialize)]
struct SerializableMacro {
    attrs: Vec<String>,
    path: Option<String>,
    tokens: String,
}

#[derive(Serialize)]
struct SerializableMod {
    attrs: Vec<String>,
    vis: String,
    ident: String,
    content: Option<Vec<String>>,
}

#[derive(Serialize)]
struct SerializableStatic {
    attrs: Vec<String>,
    vis: String,
    ident: String,
    ty: String,
    expr: String,
}

#[derive(Serialize)]
struct SerializableStruct {
    attrs: Vec<String>,
    vis: String,
    ident: String,
    fields: Vec<String>,
}

#[derive(Serialize)]
struct SerializableTrait {
    attrs: Vec<String>,
    vis: String,
    ident: String,
    items: Vec<String>,
}

#[derive(Serialize)]
struct SerializableTraitAlias {
    attrs: Vec<String>,
    vis: String,
    ident: String,
    bounds: Vec<String>,
}

#[derive(Serialize)]
struct SerializableType {
    attrs: Vec<String>,
    vis: String,
    ident: String,
    ty: String,
}

#[derive(Serialize)]
struct SerializableUnion {
    attrs: Vec<String>,
    vis: String,
    ident: String,
    fields: Vec<String>,
}

#[derive(Serialize)]
struct SerializableUse {
    attrs: Vec<String>,
    tree: String,
}

#[derive(Serialize)]
struct SerializableVerbatim {
    tokens: String,
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
                Item::Const(item_const) => SerializableItem::Const(SerializableConst {
                    attrs: item_const.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect(),
                    vis: item_const.vis.to_token_stream().to_string(),
                    ident: item_const.ident.to_string(),
                    generics: item_const.generics.to_token_stream().to_string(),
                    ty: item_const.ty.to_token_stream().to_string(),
                    expr: item_const.expr.to_token_stream().to_string(),
                }),
                Item::Enum(item_enum) => SerializableItem::Enum(SerializableEnum {
                    attrs: item_enum.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect(),
                    vis: item_enum.vis.to_token_stream().to_string(),
                    ident: item_enum.ident.to_string(),
                    variants: item_enum.variants.iter().map(|v| v.ident.to_string()).collect(),
                }),
                Item::ExternCrate(item_extern_crate) => SerializableItem::ExternCrate(SerializableExternCrate {
                    attrs: item_extern_crate.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect(),
                    vis: item_extern_crate.vis.to_token_stream().to_string(),
                    ident: item_extern_crate.ident.to_string(),
                }),
                Item::Fn(item_fn) => SerializableItem::Fn(SerializableFunction {
                    attrs: item_fn.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect(),
                    vis: item_fn.vis.to_token_stream().to_string(),
                    ident: item_fn.sig.ident.to_string(),
                    inputs: item_fn.sig.inputs.iter().map(|i| match i {
                        syn::FnArg::Receiver(_) => "self".to_string(),
                        syn::FnArg::Typed(pat_type) => format!("{}: {}", quote::quote!(#pat_type.pat), quote::quote!(#pat_type.ty)),
                    }).collect(),
                    output: match &item_fn.sig.output {
                        syn::ReturnType::Default => None,
                        syn::ReturnType::Type(_, ty) => Some(quote::quote!(#ty).to_string()),
                    },
                    block: item_fn.block.to_token_stream().to_string(),
                }),
                Item::ForeignMod(item_foreign_mod) => SerializableItem::ForeignMod(SerializableForeignMod {
                    attrs: item_foreign_mod.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect(),
                    abi: item_foreign_mod.abi.name.as_ref().map_or_else(|| "".to_string(), |lit_str| lit_str.value()),
                    items: item_foreign_mod.items.iter().map(|i| i.to_token_stream().to_string()).collect(),
                }),
                // Item::Impl(item_impl) => SerializableItem::Impl(SerializableImpl {
                //     attrs: item_impl.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect(),
                //     trait_: item_impl.trait_.as_ref().map(|t| quote::quote!(#t).to_string()),
                //     self_ty: quote::quote!(#item_impl.self_ty).to_string(),
                //     items: item_impl.items.iter().map(|i| i.to_token_stream().to_string()).collect(),
                // }),
                // Item::Macro(item_macro) => SerializableItem::Macro(SerializableMacro {
                //     attrs: item_macro.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect(),
                //     path: item_macro.path.as_ref().map(|p| p.to_token_stream().to_string()),
                //     tokens: item_macro.mac.tokens.to_string(),
                // }),
                Item::Mod(item_mod) => SerializableItem::Mod(SerializableMod {
                    attrs: item_mod.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect(),
                    vis: item_mod.vis.to_token_stream().to_string(),
                    ident: item_mod.ident.to_string(),
                    content: item_mod.content.as_ref().map(|(_, items)| items.iter().map(|i| i.to_token_stream().to_string()).collect()),
                }),
                Item::Static(item_static) => SerializableItem::Static(SerializableStatic {
                    attrs: item_static.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect(),
                    vis: item_static.vis.to_token_stream().to_string(),
                    ident: item_static.ident.to_string(),
                    ty: quote::quote!(#item_static.ty).to_string(),
                    expr: item_static.expr.to_token_stream().to_string(),
                }),
                Item::Struct(item_struct) => SerializableItem::Struct(SerializableStruct {
                    attrs: item_struct.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect(),
                    vis: item_struct.vis.to_token_stream().to_string(),
                    ident: item_struct.ident.to_string(),
                    fields: item_struct.fields.to_token_stream().to_string().split_whitespace().map(|s| s.to_string()).collect(),
                }),
                Item::Trait(item_trait) => SerializableItem::Trait(SerializableTrait {
                    attrs: item_trait.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect(),
                    vis: item_trait.vis.to_token_stream().to_string(),
                    ident: item_trait.ident.to_string(),
                    items: item_trait.items.iter().map(|i| i.to_token_stream().to_string()).collect(),
                }),
                Item::TraitAlias(item_trait_alias) => SerializableItem::TraitAlias(SerializableTraitAlias {
                    attrs: item_trait_alias.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect(),
                    vis: item_trait_alias.vis.to_token_stream().to_string(),
                    ident: item_trait_alias.ident.to_string(),
                    bounds: item_trait_alias.bounds.to_token_stream().to_string().split_whitespace().map(|s| s.to_string()).collect(),
                }),
                Item::Type(item_type) => SerializableItem::Type(SerializableType {
                    attrs: item_type.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect(),
                    vis: item_type.vis.to_token_stream().to_string(),
                    ident: item_type.ident.to_string(),
                    ty: quote::quote!(#item_type.ty).to_string(),
                }),
                Item::Union(item_union) => SerializableItem::Union(SerializableUnion {
                    attrs: item_union.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect(),
                    vis: item_union.vis.to_token_stream().to_string(),
                    ident: item_union.ident.to_string(),
                    fields: item_union.fields.to_token_stream().to_string().split_whitespace().map(|s| s.to_string()).collect(),
                }),
                Item::Use(item_use) => SerializableItem::Use(SerializableUse {
                    attrs: item_use.attrs.iter().map(|attr| attr.to_token_stream().to_string()).collect(),
                    tree: item_use.tree.to_token_stream().to_string(),
                }),
                Item::Verbatim(item_verbatim) => SerializableItem::Verbatim(SerializableVerbatim {
                    tokens: item_verbatim.to_string(),
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
