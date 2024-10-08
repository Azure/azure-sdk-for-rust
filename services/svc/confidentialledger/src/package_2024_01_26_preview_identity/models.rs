#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An error response from Confidential Ledger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfidentialLedgerError {
    #[doc = "An error response from Confidential Ledger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ConfidentialLedgerErrorBody>,
}
impl ConfidentialLedgerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from Confidential Ledger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfidentialLedgerErrorBody {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ConfidentialLedgerErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the information about a Confidential Ledger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerIdentityInformation {
    #[doc = "Id for the ledger."]
    #[serde(rename = "ledgerId", default, skip_serializing_if = "Option::is_none")]
    pub ledger_id: Option<String>,
    #[doc = "PEM-encoded certificate used for TLS by the Confidential Ledger."]
    #[serde(rename = "ledgerTlsCertificate")]
    pub ledger_tls_certificate: String,
}
impl LedgerIdentityInformation {
    pub fn new(ledger_tls_certificate: String) -> Self {
        Self {
            ledger_id: None,
            ledger_tls_certificate,
        }
    }
}
