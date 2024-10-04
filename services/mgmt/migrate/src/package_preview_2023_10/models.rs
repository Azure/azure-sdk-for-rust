#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Application in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Application {
    #[doc = "Gets or sets Name of the Application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets Version of the Application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets or sets Provider of the Application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
impl Application {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains data related application and roles discovery scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationDiscovery {
    #[doc = "Application Discovery Scope Status"]
    #[serde(rename = "discoveryScopeStatus", default, skip_serializing_if = "Option::is_none")]
    pub discovery_scope_status: Option<ApplicationDiscoveryScopeStatus>,
    #[doc = "Gets errors for discovery scope."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "Gets the run as account ID with which feature worked successfully.\n           \nIt is discovered by the agent from the list of credentials."]
    #[serde(rename = "hydratedRunAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub hydrated_run_as_account_id: Option<String>,
}
impl ApplicationDiscovery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application Discovery Scope Status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApplicationDiscoveryScopeStatus")]
pub enum ApplicationDiscoveryScopeStatus {
    DiscoverySucceededAtleastOnce,
    DiscoveryFailed,
    RunAsAccountNotAssociated,
    DiscoveryNotStarted,
    DiscoveryInProgress,
    Disabled,
    DiscoveryPartiallySucceded,
    DiscoverySucceeded,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApplicationDiscoveryScopeStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApplicationDiscoveryScopeStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApplicationDiscoveryScopeStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DiscoverySucceededAtleastOnce => {
                serializer.serialize_unit_variant("ApplicationDiscoveryScopeStatus", 0u32, "DiscoverySucceededAtleastOnce")
            }
            Self::DiscoveryFailed => serializer.serialize_unit_variant("ApplicationDiscoveryScopeStatus", 1u32, "DiscoveryFailed"),
            Self::RunAsAccountNotAssociated => {
                serializer.serialize_unit_variant("ApplicationDiscoveryScopeStatus", 2u32, "RunAsAccountNotAssociated")
            }
            Self::DiscoveryNotStarted => serializer.serialize_unit_variant("ApplicationDiscoveryScopeStatus", 3u32, "DiscoveryNotStarted"),
            Self::DiscoveryInProgress => serializer.serialize_unit_variant("ApplicationDiscoveryScopeStatus", 4u32, "DiscoveryInProgress"),
            Self::Disabled => serializer.serialize_unit_variant("ApplicationDiscoveryScopeStatus", 5u32, "Disabled"),
            Self::DiscoveryPartiallySucceded => {
                serializer.serialize_unit_variant("ApplicationDiscoveryScopeStatus", 6u32, "DiscoveryPartiallySucceded")
            }
            Self::DiscoverySucceeded => serializer.serialize_unit_variant("ApplicationDiscoveryScopeStatus", 7u32, "DiscoverySucceeded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "AppsAndRoles in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppsAndRoles {
    #[doc = "Gets or sets Applications of the AppsAndRoles."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub applications: Vec<Application>,
    #[doc = "Gets or sets WebApplications of the AppsAndRoles."]
    #[serde(
        rename = "webApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub web_applications: Vec<WebApplicationAppsAndRolesModel>,
    #[doc = "Gets or sets Features of the AppsAndRoles."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub features: Vec<Feature>,
    #[doc = "Gets or sets SQLServers of the AppsAndRoles."]
    #[serde(
        rename = "sqlServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sql_servers: Vec<SqlServerApplication>,
    #[doc = "Gets or sets SharePointServers of the AppsAndRoles."]
    #[serde(
        rename = "sharePointServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub share_point_servers: Vec<SharePointServer>,
    #[doc = "Gets or sets SystemCenters of the AppsAndRoles."]
    #[serde(
        rename = "systemCenters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub system_centers: Vec<SystemCenter>,
    #[doc = "Gets or sets BizTalkServers of the AppsAndRoles."]
    #[serde(
        rename = "bizTalkServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub biz_talk_servers: Vec<BizTalkServer>,
    #[doc = "Gets or sets ExchangeServers of the AppsAndRoles."]
    #[serde(
        rename = "exchangeServers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exchange_servers: Vec<ExchangeServer>,
    #[doc = "Gets or sets OtherDatabaseServers of the AppsAndRoles."]
    #[serde(
        rename = "otherDatabases",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub other_databases: Vec<OtherDatabase>,
}
impl AppsAndRoles {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "object model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceManagerObject {}
impl AzureResourceManagerObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "BizTalkServer in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BizTalkServer {
    #[doc = "Gets or sets ProductName of the BizTalkServer."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "Gets or sets Status of the BizTalkServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl BizTalkServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The connectors of the web server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectorUnit {
    #[doc = "Gets or sets the bindings for the connector."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bindings: Vec<FrontEndBinding>,
}
impl ConnectorUnit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "HealthError Details Source"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Default")]
pub enum Default {
    #[serde(rename = "default")]
    Default,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Default {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Default {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Default {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Default => serializer.serialize_unit_variant("Default", 0u32, "default"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "HealthError Details Source"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DefaultValues")]
pub enum DefaultValues {
    #[serde(rename = "default")]
    Default,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DefaultValues {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DefaultValues {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DefaultValues {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Default => serializer.serialize_unit_variant("DefaultValues", 0u32, "default"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A host resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteImportMachinesJob {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Delete Imported Machines Job Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeleteImportedMachinesJobProperties>,
}
impl DeleteImportMachinesJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Delete import machines job collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteImportMachinesJobCollection {
    #[doc = "Gets the list of jobs."]
    pub value: Vec<DeleteImportMachinesJob>,
    #[doc = "Gets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DeleteImportMachinesJobCollection {
    pub fn new(value: Vec<DeleteImportMachinesJob>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Delete Imported Machines Job Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteImportedMachinesJobProperties {
    #[doc = "blob name"]
    #[serde(rename = "blobName", default, skip_serializing_if = "Option::is_none")]
    pub blob_name: Option<String>,
    #[doc = "error Sas Uri"]
    #[serde(rename = "errorSasUri", default, skip_serializing_if = "Option::is_none")]
    pub error_sas_uri: Option<String>,
    #[doc = "Cosmos db Imported Machines JobEntity"]
    #[serde(rename = "jobState", default, skip_serializing_if = "Option::is_none")]
    pub job_state: Option<DeleteImportedMachinesJobPropertiesJobState>,
    #[doc = "number Of Machines Deleted"]
    #[serde(rename = "numberOfMachinesDeleted", default, skip_serializing_if = "Option::is_none")]
    pub number_of_machines_deleted: Option<i32>,
    #[doc = "deletion Confirmation"]
    #[serde(rename = "deletionConfirmation", default, skip_serializing_if = "Option::is_none")]
    pub deletion_confirmation: Option<bool>,
    #[doc = "errors list"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<String>,
    #[doc = "Gets or sets the Job status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets the Job start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Gets or sets the Job end time."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Gets or sets the Display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl DeleteImportedMachinesJobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos db Imported Machines JobEntity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeleteImportedMachinesJobPropertiesJobState")]
pub enum DeleteImportedMachinesJobPropertiesJobState {
    Unknown,
    Verified,
    VerifiedWithErrors,
    Completed,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeleteImportedMachinesJobPropertiesJobState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeleteImportedMachinesJobPropertiesJobState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeleteImportedMachinesJobPropertiesJobState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("DeleteImportedMachinesJobPropertiesJobState", 0u32, "Unknown"),
            Self::Verified => serializer.serialize_unit_variant("DeleteImportedMachinesJobPropertiesJobState", 1u32, "Verified"),
            Self::VerifiedWithErrors => {
                serializer.serialize_unit_variant("DeleteImportedMachinesJobPropertiesJobState", 2u32, "VerifiedWithErrors")
            }
            Self::Completed => serializer.serialize_unit_variant("DeleteImportedMachinesJobPropertiesJobState", 3u32, "Completed"),
            Self::Failed => serializer.serialize_unit_variant("DeleteImportedMachinesJobPropertiesJobState", 4u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Contains data related dependency map discovery scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependencyMapDiscovery {
    #[doc = "DependencyMap DiscoveryScope Status"]
    #[serde(rename = "discoveryScopeStatus", default, skip_serializing_if = "Option::is_none")]
    pub discovery_scope_status: Option<DependencyMapDiscoveryScopeStatus>,
    #[doc = "Gets errors for discovery scope."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "Gets the run as account ID with which feature worked successfully.\n           \nIt is discovered by the agent from the list of credentials."]
    #[serde(rename = "hydratedRunAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub hydrated_run_as_account_id: Option<String>,
}
impl DependencyMapDiscovery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DependencyMap DiscoveryScope Status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DependencyMapDiscoveryScopeStatus")]
pub enum DependencyMapDiscoveryScopeStatus {
    DiscoverySucceededAtleastOnce,
    DiscoveryFailed,
    RunAsAccountNotAssociated,
    DiscoveryNotStarted,
    DiscoveryInProgress,
    Disabled,
    DiscoveryPartiallySucceded,
    DiscoverySucceeded,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DependencyMapDiscoveryScopeStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DependencyMapDiscoveryScopeStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DependencyMapDiscoveryScopeStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DiscoverySucceededAtleastOnce => {
                serializer.serialize_unit_variant("DependencyMapDiscoveryScopeStatus", 0u32, "DiscoverySucceededAtleastOnce")
            }
            Self::DiscoveryFailed => serializer.serialize_unit_variant("DependencyMapDiscoveryScopeStatus", 1u32, "DiscoveryFailed"),
            Self::RunAsAccountNotAssociated => {
                serializer.serialize_unit_variant("DependencyMapDiscoveryScopeStatus", 2u32, "RunAsAccountNotAssociated")
            }
            Self::DiscoveryNotStarted => {
                serializer.serialize_unit_variant("DependencyMapDiscoveryScopeStatus", 3u32, "DiscoveryNotStarted")
            }
            Self::DiscoveryInProgress => {
                serializer.serialize_unit_variant("DependencyMapDiscoveryScopeStatus", 4u32, "DiscoveryInProgress")
            }
            Self::Disabled => serializer.serialize_unit_variant("DependencyMapDiscoveryScopeStatus", 5u32, "Disabled"),
            Self::DiscoveryPartiallySucceded => {
                serializer.serialize_unit_variant("DependencyMapDiscoveryScopeStatus", 6u32, "DiscoveryPartiallySucceded")
            }
            Self::DiscoverySucceeded => serializer.serialize_unit_variant("DependencyMapDiscoveryScopeStatus", 7u32, "DiscoverySucceeded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Machine class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependencyMapMachineInput {
    #[doc = "Gets or sets the ARM id of the machine."]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "Gets or sets a value indicating whether\n            dependency mapping is to\nbe enabled or not."]
    #[serde(rename = "isDependencyMapToBeEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_dependency_map_to_be_enabled: Option<bool>,
}
impl DependencyMapMachineInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DependencyMap ServiceMap extensions Client GroupMembers Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependencyMapServiceMapextensionsClientGroupMembersRequest {
    #[doc = "id of machine"]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "process group name "]
    #[serde(rename = "processGroupName", default, skip_serializing_if = "Option::is_none")]
    pub process_group_name: Option<String>,
    #[doc = "name of process"]
    #[serde(rename = "processName", default, skip_serializing_if = "Option::is_none")]
    pub process_name: Option<String>,
    #[doc = "start time"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "end time"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "DependencyMap Service Map extensions Dependency Map Request Filters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<DependencyMapServiceMapextensionsDependencyMapRequestFilters>,
}
impl DependencyMapServiceMapextensionsClientGroupMembersRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DependencyMap Service Map extensions Dependency Map Request Filters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependencyMapServiceMapextensionsDependencyMapRequestFilters {
    #[doc = "array of machine ids"]
    #[serde(
        rename = "machineIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machine_ids: Vec<String>,
    #[doc = "array of process Ids"]
    #[serde(
        rename = "processIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub process_ids: Vec<String>,
}
impl DependencyMapServiceMapextensionsDependencyMapRequestFilters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DependencyMap ServiceMap extensions ExportDependencies Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependencyMapServiceMapextensionsExportDependenciesRequest {
    #[doc = "start time"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "end time"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl DependencyMapServiceMapextensionsExportDependenciesRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DependencyMap ServiceMapextensions Scope MapRequest"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependencyMapServiceMapextensionsScopeMapRequest {
    #[doc = "start time"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "end time"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "DependencyMap Service Map extensions Dependency Map Request Filters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<DependencyMapServiceMapextensionsDependencyMapRequestFilters>,
}
impl DependencyMapServiceMapextensionsScopeMapRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DependencyMap ServiceMap extensions ServerGroup Members Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependencyMapServiceMapextensionsServerGroupMembersRequest {
    #[doc = "port of server"]
    #[serde(rename = "serverPort", default, skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i32>,
    #[doc = "start time"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "end time"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "DependencyMap Service Map extensions Dependency Map Request Filters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<DependencyMapServiceMapextensionsDependencyMapRequestFilters>,
}
impl DependencyMapServiceMapextensionsServerGroupMembersRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DependencyMap ServiceMap extensions SingleMachine DetailedMap Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependencyMapServiceMapextensionsSingleMachineDetailedMapRequest {
    #[doc = "id of machine"]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "start time"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "end time"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "DependencyMap Service Map extensions Dependency Map Request Filters"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<DependencyMapServiceMapextensionsDependencyMapRequestFilters>,
}
impl DependencyMapServiceMapextensionsSingleMachineDetailedMapRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Second level entity for virtual directories."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DirectoryPath {
    #[doc = "Gets or sets the virtual path for the directory."]
    #[serde(rename = "virtual", default, skip_serializing_if = "Option::is_none")]
    pub virtual_: Option<String>,
    #[doc = "Gets or sets the physical path of the directory on the web server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical: Option<String>,
}
impl DirectoryPath {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The site error summary model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoveryScopeErrorSummary {
    #[doc = "Gets or sets the affected resource type."]
    #[serde(rename = "affectedResourceType")]
    pub affected_resource_type: String,
    #[doc = "Count of affected objects."]
    #[serde(rename = "affectedObjectsCount")]
    pub affected_objects_count: i64,
    #[doc = "Discovery scopes"]
    #[serde(rename = "discoveryScope")]
    pub discovery_scope: DiscoveryScopes,
}
impl DiscoveryScopeErrorSummary {
    pub fn new(affected_resource_type: String, affected_objects_count: i64, discovery_scope: DiscoveryScopes) -> Self {
        Self {
            affected_resource_type,
            affected_objects_count,
            discovery_scope,
        }
    }
}
#[doc = "Discovery Scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiscoveryScopeStatus")]
pub enum DiscoveryScopeStatus {
    DiscoverySucceededAtleastOnce,
    DiscoveryFailed,
    RunAsAccountNotAssociated,
    DiscoveryNotStarted,
    DiscoveryInProgress,
    Disabled,
    DiscoveryPartiallySucceded,
    DiscoverySucceeded,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DiscoveryScopeStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DiscoveryScopeStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DiscoveryScopeStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DiscoverySucceededAtleastOnce => {
                serializer.serialize_unit_variant("DiscoveryScopeStatus", 0u32, "DiscoverySucceededAtleastOnce")
            }
            Self::DiscoveryFailed => serializer.serialize_unit_variant("DiscoveryScopeStatus", 1u32, "DiscoveryFailed"),
            Self::RunAsAccountNotAssociated => serializer.serialize_unit_variant("DiscoveryScopeStatus", 2u32, "RunAsAccountNotAssociated"),
            Self::DiscoveryNotStarted => serializer.serialize_unit_variant("DiscoveryScopeStatus", 3u32, "DiscoveryNotStarted"),
            Self::DiscoveryInProgress => serializer.serialize_unit_variant("DiscoveryScopeStatus", 4u32, "DiscoveryInProgress"),
            Self::Disabled => serializer.serialize_unit_variant("DiscoveryScopeStatus", 5u32, "Disabled"),
            Self::DiscoveryPartiallySucceded => {
                serializer.serialize_unit_variant("DiscoveryScopeStatus", 6u32, "DiscoveryPartiallySucceded")
            }
            Self::DiscoverySucceeded => serializer.serialize_unit_variant("DiscoveryScopeStatus", 7u32, "DiscoverySucceeded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Discovery scopes"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiscoveryScopes")]
pub enum DiscoveryScopes {
    AppsAndRoles,
    DependencyMap,
    StaticData,
    #[serde(rename = "SQLServerConnectionInfo")]
    SqlServerConnectionInfo,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DiscoveryScopes {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DiscoveryScopes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DiscoveryScopes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AppsAndRoles => serializer.serialize_unit_variant("DiscoveryScopes", 0u32, "AppsAndRoles"),
            Self::DependencyMap => serializer.serialize_unit_variant("DiscoveryScopes", 1u32, "DependencyMap"),
            Self::StaticData => serializer.serialize_unit_variant("DiscoveryScopes", 2u32, "StaticData"),
            Self::SqlServerConnectionInfo => serializer.serialize_unit_variant("DiscoveryScopes", 3u32, "SQLServerConnectionInfo"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Web app data source web model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscoverySiteDataSource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Discovery site data source properties class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiscoverySiteDataSourceProperties>,
}
impl DiscoverySiteDataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a DiscoverySiteDataSource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoverySiteDataSourceListResult {
    #[doc = "The DiscoverySiteDataSource items on this page"]
    pub value: Vec<DiscoverySiteDataSource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiscoverySiteDataSourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DiscoverySiteDataSourceListResult {
    pub fn new(value: Vec<DiscoverySiteDataSource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Discovery site data source properties class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscoverySiteDataSourceProperties {
    #[doc = "Gets or sets the discovery site Id."]
    #[serde(rename = "discoverySiteId", default, skip_serializing_if = "Option::is_none")]
    pub discovery_site_id: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl DiscoverySiteDataSourceProperties {
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
#[doc = "Error contract returned when some exception occurs in Rest API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Gets the error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Gets the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Gets the possible causes of error."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "Gets the recommended action to resolve error."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "Gets the error severity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "Gets a value indicating whether the error originated from a agent or not."]
    #[serde(rename = "isAgentReportedError", default, skip_serializing_if = "Option::is_none")]
    pub is_agent_reported_error: Option<bool>,
    #[doc = "Gets the agent error code."]
    #[serde(rename = "agentErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub agent_error_code: Option<String>,
    #[doc = "Gets the error message from the agent."]
    #[serde(rename = "agentErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub agent_error_message: Option<String>,
    #[doc = "Gets possible causes for the agent error."]
    #[serde(rename = "agentErrorPossibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub agent_error_possible_causes: Option<String>,
    #[doc = "Gets the recommended action for the agent error."]
    #[serde(rename = "agentErrorRecommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub agent_error_recommended_action: Option<String>,
}
impl ErrorDetails {
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
#[doc = "ErrorSummaryRequest body"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorSummaryRequest {
    #[doc = "Gets or sets the Appliance Name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
}
impl ErrorSummaryRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a server machine tracked resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Errors {
    #[doc = "Gets the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Gets the message parameters."]
    #[serde(rename = "messageParameters", default, skip_serializing_if = "Option::is_none")]
    pub message_parameters: Option<serde_json::Value>,
    #[doc = "Gets the appliance name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
    #[doc = "Gets the error ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[doc = "Gets the error name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Gets the possible causes of error."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "Gets the recommended action to resolve error."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "Gets the error severity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "Gets the error summary message."]
    #[serde(rename = "summaryMessage", default, skip_serializing_if = "Option::is_none")]
    pub summary_message: Option<String>,
    #[doc = "HealthError Details Source"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<MicrosoftAzureFdsWebRoleHealthErrorDetailsSource>,
    #[doc = "Gets the time stamp when the error was updated."]
    #[serde(rename = "updatedTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_time_stamp: Option<time::OffsetDateTime>,
    #[doc = "Gets run as account id used while performing discovery             of entity."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Gets discovery scope for which             error is encountered."]
    #[serde(rename = "discoveryScope", default, skip_serializing_if = "Option::is_none")]
    pub discovery_scope: Option<HealthErrorDetailsDiscoveryScope>,
}
impl Errors {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ExchangeServer in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExchangeServer {
    #[doc = "Gets or sets ProductName of the ExchangeServer."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "Gets or sets Edition of the ExchangeServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[doc = "Gets or sets Roles of the ExchangeServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<String>,
    #[doc = "Gets or sets ServicePack of the ExchangeServer."]
    #[serde(rename = "servicePack", default, skip_serializing_if = "Option::is_none")]
    pub service_pack: Option<String>,
    #[doc = "Gets or sets Version of the ExchangeServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ExchangeServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Export machines job REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportImportedMachinesJob {
    #[doc = "Type name for export job."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = " Export Imported Machines JobEntity Properties "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExportImportedMachinesJobEntityProperties>,
    #[doc = "Gets or sets the relative ARM name to get job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the Job ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the Job status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets the Job start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Gets or sets the Job end time."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Gets or sets the Display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl ExportImportedMachinesJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = " Export Imported Machines JobEntity Properties "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportImportedMachinesJobEntityProperties {
    #[doc = "blob name"]
    #[serde(rename = "blobName", default, skip_serializing_if = "Option::is_none")]
    pub blob_name: Option<String>,
    #[doc = "sas uri"]
    #[serde(rename = "sasUri", default, skip_serializing_if = "Option::is_none")]
    pub sas_uri: Option<String>,
}
impl ExportImportedMachinesJobEntityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Export Machine Errors Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ExportMachineErrorsProperties")]
pub enum ExportMachineErrorsProperties {
    AppsAndRoles,
    DependencyMap,
    StaticData,
    #[serde(rename = "SQLServerConnectionInfo")]
    SqlServerConnectionInfo,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ExportMachineErrorsProperties {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ExportMachineErrorsProperties {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ExportMachineErrorsProperties {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AppsAndRoles => serializer.serialize_unit_variant("ExportMachineErrorsProperties", 0u32, "AppsAndRoles"),
            Self::DependencyMap => serializer.serialize_unit_variant("ExportMachineErrorsProperties", 1u32, "DependencyMap"),
            Self::StaticData => serializer.serialize_unit_variant("ExportMachineErrorsProperties", 2u32, "StaticData"),
            Self::SqlServerConnectionInfo => {
                serializer.serialize_unit_variant("ExportMachineErrorsProperties", 3u32, "SQLServerConnectionInfo")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The body of export machine errors request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportMachineErrorsRequest {
    #[doc = "The Properties class for export machine errors request body."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RequestExportMachineErrorsProperties>,
}
impl ExportMachineErrorsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ExportMachinesRequest body"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportMachinesRequest {
    #[doc = "filter options."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}
impl ExportMachinesRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ExportSqlServerRequest body"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportSqlServersRequest {
    #[doc = "Gets or sets the Appliance Name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
    #[doc = "filter options."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}
impl ExportSqlServersRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ExportWebAppsRequest body"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportWebAppsRequest {
    #[doc = "filter options."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}
impl ExportWebAppsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "fci instance state "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FciInstanceState")]
pub enum FciInstanceState {
    Unknown,
    Inherited,
    Initializing,
    Online,
    Offline,
    Failed,
    Pending,
    OnlinePending,
    OfflinePending,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FciInstanceState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FciInstanceState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FciInstanceState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("FciInstanceState", 0u32, "Unknown"),
            Self::Inherited => serializer.serialize_unit_variant("FciInstanceState", 1u32, "Inherited"),
            Self::Initializing => serializer.serialize_unit_variant("FciInstanceState", 2u32, "Initializing"),
            Self::Online => serializer.serialize_unit_variant("FciInstanceState", 3u32, "Online"),
            Self::Offline => serializer.serialize_unit_variant("FciInstanceState", 4u32, "Offline"),
            Self::Failed => serializer.serialize_unit_variant("FciInstanceState", 5u32, "Failed"),
            Self::Pending => serializer.serialize_unit_variant("FciInstanceState", 6u32, "Pending"),
            Self::OnlinePending => serializer.serialize_unit_variant("FciInstanceState", 7u32, "OnlinePending"),
            Self::OfflinePending => serializer.serialize_unit_variant("FciInstanceState", 8u32, "OfflinePending"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Feature in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Feature {
    #[doc = "Gets or sets Name of the Feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets FeatureType of the Feature."]
    #[serde(rename = "featureType", default, skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<String>,
    #[doc = "Gets or sets Parent of the Feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[doc = "Gets or sets Status of the Feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl Feature {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "File Metadata web model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileMetaData {
    #[doc = "Gets or sets the logical name of the file."]
    #[serde(rename = "logicalName", default, skip_serializing_if = "Option::is_none")]
    pub logical_name: Option<String>,
    #[doc = "Gets or sets the operating-system full path of the file."]
    #[serde(rename = "physicalFullName", default, skip_serializing_if = "Option::is_none")]
    pub physical_full_name: Option<String>,
    #[doc = "file type"]
    #[serde(rename = "fileType", default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<FileType>,
    #[doc = "Gets or sets the size of the file in MB."]
    #[serde(rename = "sizeInMb", default, skip_serializing_if = "Option::is_none")]
    pub size_in_mb: Option<f32>,
    #[doc = "Gets or sets a value indicating whether memory optimized data option is enabled."]
    #[serde(rename = "isMemoryOptimizedDataOptionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_memory_optimized_data_option_enabled: Option<bool>,
}
impl FileMetaData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "file type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FileType")]
pub enum FileType {
    Rows,
    Log,
    Filestream,
    NotSupported,
    Fulltext,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FileType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FileType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FileType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Rows => serializer.serialize_unit_variant("FileType", 0u32, "Rows"),
            Self::Log => serializer.serialize_unit_variant("FileType", 1u32, "Log"),
            Self::Filestream => serializer.serialize_unit_variant("FileType", 2u32, "Filestream"),
            Self::NotSupported => serializer.serialize_unit_variant("FileType", 3u32, "NotSupported"),
            Self::Fulltext => serializer.serialize_unit_variant("FileType", 4u32, "Fulltext"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Frontend bindings for a web application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FrontEndBinding {
    #[doc = "Gets or sets the Binding protocol."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[doc = "Gets or sets the Host name."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "Gets or sets the Port number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[doc = "Gets or sets the IP Address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
impl FrontEndBinding {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Second level object.  Data related to a machine's operating system.             Serialized and stored as part of Machine Rest object. "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GuestOsDetails {
    #[doc = "Gets or sets the type of the operating system."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Gets or sets the Name of the operating system."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "Gets or sets the Version of the operating system."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "Gets or sets the Architecture of the operating system."]
    #[serde(rename = "osArchitecture", default, skip_serializing_if = "Option::is_none")]
    pub os_architecture: Option<String>,
}
impl GuestOsDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error contract returned when some exception occurs in Rest API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthErrorDetails {
    #[doc = "Gets the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Gets the message parameters."]
    #[serde(rename = "messageParameters", default, skip_serializing_if = "Option::is_none")]
    pub message_parameters: Option<serde_json::Value>,
    #[doc = "Gets the appliance name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
    #[doc = "Gets the error ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Gets the error name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Gets the possible causes of error."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "Gets the recommended action to resolve error."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "Gets the error severity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "Gets the error summary message."]
    #[serde(rename = "summaryMessage", default, skip_serializing_if = "Option::is_none")]
    pub summary_message: Option<String>,
    #[doc = "HealthError Details Source"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<HealthErrorDetailsSource>,
    #[doc = "Gets the time stamp when the error was updated."]
    #[serde(rename = "updatedTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub updated_time_stamp: Option<time::OffsetDateTime>,
    #[doc = "Gets run as account id used while performing discovery             of entity."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Gets discovery scope for which             error is encountered."]
    #[serde(rename = "discoveryScope", default, skip_serializing_if = "Option::is_none")]
    pub discovery_scope: Option<HealthErrorDetailsDiscoveryScope>,
}
impl HealthErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets discovery scope for which             error is encountered."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HealthErrorDetailsDiscoveryScope")]
pub enum HealthErrorDetailsDiscoveryScope {
    AppsAndRoles,
    DependencyMap,
    StaticData,
    #[serde(rename = "SQLServerConnectionInfo")]
    SqlServerConnectionInfo,
    DiscoveryTargets,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HealthErrorDetailsDiscoveryScope {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HealthErrorDetailsDiscoveryScope {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HealthErrorDetailsDiscoveryScope {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AppsAndRoles => serializer.serialize_unit_variant("HealthErrorDetailsDiscoveryScope", 0u32, "AppsAndRoles"),
            Self::DependencyMap => serializer.serialize_unit_variant("HealthErrorDetailsDiscoveryScope", 1u32, "DependencyMap"),
            Self::StaticData => serializer.serialize_unit_variant("HealthErrorDetailsDiscoveryScope", 2u32, "StaticData"),
            Self::SqlServerConnectionInfo => {
                serializer.serialize_unit_variant("HealthErrorDetailsDiscoveryScope", 3u32, "SQLServerConnectionInfo")
            }
            Self::DiscoveryTargets => serializer.serialize_unit_variant("HealthErrorDetailsDiscoveryScope", 4u32, "DiscoveryTargets"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "HealthError Details Source"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HealthErrorDetailsSource")]
pub enum HealthErrorDetailsSource {
    RefreshFabricLayout,
    RefreshFabricLayoutGuest,
    RefreshFabricLayoutDependencyMap,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HealthErrorDetailsSource {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HealthErrorDetailsSource {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HealthErrorDetailsSource {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::RefreshFabricLayout => serializer.serialize_unit_variant("HealthErrorDetailsSource", 0u32, "RefreshFabricLayout"),
            Self::RefreshFabricLayoutGuest => {
                serializer.serialize_unit_variant("HealthErrorDetailsSource", 1u32, "RefreshFabricLayoutGuest")
            }
            Self::RefreshFabricLayoutDependencyMap => {
                serializer.serialize_unit_variant("HealthErrorDetailsSource", 2u32, "RefreshFabricLayoutDependencyMap")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Value indicating whether the VM is highly available"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HighAvailability")]
pub enum HighAvailability {
    Unknown,
    No,
    Yes,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HighAvailability {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HighAvailability {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HighAvailability {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("HighAvailability", 0u32, "Unknown"),
            Self::No => serializer.serialize_unit_variant("HighAvailability", 1u32, "No"),
            Self::Yes => serializer.serialize_unit_variant("HighAvailability", 2u32, "Yes"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Value indicating whether the VM is highly available"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HighAvailabilityValues")]
pub enum HighAvailabilityValues {
    Unknown,
    No,
    Yes,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HighAvailabilityValues {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HighAvailabilityValues {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HighAvailabilityValues {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("HighAvailabilityValues", 0u32, "Unknown"),
            Self::No => serializer.serialize_unit_variant("HighAvailabilityValues", 1u32, "No"),
            Self::Yes => serializer.serialize_unit_variant("HighAvailabilityValues", 2u32, "Yes"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A cluster resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervCluster {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of Hyperv Cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HypervClusterProperties>,
}
impl HypervCluster {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a HypervCluster list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HypervClusterListResult {
    #[doc = "The HypervCluster items on this page"]
    pub value: Vec<HypervCluster>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HypervClusterListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HypervClusterListResult {
    pub fn new(value: Vec<HypervCluster>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of Hyperv Cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervClusterProperties {
    #[doc = "Gets the timestamp marking Hyper-V cluster creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets the timestamp marking last updated on the Hyper-V cluster."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "Gets or sets the FQDN/IPAddress of the Hyper-V cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Gets the functional level of the Hyper-V cluster."]
    #[serde(rename = "functionalLevel", default, skip_serializing_if = "Option::is_none")]
    pub functional_level: Option<i32>,
    #[doc = "Gets the status of the Hyper-V cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets Run as account ID of the Hyper-V cluster."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Gets or sets list of hosts (FQDN) currently being tracked by the cluster."]
    #[serde(
        rename = "hostFqdnList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub host_fqdn_list: Vec<String>,
    #[doc = "Gets the errors."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl HypervClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Second level object returned as part of Machine REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervDisk {
    #[doc = "Id of the disk."]
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[doc = "VHD Id of the disk."]
    #[serde(rename = "vhdId", default, skip_serializing_if = "Option::is_none")]
    pub vhd_id: Option<String>,
    #[doc = "Gets or sets Bytes allocated for the disk."]
    #[serde(rename = "maxSizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_in_bytes: Option<i64>,
    #[doc = "Gets or sets Name of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets Type of the disk."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
    #[doc = "Gets or sets LUN of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[doc = "Gets or sets Path of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl HypervDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A host resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervHost {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of Hyperv Host"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HypervHostProperties>,
}
impl HypervHost {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a HypervHost list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HypervHostListResult {
    #[doc = "The HypervHost items on this page"]
    pub value: Vec<HypervHost>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HypervHostListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HypervHostListResult {
    pub fn new(value: Vec<HypervHost>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of Hyperv Host"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervHostProperties {
    #[doc = "Gets the timestamp marking Hyper-V host creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets the timestamp marking last updated on the Hyper-V host."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "Gets or sets the FQDN/IPAddress of the Hyper-V host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Gets or sets the run as account ID of the Hyper-V host."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Gets the version of the Hyper-V host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets the errors."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl HypervHostProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A job resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervJob {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Job REST Resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
impl HypervJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a HypervJob list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HypervJobListResult {
    #[doc = "The HypervJob items on this page"]
    pub value: Vec<HypervJob>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HypervJobListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HypervJobListResult {
    pub fn new(value: Vec<HypervJob>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A machine resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervMachine {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of AddressResource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HypervMachineProperties>,
}
impl HypervMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a HypervMachine list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HypervMachineListResult {
    #[doc = "The HypervMachine items on this page"]
    pub value: Vec<HypervMachine>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HypervMachineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HypervMachineListResult {
    pub fn new(value: Vec<HypervMachine>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of AddressResource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervMachineProperties {
    #[doc = "On-premise Instance UUID of the machine."]
    #[serde(rename = "instanceUuid", default, skip_serializing_if = "Option::is_none")]
    pub instance_uuid: Option<String>,
    #[doc = "Host FQDN/IPAddress."]
    #[serde(rename = "hostFqdn", default, skip_serializing_if = "Option::is_none")]
    pub host_fqdn: Option<String>,
    #[doc = "Host ARM ID."]
    #[serde(rename = "hostId", default, skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,
    #[doc = "Generation of the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<i32>,
    #[doc = "VM version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Value indicating whether the VM is highly available"]
    #[serde(rename = "highAvailability", default, skip_serializing_if = "Option::is_none")]
    pub high_availability: Option<HighAvailability>,
    #[doc = "Cluster FQDN/IPAddress."]
    #[serde(rename = "clusterFqdn", default, skip_serializing_if = "Option::is_none")]
    pub cluster_fqdn: Option<String>,
    #[doc = "Cluster ARM ID."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "Max memory of the virtual machine in MB."]
    #[serde(rename = "maxMemoryMb", default, skip_serializing_if = "Option::is_none")]
    pub max_memory_mb: Option<i32>,
    #[doc = "Value indicating whether dynamic memory is enabled for the VM."]
    #[serde(rename = "isDynamicMemoryEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_dynamic_memory_enabled: Option<bool>,
    #[doc = "Disks attached to the machine."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disks: Vec<HypervDisk>,
    #[doc = "Network adapters attached to the machine."]
    #[serde(
        rename = "networkAdapters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_adapters: Vec<HypervNetworkAdapter>,
    #[doc = "Management server type captured as a string representation of the\n           \n{Microsoft.Azure.FDS.WebRole.HyperVMachineBase.HyperVMachineBaseProperties.ManagementServerType}\nenumeration."]
    #[serde(rename = "managementServerType", default, skip_serializing_if = "Option::is_none")]
    pub management_server_type: Option<String>,
    #[doc = "Gets or sets the SecureBootTemplateId setting of the VM."]
    #[serde(rename = "secureBootTemplateId", default, skip_serializing_if = "Option::is_none")]
    pub secure_boot_template_id: Option<String>,
    #[doc = "Gets or sets a value indicating whether Secure boot is enabled for the VM."]
    #[serde(rename = "secureBootEnabled", default, skip_serializing_if = "Option::is_none")]
    pub secure_boot_enabled: Option<bool>,
    #[doc = "Gets or sets the SecureBootTemplateId setting of the VM."]
    #[serde(rename = "secureBootTemplate", default, skip_serializing_if = "Option::is_none")]
    pub secure_boot_template: Option<String>,
    #[doc = "Gets or sets a value indicating whether trusted platform module is enabled on\nthe VM."]
    #[serde(rename = "tpmEnabled", default, skip_serializing_if = "Option::is_none")]
    pub tpm_enabled: Option<bool>,
    #[doc = "Gets or sets a value indicating whether key storage device is enabled on the VM."]
    #[serde(rename = "ksdEnabled", default, skip_serializing_if = "Option::is_none")]
    pub ksd_enabled: Option<bool>,
    #[doc = "Gets or sets a value indicating whether shielding is enabled for the VM."]
    #[serde(rename = "shieldingRequested", default, skip_serializing_if = "Option::is_none")]
    pub shielding_requested: Option<bool>,
    #[doc = "Gets or sets a value indicating whether data protection is requested for a VM."]
    #[serde(rename = "dataProtectionRequested", default, skip_serializing_if = "Option::is_none")]
    pub data_protection_requested: Option<bool>,
    #[doc = "Gets or sets a value indicating whether encryption of state and migration\ntraffic is enabled for the VM."]
    #[serde(rename = "encryptStateAndVmMigrationTraffic", default, skip_serializing_if = "Option::is_none")]
    pub encrypt_state_and_vm_migration_traffic: Option<bool>,
    #[doc = "Gets or sets a value indicating whether VM virtualization based security is\nenabled for the VM."]
    #[serde(rename = "virtualizationBasedSecurityOptOut", default, skip_serializing_if = "Option::is_none")]
    pub virtualization_based_security_opt_out: Option<bool>,
    #[doc = "Gets the Machine power status."]
    #[serde(rename = "powerStatus", default, skip_serializing_if = "Option::is_none")]
    pub power_status: Option<String>,
    #[doc = "Gets the VM FQDN."]
    #[serde(rename = "vmFqdn", default, skip_serializing_if = "Option::is_none")]
    pub vm_fqdn: Option<String>,
    #[doc = "Gets the Root location of the VM configuration file."]
    #[serde(rename = "vmConfigurationFileLocation", default, skip_serializing_if = "Option::is_none")]
    pub vm_configuration_file_location: Option<String>,
    #[doc = "Gets or sets the firmware."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    #[doc = "Second level object.  Data related to a machine's operating system.             Serialized and stored as part of Machine Rest object. "]
    #[serde(rename = "guestOsDetails", default, skip_serializing_if = "Option::is_none")]
    pub guest_os_details: Option<GuestOsDetails>,
    #[doc = "Number of applications installed in the guest VM."]
    #[serde(rename = "numberOfApplications", default, skip_serializing_if = "Option::is_none")]
    pub number_of_applications: Option<i32>,
    #[doc = "The last time at which the Guest Details was discovered\n            or the\nerror while discovering guest details based discovery\n            of the\nmachine."]
    #[serde(rename = "guestDetailsDiscoveryTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub guest_details_discovery_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Whether Refresh Fabric Layout Guest Details has been completed once.\n         \n  Portal will show discovery in progress, if this value is true."]
    #[serde(rename = "isGuestDetailsDiscoveryInProgress", default, skip_serializing_if = "Option::is_none")]
    pub is_guest_details_discovery_in_progress: Option<bool>,
    #[doc = "Gets or sets if dependency mapping feature is enabled or not\n            for\nthe VM."]
    #[serde(rename = "dependencyMapping", default, skip_serializing_if = "Option::is_none")]
    pub dependency_mapping: Option<String>,
    #[doc = "Gets or sets when dependency mapping collection is last started."]
    #[serde(rename = "dependencyMappingStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub dependency_mapping_start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets when dependency mapping collection was last disabled."]
    #[serde(rename = "dependencyMappingEndTime", default, skip_serializing_if = "Option::is_none")]
    pub dependency_mapping_end_time: Option<String>,
    #[doc = "Gets or sets the run as account ID of the machine."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Gets the appliance names."]
    #[serde(
        rename = "applianceNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub appliance_names: Vec<String>,
    #[doc = "Errors for machine."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "Contains data related application and roles discovery scope."]
    #[serde(rename = "applicationDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub application_discovery: Option<ApplicationDiscovery>,
    #[doc = "Contains data related dependency map discovery scope."]
    #[serde(rename = "dependencyMapDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub dependency_map_discovery: Option<DependencyMapDiscovery>,
    #[doc = "Contains data related static data discovery scope."]
    #[serde(rename = "staticDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub static_discovery: Option<StaticDiscovery>,
    #[doc = "Contains data related SQL discovery."]
    #[serde(rename = "sqlDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub sql_discovery: Option<SqlDiscovery>,
    #[doc = "Data related to a machine's WebApps discovery."]
    #[serde(rename = "webAppDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub web_app_discovery: Option<WebAppDiscovery>,
    #[doc = "Data related to a machine's Oracle discovery."]
    #[serde(rename = "oracleDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub oracle_discovery: Option<OracleDiscovery>,
    #[doc = "Data related to a machine's spring boot discovery."]
    #[serde(rename = "springBootDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub spring_boot_discovery: Option<SpringBootDiscovery>,
    #[doc = "Data related to a machine's WebApps discovery."]
    #[serde(rename = "iisDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub iis_discovery: Option<WebAppDiscovery>,
    #[doc = "Data related to a machine's WebApps discovery."]
    #[serde(rename = "tomcatDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub tomcat_discovery: Option<WebAppDiscovery>,
    #[doc = "AppsAndRoles in the guest virtual machine."]
    #[serde(rename = "appsAndRoles", default, skip_serializing_if = "Option::is_none")]
    pub apps_and_roles: Option<AppsAndRoles>,
    #[doc = "product support status."]
    #[serde(rename = "productSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_support_status: Option<ProductSupportStatus>,
    #[doc = "Gets or sets the Number of Processor Cores \n            allocated for the\nmachine."]
    #[serde(rename = "numberOfProcessorCore", default, skip_serializing_if = "Option::is_none")]
    pub number_of_processor_core: Option<i32>,
    #[doc = "Gets or sets the allocated Memory in MB."]
    #[serde(rename = "allocatedMemoryInMb", default, skip_serializing_if = "Option::is_none")]
    pub allocated_memory_in_mb: Option<f64>,
    #[doc = "Second level object returned as part of Machine REST resource."]
    #[serde(rename = "operatingSystemDetails", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_details: Option<OperatingSystem>,
    #[doc = "Gets or sets the Machine BIOS serial number."]
    #[serde(rename = "biosSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub bios_serial_number: Option<String>,
    #[doc = "Gets or sets the BIOS GUID."]
    #[serde(rename = "biosGuid", default, skip_serializing_if = "Option::is_none")]
    pub bios_guid: Option<String>,
    #[doc = "Gets the Display name of the machine."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Value indicating whether VM is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Gets the marking machine creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets or sets tags on the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets the timestamp marking last updated on the machine."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl HypervMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for update operations of the HypervMachine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervMachineUpdate {
    #[doc = "The updatable properties of the HypervMachine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HypervMachineUpdateProperties>,
}
impl HypervMachineUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the HypervMachine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervMachineUpdateProperties {
    #[doc = "Gets or sets the firmware."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    #[doc = "Gets or sets the run as account ID of the machine."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "product support status."]
    #[serde(rename = "productSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_support_status: Option<ProductSupportStatus>,
    #[doc = "Gets or sets the Number of Processor Cores \n            allocated for the\nmachine."]
    #[serde(rename = "numberOfProcessorCore", default, skip_serializing_if = "Option::is_none")]
    pub number_of_processor_core: Option<i32>,
    #[doc = "Gets or sets the allocated Memory in MB."]
    #[serde(rename = "allocatedMemoryInMb", default, skip_serializing_if = "Option::is_none")]
    pub allocated_memory_in_mb: Option<f64>,
    #[doc = "Second level object returned as part of Machine REST resource."]
    #[serde(rename = "operatingSystemDetails", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_details: Option<OperatingSystem>,
    #[doc = "Gets or sets the Machine BIOS serial number."]
    #[serde(rename = "biosSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub bios_serial_number: Option<String>,
    #[doc = "Gets or sets the BIOS GUID."]
    #[serde(rename = "biosGuid", default, skip_serializing_if = "Option::is_none")]
    pub bios_guid: Option<String>,
    #[doc = "Gets or sets tags on the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl HypervMachineUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Second level object represented in responses as part of Machine REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervNetworkAdapter {
    #[doc = "Network Id."]
    #[serde(rename = "networkId", default, skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[doc = "Name of the VM subnet within the virtual network the NIC is attached to."]
    #[serde(rename = "subnetName", default, skip_serializing_if = "Option::is_none")]
    pub subnet_name: Option<String>,
    #[doc = "Static IP address."]
    #[serde(rename = "staticIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub static_ip_address: Option<String>,
    #[doc = "Mac address of the NIC."]
    #[serde(rename = "nicType", default, skip_serializing_if = "Option::is_none")]
    pub nic_type: Option<String>,
    #[doc = "Gets or sets the NIC Id."]
    #[serde(rename = "nicId", default, skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    #[doc = "Gets or sets Mac address of the NIC."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "Gets IP addresses for the machine."]
    #[serde(
        rename = "ipAddressList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_address_list: Vec<String>,
    #[doc = "Gets or sets Network Name."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "Gets or sets Type of the IP address."]
    #[serde(rename = "ipAddressType", default, skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
}
impl HypervNetworkAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A cluster resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervOperationsStatusResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of Hyperv Cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HypervClusterProperties>,
}
impl HypervOperationsStatusResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A machine resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervRunAsAccountResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for run as account properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunAsAccountProperties>,
}
impl HypervRunAsAccountResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a HypervRunAsAccountResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HypervRunAsAccountResourceListResult {
    #[doc = "The HypervRunAsAccountResource items on this page"]
    pub value: Vec<HypervRunAsAccountResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HypervRunAsAccountResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HypervRunAsAccountResourceListResult {
    pub fn new(value: Vec<HypervRunAsAccountResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A HyperV SiteResource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HypervSite {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of VMwareSiteResource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SiteProperties>,
}
impl HypervSite {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a HypervSite list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HypervSiteListResult {
    #[doc = "The HypervSite items on this page"]
    pub value: Vec<HypervSite>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HypervSiteListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HypervSiteListResult {
    pub fn new(value: Vec<HypervSite>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the HypervSite."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervSiteUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the HypervSite."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HypervSiteUpdateProperties>,
}
impl HypervSiteUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the HypervSite."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervSiteUpdateProperties {
    #[doc = "Class for site properties."]
    #[serde(rename = "servicePrincipalIdentityDetails", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_identity_details: Option<SiteSpnProperties>,
    #[doc = "Class for site agent properties."]
    #[serde(rename = "agentDetails", default, skip_serializing_if = "Option::is_none")]
    pub agent_details: Option<SiteAgentProperties>,
    #[doc = "Gets or sets the Appliance Name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
    #[doc = "Gets or sets the ARM ID of migration hub solution for SDS."]
    #[serde(rename = "discoverySolutionId", default, skip_serializing_if = "Option::is_none")]
    pub discovery_solution_id: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl HypervSiteUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Hyper-V site usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervSiteUsage {
    #[doc = "Gets or sets the number of machines discovered in the site."]
    #[serde(rename = "machineCount", default, skip_serializing_if = "Option::is_none")]
    pub machine_count: Option<i32>,
    #[doc = "Gets or sets the number of run as accounts in the site."]
    #[serde(rename = "runAsAccountCount", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_count: Option<i32>,
    #[doc = "Gets or sets the number of hosts part of the site."]
    #[serde(rename = "hostCount", default, skip_serializing_if = "Option::is_none")]
    pub host_count: Option<i32>,
    #[doc = "Gets or sets the number of clusters part of the site."]
    #[serde(rename = "clusterCount", default, skip_serializing_if = "Option::is_none")]
    pub cluster_count: Option<i32>,
}
impl HypervSiteUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "HyperV VM software inventory REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HypervVmSoftwareInventory {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for machine software inventory properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineSoftwareInventoryProperties>,
}
impl HypervVmSoftwareInventory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a HypervVmSoftwareInventory list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HypervVmSoftwareInventoryListResult {
    #[doc = "The HypervVmSoftwareInventory items on this page"]
    pub value: Vec<HypervVmSoftwareInventory>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HypervVmSoftwareInventoryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HypervVmSoftwareInventoryListResult {
    pub fn new(value: Vec<HypervVmSoftwareInventory>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Second-level object for identification of application units in a web site\nhosted on IIS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IisApplicationUnit {
    #[doc = "Gets or sets the path of the directory."]
    #[serde(rename = "applicationPoolName", default, skip_serializing_if = "Option::is_none")]
    pub application_pool_name: Option<String>,
    #[doc = "Gets or sets the managed pipeline mode."]
    #[serde(rename = "managedPipelineMode", default, skip_serializing_if = "Option::is_none")]
    pub managed_pipeline_mode: Option<String>,
    #[doc = "Gets or sets the runtime version."]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    #[doc = "Gets or sets a value indicating whether 32 bit applications are allowed to run\non 64 bit."]
    #[serde(rename = "enable32BitApiOnWin64", default, skip_serializing_if = "Option::is_none")]
    pub enable32_bit_api_on_win64: Option<bool>,
    #[doc = "Second level entity for virtual directories."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<DirectoryPath>,
    #[doc = "Gets or sets the list of directories."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub directories: Vec<DirectoryPath>,
}
impl IisApplicationUnit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Second-level object for identification of virtual applications in a web site\nhosted on IIS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IisVirtualApplicationUnit {
    #[doc = "Gets a value indicating whether the application corresponds to a directory."]
    #[serde(rename = "isVirtualDirectory", default, skip_serializing_if = "Option::is_none")]
    pub is_virtual_directory: Option<bool>,
    #[doc = "Second level entity for virtual directories."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<DirectoryPath>,
    #[doc = "Gets or sets the list of directories."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub directories: Vec<DirectoryPath>,
}
impl IisVirtualApplicationUnit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for web application properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IisWebApplicationProperties {
    #[doc = "Gets the list of application units for the web site."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub applications: Vec<IisApplicationUnit>,
    #[doc = "Gets the list of application units for the web site."]
    #[serde(
        rename = "virtualApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub virtual_applications: Vec<IisVirtualApplicationUnit>,
    #[doc = "Gets the web server id."]
    #[serde(rename = "webServerId", default, skip_serializing_if = "Option::is_none")]
    pub web_server_id: Option<String>,
    #[doc = "Gets the web server name."]
    #[serde(rename = "webServerName", default, skip_serializing_if = "Option::is_none")]
    pub web_server_name: Option<String>,
    #[doc = "Gets the list of machine ARM Ids on which the SQL server is deployed."]
    #[serde(
        rename = "machineArmIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machine_arm_ids: Vec<String>,
    #[doc = "Gets the virtual path of the application."]
    #[serde(rename = "virtualPath", default, skip_serializing_if = "Option::is_none")]
    pub virtual_path: Option<String>,
    #[doc = "Gets the physical path of the application."]
    #[serde(rename = "physicalPath", default, skip_serializing_if = "Option::is_none")]
    pub physical_path: Option<String>,
    #[doc = "Gets the front end bindings for the application."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bindings: Vec<FrontEndBinding>,
    #[doc = "Gets the Error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub frameworks: Vec<WebApplicationFramework>,
    #[doc = "Gets the configuration."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub configurations: Vec<WebApplicationConfigurationUnit>,
    #[doc = "Gets the directories."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub directories: Vec<WebApplicationDirectoryUnit>,
    #[doc = "Gets the display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets tags on the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Static folders."]
    #[serde(
        rename = "staticFolders",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub static_folders: Vec<String>,
    #[doc = "Machine display name"]
    #[serde(rename = "machineDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub machine_display_name: Option<String>,
    #[doc = "Gets a value indicating whether application is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Gets the timestamp marking creation of the entity."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets the timestamp marking last update operation."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "Gets the server type."]
    #[serde(rename = "serverType", default, skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[doc = "Gets the Error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "Gets the appliance names."]
    #[serde(
        rename = "applianceNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub appliance_names: Vec<String>,
    #[doc = "Gets a value indicating whether the WebApp has errors or not."]
    #[serde(rename = "hasErrors", default, skip_serializing_if = "Option::is_none")]
    pub has_errors: Option<bool>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl IisWebApplicationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web application REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IisWebApplications {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for web application properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IisWebApplicationProperties>,
}
impl IisWebApplications {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a IisWebApplications list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IisWebApplicationsListResult {
    #[doc = "The IisWebApplications items on this page"]
    pub value: Vec<IisWebApplications>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IisWebApplicationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl IisWebApplicationsListResult {
    pub fn new(value: Vec<IisWebApplications>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the IisWebApplications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IisWebApplicationsUpdate {
    #[doc = "The updatable properties of the IisWebApplications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IisWebApplicationsUpdateProperties>,
}
impl IisWebApplicationsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the IisWebApplications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IisWebApplicationsUpdateProperties {
    #[doc = "Gets or sets tags on the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl IisWebApplicationsUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for web server properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IisWebServerProperties {
    #[doc = "Gets the configuration location."]
    #[serde(rename = "configurationLocation", default, skip_serializing_if = "Option::is_none")]
    pub configuration_location: Option<String>,
    #[doc = "Gets the configuration location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets the list of machines."]
    #[serde(
        rename = "machineIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machine_ids: Vec<String>,
    #[doc = "Gets the list of web applications."]
    #[serde(
        rename = "webApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub web_applications: Vec<String>,
    #[doc = "Gets the display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets the server FQDN."]
    #[serde(rename = "serverFqdn", default, skip_serializing_if = "Option::is_none")]
    pub server_fqdn: Option<String>,
    #[doc = "Gets the run as account id."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Gets a value indicating whether application is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Gets the timestamp marking creation of the entity."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets the timestamp marking last update operation."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "Gets the server type."]
    #[serde(rename = "serverType", default, skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[doc = "Gets the Error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "Gets the appliance names."]
    #[serde(
        rename = "applianceNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub appliance_names: Vec<String>,
    #[doc = "Gets a value indicating whether the WebApp has errors or not."]
    #[serde(rename = "hasErrors", default, skip_serializing_if = "Option::is_none")]
    pub has_errors: Option<bool>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl IisWebServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web server REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IisWebServers {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for web server properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IisWebServerProperties>,
}
impl IisWebServers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a IisWebServers list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IisWebServersListResult {
    #[doc = "The IisWebServers items on this page"]
    pub value: Vec<IisWebServers>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IisWebServersListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl IisWebServersListResult {
    pub fn new(value: Vec<IisWebServers>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A host resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportJob {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Job REST Resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
impl ImportJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a ImportJob list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportJobListResult {
    #[doc = "The ImportJob items on this page"]
    pub value: Vec<ImportJob>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImportJobListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ImportJobListResult {
    pub fn new(value: Vec<ImportJob>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "An machine resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportMachine {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for machine properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImportMachineProperties>,
}
impl ImportMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a ImportMachine list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportMachineListResult {
    #[doc = "The ImportMachine items on this page"]
    pub value: Vec<ImportMachine>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImportMachineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ImportMachineListResult {
    pub fn new(value: Vec<ImportMachine>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class for machine properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportMachineProperties {
    #[doc = "Firmware of the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    #[doc = "CPU utilization."]
    #[serde(rename = "percentageCpuUtilization", default, skip_serializing_if = "Option::is_none")]
    pub percentage_cpu_utilization: Option<f32>,
    #[doc = "Memory utilization."]
    #[serde(rename = "percentageMemoryUtilization", default, skip_serializing_if = "Option::is_none")]
    pub percentage_memory_utilization: Option<f32>,
    #[doc = "Number of disks."]
    #[serde(rename = "numberOfDisks", default, skip_serializing_if = "Option::is_none")]
    pub number_of_disks: Option<i32>,
    #[doc = "Total disk read operations per second."]
    #[serde(rename = "totalDiskReadOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub total_disk_read_operations_per_second: Option<f32>,
    #[doc = "Total disk write operations per second."]
    #[serde(rename = "totalDiskWriteOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub total_disk_write_operations_per_second: Option<f32>,
    #[doc = "Total disk write throughput."]
    #[serde(rename = "totalDiskWriteThroughput", default, skip_serializing_if = "Option::is_none")]
    pub total_disk_write_throughput: Option<f32>,
    #[doc = "Total disk read throughput."]
    #[serde(rename = "totalDiskReadThroughput", default, skip_serializing_if = "Option::is_none")]
    pub total_disk_read_throughput: Option<f32>,
    #[doc = "MAC Address."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "IP Addresses."]
    #[serde(
        rename = "ipAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_addresses: Vec<String>,
    #[doc = "Machine ID."]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "Machine manager ID."]
    #[serde(rename = "machineManagerId", default, skip_serializing_if = "Option::is_none")]
    pub machine_manager_id: Option<String>,
    #[doc = "Number of network adapters."]
    #[serde(rename = "numberOfNetworkAdapters", default, skip_serializing_if = "Option::is_none")]
    pub number_of_network_adapters: Option<i32>,
    #[doc = "Network in throughput."]
    #[serde(rename = "networkInThroughput", default, skip_serializing_if = "Option::is_none")]
    pub network_in_throughput: Option<f32>,
    #[doc = "Network out throughput."]
    #[serde(rename = "networkOutThroughput", default, skip_serializing_if = "Option::is_none")]
    pub network_out_throughput: Option<f32>,
    #[doc = "Server type."]
    #[serde(rename = "serverType", default, skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[doc = "Fabric type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hypervisor: Option<String>,
    #[doc = "Hypervisor version number."]
    #[serde(rename = "hypervisorVersionNumber", default, skip_serializing_if = "Option::is_none")]
    pub hypervisor_version_number: Option<String>,
    #[doc = "Disks attached to the machine."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disks: Vec<WebRoleImportDisk>,
    #[doc = "Server FQDN."]
    #[serde(rename = "vmFqdn", default, skip_serializing_if = "Option::is_none")]
    pub vm_fqdn: Option<String>,
    #[doc = "Total storage in use."]
    #[serde(rename = "storageInUseGb", default, skip_serializing_if = "Option::is_none")]
    pub storage_in_use_gb: Option<f32>,
    #[doc = "Gets or sets the Number of Processor Cores \n            allocated for the\nmachine."]
    #[serde(rename = "numberOfProcessorCore", default, skip_serializing_if = "Option::is_none")]
    pub number_of_processor_core: Option<i32>,
    #[doc = "Gets or sets the allocated Memory in MB."]
    #[serde(rename = "allocatedMemoryInMb", default, skip_serializing_if = "Option::is_none")]
    pub allocated_memory_in_mb: Option<f64>,
    #[doc = "Second level object returned as part of Machine REST resource."]
    #[serde(rename = "operatingSystemDetails", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_details: Option<WebRoleOperatingSystem>,
    #[doc = "Gets or sets the Machine BIOS serial number."]
    #[serde(rename = "biosSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub bios_serial_number: Option<String>,
    #[doc = "Gets or sets the BIOS GUID."]
    #[serde(rename = "biosGuid", default, skip_serializing_if = "Option::is_none")]
    pub bios_guid: Option<String>,
    #[doc = "Gets the Display name of the machine."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Value indicating whether VM is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Gets the marking machine creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets the timestamp marking last updated on the machine."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "machine tags"]
    pub tags: serde_json::Value,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ImportMachineProperties {
    pub fn new(tags: serde_json::Value) -> Self {
        Self {
            firmware: None,
            percentage_cpu_utilization: None,
            percentage_memory_utilization: None,
            number_of_disks: None,
            total_disk_read_operations_per_second: None,
            total_disk_write_operations_per_second: None,
            total_disk_write_throughput: None,
            total_disk_read_throughput: None,
            mac_address: None,
            ip_addresses: Vec::new(),
            machine_id: None,
            machine_manager_id: None,
            number_of_network_adapters: None,
            network_in_throughput: None,
            network_out_throughput: None,
            server_type: None,
            hypervisor: None,
            hypervisor_version_number: None,
            disks: Vec::new(),
            vm_fqdn: None,
            storage_in_use_gb: None,
            number_of_processor_core: None,
            allocated_memory_in_mb: None,
            operating_system_details: None,
            bios_serial_number: None,
            bios_guid: None,
            display_name: None,
            is_deleted: None,
            created_timestamp: None,
            updated_timestamp: None,
            tags,
            provisioning_state: None,
        }
    }
}
#[doc = "Import machines Job REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportMachinesJob {
    #[doc = "Handled by resource provider. Type =\nMicrosoft.OffAzure/ImportSites/jobs/importJobs."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "ImportMachines JobProperties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImportMachinesJobProperties>,
    #[doc = "Gets or sets the relative ARM name to get job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the Job ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the Job status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets the Job start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Gets or sets the Job end time."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Gets or sets the Display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl ImportMachinesJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ImportMachines JobProperties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportMachinesJobProperties {
    #[doc = "blob name"]
    #[serde(rename = "blobName", default, skip_serializing_if = "Option::is_none")]
    pub blob_name: Option<String>,
    #[doc = "blob sasUri"]
    #[serde(rename = "blobSasUri", default, skip_serializing_if = "Option::is_none")]
    pub blob_sas_uri: Option<String>,
    #[doc = "JobResultValues"]
    #[serde(rename = "jobResult", default, skip_serializing_if = "Option::is_none")]
    pub job_result: Option<JobResult>,
    #[doc = "number Of Machines Imported"]
    #[serde(rename = "numberOfMachinesImported", default, skip_serializing_if = "Option::is_none")]
    pub number_of_machines_imported: Option<i32>,
    #[doc = "blob Creation TimeStamp"]
    #[serde(rename = "blobCreationTimeStamp", default, with = "azure_core::date::rfc3339::option")]
    pub blob_creation_time_stamp: Option<time::OffsetDateTime>,
    #[doc = "Job Error Summary"]
    #[serde(rename = "errorSummary", default, skip_serializing_if = "Option::is_none")]
    pub error_summary: Option<JobErrorSummary>,
}
impl ImportMachinesJobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A ImportSite"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportSite {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of ImportSiteResource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImportSiteProperties>,
}
impl ImportSite {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a ImportSite list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportSiteListResult {
    #[doc = "The ImportSite items on this page"]
    pub value: Vec<ImportSite>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImportSiteListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ImportSiteListResult {
    pub fn new(value: Vec<ImportSite>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of ImportSiteResource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportSiteProperties {
    #[doc = "Gets or sets the ARM ID of migration hub solution for SDS."]
    #[serde(rename = "discoverySolutionId", default, skip_serializing_if = "Option::is_none")]
    pub discovery_solution_id: Option<String>,
    #[doc = "Gets the Master Site this site is linked to."]
    #[serde(rename = "masterSiteId", default, skip_serializing_if = "Option::is_none")]
    pub master_site_id: Option<String>,
    #[doc = "Gets the service endpoint."]
    #[serde(rename = "serviceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ImportSiteProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for update operations of the ImportSite."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportSiteUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the ImportSite."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImportSiteUpdateProperties>,
}
impl ImportSiteUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the ImportSite."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportSiteUpdateProperties {
    #[doc = "Gets or sets the ARM ID of migration hub solution for SDS."]
    #[serde(rename = "discoverySolutionId", default, skip_serializing_if = "Option::is_none")]
    pub discovery_solution_id: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ImportSiteUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ImportTypeValues enum."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ImportTypeValues")]
pub enum ImportTypeValues {
    #[serde(rename = "AzureMigrateCSV")]
    AzureMigrateCsv,
    #[serde(rename = "RVToolsXlsx")]
    RvToolsXlsx,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ImportTypeValues {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ImportTypeValues {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ImportTypeValues {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureMigrateCsv => serializer.serialize_unit_variant("ImportTypeValues", 0u32, "AzureMigrateCSV"),
            Self::RvToolsXlsx => serializer.serialize_unit_variant("ImportTypeValues", 1u32, "RVToolsXlsx"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Job Error Summary"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobErrorSummary {
    #[doc = "errors list"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<String>,
    #[doc = "error count"]
    #[serde(rename = "errorCount", default, skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i32>,
    #[doc = "warning Count"]
    #[serde(rename = "warningCount", default, skip_serializing_if = "Option::is_none")]
    pub warning_count: Option<i32>,
}
impl JobErrorSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobProperties {
    #[doc = "Gets operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets operation start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Gets operation end time."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Gets or sets the display name of the Job."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets the client request Id used in the operation execution context."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "Gets the activity Id used in the operation execution context."]
    #[serde(rename = "activityId", default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[doc = "Gets the errors."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<ErrorDetails>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl JobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "JobResultValues"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "JobResult")]
pub enum JobResult {
    Unknown,
    Completed,
    CompletedWithWarnings,
    CompletedWithErrors,
    Failed,
    WaitingForBlobUpload,
    InProgress,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for JobResult {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for JobResult {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for JobResult {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("JobResult", 0u32, "Unknown"),
            Self::Completed => serializer.serialize_unit_variant("JobResult", 1u32, "Completed"),
            Self::CompletedWithWarnings => serializer.serialize_unit_variant("JobResult", 2u32, "CompletedWithWarnings"),
            Self::CompletedWithErrors => serializer.serialize_unit_variant("JobResult", 3u32, "CompletedWithErrors"),
            Self::Failed => serializer.serialize_unit_variant("JobResult", 4u32, "Failed"),
            Self::WaitingForBlobUpload => serializer.serialize_unit_variant("JobResult", 5u32, "WaitingForBlobUpload"),
            Self::InProgress => serializer.serialize_unit_variant("JobResult", 6u32, "InProgress"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties of Machine MetaData"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineMetadata {
    #[doc = "arm id of the machine."]
    #[serde(rename = "machineArmId")]
    pub machine_arm_id: String,
    #[doc = "value representing state of dependency mapping (enabled/disabled)."]
    #[serde(rename = "dependencyMapping")]
    pub dependency_mapping: String,
    #[doc = "machine tags"]
    pub tags: serde_json::Value,
}
impl MachineMetadata {
    pub fn new(machine_arm_id: String, dependency_mapping: String, tags: serde_json::Value) -> Self {
        Self {
            machine_arm_id,
            dependency_mapping,
            tags,
        }
    }
}
#[doc = "The list of Machine MetaData."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineMetadataCollection {
    #[doc = "The list of Machine MetaData."]
    pub value: Vec<MachineMetadata>,
}
impl MachineMetadataCollection {
    pub fn new(value: Vec<MachineMetadata>) -> Self {
        Self { value }
    }
}
#[doc = "An machine resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for machine properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VmwareMachineProperties>,
}
impl MachineResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a MachineResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineResourceListResult {
    #[doc = "The MachineResource items on this page"]
    pub value: Vec<MachineResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MachineResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl MachineResourceListResult {
    pub fn new(value: Vec<MachineResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the MachineResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineResourceUpdate {
    #[doc = "The updatable properties of the MachineResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineResourceUpdateProperties>,
}
impl MachineResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the MachineResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineResourceUpdateProperties {
    #[doc = "Gets or sets the firmware."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    #[doc = "Gets or sets the run as account ID of the machine."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "product support status."]
    #[serde(rename = "productSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_support_status: Option<ProductSupportStatus>,
    #[doc = "Gets or sets the Number of Processor Cores \n            allocated for the\nmachine."]
    #[serde(rename = "numberOfProcessorCore", default, skip_serializing_if = "Option::is_none")]
    pub number_of_processor_core: Option<i32>,
    #[doc = "Gets or sets the allocated Memory in MB."]
    #[serde(rename = "allocatedMemoryInMb", default, skip_serializing_if = "Option::is_none")]
    pub allocated_memory_in_mb: Option<f64>,
    #[doc = "Second level object returned as part of Machine REST resource."]
    #[serde(rename = "operatingSystemDetails", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_details: Option<OperatingSystem>,
    #[doc = "Gets or sets the Machine BIOS serial number."]
    #[serde(rename = "biosSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub bios_serial_number: Option<String>,
    #[doc = "Gets or sets the BIOS GUID."]
    #[serde(rename = "biosGuid", default, skip_serializing_if = "Option::is_none")]
    pub bios_guid: Option<String>,
    #[doc = "Gets or sets tags on the VMware machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl MachineResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for machine software inventory properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineSoftwareInventoryProperties {
    #[doc = "AppsAndRoles in the guest virtual machine."]
    #[serde(rename = "appsAndRoles", default, skip_serializing_if = "Option::is_none")]
    pub apps_and_roles: Option<AppsAndRoles>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl MachineSoftwareInventoryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A MasterSite"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MasterSite {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Class for site properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MasterSiteProperties>,
}
impl MasterSite {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a MasterSite list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MasterSiteListResult {
    #[doc = "The MasterSite items on this page"]
    pub value: Vec<MasterSite>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MasterSiteListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl MasterSiteListResult {
    pub fn new(value: Vec<MasterSite>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class for site properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MasterSiteProperties {
    #[doc = "PublicNetworkAccess"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<MasterSitePropertiesPublicNetworkAccess>,
    #[doc = "Gets or sets a value indicating whether multiple sites per site type are\nallowed."]
    #[serde(rename = "allowMultipleSites", default, skip_serializing_if = "Option::is_none")]
    pub allow_multiple_sites: Option<bool>,
    #[doc = "Gets or sets the sites that are a part of Master Site.\n            The key\nshould contain the Site ARM name."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sites: Vec<String>,
    #[doc = "Gets or sets a value for customer storage account ARM id."]
    #[serde(rename = "customerStorageAccountArmId", default, skip_serializing_if = "Option::is_none")]
    pub customer_storage_account_arm_id: Option<String>,
    #[doc = "Gets the private endpoint connections."]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Gets the nested sites under Master Site."]
    #[serde(
        rename = "nestedSites",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub nested_sites: Vec<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl MasterSiteProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "PublicNetworkAccess"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MasterSitePropertiesPublicNetworkAccess")]
pub enum MasterSitePropertiesPublicNetworkAccess {
    NotSpecified,
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MasterSitePropertiesPublicNetworkAccess {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MasterSitePropertiesPublicNetworkAccess {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MasterSitePropertiesPublicNetworkAccess {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("MasterSitePropertiesPublicNetworkAccess", 0u32, "NotSpecified"),
            Self::Enabled => serializer.serialize_unit_variant("MasterSitePropertiesPublicNetworkAccess", 1u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("MasterSitePropertiesPublicNetworkAccess", 2u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type used for update operations of the MasterSite."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MasterSiteUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the MasterSite."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MasterSiteUpdateProperties>,
}
impl MasterSiteUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the MasterSite."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MasterSiteUpdateProperties {
    #[doc = "PublicNetworkAccess"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<MasterSitePropertiesPublicNetworkAccess>,
    #[doc = "Gets or sets a value indicating whether multiple sites per site type are\nallowed."]
    #[serde(rename = "allowMultipleSites", default, skip_serializing_if = "Option::is_none")]
    pub allow_multiple_sites: Option<bool>,
    #[doc = "Gets or sets the sites that are a part of Master Site.\n            The key\nshould contain the Site ARM name."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sites: Vec<String>,
    #[doc = "Gets or sets a value for customer storage account ARM id."]
    #[serde(rename = "customerStorageAccountArmId", default, skip_serializing_if = "Option::is_none")]
    pub customer_storage_account_arm_id: Option<String>,
}
impl MasterSiteUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "HealthError Details Source"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MicrosoftAzureFdsWebRoleHealthErrorDetailsSource")]
pub enum MicrosoftAzureFdsWebRoleHealthErrorDetailsSource {
    RefreshFabricLayout,
    RefreshFabricLayoutGuest,
    RefreshFabricLayoutDependencyMap,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MicrosoftAzureFdsWebRoleHealthErrorDetailsSource {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MicrosoftAzureFdsWebRoleHealthErrorDetailsSource {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MicrosoftAzureFdsWebRoleHealthErrorDetailsSource {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::RefreshFabricLayout => {
                serializer.serialize_unit_variant("MicrosoftAzureFdsWebRoleHealthErrorDetailsSource", 0u32, "RefreshFabricLayout")
            }
            Self::RefreshFabricLayoutGuest => {
                serializer.serialize_unit_variant("MicrosoftAzureFdsWebRoleHealthErrorDetailsSource", 1u32, "RefreshFabricLayoutGuest")
            }
            Self::RefreshFabricLayoutDependencyMap => serializer.serialize_unit_variant(
                "MicrosoftAzureFdsWebRoleHealthErrorDetailsSource",
                2u32,
                "RefreshFabricLayoutDependencyMap",
            ),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "object model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Object {}
impl Object {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Second level object returned as part of Machine REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperatingSystem {
    #[doc = "Gets or sets the type of the operating system."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Gets or sets the Name of the operating system."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "Gets or sets the Version of the operating system."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "Gets or sets the Architecture of the operating system."]
    #[serde(rename = "osArchitecture", default, skip_serializing_if = "Option::is_none")]
    pub os_architecture: Option<String>,
}
impl OperatingSystem {
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
#[doc = "Operation status REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "Gets the Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the status of the operation. ARM expects the terminal status to be one\nof\n            Succeeded/ Failed/ Canceled. All other values imply that the\noperation is still running."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets the start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Gets the start time."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Class for operation status errors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationStatusError>,
    #[doc = "Class for operation result properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationStatusProperties>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for operation status errors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatusError {
    #[doc = "Gets the error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Gets the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl OperationStatusError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for operation result properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatusProperties {
    #[doc = "Gets or sets the result or output of the workflow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}
impl OperationStatusProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data related to a machine's Oracle discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OracleDiscovery {
    #[doc = "Gets or sets number of successfully discovered instances."]
    #[serde(rename = "totalInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub total_instance_count: Option<i64>,
    #[doc = "Gets or sets number of successfully discovered databases."]
    #[serde(rename = "totalDatabaseCount", default, skip_serializing_if = "Option::is_none")]
    pub total_database_count: Option<i64>,
    #[doc = "Shallow Discovery Status."]
    #[serde(rename = "shallowDiscoveryStatus", default, skip_serializing_if = "Option::is_none")]
    pub shallow_discovery_status: Option<ShallowDiscoveryStatus>,
    #[doc = "Discovery Scope."]
    #[serde(rename = "discoveryScopeStatus", default, skip_serializing_if = "Option::is_none")]
    pub discovery_scope_status: Option<DiscoveryScopeStatus>,
}
impl OracleDiscovery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OtherDatabase in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OtherDatabase {
    #[doc = "Gets or sets DatabaseType of the OtherDatabase."]
    #[serde(rename = "databaseType", default, skip_serializing_if = "Option::is_none")]
    pub database_type: Option<String>,
    #[doc = "Gets or sets Instance of the OtherDatabase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[doc = "Gets or sets Version of the OtherDatabase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl OtherDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged collection of DeleteImportMachinesJob items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedDeleteImportMachinesJob {
    #[doc = "The DeleteImportMachinesJob items on this page"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DeleteImportMachinesJob>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedDeleteImportMachinesJob {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedDeleteImportMachinesJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged collection of ExportImportedMachinesJob items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedExportImportedMachinesJob {
    #[doc = "The ExportImportedMachinesJob items on this page"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ExportImportedMachinesJob>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedExportImportedMachinesJob {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedExportImportedMachinesJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged collection of ImportMachinesJob items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedImportMachinesJob {
    #[doc = "The ImportMachinesJob items on this page"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ImportMachinesJob>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedImportMachinesJob {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedImportMachinesJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "REST model used to encapsulate Private Link properties for tracked resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "PrivateEndpointConnectionProperties V2"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionPropertiesV2>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a PrivateEndpointConnection list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "The PrivateEndpointConnection items on this page"]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new(value: Vec<PrivateEndpointConnection>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "PrivateEndpointConnectionProperties V2"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionPropertiesV2 {
    #[doc = "array of group ids"]
    #[serde(
        rename = "groupIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_ids: Vec<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "PrivateEndpointModelsResourceId"]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<ResourceId>,
    #[doc = "Service Connection State"]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
}
impl PrivateEndpointConnectionPropertiesV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "REST model used to encapsulate Private Link properties for tracked resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "private link resource properties model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a PrivateLinkResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceListResult {
    #[doc = "The PrivateLinkResource items on this page"]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PrivateLinkResourceListResult {
    pub fn new(value: Vec<PrivateLinkResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "private link resource properties model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "required members"]
    #[serde(
        rename = "requiredMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_members: Vec<String>,
    #[doc = "required zone names"]
    #[serde(
        rename = "requiredZoneNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_zone_names: Vec<String>,
    #[doc = "group id"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service Connection State"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "PrivateLinkServiceConnectionState"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateLinkServiceConnectionStateStatus>,
    #[doc = "description string"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "actions required"]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "PrivateLinkServiceConnectionState"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateLinkServiceConnectionStateStatus")]
pub enum PrivateLinkServiceConnectionStateStatus {
    Approved,
    Pending,
    Rejected,
    Disconnected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateLinkServiceConnectionStateStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateLinkServiceConnectionStateStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateLinkServiceConnectionStateStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Approved => serializer.serialize_unit_variant("PrivateLinkServiceConnectionStateStatus", 0u32, "Approved"),
            Self::Pending => serializer.serialize_unit_variant("PrivateLinkServiceConnectionStateStatus", 1u32, "Pending"),
            Self::Rejected => serializer.serialize_unit_variant("PrivateLinkServiceConnectionStateStatus", 2u32, "Rejected"),
            Self::Disconnected => serializer.serialize_unit_variant("PrivateLinkServiceConnectionStateStatus", 3u32, "Disconnected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Processor Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessorInfo {
    #[doc = "Gets or sets the name\\model of a processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the number of sockets."]
    #[serde(rename = "numberOfSockets", default, skip_serializing_if = "Option::is_none")]
    pub number_of_sockets: Option<i32>,
    #[doc = "Gets or sets the total number of cores in a socket."]
    #[serde(rename = "numberOfCoresPerSocket", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores_per_socket: Option<i32>,
}
impl ProcessorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "product support status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductSupportStatus {
    #[doc = "current version."]
    #[serde(rename = "currentVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[doc = "esu Status"]
    #[serde(rename = "esuStatus", default, skip_serializing_if = "Option::is_none")]
    pub esu_status: Option<EsuStatus>,
    #[doc = "support status"]
    #[serde(rename = "supportStatus", default, skip_serializing_if = "Option::is_none")]
    pub support_status: Option<SupportStatus>,
    #[doc = "support end date."]
    #[serde(rename = "supportEndDate", default, with = "azure_core::date::rfc3339::option")]
    pub support_end_date: Option<time::OffsetDateTime>,
    #[doc = "esu year"]
    #[serde(rename = "esuYear", default, skip_serializing_if = "Option::is_none")]
    pub esu_year: Option<EsuYear>,
}
impl ProductSupportStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of the current operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Created,
    Updated,
    Running,
    Completed,
    Failed,
    Succeeded,
    Canceled,
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
            Self::Created => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Created"),
            Self::Updated => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updated"),
            Self::Running => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Running"),
            Self::Completed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Completed"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Succeeded"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Canceled"),
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
#[doc = "Request body for Proxy site refresh action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxySiteRefreshBody {
    #[doc = "Gets or sets the appliance name of the agent in the site."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
}
impl ProxySiteRefreshBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Properties class for export machine errors request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestExportMachineErrorsProperties {
    #[doc = "Export Machine Errors Properties"]
    #[serde(rename = "discoveryScope", default, skip_serializing_if = "Option::is_none")]
    pub discovery_scope: Option<ExportMachineErrorsProperties>,
}
impl RequestExportMachineErrorsProperties {
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
#[doc = "PrivateEndpointModelsResourceId"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceId {
    #[doc = "id name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ResourceId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Machine class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunAsAccountMachineInput {
    #[doc = "Gets or sets the ARM id of the machine."]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "run as AccountId"]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
}
impl RunAsAccountMachineInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for run as account properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunAsAccountProperties {
    #[doc = "Display name of the run as account."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets the credential type of the run as account."]
    #[serde(rename = "credentialType", default, skip_serializing_if = "Option::is_none")]
    pub credential_type: Option<String>,
    #[doc = "Timestamp marking run as account creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Timestamp marking last updated on the run as account."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "Gets the appliance name of the run as account."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl RunAsAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQLDiscoveryScope Status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlDiscoveryScopeStatus")]
pub enum SqlDiscoveryScopeStatus {
    DiscoverySucceededAtleastOnce,
    DiscoveryFailed,
    RunAsAccountNotAssociated,
    DiscoveryNotStarted,
    DiscoveryInProgress,
    Disabled,
    DiscoveryPartiallySucceded,
    DiscoverySucceeded,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlDiscoveryScopeStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlDiscoveryScopeStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlDiscoveryScopeStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DiscoverySucceededAtleastOnce => {
                serializer.serialize_unit_variant("SqlDiscoveryScopeStatus", 0u32, "DiscoverySucceededAtleastOnce")
            }
            Self::DiscoveryFailed => serializer.serialize_unit_variant("SqlDiscoveryScopeStatus", 1u32, "DiscoveryFailed"),
            Self::RunAsAccountNotAssociated => {
                serializer.serialize_unit_variant("SqlDiscoveryScopeStatus", 2u32, "RunAsAccountNotAssociated")
            }
            Self::DiscoveryNotStarted => serializer.serialize_unit_variant("SqlDiscoveryScopeStatus", 3u32, "DiscoveryNotStarted"),
            Self::DiscoveryInProgress => serializer.serialize_unit_variant("SqlDiscoveryScopeStatus", 4u32, "DiscoveryInProgress"),
            Self::Disabled => serializer.serialize_unit_variant("SqlDiscoveryScopeStatus", 5u32, "Disabled"),
            Self::DiscoveryPartiallySucceded => {
                serializer.serialize_unit_variant("SqlDiscoveryScopeStatus", 6u32, "DiscoveryPartiallySucceded")
            }
            Self::DiscoverySucceeded => serializer.serialize_unit_variant("SqlDiscoveryScopeStatus", 7u32, "DiscoverySucceeded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Import URI response class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SasUriResponse {
    #[doc = "Gets or sets the job ARM ID."]
    #[serde(rename = "jobArmId", default, skip_serializing_if = "Option::is_none")]
    pub job_arm_id: Option<String>,
    #[doc = "Gets or sets the SAS URI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "ImportTypeValues enum."]
    #[serde(rename = "importType", default, skip_serializing_if = "Option::is_none")]
    pub import_type: Option<ImportTypeValues>,
}
impl SasUriResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A machine resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Server {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Server REST resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerProperties>,
}
impl Server {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server disk details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerDisk {
    #[doc = "Gets or sets Id of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets generated Id of the disk."]
    #[serde(rename = "generatedId", default, skip_serializing_if = "Option::is_none")]
    pub generated_id: Option<String>,
    #[doc = "Gets or sets Bytes allocated for the disk."]
    #[serde(rename = "maxSizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_in_bytes: Option<i32>,
    #[doc = "Gets or sets Name of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets Type of the disk."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
    #[doc = "Gets or sets LUN of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[doc = "Gets or sets Path of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl ServerDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A host resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerJob {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Job REST Resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
impl ServerJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a ServerJob list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerJobListResult {
    #[doc = "The ServerJob items on this page"]
    pub value: Vec<ServerJob>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerJobListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerJobListResult {
    pub fn new(value: Vec<ServerJob>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The response of a Server list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerListResult {
    #[doc = "The Server items on this page"]
    pub value: Vec<Server>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerListResult {
    pub fn new(value: Vec<Server>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Second level object represented in responses as part of Machine REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerNetworkAdapter {
    #[doc = "Gets or sets the NIC Id."]
    #[serde(rename = "nicId", default, skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    #[doc = "Gets or sets Mac address of the NIC."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "Gets IP addresses for the machine."]
    #[serde(
        rename = "ipAddressList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_address_list: Vec<String>,
    #[doc = "Gets or sets Network Name."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "Gets or sets Type of the IP address."]
    #[serde(rename = "ipAddressType", default, skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
}
impl ServerNetworkAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerProperties {
    #[doc = "Gets the Display name of the machine.\n            For server entity hydrated\nFQDN is set as display name\n            as the server id and server name are\nsame."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the FQDN/IPAddress of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Gets or sets the network adapters of the server."]
    #[serde(
        rename = "networkAdapters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_adapters: Vec<ServerNetworkAdapter>,
    #[doc = "Gets or sets the FQDN of machine which can be changed."]
    #[serde(rename = "hydratedFqdn", default, skip_serializing_if = "Option::is_none")]
    pub hydrated_fqdn: Option<String>,
    #[doc = "Gets or sets the disk details of server."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disks: Vec<ServerDisk>,
    #[doc = "Gets or sets the validation required for the machine."]
    #[serde(rename = "validationRequired", default, skip_serializing_if = "Option::is_none")]
    pub validation_required: Option<String>,
    #[doc = "Processor Information."]
    #[serde(rename = "processorInfo", default, skip_serializing_if = "Option::is_none")]
    pub processor_info: Option<ProcessorInfo>,
    #[doc = "Gets or sets the firmware."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    #[doc = "Second level object.  Data related to a machine's operating system.             Serialized and stored as part of Machine Rest object. "]
    #[serde(rename = "guestOsDetails", default, skip_serializing_if = "Option::is_none")]
    pub guest_os_details: Option<GuestOsDetails>,
    #[doc = "Number of applications installed in the guest VM."]
    #[serde(rename = "numberOfApplications", default, skip_serializing_if = "Option::is_none")]
    pub number_of_applications: Option<i32>,
    #[doc = "The last time at which the Guest Details was discovered\n            or the\nerror while discovering guest details based discovery\n            of the\nmachine."]
    #[serde(rename = "guestDetailsDiscoveryTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub guest_details_discovery_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Whether Refresh Fabric Layout Guest Details has been completed once.\n         \n  Portal will show discovery in progress, if this value is true."]
    #[serde(rename = "isGuestDetailsDiscoveryInProgress", default, skip_serializing_if = "Option::is_none")]
    pub is_guest_details_discovery_in_progress: Option<bool>,
    #[doc = "Gets or sets if dependency mapping feature is enabled or not\n            for\nthe VM."]
    #[serde(rename = "dependencyMapping", default, skip_serializing_if = "Option::is_none")]
    pub dependency_mapping: Option<String>,
    #[doc = "Gets or sets when dependency mapping collection is last started."]
    #[serde(rename = "dependencyMappingStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub dependency_mapping_start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets when dependency mapping collection was last disabled."]
    #[serde(rename = "dependencyMappingEndTime", default, skip_serializing_if = "Option::is_none")]
    pub dependency_mapping_end_time: Option<String>,
    #[doc = "Gets or sets the run as account ID of the machine."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Gets the appliance names."]
    #[serde(
        rename = "applianceNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub appliance_names: Vec<String>,
    #[doc = "Errors for machine."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "Contains data related application and roles discovery scope."]
    #[serde(rename = "applicationDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub application_discovery: Option<ApplicationDiscovery>,
    #[doc = "Contains data related dependency map discovery scope."]
    #[serde(rename = "dependencyMapDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub dependency_map_discovery: Option<DependencyMapDiscovery>,
    #[doc = "Contains data related static data discovery scope."]
    #[serde(rename = "staticDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub static_discovery: Option<StaticDiscovery>,
    #[doc = "Contains data related SQL discovery."]
    #[serde(rename = "sqlDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub sql_discovery: Option<SqlDiscovery>,
    #[doc = "Data related to a machine's WebApps discovery."]
    #[serde(rename = "webAppDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub web_app_discovery: Option<WebAppDiscovery>,
    #[doc = "Data related to a machine's Oracle discovery."]
    #[serde(rename = "oracleDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub oracle_discovery: Option<OracleDiscovery>,
    #[doc = "Data related to a machine's spring boot discovery."]
    #[serde(rename = "springBootDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub spring_boot_discovery: Option<SpringBootDiscovery>,
    #[doc = "Data related to a machine's WebApps discovery."]
    #[serde(rename = "iisDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub iis_discovery: Option<WebAppDiscovery>,
    #[doc = "Data related to a machine's WebApps discovery."]
    #[serde(rename = "tomcatDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub tomcat_discovery: Option<WebAppDiscovery>,
    #[doc = "AppsAndRoles in the guest virtual machine."]
    #[serde(rename = "appsAndRoles", default, skip_serializing_if = "Option::is_none")]
    pub apps_and_roles: Option<AppsAndRoles>,
    #[doc = "product support status."]
    #[serde(rename = "productSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_support_status: Option<ProductSupportStatus>,
    #[doc = "Gets or sets the Number of Processor Cores \n            allocated for the\nmachine."]
    #[serde(rename = "numberOfProcessorCore", default, skip_serializing_if = "Option::is_none")]
    pub number_of_processor_core: Option<i32>,
    #[doc = "Gets or sets the allocated Memory in MB."]
    #[serde(rename = "allocatedMemoryInMb", default, skip_serializing_if = "Option::is_none")]
    pub allocated_memory_in_mb: Option<f32>,
    #[doc = "Second level object returned as part of Machine REST resource."]
    #[serde(rename = "operatingSystemDetails", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_details: Option<OperatingSystem>,
    #[doc = "Gets or sets the Machine BIOS serial number."]
    #[serde(rename = "biosSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub bios_serial_number: Option<String>,
    #[doc = "Gets or sets the BIOS GUID."]
    #[serde(rename = "biosGuid", default, skip_serializing_if = "Option::is_none")]
    pub bios_guid: Option<String>,
    #[doc = "Value indicating whether VM is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Gets the marking machine creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets or sets tags on the Server machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets the timestamp marking last updated on the machine."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A run as account resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerRunAsAccount {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for run as account properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunAsAccountProperties>,
}
impl ServerRunAsAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a ServerRunAsAccount list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerRunAsAccountListResult {
    #[doc = "The ServerRunAsAccount items on this page"]
    pub value: Vec<ServerRunAsAccount>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerRunAsAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerRunAsAccountListResult {
    pub fn new(value: Vec<ServerRunAsAccount>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A ServerSiteResource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerSiteResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of SiteResource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SitesProperties>,
}
impl ServerSiteResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a ServerSiteResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerSiteResourceListResult {
    #[doc = "The ServerSiteResource items on this page"]
    pub value: Vec<ServerSiteResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerSiteResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerSiteResourceListResult {
    pub fn new(value: Vec<ServerSiteResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the ServerSiteResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerSiteResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the ServerSiteResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerSiteResourceUpdateProperties>,
}
impl ServerSiteResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the ServerSiteResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerSiteResourceUpdateProperties {
    #[doc = "Class for site properties."]
    #[serde(rename = "servicePrincipalIdentityDetails", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_identity_details: Option<SiteSpnProperties>,
    #[doc = "Class for site agent properties."]
    #[serde(rename = "agentDetails", default, skip_serializing_if = "Option::is_none")]
    pub agent_details: Option<SiteAgentProperties>,
    #[doc = "Gets or sets the Appliance Name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
    #[doc = "Gets or sets the ARM ID of migration hub solution for SDS."]
    #[serde(rename = "discoverySolutionId", default, skip_serializing_if = "Option::is_none")]
    pub discovery_solution_id: Option<String>,
}
impl ServerSiteResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server site usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerSiteUsage {
    #[doc = "Gets or sets the number of run as accounts in the site."]
    #[serde(rename = "runAsAccountCount", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_count: Option<i32>,
    #[doc = "Gets or sets the number of servers part of the site."]
    #[serde(rename = "serverCount", default, skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i32>,
}
impl ServerSiteUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of ServerSiteUsageResponse."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerSiteUsageResponse {
    #[doc = "Gets or sets the number of run as accounts in the site."]
    #[serde(rename = "runAsAccounts")]
    pub run_as_accounts: i32,
    #[doc = "Gets or sets the number of servers part of the site."]
    #[serde(rename = "serverCount")]
    pub server_count: i32,
}
impl ServerSiteUsageResponse {
    pub fn new(run_as_accounts: i32, server_count: i32) -> Self {
        Self {
            run_as_accounts,
            server_count,
        }
    }
}
#[doc = "An software inventory resource belonging to a server resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerSoftwareInventory {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for machine software inventory properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineSoftwareInventoryProperties>,
}
impl ServerSoftwareInventory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a ServerSoftwareInventory list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerSoftwareInventoryListResult {
    #[doc = "The ServerSoftwareInventory items on this page"]
    pub value: Vec<ServerSoftwareInventory>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServerSoftwareInventoryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServerSoftwareInventoryListResult {
    pub fn new(value: Vec<ServerSoftwareInventory>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the Server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerUpdate {
    #[doc = "The updatable properties of the Server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerUpdateProperties>,
}
impl ServerUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the Server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerUpdateProperties {
    #[doc = "Gets or sets the FQDN/IPAddress of the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Gets or sets the network adapters of the server."]
    #[serde(
        rename = "networkAdapters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_adapters: Vec<ServerNetworkAdapter>,
    #[doc = "Gets or sets the FQDN of machine which can be changed."]
    #[serde(rename = "hydratedFqdn", default, skip_serializing_if = "Option::is_none")]
    pub hydrated_fqdn: Option<String>,
    #[doc = "Gets or sets the disk details of server."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disks: Vec<ServerDisk>,
    #[doc = "Gets or sets the validation required for the machine."]
    #[serde(rename = "validationRequired", default, skip_serializing_if = "Option::is_none")]
    pub validation_required: Option<String>,
    #[doc = "Gets or sets the firmware."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    #[doc = "Gets or sets the run as account ID of the machine."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "product support status."]
    #[serde(rename = "productSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_support_status: Option<ProductSupportStatus>,
    #[doc = "Gets or sets the Number of Processor Cores \n            allocated for the\nmachine."]
    #[serde(rename = "numberOfProcessorCore", default, skip_serializing_if = "Option::is_none")]
    pub number_of_processor_core: Option<i32>,
    #[doc = "Gets or sets the allocated Memory in MB."]
    #[serde(rename = "allocatedMemoryInMb", default, skip_serializing_if = "Option::is_none")]
    pub allocated_memory_in_mb: Option<f32>,
    #[doc = "Second level object returned as part of Machine REST resource."]
    #[serde(rename = "operatingSystemDetails", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_details: Option<OperatingSystem>,
    #[doc = "Gets or sets the Machine BIOS serial number."]
    #[serde(rename = "biosSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub bios_serial_number: Option<String>,
    #[doc = "Gets or sets the BIOS GUID."]
    #[serde(rename = "biosGuid", default, skip_serializing_if = "Option::is_none")]
    pub bios_guid: Option<String>,
    #[doc = "Gets or sets tags on the Server machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ServerUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Shallow Discovery Status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ShallowDiscoveryStatus")]
pub enum ShallowDiscoveryStatus {
    DiscoverySucceededAtleastOnce,
    DiscoveryFailed,
    RunAsAccountNotAssociated,
    DiscoveryNotStarted,
    DiscoveryInProgress,
    Disabled,
    DiscoveryPartiallySucceded,
    DiscoverySucceeded,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ShallowDiscoveryStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ShallowDiscoveryStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ShallowDiscoveryStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DiscoverySucceededAtleastOnce => {
                serializer.serialize_unit_variant("ShallowDiscoveryStatus", 0u32, "DiscoverySucceededAtleastOnce")
            }
            Self::DiscoveryFailed => serializer.serialize_unit_variant("ShallowDiscoveryStatus", 1u32, "DiscoveryFailed"),
            Self::RunAsAccountNotAssociated => {
                serializer.serialize_unit_variant("ShallowDiscoveryStatus", 2u32, "RunAsAccountNotAssociated")
            }
            Self::DiscoveryNotStarted => serializer.serialize_unit_variant("ShallowDiscoveryStatus", 3u32, "DiscoveryNotStarted"),
            Self::DiscoveryInProgress => serializer.serialize_unit_variant("ShallowDiscoveryStatus", 4u32, "DiscoveryInProgress"),
            Self::Disabled => serializer.serialize_unit_variant("ShallowDiscoveryStatus", 5u32, "Disabled"),
            Self::DiscoveryPartiallySucceded => {
                serializer.serialize_unit_variant("ShallowDiscoveryStatus", 6u32, "DiscoveryPartiallySucceded")
            }
            Self::DiscoverySucceeded => serializer.serialize_unit_variant("ShallowDiscoveryStatus", 7u32, "DiscoverySucceeded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SharePointServer in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharePointServer {
    #[doc = "Gets or sets ProductName of the SharePointServer."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "Gets or sets a value indicating whether the SharePointServer is Enterprise."]
    #[serde(rename = "isEnterprise", default, skip_serializing_if = "Option::is_none")]
    pub is_enterprise: Option<bool>,
    #[doc = "Gets or sets Status of the SharePointServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets Version of the SharePointServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl SharePointServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for site agent properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteAgentProperties {
    #[doc = "Gets the ID of the agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the version of the agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets the last heartbeat time of the agent in UTC."]
    #[serde(rename = "lastHeartBeatUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_heart_beat_utc: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the key vault URI."]
    #[serde(rename = "keyVaultUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
    #[doc = "Gets or sets the key vault ARM Id."]
    #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_id: Option<String>,
}
impl SiteAgentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for site appliance properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteApplianceProperties {
    #[doc = "Class for site properties."]
    #[serde(rename = "servicePrincipalIdentityDetails", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_identity_details: Option<SiteSpnProperties>,
    #[doc = "Class for site agent properties."]
    #[serde(rename = "agentDetails", default, skip_serializing_if = "Option::is_none")]
    pub agent_details: Option<SiteAgentProperties>,
    #[doc = "Gets or sets the Appliance Name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
}
impl SiteApplianceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Site error summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteErrorSummary {
    #[doc = "Appliance Name."]
    #[serde(rename = "applianceName")]
    pub appliance_name: String,
    #[doc = "The site error summary model."]
    #[serde(rename = "discoveryScopeErrorSummaries")]
    pub discovery_scope_error_summaries: DiscoveryScopeErrorSummary,
    #[doc = "The link to fetch more models."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SiteErrorSummary {
    pub fn new(appliance_name: String, discovery_scope_error_summaries: DiscoveryScopeErrorSummary) -> Self {
        Self {
            appliance_name,
            discovery_scope_error_summaries,
            next_link: None,
        }
    }
}
#[doc = "The properties of ServerSiteResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteHealthSummary {
    #[doc = "Gets the appliance name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
    #[doc = "Gets the error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Gets the summary message."]
    #[serde(rename = "summaryMessage", default, skip_serializing_if = "Option::is_none")]
    pub summary_message: Option<String>,
    #[doc = "Gets the error Id."]
    #[serde(rename = "errorId", default, skip_serializing_if = "Option::is_none")]
    pub error_id: Option<i64>,
    #[doc = "Gets the error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Gets or sets the count of affected objects."]
    #[serde(rename = "affectedObjectsCount", default, skip_serializing_if = "Option::is_none")]
    pub affected_objects_count: Option<i64>,
    #[doc = "Gets or sets the hit count of the error."]
    #[serde(rename = "hitCount", default, skip_serializing_if = "Option::is_none")]
    pub hit_count: Option<i64>,
    #[doc = "Gets the severity of error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "Gets the remediation guidance."]
    #[serde(rename = "remediationGuidance", default, skip_serializing_if = "Option::is_none")]
    pub remediation_guidance: Option<String>,
    #[doc = "Gets the affected resource type."]
    #[serde(rename = "affectedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub affected_resource_type: Option<String>,
    #[doc = "Gets or sets the affected resources."]
    #[serde(
        rename = "affectedResources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub affected_resources: Vec<String>,
    #[doc = "Gets or sets sources of the exception."]
    #[serde(
        rename = "fabricLayoutUpdateSources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fabric_layout_update_sources: Vec<SiteHealthSummaryFabricLayoutUpdateSourcesItem>,
}
impl SiteHealthSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of SiteHealthSummary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteHealthSummaryCollection {
    #[doc = "Gets the list of SiteHealthSummary."]
    pub value: Vec<SiteHealthSummary>,
    #[doc = "Gets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SiteHealthSummaryCollection {
    pub fn new(value: Vec<SiteHealthSummary>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "SiteHealthSummary FabricLayout UpdateSources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SiteHealthSummaryFabricLayoutUpdateSourcesItem")]
pub enum SiteHealthSummaryFabricLayoutUpdateSourcesItem {
    RefreshFabricLayout,
    RefreshFabricLayoutGuest,
    RefreshFabricLayoutDependencyMap,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SiteHealthSummaryFabricLayoutUpdateSourcesItem {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SiteHealthSummaryFabricLayoutUpdateSourcesItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SiteHealthSummaryFabricLayoutUpdateSourcesItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::RefreshFabricLayout => {
                serializer.serialize_unit_variant("SiteHealthSummaryFabricLayoutUpdateSourcesItem", 0u32, "RefreshFabricLayout")
            }
            Self::RefreshFabricLayoutGuest => {
                serializer.serialize_unit_variant("SiteHealthSummaryFabricLayoutUpdateSourcesItem", 1u32, "RefreshFabricLayoutGuest")
            }
            Self::RefreshFabricLayoutDependencyMap => serializer.serialize_unit_variant(
                "SiteHealthSummaryFabricLayoutUpdateSourcesItem",
                2u32,
                "RefreshFabricLayoutDependencyMap",
            ),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties of VMwareSiteResource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteProperties {
    #[doc = "Gets the Master Site this site is linked to."]
    #[serde(rename = "masterSiteId", default, skip_serializing_if = "Option::is_none")]
    pub master_site_id: Option<String>,
    #[doc = "Class for site properties."]
    #[serde(rename = "servicePrincipalIdentityDetails", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_identity_details: Option<SiteSpnProperties>,
    #[doc = "Class for site agent properties."]
    #[serde(rename = "agentDetails", default, skip_serializing_if = "Option::is_none")]
    pub agent_details: Option<SiteAgentProperties>,
    #[doc = "Gets or sets the Appliance Name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
    #[doc = "Gets or sets the ARM ID of migration hub solution for SDS."]
    #[serde(rename = "discoverySolutionId", default, skip_serializing_if = "Option::is_none")]
    pub discovery_solution_id: Option<String>,
    #[doc = "Gets the service endpoint."]
    #[serde(rename = "serviceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl SiteProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for site properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteSpnProperties {
    #[doc = "Gets or sets the tenant Id for the service principal with which the\non-premise\n            management/data plane components would communicate with\nour Azure services."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Gets or sets the application/client Id for the service principal with which\nthe\n            on-premise management/data plane components would communicate\nwith our Azure \n            services."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "Gets or sets the object Id of the service principal with which the on-premise\n\n           management/data plane components would communicate with our Azure\nservices."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Gets or sets the intended audience for the service principal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[doc = "Gets or sets the AAD Authority URL which was used to request the token for\nthe\n            service principal."]
    #[serde(rename = "aadAuthority", default, skip_serializing_if = "Option::is_none")]
    pub aad_authority: Option<String>,
    #[doc = "Gets or sets the raw certificate data for building certificate expiry flows."]
    #[serde(rename = "rawCertData", default, skip_serializing_if = "Option::is_none")]
    pub raw_cert_data: Option<String>,
}
impl SiteSpnProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of SiteResource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SitesProperties {
    #[doc = "Gets the Master Site this site is linked to."]
    #[serde(rename = "masterSiteId", default, skip_serializing_if = "Option::is_none")]
    pub master_site_id: Option<String>,
    #[doc = "Class for site properties."]
    #[serde(rename = "servicePrincipalIdentityDetails", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_identity_details: Option<SiteSpnProperties>,
    #[doc = "Class for site agent properties."]
    #[serde(rename = "agentDetails", default, skip_serializing_if = "Option::is_none")]
    pub agent_details: Option<SiteAgentProperties>,
    #[doc = "Gets or sets the Appliance Name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
    #[doc = "Gets or sets the ARM ID of migration hub solution for SDS."]
    #[serde(rename = "discoverySolutionId", default, skip_serializing_if = "Option::is_none")]
    pub discovery_solution_id: Option<String>,
    #[doc = "Gets the service endpoint."]
    #[serde(rename = "serviceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl SitesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data related to a machine's spring boot discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpringBootDiscovery {
    #[doc = "Gets or sets number of successfully discovered instances."]
    #[serde(rename = "totalInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub total_instance_count: Option<i64>,
    #[doc = "Gets or sets number of successfully discovered applications."]
    #[serde(rename = "totalApplicationCount", default, skip_serializing_if = "Option::is_none")]
    pub total_application_count: Option<i64>,
    #[doc = "Shallow Discovery Status."]
    #[serde(rename = "shallowDiscoveryStatus", default, skip_serializing_if = "Option::is_none")]
    pub shallow_discovery_status: Option<ShallowDiscoveryStatus>,
    #[doc = "Discovery Scope."]
    #[serde(rename = "discoveryScopeStatus", default, skip_serializing_if = "Option::is_none")]
    pub discovery_scope_status: Option<DiscoveryScopeStatus>,
}
impl SpringBootDiscovery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the web model of SQL Availability Group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAvailabilityGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for SQL Server availability group properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlAvailabilityGroupProperties>,
}
impl SqlAvailabilityGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SqlAvailabilityGroup list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlAvailabilityGroupListResult {
    #[doc = "The SqlAvailabilityGroup items on this page"]
    pub value: Vec<SqlAvailabilityGroup>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlAvailabilityGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlAvailabilityGroupListResult {
    pub fn new(value: Vec<SqlAvailabilityGroup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class for SQL Server availability group properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAvailabilityGroupProperties {
    #[doc = "Gets or sets the SQL Availability Group Name."]
    #[serde(rename = "availabilityGroupName", default, skip_serializing_if = "Option::is_none")]
    pub availability_group_name: Option<String>,
    #[doc = "AvailabilityGroupType"]
    #[serde(rename = "availabilityGroupType", default, skip_serializing_if = "Option::is_none")]
    pub availability_group_type: Option<SqlAvailabilityGroupPropertiesAvailabilityGroupType>,
    #[doc = "Gets or sets a value indicating whether AG is multi subnet or not."]
    #[serde(rename = "isMultiSubNet", default, skip_serializing_if = "Option::is_none")]
    pub is_multi_sub_net: Option<bool>,
    #[doc = "Gets or sets the Cluster name where AG is hosted."]
    #[serde(rename = "clusterName", default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[doc = "Gets the list of availability replica properties which together forms this availability group."]
    #[serde(
        rename = "availabilityReplicas",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub availability_replicas: Vec<SqlAvailabilityReplicaProperties>,
    #[doc = "Gets the parent availability replica overview if any.\n            This would be\nset with details of parent AG and AR for cases where this availability group is\na part of a distributed AG.\n            Currently, we do not populate this\nsince discovery and linking of DAG(Distributed Availability Group) is not\nimplemented."]
    #[serde(
        rename = "parentReplicaOverviewList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parent_replica_overview_list: Vec<SqlAvailabilityReplicaOverview>,
    #[doc = "Gets or sets a value indicating whether this Availability group is part of a\ndistributed AG."]
    #[serde(rename = "isPartOfDistributedAvailabilityGroup", default, skip_serializing_if = "Option::is_none")]
    pub is_part_of_distributed_availability_group: Option<bool>,
    #[doc = "Gets or sets a value indicating whether the entity is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Gets or sets the marking machine creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets or sets the timestamp marking last updated on the machine."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl SqlAvailabilityGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AvailabilityGroupType"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlAvailabilityGroupPropertiesAvailabilityGroupType")]
pub enum SqlAvailabilityGroupPropertiesAvailabilityGroupType {
    Unknown,
    Traditional,
    Distributed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlAvailabilityGroupPropertiesAvailabilityGroupType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlAvailabilityGroupPropertiesAvailabilityGroupType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlAvailabilityGroupPropertiesAvailabilityGroupType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("SqlAvailabilityGroupPropertiesAvailabilityGroupType", 0u32, "Unknown"),
            Self::Traditional => {
                serializer.serialize_unit_variant("SqlAvailabilityGroupPropertiesAvailabilityGroupType", 1u32, "Traditional")
            }
            Self::Distributed => {
                serializer.serialize_unit_variant("SqlAvailabilityGroupPropertiesAvailabilityGroupType", 2u32, "Distributed")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Class which represents the SQL availability replica properties of type AG."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAvailabilityGroupReplicaInfo {
    #[doc = "Gets or sets the name of the cluster on which this replica is hosted."]
    #[serde(rename = "clusterName", default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[doc = "Gets or sets the name of the availability replica."]
    #[serde(rename = "availabilityGroupName", default, skip_serializing_if = "Option::is_none")]
    pub availability_group_name: Option<String>,
    #[doc = "Gets or sets the AG ARM ID which are part of this Replica."]
    #[serde(rename = "availabilityGroupArmId", default, skip_serializing_if = "Option::is_none")]
    pub availability_group_arm_id: Option<String>,
}
impl SqlAvailabilityGroupReplicaInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ReplicaCommitMode"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaCommitMode")]
pub enum SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaCommitMode {
    Unknown,
    Synchronous,
    Asynchronous,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaCommitMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaCommitMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaCommitMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant(
                "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaCommitMode",
                0u32,
                "Unknown",
            ),
            Self::Synchronous => serializer.serialize_unit_variant(
                "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaCommitMode",
                1u32,
                "Synchronous",
            ),
            Self::Asynchronous => serializer.serialize_unit_variant(
                "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaCommitMode",
                2u32,
                "Asynchronous",
            ),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "ReplicaReadMode"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaReadMode")]
pub enum SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaReadMode {
    Unknown,
    None,
    ReadOnly,
    ReadWrite,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaReadMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaReadMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaReadMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant(
                "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaReadMode",
                0u32,
                "Unknown",
            ),
            Self::None => {
                serializer.serialize_unit_variant("SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaReadMode", 1u32, "None")
            }
            Self::ReadOnly => serializer.serialize_unit_variant(
                "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaReadMode",
                2u32,
                "ReadOnly",
            ),
            Self::ReadWrite => serializer.serialize_unit_variant(
                "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaReadMode",
                3u32,
                "ReadWrite",
            ),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "ReplicaSeedMode"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSeedMode")]
pub enum SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSeedMode {
    Unknown,
    Manual,
    Automatic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSeedMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSeedMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSeedMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant(
                "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSeedMode",
                0u32,
                "Unknown",
            ),
            Self::Manual => serializer.serialize_unit_variant(
                "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSeedMode",
                1u32,
                "Manual",
            ),
            Self::Automatic => serializer.serialize_unit_variant(
                "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSeedMode",
                2u32,
                "Automatic",
            ),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "ReplicaState"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaState")]
pub enum SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaState {
    Unknown,
    Primary,
    Secondary,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => {
                serializer.serialize_unit_variant("SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaState", 0u32, "Unknown")
            }
            Self::Primary => {
                serializer.serialize_unit_variant("SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaState", 1u32, "Primary")
            }
            Self::Secondary => serializer.serialize_unit_variant(
                "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaState",
                2u32,
                "Secondary",
            ),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "ReplicaSyncStatus"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSyncStatus")]
pub enum SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSyncStatus {
    Unknown,
    Synchronized,
    Unsynchronized,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSyncStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSyncStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSyncStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant(
                "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSyncStatus",
                0u32,
                "Unknown",
            ),
            Self::Synchronized => serializer.serialize_unit_variant(
                "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSyncStatus",
                1u32,
                "Synchronized",
            ),
            Self::Unsynchronized => serializer.serialize_unit_variant(
                "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSyncStatus",
                2u32,
                "Unsynchronized",
            ),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "ReplicaType"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaType")]
pub enum SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaType {
    Unknown,
    DatabaseReplica,
    AvailabilityGroupReplica,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => {
                serializer.serialize_unit_variant("SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaType", 0u32, "Unknown")
            }
            Self::DatabaseReplica => serializer.serialize_unit_variant(
                "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaType",
                1u32,
                "DatabaseReplica",
            ),
            Self::AvailabilityGroupReplica => serializer.serialize_unit_variant(
                "SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaType",
                2u32,
                "AvailabilityGroupReplica",
            ),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Web model for SQL replica overview."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAvailabilityReplicaOverview {
    #[doc = "Overview Replica State"]
    #[serde(rename = "replicaState", default, skip_serializing_if = "Option::is_none")]
    pub replica_state: Option<SqlAvailabilityReplicaOverviewReplicaState>,
    #[doc = "Gets or sets the SQL Availability Replica Id within the Availability Group this\ndatabase is a part of."]
    #[serde(rename = "availabilityReplicaId", default, skip_serializing_if = "Option::is_none")]
    pub availability_replica_id: Option<String>,
    #[doc = "Gets or sets the Availability Group Id in which this database participates if\nany. It would be set if the database has\n{Microsoft.Azure.FDS.CosmosDB.SqlDatabaseEntity.IsDatabaseHighlyAvailable} has\na value \"true\"."]
    #[serde(rename = "availabilityGroupArmId", default, skip_serializing_if = "Option::is_none")]
    pub availability_group_arm_id: Option<String>,
    #[doc = "Gets or sets the SQL Availability group name."]
    #[serde(rename = "availabilityGroupName", default, skip_serializing_if = "Option::is_none")]
    pub availability_group_name: Option<String>,
}
impl SqlAvailabilityReplicaOverview {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Overview Replica State"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlAvailabilityReplicaOverviewReplicaState")]
pub enum SqlAvailabilityReplicaOverviewReplicaState {
    Unknown,
    Primary,
    Secondary,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlAvailabilityReplicaOverviewReplicaState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlAvailabilityReplicaOverviewReplicaState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlAvailabilityReplicaOverviewReplicaState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("SqlAvailabilityReplicaOverviewReplicaState", 0u32, "Unknown"),
            Self::Primary => serializer.serialize_unit_variant("SqlAvailabilityReplicaOverviewReplicaState", 1u32, "Primary"),
            Self::Secondary => serializer.serialize_unit_variant("SqlAvailabilityReplicaOverviewReplicaState", 2u32, "Secondary"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Class for SQL Server availability replica properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAvailabilityReplicaProperties {
    #[doc = "Gets or sets the SQL Availability Replica Name."]
    #[serde(rename = "availabilityReplicaName", default, skip_serializing_if = "Option::is_none")]
    pub availability_replica_name: Option<String>,
    #[doc = "Gets or sets the SQL Availability Replica Id."]
    #[serde(rename = "availabilityReplicaId", default, skip_serializing_if = "Option::is_none")]
    pub availability_replica_id: Option<String>,
    #[doc = "ReplicaType"]
    #[serde(rename = "replicaType", default, skip_serializing_if = "Option::is_none")]
    pub replica_type: Option<SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaType>,
    #[doc = "ReplicaState"]
    #[serde(rename = "replicaState", default, skip_serializing_if = "Option::is_none")]
    pub replica_state: Option<SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaState>,
    #[doc = "ReplicaSyncStatus"]
    #[serde(rename = "replicaSyncStatus", default, skip_serializing_if = "Option::is_none")]
    pub replica_sync_status: Option<SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSyncStatus>,
    #[doc = "ReplicaCommitMode"]
    #[serde(rename = "replicaCommitMode", default, skip_serializing_if = "Option::is_none")]
    pub replica_commit_mode: Option<SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaCommitMode>,
    #[doc = "ReplicaReadMode"]
    #[serde(rename = "replicaReadMode", default, skip_serializing_if = "Option::is_none")]
    pub replica_read_mode: Option<SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaReadMode>,
    #[doc = "ReplicaSeedMode"]
    #[serde(rename = "replicaSeedMode", default, skip_serializing_if = "Option::is_none")]
    pub replica_seed_mode: Option<SqlAvailabilityGroupSqlAvailabilityReplicaPropertiesReplicaSeedMode>,
    #[doc = "Class which represents the SQL availability replica properties of type\ndatabases."]
    #[serde(rename = "sqlDatabaseReplicaInfo", default, skip_serializing_if = "Option::is_none")]
    pub sql_database_replica_info: Option<SqlDatabaseReplicaInfo>,
    #[doc = "Class which represents the SQL availability replica properties of type AG."]
    #[serde(rename = "sqlAvailabilityGroupReplicaInfo", default, skip_serializing_if = "Option::is_none")]
    pub sql_availability_group_replica_info: Option<SqlAvailabilityGroupReplicaInfo>,
}
impl SqlAvailabilityReplicaProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for SQL Server database properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDatabasePropertiesV2 {
    #[doc = "Web model for SQL replica overview."]
    #[serde(rename = "parentReplicaOverview", default, skip_serializing_if = "Option::is_none")]
    pub parent_replica_overview: Option<SqlAvailabilityReplicaOverview>,
    #[doc = "Gets or sets a value indicating whether this database is a part of an HA setup."]
    #[serde(rename = "isDatabaseHighlyAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_database_highly_available: Option<bool>,
    #[doc = "Gets the file metadata list."]
    #[serde(
        rename = "fileMetadataList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub file_metadata_list: Vec<FileMetaData>,
    #[doc = "Gets or sets the hostname."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "Gets or sets the SQL Server Id in which the database resides."]
    #[serde(rename = "sqlServerName", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_name: Option<String>,
    #[doc = "Gets or sets the status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets the size in Mb."]
    #[serde(rename = "sizeMb", default, skip_serializing_if = "Option::is_none")]
    pub size_mb: Option<f32>,
    #[doc = "Gets or sets the SQL database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "Gets or sets the SQL Server ARM Id in which the database resides."]
    #[serde(rename = "sqlServerArmId", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_arm_id: Option<String>,
    #[doc = "Gets or sets the database compatibility level."]
    #[serde(rename = "compatibilityLevel", default, skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<String>,
    #[doc = "Gets or sets a value indicating whether the entity is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Gets the errors."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<Errors>,
    #[doc = "Gets or sets the marking machine creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets or sets the timestamp marking last updated on the machine."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl SqlDatabasePropertiesV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class which represents the SQL availability replica properties of type\ndatabases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDatabaseReplicaInfo {
    #[doc = "Gets or sets the host name of the availability replica."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "Gets or sets the SQL Server name of the availability replica."]
    #[serde(rename = "sqlServerName", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_name: Option<String>,
    #[doc = "Gets or sets the SQL Server name of the availability replica."]
    #[serde(rename = "sqlServerArmId", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_arm_id: Option<String>,
}
impl SqlDatabaseReplicaInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the web model of SQL Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDatabaseV2 {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for SQL Server database properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlDatabasePropertiesV2>,
}
impl SqlDatabaseV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SqlDatabaseV2 list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDatabaseV2ListResult {
    #[doc = "The SqlDatabaseV2 items on this page"]
    pub value: Vec<SqlDatabaseV2>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlDatabaseV2ListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlDatabaseV2ListResult {
    pub fn new(value: Vec<SqlDatabaseV2>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Contains data related SQL discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDiscovery {
    #[doc = " successfully Discovered ServerCount"]
    #[serde(rename = "successfullyDiscoveredServerCount", default, skip_serializing_if = "Option::is_none")]
    pub successfully_discovered_server_count: Option<i32>,
    #[doc = "total Server Count"]
    #[serde(rename = "totalServerCount", default, skip_serializing_if = "Option::is_none")]
    pub total_server_count: Option<i32>,
    #[doc = "sql Metadata Hydrated RunAsAccountId"]
    #[serde(rename = "sqlMetadataHydratedRunAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub sql_metadata_hydrated_run_as_account_id: Option<String>,
    #[doc = "SQL metadata discovery pipe"]
    #[serde(rename = "sqlMetadataDiscoveryPipe", default, skip_serializing_if = "Option::is_none")]
    pub sql_metadata_discovery_pipe: Option<SqlMetadataDiscoveryPipe>,
    #[doc = "SQLDiscoveryScope Status"]
    #[serde(rename = "discoveryScopeStatus", default, skip_serializing_if = "Option::is_none")]
    pub discovery_scope_status: Option<SqlDiscoveryScopeStatus>,
}
impl SqlDiscovery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A SQL discovery site data source resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDiscoverySiteDataSource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Discovery site data source properties class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlDiscoverySiteDataSourceProperties>,
}
impl SqlDiscoverySiteDataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SqlDiscoverySiteDataSource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDiscoverySiteDataSourceListResult {
    #[doc = "The SqlDiscoverySiteDataSource items on this page"]
    pub value: Vec<SqlDiscoverySiteDataSource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlDiscoverySiteDataSourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlDiscoverySiteDataSourceListResult {
    pub fn new(value: Vec<SqlDiscoverySiteDataSource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Discovery site data source properties class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDiscoverySiteDataSourceProperties {
    #[doc = "Gets or sets the discovery site Id."]
    #[serde(rename = "discoverySiteId", default, skip_serializing_if = "Option::is_none")]
    pub discovery_site_id: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl SqlDiscoverySiteDataSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The class representing the properties for an FCI instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlFciProperties {
    #[doc = "fci instance state "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<FciInstanceState>,
    #[doc = "Gets or sets the FCI Network Name used to connect to this FCI instance."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "Gets or sets a value indicating whether the FCI is hosted on possible owners\nwhich are in multiple subnets."]
    #[serde(rename = "isMultiSubnet", default, skip_serializing_if = "Option::is_none")]
    pub is_multi_subnet: Option<bool>,
    #[doc = "Gets or sets the count of Shared Disks for SQL FCI."]
    #[serde(rename = "sharedDiskCount", default, skip_serializing_if = "Option::is_none")]
    pub shared_disk_count: Option<i32>,
}
impl SqlFciProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the web model of SQL Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlJob {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Job REST Resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
impl SqlJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SqlJob list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlJobListResult {
    #[doc = "The SqlJob items on this page"]
    pub value: Vec<SqlJob>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlJobListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlJobListResult {
    pub fn new(value: Vec<SqlJob>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Gets or sets SQL machine overview data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlMachineOverview {
    #[doc = "Gets or sets SQL machine ARM ID."]
    #[serde(rename = "machineArmId", default, skip_serializing_if = "Option::is_none")]
    pub machine_arm_id: Option<String>,
    #[doc = "Gets or sets SQL machine display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "sql fci role "]
    #[serde(rename = "fciRole", default, skip_serializing_if = "Option::is_none")]
    pub fci_role: Option<SqlMachineOverviewFciRole>,
}
impl SqlMachineOverview {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "sql fci role "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlMachineOverviewFciRole")]
pub enum SqlMachineOverviewFciRole {
    Unknown,
    NotApplicable,
    ActiveNode,
    PossibleOwnerNode,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlMachineOverviewFciRole {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlMachineOverviewFciRole {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlMachineOverviewFciRole {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("SqlMachineOverviewFciRole", 0u32, "Unknown"),
            Self::NotApplicable => serializer.serialize_unit_variant("SqlMachineOverviewFciRole", 1u32, "NotApplicable"),
            Self::ActiveNode => serializer.serialize_unit_variant("SqlMachineOverviewFciRole", 2u32, "ActiveNode"),
            Self::PossibleOwnerNode => serializer.serialize_unit_variant("SqlMachineOverviewFciRole", 3u32, "PossibleOwnerNode"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SQL metadata discovery pipe"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlMetadataDiscoveryPipe")]
pub enum SqlMetadataDiscoveryPipe {
    Unknown,
    VMware,
    PowerShell,
    #[serde(rename = "SSH")]
    Ssh,
    #[serde(rename = "CIM")]
    Cim,
    Other,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlMetadataDiscoveryPipe {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlMetadataDiscoveryPipe {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlMetadataDiscoveryPipe {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("SqlMetadataDiscoveryPipe", 0u32, "Unknown"),
            Self::VMware => serializer.serialize_unit_variant("SqlMetadataDiscoveryPipe", 1u32, "VMware"),
            Self::PowerShell => serializer.serialize_unit_variant("SqlMetadataDiscoveryPipe", 2u32, "PowerShell"),
            Self::Ssh => serializer.serialize_unit_variant("SqlMetadataDiscoveryPipe", 3u32, "SSH"),
            Self::Cim => serializer.serialize_unit_variant("SqlMetadataDiscoveryPipe", 4u32, "CIM"),
            Self::Other => serializer.serialize_unit_variant("SqlMetadataDiscoveryPipe", 5u32, "Other"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A runasaccount resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlRunAsAccount {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for run as account properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunAsAccountProperties>,
}
impl SqlRunAsAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SqlRunAsAccount list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlRunAsAccountListResult {
    #[doc = "The SqlRunAsAccount items on this page"]
    pub value: Vec<SqlRunAsAccount>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlRunAsAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlRunAsAccountListResult {
    pub fn new(value: Vec<SqlRunAsAccount>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "SQLServer in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerApplication {
    #[doc = "Gets or sets Name of the SQLServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets Edition of the SQLServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[doc = "Gets or sets ServicePack of the SQLServer."]
    #[serde(rename = "servicePack", default, skip_serializing_if = "Option::is_none")]
    pub service_pack: Option<String>,
    #[doc = "Gets or sets Version of the SQLServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets or sets Clustered of the SQLServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clustered: Option<String>,
    #[doc = "Gets or sets ClusterName of the SQLServer."]
    #[serde(rename = "clusterName", default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[doc = "Gets or sets the DNS host name of SQLServer."]
    #[serde(rename = "dnsHostName", default, skip_serializing_if = "Option::is_none")]
    pub dns_host_name: Option<String>,
    #[doc = "Gets or sets the port of the SQLServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[doc = "Gets or sets the comma separated IPs of the SQLServer."]
    #[serde(rename = "commaSeparatedIps", default, skip_serializing_if = "Option::is_none")]
    pub comma_separated_ips: Option<String>,
    #[doc = "Gets the status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets the value which reflects if Named Pipe is enabled or not."]
    #[serde(rename = "isNamedPipeEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_named_pipe_enabled: Option<bool>,
    #[doc = "Gets or sets the value which reflects if Named Pipe is enabled or not."]
    #[serde(rename = "isTcpIpEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_tcp_ip_enabled: Option<bool>,
    #[doc = "Gets the status."]
    #[serde(rename = "namedPipeName", default, skip_serializing_if = "Option::is_none")]
    pub named_pipe_name: Option<String>,
}
impl SqlServerApplication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for SQL Server properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerProperties {
    #[doc = "Gets the Machine Overview properties of all machines on which the SQL server is deployed."]
    #[serde(
        rename = "machineOverviewList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machine_overview_list: Vec<SqlMachineOverview>,
    #[doc = "Gets or sets the number of databases part of availability group on this SQL instance."]
    #[serde(rename = "numberOfAgDatabases", default, skip_serializing_if = "Option::is_none")]
    pub number_of_ag_databases: Option<i32>,
    #[doc = "The class representing the properties for an FCI instance."]
    #[serde(rename = "sqlFciProperties", default, skip_serializing_if = "Option::is_none")]
    pub sql_fci_properties: Option<SqlFciProperties>,
    #[doc = "product support status."]
    #[serde(rename = "productSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_support_status: Option<ProductSupportStatus>,
    #[doc = "Gets or sets the SQL server version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets or sets total number of users databases."]
    #[serde(rename = "numberOfUserDatabases", default, skip_serializing_if = "Option::is_none")]
    pub number_of_user_databases: Option<i32>,
    #[doc = "Gets or sets total size of all user databases."]
    #[serde(rename = "sumOfUserDatabasesSizeInMb", default, skip_serializing_if = "Option::is_none")]
    pub sum_of_user_databases_size_in_mb: Option<f32>,
    #[doc = "Gets or sets size of temp database."]
    #[serde(rename = "tempDbSizeInMb", default, skip_serializing_if = "Option::is_none")]
    pub temp_db_size_in_mb: Option<f32>,
    #[doc = "Gets or sets maximum server memory in use."]
    #[serde(rename = "maxServerMemoryInUseInMb", default, skip_serializing_if = "Option::is_none")]
    pub max_server_memory_in_use_in_mb: Option<f32>,
    #[doc = "Gets or sets the SQL server number of cores that have visible online status."]
    #[serde(rename = "visibleOnlineCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub visible_online_core_count: Option<i32>,
    #[doc = "Gets or sets number of logins."]
    #[serde(rename = "numOfLogins", default, skip_serializing_if = "Option::is_none")]
    pub num_of_logins: Option<i32>,
    #[doc = "Gets or sets physical CPU count."]
    #[serde(rename = "physicalCpuCount", default, skip_serializing_if = "Option::is_none")]
    pub physical_cpu_count: Option<f32>,
    #[doc = "Gets or sets logical CPU count."]
    #[serde(rename = "logicalCpuCount", default, skip_serializing_if = "Option::is_none")]
    pub logical_cpu_count: Option<i32>,
    #[doc = "Gets or sets the SQL server engine edition."]
    #[serde(rename = "engineEdition", default, skip_serializing_if = "Option::is_none")]
    pub engine_edition: Option<String>,
    #[doc = "Gets or sets the SQL server edition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[doc = "Gets or sets a value indicating whether High Availability is enabled or not."]
    #[serde(rename = "isHighAvailabilityEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_high_availability_enabled: Option<bool>,
    #[doc = "Gets or sets a value indicating whether failover cluster is configured or not."]
    #[serde(rename = "isClustered", default, skip_serializing_if = "Option::is_none")]
    pub is_clustered: Option<bool>,
    #[doc = "Gets or sets the Hyper thread ratio."]
    #[serde(rename = "hyperthreadRatio", default, skip_serializing_if = "Option::is_none")]
    pub hyperthread_ratio: Option<i32>,
    #[doc = "Gets or sets the SQL start time."]
    #[serde(rename = "sqlStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub sql_start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the list of machine ARM Ids on which the SQL server is deployed."]
    #[serde(
        rename = "machineArmIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machine_arm_ids: Vec<String>,
    #[doc = "Gets or sets the run as account ID of the SQL server."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Gets or sets the hydrated run as account ID of the SQL server."]
    #[serde(rename = "hydratedRunAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub hydrated_run_as_account_id: Option<String>,
    #[doc = "Gets or sets hostname."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "Gets or sets the SQL server instance name."]
    #[serde(rename = "sqlServerName", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_name: Option<String>,
    #[doc = "Gets or sets the Preferred Port Number."]
    #[serde(rename = "portNumber", default, skip_serializing_if = "Option::is_none")]
    pub port_number: Option<i32>,
    #[doc = "Gets the errors."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<Errors>,
    #[doc = "Gets or sets tags on the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets a value indicating whether the entity is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Gets or sets the marking machine creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets or sets the timestamp marking last updated on the machine."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "sql server status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SqlServerStatus>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl SqlServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "sql server status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlServerStatus")]
pub enum SqlServerStatus {
    Unknown,
    ContinuePending,
    Paused,
    PausePending,
    Running,
    StartPending,
    Stopped,
    StopPending,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlServerStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlServerStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlServerStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("SqlServerStatus", 0u32, "Unknown"),
            Self::ContinuePending => serializer.serialize_unit_variant("SqlServerStatus", 1u32, "ContinuePending"),
            Self::Paused => serializer.serialize_unit_variant("SqlServerStatus", 2u32, "Paused"),
            Self::PausePending => serializer.serialize_unit_variant("SqlServerStatus", 3u32, "PausePending"),
            Self::Running => serializer.serialize_unit_variant("SqlServerStatus", 4u32, "Running"),
            Self::StartPending => serializer.serialize_unit_variant("SqlServerStatus", 5u32, "StartPending"),
            Self::Stopped => serializer.serialize_unit_variant("SqlServerStatus", 6u32, "Stopped"),
            Self::StopPending => serializer.serialize_unit_variant("SqlServerStatus", 7u32, "StopPending"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Class representing the web model of SQL Server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerV2 {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for SQL Server properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlServerProperties>,
}
impl SqlServerV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SqlServerV2 list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerV2ListResult {
    #[doc = "The SqlServerV2 items on this page"]
    pub value: Vec<SqlServerV2>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlServerV2ListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlServerV2ListResult {
    pub fn new(value: Vec<SqlServerV2>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the SqlServerV2."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerV2Update {
    #[doc = "The updatable properties of the SqlServerV2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlServerV2UpdateProperties>,
}
impl SqlServerV2Update {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the SqlServerV2."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerV2UpdateProperties {
    #[doc = "Gets or sets the number of databases part of availability group on this SQL instance."]
    #[serde(rename = "numberOfAgDatabases", default, skip_serializing_if = "Option::is_none")]
    pub number_of_ag_databases: Option<i32>,
    #[doc = "The class representing the properties for an FCI instance."]
    #[serde(rename = "sqlFciProperties", default, skip_serializing_if = "Option::is_none")]
    pub sql_fci_properties: Option<SqlFciProperties>,
    #[doc = "product support status."]
    #[serde(rename = "productSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_support_status: Option<ProductSupportStatus>,
    #[doc = "Gets or sets the SQL server version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets or sets total number of users databases."]
    #[serde(rename = "numberOfUserDatabases", default, skip_serializing_if = "Option::is_none")]
    pub number_of_user_databases: Option<i32>,
    #[doc = "Gets or sets total size of all user databases."]
    #[serde(rename = "sumOfUserDatabasesSizeInMb", default, skip_serializing_if = "Option::is_none")]
    pub sum_of_user_databases_size_in_mb: Option<f32>,
    #[doc = "Gets or sets size of temp database."]
    #[serde(rename = "tempDbSizeInMb", default, skip_serializing_if = "Option::is_none")]
    pub temp_db_size_in_mb: Option<f32>,
    #[doc = "Gets or sets maximum server memory in use."]
    #[serde(rename = "maxServerMemoryInUseInMb", default, skip_serializing_if = "Option::is_none")]
    pub max_server_memory_in_use_in_mb: Option<f32>,
    #[doc = "Gets or sets the SQL server number of cores that have visible online status."]
    #[serde(rename = "visibleOnlineCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub visible_online_core_count: Option<i32>,
    #[doc = "Gets or sets number of logins."]
    #[serde(rename = "numOfLogins", default, skip_serializing_if = "Option::is_none")]
    pub num_of_logins: Option<i32>,
    #[doc = "Gets or sets physical CPU count."]
    #[serde(rename = "physicalCpuCount", default, skip_serializing_if = "Option::is_none")]
    pub physical_cpu_count: Option<f32>,
    #[doc = "Gets or sets logical CPU count."]
    #[serde(rename = "logicalCpuCount", default, skip_serializing_if = "Option::is_none")]
    pub logical_cpu_count: Option<i32>,
    #[doc = "Gets or sets the SQL server engine edition."]
    #[serde(rename = "engineEdition", default, skip_serializing_if = "Option::is_none")]
    pub engine_edition: Option<String>,
    #[doc = "Gets or sets the SQL server edition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[doc = "Gets or sets a value indicating whether High Availability is enabled or not."]
    #[serde(rename = "isHighAvailabilityEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_high_availability_enabled: Option<bool>,
    #[doc = "Gets or sets a value indicating whether failover cluster is configured or not."]
    #[serde(rename = "isClustered", default, skip_serializing_if = "Option::is_none")]
    pub is_clustered: Option<bool>,
    #[doc = "Gets or sets the Hyper thread ratio."]
    #[serde(rename = "hyperthreadRatio", default, skip_serializing_if = "Option::is_none")]
    pub hyperthread_ratio: Option<i32>,
    #[doc = "Gets or sets the SQL start time."]
    #[serde(rename = "sqlStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub sql_start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the run as account ID of the SQL server."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Gets or sets the hydrated run as account ID of the SQL server."]
    #[serde(rename = "hydratedRunAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub hydrated_run_as_account_id: Option<String>,
    #[doc = "Gets or sets hostname."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "Gets or sets the SQL server instance name."]
    #[serde(rename = "sqlServerName", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_name: Option<String>,
    #[doc = "Gets or sets the Preferred Port Number."]
    #[serde(rename = "portNumber", default, skip_serializing_if = "Option::is_none")]
    pub port_number: Option<i32>,
    #[doc = "Gets or sets tags on the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets or sets a value indicating whether the entity is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Gets or sets the marking machine creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets or sets the timestamp marking last updated on the machine."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "sql server status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SqlServerStatus>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl SqlServerV2UpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQL site web model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlSite {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for SQL site properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlSiteProperties>,
}
impl SqlSite {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a SqlSite list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlSiteListResult {
    #[doc = "The SqlSite items on this page"]
    pub value: Vec<SqlSite>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlSiteListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlSiteListResult {
    pub fn new(value: Vec<SqlSite>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class for SQL site properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlSiteProperties {
    #[doc = "Gets or sets the appliance details used by service to communicate\n           \nto the appliance."]
    #[serde(
        rename = "siteAppliancePropertiesCollection",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub site_appliance_properties_collection: Vec<SiteApplianceProperties>,
    #[doc = "SqlSiteProperties DiscoveryScenario"]
    #[serde(rename = "discoveryScenario", default, skip_serializing_if = "Option::is_none")]
    pub discovery_scenario: Option<SqlSitePropertiesDiscoveryScenario>,
    #[doc = "Gets the service endpoint."]
    #[serde(rename = "serviceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl SqlSiteProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SqlSiteProperties DiscoveryScenario"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SqlSitePropertiesDiscoveryScenario")]
pub enum SqlSitePropertiesDiscoveryScenario {
    Migrate,
    #[serde(rename = "DR")]
    Dr,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SqlSitePropertiesDiscoveryScenario {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SqlSitePropertiesDiscoveryScenario {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SqlSitePropertiesDiscoveryScenario {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Migrate => serializer.serialize_unit_variant("SqlSitePropertiesDiscoveryScenario", 0u32, "Migrate"),
            Self::Dr => serializer.serialize_unit_variant("SqlSitePropertiesDiscoveryScenario", 1u32, "DR"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SQL site refresh."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlSiteRefreshBody {
    #[doc = "Gets or sets the appliance name of the agent in the site."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
}
impl SqlSiteRefreshBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for update operations of the SqlSite."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlSiteUpdate {
    #[doc = "The updatable properties of the SqlSite."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlSiteUpdateProperties>,
}
impl SqlSiteUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the SqlSite."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlSiteUpdateProperties {
    #[doc = "Gets or sets the appliance details used by service to communicate\n           \nto the appliance."]
    #[serde(
        rename = "siteAppliancePropertiesCollection",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub site_appliance_properties_collection: Vec<SiteApplianceProperties>,
    #[doc = "SqlSiteProperties DiscoveryScenario"]
    #[serde(rename = "discoveryScenario", default, skip_serializing_if = "Option::is_none")]
    pub discovery_scenario: Option<SqlSitePropertiesDiscoveryScenario>,
}
impl SqlSiteUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SQL site usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlSiteUsage {
    #[doc = "Gets or sets the number of servers discovered in the site."]
    #[serde(rename = "serverCount", default, skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i32>,
    #[doc = "Gets or sets the number of databases discovered in the site."]
    #[serde(rename = "databaseCount", default, skip_serializing_if = "Option::is_none")]
    pub database_count: Option<i32>,
    #[doc = "Gets or sets the number of run as accounts in the site."]
    #[serde(rename = "runAsAccountCount", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_count: Option<i32>,
}
impl SqlSiteUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains data related static data discovery scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticDiscovery {
    #[doc = "Static DiscoveryScopeStatus"]
    #[serde(rename = "discoveryScopeStatus", default, skip_serializing_if = "Option::is_none")]
    pub discovery_scope_status: Option<StaticDiscoveryScopeStatus>,
    #[doc = "Gets errors for discovery scope."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "Gets the run as account ID with which feature worked successfully.\n           \nIt is discovered by the agent from the list of credentials."]
    #[serde(rename = "hydratedRunAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub hydrated_run_as_account_id: Option<String>,
}
impl StaticDiscovery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Static DiscoveryScopeStatus"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StaticDiscoveryScopeStatus")]
pub enum StaticDiscoveryScopeStatus {
    DiscoverySucceededAtleastOnce,
    DiscoveryFailed,
    RunAsAccountNotAssociated,
    DiscoveryNotStarted,
    DiscoveryInProgress,
    Disabled,
    DiscoveryPartiallySucceded,
    DiscoverySucceeded,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StaticDiscoveryScopeStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StaticDiscoveryScopeStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StaticDiscoveryScopeStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DiscoverySucceededAtleastOnce => {
                serializer.serialize_unit_variant("StaticDiscoveryScopeStatus", 0u32, "DiscoverySucceededAtleastOnce")
            }
            Self::DiscoveryFailed => serializer.serialize_unit_variant("StaticDiscoveryScopeStatus", 1u32, "DiscoveryFailed"),
            Self::RunAsAccountNotAssociated => {
                serializer.serialize_unit_variant("StaticDiscoveryScopeStatus", 2u32, "RunAsAccountNotAssociated")
            }
            Self::DiscoveryNotStarted => serializer.serialize_unit_variant("StaticDiscoveryScopeStatus", 3u32, "DiscoveryNotStarted"),
            Self::DiscoveryInProgress => serializer.serialize_unit_variant("StaticDiscoveryScopeStatus", 4u32, "DiscoveryInProgress"),
            Self::Disabled => serializer.serialize_unit_variant("StaticDiscoveryScopeStatus", 5u32, "Disabled"),
            Self::DiscoveryPartiallySucceded => {
                serializer.serialize_unit_variant("StaticDiscoveryScopeStatus", 6u32, "DiscoveryPartiallySucceded")
            }
            Self::DiscoverySucceeded => serializer.serialize_unit_variant("StaticDiscoveryScopeStatus", 7u32, "DiscoverySucceeded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SystemCenter in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemCenter {
    #[doc = "Gets or sets ProductName of the SystemCenter."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "Gets or sets Status of the SystemCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets Version of the SystemCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl SystemCenter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Machine class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsMachineInput {
    #[doc = "Gets or sets the ARM id of the machine."]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "run as AccountId"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsMachineInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tomcat engine data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TomcatEngineUnit {
    #[doc = "Gets or sets the name of the engine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the hosts defined for the engine."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hosts: Vec<TomcatHostUnit>,
}
impl TomcatEngineUnit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tomcat host data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TomcatHostUnit {
    #[doc = "Gets or sets the name of the host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the app base path for the host."]
    #[serde(rename = "appBasePath", default, skip_serializing_if = "Option::is_none")]
    pub app_base_path: Option<String>,
    #[doc = "Gets or sets the app base value configured for the host."]
    #[serde(rename = "appBase", default, skip_serializing_if = "Option::is_none")]
    pub app_base: Option<String>,
}
impl TomcatHostUnit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tomcat service data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TomcatServiceUnit {
    #[doc = "Gets or sets the name of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the connectors defined for the service component."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub connectors: Vec<ConnectorUnit>,
    #[doc = "Tomcat engine data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine: Option<TomcatEngineUnit>,
}
impl TomcatServiceUnit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for web application properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TomcatWebApplicationProperties {
    #[doc = "Boolean value having true if the application has database dependency."]
    #[serde(rename = "hasDatabaseDependency", default, skip_serializing_if = "Option::is_none")]
    pub has_database_dependency: Option<bool>,
    #[doc = "Boolean value having true if the application has file dependency."]
    #[serde(rename = "isExternalLoggingConfigured", default, skip_serializing_if = "Option::is_none")]
    pub is_external_logging_configured: Option<bool>,
    #[doc = "Gets the web server id."]
    #[serde(rename = "webServerId", default, skip_serializing_if = "Option::is_none")]
    pub web_server_id: Option<String>,
    #[doc = "Gets the web server name."]
    #[serde(rename = "webServerName", default, skip_serializing_if = "Option::is_none")]
    pub web_server_name: Option<String>,
    #[doc = "Gets the list of machine ARM Ids on which the web application is deployed."]
    #[serde(
        rename = "machineArmIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machine_arm_ids: Vec<String>,
    #[doc = "Gets the virtual path of the application."]
    #[serde(rename = "virtualPath", default, skip_serializing_if = "Option::is_none")]
    pub virtual_path: Option<String>,
    #[doc = "Gets the physical path of the application."]
    #[serde(rename = "physicalPath", default, skip_serializing_if = "Option::is_none")]
    pub physical_path: Option<String>,
    #[doc = "Gets the front end bindings for the application."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bindings: Vec<FrontEndBinding>,
    #[doc = "Gets the Error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub frameworks: Vec<WebApplicationFramework>,
    #[doc = "Gets the configuration."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub configurations: Vec<WebApplicationConfigurationUnit>,
    #[doc = "Gets the directories."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub directories: Vec<WebApplicationDirectoryUnit>,
    #[doc = "Gets the display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets tags that can be used with ODATA."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Static folders."]
    #[serde(
        rename = "staticFolders",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub static_folders: Vec<String>,
    #[doc = "Machine display name"]
    #[serde(rename = "machineDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub machine_display_name: Option<String>,
    #[doc = "Gets a value indicating whether application is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Gets the timestamp marking creation of the entity."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets the timestamp marking last update operation."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "Gets the server type."]
    #[serde(rename = "serverType", default, skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[doc = "Gets the Error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "Gets the appliance names."]
    #[serde(
        rename = "applianceNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub appliance_names: Vec<String>,
    #[doc = "Gets a value indicating whether the WebApp has errors or not."]
    #[serde(rename = "hasErrors", default, skip_serializing_if = "Option::is_none")]
    pub has_errors: Option<bool>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl TomcatWebApplicationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web application REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TomcatWebApplications {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for web application properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TomcatWebApplicationProperties>,
}
impl TomcatWebApplications {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a TomcatWebApplications list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TomcatWebApplicationsListResult {
    #[doc = "The TomcatWebApplications items on this page"]
    pub value: Vec<TomcatWebApplications>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TomcatWebApplicationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TomcatWebApplicationsListResult {
    pub fn new(value: Vec<TomcatWebApplications>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the TomcatWebApplications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TomcatWebApplicationsUpdate {}
impl TomcatWebApplicationsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for web server properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TomcatWebServerProperties {
    #[doc = "Gets or sets the services defined in the server."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub services: Vec<TomcatServiceUnit>,
    #[doc = "Catalina Home"]
    #[serde(rename = "catalinaHome", default, skip_serializing_if = "Option::is_none")]
    pub catalina_home: Option<String>,
    #[doc = "Version  of the JVM"]
    #[serde(rename = "jvmVersion", default, skip_serializing_if = "Option::is_none")]
    pub jvm_version: Option<String>,
    #[doc = "session persistence mechanism"]
    #[serde(rename = "sessionPersistenceMechanism", default, skip_serializing_if = "Option::is_none")]
    pub session_persistence_mechanism: Option<String>,
    #[doc = "Gets a value indicating whether application is deleted."]
    #[serde(rename = "isClusteringPresent", default, skip_serializing_if = "Option::is_none")]
    pub is_clustering_present: Option<bool>,
    #[doc = "Gets a value indicating whether application is deleted."]
    #[serde(rename = "isMemoryRealmPresent", default, skip_serializing_if = "Option::is_none")]
    pub is_memory_realm_present: Option<bool>,
    #[doc = "is session tracking present"]
    #[serde(rename = "isSessionTrackingPresent", default, skip_serializing_if = "Option::is_none")]
    pub is_session_tracking_present: Option<bool>,
    #[doc = "is access log valve present"]
    #[serde(rename = "isAccessLogValvePresent", default, skip_serializing_if = "Option::is_none")]
    pub is_access_log_valve_present: Option<bool>,
    #[doc = "max memory usage in mb"]
    #[serde(rename = "maxMemoryUsageInMb", default, skip_serializing_if = "Option::is_none")]
    pub max_memory_usage_in_mb: Option<String>,
    #[doc = "Gets the configuration location."]
    #[serde(rename = "configurationLocation", default, skip_serializing_if = "Option::is_none")]
    pub configuration_location: Option<String>,
    #[doc = "Gets the configuration location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets the list of machines."]
    #[serde(
        rename = "machineIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machine_ids: Vec<String>,
    #[doc = "Gets the list of web applications."]
    #[serde(
        rename = "webApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub web_applications: Vec<String>,
    #[doc = "Gets the display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets the server FQDN."]
    #[serde(rename = "serverFqdn", default, skip_serializing_if = "Option::is_none")]
    pub server_fqdn: Option<String>,
    #[doc = "Gets the run as account id."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Second level object returned as part of Machine REST resource."]
    #[serde(rename = "operatingSystemDetails", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_details: Option<OperatingSystem>,
    #[doc = "Gets a value indicating whether application is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Gets the timestamp marking creation of the entity."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets the timestamp marking last update operation."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "Gets the server type."]
    #[serde(rename = "serverType", default, skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[doc = "Gets the Error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "Gets the appliance names."]
    #[serde(
        rename = "applianceNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub appliance_names: Vec<String>,
    #[doc = "Gets a value indicating whether the WebApp has errors or not."]
    #[serde(rename = "hasErrors", default, skip_serializing_if = "Option::is_none")]
    pub has_errors: Option<bool>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl TomcatWebServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web server REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TomcatWebServers {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for web server properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TomcatWebServerProperties>,
}
impl TomcatWebServers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a TomcatWebServers list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TomcatWebServersListResult {
    #[doc = "The TomcatWebServers items on this page"]
    pub value: Vec<TomcatWebServers>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TomcatWebServersListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TomcatWebServersListResult {
    pub fn new(value: Vec<TomcatWebServers>) -> Self {
        Self { value, next_link: None }
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
#[doc = "Defines class that represents input to enable or disable DMP on machine\n      \n      for cosmos entity operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateMachineDepMapStatus {
    #[doc = "Gets or sets the machine collection."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machines: Vec<DependencyMapMachineInput>,
}
impl UpdateMachineDepMapStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines class that represents input to update RunAsAccount on machine\n      \n      for cosmos entity operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateMachineRunAsAccount {
    #[doc = "Gets or sets the machine collection."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machines: Vec<RunAsAccountMachineInput>,
}
impl UpdateMachineRunAsAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines class that represents input to update Tags on machine\n      \n      for cosmos entity operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateMachineTags {
    #[doc = "Gets or sets the machine collection."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machines: Vec<TagsMachineInput>,
}
impl UpdateMachineTags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Run as account REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct V20180501PreviewVmwareRunAsAccount {
    #[doc = "Relative URL to get this run as account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the Name of the Run as account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Handled by resource provider. Type =\nMicrosoft.OffAzure/VMWareSites/RunAsAccounts."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Class for run as account properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunAsAccountProperties>,
}
impl V20180501PreviewVmwareRunAsAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of VMware run as accounts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct V20180501PreviewVmwareRunAsAccountVmwareRunAsAccountCollection {
    #[doc = "Gets the list of run as accounts."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<V20180501PreviewVmwareRunAsAccount>,
    #[doc = "Gets the value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl V20180501PreviewVmwareRunAsAccountVmwareRunAsAccountCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "vmware datastore type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VMwareDatastoreType")]
pub enum VMwareDatastoreType {
    Unknown,
    #[serde(rename = "CIFS")]
    Cifs,
    #[serde(rename = "NFS")]
    Nfs,
    #[serde(rename = "NFS41")]
    Nfs41,
    #[serde(rename = "PMEM")]
    Pmem,
    #[serde(rename = "VFFS")]
    Vffs,
    #[serde(rename = "VMFS")]
    Vmfs,
    #[serde(rename = "VSAN")]
    Vsan,
    #[serde(rename = "VVOL")]
    Vvol,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VMwareDatastoreType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VMwareDatastoreType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VMwareDatastoreType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("VMwareDatastoreType", 0u32, "Unknown"),
            Self::Cifs => serializer.serialize_unit_variant("VMwareDatastoreType", 1u32, "CIFS"),
            Self::Nfs => serializer.serialize_unit_variant("VMwareDatastoreType", 2u32, "NFS"),
            Self::Nfs41 => serializer.serialize_unit_variant("VMwareDatastoreType", 3u32, "NFS41"),
            Self::Pmem => serializer.serialize_unit_variant("VMwareDatastoreType", 4u32, "PMEM"),
            Self::Vffs => serializer.serialize_unit_variant("VMwareDatastoreType", 5u32, "VFFS"),
            Self::Vmfs => serializer.serialize_unit_variant("VMwareDatastoreType", 6u32, "VMFS"),
            Self::Vsan => serializer.serialize_unit_variant("VMwareDatastoreType", 7u32, "VSAN"),
            Self::Vvol => serializer.serialize_unit_variant("VMwareDatastoreType", 8u32, "VVOL"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A vcenter resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Vcenter {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of VMwareSiteResource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VcenterProperties>,
}
impl Vcenter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a Vcenter list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VcenterListResult {
    #[doc = "The Vcenter items on this page"]
    pub value: Vec<Vcenter>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VcenterListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VcenterListResult {
    pub fn new(value: Vec<Vcenter>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of VMwareSiteResource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VcenterProperties {
    #[doc = "Gets or sets the run as account ID of the vCenter."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Gets the errors."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "Gets the timestamp marking vCenter creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets the timestamp marking last updated on the vCenter."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "Gets or sets the FQDN/IPAddress of the vCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Gets or sets the port of the vCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[doc = "Gets the version of the vCenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets the performance statistics enabled on the vCenter."]
    #[serde(rename = "perfStatisticsLevel", default, skip_serializing_if = "Option::is_none")]
    pub perf_statistics_level: Option<String>,
    #[doc = "Gets the instance UUID of the vCenter."]
    #[serde(rename = "instanceUuid", default, skip_serializing_if = "Option::is_none")]
    pub instance_uuid: Option<String>,
    #[doc = "Gets or sets the friendly name of the vCenter."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl VcenterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "service api versions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Versions")]
pub enum Versions {
    #[serde(rename = "2018-05-01-preview")]
    N2018_05_01_preview,
    #[serde(rename = "2019-05-01-preview")]
    N2019_05_01_preview,
    #[serde(rename = "2020-01-01")]
    N2020_01_01,
    #[serde(rename = "2020-01-01-preview")]
    N2020_01_01_preview,
    #[serde(rename = "2020-02-01")]
    N2020_02_01,
    #[serde(rename = "2020-07-07")]
    N2020_07_07,
    #[serde(rename = "2020-07-10")]
    N2020_07_10,
    #[serde(rename = "2020-08-01-preview")]
    N2020_08_01_preview,
    #[serde(rename = "2020-11-11-preview")]
    N2020_11_11_preview,
    #[serde(rename = "2022-10-27")]
    N2022_10_27,
    #[serde(rename = "2023-06-06")]
    N2023_06_06,
    #[serde(rename = "2023-10-01-preview")]
    N2023_10_01_preview,
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
            Self::N2018_05_01_preview => serializer.serialize_unit_variant("Versions", 0u32, "2018-05-01-preview"),
            Self::N2019_05_01_preview => serializer.serialize_unit_variant("Versions", 1u32, "2019-05-01-preview"),
            Self::N2020_01_01 => serializer.serialize_unit_variant("Versions", 2u32, "2020-01-01"),
            Self::N2020_01_01_preview => serializer.serialize_unit_variant("Versions", 3u32, "2020-01-01-preview"),
            Self::N2020_02_01 => serializer.serialize_unit_variant("Versions", 4u32, "2020-02-01"),
            Self::N2020_07_07 => serializer.serialize_unit_variant("Versions", 5u32, "2020-07-07"),
            Self::N2020_07_10 => serializer.serialize_unit_variant("Versions", 6u32, "2020-07-10"),
            Self::N2020_08_01_preview => serializer.serialize_unit_variant("Versions", 7u32, "2020-08-01-preview"),
            Self::N2020_11_11_preview => serializer.serialize_unit_variant("Versions", 8u32, "2020-11-11-preview"),
            Self::N2022_10_27 => serializer.serialize_unit_variant("Versions", 9u32, "2022-10-27"),
            Self::N2023_06_06 => serializer.serialize_unit_variant("Versions", 10u32, "2023-06-06"),
            Self::N2023_10_01_preview => serializer.serialize_unit_variant("Versions", 11u32, "2023-10-01-preview"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Second level object returned as part of VMware host REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmwareDatastore {
    #[doc = "Data store UUID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Capacity of the data store."]
    #[serde(rename = "capacityInGb", default, skip_serializing_if = "Option::is_none")]
    pub capacity_in_gb: Option<f32>,
    #[doc = "Free space of the data store."]
    #[serde(rename = "freeSpaceInGb", default, skip_serializing_if = "Option::is_none")]
    pub free_space_in_gb: Option<f32>,
    #[doc = "vmware datastore type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<VMwareDatastoreType>,
    #[doc = "Symbolic name of the data store."]
    #[serde(rename = "symbolicName", default, skip_serializing_if = "Option::is_none")]
    pub symbolic_name: Option<String>,
}
impl VmwareDatastore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Second level object returned as part of Machine REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmwareDisk {
    #[doc = "Disk UUID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Label of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "The provisioning policy of the disk.\n            It is Thin or Thick or\nUnknown for the VMWare VMDK."]
    #[serde(rename = "diskProvisioningPolicy", default, skip_serializing_if = "Option::is_none")]
    pub disk_provisioning_policy: Option<String>,
    #[doc = "The scrubbing policy of disks which can be\n            eagerly zeroed or\nlazily zeroed."]
    #[serde(rename = "diskScrubbingPolicy", default, skip_serializing_if = "Option::is_none")]
    pub disk_scrubbing_policy: Option<String>,
    #[doc = "Disk mode property used for identifying independent disks."]
    #[serde(rename = "diskMode", default, skip_serializing_if = "Option::is_none")]
    pub disk_mode: Option<String>,
    #[doc = "Gets or sets a value indicating the type of the disk controller type."]
    #[serde(rename = "controllerType", default, skip_serializing_if = "Option::is_none")]
    pub controller_type: Option<String>,
    #[doc = "Gets or sets Bytes allocated for the disk."]
    #[serde(rename = "maxSizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_in_bytes: Option<i64>,
    #[doc = "Gets or sets Name of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets Type of the disk."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
    #[doc = "Gets or sets LUN of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[doc = "Gets or sets Path of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl VmwareDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A host resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmwareHost {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for host properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VmwareHostProperties>,
}
impl VmwareHost {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a VmwareHost list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmwareHostListResult {
    #[doc = "The VmwareHost items on this page"]
    pub value: Vec<VmwareHost>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VmwareHostListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VmwareHostListResult {
    pub fn new(value: Vec<VmwareHost>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class for host properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmwareHostProperties {
    #[doc = "Gets the timestamp marking VMware host creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets the timestamp marking last updated on the VMware host."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "Gets the data stores."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub datastores: Vec<VmwareDatastore>,
    #[doc = "Gets the V-center ID."]
    #[serde(rename = "vcenterId", default, skip_serializing_if = "Option::is_none")]
    pub vcenter_id: Option<String>,
    #[doc = "Gets the instance UUID of the vmware host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Gets the appliance names."]
    #[serde(
        rename = "applianceNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub appliance_names: Vec<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl VmwareHostProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A host resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmwareJob {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Job REST Resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
impl VmwareJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a VmwareJob list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmwareJobListResult {
    #[doc = "The VmwareJob items on this page"]
    pub value: Vec<VmwareJob>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VmwareJobListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VmwareJobListResult {
    pub fn new(value: Vec<VmwareJob>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class for machine properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmwareMachineProperties {
    #[doc = "VCenter FQDN/IPAddress."]
    #[serde(rename = "vCenterFqdn", default, skip_serializing_if = "Option::is_none")]
    pub v_center_fqdn: Option<String>,
    #[doc = "VCenter ARM ID."]
    #[serde(rename = "vCenterId", default, skip_serializing_if = "Option::is_none")]
    pub v_center_id: Option<String>,
    #[doc = "On-premise Instance UUID of the machine."]
    #[serde(rename = "instanceUuid", default, skip_serializing_if = "Option::is_none")]
    pub instance_uuid: Option<String>,
    #[doc = "Scope of the data center."]
    #[serde(rename = "dataCenterScope", default, skip_serializing_if = "Option::is_none")]
    pub data_center_scope: Option<String>,
    #[doc = "User description of the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Disks attached to the machine."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disks: Vec<VmwareDisk>,
    #[doc = "Indicates whether the host is in maintenance mode."]
    #[serde(rename = "hostInMaintenanceMode", default, skip_serializing_if = "Option::is_none")]
    pub host_in_maintenance_mode: Option<bool>,
    #[doc = "The host name."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "The host power state."]
    #[serde(rename = "hostPowerState", default, skip_serializing_if = "Option::is_none")]
    pub host_power_state: Option<String>,
    #[doc = "The host version."]
    #[serde(rename = "hostVersion", default, skip_serializing_if = "Option::is_none")]
    pub host_version: Option<String>,
    #[doc = "Network adapters attached to the machine."]
    #[serde(
        rename = "networkAdapters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_adapters: Vec<VmwareNetworkAdapter>,
    #[doc = "VMware tools status."]
    #[serde(rename = "vMwareToolsStatus", default, skip_serializing_if = "Option::is_none")]
    pub v_mware_tools_status: Option<String>,
    #[doc = "VMware tools version."]
    #[serde(rename = "vMwareToolsVersion", default, skip_serializing_if = "Option::is_none")]
    pub v_mware_tools_version: Option<String>,
    #[doc = "Value indicating whether change tracking is supported."]
    #[serde(rename = "changeTrackingSupported", default, skip_serializing_if = "Option::is_none")]
    pub change_tracking_supported: Option<bool>,
    #[doc = "Value indicating whether change tracking is enabled."]
    #[serde(rename = "changeTrackingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub change_tracking_enabled: Option<bool>,
    #[doc = "Maximum number of snapshots for the VM.            Default value is -1."]
    #[serde(rename = "maxSnapshots", default, skip_serializing_if = "Option::is_none")]
    pub max_snapshots: Option<i32>,
    #[doc = "Gets or sets whether Disk Enabled UUID is set or not."]
    #[serde(rename = "diskEnabledUuid", default, skip_serializing_if = "Option::is_none")]
    pub disk_enabled_uuid: Option<String>,
    #[doc = "Number of snapshots for the VM.             Default value is -1."]
    #[serde(rename = "numberOfSnapshots", default, skip_serializing_if = "Option::is_none")]
    pub number_of_snapshots: Option<i32>,
    #[doc = "Gets or sets the machine alt guest name."]
    #[serde(rename = "altGuestName", default, skip_serializing_if = "Option::is_none")]
    pub alt_guest_name: Option<String>,
    #[doc = "Gets the Machine power status."]
    #[serde(rename = "powerStatus", default, skip_serializing_if = "Option::is_none")]
    pub power_status: Option<String>,
    #[doc = "Gets the VM FQDN."]
    #[serde(rename = "vmFqdn", default, skip_serializing_if = "Option::is_none")]
    pub vm_fqdn: Option<String>,
    #[doc = "Gets the Root location of the VM configuration file."]
    #[serde(rename = "vmConfigurationFileLocation", default, skip_serializing_if = "Option::is_none")]
    pub vm_configuration_file_location: Option<String>,
    #[doc = "Gets or sets the firmware."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware: Option<String>,
    #[doc = "Second level object.  Data related to a machine's operating system.             Serialized and stored as part of Machine Rest object. "]
    #[serde(rename = "guestOsDetails", default, skip_serializing_if = "Option::is_none")]
    pub guest_os_details: Option<GuestOsDetails>,
    #[doc = "Number of applications installed in the guest VM."]
    #[serde(rename = "numberOfApplications", default, skip_serializing_if = "Option::is_none")]
    pub number_of_applications: Option<i32>,
    #[doc = "The last time at which the Guest Details was discovered\n            or the\nerror while discovering guest details based discovery\n            of the\nmachine."]
    #[serde(rename = "guestDetailsDiscoveryTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub guest_details_discovery_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Whether Refresh Fabric Layout Guest Details has been completed once.\n         \n  Portal will show discovery in progress, if this value is true."]
    #[serde(rename = "isGuestDetailsDiscoveryInProgress", default, skip_serializing_if = "Option::is_none")]
    pub is_guest_details_discovery_in_progress: Option<bool>,
    #[doc = "Gets or sets if dependency mapping feature is enabled or not\n            for\nthe VM."]
    #[serde(rename = "dependencyMapping", default, skip_serializing_if = "Option::is_none")]
    pub dependency_mapping: Option<String>,
    #[doc = "Gets or sets when dependency mapping collection is last started."]
    #[serde(rename = "dependencyMappingStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub dependency_mapping_start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets when dependency mapping collection was last disabled."]
    #[serde(rename = "dependencyMappingEndTime", default, skip_serializing_if = "Option::is_none")]
    pub dependency_mapping_end_time: Option<String>,
    #[doc = "Gets or sets the run as account ID of the machine."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Gets the appliance names."]
    #[serde(
        rename = "applianceNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub appliance_names: Vec<String>,
    #[doc = "Errors for machine."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "Contains data related application and roles discovery scope."]
    #[serde(rename = "applicationDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub application_discovery: Option<ApplicationDiscovery>,
    #[doc = "Contains data related dependency map discovery scope."]
    #[serde(rename = "dependencyMapDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub dependency_map_discovery: Option<DependencyMapDiscovery>,
    #[doc = "Contains data related static data discovery scope."]
    #[serde(rename = "staticDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub static_discovery: Option<StaticDiscovery>,
    #[doc = "Contains data related SQL discovery."]
    #[serde(rename = "sqlDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub sql_discovery: Option<SqlDiscovery>,
    #[doc = "Data related to a machine's WebApps discovery."]
    #[serde(rename = "webAppDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub web_app_discovery: Option<WebAppDiscovery>,
    #[doc = "Data related to a machine's Oracle discovery."]
    #[serde(rename = "oracleDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub oracle_discovery: Option<OracleDiscovery>,
    #[doc = "Data related to a machine's spring boot discovery."]
    #[serde(rename = "springBootDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub spring_boot_discovery: Option<SpringBootDiscovery>,
    #[doc = "Data related to a machine's WebApps discovery."]
    #[serde(rename = "iisDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub iis_discovery: Option<WebAppDiscovery>,
    #[doc = "Data related to a machine's WebApps discovery."]
    #[serde(rename = "tomcatDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub tomcat_discovery: Option<WebAppDiscovery>,
    #[doc = "AppsAndRoles in the guest virtual machine."]
    #[serde(rename = "appsAndRoles", default, skip_serializing_if = "Option::is_none")]
    pub apps_and_roles: Option<AppsAndRoles>,
    #[doc = "product support status."]
    #[serde(rename = "productSupportStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_support_status: Option<ProductSupportStatus>,
    #[doc = "Gets or sets the Number of Processor Cores \n            allocated for the\nmachine."]
    #[serde(rename = "numberOfProcessorCore", default, skip_serializing_if = "Option::is_none")]
    pub number_of_processor_core: Option<i32>,
    #[doc = "Gets or sets the allocated Memory in MB."]
    #[serde(rename = "allocatedMemoryInMb", default, skip_serializing_if = "Option::is_none")]
    pub allocated_memory_in_mb: Option<f64>,
    #[doc = "Second level object returned as part of Machine REST resource."]
    #[serde(rename = "operatingSystemDetails", default, skip_serializing_if = "Option::is_none")]
    pub operating_system_details: Option<OperatingSystem>,
    #[doc = "Gets or sets the Machine BIOS serial number."]
    #[serde(rename = "biosSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub bios_serial_number: Option<String>,
    #[doc = "Gets or sets the BIOS GUID."]
    #[serde(rename = "biosGuid", default, skip_serializing_if = "Option::is_none")]
    pub bios_guid: Option<String>,
    #[doc = "Gets the Display name of the machine."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Value indicating whether VM is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Gets the marking machine creation."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets or sets tags on the VMware machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Gets the timestamp marking last updated on the machine."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl VmwareMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An software inventory resource belonging to a machine resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmwareMachineSoftwareInventory {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for machine software inventory properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineSoftwareInventoryProperties>,
}
impl VmwareMachineSoftwareInventory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a VmwareMachineSoftwareInventory list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmwareMachineSoftwareInventoryListResult {
    #[doc = "The VmwareMachineSoftwareInventory items on this page"]
    pub value: Vec<VmwareMachineSoftwareInventory>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VmwareMachineSoftwareInventoryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VmwareMachineSoftwareInventoryListResult {
    pub fn new(value: Vec<VmwareMachineSoftwareInventory>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Second level object represented in responses as part of Machine REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmwareNetworkAdapter {
    #[doc = "Label of the NIC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Gets or sets the adapter type."]
    #[serde(rename = "adapterType", default, skip_serializing_if = "Option::is_none")]
    pub adapter_type: Option<String>,
    #[doc = "Gets or sets the NIC Id."]
    #[serde(rename = "nicId", default, skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    #[doc = "Gets or sets Mac address of the NIC."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "Gets IP addresses for the machine."]
    #[serde(
        rename = "ipAddressList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_address_list: Vec<String>,
    #[doc = "Gets or sets Network Name."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "Gets or sets Type of the IP address."]
    #[serde(rename = "ipAddressType", default, skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
}
impl VmwareNetworkAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A runasaccount resource belonging to a site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmwareRunAsAccountResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for run as account properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunAsAccountProperties>,
}
impl VmwareRunAsAccountResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a VmwareRunAsAccountResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmwareRunAsAccountResourceListResult {
    #[doc = "The VmwareRunAsAccountResource items on this page"]
    pub value: Vec<VmwareRunAsAccountResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VmwareRunAsAccountResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VmwareRunAsAccountResourceListResult {
    pub fn new(value: Vec<VmwareRunAsAccountResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A VmwareSite"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmwareSite {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of SiteResource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SitesProperties>,
    #[doc = "If eTag is provided in the response body, it may also be provided as a header per the normal etag convention.  Entity tags are used for comparing two or more entities from the same requested resource. HTTP/1.1 uses entity tags in the etag (section 14.19), If-Match (section 14.24), If-None-Match (section 14.26), and If-Range (section 14.27) header fields."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl VmwareSite {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            e_tag: None,
        }
    }
}
#[doc = "The response of a VmwareSite list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmwareSiteListResult {
    #[doc = "The VmwareSite items on this page"]
    pub value: Vec<VmwareSite>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VmwareSiteListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VmwareSiteListResult {
    pub fn new(value: Vec<VmwareSite>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the VmwareSite."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmwareSiteUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the VmwareSite."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VmwareSiteUpdateProperties>,
}
impl VmwareSiteUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the VmwareSite."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmwareSiteUpdateProperties {
    #[doc = "Class for site properties."]
    #[serde(rename = "servicePrincipalIdentityDetails", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_identity_details: Option<SiteSpnProperties>,
    #[doc = "Class for site agent properties."]
    #[serde(rename = "agentDetails", default, skip_serializing_if = "Option::is_none")]
    pub agent_details: Option<SiteAgentProperties>,
    #[doc = "Gets or sets the Appliance Name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
    #[doc = "Gets or sets the ARM ID of migration hub solution for SDS."]
    #[serde(rename = "discoverySolutionId", default, skip_serializing_if = "Option::is_none")]
    pub discovery_solution_id: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl VmwareSiteUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VMware site usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmwareSiteUsage {
    #[doc = "Gets or sets the number of machines discovered in the site."]
    #[serde(rename = "machineCount", default, skip_serializing_if = "Option::is_none")]
    pub machine_count: Option<i32>,
    #[doc = "Gets or sets the number of run as accounts in the site."]
    #[serde(rename = "runAsAccountCount", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_count: Option<i32>,
    #[doc = "Gets or sets the number of vCenters part of the site."]
    #[serde(rename = "vCenterCount", default, skip_serializing_if = "Option::is_none")]
    pub v_center_count: Option<i32>,
}
impl VmwareSiteUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data related to a machine's WebApps discovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppDiscovery {
    #[doc = "Gets or sets number of successfully discovered web servers.."]
    #[serde(rename = "totalWebServerCount", default, skip_serializing_if = "Option::is_none")]
    pub total_web_server_count: Option<i64>,
    #[doc = "Gets or sets number of successfully discovered web applications."]
    #[serde(rename = "totalWebApplicationCount", default, skip_serializing_if = "Option::is_none")]
    pub total_web_application_count: Option<i64>,
    #[doc = "Discovery Scope."]
    #[serde(rename = "discoveryScopeStatus", default, skip_serializing_if = "Option::is_none")]
    pub discovery_scope_status: Option<DiscoveryScopeStatus>,
}
impl WebAppDiscovery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web app extended machine REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppExtendedMachine {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for web extended machine properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebAppExtendedMachineProperties>,
}
impl WebAppExtendedMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a WebAppExtendedMachine list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAppExtendedMachineListResult {
    #[doc = "The WebAppExtendedMachine items on this page"]
    pub value: Vec<WebAppExtendedMachine>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebAppExtendedMachineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WebAppExtendedMachineListResult {
    pub fn new(value: Vec<WebAppExtendedMachine>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class for web extended machine properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppExtendedMachineProperties {
    #[doc = "Gets a value indicating whether application is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Gets the timestamp marking creation of the entity."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets the timestamp marking last update operation."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "Gets the Error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "Gets the hydrated host name."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "Gets the Name of the extended machine."]
    #[serde(rename = "machineDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub machine_display_name: Option<String>,
    #[doc = "Gets the machine ARM id."]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "Gets the run as account ID of the machine used for web app discovery."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Gets the agent hydrated run as account."]
    #[serde(rename = "hydratedRunAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub hydrated_run_as_account_id: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl WebAppExtendedMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing web app properties web model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppProperties {
    #[doc = "Gets or sets the arm id of the web app."]
    #[serde(rename = "webAppArmId", default, skip_serializing_if = "Option::is_none")]
    pub web_app_arm_id: Option<String>,
    #[doc = "Gets or sets Tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl WebAppProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of web app properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppPropertiesCollection {
    #[doc = "Gets or sets the list of web app properties."]
    #[serde(
        rename = "webApps",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub web_apps: Vec<WebAppProperties>,
}
impl WebAppPropertiesCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Run as account REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppRunAsAccount {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for run as account properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunAsAccountProperties>,
}
impl WebAppRunAsAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a WebAppRunAsAccount list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAppRunAsAccountListResult {
    #[doc = "The WebAppRunAsAccount items on this page"]
    pub value: Vec<WebAppRunAsAccount>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebAppRunAsAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WebAppRunAsAccountListResult {
    pub fn new(value: Vec<WebAppRunAsAccount>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "WebApp site web model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppSite {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for Web app site properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebAppSiteProperties>,
}
impl WebAppSite {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a WebAppSite list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebAppSiteListResult {
    #[doc = "The WebAppSite items on this page"]
    pub value: Vec<WebAppSite>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebAppSiteListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WebAppSiteListResult {
    pub fn new(value: Vec<WebAppSite>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class for Web app site properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppSiteProperties {
    #[doc = "Gets or sets the appliance details used by service to communicate\n           \nto the appliance."]
    #[serde(
        rename = "siteAppliancePropertiesCollection",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub site_appliance_properties_collection: Vec<SiteApplianceProperties>,
    #[doc = "Discovery Scenario"]
    #[serde(rename = "discoveryScenario", default, skip_serializing_if = "Option::is_none")]
    pub discovery_scenario: Option<WebAppSitePropertiesDiscoveryScenario>,
    #[doc = "Gets the service endpoint."]
    #[serde(rename = "serviceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl WebAppSiteProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Discovery Scenario"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WebAppSitePropertiesDiscoveryScenario")]
pub enum WebAppSitePropertiesDiscoveryScenario {
    Migrate,
    #[serde(rename = "DR")]
    Dr,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WebAppSitePropertiesDiscoveryScenario {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WebAppSitePropertiesDiscoveryScenario {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WebAppSitePropertiesDiscoveryScenario {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Migrate => serializer.serialize_unit_variant("WebAppSitePropertiesDiscoveryScenario", 0u32, "Migrate"),
            Self::Dr => serializer.serialize_unit_variant("WebAppSitePropertiesDiscoveryScenario", 1u32, "DR"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type used for update operations of the WebAppSite."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppSiteUpdate {
    #[doc = "The updatable properties of the WebAppSite."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebAppSiteUpdateProperties>,
}
impl WebAppSiteUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the WebAppSite."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppSiteUpdateProperties {
    #[doc = "Gets or sets the appliance details used by service to communicate\n           \nto the appliance."]
    #[serde(
        rename = "siteAppliancePropertiesCollection",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub site_appliance_properties_collection: Vec<SiteApplianceProperties>,
    #[doc = "Discovery Scenario"]
    #[serde(rename = "discoveryScenario", default, skip_serializing_if = "Option::is_none")]
    pub discovery_scenario: Option<WebAppSitePropertiesDiscoveryScenario>,
}
impl WebAppSiteUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web app site usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebAppSiteUsage {
    #[doc = "Gets or sets the number of web servers discovered in the site."]
    #[serde(rename = "webServerCount", default, skip_serializing_if = "Option::is_none")]
    pub web_server_count: Option<i32>,
    #[doc = "Gets or sets the number of web applications discovered in the site."]
    #[serde(rename = "webApplicationCount", default, skip_serializing_if = "Option::is_none")]
    pub web_application_count: Option<i32>,
    #[doc = "Gets or sets the number of run as accounts in the site."]
    #[serde(rename = "runAsAccountCount", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_count: Option<i32>,
}
impl WebAppSiteUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web application REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplication {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for web application properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebApplicationProperties>,
}
impl WebApplication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "WebApplication in the guest virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationAppsAndRolesModel {
    #[doc = "Gets or sets Name of the WebApplication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets Status of the WebApplication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets Platform of the WebApplication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[doc = "Gets or sets GroupName of the WebApplication."]
    #[serde(rename = "groupName", default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[doc = "Gets or sets WebServer of the WebApplication."]
    #[serde(rename = "webServer", default, skip_serializing_if = "Option::is_none")]
    pub web_server: Option<String>,
    #[doc = "Gets or sets ApplicationPool of the WebApplication."]
    #[serde(rename = "applicationPool", default, skip_serializing_if = "Option::is_none")]
    pub application_pool: Option<String>,
}
impl WebApplicationAppsAndRolesModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web Application configuration unit."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationConfigurationUnit {
    #[doc = "Gets or sets the configuration name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the configuration file path."]
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[doc = "Gets or sets the configuration local file path."]
    #[serde(rename = "localFilePath", default, skip_serializing_if = "Option::is_none")]
    pub local_file_path: Option<String>,
    #[doc = "Gets or sets the configuration target file path."]
    #[serde(rename = "targetFilePath", default, skip_serializing_if = "Option::is_none")]
    pub target_file_path: Option<String>,
    #[doc = "Gets or sets the configuration section in the file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    #[doc = "Gets or sets the configuration type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets a value indicating whether the configuration is edited or not by\nthe user."]
    #[serde(rename = "isDeploymentTimeEditable", default, skip_serializing_if = "Option::is_none")]
    pub is_deployment_time_editable: Option<bool>,
    #[doc = "Gets or sets the identifier for the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}
impl WebApplicationConfigurationUnit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web Application directory unit."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationDirectoryUnit {
    #[doc = "Gets or sets the unique id corresponding to the application directory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets a value indicating whether the directory object is editable.\n     \n      True when the directory is added as an optional directory, false when\ndiscovery is done\n            manually."]
    #[serde(rename = "isEditable", default, skip_serializing_if = "Option::is_none")]
    pub is_editable: Option<bool>,
    #[doc = "Gets or sets the paths of the directory on the source machine."]
    #[serde(
        rename = "sourcePaths",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub source_paths: Vec<String>,
    #[doc = "Gets or sets the local scratch path at which the directories has been copied."]
    #[serde(rename = "localScratchPath", default, skip_serializing_if = "Option::is_none")]
    pub local_scratch_path: Option<String>,
    #[doc = "Gets or sets the mount path of the application directory."]
    #[serde(rename = "mountPath", default, skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    #[doc = "Gets or sets the size of the directory on the source machine."]
    #[serde(rename = "sourceSize", default, skip_serializing_if = "Option::is_none")]
    pub source_size: Option<String>,
}
impl WebApplicationDirectoryUnit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Framework specific data for a web application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationFramework {
    #[doc = "Gets or sets Name of the framework."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets Version of the framework."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl WebApplicationFramework {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a WebApplication list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebApplicationListResult {
    #[doc = "The WebApplication items on this page"]
    pub value: Vec<WebApplication>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebApplicationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WebApplicationListResult {
    pub fn new(value: Vec<WebApplication>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class for web application properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebApplicationProperties {
    #[doc = "Gets the web server id."]
    #[serde(rename = "webServerId", default, skip_serializing_if = "Option::is_none")]
    pub web_server_id: Option<String>,
    #[doc = "Gets the web server name."]
    #[serde(rename = "webServerName", default, skip_serializing_if = "Option::is_none")]
    pub web_server_name: Option<String>,
    #[doc = "Gets the list of machine ARM Ids on which the SQL server is deployed."]
    #[serde(
        rename = "machineArmIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machine_arm_ids: Vec<String>,
    #[doc = "Gets the virtual path of the application."]
    #[serde(rename = "virtualPath", default, skip_serializing_if = "Option::is_none")]
    pub virtual_path: Option<String>,
    #[doc = "Gets the physical path of the application."]
    #[serde(rename = "physicalPath", default, skip_serializing_if = "Option::is_none")]
    pub physical_path: Option<String>,
    #[doc = "Gets the front end bindings for the application."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bindings: Vec<FrontEndBinding>,
    #[doc = "Gets the Error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub frameworks: Vec<WebApplicationFramework>,
    #[doc = "Gets the configuration."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub configurations: Vec<WebApplicationConfigurationUnit>,
    #[doc = "Gets the directories."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub directories: Vec<WebApplicationDirectoryUnit>,
    #[doc = "Gets the display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets tags that can be used with ODATA."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Static folders."]
    #[serde(
        rename = "staticFolders",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub static_folders: Vec<String>,
    #[doc = "Machine display name"]
    #[serde(rename = "machineDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub machine_display_name: Option<String>,
    #[doc = "Gets a value indicating whether application is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Gets the timestamp marking creation of the entity."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets the timestamp marking last update operation."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "Gets the server type."]
    #[serde(rename = "serverType", default, skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[doc = "Gets the Error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "Gets the appliance names."]
    #[serde(
        rename = "applianceNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub appliance_names: Vec<String>,
    #[doc = "Gets a value indicating whether the WebApp has errors or not."]
    #[serde(rename = "hasErrors", default, skip_serializing_if = "Option::is_none")]
    pub has_errors: Option<bool>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl WebApplicationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class representing the imported machine web model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebRoleImportDisk {
    #[doc = "Disk read throughput."]
    #[serde(rename = "megabytesPerSecondOfRead", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_of_read: Option<f32>,
    #[doc = "Disk write throughput."]
    #[serde(rename = "megabytesPerSecondOfWrite", default, skip_serializing_if = "Option::is_none")]
    pub megabytes_per_second_of_write: Option<f32>,
    #[doc = "Disk read IOPS."]
    #[serde(rename = "numberOfReadOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub number_of_read_operations_per_second: Option<f32>,
    #[doc = "Disk write IOPS."]
    #[serde(rename = "numberOfWriteOperationsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub number_of_write_operations_per_second: Option<f32>,
    #[doc = "Gets or sets Bytes allocated for the disk."]
    #[serde(rename = "maxSizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub max_size_in_bytes: Option<i64>,
    #[doc = "Gets or sets Name of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets Type of the disk."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
    #[doc = "Gets or sets LUN of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[doc = "Gets or sets Path of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl WebRoleImportDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Second level object returned as part of Machine REST resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebRoleOperatingSystem {
    #[doc = "Gets or sets the type of the operating system."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Gets or sets the Name of the operating system."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "Gets or sets the Version of the operating system."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "Gets or sets the Architecture of the operating system."]
    #[serde(rename = "osArchitecture", default, skip_serializing_if = "Option::is_none")]
    pub os_architecture: Option<String>,
}
impl WebRoleOperatingSystem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web application REST Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebServer {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Class for web server properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebServerProperties>,
}
impl WebServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a WebServer list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebServerListResult {
    #[doc = "The WebServer items on this page"]
    pub value: Vec<WebServer>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebServerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WebServerListResult {
    pub fn new(value: Vec<WebServer>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Class for web server properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebServerProperties {
    #[doc = "Gets the configuration location."]
    #[serde(rename = "configurationLocation", default, skip_serializing_if = "Option::is_none")]
    pub configuration_location: Option<String>,
    #[doc = "Gets the configuration location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets the list of machines."]
    #[serde(
        rename = "machineIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub machine_ids: Vec<String>,
    #[doc = "Gets the list of web applications."]
    #[serde(
        rename = "webApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub web_applications: Vec<String>,
    #[doc = "Gets the display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets the server FQDN."]
    #[serde(rename = "serverFqdn", default, skip_serializing_if = "Option::is_none")]
    pub server_fqdn: Option<String>,
    #[doc = "Gets the run as account id."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "Gets a value indicating whether application is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "Gets the timestamp marking creation of the entity."]
    #[serde(rename = "createdTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[doc = "Gets the timestamp marking last update operation."]
    #[serde(rename = "updatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
    #[doc = "Gets the server type."]
    #[serde(rename = "serverType", default, skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[doc = "Gets the Error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<HealthErrorDetails>,
    #[doc = "Gets the appliance names."]
    #[serde(
        rename = "applianceNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub appliance_names: Vec<String>,
    #[doc = "Gets a value indicating whether the WebApp has errors or not."]
    #[serde(rename = "hasErrors", default, skip_serializing_if = "Option::is_none")]
    pub has_errors: Option<bool>,
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl WebServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "esu Status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EsuStatus")]
pub enum EsuStatus {
    Unknown,
    Active,
    InActive,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EsuStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EsuStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EsuStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("EsuStatus", 0u32, "Unknown"),
            Self::Active => serializer.serialize_unit_variant("EsuStatus", 1u32, "Active"),
            Self::InActive => serializer.serialize_unit_variant("EsuStatus", 2u32, "InActive"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "esu year"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EsuYear")]
pub enum EsuYear {
    Unknown,
    FirstYear,
    SecondYear,
    ThirdYear,
    UpgradeYear,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EsuYear {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EsuYear {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EsuYear {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("EsuYear", 0u32, "Unknown"),
            Self::FirstYear => serializer.serialize_unit_variant("EsuYear", 1u32, "FirstYear"),
            Self::SecondYear => serializer.serialize_unit_variant("EsuYear", 2u32, "SecondYear"),
            Self::ThirdYear => serializer.serialize_unit_variant("EsuYear", 3u32, "ThirdYear"),
            Self::UpgradeYear => serializer.serialize_unit_variant("EsuYear", 4u32, "UpgradeYear"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "support status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SupportStatus")]
pub enum SupportStatus {
    Unknown,
    Mainstream,
    Extended,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SupportStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SupportStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SupportStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("SupportStatus", 0u32, "Unknown"),
            Self::Mainstream => serializer.serialize_unit_variant("SupportStatus", 1u32, "Mainstream"),
            Self::Extended => serializer.serialize_unit_variant("SupportStatus", 2u32, "Extended"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
