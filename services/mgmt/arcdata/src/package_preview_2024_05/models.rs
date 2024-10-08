#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "DNS server details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryConnectorDnsDetails {
    #[doc = "DNS domain name for which DNS lookups should be forwarded to the Active Directory DNS servers."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "List of Active Directory DNS server IP addresses."]
    #[serde(rename = "nameserverIPAddresses")]
    pub nameserver_ip_addresses: Vec<String>,
    #[doc = "Replica count for DNS proxy service. Default value is 1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    #[doc = "Flag indicating whether to prefer Kubernetes DNS server response over AD DNS server response for IP address lookups."]
    #[serde(rename = "preferK8sDnsForPtrLookups", default, skip_serializing_if = "Option::is_none")]
    pub prefer_k8s_dns_for_ptr_lookups: Option<bool>,
}
impl ActiveDirectoryConnectorDnsDetails {
    pub fn new(nameserver_ip_addresses: Vec<String>) -> Self {
        Self {
            domain_name: None,
            nameserver_ip_addresses,
            replicas: None,
            prefer_k8s_dns_for_ptr_lookups: None,
        }
    }
}
#[doc = "Active Directory domain details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryConnectorDomainDetails {
    #[doc = "Name (uppercase) of the Active Directory domain that this AD connector will be associated with."]
    pub realm: String,
    #[doc = "NETBIOS name of the Active Directory domain."]
    #[serde(rename = "netbiosDomainName", default, skip_serializing_if = "Option::is_none")]
    pub netbios_domain_name: Option<String>,
    #[doc = "The service account provisioning mode for this Active Directory connector."]
    #[serde(rename = "serviceAccountProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub service_account_provisioning: Option<active_directory_connector_domain_details::ServiceAccountProvisioning>,
    #[doc = "The distinguished name of the Active Directory Organizational Unit."]
    #[serde(rename = "ouDistinguishedName", default, skip_serializing_if = "Option::is_none")]
    pub ou_distinguished_name: Option<String>,
    #[doc = "Details about the Active Directory domain controllers associated with this AD connector instance"]
    #[serde(rename = "domainControllers", default, skip_serializing_if = "Option::is_none")]
    pub domain_controllers: Option<ActiveDirectoryDomainControllers>,
}
impl ActiveDirectoryConnectorDomainDetails {
    pub fn new(realm: String) -> Self {
        Self {
            realm,
            netbios_domain_name: None,
            service_account_provisioning: None,
            ou_distinguished_name: None,
            domain_controllers: None,
        }
    }
}
pub mod active_directory_connector_domain_details {
    use super::*;
    #[doc = "The service account provisioning mode for this Active Directory connector."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServiceAccountProvisioning")]
    pub enum ServiceAccountProvisioning {
        #[serde(rename = "automatic")]
        Automatic,
        #[serde(rename = "manual")]
        Manual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServiceAccountProvisioning {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServiceAccountProvisioning {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServiceAccountProvisioning {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Automatic => serializer.serialize_unit_variant("ServiceAccountProvisioning", 0u32, "automatic"),
                Self::Manual => serializer.serialize_unit_variant("ServiceAccountProvisioning", 1u32, "manual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ServiceAccountProvisioning {
        fn default() -> Self {
            Self::Manual
        }
    }
}
#[doc = "A list of active directory connectors"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveDirectoryConnectorListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ActiveDirectoryConnectorResource>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ActiveDirectoryConnectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ActiveDirectoryConnectorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an Active Directory connector resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryConnectorProperties {
    #[doc = "Username and password for basic login authentication."]
    #[serde(rename = "domainServiceAccountLoginInformation", default, skip_serializing_if = "Option::is_none")]
    pub domain_service_account_login_information: Option<BasicLoginInformation>,
    #[doc = "The provisioning state of the Active Directory connector resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The specifications of the AD Kubernetes resource."]
    pub spec: ActiveDirectoryConnectorSpec,
    #[doc = "The status of the Kubernetes custom resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ActiveDirectoryConnectorStatus>,
}
impl ActiveDirectoryConnectorProperties {
    pub fn new(spec: ActiveDirectoryConnectorSpec) -> Self {
        Self {
            domain_service_account_login_information: None,
            provisioning_state: None,
            spec,
            status: None,
        }
    }
}
#[doc = "Active directory connector resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryConnectorResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of an Active Directory connector resource"]
    pub properties: ActiveDirectoryConnectorProperties,
}
impl ActiveDirectoryConnectorResource {
    pub fn new(properties: ActiveDirectoryConnectorProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "The specifications of the AD Kubernetes resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryConnectorSpec {
    #[doc = "Active Directory domain details"]
    #[serde(rename = "activeDirectory")]
    pub active_directory: ActiveDirectoryConnectorDomainDetails,
    #[doc = "DNS server details"]
    pub dns: ActiveDirectoryConnectorDnsDetails,
}
impl ActiveDirectoryConnectorSpec {
    pub fn new(active_directory: ActiveDirectoryConnectorDomainDetails, dns: ActiveDirectoryConnectorDnsDetails) -> Self {
        Self { active_directory, dns }
    }
}
#[doc = "The status of the Kubernetes custom resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveDirectoryConnectorStatus {
    #[doc = "The time that the custom resource was last updated."]
    #[serde(rename = "lastUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[doc = "The version of the replicaSet associated with the AD connector custom resource."]
    #[serde(rename = "observedGeneration", default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    #[doc = "The state of the AD connector custom resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl ActiveDirectoryConnectorStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a domain controller in the AD domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveDirectoryDomainController {
    #[doc = "Fully-qualified domain name of a domain controller in the AD domain."]
    pub hostname: String,
}
impl ActiveDirectoryDomainController {
    pub fn new(hostname: String) -> Self {
        Self { hostname }
    }
}
#[doc = "Details about the Active Directory domain controllers associated with this AD connector instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveDirectoryDomainControllers {
    #[doc = "Information about a domain controller in the AD domain."]
    #[serde(rename = "primaryDomainController", default, skip_serializing_if = "Option::is_none")]
    pub primary_domain_controller: Option<ActiveDirectoryDomainController>,
    #[doc = "Information about the secondary domain controllers in the AD domain."]
    #[serde(rename = "secondaryDomainControllers", default, skip_serializing_if = "Option::is_none")]
    pub secondary_domain_controllers: Option<ActiveDirectorySecondaryDomainControllers>,
}
impl ActiveDirectoryDomainControllers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Active Directory information that related to the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveDirectoryInformation {
    #[doc = "Keytab used for authenticate with Active Directory."]
    #[serde(rename = "keytabInformation", default, skip_serializing_if = "Option::is_none")]
    pub keytab_information: Option<KeytabInformation>,
}
impl ActiveDirectoryInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ActiveDirectorySecondaryDomainControllers = Vec<ActiveDirectoryDomainController>;
#[doc = "A list of Arc Sql Server Availability Groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArcSqlServerAvailabilityGroupListResult {
    #[doc = "Array of Arc Sql Server Availability Groups."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SqlServerAvailabilityGroupResource>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ArcSqlServerAvailabilityGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ArcSqlServerAvailabilityGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of Arc Sql Server database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArcSqlServerDatabaseListResult {
    #[doc = "Array of  Arc Sql Server database."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SqlServerDatabaseResource>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ArcSqlServerDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ArcSqlServerDatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The specifications of the availability group replica configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityGroupConfigure {
    #[doc = "Name of the mirroring endpoint URL"]
    #[serde(rename = "endpointName", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    #[doc = "Mirroring endpoint URL of availability group replica"]
    #[serde(rename = "endpointUrl", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[doc = "The endpoint connection authentication type(s)."]
    #[serde(rename = "endpointAuthenticationMode", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_authentication_mode: Option<ConnectionAuth>,
    #[doc = "Name of certificate to use for authentication. Required if any CERTIFICATE authentication modes are specified."]
    #[serde(rename = "certificateName", default, skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    #[doc = "The login which will connect to the mirroring endpoint."]
    #[serde(rename = "endpointConnectLogin", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_connect_login: Option<String>,
    #[doc = "Property that determines whether a given availability replica can run in synchronous-commit mode"]
    #[serde(rename = "availabilityMode", default, skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<availability_group_configure::AvailabilityMode>,
    #[doc = "The Availability Synchronization mode of the availability group replica."]
    #[serde(rename = "availabilityModeDescription", default, skip_serializing_if = "Option::is_none")]
    pub availability_mode_description: Option<String>,
    #[doc = "Property to set the failover mode of the availability group replica"]
    #[serde(rename = "failoverMode", default, skip_serializing_if = "Option::is_none")]
    pub failover_mode: Option<availability_group_configure::FailoverMode>,
    #[doc = "The failover mode of the availability group replica."]
    #[serde(rename = "failoverModeDescription", default, skip_serializing_if = "Option::is_none")]
    pub failover_mode_description: Option<String>,
    #[doc = "The time-out period of availability group session replica, in seconds."]
    #[serde(rename = "sessionTimeout", default, skip_serializing_if = "Option::is_none")]
    pub session_timeout: Option<i32>,
    #[doc = "Whether the primary replica should allow all connections or only READ_WRITE connections (disallowing ReadOnly connections)"]
    #[serde(rename = "primaryAllowConnections", default, skip_serializing_if = "Option::is_none")]
    pub primary_allow_connections: Option<availability_group_configure::PrimaryAllowConnections>,
    #[doc = "Whether the availability allows all connections or only read-write connections."]
    #[serde(
        rename = "primaryRoleAllowConnectionsDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_role_allow_connections_description: Option<String>,
    #[doc = "Whether the secondary replica should allow all connections, no connections, or only ReadOnly connections."]
    #[serde(rename = "secondaryAllowConnections", default, skip_serializing_if = "Option::is_none")]
    pub secondary_allow_connections: Option<availability_group_configure::SecondaryAllowConnections>,
    #[doc = "Whether an availability replica that is performing the secondary role (that is, a secondary replica) can accept connections from clients."]
    #[serde(
        rename = "secondaryRoleAllowConnectionsDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary_role_allow_connections_description: Option<String>,
    #[doc = "Date that the replica was created."]
    #[serde(rename = "replicaCreateDate", default, with = "azure_core::date::rfc3339::option")]
    pub replica_create_date: Option<::time::OffsetDateTime>,
    #[doc = "Date that the replica was modified."]
    #[serde(rename = "replicaModifyDate", default, with = "azure_core::date::rfc3339::option")]
    pub replica_modify_date: Option<::time::OffsetDateTime>,
    #[doc = "Represents the user-specified priority for performing backups on this replica relative to the other replicas in the same availability group."]
    #[serde(rename = "backupPriority", default, skip_serializing_if = "Option::is_none")]
    pub backup_priority: Option<i32>,
    #[doc = "Connectivity endpoint (URL) of the read only availability replica."]
    #[serde(rename = "readOnlyRoutingUrl", default, skip_serializing_if = "Option::is_none")]
    pub read_only_routing_url: Option<String>,
    #[doc = "Connectivity endpoint (URL) of the read write availability replica."]
    #[serde(rename = "readWriteRoutingUrl", default, skip_serializing_if = "Option::is_none")]
    pub read_write_routing_url: Option<String>,
    #[doc = "Specifies how the secondary replica will be initially seeded. AUTOMATIC enables direct seeding. This method will seed the secondary replica over the network. This method does not require you to backup and restore a copy of the primary database on the replica. MANUAL specifies manual seeding (default). This method requires you to create a backup of the database on the primary replica and manually restore that backup on the secondary replica."]
    #[serde(rename = "seedingMode", default, skip_serializing_if = "Option::is_none")]
    pub seeding_mode: Option<availability_group_configure::SeedingMode>,
    #[doc = "Describes seeding mode."]
    #[serde(rename = "seedingModeDescription", default, skip_serializing_if = "Option::is_none")]
    pub seeding_mode_description: Option<String>,
}
impl AvailabilityGroupConfigure {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod availability_group_configure {
    use super::*;
    #[doc = "Property that determines whether a given availability replica can run in synchronous-commit mode"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AvailabilityMode")]
    pub enum AvailabilityMode {
        #[serde(rename = "SYNCHRONOUS_COMMIT")]
        SynchronousCommit,
        #[serde(rename = "ASYNCHRONOUS_COMMIT")]
        AsynchronousCommit,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AvailabilityMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AvailabilityMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AvailabilityMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SynchronousCommit => serializer.serialize_unit_variant("AvailabilityMode", 0u32, "SYNCHRONOUS_COMMIT"),
                Self::AsynchronousCommit => serializer.serialize_unit_variant("AvailabilityMode", 1u32, "ASYNCHRONOUS_COMMIT"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Property to set the failover mode of the availability group replica"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FailoverMode")]
    pub enum FailoverMode {
        #[serde(rename = "AUTOMATIC")]
        Automatic,
        #[serde(rename = "MANUAL")]
        Manual,
        #[serde(rename = "EXTERNAL")]
        External,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FailoverMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FailoverMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FailoverMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Automatic => serializer.serialize_unit_variant("FailoverMode", 0u32, "AUTOMATIC"),
                Self::Manual => serializer.serialize_unit_variant("FailoverMode", 1u32, "MANUAL"),
                Self::External => serializer.serialize_unit_variant("FailoverMode", 2u32, "EXTERNAL"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether the primary replica should allow all connections or only READ_WRITE connections (disallowing ReadOnly connections)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrimaryAllowConnections {
        #[serde(rename = "ALL")]
        All,
        #[serde(rename = "READ_WRITE")]
        ReadWrite,
    }
    #[doc = "Whether the secondary replica should allow all connections, no connections, or only ReadOnly connections."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SecondaryAllowConnections {
        #[serde(rename = "NO")]
        No,
        #[serde(rename = "ALL")]
        All,
        #[serde(rename = "READ_ONLY")]
        ReadOnly,
    }
    #[doc = "Specifies how the secondary replica will be initially seeded. AUTOMATIC enables direct seeding. This method will seed the secondary replica over the network. This method does not require you to backup and restore a copy of the primary database on the replica. MANUAL specifies manual seeding (default). This method requires you to create a backup of the database on the primary replica and manually restore that backup on the secondary replica."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SeedingMode {
        #[serde(rename = "AUTOMATIC")]
        Automatic,
        #[serde(rename = "MANUAL")]
        Manual,
    }
}
#[doc = "Options used in creating an availability group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityGroupCreateUpdateConfiguration {
    #[doc = "Name of the availability group."]
    #[serde(rename = "availabilityGroupName", default, skip_serializing_if = "Option::is_none")]
    pub availability_group_name: Option<String>,
    #[doc = "List of availability group replicas."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub replicas: Vec<AvailabilityGroupCreateUpdateReplicaConfiguration>,
    #[doc = "List of databases to include in the availability group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub databases: Vec<String>,
    #[doc = "Preferred replica for running automated backups."]
    #[serde(rename = "automatedBackupPreference", default, skip_serializing_if = "Option::is_none")]
    pub automated_backup_preference: Option<availability_group_create_update_configuration::AutomatedBackupPreference>,
    #[doc = "User-defined failure condition level under which an automatic failover must be triggered."]
    #[serde(rename = "failureConditionLevel", default, skip_serializing_if = "Option::is_none")]
    pub failure_condition_level: Option<availability_group_create_update_configuration::FailureConditionLevel>,
    #[doc = "Wait time (in milliseconds) for the sp_server_diagnostics system stored procedure to return server-health information, before the server instance is assumed to be slow or not responding."]
    #[serde(rename = "healthCheckTimeout", default, skip_serializing_if = "Option::is_none")]
    pub health_check_timeout: Option<i32>,
    #[doc = "Specifies whether the availability group supports failover for database health conditions."]
    #[serde(rename = "dbFailover", default, skip_serializing_if = "Option::is_none")]
    pub db_failover: Option<availability_group_create_update_configuration::DbFailover>,
    #[doc = "Specifies whether DTC support has been enabled for this availability group."]
    #[serde(rename = "dtcSupport", default, skip_serializing_if = "Option::is_none")]
    pub dtc_support: Option<availability_group_create_update_configuration::DtcSupport>,
    #[doc = "The number of secondary replicas that must be in a synchronized state for a commit to complete."]
    #[serde(
        rename = "requiredSynchronizedSecondariesToCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub required_synchronized_secondaries_to_commit: Option<i32>,
    #[doc = "Set to WSFC when availability group is on a failover cluster instance on a Windows Server failover cluster. Set to NONE when availability group not using WSFC for cluster coordination."]
    #[serde(rename = "clusterType", default, skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<availability_group_create_update_configuration::ClusterType>,
    #[doc = "The properties of a static IP Arc Sql availability group listener"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listener: Option<SqlAvailabilityGroupStaticIpListenerProperties>,
}
impl AvailabilityGroupCreateUpdateConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod availability_group_create_update_configuration {
    use super::*;
    #[doc = "Preferred replica for running automated backups."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AutomatedBackupPreference {
        #[serde(rename = "PRIMARY")]
        Primary,
        #[serde(rename = "SECONDARY_ONLY")]
        SecondaryOnly,
        #[serde(rename = "SECONDARY")]
        Secondary,
        #[serde(rename = "NONE")]
        None,
    }
    #[doc = "User-defined failure condition level under which an automatic failover must be triggered."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FailureConditionLevel {}
    #[doc = "Specifies whether the availability group supports failover for database health conditions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DbFailover {
        #[serde(rename = "ON")]
        On,
        #[serde(rename = "OFF")]
        Off,
    }
    #[doc = "Specifies whether DTC support has been enabled for this availability group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DtcSupport {
        #[serde(rename = "PER_DB")]
        PerDb,
        #[serde(rename = "NONE")]
        None,
    }
    #[doc = "Set to WSFC when availability group is on a failover cluster instance on a Windows Server failover cluster. Set to NONE when availability group not using WSFC for cluster coordination."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ClusterType {
        #[serde(rename = "WSFC")]
        Wsfc,
        #[serde(rename = "NONE")]
        None,
    }
}
#[doc = "The specifications of the availability group replica configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityGroupCreateUpdateReplicaConfiguration {
    #[doc = "the server instance hosting the replica."]
    #[serde(rename = "serverInstance", default, skip_serializing_if = "Option::is_none")]
    pub server_instance: Option<String>,
    #[doc = "Name of the database mirroring endpoint URL for the availability group replica"]
    #[serde(rename = "endpointName", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    #[doc = "Database mirroring endpoint URL of availability group replica"]
    #[serde(rename = "endpointUrl", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[doc = "The endpoint connection authentication type(s)."]
    #[serde(rename = "endpointAuthenticationMode", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_authentication_mode: Option<ConnectionAuth>,
    #[doc = "Name of certificate to use for authentication. Required if any CERTIFICATE authentication modes are specified."]
    #[serde(rename = "certificateName", default, skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    #[doc = "The login which will connect to the mirroring endpoint"]
    #[serde(rename = "endpointConnectLogin", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_connect_login: Option<String>,
    #[doc = "Property that determines whether a given availability replica can run in synchronous-commit mode"]
    #[serde(rename = "availabilityMode", default, skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<availability_group_create_update_replica_configuration::AvailabilityMode>,
    #[doc = "Property to set the failover mode of the availability group replica"]
    #[serde(rename = "failoverMode", default, skip_serializing_if = "Option::is_none")]
    pub failover_mode: Option<availability_group_create_update_replica_configuration::FailoverMode>,
    #[doc = "Specifies how the secondary replica will be initially seeded. AUTOMATIC enables direct seeding. This method will seed the secondary replica over the network. This method does not require you to backup and restore a copy of the primary database on the replica. MANUAL specifies manual seeding (default). This method requires you to create a backup of the database on the primary replica and manually restore that backup on the secondary replica."]
    #[serde(rename = "seedingMode", default, skip_serializing_if = "Option::is_none")]
    pub seeding_mode: Option<availability_group_create_update_replica_configuration::SeedingMode>,
    #[doc = "Represents the user-specified priority for performing backups on this replica relative to the other replicas in the same availability group."]
    #[serde(rename = "backupPriority", default, skip_serializing_if = "Option::is_none")]
    pub backup_priority: Option<i32>,
    #[doc = "Whether the secondary replica should allow all connections, no connections, or only ReadOnly connections."]
    #[serde(rename = "secondaryRoleAllowConnections", default, skip_serializing_if = "Option::is_none")]
    pub secondary_role_allow_connections: Option<availability_group_create_update_replica_configuration::SecondaryRoleAllowConnections>,
    #[doc = "Connectivity endpoint (URL) of the read only availability replica."]
    #[serde(rename = "secondaryRoleReadOnlyRoutingUrl", default, skip_serializing_if = "Option::is_none")]
    pub secondary_role_read_only_routing_url: Option<String>,
    #[doc = "Whether the primary replica should allow all connections or only READ_WRITE connections (disallowing ReadOnly connections)"]
    #[serde(rename = "primaryRoleAllowConnections", default, skip_serializing_if = "Option::is_none")]
    pub primary_role_allow_connections: Option<availability_group_create_update_replica_configuration::PrimaryRoleAllowConnections>,
    #[doc = "List of read only routing URLs."]
    #[serde(
        rename = "primaryRoleReadOnlyRoutingList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub primary_role_read_only_routing_list: Vec<String>,
    #[doc = "The time-out period of availability group session replica, in seconds."]
    #[serde(rename = "sessionTimeout", default, skip_serializing_if = "Option::is_none")]
    pub session_timeout: Option<i32>,
}
impl AvailabilityGroupCreateUpdateReplicaConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod availability_group_create_update_replica_configuration {
    use super::*;
    #[doc = "Property that determines whether a given availability replica can run in synchronous-commit mode"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AvailabilityMode")]
    pub enum AvailabilityMode {
        #[serde(rename = "SYNCHRONOUS_COMMIT")]
        SynchronousCommit,
        #[serde(rename = "ASYNCHRONOUS_COMMIT")]
        AsynchronousCommit,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AvailabilityMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AvailabilityMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AvailabilityMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SynchronousCommit => serializer.serialize_unit_variant("AvailabilityMode", 0u32, "SYNCHRONOUS_COMMIT"),
                Self::AsynchronousCommit => serializer.serialize_unit_variant("AvailabilityMode", 1u32, "ASYNCHRONOUS_COMMIT"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Property to set the failover mode of the availability group replica"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FailoverMode")]
    pub enum FailoverMode {
        #[serde(rename = "AUTOMATIC")]
        Automatic,
        #[serde(rename = "MANUAL")]
        Manual,
        #[serde(rename = "EXTERNAL")]
        External,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FailoverMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FailoverMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FailoverMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Automatic => serializer.serialize_unit_variant("FailoverMode", 0u32, "AUTOMATIC"),
                Self::Manual => serializer.serialize_unit_variant("FailoverMode", 1u32, "MANUAL"),
                Self::External => serializer.serialize_unit_variant("FailoverMode", 2u32, "EXTERNAL"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies how the secondary replica will be initially seeded. AUTOMATIC enables direct seeding. This method will seed the secondary replica over the network. This method does not require you to backup and restore a copy of the primary database on the replica. MANUAL specifies manual seeding (default). This method requires you to create a backup of the database on the primary replica and manually restore that backup on the secondary replica."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SeedingMode {
        #[serde(rename = "AUTOMATIC")]
        Automatic,
        #[serde(rename = "MANUAL")]
        Manual,
    }
    #[doc = "Whether the secondary replica should allow all connections, no connections, or only ReadOnly connections."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SecondaryRoleAllowConnections {
        #[serde(rename = "NO")]
        No,
        #[serde(rename = "ALL")]
        All,
        #[serde(rename = "READ_ONLY")]
        ReadOnly,
    }
    #[doc = "Whether the primary replica should allow all connections or only READ_WRITE connections (disallowing ReadOnly connections)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrimaryRoleAllowConnections {
        #[serde(rename = "ALL")]
        All,
        #[serde(rename = "READ_WRITE")]
        ReadWrite,
    }
}
#[doc = "The specifications of the availability group state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityGroupInfo {
    #[doc = "User-defined failure condition level under which an automatic failover must be triggered."]
    #[serde(rename = "failureConditionLevel", default, skip_serializing_if = "Option::is_none")]
    pub failure_condition_level: Option<i32>,
    #[doc = "Wait time (in milliseconds) for the sp_server_diagnostics system stored procedure to return server-health information, before the server instance is assumed to be slow or not responding."]
    #[serde(rename = "healthCheckTimeout", default, skip_serializing_if = "Option::is_none")]
    pub health_check_timeout: Option<i32>,
    #[doc = "Preferred location for performing backups on the availability databases in this availability group."]
    #[serde(rename = "automatedBackupPreferenceDescription", default, skip_serializing_if = "Option::is_none")]
    pub automated_backup_preference_description: Option<String>,
    #[doc = "SQL Server availability group current version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[doc = "Specifies whether this is a basic availability group."]
    #[serde(rename = "basicFeatures", default, skip_serializing_if = "Option::is_none")]
    pub basic_features: Option<bool>,
    #[doc = "Specifies whether DTC support has been enabled for this availability group."]
    #[serde(rename = "dtcSupport", default, skip_serializing_if = "Option::is_none")]
    pub dtc_support: Option<bool>,
    #[doc = "Specifies whether the availability group supports failover for database health conditions."]
    #[serde(rename = "dbFailover", default, skip_serializing_if = "Option::is_none")]
    pub db_failover: Option<bool>,
    #[doc = "Specifies whether this is a distributed availability group."]
    #[serde(rename = "isDistributed", default, skip_serializing_if = "Option::is_none")]
    pub is_distributed: Option<bool>,
    #[doc = "SQL Server availability group cluster type description"]
    #[serde(rename = "clusterTypeDescription", default, skip_serializing_if = "Option::is_none")]
    pub cluster_type_description: Option<String>,
    #[doc = "The number of secondary replicas that must be in a synchronized state for a commit to complete."]
    #[serde(
        rename = "requiredSynchronizedSecondariesToCommit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub required_synchronized_secondaries_to_commit: Option<i32>,
    #[doc = "SQL Server availability group contained system databases."]
    #[serde(rename = "isContained", default, skip_serializing_if = "Option::is_none")]
    pub is_contained: Option<bool>,
    #[doc = "Name of the server instance that is hosting the current primary replica."]
    #[serde(rename = "primaryReplica", default, skip_serializing_if = "Option::is_none")]
    pub primary_replica: Option<String>,
    #[doc = "Indicates the recovery health of the primary replica."]
    #[serde(rename = "primaryRecoveryHealthDescription", default, skip_serializing_if = "Option::is_none")]
    pub primary_recovery_health_description: Option<String>,
    #[doc = "Indicates the recovery health of a secondary replica."]
    #[serde(rename = "secondaryRecoveryHealthDescription", default, skip_serializing_if = "Option::is_none")]
    pub secondary_recovery_health_description: Option<String>,
    #[doc = "Reflects a roll-up of the synchronization health of all availability replicas in the availability group."]
    #[serde(rename = "synchronizationHealthDescription", default, skip_serializing_if = "Option::is_none")]
    pub synchronization_health_description: Option<String>,
    #[serde(rename = "replicationPartnerType", default, skip_serializing_if = "Option::is_none")]
    pub replication_partner_type: Option<availability_group_info::ReplicationPartnerType>,
    #[doc = "The properties of a static IP Arc Sql availability group listener"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listener: Option<SqlAvailabilityGroupStaticIpListenerProperties>,
}
impl AvailabilityGroupInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod availability_group_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReplicationPartnerType {
        #[serde(rename = "SQLServer")]
        SqlServer,
        #[serde(rename = "AzureSQLVM")]
        AzureSqlvm,
        #[serde(rename = "AzureSQLManagedInstance")]
        AzureSqlManagedInstance,
        Unknown,
    }
}
#[doc = "The specifications of the availability group state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityGroupState {
    #[doc = "Current Always On availability groups role of the availability group replica."]
    #[serde(rename = "availabilityGroupReplicaRole", default, skip_serializing_if = "Option::is_none")]
    pub availability_group_replica_role: Option<String>,
    #[doc = "Current operational state of the availability group replica"]
    #[serde(rename = "operationalStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub operational_state_description: Option<String>,
    #[doc = "Recovery health of the availability group replica."]
    #[serde(rename = "recoveryHealthDescription", default, skip_serializing_if = "Option::is_none")]
    pub recovery_health_description: Option<String>,
    #[doc = "Reflects a rollup of the database synchronization state (synchronization_state) of all joined availability databases (also known as replicas) and the availability mode of the replica (synchronous-commit or asynchronous-commit mode). The rollup will reflect the least healthy accumulated state the databases on the replica."]
    #[serde(rename = "synchronizationHealthDescription", default, skip_serializing_if = "Option::is_none")]
    pub synchronization_health_description: Option<String>,
    #[doc = "Whether a secondary replica is currently connected to the primary replica."]
    #[serde(rename = "connectedStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub connected_state_description: Option<String>,
    #[doc = "Text description of the last connection error of the availability group replica."]
    #[serde(rename = "lastConnectErrorDescription", default, skip_serializing_if = "Option::is_none")]
    pub last_connect_error_description: Option<String>,
    #[doc = "Date and time timestamp indicating when the last connect error occurred."]
    #[serde(rename = "lastConnectErrorTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub last_connect_error_timestamp: Option<::time::OffsetDateTime>,
}
impl AvailabilityGroupState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The background job details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackgroundJob {
    #[doc = "The state of the background job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<background_job::State>,
    #[doc = "The execution state of the background job."]
    #[serde(rename = "executionState", default, skip_serializing_if = "Option::is_none")]
    pub execution_state: Option<background_job::ExecutionState>,
    #[doc = "The start time of the background job."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "The end time of the background job."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
    #[doc = "The last execution status of the background job."]
    #[serde(rename = "lastExecutionStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_execution_status: Option<background_job::LastExecutionStatus>,
    #[doc = "The last execution time of the background job."]
    #[serde(rename = "lastExecutionTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_execution_time: Option<::time::OffsetDateTime>,
}
impl BackgroundJob {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod background_job {
    use super::*;
    #[doc = "The state of the background job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Enabled,
        Disabled,
        Deleted,
        Completed,
        Faulted,
        Suspended,
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
                Self::Enabled => serializer.serialize_unit_variant("State", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("State", 1u32, "Disabled"),
                Self::Deleted => serializer.serialize_unit_variant("State", 2u32, "Deleted"),
                Self::Completed => serializer.serialize_unit_variant("State", 3u32, "Completed"),
                Self::Faulted => serializer.serialize_unit_variant("State", 4u32, "Faulted"),
                Self::Suspended => serializer.serialize_unit_variant("State", 5u32, "Suspended"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The execution state of the background job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ExecutionState")]
    pub enum ExecutionState {
        Waiting,
        Running,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ExecutionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ExecutionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ExecutionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Waiting => serializer.serialize_unit_variant("ExecutionState", 0u32, "Waiting"),
                Self::Running => serializer.serialize_unit_variant("ExecutionState", 1u32, "Running"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The last execution status of the background job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastExecutionStatus")]
    pub enum LastExecutionStatus {
        Succeeded,
        Completed,
        Failed,
        Faulted,
        Postponed,
        Rescheduled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastExecutionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastExecutionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastExecutionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("LastExecutionStatus", 0u32, "Succeeded"),
                Self::Completed => serializer.serialize_unit_variant("LastExecutionStatus", 1u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("LastExecutionStatus", 2u32, "Failed"),
                Self::Faulted => serializer.serialize_unit_variant("LastExecutionStatus", 3u32, "Faulted"),
                Self::Postponed => serializer.serialize_unit_variant("LastExecutionStatus", 4u32, "Postponed"),
                Self::Rescheduled => serializer.serialize_unit_variant("LastExecutionStatus", 5u32, "Rescheduled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The backup profile for the SQL server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupPolicy {
    #[doc = "The retention period for all the databases in this managed instance."]
    #[serde(rename = "retentionPeriodDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_period_days: Option<i32>,
    #[doc = "The value indicating days between full backups."]
    #[serde(rename = "fullBackupDays", default, skip_serializing_if = "Option::is_none")]
    pub full_backup_days: Option<i32>,
    #[doc = "The differential backup interval in hours."]
    #[serde(rename = "differentialBackupHours", default, skip_serializing_if = "Option::is_none")]
    pub differential_backup_hours: Option<backup_policy::DifferentialBackupHours>,
    #[doc = "The value indicating minutes between transaction log backups."]
    #[serde(rename = "transactionLogBackupMinutes", default, skip_serializing_if = "Option::is_none")]
    pub transaction_log_backup_minutes: Option<i32>,
}
impl BackupPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup_policy {
    use super::*;
    #[doc = "The differential backup interval in hours."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DifferentialBackupHours {}
}
#[doc = "Username and password for basic login authentication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BasicLoginInformation {
    #[doc = "Login username."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Login password."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl BasicLoginInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Client connection related configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientConnection {
    #[doc = "Indicates if client connection is enabled for this SQL Server instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ClientConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition representing SKU for ARM resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommonSku {
    #[doc = "The name of the SKU.  It is typically a letter+number code"]
    pub name: String,
    #[doc = "Whether dev/test is enabled. When the dev field is set to true, the resource is used for dev/test purpose. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dev: Option<bool>,
    #[doc = "The SKU size. When the name field is the combination of tier and some other value, this would be the standalone code. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "If the service has different generations of hardware, for the same SKU, then that can be captured here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "If the SKU supports scale out/in then the capacity integer should be included. If scale out/in is not possible for the resource this may be omitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl CommonSku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            dev: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
#[doc = "The endpoint connection authentication type(s)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConnectionAuth")]
pub enum ConnectionAuth {
    #[serde(rename = "Windows_NTLM")]
    WindowsNtlm,
    #[serde(rename = "Windows_Kerberos")]
    WindowsKerberos,
    #[serde(rename = "Windows_Negotiate")]
    WindowsNegotiate,
    Certificate,
    #[serde(rename = "Windows_NTLM_Certificate")]
    WindowsNtlmCertificate,
    #[serde(rename = "Windows_Kerberos_Certificate")]
    WindowsKerberosCertificate,
    #[serde(rename = "Windows_Negotiate_Certificate")]
    WindowsNegotiateCertificate,
    #[serde(rename = "Certificate_Windows_NTLM")]
    CertificateWindowsNtlm,
    #[serde(rename = "Certificate_Windows_Kerberos")]
    CertificateWindowsKerberos,
    #[serde(rename = "Certificate_Windows_Negotiate")]
    CertificateWindowsNegotiate,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConnectionAuth {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConnectionAuth {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConnectionAuth {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::WindowsNtlm => serializer.serialize_unit_variant("ConnectionAuth", 0u32, "Windows_NTLM"),
            Self::WindowsKerberos => serializer.serialize_unit_variant("ConnectionAuth", 1u32, "Windows_Kerberos"),
            Self::WindowsNegotiate => serializer.serialize_unit_variant("ConnectionAuth", 2u32, "Windows_Negotiate"),
            Self::Certificate => serializer.serialize_unit_variant("ConnectionAuth", 3u32, "Certificate"),
            Self::WindowsNtlmCertificate => serializer.serialize_unit_variant("ConnectionAuth", 4u32, "Windows_NTLM_Certificate"),
            Self::WindowsKerberosCertificate => serializer.serialize_unit_variant("ConnectionAuth", 5u32, "Windows_Kerberos_Certificate"),
            Self::WindowsNegotiateCertificate => serializer.serialize_unit_variant("ConnectionAuth", 6u32, "Windows_Negotiate_Certificate"),
            Self::CertificateWindowsNtlm => serializer.serialize_unit_variant("ConnectionAuth", 7u32, "Certificate_Windows_NTLM"),
            Self::CertificateWindowsKerberos => serializer.serialize_unit_variant("ConnectionAuth", 8u32, "Certificate_Windows_Kerberos"),
            Self::CertificateWindowsNegotiate => serializer.serialize_unit_variant("ConnectionAuth", 9u32, "Certificate_Windows_Negotiate"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Database mirroring endpoint related properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DbmEndpoint {
    #[doc = "Name of the database mirroring endpoint."]
    #[serde(rename = "endpointName", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    #[doc = "Mirroring Role"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<dbm_endpoint::Role>,
    #[doc = "Is Encryption enabled"]
    #[serde(rename = "isEncryptionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_encryption_enabled: Option<bool>,
    #[doc = "The encryption algorithm(s) used by the endpoint."]
    #[serde(rename = "encryptionAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<EncryptionAlgorithm>,
    #[doc = "The endpoint connection authentication type(s)."]
    #[serde(rename = "connectionAuth", default, skip_serializing_if = "Option::is_none")]
    pub connection_auth: Option<ConnectionAuth>,
    #[doc = "The port number that the endpoint is listening on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "Is the port number dynamically assigned."]
    #[serde(rename = "isDynamicPort", default, skip_serializing_if = "Option::is_none")]
    pub is_dynamic_port: Option<bool>,
    #[doc = "Listener IP address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Name of the certificate."]
    #[serde(rename = "certificateName", default, skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
}
impl DbmEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dbm_endpoint {
    use super::*;
    #[doc = "Mirroring Role"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Role")]
    pub enum Role {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "PARTNER")]
        Partner,
        #[serde(rename = "WITNESS")]
        Witness,
        #[serde(rename = "ALL")]
        All,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Role {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Role {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Role {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Role", 0u32, "NONE"),
                Self::Partner => serializer.serialize_unit_variant("Role", 1u32, "PARTNER"),
                Self::Witness => serializer.serialize_unit_variant("Role", 2u32, "WITNESS"),
                Self::All => serializer.serialize_unit_variant("Role", 3u32, "ALL"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Migration related configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBaseMigration {
    #[doc = "The migration assessment related configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assessment: Option<DataBaseMigrationAssessment>,
}
impl DataBaseMigration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The migration assessment related configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBaseMigrationAssessment {
    #[doc = "The time when Migration Assessment Report upload was last performed."]
    #[serde(rename = "assessmentUploadTime", default, with = "azure_core::date::rfc3339::option")]
    pub assessment_upload_time: Option<::time::OffsetDateTime>,
    #[doc = "Issues and warnings impacting the migration of Database to particular Azure Migration Target."]
    #[serde(rename = "databaseAssessments", default, skip_serializing_if = "Option::is_none")]
    pub database_assessments: Option<DatabaseAssessments>,
    #[doc = "The target readiness for migration for this database."]
    #[serde(rename = "targetReadiness", default, skip_serializing_if = "Option::is_none")]
    pub target_readiness: Option<TargetReadiness>,
}
impl DataBaseMigrationAssessment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The data controller properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataControllerProperties {
    #[doc = "The infrastructure the data controller is running on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infrastructure: Option<data_controller_properties::Infrastructure>,
    #[doc = "Properties from the Kubernetes data controller"]
    #[serde(rename = "onPremiseProperty", default, skip_serializing_if = "Option::is_none")]
    pub on_premise_property: Option<OnPremiseProperty>,
    #[doc = "The raw kubernetes information"]
    #[serde(rename = "k8sRaw", default, skip_serializing_if = "Option::is_none")]
    pub k8s_raw: Option<serde_json::Value>,
    #[doc = "Properties on upload watermark.  Mostly timestamp for each upload data type"]
    #[serde(rename = "uploadWatermark", default, skip_serializing_if = "Option::is_none")]
    pub upload_watermark: Option<UploadWatermark>,
    #[doc = "Last uploaded date from Kubernetes cluster. Defaults to current date time"]
    #[serde(rename = "lastUploadedDate", default, with = "azure_core::date::rfc3339::option")]
    pub last_uploaded_date: Option<::time::OffsetDateTime>,
    #[doc = "Username and password for basic login authentication."]
    #[serde(rename = "basicLoginInformation", default, skip_serializing_if = "Option::is_none")]
    pub basic_login_information: Option<BasicLoginInformation>,
    #[doc = "Username and password for basic login authentication."]
    #[serde(rename = "metricsDashboardCredential", default, skip_serializing_if = "Option::is_none")]
    pub metrics_dashboard_credential: Option<BasicLoginInformation>,
    #[doc = "Username and password for basic login authentication."]
    #[serde(rename = "logsDashboardCredential", default, skip_serializing_if = "Option::is_none")]
    pub logs_dashboard_credential: Option<BasicLoginInformation>,
    #[doc = "Log analytics workspace id and primary key"]
    #[serde(rename = "logAnalyticsWorkspaceConfig", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_workspace_config: Option<LogAnalyticsWorkspaceConfig>,
    #[doc = "Service principal for uploading billing, metrics and logs."]
    #[serde(rename = "uploadServicePrincipal", default, skip_serializing_if = "Option::is_none")]
    pub upload_service_principal: Option<UploadServicePrincipal>,
    #[doc = "The provisioning state of the Arc Data Controller resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "If a CustomLocation is provided, this contains the ARM id of the connected cluster the custom location belongs to."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "If a CustomLocation is provided, this contains the ARM id of the extension the custom location belongs to."]
    #[serde(rename = "extensionId", default, skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,
}
impl DataControllerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_controller_properties {
    use super::*;
    #[doc = "The infrastructure the data controller is running on."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Infrastructure {
        #[serde(rename = "azure")]
        Azure,
        #[serde(rename = "gcp")]
        Gcp,
        #[serde(rename = "aws")]
        Aws,
        #[serde(rename = "alibaba")]
        Alibaba,
        #[serde(rename = "onpremises")]
        Onpremises,
        #[serde(rename = "other")]
        Other,
    }
    impl Default for Infrastructure {
        fn default() -> Self {
            Self::Other
        }
    }
}
#[doc = "Data controller resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataControllerResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "The data controller properties."]
    pub properties: DataControllerProperties,
}
impl DataControllerResource {
    pub fn new(tracked_resource: TrackedResource, properties: DataControllerProperties) -> Self {
        Self {
            tracked_resource,
            extended_location: None,
            properties,
        }
    }
}
#[doc = "Used for updating a data controller resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataControllerUpdate {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The data controller properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataControllerProperties>,
}
impl DataControllerUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type DatabaseAssessments = Vec<serde_json::Value>;
#[doc = "The availability group certificate configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DistributedAvailabilityGroupCreateUpdateAvailabilityGroupCertificateConfiguration {
    #[doc = "Name of the certificate."]
    #[serde(rename = "certificateName", default, skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
}
impl DistributedAvailabilityGroupCreateUpdateAvailabilityGroupCertificateConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The availability group configuration specification for a distributed availability group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DistributedAvailabilityGroupCreateUpdateAvailabilityGroupConfiguration {
    #[doc = "The azure resource identifier for the availability group."]
    #[serde(rename = "availabilityGroup", default, skip_serializing_if = "Option::is_none")]
    pub availability_group: Option<String>,
    #[doc = "The listener URL of the availability group."]
    #[serde(rename = "listenerUrl", default, skip_serializing_if = "Option::is_none")]
    pub listener_url: Option<String>,
    #[doc = "The availability mode of the availability group."]
    #[serde(rename = "availabilityMode", default, skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<distributed_availability_group_create_update_availability_group_configuration::AvailabilityMode>,
    #[doc = "The failover mode of the availability group."]
    #[serde(rename = "failoverMode", default, skip_serializing_if = "Option::is_none")]
    pub failover_mode: Option<distributed_availability_group_create_update_availability_group_configuration::FailoverMode>,
    #[doc = "The seeding mode of the availability group."]
    #[serde(rename = "seedingMode", default, skip_serializing_if = "Option::is_none")]
    pub seeding_mode: Option<distributed_availability_group_create_update_availability_group_configuration::SeedingMode>,
    #[doc = "The availability group certificate configuration."]
    #[serde(rename = "certificateConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub certificate_configuration: Option<DistributedAvailabilityGroupCreateUpdateAvailabilityGroupCertificateConfiguration>,
}
impl DistributedAvailabilityGroupCreateUpdateAvailabilityGroupConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod distributed_availability_group_create_update_availability_group_configuration {
    use super::*;
    #[doc = "The availability mode of the availability group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AvailabilityMode")]
    pub enum AvailabilityMode {
        #[serde(rename = "SYNCHRONOUS_COMMIT")]
        SynchronousCommit,
        #[serde(rename = "ASYNCHRONOUS_COMMIT")]
        AsynchronousCommit,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AvailabilityMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AvailabilityMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AvailabilityMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SynchronousCommit => serializer.serialize_unit_variant("AvailabilityMode", 0u32, "SYNCHRONOUS_COMMIT"),
                Self::AsynchronousCommit => serializer.serialize_unit_variant("AvailabilityMode", 1u32, "ASYNCHRONOUS_COMMIT"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The failover mode of the availability group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FailoverMode")]
    pub enum FailoverMode {
        #[serde(rename = "AUTOMATIC")]
        Automatic,
        #[serde(rename = "MANUAL")]
        Manual,
        #[serde(rename = "EXTERNAL")]
        External,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FailoverMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FailoverMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FailoverMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Automatic => serializer.serialize_unit_variant("FailoverMode", 0u32, "AUTOMATIC"),
                Self::Manual => serializer.serialize_unit_variant("FailoverMode", 1u32, "MANUAL"),
                Self::External => serializer.serialize_unit_variant("FailoverMode", 2u32, "EXTERNAL"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The seeding mode of the availability group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SeedingMode {
        #[serde(rename = "AUTOMATIC")]
        Automatic,
        #[serde(rename = "MANUAL")]
        Manual,
    }
}
#[doc = "Options used in creating a distributed availability group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DistributedAvailabilityGroupCreateUpdateConfiguration {
    #[doc = "Name of the availability group."]
    #[serde(rename = "availabilityGroupName", default, skip_serializing_if = "Option::is_none")]
    pub availability_group_name: Option<String>,
    #[doc = "The availability group configuration specification for a distributed availability group."]
    #[serde(rename = "primaryAvailabilityGroup", default, skip_serializing_if = "Option::is_none")]
    pub primary_availability_group: Option<DistributedAvailabilityGroupCreateUpdateAvailabilityGroupConfiguration>,
    #[doc = "The availability group configuration specification for a distributed availability group."]
    #[serde(rename = "secondaryAvailabilityGroup", default, skip_serializing_if = "Option::is_none")]
    pub secondary_availability_group: Option<DistributedAvailabilityGroupCreateUpdateAvailabilityGroupConfiguration>,
}
impl DistributedAvailabilityGroupCreateUpdateConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The encryption algorithm(s) used by the endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EncryptionAlgorithm")]
pub enum EncryptionAlgorithm {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "RC4")]
    Rc4,
    #[serde(rename = "AES")]
    Aes,
    #[serde(rename = "NONE, RC4")]
    NoneRc4,
    #[serde(rename = "NONE, AES")]
    NoneAes,
    #[serde(rename = "RC4, AES")]
    Rc4Aes,
    #[serde(rename = "AES, RC4")]
    AesRc4,
    #[serde(rename = "NONE, RC4, AES")]
    NoneRc4Aes,
    #[serde(rename = "NONE, AES, RC4")]
    NoneAesRc4,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EncryptionAlgorithm {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EncryptionAlgorithm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EncryptionAlgorithm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("EncryptionAlgorithm", 0u32, "NONE"),
            Self::Rc4 => serializer.serialize_unit_variant("EncryptionAlgorithm", 1u32, "RC4"),
            Self::Aes => serializer.serialize_unit_variant("EncryptionAlgorithm", 2u32, "AES"),
            Self::NoneRc4 => serializer.serialize_unit_variant("EncryptionAlgorithm", 3u32, "NONE, RC4"),
            Self::NoneAes => serializer.serialize_unit_variant("EncryptionAlgorithm", 4u32, "NONE, AES"),
            Self::Rc4Aes => serializer.serialize_unit_variant("EncryptionAlgorithm", 5u32, "RC4, AES"),
            Self::AesRc4 => serializer.serialize_unit_variant("EncryptionAlgorithm", 6u32, "AES, RC4"),
            Self::NoneRc4Aes => serializer.serialize_unit_variant("EncryptionAlgorithm", 7u32, "NONE, RC4, AES"),
            Self::NoneAesRc4 => serializer.serialize_unit_variant("EncryptionAlgorithm", 8u32, "NONE, AES, RC4"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "The complex type of the extended location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocation {
    #[doc = "The name of the extended location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of extendedLocation."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ExtendedLocationType>,
}
impl ExtendedLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of extendedLocation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ExtendedLocationType")]
pub enum ExtendedLocationType {
    CustomLocation,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ExtendedLocationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ExtendedLocationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ExtendedLocationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CustomLocation => serializer.serialize_unit_variant("ExtendedLocationType", 0u32, "CustomLocation"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Failover Cluster Instance properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverCluster {
    #[doc = "The GUID of the SQL Server's underlying Failover Cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The network name to connect to the SQL FCI."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "The ARM IDs of the Arc SQL Server resources, belonging to the current server's Failover cluster."]
    #[serde(
        rename = "sqlInstanceIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sql_instance_ids: Vec<String>,
    #[doc = "The host names which are part of the SQL FCI resource group."]
    #[serde(
        rename = "hostNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub host_names: Vec<String>,
}
impl FailoverCluster {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of failover groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverGroupListResult {
    #[doc = "Array of failover group results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<FailoverGroupResource>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FailoverGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FailoverGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a failover group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailoverGroupProperties {
    #[doc = "The provisioning state of the failover group resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<failover_group_properties::ProvisioningState>,
    #[doc = "The resource ID of the partner SQL managed instance."]
    #[serde(rename = "partnerManagedInstanceId")]
    pub partner_managed_instance_id: String,
    #[doc = "The specifications of the failover group resource."]
    pub spec: FailoverGroupSpec,
    #[doc = "The status of the failover group custom resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<serde_json::Value>,
}
impl FailoverGroupProperties {
    pub fn new(partner_managed_instance_id: String, spec: FailoverGroupSpec) -> Self {
        Self {
            provisioning_state: None,
            partner_managed_instance_id,
            spec,
            status: None,
        }
    }
}
pub mod failover_group_properties {
    use super::*;
    #[doc = "The provisioning state of the failover group resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Accepted,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A failover group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailoverGroupResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a failover group resource."]
    pub properties: FailoverGroupProperties,
}
impl FailoverGroupResource {
    pub fn new(properties: FailoverGroupProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "The specifications of the failover group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailoverGroupSpec {
    #[doc = "The shared name of the failover group for this SQL managed instance. Both SQL managed instance and its partner have to use the same shared name."]
    #[serde(rename = "sharedName", default, skip_serializing_if = "Option::is_none")]
    pub shared_name: Option<String>,
    #[doc = "The name of the SQL managed instance with this failover group role."]
    #[serde(rename = "sourceMI", default, skip_serializing_if = "Option::is_none")]
    pub source_mi: Option<String>,
    #[doc = "The name of the partner SQL managed instance."]
    #[serde(rename = "partnerMI", default, skip_serializing_if = "Option::is_none")]
    pub partner_mi: Option<String>,
    #[doc = "The mirroring endpoint URL of the partner SQL managed instance."]
    #[serde(rename = "partnerMirroringURL", default, skip_serializing_if = "Option::is_none")]
    pub partner_mirroring_url: Option<String>,
    #[doc = "The mirroring endpoint public certificate for the partner SQL managed instance. Only PEM format is supported."]
    #[serde(rename = "partnerMirroringCert", default, skip_serializing_if = "Option::is_none")]
    pub partner_mirroring_cert: Option<String>,
    #[doc = "The partner sync mode of the SQL managed instance."]
    #[serde(rename = "partnerSyncMode", default, skip_serializing_if = "Option::is_none")]
    pub partner_sync_mode: Option<failover_group_spec::PartnerSyncMode>,
    #[doc = "The role of the SQL managed instance in this failover group."]
    pub role: failover_group_spec::Role,
}
impl FailoverGroupSpec {
    pub fn new(role: failover_group_spec::Role) -> Self {
        Self {
            shared_name: None,
            source_mi: None,
            partner_mi: None,
            partner_mirroring_url: None,
            partner_mirroring_cert: None,
            partner_sync_mode: None,
            role,
        }
    }
}
pub mod failover_group_spec {
    use super::*;
    #[doc = "The partner sync mode of the SQL managed instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PartnerSyncMode")]
    pub enum PartnerSyncMode {
        #[serde(rename = "async")]
        Async,
        #[serde(rename = "sync")]
        Sync,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PartnerSyncMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PartnerSyncMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PartnerSyncMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Async => serializer.serialize_unit_variant("PartnerSyncMode", 0u32, "async"),
                Self::Sync => serializer.serialize_unit_variant("PartnerSyncMode", 1u32, "sync"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for PartnerSyncMode {
        fn default() -> Self {
            Self::Async
        }
    }
    #[doc = "The role of the SQL managed instance in this failover group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Role")]
    pub enum Role {
        #[serde(rename = "primary")]
        Primary,
        #[serde(rename = "secondary")]
        Secondary,
        #[serde(rename = "force-primary-allow-data-loss")]
        ForcePrimaryAllowDataLoss,
        #[serde(rename = "force-secondary")]
        ForceSecondary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Role {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Role {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Role {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("Role", 0u32, "primary"),
                Self::Secondary => serializer.serialize_unit_variant("Role", 1u32, "secondary"),
                Self::ForcePrimaryAllowDataLoss => serializer.serialize_unit_variant("Role", 2u32, "force-primary-allow-data-loss"),
                Self::ForceSecondary => serializer.serialize_unit_variant("Role", 3u32, "force-secondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Role {
        fn default() -> Self {
            Self::Primary
        }
    }
}
#[doc = "The kubernetes active directory information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct K8sActiveDirectory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connector: Option<k8s_active_directory::Connector>,
    #[doc = "Account name for AAD"]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "Keytab secret used to authenticate with Active Directory."]
    #[serde(rename = "keytabSecret", default, skip_serializing_if = "Option::is_none")]
    pub keytab_secret: Option<String>,
    #[doc = "An array of encryption types"]
    #[serde(
        rename = "encryptionTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub encryption_types: Vec<String>,
}
impl K8sActiveDirectory {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod k8s_active_directory {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Connector {
        #[doc = "Name of the connector"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[doc = "Name space of the connector"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub namespace: Option<String>,
    }
    impl Connector {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The kubernetes network settings information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct K8sNetworkSettings {
    #[doc = "If 1, then SQL Server forces all connections to be encrypted. By default, this option is 0"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forceencryption: Option<i32>,
    #[doc = "Specifies which ciphers are allowed by SQL Server for TLS"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tlsciphers: Option<String>,
    #[doc = "A comma-separated list of which TLS protocols are allowed by SQL Server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tlsprotocols: Option<String>,
}
impl K8sNetworkSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kubernetes resource limits and requests used to restrict or reserve resource usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct K8sResourceRequirements {
    #[doc = "Requests for a kubernetes resource type (e.g 'cpu', 'memory'). The 'cpu' request must be less than or equal to 'cpu' limit. Default 'cpu' is 2, minimum is 1. Default 'memory' is '4Gi', minimum is '2Gi. If sku.tier is GeneralPurpose, maximum 'cpu' is 24 and maximum 'memory' is '128Gi'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<serde_json::Value>,
    #[doc = "Limits for a kubernetes resource type (e.g 'cpu', 'memory'). The 'cpu' request must be less than or equal to 'cpu' limit. Default 'cpu' is 2, minimum is 1. Default 'memory' is '4Gi', minimum is '2Gi. If sku.tier is GeneralPurpose, maximum 'cpu' is 24 and maximum 'memory' is '128Gi'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<serde_json::Value>,
}
impl K8sResourceRequirements {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kubernetes scheduling information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct K8sScheduling {
    #[doc = "The kubernetes scheduling options. It describes restrictions used to help Kubernetes select appropriate nodes to host the database service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<K8sSchedulingOptions>,
}
impl K8sScheduling {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kubernetes scheduling options. It describes restrictions used to help Kubernetes select appropriate nodes to host the database service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct K8sSchedulingOptions {
    #[doc = "The kubernetes resource limits and requests used to restrict or reserve resource usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<K8sResourceRequirements>,
}
impl K8sSchedulingOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kubernetes security information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct K8sSecurity {
    #[doc = "Admin login secret key"]
    #[serde(rename = "adminLoginSecret", default, skip_serializing_if = "Option::is_none")]
    pub admin_login_secret: Option<String>,
    #[doc = "Service certificate secret used"]
    #[serde(rename = "serviceCertificateSecret", default, skip_serializing_if = "Option::is_none")]
    pub service_certificate_secret: Option<String>,
    #[doc = "The kubernetes active directory information."]
    #[serde(rename = "activeDirectory", default, skip_serializing_if = "Option::is_none")]
    pub active_directory: Option<K8sActiveDirectory>,
    #[doc = "Transparent data encryption information."]
    #[serde(rename = "transparentDataEncryption", default, skip_serializing_if = "Option::is_none")]
    pub transparent_data_encryption: Option<K8stransparentDataEncryption>,
}
impl K8sSecurity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kubernetes settings information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct K8sSettings {
    #[doc = "The kubernetes network settings information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<K8sNetworkSettings>,
}
impl K8sSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Keytab used for authenticate with Active Directory."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeytabInformation {
    #[doc = "A base64-encoded keytab."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keytab: Option<String>,
}
impl KeytabInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Log analytics workspace id and primary key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogAnalyticsWorkspaceConfig {
    #[doc = "Azure Log Analytics workspace ID"]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "Primary key of the workspace"]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
}
impl LogAnalyticsWorkspaceConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Migration related configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Migration {
    #[doc = "The migration assessment related configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assessment: Option<MigrationAssessment>,
}
impl Migration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The migration assessment related configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationAssessment {
    #[doc = "Indicates if migration assessment is enabled for this SQL Server instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The time when Migration Assessment Report upload was last performed."]
    #[serde(rename = "assessmentUploadTime", default, with = "azure_core::date::rfc3339::option")]
    pub assessment_upload_time: Option<::time::OffsetDateTime>,
    #[doc = "Issues and warnings impacting the migration of SQL Server instance to particular Azure Migration Target."]
    #[serde(rename = "serverAssessments", default, skip_serializing_if = "Option::is_none")]
    pub server_assessments: Option<ServerAssessments>,
    #[doc = "SKU Recommendation results for Azure migration targets for SQL Server."]
    #[serde(rename = "skuRecommendationResults", default, skip_serializing_if = "Option::is_none")]
    pub sku_recommendation_results: Option<SkuRecommendationResults>,
}
impl MigrationAssessment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The monitoring configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Monitoring {
    #[doc = "Indicates if monitoring is enabled for this SQL Server instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl Monitoring {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties from the Kubernetes data controller"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnPremiseProperty {
    #[doc = "A globally unique ID identifying the associated Kubernetes cluster"]
    pub id: String,
    #[doc = "Certificate that contains the Kubernetes cluster public key used to verify signing"]
    #[serde(rename = "publicSigningKey")]
    pub public_signing_key: String,
    #[doc = "Unique thumbprint returned to customer to verify the certificate being uploaded"]
    #[serde(rename = "signingCertificateThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub signing_certificate_thumbprint: Option<String>,
}
impl OnPremiseProperty {
    pub fn new(id: String, public_signing_key: String) -> Self {
        Self {
            id,
            public_signing_key,
            signing_certificate_thumbprint: None,
        }
    }
}
#[doc = "Azure Data Services on Azure Arc operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[doc = "The name of the operation being performed on this particular object."]
    pub name: String,
    #[doc = "Display metadata associated with the operation."]
    pub display: OperationDisplay,
    #[doc = "The intended executor of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction")]
    pub is_data_action: bool,
    #[doc = "Additional descriptions for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new(name: String, display: OperationDisplay, is_data_action: bool) -> Self {
        Self {
            name,
            display,
            origin: None,
            is_data_action,
            properties: None,
        }
    }
}
pub mod operation {
    use super::*;
    #[doc = "The intended executor of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Display metadata associated with the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    #[doc = "The localized friendly form of the resource provider name."]
    pub provider: String,
    #[doc = "The localized friendly form of the resource type related to this action/operation."]
    pub resource: String,
    #[doc = "The localized friendly name for the operation."]
    pub operation: String,
    #[doc = "The localized friendly description for the operation."]
    pub description: String,
}
impl OperationDisplay {
    pub fn new(provider: String, resource: String, operation: String, description: String) -> Self {
        Self {
            provider,
            resource,
            operation,
            description,
        }
    }
}
#[doc = "Result of the request to list Azure Data Services on Azure Arc operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "Link to retrieve next page of results."]
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
#[doc = "A list of data controllers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PageOfDataControllerResource {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DataControllerResource>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PageOfDataControllerResource {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PageOfDataControllerResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Postgres Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostgresInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "Postgres Instance properties."]
    pub properties: PostgresInstanceProperties,
    #[doc = "The resource model definition representing SKU for Azure Database for PostgresSQL - Azure Arc"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PostgresInstanceSku>,
}
impl PostgresInstance {
    pub fn new(tracked_resource: TrackedResource, properties: PostgresInstanceProperties) -> Self {
        Self {
            tracked_resource,
            extended_location: None,
            properties,
            sku: None,
        }
    }
}
#[doc = "A list of PostgresInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostgresInstanceListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PostgresInstance>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PostgresInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PostgresInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Postgres Instance properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostgresInstanceProperties {
    #[doc = "The data controller id"]
    #[serde(rename = "dataControllerId", default, skip_serializing_if = "Option::is_none")]
    pub data_controller_id: Option<String>,
    #[doc = "The instance admin"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<String>,
    #[doc = "Username and password for basic login authentication."]
    #[serde(rename = "basicLoginInformation", default, skip_serializing_if = "Option::is_none")]
    pub basic_login_information: Option<BasicLoginInformation>,
    #[doc = "The raw kubernetes information"]
    #[serde(rename = "k8sRaw", default, skip_serializing_if = "Option::is_none")]
    pub k8s_raw: Option<serde_json::Value>,
    #[doc = "Last uploaded date from Kubernetes cluster. Defaults to current date time"]
    #[serde(rename = "lastUploadedDate", default, with = "azure_core::date::rfc3339::option")]
    pub last_uploaded_date: Option<::time::OffsetDateTime>,
    #[doc = "The provisioning state of the Azure Arc-enabled PostgreSQL instance."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl PostgresInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition representing SKU for Azure Database for PostgresSQL - Azure Arc"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostgresInstanceSku {
    #[serde(flatten)]
    pub common_sku: CommonSku,
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<postgres_instance_sku::Tier>,
}
impl PostgresInstanceSku {
    pub fn new(common_sku: CommonSku) -> Self {
        Self { common_sku, tier: None }
    }
}
pub mod postgres_instance_sku {
    use super::*;
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Hyperscale,
    }
    impl Default for Tier {
        fn default() -> Self {
            Self::Hyperscale
        }
    }
}
#[doc = "An update to a Postgres Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostgresInstanceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Postgres Instance properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PostgresInstanceProperties>,
}
impl PostgresInstanceUpdate {
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
#[doc = "The sequencer action details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SequencerAction {
    #[doc = "The unique identifier of the sequencer action."]
    #[serde(rename = "actionId", default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[doc = "The state of the sequencer action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<sequencer_action::State>,
    #[doc = "The result of the sequencer action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<sequencer_action::Result>,
}
impl SequencerAction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sequencer_action {
    use super::*;
    #[doc = "The state of the sequencer action."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        NotStarted,
        WaitingPredecessors,
        ExecutingAction,
        CreatingSuccessors,
        Completed,
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
                Self::NotStarted => serializer.serialize_unit_variant("State", 0u32, "NotStarted"),
                Self::WaitingPredecessors => serializer.serialize_unit_variant("State", 1u32, "WaitingPredecessors"),
                Self::ExecutingAction => serializer.serialize_unit_variant("State", 2u32, "ExecutingAction"),
                Self::CreatingSuccessors => serializer.serialize_unit_variant("State", 3u32, "CreatingSuccessors"),
                Self::Completed => serializer.serialize_unit_variant("State", 4u32, "Completed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The result of the sequencer action."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Result")]
    pub enum Result {
        NotCompleted,
        Succeeded,
        Failed,
        TimedOut,
        Skipped,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Result {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Result {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Result {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotCompleted => serializer.serialize_unit_variant("Result", 0u32, "NotCompleted"),
                Self::Succeeded => serializer.serialize_unit_variant("Result", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Result", 2u32, "Failed"),
                Self::TimedOut => serializer.serialize_unit_variant("Result", 3u32, "TimedOut"),
                Self::Skipped => serializer.serialize_unit_variant("Result", 4u32, "Skipped"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type ServerAssessments = Vec<serde_json::Value>;
#[doc = "Indicates if the resource represents a SQL Server engine or a SQL Server component service installed on the host."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceType")]
pub enum ServiceType {
    Engine,
    #[serde(rename = "SSRS")]
    Ssrs,
    #[serde(rename = "SSAS")]
    Ssas,
    #[serde(rename = "SSIS")]
    Ssis,
    #[serde(rename = "PBIRS")]
    Pbirs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Engine => serializer.serialize_unit_variant("ServiceType", 0u32, "Engine"),
            Self::Ssrs => serializer.serialize_unit_variant("ServiceType", 1u32, "SSRS"),
            Self::Ssas => serializer.serialize_unit_variant("ServiceType", 2u32, "SSAS"),
            Self::Ssis => serializer.serialize_unit_variant("ServiceType", 3u32, "SSIS"),
            Self::Pbirs => serializer.serialize_unit_variant("ServiceType", 4u32, "PBIRS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SKU Recommendation results for Azure migration targets for SQL Server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuRecommendationResults {
    #[doc = "SKU Recommendation results for Azure SQL Database."]
    #[serde(rename = "azureSqlDatabase", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_database: Option<SkuRecommendationResultsAzureSqlDatabase>,
    #[doc = "SKU Recommendation results for Azure SQL Managed Instance."]
    #[serde(rename = "azureSqlManagedInstance", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_managed_instance: Option<SkuRecommendationResultsAzureSqlManagedInstance>,
    #[doc = "SKU Recommendation results for Azure SQL Virtual Machine."]
    #[serde(rename = "azureSqlVirtualMachine", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_virtual_machine: Option<SkuRecommendationResultsAzureSqlVirtualMachine>,
}
impl SkuRecommendationResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU Recommendation results for Azure SQL Database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuRecommendationResultsAzureSqlDatabase {
    #[doc = "The target recommendation Status for this database."]
    #[serde(rename = "recommendationStatus", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_status: Option<SkuRecommendationResultsRecommendationStatus>,
    #[doc = "Number of blocker issues to fix before migrating to the target platform."]
    #[serde(rename = "numberOfServerBlockerIssues", default, skip_serializing_if = "Option::is_none")]
    pub number_of_server_blocker_issues: Option<SkuRecommendationResultsNumberOfServerBlockerIssues>,
    #[doc = "The Monthly cost of the particular SKU."]
    #[serde(rename = "monthlyCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_cost: Option<SkuRecommendationResultsMonthlyCost>,
    #[serde(rename = "targetSku", default, skip_serializing_if = "Option::is_none")]
    pub target_sku: Option<sku_recommendation_results_azure_sql_database::TargetSku>,
}
impl SkuRecommendationResultsAzureSqlDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku_recommendation_results_azure_sql_database {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct TargetSku {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub category: Option<target_sku::Category>,
    }
    impl TargetSku {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod target_sku {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Category {
            #[doc = "The compute tier of the target SKU."]
            #[serde(rename = "computeTier", default, skip_serializing_if = "Option::is_none")]
            pub compute_tier: Option<String>,
            #[doc = "The hardware type of the target SKU."]
            #[serde(rename = "hardwareType", default, skip_serializing_if = "Option::is_none")]
            pub hardware_type: Option<String>,
            #[doc = "The SQL purchasing model of the target SKU."]
            #[serde(rename = "sqlPurchasingModel", default, skip_serializing_if = "Option::is_none")]
            pub sql_purchasing_model: Option<String>,
            #[doc = "The SQL service tier of the target SKU."]
            #[serde(rename = "sqlServiceTier", default, skip_serializing_if = "Option::is_none")]
            pub sql_service_tier: Option<String>,
            #[doc = "Indicates if zone redundancy is available for the target SKU."]
            #[serde(rename = "zoneRedundancyAvailable", default, skip_serializing_if = "Option::is_none")]
            pub zone_redundancy_available: Option<bool>,
        }
        impl Category {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[doc = "SKU Recommendation results for Azure SQL Managed Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuRecommendationResultsAzureSqlManagedInstance {
    #[doc = "The target recommendation Status for this database."]
    #[serde(rename = "recommendationStatus", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_status: Option<SkuRecommendationResultsRecommendationStatus>,
    #[doc = "Number of blocker issues to fix before migrating to the target platform."]
    #[serde(rename = "numberOfServerBlockerIssues", default, skip_serializing_if = "Option::is_none")]
    pub number_of_server_blocker_issues: Option<SkuRecommendationResultsNumberOfServerBlockerIssues>,
    #[doc = "The Monthly cost of the particular SKU."]
    #[serde(rename = "monthlyCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_cost: Option<SkuRecommendationResultsMonthlyCost>,
    #[serde(rename = "targetSku", default, skip_serializing_if = "Option::is_none")]
    pub target_sku: Option<sku_recommendation_results_azure_sql_managed_instance::TargetSku>,
}
impl SkuRecommendationResultsAzureSqlManagedInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku_recommendation_results_azure_sql_managed_instance {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct TargetSku {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub category: Option<target_sku::Category>,
    }
    impl TargetSku {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod target_sku {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Category {
            #[doc = "The compute tier of the target SKU."]
            #[serde(rename = "computeTier", default, skip_serializing_if = "Option::is_none")]
            pub compute_tier: Option<String>,
            #[doc = "The hardware type of the target SKU."]
            #[serde(rename = "hardwareType", default, skip_serializing_if = "Option::is_none")]
            pub hardware_type: Option<String>,
            #[doc = "The SQL purchasing model of the target SKU."]
            #[serde(rename = "sqlPurchasingModel", default, skip_serializing_if = "Option::is_none")]
            pub sql_purchasing_model: Option<String>,
            #[doc = "The SQL service tier of the target SKU."]
            #[serde(rename = "sqlServiceTier", default, skip_serializing_if = "Option::is_none")]
            pub sql_service_tier: Option<String>,
            #[doc = "Indicates if zone redundancy is available for the target SKU."]
            #[serde(rename = "zoneRedundancyAvailable", default, skip_serializing_if = "Option::is_none")]
            pub zone_redundancy_available: Option<bool>,
        }
        impl Category {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[doc = "SKU Recommendation results for Azure SQL Virtual Machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuRecommendationResultsAzureSqlVirtualMachine {
    #[doc = "The target recommendation Status for this database."]
    #[serde(rename = "recommendationStatus", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_status: Option<SkuRecommendationResultsRecommendationStatus>,
    #[doc = "Number of blocker issues to fix before migrating to the target platform."]
    #[serde(rename = "numberOfServerBlockerIssues", default, skip_serializing_if = "Option::is_none")]
    pub number_of_server_blocker_issues: Option<SkuRecommendationResultsNumberOfServerBlockerIssues>,
    #[doc = "The Monthly cost of the particular SKU."]
    #[serde(rename = "monthlyCost", default, skip_serializing_if = "Option::is_none")]
    pub monthly_cost: Option<SkuRecommendationResultsMonthlyCost>,
    #[serde(rename = "targetSku", default, skip_serializing_if = "Option::is_none")]
    pub target_sku: Option<sku_recommendation_results_azure_sql_virtual_machine::TargetSku>,
}
impl SkuRecommendationResultsAzureSqlVirtualMachine {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku_recommendation_results_azure_sql_virtual_machine {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct TargetSku {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub category: Option<target_sku::Category>,
    }
    impl TargetSku {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod target_sku {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Category {
            #[doc = "Available VM SKUs for the Azure SQL Virtual Machine."]
            #[serde(
                rename = "availableVmSkus",
                default,
                deserialize_with = "azure_core::util::deserialize_null_as_default",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub available_vm_skus: Vec<String>,
            #[doc = "The virtual machine family of the target SKU."]
            #[serde(rename = "virtualMachineFamily", default, skip_serializing_if = "Option::is_none")]
            pub virtual_machine_family: Option<String>,
        }
        impl Category {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[doc = "The Monthly cost of the particular SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuRecommendationResultsMonthlyCost {
    #[doc = "Represents the Cost of Compute."]
    #[serde(rename = "computeCost", default, skip_serializing_if = "Option::is_none")]
    pub compute_cost: Option<f32>,
    #[doc = "Represents the Cost of Storage."]
    #[serde(rename = "storageCost", default, skip_serializing_if = "Option::is_none")]
    pub storage_cost: Option<f32>,
    #[doc = "Represents the Total Cost."]
    #[serde(rename = "totalCost", default, skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<f32>,
}
impl SkuRecommendationResultsMonthlyCost {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type SkuRecommendationResultsNumberOfBlockerIssues = i32;
pub type SkuRecommendationResultsNumberOfServerBlockerIssues = i32;
#[doc = "The target recommendation Status for this database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuRecommendationResultsRecommendationStatus")]
pub enum SkuRecommendationResultsRecommendationStatus {
    NotReady,
    Ready,
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SkuRecommendationResultsRecommendationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SkuRecommendationResultsRecommendationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SkuRecommendationResultsRecommendationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotReady => serializer.serialize_unit_variant("SkuRecommendationResultsRecommendationStatus", 0u32, "NotReady"),
            Self::Ready => serializer.serialize_unit_variant("SkuRecommendationResultsRecommendationStatus", 1u32, "Ready"),
            Self::Unknown => serializer.serialize_unit_variant("SkuRecommendationResultsRecommendationStatus", 2u32, "Unknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The SKU recommendation summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuRecommendationSummary {
    #[doc = "Number of blocker issues to fix before migrating this database to the target platform."]
    #[serde(rename = "numOfBlockerIssues", default, skip_serializing_if = "Option::is_none")]
    pub num_of_blocker_issues: Option<SkuRecommendationResultsNumberOfBlockerIssues>,
    #[doc = "The target recommendation Status for this database."]
    #[serde(rename = "recommendationStatus", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_status: Option<SkuRecommendationResultsRecommendationStatus>,
}
impl SkuRecommendationSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of Arc Sql availability group database replica resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAvailabilityGroupDatabaseReplicaResourceProperties {
    #[doc = "the database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "the database replica name."]
    #[serde(rename = "replicaName", default, skip_serializing_if = "Option::is_none")]
    pub replica_name: Option<String>,
    #[doc = "Whether the availability database is local."]
    #[serde(rename = "isLocal", default, skip_serializing_if = "Option::is_none")]
    pub is_local: Option<bool>,
    #[doc = "Returns 1 if the replica is primary, or 0 if it is a secondary replica."]
    #[serde(rename = "isPrimaryReplica", default, skip_serializing_if = "Option::is_none")]
    pub is_primary_replica: Option<bool>,
    #[doc = "Description of the data-movement state."]
    #[serde(rename = "synchronizationStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub synchronization_state_description: Option<String>,
    #[doc = "Whether this replica is transaction committer."]
    #[serde(rename = "isCommitParticipant", default, skip_serializing_if = "Option::is_none")]
    pub is_commit_participant: Option<bool>,
    #[doc = "Description of the health of database."]
    #[serde(rename = "synchronizationHealthDescription", default, skip_serializing_if = "Option::is_none")]
    pub synchronization_health_description: Option<String>,
    #[doc = "Description of the database state of the availability replica."]
    #[serde(rename = "databaseStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub database_state_description: Option<String>,
    #[doc = "Whether this data movement is suspended."]
    #[serde(rename = "isSuspended", default, skip_serializing_if = "Option::is_none")]
    pub is_suspended: Option<bool>,
    #[doc = "Description of the database suspended state reason."]
    #[serde(rename = "suspendReasonDescription", default, skip_serializing_if = "Option::is_none")]
    pub suspend_reason_description: Option<String>,
}
impl SqlAvailabilityGroupDatabaseReplicaResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a DHCP Arc Sql availability group listener"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAvailabilityGroupDhcpListenerProperties {
    #[doc = "the DNS name for the listener."]
    #[serde(rename = "dnsName", default, skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[doc = "The IPV4 subnet for the listener."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[doc = "The netmask for the listener."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    #[doc = "Network port for the listener. Default is 1433."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
impl SqlAvailabilityGroupDhcpListenerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type SqlAvailabilityGroupIpV4AddressesAndMasksProperties = Vec<serde_json::Value>;
#[doc = "The properties of Arc Sql availability group replica resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAvailabilityGroupReplicaResourceProperties {
    #[doc = "ID GUID of the availability group."]
    #[serde(rename = "replicaId", default, skip_serializing_if = "Option::is_none")]
    pub replica_id: Option<String>,
    #[doc = "the replica name."]
    #[serde(rename = "replicaName", default, skip_serializing_if = "Option::is_none")]
    pub replica_name: Option<String>,
    #[doc = "The specifications of the availability group replica configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configure: Option<AvailabilityGroupConfigure>,
    #[doc = "The specifications of the availability group state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<AvailabilityGroupState>,
}
impl SqlAvailabilityGroupReplicaResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a static IP Arc Sql availability group listener"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlAvailabilityGroupStaticIpListenerProperties {
    #[doc = "the DNS name for the listener."]
    #[serde(rename = "dnsName", default, skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[doc = "Address and netmask information for an IPv4 AG listener."]
    #[serde(rename = "ipV4AddressesAndMasks", default, skip_serializing_if = "Option::is_none")]
    pub ip_v4_addresses_and_masks: Option<SqlAvailabilityGroupIpV4AddressesAndMasksProperties>,
    #[doc = "IP V6 Addresses for the listener"]
    #[serde(
        rename = "ipV6Addresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ip_v6_addresses: Vec<String>,
    #[doc = "Network port for the listener. Default is 1433."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
impl SqlAvailabilityGroupStaticIpListenerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A SqlManagedInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlManagedInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of sqlManagedInstance."]
    pub properties: SqlManagedInstanceProperties,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "The resource model definition representing SKU for Azure Managed Instance - Azure Arc"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SqlManagedInstanceSku>,
}
impl SqlManagedInstance {
    pub fn new(tracked_resource: TrackedResource, properties: SqlManagedInstanceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
            extended_location: None,
            sku: None,
        }
    }
}
#[doc = "The raw kubernetes information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlManagedInstanceK8sRaw {
    #[doc = "The kubernetes spec information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<SqlManagedInstanceK8sSpec>,
}
impl SqlManagedInstanceK8sRaw {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kubernetes spec information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlManagedInstanceK8sSpec {
    #[doc = "The kubernetes scheduling information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<K8sScheduling>,
    #[doc = "This option specifies the number of SQL Managed Instance replicas that will be deployed in your Kubernetes cluster for high availability purposes. If sku.tier is BusinessCritical, allowed values are '2' or '3' with default of '3'. If sku.tier is GeneralPurpose, replicas must be '1'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[doc = "The kubernetes security information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<K8sSecurity>,
    #[doc = "The kubernetes settings information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<K8sSettings>,
}
impl SqlManagedInstanceK8sSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of SqlManagedInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlManagedInstanceListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SqlManagedInstance>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlManagedInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlManagedInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of sqlManagedInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlManagedInstanceProperties {
    #[doc = "null"]
    #[serde(rename = "dataControllerId", default, skip_serializing_if = "Option::is_none")]
    pub data_controller_id: Option<String>,
    #[doc = "The instance admin user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<String>,
    #[doc = "The instance start time"]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The instance end time"]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The raw kubernetes information."]
    #[serde(rename = "k8sRaw", default, skip_serializing_if = "Option::is_none")]
    pub k8s_raw: Option<SqlManagedInstanceK8sRaw>,
    #[doc = "Username and password for basic login authentication."]
    #[serde(rename = "basicLoginInformation", default, skip_serializing_if = "Option::is_none")]
    pub basic_login_information: Option<BasicLoginInformation>,
    #[doc = "Last uploaded date from Kubernetes cluster. Defaults to current date time"]
    #[serde(rename = "lastUploadedDate", default, with = "azure_core::date::rfc3339::option")]
    pub last_uploaded_date: Option<::time::OffsetDateTime>,
    #[doc = "The provisioning state of the Arc-enabled SQL Managed Instance resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Active Directory information that related to the resource."]
    #[serde(rename = "activeDirectoryInformation", default, skip_serializing_if = "Option::is_none")]
    pub active_directory_information: Option<ActiveDirectoryInformation>,
    #[doc = "The license type to apply for this managed instance."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<sql_managed_instance_properties::LicenseType>,
    #[doc = "If a CustomLocation is provided, this contains the ARM id of the connected cluster the custom location belongs to."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "If a CustomLocation is provided, this contains the ARM id of the extension the custom location belongs to."]
    #[serde(rename = "extensionId", default, skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,
}
impl SqlManagedInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_managed_instance_properties {
    use super::*;
    #[doc = "The license type to apply for this managed instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        BasePrice,
        LicenseIncluded,
        DisasterRecovery,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BasePrice => serializer.serialize_unit_variant("LicenseType", 0u32, "BasePrice"),
                Self::LicenseIncluded => serializer.serialize_unit_variant("LicenseType", 1u32, "LicenseIncluded"),
                Self::DisasterRecovery => serializer.serialize_unit_variant("LicenseType", 2u32, "DisasterRecovery"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for LicenseType {
        fn default() -> Self {
            Self::BasePrice
        }
    }
}
#[doc = "The resource model definition representing SKU for Azure Managed Instance - Azure Arc"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlManagedInstanceSku {
    #[doc = "The name of the SKU."]
    pub name: sql_managed_instance_sku::Name,
    #[doc = "The pricing tier for the instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sql_managed_instance_sku::Tier>,
    #[doc = "Whether dev/test is enabled. When the dev field is set to true, the resource is used for dev/test purpose. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dev: Option<bool>,
    #[doc = "The SKU size. When the name field is the combination of tier and some other value, this would be the standalone code. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The SKU family"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The SKU capacity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl SqlManagedInstanceSku {
    pub fn new(name: sql_managed_instance_sku::Name) -> Self {
        Self {
            name,
            tier: None,
            dev: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
pub mod sql_managed_instance_sku {
    use super::*;
    #[doc = "The name of the SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "vCore")]
        VCore,
    }
    #[doc = "The pricing tier for the instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        GeneralPurpose,
        BusinessCritical,
    }
    impl Default for Tier {
        fn default() -> Self {
            Self::GeneralPurpose
        }
    }
}
#[doc = "An update to a SQL Managed Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlManagedInstanceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SqlManagedInstanceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Arc Sql Server Availability Group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerAvailabilityGroupResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of Arc Sql Server availability group resource"]
    pub properties: SqlServerAvailabilityGroupResourceProperties,
}
impl SqlServerAvailabilityGroupResource {
    pub fn new(tracked_resource: TrackedResource, properties: SqlServerAvailabilityGroupResourceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "The properties of Arc Sql Server availability group resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerAvailabilityGroupResourceProperties {
    #[doc = "ID GUID of the availability group."]
    #[serde(rename = "availabilityGroupId", default, skip_serializing_if = "Option::is_none")]
    pub availability_group_id: Option<String>,
    #[doc = "the SQL server name."]
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[doc = "the SQL Server Instance name."]
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[doc = "Timestamp for when the data was collected from the client machine."]
    #[serde(rename = "collectionTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub collection_timestamp: Option<::time::OffsetDateTime>,
    #[doc = "The specifications of the availability group state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<AvailabilityGroupInfo>,
    #[doc = "A list of Availability Group Replicas."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<sql_server_availability_group_resource_properties::Replicas>,
    #[doc = "A list of Availability Group Database Replicas."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub databases: Option<sql_server_availability_group_resource_properties::Databases>,
    #[doc = "The provisioning state of the Arc-enabled SQL Server availability group resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl SqlServerAvailabilityGroupResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_server_availability_group_resource_properties {
    use super::*;
    #[doc = "A list of Availability Group Replicas."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Replicas {
        #[doc = "Array of Availability Group Replicas."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub value: Vec<SqlAvailabilityGroupReplicaResourceProperties>,
        #[doc = "Link to retrieve next page of results."]
        #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
    }
    impl Replicas {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "A list of Availability Group Database Replicas."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Databases {
        #[doc = "Array of Availability Group Database Replicas."]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub value: Vec<SqlAvailabilityGroupDatabaseReplicaResourceProperties>,
        #[doc = "Link to retrieve next page of results."]
        #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
        pub next_link: Option<String>,
    }
    impl Databases {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "An update to availability group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerAvailabilityGroupUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The properties of Arc Sql Server availability group resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlServerAvailabilityGroupResourceProperties>,
}
impl SqlServerAvailabilityGroupUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Arc Sql Server database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerDatabaseResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of Arc Sql Server database resource"]
    pub properties: SqlServerDatabaseResourceProperties,
}
impl SqlServerDatabaseResource {
    pub fn new(tracked_resource: TrackedResource, properties: SqlServerDatabaseResourceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "The properties of Arc Sql Server database resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerDatabaseResourceProperties {
    #[doc = "Collation of the database."]
    #[serde(rename = "collationName", default, skip_serializing_if = "Option::is_none")]
    pub collation_name: Option<String>,
    #[doc = "Creation date of the database."]
    #[serde(rename = "databaseCreationDate", default, with = "azure_core::date::rfc3339::option")]
    pub database_creation_date: Option<::time::OffsetDateTime>,
    #[doc = "Compatibility level of the database"]
    #[serde(rename = "compatibilityLevel", default, skip_serializing_if = "Option::is_none")]
    pub compatibility_level: Option<i32>,
    #[doc = "Size of the database."]
    #[serde(rename = "sizeMB", default, skip_serializing_if = "Option::is_none")]
    pub size_mb: Option<f32>,
    #[doc = "Space left of the database."]
    #[serde(rename = "spaceAvailableMB", default, skip_serializing_if = "Option::is_none")]
    pub space_available_mb: Option<f32>,
    #[doc = "State of the database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<sql_server_database_resource_properties::State>,
    #[doc = "Whether the database is read only or not."]
    #[serde(rename = "isReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
    #[doc = "Status of the database."]
    #[serde(rename = "recoveryMode", default, skip_serializing_if = "Option::is_none")]
    pub recovery_mode: Option<sql_server_database_resource_properties::RecoveryMode>,
    #[doc = "List of features that are enabled for the database"]
    #[serde(rename = "databaseOptions", default, skip_serializing_if = "Option::is_none")]
    pub database_options: Option<sql_server_database_resource_properties::DatabaseOptions>,
    #[serde(rename = "backupInformation", default, skip_serializing_if = "Option::is_none")]
    pub backup_information: Option<sql_server_database_resource_properties::BackupInformation>,
    #[doc = "The backup profile for the SQL server."]
    #[serde(rename = "backupPolicy", default, skip_serializing_if = "Option::is_none")]
    pub backup_policy: Option<BackupPolicy>,
    #[doc = "This records the earliest start date and time that restore is available for this database (ISO8601 format)."]
    #[serde(rename = "earliestRestoreDate", default, with = "azure_core::date::rfc3339::option")]
    pub earliest_restore_date: Option<::time::OffsetDateTime>,
    #[doc = "Database create mode. PointInTimeRestore: Create a database by restoring a point in time backup of an existing database. sourceDatabaseId and restorePointInTime must be specified."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<sql_server_database_resource_properties::CreateMode>,
    #[doc = "The name of the source database associated with create operation of this database."]
    #[serde(rename = "sourceDatabaseId", default, skip_serializing_if = "Option::is_none")]
    pub source_database_id: Option<String>,
    #[doc = "Conditional. If createMode is PointInTimeRestore, this value is required. Specifies the point in time (ISO8601 format) of the source database that will be restored to create the new database."]
    #[serde(rename = "restorePointInTime", default, with = "azure_core::date::rfc3339::option")]
    pub restore_point_in_time: Option<::time::OffsetDateTime>,
    #[doc = "The provisioning state of the Arc-enabled SQL Server database resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The time when last successful database upload was performed."]
    #[serde(rename = "lastDatabaseUploadTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_database_upload_time: Option<::time::OffsetDateTime>,
    #[doc = "Migration related configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration: Option<DataBaseMigration>,
}
impl SqlServerDatabaseResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_server_database_resource_properties {
    use super::*;
    #[doc = "State of the database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Online,
        Restoring,
        Recovering,
        RecoveryPending,
        Suspect,
        Emergency,
        Offline,
        Copying,
        OfflineSecondary,
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
                Self::Online => serializer.serialize_unit_variant("State", 0u32, "Online"),
                Self::Restoring => serializer.serialize_unit_variant("State", 1u32, "Restoring"),
                Self::Recovering => serializer.serialize_unit_variant("State", 2u32, "Recovering"),
                Self::RecoveryPending => serializer.serialize_unit_variant("State", 3u32, "RecoveryPending"),
                Self::Suspect => serializer.serialize_unit_variant("State", 4u32, "Suspect"),
                Self::Emergency => serializer.serialize_unit_variant("State", 5u32, "Emergency"),
                Self::Offline => serializer.serialize_unit_variant("State", 6u32, "Offline"),
                Self::Copying => serializer.serialize_unit_variant("State", 7u32, "Copying"),
                Self::OfflineSecondary => serializer.serialize_unit_variant("State", 8u32, "OfflineSecondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Status of the database."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryMode")]
    pub enum RecoveryMode {
        Full,
        #[serde(rename = "Bulk-logged")]
        BulkLogged,
        Simple,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Full => serializer.serialize_unit_variant("RecoveryMode", 0u32, "Full"),
                Self::BulkLogged => serializer.serialize_unit_variant("RecoveryMode", 1u32, "Bulk-logged"),
                Self::Simple => serializer.serialize_unit_variant("RecoveryMode", 2u32, "Simple"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "List of features that are enabled for the database"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DatabaseOptions {
        #[serde(rename = "isAutoCloseOn", default, skip_serializing_if = "Option::is_none")]
        pub is_auto_close_on: Option<bool>,
        #[serde(rename = "isAutoShrinkOn", default, skip_serializing_if = "Option::is_none")]
        pub is_auto_shrink_on: Option<bool>,
        #[serde(rename = "isAutoCreateStatsOn", default, skip_serializing_if = "Option::is_none")]
        pub is_auto_create_stats_on: Option<bool>,
        #[serde(rename = "isAutoUpdateStatsOn", default, skip_serializing_if = "Option::is_none")]
        pub is_auto_update_stats_on: Option<bool>,
        #[serde(rename = "isRemoteDataArchiveEnabled", default, skip_serializing_if = "Option::is_none")]
        pub is_remote_data_archive_enabled: Option<bool>,
        #[serde(rename = "isMemoryOptimizationEnabled", default, skip_serializing_if = "Option::is_none")]
        pub is_memory_optimization_enabled: Option<bool>,
        #[serde(rename = "isEncrypted", default, skip_serializing_if = "Option::is_none")]
        pub is_encrypted: Option<bool>,
        #[serde(rename = "isTrustworthyOn", default, skip_serializing_if = "Option::is_none")]
        pub is_trustworthy_on: Option<bool>,
    }
    impl DatabaseOptions {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct BackupInformation {
        #[doc = "Date time of last full backup."]
        #[serde(rename = "lastFullBackup", default, with = "azure_core::date::rfc3339::option")]
        pub last_full_backup: Option<::time::OffsetDateTime>,
        #[doc = "Date time of last log backup."]
        #[serde(rename = "lastLogBackup", default, with = "azure_core::date::rfc3339::option")]
        pub last_log_backup: Option<::time::OffsetDateTime>,
    }
    impl BackupInformation {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Database create mode. PointInTimeRestore: Create a database by restoring a point in time backup of an existing database. sourceDatabaseId and restorePointInTime must be specified."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateMode")]
    pub enum CreateMode {
        Default,
        PointInTimeRestore,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreateMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreateMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreateMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("CreateMode", 0u32, "Default"),
                Self::PointInTimeRestore => serializer.serialize_unit_variant("CreateMode", 1u32, "PointInTimeRestore"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An update to database resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerDatabaseUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The properties of Arc Sql Server database resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlServerDatabaseResourceProperties>,
}
impl SqlServerDatabaseUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describe SQL Server ESU license resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerEsuLicense {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of SQL Server ESU license."]
    pub properties: SqlServerEsuLicenseProperties,
}
impl SqlServerEsuLicense {
    pub fn new(tracked_resource: TrackedResource, properties: SqlServerEsuLicenseProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "A list of SQL Server ESU licenses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerEsuLicenseListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SqlServerEsuLicense>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlServerEsuLicenseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlServerEsuLicenseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of SQL Server ESU license."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerEsuLicenseProperties {
    #[doc = "SQL Server ESU license type."]
    #[serde(rename = "billingPlan")]
    pub billing_plan: sql_server_esu_license_properties::BillingPlan,
    #[doc = "The SQL Server version the license covers."]
    pub version: sql_server_esu_license_properties::Version,
    #[doc = "The unique ID of this license. This is a GUID-formatted string (e.g. 00000000-0000-0000-0000-000000000000)."]
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[doc = "The number of total cores of the license covers."]
    #[serde(rename = "physicalCores")]
    pub physical_cores: i32,
    #[doc = "The activation state of the license."]
    #[serde(rename = "activationState")]
    pub activation_state: sql_server_esu_license_properties::ActivationState,
    #[doc = "The Azure scope to which the license will apply."]
    #[serde(rename = "scopeType")]
    pub scope_type: sql_server_esu_license_properties::ScopeType,
    #[doc = "The timestamp of the activation of the SqlServerEsuLicense in ISO 8601 date-time format."]
    #[serde(rename = "activatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub activated_at: Option<::time::OffsetDateTime>,
    #[doc = "The timestamp of the termination of the SqlServerEsuLicense in ISO 8601 date-time format."]
    #[serde(rename = "terminatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub terminated_at: Option<::time::OffsetDateTime>,
    #[doc = "The tenantId the SQL Server ESU license resource subscription resides in."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl SqlServerEsuLicenseProperties {
    pub fn new(
        billing_plan: sql_server_esu_license_properties::BillingPlan,
        version: sql_server_esu_license_properties::Version,
        physical_cores: i32,
        activation_state: sql_server_esu_license_properties::ActivationState,
        scope_type: sql_server_esu_license_properties::ScopeType,
    ) -> Self {
        Self {
            billing_plan,
            version,
            unique_id: None,
            physical_cores,
            activation_state,
            scope_type,
            activated_at: None,
            terminated_at: None,
            tenant_id: None,
        }
    }
}
pub mod sql_server_esu_license_properties {
    use super::*;
    #[doc = "SQL Server ESU license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingPlan")]
    pub enum BillingPlan {
        #[serde(rename = "PAYG")]
        Payg,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingPlan {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingPlan {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingPlan {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Payg => serializer.serialize_unit_variant("BillingPlan", 0u32, "PAYG"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The SQL Server version the license covers."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Version")]
    pub enum Version {
        #[serde(rename = "SQL Server 2012")]
        SqlServer2012,
        #[serde(rename = "SQL Server 2014")]
        SqlServer2014,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Version {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Version {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Version {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SqlServer2012 => serializer.serialize_unit_variant("Version", 0u32, "SQL Server 2012"),
                Self::SqlServer2014 => serializer.serialize_unit_variant("Version", 1u32, "SQL Server 2014"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The activation state of the license."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActivationState")]
    pub enum ActivationState {
        Inactive,
        Active,
        Terminated,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActivationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActivationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActivationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inactive => serializer.serialize_unit_variant("ActivationState", 0u32, "Inactive"),
                Self::Active => serializer.serialize_unit_variant("ActivationState", 1u32, "Active"),
                Self::Terminated => serializer.serialize_unit_variant("ActivationState", 2u32, "Terminated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The Azure scope to which the license will apply."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScopeType")]
    pub enum ScopeType {
        Tenant,
        Subscription,
        ResourceGroup,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScopeType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScopeType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScopeType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tenant => serializer.serialize_unit_variant("ScopeType", 0u32, "Tenant"),
                Self::Subscription => serializer.serialize_unit_variant("ScopeType", 1u32, "Subscription"),
                Self::ResourceGroup => serializer.serialize_unit_variant("ScopeType", 2u32, "ResourceGroup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An update to a SQL Server ESU license resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerEsuLicenseUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of update SqlServerEsuLicense."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlServerEsuLicenseUpdateProperties>,
}
impl SqlServerEsuLicenseUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of update SqlServerEsuLicense."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerEsuLicenseUpdateProperties {
    #[doc = "SQL Server ESU license type."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<sql_server_esu_license_update_properties::BillingPlan>,
    #[doc = "The SQL Server version the license covers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<sql_server_esu_license_update_properties::Version>,
    #[doc = "The unique ID of this license. This is a GUID-formatted string (e.g. 00000000-0000-0000-0000-000000000000)."]
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[doc = "The number of total cores of the license covers."]
    #[serde(rename = "physicalCores", default, skip_serializing_if = "Option::is_none")]
    pub physical_cores: Option<i32>,
    #[doc = "The activation state of the license."]
    #[serde(rename = "activationState", default, skip_serializing_if = "Option::is_none")]
    pub activation_state: Option<sql_server_esu_license_update_properties::ActivationState>,
    #[doc = "The Azure scope to which the license will apply."]
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<sql_server_esu_license_update_properties::ScopeType>,
    #[doc = "The timestamp of the activation of the SqlServerEsuLicense in ISO 8601 date-time format."]
    #[serde(rename = "activatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub activated_at: Option<::time::OffsetDateTime>,
    #[doc = "The timestamp of the termination of the SqlServerEsuLicense in ISO 8601 date-time format."]
    #[serde(rename = "terminatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub terminated_at: Option<::time::OffsetDateTime>,
    #[doc = "The tenantId the SQL Server ESU license resource subscription resides in."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl SqlServerEsuLicenseUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_server_esu_license_update_properties {
    use super::*;
    #[doc = "SQL Server ESU license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingPlan")]
    pub enum BillingPlan {
        #[serde(rename = "PAYG")]
        Payg,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingPlan {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingPlan {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingPlan {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Payg => serializer.serialize_unit_variant("BillingPlan", 0u32, "PAYG"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The SQL Server version the license covers."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Version")]
    pub enum Version {
        #[serde(rename = "SQL Server 2012")]
        SqlServer2012,
        #[serde(rename = "SQL Server 2014")]
        SqlServer2014,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Version {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Version {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Version {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SqlServer2012 => serializer.serialize_unit_variant("Version", 0u32, "SQL Server 2012"),
                Self::SqlServer2014 => serializer.serialize_unit_variant("Version", 1u32, "SQL Server 2014"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The activation state of the license."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActivationState")]
    pub enum ActivationState {
        Inactive,
        Active,
        Terminated,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActivationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActivationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActivationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inactive => serializer.serialize_unit_variant("ActivationState", 0u32, "Inactive"),
                Self::Active => serializer.serialize_unit_variant("ActivationState", 1u32, "Active"),
                Self::Terminated => serializer.serialize_unit_variant("ActivationState", 2u32, "Terminated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The Azure scope to which the license will apply."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScopeType")]
    pub enum ScopeType {
        Tenant,
        Subscription,
        ResourceGroup,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScopeType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScopeType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScopeType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tenant => serializer.serialize_unit_variant("ScopeType", 0u32, "Tenant"),
                Self::Subscription => serializer.serialize_unit_variant("ScopeType", 1u32, "Subscription"),
                Self::ResourceGroup => serializer.serialize_unit_variant("ScopeType", 2u32, "ResourceGroup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A SqlServerInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of SqlServerInstance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlServerInstanceProperties>,
}
impl SqlServerInstance {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The status of the job running on the SQL Server instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerInstanceJobStatus {
    #[doc = "The unique identifier of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the SQL Server instance."]
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[doc = "The status of the job."]
    #[serde(rename = "jobStatus", default, skip_serializing_if = "Option::is_none")]
    pub job_status: Option<sql_server_instance_job_status::JobStatus>,
    #[doc = "The exception message if the job failed."]
    #[serde(rename = "jobException", default, skip_serializing_if = "Option::is_none")]
    pub job_exception: Option<String>,
    #[doc = "The background job details."]
    #[serde(rename = "backgroundJob", default, skip_serializing_if = "Option::is_none")]
    pub background_job: Option<BackgroundJob>,
    #[doc = "The list of sequencer actions."]
    #[serde(
        rename = "sequencerActions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sequencer_actions: Vec<SequencerAction>,
}
impl SqlServerInstanceJobStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_server_instance_job_status {
    use super::*;
    #[doc = "The status of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "JobStatus")]
    pub enum JobStatus {
        NotStarted,
        InProgress,
        Succeeded,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for JobStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for JobStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for JobStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotStarted => serializer.serialize_unit_variant("JobStatus", 0u32, "NotStarted"),
                Self::InProgress => serializer.serialize_unit_variant("JobStatus", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("JobStatus", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("JobStatus", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The request for the status of the jobs running on the SQL Server instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerInstanceJobsStatusRequest {
    #[doc = "The name of the feature to retrieve the job status for."]
    #[serde(rename = "featureName", default, skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    #[doc = "The type of the job to retrieve the status for."]
    #[serde(rename = "jobType", default, skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
}
impl SqlServerInstanceJobsStatusRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response for the status of the jobs running on the SQL Server instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerInstanceJobsStatusResponse {
    #[doc = "The list of jobs status running on the SQL Server instance."]
    #[serde(
        rename = "jobsStatus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub jobs_status: Vec<SqlServerInstanceJobStatus>,
}
impl SqlServerInstanceJobsStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of SqlServerInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerInstanceListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SqlServerInstance>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlServerInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlServerInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of SqlServerInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerInstanceProperties {
    #[doc = "SQL Server version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<sql_server_instance_properties::Version>,
    #[doc = "SQL Server edition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<sql_server_instance_properties::Edition>,
    #[doc = "ARM Resource id of the container resource (Azure Arc for Servers)."]
    #[serde(rename = "containerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub container_resource_id: Option<String>,
    #[doc = "The time when the resource was created."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[doc = "The number of logical processors used by the SQL Server instance."]
    #[serde(rename = "vCore", default, skip_serializing_if = "Option::is_none")]
    pub v_core: Option<String>,
    #[doc = "The number of total cores of the Operating System Environment (OSE) hosting the SQL Server instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cores: Option<String>,
    #[doc = "The cloud connectivity status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<sql_server_instance_properties::Status>,
    #[doc = "SQL Server update level."]
    #[serde(rename = "patchLevel", default, skip_serializing_if = "Option::is_none")]
    pub patch_level: Option<String>,
    #[doc = "SQL Server collation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    #[doc = "Indicates whether database master key exists in SQL Server."]
    #[serde(rename = "dbMasterKeyExists", default, skip_serializing_if = "Option::is_none")]
    pub db_master_key_exists: Option<bool>,
    #[doc = "Indicates whether always On availability groups is enabled in SQL Server."]
    #[serde(rename = "isHadrEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_hadr_enabled: Option<bool>,
    #[doc = "An array of integers, where each value represents the enabled trace flags in SQL Server."]
    #[serde(
        rename = "traceFlags",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trace_flags: Vec<i32>,
    #[doc = "SQL Server current version."]
    #[serde(rename = "currentVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[doc = "SQL Server instance name."]
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[doc = "Dynamic TCP ports used by SQL Server."]
    #[serde(rename = "tcpDynamicPorts", default, skip_serializing_if = "Option::is_none")]
    pub tcp_dynamic_ports: Option<String>,
    #[doc = "Static TCP ports used by SQL Server."]
    #[serde(rename = "tcpStaticPorts", default, skip_serializing_if = "Option::is_none")]
    pub tcp_static_ports: Option<String>,
    #[doc = "SQL Server product ID."]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[doc = "SQL Server license type."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<sql_server_instance_properties::LicenseType>,
    #[doc = "Timestamp of last Azure Defender status update."]
    #[serde(rename = "azureDefenderStatusLastUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub azure_defender_status_last_updated: Option<::time::OffsetDateTime>,
    #[doc = "Status of Azure Defender."]
    #[serde(rename = "azureDefenderStatus", default, skip_serializing_if = "Option::is_none")]
    pub azure_defender_status: Option<sql_server_instance_properties::AzureDefenderStatus>,
    #[doc = "The provisioning state of the Arc-enabled SQL Server resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The time when last successful inventory upload was performed."]
    #[serde(rename = "lastInventoryUploadTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_inventory_upload_time: Option<::time::OffsetDateTime>,
    #[doc = "The time when last successful usage upload was performed."]
    #[serde(rename = "lastUsageUploadTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_usage_upload_time: Option<::time::OffsetDateTime>,
    #[doc = "Type of host for Azure Arc SQL Server"]
    #[serde(rename = "hostType", default, skip_serializing_if = "Option::is_none")]
    pub host_type: Option<sql_server_instance_properties::HostType>,
    #[doc = "The role of the SQL Server, based on availability."]
    #[serde(rename = "alwaysOnRole", default, skip_serializing_if = "Option::is_none")]
    pub always_on_role: Option<sql_server_instance_properties::AlwaysOnRole>,
    #[doc = "Database mirroring endpoint related properties."]
    #[serde(rename = "databaseMirroringEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub database_mirroring_endpoint: Option<DbmEndpoint>,
    #[doc = "Failover Cluster Instance properties."]
    #[serde(rename = "failoverCluster", default, skip_serializing_if = "Option::is_none")]
    pub failover_cluster: Option<FailoverCluster>,
    #[doc = "The backup profile for the SQL server."]
    #[serde(rename = "backupPolicy", default, skip_serializing_if = "Option::is_none")]
    pub backup_policy: Option<BackupPolicy>,
    #[doc = "Upgrade Action for this resource is locked until it expires. The Expiration time indicated by this value. It is not locked when it is empty."]
    #[serde(rename = "upgradeLockedUntil", default, with = "azure_core::date::rfc3339::option")]
    pub upgrade_locked_until: Option<::time::OffsetDateTime>,
    #[doc = "The monitoring configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<Monitoring>,
    #[doc = "Migration related configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration: Option<Migration>,
    #[doc = "Client connection related configuration."]
    #[serde(rename = "clientConnection", default, skip_serializing_if = "Option::is_none")]
    pub client_connection: Option<ClientConnection>,
    #[doc = "Indicates if the resource represents a SQL Server engine or a SQL Server component service installed on the host."]
    #[serde(rename = "serviceType", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<ServiceType>,
}
impl SqlServerInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_server_instance_properties {
    use super::*;
    #[doc = "SQL Server version."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Version")]
    pub enum Version {
        #[serde(rename = "SQL Server 2012")]
        SqlServer2012,
        #[serde(rename = "SQL Server 2014")]
        SqlServer2014,
        #[serde(rename = "SQL Server 2016")]
        SqlServer2016,
        #[serde(rename = "SQL Server 2017")]
        SqlServer2017,
        #[serde(rename = "SQL Server 2019")]
        SqlServer2019,
        #[serde(rename = "SQL Server 2022")]
        SqlServer2022,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Version {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Version {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Version {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SqlServer2012 => serializer.serialize_unit_variant("Version", 0u32, "SQL Server 2012"),
                Self::SqlServer2014 => serializer.serialize_unit_variant("Version", 1u32, "SQL Server 2014"),
                Self::SqlServer2016 => serializer.serialize_unit_variant("Version", 2u32, "SQL Server 2016"),
                Self::SqlServer2017 => serializer.serialize_unit_variant("Version", 3u32, "SQL Server 2017"),
                Self::SqlServer2019 => serializer.serialize_unit_variant("Version", 4u32, "SQL Server 2019"),
                Self::SqlServer2022 => serializer.serialize_unit_variant("Version", 5u32, "SQL Server 2022"),
                Self::Unknown => serializer.serialize_unit_variant("Version", 6u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "SQL Server edition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Edition")]
    pub enum Edition {
        Evaluation,
        Enterprise,
        Standard,
        Web,
        Developer,
        Express,
        #[serde(rename = "Business Intelligence")]
        BusinessIntelligence,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Edition {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Edition {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Edition {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Evaluation => serializer.serialize_unit_variant("Edition", 0u32, "Evaluation"),
                Self::Enterprise => serializer.serialize_unit_variant("Edition", 1u32, "Enterprise"),
                Self::Standard => serializer.serialize_unit_variant("Edition", 2u32, "Standard"),
                Self::Web => serializer.serialize_unit_variant("Edition", 3u32, "Web"),
                Self::Developer => serializer.serialize_unit_variant("Edition", 4u32, "Developer"),
                Self::Express => serializer.serialize_unit_variant("Edition", 5u32, "Express"),
                Self::BusinessIntelligence => serializer.serialize_unit_variant("Edition", 6u32, "Business Intelligence"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The cloud connectivity status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Connected,
        Disconnected,
        Registered,
        Unknown,
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
                Self::Connected => serializer.serialize_unit_variant("Status", 0u32, "Connected"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 1u32, "Disconnected"),
                Self::Registered => serializer.serialize_unit_variant("Status", 2u32, "Registered"),
                Self::Unknown => serializer.serialize_unit_variant("Status", 3u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "SQL Server license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        Undefined,
        Free,
        #[serde(rename = "HADR")]
        Hadr,
        #[serde(rename = "ServerCAL")]
        ServerCal,
        LicenseOnly,
        #[serde(rename = "PAYG")]
        Payg,
        Paid,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Undefined => serializer.serialize_unit_variant("LicenseType", 0u32, "Undefined"),
                Self::Free => serializer.serialize_unit_variant("LicenseType", 1u32, "Free"),
                Self::Hadr => serializer.serialize_unit_variant("LicenseType", 2u32, "HADR"),
                Self::ServerCal => serializer.serialize_unit_variant("LicenseType", 3u32, "ServerCAL"),
                Self::LicenseOnly => serializer.serialize_unit_variant("LicenseType", 4u32, "LicenseOnly"),
                Self::Payg => serializer.serialize_unit_variant("LicenseType", 5u32, "PAYG"),
                Self::Paid => serializer.serialize_unit_variant("LicenseType", 6u32, "Paid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Status of Azure Defender."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AzureDefenderStatus")]
    pub enum AzureDefenderStatus {
        Protected,
        Unprotected,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AzureDefenderStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AzureDefenderStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AzureDefenderStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Protected => serializer.serialize_unit_variant("AzureDefenderStatus", 0u32, "Protected"),
                Self::Unprotected => serializer.serialize_unit_variant("AzureDefenderStatus", 1u32, "Unprotected"),
                Self::Unknown => serializer.serialize_unit_variant("AzureDefenderStatus", 2u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of host for Azure Arc SQL Server"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HostType")]
    pub enum HostType {
        #[serde(rename = "Azure Virtual Machine")]
        AzureVirtualMachine,
        #[serde(rename = "Azure VMWare Virtual Machine")]
        AzureVmWareVirtualMachine,
        #[serde(rename = "Azure Kubernetes Service")]
        AzureKubernetesService,
        #[serde(rename = "AWS VMWare Virtual Machine")]
        AwsVmWareVirtualMachine,
        #[serde(rename = "AWS Kubernetes Service")]
        AwsKubernetesService,
        #[serde(rename = "GCP VMWare Virtual Machine")]
        GcpVmWareVirtualMachine,
        #[serde(rename = "GCP Kubernetes Service")]
        GcpKubernetesService,
        Container,
        #[serde(rename = "Virtual Machine")]
        VirtualMachine,
        #[serde(rename = "Physical Server")]
        PhysicalServer,
        #[serde(rename = "AWS Virtual Machine")]
        AwsVirtualMachine,
        #[serde(rename = "GCP Virtual Machine")]
        GcpVirtualMachine,
        Other,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HostType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HostType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HostType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureVirtualMachine => serializer.serialize_unit_variant("HostType", 0u32, "Azure Virtual Machine"),
                Self::AzureVmWareVirtualMachine => serializer.serialize_unit_variant("HostType", 1u32, "Azure VMWare Virtual Machine"),
                Self::AzureKubernetesService => serializer.serialize_unit_variant("HostType", 2u32, "Azure Kubernetes Service"),
                Self::AwsVmWareVirtualMachine => serializer.serialize_unit_variant("HostType", 3u32, "AWS VMWare Virtual Machine"),
                Self::AwsKubernetesService => serializer.serialize_unit_variant("HostType", 4u32, "AWS Kubernetes Service"),
                Self::GcpVmWareVirtualMachine => serializer.serialize_unit_variant("HostType", 5u32, "GCP VMWare Virtual Machine"),
                Self::GcpKubernetesService => serializer.serialize_unit_variant("HostType", 6u32, "GCP Kubernetes Service"),
                Self::Container => serializer.serialize_unit_variant("HostType", 7u32, "Container"),
                Self::VirtualMachine => serializer.serialize_unit_variant("HostType", 8u32, "Virtual Machine"),
                Self::PhysicalServer => serializer.serialize_unit_variant("HostType", 9u32, "Physical Server"),
                Self::AwsVirtualMachine => serializer.serialize_unit_variant("HostType", 10u32, "AWS Virtual Machine"),
                Self::GcpVirtualMachine => serializer.serialize_unit_variant("HostType", 11u32, "GCP Virtual Machine"),
                Self::Other => serializer.serialize_unit_variant("HostType", 12u32, "Other"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The role of the SQL Server, based on availability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AlwaysOnRole")]
    pub enum AlwaysOnRole {
        None,
        FailoverClusterInstance,
        FailoverClusterNode,
        AvailabilityGroupReplica,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AlwaysOnRole {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AlwaysOnRole {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AlwaysOnRole {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("AlwaysOnRole", 0u32, "None"),
                Self::FailoverClusterInstance => serializer.serialize_unit_variant("AlwaysOnRole", 1u32, "FailoverClusterInstance"),
                Self::FailoverClusterNode => serializer.serialize_unit_variant("AlwaysOnRole", 2u32, "FailoverClusterNode"),
                Self::AvailabilityGroupReplica => serializer.serialize_unit_variant("AlwaysOnRole", 3u32, "AvailabilityGroupReplica"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response for running migration assessment on the SQL Server instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerInstanceRunMigrationAssessmentResponse {
    #[doc = "The unique identifier of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the SQL Server instance."]
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[doc = "The status of the job."]
    #[serde(rename = "jobStatus", default, skip_serializing_if = "Option::is_none")]
    pub job_status: Option<sql_server_instance_run_migration_assessment_response::JobStatus>,
    #[doc = "The exception message if the job failed."]
    #[serde(rename = "jobException", default, skip_serializing_if = "Option::is_none")]
    pub job_exception: Option<String>,
    #[doc = "The background job details."]
    #[serde(rename = "backgroundJob", default, skip_serializing_if = "Option::is_none")]
    pub background_job: Option<BackgroundJob>,
    #[doc = "The list of sequencer actions."]
    #[serde(
        rename = "sequencerActions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sequencer_actions: Vec<SequencerAction>,
}
impl SqlServerInstanceRunMigrationAssessmentResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_server_instance_run_migration_assessment_response {
    use super::*;
    #[doc = "The status of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "JobStatus")]
    pub enum JobStatus {
        NotStarted,
        InProgress,
        Succeeded,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for JobStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for JobStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for JobStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotStarted => serializer.serialize_unit_variant("JobStatus", 0u32, "NotStarted"),
                Self::InProgress => serializer.serialize_unit_variant("JobStatus", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("JobStatus", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("JobStatus", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The telemetry column for the SQL Server instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerInstanceTelemetryColumn {
    #[doc = "The name of the telemetry column."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the telemetry column."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<sql_server_instance_telemetry_column::Type>,
}
impl SqlServerInstanceTelemetryColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_server_instance_telemetry_column {
    use super::*;
    #[doc = "The type of the telemetry column."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "bool")]
        Bool,
        #[serde(rename = "datetime")]
        Datetime,
        #[serde(rename = "int")]
        Int,
        #[serde(rename = "long")]
        Long,
        #[serde(rename = "double")]
        Double,
        #[serde(rename = "string")]
        String,
        #[serde(rename = "guid")]
        Guid,
        #[serde(rename = "timespan")]
        Timespan,
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
                Self::Bool => serializer.serialize_unit_variant("Type", 0u32, "bool"),
                Self::Datetime => serializer.serialize_unit_variant("Type", 1u32, "datetime"),
                Self::Int => serializer.serialize_unit_variant("Type", 2u32, "int"),
                Self::Long => serializer.serialize_unit_variant("Type", 3u32, "long"),
                Self::Double => serializer.serialize_unit_variant("Type", 4u32, "double"),
                Self::String => serializer.serialize_unit_variant("Type", 5u32, "string"),
                Self::Guid => serializer.serialize_unit_variant("Type", 6u32, "guid"),
                Self::Timespan => serializer.serialize_unit_variant("Type", 7u32, "timespan"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Arc SQL Server instance telemetry retrieval request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerInstanceTelemetryRequest {
    #[doc = "The name of the telemetry dataset to retrieve."]
    #[serde(rename = "datasetName")]
    pub dataset_name: String,
    #[doc = "The start time for the time range to fetch telemetry for. If not specified, the current time minus 1 hour is used."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "The end time for the time range to fetch telemetry for. If not specified, the current time is used."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
    #[doc = "The time granularity to fetch telemetry for. This is an ISO8601 duration. Examples: PT15M, PT1H, P1D"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[doc = "The aggregation type to use for the numerical columns in the dataset."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<sql_server_instance_telemetry_request::AggregationType>,
    #[doc = "The list of database names to return telemetry for. If not specified, telemetry for all databases will be aggregated and returned."]
    #[serde(
        rename = "databaseNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub database_names: Vec<String>,
}
impl SqlServerInstanceTelemetryRequest {
    pub fn new(dataset_name: String) -> Self {
        Self {
            dataset_name,
            start_time: None,
            end_time: None,
            interval: None,
            aggregation_type: None,
            database_names: Vec::new(),
        }
    }
}
pub mod sql_server_instance_telemetry_request {
    use super::*;
    #[doc = "The aggregation type to use for the numerical columns in the dataset."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AggregationType")]
    pub enum AggregationType {
        Average,
        Minimum,
        Maximum,
        Sum,
        Count,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AggregationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AggregationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AggregationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Average => serializer.serialize_unit_variant("AggregationType", 0u32, "Average"),
                Self::Minimum => serializer.serialize_unit_variant("AggregationType", 1u32, "Minimum"),
                Self::Maximum => serializer.serialize_unit_variant("AggregationType", 2u32, "Maximum"),
                Self::Sum => serializer.serialize_unit_variant("AggregationType", 3u32, "Sum"),
                Self::Count => serializer.serialize_unit_variant("AggregationType", 4u32, "Count"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for AggregationType {
        fn default() -> Self {
            Self::Average
        }
    }
}
#[doc = "A section of the telemetry response for the SQL Server instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerInstanceTelemetryResponse {
    #[doc = "The columns of the result telemetry table for the SQL Server instance."]
    pub columns: Vec<SqlServerInstanceTelemetryColumn>,
    #[doc = "A list of rows from the result telemetry table for the SQL Server instance."]
    pub rows: Vec<SqlServerInstanceTelemetryRow>,
    #[doc = "The link to the next section of rows of the telemetry response for the SQL Server instance. Null if no more sections are available."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlServerInstanceTelemetryResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlServerInstanceTelemetryResponse {
    pub fn new(columns: Vec<SqlServerInstanceTelemetryColumn>, rows: Vec<SqlServerInstanceTelemetryRow>) -> Self {
        Self {
            columns,
            rows,
            next_link: None,
        }
    }
}
pub type SqlServerInstanceTelemetryRow = Vec<String>;
#[doc = "An update to a SQL Server Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerInstanceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of update SqlServerInstance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlServerInstanceUpdateProperties>,
}
impl SqlServerInstanceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of update SqlServerInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerInstanceUpdateProperties {
    #[doc = "SQL Server version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<sql_server_instance_update_properties::Version>,
    #[doc = "SQL Server edition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<sql_server_instance_update_properties::Edition>,
    #[doc = "ARM Resource id of the container resource (Azure Arc for Servers)."]
    #[serde(rename = "containerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub container_resource_id: Option<String>,
    #[doc = "The time when the resource was created."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[doc = "The number of logical processors used by the SQL Server instance."]
    #[serde(rename = "vCore", default, skip_serializing_if = "Option::is_none")]
    pub v_core: Option<String>,
    #[doc = "The number of total cores of the Operating System Environment (OSE) hosting the SQL Server instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cores: Option<String>,
    #[doc = "The cloud connectivity status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<sql_server_instance_update_properties::Status>,
    #[doc = "SQL Server update level."]
    #[serde(rename = "patchLevel", default, skip_serializing_if = "Option::is_none")]
    pub patch_level: Option<String>,
    #[doc = "SQL Server collation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    #[doc = "Indicates whether database master key exists in SQL Server."]
    #[serde(rename = "dbMasterKeyExists", default, skip_serializing_if = "Option::is_none")]
    pub db_master_key_exists: Option<bool>,
    #[doc = "Indicates whether always On availability groups is enabled in SQL Server."]
    #[serde(rename = "isHadrEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_hadr_enabled: Option<bool>,
    #[doc = "An array of integers, where each value represents the enabled trace flags in SQL Server."]
    #[serde(
        rename = "traceFlags",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trace_flags: Vec<i32>,
    #[doc = "SQL Server current version."]
    #[serde(rename = "currentVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[doc = "SQL Server instance name."]
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[doc = "Dynamic TCP ports used by SQL Server."]
    #[serde(rename = "tcpDynamicPorts", default, skip_serializing_if = "Option::is_none")]
    pub tcp_dynamic_ports: Option<String>,
    #[doc = "Static TCP ports used by SQL Server."]
    #[serde(rename = "tcpStaticPorts", default, skip_serializing_if = "Option::is_none")]
    pub tcp_static_ports: Option<String>,
    #[doc = "SQL Server product ID."]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[doc = "SQL Server license type."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<sql_server_instance_update_properties::LicenseType>,
    #[doc = "Timestamp of last Azure Defender status update."]
    #[serde(rename = "azureDefenderStatusLastUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub azure_defender_status_last_updated: Option<::time::OffsetDateTime>,
    #[doc = "Status of Azure Defender."]
    #[serde(rename = "azureDefenderStatus", default, skip_serializing_if = "Option::is_none")]
    pub azure_defender_status: Option<sql_server_instance_update_properties::AzureDefenderStatus>,
    #[doc = "The provisioning state of the Arc-enabled SQL Server resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The time when last successful inventory upload was performed."]
    #[serde(rename = "lastInventoryUploadTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_inventory_upload_time: Option<::time::OffsetDateTime>,
    #[doc = "The time when last successful usage upload was performed."]
    #[serde(rename = "lastUsageUploadTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_usage_upload_time: Option<::time::OffsetDateTime>,
    #[doc = "Type of host for Azure Arc SQL Server"]
    #[serde(rename = "hostType", default, skip_serializing_if = "Option::is_none")]
    pub host_type: Option<sql_server_instance_update_properties::HostType>,
    #[doc = "The role of the SQL Server, based on availability."]
    #[serde(rename = "alwaysOnRole", default, skip_serializing_if = "Option::is_none")]
    pub always_on_role: Option<sql_server_instance_update_properties::AlwaysOnRole>,
    #[doc = "Failover Cluster Instance properties."]
    #[serde(rename = "failoverCluster", default, skip_serializing_if = "Option::is_none")]
    pub failover_cluster: Option<FailoverCluster>,
    #[doc = "The backup profile for the SQL server."]
    #[serde(rename = "backupPolicy", default, skip_serializing_if = "Option::is_none")]
    pub backup_policy: Option<BackupPolicy>,
    #[doc = "Upgrade Action for this resource is locked until it expires. The Expiration time indicated by this value. It is not locked when it is empty."]
    #[serde(rename = "upgradeLockedUntil", default, with = "azure_core::date::rfc3339::option")]
    pub upgrade_locked_until: Option<::time::OffsetDateTime>,
    #[doc = "The monitoring configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<Monitoring>,
    #[doc = "Migration related configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration: Option<Migration>,
    #[doc = "Client connection related configuration."]
    #[serde(rename = "clientConnection", default, skip_serializing_if = "Option::is_none")]
    pub client_connection: Option<ClientConnection>,
    #[doc = "Indicates if the resource represents a SQL Server engine or a SQL Server component service installed on the host."]
    #[serde(rename = "serviceType", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<ServiceType>,
}
impl SqlServerInstanceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_server_instance_update_properties {
    use super::*;
    #[doc = "SQL Server version."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Version")]
    pub enum Version {
        #[serde(rename = "SQL Server 2012")]
        SqlServer2012,
        #[serde(rename = "SQL Server 2014")]
        SqlServer2014,
        #[serde(rename = "SQL Server 2016")]
        SqlServer2016,
        #[serde(rename = "SQL Server 2017")]
        SqlServer2017,
        #[serde(rename = "SQL Server 2019")]
        SqlServer2019,
        #[serde(rename = "SQL Server 2022")]
        SqlServer2022,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Version {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Version {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Version {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SqlServer2012 => serializer.serialize_unit_variant("Version", 0u32, "SQL Server 2012"),
                Self::SqlServer2014 => serializer.serialize_unit_variant("Version", 1u32, "SQL Server 2014"),
                Self::SqlServer2016 => serializer.serialize_unit_variant("Version", 2u32, "SQL Server 2016"),
                Self::SqlServer2017 => serializer.serialize_unit_variant("Version", 3u32, "SQL Server 2017"),
                Self::SqlServer2019 => serializer.serialize_unit_variant("Version", 4u32, "SQL Server 2019"),
                Self::SqlServer2022 => serializer.serialize_unit_variant("Version", 5u32, "SQL Server 2022"),
                Self::Unknown => serializer.serialize_unit_variant("Version", 6u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "SQL Server edition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Edition")]
    pub enum Edition {
        Evaluation,
        Enterprise,
        Standard,
        Web,
        Developer,
        Express,
        #[serde(rename = "Business Intelligence")]
        BusinessIntelligence,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Edition {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Edition {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Edition {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Evaluation => serializer.serialize_unit_variant("Edition", 0u32, "Evaluation"),
                Self::Enterprise => serializer.serialize_unit_variant("Edition", 1u32, "Enterprise"),
                Self::Standard => serializer.serialize_unit_variant("Edition", 2u32, "Standard"),
                Self::Web => serializer.serialize_unit_variant("Edition", 3u32, "Web"),
                Self::Developer => serializer.serialize_unit_variant("Edition", 4u32, "Developer"),
                Self::Express => serializer.serialize_unit_variant("Edition", 5u32, "Express"),
                Self::BusinessIntelligence => serializer.serialize_unit_variant("Edition", 6u32, "Business Intelligence"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The cloud connectivity status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Connected,
        Disconnected,
        Registered,
        Unknown,
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
                Self::Connected => serializer.serialize_unit_variant("Status", 0u32, "Connected"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 1u32, "Disconnected"),
                Self::Registered => serializer.serialize_unit_variant("Status", 2u32, "Registered"),
                Self::Unknown => serializer.serialize_unit_variant("Status", 3u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "SQL Server license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        Undefined,
        Free,
        #[serde(rename = "HADR")]
        Hadr,
        #[serde(rename = "ServerCAL")]
        ServerCal,
        LicenseOnly,
        #[serde(rename = "PAYG")]
        Payg,
        Paid,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Undefined => serializer.serialize_unit_variant("LicenseType", 0u32, "Undefined"),
                Self::Free => serializer.serialize_unit_variant("LicenseType", 1u32, "Free"),
                Self::Hadr => serializer.serialize_unit_variant("LicenseType", 2u32, "HADR"),
                Self::ServerCal => serializer.serialize_unit_variant("LicenseType", 3u32, "ServerCAL"),
                Self::LicenseOnly => serializer.serialize_unit_variant("LicenseType", 4u32, "LicenseOnly"),
                Self::Payg => serializer.serialize_unit_variant("LicenseType", 5u32, "PAYG"),
                Self::Paid => serializer.serialize_unit_variant("LicenseType", 6u32, "Paid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Status of Azure Defender."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AzureDefenderStatus")]
    pub enum AzureDefenderStatus {
        Protected,
        Unprotected,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AzureDefenderStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AzureDefenderStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AzureDefenderStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Protected => serializer.serialize_unit_variant("AzureDefenderStatus", 0u32, "Protected"),
                Self::Unprotected => serializer.serialize_unit_variant("AzureDefenderStatus", 1u32, "Unprotected"),
                Self::Unknown => serializer.serialize_unit_variant("AzureDefenderStatus", 2u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of host for Azure Arc SQL Server"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HostType")]
    pub enum HostType {
        #[serde(rename = "Azure Virtual Machine")]
        AzureVirtualMachine,
        #[serde(rename = "Azure VMWare Virtual Machine")]
        AzureVmWareVirtualMachine,
        #[serde(rename = "Azure Kubernetes Service")]
        AzureKubernetesService,
        #[serde(rename = "AWS VMWare Virtual Machine")]
        AwsVmWareVirtualMachine,
        #[serde(rename = "AWS Kubernetes Service")]
        AwsKubernetesService,
        #[serde(rename = "GCP VMWare Virtual Machine")]
        GcpVmWareVirtualMachine,
        #[serde(rename = "GCP Kubernetes Service")]
        GcpKubernetesService,
        Container,
        #[serde(rename = "Virtual Machine")]
        VirtualMachine,
        #[serde(rename = "Physical Server")]
        PhysicalServer,
        #[serde(rename = "AWS Virtual Machine")]
        AwsVirtualMachine,
        #[serde(rename = "GCP Virtual Machine")]
        GcpVirtualMachine,
        Other,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HostType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HostType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HostType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureVirtualMachine => serializer.serialize_unit_variant("HostType", 0u32, "Azure Virtual Machine"),
                Self::AzureVmWareVirtualMachine => serializer.serialize_unit_variant("HostType", 1u32, "Azure VMWare Virtual Machine"),
                Self::AzureKubernetesService => serializer.serialize_unit_variant("HostType", 2u32, "Azure Kubernetes Service"),
                Self::AwsVmWareVirtualMachine => serializer.serialize_unit_variant("HostType", 3u32, "AWS VMWare Virtual Machine"),
                Self::AwsKubernetesService => serializer.serialize_unit_variant("HostType", 4u32, "AWS Kubernetes Service"),
                Self::GcpVmWareVirtualMachine => serializer.serialize_unit_variant("HostType", 5u32, "GCP VMWare Virtual Machine"),
                Self::GcpKubernetesService => serializer.serialize_unit_variant("HostType", 6u32, "GCP Kubernetes Service"),
                Self::Container => serializer.serialize_unit_variant("HostType", 7u32, "Container"),
                Self::VirtualMachine => serializer.serialize_unit_variant("HostType", 8u32, "Virtual Machine"),
                Self::PhysicalServer => serializer.serialize_unit_variant("HostType", 9u32, "Physical Server"),
                Self::AwsVirtualMachine => serializer.serialize_unit_variant("HostType", 10u32, "AWS Virtual Machine"),
                Self::GcpVirtualMachine => serializer.serialize_unit_variant("HostType", 11u32, "GCP Virtual Machine"),
                Self::Other => serializer.serialize_unit_variant("HostType", 12u32, "Other"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The role of the SQL Server, based on availability."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AlwaysOnRole")]
    pub enum AlwaysOnRole {
        None,
        FailoverClusterInstance,
        FailoverClusterNode,
        AvailabilityGroupReplica,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AlwaysOnRole {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AlwaysOnRole {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AlwaysOnRole {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("AlwaysOnRole", 0u32, "None"),
                Self::FailoverClusterInstance => serializer.serialize_unit_variant("AlwaysOnRole", 1u32, "FailoverClusterInstance"),
                Self::FailoverClusterNode => serializer.serialize_unit_variant("AlwaysOnRole", 2u32, "FailoverClusterNode"),
                Self::AvailabilityGroupReplica => serializer.serialize_unit_variant("AlwaysOnRole", 3u32, "AvailabilityGroupReplica"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describe SQL Server license resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerLicense {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of SQL Server License."]
    pub properties: SqlServerLicenseProperties,
}
impl SqlServerLicense {
    pub fn new(tracked_resource: TrackedResource, properties: SqlServerLicenseProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[doc = "A list of SQL Server licenses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerLicenseListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SqlServerLicense>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SqlServerLicenseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SqlServerLicenseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of SQL Server License."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerLicenseProperties {
    #[doc = "SQL Server license type."]
    #[serde(rename = "billingPlan")]
    pub billing_plan: sql_server_license_properties::BillingPlan,
    #[doc = "The number of total cores of the license covers."]
    #[serde(rename = "physicalCores")]
    pub physical_cores: i32,
    #[doc = "This property represents the choice between SQL Server Core and ESU licenses."]
    #[serde(rename = "licenseCategory")]
    pub license_category: sql_server_license_properties::LicenseCategory,
    #[doc = "The activation state of the license."]
    #[serde(rename = "activationState")]
    pub activation_state: sql_server_license_properties::ActivationState,
    #[doc = "The Azure scope to which the license will apply."]
    #[serde(rename = "scopeType")]
    pub scope_type: sql_server_license_properties::ScopeType,
    #[doc = "The timestamp of the most recent activation of the SqlServerLicense."]
    #[serde(rename = "lastActivatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_activated_at: Option<::time::OffsetDateTime>,
    #[doc = "The timestamp of the most recent deactivation of the SqlServerLicense."]
    #[serde(rename = "lastDeactivatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_deactivated_at: Option<::time::OffsetDateTime>,
    #[doc = "The tenantId the SQL Server license resource subscription resides in."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl SqlServerLicenseProperties {
    pub fn new(
        billing_plan: sql_server_license_properties::BillingPlan,
        physical_cores: i32,
        license_category: sql_server_license_properties::LicenseCategory,
        activation_state: sql_server_license_properties::ActivationState,
        scope_type: sql_server_license_properties::ScopeType,
    ) -> Self {
        Self {
            billing_plan,
            physical_cores,
            license_category,
            activation_state,
            scope_type,
            last_activated_at: None,
            last_deactivated_at: None,
            tenant_id: None,
        }
    }
}
pub mod sql_server_license_properties {
    use super::*;
    #[doc = "SQL Server license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingPlan")]
    pub enum BillingPlan {
        #[serde(rename = "PAYG")]
        Payg,
        Paid,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingPlan {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingPlan {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingPlan {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Payg => serializer.serialize_unit_variant("BillingPlan", 0u32, "PAYG"),
                Self::Paid => serializer.serialize_unit_variant("BillingPlan", 1u32, "Paid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "This property represents the choice between SQL Server Core and ESU licenses."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseCategory")]
    pub enum LicenseCategory {
        Core,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseCategory {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseCategory {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseCategory {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Core => serializer.serialize_unit_variant("LicenseCategory", 0u32, "Core"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The activation state of the license."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActivationState")]
    pub enum ActivationState {
        Activated,
        Deactivated,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActivationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActivationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActivationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Activated => serializer.serialize_unit_variant("ActivationState", 0u32, "Activated"),
                Self::Deactivated => serializer.serialize_unit_variant("ActivationState", 1u32, "Deactivated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The Azure scope to which the license will apply."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScopeType")]
    pub enum ScopeType {
        Tenant,
        Subscription,
        ResourceGroup,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScopeType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScopeType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScopeType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tenant => serializer.serialize_unit_variant("ScopeType", 0u32, "Tenant"),
                Self::Subscription => serializer.serialize_unit_variant("ScopeType", 1u32, "Subscription"),
                Self::ResourceGroup => serializer.serialize_unit_variant("ScopeType", 2u32, "ResourceGroup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An update to a SQL Server license resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerLicenseUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of update SqlServerLicense."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlServerLicenseUpdateProperties>,
}
impl SqlServerLicenseUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of update SqlServerLicense."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlServerLicenseUpdateProperties {
    #[doc = "SQL Server license type."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<sql_server_license_update_properties::BillingPlan>,
    #[doc = "The number of total cores of the license covers."]
    #[serde(rename = "physicalCores", default, skip_serializing_if = "Option::is_none")]
    pub physical_cores: Option<i32>,
    #[doc = "This property represents the choice between SQL Server Core and ESU licenses."]
    #[serde(rename = "licenseCategory", default, skip_serializing_if = "Option::is_none")]
    pub license_category: Option<sql_server_license_update_properties::LicenseCategory>,
    #[doc = "The activation state of the license."]
    #[serde(rename = "activationState", default, skip_serializing_if = "Option::is_none")]
    pub activation_state: Option<sql_server_license_update_properties::ActivationState>,
    #[doc = "The Azure scope to which the license will apply."]
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<sql_server_license_update_properties::ScopeType>,
    #[doc = "The timestamp of the most recent activation of the SqlServerLicense."]
    #[serde(rename = "lastActivatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_activated_at: Option<::time::OffsetDateTime>,
    #[doc = "The timestamp of the most recent deactivation of the SqlServerLicense."]
    #[serde(rename = "lastDeactivatedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_deactivated_at: Option<::time::OffsetDateTime>,
    #[doc = "The tenantId the SQL Server license resource subscription resides in."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl SqlServerLicenseUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_server_license_update_properties {
    use super::*;
    #[doc = "SQL Server license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingPlan")]
    pub enum BillingPlan {
        #[serde(rename = "PAYG")]
        Payg,
        Paid,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingPlan {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingPlan {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingPlan {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Payg => serializer.serialize_unit_variant("BillingPlan", 0u32, "PAYG"),
                Self::Paid => serializer.serialize_unit_variant("BillingPlan", 1u32, "Paid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "This property represents the choice between SQL Server Core and ESU licenses."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseCategory")]
    pub enum LicenseCategory {
        Core,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseCategory {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseCategory {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseCategory {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Core => serializer.serialize_unit_variant("LicenseCategory", 0u32, "Core"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The activation state of the license."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActivationState")]
    pub enum ActivationState {
        Activated,
        Deactivated,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActivationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActivationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActivationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Activated => serializer.serialize_unit_variant("ActivationState", 0u32, "Activated"),
                Self::Deactivated => serializer.serialize_unit_variant("ActivationState", 1u32, "Deactivated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The Azure scope to which the license will apply."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScopeType")]
    pub enum ScopeType {
        Tenant,
        Subscription,
        ResourceGroup,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScopeType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScopeType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScopeType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tenant => serializer.serialize_unit_variant("ScopeType", 0u32, "Tenant"),
                Self::Subscription => serializer.serialize_unit_variant("ScopeType", 1u32, "Subscription"),
                Self::ResourceGroup => serializer.serialize_unit_variant("ScopeType", 2u32, "ResourceGroup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The target readiness for migration for this database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetReadiness {
    #[doc = "The SKU recommendation summary."]
    #[serde(rename = "azureSqlDatabase", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_database: Option<SkuRecommendationSummary>,
    #[doc = "The SKU recommendation summary."]
    #[serde(rename = "azureSqlManagedInstance", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_managed_instance: Option<SkuRecommendationSummary>,
    #[doc = "The SKU recommendation summary."]
    #[serde(rename = "azureSqlVirtualMachine", default, skip_serializing_if = "Option::is_none")]
    pub azure_sql_virtual_machine: Option<SkuRecommendationSummary>,
}
impl TargetReadiness {
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
#[doc = "Service principal for uploading billing, metrics and logs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UploadServicePrincipal {
    #[doc = "Client ID of the service principal for uploading data."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Tenant ID of the service principal."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Authority for the service principal. Example: https://login.microsoftonline.com/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[doc = "Secret of the service principal"]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}
impl UploadServicePrincipal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties on upload watermark.  Mostly timestamp for each upload data type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UploadWatermark {
    #[doc = "Last uploaded date for metrics from kubernetes cluster. Defaults to current date time"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub metrics: Option<::time::OffsetDateTime>,
    #[doc = "Last uploaded date for logs from kubernetes cluster. Defaults to current date time"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub logs: Option<::time::OffsetDateTime>,
    #[doc = "Last uploaded date for usages from kubernetes cluster. Defaults to current date time"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub usages: Option<::time::OffsetDateTime>,
}
impl UploadWatermark {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of database names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Databases {
    #[doc = "List of database names."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl Databases {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Transparent data encryption information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct K8stransparentDataEncryption {
    #[doc = "Transparent data encryption mode. Can be Service Managed, Customer managed or disabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[doc = "Protector secret for customer managed Transparent data encryption mode"]
    #[serde(rename = "protectorSecret", default, skip_serializing_if = "Option::is_none")]
    pub protector_secret: Option<String>,
}
impl K8stransparentDataEncryption {
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
