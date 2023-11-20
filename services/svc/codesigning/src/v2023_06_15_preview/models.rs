#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreFoundationsError {
    #[doc = "One of a server-defined set of error codes."]
    pub code: String,
    #[doc = "A human-readable representation of the error."]
    pub message: String,
    #[doc = "The target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "An array of details about specific errors that led to this reported error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<AzureCoreFoundationsError>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<AzureCoreFoundationsInnerError>,
}
impl AzureCoreFoundationsError {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
            innererror: None,
        }
    }
}
#[doc = "A response containing error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreFoundationsErrorResponse {
    #[doc = "The error object."]
    pub error: AzureCoreFoundationsError,
}
impl azure_core::Continuable for AzureCoreFoundationsErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AzureCoreFoundationsErrorResponse {
    pub fn new(error: AzureCoreFoundationsError) -> Self {
        Self { error }
    }
}
#[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCoreFoundationsInnerError {
    #[doc = "One of a server-defined set of error codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<Box<AzureCoreFoundationsInnerError>>,
}
impl AzureCoreFoundationsInnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enum describing allowed operation states."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureCoreFoundationsOperationState")]
pub enum AzureCoreFoundationsOperationState {
    InProgress,
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureCoreFoundationsOperationState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureCoreFoundationsOperationState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureCoreFoundationsOperationState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::InProgress => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 0u32, "InProgress"),
            Self::Succeeded => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 1u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 2u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 3u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Azure CodeSigning certificate profile configured on a codesign account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateProfile {
    #[doc = "Azure Codesigning certificate profile name under a codesign account."]
    #[serde(rename = "certificateProfileName")]
    pub certificate_profile_name: String,
}
impl CertificateProfile {
    pub fn new(certificate_profile_name: String) -> Self {
        Self { certificate_profile_name }
    }
}
#[doc = "The codesign operation status response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeSignOperationStatus {
    #[doc = "Unique Id of the operation."]
    pub id: String,
    #[doc = "The sign status model."]
    #[serde(rename = "signResult", default, skip_serializing_if = "Option::is_none")]
    pub sign_result: Option<CodeSignResult>,
}
impl CodeSignOperationStatus {
    pub fn new(id: String) -> Self {
        Self { id, sign_result: None }
    }
}
#[doc = "The sign status model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeSignResult {
    #[doc = "Response Id of the codesign operation."]
    #[serde(rename = "operationId")]
    pub operation_id: String,
    #[doc = "Digital signature of the requested content digest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[doc = "Signing certificate corresponding to the private key used to codesign the requested\ndigest."]
    #[serde(rename = "signingCertificate", default, skip_serializing_if = "Option::is_none")]
    pub signing_certificate: Option<String>,
}
impl CodeSignResult {
    pub fn new(operation_id: String) -> Self {
        Self {
            operation_id,
            signature: None,
            signing_certificate: None,
        }
    }
}
#[doc = "Azure CodeSigning customer account created upon registration on the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeSigningAccount {
    #[doc = "Azure CodeSigning account name."]
    #[serde(rename = "codeSigningAccountName")]
    pub code_signing_account_name: String,
}
impl CodeSigningAccount {
    pub fn new(code_signing_account_name: String) -> Self {
        Self { code_signing_account_name }
    }
}
#[doc = "The codesign request information to be signed by the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeSigningSubmissionOptions {
    #[doc = "Algorithms supported for signing."]
    #[serde(rename = "signatureAlgorithm")]
    pub signature_algorithm: SignatureAlgorithm,
    #[doc = "Content digest to codesign."]
    pub digest: String,
    #[doc = "List of full file digital signatures."]
    #[serde(
        rename = "fileHashList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub file_hash_list: Vec<String>,
    #[doc = "List of authenticode digital signatures."]
    #[serde(
        rename = "authenticodeHashList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub authenticode_hash_list: Vec<String>,
}
impl CodeSigningSubmissionOptions {
    pub fn new(signature_algorithm: SignatureAlgorithm, digest: String) -> Self {
        Self {
            signature_algorithm,
            digest,
            file_hash_list: Vec::new(),
            authenticode_hash_list: Vec::new(),
        }
    }
}
#[doc = "Extended key usage object identifier that are allowed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedKeyUsage {
    #[doc = "An element of ekus."]
    pub ekus: Vec<String>,
}
impl ExtendedKeyUsage {
    pub fn new(ekus: Vec<String>) -> Self {
        Self { ekus }
    }
}
#[doc = "Paged collection of ExtendedKeyUsage items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedExtendedKeyUsage {
    #[doc = "The ExtendedKeyUsage items on this page"]
    pub value: Vec<ExtendedKeyUsage>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedExtendedKeyUsage {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedExtendedKeyUsage {
    pub fn new(value: Vec<ExtendedKeyUsage>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Algorithms supported for signing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SignatureAlgorithm")]
pub enum SignatureAlgorithm {
    #[serde(rename = "RS256")]
    Rs256,
    #[serde(rename = "RS384")]
    Rs384,
    #[serde(rename = "RS512")]
    Rs512,
    #[serde(rename = "PS256")]
    Ps256,
    #[serde(rename = "PS384")]
    Ps384,
    #[serde(rename = "PS512")]
    Ps512,
    #[serde(rename = "ES256")]
    Es256,
    #[serde(rename = "ES384")]
    Es384,
    #[serde(rename = "ES512")]
    Es512,
    #[serde(rename = "ES256K")]
    Es256k,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SignatureAlgorithm {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SignatureAlgorithm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SignatureAlgorithm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Rs256 => serializer.serialize_unit_variant("SignatureAlgorithm", 0u32, "RS256"),
            Self::Rs384 => serializer.serialize_unit_variant("SignatureAlgorithm", 1u32, "RS384"),
            Self::Rs512 => serializer.serialize_unit_variant("SignatureAlgorithm", 2u32, "RS512"),
            Self::Ps256 => serializer.serialize_unit_variant("SignatureAlgorithm", 3u32, "PS256"),
            Self::Ps384 => serializer.serialize_unit_variant("SignatureAlgorithm", 4u32, "PS384"),
            Self::Ps512 => serializer.serialize_unit_variant("SignatureAlgorithm", 5u32, "PS512"),
            Self::Es256 => serializer.serialize_unit_variant("SignatureAlgorithm", 6u32, "ES256"),
            Self::Es384 => serializer.serialize_unit_variant("SignatureAlgorithm", 7u32, "ES384"),
            Self::Es512 => serializer.serialize_unit_variant("SignatureAlgorithm", 8u32, "ES512"),
            Self::Es256k => serializer.serialize_unit_variant("SignatureAlgorithm", 9u32, "ES256K"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Versions info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Versions")]
pub enum Versions {
    #[serde(rename = "2023-06-15-preview")]
    N2023_06_15_preview,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Versions {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Versions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Versions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::N2023_06_15_preview => serializer.serialize_unit_variant("Versions", 0u32, "2023-06-15-preview"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
