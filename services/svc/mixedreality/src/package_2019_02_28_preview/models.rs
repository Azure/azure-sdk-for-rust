#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents a token response message from the STS service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StsTokenResponseMessage {
    #[doc = "An access token for the account."]
    #[serde(rename = "AccessToken")]
    pub access_token: String,
}
impl StsTokenResponseMessage {
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }
}
