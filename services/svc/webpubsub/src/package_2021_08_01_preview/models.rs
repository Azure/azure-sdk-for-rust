#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientTokenResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl ClientTokenResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
