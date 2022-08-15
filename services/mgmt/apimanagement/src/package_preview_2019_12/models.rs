#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Tenant access information contract of the API Management service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessInformationContract {
    #[doc = "Identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
impl AccessInformationContract {
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
    #[doc = "Public Static Load Balanced IP addresses of the API Management service in the additional location. Available only for Basic, Standard and Premium SKU."]
    #[serde(rename = "publicIPAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub public_ip_addresses: Vec<String>,
    #[doc = "Private Static Load Balanced IP addresses of the API Management service which is deployed in an Internal Virtual Network in a particular additional location. Available only for Basic, Standard and Premium SKU."]
    #[serde(rename = "privateIPAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub private_ip_addresses: Vec<String>,
    #[doc = "Configuration of a virtual network to which API Management service is deployed."]
    #[serde(rename = "virtualNetworkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_configuration: Option<VirtualNetworkConfiguration>,
    #[doc = "Gateway URL of the API Management service in the Region."]
    #[serde(rename = "gatewayRegionalUrl", default, skip_serializing_if = "Option::is_none")]
    pub gateway_regional_url: Option<String>,
    #[doc = "Property only valid for an Api Management service deployed in multiple locations. This can be used to disable the gateway in this additional location."]
    #[serde(rename = "disableGateway", default, skip_serializing_if = "Option::is_none")]
    pub disable_gateway: Option<bool>,
}
impl AdditionalLocation {
    pub fn new(location: String, sku: ApiManagementServiceSkuProperties) -> Self {
        Self {
            location,
            sku,
            public_ip_addresses: Vec::new(),
            private_ip_addresses: Vec::new(),
            virtual_network_configuration: None,
            gateway_regional_url: None,
            disable_gateway: None,
        }
    }
}
#[doc = "Paged Api list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiCollection {
    #[doc = "Page values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApiContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApiCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Api details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiContract {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Api Entity Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiContractProperties>,
}
impl ApiContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Api Entity Properties"]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[doc = "Api Create or Update Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiCreateOrUpdateProperties>,
}
impl ApiCreateOrUpdateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Api Create or Update Properties."]
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
    #[doc = "Type of Api to create. \n * `http` creates a SOAP to REST API \n * `soap` creates a SOAP pass-through API ."]
    #[serde(rename = "apiType", default, skip_serializing_if = "Option::is_none")]
    pub api_type: Option<api_create_or_update_properties::ApiType>,
}
impl ApiCreateOrUpdateProperties {
    pub fn new(api_contract_properties: ApiContractProperties) -> Self {
        Self {
            api_contract_properties,
            value: None,
            format: None,
            wsdl_selector: None,
            api_type: None,
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
    #[doc = "Type of Api to create. \n * `http` creates a SOAP to REST API \n * `soap` creates a SOAP pass-through API ."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ApiType")]
    pub enum ApiType {
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "soap")]
        Soap,
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
    #[doc = "Describes the Revision of the Api. If no value is provided, default revision 1 is created"]
    #[serde(rename = "apiRevision", default, skip_serializing_if = "Option::is_none")]
    pub api_revision: Option<String>,
    #[doc = "Indicates the Version identifier of the API if the API is versioned"]
    #[serde(rename = "apiVersion", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[doc = "Indicates if API revision is current api revision."]
    #[serde(rename = "isCurrent", default, skip_serializing_if = "Option::is_none")]
    pub is_current: Option<bool>,
    #[doc = "Indicates if API revision is accessible via the gateway."]
    #[serde(rename = "isOnline", default, skip_serializing_if = "Option::is_none")]
    pub is_online: Option<bool>,
    #[doc = "Description of the Api Revision."]
    #[serde(rename = "apiRevisionDescription", default, skip_serializing_if = "Option::is_none")]
    pub api_revision_description: Option<String>,
    #[doc = "Description of the Api Version."]
    #[serde(rename = "apiVersionDescription", default, skip_serializing_if = "Option::is_none")]
    pub api_version_description: Option<String>,
    #[doc = "A resource identifier for the related ApiVersionSet."]
    #[serde(rename = "apiVersionSetId", default, skip_serializing_if = "Option::is_none")]
    pub api_version_set_id: Option<String>,
    #[doc = "Specifies whether an API or Product subscription is required for accessing the API."]
    #[serde(rename = "subscriptionRequired", default, skip_serializing_if = "Option::is_none")]
    pub subscription_required: Option<bool>,
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
    #[doc = "Format in which the Api Details are exported to the Storage Blob with Sas Key valid for 5 minutes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<api_export_result::Format>,
    #[doc = "The object defining the schema of the exported Api Detail"]
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
    #[doc = "Format in which the Api Details are exported to the Storage Blob with Sas Key valid for 5 minutes."]
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
    #[doc = "The object defining the schema of the exported Api Detail"]
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
    #[doc = "Azure Cloud Storage account (used to place/retrieve the backup) name."]
    #[serde(rename = "storageAccount")]
    pub storage_account: String,
    #[doc = "Azure Cloud Storage account (used to place/retrieve the backup) access key."]
    #[serde(rename = "accessKey")]
    pub access_key: String,
    #[doc = "Azure Cloud Storage blob container name used to place/retrieve the backup."]
    #[serde(rename = "containerName")]
    pub container_name: String,
    #[doc = "The name of the backup file to create."]
    #[serde(rename = "backupName")]
    pub backup_name: String,
}
impl ApiManagementServiceBackupRestoreParameters {
    pub fn new(storage_account: String, access_key: String, container_name: String, backup_name: String) -> Self {
        Self {
            storage_account,
            access_key,
            container_name,
            backup_name,
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
    #[serde(rename = "createdAtUtc", with = "azure_core::date::rfc3339::option")]
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
    #[serde(rename = "hostnameConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub hostname_configurations: Vec<HostnameConfiguration>,
    #[doc = "Public Static Load Balanced IP addresses of the API Management service in Primary region. Available only for Basic, Standard and Premium SKU."]
    #[serde(rename = "publicIPAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub public_ip_addresses: Vec<String>,
    #[doc = "Private Static Load Balanced IP addresses of the API Management service in Primary region which is deployed in an Internal Virtual Network. Available only for Basic, Standard and Premium SKU."]
    #[serde(rename = "privateIPAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub private_ip_addresses: Vec<String>,
    #[doc = "Configuration of a virtual network to which API Management service is deployed."]
    #[serde(rename = "virtualNetworkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_configuration: Option<VirtualNetworkConfiguration>,
    #[doc = "Additional datacenter locations of the API Management service."]
    #[serde(rename = "additionalLocations", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_locations: Vec<AdditionalLocation>,
    #[doc = "Custom properties of the API Management service.</br>Setting `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.TripleDes168` will disable the cipher TLS_RSA_WITH_3DES_EDE_CBC_SHA for all TLS(1.0, 1.1 and 1.2).</br>Setting `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Protocols.Tls11` can be used to disable just TLS 1.1.</br>Setting `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Protocols.Tls10` can be used to disable TLS 1.0 on an API Management service.</br>Setting `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Backend.Protocols.Tls11` can be used to disable just TLS 1.1 for communications with backends.</br>Setting `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Backend.Protocols.Tls10` can be used to disable TLS 1.0 for communications with backends.</br>Setting `Microsoft.WindowsAzure.ApiManagement.Gateway.Protocols.Server.Http2` can be used to enable HTTP2 protocol on an API Management service.</br>Not specifying any of these properties on PATCH operation will reset omitted properties' values to their defaults. For all the settings except Http2 the default value is `True` if the service was created on or before April 1st 2018 and `False` otherwise. Http2 setting's default value is `False`.</br></br>You can disable any of next ciphers by using settings `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.[cipher_name]`: TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA, TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA, TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA, TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA, TLS_RSA_WITH_AES_128_GCM_SHA256, TLS_RSA_WITH_AES_256_CBC_SHA256, TLS_RSA_WITH_AES_128_CBC_SHA256, TLS_RSA_WITH_AES_256_CBC_SHA, TLS_RSA_WITH_AES_128_CBC_SHA. For example, `Microsoft.WindowsAzure.ApiManagement.Gateway.Security.Ciphers.TLS_RSA_WITH_AES_128_CBC_SHA256`:`false`. The default value is `true` for them.  Note: next ciphers can't be disabled since they are required by Azure CloudService internal components: TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256,TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384,TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256,TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384,TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256,TLS_RSA_WITH_AES_256_GCM_SHA384"]
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<serde_json::Value>,
    #[doc = "List of Certificates that need to be installed in the API Management service. Max supported certificates that can be installed is 10."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certificates: Vec<CertificateConfiguration>,
    #[doc = "Property only meant to be used for Consumption SKU Service. This enforces a client certificate to be presented on each request to the gateway. This also enables the ability to authenticate the certificate in the policy on the gateway."]
    #[serde(rename = "enableClientCertificate", default, skip_serializing_if = "Option::is_none")]
    pub enable_client_certificate: Option<bool>,
    #[doc = "Property only valid for an Api Management service deployed in multiple locations. This can be used to disable the gateway in master region."]
    #[serde(rename = "disableGateway", default, skip_serializing_if = "Option::is_none")]
    pub disable_gateway: Option<bool>,
    #[doc = "The type of VPN in which API Management service needs to be configured in. None (Default Value) means the API Management service is not part of any Virtual Network, External means the API Management deployment is set up inside a Virtual Network having an Internet Facing Endpoint, and Internal means that API Management deployment is setup inside a Virtual Network having an Intranet Facing Endpoint only."]
    #[serde(rename = "virtualNetworkType", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_type: Option<api_management_service_base_properties::VirtualNetworkType>,
    #[doc = "Control Plane Apis version constraint for the API Management service."]
    #[serde(rename = "apiVersionConstraint", default, skip_serializing_if = "Option::is_none")]
    pub api_version_constraint: Option<ApiVersionConstraint>,
}
impl ApiManagementServiceBaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_management_service_base_properties {
    use super::*;
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
        self.next_link.clone()
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
    #[doc = "Resource location."]
    pub location: String,
    #[doc = "ETag of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ApiManagementServiceResource {
    pub fn new(properties: ApiManagementServiceProperties, sku: ApiManagementServiceSkuProperties, location: String) -> Self {
        Self {
            apim_resource: ApimResource::default(),
            properties,
            sku,
            identity: None,
            location,
            etag: None,
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
#[doc = "Paged ApiRelease list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiReleaseCollection {
    #[doc = "Page values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApiReleaseContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiReleaseCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
    #[serde(rename = "createdDateTime", with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "The time the API release was updated."]
    #[serde(rename = "updatedDateTime", with = "azure_core::date::rfc3339::option")]
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
#[doc = "Paged Api Revision list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiRevisionCollection {
    #[doc = "Page values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApiRevisionContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiRevisionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(rename = "createdDateTime", with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "The time the API Revision were updated. The date conforms to the following format: yyyy-MM-ddTHH:mm:ssZ as specified by the ISO 8601 standard."]
    #[serde(rename = "updatedDateTime", with = "azure_core::date::rfc3339::option")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Paged Api Version Set list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiVersionSetCollection {
    #[doc = "Page values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApiVersionSetContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiVersionSetCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApiVersionSetCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Api Version Set Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiVersionSetContract {
    #[serde(flatten)]
    pub resource: Resource,
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
    pub enum VersioningScheme {
        Segment,
        Query,
        Header,
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
#[doc = "Api Version set base parameters"]
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
#[doc = "Parameters to update or create an Api Version Set Contract."]
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
#[doc = "Association entity details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssociationContract {
    #[serde(flatten)]
    pub resource: Resource,
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
}
impl AuthenticationSettingsContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged OAuth2 Authorization Servers list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationServerCollection {
    #[doc = "Page values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
        self.next_link.clone()
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
    pub resource: Resource,
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
    #[serde(rename = "authorizationMethods", default, skip_serializing_if = "Vec::is_empty")]
    pub authorization_methods: Vec<String>,
    #[doc = "Method of authentication supported by the token endpoint of this authorization server. Possible values are Basic and/or Body. When Body is specified, client credentials and other parameters are passed within the request body in the application/x-www-form-urlencoded format."]
    #[serde(rename = "clientAuthenticationMethod", default, skip_serializing_if = "Vec::is_empty")]
    pub client_authentication_method: Vec<String>,
    #[doc = "Additional parameters required by the token endpoint of this authorization server represented as an array of JSON objects with name and value string properties, i.e. {\"name\" : \"name value\", \"value\": \"a value\"}."]
    #[serde(rename = "tokenBodyParameters", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "bearerTokenSendingMethods", default, skip_serializing_if = "Vec::is_empty")]
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
            client_registration_endpoint,
            authorization_endpoint,
            grant_types,
            client_id,
            client_secret: None,
        }
    }
}
#[doc = "External OAuth authorization server settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationServerUpdateContract {
    #[serde(flatten)]
    pub resource: Resource,
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
    #[doc = "Optional reference to a page where client or app registration for this authorization server is performed. Contains absolute URL to entity being referenced."]
    #[serde(rename = "clientRegistrationEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub client_registration_endpoint: Option<String>,
    #[doc = "OAuth authorization endpoint. See http://tools.ietf.org/html/rfc6749#section-3.2."]
    #[serde(rename = "authorizationEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint: Option<String>,
    #[doc = "Form of an authorization grant, which the client uses to request the access token."]
    #[serde(rename = "grantTypes", default, skip_serializing_if = "Vec::is_empty")]
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
    #[doc = "Management Uri of the Resource in External System. This url can be the Arm Resource Id of Logic Apps, Function Apps or Api Apps."]
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
}
impl BackendBaseParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Backend list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendCollection {
    #[doc = "Backend values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BackendContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BackendCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
    #[doc = "List of Client Certificate Thumbprint."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    pub resource: Resource,
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
    #[doc = "The client certificate thumbprint for the management endpoint."]
    #[serde(rename = "clientCertificatethumbprint")]
    pub client_certificatethumbprint: String,
    #[doc = "Maximum number of retries while attempting resolve the partition."]
    #[serde(rename = "maxPartitionResolutionRetries", default, skip_serializing_if = "Option::is_none")]
    pub max_partition_resolution_retries: Option<i32>,
    #[doc = "The cluster management endpoint."]
    #[serde(rename = "managementEndpoints")]
    pub management_endpoints: Vec<String>,
    #[doc = "Thumbprints of certificates cluster management service uses for tls communication"]
    #[serde(rename = "serverCertificateThumbprints", default, skip_serializing_if = "Vec::is_empty")]
    pub server_certificate_thumbprints: Vec<String>,
    #[doc = "Server X509 Certificate Names Collection"]
    #[serde(rename = "serverX509Names", default, skip_serializing_if = "Vec::is_empty")]
    pub server_x509_names: Vec<X509CertificateName>,
}
impl BackendServiceFabricClusterProperties {
    pub fn new(client_certificatethumbprint: String, management_endpoints: Vec<String>) -> Self {
        Self {
            client_certificatethumbprint,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CacheContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CacheCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
    #[doc = "Original uri of entity in external system cache points to"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl CacheContractProperties {
    pub fn new(connection_string: String) -> Self {
        Self {
            description: None,
            connection_string,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CertificateContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CertificateCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
}
impl CertificateContractProperties {
    pub fn new(subject: String, thumbprint: String, expiration_date: time::OffsetDateTime) -> Self {
        Self {
            subject,
            thumbprint,
            expiration_date,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateCreateOrUpdateProperties {
    #[doc = "Base 64 encoded certificate using the application/x-pkcs12 representation."]
    pub data: String,
    #[doc = "Password for the Certificate"]
    pub password: String,
}
impl CertificateCreateOrUpdateProperties {
    pub fn new(data: String, password: String) -> Self {
        Self { data, password }
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
}
impl ConnectivityStatusContract {
    pub fn new(
        name: String,
        status: connectivity_status_contract::Status,
        last_updated: time::OffsetDateTime,
        last_status_change: time::OffsetDateTime,
    ) -> Self {
        Self {
            name,
            status,
            error: None,
            last_updated,
            last_status_change,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DiagnosticContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiagnosticCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
}
#[doc = "Paged email template list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailTemplateCollection {
    #[doc = "Page values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EmailTemplateContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EmailTemplateCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
    #[doc = "Whether the template is the default template provided by Api Management or has been edited."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "Email Template Parameter values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorFieldContract>,
}
impl ErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Gateway list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayCollection {
    #[doc = "Page values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GatewayContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GatewayCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GatewayHostnameConfigurationContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GatewayHostnameConfigurationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
#[doc = "Paged Group list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupCollection {
    #[doc = "Page values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GroupContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GroupCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
#[doc = "Custom hostname configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostnameConfiguration {
    #[doc = "Hostname type."]
    #[serde(rename = "type")]
    pub type_: hostname_configuration::Type,
    #[doc = "Hostname to configure on the Api Management service."]
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[doc = "Url to the KeyVault Secret containing the Ssl Certificate. If absolute Url containing version is provided, auto-update of ssl certificate will not work. This requires Api Management service to be configured with MSI. The secret should be of type *application/x-pkcs12*"]
    #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_id: Option<String>,
    #[doc = "Base64 Encoded certificate."]
    #[serde(rename = "encodedCertificate", default, skip_serializing_if = "Option::is_none")]
    pub encoded_certificate: Option<String>,
    #[doc = "Certificate Password."]
    #[serde(rename = "certificatePassword", default, skip_serializing_if = "Option::is_none")]
    pub certificate_password: Option<String>,
    #[doc = "Specify true to setup the certificate associated with this Hostname as the Default SSL Certificate. If a client does not send the SNI header, then this will be the certificate that will be challenged. The property is useful if a service has multiple custom hostname enabled and it needs to decide on the default ssl certificate. The setting only applied to Proxy Hostname Type."]
    #[serde(rename = "defaultSslBinding", default, skip_serializing_if = "Option::is_none")]
    pub default_ssl_binding: Option<bool>,
    #[doc = "Specify true to always negotiate client certificate on the hostname. Default Value is false."]
    #[serde(rename = "negotiateClientCertificate", default, skip_serializing_if = "Option::is_none")]
    pub negotiate_client_certificate: Option<bool>,
    #[doc = "SSL certificate information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<CertificateInformation>,
}
impl HostnameConfiguration {
    pub fn new(type_: hostname_configuration::Type, host_name: String) -> Self {
        Self {
            type_,
            host_name,
            key_vault_id: None,
            encoded_certificate: None,
            certificate_password: None,
            default_ssl_binding: None,
            negotiate_client_certificate: None,
            certificate: None,
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
}
#[doc = "Http message diagnostic settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpMessageDiagnostic {
    #[doc = "Array of HTTP Headers to log."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<String>,
    #[doc = "Body logging settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<BodyDiagnosticSettings>,
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
    #[serde(rename = "allowedTenants", default, skip_serializing_if = "Vec::is_empty")]
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
    pub resource: Resource,
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
    pub resource: Resource,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IdentityProviderContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IdentityProviderList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IssueAttachmentContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IssueAttachmentCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IssueContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IssueCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IssueCommentContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IssueCommentCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
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
#[doc = "Issue Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IssueContract {
    #[serde(flatten)]
    pub resource: Resource,
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
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
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
#[doc = "Paged Logger list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoggerCollection {
    #[doc = "Logger values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
        self.next_link.clone()
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
    pub resource: Resource,
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
    pub credentials: serde_json::Value,
    #[doc = "Whether records are buffered in the logger before publishing. Default is assumed to be true."]
    #[serde(rename = "isBuffered", default, skip_serializing_if = "Option::is_none")]
    pub is_buffered: Option<bool>,
    #[doc = "Azure Resource Id of a log target (either Azure Event Hub resource or Azure Application Insights resource)."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl LoggerContractProperties {
    pub fn new(logger_type: logger_contract_properties::LoggerType, credentials: serde_json::Value) -> Self {
        Self {
            logger_type,
            description: None,
            credentials,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Paged NamedValue list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamedValueCollection {
    #[doc = "Page values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NamedValueContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NamedValueCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
}
impl NamedValueContractProperties {
    pub fn new(display_name: String) -> Self {
        Self {
            named_value_entity_base_parameters: NamedValueEntityBaseParameters::default(),
            display_name,
            value: None,
        }
    }
}
#[doc = "NamedValue details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamedValueCreateContract {
    #[serde(flatten)]
    pub resource: Resource,
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
    pub value: String,
}
impl NamedValueCreateContractProperties {
    pub fn new(display_name: String, value: String) -> Self {
        Self {
            named_value_entity_base_parameters: NamedValueEntityBaseParameters::default(),
            display_name,
            value,
        }
    }
}
#[doc = "NamedValue Entity Base Parameters set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamedValueEntityBaseParameters {
    #[doc = "Optional tags that when provided can be used to filter the NamedValue list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NotificationContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NotificationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
    #[serde(rename = "bearerTokenSendingMethods", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OpenidConnectProviderContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OpenIdConnectProviderCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
}
impl OpenidConnectProviderContractProperties {
    pub fn new(display_name: String, metadata_endpoint: String, client_id: String) -> Self {
        Self {
            display_name,
            description: None,
            metadata_endpoint,
            client_id,
            client_secret: None,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Api Operation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationContract {
    #[serde(flatten)]
    pub resource: Resource,
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
#[doc = "Api Operation Entity Base Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntityBaseContract {
    #[doc = "Collection of URL template parameters."]
    #[serde(rename = "templateParameters", default, skip_serializing_if = "Vec::is_empty")]
    pub template_parameters: Vec<ParameterContract>,
    #[doc = "Description of the operation. May include HTML formatting tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Operation request details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<RequestContract>,
    #[doc = "Array of Operation responses."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResultContract {
    #[doc = "Operation result identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of an async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_result_contract::Status>,
    #[doc = "Start time of an async operation. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub started: Option<time::OffsetDateTime>,
    #[doc = "Last update time of an async operation. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub updated: Option<time::OffsetDateTime>,
    #[doc = "Optional result info."]
    #[serde(rename = "resultInfo", default, skip_serializing_if = "Option::is_none")]
    pub result_info: Option<String>,
    #[doc = "Error Body contract."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
    #[doc = "This property if only provided as part of the TenantConfiguration_Validate operation. It contains the log the entities which will be updated/created/deleted as part of the TenantConfiguration_Deploy operation."]
    #[serde(rename = "actionLog", default, skip_serializing_if = "Vec::is_empty")]
    pub action_log: Vec<OperationResultLogItemContract>,
}
impl OperationResultContract {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_result_contract {
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
    #[doc = "Api Name."]
    #[serde(rename = "apiName", default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    #[doc = "Api Revision."]
    #[serde(rename = "apiRevision", default, skip_serializing_if = "Option::is_none")]
    pub api_revision: Option<String>,
    #[doc = "Api Version."]
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
#[doc = "Api Operation Update Contract details."]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
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
        }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
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
    pub resource: Resource,
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
#[doc = "Descriptions of APIM policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDescriptionCollection {
    #[doc = "Descriptions of APIM policies."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    pub resource: Resource,
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
#[doc = "Delegation settings for a developer portal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalDelegationSettings {
    #[serde(flatten)]
    pub resource: Resource,
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
    pub resource: Resource,
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
    pub resource: Resource,
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
#[doc = "Paged Products list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductCollection {
    #[doc = "Page values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProductContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProductCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
    #[doc = "whether subscription approval is required. If false, new subscriptions will be approved automatically enabling developers to call the product’s APIs immediately after subscribing. If true, administrators must manually approve the subscription before the developer can any of the product’s APIs. Can be present only if subscriptionRequired property is present and has a value of true."]
    #[serde(rename = "approvalRequired", default, skip_serializing_if = "Option::is_none")]
    pub approval_required: Option<bool>,
    #[doc = "Whether the number of subscriptions a user can have to this product at the same time. Set to null or omit to allow unlimited per user subscriptions. Can be present only if subscriptionRequired property is present and has a value of true."]
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
#[doc = "Client or app secret used in IdentityProviders, Aad, OpenID or OAuth."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PropertyValueContract {
    #[doc = "This is secret value of the NamedValue entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl PropertyValueContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged Quota Counter list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaCounterCollection {
    #[doc = "Quota counter values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Paged Recipient User list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecipientEmailCollection {
    #[doc = "Page values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RecipientEmailContract>,
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
    pub resource: Resource,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RecipientUserContract>,
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
    pub resource: Resource,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emails: Vec<String>,
    #[doc = "List of Users subscribed for the notification."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
        self.next_link.clone()
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
#[doc = "Paged Report records list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportCollection {
    #[doc = "Page values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
        self.next_link.clone()
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
    #[serde(with = "azure_core::date::rfc3339::option")]
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
    #[doc = "Number of calls failed due to proxy or backend errors. This includes calls returning HttpStatusCode.BadRequest(400) and any Code between HttpStatusCode.InternalServerError (500) and 600"]
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
    #[doc = "An example of the representation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample: Option<String>,
    #[doc = "Schema identifier. Applicable only if 'contentType' value is neither 'application/x-www-form-urlencoded' nor 'multipart/form-data'."]
    #[serde(rename = "schemaId", default, skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    #[doc = "Type name defined by the schema. Applicable only if 'contentType' value is neither 'application/x-www-form-urlencoded' nor 'multipart/form-data'."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "Collection of form parameters. Required if 'contentType' value is either 'application/x-www-form-urlencoded' or 'multipart/form-data'.."]
    #[serde(rename = "formParameters", default, skip_serializing_if = "Vec::is_empty")]
    pub form_parameters: Vec<ParameterContract>,
}
impl RepresentationContract {
    pub fn new(content_type: String) -> Self {
        Self {
            content_type,
            sample: None,
            schema_id: None,
            type_name: None,
            form_parameters: Vec::new(),
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
    #[serde(rename = "queryParameters", default, skip_serializing_if = "Vec::is_empty")]
    pub query_parameters: Vec<ParameterContract>,
    #[doc = "Collection of operation request headers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<ParameterContract>,
    #[doc = "Collection of operation request representations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(with = "azure_core::date::rfc3339::option")]
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
#[doc = "The Resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type for API Management resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
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
        self.next_link.clone()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub representations: Vec<RepresentationContract>,
    #[doc = "Collection of operation response headers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[doc = "Api Schema Contract value."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SchemaContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SchemaCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SchemaCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema Contract details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaContract {
    #[serde(flatten)]
    pub resource: Resource,
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
    #[doc = "Schema Document Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<SchemaDocumentProperties>,
}
impl SchemaContractProperties {
    pub fn new(content_type: String) -> Self {
        Self {
            content_type,
            document: None,
        }
    }
}
#[doc = "Schema Document Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaDocumentProperties {
    #[doc = "Json escaped string defining the document representing the Schema. Used for schemas other than Swagger/OpenAPI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Types definitions. Used for Swagger/OpenAPI schemas only, null otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definitions: Option<serde_json::Value>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubscriptionContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SubscriptionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "Subscription activation date. The setting is for audit purposes only and the subscription is not automatically activated. The subscription lifecycle can be managed by using the `state` property. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "startDate", with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "Subscription expiration date. The setting is for audit purposes only and the subscription is not automatically expired. The subscription lifecycle can be managed by using the `state` property. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "expirationDate", with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Date when subscription was cancelled or expired. The setting is for audit purposes only and the subscription is not automatically cancelled. The subscription lifecycle can be managed by using the `state` property. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "endDate", with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<time::OffsetDateTime>,
    #[doc = "Upcoming subscription expiration notification date. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "notificationDate", with = "azure_core::date::rfc3339::option")]
    pub notification_date: Option<time::OffsetDateTime>,
    #[doc = "Subscription primary key. This property will not be filled on 'GET' operations! Use '/listSecrets' POST request to get the value."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "Subscription secondary key. This property will not be filled on 'GET' operations! Use '/listSecrets' POST request to get the value."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "Optional subscription comment added by an administrator."]
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
    #[serde(rename = "expirationDate", with = "azure_core::date::rfc3339::option")]
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
    #[doc = "Comments describing subscription state change by the administrator."]
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
#[doc = "Paged Tag list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagCollection {
    #[doc = "Page values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TagContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TagCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TagDescriptionContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TagDescriptionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
#[doc = "Paged Tag list representation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagResourceCollection {
    #[doc = "Page values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TagResourceContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TagResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[doc = "Tenant Configuration Synchronization State."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantConfigurationSyncStateContract {
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
    #[serde(rename = "syncDate", with = "azure_core::date::rfc3339::option")]
    pub sync_date: Option<time::OffsetDateTime>,
    #[doc = "The date of the latest configuration change. The date conforms to the following format: `yyyy-MM-ddTHH:mm:ssZ` as specified by the ISO 8601 standard.\n"]
    #[serde(rename = "configurationChangeDate", with = "azure_core::date::rfc3339::option")]
    pub configuration_change_date: Option<time::OffsetDateTime>,
}
impl TenantConfigurationSyncStateContract {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UserContract>,
    #[doc = "Next page link if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UserCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
    pub resource: Resource,
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
    #[serde(rename = "registrationDate", with = "azure_core::date::rfc3339::option")]
    pub registration_date: Option<time::OffsetDateTime>,
    #[doc = "Collection of groups user is part of."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[doc = "Determines the type of application which send the create user request. Default is old publisher portal."]
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
    #[doc = "Determines the type of application which send the create user request. Default is old publisher portal."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AppType")]
    pub enum AppType {
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
                Self::DeveloperPortal => serializer.serialize_unit_variant("AppType", 0u32, "developerPortal"),
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
        self.next_link.clone()
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
