#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The resource definition of this association."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Association {
    #[doc = "The association id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The association name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The association type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The properties of the association."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<association::Properties>,
}
impl Association {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod association {
    use super::*;
    #[doc = "The properties of the association."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The REST resource instance of the target resource for this association."]
        #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
        pub target_resource_id: Option<String>,
        #[doc = "The provisioning state of the association."]
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
        #[doc = "The provisioning state of the association."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ProvisioningState")]
        pub enum ProvisioningState {
            Accepted,
            Deleting,
            Running,
            Succeeded,
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
                    Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Deleting"),
                    Self::Running => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Running"),
                    Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                    Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "List of associations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssociationsList {
    #[doc = "The array of associations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Association>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssociationsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AssociationsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The route definition for an action implemented by the custom resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRpActionRouteDefinition {
    #[serde(flatten)]
    pub custom_rp_route_definition: CustomRpRouteDefinition,
    #[doc = "The routing types that are supported for action requests."]
    #[serde(rename = "routingType", default, skip_serializing_if = "Option::is_none")]
    pub routing_type: Option<custom_rp_action_route_definition::RoutingType>,
}
impl CustomRpActionRouteDefinition {
    pub fn new(custom_rp_route_definition: CustomRpRouteDefinition) -> Self {
        Self {
            custom_rp_route_definition,
            routing_type: None,
        }
    }
}
pub mod custom_rp_action_route_definition {
    use super::*;
    #[doc = "The routing types that are supported for action requests."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RoutingType")]
    pub enum RoutingType {
        Proxy,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RoutingType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RoutingType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RoutingType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Proxy => serializer.serialize_unit_variant("RoutingType", 0u32, "Proxy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A manifest file that defines the custom resource provider resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRpManifest {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The manifest for the custom resource provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<custom_rp_manifest::Properties>,
}
impl CustomRpManifest {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
pub mod custom_rp_manifest {
    use super::*;
    #[doc = "The manifest for the custom resource provider"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "A list of actions that the custom resource provider implements."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub actions: Vec<CustomRpActionRouteDefinition>,
        #[doc = "A list of resource types that the custom resource provider implements."]
        #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
        pub resource_types: Vec<CustomRpResourceTypeRouteDefinition>,
        #[doc = "A list of validations to run on the custom resource provider's requests."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub validations: Vec<CustomRpValidations>,
        #[doc = "The provisioning state of the resource provider."]
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
        #[doc = "The provisioning state of the resource provider."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "ProvisioningState")]
        pub enum ProvisioningState {
            Accepted,
            Deleting,
            Running,
            Succeeded,
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
                    Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Deleting"),
                    Self::Running => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Running"),
                    Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                    Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "The route definition for a resource implemented by the custom resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRpResourceTypeRouteDefinition {
    #[serde(flatten)]
    pub custom_rp_route_definition: CustomRpRouteDefinition,
    #[doc = "The routing types that are supported for resource requests."]
    #[serde(rename = "routingType", default, skip_serializing_if = "Option::is_none")]
    pub routing_type: Option<custom_rp_resource_type_route_definition::RoutingType>,
}
impl CustomRpResourceTypeRouteDefinition {
    pub fn new(custom_rp_route_definition: CustomRpRouteDefinition) -> Self {
        Self {
            custom_rp_route_definition,
            routing_type: None,
        }
    }
}
pub mod custom_rp_resource_type_route_definition {
    use super::*;
    #[doc = "The routing types that are supported for resource requests."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RoutingType")]
    pub enum RoutingType {
        Proxy,
        #[serde(rename = "Proxy,Cache")]
        ProxyCache,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RoutingType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RoutingType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RoutingType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Proxy => serializer.serialize_unit_variant("RoutingType", 0u32, "Proxy"),
                Self::ProxyCache => serializer.serialize_unit_variant("RoutingType", 1u32, "Proxy,Cache"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A route definition that defines an action or resource that can be interacted with through the custom resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRpRouteDefinition {
    #[doc = "The name of the route definition. This becomes the name for the ARM extension (e.g. '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.CustomProviders/resourceProviders/{resourceProviderName}/{name}')"]
    pub name: String,
    #[doc = "The route definition endpoint URI that the custom resource provider will proxy requests to. This can be in the form of a flat URI (e.g. 'https://testendpoint/') or can specify to route via a path (e.g. 'https://testendpoint/{requestPath}')"]
    pub endpoint: String,
}
impl CustomRpRouteDefinition {
    pub fn new(name: String, endpoint: String) -> Self {
        Self { name, endpoint }
    }
}
#[doc = "A validation to apply on custom resource provider requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRpValidations {
    #[doc = "The type of validation to run against a matching request."]
    #[serde(rename = "validationType", default, skip_serializing_if = "Option::is_none")]
    pub validation_type: Option<custom_rp_validations::ValidationType>,
    #[doc = "A link to the validation specification. The specification must be hosted on raw.githubusercontent.com."]
    pub specification: String,
}
impl CustomRpValidations {
    pub fn new(specification: String) -> Self {
        Self {
            validation_type: None,
            specification,
        }
    }
}
pub mod custom_rp_validations {
    use super::*;
    #[doc = "The type of validation to run against a matching request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ValidationType")]
    pub enum ValidationType {
        Swagger,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ValidationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ValidationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ValidationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Swagger => serializer.serialize_unit_variant("ValidationType", 0u32, "Swagger"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
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
#[doc = "List of custom resource providers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListByCustomRpManifest {
    #[doc = "The array of custom resource provider manifests."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomRpManifest>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListByCustomRpManifest {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListByCustomRpManifest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "Supported operations of this resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperation {
    #[doc = "Operation name, in format of {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<resource_provider_operation::Display>,
}
impl ResourceProviderOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_provider_operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Resource provider: Microsoft Custom Providers."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of this operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Results of the request to list operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationList {
    #[doc = "List of operations supported by this resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderOperation>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceProviderOperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceProviderOperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "custom resource provider update information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProvidersUpdate {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceProvidersUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
