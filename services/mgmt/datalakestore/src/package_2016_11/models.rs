#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Subscription-level properties and limits for Data Lake Store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapabilityInformation {
    #[doc = "The subscription credentials that uniquely identifies the subscription."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The subscription state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<capability_information::State>,
    #[doc = "The maximum supported number of accounts under this subscription."]
    #[serde(rename = "maxAccountCount", default, skip_serializing_if = "Option::is_none")]
    pub max_account_count: Option<i32>,
    #[doc = "The current number of accounts under this subscription."]
    #[serde(rename = "accountCount", default, skip_serializing_if = "Option::is_none")]
    pub account_count: Option<i32>,
    #[doc = "The Boolean value of true or false to indicate the maintenance state."]
    #[serde(rename = "migrationState", default, skip_serializing_if = "Option::is_none")]
    pub migration_state: Option<bool>,
}
impl CapabilityInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod capability_information {
    use super::*;
    #[doc = "The subscription state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Registered,
        Suspended,
        Deleted,
        Unregistered,
        Warned,
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
                Self::Registered => serializer.serialize_unit_variant("State", 0u32, "Registered"),
                Self::Suspended => serializer.serialize_unit_variant("State", 1u32, "Suspended"),
                Self::Deleted => serializer.serialize_unit_variant("State", 2u32, "Deleted"),
                Self::Unregistered => serializer.serialize_unit_variant("State", 3u32, "Unregistered"),
                Self::Warned => serializer.serialize_unit_variant("State", 4u32, "Warned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Data Lake Store account name availability check parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameters {
    #[doc = "The Data Lake Store name to check availability for."]
    pub name: String,
    #[doc = "The resource type. Note: This should not be set by the user, as the constant value is Microsoft.DataLakeStore/accounts"]
    #[serde(rename = "type")]
    pub type_: check_name_availability_parameters::Type,
}
impl CheckNameAvailabilityParameters {
    pub fn new(name: String, type_: check_name_availability_parameters::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod check_name_availability_parameters {
    use super::*;
    #[doc = "The resource type. Note: This should not be set by the user, as the constant value is Microsoft.DataLakeStore/accounts"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.DataLakeStore/accounts")]
        MicrosoftDataLakeStoreAccounts,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateDataLakeStoreAccountParameters {
    #[doc = "The resource location."]
    pub location: String,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The encryption identity properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<EncryptionIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CreateDataLakeStoreAccountProperties>,
}
impl CreateDataLakeStoreAccountParameters {
    pub fn new(location: String) -> Self {
        Self {
            location,
            tags: None,
            identity: None,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateDataLakeStoreAccountProperties {
    #[doc = "The default owner group for all new folders and files created in the Data Lake Store account."]
    #[serde(rename = "defaultGroup", default, skip_serializing_if = "Option::is_none")]
    pub default_group: Option<String>,
    #[doc = "The encryption configuration for the account."]
    #[serde(rename = "encryptionConfig", default, skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    #[doc = "The current state of encryption for this Data Lake Store account."]
    #[serde(rename = "encryptionState", default, skip_serializing_if = "Option::is_none")]
    pub encryption_state: Option<create_data_lake_store_account_properties::EncryptionState>,
    #[doc = "The list of firewall rules associated with this Data Lake Store account."]
    #[serde(rename = "firewallRules", default, skip_serializing_if = "Vec::is_empty")]
    pub firewall_rules: Vec<CreateFirewallRuleWithAccountParameters>,
    #[doc = "The list of virtual network rules associated with this Data Lake Store account."]
    #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<CreateVirtualNetworkRuleWithAccountParameters>,
    #[doc = "The current state of the IP address firewall for this Data Lake Store account."]
    #[serde(rename = "firewallState", default, skip_serializing_if = "Option::is_none")]
    pub firewall_state: Option<create_data_lake_store_account_properties::FirewallState>,
    #[doc = "The current state of allowing or disallowing IPs originating within Azure through the firewall. If the firewall is disabled, this is not enforced."]
    #[serde(rename = "firewallAllowAzureIps", default, skip_serializing_if = "Option::is_none")]
    pub firewall_allow_azure_ips: Option<create_data_lake_store_account_properties::FirewallAllowAzureIps>,
    #[doc = "The list of trusted identity providers associated with this Data Lake Store account."]
    #[serde(rename = "trustedIdProviders", default, skip_serializing_if = "Vec::is_empty")]
    pub trusted_id_providers: Vec<CreateTrustedIdProviderWithAccountParameters>,
    #[doc = "The current state of the trusted identity provider feature for this Data Lake Store account."]
    #[serde(rename = "trustedIdProviderState", default, skip_serializing_if = "Option::is_none")]
    pub trusted_id_provider_state: Option<create_data_lake_store_account_properties::TrustedIdProviderState>,
    #[doc = "The commitment tier to use for next month."]
    #[serde(rename = "newTier", default, skip_serializing_if = "Option::is_none")]
    pub new_tier: Option<create_data_lake_store_account_properties::NewTier>,
}
impl CreateDataLakeStoreAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod create_data_lake_store_account_properties {
    use super::*;
    #[doc = "The current state of encryption for this Data Lake Store account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionState {
        Enabled,
        Disabled,
    }
    #[doc = "The current state of the IP address firewall for this Data Lake Store account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallState {
        Enabled,
        Disabled,
    }
    #[doc = "The current state of allowing or disallowing IPs originating within Azure through the firewall. If the firewall is disabled, this is not enforced."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallAllowAzureIps {
        Enabled,
        Disabled,
    }
    #[doc = "The current state of the trusted identity provider feature for this Data Lake Store account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TrustedIdProviderState {
        Enabled,
        Disabled,
    }
    #[doc = "The commitment tier to use for next month."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTier {
        Consumption,
        #[serde(rename = "Commitment_1TB")]
        Commitment1tb,
        #[serde(rename = "Commitment_10TB")]
        Commitment10tb,
        #[serde(rename = "Commitment_100TB")]
        Commitment100tb,
        #[serde(rename = "Commitment_500TB")]
        Commitment500tb,
        #[serde(rename = "Commitment_1PB")]
        Commitment1pb,
        #[serde(rename = "Commitment_5PB")]
        Commitment5pb,
    }
}
#[doc = "The parameters used to create a new firewall rule while creating a new Data Lake Store account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFirewallRuleWithAccountParameters {
    #[doc = "The unique name of the firewall rule to create."]
    pub name: String,
    #[doc = "The firewall rule properties to use when creating a new firewall rule."]
    pub properties: CreateOrUpdateFirewallRuleProperties,
}
impl CreateFirewallRuleWithAccountParameters {
    pub fn new(name: String, properties: CreateOrUpdateFirewallRuleProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "The parameters used to create a new firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateFirewallRuleParameters {
    #[doc = "The firewall rule properties to use when creating a new firewall rule."]
    pub properties: CreateOrUpdateFirewallRuleProperties,
}
impl CreateOrUpdateFirewallRuleParameters {
    pub fn new(properties: CreateOrUpdateFirewallRuleProperties) -> Self {
        Self { properties }
    }
}
#[doc = "The firewall rule properties to use when creating a new firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateFirewallRuleProperties {
    #[doc = "The start IP address for the firewall rule. This can be either ipv4 or ipv6. Start and End should be in the same protocol."]
    #[serde(rename = "startIpAddress")]
    pub start_ip_address: String,
    #[doc = "The end IP address for the firewall rule. This can be either ipv4 or ipv6. Start and End should be in the same protocol."]
    #[serde(rename = "endIpAddress")]
    pub end_ip_address: String,
}
impl CreateOrUpdateFirewallRuleProperties {
    pub fn new(start_ip_address: String, end_ip_address: String) -> Self {
        Self {
            start_ip_address,
            end_ip_address,
        }
    }
}
#[doc = "The parameters used to create a new trusted identity provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateTrustedIdProviderParameters {
    #[doc = "The trusted identity provider properties to use when creating a new trusted identity provider."]
    pub properties: CreateOrUpdateTrustedIdProviderProperties,
}
impl CreateOrUpdateTrustedIdProviderParameters {
    pub fn new(properties: CreateOrUpdateTrustedIdProviderProperties) -> Self {
        Self { properties }
    }
}
#[doc = "The trusted identity provider properties to use when creating a new trusted identity provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateTrustedIdProviderProperties {
    #[doc = "The URL of this trusted identity provider."]
    #[serde(rename = "idProvider")]
    pub id_provider: String,
}
impl CreateOrUpdateTrustedIdProviderProperties {
    pub fn new(id_provider: String) -> Self {
        Self { id_provider }
    }
}
#[doc = "The parameters used to create a new virtual network rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateVirtualNetworkRuleParameters {
    #[doc = "The virtual network rule properties to use when creating a new virtual network rule."]
    pub properties: CreateOrUpdateVirtualNetworkRuleProperties,
}
impl CreateOrUpdateVirtualNetworkRuleParameters {
    pub fn new(properties: CreateOrUpdateVirtualNetworkRuleProperties) -> Self {
        Self { properties }
    }
}
#[doc = "The virtual network rule properties to use when creating a new virtual network rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateVirtualNetworkRuleProperties {
    #[doc = "The resource identifier for the subnet."]
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
}
impl CreateOrUpdateVirtualNetworkRuleProperties {
    pub fn new(subnet_id: String) -> Self {
        Self { subnet_id }
    }
}
#[doc = "The parameters used to create a new trusted identity provider while creating a new Data Lake Store account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTrustedIdProviderWithAccountParameters {
    #[doc = "The unique name of the trusted identity provider to create."]
    pub name: String,
    #[doc = "The trusted identity provider properties to use when creating a new trusted identity provider."]
    pub properties: CreateOrUpdateTrustedIdProviderProperties,
}
impl CreateTrustedIdProviderWithAccountParameters {
    pub fn new(name: String, properties: CreateOrUpdateTrustedIdProviderProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "The parameters used to create a new virtual network rule while creating a new Data Lake Store account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateVirtualNetworkRuleWithAccountParameters {
    #[doc = "The unique name of the virtual network rule to create."]
    pub name: String,
    #[doc = "The virtual network rule properties to use when creating a new virtual network rule."]
    pub properties: CreateOrUpdateVirtualNetworkRuleProperties,
}
impl CreateVirtualNetworkRuleWithAccountParameters {
    pub fn new(name: String, properties: CreateOrUpdateVirtualNetworkRuleProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "Data Lake Store account information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeStoreAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The encryption identity properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<EncryptionIdentity>,
    #[doc = "Data Lake Store account properties information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeStoreAccountProperties>,
}
impl DataLakeStoreAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Basic Data Lake Store account information, returned on list calls."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeStoreAccountBasic {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The basic account specific properties that are associated with an underlying Data Lake Store account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeStoreAccountPropertiesBasic>,
}
impl DataLakeStoreAccountBasic {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store account list information response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeStoreAccountListResult {
    #[doc = "The results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataLakeStoreAccountBasic>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataLakeStoreAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataLakeStoreAccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store account properties information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeStoreAccountProperties {
    #[serde(flatten)]
    pub data_lake_store_account_properties_basic: DataLakeStoreAccountPropertiesBasic,
    #[doc = "The default owner group for all new folders and files created in the Data Lake Store account."]
    #[serde(rename = "defaultGroup", default, skip_serializing_if = "Option::is_none")]
    pub default_group: Option<String>,
    #[doc = "The encryption configuration for the account."]
    #[serde(rename = "encryptionConfig", default, skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    #[doc = "The current state of encryption for this Data Lake Store account."]
    #[serde(rename = "encryptionState", default, skip_serializing_if = "Option::is_none")]
    pub encryption_state: Option<data_lake_store_account_properties::EncryptionState>,
    #[doc = "The current state of encryption provisioning for this Data Lake Store account."]
    #[serde(rename = "encryptionProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub encryption_provisioning_state: Option<data_lake_store_account_properties::EncryptionProvisioningState>,
    #[doc = "The list of firewall rules associated with this Data Lake Store account."]
    #[serde(rename = "firewallRules", default, skip_serializing_if = "Vec::is_empty")]
    pub firewall_rules: Vec<FirewallRule>,
    #[doc = "The list of virtual network rules associated with this Data Lake Store account."]
    #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
    #[doc = "The current state of the IP address firewall for this Data Lake Store account."]
    #[serde(rename = "firewallState", default, skip_serializing_if = "Option::is_none")]
    pub firewall_state: Option<data_lake_store_account_properties::FirewallState>,
    #[doc = "The current state of allowing or disallowing IPs originating within Azure through the firewall. If the firewall is disabled, this is not enforced."]
    #[serde(rename = "firewallAllowAzureIps", default, skip_serializing_if = "Option::is_none")]
    pub firewall_allow_azure_ips: Option<data_lake_store_account_properties::FirewallAllowAzureIps>,
    #[doc = "The list of trusted identity providers associated with this Data Lake Store account."]
    #[serde(rename = "trustedIdProviders", default, skip_serializing_if = "Vec::is_empty")]
    pub trusted_id_providers: Vec<TrustedIdProvider>,
    #[doc = "The current state of the trusted identity provider feature for this Data Lake Store account."]
    #[serde(rename = "trustedIdProviderState", default, skip_serializing_if = "Option::is_none")]
    pub trusted_id_provider_state: Option<data_lake_store_account_properties::TrustedIdProviderState>,
    #[doc = "The commitment tier to use for next month."]
    #[serde(rename = "newTier", default, skip_serializing_if = "Option::is_none")]
    pub new_tier: Option<data_lake_store_account_properties::NewTier>,
    #[doc = "The commitment tier in use for the current month."]
    #[serde(rename = "currentTier", default, skip_serializing_if = "Option::is_none")]
    pub current_tier: Option<data_lake_store_account_properties::CurrentTier>,
}
impl DataLakeStoreAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_lake_store_account_properties {
    use super::*;
    #[doc = "The current state of encryption for this Data Lake Store account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionState {
        Enabled,
        Disabled,
    }
    #[doc = "The current state of encryption provisioning for this Data Lake Store account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionProvisioningState {
        Creating,
        Succeeded,
    }
    #[doc = "The current state of the IP address firewall for this Data Lake Store account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallState {
        Enabled,
        Disabled,
    }
    #[doc = "The current state of allowing or disallowing IPs originating within Azure through the firewall. If the firewall is disabled, this is not enforced."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallAllowAzureIps {
        Enabled,
        Disabled,
    }
    #[doc = "The current state of the trusted identity provider feature for this Data Lake Store account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TrustedIdProviderState {
        Enabled,
        Disabled,
    }
    #[doc = "The commitment tier to use for next month."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTier {
        Consumption,
        #[serde(rename = "Commitment_1TB")]
        Commitment1tb,
        #[serde(rename = "Commitment_10TB")]
        Commitment10tb,
        #[serde(rename = "Commitment_100TB")]
        Commitment100tb,
        #[serde(rename = "Commitment_500TB")]
        Commitment500tb,
        #[serde(rename = "Commitment_1PB")]
        Commitment1pb,
        #[serde(rename = "Commitment_5PB")]
        Commitment5pb,
    }
    #[doc = "The commitment tier in use for the current month."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CurrentTier {
        Consumption,
        #[serde(rename = "Commitment_1TB")]
        Commitment1tb,
        #[serde(rename = "Commitment_10TB")]
        Commitment10tb,
        #[serde(rename = "Commitment_100TB")]
        Commitment100tb,
        #[serde(rename = "Commitment_500TB")]
        Commitment500tb,
        #[serde(rename = "Commitment_1PB")]
        Commitment1pb,
        #[serde(rename = "Commitment_5PB")]
        Commitment5pb,
    }
}
#[doc = "The basic account specific properties that are associated with an underlying Data Lake Store account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeStoreAccountPropertiesBasic {
    #[doc = "The unique identifier associated with this Data Lake Store account."]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "The provisioning status of the Data Lake Store account."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<data_lake_store_account_properties_basic::ProvisioningState>,
    #[doc = "The state of the Data Lake Store account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<data_lake_store_account_properties_basic::State>,
    #[doc = "The account creation time."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The account last modified time."]
    #[serde(rename = "lastModifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "The full CName endpoint for this account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl DataLakeStoreAccountPropertiesBasic {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_lake_store_account_properties_basic {
    use super::*;
    #[doc = "The provisioning status of the Data Lake Store account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Failed,
        Creating,
        Running,
        Succeeded,
        Patching,
        Suspending,
        Resuming,
        Deleting,
        Deleted,
        Undeleting,
        Canceled,
    }
    #[doc = "The state of the Data Lake Store account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Active,
        Suspended,
    }
}
#[doc = "The encryption configuration for the account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionConfig {
    #[doc = "The type of encryption configuration being used. Currently the only supported types are 'UserManaged' and 'ServiceManaged'."]
    #[serde(rename = "type")]
    pub type_: encryption_config::Type,
    #[doc = "Metadata information used by account encryption."]
    #[serde(rename = "keyVaultMetaInfo", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_meta_info: Option<KeyVaultMetaInfo>,
}
impl EncryptionConfig {
    pub fn new(type_: encryption_config::Type) -> Self {
        Self {
            type_,
            key_vault_meta_info: None,
        }
    }
}
pub mod encryption_config {
    use super::*;
    #[doc = "The type of encryption configuration being used. Currently the only supported types are 'UserManaged' and 'ServiceManaged'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        UserManaged,
        ServiceManaged,
    }
}
#[doc = "The encryption identity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionIdentity {
    #[doc = "The type of encryption being used. Currently the only supported type is 'SystemAssigned'."]
    #[serde(rename = "type")]
    pub type_: encryption_identity::Type,
    #[doc = "The principal identifier associated with the encryption."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant identifier associated with the encryption."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl EncryptionIdentity {
    pub fn new(type_: encryption_identity::Type) -> Self {
        Self {
            type_,
            principal_id: None,
            tenant_id: None,
        }
    }
}
pub mod encryption_identity {
    use super::*;
    #[doc = "The type of encryption being used. Currently the only supported type is 'SystemAssigned'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[doc = "Data Lake Store firewall rule information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRule {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The firewall rule properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FirewallRuleProperties>,
}
impl FirewallRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store firewall rule list information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRuleListResult {
    #[doc = "The results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FirewallRule>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FirewallRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FirewallRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The firewall rule properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FirewallRuleProperties {
    #[doc = "The start IP address for the firewall rule. This can be either ipv4 or ipv6. Start and End should be in the same protocol."]
    #[serde(rename = "startIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub start_ip_address: Option<String>,
    #[doc = "The end IP address for the firewall rule. This can be either ipv4 or ipv6. Start and End should be in the same protocol."]
    #[serde(rename = "endIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub end_ip_address: Option<String>,
}
impl FirewallRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata information used by account encryption."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultMetaInfo {
    #[doc = "The resource identifier for the user managed Key Vault being used to encrypt."]
    #[serde(rename = "keyVaultResourceId")]
    pub key_vault_resource_id: String,
    #[doc = "The name of the user managed encryption key."]
    #[serde(rename = "encryptionKeyName")]
    pub encryption_key_name: String,
    #[doc = "The version of the user managed encryption key."]
    #[serde(rename = "encryptionKeyVersion")]
    pub encryption_key_version: String,
}
impl KeyVaultMetaInfo {
    pub fn new(key_vault_resource_id: String, encryption_key_name: String, encryption_key_version: String) -> Self {
        Self {
            key_vault_resource_id,
            encryption_key_name,
            encryption_key_version,
        }
    }
}
#[doc = "Data Lake Store account name availability result information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailabilityInformation {
    #[doc = "The Boolean value of true or false to indicate whether the Data Lake Store account name is available or not."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the Data Lake Store account name is not available, if nameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The message describing why the Data Lake Store account name is not available, if nameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl NameAvailabilityInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An available operation for Data Lake Store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The display information for a particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "The intended executor of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
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
}
#[doc = "The display information for a particular operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "The resource provider of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The resource type of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "A friendly name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "A friendly description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of available operations for Data Lake Store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a nested resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store trusted identity provider information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrustedIdProvider {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The trusted identity provider properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TrustedIdProviderProperties>,
}
impl TrustedIdProvider {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store trusted identity provider list information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrustedIdProviderListResult {
    #[doc = "The results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TrustedIdProvider>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TrustedIdProviderListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TrustedIdProviderListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The trusted identity provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrustedIdProviderProperties {
    #[doc = "The URL of this trusted identity provider."]
    #[serde(rename = "idProvider", default, skip_serializing_if = "Option::is_none")]
    pub id_provider: Option<String>,
}
impl TrustedIdProviderProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store account information to update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateDataLakeStoreAccountParameters {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Data Lake Store account properties information to be updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateDataLakeStoreAccountProperties>,
}
impl UpdateDataLakeStoreAccountParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store account properties information to be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateDataLakeStoreAccountProperties {
    #[doc = "The default owner group for all new folders and files created in the Data Lake Store account."]
    #[serde(rename = "defaultGroup", default, skip_serializing_if = "Option::is_none")]
    pub default_group: Option<String>,
    #[doc = "The encryption configuration used to update a user managed Key Vault key."]
    #[serde(rename = "encryptionConfig", default, skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<UpdateEncryptionConfig>,
    #[doc = "The list of firewall rules associated with this Data Lake Store account."]
    #[serde(rename = "firewallRules", default, skip_serializing_if = "Vec::is_empty")]
    pub firewall_rules: Vec<UpdateFirewallRuleWithAccountParameters>,
    #[doc = "The list of virtual network rules associated with this Data Lake Store account."]
    #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<UpdateVirtualNetworkRuleWithAccountParameters>,
    #[doc = "The current state of the IP address firewall for this Data Lake Store account. Disabling the firewall does not remove existing rules, they will just be ignored until the firewall is re-enabled."]
    #[serde(rename = "firewallState", default, skip_serializing_if = "Option::is_none")]
    pub firewall_state: Option<update_data_lake_store_account_properties::FirewallState>,
    #[doc = "The current state of allowing or disallowing IPs originating within Azure through the firewall. If the firewall is disabled, this is not enforced."]
    #[serde(rename = "firewallAllowAzureIps", default, skip_serializing_if = "Option::is_none")]
    pub firewall_allow_azure_ips: Option<update_data_lake_store_account_properties::FirewallAllowAzureIps>,
    #[doc = "The list of trusted identity providers associated with this Data Lake Store account."]
    #[serde(rename = "trustedIdProviders", default, skip_serializing_if = "Vec::is_empty")]
    pub trusted_id_providers: Vec<UpdateTrustedIdProviderWithAccountParameters>,
    #[doc = "The current state of the trusted identity provider feature for this Data Lake Store account. Disabling trusted identity provider functionality does not remove the providers, they will just be ignored until this feature is re-enabled."]
    #[serde(rename = "trustedIdProviderState", default, skip_serializing_if = "Option::is_none")]
    pub trusted_id_provider_state: Option<update_data_lake_store_account_properties::TrustedIdProviderState>,
    #[doc = "The commitment tier to use for next month."]
    #[serde(rename = "newTier", default, skip_serializing_if = "Option::is_none")]
    pub new_tier: Option<update_data_lake_store_account_properties::NewTier>,
}
impl UpdateDataLakeStoreAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_data_lake_store_account_properties {
    use super::*;
    #[doc = "The current state of the IP address firewall for this Data Lake Store account. Disabling the firewall does not remove existing rules, they will just be ignored until the firewall is re-enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallState {
        Enabled,
        Disabled,
    }
    #[doc = "The current state of allowing or disallowing IPs originating within Azure through the firewall. If the firewall is disabled, this is not enforced."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallAllowAzureIps {
        Enabled,
        Disabled,
    }
    #[doc = "The current state of the trusted identity provider feature for this Data Lake Store account. Disabling trusted identity provider functionality does not remove the providers, they will just be ignored until this feature is re-enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TrustedIdProviderState {
        Enabled,
        Disabled,
    }
    #[doc = "The commitment tier to use for next month."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTier {
        Consumption,
        #[serde(rename = "Commitment_1TB")]
        Commitment1tb,
        #[serde(rename = "Commitment_10TB")]
        Commitment10tb,
        #[serde(rename = "Commitment_100TB")]
        Commitment100tb,
        #[serde(rename = "Commitment_500TB")]
        Commitment500tb,
        #[serde(rename = "Commitment_1PB")]
        Commitment1pb,
        #[serde(rename = "Commitment_5PB")]
        Commitment5pb,
    }
}
#[doc = "The encryption configuration used to update a user managed Key Vault key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateEncryptionConfig {
    #[doc = "The Key Vault update information used for user managed key rotation."]
    #[serde(rename = "keyVaultMetaInfo", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_meta_info: Option<UpdateKeyVaultMetaInfo>,
}
impl UpdateEncryptionConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters used to update a firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateFirewallRuleParameters {
    #[doc = "The firewall rule properties to use when updating a firewall rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateFirewallRuleProperties>,
}
impl UpdateFirewallRuleParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The firewall rule properties to use when updating a firewall rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateFirewallRuleProperties {
    #[doc = "The start IP address for the firewall rule. This can be either ipv4 or ipv6. Start and End should be in the same protocol."]
    #[serde(rename = "startIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub start_ip_address: Option<String>,
    #[doc = "The end IP address for the firewall rule. This can be either ipv4 or ipv6. Start and End should be in the same protocol."]
    #[serde(rename = "endIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub end_ip_address: Option<String>,
}
impl UpdateFirewallRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters used to update a firewall rule while updating a Data Lake Store account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateFirewallRuleWithAccountParameters {
    #[doc = "The unique name of the firewall rule to update."]
    pub name: String,
    #[doc = "The firewall rule properties to use when updating a firewall rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateFirewallRuleProperties>,
}
impl UpdateFirewallRuleWithAccountParameters {
    pub fn new(name: String) -> Self {
        Self { name, properties: None }
    }
}
#[doc = "The Key Vault update information used for user managed key rotation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateKeyVaultMetaInfo {
    #[doc = "The version of the user managed encryption key to update through a key rotation."]
    #[serde(rename = "encryptionKeyVersion", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key_version: Option<String>,
}
impl UpdateKeyVaultMetaInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters used to update a trusted identity provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateTrustedIdProviderParameters {
    #[doc = "The trusted identity provider properties to use when updating a trusted identity provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateTrustedIdProviderProperties>,
}
impl UpdateTrustedIdProviderParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The trusted identity provider properties to use when updating a trusted identity provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateTrustedIdProviderProperties {
    #[doc = "The URL of this trusted identity provider."]
    #[serde(rename = "idProvider", default, skip_serializing_if = "Option::is_none")]
    pub id_provider: Option<String>,
}
impl UpdateTrustedIdProviderProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters used to update a trusted identity provider while updating a Data Lake Store account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateTrustedIdProviderWithAccountParameters {
    #[doc = "The unique name of the trusted identity provider to update."]
    pub name: String,
    #[doc = "The trusted identity provider properties to use when updating a trusted identity provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateTrustedIdProviderProperties>,
}
impl UpdateTrustedIdProviderWithAccountParameters {
    pub fn new(name: String) -> Self {
        Self { name, properties: None }
    }
}
#[doc = "The parameters used to update a virtual network rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateVirtualNetworkRuleParameters {
    #[doc = "The virtual network rule properties to use when updating a virtual network rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateVirtualNetworkRuleProperties>,
}
impl UpdateVirtualNetworkRuleParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The virtual network rule properties to use when updating a virtual network rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateVirtualNetworkRuleProperties {
    #[doc = "The resource identifier for the subnet."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}
impl UpdateVirtualNetworkRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters used to update a virtual network rule while updating a Data Lake Store account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateVirtualNetworkRuleWithAccountParameters {
    #[doc = "The unique name of the virtual network rule to update."]
    pub name: String,
    #[doc = "The virtual network rule properties to use when updating a virtual network rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateVirtualNetworkRuleProperties>,
}
impl UpdateVirtualNetworkRuleWithAccountParameters {
    pub fn new(name: String) -> Self {
        Self { name, properties: None }
    }
}
#[doc = "Describes the Resource Usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[doc = "Gets the unit of measurement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<usage::Unit>,
    #[doc = "Resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the current count of the allocated resources in the subscription."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i32>,
    #[doc = "Gets the maximum count of the resources that can be allocated in the subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "The usage names that can be used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<UsageName>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod usage {
    use super::*;
    #[doc = "Gets the unit of measurement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
        Bytes,
        Seconds,
        Percent,
        CountsPerSecond,
        BytesPerSecond,
    }
}
#[doc = "The response from the List Usages operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageListResult {
    #[doc = "Gets or sets the list of Storage Resource Usages."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
}
impl azure_core::Continuable for UsageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl UsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The usage names that can be used."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageName {
    #[doc = "Gets a string describing the resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Gets a localized string describing the resource name."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl UsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store virtual network rule information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkRule {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The virtual network rule properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkRuleProperties>,
}
impl VirtualNetworkRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store virtual network rule list information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkRuleListResult {
    #[doc = "The results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetworkRule>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualNetworkRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The virtual network rule properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkRuleProperties {
    #[doc = "The resource identifier for the subnet."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}
impl VirtualNetworkRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
