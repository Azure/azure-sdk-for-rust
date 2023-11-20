#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The properties to maximize savings by using Azure Hybrid Benefit"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureHybridBenefitProperties {
    #[doc = "The license type associated with different SCOM infrastructure components."]
    #[serde(rename = "scomLicenseType", default, skip_serializing_if = "Option::is_none")]
    pub scom_license_type: Option<LicenseTypeEnum>,
    #[doc = "The license type associated with different SCOM infrastructure components."]
    #[serde(rename = "windowsServerLicenseType", default, skip_serializing_if = "Option::is_none")]
    pub windows_server_license_type: Option<LicenseTypeEnum>,
    #[doc = "The license type associated with different SCOM infrastructure components."]
    #[serde(rename = "sqlServerLicenseType", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_license_type: Option<LicenseTypeEnum>,
}
impl AzureHybridBenefitProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of database instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseInstanceProperties {
    #[doc = "Resource Id of existing database instance"]
    #[serde(rename = "databaseInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub database_instance_id: Option<String>,
    #[doc = "Fully qualified domain name of existing database instance"]
    #[serde(rename = "databaseFqdn", default, skip_serializing_if = "Option::is_none")]
    pub database_fqdn: Option<String>,
    #[doc = "Name of warehouse database on database instance"]
    #[serde(rename = "dwDatabaseName", default, skip_serializing_if = "Option::is_none")]
    pub dw_database_name: Option<String>,
    #[doc = "Resource Id of operational database on database instance"]
    #[serde(rename = "operationalDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub operational_database_id: Option<String>,
    #[doc = "Resource Id of warehouse database on database instance"]
    #[serde(rename = "dwDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub dw_database_id: Option<String>,
}
impl DatabaseInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of domain controller to which SCOM and SQL servers join for AuthN/AuthZ."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainControllerProperties {
    #[doc = "Fully qualified domain name"]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "IP address of DNS server "]
    #[serde(rename = "dnsServer", default, skip_serializing_if = "Option::is_none")]
    pub dns_server: Option<String>,
    #[doc = "Organizational Unit path in which the SCOM servers will be present"]
    #[serde(rename = "ouPath", default, skip_serializing_if = "Option::is_none")]
    pub ou_path: Option<String>,
}
impl DomainControllerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Get Domain user name and password from key vault"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainUserCredentials {
    #[doc = "Key vault url to get the domain username and password"]
    #[serde(rename = "keyVaultUrl", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_url: Option<String>,
    #[doc = "Domain user name secret "]
    #[serde(rename = "userNameSecret", default, skip_serializing_if = "Option::is_none")]
    pub user_name_secret: Option<String>,
    #[doc = "Domain Password secret "]
    #[serde(rename = "passwordSecret", default, skip_serializing_if = "Option::is_none")]
    pub password_secret: Option<String>,
}
impl DomainUserCredentials {
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
#[doc = "Gmsa Details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GmsaDetails {
    #[doc = "Frontend IP configuration for Load Balancer, which should be an available IP in customer VNet"]
    #[serde(rename = "loadBalancerIP", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_ip: Option<String>,
    #[doc = "gMSA account under which context all Management Server services will run"]
    #[serde(rename = "gmsaAccount", default, skip_serializing_if = "Option::is_none")]
    pub gmsa_account: Option<String>,
    #[doc = "OnPrem AD Computer Group where we will join VMs for ease of management"]
    #[serde(rename = "managementServerGroupName", default, skip_serializing_if = "Option::is_none")]
    pub management_server_group_name: Option<String>,
    #[doc = "Frontend DNS name for Load Balancer which will be used by Agents to initiate communication"]
    #[serde(rename = "dnsName", default, skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
}
impl GmsaDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The license type associated with different SCOM infrastructure components."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LicenseTypeEnum")]
pub enum LicenseTypeEnum {
    None,
    AzureHybridBenefit,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LicenseTypeEnum {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LicenseTypeEnum {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LicenseTypeEnum {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("LicenseTypeEnum", 0u32, "None"),
            Self::AzureHybridBenefit => serializer.serialize_unit_variant("LicenseTypeEnum", 1u32, "AzureHybridBenefit"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A gateway resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedGateway {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a gateway resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagedGatewayProperties>,
}
impl ManagedGateway {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a gateway resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedGatewayProperties {
    #[doc = "ArmId of the gateway to be monitored."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Location of the gateway to be monitored."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "ComputerName of the gateway to be monitored."]
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[doc = "The domain name associated with the gateway to be monitored."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "The management server endpoint to which the gateway is directed."]
    #[serde(rename = "managementServerEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub management_server_endpoint: Option<String>,
    #[doc = "The health status of the gateway resource."]
    #[serde(rename = "healthStatus", default, skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[doc = "The connection status of the gateway resource."]
    #[serde(rename = "connectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<String>,
    #[doc = "The version of the gateway resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Install type of gateway resource."]
    #[serde(rename = "installType", default, skip_serializing_if = "Option::is_none")]
    pub install_type: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ManagedGatewayProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paginated list of SCOM gateways."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedGateways {
    #[doc = "The contents displayed on the page."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedGateway>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagedGateways {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ManagedGateways {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Active Directory identity configuration for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentity {
    #[doc = "The identity type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<managed_identity::Type>,
    #[doc = "System Assigned Identity ObjectId."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The Azure Active Directory tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The resource ids of the user assigned identities to use"]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ManagedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_identity {
    use super::*;
    #[doc = "The identity type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        UserAssigned,
        SystemAssigned,
        #[serde(rename = "SystemAssigned,UserAssigned")]
        SystemAssignedUserAssigned,
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
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 1u32, "UserAssigned"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 2u32, "SystemAssigned"),
                Self::SystemAssignedUserAssigned => serializer.serialize_unit_variant("Type", 3u32, "SystemAssigned,UserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A SCOM instance resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of a SCOM instance resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitoringInstanceProperties>,
    #[doc = "Azure Active Directory identity configuration for a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedIdentity>,
}
impl ManagedInstance {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "Gets status of current and latest SCOM managed instance operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedInstanceOperationStatus {
    #[doc = "Operation Name"]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[doc = "Operation status"]
    #[serde(rename = "operationState", default, skip_serializing_if = "Option::is_none")]
    pub operation_state: Option<String>,
    #[doc = "Operation id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ManagedInstanceOperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of management server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementServerProperties {
    #[doc = "Management server Name"]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "Azure VM Resource Id of the Management server."]
    #[serde(rename = "vmResId", default, skip_serializing_if = "Option::is_none")]
    pub vm_res_id: Option<String>,
    #[doc = "Management server Fully Qualified Domain Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Represent whether the Server is a Management Server and/or Web Console Server."]
    #[serde(rename = "serverRoles", default, skip_serializing_if = "Option::is_none")]
    pub server_roles: Option<String>,
    #[doc = "Management server health state."]
    #[serde(rename = "healthState", default, skip_serializing_if = "Option::is_none")]
    pub health_state: Option<String>,
}
impl ManagementServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A monitored resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoredResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a monitored resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitoredResourceProperties>,
}
impl MonitoredResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a monitored resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoredResourceProperties {
    #[doc = "ArmId of the monitored resource."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Location of the monitored resource."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "ComputerName of the monitored resource."]
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[doc = "The domain name associated with the monitored resource."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "The management server endpoint to which the monitored resource is directed."]
    #[serde(rename = "managementServerEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub management_server_endpoint: Option<String>,
    #[doc = "The health status of the monitored resource."]
    #[serde(rename = "healthStatus", default, skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[doc = "The connection status of the monitored resource."]
    #[serde(rename = "connectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<String>,
    #[doc = "The version of the monitored resource agent version."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Install type of monitored resource."]
    #[serde(rename = "installType", default, skip_serializing_if = "Option::is_none")]
    pub install_type: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl MonitoredResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paginated list of SCOM monitored resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoredResources {
    #[doc = "The contents displayed on the page."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<MonitoredResource>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MonitoredResources {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl MonitoredResources {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A paged list of SCOM managed instances"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringInstanceList {
    #[doc = "The items on the page"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagedInstance>,
    #[doc = "URL to get the next page if any"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MonitoringInstanceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl MonitoringInstanceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object containing updates for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringInstancePatch {
    #[doc = "Azure Active Directory identity configuration for a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedIdentity>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl MonitoringInstancePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a SCOM instance resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringInstanceProperties {
    #[doc = "SCOM product version to be installed on instance"]
    #[serde(rename = "productVersion", default, skip_serializing_if = "Option::is_none")]
    pub product_version: Option<String>,
    #[doc = "Virtual Network subnet id on which Aquila instance will be provisioned"]
    #[serde(rename = "vNetSubnetId", default, skip_serializing_if = "Option::is_none")]
    pub v_net_subnet_id: Option<String>,
    #[doc = "List of management server endpoints"]
    #[serde(
        rename = "managementEndpoints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub management_endpoints: Vec<ManagementServerProperties>,
    #[doc = "The properties of database instance"]
    #[serde(rename = "databaseInstance", default, skip_serializing_if = "Option::is_none")]
    pub database_instance: Option<DatabaseInstanceProperties>,
    #[doc = "The properties of domain controller to which SCOM and SQL servers join for AuthN/AuthZ."]
    #[serde(rename = "domainController", default, skip_serializing_if = "Option::is_none")]
    pub domain_controller: Option<DomainControllerProperties>,
    #[doc = "Get Domain user name and password from key vault"]
    #[serde(rename = "domainUserCredentials", default, skip_serializing_if = "Option::is_none")]
    pub domain_user_credentials: Option<DomainUserCredentials>,
    #[doc = "Gmsa Details"]
    #[serde(rename = "gmsaDetails", default, skip_serializing_if = "Option::is_none")]
    pub gmsa_details: Option<GmsaDetails>,
    #[doc = "The properties to maximize savings by using Azure Hybrid Benefit"]
    #[serde(rename = "azureHybridBenefit", default, skip_serializing_if = "Option::is_none")]
    pub azure_hybrid_benefit: Option<AzureHybridBenefitProperties>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Configuration for Log Analytics linking to SCOM managed instance."]
    #[serde(rename = "logAnalyticsProperties", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_properties: Option<LogAnalyticsConfiguration>,
    #[doc = "Gets status of current and latest SCOM managed instance operations."]
    #[serde(
        rename = "operationsStatus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operations_status: Vec<ManagedInstanceOperationStatus>,
}
impl MonitoringInstanceProperties {
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
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
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
#[doc = "Properties for Scaling"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScalingProperties {
    #[doc = "Required management server count"]
    #[serde(rename = "serverCount", default, skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i64>,
}
impl ScalingProperties {
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
#[doc = "Azure Active Directory identity configuration for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserIdentity {
    #[doc = "The Azure Active Directory principal id."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The Azure Active Directory client id."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration for Log Analytics linking to SCOM managed instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogAnalyticsConfiguration {
    #[doc = "The resource ID of the Log Analytics workspace to be used."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The types of data to be ingested to Log Analytics workspace."]
    #[serde(
        rename = "dataTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_types: Vec<String>,
    #[doc = "A one-time optional parameter to import data of last 7 days."]
    #[serde(rename = "importData", default, skip_serializing_if = "Option::is_none")]
    pub import_data: Option<bool>,
}
impl LogAnalyticsConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Updated configuration for Log Analytics linking"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogAnalyticsUpdateConfiguration {
    #[doc = "The types of data to be ingested to Log Analytics workspace."]
    #[serde(
        rename = "dataTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_types: Vec<String>,
}
impl LogAnalyticsUpdateConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for patching servers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchServersResponseProperties {
    #[doc = "Status of the patch operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl PatchServersResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for set server count operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SetServerCountResponseProperties {
    #[doc = "Management server count"]
    #[serde(rename = "serverCount", default, skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i64>,
}
impl SetServerCountResponseProperties {
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
#[doc = "Properties for unlinking log analytics"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnlinkLogAnalyticsResponseProperties {
    #[doc = "Status of the unlink operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl UnlinkLogAnalyticsResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
