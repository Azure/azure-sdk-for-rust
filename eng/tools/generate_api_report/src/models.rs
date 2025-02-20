//! Minimal version of the `rustdoc` models from https://github.com/rust-lang/rust/blob/fb65a3ee576feab95a632eb062f466d7a0342310/src/rustdoc-json-types/lib.rs
//! pub const FORMAT_VERSION: u32 = 37;
//! If the format version changes, this code might need to be updated.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Crate {
    pub root: u32,
    pub crate_version: Option<String>,
    pub index: HashMap<String, Item>,
    // pub paths: HashMap<String, Value>,
    // pub external_crates: Value,
    pub format_version: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub crate_id: u32,
    pub name: Option<String>,
    // pub span: Option<Value>,
    pub visibility: Value,
    pub docs: Option<String>,
    pub links: HashMap<String, Value>,
    pub attrs: Vec<String>,
    pub deprecation: Value,
    pub inner: Value,
}
