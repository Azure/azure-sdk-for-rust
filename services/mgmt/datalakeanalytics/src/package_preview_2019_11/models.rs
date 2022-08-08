#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The parameters used to add a new Data Lake Store account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddDataLakeStoreParameters {
    #[doc = "The Data Lake Store account properties to use when adding a new Data Lake Store account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AddDataLakeStoreProperties>,
}
impl AddDataLakeStoreParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Data Lake Store account properties to use when adding a new Data Lake Store account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddDataLakeStoreProperties {
    #[doc = "The optional suffix for the Data Lake Store account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
impl AddDataLakeStoreProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters used to add a new Data Lake Store account while creating a new Data Lake Analytics account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddDataLakeStoreWithAccountParameters {
    #[doc = "The unique name of the Data Lake Store account to add."]
    pub name: String,
    #[doc = "The Data Lake Store account properties to use when adding a new Data Lake Store account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AddDataLakeStoreProperties>,
}
impl AddDataLakeStoreWithAccountParameters {
    pub fn new(name: String) -> Self {
        Self { name, properties: None }
    }
}
#[doc = "The parameters used to add a new Azure Storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddStorageAccountParameters {
    #[doc = "The Azure Storage account properties to use when adding a new Azure Storage account."]
    pub properties: AddStorageAccountProperties,
}
impl AddStorageAccountParameters {
    pub fn new(properties: AddStorageAccountProperties) -> Self {
        Self { properties }
    }
}
#[doc = "The Azure Storage account properties to use when adding a new Azure Storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddStorageAccountProperties {
    #[doc = "The access key associated with this Azure Storage account that will be used to connect to it."]
    #[serde(rename = "accessKey")]
    pub access_key: String,
    #[doc = "The optional suffix for the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
impl AddStorageAccountProperties {
    pub fn new(access_key: String) -> Self {
        Self { access_key, suffix: None }
    }
}
#[doc = "The parameters used to add a new Azure Storage account while creating a new Data Lake Analytics account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddStorageAccountWithAccountParameters {
    #[doc = "The unique name of the Azure Storage account to add."]
    pub name: String,
    #[doc = "The Azure Storage account properties to use when adding a new Azure Storage account."]
    pub properties: AddStorageAccountProperties,
}
impl AddStorageAccountWithAccountParameters {
    pub fn new(name: String, properties: AddStorageAccountProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "Subscription-level properties and limits for Data Lake Analytics."]
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
#[doc = "Data Lake Analytics account name availability check parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameters {
    #[doc = "The Data Lake Analytics name to check availability for."]
    pub name: String,
    #[doc = "The resource type. Note: This should not be set by the user, as the constant value is Microsoft.DataLakeAnalytics/accounts"]
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
    #[doc = "The resource type. Note: This should not be set by the user, as the constant value is Microsoft.DataLakeAnalytics/accounts"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.DataLakeAnalytics/accounts")]
        MicrosoftDataLakeAnalyticsAccounts,
    }
}
#[doc = "Data Lake Analytics compute policy information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputePolicy {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The compute policy properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ComputePolicyProperties>,
}
impl ComputePolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of compute policies in the account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputePolicyListResult {
    #[doc = "The results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ComputePolicy>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ComputePolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ComputePolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The compute policy properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputePolicyProperties {
    #[doc = "The AAD object identifier for the entity to create a policy for."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "The type of AAD object the object identifier refers to."]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<compute_policy_properties::ObjectType>,
    #[doc = "The maximum degree of parallelism per job this user can use to submit jobs."]
    #[serde(rename = "maxDegreeOfParallelismPerJob", default, skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism_per_job: Option<i32>,
    #[doc = "The minimum priority per job this user can use to submit jobs."]
    #[serde(rename = "minPriorityPerJob", default, skip_serializing_if = "Option::is_none")]
    pub min_priority_per_job: Option<i32>,
}
impl ComputePolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod compute_policy_properties {
    use super::*;
    #[doc = "The type of AAD object the object identifier refers to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ObjectType")]
    pub enum ObjectType {
        User,
        Group,
        ServicePrincipal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ObjectType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ObjectType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ObjectType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("ObjectType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("ObjectType", 1u32, "Group"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("ObjectType", 2u32, "ServicePrincipal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters used to create a new compute policy while creating a new Data Lake Analytics account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateComputePolicyWithAccountParameters {
    #[doc = "The unique name of the compute policy to create."]
    pub name: String,
    #[doc = "The compute policy properties to use when creating a new compute policy."]
    pub properties: CreateOrUpdateComputePolicyProperties,
}
impl CreateComputePolicyWithAccountParameters {
    pub fn new(name: String, properties: CreateOrUpdateComputePolicyProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "The parameters to use for creating a Data Lake Analytics account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateDataLakeAnalyticsAccountParameters {
    #[doc = "The resource location."]
    pub location: String,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub properties: CreateDataLakeAnalyticsAccountProperties,
}
impl CreateDataLakeAnalyticsAccountParameters {
    pub fn new(location: String, properties: CreateDataLakeAnalyticsAccountProperties) -> Self {
        Self {
            location,
            tags: None,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateDataLakeAnalyticsAccountProperties {
    #[doc = "The default Data Lake Store account associated with this account."]
    #[serde(rename = "defaultDataLakeStoreAccount")]
    pub default_data_lake_store_account: String,
    #[doc = "The list of Data Lake Store accounts associated with this account."]
    #[serde(rename = "dataLakeStoreAccounts")]
    pub data_lake_store_accounts: Vec<AddDataLakeStoreWithAccountParameters>,
    #[doc = "The list of Azure Blob Storage accounts associated with this account."]
    #[serde(rename = "storageAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_accounts: Vec<AddStorageAccountWithAccountParameters>,
    #[doc = "The list of compute policies associated with this account."]
    #[serde(rename = "computePolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub compute_policies: Vec<CreateComputePolicyWithAccountParameters>,
    #[doc = "The list of firewall rules associated with this account."]
    #[serde(rename = "firewallRules", default, skip_serializing_if = "Vec::is_empty")]
    pub firewall_rules: Vec<CreateFirewallRuleWithAccountParameters>,
    #[doc = "The current state of the IP address firewall for this account."]
    #[serde(rename = "firewallState", default, skip_serializing_if = "Option::is_none")]
    pub firewall_state: Option<create_data_lake_analytics_account_properties::FirewallState>,
    #[doc = "The current state of allowing or disallowing IPs originating within Azure through the firewall. If the firewall is disabled, this is not enforced."]
    #[serde(rename = "firewallAllowAzureIps", default, skip_serializing_if = "Option::is_none")]
    pub firewall_allow_azure_ips: Option<create_data_lake_analytics_account_properties::FirewallAllowAzureIps>,
    #[doc = "The commitment tier for the next month."]
    #[serde(rename = "newTier", default, skip_serializing_if = "Option::is_none")]
    pub new_tier: Option<create_data_lake_analytics_account_properties::NewTier>,
    #[doc = "The maximum supported jobs running under the account at the same time."]
    #[serde(rename = "maxJobCount", default, skip_serializing_if = "Option::is_none")]
    pub max_job_count: Option<i32>,
    #[doc = "The maximum supported degree of parallelism for this account."]
    #[serde(rename = "maxDegreeOfParallelism", default, skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism: Option<i32>,
    #[doc = "The maximum supported degree of parallelism per job for this account."]
    #[serde(rename = "maxDegreeOfParallelismPerJob", default, skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism_per_job: Option<i32>,
    #[doc = "The minimum supported priority per job for this account."]
    #[serde(rename = "minPriorityPerJob", default, skip_serializing_if = "Option::is_none")]
    pub min_priority_per_job: Option<i32>,
    #[doc = "The number of days that job metadata is retained."]
    #[serde(rename = "queryStoreRetention", default, skip_serializing_if = "Option::is_none")]
    pub query_store_retention: Option<i32>,
}
impl CreateDataLakeAnalyticsAccountProperties {
    pub fn new(default_data_lake_store_account: String, data_lake_store_accounts: Vec<AddDataLakeStoreWithAccountParameters>) -> Self {
        Self {
            default_data_lake_store_account,
            data_lake_store_accounts,
            storage_accounts: Vec::new(),
            compute_policies: Vec::new(),
            firewall_rules: Vec::new(),
            firewall_state: None,
            firewall_allow_azure_ips: None,
            new_tier: None,
            max_job_count: None,
            max_degree_of_parallelism: None,
            max_degree_of_parallelism_per_job: None,
            min_priority_per_job: None,
            query_store_retention: None,
        }
    }
}
pub mod create_data_lake_analytics_account_properties {
    use super::*;
    #[doc = "The current state of the IP address firewall for this account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallState {
        Enabled,
        Disabled,
    }
    impl Default for FirewallState {
        fn default() -> Self {
            Self::Disabled
        }
    }
    #[doc = "The current state of allowing or disallowing IPs originating within Azure through the firewall. If the firewall is disabled, this is not enforced."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FirewallAllowAzureIps {
        Enabled,
        Disabled,
    }
    impl Default for FirewallAllowAzureIps {
        fn default() -> Self {
            Self::Disabled
        }
    }
    #[doc = "The commitment tier for the next month."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTier {
        Consumption,
        #[serde(rename = "Commitment_100AUHours")]
        Commitment100auHours,
        #[serde(rename = "Commitment_500AUHours")]
        Commitment500auHours,
        #[serde(rename = "Commitment_1000AUHours")]
        Commitment1000auHours,
        #[serde(rename = "Commitment_5000AUHours")]
        Commitment5000auHours,
        #[serde(rename = "Commitment_10000AUHours")]
        Commitment10000auHours,
        #[serde(rename = "Commitment_50000AUHours")]
        Commitment50000auHours,
        #[serde(rename = "Commitment_100000AUHours")]
        Commitment100000auHours,
        #[serde(rename = "Commitment_500000AUHours")]
        Commitment500000auHours,
    }
    impl Default for NewTier {
        fn default() -> Self {
            Self::Consumption
        }
    }
}
#[doc = "The parameters used to create a new firewall rule while creating a new Data Lake Analytics account."]
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
#[doc = "The parameters used to create a new compute policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateComputePolicyParameters {
    #[doc = "The compute policy properties to use when creating a new compute policy."]
    pub properties: CreateOrUpdateComputePolicyProperties,
}
impl CreateOrUpdateComputePolicyParameters {
    pub fn new(properties: CreateOrUpdateComputePolicyProperties) -> Self {
        Self { properties }
    }
}
#[doc = "The compute policy properties to use when creating a new compute policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrUpdateComputePolicyProperties {
    #[doc = "The AAD object identifier for the entity to create a policy for."]
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[doc = "The type of AAD object the object identifier refers to."]
    #[serde(rename = "objectType")]
    pub object_type: create_or_update_compute_policy_properties::ObjectType,
    #[doc = "The maximum degree of parallelism per job this user can use to submit jobs. This property, the min priority per job property, or both must be passed."]
    #[serde(rename = "maxDegreeOfParallelismPerJob", default, skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism_per_job: Option<i32>,
    #[doc = "The minimum priority per job this user can use to submit jobs. This property, the max degree of parallelism per job property, or both must be passed."]
    #[serde(rename = "minPriorityPerJob", default, skip_serializing_if = "Option::is_none")]
    pub min_priority_per_job: Option<i32>,
}
impl CreateOrUpdateComputePolicyProperties {
    pub fn new(object_id: String, object_type: create_or_update_compute_policy_properties::ObjectType) -> Self {
        Self {
            object_id,
            object_type,
            max_degree_of_parallelism_per_job: None,
            min_priority_per_job: None,
        }
    }
}
pub mod create_or_update_compute_policy_properties {
    use super::*;
    #[doc = "The type of AAD object the object identifier refers to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ObjectType")]
    pub enum ObjectType {
        User,
        Group,
        ServicePrincipal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ObjectType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ObjectType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ObjectType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("ObjectType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("ObjectType", 1u32, "Group"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("ObjectType", 2u32, "ServicePrincipal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "A Data Lake Analytics account object, containing all information associated with the named Data Lake Analytics account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeAnalyticsAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The account specific properties that are associated with an underlying Data Lake Analytics account. Returned only when retrieving a specific account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeAnalyticsAccountProperties>,
}
impl DataLakeAnalyticsAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics account object, containing all information associated with the named Data Lake Analytics account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeAnalyticsAccountBasic {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The basic account specific properties that are associated with an underlying Data Lake Analytics account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeAnalyticsAccountPropertiesBasic>,
}
impl DataLakeAnalyticsAccountBasic {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Analytics account list information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeAnalyticsAccountListResult {
    #[doc = "The results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataLakeAnalyticsAccountBasic>,
    #[doc = "The current number of data lake analytics accounts under this subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataLakeAnalyticsAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataLakeAnalyticsAccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The account specific properties that are associated with an underlying Data Lake Analytics account. Returned only when retrieving a specific account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeAnalyticsAccountProperties {
    #[serde(flatten)]
    pub data_lake_analytics_account_properties_basic: DataLakeAnalyticsAccountPropertiesBasic,
    #[doc = "The type of the default Data Lake Store account associated with this account."]
    #[serde(rename = "defaultDataLakeStoreAccountType", default, skip_serializing_if = "Option::is_none")]
    pub default_data_lake_store_account_type: Option<String>,
    #[doc = "The default Data Lake Store account associated with this account."]
    #[serde(rename = "defaultDataLakeStoreAccount", default, skip_serializing_if = "Option::is_none")]
    pub default_data_lake_store_account: Option<String>,
    #[doc = "The list of Data Lake Store accounts associated with this account."]
    #[serde(rename = "dataLakeStoreAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub data_lake_store_accounts: Vec<DataLakeStoreAccountInformation>,
    #[doc = "The list of Data Lake Store accounts associated with this account."]
    #[serde(rename = "publicDataLakeStoreAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub public_data_lake_store_accounts: Vec<DataLakeStoreAccountInformation>,
    #[doc = "The list of Azure Blob Storage accounts associated with this account."]
    #[serde(rename = "storageAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_accounts: Vec<StorageAccountInformation>,
    #[doc = "The list of compute policies associated with this account."]
    #[serde(rename = "computePolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub compute_policies: Vec<ComputePolicy>,
    #[doc = "The list of hiveMetastores associated with this account."]
    #[serde(rename = "hiveMetastores", default, skip_serializing_if = "Vec::is_empty")]
    pub hive_metastores: Vec<HiveMetastore>,
    #[doc = "The list of virtualNetwork rules associated with this account."]
    #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
    #[doc = "The list of firewall rules associated with this account."]
    #[serde(rename = "firewallRules", default, skip_serializing_if = "Vec::is_empty")]
    pub firewall_rules: Vec<FirewallRule>,
    #[doc = "The current state of the IP address firewall for this account."]
    #[serde(rename = "firewallState", default, skip_serializing_if = "Option::is_none")]
    pub firewall_state: Option<data_lake_analytics_account_properties::FirewallState>,
    #[doc = "The current state of allowing or disallowing IPs originating within Azure through the firewall. If the firewall is disabled, this is not enforced."]
    #[serde(rename = "firewallAllowAzureIps", default, skip_serializing_if = "Option::is_none")]
    pub firewall_allow_azure_ips: Option<data_lake_analytics_account_properties::FirewallAllowAzureIps>,
    #[doc = "The commitment tier for the next month."]
    #[serde(rename = "newTier", default, skip_serializing_if = "Option::is_none")]
    pub new_tier: Option<data_lake_analytics_account_properties::NewTier>,
    #[doc = "The commitment tier in use for the current month."]
    #[serde(rename = "currentTier", default, skip_serializing_if = "Option::is_none")]
    pub current_tier: Option<data_lake_analytics_account_properties::CurrentTier>,
    #[doc = "The maximum supported jobs running under the account at the same time."]
    #[serde(rename = "maxJobCount", default, skip_serializing_if = "Option::is_none")]
    pub max_job_count: Option<i32>,
    #[doc = "The maximum supported active jobs under the account at the same time."]
    #[serde(rename = "maxActiveJobCountPerUser", default, skip_serializing_if = "Option::is_none")]
    pub max_active_job_count_per_user: Option<i32>,
    #[doc = "The maximum supported jobs queued under the account at the same time."]
    #[serde(rename = "maxQueuedJobCountPerUser", default, skip_serializing_if = "Option::is_none")]
    pub max_queued_job_count_per_user: Option<i32>,
    #[doc = "The maximum supported active jobs under the account at the same time."]
    #[serde(rename = "maxJobRunningTimeInMin", default, skip_serializing_if = "Option::is_none")]
    pub max_job_running_time_in_min: Option<i32>,
    #[doc = "The system defined maximum supported jobs running under the account at the same time, which restricts the maximum number of running jobs the user can set for the account."]
    #[serde(rename = "systemMaxJobCount", default, skip_serializing_if = "Option::is_none")]
    pub system_max_job_count: Option<i32>,
    #[doc = "The maximum supported degree of parallelism for this account."]
    #[serde(rename = "maxDegreeOfParallelism", default, skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism: Option<i32>,
    #[doc = "The system defined maximum supported degree of parallelism for this account, which restricts the maximum value of parallelism the user can set for the account."]
    #[serde(rename = "systemMaxDegreeOfParallelism", default, skip_serializing_if = "Option::is_none")]
    pub system_max_degree_of_parallelism: Option<i32>,
    #[doc = "The maximum supported degree of parallelism per job for this account."]
    #[serde(rename = "maxDegreeOfParallelismPerJob", default, skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism_per_job: Option<i32>,
    #[doc = "The minimum supported priority per job for this account."]
    #[serde(rename = "minPriorityPerJob", default, skip_serializing_if = "Option::is_none")]
    pub min_priority_per_job: Option<i32>,
    #[doc = "The number of days that job metadata is retained."]
    #[serde(rename = "queryStoreRetention", default, skip_serializing_if = "Option::is_none")]
    pub query_store_retention: Option<i32>,
    #[doc = "The current state of the DebugDataAccessLevel for this account."]
    #[serde(rename = "debugDataAccessLevel", default, skip_serializing_if = "Option::is_none")]
    pub debug_data_access_level: Option<data_lake_analytics_account_properties::DebugDataAccessLevel>,
}
impl DataLakeAnalyticsAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_lake_analytics_account_properties {
    use super::*;
    #[doc = "The current state of the IP address firewall for this account."]
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
    #[doc = "The commitment tier for the next month."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTier {
        Consumption,
        #[serde(rename = "Commitment_100AUHours")]
        Commitment100auHours,
        #[serde(rename = "Commitment_500AUHours")]
        Commitment500auHours,
        #[serde(rename = "Commitment_1000AUHours")]
        Commitment1000auHours,
        #[serde(rename = "Commitment_5000AUHours")]
        Commitment5000auHours,
        #[serde(rename = "Commitment_10000AUHours")]
        Commitment10000auHours,
        #[serde(rename = "Commitment_50000AUHours")]
        Commitment50000auHours,
        #[serde(rename = "Commitment_100000AUHours")]
        Commitment100000auHours,
        #[serde(rename = "Commitment_500000AUHours")]
        Commitment500000auHours,
    }
    #[doc = "The commitment tier in use for the current month."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CurrentTier {
        Consumption,
        #[serde(rename = "Commitment_100AUHours")]
        Commitment100auHours,
        #[serde(rename = "Commitment_500AUHours")]
        Commitment500auHours,
        #[serde(rename = "Commitment_1000AUHours")]
        Commitment1000auHours,
        #[serde(rename = "Commitment_5000AUHours")]
        Commitment5000auHours,
        #[serde(rename = "Commitment_10000AUHours")]
        Commitment10000auHours,
        #[serde(rename = "Commitment_50000AUHours")]
        Commitment50000auHours,
        #[serde(rename = "Commitment_100000AUHours")]
        Commitment100000auHours,
        #[serde(rename = "Commitment_500000AUHours")]
        Commitment500000auHours,
    }
    #[doc = "The current state of the DebugDataAccessLevel for this account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DebugDataAccessLevel {
        All,
        Customer,
        None,
    }
}
#[doc = "The basic account specific properties that are associated with an underlying Data Lake Analytics account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeAnalyticsAccountPropertiesBasic {
    #[doc = "The unique identifier associated with this Data Lake Analytics account."]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "The provisioning status of the Data Lake Analytics account."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<data_lake_analytics_account_properties_basic::ProvisioningState>,
    #[doc = "The state of the Data Lake Analytics account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<data_lake_analytics_account_properties_basic::State>,
    #[doc = "The account creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The account last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "The full CName endpoint for this account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl DataLakeAnalyticsAccountPropertiesBasic {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_lake_analytics_account_properties_basic {
    use super::*;
    #[doc = "The provisioning status of the Data Lake Analytics account."]
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
    #[doc = "The state of the Data Lake Analytics account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Active,
        Suspended,
    }
}
#[doc = "Data Lake Store account information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeStoreAccountInformation {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The Data Lake Store account properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataLakeStoreAccountInformationProperties>,
}
impl DataLakeStoreAccountInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Store account list information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeStoreAccountInformationListResult {
    #[doc = "The results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataLakeStoreAccountInformation>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataLakeStoreAccountInformationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataLakeStoreAccountInformationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Data Lake Store account properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeStoreAccountInformationProperties {
    #[doc = "The optional suffix for the Data Lake Store account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
impl DataLakeStoreAccountInformationProperties {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Data Lake Analytics firewall rule information."]
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
#[doc = "Data Lake Analytics firewall rule list information."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HiveMetastore {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The HiveMetastore  properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HiveMetastoreProperties>,
}
impl HiveMetastore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Analytics HiveMetastore list information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HiveMetastoreListResult {
    #[doc = "The results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HiveMetastore>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl HiveMetastoreListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The HiveMetastore  properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HiveMetastoreProperties {
    #[doc = "The serverUri for the Hive MetaStore"]
    #[serde(rename = "serverUri", default, skip_serializing_if = "Option::is_none")]
    pub server_uri: Option<String>,
    #[doc = "The databaseName for the Hive MetaStore"]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The runtimeVersion for the Hive MetaStore"]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    #[doc = "The userName for the Hive MetaStore"]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "The password for the Hive MetaStore"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The current state of the NestedResourceProvisioning for this account."]
    #[serde(rename = "nestedResourceProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub nested_resource_provisioning_state: Option<NestedResourceProvisioningState>,
}
impl HiveMetastoreProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Analytics account name availability result information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailabilityInformation {
    #[doc = "The Boolean value of true or false to indicate whether the Data Lake Analytics account name is available or not."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the Data Lake Analytics account name is not available, if nameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The message describing why the Data Lake Analytics account name is not available, if nameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl NameAvailabilityInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current state of the NestedResourceProvisioning for this account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NestedResourceProvisioningState {
    Succeeded,
    Canceled,
    Failed,
}
#[doc = "An available operation for Data Lake Analytics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The display information for a particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationMetaPropertyInfo>,
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
#[doc = "The list of available operations for Data Lake Analytics."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaLogSpecification {
    #[doc = "The name for OperationMetaLogSpecification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The displayName for OperationMetaLogSpecification."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The blobDuration for OperationMetaLogSpecification."]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl OperationMetaLogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaMetricAvailabilitiesSpecification {
    #[doc = "The timegrain for OperationMetaMetricAvailabilitiesSpecification."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "The blobDuration for OperationMetaMetricAvailabilitiesSpecification."]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl OperationMetaMetricAvailabilitiesSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaMetricSpecification {
    #[doc = "The name for OperationMetaMetricSpecification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The displayName for OperationMetaMetricSpecification."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "The displayName for OperationMetaMetricSpecification."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The unit for OperationMetaMetricSpecification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The aggregationType for OperationMetaMetricSpecification."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "The availabilities for OperationMetaMetricSpecification."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub availabilities: Vec<OperationMetaMetricAvailabilitiesSpecification>,
}
impl OperationMetaMetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaPropertyInfo {
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<OperationMetaServiceSpecification>,
}
impl OperationMetaPropertyInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetaServiceSpecification {
    #[doc = "The metricSpecifications for OperationMetaServiceSpecification."]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<OperationMetaMetricSpecification>,
    #[doc = "The logSpecifications for OperationMetaServiceSpecification."]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<OperationMetaLogSpecification>,
}
impl OperationMetaServiceSpecification {
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
#[doc = "SAS token information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SasTokenInformation {
    #[doc = "The access token for the associated Azure Storage Container."]
    #[serde(rename = "accessToken", default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}
impl SasTokenInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SAS response that contains the storage account, container and associated SAS token for connection use."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SasTokenInformationListResult {
    #[doc = "The results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SasTokenInformation>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SasTokenInformationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SasTokenInformationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Storage account information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountInformation {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The Azure Storage account properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAccountInformationProperties>,
}
impl StorageAccountInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Storage account list information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountInformationListResult {
    #[doc = "The results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageAccountInformation>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StorageAccountInformationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StorageAccountInformationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Azure Storage account properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAccountInformationProperties {
    #[doc = "The optional suffix for the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
impl StorageAccountInformationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Storage blob container information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageContainer {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Azure Storage blob container properties information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageContainerProperties>,
}
impl StorageContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of blob containers associated with the storage account attached to the Data Lake Analytics account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageContainerListResult {
    #[doc = "The results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageContainer>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StorageContainerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StorageContainerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Storage blob container properties information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageContainerProperties {
    #[doc = "The last modified time of the blob container."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
}
impl StorageContainerProperties {
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
#[doc = "The parameters used to update a compute policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateComputePolicyParameters {
    #[doc = "The compute policy properties to use when updating a compute policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateComputePolicyProperties>,
}
impl UpdateComputePolicyParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The compute policy properties to use when updating a compute policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateComputePolicyProperties {
    #[doc = "The AAD object identifier for the entity to create a policy for."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "The type of AAD object the object identifier refers to."]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<update_compute_policy_properties::ObjectType>,
    #[doc = "The maximum degree of parallelism per job this user can use to submit jobs. This property, the min priority per job property, or both must be passed."]
    #[serde(rename = "maxDegreeOfParallelismPerJob", default, skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism_per_job: Option<i32>,
    #[doc = "The minimum priority per job this user can use to submit jobs. This property, the max degree of parallelism per job property, or both must be passed."]
    #[serde(rename = "minPriorityPerJob", default, skip_serializing_if = "Option::is_none")]
    pub min_priority_per_job: Option<i32>,
}
impl UpdateComputePolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_compute_policy_properties {
    use super::*;
    #[doc = "The type of AAD object the object identifier refers to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ObjectType")]
    pub enum ObjectType {
        User,
        Group,
        ServicePrincipal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ObjectType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ObjectType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ObjectType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("ObjectType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("ObjectType", 1u32, "Group"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("ObjectType", 2u32, "ServicePrincipal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters used to update a compute policy while updating a Data Lake Analytics account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateComputePolicyWithAccountParameters {
    #[doc = "The unique name of the compute policy to update."]
    pub name: String,
    #[doc = "The compute policy properties to use when updating a compute policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateComputePolicyProperties>,
}
impl UpdateComputePolicyWithAccountParameters {
    pub fn new(name: String) -> Self {
        Self { name, properties: None }
    }
}
#[doc = "The parameters that can be used to update an existing Data Lake Analytics account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateDataLakeAnalyticsAccountParameters {
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The properties to update that are associated with an underlying Data Lake Analytics account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateDataLakeAnalyticsAccountProperties>,
}
impl UpdateDataLakeAnalyticsAccountParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties to update that are associated with an underlying Data Lake Analytics account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateDataLakeAnalyticsAccountProperties {
    #[doc = "The list of Data Lake Store accounts associated with this account."]
    #[serde(rename = "dataLakeStoreAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub data_lake_store_accounts: Vec<UpdateDataLakeStoreWithAccountParameters>,
    #[doc = "The list of Azure Blob storage accounts associated with this account."]
    #[serde(rename = "storageAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_accounts: Vec<UpdateStorageAccountWithAccountParameters>,
    #[doc = "The list of compute policies associated with this account."]
    #[serde(rename = "computePolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub compute_policies: Vec<UpdateComputePolicyWithAccountParameters>,
    #[doc = "The list of firewall rules associated with this account."]
    #[serde(rename = "firewallRules", default, skip_serializing_if = "Vec::is_empty")]
    pub firewall_rules: Vec<UpdateFirewallRuleWithAccountParameters>,
    #[doc = "The current state of the IP address firewall for this account. Disabling the firewall does not remove existing rules, they will just be ignored until the firewall is re-enabled."]
    #[serde(rename = "firewallState", default, skip_serializing_if = "Option::is_none")]
    pub firewall_state: Option<update_data_lake_analytics_account_properties::FirewallState>,
    #[doc = "The current state of allowing or disallowing IPs originating within Azure through the firewall. If the firewall is disabled, this is not enforced."]
    #[serde(rename = "firewallAllowAzureIps", default, skip_serializing_if = "Option::is_none")]
    pub firewall_allow_azure_ips: Option<update_data_lake_analytics_account_properties::FirewallAllowAzureIps>,
    #[doc = "The commitment tier to use for next month."]
    #[serde(rename = "newTier", default, skip_serializing_if = "Option::is_none")]
    pub new_tier: Option<update_data_lake_analytics_account_properties::NewTier>,
    #[doc = "The maximum supported jobs running under the account at the same time."]
    #[serde(rename = "maxJobCount", default, skip_serializing_if = "Option::is_none")]
    pub max_job_count: Option<i32>,
    #[doc = "The maximum supported degree of parallelism for this account."]
    #[serde(rename = "maxDegreeOfParallelism", default, skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism: Option<i32>,
    #[doc = "The maximum supported degree of parallelism per job for this account."]
    #[serde(rename = "maxDegreeOfParallelismPerJob", default, skip_serializing_if = "Option::is_none")]
    pub max_degree_of_parallelism_per_job: Option<i32>,
    #[doc = "The minimum supported priority per job for this account."]
    #[serde(rename = "minPriorityPerJob", default, skip_serializing_if = "Option::is_none")]
    pub min_priority_per_job: Option<i32>,
    #[doc = "The number of days that job metadata is retained."]
    #[serde(rename = "queryStoreRetention", default, skip_serializing_if = "Option::is_none")]
    pub query_store_retention: Option<i32>,
}
impl UpdateDataLakeAnalyticsAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_data_lake_analytics_account_properties {
    use super::*;
    #[doc = "The current state of the IP address firewall for this account. Disabling the firewall does not remove existing rules, they will just be ignored until the firewall is re-enabled."]
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
    #[doc = "The commitment tier to use for next month."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTier {
        Consumption,
        #[serde(rename = "Commitment_100AUHours")]
        Commitment100auHours,
        #[serde(rename = "Commitment_500AUHours")]
        Commitment500auHours,
        #[serde(rename = "Commitment_1000AUHours")]
        Commitment1000auHours,
        #[serde(rename = "Commitment_5000AUHours")]
        Commitment5000auHours,
        #[serde(rename = "Commitment_10000AUHours")]
        Commitment10000auHours,
        #[serde(rename = "Commitment_50000AUHours")]
        Commitment50000auHours,
        #[serde(rename = "Commitment_100000AUHours")]
        Commitment100000auHours,
        #[serde(rename = "Commitment_500000AUHours")]
        Commitment500000auHours,
    }
}
#[doc = "The Data Lake Store account properties to use when updating a Data Lake Store account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateDataLakeStoreProperties {
    #[doc = "The optional suffix for the Data Lake Store account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
impl UpdateDataLakeStoreProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters used to update a Data Lake Store account while updating a Data Lake Analytics account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDataLakeStoreWithAccountParameters {
    #[doc = "The unique name of the Data Lake Store account to update."]
    pub name: String,
    #[doc = "The Data Lake Store account properties to use when updating a Data Lake Store account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateDataLakeStoreProperties>,
}
impl UpdateDataLakeStoreWithAccountParameters {
    pub fn new(name: String) -> Self {
        Self { name, properties: None }
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
#[doc = "The parameters used to update a firewall rule while updating a Data Lake Analytics account."]
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
#[doc = "The parameters used to update an Azure Storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateStorageAccountParameters {
    #[doc = "The Azure Storage account properties to use when updating an Azure Storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateStorageAccountProperties>,
}
impl UpdateStorageAccountParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Azure Storage account properties to use when updating an Azure Storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateStorageAccountProperties {
    #[doc = "The updated access key associated with this Azure Storage account that will be used to connect to it."]
    #[serde(rename = "accessKey", default, skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    #[doc = "The optional suffix for the storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}
impl UpdateStorageAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters used to update an Azure Storage account while updating a Data Lake Analytics account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateStorageAccountWithAccountParameters {
    #[doc = "The unique name of the Azure Storage account to update."]
    pub name: String,
    #[doc = "The Azure Storage account properties to use when updating an Azure Storage account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateStorageAccountProperties>,
}
impl UpdateStorageAccountWithAccountParameters {
    pub fn new(name: String) -> Self {
        Self { name, properties: None }
    }
}
#[doc = "Data Lake Analytics  VirtualNetwork Rule information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkRule {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The VirtualNetwork Rule properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkRuleProperties>,
}
impl VirtualNetworkRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Analytics VirtualNetwork rule list information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkRuleListResult {
    #[doc = "The results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetworkRule>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl VirtualNetworkRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The VirtualNetwork Rule properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkRuleProperties {
    #[doc = "The resource identifier for the subnet"]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "The current state of the VirtualNetworkRule for this account."]
    #[serde(rename = "virtualNetworkRuleState", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_rule_state: Option<VirtualNetworkRuleState>,
}
impl VirtualNetworkRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current state of the VirtualNetworkRule for this account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VirtualNetworkRuleState {
    Active,
    NetworkSourceDeleted,
    Failed,
}
