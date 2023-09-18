#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Binary hardening of a firmware."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BinaryHardening {
    #[doc = "ID for the binary hardening result."]
    #[serde(rename = "binaryHardeningId", default, skip_serializing_if = "Option::is_none")]
    pub binary_hardening_id: Option<String>,
    #[doc = "Binary hardening features."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<BinaryHardeningFeatures>,
    #[doc = "The architecture of the uploaded firmware."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[doc = "path for binary hardening."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "class for binary hardening."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[doc = "The runpath of the uploaded firmware."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runpath: Option<String>,
    #[doc = "The rpath of the uploaded firmware."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rpath: Option<String>,
}
impl BinaryHardening {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Binary hardening features."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BinaryHardeningFeatures {
    #[doc = "NX flag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nx: Option<binary_hardening_features::Nx>,
    #[doc = "PIE flag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pie: Option<binary_hardening_features::Pie>,
    #[doc = "RELRO flag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relro: Option<binary_hardening_features::Relro>,
    #[doc = "Canary flag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canary: Option<binary_hardening_features::Canary>,
    #[doc = "Stripped flag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stripped: Option<binary_hardening_features::Stripped>,
}
impl BinaryHardeningFeatures {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod binary_hardening_features {
    use super::*;
    #[doc = "NX flag."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Nx")]
    pub enum Nx {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Nx {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Nx {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Nx {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Nx", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("Nx", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "PIE flag."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Pie")]
    pub enum Pie {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Pie {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Pie {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Pie {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Pie", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("Pie", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "RELRO flag."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Relro")]
    pub enum Relro {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Relro {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Relro {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Relro {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Relro", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("Relro", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Canary flag."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Canary")]
    pub enum Canary {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Canary {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Canary {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Canary {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Canary", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("Canary", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Stripped flag."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Stripped")]
    pub enum Stripped {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Stripped {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Stripped {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Stripped {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Stripped", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("Stripped", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List result for binary hardening"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BinaryHardeningList {
    #[doc = "The list of binary hardening results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BinaryHardening>,
    #[doc = "The uri to fetch the next page of asset."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BinaryHardeningList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BinaryHardeningList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Binary hardening summary percentages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BinaryHardeningSummary {
    #[doc = "Total number of binaries that were analyzed"]
    #[serde(rename = "totalFiles", default, skip_serializing_if = "Option::is_none")]
    pub total_files: Option<i64>,
    #[doc = "NX summary percentage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nx: Option<i32>,
    #[doc = "PIE summary percentage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pie: Option<i32>,
    #[doc = "RELRO summary percentage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relro: Option<i32>,
    #[doc = "Canary summary percentage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canary: Option<i32>,
    #[doc = "Stripped summary percentage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stripped: Option<i32>,
}
impl BinaryHardeningSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Component of a firmware."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Component {
    #[doc = "ID for the component."]
    #[serde(rename = "componentId", default, skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    #[doc = "Name for the component."]
    #[serde(rename = "componentName", default, skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    #[doc = "Version for the component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "License for the component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[doc = "Release date for the component."]
    #[serde(rename = "releaseDate", default, with = "azure_core::date::rfc3339::option")]
    pub release_date: Option<time::OffsetDateTime>,
    #[doc = "Paths of the component."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub paths: Vec<String>,
    #[doc = "Flag if new update is available for the component."]
    #[serde(rename = "isUpdateAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_update_available: Option<component::IsUpdateAvailable>,
}
impl Component {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod component {
    use super::*;
    #[doc = "Flag if new update is available for the component."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsUpdateAvailable")]
    pub enum IsUpdateAvailable {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsUpdateAvailable {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsUpdateAvailable {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsUpdateAvailable {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("IsUpdateAvailable", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("IsUpdateAvailable", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List result for components"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentList {
    #[doc = "The list of components."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Component>,
    #[doc = "The uri to fetch the next page of asset."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ComponentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ComponentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Crypto certificate properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CryptoCertificate {
    #[doc = "ID for the certificate."]
    #[serde(rename = "cryptoCertId", default, skip_serializing_if = "Option::is_none")]
    pub crypto_cert_id: Option<String>,
    #[doc = "Name of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Information on an entity (distinguished name) in a cryptographic certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<CryptoCertificateEntity>,
    #[doc = "Information on an entity (distinguished name) in a cryptographic certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CryptoCertificateEntity>,
    #[doc = "Issue date for the certificate."]
    #[serde(rename = "issuedDate", default, with = "azure_core::date::rfc3339::option")]
    pub issued_date: Option<time::OffsetDateTime>,
    #[doc = "Expiration date for the certificate."]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Role of the certificate (Root CA, etc)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "The signature algorithm used in the certificate."]
    #[serde(rename = "signatureAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub signature_algorithm: Option<String>,
    #[doc = "Size of the certificate's key in bits"]
    #[serde(rename = "keySize", default, skip_serializing_if = "Option::is_none")]
    pub key_size: Option<i64>,
    #[doc = "Key algorithm used in the certificate."]
    #[serde(rename = "keyAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    #[doc = "Encoding used for the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[doc = "Serial number of the certificate."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Fingerprint of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "List of functions the certificate can fulfill."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub usage: Vec<String>,
    #[doc = "List of files paths for this certificate"]
    #[serde(
        rename = "filePaths",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub file_paths: Vec<String>,
    #[doc = "Details of a matching paired key or certificate."]
    #[serde(rename = "pairedKey", default, skip_serializing_if = "Option::is_none")]
    pub paired_key: Option<PairedKey>,
    #[doc = "Indicates if the certificate is expired."]
    #[serde(rename = "isExpired", default, skip_serializing_if = "Option::is_none")]
    pub is_expired: Option<crypto_certificate::IsExpired>,
    #[doc = "Indicates if the certificate was self-signed."]
    #[serde(rename = "isSelfSigned", default, skip_serializing_if = "Option::is_none")]
    pub is_self_signed: Option<crypto_certificate::IsSelfSigned>,
    #[doc = "Indicates the signature algorithm used is insecure."]
    #[serde(rename = "isWeakSignature", default, skip_serializing_if = "Option::is_none")]
    pub is_weak_signature: Option<crypto_certificate::IsWeakSignature>,
    #[doc = "Indicates the certificate's key size is considered too small to be secure for the key algorithm."]
    #[serde(rename = "isShortKeySize", default, skip_serializing_if = "Option::is_none")]
    pub is_short_key_size: Option<crypto_certificate::IsShortKeySize>,
}
impl CryptoCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod crypto_certificate {
    use super::*;
    #[doc = "Indicates if the certificate is expired."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsExpired")]
    pub enum IsExpired {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsExpired {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsExpired {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsExpired {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("IsExpired", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("IsExpired", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates if the certificate was self-signed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsSelfSigned")]
    pub enum IsSelfSigned {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsSelfSigned {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsSelfSigned {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsSelfSigned {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("IsSelfSigned", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("IsSelfSigned", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates the signature algorithm used is insecure."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsWeakSignature")]
    pub enum IsWeakSignature {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsWeakSignature {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsWeakSignature {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsWeakSignature {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("IsWeakSignature", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("IsWeakSignature", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates the certificate's key size is considered too small to be secure for the key algorithm."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsShortKeySize")]
    pub enum IsShortKeySize {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsShortKeySize {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsShortKeySize {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsShortKeySize {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("IsShortKeySize", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("IsShortKeySize", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information on an entity (distinguished name) in a cryptographic certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CryptoCertificateEntity {
    #[doc = "Common name of the certificate entity."]
    #[serde(rename = "commonName", default, skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    #[doc = "Organization of the certificate entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[doc = "The organizational unit of the certificate entity."]
    #[serde(rename = "organizationalUnit", default, skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<String>,
    #[doc = "Geographical state or province of the certificate entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Country code of the certificate entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}
impl CryptoCertificateEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Crypto certificates list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CryptoCertificateList {
    #[doc = "Crypto certificates list"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CryptoCertificate>,
    #[doc = "The uri to fetch the next page of asset."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CryptoCertificateList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CryptoCertificateList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cryptographic certificate summary values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CryptoCertificateSummary {
    #[doc = "Total number of certificates found."]
    #[serde(rename = "totalCertificates", default, skip_serializing_if = "Option::is_none")]
    pub total_certificates: Option<i64>,
    #[doc = "Total number of paired private keys found for the certificates."]
    #[serde(rename = "pairedKeys", default, skip_serializing_if = "Option::is_none")]
    pub paired_keys: Option<i64>,
    #[doc = "Total number of expired certificates found."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expired: Option<i64>,
    #[doc = "Total number of nearly expired certificates found."]
    #[serde(rename = "expiringSoon", default, skip_serializing_if = "Option::is_none")]
    pub expiring_soon: Option<i64>,
    #[doc = "Total number of certificates found using a weak signature algorithm."]
    #[serde(rename = "weakSignature", default, skip_serializing_if = "Option::is_none")]
    pub weak_signature: Option<i64>,
    #[doc = "Total number of certificates found that are self-signed."]
    #[serde(rename = "selfSigned", default, skip_serializing_if = "Option::is_none")]
    pub self_signed: Option<i64>,
    #[doc = "Total number of certificates found that have an insecure key size for the key algorithm."]
    #[serde(rename = "shortKeySize", default, skip_serializing_if = "Option::is_none")]
    pub short_key_size: Option<i64>,
}
impl CryptoCertificateSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Crypto key properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CryptoKey {
    #[doc = "ID for the key."]
    #[serde(rename = "cryptoKeyId", default, skip_serializing_if = "Option::is_none")]
    pub crypto_key_id: Option<String>,
    #[doc = "Type of the key (public or private)."]
    #[serde(rename = "keyType", default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[doc = "Size of the key in bits."]
    #[serde(rename = "keySize", default, skip_serializing_if = "Option::is_none")]
    pub key_size: Option<i64>,
    #[doc = "Key algorithm name."]
    #[serde(rename = "keyAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    #[doc = "Functions the key can fulfill."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub usage: Vec<String>,
    #[doc = "List of files paths for this key."]
    #[serde(
        rename = "filePaths",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub file_paths: Vec<String>,
    #[doc = "Details of a matching paired key or certificate."]
    #[serde(rename = "pairedKey", default, skip_serializing_if = "Option::is_none")]
    pub paired_key: Option<PairedKey>,
    #[doc = "Indicates the key size is considered too small to be secure for the algorithm."]
    #[serde(rename = "isShortKeySize", default, skip_serializing_if = "Option::is_none")]
    pub is_short_key_size: Option<crypto_key::IsShortKeySize>,
}
impl CryptoKey {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod crypto_key {
    use super::*;
    #[doc = "Indicates the key size is considered too small to be secure for the algorithm."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsShortKeySize")]
    pub enum IsShortKeySize {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsShortKeySize {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsShortKeySize {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsShortKeySize {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("IsShortKeySize", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("IsShortKeySize", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Crypto keys list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CryptoKeyList {
    #[doc = "Crypto keys list"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CryptoKey>,
    #[doc = "The uri to fetch the next page of asset."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CryptoKeyList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CryptoKeyList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cryptographic key summary values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CryptoKeySummary {
    #[doc = "Total number of cryptographic keys found."]
    #[serde(rename = "totalKeys", default, skip_serializing_if = "Option::is_none")]
    pub total_keys: Option<i64>,
    #[doc = "Total number of (non-certificate) public keys found."]
    #[serde(rename = "publicKeys", default, skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<i64>,
    #[doc = "Total number of private keys found."]
    #[serde(rename = "privateKeys", default, skip_serializing_if = "Option::is_none")]
    pub private_keys: Option<i64>,
    #[doc = "Total number of keys found that have a matching paired key or certificate."]
    #[serde(rename = "pairedKeys", default, skip_serializing_if = "Option::is_none")]
    pub paired_keys: Option<i64>,
    #[doc = "Total number of keys found that have an insecure key size for the algorithm."]
    #[serde(rename = "shortKeySize", default, skip_serializing_if = "Option::is_none")]
    pub short_key_size: Option<i64>,
}
impl CryptoKeySummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Known CVEs of a firmware."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Cve {
    #[doc = "ID of CVE"]
    #[serde(rename = "cveId", default, skip_serializing_if = "Option::is_none")]
    pub cve_id: Option<String>,
    #[doc = "Component of CVE"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component: Option<serde_json::Value>,
    #[doc = "Severity of CVE"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "Name of CVE"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A single CVSS score to represent the CVE. If a V3 score is specified, then it will use the V3 score. Otherwise if the V2 score is specified it will be the V2 score"]
    #[serde(rename = "cvssScore", default, skip_serializing_if = "Option::is_none")]
    pub cvss_score: Option<String>,
    #[doc = "Cvss version of CVE"]
    #[serde(rename = "cvssVersion", default, skip_serializing_if = "Option::is_none")]
    pub cvss_version: Option<String>,
    #[doc = "Cvss V2 score of CVE"]
    #[serde(rename = "cvssV2Score", default, skip_serializing_if = "Option::is_none")]
    pub cvss_v2_score: Option<String>,
    #[doc = "Cvss V3 score of CVE"]
    #[serde(rename = "cvssV3Score", default, skip_serializing_if = "Option::is_none")]
    pub cvss_v3_score: Option<String>,
    #[doc = "Publish date of CVE"]
    #[serde(rename = "publishDate", default, with = "azure_core::date::rfc3339::option")]
    pub publish_date: Option<time::OffsetDateTime>,
    #[doc = "Updated date of CVE"]
    #[serde(rename = "updatedDate", default, with = "azure_core::date::rfc3339::option")]
    pub updated_date: Option<time::OffsetDateTime>,
    #[doc = "The list of CVE links."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub links: Vec<CveLink>,
    #[doc = "Description of CVE"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl Cve {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Component for CVE"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CveComponent {
    #[doc = "ID of CVE component"]
    #[serde(rename = "componentId", default, skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    #[doc = "Name of CVE component"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Version of CVE component"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl CveComponent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Link for CVE"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CveLink {
    #[doc = "Href of CVE link"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[doc = "Label of CVE link"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
impl CveLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List result for CVE"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CveList {
    #[doc = "The list of CVE results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Cve>,
    #[doc = "The uri to fetch the next page of asset."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CveList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CveList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CVE summary values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CveSummary {
    #[doc = "The total number of critical severity CVEs detected"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<i64>,
    #[doc = "The total number of high severity CVEs detected"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high: Option<i64>,
    #[doc = "The total number of medium severity CVEs detected"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<i64>,
    #[doc = "The total number of low severity CVEs detected"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low: Option<i64>,
    #[doc = "The total number of unknown severity CVEs detected"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i64>,
    #[doc = "The total number of undefined severity CVEs detected"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undefined: Option<i64>,
}
impl CveSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Firmware definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Firmware {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Firmware properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FirmwareProperties>,
}
impl Firmware {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of firmwares"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirmwareList {
    #[doc = "The list of firmwares."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Firmware>,
    #[doc = "The uri to fetch the next page of asset."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FirmwareList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FirmwareList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Firmware properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirmwareProperties {
    #[doc = "File name for a firmware that user uploaded."]
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "Firmware vendor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[doc = "Firmware model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[doc = "Firmware version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "User-specified description of the firmware."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "File size of the uploaded firmware image."]
    #[serde(rename = "fileSize", default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[doc = "The status of firmware scan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<firmware_properties::Status>,
    #[doc = "A list of errors or other messages generated during firmware analysis"]
    #[serde(
        rename = "statusMessages",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub status_messages: Vec<StatusMessage>,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<firmware_properties::ProvisioningState>,
}
impl FirmwareProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod firmware_properties {
    use super::*;
    #[doc = "The status of firmware scan."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Pending,
        Extracting,
        Analyzing,
        Ready,
        Error,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pending => serializer.serialize_unit_variant("Status", 0u32, "Pending"),
                Self::Extracting => serializer.serialize_unit_variant("Status", 1u32, "Extracting"),
                Self::Analyzing => serializer.serialize_unit_variant("Status", 2u32, "Analyzing"),
                Self::Ready => serializer.serialize_unit_variant("Status", 3u32, "Ready"),
                Self::Error => serializer.serialize_unit_variant("Status", 4u32, "Error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Pending
        }
    }
    #[doc = "Provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Accepted,
        Succeeded,
        Canceled,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Accepted"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Summary result after scanning the firmware."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirmwareSummary {
    #[doc = "Total extracted size of the firmware in bytes."]
    #[serde(rename = "extractedSize", default, skip_serializing_if = "Option::is_none")]
    pub extracted_size: Option<i64>,
    #[doc = "Firmware file size in bytes."]
    #[serde(rename = "fileSize", default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[doc = "Extracted file count."]
    #[serde(rename = "extractedFileCount", default, skip_serializing_if = "Option::is_none")]
    pub extracted_file_count: Option<i64>,
    #[doc = "Components count."]
    #[serde(rename = "componentCount", default, skip_serializing_if = "Option::is_none")]
    pub component_count: Option<i64>,
    #[doc = "Binary count"]
    #[serde(rename = "binaryCount", default, skip_serializing_if = "Option::is_none")]
    pub binary_count: Option<i64>,
    #[doc = "Time used for analysis"]
    #[serde(rename = "analysisTimeSeconds", default, skip_serializing_if = "Option::is_none")]
    pub analysis_time_seconds: Option<i64>,
    #[doc = "The number of root file systems found."]
    #[serde(rename = "rootFileSystems", default, skip_serializing_if = "Option::is_none")]
    pub root_file_systems: Option<i64>,
}
impl FirmwareSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Firmware definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirmwareUpdateDefinition {
    #[doc = "Firmware properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FirmwareProperties>,
}
impl FirmwareUpdateDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for generating an upload URL"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateUploadUrlRequest {
    #[doc = "A unique ID for the firmware to be uploaded."]
    #[serde(rename = "firmwareId", default, skip_serializing_if = "Option::is_none")]
    pub firmware_id: Option<String>,
}
impl GenerateUploadUrlRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of a REST API operation, returned from the Resource Provider Operations API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation, as per Resource-Based Access Control (RBAC). Examples: \"Microsoft.Compute/virtualMachines/write\", \"Microsoft.Compute/virtualMachines/capture/action\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Localized display information for this particular operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name, e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly name of the resource type related to this operation. E.g. \"Virtual Machines\" or \"Job Schedule Collections\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The concise, localized friendly name for the operation; suitable for dropdowns. E.g. \"Create or Update Virtual Machine\", \"Restart Virtual Machine\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The short, localized friendly description of the operation; suitable for tool tips and detailed views."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Origin {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Origin {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Origin {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
                Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        Internal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of a matching paired key or certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PairedKey {
    #[doc = "ID of the paired key or certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type indicating whether the paired object is a key or certificate."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Additional paired key properties"]
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
}
impl PairedKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Password hash properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PasswordHash {
    #[doc = "ID for password hash"]
    #[serde(rename = "passwordHashId", default, skip_serializing_if = "Option::is_none")]
    pub password_hash_id: Option<String>,
    #[doc = "File path of the password hash"]
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[doc = "Salt of the password hash"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
    #[doc = "Hash of the password"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[doc = "Context of password hash"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "User name of password hash"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Algorithm of the password hash"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
}
impl PasswordHash {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Password hashes list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PasswordHashList {
    #[doc = "Password hashes list"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PasswordHash>,
    #[doc = "The uri to fetch the next page of asset."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PasswordHashList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PasswordHashList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a Azure Resource Manager proxy resource. It will not have tags and a location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error and status message"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusMessage {}
impl StatusMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[doc = "Url data for creating or accessing a blob file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UrlToken {
    #[doc = "SAS URL for creating or accessing a blob file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "SAS URL for file uploading. Kept for backwards compatibility"]
    #[serde(rename = "uploadUrl", default, skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
}
impl UrlToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Firmware analysis workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Workspace properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
}
impl Workspace {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Return a list of firmware analysis workspaces."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceList {
    #[doc = "The list of firmware analysis workspaces."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Workspace>,
    #[doc = "The uri to fetch the next page of asset."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkspaceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkspaceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workspace properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workspace_properties::ProvisioningState>,
}
impl WorkspaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace_properties {
    use super::*;
    #[doc = "Provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Accepted,
        Succeeded,
        Canceled,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Accepted"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Firmware analysis workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceUpdateDefinition {
    #[doc = "Workspace properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
}
impl WorkspaceUpdateDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreatedByType")]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreatedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreatedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreatedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastModifiedByType")]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastModifiedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastModifiedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastModifiedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("LastModifiedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("LastModifiedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("LastModifiedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("LastModifiedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
