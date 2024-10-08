#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Informatica Serverless advanced custom properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdvancedCustomProperties {
    #[doc = "advanced custom properties key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "advanced custom properties value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl AdvancedCustomProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application configs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationConfigs {
    #[doc = "Type of the application config."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Name of the application config."]
    pub name: String,
    #[doc = "Value of the application config."]
    pub value: String,
    #[doc = "Platform type of the application config."]
    pub platform: String,
    #[doc = "Customized value of the application config."]
    pub customized: String,
    #[doc = "Default value of the application config."]
    #[serde(rename = "defaultValue")]
    pub default_value: String,
}
impl ApplicationConfigs {
    pub fn new(type_: String, name: String, value: String, platform: String, customized: String, default_value: String) -> Self {
        Self {
            type_,
            name,
            value,
            platform,
            customized,
            default_value,
        }
    }
}
#[doc = "Various application types of the Serverless Runtime environments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApplicationType")]
pub enum ApplicationType {
    #[serde(rename = "CDI")]
    Cdi,
    #[serde(rename = "CDIE")]
    Cdie,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApplicationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApplicationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApplicationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Cdi => serializer.serialize_unit_variant("ApplicationType", 0u32, "CDI"),
            Self::Cdie => serializer.serialize_unit_variant("ApplicationType", 1u32, "CDIE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Informatica Serverless Runtime Application type Metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationTypeMetadata {
    #[doc = "Application type name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Application type value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ApplicationTypeMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type AzureCoreUuid = String;
#[doc = "Informatica CDI Configuration Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CdiConfigProps {
    #[doc = "EngineName of the application config."]
    #[serde(rename = "engineName")]
    pub engine_name: String,
    #[doc = "EngineVersion of the application config."]
    #[serde(rename = "engineVersion")]
    pub engine_version: String,
    #[doc = "ApplicationConfigs of the CDI or CDIE."]
    #[serde(rename = "applicationConfigs")]
    pub application_configs: Vec<ApplicationConfigs>,
}
impl CdiConfigProps {
    pub fn new(engine_name: String, engine_version: String, application_configs: Vec<ApplicationConfigs>) -> Self {
        Self {
            engine_name,
            engine_version,
            application_configs,
        }
    }
}
#[doc = "Model for the check dependencies API for an informatica serverless runtime resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckDependenciesResponse {
    #[doc = "Count of dependencies"]
    pub count: i32,
    #[doc = "id of resource"]
    pub id: String,
    #[doc = "List of dependencies"]
    pub references: Vec<ServerlessRuntimeDependency>,
}
impl CheckDependenciesResponse {
    pub fn new(count: i32, id: String, references: Vec<ServerlessRuntimeDependency>) -> Self {
        Self { count, id, references }
    }
}
#[doc = "Company Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CompanyDetails {
    #[doc = "company Name"]
    #[serde(rename = "companyName", default, skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[doc = "Office Address"]
    #[serde(rename = "officeAddress", default, skip_serializing_if = "Option::is_none")]
    pub office_address: Option<String>,
    #[doc = "Country name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Domain name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "Business phone number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business: Option<String>,
    #[doc = "Number Of Employees"]
    #[serde(rename = "numberOfEmployees", default, skip_serializing_if = "Option::is_none")]
    pub number_of_employees: Option<i32>,
}
impl CompanyDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Company details of Informatica Organization resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CompanyDetailsUpdate {
    #[doc = "company Name"]
    #[serde(rename = "companyName", default, skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[doc = "Office Address"]
    #[serde(rename = "officeAddress", default, skip_serializing_if = "Option::is_none")]
    pub office_address: Option<String>,
    #[doc = "Country name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Domain name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "Business phone number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business: Option<String>,
    #[doc = "Number Of Employees"]
    #[serde(rename = "numberOfEmployees", default, skip_serializing_if = "Option::is_none")]
    pub number_of_employees: Option<i32>,
}
impl CompanyDetailsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Informatica Serverless Runtime Application type Metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeUnitsMetadata {
    #[doc = "ComputeUnit name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "ComputeUnit value"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<String>,
}
impl ComputeUnitsMetadata {
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
#[doc = "Informatica runtime resource metadata as received via the informatica fetch all runtime environments API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfaRuntimeResourceFetchMetaData {
    #[doc = "Environment name"]
    pub name: String,
    #[doc = "Created time"]
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[doc = "Updated Time"]
    #[serde(rename = "updatedTime")]
    pub updated_time: String,
    #[doc = "Created by"]
    #[serde(rename = "createdBy")]
    pub created_by: String,
    #[doc = "Last Updated by"]
    #[serde(rename = "updatedBy")]
    pub updated_by: String,
    #[doc = "Informatica serverless runtime id"]
    pub id: String,
    #[doc = "Various types of the runtime types."]
    #[serde(rename = "type")]
    pub type_: RuntimeType,
    #[doc = "Status of the environment"]
    pub status: String,
    #[doc = "Display message for the given status"]
    #[serde(rename = "statusLocalized")]
    pub status_localized: String,
    #[doc = "status message"]
    #[serde(rename = "statusMessage")]
    pub status_message: String,
    #[doc = "InfaServerlessFetchConfigProperties for the fetch all serverless API as received from informatica API response"]
    #[serde(rename = "serverlessConfigProperties")]
    pub serverless_config_properties: InfaServerlessFetchConfigProperties,
    #[doc = "Description of the runtime resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl InfaRuntimeResourceFetchMetaData {
    pub fn new(
        name: String,
        created_time: String,
        updated_time: String,
        created_by: String,
        updated_by: String,
        id: String,
        type_: RuntimeType,
        status: String,
        status_localized: String,
        status_message: String,
        serverless_config_properties: InfaServerlessFetchConfigProperties,
    ) -> Self {
        Self {
            name,
            created_time,
            updated_time,
            created_by,
            updated_by,
            id,
            type_,
            status,
            status_localized,
            status_message,
            serverless_config_properties,
            description: None,
        }
    }
}
#[doc = "InfaServerlessFetchConfigProperties for the fetch all serverless API as received from informatica API response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InfaServerlessFetchConfigProperties {
    #[doc = "subnet name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[doc = "applicationType name"]
    #[serde(rename = "applicationType", default, skip_serializing_if = "Option::is_none")]
    pub application_type: Option<String>,
    #[doc = "Resource group name"]
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[doc = "Advanced custom properties"]
    #[serde(rename = "advancedCustomProperties", default, skip_serializing_if = "Option::is_none")]
    pub advanced_custom_properties: Option<String>,
    #[doc = "Supplementary File location"]
    #[serde(rename = "supplementaryFileLocation", default, skip_serializing_if = "Option::is_none")]
    pub supplementary_file_location: Option<String>,
    #[doc = "Serverless Account Platform"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[doc = "Tags for the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[doc = "virtual network"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vnet: Option<String>,
    #[doc = "Execution timeout"]
    #[serde(rename = "executionTimeout", default, skip_serializing_if = "Option::is_none")]
    pub execution_timeout: Option<String>,
    #[doc = "Compute Units"]
    #[serde(rename = "computeUnits", default, skip_serializing_if = "Option::is_none")]
    pub compute_units: Option<String>,
    #[doc = "Universally Unique Identifier"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<AzureCoreUuid>,
    #[doc = "subscription ID"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "region name for the runtime environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "Serverless Arm Resource ID"]
    #[serde(rename = "serverlessArmResourceId", default, skip_serializing_if = "Option::is_none")]
    pub serverless_arm_resource_id: Option<String>,
}
impl InfaServerlessFetchConfigProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Organization Resource by Informatica."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InformaticaOrganizationResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties specific to the Informatica DataManagement Organization resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OrganizationProperties>,
}
impl InformaticaOrganizationResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a InformaticaOrganizationResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InformaticaOrganizationResourceListResult {
    #[doc = "The InformaticaOrganizationResource items on this page"]
    pub value: Vec<InformaticaOrganizationResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InformaticaOrganizationResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl InformaticaOrganizationResourceListResult {
    pub fn new(value: Vec<InformaticaOrganizationResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The template for adding optional properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InformaticaOrganizationResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Patchable Properties of the Informatica Organization resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<OrganizationPropertiesCustomUpdate>>,
}
impl InformaticaOrganizationResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Informatica organization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InformaticaProperties {
    #[doc = "Organization id"]
    #[serde(rename = "organizationId", default, skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[doc = "Organization name"]
    #[serde(rename = "organizationName", default, skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    #[doc = "Informatica organization region"]
    #[serde(rename = "informaticaRegion", default, skip_serializing_if = "Option::is_none")]
    pub informatica_region: Option<String>,
    #[doc = "Single sing on URL for informatica organization"]
    #[serde(rename = "singleSignOnUrl", default, skip_serializing_if = "Option::is_none")]
    pub single_sign_on_url: Option<String>,
}
impl InformaticaProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Serverless Runtime properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InformaticaServerlessRuntimeProperties {
    #[doc = "Provisioning State of the Organization resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "description of the serverless runtime."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Various types of the Platform types."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformType>,
    #[doc = "Various application types of the Serverless Runtime environments"]
    #[serde(rename = "applicationType", default, skip_serializing_if = "Option::is_none")]
    pub application_type: Option<ApplicationType>,
    #[doc = "Compute units of the serverless runtime."]
    #[serde(rename = "computeUnits", default, skip_serializing_if = "Option::is_none")]
    pub compute_units: Option<String>,
    #[doc = "Serverless Execution timeout"]
    #[serde(rename = "executionTimeout", default, skip_serializing_if = "Option::is_none")]
    pub execution_timeout: Option<String>,
    #[doc = "Serverless account creation location"]
    #[serde(rename = "serverlessAccountLocation")]
    pub serverless_account_location: String,
    #[doc = "Informatica Serverless Runtime Network Profile."]
    #[serde(rename = "serverlessRuntimeNetworkProfile", default, skip_serializing_if = "Option::is_none")]
    pub serverless_runtime_network_profile: Option<ServerlessRuntimeNetworkProfile>,
    #[doc = "String KV pairs indicating Advanced custom properties."]
    #[serde(
        rename = "advancedCustomProperties",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub advanced_custom_properties: Vec<AdvancedCustomProperties>,
    #[doc = "Supplementary file location."]
    #[serde(rename = "supplementaryFileLocation", default, skip_serializing_if = "Option::is_none")]
    pub supplementary_file_location: Option<String>,
    #[doc = "Serverless Runtime config properties."]
    #[serde(rename = "serverlessRuntimeConfig", default, skip_serializing_if = "Option::is_none")]
    pub serverless_runtime_config: Option<ServerlessRuntimeConfigProperties>,
    #[doc = "Serverless Runtime Tags"]
    #[serde(
        rename = "serverlessRuntimeTags",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub serverless_runtime_tags: Vec<ServerlessRuntimeTag>,
    #[doc = "Informatica Serverless Runtime User context properties"]
    #[serde(
        rename = "serverlessRuntimeUserContextProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub serverless_runtime_user_context_properties: Option<ServerlessRuntimeUserContextProperties>,
}
impl InformaticaServerlessRuntimeProperties {
    pub fn new(serverless_account_location: String) -> Self {
        Self {
            provisioning_state: None,
            description: None,
            platform: None,
            application_type: None,
            compute_units: None,
            execution_timeout: None,
            serverless_account_location,
            serverless_runtime_network_profile: None,
            advanced_custom_properties: Vec::new(),
            supplementary_file_location: None,
            serverless_runtime_config: None,
            serverless_runtime_tags: Vec::new(),
            serverless_runtime_user_context_properties: None,
        }
    }
}
#[doc = "A Serverless Runtime environment  resource by Informatica."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InformaticaServerlessRuntimeResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Serverless Runtime properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InformaticaServerlessRuntimeProperties>,
}
impl InformaticaServerlessRuntimeResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of serverless runtime resources as fetched using the informatica APIs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InformaticaServerlessRuntimeResourceList {
    #[doc = "List of runtime resources for the fetch all API"]
    #[serde(rename = "informaticaRuntimeResources")]
    pub informatica_runtime_resources: Vec<InfaRuntimeResourceFetchMetaData>,
}
impl InformaticaServerlessRuntimeResourceList {
    pub fn new(informatica_runtime_resources: Vec<InfaRuntimeResourceFetchMetaData>) -> Self {
        Self {
            informatica_runtime_resources,
        }
    }
}
#[doc = "The response of a InformaticaServerlessRuntimeResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InformaticaServerlessRuntimeResourceListResult {
    #[doc = "The InformaticaServerlessRuntimeResource items on this page"]
    pub value: Vec<InformaticaServerlessRuntimeResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InformaticaServerlessRuntimeResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl InformaticaServerlessRuntimeResourceListResult {
    pub fn new(value: Vec<InformaticaServerlessRuntimeResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The template for adding optional properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InformaticaServerlessRuntimeResourceUpdate {
    #[doc = "Patchable Properties of the Informatica Serverless Runtime resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerlessRuntimePropertiesCustomUpdate>,
}
impl InformaticaServerlessRuntimeResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Link Organization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkOrganization {
    #[doc = "Link organization token"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl LinkOrganization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Marketplace details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceDetails {
    #[doc = "Marketplace Subscription Id"]
    #[serde(rename = "marketplaceSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_subscription_id: Option<String>,
    #[doc = "Details of the product offering."]
    #[serde(rename = "offerDetails")]
    pub offer_details: OfferDetails,
}
impl MarketplaceDetails {
    pub fn new(offer_details: OfferDetails) -> Self {
        Self {
            marketplace_subscription_id: None,
            offer_details,
        }
    }
}
#[doc = "Marketplace details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketplaceDetailsUpdate {
    #[doc = "Marketplace Subscription Id"]
    #[serde(rename = "marketplaceSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_subscription_id: Option<String>,
    #[doc = "Details of the product offering"]
    #[serde(rename = "offerDetails", default, skip_serializing_if = "Option::is_none")]
    pub offer_details: Option<OfferDetailsUpdate>,
}
impl MarketplaceDetailsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Informatica Serverless Runtime Network Interface configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkInterfaceConfiguration {
    #[doc = "Virtual network resource id"]
    #[serde(rename = "vnetId")]
    pub vnet_id: String,
    #[doc = "Virtual network subnet resource id"]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[doc = "Virtual network resource guid"]
    #[serde(rename = "vnetResourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub vnet_resource_guid: Option<String>,
}
impl NetworkInterfaceConfiguration {
    pub fn new(vnet_id: String, subnet_id: String) -> Self {
        Self {
            vnet_id,
            subnet_id,
            vnet_resource_guid: None,
        }
    }
}
#[doc = "The template for adding optional properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceConfigurationUpdate {
    #[doc = "Virtual network resource id"]
    #[serde(rename = "vnetId", default, skip_serializing_if = "Option::is_none")]
    pub vnet_id: Option<String>,
    #[doc = "Virtual network subnet resource id"]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "Virtual network resource guid"]
    #[serde(rename = "vnetResourceGuid", default, skip_serializing_if = "Option::is_none")]
    pub vnet_resource_guid: Option<String>,
}
impl NetworkInterfaceConfigurationUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the product offering."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfferDetails {
    #[doc = "Id of the product publisher."]
    #[serde(rename = "publisherId")]
    pub publisher_id: String,
    #[doc = "Id of the product offering."]
    #[serde(rename = "offerId")]
    pub offer_id: String,
    #[doc = "Id of the product offer plan."]
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[doc = "Name of the product offer plan."]
    #[serde(rename = "planName")]
    pub plan_name: String,
    #[doc = "Offer plan term unit."]
    #[serde(rename = "termUnit", default, skip_serializing_if = "Option::is_none")]
    pub term_unit: Option<String>,
    #[doc = "Offer plan term id."]
    #[serde(rename = "termId")]
    pub term_id: String,
}
impl OfferDetails {
    pub fn new(publisher_id: String, offer_id: String, plan_id: String, plan_name: String, term_id: String) -> Self {
        Self {
            publisher_id,
            offer_id,
            plan_id,
            plan_name,
            term_unit: None,
            term_id,
        }
    }
}
#[doc = "Details of the product offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OfferDetailsUpdate {
    #[doc = "Id of the product publisher."]
    #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[doc = "Id of the product offering."]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "Id of the product offer plan."]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Name of the product offer plan."]
    #[serde(rename = "planName", default, skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[doc = "Offer plan term unit."]
    #[serde(rename = "termUnit", default, skip_serializing_if = "Option::is_none")]
    pub term_unit: Option<String>,
    #[doc = "Offer plan term id."]
    #[serde(rename = "termId", default, skip_serializing_if = "Option::is_none")]
    pub term_id: Option<String>,
}
impl OfferDetailsUpdate {
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
#[doc = "Properties specific to the Informatica DataManagement Organization resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrganizationProperties {
    #[doc = "Provisioning State of the Organization resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Properties of the Informatica organization."]
    #[serde(rename = "informaticaProperties", default, skip_serializing_if = "Option::is_none")]
    pub informatica_properties: Option<InformaticaProperties>,
    #[doc = "Marketplace details."]
    #[serde(rename = "marketplaceDetails", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_details: Option<MarketplaceDetails>,
    #[doc = "User Info of Informatica Organization resource."]
    #[serde(rename = "userDetails", default, skip_serializing_if = "Option::is_none")]
    pub user_details: Option<UserDetails>,
    #[doc = "Company Details."]
    #[serde(rename = "companyDetails", default, skip_serializing_if = "Option::is_none")]
    pub company_details: Option<CompanyDetails>,
    #[doc = "Link Organization"]
    #[serde(rename = "linkOrganization", default, skip_serializing_if = "Option::is_none")]
    pub link_organization: Option<LinkOrganization>,
}
impl OrganizationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Patchable Properties of the Informatica Organization resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrganizationPropertiesCustomUpdate {
    #[doc = "The template for adding optional properties."]
    #[serde(rename = "informaticaOrganizationProperties", default, skip_serializing_if = "Option::is_none")]
    pub informatica_organization_properties: Option<Box<InformaticaOrganizationResourceUpdate>>,
    #[doc = "Marketplace details"]
    #[serde(rename = "marketplaceDetails", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_details: Option<MarketplaceDetailsUpdate>,
    #[doc = "User Info of Informatica Organization resource"]
    #[serde(rename = "userDetails", default, skip_serializing_if = "Option::is_none")]
    pub user_details: Option<UserDetailsUpdate>,
    #[doc = "Company details of Informatica Organization resource"]
    #[serde(rename = "companyDetails", default, skip_serializing_if = "Option::is_none")]
    pub company_details: Option<CompanyDetailsUpdate>,
    #[doc = "Existing Resource Id"]
    #[serde(rename = "existingResourceId", default, skip_serializing_if = "Option::is_none")]
    pub existing_resource_id: Option<String>,
}
impl OrganizationPropertiesCustomUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Various types of the Platform types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PlatformType")]
pub enum PlatformType {
    #[serde(rename = "AZURE")]
    Azure,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PlatformType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PlatformType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PlatformType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Azure => serializer.serialize_unit_variant("PlatformType", 0u32, "AZURE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Provisioning State of the Organization resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Accepted,
    Creating,
    Updating,
    Deleting,
    Succeeded,
    Failed,
    Canceled,
    Deleted,
    NotSpecified,
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
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Canceled"),
            Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleted"),
            Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 8u32, "NotSpecified"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "Informatica Serverless Runtime Regions Metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegionsMetadata {
    #[doc = "Region Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Region name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl RegionsMetadata {
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
#[doc = "Various types of the runtime types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RuntimeType")]
pub enum RuntimeType {
    #[serde(rename = "SERVERLESS")]
    Serverless,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RuntimeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RuntimeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RuntimeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Serverless => serializer.serialize_unit_variant("RuntimeType", 0u32, "SERVERLESS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Metadata Serverless Config Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerlessConfigProperties {
    #[doc = "Various types of the Platform types."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformType>,
    #[doc = "List of application types supported by informatica"]
    #[serde(
        rename = "applicationTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub application_types: Vec<ApplicationTypeMetadata>,
    #[doc = "The list of compute units with possible array of values"]
    #[serde(
        rename = "computeUnits",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub compute_units: Vec<ComputeUnitsMetadata>,
    #[doc = "Serverless Runtime execution timeout"]
    #[serde(rename = "executionTimeout", default, skip_serializing_if = "Option::is_none")]
    pub execution_timeout: Option<String>,
    #[doc = "List of supported serverless informatica regions"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub regions: Vec<RegionsMetadata>,
}
impl ServerlessConfigProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Serverless Runtime environment Metadata response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerlessMetadataResponse {
    #[doc = "Various types of the runtime types."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<RuntimeType>,
    #[doc = "Metadata Serverless Config Properties"]
    #[serde(rename = "serverlessConfigProperties", default, skip_serializing_if = "Option::is_none")]
    pub serverless_config_properties: Option<ServerlessConfigProperties>,
    #[doc = "Serverless Runtime config properties."]
    #[serde(rename = "serverlessRuntimeConfigProperties", default, skip_serializing_if = "Option::is_none")]
    pub serverless_runtime_config_properties: Option<ServerlessRuntimeConfigProperties>,
}
impl ServerlessMetadataResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Serverless Runtime config properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerlessRuntimeConfigProperties {
    #[doc = "The List of Informatica Serverless Runtime CDI Config Properties."]
    #[serde(
        rename = "cdiConfigProps",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cdi_config_props: Vec<CdiConfigProps>,
    #[doc = "The List of Informatica Serverless Runtime CDIE Config Properties."]
    #[serde(
        rename = "cdieConfigProps",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cdie_config_props: Vec<CdiConfigProps>,
}
impl ServerlessRuntimeConfigProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The template for adding optional properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerlessRuntimeConfigPropertiesUpdate {
    #[doc = "The List of Informatica Serverless Runtime CDI Config Properties."]
    #[serde(
        rename = "cdiConfigProps",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cdi_config_props: Vec<CdiConfigProps>,
    #[doc = "The List of Informatica Serverless Runtime CDIE Config Properties."]
    #[serde(
        rename = "cdieConfigProps",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cdie_config_props: Vec<CdiConfigProps>,
}
impl ServerlessRuntimeConfigPropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dependency reference for a serverless runtime resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerlessRuntimeDependency {
    #[doc = "Dependency ID"]
    pub id: String,
    #[doc = "Application context ID"]
    #[serde(rename = "appContextId")]
    pub app_context_id: String,
    #[doc = "Dependency path"]
    pub path: String,
    #[doc = "document type"]
    #[serde(rename = "documentType")]
    pub document_type: String,
    #[doc = "description of Dependency"]
    pub description: String,
    #[doc = "Last Update Time"]
    #[serde(rename = "lastUpdatedTime")]
    pub last_updated_time: String,
}
impl ServerlessRuntimeDependency {
    pub fn new(
        id: String,
        app_context_id: String,
        path: String,
        document_type: String,
        description: String,
        last_updated_time: String,
    ) -> Self {
        Self {
            id,
            app_context_id,
            path,
            document_type,
            description,
            last_updated_time,
        }
    }
}
#[doc = "Informatica Serverless Runtime Network Profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerlessRuntimeNetworkProfile {
    #[doc = "Informatica Serverless Runtime Network Interface configurations."]
    #[serde(rename = "networkInterfaceConfiguration")]
    pub network_interface_configuration: NetworkInterfaceConfiguration,
}
impl ServerlessRuntimeNetworkProfile {
    pub fn new(network_interface_configuration: NetworkInterfaceConfiguration) -> Self {
        Self {
            network_interface_configuration,
        }
    }
}
#[doc = "Informatica Serverless Network profile properties update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerlessRuntimeNetworkProfileUpdate {
    #[doc = "The template for adding optional properties."]
    #[serde(rename = "networkInterfaceConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub network_interface_configuration: Option<NetworkInterfaceConfigurationUpdate>,
}
impl ServerlessRuntimeNetworkProfileUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Patchable Properties of the Informatica Serverless Runtime resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerlessRuntimePropertiesCustomUpdate {
    #[doc = "description of the serverless runtime."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Various types of the Platform types."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<PlatformType>,
    #[doc = "Various application types of the Serverless Runtime environments"]
    #[serde(rename = "applicationType", default, skip_serializing_if = "Option::is_none")]
    pub application_type: Option<ApplicationType>,
    #[doc = "Compute units of the serverless runtime."]
    #[serde(rename = "computeUnits", default, skip_serializing_if = "Option::is_none")]
    pub compute_units: Option<String>,
    #[doc = "Serverless Execution timeout"]
    #[serde(rename = "executionTimeout", default, skip_serializing_if = "Option::is_none")]
    pub execution_timeout: Option<String>,
    #[doc = "Serverless account creation location"]
    #[serde(rename = "serverlessAccountLocation", default, skip_serializing_if = "Option::is_none")]
    pub serverless_account_location: Option<String>,
    #[doc = "Informatica Serverless Network profile properties update."]
    #[serde(rename = "serverlessRuntimeNetworkProfile", default, skip_serializing_if = "Option::is_none")]
    pub serverless_runtime_network_profile: Option<ServerlessRuntimeNetworkProfileUpdate>,
    #[doc = "String KV pairs indicating Advanced custom properties."]
    #[serde(
        rename = "advancedCustomProperties",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub advanced_custom_properties: Vec<AdvancedCustomProperties>,
    #[doc = "Supplementary file location."]
    #[serde(rename = "supplementaryFileLocation", default, skip_serializing_if = "Option::is_none")]
    pub supplementary_file_location: Option<String>,
    #[doc = "The template for adding optional properties."]
    #[serde(rename = "serverlessRuntimeConfig", default, skip_serializing_if = "Option::is_none")]
    pub serverless_runtime_config: Option<ServerlessRuntimeConfigPropertiesUpdate>,
    #[doc = "Serverless Runtime Tags"]
    #[serde(
        rename = "serverlessRuntimeTags",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub serverless_runtime_tags: Vec<ServerlessRuntimeTag>,
    #[doc = "The template for adding optional properties."]
    #[serde(
        rename = "serverlessRuntimeUserContextProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub serverless_runtime_user_context_properties: Option<ServerlessRuntimeUserContextPropertiesUpdate>,
}
impl ServerlessRuntimePropertiesCustomUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Serverless Runtime Tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerlessRuntimeTag {
    #[doc = "The name (also known as the key) of the tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ServerlessRuntimeTag {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Informatica Serverless Runtime User context properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerlessRuntimeUserContextProperties {
    #[doc = "User context token for OBO flow."]
    #[serde(rename = "userContextToken")]
    pub user_context_token: String,
}
impl ServerlessRuntimeUserContextProperties {
    pub fn new(user_context_token: String) -> Self {
        Self { user_context_token }
    }
}
#[doc = "The template for adding optional properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerlessRuntimeUserContextPropertiesUpdate {
    #[doc = "User context token for OBO flow."]
    #[serde(rename = "userContextToken", default, skip_serializing_if = "Option::is_none")]
    pub user_context_token: Option<String>,
}
impl ServerlessRuntimeUserContextPropertiesUpdate {
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
#[doc = "User Info of Informatica Organization resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserDetails {
    #[doc = "User first name."]
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "User last name."]
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Reusable representation of an email address."]
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<Email>,
    #[doc = "UPN of user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upn: Option<String>,
    #[doc = "Phone number of the user used by for contacting them if needed"]
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl UserDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User Info of Informatica Organization resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserDetailsUpdate {
    #[doc = "User first name."]
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "User last name."]
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Reusable representation of an email address."]
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<Email>,
    #[doc = "UPN of user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upn: Option<String>,
    #[doc = "Phone number of the user used by for contacting them if needed"]
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl UserDetailsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Email = String;
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
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
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
