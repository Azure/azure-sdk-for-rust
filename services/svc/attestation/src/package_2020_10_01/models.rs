#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Attestation request for Intel SGX enclaves"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttestOpenEnclaveRequest {
    #[doc = "OpenEnclave report from the enclave to be attested"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report: Option<String>,
    #[doc = "Runtime data are a conduit for any information defined by the Trusted Execution Environment (TEE) when actually running."]
    #[serde(rename = "runtimeData", default, skip_serializing_if = "Option::is_none")]
    pub runtime_data: Option<RuntimeData>,
    #[doc = "Initialization time data are a conduit for any configuration information that is unknown when building the Trusted Execution Environment (TEE) and is defined at TEE launch time. This data can be used with confidential container or VM scenarios to capture configuration settings such as disk volume content, network configuration, etc."]
    #[serde(rename = "initTimeData", default, skip_serializing_if = "Option::is_none")]
    pub init_time_data: Option<InitTimeData>,
    #[doc = "Attest against the provided draft policy. Note that the resulting token cannot be validated."]
    #[serde(rename = "draftPolicyForAttestation", default, skip_serializing_if = "Option::is_none")]
    pub draft_policy_for_attestation: Option<String>,
    #[doc = "Nonce for incoming request - emitted in the generated attestation token"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}
impl AttestOpenEnclaveRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Attestation request for Intel SGX enclaves"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttestSgxEnclaveRequest {
    #[doc = "Quote of the enclave to be attested"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    #[doc = "Runtime data are a conduit for any information defined by the Trusted Execution Environment (TEE) when actually running."]
    #[serde(rename = "runtimeData", default, skip_serializing_if = "Option::is_none")]
    pub runtime_data: Option<RuntimeData>,
    #[doc = "Initialization time data are a conduit for any configuration information that is unknown when building the Trusted Execution Environment (TEE) and is defined at TEE launch time. This data can be used with confidential container or VM scenarios to capture configuration settings such as disk volume content, network configuration, etc."]
    #[serde(rename = "initTimeData", default, skip_serializing_if = "Option::is_none")]
    pub init_time_data: Option<InitTimeData>,
    #[doc = "Attest against the provided draft policy. Note that the resulting token cannot be validated."]
    #[serde(rename = "draftPolicyForAttestation", default, skip_serializing_if = "Option::is_none")]
    pub draft_policy_for_attestation: Option<String>,
    #[doc = "Nonce for incoming request - emitted in the generated attestation token"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}
impl AttestSgxEnclaveRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The body of the JWT used for the PolicyCertificates APIs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttestationCertificateManagementBody {
    #[serde(rename = "policyCertificate", default, skip_serializing_if = "Option::is_none")]
    pub policy_certificate: Option<JsonWebKey>,
}
impl AttestationCertificateManagementBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of an attestation operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttestationResponse {
    #[doc = "An RFC 7519 Json Web Token"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<JsonWebToken>,
}
impl AttestationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Microsoft Azure Attestation response token body - the body of a response token issued by MAA"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttestationResult {
    #[doc = "Unique Identifier for the token"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    #[doc = "The Principal who issued the token"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    #[doc = "The time at which the token was issued, in the number of seconds since 1970-01-0T00:00:00Z UTC"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iat: Option<f64>,
    #[doc = "The expiration time after which the token is no longer valid, in the number of seconds since 1970-01-0T00:00:00Z UTC"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exp: Option<f64>,
    #[doc = "The not before time before which the token cannot be considered valid, in the number of seconds since 1970-01-0T00:00:00Z UTC"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<f64>,
    #[doc = "An RFC 7800 Proof of Possession Key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cnf: Option<serde_json::Value>,
    #[doc = "The Nonce input to the attestation request, if provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[doc = "The Schema version of this structure. Current Value: 1.0"]
    #[serde(rename = "x-ms-ver", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_ver: Option<String>,
    #[doc = "Runtime Claims"]
    #[serde(rename = "x-ms-runtime", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_runtime: Option<serde_json::Value>,
    #[doc = "Inittime Claims"]
    #[serde(rename = "x-ms-inittime", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_inittime: Option<serde_json::Value>,
    #[doc = "Policy Generated Claims"]
    #[serde(rename = "x-ms-policy", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_policy: Option<serde_json::Value>,
    #[doc = "The Attestation type being attested."]
    #[serde(rename = "x-ms-attestation-type", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_attestation_type: Option<String>,
    #[serde(rename = "x-ms-policy-signer", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_policy_signer: Option<JsonWebKey>,
    #[doc = "The SHA256 hash of the BASE64URL encoded policy text used for attestation"]
    #[serde(rename = "x-ms-policy-hash", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_policy_hash: Option<String>,
    #[doc = "True if the enclave is debuggable, false otherwise"]
    #[serde(rename = "x-ms-sgx-is-debuggable", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_sgx_is_debuggable: Option<bool>,
    #[doc = "The SGX Product ID for the enclave."]
    #[serde(rename = "x-ms-sgx-product-id", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_sgx_product_id: Option<f64>,
    #[doc = "The HEX encoded SGX MRENCLAVE value for the enclave."]
    #[serde(rename = "x-ms-sgx-mrenclave", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_sgx_mrenclave: Option<String>,
    #[doc = "The HEX encoded SGX MRSIGNER value for the enclave."]
    #[serde(rename = "x-ms-sgx-mrsigner", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_sgx_mrsigner: Option<String>,
    #[doc = "The SGX SVN value for the enclave."]
    #[serde(rename = "x-ms-sgx-svn", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_sgx_svn: Option<f64>,
    #[doc = "A copy of the RuntimeData specified as an input to the attest call."]
    #[serde(rename = "x-ms-sgx-ehd", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_sgx_ehd: Option<String>,
    #[doc = "The SGX SVN value for the enclave."]
    #[serde(rename = "x-ms-sgx-collateral", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_sgx_collateral: Option<serde_json::Value>,
    #[doc = "DEPRECATED: Private Preview version of x-ms-ver claim."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
    #[doc = "DEPRECATED: Private Preview version of x-ms-sgx-is-debuggable claim."]
    #[serde(rename = "is-debuggable", default, skip_serializing_if = "Option::is_none")]
    pub is_debuggable: Option<bool>,
    #[doc = "DEPRECATED: Private Preview version of x-ms-sgx-collateral claim."]
    #[serde(rename = "maa-attestationcollateral", default, skip_serializing_if = "Option::is_none")]
    pub maa_attestationcollateral: Option<serde_json::Value>,
    #[doc = "DEPRECATED: Private Preview version of x-ms-sgx-ehd claim."]
    #[serde(rename = "aas-ehd", default, skip_serializing_if = "Option::is_none")]
    pub aas_ehd: Option<String>,
    #[doc = "DEPRECATED: Private Preview version of x-ms-sgx-ehd claim."]
    #[serde(rename = "maa-ehd", default, skip_serializing_if = "Option::is_none")]
    pub maa_ehd: Option<String>,
    #[doc = "DEPRECATED: Private Preview version of x-ms-sgx-product-id"]
    #[serde(rename = "product-id", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<f64>,
    #[doc = "DEPRECATED: Private Preview version of x-ms-sgx-mrenclave."]
    #[serde(rename = "sgx-mrenclave", default, skip_serializing_if = "Option::is_none")]
    pub sgx_mrenclave: Option<String>,
    #[doc = "DEPRECATED: Private Preview version of x-ms-sgx-mrsigner."]
    #[serde(rename = "sgx-mrsigner", default, skip_serializing_if = "Option::is_none")]
    pub sgx_mrsigner: Option<String>,
    #[doc = "DEPRECATED: Private Preview version of x-ms-sgx-svn."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub svn: Option<f64>,
    #[doc = "DEPRECATED: Private Preview version of x-ms-tee."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tee: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_signer: Option<JsonWebKey>,
    #[doc = "DEPRECATED: Private Preview version of x-ms-policy-hash"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_hash: Option<String>,
    #[doc = "DEPRECATED: Private Preview version of nonce"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rp_data: Option<String>,
}
impl AttestationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from Attestation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from Attestation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from Attestation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for displaying in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the type of the data encoded contained within the \"data\" field of a \"RuntimeData\" or \"InitTimeData\" object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataType")]
pub enum DataType {
    Binary,
    #[serde(rename = "JSON")]
    Json,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Binary => serializer.serialize_unit_variant("DataType", 0u32, "Binary"),
            Self::Json => serializer.serialize_unit_variant("DataType", 1u32, "JSON"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Initialization time data are a conduit for any configuration information that is unknown when building the Trusted Execution Environment (TEE) and is defined at TEE launch time. This data can be used with confidential container or VM scenarios to capture configuration settings such as disk volume content, network configuration, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InitTimeData {
    #[doc = "Initialization time data are passed into the Trusted Execution Environment (TEE) when it is created. For an Icelake SGX quote, the SHA256 hash of the InitTimeData must match the lower 32 bytes of the quote's \"config id\" attribute. For a SEV-SNP quote, the SHA256 hash of the InitTimeData must match the quote's \"host data\" attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[doc = "Specifies the type of the data encoded contained within the \"data\" field of a \"RuntimeData\" or \"InitTimeData\" object"]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<DataType>,
}
impl InitTimeData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonWebKey {
    #[doc = "The \"alg\" (algorithm) parameter identifies the algorithm intended for\nuse with the key.  The values used should either be registered in the\nIANA \"JSON Web Signature and Encryption Algorithms\" registry\nestablished by [JWA] or be a value that contains a Collision-\nResistant Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,
    #[doc = "The \"crv\" (curve) parameter identifies the curve type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>,
    #[doc = "RSA private exponent or ECC private key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,
    #[doc = "RSA Private Key Parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dp: Option<String>,
    #[doc = "RSA Private Key Parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dq: Option<String>,
    #[doc = "RSA public exponent, in Base64"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[doc = "Symmetric key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub k: Option<String>,
    #[doc = "The \"kid\" (key ID) parameter is used to match a specific key.  This\nis used, for instance, to choose among a set of keys within a JWK Set\nduring key rollover.  The structure of the \"kid\" value is\nunspecified.  When \"kid\" values are used within a JWK Set, different\nkeys within the JWK Set SHOULD use distinct \"kid\" values.  (One\nexample in which different keys might use the same \"kid\" value is if\nthey have different \"kty\" (key type) values but are considered to be\nequivalent alternatives by the application using them.)  The \"kid\"\nvalue is a case-sensitive string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[doc = "The \"kty\" (key type) parameter identifies the cryptographic algorithm\nfamily used with the key, such as \"RSA\" or \"EC\". \"kty\" values should\neither be registered in the IANA \"JSON Web Key Types\" registry\nestablished by [JWA] or be a value that contains a Collision-\nResistant Name.  The \"kty\" value is a case-sensitive string."]
    pub kty: String,
    #[doc = "RSA modulus, in Base64"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    #[doc = "RSA secret prime"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    #[doc = "RSA secret prime, with p < q"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[doc = "RSA Private Key Parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qi: Option<String>,
    #[doc = "Use (\"public key use\") identifies the intended use of\nthe public key. The \"use\" parameter is employed to indicate whether\na public key is used for encrypting data or verifying the signature\non data. Values are commonly \"sig\" (signature) or \"enc\" (encryption)."]
    #[serde(rename = "use", default, skip_serializing_if = "Option::is_none")]
    pub use_: Option<String>,
    #[doc = "X coordinate for the Elliptic Curve point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[doc = "The \"x5c\" (X.509 certificate chain) parameter contains a chain of one\nor more PKIX certificates [RFC5280].  The certificate chain is\nrepresented as a JSON array of certificate value strings.  Each\nstring in the array is a base64-encoded (Section 4 of [RFC4648] --\nnot base64url-encoded) DER [ITU.X690.1994] PKIX certificate value.\nThe PKIX certificate containing the key value MUST be the first\ncertificate."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub x5c: Vec<String>,
    #[doc = "Y coordinate for the Elliptic Curve point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
}
impl JsonWebKey {
    pub fn new(kty: String) -> Self {
        Self {
            alg: None,
            crv: None,
            d: None,
            dp: None,
            dq: None,
            e: None,
            k: None,
            kid: None,
            kty,
            n: None,
            p: None,
            q: None,
            qi: None,
            use_: None,
            x: None,
            x5c: Vec::new(),
            y: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonWebKeySet {
    #[doc = "The value of the \"keys\" parameter is an array of JWK values.  By\ndefault, the order of the JWK values within the array does not imply\nan order of preference among them, although applications of JWK Sets\ncan choose to assign a meaning to the order for their purposes, if\ndesired."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<JsonWebKey>,
}
impl JsonWebKeySet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type JsonWebToken = String;
#[doc = "The result of a policy certificate modification"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyCertificatesModificationResult {
    #[doc = "Hex encoded SHA1 Hash of the binary representation certificate which was added or removed"]
    #[serde(rename = "x-ms-certificate-thumbprint", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_certificate_thumbprint: Option<String>,
    #[doc = "The result of the operation"]
    #[serde(rename = "x-ms-policycertificates-result", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_policycertificates_result: Option<policy_certificates_modification_result::XMsPolicycertificatesResult>,
}
impl PolicyCertificatesModificationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_certificates_modification_result {
    use super::*;
    #[doc = "The result of the operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "XMsPolicycertificatesResult")]
    pub enum XMsPolicycertificatesResult {
        IsPresent,
        IsAbsent,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for XMsPolicycertificatesResult {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for XMsPolicycertificatesResult {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for XMsPolicycertificatesResult {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IsPresent => serializer.serialize_unit_variant("XMsPolicycertificatesResult", 0u32, "IsPresent"),
                Self::IsAbsent => serializer.serialize_unit_variant("XMsPolicycertificatesResult", 1u32, "IsAbsent"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response to an attestation policy management API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyCertificatesModifyResponse {
    #[doc = "An RFC 7519 Json Web Token"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<JsonWebToken>,
}
impl PolicyCertificatesModifyResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to an attestation policy management API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyCertificatesResponse {
    #[doc = "An RFC 7519 Json Web Token"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<JsonWebToken>,
}
impl PolicyCertificatesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a call to retrieve policy certificates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyCertificatesResult {
    #[serde(rename = "x-ms-policy-certificates", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_policy_certificates: Option<JsonWebKeySet>,
}
impl PolicyCertificatesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to an attestation policy operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyResponse {
    #[doc = "An RFC 7519 Json Web Token"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<JsonWebToken>,
}
impl PolicyResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a policy certificate modification"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyResult {
    #[doc = "The result of the operation"]
    #[serde(rename = "x-ms-policy-result", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_policy_result: Option<policy_result::XMsPolicyResult>,
    #[doc = "The SHA256 hash of the policy object modified"]
    #[serde(rename = "x-ms-policy-token-hash", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_policy_token_hash: Option<String>,
    #[serde(rename = "x-ms-policy-signer", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_policy_signer: Option<JsonWebKey>,
    #[doc = "An RFC 7519 Json Web Token"]
    #[serde(rename = "x-ms-policy", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_policy: Option<JsonWebToken>,
}
impl PolicyResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_result {
    use super::*;
    #[doc = "The result of the operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "XMsPolicyResult")]
    pub enum XMsPolicyResult {
        Updated,
        Removed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for XMsPolicyResult {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for XMsPolicyResult {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for XMsPolicyResult {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Updated => serializer.serialize_unit_variant("XMsPolicyResult", 0u32, "Updated"),
                Self::Removed => serializer.serialize_unit_variant("XMsPolicyResult", 1u32, "Removed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Runtime data are a conduit for any information defined by the Trusted Execution Environment (TEE) when actually running."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuntimeData {
    #[doc = "Runtime data are generated by the Trusted Execution Environment (TEE). For an SGX quote (Coffeelake or Icelake), the SHA256 hash of the RuntimeData must match the lower 32 bytes of the quote's \"report data\" attribute. For a SEV-SNP quote, the SHA256 hash of the RuntimeData must match the quote's \"report data\" attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[doc = "Specifies the type of the data encoded contained within the \"data\" field of a \"RuntimeData\" or \"InitTimeData\" object"]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<DataType>,
}
impl RuntimeData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StoredAttestationPolicy {
    #[doc = "Policy text to set as a sequence of UTF-8 encoded octets."]
    #[serde(rename = "AttestationPolicy", default, skip_serializing_if = "Option::is_none")]
    pub attestation_policy: Option<String>,
}
impl StoredAttestationPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Attestation request for Trusted Platform Module (TPM) attestation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TpmAttestationRequest {
    #[doc = "Protocol data containing artifacts for attestation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}
impl TpmAttestationRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Attestation response for Trusted Platform Module (TPM) attestation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TpmAttestationResponse {
    #[doc = "Protocol data containing attestation service response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}
impl TpmAttestationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
