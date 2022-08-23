#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Internal error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDefinition>,
}
impl ErrorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
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
#[doc = "MAK key details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultipleActivationKey {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "MAK key specific properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<multiple_activation_key::Properties>,
}
impl MultipleActivationKey {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
pub mod multiple_activation_key {
    use super::*;
    #[doc = "MAK key specific properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "MAK 5x5 key."]
        #[serde(rename = "multipleActivationKey", default, skip_serializing_if = "Option::is_none")]
        pub multiple_activation_key: Option<String>,
        #[doc = "End of support of security updates activated by the MAK key."]
        #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
        pub expiration_date: Option<time::OffsetDateTime>,
        #[doc = "Type of OS for which the key is requested."]
        #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
        pub os_type: Option<properties::OsType>,
        #[doc = "Type of support"]
        #[serde(rename = "supportType", default, skip_serializing_if = "Option::is_none")]
        pub support_type: Option<properties::SupportType>,
        #[doc = "Number of activations/servers using the MAK key."]
        #[serde(rename = "installedServerNumber", default, skip_serializing_if = "Option::is_none")]
        pub installed_server_number: Option<i64>,
        #[doc = "Agreement number under which the key is requested."]
        #[serde(rename = "agreementNumber", default, skip_serializing_if = "Option::is_none")]
        pub agreement_number: Option<String>,
        #[doc = "<code> true </code> if user has eligible on-premises Windows physical or virtual machines, and that the requested key will only be used in their organization; <code> false </code> otherwise."]
        #[serde(rename = "isEligible", default, skip_serializing_if = "Option::is_none")]
        pub is_eligible: Option<bool>,
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Type of OS for which the key is requested."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "OsType")]
        pub enum OsType {
            Windows7,
            WindowsServer2008,
            WindowsServer2008R2,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for OsType {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for OsType {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for OsType {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Windows7 => serializer.serialize_unit_variant("OsType", 0u32, "Windows7"),
                    Self::WindowsServer2008 => serializer.serialize_unit_variant("OsType", 1u32, "WindowsServer2008"),
                    Self::WindowsServer2008R2 => serializer.serialize_unit_variant("OsType", 2u32, "WindowsServer2008R2"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "Type of support"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "SupportType")]
        pub enum SupportType {
            SupplementalServicing,
            PremiumAssurance,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for SupportType {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for SupportType {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for SupportType {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::SupplementalServicing => serializer.serialize_unit_variant("SupportType", 0u32, "SupplementalServicing"),
                    Self::PremiumAssurance => serializer.serialize_unit_variant("SupportType", 1u32, "PremiumAssurance"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        impl Default for SupportType {
            fn default() -> Self {
                Self::SupplementalServicing
            }
        }
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ProvisioningState")]
        pub enum ProvisioningState {
            Succeeded,
            Failed,
            Canceled,
            Accepted,
            Provisioning,
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
                    Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                    Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                    Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                    Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Accepted"),
                    Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Provisioning"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "List of MAK keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MultipleActivationKeyList {
    #[doc = "List of MAK keys."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MultipleActivationKey>,
    #[doc = "Link to the next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MultipleActivationKeyList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MultipleActivationKeyList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MAK key details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MultipleActivationKeyUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl MultipleActivationKeyUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "REST API operation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Meta data about operation used for display in portal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Meta data about operation used for display in portal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of available REST API operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "Link to the next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationList {
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
