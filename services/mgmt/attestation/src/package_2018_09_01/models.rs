#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Attestation service response message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttestationProvider {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Status of attestation service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StatusResult>,
}
impl AttestationProvider {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Attestation Providers List."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttestationProviderListResult {
    #[doc = "Attestation Provider array."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AttestationProvider>,
}
impl AttestationProviderListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for creating an attestation service instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttestationServiceCreationParams {
    #[doc = "The supported Azure location where the attestation service instance should be created."]
    pub location: String,
    #[doc = "The tags that will be assigned to the attestation service instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Client supplied parameters used to create a new attestation service instance."]
    pub properties: AttestationServiceCreationSpecificParams,
}
impl AttestationServiceCreationParams {
    pub fn new(location: String, properties: AttestationServiceCreationSpecificParams) -> Self {
        Self {
            location,
            tags: None,
            properties,
        }
    }
}
#[doc = "Client supplied parameters used to create a new attestation service instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttestationServiceCreationSpecificParams {
    #[doc = "Name of attestation policy."]
    #[serde(rename = "attestationPolicy", default, skip_serializing_if = "Option::is_none")]
    pub attestation_policy: Option<String>,
    #[serde(rename = "policySigningCertificates", default, skip_serializing_if = "Option::is_none")]
    pub policy_signing_certificates: Option<JsonWebKeySet>,
}
impl AttestationServiceCreationSpecificParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for patching an attestation service instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttestationServicePatchParams {
    #[doc = "The tags that will be assigned to the attestation service instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AttestationServicePatchParams {
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
#[doc = "List of supported operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of supported operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationsDefinition>,
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition object with the name and properties of an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsDefinition {
    #[doc = "Name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display object with properties of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationsDisplayDefinition>,
}
impl OperationsDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Display object with properties of the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsDisplayDefinition {
    #[doc = "Resource provider of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Short description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationsDisplayDefinition {
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status of attestation service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusResult {
    #[doc = "Trust model for the attestation service instance."]
    #[serde(rename = "trustModel", default, skip_serializing_if = "Option::is_none")]
    pub trust_model: Option<String>,
    #[doc = "Status of attestation service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<status_result::Status>,
    #[doc = "Gets the uri of attestation service"]
    #[serde(rename = "attestUri", default, skip_serializing_if = "Option::is_none")]
    pub attest_uri: Option<String>,
}
impl StatusResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod status_result {
    use super::*;
    #[doc = "Status of attestation service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Ready,
        NotReady,
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
                Self::Ready => serializer.serialize_unit_variant("Status", 0u32, "Ready"),
                Self::NotReady => serializer.serialize_unit_variant("Status", 1u32, "NotReady"),
                Self::Error => serializer.serialize_unit_variant("Status", 2u32, "Error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
