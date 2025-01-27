use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub crate_id: u32,
    pub name: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub span: Option<Value>,
    pub visibility: Value,
    pub docs: Option<String>,
    pub links: HashMap<String, Value>,
    pub attrs: Vec<String>,
    pub deprecation: Value,
    pub inner: Value,
}

#[derive(Serialize, Deserialize)]
pub struct Crate {
    pub root: u32,
    pub crate_version: Option<String>,
    pub index: HashMap<String, Item>,
    // pub paths: HashMap<String, Value>,
    // pub external_crates: Value,
    pub format_version: u32
}
