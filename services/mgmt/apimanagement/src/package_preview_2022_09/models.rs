#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Paged AccessInformation list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessInformationCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AccessInformationContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessInformationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AccessInformationCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant Settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessInformationContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Tenant access information contract of the API Management service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessInformationContractProperties>,
}
impl AccessInformationContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant access information contract of the API Management service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessInformationContractProperties {
    #[doc = "Access Information type ('access' or 'gitAccess')"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Principal (User) Identifier."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Determines whether direct access is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl AccessInformationContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant access information update parameters of the API Management service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessInformationCreateParameterProperties {
    #[doc = "Principal (User) Identifier."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Primary access key. This property will not be filled on 'GET' operations! Use '/listSecrets' POST request to get the value."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "Secondary access key. This property will not be filled on 'GET' operations! Use '/listSecrets' POST request to get the value."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "Determines whether direct access is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl AccessInformationCreateParameterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant access information update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessInformationCreateParameters {
    #[doc = "Tenant access information update parameters of the API Management service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessInformationCreateParameterProperties>,
}
impl AccessInformationCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant access information contract of the API Management service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessInformationSecretsContract {
    #[doc = "Access Information type ('access' or 'gitAccess')"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Principal (User) Identifier."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Primary access key. This property will not be filled on 'GET' operations! Use '/listSecrets' POST request to get the value."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "Secondary access key. This property will not be filled on 'GET' operations! Use '/listSecrets' POST request to get the value."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "Determines whether direct access is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl AccessInformationSecretsContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant access information update parameters of the API Management service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessInformationUpdateParameterProperties {
    #[doc = "Determines whether direct access is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl AccessInformationUpdateParameterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant access information update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessInformationUpdateParameters {
    #[doc = "Tenant access information update parameters of the API Management service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessInformationUpdateParameterProperties>,
}
impl AccessInformationUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of an additional API Management resource location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdditionalLocation {
    #[doc = "The location name of the additional region among Azure Data center regions."]
    pub location: String,
    #[doc = "API Management service resource SKU properties."]
    pub sku: ApiManagementServiceSkuProperties,
    #[doc = "A list of availability zones denoting where the resource needs to come from."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zones: Vec<String>,
    #[doc = "Public Static Load Balanced IP addresses of the API Management service in the additional location. Available only for Basic, Standard, Premium and Isolated SKU."]
    #[serde(
        rename = "publicIPAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub public_ip_addresses: Vec<String>,
    #[doc = "Private Static Load Balanced IP addresses of the API Management service which is deployed in an Internal Virtual Network in a particular additional location. Available only for Basic, Standard, Premium and Isolated SKU."]
    #[serde(
        rename = "privateIPAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_ip_addresses: Vec<String>,
    #[doc = "Public Standard SKU IP V4 based IP address to be associated with Virtual Network deployed service in the location. Supported only for Premium SKU being deployed in Virtual Network."]
    #[serde(rename = "publicIpAddressId", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address_id: Option<String>,
    #[doc = "Configuration of a virtual network to which API Management service is deployed."]
    #[serde(rename = "virtualNetworkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_configuration: Option<VirtualNetworkConfiguration>,
    #[doc = "Gateway URL of the API Management service in the Region."]
    #[serde(rename = "gatewayRegionalUrl", default, skip_serializing_if = "Option::is_none")]
    pub gateway_regional_url: Option<String>,
    #[doc = "Property can be used to enable NAT Gateway for this API Management service."]
    #[serde(rename = "natGatewayState", default, skip_serializing_if = "Option::is_none")]
    pub nat_gateway_state: Option<additional_location::NatGatewayState>,
    #[doc = "Outbound public IPV4 address prefixes associated with NAT Gateway deployed service. Available only for Premium SKU on stv2 platform."]
    #[serde(
        rename = "outboundPublicIPAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub outbound_public_ip_addresses: Vec<String>,
    #[doc = "Property only valid for an Api Management service deployed in multiple locations. This can be used to disable the gateway in this additional location."]
    #[serde(rename = "disableGateway", default, skip_serializing_if = "Option::is_none")]
    pub disable_gateway: Option<bool>,
    #[doc = "Compute Platform Version running the service."]
    #[serde(rename = "platformVersion", default, skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<additional_location::PlatformVersion>,
}
impl AdditionalLocation {
    pub fn new(location: String, sku: ApiManagementServiceSkuProperties) -> Self {
        Self {
            location,
            sku,
            zones: Vec::new(),
            public_ip_addresses: Vec::new(),
            private_ip_addresses: Vec::new(),
            public_ip_address_id: None,
            virtual_network_configuration: None,
            gateway_regional_url: None,
            nat_gateway_state: None,
            outbound_public_ip_addresses: Vec::new(),
            disable_gateway: None,
            platform_version: None,
        }
    }
}
pub mod additional_location {
    use super::*;
    #[doc = "Property can be used to enable NAT Gateway for this API Management service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NatGatewayState")]
    pub enum NatGatewayState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NatGatewayState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NatGatewayState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NatGatewayState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("NatGatewayState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("NatGatewayState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for NatGatewayState {
        fn default() -> Self {
            Self::Disabled
        }
    }
    #[doc = "Compute Platform Version running the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PlatformVersion")]
    pub enum PlatformVersion {
        #[serde(rename = "undetermined")]
        Undetermined,
        #[serde(rename = "stv1")]
        Stv1,
        #[serde(rename = "stv2")]
        Stv2,
        #[serde(rename = "mtv1")]
        Mtv1,
        #[serde(rename = "stv2.1")]
        Stv21,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PlatformVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PlatformVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PlatformVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Undetermined => serializer.serialize_unit_variant("PlatformVersion", 0u32, "undetermined"),
                Self::Stv1 => serializer.serialize_unit_variant("PlatformVersion", 1u32, "stv1"),
                Self::Stv2 => serializer.serialize_unit_variant("PlatformVersion", 2u32, "stv2"),
                Self::Mtv1 => serializer.serialize_unit_variant("PlatformVersion", 3u32, "mtv1"),
                Self::Stv21 => serializer.serialize_unit_variant("PlatformVersion", 4u32, "stv2.1"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Paged API list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ApiContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApiCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API contact information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiContactInformation {
    #[doc = "The identifying name of the contact person/organization"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The URL pointing to the contact information. MUST be in the format of a URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The email address of the contact person/organization. MUST be in the format of an email address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
impl ApiContactInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "API Entity Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiContractProperties>,
}
impl ApiContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API Entity Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiContractProperties {
    #[serde(flatten)]
    pub api_entity_base_contract: ApiEntityBaseContract,
    #[doc = "API identifier of the source API."]
    #[serde(rename = "sourceApiId", default, skip_serializing_if = "Option::is_none")]
    pub source_api_id: Option<String>,
    #[doc = "API name. Must be 1 to 300 characters long."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Absolute URL of the backend service implementing this API. Cannot be more than 2000 characters long."]
    #[serde(rename = "serviceUrl", default, skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
    #[doc = "Relative URL uniquely identifying this API and all of its resource paths within the API Management service instance. It is appended to the API endpoint base URL specified during the service instance creation to form a public URL for this API."]
    pub path: String,
    #[doc = "Describes on which protocols the operations in this API can be invoked."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub protocols: Vec<String>,
    #[doc = "An API Version Set contains the common configuration for a set of API Versions relating "]
    #[serde(rename = "apiVersionSet", default, skip_serializing_if = "Option::is_none")]
    pub api_version_set: Option<ApiVersionSetContractDetails>,
}
impl ApiContractProperties {
    pub fn new(path: String) -> Self {
        Self {
            api_entity_base_contract: ApiEntityBaseContract::default(),
            source_api_id: None,
            display_name: None,
            service_url: None,
            path,
            protocols: Vec::new(),
            api_version_set: None,
        }
    }
}
#[doc = "API update contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiContractUpdateProperties {
    #[serde(flatten)]
    pub api_entity_base_contract: ApiEntityBaseContract,
    #[doc = "API name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Absolute URL of the backend service implementing this API."]
    #[serde(rename = "serviceUrl", default, skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
    #[doc = "Relative URL uniquely identifying this API and all of its resource paths within the API Management service instance. It is appended to the API endpoint base URL specified during the service instance creation to form a public URL for this API."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Describes on which protocols the operations in this API can be invoked."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub protocols: Vec<String>,
}
impl ApiContractUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API Create or Update Parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiCreateOrUpdateParameter {
    #[doc = "API Create or Update Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiCreateOrUpdateProperties>,
}
impl ApiCreateOrUpdateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API Create or Update Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiCreateOrUpdateProperties {
    #[serde(flatten)]
    pub api_contract_properties: ApiContractProperties,
    #[doc = "Content value when Importing an API."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Format of the Content in which the API is getting imported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<api_create_or_update_properties::Format>,
    #[doc = "Criteria to limit import of WSDL to a subset of the document."]
    #[serde(rename = "wsdlSelector", default, skip_serializing_if = "Option::is_none")]
    pub wsdl_selector: Option<api_create_or_update_properties::WsdlSelector>,
    #[doc = "Type of API to create. \n * `http` creates a REST API \n * `soap` creates a SOAP pass-through API  \n * `websocket` creates websocket API \n * `graphql` creates GraphQL API."]
    #[serde(rename = "apiType", default, skip_serializing_if = "Option::is_none")]
    pub api_type: Option<api_create_or_update_properties::ApiType>,
    #[doc = "Strategy of translating required query parameters to template ones. By default has value 'template'. Possible values: 'template', 'query'"]
    #[serde(rename = "translateRequiredQueryParameters", default, skip_serializing_if = "Option::is_none")]
    pub translate_required_query_parameters: Option<api_create_or_update_properties::TranslateRequiredQueryParameters>,
}
impl ApiCreateOrUpdateProperties {
    pub fn new(api_contract_properties: ApiContractProperties) -> Self {
        Self {
            api_contract_properties,
            value: None,
            format: None,
            wsdl_selector: None,
            api_type: None,
            translate_required_query_parameters: None,
        }
    }
}
pub mod api_create_or_update_properties {
    use super::*;
    #[doc = "Format of the Content in which the API is getting imported."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Format")]
    pub enum Format {
        #[serde(rename = "wadl-xml")]
        WadlXml,
        #[serde(rename = "wadl-link-json")]
        WadlLinkJson,
        #[serde(rename = "swagger-json")]
        SwaggerJson,
        #[serde(rename = "swagger-link-json")]
        SwaggerLinkJson,
        #[serde(rename = "wsdl")]
        Wsdl,
        #[serde(rename = "wsdl-link")]
        WsdlLink,
        #[serde(rename = "openapi")]
        Openapi,
        #[serde(rename = "openapi+json")]
        OpenapiJson,
        #[serde(rename = "openapi-link")]
        OpenapiLink,
        #[serde(rename = "openapi+json-link")]
        OpenapiJsonLink,
        #[serde(rename = "graphql-link")]
        GraphqlLink,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Format {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Format {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Format {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::WadlXml => serializer.serialize_unit_variant("Format", 0u32, "wadl-xml"),
                Self::WadlLinkJson => serializer.serialize_unit_variant("Format", 1u32, "wadl-link-json"),
                Self::SwaggerJson => serializer.serialize_unit_variant("Format", 2u32, "swagger-json"),
                Self::SwaggerLinkJson => serializer.serialize_unit_variant("Format", 3u32, "swagger-link-json"),
                Self::Wsdl => serializer.serialize_unit_variant("Format", 4u32, "wsdl"),
                Self::WsdlLink => serializer.serialize_unit_variant("Format", 5u32, "wsdl-link"),
                Self::Openapi => serializer.serialize_unit_variant("Format", 6u32, "openapi"),
                Self::OpenapiJson => serializer.serialize_unit_variant("Format", 7u32, "openapi+json"),
                Self::OpenapiLink => serializer.serialize_unit_variant("Format", 8u32, "openapi-link"),
                Self::OpenapiJsonLink => serializer.serialize_unit_variant("Format", 9u32, "openapi+json-link"),
                Self::GraphqlLink => serializer.serialize_unit_variant("Format", 10u32, "graphql-link"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Criteria to limit import of WSDL to a subset of the document."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct WsdlSelector {
        #[doc = "Name of service to import from WSDL"]
        #[serde(rename = "wsdlServiceName", default, skip_serializing_if = "Option::is_none")]
        pub wsdl_service_name: Option<String>,
        #[doc = "Name of endpoint(port) to import from WSDL"]
        #[serde(rename = "wsdlEndpointName", default, skip_serializing_if = "Option::is_none")]
        pub wsdl_endpoint_name: Option<String>,
    }
    impl WsdlSelector {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Type of API to create. \n * `http` creates a REST API \n * `soap` creates a SOAP pass-through API  \n * `websocket` creates websocket API \n * `graphql` creates GraphQL API."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ApiType")]
    pub enum ApiType {
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "soap")]
        Soap,
        #[serde(rename = "websocket")]
        Websocket,
        #[serde(rename = "graphql")]
        Graphql,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ApiType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ApiType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ApiType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Http => serializer.serialize_unit_variant("ApiType", 0u32, "http"),
                Self::Soap => serializer.serialize_unit_variant("ApiType", 1u32, "soap"),
                Self::Websocket => serializer.serialize_unit_variant("ApiType", 2u32, "websocket"),
                Self::Graphql => serializer.serialize_unit_variant("ApiType", 3u32, "graphql"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Strategy of translating required query parameters to template ones. By default has value 'template'. Possible values: 'template', 'query'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TranslateRequiredQueryParameters")]
    pub enum TranslateRequiredQueryParameters {
        #[serde(rename = "template")]
        Template,
        #[serde(rename = "query")]
        Query,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TranslateRequiredQueryParameters {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TranslateRequiredQueryParameters {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TranslateRequiredQueryParameters {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Template => serializer.serialize_unit_variant("TranslateRequiredQueryParameters", 0u32, "template"),
                Self::Query => serializer.serialize_unit_variant("TranslateRequiredQueryParameters", 1u32, "query"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "API base contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiEntityBaseContract {
    #[doc = "Description of the API. May include HTML formatting tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "API Authentication Settings."]
    #[serde(rename = "authenticationSettings", default, skip_serializing_if = "Option::is_none")]
    pub authentication_settings: Option<AuthenticationSettingsContract>,
    #[doc = "Subscription key parameter names details."]
    #[serde(rename = "subscriptionKeyParameterNames", default, skip_serializing_if = "Option::is_none")]
    pub subscription_key_parameter_names: Option<SubscriptionKeyParameterNamesContract>,
    #[doc = "Type of API."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<api_entity_base_contract::Type>,
    #[doc = "Describes the revision of the API. If no value is provided, default revision 1 is created"]
    #[serde(rename = "apiRevision", default, skip_serializing_if = "Option::is_none")]
    pub api_revision: Option<String>,
    #[doc = "Indicates the version identifier of the API if the API is versioned"]
    #[serde(rename = "apiVersion", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[doc = "Indicates if API revision is current api revision."]
    #[serde(rename = "isCurrent", default, skip_serializing_if = "Option::is_none")]
    pub is_current: Option<bool>,
    #[doc = "Indicates if API revision is accessible via the gateway."]
    #[serde(rename = "isOnline", default, skip_serializing_if = "Option::is_none")]
    pub is_online: Option<bool>,
    #[doc = "Description of the API Revision."]
    #[serde(rename = "apiRevisionDescription", default, skip_serializing_if = "Option::is_none")]
    pub api_revision_description: Option<String>,
    #[doc = "Description of the API Version."]
    #[serde(rename = "apiVersionDescription", default, skip_serializing_if = "Option::is_none")]
    pub api_version_description: Option<String>,
    #[doc = "A resource identifier for the related ApiVersionSet."]
    #[serde(rename = "apiVersionSetId", default, skip_serializing_if = "Option::is_none")]
    pub api_version_set_id: Option<String>,
    #[doc = "Specifies whether an API or Product subscription is required for accessing the API."]
    #[serde(rename = "subscriptionRequired", default, skip_serializing_if = "Option::is_none")]
    pub subscription_required: Option<bool>,
    #[doc = " A URL to the Terms of Service for the API. MUST be in the format of a URL."]
    #[serde(rename = "termsOfServiceUrl", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<String>,
    #[doc = "API contact information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<ApiContactInformation>,
    #[doc = "API license information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<ApiLicenseInformation>,
}
impl ApiEntityBaseContract {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_entity_base_contract {
    use super::*;
    #[doc = "Type of API."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "soap")]
        Soap,
        #[serde(rename = "websocket")]
        Websocket,
        #[serde(rename = "graphql")]
        Graphql,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Http => serializer.serialize_unit_variant("Type", 0u32, "http"),
                Self::Soap => serializer.serialize_unit_variant("Type", 1u32, "soap"),
                Self::Websocket => serializer.serialize_unit_variant("Type", 2u32, "websocket"),
                Self::Graphql => serializer.serialize_unit_variant("Type", 3u32, "graphql"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "API Export result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiExportResult {
    #[doc = "ResourceId of the API which was exported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Format in which the API Details are exported to the Storage Blob with Sas Key valid for 5 minutes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<api_export_result::Format>,
    #[doc = "The object defining the schema of the exported API Detail"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<api_export_result::Value>,
}
impl ApiExportResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_export_result {
    use super::*;
    #[doc = "Format in which the API Details are exported to the Storage Blob with Sas Key valid for 5 minutes."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Format")]
    pub enum Format {
        #[serde(rename = "swagger-link-json")]
        SwaggerLinkJson,
        #[serde(rename = "wadl-link-json")]
        WadlLinkJson,
        #[serde(rename = "wsdl-link+xml")]
        WsdlLinkXml,
        #[serde(rename = "openapi-link")]
        OpenapiLink,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Format {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Format {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Format {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SwaggerLinkJson => serializer.serialize_unit_variant("Format", 0u32, "swagger-link-json"),
                Self::WadlLinkJson => serializer.serialize_unit_variant("Format", 1u32, "wadl-link-json"),
                Self::WsdlLinkXml => serializer.serialize_unit_variant("Format", 2u32, "wsdl-link+xml"),
                Self::OpenapiLink => serializer.serialize_unit_variant("Format", 3u32, "openapi-link"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The object defining the schema of the exported API Detail"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Value {
        #[doc = "Link to the Storage Blob containing the result of the export operation. The Blob Uri is only valid for 5 minutes."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub link: Option<String>,
    }
    impl Value {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "API license information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiLicenseInformation {
    #[doc = "The license name used for the API"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A URL to the license used for the API. MUST be in the format of a URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ApiLicenseInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameter supplied to the Apply Network configuration operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementServiceApplyNetworkConfigurationParameters {
    #[doc = "Location of the Api Management service to update for a multi-region service. For a service deployed in a single region, this parameter is not required."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl ApiManagementServiceApplyNetworkConfigurationParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Backup/Restore of an API Management service operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiManagementServiceBackupRestoreParameters {
    #[doc = "The name of the Azure storage account (used to place/retrieve the backup)."]
    #[serde(rename = "storageAccount")]
    pub storage_account: String,
    #[doc = "The name of the blob container (used to place/retrieve the backup)."]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "The name of the backup file to create/retrieve."]
    #[serde(rename = "backupName")]
    pub backup_name: String,
    #[doc = "The type of access to be used for the storage account."]
    #[serde(rename = "accessType", default, skip_serializing_if = "Option::is_none")]
    pub access_type: Option<api_management_service_backup_restore_parameters::AccessType>,
    #[doc = "Storage account access key. Required only if `accessType` is set to `AccessKey`."]
    #[serde(rename = "accessKey", default, skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    #[doc = "The Client ID of user assigned managed identity. Required only if `accessType` is set to `UserAssignedManagedIdentity`."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl ApiManagementServiceBackupRestoreParameters {
    pub fn new(storage_account: String, container_name: String, backup_name: String) -> Self {
        Self {
            storage_account,
            container_name,
            backup_name,
            access_type: None,
            access_key: None,
            client_id: None,
        }
    }
}
pub mod api_management_service_backup_restore_parameters {
    use super::*;
    #[doc = "The type of access to be used for the storage account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccessType")]
    pub enum AccessType {
        AccessKey,
        SystemAssignedManagedIdentity,
        UserAssignedManagedIdentity,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccessType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccessType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccessType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AccessKey => serializer.serialize_unit_variant("AccessType", 0u32, "AccessKey"),
                Self::SystemAssignedManagedIdentity => {
                    serializer.serialize_unit_variant("AccessType", 1u32, "SystemAssignedManagedIdentity")
                }
                Self::UserAssignedManagedIdentity => serializer.serialize_unit_variant("AccessType", 2u32, "UserAssignedManagedIdentity"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for AccessType {
        fn default() -> Self {
            Self::AccessKey
        }
    }
}
#[doc = "Base Properties of an API Management service resource description."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementServiceBaseProperties {
    #[doc = "Email address from which the notification will be sent."]
    #[serde(rename = "notificationSenderEmail", default, skip_serializing_if = "Option::is_none")]
    pub notification_sender_email: Option<String>,
    #[doc = "The current provisioning state of the API Management service which can be one of the following: Created/Activating/Succeeded/Updating/Failed/Stopped/Terminating/TerminationFailed/Deleted."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The provisioning state of the API Management service, which is targeted by the long running operation started on the service."]
    #[serde(rename = "targetProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub target_provisioning_state: Option<String>,
    #[doc = "Creation UTC date of the API Management service.The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard."]
    #[serde(rename = "createdAtUtc", default, with = "azure_core::date::rfc3339::option")]
    pub created_at_utc: Option<time::OffsetDateTime>,
    #[doc = "Gateway URL of the API Management service."]
    #[serde(rename = "gatewayUrl", default, skip_serializing_if = "Option::is_none")]
    pub gateway_url: Option<String>,
    #[doc = "Gateway URL of the API Management service in the Default Region."]
    #[serde(rename = "gatewayRegionalUrl", default, skip_serializing_if = "Option::is_none")]
    pub gateway_regional_url: Option<String>,
    #[doc = "Publisher portal endpoint Url of the API Management service."]
    #[serde(rename = "portalUrl", default, skip_serializing_if = "Option::is_none")]
    pub portal_url: Option<String>,
    #[doc = "Management API endpoint URL of the API Management service."]
    #[serde(rename = "managementApiUrl", default, skip_serializing_if = "Option::is_none")]
    pub management_api_url: Option<String>,
    #[doc = "SCM endpoint URL of the API Management service."]
    #[serde(rename = "scmUrl", default, skip_serializing_if = "Option::is_none")]
    pub scm_url: Option<String>,
    #[doc = "DEveloper Portal endpoint URL of the API Management service."]
    #[serde(rename = "developerPortalUrl", default, skip_serializing_if = "Option::is_none")]
    pub developer_portal_url: Option<String>,
    #[doc = "Custom hostname configuration of the API Management service."]
    #[serde(
        rename = "hostnameConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hostname_configurations: Vec<HostnameConfiguration>,
    #[doc = "Public Static Load Balanced IP addresses of the API Management service in Primary region. Available only for Basic, Standard, Premium and Isolated SKU."]
    #[serde(
        rename = "publicIPAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub public_ip_addresses: Vec<String>,
    #[doc = "Private Static Load Balanced IP addresses of the API Management service in Primary region which is deployed in an Internal Virtual Network. Available only for Basic, Standard, Premium and Isolated SKU."]
    #[serde(
        rename = "privateIPAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_ip_addresses: Vec<String>,
    #[doc = "Public Standard SKU IP V4 based IP address to be associated with Virtual Network deployed service in the region. Supported only for Developer and Premium SKU being deployed in Virtual Network."]
    #[serde(rename = "publicIpAddressId", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address_id: Option<String>,
    #[doc = "Whether or not public endpoint access is allowed for this API Management service.  Value is optional but if passed in, must be 'Enabled' or 'Disabled'. If 'Disabled', private endpoints are the exclusive access method. Default value is 'Enabled'"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<api_management_service_base_properties::PublicNetworkAccess>,
    #[doc = "Configuration of a virtual network to which API Management service is deployed."]
    #[serde(rename = "virtualNetworkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_configuration: Option<VirtualNetworkConfiguration>,
    #[doc = "Additional datacenter locations of the API Management service."]
    #[serde(
        rename = "additionalLocations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_locations: Vec<AdditionalLocation>,
    #[doc = "Custom properties of the API Management service.</br>Setting `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.TripleDes168` will disable the cipher TLS_RSA_WITH_3DES_EDE_CBC_SHA for all TLS(1.0, 1.1 and 1.2).</br>Setting `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Protocols.Tls11` can be used to disable just TLS 1.1.</br>Setting `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Protocols.Tls10` can be used to disable TLS 1.0 on an API Management service.</br>Setting `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Backend.Protocols.Tls11` can be used to disable just TLS 1.1 for communications with backends.</br>Setting `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Backend.Protocols.Tls10` can be used to disable TLS 1.0 for communications with backends.</br>Setting `Microsoft.WindowsAzure.ApiManagement.Gateway.Protocols.Server.Http2` can be used to enable HTTP2 protocol on an API Management service.</br>Not specifying any of these properties on PATCH operation will reset omitted properties' values to their defaults. For all the settings except Http2 the default value is `True` if the service was created on or before April 1, 2018 and `False` otherwise. Http2 setting's default value is `False`.</br></br>You can disable any of the following ciphers by using settings `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.[cipher_name]`: TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA, TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA, TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA, TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA, TLS_RSA_WITH_AES_128_GCM_SHA256, TLS_RSA_WITH_AES_256_CBC_SHA256, TLS_RSA_WITH_AES_128_CBC_SHA256, TLS_RSA_WITH_AES_256_CBC_SHA, TLS_RSA_WITH_AES_128_CBC_SHA. For example, `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.TLS_RSA_WITH_AES_128_CBC_SHA256`:`false`. The default value is `true` for them.</br> Note: The following ciphers can't be disabled since they are required by internal platform components: TLS_AES_256_GCM_SHA384,TLS_AES_128_GCM_SHA256,TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256,TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384,TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256,TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384,TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256"]
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<serde_json::Value>,
    #[doc = "List of Certificates that need to be installed in the API Management service. Max supported certificates that can be installed is 10."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub certificates: Vec<CertificateConfiguration>,
    #[doc = "Property only meant to be used for Consumption SKU Service. This enforces a client certificate to be presented on each request to the gateway. This also enables the ability to authenticate the certificate in the policy on the gateway."]
    #[serde(rename = "enableClientCertificate", default, skip_serializing_if = "Option::is_none")]
    pub enable_client_certificate: Option<bool>,
    #[doc = "Property can be used to enable NAT Gateway for this API Management service."]
    #[serde(rename = "natGatewayState", default, skip_serializing_if = "Option::is_none")]
    pub nat_gateway_state: Option<api_management_service_base_properties::NatGatewayState>,
    #[doc = "Outbound public IPV4 address prefixes associated with NAT Gateway deployed service. Available only for Premium SKU on stv2 platform."]
    #[serde(
        rename = "outboundPublicIPAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub outbound_public_ip_addresses: Vec<String>,
    #[doc = "Property only valid for an Api Management service deployed in multiple locations. This can be used to disable the gateway in master region."]
    #[serde(rename = "disableGateway", default, skip_serializing_if = "Option::is_none")]
    pub disable_gateway: Option<bool>,
    #[doc = "The type of VPN in which API Management service needs to be configured in. None (Default Value) means the API Management service is not part of any Virtual Network, External means the API Management deployment is set up inside a Virtual Network having an Internet Facing Endpoint, and Internal means that API Management deployment is setup inside a Virtual Network having an Intranet Facing Endpoint only."]
    #[serde(rename = "virtualNetworkType", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_type: Option<api_management_service_base_properties::VirtualNetworkType>,
    #[doc = "Control Plane Apis version constraint for the API Management service."]
    #[serde(rename = "apiVersionConstraint", default, skip_serializing_if = "Option::is_none")]
    pub api_version_constraint: Option<ApiVersionConstraint>,
    #[doc = "Undelete Api Management Service if it was previously soft-deleted. If this flag is specified and set to True all other properties will be ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore: Option<bool>,
    #[doc = "List of Private Endpoint Connections of this service."]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<RemotePrivateEndpointConnectionWrapper>,
    #[doc = "Compute Platform Version running the service in this location."]
    #[serde(rename = "platformVersion", default, skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<api_management_service_base_properties::PlatformVersion>,
}
impl ApiManagementServiceBaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_management_service_base_properties {
    use super::*;
    #[doc = "Whether or not public endpoint access is allowed for this API Management service.  Value is optional but if passed in, must be 'Enabled' or 'Disabled'. If 'Disabled', private endpoints are the exclusive access method. Default value is 'Enabled'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Property can be used to enable NAT Gateway for this API Management service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NatGatewayState")]
    pub enum NatGatewayState {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NatGatewayState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NatGatewayState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NatGatewayState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("NatGatewayState", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("NatGatewayState", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for NatGatewayState {
        fn default() -> Self {
            Self::Disabled
        }
    }
    #[doc = "The type of VPN in which API Management service needs to be configured in. None (Default Value) means the API Management service is not part of any Virtual Network, External means the API Management deployment is set up inside a Virtual Network having an Internet Facing Endpoint, and Internal means that API Management deployment is setup inside a Virtual Network having an Intranet Facing Endpoint only."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VirtualNetworkType")]
    pub enum VirtualNetworkType {
        None,
        External,
        Internal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VirtualNetworkType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VirtualNetworkType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VirtualNetworkType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("VirtualNetworkType", 0u32, "None"),
                Self::External => serializer.serialize_unit_variant("VirtualNetworkType", 1u32, "External"),
                Self::Internal => serializer.serialize_unit_variant("VirtualNetworkType", 2u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for VirtualNetworkType {
        fn default() -> Self {
            Self::None
        }
    }
    #[doc = "Compute Platform Version running the service in this location."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PlatformVersion")]
    pub enum PlatformVersion {
        #[serde(rename = "undetermined")]
        Undetermined,
        #[serde(rename = "stv1")]
        Stv1,
        #[serde(rename = "stv2")]
        Stv2,
        #[serde(rename = "mtv1")]
        Mtv1,
        #[serde(rename = "stv2.1")]
        Stv21,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PlatformVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PlatformVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PlatformVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Undetermined => serializer.serialize_unit_variant("PlatformVersion", 0u32, "undetermined"),
                Self::Stv1 => serializer.serialize_unit_variant("PlatformVersion", 1u32, "stv1"),
                Self::Stv2 => serializer.serialize_unit_variant("PlatformVersion", 2u32, "stv2"),
                Self::Mtv1 => serializer.serialize_unit_variant("PlatformVersion", 3u32, "mtv1"),
                Self::Stv21 => serializer.serialize_unit_variant("PlatformVersion", 4u32, "stv2.1"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Parameters supplied to the CheckNameAvailability operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiManagementServiceCheckNameAvailabilityParameters {
    #[doc = "The name to check for availability."]
    pub name: String,
}
impl ApiManagementServiceCheckNameAvailabilityParameters {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "Response of the GetDomainOwnershipIdentifier operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementServiceGetDomainOwnershipIdentifierResult {
    #[doc = "The domain ownership identifier value."]
    #[serde(rename = "domainOwnershipIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub domain_ownership_identifier: Option<String>,
}
impl ApiManagementServiceGetDomainOwnershipIdentifierResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of the GetSsoToken operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementServiceGetSsoTokenResult {
    #[doc = "Redirect URL to the Publisher Portal containing the SSO token."]
    #[serde(rename = "redirectUri", default, skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
}
impl ApiManagementServiceGetSsoTokenResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity properties of the Api Management service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiManagementServiceIdentity {
    #[doc = "The type of identity used for the resource. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the service."]
    #[serde(rename = "type")]
    pub type_: api_management_service_identity::Type,
    #[doc = "The principal id of the identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client tenant id of the identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The list of user identities associated with the resource. The user identity \r\ndictionary key references will be ARM resource ids in the form: \r\n'/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/\r\n    providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ApiManagementServiceIdentity {
    pub fn new(type_: api_management_service_identity::Type) -> Self {
        Self {
            type_,
            principal_id: None,
            tenant_id: None,
            user_assigned_identities: None,
        }
    }
}
pub mod api_management_service_identity {
    use super::*;
    #[doc = "The type of identity used for the resource. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 0u32, "SystemAssigned"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 1u32, "UserAssigned"),
                Self::SystemAssignedUserAssigned => serializer.serialize_unit_variant("Type", 2u32, "SystemAssigned, UserAssigned"),
                Self::None => serializer.serialize_unit_variant("Type", 3u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response of the List API Management services operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiManagementServiceListResult {
    #[doc = "Result of the List API Management services operation."]
    pub value: Vec<ApiManagementServiceResource>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of API Management services."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiManagementServiceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApiManagementServiceListResult {
    pub fn new(value: Vec<ApiManagementServiceResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Response of the CheckNameAvailability operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementServiceNameAvailabilityResult {
    #[doc = "True if the name is available and can be used to create a new API Management service; otherwise false."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "If reason == invalid, provide the user with the reason why the given name is invalid, and provide the resource naming requirements so that the user can select a valid name. If reason == AlreadyExists, explain that <resourceName> is already in use, and direct them to select a different name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Invalid indicates the name provided does not match the resource provider’s naming requirements (incorrect length, unsupported characters, etc.)  AlreadyExists indicates that the name is already in use and is therefore unavailable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<api_management_service_name_availability_result::Reason>,
}
impl ApiManagementServiceNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_management_service_name_availability_result {
    use super::*;
    #[doc = "Invalid indicates the name provided does not match the resource provider’s naming requirements (incorrect length, unsupported characters, etc.)  AlreadyExists indicates that the name is already in use and is therefore unavailable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Valid,
        Invalid,
        AlreadyExists,
    }
}
#[doc = "Properties of an API Management service resource description."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiManagementServiceProperties {
    #[serde(flatten)]
    pub api_management_service_base_properties: ApiManagementServiceBaseProperties,
    #[doc = "Publisher email."]
    #[serde(rename = "publisherEmail")]
    pub publisher_email: String,
    #[doc = "Publisher name."]
    #[serde(rename = "publisherName")]
    pub publisher_name: String,
}
impl ApiManagementServiceProperties {
    pub fn new(publisher_email: String, publisher_name: String) -> Self {
        Self {
            api_management_service_base_properties: ApiManagementServiceBaseProperties::default(),
            publisher_email,
            publisher_name,
        }
    }
}
#[doc = "A single API Management service resource in List or Get response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiManagementServiceResource {
    #[serde(flatten)]
    pub apim_resource: ApimResource,
    #[doc = "Properties of an API Management service resource description."]
    pub properties: ApiManagementServiceProperties,
    #[doc = "API Management service resource SKU properties."]
    pub sku: ApiManagementServiceSkuProperties,
    #[doc = "Identity properties of the Api Management service resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ApiManagementServiceIdentity>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Resource location."]
    pub location: String,
    #[doc = "ETag of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "A list of availability zones denoting where the resource needs to come from."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zones: Vec<String>,
}
impl ApiManagementServiceResource {
    pub fn new(properties: ApiManagementServiceProperties, sku: ApiManagementServiceSkuProperties, location: String) -> Self {
        Self {
            apim_resource: ApimResource::default(),
            properties,
            sku,
            identity: None,
            system_data: None,
            location,
            etag: None,
            zones: Vec::new(),
        }
    }
}
#[doc = "API Management service resource SKU properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiManagementServiceSkuProperties {
    #[doc = "Name of the Sku."]
    pub name: api_management_service_sku_properties::Name,
    #[doc = "Capacity of the SKU (number of deployed units of the SKU). For Consumption SKU capacity must be specified as 0."]
    pub capacity: i32,
}
impl ApiManagementServiceSkuProperties {
    pub fn new(name: api_management_service_sku_properties::Name, capacity: i32) -> Self {
        Self { name, capacity }
    }
}
pub mod api_management_service_sku_properties {
    use super::*;
    #[doc = "Name of the Sku."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Developer,
        Standard,
        Premium,
        Basic,
        Consumption,
        Isolated,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Developer => serializer.serialize_unit_variant("Name", 0u32, "Developer"),
                Self::Standard => serializer.serialize_unit_variant("Name", 1u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("Name", 2u32, "Premium"),
                Self::Basic => serializer.serialize_unit_variant("Name", 3u32, "Basic"),
                Self::Consumption => serializer.serialize_unit_variant("Name", 4u32, "Consumption"),
                Self::Isolated => serializer.serialize_unit_variant("Name", 5u32, "Isolated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Parameter supplied to Update Api Management Service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementServiceUpdateParameters {
    #[serde(flatten)]
    pub apim_resource: ApimResource,
    #[doc = "Properties of an API Management service resource description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiManagementServiceUpdateProperties>,
    #[doc = "API Management service resource SKU properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ApiManagementServiceSkuProperties>,
    #[doc = "Identity properties of the Api Management service resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ApiManagementServiceIdentity>,
    #[doc = "ETag of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "A list of availability zones denoting where the resource needs to come from."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zones: Vec<String>,
}
impl ApiManagementServiceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an API Management service resource description."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementServiceUpdateProperties {
    #[serde(flatten)]
    pub api_management_service_base_properties: ApiManagementServiceBaseProperties,
    #[doc = "Publisher email."]
    #[serde(rename = "publisherEmail", default, skip_serializing_if = "Option::is_none")]
    pub publisher_email: Option<String>,
    #[doc = "Publisher name."]
    #[serde(rename = "publisherName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
}
impl ApiManagementServiceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an available ApiManagement SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementSku {
    #[doc = "The type of resource the SKU applies to."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The name of SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the tier of virtual machines in a scale set.<br /><br /> Possible Values:<br /><br /> **Standard**<br /><br /> **Basic**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The Size of the SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The Family of this particular SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The Kind of resources that are supported in this SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Describes scaling information of a SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<ApiManagementSkuCapacity>,
    #[doc = "The set of locations that the SKU is available."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub locations: Vec<String>,
    #[doc = "A list of locations and availability zones in those locations where the SKU is available."]
    #[serde(
        rename = "locationInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub location_info: Vec<ApiManagementSkuLocationInfo>,
    #[doc = "The api versions that support this SKU."]
    #[serde(
        rename = "apiVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub api_versions: Vec<String>,
    #[doc = "Metadata for retrieving price info."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub costs: Vec<ApiManagementSkuCosts>,
    #[doc = "A name value pair to describe the capability."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub capabilities: Vec<ApiManagementSkuCapabilities>,
    #[doc = "The restrictions because of which SKU cannot be used. This is empty if there are no restrictions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub restrictions: Vec<ApiManagementSkuRestrictions>,
}
impl ApiManagementSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes The SKU capabilities object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementSkuCapabilities {
    #[doc = "An invariant to describe the feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "An invariant if the feature is measured by quantity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ApiManagementSkuCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes scaling information of a SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementSkuCapacity {
    #[doc = "The minimum capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,
    #[doc = "The maximum capacity that can be set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
    #[doc = "The default capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i32>,
    #[doc = "The scale type applicable to the sku."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<api_management_sku_capacity::ScaleType>,
}
impl ApiManagementSkuCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_management_sku_capacity {
    use super::*;
    #[doc = "The scale type applicable to the sku."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScaleType {
        Automatic,
        Manual,
        None,
    }
}
#[doc = "Describes metadata for retrieving price info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementSkuCosts {
    #[doc = "Used for querying price from commerce."]
    #[serde(rename = "meterID", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "The multiplier is needed to extend the base metered cost."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "An invariant to show the extended unit."]
    #[serde(rename = "extendedUnit", default, skip_serializing_if = "Option::is_none")]
    pub extended_unit: Option<String>,
}
impl ApiManagementSkuCosts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementSkuLocationInfo {
    #[doc = "Location of the SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "List of availability zones where the SKU is supported."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zones: Vec<String>,
    #[doc = "Details of capabilities available to a SKU in specific zones."]
    #[serde(
        rename = "zoneDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zone_details: Vec<ApiManagementSkuZoneDetails>,
}
impl ApiManagementSkuLocationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementSkuRestrictionInfo {
    #[doc = "Locations where the SKU is restricted"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub locations: Vec<String>,
    #[doc = "List of availability zones where the SKU is restricted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zones: Vec<String>,
}
impl ApiManagementSkuRestrictionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes scaling information of a SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementSkuRestrictions {
    #[doc = "The type of restrictions."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<api_management_sku_restrictions::Type>,
    #[doc = "The value of restrictions. If the restriction type is set to location. This would be different locations where the SKU is restricted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
    #[serde(rename = "restrictionInfo", default, skip_serializing_if = "Option::is_none")]
    pub restriction_info: Option<ApiManagementSkuRestrictionInfo>,
    #[doc = "The reason for restriction."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<api_management_sku_restrictions::ReasonCode>,
}
impl ApiManagementSkuRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_management_sku_restrictions {
    use super::*;
    #[doc = "The type of restrictions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Location,
        Zone,
    }
    #[doc = "The reason for restriction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
    }
}
#[doc = "Describes The zonal capabilities of a SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiManagementSkuZoneDetails {
    #[doc = "The set of zones that the SKU is available in with the specified capabilities."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub name: Vec<String>,
    #[doc = "A list of capabilities that are available for the SKU in the specified list of zones."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub capabilities: Vec<ApiManagementSkuCapabilities>,
}
impl ApiManagementSkuZoneDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Resource Skus operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiManagementSkusResult {
    #[doc = "The list of skus available for the subscription."]
    pub value: Vec<ApiManagementSku>,
    #[doc = "The URI to fetch the next page of Resource Skus. Call ListNext() with this URI to fetch the next page of Resource Skus"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiManagementSkusResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApiManagementSkusResult {
    pub fn new(value: Vec<ApiManagementSku>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged ApiRelease list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiReleaseCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ApiReleaseContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiReleaseCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApiReleaseCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ApiRelease details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiReleaseContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "API Release details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiReleaseContractProperties>,
}
impl ApiReleaseContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API Release details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiReleaseContractProperties {
    #[doc = "Identifier of the API the release belongs to."]
    #[serde(rename = "apiId", default, skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[doc = "The time the API was released. The date conforms to the following format: yyyy-MM-ddTHH:mm:ssZ as specified by the ISO 8601 standard."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "The time the API release was updated."]
    #[serde(rename = "updatedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub updated_date_time: Option<time::OffsetDateTime>,
    #[doc = "Release Notes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
impl ApiReleaseContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged API Revision list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiRevisionCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ApiRevisionContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiRevisionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApiRevisionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summary of revision metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiRevisionContract {
    #[doc = "Identifier of the API Revision."]
    #[serde(rename = "apiId", default, skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[doc = "Revision number of API."]
    #[serde(rename = "apiRevision", default, skip_serializing_if = "Option::is_none")]
    pub api_revision: Option<String>,
    #[doc = "The time the API Revision was created. The date conforms to the following format: yyyy-MM-ddTHH:mm:ssZ as specified by the ISO 8601 standard."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "The time the API Revision were updated. The date conforms to the following format: yyyy-MM-ddTHH:mm:ssZ as specified by the ISO 8601 standard."]
    #[serde(rename = "updatedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub updated_date_time: Option<time::OffsetDateTime>,
    #[doc = "Description of the API Revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gateway URL for accessing the non-current API Revision."]
    #[serde(rename = "privateUrl", default, skip_serializing_if = "Option::is_none")]
    pub private_url: Option<String>,
    #[doc = "Indicates if API revision is the current api revision."]
    #[serde(rename = "isOnline", default, skip_serializing_if = "Option::is_none")]
    pub is_online: Option<bool>,
    #[doc = "Indicates if API revision is accessible via the gateway."]
    #[serde(rename = "isCurrent", default, skip_serializing_if = "Option::is_none")]
    pub is_current: Option<bool>,
}
impl ApiRevisionContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object used to create an API Revision or Version based on an existing API Revision"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiRevisionInfoContract {
    #[doc = "Resource identifier of API to be used to create the revision from."]
    #[serde(rename = "sourceApiId", default, skip_serializing_if = "Option::is_none")]
    pub source_api_id: Option<String>,
    #[doc = "Version identifier for the new API Version."]
    #[serde(rename = "apiVersionName", default, skip_serializing_if = "Option::is_none")]
    pub api_version_name: Option<String>,
    #[doc = "Description of new API Revision."]
    #[serde(rename = "apiRevisionDescription", default, skip_serializing_if = "Option::is_none")]
    pub api_revision_description: Option<String>,
    #[doc = "An API Version Set contains the common configuration for a set of API Versions relating "]
    #[serde(rename = "apiVersionSet", default, skip_serializing_if = "Option::is_none")]
    pub api_version_set: Option<ApiVersionSetContractDetails>,
}
impl ApiRevisionInfoContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API contract properties for the Tag Resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiTagResourceContractProperties {
    #[serde(flatten)]
    pub api_entity_base_contract: ApiEntityBaseContract,
    #[doc = "API identifier in the form /apis/{apiId}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "API name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Absolute URL of the backend service implementing this API."]
    #[serde(rename = "serviceUrl", default, skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
    #[doc = "Relative URL uniquely identifying this API and all of its resource paths within the API Management service instance. It is appended to the API endpoint base URL specified during the service instance creation to form a public URL for this API."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Describes on which protocols the operations in this API can be invoked."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub protocols: Vec<String>,
}
impl ApiTagResourceContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API update contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiUpdateContract {
    #[doc = "API update contract properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiContractUpdateProperties>,
}
impl ApiUpdateContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Control Plane Apis version constraint for the API Management service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiVersionConstraint {
    #[doc = "Limit control plane API calls to API Management service with version equal to or newer than this value."]
    #[serde(rename = "minApiVersion", default, skip_serializing_if = "Option::is_none")]
    pub min_api_version: Option<String>,
}
impl ApiVersionConstraint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged API Version Set list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiVersionSetCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ApiVersionSetContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiVersionSetCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApiVersionSetCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API Version Set Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiVersionSetContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an API Version Set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiVersionSetContractProperties>,
}
impl ApiVersionSetContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An API Version Set contains the common configuration for a set of API Versions relating "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiVersionSetContractDetails {
    #[doc = "Identifier for existing API Version Set. Omit this value to create a new Version Set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The display Name of the API Version Set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Description of API Version Set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An value that determines where the API Version identifier will be located in a HTTP request."]
    #[serde(rename = "versioningScheme", default, skip_serializing_if = "Option::is_none")]
    pub versioning_scheme: Option<api_version_set_contract_details::VersioningScheme>,
    #[doc = "Name of query parameter that indicates the API Version if versioningScheme is set to `query`."]
    #[serde(rename = "versionQueryName", default, skip_serializing_if = "Option::is_none")]
    pub version_query_name: Option<String>,
    #[doc = "Name of HTTP header parameter that indicates the API Version if versioningScheme is set to `header`."]
    #[serde(rename = "versionHeaderName", default, skip_serializing_if = "Option::is_none")]
    pub version_header_name: Option<String>,
}
impl ApiVersionSetContractDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_version_set_contract_details {
    use super::*;
    #[doc = "An value that determines where the API Version identifier will be located in a HTTP request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VersioningScheme")]
    pub enum VersioningScheme {
        Segment,
        Query,
        Header,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VersioningScheme {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VersioningScheme {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VersioningScheme {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Segment => serializer.serialize_unit_variant("VersioningScheme", 0u32, "Segment"),
                Self::Query => serializer.serialize_unit_variant("VersioningScheme", 1u32, "Query"),
                Self::Header => serializer.serialize_unit_variant("VersioningScheme", 2u32, "Header"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of an API Version Set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiVersionSetContractProperties {
    #[serde(flatten)]
    pub api_version_set_entity_base: ApiVersionSetEntityBase,
    #[doc = "Name of API Version Set"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "An value that determines where the API Version identifier will be located in a HTTP request."]
    #[serde(rename = "versioningScheme")]
    pub versioning_scheme: api_version_set_contract_properties::VersioningScheme,
}
impl ApiVersionSetContractProperties {
    pub fn new(display_name: String, versioning_scheme: api_version_set_contract_properties::VersioningScheme) -> Self {
        Self {
            api_version_set_entity_base: ApiVersionSetEntityBase::default(),
            display_name,
            versioning_scheme,
        }
    }
}
pub mod api_version_set_contract_properties {
    use super::*;
    #[doc = "An value that determines where the API Version identifier will be located in a HTTP request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VersioningScheme")]
    pub enum VersioningScheme {
        Segment,
        Query,
        Header,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VersioningScheme {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VersioningScheme {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VersioningScheme {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Segment => serializer.serialize_unit_variant("VersioningScheme", 0u32, "Segment"),
                Self::Query => serializer.serialize_unit_variant("VersioningScheme", 1u32, "Query"),
                Self::Header => serializer.serialize_unit_variant("VersioningScheme", 2u32, "Header"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "API Version set base parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiVersionSetEntityBase {
    #[doc = "Description of API Version Set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name of query parameter that indicates the API Version if versioningScheme is set to `query`."]
    #[serde(rename = "versionQueryName", default, skip_serializing_if = "Option::is_none")]
    pub version_query_name: Option<String>,
    #[doc = "Name of HTTP header parameter that indicates the API Version if versioningScheme is set to `header`."]
    #[serde(rename = "versionHeaderName", default, skip_serializing_if = "Option::is_none")]
    pub version_header_name: Option<String>,
}
impl ApiVersionSetEntityBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters to update or create an API Version Set Contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiVersionSetUpdateParameters {
    #[doc = "Properties used to create or update an API Version Set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiVersionSetUpdateParametersProperties>,
}
impl ApiVersionSetUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties used to create or update an API Version Set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiVersionSetUpdateParametersProperties {
    #[serde(flatten)]
    pub api_version_set_entity_base: ApiVersionSetEntityBase,
    #[doc = "Name of API Version Set"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "An value that determines where the API Version identifier will be located in a HTTP request."]
    #[serde(rename = "versioningScheme", default, skip_serializing_if = "Option::is_none")]
    pub versioning_scheme: Option<api_version_set_update_parameters_properties::VersioningScheme>,
}
impl ApiVersionSetUpdateParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_version_set_update_parameters_properties {
    use super::*;
    #[doc = "An value that determines where the API Version identifier will be located in a HTTP request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VersioningScheme")]
    pub enum VersioningScheme {
        Segment,
        Query,
        Header,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VersioningScheme {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VersioningScheme {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VersioningScheme {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Segment => serializer.serialize_unit_variant("VersioningScheme", 0u32, "Segment"),
                Self::Query => serializer.serialize_unit_variant("VersioningScheme", 1u32, "Query"),
                Self::Header => serializer.serialize_unit_variant("VersioningScheme", 2u32, "Header"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApimResource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type for API Management resource is set to Microsoft.ApiManagement."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ApimResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A wrapper for an ARM resource id"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmIdWrapper {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ArmIdWrapper {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Association entity details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssociationContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Association entity contract properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<association_contract::Properties>,
}
impl AssociationContract {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod association_contract {
    use super::*;
    #[doc = "Association entity contract properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Provisioning state."]
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
        #[doc = "Provisioning state."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            #[serde(rename = "created")]
            Created,
        }
    }
}
#[doc = "API Authentication Settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthenticationSettingsContract {
    #[doc = "API OAuth2 Authentication settings details."]
    #[serde(rename = "oAuth2", default, skip_serializing_if = "Option::is_none")]
    pub o_auth2: Option<OAuth2AuthenticationSettingsContract>,
    #[doc = "API OAuth2 Authentication settings details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub openid: Option<OpenIdAuthenticationSettingsContract>,
    #[doc = "Collection of OAuth2 authentication settings included into this API."]
    #[serde(
        rename = "oAuth2AuthenticationSettings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub o_auth2_authentication_settings: Vec<OAuth2AuthenticationSettingsContract>,
    #[doc = "Collection of Open ID Connect authentication settings included into this API."]
    #[serde(
        rename = "openidAuthenticationSettings",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub openid_authentication_settings: Vec<OpenIdAuthenticationSettingsContract>,
}
impl AuthenticationSettingsContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Authorization Access Policy list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationAccessPolicyCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AuthorizationAccessPolicyContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AuthorizationAccessPolicyCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AuthorizationAccessPolicyCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authorization access policy contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationAccessPolicyContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Authorization Access Policy details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AuthorizationAccessPolicyContractProperties>,
}
impl AuthorizationAccessPolicyContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authorization Access Policy details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationAccessPolicyContractProperties {
    #[doc = "The Tenant Id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The Object Id"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}
impl AuthorizationAccessPolicyContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Authorization list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AuthorizationContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AuthorizationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AuthorizationCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authorization confirm consent code request contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationConfirmConsentCodeRequestContract {
    #[doc = "The consent code from the authorization server after authorizing and consenting."]
    #[serde(rename = "consentCode", default, skip_serializing_if = "Option::is_none")]
    pub consent_code: Option<String>,
}
impl AuthorizationConfirmConsentCodeRequestContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authorization contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Authorization details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AuthorizationContractProperties>,
}
impl AuthorizationContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authorization details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationContractProperties {
    #[doc = "Authorization type options"]
    #[serde(rename = "authorizationType", default, skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<authorization_contract_properties::AuthorizationType>,
    #[doc = "OAuth2 grant type options"]
    #[serde(rename = "oauth2grantType", default, skip_serializing_if = "Option::is_none")]
    pub oauth2grant_type: Option<authorization_contract_properties::Oauth2grantType>,
    #[doc = "Authorization parameters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Authorization error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<AuthorizationError>,
    #[doc = "Status of the Authorization"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl AuthorizationContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod authorization_contract_properties {
    use super::*;
    #[doc = "Authorization type options"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthorizationType")]
    pub enum AuthorizationType {
        OAuth2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthorizationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthorizationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthorizationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::OAuth2 => serializer.serialize_unit_variant("AuthorizationType", 0u32, "OAuth2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "OAuth2 grant type options"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Oauth2grantType")]
    pub enum Oauth2grantType {
        AuthorizationCode,
        ClientCredentials,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Oauth2grantType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Oauth2grantType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Oauth2grantType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AuthorizationCode => serializer.serialize_unit_variant("Oauth2grantType", 0u32, "AuthorizationCode"),
                Self::ClientCredentials => serializer.serialize_unit_variant("Oauth2grantType", 1u32, "ClientCredentials"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Authorization error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationError {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl AuthorizationError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authorization login request contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationLoginRequestContract {
    #[doc = "The redirect URL after login has completed."]
    #[serde(rename = "postLoginRedirectUrl", default, skip_serializing_if = "Option::is_none")]
    pub post_login_redirect_url: Option<String>,
}
impl AuthorizationLoginRequestContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authorization login response contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationLoginResponseContract {
    #[doc = "The login link"]
    #[serde(rename = "loginLink", default, skip_serializing_if = "Option::is_none")]
    pub login_link: Option<String>,
}
impl AuthorizationLoginResponseContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Authorization Provider list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationProviderCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AuthorizationProviderContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AuthorizationProviderCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AuthorizationProviderCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authorization Provider contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationProviderContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Authorization Provider details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AuthorizationProviderContractProperties>,
}
impl AuthorizationProviderContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authorization Provider details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationProviderContractProperties {
    #[doc = "Authorization Provider name. Must be 1 to 300 characters long."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Identity provider name. Must be 1 to 300 characters long."]
    #[serde(rename = "identityProvider", default, skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<String>,
    #[doc = "OAuth2 settings details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth2: Option<AuthorizationProviderOAuth2Settings>,
}
impl AuthorizationProviderContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authorization Provider oauth2 grant types settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationProviderOAuth2GrantTypes {
    #[doc = "OAuth2 authorization code grant parameters"]
    #[serde(rename = "authorizationCode", default, skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<serde_json::Value>,
    #[doc = "OAuth2 client credential grant parameters"]
    #[serde(rename = "clientCredentials", default, skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<serde_json::Value>,
}
impl AuthorizationProviderOAuth2GrantTypes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OAuth2 settings details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationProviderOAuth2Settings {
    #[doc = "Redirect URL to be set in the OAuth application."]
    #[serde(rename = "redirectUrl", default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[doc = "Authorization Provider oauth2 grant types settings"]
    #[serde(rename = "grantTypes", default, skip_serializing_if = "Option::is_none")]
    pub grant_types: Option<AuthorizationProviderOAuth2GrantTypes>,
}
impl AuthorizationProviderOAuth2Settings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged OAuth2 Authorization Servers list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationServerCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AuthorizationServerContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AuthorizationServerCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AuthorizationServerCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "External OAuth authorization server settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationServerContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "External OAuth authorization server settings Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AuthorizationServerContractProperties>,
}
impl AuthorizationServerContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "External OAuth authorization server Update settings contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationServerContractBaseProperties {
    #[doc = "Description of the authorization server. Can contain HTML formatting tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "HTTP verbs supported by the authorization endpoint. GET must be always present. POST is optional."]
    #[serde(
        rename = "authorizationMethods",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub authorization_methods: Vec<String>,
    #[doc = "Method of authentication supported by the token endpoint of this authorization server. Possible values are Basic and/or Body. When Body is specified, client credentials and other parameters are passed within the request body in the application/x-www-form-urlencoded format."]
    #[serde(
        rename = "clientAuthenticationMethod",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub client_authentication_method: Vec<String>,
    #[doc = "Additional parameters required by the token endpoint of this authorization server represented as an array of JSON objects with name and value string properties, i.e. {\"name\" : \"name value\", \"value\": \"a value\"}."]
    #[serde(
        rename = "tokenBodyParameters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub token_body_parameters: Vec<TokenBodyParameterContract>,
    #[doc = "OAuth token endpoint. Contains absolute URI to entity being referenced."]
    #[serde(rename = "tokenEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub token_endpoint: Option<String>,
    #[doc = "If true, authorization server will include state parameter from the authorization request to its response. Client may use state parameter to raise protocol security."]
    #[serde(rename = "supportState", default, skip_serializing_if = "Option::is_none")]
    pub support_state: Option<bool>,
    #[doc = "Access token scope that is going to be requested by default. Can be overridden at the API level. Should be provided in the form of a string containing space-delimited values."]
    #[serde(rename = "defaultScope", default, skip_serializing_if = "Option::is_none")]
    pub default_scope: Option<String>,
    #[doc = "Specifies the mechanism by which access token is passed to the API. "]
    #[serde(
        rename = "bearerTokenSendingMethods",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bearer_token_sending_methods: Vec<String>,
    #[doc = "Can be optionally specified when resource owner password grant type is supported by this authorization server. Default resource owner username."]
    #[serde(rename = "resourceOwnerUsername", default, skip_serializing_if = "Option::is_none")]
    pub resource_owner_username: Option<String>,
    #[doc = "Can be optionally specified when resource owner password grant type is supported by this authorization server. Default resource owner password."]
    #[serde(rename = "resourceOwnerPassword", default, skip_serializing_if = "Option::is_none")]
    pub resource_owner_password: Option<String>,
}
impl AuthorizationServerContractBaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "External OAuth authorization server settings Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationServerContractProperties {
    #[serde(flatten)]
    pub authorization_server_contract_base_properties: AuthorizationServerContractBaseProperties,
    #[doc = "User-friendly authorization server name."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "If true, the authorization server may be used in the developer portal test console. True by default if no value is provided."]
    #[serde(rename = "useInTestConsole", default, skip_serializing_if = "Option::is_none")]
    pub use_in_test_console: Option<bool>,
    #[doc = "If true, the authorization server will be used in the API documentation in the developer portal. False by default if no value is provided."]
    #[serde(rename = "useInApiDocumentation", default, skip_serializing_if = "Option::is_none")]
    pub use_in_api_documentation: Option<bool>,
    #[doc = "Optional reference to a page where client or app registration for this authorization server is performed. Contains absolute URL to entity being referenced."]
    #[serde(rename = "clientRegistrationEndpoint")]
    pub client_registration_endpoint: String,
    #[doc = "OAuth authorization endpoint. See http://tools.ietf.org/html/rfc6749#section-3.2."]
    #[serde(rename = "authorizationEndpoint")]
    pub authorization_endpoint: String,
    #[doc = "Form of an authorization grant, which the client uses to request the access token."]
    #[serde(rename = "grantTypes")]
    pub grant_types: Vec<String>,
    #[doc = "Client or app id registered with this authorization server."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Client or app secret registered with this authorization server. This property will not be filled on 'GET' operations! Use '/listSecrets' POST request to get the value."]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}
impl AuthorizationServerContractProperties {
    pub fn new(
        display_name: String,
        client_registration_endpoint: String,
        authorization_endpoint: String,
        grant_types: Vec<String>,
        client_id: String,
    ) -> Self {
        Self {
            authorization_server_contract_base_properties: AuthorizationServerContractBaseProperties::default(),
            display_name,
            use_in_test_console: None,
            use_in_api_documentation: None,
            client_registration_endpoint,
            authorization_endpoint,
            grant_types,
            client_id,
            client_secret: None,
        }
    }
}
#[doc = "OAuth Server Secrets Contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationServerSecretsContract {
    #[doc = "oAuth Authorization Server Secrets."]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "Can be optionally specified when resource owner password grant type is supported by this authorization server. Default resource owner username."]
    #[serde(rename = "resourceOwnerUsername", default, skip_serializing_if = "Option::is_none")]
    pub resource_owner_username: Option<String>,
    #[doc = "Can be optionally specified when resource owner password grant type is supported by this authorization server. Default resource owner password."]
    #[serde(rename = "resourceOwnerPassword", default, skip_serializing_if = "Option::is_none")]
    pub resource_owner_password: Option<String>,
}
impl AuthorizationServerSecretsContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "External OAuth authorization server settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationServerUpdateContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "External OAuth authorization server Update settings contract."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AuthorizationServerUpdateContractProperties>,
}
impl AuthorizationServerUpdateContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "External OAuth authorization server Update settings contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationServerUpdateContractProperties {
    #[serde(flatten)]
    pub authorization_server_contract_base_properties: AuthorizationServerContractBaseProperties,
    #[doc = "User-friendly authorization server name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "If true, the authorization server may be used in the developer portal test console. True by default if no value is provided."]
    #[serde(rename = "useInTestConsole", default, skip_serializing_if = "Option::is_none")]
    pub use_in_test_console: Option<bool>,
    #[doc = "If true, the authorization server will be used in the API documentation in the developer portal. False by default if no value is provided."]
    #[serde(rename = "useInApiDocumentation", default, skip_serializing_if = "Option::is_none")]
    pub use_in_api_documentation: Option<bool>,
    #[doc = "Optional reference to a page where client or app registration for this authorization server is performed. Contains absolute URL to entity being referenced."]
    #[serde(rename = "clientRegistrationEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub client_registration_endpoint: Option<String>,
    #[doc = "OAuth authorization endpoint. See http://tools.ietf.org/html/rfc6749#section-3.2."]
    #[serde(rename = "authorizationEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint: Option<String>,
    #[doc = "Form of an authorization grant, which the client uses to request the access token."]
    #[serde(
        rename = "grantTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub grant_types: Vec<String>,
    #[doc = "Client or app id registered with this authorization server."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Client or app secret registered with this authorization server. This property will not be filled on 'GET' operations! Use '/listSecrets' POST request to get the value."]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}
impl AuthorizationServerUpdateContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Authorization header information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackendAuthorizationHeaderCredentials {
    #[doc = "Authentication Scheme name."]
    pub scheme: String,
    #[doc = "Authentication Parameter value."]
    pub parameter: String,
}
impl BackendAuthorizationHeaderCredentials {
    pub fn new(scheme: String, parameter: String) -> Self {
        Self { scheme, parameter }
    }
}
#[doc = "Backend entity base Parameter set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendBaseParameters {
    #[doc = "Backend Title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Backend Description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Management Uri of the Resource in External System. This URL can be the Arm Resource Id of Logic Apps, Function Apps or API Apps."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Properties specific to the Backend Type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackendProperties>,
    #[doc = "Details of the Credentials used to connect to Backend."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<BackendCredentialsContract>,
    #[doc = "Details of the Backend WebProxy Server to use in the Request to Backend."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<BackendProxyContract>,
    #[doc = "Properties controlling TLS Certificate Validation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<BackendTlsProperties>,
    #[doc = "The configuration of the backend circuit breaker"]
    #[serde(rename = "circuitBreaker", default, skip_serializing_if = "Option::is_none")]
    pub circuit_breaker: Option<BackendCircuitBreaker>,
}
impl BackendBaseParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration of the backend circuit breaker"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendCircuitBreaker {
    #[doc = "The rules for tripping the backend."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rules: Vec<CircuitBreakerRule>,
}
impl BackendCircuitBreaker {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Backend list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendCollection {
    #[doc = "Backend values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BackendContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BackendCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BackendCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Backend details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Parameters supplied to the Create Backend operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackendContractProperties>,
}
impl BackendContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Create Backend operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackendContractProperties {
    #[serde(flatten)]
    pub backend_base_parameters: BackendBaseParameters,
    #[doc = "Runtime Url of the Backend."]
    pub url: String,
    #[doc = "Backend communication protocol."]
    pub protocol: backend_contract_properties::Protocol,
}
impl BackendContractProperties {
    pub fn new(url: String, protocol: backend_contract_properties::Protocol) -> Self {
        Self {
            backend_base_parameters: BackendBaseParameters::default(),
            url,
            protocol,
        }
    }
}
pub mod backend_contract_properties {
    use super::*;
    #[doc = "Backend communication protocol."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "soap")]
        Soap,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Protocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Protocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Protocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Http => serializer.serialize_unit_variant("Protocol", 0u32, "http"),
                Self::Soap => serializer.serialize_unit_variant("Protocol", 1u32, "soap"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Details of the Credentials used to connect to Backend."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendCredentialsContract {
    #[doc = "List of Client Certificate Ids."]
    #[serde(
        rename = "certificateIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub certificate_ids: Vec<String>,
    #[doc = "List of Client Certificate Thumbprints. Will be ignored if certificatesIds are provided."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub certificate: Vec<String>,
    #[doc = "Query Parameter description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<serde_json::Value>,
    #[doc = "Header Parameter description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<serde_json::Value>,
    #[doc = "Authorization header information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<BackendAuthorizationHeaderCredentials>,
}
impl BackendCredentialsContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties specific to the Backend Type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendProperties {
    #[doc = "Properties of the Service Fabric Type Backend."]
    #[serde(rename = "serviceFabricCluster", default, skip_serializing_if = "Option::is_none")]
    pub service_fabric_cluster: Option<BackendServiceFabricClusterProperties>,
}
impl BackendProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the Backend WebProxy Server to use in the Request to Backend."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackendProxyContract {
    #[doc = "WebProxy Server AbsoluteUri property which includes the entire URI stored in the Uri instance, including all fragments and query strings."]
    pub url: String,
    #[doc = "Username to connect to the WebProxy server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Password to connect to the WebProxy Server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl BackendProxyContract {
    pub fn new(url: String) -> Self {
        Self {
            url,
            username: None,
            password: None,
        }
    }
}
#[doc = "Reconnect request parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendReconnectContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties to control reconnect requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackendReconnectProperties>,
}
impl BackendReconnectContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties to control reconnect requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendReconnectProperties {
    #[doc = "Duration in ISO8601 format after which reconnect will be initiated. Minimum duration of the Reconnect is PT2M."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}
impl BackendReconnectProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Service Fabric Type Backend."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackendServiceFabricClusterProperties {
    #[doc = "The client certificate id for the management endpoint."]
    #[serde(rename = "clientCertificateId", default, skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    #[doc = "The client certificate thumbprint for the management endpoint. Will be ignored if certificatesIds are provided"]
    #[serde(rename = "clientCertificatethumbprint", default, skip_serializing_if = "Option::is_none")]
    pub client_certificatethumbprint: Option<String>,
    #[doc = "Maximum number of retries while attempting resolve the partition."]
    #[serde(rename = "maxPartitionResolutionRetries", default, skip_serializing_if = "Option::is_none")]
    pub max_partition_resolution_retries: Option<i32>,
    #[doc = "The cluster management endpoint."]
    #[serde(rename = "managementEndpoints")]
    pub management_endpoints: Vec<String>,
    #[doc = "Thumbprints of certificates cluster management service uses for tls communication"]
    #[serde(
        rename = "serverCertificateThumbprints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub server_certificate_thumbprints: Vec<String>,
    #[doc = "Server X509 Certificate Names Collection"]
    #[serde(
        rename = "serverX509Names",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub server_x509_names: Vec<X509CertificateName>,
}
impl BackendServiceFabricClusterProperties {
    pub fn new(management_endpoints: Vec<String>) -> Self {
        Self {
            client_certificate_id: None,
            client_certificatethumbprint: None,
            max_partition_resolution_retries: None,
            management_endpoints,
            server_certificate_thumbprints: Vec::new(),
            server_x509_names: Vec::new(),
        }
    }
}
#[doc = "Properties controlling TLS Certificate Validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendTlsProperties {
    #[doc = "Flag indicating whether SSL certificate chain validation should be done when using self-signed certificates for this backend host."]
    #[serde(rename = "validateCertificateChain", default, skip_serializing_if = "Option::is_none")]
    pub validate_certificate_chain: Option<bool>,
    #[doc = "Flag indicating whether SSL certificate name validation should be done when using self-signed certificates for this backend host."]
    #[serde(rename = "validateCertificateName", default, skip_serializing_if = "Option::is_none")]
    pub validate_certificate_name: Option<bool>,
}
impl BackendTlsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Update Backend operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendUpdateParameterProperties {
    #[serde(flatten)]
    pub backend_base_parameters: BackendBaseParameters,
    #[doc = "Runtime Url of the Backend."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Backend communication protocol."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<backend_update_parameter_properties::Protocol>,
}
impl BackendUpdateParameterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backend_update_parameter_properties {
    use super::*;
    #[doc = "Backend communication protocol."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "soap")]
        Soap,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Protocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Protocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Protocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Http => serializer.serialize_unit_variant("Protocol", 0u32, "http"),
                Self::Soap => serializer.serialize_unit_variant("Protocol", 1u32, "soap"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Backend update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendUpdateParameters {
    #[doc = "Parameters supplied to the Update Backend operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BackendUpdateParameterProperties>,
}
impl BackendUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Form of an authorization grant, which the client uses to request the access token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BearerTokenSendingMethodsContract")]
pub enum BearerTokenSendingMethodsContract {
    #[serde(rename = "authorizationHeader")]
    AuthorizationHeader,
    #[serde(rename = "query")]
    Query,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BearerTokenSendingMethodsContract {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BearerTokenSendingMethodsContract {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BearerTokenSendingMethodsContract {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AuthorizationHeader => {
                serializer.serialize_unit_variant("BearerTokenSendingMethodsContract", 0u32, "authorizationHeader")
            }
            Self::Query => serializer.serialize_unit_variant("BearerTokenSendingMethodsContract", 1u32, "query"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Body logging settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BodyDiagnosticSettings {
    #[doc = "Number of request body bytes to log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<i32>,
}
impl BodyDiagnosticSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Caches list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CacheContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CacheCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CacheCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cache details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the Cache contract."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CacheContractProperties>,
}
impl CacheContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Cache contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheContractProperties {
    #[doc = "Cache description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Runtime connection string to cache"]
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    #[doc = "Location identifier to use cache from (should be either 'default' or valid Azure region identifier)"]
    #[serde(rename = "useFromLocation")]
    pub use_from_location: String,
    #[doc = "Original uri of entity in external system cache points to"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl CacheContractProperties {
    pub fn new(connection_string: String, use_from_location: String) -> Self {
        Self {
            description: None,
            connection_string,
            use_from_location,
            resource_id: None,
        }
    }
}
#[doc = "Cache update details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheUpdateParameters {
    #[doc = "Parameters supplied to the Update Cache operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CacheUpdateProperties>,
}
impl CacheUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Update Cache operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CacheUpdateProperties {
    #[doc = "Cache description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Runtime connection string to cache"]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    #[doc = "Location identifier to use cache from (should be either 'default' or valid Azure region identifier)"]
    #[serde(rename = "useFromLocation", default, skip_serializing_if = "Option::is_none")]
    pub use_from_location: Option<String>,
    #[doc = "Original uri of entity in external system cache points to"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl CacheUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Certificates list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CertificateContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CertificateCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CertificateCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Certificate configuration which consist of non-trusted intermediates and root certificates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateConfiguration {
    #[doc = "Base64 Encoded certificate."]
    #[serde(rename = "encodedCertificate", default, skip_serializing_if = "Option::is_none")]
    pub encoded_certificate: Option<String>,
    #[doc = "Certificate Password."]
    #[serde(rename = "certificatePassword", default, skip_serializing_if = "Option::is_none")]
    pub certificate_password: Option<String>,
    #[doc = "The System.Security.Cryptography.x509certificates.StoreName certificate store location. Only Root and CertificateAuthority are valid locations."]
    #[serde(rename = "storeName")]
    pub store_name: certificate_configuration::StoreName,
    #[doc = "SSL certificate information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<CertificateInformation>,
}
impl CertificateConfiguration {
    pub fn new(store_name: certificate_configuration::StoreName) -> Self {
        Self {
            encoded_certificate: None,
            certificate_password: None,
            store_name,
            certificate: None,
        }
    }
}
pub mod certificate_configuration {
    use super::*;
    #[doc = "The System.Security.Cryptography.x509certificates.StoreName certificate store location. Only Root and CertificateAuthority are valid locations."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StoreName {
        CertificateAuthority,
        Root,
    }
}
#[doc = "Certificate details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the Certificate contract."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificateContractProperties>,
}
impl CertificateContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Certificate contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateContractProperties {
    #[doc = "Subject attribute of the certificate."]
    pub subject: String,
    #[doc = "Thumbprint of the certificate."]
    pub thumbprint: String,
    #[doc = "Expiration date of the certificate. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "expirationDate", with = "azure_core::date::rfc3339")]
    pub expiration_date: time::OffsetDateTime,
    #[doc = "KeyVault contract details."]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KeyVaultContractProperties>,
}
impl CertificateContractProperties {
    pub fn new(subject: String, thumbprint: String, expiration_date: time::OffsetDateTime) -> Self {
        Self {
            subject,
            thumbprint,
            expiration_date,
            key_vault: None,
        }
    }
}
#[doc = "Certificate create or update details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateCreateOrUpdateParameters {
    #[doc = "Parameters supplied to the CreateOrUpdate certificate operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificateCreateOrUpdateProperties>,
}
impl CertificateCreateOrUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the CreateOrUpdate certificate operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateCreateOrUpdateProperties {
    #[doc = "Base 64 encoded certificate using the application/x-pkcs12 representation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[doc = "Password for the Certificate"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Create keyVault contract details."]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KeyVaultContractCreateProperties>,
}
impl CertificateCreateOrUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SSL certificate information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateInformation {
    #[doc = "Expiration date of the certificate. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub expiry: time::OffsetDateTime,
    #[doc = "Thumbprint of the certificate."]
    pub thumbprint: String,
    #[doc = "Subject of the certificate."]
    pub subject: String,
}
impl CertificateInformation {
    pub fn new(expiry: time::OffsetDateTime, thumbprint: String, subject: String) -> Self {
        Self {
            expiry,
            thumbprint,
            subject,
        }
    }
}
#[doc = "The trip conditions of the circuit breaker"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CircuitBreakerFailureCondition {
    #[doc = "The threshold for opening the circuit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "The threshold for opening the circuit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<i64>,
    #[doc = "The interval during which the failures are counted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[doc = "The status code ranges which are considered as failure."]
    #[serde(
        rename = "statusCodeRanges",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub status_code_ranges: Vec<FailureStatusCodeRange>,
    #[doc = "The error reasons which are considered as failure."]
    #[serde(
        rename = "errorReasons",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub error_reasons: Vec<String>,
}
impl CircuitBreakerFailureCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rule configuration to trip the backend."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CircuitBreakerRule {
    #[doc = "The rule name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The trip conditions of the circuit breaker"]
    #[serde(rename = "failureCondition", default, skip_serializing_if = "Option::is_none")]
    pub failure_condition: Option<CircuitBreakerFailureCondition>,
    #[doc = "The duration for which the circuit will be tripped."]
    #[serde(rename = "tripDuration", default, skip_serializing_if = "Option::is_none")]
    pub trip_duration: Option<String>,
}
impl CircuitBreakerRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Client or app secret used in IdentityProviders, Aad, OpenID or OAuth."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientSecretContract {
    #[doc = "Client or app secret used in IdentityProviders, Aad, OpenID or OAuth."]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}
impl ClientSecretContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A request to perform the connectivity check operation on a API Management service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectivityCheckRequest {
    #[doc = "Definitions about the connectivity check origin."]
    pub source: connectivity_check_request::Source,
    #[doc = "The connectivity check operation destination."]
    pub destination: connectivity_check_request::Destination,
    #[doc = "The IP version to be used. Only IPv4 is supported for now."]
    #[serde(rename = "preferredIPVersion", default, skip_serializing_if = "Option::is_none")]
    pub preferred_ip_version: Option<connectivity_check_request::PreferredIpVersion>,
    #[doc = "The request's protocol. Specific protocol configuration can be available based on this selection. The specified destination address must be coherent with this value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<connectivity_check_request::Protocol>,
    #[doc = "Protocol-specific configuration."]
    #[serde(rename = "protocolConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub protocol_configuration: Option<connectivity_check_request::ProtocolConfiguration>,
}
impl ConnectivityCheckRequest {
    pub fn new(source: connectivity_check_request::Source, destination: connectivity_check_request::Destination) -> Self {
        Self {
            source,
            destination,
            preferred_ip_version: None,
            protocol: None,
            protocol_configuration: None,
        }
    }
}
pub mod connectivity_check_request {
    use super::*;
    #[doc = "Definitions about the connectivity check origin."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Source {
        #[doc = "The API Management service region from where to start the connectivity check operation."]
        pub region: String,
        #[doc = "The particular VMSS instance from which to fire the request."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub instance: Option<i64>,
    }
    impl Source {
        pub fn new(region: String) -> Self {
            Self { region, instance: None }
        }
    }
    #[doc = "The connectivity check operation destination."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Destination {
        #[doc = "Destination address. Can either be an IP address or a FQDN."]
        pub address: String,
        #[doc = "Destination port."]
        pub port: i64,
    }
    impl Destination {
        pub fn new(address: String, port: i64) -> Self {
            Self { address, port }
        }
    }
    #[doc = "The IP version to be used. Only IPv4 is supported for now."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PreferredIpVersion")]
    pub enum PreferredIpVersion {
        IPv4,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PreferredIpVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PreferredIpVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PreferredIpVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IPv4 => serializer.serialize_unit_variant("PreferredIpVersion", 0u32, "IPv4"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The request's protocol. Specific protocol configuration can be available based on this selection. The specified destination address must be coherent with this value."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Protocol")]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "HTTP")]
        Http,
        #[serde(rename = "HTTPS")]
        Https,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Protocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Protocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Protocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tcp => serializer.serialize_unit_variant("Protocol", 0u32, "TCP"),
                Self::Http => serializer.serialize_unit_variant("Protocol", 1u32, "HTTP"),
                Self::Https => serializer.serialize_unit_variant("Protocol", 2u32, "HTTPS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Protocol-specific configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ProtocolConfiguration {
        #[doc = "Configuration for HTTP or HTTPS requests."]
        #[serde(rename = "HTTPConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub http_configuration: Option<protocol_configuration::HttpConfiguration>,
    }
    impl ProtocolConfiguration {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod protocol_configuration {
        use super::*;
        #[doc = "Configuration for HTTP or HTTPS requests."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct HttpConfiguration {
            #[doc = "The HTTP method to be used."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub method: Option<http_configuration::Method>,
            #[doc = "List of HTTP status codes considered valid for the request response."]
            #[serde(
                rename = "validStatusCodes",
                default,
                deserialize_with = "azure_core::util::deserialize_null_as_default",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub valid_status_codes: Vec<i64>,
            #[doc = "List of headers to be included in the request."]
            #[serde(
                default,
                deserialize_with = "azure_core::util::deserialize_null_as_default",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub headers: Vec<HttpHeader>,
        }
        impl HttpConfiguration {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod http_configuration {
            use super::*;
            #[doc = "The HTTP method to be used."]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(remote = "Method")]
            pub enum Method {
                #[serde(rename = "GET")]
                Get,
                #[serde(rename = "POST")]
                Post,
                #[serde(skip_deserializing)]
                UnknownValue(String),
            }
            impl FromStr for Method {
                type Err = value::Error;
                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Self::deserialize(s.into_deserializer())
                }
            }
            impl<'de> Deserialize<'de> for Method {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let s = String::deserialize(deserializer)?;
                    let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                    Ok(deserialized)
                }
            }
            impl Serialize for Method {
                fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    match self {
                        Self::Get => serializer.serialize_unit_variant("Method", 0u32, "GET"),
                        Self::Post => serializer.serialize_unit_variant("Method", 1u32, "POST"),
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
    }
}
#[doc = "Information on the connectivity status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectivityCheckResponse {
    #[doc = "List of hops between the source and the destination."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hops: Vec<ConnectivityHop>,
    #[doc = "The connection status."]
    #[serde(rename = "connectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<connectivity_check_response::ConnectionStatus>,
    #[doc = "Average latency in milliseconds."]
    #[serde(rename = "avgLatencyInMs", default, skip_serializing_if = "Option::is_none")]
    pub avg_latency_in_ms: Option<i64>,
    #[doc = "Minimum latency in milliseconds."]
    #[serde(rename = "minLatencyInMs", default, skip_serializing_if = "Option::is_none")]
    pub min_latency_in_ms: Option<i64>,
    #[doc = "Maximum latency in milliseconds."]
    #[serde(rename = "maxLatencyInMs", default, skip_serializing_if = "Option::is_none")]
    pub max_latency_in_ms: Option<i64>,
    #[doc = "Total number of probes sent."]
    #[serde(rename = "probesSent", default, skip_serializing_if = "Option::is_none")]
    pub probes_sent: Option<i64>,
    #[doc = "Number of failed probes."]
    #[serde(rename = "probesFailed", default, skip_serializing_if = "Option::is_none")]
    pub probes_failed: Option<i64>,
}
impl ConnectivityCheckResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connectivity_check_response {
    use super::*;
    #[doc = "The connection status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectionStatus")]
    pub enum ConnectionStatus {
        Unknown,
        Connected,
        Disconnected,
        Degraded,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("ConnectionStatus", 0u32, "Unknown"),
                Self::Connected => serializer.serialize_unit_variant("ConnectionStatus", 1u32, "Connected"),
                Self::Disconnected => serializer.serialize_unit_variant("ConnectionStatus", 2u32, "Disconnected"),
                Self::Degraded => serializer.serialize_unit_variant("ConnectionStatus", 3u32, "Degraded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information about a hop between the source and the destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectivityHop {
    #[doc = "The type of the hop."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The ID of the hop."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The IP address of the hop."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "The ID of the resource corresponding to this hop."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "List of next hop identifiers."]
    #[serde(
        rename = "nextHopIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub next_hop_ids: Vec<String>,
    #[doc = "List of issues."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub issues: Vec<ConnectivityIssue>,
}
impl ConnectivityHop {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about an issue encountered in the process of checking for connectivity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectivityIssue {
    #[doc = "The origin of the issue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<connectivity_issue::Origin>,
    #[doc = "The severity of the issue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<connectivity_issue::Severity>,
    #[doc = "The type of issue."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<connectivity_issue::Type>,
    #[doc = "Provides additional context on the issue."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub context: Vec<IssueContext>,
}
impl ConnectivityIssue {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connectivity_issue {
    use super::*;
    #[doc = "The origin of the issue."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        Local,
        Inbound,
        Outbound,
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
                Self::Local => serializer.serialize_unit_variant("Origin", 0u32, "Local"),
                Self::Inbound => serializer.serialize_unit_variant("Origin", 1u32, "Inbound"),
                Self::Outbound => serializer.serialize_unit_variant("Origin", 2u32, "Outbound"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The severity of the issue."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        Error,
        Warning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Severity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Severity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Severity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("Severity", 0u32, "Error"),
                Self::Warning => serializer.serialize_unit_variant("Severity", 1u32, "Warning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of issue."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Unknown,
        AgentStopped,
        GuestFirewall,
        DnsResolution,
        SocketBind,
        NetworkSecurityRule,
        UserDefinedRoute,
        PortThrottled,
        Platform,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("Type", 0u32, "Unknown"),
                Self::AgentStopped => serializer.serialize_unit_variant("Type", 1u32, "AgentStopped"),
                Self::GuestFirewall => serializer.serialize_unit_variant("Type", 2u32, "GuestFirewall"),
                Self::DnsResolution => serializer.serialize_unit_variant("Type", 3u32, "DnsResolution"),
                Self::SocketBind => serializer.serialize_unit_variant("Type", 4u32, "SocketBind"),
                Self::NetworkSecurityRule => serializer.serialize_unit_variant("Type", 5u32, "NetworkSecurityRule"),
                Self::UserDefinedRoute => serializer.serialize_unit_variant("Type", 6u32, "UserDefinedRoute"),
                Self::PortThrottled => serializer.serialize_unit_variant("Type", 7u32, "PortThrottled"),
                Self::Platform => serializer.serialize_unit_variant("Type", 8u32, "Platform"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Details about connectivity to a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectivityStatusContract {
    #[doc = "The hostname of the resource which the service depends on. This can be the database, storage or any other azure resource on which the service depends upon."]
    pub name: String,
    #[doc = "Resource Connectivity Status Type identifier."]
    pub status: connectivity_status_contract::Status,
    #[doc = "Error details of the connectivity to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The date when the resource connectivity status was last updated. This status should be updated every 15 minutes. If this status has not been updated, then it means that the service has lost network connectivity to the resource, from inside the Virtual Network.The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "lastUpdated", with = "azure_core::date::rfc3339")]
    pub last_updated: time::OffsetDateTime,
    #[doc = "The date when the resource connectivity status last Changed from success to failure or vice-versa. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "lastStatusChange", with = "azure_core::date::rfc3339")]
    pub last_status_change: time::OffsetDateTime,
    #[doc = "Resource Type."]
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[doc = "Whether this is optional."]
    #[serde(rename = "isOptional")]
    pub is_optional: bool,
}
impl ConnectivityStatusContract {
    pub fn new(
        name: String,
        status: connectivity_status_contract::Status,
        last_updated: time::OffsetDateTime,
        last_status_change: time::OffsetDateTime,
        resource_type: String,
        is_optional: bool,
    ) -> Self {
        Self {
            name,
            status,
            error: None,
            last_updated,
            last_status_change,
            resource_type,
            is_optional,
        }
    }
}
pub mod connectivity_status_contract {
    use super::*;
    #[doc = "Resource Connectivity Status Type identifier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "initializing")]
        Initializing,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "failure")]
        Failure,
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
                Self::Initializing => serializer.serialize_unit_variant("Status", 0u32, "initializing"),
                Self::Success => serializer.serialize_unit_variant("Status", 1u32, "success"),
                Self::Failure => serializer.serialize_unit_variant("Status", 2u32, "failure"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Paged list of content items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContentItemCollection {
    #[doc = "Collection of content items."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ContentItemContract>,
    #[doc = "Next page link, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ContentItemCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ContentItemCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Content type contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContentItemContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ContentItemContractProperties>,
}
impl ContentItemContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContentItemContractProperties {}
impl ContentItemContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged list of content types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContentTypeCollection {
    #[doc = "Collection of content types."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ContentTypeContract>,
    #[doc = "Next page link, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ContentTypeCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ContentTypeCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Content type contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContentTypeContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ContentTypeContractProperties>,
}
impl ContentTypeContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContentTypeContractProperties {
    #[doc = "Content type identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Content type name. Must be 1 to 250 characters long."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Content type description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Content type schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<serde_json::Value>,
    #[doc = "Content type version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ContentTypeContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataMasking {
    #[doc = "Masking settings for Url query parameters"]
    #[serde(
        rename = "queryParams",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub query_params: Vec<DataMaskingEntity>,
    #[doc = "Masking settings for headers"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub headers: Vec<DataMaskingEntity>,
}
impl DataMasking {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataMaskingEntity {
    #[doc = "The name of an entity to mask (e.g. a name of a header or a query parameter)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Data masking mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<data_masking_entity::Mode>,
}
impl DataMaskingEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_masking_entity {
    use super::*;
    #[doc = "Data masking mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        Mask,
        Hide,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Mask => serializer.serialize_unit_variant("Mode", 0u32, "Mask"),
                Self::Hide => serializer.serialize_unit_variant("Mode", 1u32, "Hide"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Deleted API Management Service information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedServiceContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeletedServiceContractProperties>,
    #[doc = "API Management Service Master Location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl DeletedServiceContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedServiceContractProperties {
    #[doc = "Fully-qualified API Management Service Resource ID"]
    #[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[doc = "UTC Date and Time when the service will be automatically purged. The date conforms to the following format: yyyy-MM-ddTHH:mm:ssZ as specified by the ISO 8601 standard."]
    #[serde(rename = "scheduledPurgeDate", default, with = "azure_core::date::rfc3339::option")]
    pub scheduled_purge_date: Option<time::OffsetDateTime>,
    #[doc = "UTC Timestamp when the service was soft-deleted. The date conforms to the following format: yyyy-MM-ddTHH:mm:ssZ as specified by the ISO 8601 standard."]
    #[serde(rename = "deletionDate", default, with = "azure_core::date::rfc3339::option")]
    pub deletion_date: Option<time::OffsetDateTime>,
}
impl DeletedServiceContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged deleted API Management Services List Representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeletedServicesCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DeletedServiceContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeletedServicesCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DeletedServicesCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Deploy Configuration operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeployConfigurationParameterProperties {
    #[doc = "The name of the Git branch from which the configuration is to be deployed to the configuration database."]
    pub branch: String,
    #[doc = "The value enforcing deleting subscriptions to products that are deleted in this update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}
impl DeployConfigurationParameterProperties {
    pub fn new(branch: String) -> Self {
        Self { branch, force: None }
    }
}
#[doc = "Deploy Tenant Configuration Contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeployConfigurationParameters {
    #[doc = "Parameters supplied to the Deploy Configuration operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeployConfigurationParameterProperties>,
}
impl DeployConfigurationParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Diagnostic list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DiagnosticContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiagnosticCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DiagnosticCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Diagnostic details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Diagnostic Entity Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticContractProperties>,
}
impl DiagnosticContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Diagnostic Entity Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticContractProperties {
    #[doc = "Specifies for what type of messages sampling settings should not apply."]
    #[serde(rename = "alwaysLog", default, skip_serializing_if = "Option::is_none")]
    pub always_log: Option<diagnostic_contract_properties::AlwaysLog>,
    #[doc = "Resource Id of a target logger."]
    #[serde(rename = "loggerId")]
    pub logger_id: String,
    #[doc = "Sampling settings for Diagnostic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sampling: Option<SamplingSettings>,
    #[doc = "Diagnostic settings for incoming/outgoing HTTP messages to the Gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frontend: Option<PipelineDiagnosticSettings>,
    #[doc = "Diagnostic settings for incoming/outgoing HTTP messages to the Gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend: Option<PipelineDiagnosticSettings>,
    #[doc = "Log the ClientIP. Default is false."]
    #[serde(rename = "logClientIp", default, skip_serializing_if = "Option::is_none")]
    pub log_client_ip: Option<bool>,
    #[doc = "Sets correlation protocol to use for Application Insights diagnostics."]
    #[serde(rename = "httpCorrelationProtocol", default, skip_serializing_if = "Option::is_none")]
    pub http_correlation_protocol: Option<diagnostic_contract_properties::HttpCorrelationProtocol>,
    #[doc = "The verbosity level applied to traces emitted by trace policies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<diagnostic_contract_properties::Verbosity>,
    #[doc = "The format of the Operation Name for Application Insights telemetries. Default is Name."]
    #[serde(rename = "operationNameFormat", default, skip_serializing_if = "Option::is_none")]
    pub operation_name_format: Option<diagnostic_contract_properties::OperationNameFormat>,
    #[doc = "Emit custom metrics via emit-metric policy. Applicable only to Application Insights diagnostic settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<bool>,
}
impl DiagnosticContractProperties {
    pub fn new(logger_id: String) -> Self {
        Self {
            always_log: None,
            logger_id,
            sampling: None,
            frontend: None,
            backend: None,
            log_client_ip: None,
            http_correlation_protocol: None,
            verbosity: None,
            operation_name_format: None,
            metrics: None,
        }
    }
}
pub mod diagnostic_contract_properties {
    use super::*;
    #[doc = "Specifies for what type of messages sampling settings should not apply."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AlwaysLog")]
    pub enum AlwaysLog {
        #[serde(rename = "allErrors")]
        AllErrors,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AlwaysLog {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AlwaysLog {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AlwaysLog {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AllErrors => serializer.serialize_unit_variant("AlwaysLog", 0u32, "allErrors"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Sets correlation protocol to use for Application Insights diagnostics."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HttpCorrelationProtocol")]
    pub enum HttpCorrelationProtocol {
        None,
        Legacy,
        #[serde(rename = "W3C")]
        W3c,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HttpCorrelationProtocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HttpCorrelationProtocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HttpCorrelationProtocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("HttpCorrelationProtocol", 0u32, "None"),
                Self::Legacy => serializer.serialize_unit_variant("HttpCorrelationProtocol", 1u32, "Legacy"),
                Self::W3c => serializer.serialize_unit_variant("HttpCorrelationProtocol", 2u32, "W3C"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The verbosity level applied to traces emitted by trace policies."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Verbosity")]
    pub enum Verbosity {
        #[serde(rename = "verbose")]
        Verbose,
        #[serde(rename = "information")]
        Information,
        #[serde(rename = "error")]
        Error,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Verbosity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Verbosity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Verbosity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Verbose => serializer.serialize_unit_variant("Verbosity", 0u32, "verbose"),
                Self::Information => serializer.serialize_unit_variant("Verbosity", 1u32, "information"),
                Self::Error => serializer.serialize_unit_variant("Verbosity", 2u32, "error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The format of the Operation Name for Application Insights telemetries. Default is Name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationNameFormat")]
    pub enum OperationNameFormat {
        Name,
        Url,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OperationNameFormat {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OperationNameFormat {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OperationNameFormat {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Name => serializer.serialize_unit_variant("OperationNameFormat", 0u32, "Name"),
                Self::Url => serializer.serialize_unit_variant("OperationNameFormat", 1u32, "Url"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Paged Documentation list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DocumentationCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DocumentationContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DocumentationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DocumentationCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Markdown documentation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DocumentationContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Markdown documentation details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DocumentationContractProperties>,
}
impl DocumentationContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Markdown documentation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DocumentationContractProperties {
    #[doc = "documentation title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Markdown documentation content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
impl DocumentationContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Documentation update contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DocumentationUpdateContract {
    #[doc = "Markdown documentation details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DocumentationContractProperties>,
}
impl DocumentationUpdateContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged email template list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailTemplateCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EmailTemplateContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EmailTemplateCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EmailTemplateCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Email Template details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailTemplateContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Email Template Contract properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EmailTemplateContractProperties>,
}
impl EmailTemplateContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Email Template Contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailTemplateContractProperties {
    #[doc = "Subject of the Template."]
    pub subject: String,
    #[doc = "Email Template Body. This should be a valid XDocument"]
    pub body: String,
    #[doc = "Title of the Template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Description of the Email Template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the template is the default template provided by API Management or has been edited."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "Email Template Parameter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parameters: Vec<EmailTemplateParametersContractProperties>,
}
impl EmailTemplateContractProperties {
    pub fn new(subject: String, body: String) -> Self {
        Self {
            subject,
            body,
            title: None,
            description: None,
            is_default: None,
            parameters: Vec::new(),
        }
    }
}
#[doc = "Email Template Parameter contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailTemplateParametersContractProperties {
    #[doc = "Template parameter name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Template parameter title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Template parameter description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl EmailTemplateParametersContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Email Template Update Contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailTemplateUpdateParameterProperties {
    #[doc = "Subject of the Template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Title of the Template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Description of the Email Template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Email Template Body. This should be a valid XDocument"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[doc = "Email Template Parameter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parameters: Vec<EmailTemplateParametersContractProperties>,
}
impl EmailTemplateUpdateParameterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Email Template update Parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailTemplateUpdateParameters {
    #[doc = "Email Template Update Contract properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EmailTemplateUpdateParameterProperties>,
}
impl EmailTemplateUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A domain name that a service is reached at."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDependency {
    #[doc = "The domain name of the dependency."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "The Ports used when connecting to DomainName."]
    #[serde(
        rename = "endpointDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endpoint_details: Vec<EndpointDetail>,
}
impl EndpointDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Current TCP connectivity information from the Api Management Service to a single endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDetail {
    #[doc = "The port an endpoint is connected to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "The region of the dependency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}
impl EndpointDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error Field contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorFieldContract {
    #[doc = "Property level error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Human-readable representation of property-level error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Property name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl ErrorFieldContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error Response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error Body contract."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
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
#[doc = "Error Body contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseBody {
    #[doc = "Service-defined error code. This code serves as a sub-status for the HTTP error code specified in the response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The list of invalid fields send in request, in case of validation error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorFieldContract>,
}
impl ErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The failure http status code range"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailureStatusCodeRange {
    #[doc = "The minimum http status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    #[doc = "The maximum http status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
}
impl FailureStatusCodeRange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Gateway certificate authority list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayCertificateAuthorityCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GatewayCertificateAuthorityContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GatewayCertificateAuthorityCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GatewayCertificateAuthorityCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gateway certificate authority details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayCertificateAuthorityContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Gateway certificate authority details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GatewayCertificateAuthorityContractProperties>,
}
impl GatewayCertificateAuthorityContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gateway certificate authority details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayCertificateAuthorityContractProperties {
    #[doc = "Determines whether certificate authority is trusted."]
    #[serde(rename = "isTrusted", default, skip_serializing_if = "Option::is_none")]
    pub is_trusted: Option<bool>,
}
impl GatewayCertificateAuthorityContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Gateway list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GatewayContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GatewayCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GatewayCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gateway details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the Gateway contract."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GatewayContractProperties>,
}
impl GatewayContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Gateway contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayContractProperties {
    #[doc = "Resource location data properties."]
    #[serde(rename = "locationData", default, skip_serializing_if = "Option::is_none")]
    pub location_data: Option<ResourceLocationDataContract>,
    #[doc = "Gateway description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl GatewayContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Gateway hostname configuration list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayHostnameConfigurationCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GatewayHostnameConfigurationContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GatewayHostnameConfigurationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GatewayHostnameConfigurationCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gateway hostname configuration details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayHostnameConfigurationContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Gateway hostname configuration details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GatewayHostnameConfigurationContractProperties>,
}
impl GatewayHostnameConfigurationContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gateway hostname configuration details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayHostnameConfigurationContractProperties {
    #[doc = "Hostname value. Supports valid domain name, partial or full wildcard"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "Identifier of Certificate entity that will be used for TLS connection establishment"]
    #[serde(rename = "certificateId", default, skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    #[doc = "Determines whether gateway requests client certificate"]
    #[serde(rename = "negotiateClientCertificate", default, skip_serializing_if = "Option::is_none")]
    pub negotiate_client_certificate: Option<bool>,
    #[doc = "Specifies if TLS 1.0 is supported"]
    #[serde(rename = "tls10Enabled", default, skip_serializing_if = "Option::is_none")]
    pub tls10_enabled: Option<bool>,
    #[doc = "Specifies if TLS 1.1 is supported"]
    #[serde(rename = "tls11Enabled", default, skip_serializing_if = "Option::is_none")]
    pub tls11_enabled: Option<bool>,
    #[doc = "Specifies if HTTP/2.0 is supported"]
    #[serde(rename = "http2Enabled", default, skip_serializing_if = "Option::is_none")]
    pub http2_enabled: Option<bool>,
}
impl GatewayHostnameConfigurationContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gateway key regeneration request contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayKeyRegenerationRequestContract {
    #[doc = "The Key being regenerated."]
    #[serde(rename = "keyType")]
    pub key_type: gateway_key_regeneration_request_contract::KeyType,
}
impl GatewayKeyRegenerationRequestContract {
    pub fn new(key_type: gateway_key_regeneration_request_contract::KeyType) -> Self {
        Self { key_type }
    }
}
pub mod gateway_key_regeneration_request_contract {
    use super::*;
    #[doc = "The Key being regenerated."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        #[serde(rename = "primary")]
        Primary,
        #[serde(rename = "secondary")]
        Secondary,
    }
}
#[doc = "Gateway authentication keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayKeysContract {
    #[doc = "Primary gateway key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
    #[doc = "Secondary gateway key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
}
impl GatewayKeysContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gateway access token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayTokenContract {
    #[doc = "Shared Access Authentication token value for the Gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl GatewayTokenContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gateway token request contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewayTokenRequestContract {
    #[doc = "The Key to be used to generate gateway token."]
    #[serde(rename = "keyType")]
    pub key_type: gateway_token_request_contract::KeyType,
    #[doc = "The Expiry time of the Token. Maximum token expiry time is set to 30 days. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(with = "azure_core::date::rfc3339")]
    pub expiry: time::OffsetDateTime,
}
impl GatewayTokenRequestContract {
    pub fn new(key_type: gateway_token_request_contract::KeyType, expiry: time::OffsetDateTime) -> Self {
        Self { key_type, expiry }
    }
}
pub mod gateway_token_request_contract {
    use super::*;
    #[doc = "The Key to be used to generate gateway token."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        #[serde(rename = "primary")]
        Primary,
        #[serde(rename = "secondary")]
        Secondary,
    }
    impl Default for KeyType {
        fn default() -> Self {
            Self::Primary
        }
    }
}
#[doc = "Generate SSO Url operations response details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateSsoUrlResult {
    #[doc = "Redirect Url containing the SSO URL value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl GenerateSsoUrlResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of the list schema operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalSchemaCollection {
    #[doc = "Global Schema Contract value."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GlobalSchemaContract>,
    #[doc = "Total record count number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GlobalSchemaCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GlobalSchemaCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Global Schema Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalSchemaContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Schema create or update contract Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GlobalSchemaContractProperties>,
}
impl GlobalSchemaContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema create or update contract Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlobalSchemaContractProperties {
    #[doc = "Schema Type. Immutable."]
    #[serde(rename = "schemaType")]
    pub schema_type: global_schema_contract_properties::SchemaType,
    #[doc = "Free-form schema entity description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Json-encoded string for non json-based schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[doc = "Global Schema Document Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<GlobalSchemaDocumentProperties>,
}
impl GlobalSchemaContractProperties {
    pub fn new(schema_type: global_schema_contract_properties::SchemaType) -> Self {
        Self {
            schema_type,
            description: None,
            value: None,
            document: None,
        }
    }
}
pub mod global_schema_contract_properties {
    use super::*;
    #[doc = "Schema Type. Immutable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SchemaType")]
    pub enum SchemaType {
        #[serde(rename = "xml")]
        Xml,
        #[serde(rename = "json")]
        Json,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SchemaType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SchemaType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SchemaType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Xml => serializer.serialize_unit_variant("SchemaType", 0u32, "xml"),
                Self::Json => serializer.serialize_unit_variant("SchemaType", 1u32, "json"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Global Schema Document Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalSchemaDocumentProperties {}
impl GlobalSchemaDocumentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Group list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GroupContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GroupCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GroupCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Group contract Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GroupContractProperties>,
}
impl GroupContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Group contract Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupContractProperties {
    #[doc = "Group name."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Group description. Can contain HTML formatting tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "true if the group is one of the three system groups (Administrators, Developers, or Guests); otherwise false."]
    #[serde(rename = "builtIn", default, skip_serializing_if = "Option::is_none")]
    pub built_in: Option<bool>,
    #[doc = "Group type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<group_contract_properties::Type>,
    #[doc = "For external groups, this property contains the id of the group from the external identity provider, e.g. for Azure Active Directory `aad://<tenant>.onmicrosoft.com/groups/<group object id>`; otherwise the value is null."]
    #[serde(rename = "externalId", default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}
impl GroupContractProperties {
    pub fn new(display_name: String) -> Self {
        Self {
            display_name,
            description: None,
            built_in: None,
            type_: None,
            external_id: None,
        }
    }
}
pub mod group_contract_properties {
    use super::*;
    #[doc = "Group type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "custom")]
        Custom,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "external")]
        External,
    }
}
#[doc = "Parameters supplied to the Create Group operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupCreateParameters {
    #[doc = "Parameters supplied to the Create Group operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GroupCreateParametersProperties>,
}
impl GroupCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Create Group operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupCreateParametersProperties {
    #[doc = "Group name."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Group description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Group type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<group_create_parameters_properties::Type>,
    #[doc = "Identifier of the external groups, this property contains the id of the group from the external identity provider, e.g. for Azure Active Directory `aad://<tenant>.onmicrosoft.com/groups/<group object id>`; otherwise the value is null."]
    #[serde(rename = "externalId", default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}
impl GroupCreateParametersProperties {
    pub fn new(display_name: String) -> Self {
        Self {
            display_name,
            description: None,
            type_: None,
            external_id: None,
        }
    }
}
pub mod group_create_parameters_properties {
    use super::*;
    #[doc = "Group type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "custom")]
        Custom,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "external")]
        External,
    }
}
#[doc = "Parameters supplied to the Update Group operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupUpdateParameters {
    #[doc = "Parameters supplied to the Update Group operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GroupUpdateParametersProperties>,
}
impl GroupUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Update Group operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupUpdateParametersProperties {
    #[doc = "Group name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Group description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Group type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<group_update_parameters_properties::Type>,
    #[doc = "Identifier of the external groups, this property contains the id of the group from the external identity provider, e.g. for Azure Active Directory `aad://<tenant>.onmicrosoft.com/groups/<group object id>`; otherwise the value is null."]
    #[serde(rename = "externalId", default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}
impl GroupUpdateParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod group_update_parameters_properties {
    use super::*;
    #[doc = "Group type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "custom")]
        Custom,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "external")]
        External,
    }
}
#[doc = "HTTP header and it's value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpHeader {
    #[doc = "Header name."]
    pub name: String,
    #[doc = "Header value."]
    pub value: String,
}
impl HttpHeader {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "Custom hostname configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostnameConfiguration {
    #[doc = "Hostname type."]
    #[serde(rename = "type")]
    pub type_: hostname_configuration::Type,
    #[doc = "Hostname to configure on the Api Management service."]
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[doc = "Url to the KeyVault Secret containing the Ssl Certificate. If absolute Url containing version is provided, auto-update of ssl certificate will not work. This requires Api Management service to be configured with aka.ms/apimmsi. The secret should be of type *application/x-pkcs12*"]
    #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_id: Option<String>,
    #[doc = "System or User Assigned Managed identity clientId as generated by Azure AD, which has GET access to the keyVault containing the SSL certificate."]
    #[serde(rename = "identityClientId", default, skip_serializing_if = "Option::is_none")]
    pub identity_client_id: Option<String>,
    #[doc = "Base64 Encoded certificate."]
    #[serde(rename = "encodedCertificate", default, skip_serializing_if = "Option::is_none")]
    pub encoded_certificate: Option<String>,
    #[doc = "Certificate Password."]
    #[serde(rename = "certificatePassword", default, skip_serializing_if = "Option::is_none")]
    pub certificate_password: Option<String>,
    #[doc = "Specify true to setup the certificate associated with this Hostname as the Default SSL Certificate. If a client does not send the SNI header, then this will be the certificate that will be challenged. The property is useful if a service has multiple custom hostname enabled and it needs to decide on the default ssl certificate. The setting only applied to gateway Hostname Type."]
    #[serde(rename = "defaultSslBinding", default, skip_serializing_if = "Option::is_none")]
    pub default_ssl_binding: Option<bool>,
    #[doc = "Specify true to always negotiate client certificate on the hostname. Default Value is false."]
    #[serde(rename = "negotiateClientCertificate", default, skip_serializing_if = "Option::is_none")]
    pub negotiate_client_certificate: Option<bool>,
    #[doc = "SSL certificate information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<CertificateInformation>,
    #[doc = "Certificate Source."]
    #[serde(rename = "certificateSource", default, skip_serializing_if = "Option::is_none")]
    pub certificate_source: Option<hostname_configuration::CertificateSource>,
    #[doc = "Certificate Status."]
    #[serde(rename = "certificateStatus", default, skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<hostname_configuration::CertificateStatus>,
}
impl HostnameConfiguration {
    pub fn new(type_: hostname_configuration::Type, host_name: String) -> Self {
        Self {
            type_,
            host_name,
            key_vault_id: None,
            identity_client_id: None,
            encoded_certificate: None,
            certificate_password: None,
            default_ssl_binding: None,
            negotiate_client_certificate: None,
            certificate: None,
            certificate_source: None,
            certificate_status: None,
        }
    }
}
pub mod hostname_configuration {
    use super::*;
    #[doc = "Hostname type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Proxy,
        Portal,
        Management,
        Scm,
        DeveloperPortal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Proxy => serializer.serialize_unit_variant("Type", 0u32, "Proxy"),
                Self::Portal => serializer.serialize_unit_variant("Type", 1u32, "Portal"),
                Self::Management => serializer.serialize_unit_variant("Type", 2u32, "Management"),
                Self::Scm => serializer.serialize_unit_variant("Type", 3u32, "Scm"),
                Self::DeveloperPortal => serializer.serialize_unit_variant("Type", 4u32, "DeveloperPortal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Certificate Source."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CertificateSource")]
    pub enum CertificateSource {
        Managed,
        KeyVault,
        Custom,
        BuiltIn,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CertificateSource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CertificateSource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CertificateSource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Managed => serializer.serialize_unit_variant("CertificateSource", 0u32, "Managed"),
                Self::KeyVault => serializer.serialize_unit_variant("CertificateSource", 1u32, "KeyVault"),
                Self::Custom => serializer.serialize_unit_variant("CertificateSource", 2u32, "Custom"),
                Self::BuiltIn => serializer.serialize_unit_variant("CertificateSource", 3u32, "BuiltIn"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Certificate Status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CertificateStatus")]
    pub enum CertificateStatus {
        Completed,
        Failed,
        InProgress,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CertificateStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CertificateStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CertificateStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Completed => serializer.serialize_unit_variant("CertificateStatus", 0u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("CertificateStatus", 1u32, "Failed"),
                Self::InProgress => serializer.serialize_unit_variant("CertificateStatus", 2u32, "InProgress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Http message diagnostic settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpMessageDiagnostic {
    #[doc = "Array of HTTP Headers to log."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub headers: Vec<String>,
    #[doc = "Body logging settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<BodyDiagnosticSettings>,
    #[serde(rename = "dataMasking", default, skip_serializing_if = "Option::is_none")]
    pub data_masking: Option<DataMasking>,
}
impl HttpMessageDiagnostic {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity Provider Base Parameter Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityProviderBaseParameters {
    #[doc = "Identity Provider Type identifier."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity_provider_base_parameters::Type>,
    #[doc = "The TenantId to use instead of Common when logging into Active Directory"]
    #[serde(rename = "signinTenant", default, skip_serializing_if = "Option::is_none")]
    pub signin_tenant: Option<String>,
    #[doc = "List of Allowed Tenants when configuring Azure Active Directory login."]
    #[serde(
        rename = "allowedTenants",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_tenants: Vec<String>,
    #[doc = "OpenID Connect discovery endpoint hostname for AAD or AAD B2C."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[doc = "Signup Policy Name. Only applies to AAD B2C Identity Provider."]
    #[serde(rename = "signupPolicyName", default, skip_serializing_if = "Option::is_none")]
    pub signup_policy_name: Option<String>,
    #[doc = "Signin Policy Name. Only applies to AAD B2C Identity Provider."]
    #[serde(rename = "signinPolicyName", default, skip_serializing_if = "Option::is_none")]
    pub signin_policy_name: Option<String>,
    #[doc = "Profile Editing Policy Name. Only applies to AAD B2C Identity Provider."]
    #[serde(rename = "profileEditingPolicyName", default, skip_serializing_if = "Option::is_none")]
    pub profile_editing_policy_name: Option<String>,
    #[doc = "Password Reset Policy Name. Only applies to AAD B2C Identity Provider."]
    #[serde(rename = "passwordResetPolicyName", default, skip_serializing_if = "Option::is_none")]
    pub password_reset_policy_name: Option<String>,
    #[doc = "The client library to be used in the developer portal. Only applies to AAD and AAD B2C Identity Provider."]
    #[serde(rename = "clientLibrary", default, skip_serializing_if = "Option::is_none")]
    pub client_library: Option<String>,
}
impl IdentityProviderBaseParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity_provider_base_parameters {
    use super::*;
    #[doc = "Identity Provider Type identifier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "facebook")]
        Facebook,
        #[serde(rename = "google")]
        Google,
        #[serde(rename = "microsoft")]
        Microsoft,
        #[serde(rename = "twitter")]
        Twitter,
        #[serde(rename = "aad")]
        Aad,
        #[serde(rename = "aadB2C")]
        AadB2c,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Facebook => serializer.serialize_unit_variant("Type", 0u32, "facebook"),
                Self::Google => serializer.serialize_unit_variant("Type", 1u32, "google"),
                Self::Microsoft => serializer.serialize_unit_variant("Type", 2u32, "microsoft"),
                Self::Twitter => serializer.serialize_unit_variant("Type", 3u32, "twitter"),
                Self::Aad => serializer.serialize_unit_variant("Type", 4u32, "aad"),
                Self::AadB2c => serializer.serialize_unit_variant("Type", 5u32, "aadB2C"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Identity Provider details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityProviderContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The external Identity Providers like Facebook, Google, Microsoft, Twitter or Azure Active Directory which can be used to enable access to the API Management service developer portal for all users."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IdentityProviderContractProperties>,
}
impl IdentityProviderContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The external Identity Providers like Facebook, Google, Microsoft, Twitter or Azure Active Directory which can be used to enable access to the API Management service developer portal for all users."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderContractProperties {
    #[serde(flatten)]
    pub identity_provider_base_parameters: IdentityProviderBaseParameters,
    #[doc = "Client Id of the Application in the external Identity Provider. It is App ID for Facebook login, Client ID for Google login, App ID for Microsoft."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Client secret of the Application in external Identity Provider, used to authenticate login request. For example, it is App Secret for Facebook login, API Key for Google login, Public Key for Microsoft. This property will not be filled on 'GET' operations! Use '/listSecrets' POST request to get the value."]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}
impl IdentityProviderContractProperties {
    pub fn new(client_id: String) -> Self {
        Self {
            identity_provider_base_parameters: IdentityProviderBaseParameters::default(),
            client_id,
            client_secret: None,
        }
    }
}
#[doc = "Identity Provider details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityProviderCreateContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The external Identity Providers like Facebook, Google, Microsoft, Twitter or Azure Active Directory which can be used to enable access to the API Management service developer portal for all users."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IdentityProviderCreateContractProperties>,
}
impl IdentityProviderCreateContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The external Identity Providers like Facebook, Google, Microsoft, Twitter or Azure Active Directory which can be used to enable access to the API Management service developer portal for all users."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderCreateContractProperties {
    #[serde(flatten)]
    pub identity_provider_base_parameters: IdentityProviderBaseParameters,
    #[doc = "Client Id of the Application in the external Identity Provider. It is App ID for Facebook login, Client ID for Google login, App ID for Microsoft."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Client secret of the Application in external Identity Provider, used to authenticate login request. For example, it is App Secret for Facebook login, API Key for Google login, Public Key for Microsoft. This property will not be filled on 'GET' operations! Use '/listSecrets' POST request to get the value."]
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
}
impl IdentityProviderCreateContractProperties {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            identity_provider_base_parameters: IdentityProviderBaseParameters::default(),
            client_id,
            client_secret,
        }
    }
}
#[doc = "List of all the Identity Providers configured on the service instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityProviderList {
    #[doc = "Identity Provider configuration values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<IdentityProviderContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IdentityProviderList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl IdentityProviderList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to update Identity Provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityProviderUpdateParameters {
    #[doc = "Parameters supplied to the Update Identity Provider operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IdentityProviderUpdateProperties>,
}
impl IdentityProviderUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Update Identity Provider operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityProviderUpdateProperties {
    #[serde(flatten)]
    pub identity_provider_base_parameters: IdentityProviderBaseParameters,
    #[doc = "Client Id of the Application in the external Identity Provider. It is App ID for Facebook login, Client ID for Google login, App ID for Microsoft."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Client secret of the Application in external Identity Provider, used to authenticate login request. For example, it is App Secret for Facebook login, API Key for Google login, Public Key for Microsoft."]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}
impl IdentityProviderUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Issue Attachment list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssueAttachmentCollection {
    #[doc = "Issue Attachment values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<IssueAttachmentContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IssueAttachmentCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl IssueAttachmentCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Issue Attachment Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssueAttachmentContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Issue Attachment contract Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IssueAttachmentContractProperties>,
}
impl IssueAttachmentContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Issue Attachment contract Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueAttachmentContractProperties {
    #[doc = "Filename by which the binary data will be saved."]
    pub title: String,
    #[doc = "Either 'link' if content is provided via an HTTP link or the MIME type of the Base64-encoded binary data provided in the 'content' property."]
    #[serde(rename = "contentFormat")]
    pub content_format: String,
    #[doc = "An HTTP link or Base64-encoded binary data."]
    pub content: String,
}
impl IssueAttachmentContractProperties {
    pub fn new(title: String, content_format: String, content: String) -> Self {
        Self {
            title,
            content_format,
            content,
        }
    }
}
#[doc = "Paged Issue list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssueCollection {
    #[doc = "Issue values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<IssueContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IssueCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl IssueCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Issue Comment list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssueCommentCollection {
    #[doc = "Issue Comment values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<IssueCommentContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IssueCommentCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl IssueCommentCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Issue Comment Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssueCommentContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Issue Comment contract Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IssueCommentContractProperties>,
}
impl IssueCommentContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Issue Comment contract Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueCommentContractProperties {
    #[doc = "Comment text."]
    pub text: String,
    #[doc = "Date and time when the comment was created."]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "A resource identifier for the user who left the comment."]
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl IssueCommentContractProperties {
    pub fn new(text: String, user_id: String) -> Self {
        Self {
            text,
            created_date: None,
            user_id,
        }
    }
}
#[doc = "A key-value pair that provides additional context on the issue."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssueContext {}
impl IssueContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Issue Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssueContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Issue contract Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IssueContractProperties>,
}
impl IssueContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Issue contract Base Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssueContractBaseProperties {
    #[doc = "Date and time when the issue was created."]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Status of the issue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<issue_contract_base_properties::State>,
    #[doc = "A resource identifier for the API the issue was created for."]
    #[serde(rename = "apiId", default, skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
}
impl IssueContractBaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod issue_contract_base_properties {
    use super::*;
    #[doc = "Status of the issue."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        #[serde(rename = "proposed")]
        Proposed,
        #[serde(rename = "open")]
        Open,
        #[serde(rename = "removed")]
        Removed,
        #[serde(rename = "resolved")]
        Resolved,
        #[serde(rename = "closed")]
        Closed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Proposed => serializer.serialize_unit_variant("State", 0u32, "proposed"),
                Self::Open => serializer.serialize_unit_variant("State", 1u32, "open"),
                Self::Removed => serializer.serialize_unit_variant("State", 2u32, "removed"),
                Self::Resolved => serializer.serialize_unit_variant("State", 3u32, "resolved"),
                Self::Closed => serializer.serialize_unit_variant("State", 4u32, "closed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Issue contract Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueContractProperties {
    #[serde(flatten)]
    pub issue_contract_base_properties: IssueContractBaseProperties,
    #[doc = "The issue title."]
    pub title: String,
    #[doc = "Text describing the issue."]
    pub description: String,
    #[doc = "A resource identifier for the user created the issue."]
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl IssueContractProperties {
    pub fn new(title: String, description: String, user_id: String) -> Self {
        Self {
            issue_contract_base_properties: IssueContractBaseProperties::default(),
            title,
            description,
            user_id,
        }
    }
}
#[doc = "Issue update Parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssueUpdateContract {
    #[doc = "Issue contract Update Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IssueUpdateContractProperties>,
}
impl IssueUpdateContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Issue contract Update Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssueUpdateContractProperties {
    #[serde(flatten)]
    pub issue_contract_base_properties: IssueContractBaseProperties,
    #[doc = "The issue title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Text describing the issue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A resource identifier for the user created the issue."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
impl IssueUpdateContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Create keyVault contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultContractCreateProperties {
    #[doc = "Key vault secret identifier for fetching secret. Providing a versioned secret will prevent auto-refresh. This requires API Management service to be configured with aka.ms/apimmsi"]
    #[serde(rename = "secretIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub secret_identifier: Option<String>,
    #[doc = "Null for SystemAssignedIdentity or Client Id for UserAssignedIdentity , which will be used to access key vault secret."]
    #[serde(rename = "identityClientId", default, skip_serializing_if = "Option::is_none")]
    pub identity_client_id: Option<String>,
}
impl KeyVaultContractCreateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "KeyVault contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultContractProperties {
    #[serde(flatten)]
    pub key_vault_contract_create_properties: KeyVaultContractCreateProperties,
    #[doc = "Issue contract Update Properties."]
    #[serde(rename = "lastStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_status: Option<KeyVaultLastAccessStatusContractProperties>,
}
impl KeyVaultContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Issue contract Update Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultLastAccessStatusContractProperties {
    #[doc = "Last status code for sync and refresh of secret from key vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Details of the error else empty."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Last time secret was accessed. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "timeStampUtc", default, with = "azure_core::date::rfc3339::option")]
    pub time_stamp_utc: Option<time::OffsetDateTime>,
}
impl KeyVaultLastAccessStatusContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Logger list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoggerCollection {
    #[doc = "Logger values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<LoggerContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LoggerCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LoggerCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Logger details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoggerContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The Logger entity in API Management represents an event sink that you can use to log API Management events. Currently the Logger entity supports logging API Management events to Azure Event Hubs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LoggerContractProperties>,
}
impl LoggerContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Logger entity in API Management represents an event sink that you can use to log API Management events. Currently the Logger entity supports logging API Management events to Azure Event Hubs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoggerContractProperties {
    #[doc = "Logger type."]
    #[serde(rename = "loggerType")]
    pub logger_type: logger_contract_properties::LoggerType,
    #[doc = "Logger description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The name and SendRule connection string of the event hub for azureEventHub logger.\nInstrumentation key for applicationInsights logger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<serde_json::Value>,
    #[doc = "Whether records are buffered in the logger before publishing. Default is assumed to be true."]
    #[serde(rename = "isBuffered", default, skip_serializing_if = "Option::is_none")]
    pub is_buffered: Option<bool>,
    #[doc = "Azure Resource Id of a log target (either Azure Event Hub resource or Azure Application Insights resource)."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl LoggerContractProperties {
    pub fn new(logger_type: logger_contract_properties::LoggerType) -> Self {
        Self {
            logger_type,
            description: None,
            credentials: None,
            is_buffered: None,
            resource_id: None,
        }
    }
}
pub mod logger_contract_properties {
    use super::*;
    #[doc = "Logger type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LoggerType")]
    pub enum LoggerType {
        #[serde(rename = "azureEventHub")]
        AzureEventHub,
        #[serde(rename = "applicationInsights")]
        ApplicationInsights,
        #[serde(rename = "azureMonitor")]
        AzureMonitor,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LoggerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LoggerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LoggerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureEventHub => serializer.serialize_unit_variant("LoggerType", 0u32, "azureEventHub"),
                Self::ApplicationInsights => serializer.serialize_unit_variant("LoggerType", 1u32, "applicationInsights"),
                Self::AzureMonitor => serializer.serialize_unit_variant("LoggerType", 2u32, "azureMonitor"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Logger update contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoggerUpdateContract {
    #[doc = "Parameters supplied to the Update Logger operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LoggerUpdateParameters>,
}
impl LoggerUpdateContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Update Logger operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoggerUpdateParameters {
    #[doc = "Logger type."]
    #[serde(rename = "loggerType", default, skip_serializing_if = "Option::is_none")]
    pub logger_type: Option<logger_update_parameters::LoggerType>,
    #[doc = "Logger description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Logger credentials."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<serde_json::Value>,
    #[doc = "Whether records are buffered in the logger before publishing. Default is assumed to be true."]
    #[serde(rename = "isBuffered", default, skip_serializing_if = "Option::is_none")]
    pub is_buffered: Option<bool>,
}
impl LoggerUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod logger_update_parameters {
    use super::*;
    #[doc = "Logger type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LoggerType")]
    pub enum LoggerType {
        #[serde(rename = "azureEventHub")]
        AzureEventHub,
        #[serde(rename = "applicationInsights")]
        ApplicationInsights,
        #[serde(rename = "azureMonitor")]
        AzureMonitor,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LoggerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LoggerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LoggerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureEventHub => serializer.serialize_unit_variant("LoggerType", 0u32, "azureEventHub"),
                Self::ApplicationInsights => serializer.serialize_unit_variant("LoggerType", 1u32, "applicationInsights"),
                Self::AzureMonitor => serializer.serialize_unit_variant("LoggerType", 2u32, "azureMonitor"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Paged NamedValue list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamedValueCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NamedValueContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NamedValueCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NamedValueCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NamedValue details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamedValueContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "NamedValue Contract properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamedValueContractProperties>,
}
impl NamedValueContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NamedValue Contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedValueContractProperties {
    #[serde(flatten)]
    pub named_value_entity_base_parameters: NamedValueEntityBaseParameters,
    #[doc = "Unique name of NamedValue. It may contain only letters, digits, period, dash, and underscore characters."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Value of the NamedValue. Can contain policy expressions. It may not be empty or consist only of whitespace. This property will not be filled on 'GET' operations! Use '/listSecrets' POST request to get the value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "KeyVault contract details."]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KeyVaultContractProperties>,
}
impl NamedValueContractProperties {
    pub fn new(display_name: String) -> Self {
        Self {
            named_value_entity_base_parameters: NamedValueEntityBaseParameters::default(),
            display_name,
            value: None,
            key_vault: None,
        }
    }
}
#[doc = "NamedValue details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamedValueCreateContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "NamedValue Contract properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamedValueCreateContractProperties>,
}
impl NamedValueCreateContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NamedValue Contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedValueCreateContractProperties {
    #[serde(flatten)]
    pub named_value_entity_base_parameters: NamedValueEntityBaseParameters,
    #[doc = "Unique name of NamedValue. It may contain only letters, digits, period, dash, and underscore characters."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Value of the NamedValue. Can contain policy expressions. It may not be empty or consist only of whitespace. This property will not be filled on 'GET' operations! Use '/listSecrets' POST request to get the value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Create keyVault contract details."]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KeyVaultContractCreateProperties>,
}
impl NamedValueCreateContractProperties {
    pub fn new(display_name: String) -> Self {
        Self {
            named_value_entity_base_parameters: NamedValueEntityBaseParameters::default(),
            display_name,
            value: None,
            key_vault: None,
        }
    }
}
#[doc = "NamedValue Entity Base Parameters set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamedValueEntityBaseParameters {
    #[doc = "Optional tags that when provided can be used to filter the NamedValue list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<String>,
    #[doc = "Determines whether the value is a secret and should be encrypted or not. Default value is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<bool>,
}
impl NamedValueEntityBaseParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Client or app secret used in IdentityProviders, Aad, OpenID or OAuth."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamedValueSecretContract {
    #[doc = "This is secret value of the NamedValue entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl NamedValueSecretContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NamedValue Contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamedValueUpdateParameterProperties {
    #[serde(flatten)]
    pub named_value_entity_base_parameters: NamedValueEntityBaseParameters,
    #[doc = "Unique name of NamedValue. It may contain only letters, digits, period, dash, and underscore characters."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Value of the NamedValue. Can contain policy expressions. It may not be empty or consist only of whitespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Create keyVault contract details."]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<KeyVaultContractCreateProperties>,
}
impl NamedValueUpdateParameterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "NamedValue update Parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamedValueUpdateParameters {
    #[doc = "NamedValue Contract properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamedValueUpdateParameterProperties>,
}
impl NamedValueUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network Status details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkStatusContract {
    #[doc = "Gets the list of DNS servers IPV4 addresses."]
    #[serde(rename = "dnsServers")]
    pub dns_servers: Vec<String>,
    #[doc = "Gets the list of Connectivity Status to the Resources on which the service depends upon."]
    #[serde(rename = "connectivityStatus")]
    pub connectivity_status: Vec<ConnectivityStatusContract>,
}
impl NetworkStatusContract {
    pub fn new(dns_servers: Vec<String>, connectivity_status: Vec<ConnectivityStatusContract>) -> Self {
        Self {
            dns_servers,
            connectivity_status,
        }
    }
}
#[doc = "Network Status in the Location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkStatusContractByLocation {
    #[doc = "Location of service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Network Status details."]
    #[serde(rename = "networkStatus", default, skip_serializing_if = "Option::is_none")]
    pub network_status: Option<NetworkStatusContract>,
}
impl NetworkStatusContractByLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Notification list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NotificationContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NotificationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NotificationCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Notification details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Notification Contract properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NotificationContractProperties>,
}
impl NotificationContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Notification Contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationContractProperties {
    #[doc = "Title of the Notification."]
    pub title: String,
    #[doc = "Description of the Notification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Notification Parameter contract."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<RecipientsContractProperties>,
}
impl NotificationContractProperties {
    pub fn new(title: String) -> Self {
        Self {
            title,
            description: None,
            recipients: None,
        }
    }
}
#[doc = "API OAuth2 Authentication settings details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuth2AuthenticationSettingsContract {
    #[doc = "OAuth authorization server identifier."]
    #[serde(rename = "authorizationServerId", default, skip_serializing_if = "Option::is_none")]
    pub authorization_server_id: Option<String>,
    #[doc = "operations scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
impl OAuth2AuthenticationSettingsContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API OAuth2 Authentication settings details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenIdAuthenticationSettingsContract {
    #[doc = "OAuth authorization server identifier."]
    #[serde(rename = "openidProviderId", default, skip_serializing_if = "Option::is_none")]
    pub openid_provider_id: Option<String>,
    #[doc = "How to send token to the server."]
    #[serde(
        rename = "bearerTokenSendingMethods",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bearer_token_sending_methods: Vec<BearerTokenSendingMethodsContract>,
}
impl OpenIdAuthenticationSettingsContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged OpenIdProviders list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenIdConnectProviderCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OpenidConnectProviderContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OpenIdConnectProviderCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OpenIdConnectProviderCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OpenId Connect Provider details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenidConnectProviderContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "OpenID Connect Providers Contract."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OpenidConnectProviderContractProperties>,
}
impl OpenidConnectProviderContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OpenID Connect Providers Contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenidConnectProviderContractProperties {
    #[doc = "User-friendly OpenID Connect Provider name."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "User-friendly description of OpenID Connect Provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Metadata endpoint URI."]
    #[serde(rename = "metadataEndpoint")]
    pub metadata_endpoint: String,
    #[doc = "Client ID of developer console which is the client application."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Client Secret of developer console which is the client application."]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "If true, the Open ID Connect provider may be used in the developer portal test console. True by default if no value is provided."]
    #[serde(rename = "useInTestConsole", default, skip_serializing_if = "Option::is_none")]
    pub use_in_test_console: Option<bool>,
    #[doc = "If true, the Open ID Connect provider will be used in the API documentation in the developer portal. False by default if no value is provided."]
    #[serde(rename = "useInApiDocumentation", default, skip_serializing_if = "Option::is_none")]
    pub use_in_api_documentation: Option<bool>,
}
impl OpenidConnectProviderContractProperties {
    pub fn new(display_name: String, metadata_endpoint: String, client_id: String) -> Self {
        Self {
            display_name,
            description: None,
            metadata_endpoint,
            client_id,
            client_secret: None,
            use_in_test_console: None,
            use_in_api_documentation: None,
        }
    }
}
#[doc = "Parameters supplied to the Update OpenID Connect Provider operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenidConnectProviderUpdateContract {
    #[doc = "Parameters supplied to the Update OpenID Connect Provider operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OpenidConnectProviderUpdateContractProperties>,
}
impl OpenidConnectProviderUpdateContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Update OpenID Connect Provider operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenidConnectProviderUpdateContractProperties {
    #[doc = "User-friendly OpenID Connect Provider name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "User-friendly description of OpenID Connect Provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Metadata endpoint URI."]
    #[serde(rename = "metadataEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub metadata_endpoint: Option<String>,
    #[doc = "Client ID of developer console which is the client application."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Client Secret of developer console which is the client application."]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "If true, the Open ID Connect provider may be used in the developer portal test console. True by default if no value is provided."]
    #[serde(rename = "useInTestConsole", default, skip_serializing_if = "Option::is_none")]
    pub use_in_test_console: Option<bool>,
    #[doc = "If true, the Open ID Connect provider will be used in the API documentation in the developer portal. False by default if no value is provided."]
    #[serde(rename = "useInApiDocumentation", default, skip_serializing_if = "Option::is_none")]
    pub use_in_api_documentation: Option<bool>,
}
impl OpenidConnectProviderUpdateContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that describes the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The operation origin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The operation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The object that describes the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Friendly name of the resource provider"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Operation type: read, write, delete, listKeys/action, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Resource type on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Friendly name of the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Paged Operation list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OperationContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API Operation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Operation Contract Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationContractProperties>,
}
impl OperationContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation Contract Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationContractProperties {
    #[serde(flatten)]
    pub operation_entity_base_contract: OperationEntityBaseContract,
    #[doc = "Operation Name."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "A Valid HTTP Operation Method. Typical Http Methods like GET, PUT, POST but not limited by only them."]
    pub method: String,
    #[doc = "Relative URL template identifying the target resource for this operation. May include parameters. Example: /customers/{cid}/orders/{oid}/?date={date}"]
    #[serde(rename = "urlTemplate")]
    pub url_template: String,
}
impl OperationContractProperties {
    pub fn new(display_name: String, method: String, url_template: String) -> Self {
        Self {
            operation_entity_base_contract: OperationEntityBaseContract::default(),
            display_name,
            method,
            url_template,
        }
    }
}
#[doc = "API Operation Entity Base Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntityBaseContract {
    #[doc = "Collection of URL template parameters."]
    #[serde(
        rename = "templateParameters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub template_parameters: Vec<ParameterContract>,
    #[doc = "Description of the operation. May include HTML formatting tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Operation request details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<RequestContract>,
    #[doc = "Array of Operation responses."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub responses: Vec<ResponseContract>,
    #[doc = "Operation Policies"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<String>,
}
impl OperationEntityBaseContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list REST API operations. It contains a list of operations and a URL nextLink to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
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
#[doc = "Long Running Git Operation Results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResultContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Operation Result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationResultContractProperties>,
}
impl OperationResultContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResultContractProperties {
    #[doc = "Operation result identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of an async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_result_contract_properties::Status>,
    #[doc = "Start time of an async operation. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub started: Option<time::OffsetDateTime>,
    #[doc = "Last update time of an async operation. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub updated: Option<time::OffsetDateTime>,
    #[doc = "Optional result info."]
    #[serde(rename = "resultInfo", default, skip_serializing_if = "Option::is_none")]
    pub result_info: Option<String>,
    #[doc = "Error Body contract."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
    #[doc = "This property if only provided as part of the TenantConfiguration_Validate operation. It contains the log the entities which will be updated/created/deleted as part of the TenantConfiguration_Deploy operation."]
    #[serde(
        rename = "actionLog",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub action_log: Vec<OperationResultLogItemContract>,
}
impl OperationResultContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_result_contract_properties {
    use super::*;
    #[doc = "Status of an async operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Started,
        InProgress,
        Succeeded,
        Failed,
    }
}
#[doc = "Log of the entity being created, updated or deleted."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResultLogItemContract {
    #[doc = "The type of entity contract."]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "Action like create/update/delete."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "Identifier of the entity being created/updated/deleted."]
    #[serde(rename = "objectKey", default, skip_serializing_if = "Option::is_none")]
    pub object_key: Option<String>,
}
impl OperationResultLogItemContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation Entity contract Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationTagResourceContractProperties {
    #[doc = "Identifier of the operation in form /operations/{operationId}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "API Name."]
    #[serde(rename = "apiName", default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    #[doc = "API Revision."]
    #[serde(rename = "apiRevision", default, skip_serializing_if = "Option::is_none")]
    pub api_revision: Option<String>,
    #[doc = "API Version."]
    #[serde(rename = "apiVersion", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[doc = "Operation Description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A Valid HTTP Operation Method. Typical Http Methods like GET, PUT, POST but not limited by only them."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "Relative URL template identifying the target resource for this operation. May include parameters. Example: /customers/{cid}/orders/{oid}/?date={date}"]
    #[serde(rename = "urlTemplate", default, skip_serializing_if = "Option::is_none")]
    pub url_template: Option<String>,
}
impl OperationTagResourceContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API Operation Update Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationUpdateContract {
    #[doc = "Operation Update Contract Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationUpdateContractProperties>,
}
impl OperationUpdateContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation Update Contract Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationUpdateContractProperties {
    #[serde(flatten)]
    pub operation_entity_base_contract: OperationEntityBaseContract,
    #[doc = "Operation Name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "A Valid HTTP Operation Method. Typical Http Methods like GET, PUT, POST but not limited by only them."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "Relative URL template identifying the target resource for this operation. May include parameters. Example: /customers/{cid}/orders/{oid}/?date={date}"]
    #[serde(rename = "urlTemplate", default, skip_serializing_if = "Option::is_none")]
    pub url_template: Option<String>,
}
impl OperationUpdateContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Endpoints accessed for a common purpose that the Api Management Service requires outbound network access to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEnvironmentEndpoint {
    #[doc = "The type of service accessed by the Api Management Service, e.g., Azure Storage, Azure SQL Database, and Azure Active Directory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The endpoints that the Api Management Service reaches the service at."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endpoints: Vec<EndpointDependency>,
}
impl OutboundEnvironmentEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of Outbound Environment Endpoints"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutboundEnvironmentEndpointList {
    #[doc = "Collection of resources."]
    pub value: Vec<OutboundEnvironmentEndpoint>,
    #[doc = "Link to next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OutboundEnvironmentEndpointList {
    pub fn new(value: Vec<OutboundEnvironmentEndpoint>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Operation parameters details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterContract {
    #[doc = "Parameter name."]
    pub name: String,
    #[doc = "Parameter description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Parameter type."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Default parameter value."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "Specifies whether parameter is required or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[doc = "Parameter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
    #[doc = "Schema identifier."]
    #[serde(rename = "schemaId", default, skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    #[doc = "Type name defined by the schema."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "Parameter examples."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub examples: Option<ParameterExamplesContract>,
}
impl ParameterContract {
    pub fn new(name: String, type_: String) -> Self {
        Self {
            name,
            description: None,
            type_,
            default_value: None,
            required: None,
            values: Vec::new(),
            schema_id: None,
            type_name: None,
            examples: None,
        }
    }
}
#[doc = "Parameter example."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterExampleContract {
    #[doc = "Short description for the example"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "Long description for the example"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Example value. May be a primitive value, or an object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[doc = "A URL that points to the literal example"]
    #[serde(rename = "externalValue", default, skip_serializing_if = "Option::is_none")]
    pub external_value: Option<String>,
}
impl ParameterExampleContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameter examples."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterExamplesContract {}
impl ParameterExamplesContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Diagnostic settings for incoming/outgoing HTTP messages to the Gateway."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineDiagnosticSettings {
    #[doc = "Http message diagnostic settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<HttpMessageDiagnostic>,
    #[doc = "Http message diagnostic settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<HttpMessageDiagnostic>,
}
impl PipelineDiagnosticSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of the list policy operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyCollection {
    #[doc = "Policy Contract value."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PolicyContract>,
    #[doc = "Total record count number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PolicyCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PolicyCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Policy contract Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyContractProperties>,
}
impl PolicyContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy contract Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyContractProperties {
    #[doc = "Contents of the Policy as defined by the format."]
    pub value: String,
    #[doc = "Format of the policyContent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<policy_contract_properties::Format>,
}
impl PolicyContractProperties {
    pub fn new(value: String) -> Self {
        Self { value, format: None }
    }
}
pub mod policy_contract_properties {
    use super::*;
    #[doc = "Format of the policyContent."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Format")]
    pub enum Format {
        #[serde(rename = "xml")]
        Xml,
        #[serde(rename = "xml-link")]
        XmlLink,
        #[serde(rename = "rawxml")]
        Rawxml,
        #[serde(rename = "rawxml-link")]
        RawxmlLink,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Format {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Format {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Format {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Xml => serializer.serialize_unit_variant("Format", 0u32, "xml"),
                Self::XmlLink => serializer.serialize_unit_variant("Format", 1u32, "xml-link"),
                Self::Rawxml => serializer.serialize_unit_variant("Format", 2u32, "rawxml"),
                Self::RawxmlLink => serializer.serialize_unit_variant("Format", 3u32, "rawxml-link"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Format {
        fn default() -> Self {
            Self::Xml
        }
    }
}
#[doc = "Descriptions of API Management policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDescriptionCollection {
    #[doc = "Descriptions of API Management policies."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PolicyDescriptionContract>,
    #[doc = "Total record count number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl PolicyDescriptionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy description details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDescriptionContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Policy description properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyDescriptionContractProperties>,
}
impl PolicyDescriptionContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy description properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDescriptionContractProperties {
    #[doc = "Policy description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Binary OR value of the Snippet scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<i64>,
}
impl PolicyDescriptionContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of the get policy fragments operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyFragmentCollection {
    #[doc = "Policy fragment contract value."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PolicyFragmentContract>,
    #[doc = "Total record count number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PolicyFragmentCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PolicyFragmentCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy fragment contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyFragmentContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Policy fragment contract properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyFragmentContractProperties>,
}
impl PolicyFragmentContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy fragment contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyFragmentContractProperties {
    #[doc = "Contents of the policy fragment."]
    pub value: String,
    #[doc = "Policy fragment description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Format of the policy fragment content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<policy_fragment_contract_properties::Format>,
}
impl PolicyFragmentContractProperties {
    pub fn new(value: String) -> Self {
        Self {
            value,
            description: None,
            format: None,
        }
    }
}
pub mod policy_fragment_contract_properties {
    use super::*;
    #[doc = "Format of the policy fragment content."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Format")]
    pub enum Format {
        #[serde(rename = "xml")]
        Xml,
        #[serde(rename = "rawxml")]
        Rawxml,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Format {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Format {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Format {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Xml => serializer.serialize_unit_variant("Format", 0u32, "xml"),
                Self::Rawxml => serializer.serialize_unit_variant("Format", 1u32, "rawxml"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Format {
        fn default() -> Self {
            Self::Xml
        }
    }
}
#[doc = "The collection of the developer portal configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalConfigCollection {
    #[doc = "The developer portal configurations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PortalConfigContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PortalConfigCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PortalConfigCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The developer portal configuration contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalConfigContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The developer portal configuration contract properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PortalConfigProperties>,
}
impl PortalConfigContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The developer portal Cross-Origin Resource Sharing (CORS) settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalConfigCorsProperties {
    #[doc = "Allowed origins, e.g. `https://trusted.com`."]
    #[serde(
        rename = "allowedOrigins",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_origins: Vec<String>,
}
impl PortalConfigCorsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The developer portal Content Security Policy (CSP) settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalConfigCspProperties {
    #[doc = "The mode of the developer portal Content Security Policy (CSP)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<portal_config_csp_properties::Mode>,
    #[doc = "The URLs used by the browser to report CSP violations."]
    #[serde(
        rename = "reportUri",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub report_uri: Vec<String>,
    #[doc = "Allowed sources, e.g. `*.trusted.com`, `trusted.com`, `https://`."]
    #[serde(
        rename = "allowedSources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_sources: Vec<String>,
}
impl PortalConfigCspProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod portal_config_csp_properties {
    use super::*;
    #[doc = "The mode of the developer portal Content Security Policy (CSP)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "reportOnly")]
        ReportOnly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("Mode", 0u32, "enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Mode", 1u32, "disabled"),
                Self::ReportOnly => serializer.serialize_unit_variant("Mode", 2u32, "reportOnly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Mode {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalConfigDelegationProperties {
    #[doc = "Enable or disable delegation for user registration."]
    #[serde(rename = "delegateRegistration", default, skip_serializing_if = "Option::is_none")]
    pub delegate_registration: Option<bool>,
    #[doc = "Enable or disable delegation for product subscriptions."]
    #[serde(rename = "delegateSubscription", default, skip_serializing_if = "Option::is_none")]
    pub delegate_subscription: Option<bool>,
    #[doc = "A delegation endpoint URL."]
    #[serde(rename = "delegationUrl", default, skip_serializing_if = "Option::is_none")]
    pub delegation_url: Option<String>,
    #[doc = "A base64-encoded validation key to ensure requests originate from Azure API Management service."]
    #[serde(rename = "validationKey", default, skip_serializing_if = "Option::is_none")]
    pub validation_key: Option<String>,
}
impl PortalConfigDelegationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The developer portal configuration contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalConfigProperties {
    #[doc = "Enable or disable Basic authentication method."]
    #[serde(rename = "enableBasicAuth", default, skip_serializing_if = "Option::is_none")]
    pub enable_basic_auth: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signin: Option<portal_config_properties::Signin>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signup: Option<portal_config_properties::Signup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delegation: Option<PortalConfigDelegationProperties>,
    #[doc = "The developer portal Cross-Origin Resource Sharing (CORS) settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cors: Option<PortalConfigCorsProperties>,
    #[doc = "The developer portal Content Security Policy (CSP) settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csp: Option<PortalConfigCspProperties>,
}
impl PortalConfigProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod portal_config_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Signin {
        #[doc = "Redirect anonymous users to the sign-in page."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub require: Option<bool>,
    }
    impl Signin {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Signup {
        #[doc = "Terms of service contract properties."]
        #[serde(rename = "termsOfService", default, skip_serializing_if = "Option::is_none")]
        pub terms_of_service: Option<PortalConfigTermsOfServiceProperties>,
    }
    impl Signup {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Terms of service contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalConfigTermsOfServiceProperties {
    #[doc = "A terms of service text."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "Ask user for consent to the terms of service."]
    #[serde(rename = "requireConsent", default, skip_serializing_if = "Option::is_none")]
    pub require_consent: Option<bool>,
}
impl PortalConfigTermsOfServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Delegation settings for a developer portal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalDelegationSettings {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Delegation settings contract properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PortalDelegationSettingsProperties>,
}
impl PortalDelegationSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Delegation settings contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalDelegationSettingsProperties {
    #[doc = "A delegation Url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "A base64-encoded validation key to validate, that a request is coming from Azure API Management."]
    #[serde(rename = "validationKey", default, skip_serializing_if = "Option::is_none")]
    pub validation_key: Option<String>,
    #[doc = "Subscriptions delegation settings properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<SubscriptionsDelegationSettingsProperties>,
    #[doc = "User registration delegation settings properties."]
    #[serde(rename = "userRegistration", default, skip_serializing_if = "Option::is_none")]
    pub user_registration: Option<RegistrationDelegationSettingsProperties>,
}
impl PortalDelegationSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged list of portal revisions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalRevisionCollection {
    #[doc = "Collection of portal revisions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PortalRevisionContract>,
    #[doc = "Next page link, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PortalRevisionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PortalRevisionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Portal Revision's contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalRevisionContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PortalRevisionContractProperties>,
}
impl PortalRevisionContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalRevisionContractProperties {
    #[doc = "Portal revision description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Portal revision publishing status details."]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "Status of the portal's revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<portal_revision_contract_properties::Status>,
    #[doc = "Indicates if the portal's revision is public."]
    #[serde(rename = "isCurrent", default, skip_serializing_if = "Option::is_none")]
    pub is_current: Option<bool>,
    #[doc = "Portal's revision creation date and time."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "Last updated date and time."]
    #[serde(rename = "updatedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub updated_date_time: Option<time::OffsetDateTime>,
}
impl PortalRevisionContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod portal_revision_contract_properties {
    use super::*;
    #[doc = "Status of the portal's revision."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "publishing")]
        Publishing,
        #[serde(rename = "completed")]
        Completed,
        #[serde(rename = "failed")]
        Failed,
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
                Self::Pending => serializer.serialize_unit_variant("Status", 0u32, "pending"),
                Self::Publishing => serializer.serialize_unit_variant("Status", 1u32, "publishing"),
                Self::Completed => serializer.serialize_unit_variant("Status", 2u32, "completed"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Client or app secret used in IdentityProviders, Aad, OpenID or OAuth."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalSettingValidationKeyContract {
    #[doc = "This is secret value of the validation key in portal settings."]
    #[serde(rename = "validationKey", default, skip_serializing_if = "Option::is_none")]
    pub validation_key: Option<String>,
}
impl PortalSettingValidationKeyContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Descriptions of API Management policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalSettingsCollection {
    #[doc = "Descriptions of API Management policies."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PortalSettingsContract>,
    #[doc = "Total record count number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl PortalSettingsCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Portal Settings for the Developer Portal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalSettingsContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Sign-in settings contract properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PortalSettingsContractProperties>,
}
impl PortalSettingsContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sign-in settings contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalSettingsContractProperties {
    #[doc = "A delegation Url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "A base64-encoded validation key to validate, that a request is coming from Azure API Management."]
    #[serde(rename = "validationKey", default, skip_serializing_if = "Option::is_none")]
    pub validation_key: Option<String>,
    #[doc = "Subscriptions delegation settings properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<SubscriptionsDelegationSettingsProperties>,
    #[doc = "User registration delegation settings properties."]
    #[serde(rename = "userRegistration", default, skip_serializing_if = "Option::is_none")]
    pub user_registration: Option<RegistrationDelegationSettingsProperties>,
    #[doc = "Redirect Anonymous users to the Sign-In page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Terms of service contract properties."]
    #[serde(rename = "termsOfService", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<TermsOfServiceProperties>,
}
impl PortalSettingsContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sign-in settings contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalSigninSettingProperties {
    #[doc = "Redirect Anonymous users to the Sign-In page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl PortalSigninSettingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sign-In settings for the Developer Portal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalSigninSettings {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Sign-in settings contract properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PortalSigninSettingProperties>,
}
impl PortalSigninSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sign-Up settings for a developer portal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalSignupSettings {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Sign-up settings contract properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PortalSignupSettingsProperties>,
}
impl PortalSignupSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sign-up settings contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalSignupSettingsProperties {
    #[doc = "Allow users to sign up on a developer portal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Terms of service contract properties."]
    #[serde(rename = "termsOfService", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<TermsOfServiceProperties>,
}
impl PortalSignupSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for Private Endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the PrivateEndpointConnectProperties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of private endpoint connection associated with the specified storage account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of private endpoint connections"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateEndpointConnection>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the PrivateEndpointConnectProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The Private Endpoint resource."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "A collection of information about the state of the connection between service consumer and provider."]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            private_endpoint: None,
            private_link_service_connection_state,
            provisioning_state: None,
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointConnectionProvisioningState")]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointConnectionProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointConnectionProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointConnectionProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 0u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 1u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A request to approve or reject a private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionRequest {
    #[doc = "Private Endpoint Connection Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The connection state of the private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<private_endpoint_connection_request::Properties>,
}
impl PrivateEndpointConnectionRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_endpoint_connection_request {
    use super::*;
    #[doc = "The connection state of the private endpoint connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "A collection of information about the state of the connection between service consumer and provider."]
        #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
        pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Properties of the PrivateEndpointConnectProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionWrapperProperties {
    #[doc = "A wrapper for an ARM resource id"]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<ArmIdWrapper>,
    #[doc = "A collection of information about the state of the connection between service consumer and provider."]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[doc = "The provisioning state of the private endpoint connection resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "All the Group ids."]
    #[serde(
        rename = "groupIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_ids: Vec<String>,
}
impl PrivateEndpointConnectionWrapperProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            private_endpoint: None,
            private_link_service_connection_state,
            provisioning_state: None,
            group_ids: Vec::new(),
        }
    }
}
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointServiceConnectionStatus")]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointServiceConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointServiceConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointServiceConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 2u32, "Rejected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Array of private link resources"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateLinkResource>,
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(
        rename = "requiredMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(
        rename = "requiredZoneNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of information about the state of the connection between service consumer and provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval/rejection of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Product-API link list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductApiLinkCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProductApiLinkContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProductApiLinkCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProductApiLinkCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Product-API link details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductApiLinkContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Product-API link entity properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductApiLinkContractProperties>,
}
impl ProductApiLinkContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Product-API link entity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductApiLinkContractProperties {
    #[doc = "Full resource Id of an API."]
    #[serde(rename = "apiId")]
    pub api_id: String,
}
impl ProductApiLinkContractProperties {
    pub fn new(api_id: String) -> Self {
        Self { api_id }
    }
}
#[doc = "Paged Products list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProductContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProductCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProductCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Product details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Product profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductContractProperties>,
}
impl ProductContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Product profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductContractProperties {
    #[serde(flatten)]
    pub product_entity_base_parameters: ProductEntityBaseParameters,
    #[doc = "Product name."]
    #[serde(rename = "displayName")]
    pub display_name: String,
}
impl ProductContractProperties {
    pub fn new(display_name: String) -> Self {
        Self {
            product_entity_base_parameters: ProductEntityBaseParameters::default(),
            display_name,
        }
    }
}
#[doc = "Product Entity Base Parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductEntityBaseParameters {
    #[doc = "Product description. May include HTML formatting tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Product terms of use. Developers trying to subscribe to the product will be presented and required to accept these terms before they can complete the subscription process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms: Option<String>,
    #[doc = "Whether a product subscription is required for accessing APIs included in this product. If true, the product is referred to as \"protected\" and a valid subscription key is required for a request to an API included in the product to succeed. If false, the product is referred to as \"open\" and requests to an API included in the product can be made without a subscription key. If property is omitted when creating a new product it's value is assumed to be true."]
    #[serde(rename = "subscriptionRequired", default, skip_serializing_if = "Option::is_none")]
    pub subscription_required: Option<bool>,
    #[doc = "whether subscription approval is required. If false, new subscriptions will be approved automatically enabling developers to call the product’s APIs immediately after subscribing. If true, administrators must manually approve the subscription before the developer can any of the product’s APIs. Can be present only if subscriptionRequired property is present and has a value of false."]
    #[serde(rename = "approvalRequired", default, skip_serializing_if = "Option::is_none")]
    pub approval_required: Option<bool>,
    #[doc = "Whether the number of subscriptions a user can have to this product at the same time. Set to null or omit to allow unlimited per user subscriptions. Can be present only if subscriptionRequired property is present and has a value of false."]
    #[serde(rename = "subscriptionsLimit", default, skip_serializing_if = "Option::is_none")]
    pub subscriptions_limit: Option<i32>,
    #[doc = "whether product is published or not. Published products are discoverable by users of developer portal. Non published products are visible only to administrators. Default state of Product is notPublished."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<product_entity_base_parameters::State>,
}
impl ProductEntityBaseParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod product_entity_base_parameters {
    use super::*;
    #[doc = "whether product is published or not. Published products are discoverable by users of developer portal. Non published products are visible only to administrators. Default state of Product is notPublished."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "notPublished")]
        NotPublished,
        #[serde(rename = "published")]
        Published,
    }
}
#[doc = "Paged Product-group link list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductGroupLinkCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProductGroupLinkContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProductGroupLinkCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProductGroupLinkCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Product-group link details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductGroupLinkContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Product-group link entity properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductGroupLinkContractProperties>,
}
impl ProductGroupLinkContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Product-group link entity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductGroupLinkContractProperties {
    #[doc = "Full resource Id of a group."]
    #[serde(rename = "groupId")]
    pub group_id: String,
}
impl ProductGroupLinkContractProperties {
    pub fn new(group_id: String) -> Self {
        Self { group_id }
    }
}
#[doc = "Product profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductTagResourceContractProperties {
    #[serde(flatten)]
    pub product_entity_base_parameters: ProductEntityBaseParameters,
    #[doc = "Identifier of the product in the form of /products/{productId}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Product name."]
    pub name: String,
}
impl ProductTagResourceContractProperties {
    pub fn new(name: String) -> Self {
        Self {
            product_entity_base_parameters: ProductEntityBaseParameters::default(),
            id: None,
            name,
        }
    }
}
#[doc = "Product Update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductUpdateParameters {
    #[doc = "Parameters supplied to the Update Product operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductUpdateProperties>,
}
impl ProductUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Update Product operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductUpdateProperties {
    #[serde(flatten)]
    pub product_entity_base_parameters: ProductEntityBaseParameters,
    #[doc = "Product name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl ProductUpdateProperties {
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
#[doc = "Paged Quota Counter list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaCounterCollection {
    #[doc = "Quota counter values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<QuotaCounterContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl QuotaCounterCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota counter details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaCounterContract {
    #[doc = "The Key value of the Counter. Must not be empty."]
    #[serde(rename = "counterKey")]
    pub counter_key: String,
    #[doc = "Identifier of the Period for which the counter was collected. Must not be empty."]
    #[serde(rename = "periodKey")]
    pub period_key: String,
    #[doc = "The date of the start of Counter Period. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "periodStartTime", with = "azure_core::date::rfc3339")]
    pub period_start_time: time::OffsetDateTime,
    #[doc = "The date of the end of Counter Period. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "periodEndTime", with = "azure_core::date::rfc3339")]
    pub period_end_time: time::OffsetDateTime,
    #[doc = "Quota counter value details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<QuotaCounterValueContractProperties>,
}
impl QuotaCounterContract {
    pub fn new(
        counter_key: String,
        period_key: String,
        period_start_time: time::OffsetDateTime,
        period_end_time: time::OffsetDateTime,
    ) -> Self {
        Self {
            counter_key,
            period_key,
            period_start_time,
            period_end_time,
            value: None,
        }
    }
}
#[doc = "Quota counter value details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaCounterValueContract {
    #[doc = "Quota counter value details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<QuotaCounterValueContractProperties>,
}
impl QuotaCounterValueContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota counter value details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaCounterValueContractProperties {
    #[doc = "Number of times Counter was called."]
    #[serde(rename = "callsCount", default, skip_serializing_if = "Option::is_none")]
    pub calls_count: Option<i32>,
    #[doc = "Data Transferred in KiloBytes."]
    #[serde(rename = "kbTransferred", default, skip_serializing_if = "Option::is_none")]
    pub kb_transferred: Option<f64>,
}
impl QuotaCounterValueContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota counter value details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaCounterValueUpdateContract {
    #[doc = "Quota counter value details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaCounterValueContractProperties>,
}
impl QuotaCounterValueUpdateContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Recipient User list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecipientEmailCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RecipientEmailContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl RecipientEmailCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recipient Email details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecipientEmailContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Recipient Email Contract Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecipientEmailContractProperties>,
}
impl RecipientEmailContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recipient Email Contract Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecipientEmailContractProperties {
    #[doc = "User Email subscribed to notification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
impl RecipientEmailContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Recipient User list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecipientUserCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RecipientUserContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl RecipientUserCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recipient User details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecipientUserContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Recipient User Contract Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecipientUsersContractProperties>,
}
impl RecipientUserContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recipient User Contract Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecipientUsersContractProperties {
    #[doc = "API Management UserId subscribed to notification."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
impl RecipientUsersContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Notification Parameter contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecipientsContractProperties {
    #[doc = "List of Emails subscribed for the notification."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub emails: Vec<String>,
    #[doc = "List of Users subscribed for the notification."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub users: Vec<String>,
}
impl RecipientsContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Region profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegionContract {
    #[doc = "Region name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "whether Region is the master region."]
    #[serde(rename = "isMasterRegion", default, skip_serializing_if = "Option::is_none")]
    pub is_master_region: Option<bool>,
    #[doc = "whether Region is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
}
impl RegionContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lists Regions operation response details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegionListResult {
    #[doc = "Lists of Regions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RegionContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RegionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RegionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User registration delegation settings properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationDelegationSettingsProperties {
    #[doc = "Enable or disable delegation for user registration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl RegistrationDelegationSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Remote Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemotePrivateEndpointConnectionWrapper {
    #[doc = "Private Endpoint connection resource id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Private Endpoint Connection Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Private Endpoint Connection Resource Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of the PrivateEndpointConnectProperties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionWrapperProperties>,
}
impl RemotePrivateEndpointConnectionWrapper {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Report records list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ReportRecordContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReportCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ReportCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Report data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportRecordContract {
    #[doc = "Name depending on report endpoint specifies product, API, operation or developer name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Start of aggregation period. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Length of aggregation period.  Interval must be multiple of 15 minutes and may not be zero. The value should be in ISO 8601 format (http://en.wikipedia.org/wiki/ISO_8601#Durations)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[doc = "Country to which this record data is related."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Country region to which this record data is related."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "Zip code to which this record data is related."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[doc = "User identifier path. /users/{userId}"]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "Product identifier path. /products/{productId}"]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[doc = "API identifier path. /apis/{apiId}"]
    #[serde(rename = "apiId", default, skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[doc = "Operation identifier path. /apis/{apiId}/operations/{operationId}"]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "API region identifier."]
    #[serde(rename = "apiRegion", default, skip_serializing_if = "Option::is_none")]
    pub api_region: Option<String>,
    #[doc = "Subscription identifier path. /subscriptions/{subscriptionId}"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Number of successful calls. This includes calls returning HttpStatusCode <= 301 and HttpStatusCode.NotModified and HttpStatusCode.TemporaryRedirect"]
    #[serde(rename = "callCountSuccess", default, skip_serializing_if = "Option::is_none")]
    pub call_count_success: Option<i32>,
    #[doc = "Number of calls blocked due to invalid credentials. This includes calls returning HttpStatusCode.Unauthorized and HttpStatusCode.Forbidden and HttpStatusCode.TooManyRequests"]
    #[serde(rename = "callCountBlocked", default, skip_serializing_if = "Option::is_none")]
    pub call_count_blocked: Option<i32>,
    #[doc = "Number of calls failed due to gateway or backend errors. This includes calls returning HttpStatusCode.BadRequest(400) and any Code between HttpStatusCode.InternalServerError (500) and 600"]
    #[serde(rename = "callCountFailed", default, skip_serializing_if = "Option::is_none")]
    pub call_count_failed: Option<i32>,
    #[doc = "Number of other calls."]
    #[serde(rename = "callCountOther", default, skip_serializing_if = "Option::is_none")]
    pub call_count_other: Option<i32>,
    #[doc = "Total number of calls."]
    #[serde(rename = "callCountTotal", default, skip_serializing_if = "Option::is_none")]
    pub call_count_total: Option<i32>,
    #[doc = "Bandwidth consumed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i64>,
    #[doc = "Number of times when content was served from cache policy."]
    #[serde(rename = "cacheHitCount", default, skip_serializing_if = "Option::is_none")]
    pub cache_hit_count: Option<i32>,
    #[doc = "Number of times content was fetched from backend."]
    #[serde(rename = "cacheMissCount", default, skip_serializing_if = "Option::is_none")]
    pub cache_miss_count: Option<i32>,
    #[doc = "Average time it took to process request."]
    #[serde(rename = "apiTimeAvg", default, skip_serializing_if = "Option::is_none")]
    pub api_time_avg: Option<f64>,
    #[doc = "Minimum time it took to process request."]
    #[serde(rename = "apiTimeMin", default, skip_serializing_if = "Option::is_none")]
    pub api_time_min: Option<f64>,
    #[doc = "Maximum time it took to process request."]
    #[serde(rename = "apiTimeMax", default, skip_serializing_if = "Option::is_none")]
    pub api_time_max: Option<f64>,
    #[doc = "Average time it took to process request on backend."]
    #[serde(rename = "serviceTimeAvg", default, skip_serializing_if = "Option::is_none")]
    pub service_time_avg: Option<f64>,
    #[doc = "Minimum time it took to process request on backend."]
    #[serde(rename = "serviceTimeMin", default, skip_serializing_if = "Option::is_none")]
    pub service_time_min: Option<f64>,
    #[doc = "Maximum time it took to process request on backend."]
    #[serde(rename = "serviceTimeMax", default, skip_serializing_if = "Option::is_none")]
    pub service_time_max: Option<f64>,
}
impl ReportRecordContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation request/response representation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepresentationContract {
    #[doc = "Specifies a registered or custom content type for this representation, e.g. application/xml."]
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[doc = "Schema identifier. Applicable only if 'contentType' value is neither 'application/x-www-form-urlencoded' nor 'multipart/form-data'."]
    #[serde(rename = "schemaId", default, skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    #[doc = "Type name defined by the schema. Applicable only if 'contentType' value is neither 'application/x-www-form-urlencoded' nor 'multipart/form-data'."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "Collection of form parameters. Required if 'contentType' value is either 'application/x-www-form-urlencoded' or 'multipart/form-data'.."]
    #[serde(
        rename = "formParameters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub form_parameters: Vec<ParameterContract>,
    #[doc = "Parameter examples."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub examples: Option<ParameterExamplesContract>,
}
impl RepresentationContract {
    pub fn new(content_type: String) -> Self {
        Self {
            content_type,
            schema_id: None,
            type_name: None,
            form_parameters: Vec::new(),
            examples: None,
        }
    }
}
#[doc = "Operation request details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestContract {
    #[doc = "Operation request description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Collection of operation request query parameters."]
    #[serde(
        rename = "queryParameters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub query_parameters: Vec<ParameterContract>,
    #[doc = "Collection of operation request headers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub headers: Vec<ParameterContract>,
    #[doc = "Collection of operation request representations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub representations: Vec<RepresentationContract>,
}
impl RequestContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Report records list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestReportCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RequestReportRecordContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl azure_core::Continuable for RequestReportCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RequestReportCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request Report data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestReportRecordContract {
    #[doc = "API identifier path. /apis/{apiId}"]
    #[serde(rename = "apiId", default, skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[doc = "Operation identifier path. /apis/{apiId}/operations/{operationId}"]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "Product identifier path. /products/{productId}"]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[doc = "User identifier path. /users/{userId}"]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "The HTTP method associated with this request.."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "The full URL associated with this request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The client IP address associated with this request."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The HTTP status code received by the gateway as a result of forwarding this request to the backend."]
    #[serde(rename = "backendResponseCode", default, skip_serializing_if = "Option::is_none")]
    pub backend_response_code: Option<String>,
    #[doc = "The HTTP status code returned by the gateway."]
    #[serde(rename = "responseCode", default, skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i32>,
    #[doc = "The size of the response returned by the gateway."]
    #[serde(rename = "responseSize", default, skip_serializing_if = "Option::is_none")]
    pub response_size: Option<i32>,
    #[doc = "The date and time when this request was received by the gateway in ISO 8601 format."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Specifies if response cache was involved in generating the response. If the value is none, the cache was not used. If the value is hit, cached response was returned. If the value is miss, the cache was used but lookup resulted in a miss and request was fulfilled by the backend."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache: Option<String>,
    #[doc = "The total time it took to process this request."]
    #[serde(rename = "apiTime", default, skip_serializing_if = "Option::is_none")]
    pub api_time: Option<f64>,
    #[doc = "he time it took to forward this request to the backend and get the response back."]
    #[serde(rename = "serviceTime", default, skip_serializing_if = "Option::is_none")]
    pub service_time: Option<f64>,
    #[doc = "Azure region where the gateway that processed this request is located."]
    #[serde(rename = "apiRegion", default, skip_serializing_if = "Option::is_none")]
    pub api_region: Option<String>,
    #[doc = "Subscription identifier path. /subscriptions/{subscriptionId}"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Request Identifier."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The size of this request.."]
    #[serde(rename = "requestSize", default, skip_serializing_if = "Option::is_none")]
    pub request_size: Option<i32>,
}
impl RequestReportRecordContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Resolver list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResolverCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ResolverContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResolverCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ResolverCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GraphQL API Resolver details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResolverContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "GraphQL API Resolver Entity Base Contract details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResolverEntityBaseContract>,
}
impl ResolverContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GraphQL API Resolver Entity Base Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResolverEntityBaseContract {
    #[doc = "Resolver Name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Path is type/field being resolved."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Description of the resolver. May include HTML formatting tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ResolverEntityBaseContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Long Running Git Resolver Results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResolverResultContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resolver Result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResolverResultContractProperties>,
}
impl ResolverResultContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resolver Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResolverResultContractProperties {
    #[doc = "Resolver result identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of an async resolver."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<resolver_result_contract_properties::Status>,
    #[doc = "Start time of an async resolver. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub started: Option<time::OffsetDateTime>,
    #[doc = "Last update time of an async resolver. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub updated: Option<time::OffsetDateTime>,
    #[doc = "Optional result info."]
    #[serde(rename = "resultInfo", default, skip_serializing_if = "Option::is_none")]
    pub result_info: Option<String>,
    #[doc = "Error Body contract."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
    #[doc = "This property if only provided as part of the TenantConfiguration_Validate resolver. It contains the log the entities which will be updated/created/deleted as part of the TenantConfiguration_Deploy resolver."]
    #[serde(
        rename = "actionLog",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub action_log: Vec<ResolverResultLogItemContract>,
}
impl ResolverResultContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resolver_result_contract_properties {
    use super::*;
    #[doc = "Status of an async resolver."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Started,
        InProgress,
        Succeeded,
        Failed,
    }
}
#[doc = "Log of the entity being created, updated or deleted."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResolverResultLogItemContract {
    #[doc = "The type of entity contract."]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "Action like create/update/delete."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "Identifier of the entity being created/updated/deleted."]
    #[serde(rename = "objectKey", default, skip_serializing_if = "Option::is_none")]
    pub object_key: Option<String>,
}
impl ResolverResultLogItemContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GraphQL API Resolver Update Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResolverUpdateContract {
    #[doc = "Resolver Update Contract Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResolverUpdateContractProperties>,
}
impl ResolverUpdateContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resolver Update Contract Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResolverUpdateContractProperties {
    #[doc = "Resolver Name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Path is type/field being resolved."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Description of the resolver. May include HTML formatting tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ResolverUpdateContractProperties {
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
#[doc = "A collection of resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceCollection {
    #[doc = "A collection of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<serde_json::Value>,
    #[doc = "Total record count number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource location data properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceLocationDataContract {
    #[doc = "A canonical name for the geographic or physical location."]
    pub name: String,
    #[doc = "The city or locality where the resource is located."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "The district, state, or province where the resource is located."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[doc = "The country or region where the resource is located."]
    #[serde(rename = "countryOrRegion", default, skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
}
impl ResourceLocationDataContract {
    pub fn new(name: String) -> Self {
        Self {
            name,
            city: None,
            district: None,
            country_or_region: None,
        }
    }
}
#[doc = "Describes an available API Management SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSku {
    #[doc = "Name of the Sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<resource_sku::Name>,
}
impl ResourceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_sku {
    use super::*;
    #[doc = "Name of the Sku."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Developer,
        Standard,
        Premium,
        Basic,
        Consumption,
        Isolated,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Developer => serializer.serialize_unit_variant("Name", 0u32, "Developer"),
                Self::Standard => serializer.serialize_unit_variant("Name", 1u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("Name", 2u32, "Premium"),
                Self::Basic => serializer.serialize_unit_variant("Name", 3u32, "Basic"),
                Self::Consumption => serializer.serialize_unit_variant("Name", 4u32, "Consumption"),
                Self::Isolated => serializer.serialize_unit_variant("Name", 5u32, "Isolated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes scaling information of a SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuCapacity {
    #[doc = "The minimum capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,
    #[doc = "The maximum capacity that can be set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
    #[doc = "The default capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i32>,
    #[doc = "The scale type applicable to the sku."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<resource_sku_capacity::ScaleType>,
}
impl ResourceSkuCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_sku_capacity {
    use super::*;
    #[doc = "The scale type applicable to the sku."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScaleType")]
    pub enum ScaleType {
        #[serde(rename = "automatic")]
        Automatic,
        #[serde(rename = "manual")]
        Manual,
        #[serde(rename = "none")]
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScaleType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScaleType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScaleType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Automatic => serializer.serialize_unit_variant("ScaleType", 0u32, "automatic"),
                Self::Manual => serializer.serialize_unit_variant("ScaleType", 1u32, "manual"),
                Self::None => serializer.serialize_unit_variant("ScaleType", 2u32, "none"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes an available API Management service SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuResult {
    #[doc = "The type of resource the SKU applies to."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Describes an available API Management SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[doc = "Describes scaling information of a SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<ResourceSkuCapacity>,
}
impl ResourceSkuResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The API Management service SKUs operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuResults {
    #[doc = "The list of skus available for the service."]
    pub value: Vec<ResourceSkuResult>,
    #[doc = "The uri to fetch the next page of API Management service Skus."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceSkuResults {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ResourceSkuResults {
    pub fn new(value: Vec<ResourceSkuResult>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Operation response details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseContract {
    #[doc = "Operation response HTTP status code."]
    #[serde(rename = "statusCode")]
    pub status_code: i32,
    #[doc = "Operation response description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Collection of operation response representations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub representations: Vec<RepresentationContract>,
    #[doc = "Collection of operation response headers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub headers: Vec<ParameterContract>,
}
impl ResponseContract {
    pub fn new(status_code: i32) -> Self {
        Self {
            status_code,
            description: None,
            representations: Vec::new(),
            headers: Vec::new(),
        }
    }
}
#[doc = "Sampling settings for Diagnostic."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SamplingSettings {
    #[doc = "Sampling type."]
    #[serde(rename = "samplingType", default, skip_serializing_if = "Option::is_none")]
    pub sampling_type: Option<sampling_settings::SamplingType>,
    #[doc = "Rate of sampling for fixed-rate sampling."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}
impl SamplingSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sampling_settings {
    use super::*;
    #[doc = "Sampling type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SamplingType")]
    pub enum SamplingType {
        #[serde(rename = "fixed")]
        Fixed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SamplingType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SamplingType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SamplingType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Fixed => serializer.serialize_unit_variant("SamplingType", 0u32, "fixed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Save Tenant Configuration Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaveConfigurationParameter {
    #[doc = "Parameters supplied to the Save Tenant Configuration operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SaveConfigurationParameterProperties>,
}
impl SaveConfigurationParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Save Tenant Configuration operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SaveConfigurationParameterProperties {
    #[doc = "The name of the Git branch in which to commit the current configuration snapshot."]
    pub branch: String,
    #[doc = "The value if true, the current configuration database is committed to the Git repository, even if the Git repository has newer changes that would be overwritten."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}
impl SaveConfigurationParameterProperties {
    pub fn new(branch: String) -> Self {
        Self { branch, force: None }
    }
}
#[doc = "The response of the list schema operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaCollection {
    #[doc = "API Schema Contract value."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SchemaContract>,
    #[doc = "Total record count number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SchemaCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SchemaCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API Schema Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "API Schema create or update contract Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SchemaContractProperties>,
}
impl SchemaContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API Schema create or update contract Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaContractProperties {
    #[doc = "Must be a valid a media type used in a Content-Type header as defined in the RFC 2616. Media type of the schema document (e.g. application/json, application/xml). </br> - `Swagger` Schema use `application/vnd.ms-azure-apim.swagger.definitions+json` </br> - `WSDL` Schema use `application/vnd.ms-azure-apim.xsd+xml` </br> - `OpenApi` Schema use `application/vnd.oai.openapi.components+json` </br> - `WADL Schema` use `application/vnd.ms-azure-apim.wadl.grammars+xml`."]
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[doc = "Api Schema Document Properties."]
    pub document: SchemaDocumentProperties,
}
impl SchemaContractProperties {
    pub fn new(content_type: String, document: SchemaDocumentProperties) -> Self {
        Self { content_type, document }
    }
}
#[doc = "Api Schema Document Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaDocumentProperties {
    #[doc = "Json escaped string defining the document representing the Schema. Used for schemas other than Swagger/OpenAPI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Types definitions. Used for Swagger/OpenAPI v1 schemas only, null otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definitions: Option<serde_json::Value>,
    #[doc = "Types definitions. Used for Swagger/OpenAPI v2/v3 schemas only, null otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<serde_json::Value>,
}
impl SchemaDocumentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Subscriptions list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SubscriptionContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SubscriptionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SubscriptionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Subscription details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionContractProperties>,
}
impl SubscriptionContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionContractProperties {
    #[doc = "The user resource identifier of the subscription owner. The value is a valid relative URL in the format of /users/{userId} where {userId} is a user identifier."]
    #[serde(rename = "ownerId", default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[doc = "Scope like /products/{productId} or /apis or /apis/{apiId}."]
    pub scope: String,
    #[doc = "The name of the subscription, or null if the subscription has no name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Subscription state. Possible states are * active – the subscription is active, * suspended – the subscription is blocked, and the subscriber cannot call any APIs of the product, * submitted – the subscription request has been made by the developer, but has not yet been approved or rejected, * rejected – the subscription request has been denied by an administrator, * cancelled – the subscription has been cancelled by the developer or administrator, * expired – the subscription reached its expiration date and was deactivated."]
    pub state: subscription_contract_properties::State,
    #[doc = "Subscription creation date. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "createdDate", default, with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Subscription activation date. The setting is for audit purposes only and the subscription is not automatically activated. The subscription lifecycle can be managed by using the `state` property. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "startDate", default, with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "Subscription expiration date. The setting is for audit purposes only and the subscription is not automatically expired. The subscription lifecycle can be managed by using the `state` property. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Date when subscription was cancelled or expired. The setting is for audit purposes only and the subscription is not automatically cancelled. The subscription lifecycle can be managed by using the `state` property. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "endDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<time::OffsetDateTime>,
    #[doc = "Upcoming subscription expiration notification date. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "notificationDate", default, with = "azure_core::date::rfc3339::option")]
    pub notification_date: Option<time::OffsetDateTime>,
    #[doc = "Subscription primary key. This property will not be filled on 'GET' operations! Use '/listSecrets' POST request to get the value."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "Subscription secondary key. This property will not be filled on 'GET' operations! Use '/listSecrets' POST request to get the value."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "Optional subscription comment added by an administrator when the state is changed to the 'rejected'."]
    #[serde(rename = "stateComment", default, skip_serializing_if = "Option::is_none")]
    pub state_comment: Option<String>,
    #[doc = "Determines whether tracing is enabled"]
    #[serde(rename = "allowTracing", default, skip_serializing_if = "Option::is_none")]
    pub allow_tracing: Option<bool>,
}
impl SubscriptionContractProperties {
    pub fn new(scope: String, state: subscription_contract_properties::State) -> Self {
        Self {
            owner_id: None,
            scope,
            display_name: None,
            state,
            created_date: None,
            start_date: None,
            expiration_date: None,
            end_date: None,
            notification_date: None,
            primary_key: None,
            secondary_key: None,
            state_comment: None,
            allow_tracing: None,
        }
    }
}
pub mod subscription_contract_properties {
    use super::*;
    #[doc = "Subscription state. Possible states are * active – the subscription is active, * suspended – the subscription is blocked, and the subscriber cannot call any APIs of the product, * submitted – the subscription request has been made by the developer, but has not yet been approved or rejected, * rejected – the subscription request has been denied by an administrator, * cancelled – the subscription has been cancelled by the developer or administrator, * expired – the subscription reached its expiration date and was deactivated."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "suspended")]
        Suspended,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "expired")]
        Expired,
        #[serde(rename = "submitted")]
        Submitted,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "cancelled")]
        Cancelled,
    }
}
#[doc = "Parameters supplied to the Create subscription operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionCreateParameterProperties {
    #[doc = "User (user id path) for whom subscription is being created in form /users/{userId}"]
    #[serde(rename = "ownerId", default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[doc = "Scope like /products/{productId} or /apis or /apis/{apiId}."]
    pub scope: String,
    #[doc = "Subscription name."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Primary subscription key. If not specified during request key will be generated automatically."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "Secondary subscription key. If not specified during request key will be generated automatically."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "Initial subscription state. If no value is specified, subscription is created with Submitted state. Possible states are * active – the subscription is active, * suspended – the subscription is blocked, and the subscriber cannot call any APIs of the product, * submitted – the subscription request has been made by the developer, but has not yet been approved or rejected, * rejected – the subscription request has been denied by an administrator, * cancelled – the subscription has been cancelled by the developer or administrator, * expired – the subscription reached its expiration date and was deactivated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<subscription_create_parameter_properties::State>,
    #[doc = "Determines whether tracing can be enabled"]
    #[serde(rename = "allowTracing", default, skip_serializing_if = "Option::is_none")]
    pub allow_tracing: Option<bool>,
}
impl SubscriptionCreateParameterProperties {
    pub fn new(scope: String, display_name: String) -> Self {
        Self {
            owner_id: None,
            scope,
            display_name,
            primary_key: None,
            secondary_key: None,
            state: None,
            allow_tracing: None,
        }
    }
}
pub mod subscription_create_parameter_properties {
    use super::*;
    #[doc = "Initial subscription state. If no value is specified, subscription is created with Submitted state. Possible states are * active – the subscription is active, * suspended – the subscription is blocked, and the subscriber cannot call any APIs of the product, * submitted – the subscription request has been made by the developer, but has not yet been approved or rejected, * rejected – the subscription request has been denied by an administrator, * cancelled – the subscription has been cancelled by the developer or administrator, * expired – the subscription reached its expiration date and was deactivated."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "suspended")]
        Suspended,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "expired")]
        Expired,
        #[serde(rename = "submitted")]
        Submitted,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "cancelled")]
        Cancelled,
    }
}
#[doc = "Subscription create details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionCreateParameters {
    #[doc = "Parameters supplied to the Create subscription operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionCreateParameterProperties>,
}
impl SubscriptionCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription key parameter names details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionKeyParameterNamesContract {
    #[doc = "Subscription key header name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[doc = "Subscription key query string parameter name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}
impl SubscriptionKeyParameterNamesContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionKeysContract {
    #[doc = "Subscription primary key."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "Subscription secondary key."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}
impl SubscriptionKeysContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Update subscription operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionUpdateParameterProperties {
    #[doc = "User identifier path: /users/{userId}"]
    #[serde(rename = "ownerId", default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[doc = "Scope like /products/{productId} or /apis or /apis/{apiId}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Subscription expiration date. The setting is for audit purposes only and the subscription is not automatically expired. The subscription lifecycle can be managed by using the `state` property. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard."]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Subscription name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Primary subscription key."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "Secondary subscription key."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "Subscription state. Possible states are * active – the subscription is active, * suspended – the subscription is blocked, and the subscriber cannot call any APIs of the product, * submitted – the subscription request has been made by the developer, but has not yet been approved or rejected, * rejected – the subscription request has been denied by an administrator, * cancelled – the subscription has been cancelled by the developer or administrator, * expired – the subscription reached its expiration date and was deactivated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<subscription_update_parameter_properties::State>,
    #[doc = "Comments describing subscription state change by the administrator when the state is changed to the 'rejected'."]
    #[serde(rename = "stateComment", default, skip_serializing_if = "Option::is_none")]
    pub state_comment: Option<String>,
    #[doc = "Determines whether tracing can be enabled"]
    #[serde(rename = "allowTracing", default, skip_serializing_if = "Option::is_none")]
    pub allow_tracing: Option<bool>,
}
impl SubscriptionUpdateParameterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_update_parameter_properties {
    use super::*;
    #[doc = "Subscription state. Possible states are * active – the subscription is active, * suspended – the subscription is blocked, and the subscriber cannot call any APIs of the product, * submitted – the subscription request has been made by the developer, but has not yet been approved or rejected, * rejected – the subscription request has been denied by an administrator, * cancelled – the subscription has been cancelled by the developer or administrator, * expired – the subscription reached its expiration date and was deactivated."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "suspended")]
        Suspended,
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "expired")]
        Expired,
        #[serde(rename = "submitted")]
        Submitted,
        #[serde(rename = "rejected")]
        Rejected,
        #[serde(rename = "cancelled")]
        Cancelled,
    }
}
#[doc = "Subscription update details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionUpdateParameters {
    #[doc = "Parameters supplied to the Update subscription operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionUpdateParameterProperties>,
}
impl SubscriptionUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscriptions delegation settings properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionsDelegationSettingsProperties {
    #[doc = "Enable or disable delegation for subscriptions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl SubscriptionsDelegationSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Tag-API link list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagApiLinkCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TagApiLinkContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TagApiLinkCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TagApiLinkCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag-API link details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagApiLinkContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Tag-API link entity properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TagApiLinkContractProperties>,
}
impl TagApiLinkContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag-API link entity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagApiLinkContractProperties {
    #[doc = "Full resource Id of an API."]
    #[serde(rename = "apiId")]
    pub api_id: String,
}
impl TagApiLinkContractProperties {
    pub fn new(api_id: String) -> Self {
        Self { api_id }
    }
}
#[doc = "Paged Tag list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TagContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TagCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TagCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Tag contract Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TagContractProperties>,
}
impl TagContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag contract Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagContractProperties {
    #[doc = "Tag name."]
    #[serde(rename = "displayName")]
    pub display_name: String,
}
impl TagContractProperties {
    pub fn new(display_name: String) -> Self {
        Self { display_name }
    }
}
#[doc = "Parameters supplied to Create/Update Tag operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagCreateUpdateParameters {
    #[doc = "Tag contract Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TagContractProperties>,
}
impl TagCreateUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Create TagDescription operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagDescriptionBaseProperties {
    #[doc = "Description of the Tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Absolute URL of external resources describing the tag."]
    #[serde(rename = "externalDocsUrl", default, skip_serializing_if = "Option::is_none")]
    pub external_docs_url: Option<String>,
    #[doc = "Description of the external resources describing the tag."]
    #[serde(rename = "externalDocsDescription", default, skip_serializing_if = "Option::is_none")]
    pub external_docs_description: Option<String>,
}
impl TagDescriptionBaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged TagDescription list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagDescriptionCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TagDescriptionContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TagDescriptionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TagDescriptionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagDescriptionContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "TagDescription contract Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TagDescriptionContractProperties>,
}
impl TagDescriptionContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "TagDescription contract Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagDescriptionContractProperties {
    #[serde(flatten)]
    pub tag_description_base_properties: TagDescriptionBaseProperties,
    #[doc = "Identifier of the tag in the form of /tags/{tagId}"]
    #[serde(rename = "tagId", default, skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<String>,
    #[doc = "Tag name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl TagDescriptionContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Create TagDescription operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagDescriptionCreateParameters {
    #[doc = "Parameters supplied to the Create TagDescription operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TagDescriptionBaseProperties>,
}
impl TagDescriptionCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Tag-operation link list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagOperationLinkCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TagOperationLinkContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TagOperationLinkCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TagOperationLinkCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag-operation link details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagOperationLinkContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Tag-operation link entity properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TagOperationLinkContractProperties>,
}
impl TagOperationLinkContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag-operation link entity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagOperationLinkContractProperties {
    #[doc = "Full resource Id of an API operation."]
    #[serde(rename = "operationId")]
    pub operation_id: String,
}
impl TagOperationLinkContractProperties {
    pub fn new(operation_id: String) -> Self {
        Self { operation_id }
    }
}
#[doc = "Paged Tag-product link list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagProductLinkCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TagProductLinkContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TagProductLinkCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TagProductLinkCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag-product link details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagProductLinkContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Tag-product link entity properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TagProductLinkContractProperties>,
}
impl TagProductLinkContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag-product link entity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagProductLinkContractProperties {
    #[doc = "Full resource Id of a product."]
    #[serde(rename = "productId")]
    pub product_id: String,
}
impl TagProductLinkContractProperties {
    pub fn new(product_id: String) -> Self {
        Self { product_id }
    }
}
#[doc = "Paged Tag list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagResourceCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TagResourceContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TagResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TagResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "TagResource contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagResourceContract {
    #[doc = "Contract defining the Tag property in the Tag Resource Contract"]
    pub tag: TagTagResourceContractProperties,
    #[doc = "API contract properties for the Tag Resources."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<ApiTagResourceContractProperties>,
    #[doc = "Operation Entity contract Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<OperationTagResourceContractProperties>,
    #[doc = "Product profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<ProductTagResourceContractProperties>,
}
impl TagResourceContract {
    pub fn new(tag: TagTagResourceContractProperties) -> Self {
        Self {
            tag,
            api: None,
            operation: None,
            product: None,
        }
    }
}
#[doc = "Contract defining the Tag property in the Tag Resource Contract"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagTagResourceContractProperties {
    #[doc = "Tag identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Tag Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl TagTagResourceContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of Tenant Configuration Sync State."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantConfigurationSyncStateContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Tenant Configuration Synchronization State."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TenantConfigurationSyncStateContractProperties>,
}
impl TenantConfigurationSyncStateContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant Configuration Synchronization State."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantConfigurationSyncStateContractProperties {
    #[doc = "The name of Git branch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[doc = "The latest commit Id."]
    #[serde(rename = "commitId", default, skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[doc = "value indicating if last sync was save (true) or deploy (false) operation."]
    #[serde(rename = "isExport", default, skip_serializing_if = "Option::is_none")]
    pub is_export: Option<bool>,
    #[doc = "value indicating if last synchronization was later than the configuration change."]
    #[serde(rename = "isSynced", default, skip_serializing_if = "Option::is_none")]
    pub is_synced: Option<bool>,
    #[doc = "value indicating whether Git configuration access is enabled."]
    #[serde(rename = "isGitEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_git_enabled: Option<bool>,
    #[doc = "The date of the latest synchronization. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "syncDate", default, with = "azure_core::date::rfc3339::option")]
    pub sync_date: Option<time::OffsetDateTime>,
    #[doc = "The date of the latest configuration change. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "configurationChangeDate", default, with = "azure_core::date::rfc3339::option")]
    pub configuration_change_date: Option<time::OffsetDateTime>,
    #[doc = "Most recent tenant configuration operation identifier"]
    #[serde(rename = "lastOperationId", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_id: Option<String>,
}
impl TenantConfigurationSyncStateContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged AccessInformation list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantSettingsCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TenantSettingsContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TenantSettingsCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TenantSettingsCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant Settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantSettingsContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Tenant access information contract of the API Management service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TenantSettingsContractProperties>,
}
impl TenantSettingsContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant access information contract of the API Management service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantSettingsContractProperties {
    #[doc = "Tenant settings"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}
impl TenantSettingsContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Terms of service contract properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TermsOfServiceProperties {
    #[doc = "A terms of service text."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "Display terms of service during a sign-up process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Ask user for consent to the terms of service."]
    #[serde(rename = "consentRequired", default, skip_serializing_if = "Option::is_none")]
    pub consent_required: Option<bool>,
}
impl TermsOfServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OAuth acquire token request body parameter (www-url-form-encoded)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenBodyParameterContract {
    #[doc = "body parameter name."]
    pub name: String,
    #[doc = "body parameter value."]
    pub value: String,
}
impl TokenBodyParameterContract {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "Paged Users list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<UserContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UserCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl UserCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "User profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserContractProperties>,
}
impl UserContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserContractProperties {
    #[serde(flatten)]
    pub user_entity_base_parameters: UserEntityBaseParameters,
    #[doc = "First name."]
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Last name."]
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Date of user registration. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "registrationDate", default, with = "azure_core::date::rfc3339::option")]
    pub registration_date: Option<time::OffsetDateTime>,
    #[doc = "Collection of groups user is part of."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub groups: Vec<GroupContractProperties>,
}
impl UserContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Create User operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCreateParameterProperties {
    #[serde(flatten)]
    pub user_entity_base_parameters: UserEntityBaseParameters,
    #[doc = "Email address. Must not be empty and must be unique within the service instance."]
    pub email: String,
    #[doc = "First name."]
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[doc = "Last name."]
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[doc = "User Password. If no value is provided, a default password is generated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Determines the type of application which send the create user request. Default is legacy portal."]
    #[serde(rename = "appType", default, skip_serializing_if = "Option::is_none")]
    pub app_type: Option<user_create_parameter_properties::AppType>,
    #[doc = "Determines the type of confirmation e-mail that will be sent to the newly created user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirmation: Option<user_create_parameter_properties::Confirmation>,
}
impl UserCreateParameterProperties {
    pub fn new(email: String, first_name: String, last_name: String) -> Self {
        Self {
            user_entity_base_parameters: UserEntityBaseParameters::default(),
            email,
            first_name,
            last_name,
            password: None,
            app_type: None,
            confirmation: None,
        }
    }
}
pub mod user_create_parameter_properties {
    use super::*;
    #[doc = "Determines the type of application which send the create user request. Default is legacy portal."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AppType")]
    pub enum AppType {
        #[serde(rename = "portal")]
        Portal,
        #[serde(rename = "developerPortal")]
        DeveloperPortal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AppType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AppType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AppType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Portal => serializer.serialize_unit_variant("AppType", 0u32, "portal"),
                Self::DeveloperPortal => serializer.serialize_unit_variant("AppType", 1u32, "developerPortal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Determines the type of confirmation e-mail that will be sent to the newly created user."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Confirmation")]
    pub enum Confirmation {
        #[serde(rename = "signup")]
        Signup,
        #[serde(rename = "invite")]
        Invite,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Confirmation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Confirmation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Confirmation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Signup => serializer.serialize_unit_variant("Confirmation", 0u32, "signup"),
                Self::Invite => serializer.serialize_unit_variant("Confirmation", 1u32, "invite"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "User create details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserCreateParameters {
    #[doc = "Parameters supplied to the Create User operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserCreateParameterProperties>,
}
impl UserCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User Entity Base Parameters set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserEntityBaseParameters {
    #[doc = "Account state. Specifies whether the user is active or not. Blocked users are unable to sign into the developer portal or call any APIs of subscribed products. Default state is Active."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<user_entity_base_parameters::State>,
    #[doc = "Optional note about a user set by the administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[doc = "Collection of user identities."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub identities: Vec<UserIdentityContract>,
}
impl UserEntityBaseParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod user_entity_base_parameters {
    use super::*;
    #[doc = "Account state. Specifies whether the user is active or not. Blocked users are unable to sign into the developer portal or call any APIs of subscribed products. Default state is Active."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "blocked")]
        Blocked,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "deleted")]
        Deleted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("State", 0u32, "active"),
                Self::Blocked => serializer.serialize_unit_variant("State", 1u32, "blocked"),
                Self::Pending => serializer.serialize_unit_variant("State", 2u32, "pending"),
                Self::Deleted => serializer.serialize_unit_variant("State", 3u32, "deleted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for State {
        fn default() -> Self {
            Self::Active
        }
    }
}
#[doc = "List of Users Identity list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserIdentityCollection {
    #[doc = "User Identity values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<UserIdentityContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UserIdentityCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl UserIdentityCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User identity details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserIdentityContract {
    #[doc = "Identity provider name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Identifier value within provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl UserIdentityContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserIdentityProperties {
    #[doc = "The principal id of user assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client id of user assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserIdentityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Get User Token operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTokenParameterProperties {
    #[doc = "The Key to be used to generate token for user."]
    #[serde(rename = "keyType")]
    pub key_type: user_token_parameter_properties::KeyType,
    #[doc = "The Expiry time of the Token. Maximum token expiry time is set to 30 days. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(with = "azure_core::date::rfc3339")]
    pub expiry: time::OffsetDateTime,
}
impl UserTokenParameterProperties {
    pub fn new(key_type: user_token_parameter_properties::KeyType, expiry: time::OffsetDateTime) -> Self {
        Self { key_type, expiry }
    }
}
pub mod user_token_parameter_properties {
    use super::*;
    #[doc = "The Key to be used to generate token for user."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        #[serde(rename = "primary")]
        Primary,
        #[serde(rename = "secondary")]
        Secondary,
    }
    impl Default for KeyType {
        fn default() -> Self {
            Self::Primary
        }
    }
}
#[doc = "Get User Token parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserTokenParameters {
    #[doc = "Parameters supplied to the Get User Token operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserTokenParameterProperties>,
}
impl UserTokenParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Get User Token response details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserTokenResult {
    #[doc = "Shared Access Authorization token for the User."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl UserTokenResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserUpdateParameters {
    #[doc = "Parameters supplied to the Update User operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserUpdateParametersProperties>,
}
impl UserUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Update User operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserUpdateParametersProperties {
    #[serde(flatten)]
    pub user_entity_base_parameters: UserEntityBaseParameters,
    #[doc = "Email address. Must not be empty and must be unique within the service instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "User Password."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "First name."]
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Last name."]
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}
impl UserUpdateParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration of a virtual network to which API Management service is deployed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkConfiguration {
    #[doc = "The virtual network ID. This is typically a GUID. Expect a null GUID by default."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vnetid: Option<String>,
    #[doc = "The name of the subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnetname: Option<String>,
    #[doc = "The full resource ID of a subnet in a virtual network to deploy the API Management service in."]
    #[serde(rename = "subnetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_resource_id: Option<String>,
}
impl VirtualNetworkConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Wiki list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WikiContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WikiCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WikiCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Wiki properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Wiki contract details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WikiContractProperties>,
}
impl WikiContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Wiki contract details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiContractProperties {
    #[doc = "Collection wiki documents included into this wiki."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub documents: Vec<WikiDocumentationContract>,
}
impl WikiContractProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Wiki documentation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiDocumentationContract {
    #[doc = "Documentation Identifier"]
    #[serde(rename = "documentationId", default, skip_serializing_if = "Option::is_none")]
    pub documentation_id: Option<String>,
}
impl WikiDocumentationContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Wiki update contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WikiUpdateContract {
    #[doc = "Wiki contract details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WikiContractProperties>,
}
impl WikiUpdateContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged workspace list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceCollection {
    #[doc = "Page values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<WorkspaceContract>,
    #[doc = "Total record count number across all pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkspaceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkspaceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workspace details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceContract {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Workspace entity properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceContractProperties>,
}
impl WorkspaceContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workspace entity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceContractProperties {
    #[doc = "Name of the workspace."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Description of the workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl WorkspaceContractProperties {
    pub fn new(display_name: String) -> Self {
        Self {
            display_name,
            description: None,
        }
    }
}
#[doc = "Properties of server X509Names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct X509CertificateName {
    #[doc = "Common Name of the Certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Thumbprint for the Issuer of the Certificate."]
    #[serde(rename = "issuerCertificateThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub issuer_certificate_thumbprint: Option<String>,
}
impl X509CertificateName {
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
