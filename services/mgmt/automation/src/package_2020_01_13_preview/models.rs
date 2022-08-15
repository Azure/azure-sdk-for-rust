#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Definition of the activity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Activity {
    #[doc = "Gets or sets the id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the name of the activity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties of the activity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActivityProperties>,
}
impl Activity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for the list activity operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityListResult {
    #[doc = "Gets or sets a list of activities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Activity>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ActivityListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ActivityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the activity output type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityOutputType {
    #[doc = "Gets or sets the name of the activity output type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the activity output type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ActivityOutputType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the activity parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityParameter {
    #[doc = "Gets or sets the name of the activity parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the activity parameter."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets a Boolean value that indicates true if the parameter is required. If the value is false, the parameter is optional."]
    #[serde(rename = "isMandatory", default, skip_serializing_if = "Option::is_none")]
    pub is_mandatory: Option<bool>,
    #[doc = "Gets or sets a Boolean value that indicates true if the parameter is dynamic."]
    #[serde(rename = "isDynamic", default, skip_serializing_if = "Option::is_none")]
    pub is_dynamic: Option<bool>,
    #[doc = "Gets or sets the position of the activity parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    #[doc = "Gets or sets a Boolean value that indicates true if the parameter can take values from the incoming pipeline objects. This setting is used if the cmdlet must access the complete input object. false indicates that the parameter cannot take values from the complete input object."]
    #[serde(rename = "valueFromPipeline", default, skip_serializing_if = "Option::is_none")]
    pub value_from_pipeline: Option<bool>,
    #[doc = "Gets or sets a Boolean value that indicates true if the parameter can be filled from a property of the incoming pipeline object that has the same name as this parameter. false indicates that the parameter cannot be filled from the incoming pipeline object property with the same name. "]
    #[serde(rename = "valueFromPipelineByPropertyName", default, skip_serializing_if = "Option::is_none")]
    pub value_from_pipeline_by_property_name: Option<bool>,
    #[doc = "Gets or sets a Boolean value that indicates true if the cmdlet parameter accepts all the remaining command-line arguments that are associated with this parameter in the form of an array. false if the cmdlet parameter does not accept all the remaining argument values."]
    #[serde(rename = "valueFromRemainingArguments", default, skip_serializing_if = "Option::is_none")]
    pub value_from_remaining_arguments: Option<bool>,
    #[doc = "Gets or sets the description of the activity parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets the validation set of activity parameter."]
    #[serde(rename = "validationSet", default, skip_serializing_if = "Vec::is_empty")]
    pub validation_set: Vec<ActivityParameterValidationSet>,
}
impl ActivityParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the activity parameter set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityParameterSet {
    #[doc = "Gets or sets the name of the activity parameter set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the parameters of the activity parameter set."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ActivityParameter>,
}
impl ActivityParameterSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the activity parameter validation set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityParameterValidationSet {
    #[doc = "Gets or sets the name of the activity parameter validation set member."]
    #[serde(rename = "memberValue", default, skip_serializing_if = "Option::is_none")]
    pub member_value: Option<String>,
}
impl ActivityParameterValidationSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the activity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityProperties {
    #[doc = "Gets or sets the user name of the activity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[doc = "Gets or sets the parameter sets of the activity."]
    #[serde(rename = "parameterSets", default, skip_serializing_if = "Vec::is_empty")]
    pub parameter_sets: Vec<ActivityParameterSet>,
    #[doc = "Gets or sets the output types of the activity."]
    #[serde(rename = "outputTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub output_types: Vec<ActivityOutputType>,
    #[doc = "Gets or sets the creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ActivityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the create Advanced Schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdvancedSchedule {
    #[doc = "Days of the week that the job should execute on."]
    #[serde(rename = "weekDays", default, skip_serializing_if = "Vec::is_empty")]
    pub week_days: Vec<String>,
    #[doc = "Days of the month that the job should execute on. Must be between 1 and 31."]
    #[serde(rename = "monthDays", default, skip_serializing_if = "Vec::is_empty")]
    pub month_days: Vec<i32>,
    #[doc = "Occurrences of days within a month."]
    #[serde(rename = "monthlyOccurrences", default, skip_serializing_if = "Vec::is_empty")]
    pub monthly_occurrences: Vec<AdvancedScheduleMonthlyOccurrence>,
}
impl AdvancedSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the create advanced schedule monthly occurrence."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdvancedScheduleMonthlyOccurrence {
    #[doc = "Occurrence of the week within the month. Must be between 1 and 5"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occurrence: Option<i32>,
    #[doc = "Day of the occurrence. Must be one of monday, tuesday, wednesday, thursday, friday, saturday, sunday."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<advanced_schedule_monthly_occurrence::Day>,
}
impl AdvancedScheduleMonthlyOccurrence {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod advanced_schedule_monthly_occurrence {
    use super::*;
    #[doc = "Day of the occurrence. Must be one of monday, tuesday, wednesday, thursday, friday, saturday, sunday."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Day")]
    pub enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Day {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Day {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Day {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Monday => serializer.serialize_unit_variant("Day", 0u32, "Monday"),
                Self::Tuesday => serializer.serialize_unit_variant("Day", 1u32, "Tuesday"),
                Self::Wednesday => serializer.serialize_unit_variant("Day", 2u32, "Wednesday"),
                Self::Thursday => serializer.serialize_unit_variant("Day", 3u32, "Thursday"),
                Self::Friday => serializer.serialize_unit_variant("Day", 4u32, "Friday"),
                Self::Saturday => serializer.serialize_unit_variant("Day", 5u32, "Saturday"),
                Self::Sunday => serializer.serialize_unit_variant("Day", 6u32, "Sunday"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Definition of the agent registration information type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentRegistration {
    #[doc = "Gets or sets the dsc meta configuration."]
    #[serde(rename = "dscMetaConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub dsc_meta_configuration: Option<String>,
    #[doc = "Gets or sets the dsc server endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "Definition of the agent registration keys."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<AgentRegistrationKeys>,
    #[doc = "Gets or sets the id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl AgentRegistration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the agent registration keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentRegistrationKeys {
    #[doc = "Gets or sets the primary key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
    #[doc = "Gets or sets the secondary key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
}
impl AgentRegistrationKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the regenerate keys operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentRegistrationRegenerateKeyParameter {
    #[doc = "Gets or sets the agent registration key name - primary or secondary."]
    #[serde(rename = "keyName")]
    pub key_name: agent_registration_regenerate_key_parameter::KeyName,
}
impl AgentRegistrationRegenerateKeyParameter {
    pub fn new(key_name: agent_registration_regenerate_key_parameter::KeyName) -> Self {
        Self { key_name }
    }
}
pub mod agent_registration_regenerate_key_parameter {
    use super::*;
    #[doc = "Gets or sets the agent registration key name - primary or secondary."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeyName")]
    pub enum KeyName {
        #[serde(rename = "primary")]
        Primary,
        #[serde(rename = "secondary")]
        Secondary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KeyName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KeyName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KeyName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("KeyName", 0u32, "primary"),
                Self::Secondary => serializer.serialize_unit_variant("KeyName", 1u32, "secondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Definition of the automation account type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationAccount {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Definition of the account property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutomationAccountProperties>,
    #[doc = "Gets or sets the etag of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl AutomationAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create or update automation account operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationAccountCreateOrUpdateParameters {
    #[doc = "The parameters supplied to the create or update account properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutomationAccountCreateOrUpdateProperties>,
    #[doc = "Gets or sets name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Gets or sets the tags attached to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AutomationAccountCreateOrUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create or update account properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationAccountCreateOrUpdateProperties {
    #[doc = "The account SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The encryption settings for automation account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionProperties>,
    #[doc = "Indicates whether traffic on the non-ARM endpoint (Webhook/Agent) is allowed from the public internet"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<bool>,
}
impl AutomationAccountCreateOrUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for the list account operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationAccountListResult {
    #[doc = "Gets or sets list of accounts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AutomationAccount>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AutomationAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AutomationAccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the account property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationAccountProperties {
    #[doc = "The account SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Gets or sets the last modified by."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "Gets status of account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<automation_account_properties::State>,
    #[doc = "Gets the creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The encryption settings for automation account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionProperties>,
    #[doc = "List of Automation operations supported by the Automation resource provider."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Indicates whether traffic on the non-ARM endpoint (Webhook/Agent) is allowed from the public internet"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<bool>,
}
impl AutomationAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod automation_account_properties {
    use super::*;
    #[doc = "Gets status of account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Ok,
        Unavailable,
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
                Self::Ok => serializer.serialize_unit_variant("State", 0u32, "Ok"),
                Self::Unavailable => serializer.serialize_unit_variant("State", 1u32, "Unavailable"),
                Self::Suspended => serializer.serialize_unit_variant("State", 2u32, "Suspended"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters supplied to the update automation account operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationAccountUpdateParameters {
    #[doc = "The parameters supplied to the update account properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutomationAccountUpdateProperties>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Gets or sets the tags attached to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AutomationAccountUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the update account properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomationAccountUpdateProperties {
    #[doc = "The account SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The encryption settings for automation account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionProperties>,
    #[doc = "Indicates whether traffic on the non-ARM endpoint (Webhook/Agent) is allowed from the public internet"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<bool>,
}
impl AutomationAccountUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure query for the update configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureQueryProperties {
    #[doc = "List of Subscription or Resource Group ARM Ids."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scope: Vec<String>,
    #[doc = "List of locations to scope the query to."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "Tag filter information for the VM."]
    #[serde(rename = "tagSettings", default, skip_serializing_if = "Option::is_none")]
    pub tag_settings: Option<TagSettingsProperties>,
}
impl AzureQueryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Certificate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificateProperties>,
}
impl Certificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create or update or replace certificate operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateCreateOrUpdateParameters {
    #[doc = "Gets or sets the name of the certificate."]
    pub name: String,
    #[doc = "The properties of the create certificate operation."]
    pub properties: CertificateCreateOrUpdateProperties,
}
impl CertificateCreateOrUpdateParameters {
    pub fn new(name: String, properties: CertificateCreateOrUpdateProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "The properties of the create certificate operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateCreateOrUpdateProperties {
    #[doc = "Gets or sets the base64 encoded value of the certificate."]
    #[serde(rename = "base64Value")]
    pub base64_value: String,
    #[doc = "Gets or sets the description of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets the thumbprint of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "Gets or sets the is exportable flag of the certificate."]
    #[serde(rename = "isExportable", default, skip_serializing_if = "Option::is_none")]
    pub is_exportable: Option<bool>,
}
impl CertificateCreateOrUpdateProperties {
    pub fn new(base64_value: String) -> Self {
        Self {
            base64_value,
            description: None,
            thumbprint: None,
            is_exportable: None,
        }
    }
}
#[doc = "The response model for the list certificate operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateListResult {
    #[doc = "Gets or sets a list of certificates."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Certificate>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CertificateListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CertificateListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateProperties {
    #[doc = "Gets the thumbprint of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "Gets the expiry time of the certificate."]
    #[serde(rename = "expiryTime", with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the is exportable flag of the certificate."]
    #[serde(rename = "isExportable", default, skip_serializing_if = "Option::is_none")]
    pub is_exportable: Option<bool>,
    #[doc = "Gets the creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl CertificateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the update certificate operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateUpdateParameters {
    #[doc = "Gets or sets the name of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The properties of the update certificate operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificateUpdateProperties>,
}
impl CertificateUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the update certificate operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateUpdateProperties {
    #[doc = "Gets or sets the description of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl CertificateUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Connection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Definition of the connection properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConnectionProperties>,
}
impl Connection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create or update connection operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionCreateOrUpdateParameters {
    #[doc = "Gets or sets the name of the connection."]
    pub name: String,
    #[doc = "The properties of the create connection properties"]
    pub properties: ConnectionCreateOrUpdateProperties,
}
impl ConnectionCreateOrUpdateParameters {
    pub fn new(name: String, properties: ConnectionCreateOrUpdateProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "The properties of the create connection properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionCreateOrUpdateProperties {
    #[doc = "Gets or sets the description of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The connection type property associated with the entity."]
    #[serde(rename = "connectionType")]
    pub connection_type: ConnectionTypeAssociationProperty,
    #[doc = "Gets or sets the field definition properties of the connection."]
    #[serde(rename = "fieldDefinitionValues", default, skip_serializing_if = "Option::is_none")]
    pub field_definition_values: Option<serde_json::Value>,
}
impl ConnectionCreateOrUpdateProperties {
    pub fn new(connection_type: ConnectionTypeAssociationProperty) -> Self {
        Self {
            description: None,
            connection_type,
            field_definition_values: None,
        }
    }
}
#[doc = "The response model for the list connection operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionListResult {
    #[doc = "Gets or sets a list of connection."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Connection>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the connection properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionProperties {
    #[doc = "The connection type property associated with the entity."]
    #[serde(rename = "connectionType", default, skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<ConnectionTypeAssociationProperty>,
    #[doc = "Gets the field definition values of the connection."]
    #[serde(rename = "fieldDefinitionValues", default, skip_serializing_if = "Option::is_none")]
    pub field_definition_values: Option<serde_json::Value>,
    #[doc = "Gets the creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the connection type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionType {
    #[doc = "Gets the id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the name of the connection type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties of the connection type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConnectionTypeProperties>,
}
impl ConnectionType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The connection type property associated with the entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionTypeAssociationProperty {
    #[doc = "Gets or sets the name of the connection type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ConnectionTypeAssociationProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create or update connection type operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionTypeCreateOrUpdateParameters {
    #[doc = "Gets or sets the name of the connection type."]
    pub name: String,
    #[doc = "The properties of the create connection type."]
    pub properties: ConnectionTypeCreateOrUpdateProperties,
}
impl ConnectionTypeCreateOrUpdateParameters {
    pub fn new(name: String, properties: ConnectionTypeCreateOrUpdateProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "The properties of the create connection type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionTypeCreateOrUpdateProperties {
    #[doc = "Gets or sets a Boolean value to indicate if the connection type is global."]
    #[serde(rename = "isGlobal", default, skip_serializing_if = "Option::is_none")]
    pub is_global: Option<bool>,
    #[doc = "Gets or sets the field definitions of the connection type."]
    #[serde(rename = "fieldDefinitions")]
    pub field_definitions: serde_json::Value,
}
impl ConnectionTypeCreateOrUpdateProperties {
    pub fn new(field_definitions: serde_json::Value) -> Self {
        Self {
            is_global: None,
            field_definitions,
        }
    }
}
#[doc = "The response model for the list connection type operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionTypeListResult {
    #[doc = "Gets or sets a list of connection types."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConnectionType>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConnectionTypeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ConnectionTypeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the connection type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionTypeProperties {
    #[doc = "Gets or sets a Boolean value to indicate if the connection type is global."]
    #[serde(rename = "isGlobal", default, skip_serializing_if = "Option::is_none")]
    pub is_global: Option<bool>,
    #[doc = "Gets the field definitions of the connection type."]
    #[serde(rename = "fieldDefinitions", default, skip_serializing_if = "Option::is_none")]
    pub field_definitions: Option<serde_json::Value>,
    #[doc = "Gets the creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ConnectionTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the update connection operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionUpdateParameters {
    #[doc = "Gets or sets the name of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The properties of the update connection operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConnectionUpdateProperties>,
}
impl ConnectionUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the update connection operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionUpdateProperties {
    #[doc = "Gets or sets the description of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets the field definition values of the connection."]
    #[serde(rename = "fieldDefinitionValues", default, skip_serializing_if = "Option::is_none")]
    pub field_definition_values: Option<serde_json::Value>,
}
impl ConnectionUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the runbook property type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentHash {
    #[doc = "Gets or sets the content hash algorithm used to hash the content."]
    pub algorithm: String,
    #[doc = "Gets or sets expected hash value of the content."]
    pub value: String,
}
impl ContentHash {
    pub fn new(algorithm: String, value: String) -> Self {
        Self { algorithm, value }
    }
}
#[doc = "Definition of the content link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContentLink {
    #[doc = "Gets or sets the uri of the runbook content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Definition of the runbook property type."]
    #[serde(rename = "contentHash", default, skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<ContentHash>,
    #[doc = "Gets or sets the version of the content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ContentLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the content source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContentSource {
    #[doc = "Definition of the runbook property type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<ContentHash>,
    #[doc = "Gets or sets the content source type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<content_source::Type>,
    #[doc = "Gets or sets the value of the content. This is based on the content source type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Gets or sets the version of the content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ContentSource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod content_source {
    use super::*;
    #[doc = "Gets or sets the content source type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "embeddedContent")]
        EmbeddedContent,
        #[serde(rename = "uri")]
        Uri,
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
                Self::EmbeddedContent => serializer.serialize_unit_variant("Type", 0u32, "embeddedContent"),
                Self::Uri => serializer.serialize_unit_variant("Type", 1u32, "uri"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Definition of the credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Credential {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Definition of the credential properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CredentialProperties>,
}
impl Credential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create or update credential operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialCreateOrUpdateParameters {
    #[doc = "Gets or sets the name of the credential."]
    pub name: String,
    #[doc = "The properties of the create credential operation."]
    pub properties: CredentialCreateOrUpdateProperties,
}
impl CredentialCreateOrUpdateParameters {
    pub fn new(name: String, properties: CredentialCreateOrUpdateProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "The properties of the create credential operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialCreateOrUpdateProperties {
    #[doc = "Gets or sets the user name of the credential."]
    #[serde(rename = "userName")]
    pub user_name: String,
    #[doc = "Gets or sets the password of the credential."]
    pub password: String,
    #[doc = "Gets or sets the description of the credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl CredentialCreateOrUpdateProperties {
    pub fn new(user_name: String, password: String) -> Self {
        Self {
            user_name,
            password,
            description: None,
        }
    }
}
#[doc = "The response model for the list credential operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CredentialListResult {
    #[doc = "Gets or sets a list of credentials."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Credential>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CredentialListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CredentialListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the credential properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CredentialProperties {
    #[doc = "Gets the user name of the credential."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "Gets the creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl CredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the Update credential operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CredentialUpdateParameters {
    #[doc = "Gets or sets the name of the credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The properties of the Update credential"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CredentialUpdateProperties>,
}
impl CredentialUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the Update credential"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CredentialUpdateProperties {
    #[doc = "Gets or sets the user name of the credential."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "Gets or sets the password of the credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Gets or sets the description of the credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl CredentialUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the Dsc Compilation job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscCompilationJob {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Definition of Dsc Compilation job properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DscCompilationJobProperties>,
}
impl DscCompilationJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create compilation job operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DscCompilationJobCreateParameters {
    #[doc = "The parameters supplied to the create compilation job operation."]
    pub properties: DscCompilationJobCreateProperties,
    #[doc = "Gets or sets name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets the tags attached to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DscCompilationJobCreateParameters {
    pub fn new(properties: DscCompilationJobCreateProperties) -> Self {
        Self {
            properties,
            name: None,
            location: None,
            tags: None,
        }
    }
}
#[doc = "The parameters supplied to the create compilation job operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DscCompilationJobCreateProperties {
    #[doc = "The Dsc configuration property associated with the entity."]
    pub configuration: DscConfigurationAssociationProperty,
    #[doc = "Gets or sets the parameters of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "If a new build version of NodeConfiguration is required."]
    #[serde(rename = "incrementNodeConfigurationBuild", default, skip_serializing_if = "Option::is_none")]
    pub increment_node_configuration_build: Option<bool>,
}
impl DscCompilationJobCreateProperties {
    pub fn new(configuration: DscConfigurationAssociationProperty) -> Self {
        Self {
            configuration,
            parameters: None,
            increment_node_configuration_build: None,
        }
    }
}
#[doc = "The response model for the list job operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscCompilationJobListResult {
    #[doc = "Gets or sets a list of Dsc Compilation jobs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DscCompilationJob>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DscCompilationJobListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DscCompilationJobListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of Dsc Compilation job properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscCompilationJobProperties {
    #[doc = "The Dsc configuration property associated with the entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<DscConfigurationAssociationProperty>,
    #[doc = "Gets the compilation job started by."]
    #[serde(rename = "startedBy", default, skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    #[doc = "Gets the id of the job."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "Gets the creation time of the job."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<JobProvisioningStateProperty>,
    #[doc = "Gets or sets the runOn which specifies the group name where the job is to be executed."]
    #[serde(rename = "runOn", default, skip_serializing_if = "Option::is_none")]
    pub run_on: Option<String>,
    #[doc = "Gets or sets the status of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<dsc_compilation_job_properties::Status>,
    #[doc = "Gets or sets the status details of the job."]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "Gets the start time of the job."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the end time of the job."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the exception of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exception: Option<String>,
    #[doc = "Gets the last modified time of the job."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the last status modified time of the job."]
    #[serde(rename = "lastStatusModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_status_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the parameters of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl DscCompilationJobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dsc_compilation_job_properties {
    use super::*;
    #[doc = "Gets or sets the status of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        New,
        Activating,
        Running,
        Completed,
        Failed,
        Stopped,
        Blocked,
        Suspended,
        Disconnected,
        Suspending,
        Stopping,
        Resuming,
        Removing,
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
                Self::New => serializer.serialize_unit_variant("Status", 0u32, "New"),
                Self::Activating => serializer.serialize_unit_variant("Status", 1u32, "Activating"),
                Self::Running => serializer.serialize_unit_variant("Status", 2u32, "Running"),
                Self::Completed => serializer.serialize_unit_variant("Status", 3u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("Status", 4u32, "Failed"),
                Self::Stopped => serializer.serialize_unit_variant("Status", 5u32, "Stopped"),
                Self::Blocked => serializer.serialize_unit_variant("Status", 6u32, "Blocked"),
                Self::Suspended => serializer.serialize_unit_variant("Status", 7u32, "Suspended"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 8u32, "Disconnected"),
                Self::Suspending => serializer.serialize_unit_variant("Status", 9u32, "Suspending"),
                Self::Stopping => serializer.serialize_unit_variant("Status", 10u32, "Stopping"),
                Self::Resuming => serializer.serialize_unit_variant("Status", 11u32, "Resuming"),
                Self::Removing => serializer.serialize_unit_variant("Status", 12u32, "Removing"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Definition of the configuration type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscConfiguration {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Definition of the configuration property type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DscConfigurationProperties>,
    #[doc = "Gets or sets the etag of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl DscConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Dsc configuration property associated with the entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscConfigurationAssociationProperty {
    #[doc = "Gets or sets the name of the Dsc configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl DscConfigurationAssociationProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create or update configuration operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DscConfigurationCreateOrUpdateParameters {
    #[doc = "The properties to create or update configuration."]
    pub properties: DscConfigurationCreateOrUpdateProperties,
    #[doc = "Gets or sets name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets the tags attached to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DscConfigurationCreateOrUpdateParameters {
    pub fn new(properties: DscConfigurationCreateOrUpdateProperties) -> Self {
        Self {
            properties,
            name: None,
            location: None,
            tags: None,
        }
    }
}
#[doc = "The properties to create or update configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DscConfigurationCreateOrUpdateProperties {
    #[doc = "Gets or sets verbose log option."]
    #[serde(rename = "logVerbose", default, skip_serializing_if = "Option::is_none")]
    pub log_verbose: Option<bool>,
    #[doc = "Gets or sets progress log option."]
    #[serde(rename = "logProgress", default, skip_serializing_if = "Option::is_none")]
    pub log_progress: Option<bool>,
    #[doc = "Definition of the content source."]
    pub source: ContentSource,
    #[doc = "Gets or sets the configuration parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Gets or sets the description of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl DscConfigurationCreateOrUpdateProperties {
    pub fn new(source: ContentSource) -> Self {
        Self {
            log_verbose: None,
            log_progress: None,
            source,
            parameters: None,
            description: None,
        }
    }
}
#[doc = "The response model for the list configuration operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscConfigurationListResult {
    #[doc = "Gets or sets a list of configurations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DscConfiguration>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Gets the total number of configurations matching filter criteria."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}
impl azure_core::Continuable for DscConfigurationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DscConfigurationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the configuration parameter type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscConfigurationParameter {
    #[doc = "Gets or sets the type of the parameter."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets a Boolean value to indicate whether the parameter is mandatory or not."]
    #[serde(rename = "isMandatory", default, skip_serializing_if = "Option::is_none")]
    pub is_mandatory: Option<bool>,
    #[doc = "Get or sets the position of the parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[doc = "Gets or sets the default value of parameter."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}
impl DscConfigurationParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the configuration property type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscConfigurationProperties {
    #[doc = "Gets or sets the provisioning state of the configuration."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<dsc_configuration_properties::ProvisioningState>,
    #[doc = "Gets or sets the job count of the configuration."]
    #[serde(rename = "jobCount", default, skip_serializing_if = "Option::is_none")]
    pub job_count: Option<i32>,
    #[doc = "Gets or sets the configuration parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Definition of the content source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<ContentSource>,
    #[doc = "Gets or sets the state of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<dsc_configuration_properties::State>,
    #[doc = "Gets or sets verbose log option."]
    #[serde(rename = "logVerbose", default, skip_serializing_if = "Option::is_none")]
    pub log_verbose: Option<bool>,
    #[doc = "Gets or sets the creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the number of compiled node configurations."]
    #[serde(rename = "nodeConfigurationCount", default, skip_serializing_if = "Option::is_none")]
    pub node_configuration_count: Option<i64>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl DscConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dsc_configuration_properties {
    use super::*;
    #[doc = "Gets or sets the provisioning state of the configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
    }
    #[doc = "Gets or sets the state of the configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        New,
        Edit,
        Published,
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
                Self::New => serializer.serialize_unit_variant("State", 0u32, "New"),
                Self::Edit => serializer.serialize_unit_variant("State", 1u32, "Edit"),
                Self::Published => serializer.serialize_unit_variant("State", 2u32, "Published"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters supplied to the create or update configuration operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscConfigurationUpdateParameters {
    #[doc = "The properties to create or update configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DscConfigurationCreateOrUpdateProperties>,
    #[doc = "Gets or sets name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the tags attached to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DscConfigurationUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the DSC Meta Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscMetaConfiguration {
    #[doc = "Gets or sets the ConfigurationModeFrequencyMins value of the meta configuration."]
    #[serde(rename = "configurationModeFrequencyMins", default, skip_serializing_if = "Option::is_none")]
    pub configuration_mode_frequency_mins: Option<i32>,
    #[doc = "Gets or sets the RebootNodeIfNeeded value of the meta configuration."]
    #[serde(rename = "rebootNodeIfNeeded", default, skip_serializing_if = "Option::is_none")]
    pub reboot_node_if_needed: Option<bool>,
    #[doc = "Gets or sets the ConfigurationMode value of the meta configuration."]
    #[serde(rename = "configurationMode", default, skip_serializing_if = "Option::is_none")]
    pub configuration_mode: Option<String>,
    #[doc = "Gets or sets the ActionAfterReboot value of the meta configuration."]
    #[serde(rename = "actionAfterReboot", default, skip_serializing_if = "Option::is_none")]
    pub action_after_reboot: Option<String>,
    #[doc = "Gets or sets the CertificateId value of the meta configuration."]
    #[serde(rename = "certificateId", default, skip_serializing_if = "Option::is_none")]
    pub certificate_id: Option<String>,
    #[doc = "Gets or sets the RefreshFrequencyMins value of the meta configuration."]
    #[serde(rename = "refreshFrequencyMins", default, skip_serializing_if = "Option::is_none")]
    pub refresh_frequency_mins: Option<i32>,
    #[doc = "Gets or sets the AllowModuleOverwrite value of the meta configuration."]
    #[serde(rename = "allowModuleOverwrite", default, skip_serializing_if = "Option::is_none")]
    pub allow_module_overwrite: Option<bool>,
}
impl DscMetaConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of a DscNode"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscNode {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a DscNode"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DscNodeProperties>,
}
impl DscNode {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the dsc node configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscNodeConfiguration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties for the DscNodeConfiguration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DscNodeConfigurationProperties>,
}
impl DscNodeConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The dsc node configuration property associated with the entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscNodeConfigurationAssociationProperty {
    #[doc = "Gets or sets the name of the dsc node configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl DscNodeConfigurationAssociationProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create or update node configuration operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscNodeConfigurationCreateOrUpdateParameters {
    #[doc = "The parameter properties supplied to the create or update node configuration operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DscNodeConfigurationCreateOrUpdateParametersProperties>,
    #[doc = "Name of the node configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the tags attached to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DscNodeConfigurationCreateOrUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameter properties supplied to the create or update node configuration operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DscNodeConfigurationCreateOrUpdateParametersProperties {
    #[doc = "Definition of the content source."]
    pub source: ContentSource,
    #[doc = "The Dsc configuration property associated with the entity."]
    pub configuration: DscConfigurationAssociationProperty,
    #[doc = "If a new build version of NodeConfiguration is required."]
    #[serde(rename = "incrementNodeConfigurationBuild", default, skip_serializing_if = "Option::is_none")]
    pub increment_node_configuration_build: Option<bool>,
}
impl DscNodeConfigurationCreateOrUpdateParametersProperties {
    pub fn new(source: ContentSource, configuration: DscConfigurationAssociationProperty) -> Self {
        Self {
            source,
            configuration,
            increment_node_configuration_build: None,
        }
    }
}
#[doc = "The response model for the list job operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscNodeConfigurationListResult {
    #[doc = "Gets or sets a list of Dsc node configurations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DscNodeConfiguration>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Gets or sets the total rows in query."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}
impl azure_core::Continuable for DscNodeConfigurationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DscNodeConfigurationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for the DscNodeConfiguration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscNodeConfigurationProperties {
    #[doc = "Gets or sets the last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The Dsc configuration property associated with the entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<DscConfigurationAssociationProperty>,
    #[doc = "Source of node configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Number of nodes with this node configuration assigned"]
    #[serde(rename = "nodeCount", default, skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i64>,
    #[doc = "If a new build version of NodeConfiguration is required."]
    #[serde(rename = "incrementNodeConfigurationBuild", default, skip_serializing_if = "Option::is_none")]
    pub increment_node_configuration_build: Option<bool>,
}
impl DscNodeConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The dsc extensionHandler property associated with the node"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscNodeExtensionHandlerAssociationProperty {
    #[doc = "Gets or sets the name of the extension handler."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the version of the extension handler."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl DscNodeExtensionHandlerAssociationProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for the list dsc nodes operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscNodeListResult {
    #[doc = "Gets or sets a list of dsc nodes."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DscNode>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Gets the total number of nodes matching filter criteria."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}
impl azure_core::Continuable for DscNodeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DscNodeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a DscNode"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscNodeProperties {
    #[doc = "Gets or sets the last seen time of the node."]
    #[serde(rename = "lastSeen", with = "azure_core::date::rfc3339::option")]
    pub last_seen: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the registration time of the node."]
    #[serde(rename = "registrationTime", with = "azure_core::date::rfc3339::option")]
    pub registration_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the ip of the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "Gets or sets the account id of the node."]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "The dsc node configuration property associated with the entity."]
    #[serde(rename = "nodeConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub node_configuration: Option<DscNodeConfigurationAssociationProperty>,
    #[doc = "Gets or sets the status of the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets the node id."]
    #[serde(rename = "nodeId", default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[doc = "Gets or sets the etag of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Gets the total number of records matching filter criteria."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "Gets or sets the list of extensionHandler properties for a Node."]
    #[serde(rename = "extensionHandler", default, skip_serializing_if = "Vec::is_empty")]
    pub extension_handler: Vec<DscNodeExtensionHandlerAssociationProperty>,
}
impl DscNodeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the dsc node report type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscNodeReport {
    #[doc = "Gets or sets the end time of the node report."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the lastModifiedTime of the node report."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the start time of the node report."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the type of the node report."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the id of the node report."]
    #[serde(rename = "reportId", default, skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    #[doc = "Gets or sets the status of the node report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets the refreshMode of the node report."]
    #[serde(rename = "refreshMode", default, skip_serializing_if = "Option::is_none")]
    pub refresh_mode: Option<String>,
    #[doc = "Gets or sets the rebootRequested of the node report."]
    #[serde(rename = "rebootRequested", default, skip_serializing_if = "Option::is_none")]
    pub reboot_requested: Option<String>,
    #[doc = "Gets or sets the reportFormatVersion of the node report."]
    #[serde(rename = "reportFormatVersion", default, skip_serializing_if = "Option::is_none")]
    pub report_format_version: Option<String>,
    #[doc = "Gets or sets the configurationVersion of the node report."]
    #[serde(rename = "configurationVersion", default, skip_serializing_if = "Option::is_none")]
    pub configuration_version: Option<String>,
    #[doc = "Gets or sets the id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the errors for the node report."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<DscReportError>,
    #[doc = "Gets or sets the resource for the node report."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<DscReportResource>,
    #[doc = "Definition of the DSC Meta Configuration."]
    #[serde(rename = "metaConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub meta_configuration: Option<DscMetaConfiguration>,
    #[doc = "Gets or sets the hostname of the node that sent the report."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "Gets or sets the IPv4 address of the node that sent the report."]
    #[serde(rename = "iPV4Addresses", default, skip_serializing_if = "Vec::is_empty")]
    pub i_pv4_addresses: Vec<String>,
    #[doc = "Gets or sets the IPv6 address of the node that sent the report."]
    #[serde(rename = "iPV6Addresses", default, skip_serializing_if = "Vec::is_empty")]
    pub i_pv6_addresses: Vec<String>,
    #[doc = "Gets or sets the number of resource in the node report."]
    #[serde(rename = "numberOfResources", default, skip_serializing_if = "Option::is_none")]
    pub number_of_resources: Option<i32>,
    #[doc = "Gets or sets the unparsed errors for the node report."]
    #[serde(rename = "rawErrors", default, skip_serializing_if = "Option::is_none")]
    pub raw_errors: Option<String>,
}
impl DscNodeReport {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for the list dsc nodes operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscNodeReportListResult {
    #[doc = "Gets or sets a list of dsc node reports."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DscNodeReport>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DscNodeReportListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DscNodeReportListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the update dsc node operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscNodeUpdateParameters {
    #[doc = "Gets or sets the id of the dsc node."]
    #[serde(rename = "nodeId", default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<dsc_node_update_parameters::Properties>,
}
impl DscNodeUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dsc_node_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The dsc node configuration property associated with the entity."]
        #[serde(rename = "nodeConfiguration", default, skip_serializing_if = "Option::is_none")]
        pub node_configuration: Option<DscNodeConfigurationAssociationProperty>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Definition of the dsc node report error type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscReportError {
    #[doc = "Gets or sets the source of the error."]
    #[serde(rename = "errorSource", default, skip_serializing_if = "Option::is_none")]
    pub error_source: Option<String>,
    #[doc = "Gets or sets the resource ID which generated the error."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Gets or sets the error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Gets or sets the error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Gets or sets the locale of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[doc = "Gets or sets the error details."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<String>,
}
impl DscReportError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the DSC Report Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscReportResource {
    #[doc = "Gets or sets the ID of the resource."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Gets or sets the source info of the resource."]
    #[serde(rename = "sourceInfo", default, skip_serializing_if = "Option::is_none")]
    pub source_info: Option<String>,
    #[doc = "Gets or sets the Resource Navigation values for resources the resource depends on."]
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<DscReportResourceNavigation>,
    #[doc = "Gets or sets the module name of the resource."]
    #[serde(rename = "moduleName", default, skip_serializing_if = "Option::is_none")]
    pub module_name: Option<String>,
    #[doc = "Gets or sets the module version of the resource."]
    #[serde(rename = "moduleVersion", default, skip_serializing_if = "Option::is_none")]
    pub module_version: Option<String>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Gets or sets the error of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "Gets or sets the status of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets the duration in seconds for the resource."]
    #[serde(rename = "durationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<f64>,
    #[doc = "Gets or sets the start date of the resource."]
    #[serde(rename = "startDate", with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
}
impl DscReportResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Navigation for DSC Report Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DscReportResourceNavigation {
    #[doc = "Gets or sets the ID of the resource to navigate to."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl DscReportResourceNavigation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The encryption settings for automation account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionProperties {
    #[doc = "Settings concerning key vault encryption for a configuration store."]
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
    #[doc = "Encryption Key Source"]
    #[serde(rename = "keySource", default, skip_serializing_if = "Option::is_none")]
    pub key_source: Option<encryption_properties::KeySource>,
    #[doc = "User identity used for CMK."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<encryption_properties::Identity>,
}
impl EncryptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption_properties {
    use super::*;
    #[doc = "Encryption Key Source"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeySource {
        #[serde(rename = "Microsoft.Automation")]
        MicrosoftAutomation,
        #[serde(rename = "Microsoft.Keyvault")]
        MicrosoftKeyvault,
    }
    #[doc = "User identity used for CMK."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Identity {
        #[doc = "The user identity used for CMK. It will be an ARM resource id in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
        #[serde(rename = "userAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
        pub user_assigned_identity: Option<serde_json::Value>,
    }
    impl Identity {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Error response of an operation failure"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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
#[doc = "Definition of the connection fields."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldDefinition {
    #[doc = "Gets or sets the isEncrypted flag of the connection field definition."]
    #[serde(rename = "isEncrypted", default, skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
    #[doc = "Gets or sets the isOptional flag of the connection field definition."]
    #[serde(rename = "isOptional", default, skip_serializing_if = "Option::is_none")]
    pub is_optional: Option<bool>,
    #[doc = "Gets or sets the type of the connection field definition."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl FieldDefinition {
    pub fn new(type_: String) -> Self {
        Self {
            is_encrypted: None,
            is_optional: None,
            type_,
        }
    }
}
pub type GroupIdsProperty = Vec<String>;
#[doc = "Definition of hybrid runbook worker."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridRunbookWorker {
    #[doc = "Gets or sets the worker machine name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the assigned machine IP address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "Gets or sets the registration time of the worker machine."]
    #[serde(rename = "registrationTime", with = "azure_core::date::rfc3339::option")]
    pub registration_time: Option<time::OffsetDateTime>,
    #[doc = "Last Heartbeat from the Worker"]
    #[serde(rename = "lastSeenDateTime", with = "azure_core::date::rfc3339::option")]
    pub last_seen_date_time: Option<time::OffsetDateTime>,
}
impl HybridRunbookWorker {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of hybrid runbook worker group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridRunbookWorkerGroup {
    #[doc = "Gets or sets the id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name of the group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the list of hybrid runbook workers."]
    #[serde(rename = "hybridRunbookWorkers", default, skip_serializing_if = "Vec::is_empty")]
    pub hybrid_runbook_workers: Vec<HybridRunbookWorker>,
    #[doc = "Definition of RunAs credential to use for hybrid worker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<RunAsCredentialAssociationProperty>,
    #[doc = "Type of the HybridWorkerGroup."]
    #[serde(rename = "groupType", default, skip_serializing_if = "Option::is_none")]
    pub group_type: Option<hybrid_runbook_worker_group::GroupType>,
}
impl HybridRunbookWorkerGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hybrid_runbook_worker_group {
    use super::*;
    #[doc = "Type of the HybridWorkerGroup."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "GroupType")]
    pub enum GroupType {
        User,
        System,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for GroupType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for GroupType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for GroupType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("GroupType", 0u32, "User"),
                Self::System => serializer.serialize_unit_variant("GroupType", 1u32, "System"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Parameters supplied to the update operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridRunbookWorkerGroupUpdateParameters {
    #[doc = "Definition of RunAs credential to use for hybrid worker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<RunAsCredentialAssociationProperty>,
}
impl HybridRunbookWorkerGroupUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for the list hybrid runbook worker groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridRunbookWorkerGroupsListResult {
    #[doc = "Gets or sets a list of hybrid runbook worker groups."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HybridRunbookWorkerGroup>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HybridRunbookWorkerGroupsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HybridRunbookWorkerGroupsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identity type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
    #[doc = "The list of user identities associated with the resource. The user identity dictionary key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[doc = "Definition of the job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Job {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Definition of job properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
impl Job {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job collection item properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobCollectionItem {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Job collection item properties."]
    pub properties: JobCollectionItemProperties,
}
impl JobCollectionItem {
    pub fn new(properties: JobCollectionItemProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "Job collection item properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobCollectionItemProperties {
    #[doc = "The runbook property associated with the entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runbook: Option<RunbookAssociationProperty>,
    #[doc = "The id of the job."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "The creation time of the job."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The status of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<job_collection_item_properties::Status>,
    #[doc = "The start time of the job."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the job."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The last modified time of the job."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of a resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Specifies the runOn group name where the job was executed."]
    #[serde(rename = "runOn", default, skip_serializing_if = "Option::is_none")]
    pub run_on: Option<String>,
}
impl JobCollectionItemProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_collection_item_properties {
    use super::*;
    #[doc = "The status of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        New,
        Activating,
        Running,
        Completed,
        Failed,
        Stopped,
        Blocked,
        Suspended,
        Disconnected,
        Suspending,
        Stopping,
        Resuming,
        Removing,
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
                Self::New => serializer.serialize_unit_variant("Status", 0u32, "New"),
                Self::Activating => serializer.serialize_unit_variant("Status", 1u32, "Activating"),
                Self::Running => serializer.serialize_unit_variant("Status", 2u32, "Running"),
                Self::Completed => serializer.serialize_unit_variant("Status", 3u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("Status", 4u32, "Failed"),
                Self::Stopped => serializer.serialize_unit_variant("Status", 5u32, "Stopped"),
                Self::Blocked => serializer.serialize_unit_variant("Status", 6u32, "Blocked"),
                Self::Suspended => serializer.serialize_unit_variant("Status", 7u32, "Suspended"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 8u32, "Disconnected"),
                Self::Suspending => serializer.serialize_unit_variant("Status", 9u32, "Suspending"),
                Self::Stopping => serializer.serialize_unit_variant("Status", 10u32, "Stopping"),
                Self::Resuming => serializer.serialize_unit_variant("Status", 11u32, "Resuming"),
                Self::Removing => serializer.serialize_unit_variant("Status", 12u32, "Removing"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters supplied to the create job operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobCreateParameters {
    pub properties: JobCreateProperties,
}
impl JobCreateParameters {
    pub fn new(properties: JobCreateProperties) -> Self {
        Self { properties }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobCreateProperties {
    #[doc = "The runbook property associated with the entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runbook: Option<RunbookAssociationProperty>,
    #[doc = "Gets or sets the parameters of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Gets or sets the runOn which specifies the group name where the job is to be executed."]
    #[serde(rename = "runOn", default, skip_serializing_if = "Option::is_none")]
    pub run_on: Option<String>,
}
impl JobCreateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for the list job operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobListResultV2 {
    #[doc = "List of jobs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobCollectionItem>,
    #[doc = "The  link to the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobListResultV2 {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobListResultV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of job properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobProperties {
    #[doc = "The runbook property associated with the entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runbook: Option<RunbookAssociationProperty>,
    #[doc = "Gets or sets the job started by."]
    #[serde(rename = "startedBy", default, skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    #[doc = "Gets or sets the runOn which specifies the group name where the job is to be executed."]
    #[serde(rename = "runOn", default, skip_serializing_if = "Option::is_none")]
    pub run_on: Option<String>,
    #[doc = "Gets or sets the id of the job."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "Gets or sets the creation time of the job."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the status of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<job_properties::Status>,
    #[doc = "Gets or sets the status details of the job."]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "Gets or sets the start time of the job."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the end time of the job."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the exception of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exception: Option<String>,
    #[doc = "Gets or sets the last modified time of the job."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last status modified time of the job."]
    #[serde(rename = "lastStatusModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_status_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the parameters of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "The provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<JobProvisioningStateProperty>,
}
impl JobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_properties {
    use super::*;
    #[doc = "Gets or sets the status of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        New,
        Activating,
        Running,
        Completed,
        Failed,
        Stopped,
        Blocked,
        Suspended,
        Disconnected,
        Suspending,
        Stopping,
        Resuming,
        Removing,
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
                Self::New => serializer.serialize_unit_variant("Status", 0u32, "New"),
                Self::Activating => serializer.serialize_unit_variant("Status", 1u32, "Activating"),
                Self::Running => serializer.serialize_unit_variant("Status", 2u32, "Running"),
                Self::Completed => serializer.serialize_unit_variant("Status", 3u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("Status", 4u32, "Failed"),
                Self::Stopped => serializer.serialize_unit_variant("Status", 5u32, "Stopped"),
                Self::Blocked => serializer.serialize_unit_variant("Status", 6u32, "Blocked"),
                Self::Suspended => serializer.serialize_unit_variant("Status", 7u32, "Suspended"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 8u32, "Disconnected"),
                Self::Suspending => serializer.serialize_unit_variant("Status", 9u32, "Suspending"),
                Self::Stopping => serializer.serialize_unit_variant("Status", 10u32, "Stopping"),
                Self::Resuming => serializer.serialize_unit_variant("Status", 11u32, "Resuming"),
                Self::Removing => serializer.serialize_unit_variant("Status", 12u32, "Removing"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The provisioning state of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "JobProvisioningStateProperty")]
pub enum JobProvisioningStateProperty {
    Failed,
    Succeeded,
    Suspended,
    Processing,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for JobProvisioningStateProperty {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for JobProvisioningStateProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for JobProvisioningStateProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Failed => serializer.serialize_unit_variant("JobProvisioningStateProperty", 0u32, "Failed"),
            Self::Succeeded => serializer.serialize_unit_variant("JobProvisioningStateProperty", 1u32, "Succeeded"),
            Self::Suspended => serializer.serialize_unit_variant("JobProvisioningStateProperty", 2u32, "Suspended"),
            Self::Processing => serializer.serialize_unit_variant("JobProvisioningStateProperty", 3u32, "Processing"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Definition of the job schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobSchedule {
    #[doc = "Gets the id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the name of the variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Definition of job schedule parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobScheduleProperties>,
}
impl JobSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create job schedule operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobScheduleCreateParameters {
    #[doc = "The parameters supplied to the create job schedule operation."]
    pub properties: JobScheduleCreateProperties,
}
impl JobScheduleCreateParameters {
    pub fn new(properties: JobScheduleCreateProperties) -> Self {
        Self { properties }
    }
}
#[doc = "The parameters supplied to the create job schedule operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobScheduleCreateProperties {
    #[doc = "The schedule property associated with the entity."]
    pub schedule: ScheduleAssociationProperty,
    #[doc = "The runbook property associated with the entity."]
    pub runbook: RunbookAssociationProperty,
    #[doc = "Gets or sets the hybrid worker group that the scheduled job should run on."]
    #[serde(rename = "runOn", default, skip_serializing_if = "Option::is_none")]
    pub run_on: Option<String>,
    #[doc = "Gets or sets a list of job properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl JobScheduleCreateProperties {
    pub fn new(schedule: ScheduleAssociationProperty, runbook: RunbookAssociationProperty) -> Self {
        Self {
            schedule,
            runbook,
            run_on: None,
            parameters: None,
        }
    }
}
#[doc = "The response model for the list job schedule operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobScheduleListResult {
    #[doc = "Gets or sets a list of job schedules."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobSchedule>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobScheduleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobScheduleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of job schedule parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobScheduleProperties {
    #[doc = "Gets or sets the id of job schedule."]
    #[serde(rename = "jobScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub job_schedule_id: Option<String>,
    #[doc = "The schedule property associated with the entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ScheduleAssociationProperty>,
    #[doc = "The runbook property associated with the entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runbook: Option<RunbookAssociationProperty>,
    #[doc = "Gets or sets the hybrid worker group that the scheduled job should run on."]
    #[serde(rename = "runOn", default, skip_serializing_if = "Option::is_none")]
    pub run_on: Option<String>,
    #[doc = "Gets or sets the parameters of the job schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl JobScheduleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the job stream."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStream {
    #[doc = "Gets or sets the id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Definition of the job stream."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobStreamProperties>,
}
impl JobStream {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for the list job stream operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStreamListResult {
    #[doc = "A list of job streams."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobStream>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobStreamListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobStreamListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the job stream."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStreamProperties {
    #[doc = "Gets or sets the id of the job stream."]
    #[serde(rename = "jobStreamId", default, skip_serializing_if = "Option::is_none")]
    pub job_stream_id: Option<String>,
    #[doc = "Gets or sets the creation time of the job."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the stream type."]
    #[serde(rename = "streamType", default, skip_serializing_if = "Option::is_none")]
    pub stream_type: Option<job_stream_properties::StreamType>,
    #[doc = "Gets or sets the stream text."]
    #[serde(rename = "streamText", default, skip_serializing_if = "Option::is_none")]
    pub stream_text: Option<String>,
    #[doc = "Gets or sets the summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "Gets or sets the values of the job stream."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
impl JobStreamProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_stream_properties {
    use super::*;
    #[doc = "Gets or sets the stream type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StreamType")]
    pub enum StreamType {
        Progress,
        Output,
        Warning,
        Error,
        Debug,
        Verbose,
        Any,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StreamType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StreamType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StreamType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Progress => serializer.serialize_unit_variant("StreamType", 0u32, "Progress"),
                Self::Output => serializer.serialize_unit_variant("StreamType", 1u32, "Output"),
                Self::Warning => serializer.serialize_unit_variant("StreamType", 2u32, "Warning"),
                Self::Error => serializer.serialize_unit_variant("StreamType", 3u32, "Error"),
                Self::Debug => serializer.serialize_unit_variant("StreamType", 4u32, "Debug"),
                Self::Verbose => serializer.serialize_unit_variant("StreamType", 5u32, "Verbose"),
                Self::Any => serializer.serialize_unit_variant("StreamType", 6u32, "Any"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Automation key which is used to register a DSC Node"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Key {
    #[doc = "Automation key name."]
    #[serde(rename = "KeyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<key::KeyName>,
    #[doc = "Automation key permissions."]
    #[serde(rename = "Permissions", default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<key::Permissions>,
    #[doc = "Value of the Automation Key used for registration."]
    #[serde(rename = "Value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Key {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod key {
    use super::*;
    #[doc = "Automation key name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeyName")]
    pub enum KeyName {
        Primary,
        Secondary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KeyName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KeyName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KeyName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("KeyName", 0u32, "Primary"),
                Self::Secondary => serializer.serialize_unit_variant("KeyName", 1u32, "Secondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Automation key permissions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Permissions")]
    pub enum Permissions {
        Read,
        Full,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Permissions {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Permissions {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Permissions {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Read => serializer.serialize_unit_variant("Permissions", 0u32, "Read"),
                Self::Full => serializer.serialize_unit_variant("Permissions", 1u32, "Full"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyListResult {
    #[doc = "Lists the automation keys."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<Key>,
}
impl KeyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Settings concerning key vault encryption for a configuration store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultProperties {
    #[doc = "The URI of the key vault key used to encrypt data."]
    #[serde(rename = "keyvaultUri", default, skip_serializing_if = "Option::is_none")]
    pub keyvault_uri: Option<String>,
    #[doc = "The name of key used to encrypt data."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "The key version of the key used to encrypt data."]
    #[serde(rename = "keyVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
}
impl KeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the linked workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedWorkspace {
    #[doc = "Gets the id of the linked workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl LinkedWorkspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Linux specific update configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxProperties {
    #[doc = "Update classifications included in the software update configuration."]
    #[serde(rename = "includedPackageClassifications", default, skip_serializing_if = "Option::is_none")]
    pub included_package_classifications: Option<linux_properties::IncludedPackageClassifications>,
    #[doc = "packages excluded from the software update configuration."]
    #[serde(rename = "excludedPackageNameMasks", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_package_name_masks: Vec<String>,
    #[doc = "packages included from the software update configuration."]
    #[serde(rename = "includedPackageNameMasks", default, skip_serializing_if = "Vec::is_empty")]
    pub included_package_name_masks: Vec<String>,
    #[doc = "Reboot setting for the software update configuration."]
    #[serde(rename = "rebootSetting", default, skip_serializing_if = "Option::is_none")]
    pub reboot_setting: Option<String>,
}
impl LinuxProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod linux_properties {
    use super::*;
    #[doc = "Update classifications included in the software update configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IncludedPackageClassifications")]
    pub enum IncludedPackageClassifications {
        Unclassified,
        Critical,
        Security,
        Other,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IncludedPackageClassifications {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IncludedPackageClassifications {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IncludedPackageClassifications {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unclassified => serializer.serialize_unit_variant("IncludedPackageClassifications", 0u32, "Unclassified"),
                Self::Critical => serializer.serialize_unit_variant("IncludedPackageClassifications", 1u32, "Critical"),
                Self::Security => serializer.serialize_unit_variant("IncludedPackageClassifications", 2u32, "Security"),
                Self::Other => serializer.serialize_unit_variant("IncludedPackageClassifications", 3u32, "Other"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Definition of the module type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Module {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Definition of the module property type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ModuleProperties>,
    #[doc = "Gets or sets the etag of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Module {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create or update module operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleCreateOrUpdateParameters {
    #[doc = "The parameters supplied to the create or update module properties."]
    pub properties: ModuleCreateOrUpdateProperties,
    #[doc = "Gets or sets name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets the tags attached to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ModuleCreateOrUpdateParameters {
    pub fn new(properties: ModuleCreateOrUpdateProperties) -> Self {
        Self {
            properties,
            name: None,
            location: None,
            tags: None,
        }
    }
}
#[doc = "The parameters supplied to the create or update module properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleCreateOrUpdateProperties {
    #[doc = "Definition of the content link."]
    #[serde(rename = "contentLink")]
    pub content_link: ContentLink,
}
impl ModuleCreateOrUpdateProperties {
    pub fn new(content_link: ContentLink) -> Self {
        Self { content_link }
    }
}
#[doc = "Definition of the module error info type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModuleErrorInfo {
    #[doc = "Gets or sets the error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Gets or sets the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ModuleErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for the list module operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModuleListResult {
    #[doc = "Gets or sets a list of modules."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Module>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ModuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ModuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the module property type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModuleProperties {
    #[doc = "Gets or sets the isGlobal flag of the module."]
    #[serde(rename = "isGlobal", default, skip_serializing_if = "Option::is_none")]
    pub is_global: Option<bool>,
    #[doc = "Gets or sets the version of the module."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets or sets the size in bytes of the module."]
    #[serde(rename = "sizeInBytes", default, skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[doc = "Gets or sets the activity count of the module."]
    #[serde(rename = "activityCount", default, skip_serializing_if = "Option::is_none")]
    pub activity_count: Option<i32>,
    #[doc = "Gets or sets the provisioning state of the module."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<module_properties::ProvisioningState>,
    #[doc = "Definition of the content link."]
    #[serde(rename = "contentLink", default, skip_serializing_if = "Option::is_none")]
    pub content_link: Option<ContentLink>,
    #[doc = "Definition of the module error info type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ModuleErrorInfo>,
    #[doc = "Gets or sets the creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets type of module, if its composite or not."]
    #[serde(rename = "isComposite", default, skip_serializing_if = "Option::is_none")]
    pub is_composite: Option<bool>,
}
impl ModuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod module_properties {
    use super::*;
    #[doc = "Gets or sets the provisioning state of the module."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Created,
        Creating,
        StartingImportModuleRunbook,
        RunningImportModuleRunbook,
        ContentRetrieved,
        ContentDownloaded,
        ContentValidated,
        ConnectionTypeImported,
        ContentStored,
        ModuleDataStored,
        ActivitiesStored,
        ModuleImportRunbookComplete,
        Succeeded,
        Failed,
        Cancelled,
        Updating,
    }
}
#[doc = "The parameters supplied to the update module operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModuleUpdateParameters {
    #[doc = "The parameters supplied to the update properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ModuleUpdateProperties>,
    #[doc = "Gets or sets name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets the tags attached to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ModuleUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the update properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModuleUpdateProperties {
    #[doc = "Definition of the content link."]
    #[serde(rename = "contentLink", default, skip_serializing_if = "Option::is_none")]
    pub content_link: Option<ContentLink>,
}
impl ModuleUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Number of nodes based on the Filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeCount {
    #[doc = "Gets the name of a count type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NodeCountProperties>,
}
impl NodeCount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeCountProperties {
    #[doc = "Gets the count for the name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl NodeCountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets the count of nodes by count type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeCounts {
    #[doc = "Gets an array of counts"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NodeCount>,
    #[doc = "Gets the total number of records matching countType criteria."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}
impl NodeCounts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Non Azure query for the update configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NonAzureQueryProperties {
    #[doc = "Log Analytics Saved Search name."]
    #[serde(rename = "functionAlias", default, skip_serializing_if = "Option::is_none")]
    pub function_alias: Option<String>,
    #[doc = "Workspace Id for Log Analytics in which the saved Search is resided."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}
impl NonAzureQueryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Automation REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Provider, Resource and Operation values"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Provider, Resource and Operation values"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.Automation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Runbooks, Jobs etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The response model for the list of Automation operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Automation operations supported by the Automation resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private endpoint connections"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of private endpoint connections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Properties of a private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "Private endpoint which the connection belongs to."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpointProperty>,
    #[doc = "Gets the groupIds."]
    #[serde(rename = "groupIds", default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<GroupIdsProperty>,
    #[doc = "Connection State of the Private Endpoint Connection."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionStateProperty>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint which the connection belongs to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointProperty {
    #[doc = "Resource id of the private endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpointProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
impl azure_core::Continuable for PrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
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
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Connection State of the Private Endpoint Connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionStateProperty {
    #[doc = "The private link service connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The private link service connection description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Any action that is required beyond basic workflow (approve/ reject/ disconnect)"]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionStateProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ARM proxy resource."]
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
#[doc = "The parameters supplied to the create or update module operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PythonPackageCreateParameters {
    #[doc = "The parameters supplied to the create or update module properties."]
    pub properties: PythonPackageCreateProperties,
    #[doc = "Gets or sets the tags attached to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PythonPackageCreateParameters {
    pub fn new(properties: PythonPackageCreateProperties) -> Self {
        Self { properties, tags: None }
    }
}
#[doc = "The parameters supplied to the create or update module properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PythonPackageCreateProperties {
    #[doc = "Definition of the content link."]
    #[serde(rename = "contentLink")]
    pub content_link: ContentLink,
}
impl PythonPackageCreateProperties {
    pub fn new(content_link: ContentLink) -> Self {
        Self { content_link }
    }
}
#[doc = "The parameters supplied to the update module operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PythonPackageUpdateParameters {
    #[doc = "Gets or sets the tags attached to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PythonPackageUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The core properties of ARM resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource Id for the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of RunAs credential to use for hybrid worker."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunAsCredentialAssociationProperty {
    #[doc = "Gets or sets the name of the credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl RunAsCredentialAssociationProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the runbook type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Runbook {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Definition of the runbook property type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunbookProperties>,
    #[doc = "Gets or sets the etag of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Runbook {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The runbook property associated with the entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunbookAssociationProperty {
    #[doc = "Gets or sets the name of the runbook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl RunbookAssociationProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create or update runbook operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunbookCreateOrUpdateDraftParameters {
    #[doc = "Content of the Runbook."]
    #[serde(rename = "runbookContent")]
    pub runbook_content: String,
}
impl RunbookCreateOrUpdateDraftParameters {
    pub fn new(runbook_content: String) -> Self {
        Self { runbook_content }
    }
}
#[doc = "The parameters supplied to the create or update draft runbook properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunbookCreateOrUpdateDraftProperties {
    #[doc = "Gets or sets verbose log option."]
    #[serde(rename = "logVerbose", default, skip_serializing_if = "Option::is_none")]
    pub log_verbose: Option<bool>,
    #[doc = "Gets or sets progress log option."]
    #[serde(rename = "logProgress", default, skip_serializing_if = "Option::is_none")]
    pub log_progress: Option<bool>,
    #[doc = "Gets or sets the type of the runbook."]
    #[serde(rename = "runbookType")]
    pub runbook_type: runbook_create_or_update_draft_properties::RunbookType,
    pub draft: RunbookDraft,
    #[doc = "Gets or sets the description of the runbook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets the activity-level tracing options of the runbook."]
    #[serde(rename = "logActivityTrace", default, skip_serializing_if = "Option::is_none")]
    pub log_activity_trace: Option<i32>,
}
impl RunbookCreateOrUpdateDraftProperties {
    pub fn new(runbook_type: runbook_create_or_update_draft_properties::RunbookType, draft: RunbookDraft) -> Self {
        Self {
            log_verbose: None,
            log_progress: None,
            runbook_type,
            draft,
            description: None,
            log_activity_trace: None,
        }
    }
}
pub mod runbook_create_or_update_draft_properties {
    use super::*;
    #[doc = "Gets or sets the type of the runbook."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RunbookType")]
    pub enum RunbookType {
        Script,
        Graph,
        PowerShellWorkflow,
        PowerShell,
        GraphPowerShellWorkflow,
        GraphPowerShell,
        Python2,
        Python3,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RunbookType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RunbookType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RunbookType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Script => serializer.serialize_unit_variant("RunbookType", 0u32, "Script"),
                Self::Graph => serializer.serialize_unit_variant("RunbookType", 1u32, "Graph"),
                Self::PowerShellWorkflow => serializer.serialize_unit_variant("RunbookType", 2u32, "PowerShellWorkflow"),
                Self::PowerShell => serializer.serialize_unit_variant("RunbookType", 3u32, "PowerShell"),
                Self::GraphPowerShellWorkflow => serializer.serialize_unit_variant("RunbookType", 4u32, "GraphPowerShellWorkflow"),
                Self::GraphPowerShell => serializer.serialize_unit_variant("RunbookType", 5u32, "GraphPowerShell"),
                Self::Python2 => serializer.serialize_unit_variant("RunbookType", 6u32, "Python2"),
                Self::Python3 => serializer.serialize_unit_variant("RunbookType", 7u32, "Python3"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters supplied to the create or update runbook operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunbookCreateOrUpdateParameters {
    #[doc = "The parameters supplied to the create or update runbook properties."]
    pub properties: RunbookCreateOrUpdateProperties,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets the tags attached to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl RunbookCreateOrUpdateParameters {
    pub fn new(properties: RunbookCreateOrUpdateProperties) -> Self {
        Self {
            properties,
            name: None,
            location: None,
            tags: None,
        }
    }
}
#[doc = "The parameters supplied to the create or update runbook properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunbookCreateOrUpdateProperties {
    #[doc = "Gets or sets verbose log option."]
    #[serde(rename = "logVerbose", default, skip_serializing_if = "Option::is_none")]
    pub log_verbose: Option<bool>,
    #[doc = "Gets or sets progress log option."]
    #[serde(rename = "logProgress", default, skip_serializing_if = "Option::is_none")]
    pub log_progress: Option<bool>,
    #[doc = "Gets or sets the type of the runbook."]
    #[serde(rename = "runbookType")]
    pub runbook_type: runbook_create_or_update_properties::RunbookType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<RunbookDraft>,
    #[doc = "Definition of the content link."]
    #[serde(rename = "publishContentLink", default, skip_serializing_if = "Option::is_none")]
    pub publish_content_link: Option<ContentLink>,
    #[doc = "Gets or sets the description of the runbook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets the activity-level tracing options of the runbook."]
    #[serde(rename = "logActivityTrace", default, skip_serializing_if = "Option::is_none")]
    pub log_activity_trace: Option<i32>,
}
impl RunbookCreateOrUpdateProperties {
    pub fn new(runbook_type: runbook_create_or_update_properties::RunbookType) -> Self {
        Self {
            log_verbose: None,
            log_progress: None,
            runbook_type,
            draft: None,
            publish_content_link: None,
            description: None,
            log_activity_trace: None,
        }
    }
}
pub mod runbook_create_or_update_properties {
    use super::*;
    #[doc = "Gets or sets the type of the runbook."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RunbookType")]
    pub enum RunbookType {
        Script,
        Graph,
        PowerShellWorkflow,
        PowerShell,
        GraphPowerShellWorkflow,
        GraphPowerShell,
        Python2,
        Python3,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RunbookType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RunbookType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RunbookType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Script => serializer.serialize_unit_variant("RunbookType", 0u32, "Script"),
                Self::Graph => serializer.serialize_unit_variant("RunbookType", 1u32, "Graph"),
                Self::PowerShellWorkflow => serializer.serialize_unit_variant("RunbookType", 2u32, "PowerShellWorkflow"),
                Self::PowerShell => serializer.serialize_unit_variant("RunbookType", 3u32, "PowerShell"),
                Self::GraphPowerShellWorkflow => serializer.serialize_unit_variant("RunbookType", 4u32, "GraphPowerShellWorkflow"),
                Self::GraphPowerShell => serializer.serialize_unit_variant("RunbookType", 5u32, "GraphPowerShell"),
                Self::Python2 => serializer.serialize_unit_variant("RunbookType", 6u32, "Python2"),
                Self::Python3 => serializer.serialize_unit_variant("RunbookType", 7u32, "Python3"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunbookDraft {
    #[doc = "Gets or sets whether runbook is in edit mode."]
    #[serde(rename = "inEdit", default, skip_serializing_if = "Option::is_none")]
    pub in_edit: Option<bool>,
    #[doc = "Definition of the content link."]
    #[serde(rename = "draftContentLink", default, skip_serializing_if = "Option::is_none")]
    pub draft_content_link: Option<ContentLink>,
    #[doc = "Gets or sets the creation time of the runbook draft."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last modified time of the runbook draft."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the runbook draft parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Gets or sets the runbook output types."]
    #[serde(rename = "outputTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub output_types: Vec<String>,
}
impl RunbookDraft {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for the undo edit runbook operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunbookDraftUndoEditResult {
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<runbook_draft_undo_edit_result::StatusCode>,
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}
impl RunbookDraftUndoEditResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod runbook_draft_undo_edit_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StatusCode")]
    pub enum StatusCode {
        Continue,
        SwitchingProtocols,
        #[serde(rename = "OK")]
        Ok,
        Created,
        Accepted,
        NonAuthoritativeInformation,
        NoContent,
        ResetContent,
        PartialContent,
        MultipleChoices,
        Ambiguous,
        MovedPermanently,
        Moved,
        Found,
        Redirect,
        SeeOther,
        RedirectMethod,
        NotModified,
        UseProxy,
        Unused,
        TemporaryRedirect,
        RedirectKeepVerb,
        BadRequest,
        Unauthorized,
        PaymentRequired,
        Forbidden,
        NotFound,
        MethodNotAllowed,
        NotAcceptable,
        ProxyAuthenticationRequired,
        RequestTimeout,
        Conflict,
        Gone,
        LengthRequired,
        PreconditionFailed,
        RequestEntityTooLarge,
        RequestUriTooLong,
        UnsupportedMediaType,
        RequestedRangeNotSatisfiable,
        ExpectationFailed,
        UpgradeRequired,
        InternalServerError,
        NotImplemented,
        BadGateway,
        ServiceUnavailable,
        GatewayTimeout,
        HttpVersionNotSupported,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StatusCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StatusCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StatusCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Continue => serializer.serialize_unit_variant("StatusCode", 0u32, "Continue"),
                Self::SwitchingProtocols => serializer.serialize_unit_variant("StatusCode", 1u32, "SwitchingProtocols"),
                Self::Ok => serializer.serialize_unit_variant("StatusCode", 2u32, "OK"),
                Self::Created => serializer.serialize_unit_variant("StatusCode", 3u32, "Created"),
                Self::Accepted => serializer.serialize_unit_variant("StatusCode", 4u32, "Accepted"),
                Self::NonAuthoritativeInformation => serializer.serialize_unit_variant("StatusCode", 5u32, "NonAuthoritativeInformation"),
                Self::NoContent => serializer.serialize_unit_variant("StatusCode", 6u32, "NoContent"),
                Self::ResetContent => serializer.serialize_unit_variant("StatusCode", 7u32, "ResetContent"),
                Self::PartialContent => serializer.serialize_unit_variant("StatusCode", 8u32, "PartialContent"),
                Self::MultipleChoices => serializer.serialize_unit_variant("StatusCode", 9u32, "MultipleChoices"),
                Self::Ambiguous => serializer.serialize_unit_variant("StatusCode", 10u32, "Ambiguous"),
                Self::MovedPermanently => serializer.serialize_unit_variant("StatusCode", 11u32, "MovedPermanently"),
                Self::Moved => serializer.serialize_unit_variant("StatusCode", 12u32, "Moved"),
                Self::Found => serializer.serialize_unit_variant("StatusCode", 13u32, "Found"),
                Self::Redirect => serializer.serialize_unit_variant("StatusCode", 14u32, "Redirect"),
                Self::SeeOther => serializer.serialize_unit_variant("StatusCode", 15u32, "SeeOther"),
                Self::RedirectMethod => serializer.serialize_unit_variant("StatusCode", 16u32, "RedirectMethod"),
                Self::NotModified => serializer.serialize_unit_variant("StatusCode", 17u32, "NotModified"),
                Self::UseProxy => serializer.serialize_unit_variant("StatusCode", 18u32, "UseProxy"),
                Self::Unused => serializer.serialize_unit_variant("StatusCode", 19u32, "Unused"),
                Self::TemporaryRedirect => serializer.serialize_unit_variant("StatusCode", 20u32, "TemporaryRedirect"),
                Self::RedirectKeepVerb => serializer.serialize_unit_variant("StatusCode", 21u32, "RedirectKeepVerb"),
                Self::BadRequest => serializer.serialize_unit_variant("StatusCode", 22u32, "BadRequest"),
                Self::Unauthorized => serializer.serialize_unit_variant("StatusCode", 23u32, "Unauthorized"),
                Self::PaymentRequired => serializer.serialize_unit_variant("StatusCode", 24u32, "PaymentRequired"),
                Self::Forbidden => serializer.serialize_unit_variant("StatusCode", 25u32, "Forbidden"),
                Self::NotFound => serializer.serialize_unit_variant("StatusCode", 26u32, "NotFound"),
                Self::MethodNotAllowed => serializer.serialize_unit_variant("StatusCode", 27u32, "MethodNotAllowed"),
                Self::NotAcceptable => serializer.serialize_unit_variant("StatusCode", 28u32, "NotAcceptable"),
                Self::ProxyAuthenticationRequired => serializer.serialize_unit_variant("StatusCode", 29u32, "ProxyAuthenticationRequired"),
                Self::RequestTimeout => serializer.serialize_unit_variant("StatusCode", 30u32, "RequestTimeout"),
                Self::Conflict => serializer.serialize_unit_variant("StatusCode", 31u32, "Conflict"),
                Self::Gone => serializer.serialize_unit_variant("StatusCode", 32u32, "Gone"),
                Self::LengthRequired => serializer.serialize_unit_variant("StatusCode", 33u32, "LengthRequired"),
                Self::PreconditionFailed => serializer.serialize_unit_variant("StatusCode", 34u32, "PreconditionFailed"),
                Self::RequestEntityTooLarge => serializer.serialize_unit_variant("StatusCode", 35u32, "RequestEntityTooLarge"),
                Self::RequestUriTooLong => serializer.serialize_unit_variant("StatusCode", 36u32, "RequestUriTooLong"),
                Self::UnsupportedMediaType => serializer.serialize_unit_variant("StatusCode", 37u32, "UnsupportedMediaType"),
                Self::RequestedRangeNotSatisfiable => {
                    serializer.serialize_unit_variant("StatusCode", 38u32, "RequestedRangeNotSatisfiable")
                }
                Self::ExpectationFailed => serializer.serialize_unit_variant("StatusCode", 39u32, "ExpectationFailed"),
                Self::UpgradeRequired => serializer.serialize_unit_variant("StatusCode", 40u32, "UpgradeRequired"),
                Self::InternalServerError => serializer.serialize_unit_variant("StatusCode", 41u32, "InternalServerError"),
                Self::NotImplemented => serializer.serialize_unit_variant("StatusCode", 42u32, "NotImplemented"),
                Self::BadGateway => serializer.serialize_unit_variant("StatusCode", 43u32, "BadGateway"),
                Self::ServiceUnavailable => serializer.serialize_unit_variant("StatusCode", 44u32, "ServiceUnavailable"),
                Self::GatewayTimeout => serializer.serialize_unit_variant("StatusCode", 45u32, "GatewayTimeout"),
                Self::HttpVersionNotSupported => serializer.serialize_unit_variant("StatusCode", 46u32, "HttpVersionNotSupported"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response model for the list runbook operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunbookListResult {
    #[doc = "Gets or sets a list of runbooks."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Runbook>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RunbookListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RunbookListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the runbook parameter type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunbookParameter {
    #[doc = "Gets or sets the type of the parameter."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets a Boolean value to indicate whether the parameter is mandatory or not."]
    #[serde(rename = "isMandatory", default, skip_serializing_if = "Option::is_none")]
    pub is_mandatory: Option<bool>,
    #[doc = "Get or sets the position of the parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[doc = "Gets or sets the default value of parameter."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}
impl RunbookParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the runbook property type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunbookProperties {
    #[doc = "Gets or sets the type of the runbook."]
    #[serde(rename = "runbookType", default, skip_serializing_if = "Option::is_none")]
    pub runbook_type: Option<runbook_properties::RunbookType>,
    #[doc = "Definition of the content link."]
    #[serde(rename = "publishContentLink", default, skip_serializing_if = "Option::is_none")]
    pub publish_content_link: Option<ContentLink>,
    #[doc = "Gets or sets the state of the runbook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<runbook_properties::State>,
    #[doc = "Gets or sets verbose log option."]
    #[serde(rename = "logVerbose", default, skip_serializing_if = "Option::is_none")]
    pub log_verbose: Option<bool>,
    #[doc = "Gets or sets progress log option."]
    #[serde(rename = "logProgress", default, skip_serializing_if = "Option::is_none")]
    pub log_progress: Option<bool>,
    #[doc = "Gets or sets the option to log activity trace of the runbook."]
    #[serde(rename = "logActivityTrace", default, skip_serializing_if = "Option::is_none")]
    pub log_activity_trace: Option<i32>,
    #[doc = "Gets or sets the job count of the runbook."]
    #[serde(rename = "jobCount", default, skip_serializing_if = "Option::is_none")]
    pub job_count: Option<i32>,
    #[doc = "Gets or sets the runbook parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Gets or sets the runbook output types."]
    #[serde(rename = "outputTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub output_types: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<RunbookDraft>,
    #[doc = "Gets or sets the provisioning state of the runbook."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<runbook_properties::ProvisioningState>,
    #[doc = "Gets or sets the last modified by."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "Gets or sets the creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl RunbookProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod runbook_properties {
    use super::*;
    #[doc = "Gets or sets the type of the runbook."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RunbookType")]
    pub enum RunbookType {
        Script,
        Graph,
        PowerShellWorkflow,
        PowerShell,
        GraphPowerShellWorkflow,
        GraphPowerShell,
        Python2,
        Python3,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RunbookType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RunbookType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RunbookType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Script => serializer.serialize_unit_variant("RunbookType", 0u32, "Script"),
                Self::Graph => serializer.serialize_unit_variant("RunbookType", 1u32, "Graph"),
                Self::PowerShellWorkflow => serializer.serialize_unit_variant("RunbookType", 2u32, "PowerShellWorkflow"),
                Self::PowerShell => serializer.serialize_unit_variant("RunbookType", 3u32, "PowerShell"),
                Self::GraphPowerShellWorkflow => serializer.serialize_unit_variant("RunbookType", 4u32, "GraphPowerShellWorkflow"),
                Self::GraphPowerShell => serializer.serialize_unit_variant("RunbookType", 5u32, "GraphPowerShell"),
                Self::Python2 => serializer.serialize_unit_variant("RunbookType", 6u32, "Python2"),
                Self::Python3 => serializer.serialize_unit_variant("RunbookType", 7u32, "Python3"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the state of the runbook."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        New,
        Edit,
        Published,
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
                Self::New => serializer.serialize_unit_variant("State", 0u32, "New"),
                Self::Edit => serializer.serialize_unit_variant("State", 1u32, "Edit"),
                Self::Published => serializer.serialize_unit_variant("State", 2u32, "Published"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets the provisioning state of the runbook."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Succeeded,
    }
}
#[doc = "The parameters supplied to the update runbook operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunbookUpdateParameters {
    #[doc = "The parameters supplied to the update runbook properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunbookUpdateProperties>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets the tags attached to the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl RunbookUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the update runbook properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunbookUpdateProperties {
    #[doc = "Gets or sets the description of the runbook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets verbose log option."]
    #[serde(rename = "logVerbose", default, skip_serializing_if = "Option::is_none")]
    pub log_verbose: Option<bool>,
    #[doc = "Gets or sets progress log option."]
    #[serde(rename = "logProgress", default, skip_serializing_if = "Option::is_none")]
    pub log_progress: Option<bool>,
    #[doc = "Gets or sets the activity-level tracing options of the runbook."]
    #[serde(rename = "logActivityTrace", default, skip_serializing_if = "Option::is_none")]
    pub log_activity_trace: Option<i32>,
}
impl RunbookUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of schedule parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SucScheduleProperties {
    #[doc = "Gets or sets the start time of the schedule."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the start time's offset in minutes."]
    #[serde(rename = "startTimeOffsetMinutes", default, skip_serializing_if = "Option::is_none")]
    pub start_time_offset_minutes: Option<f64>,
    #[doc = "Gets or sets the end time of the schedule."]
    #[serde(rename = "expiryTime", with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the expiry time's offset in minutes."]
    #[serde(rename = "expiryTimeOffsetMinutes", default, skip_serializing_if = "Option::is_none")]
    pub expiry_time_offset_minutes: Option<f64>,
    #[doc = "Gets or sets a value indicating whether this schedule is enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Gets or sets the next run time of the schedule."]
    #[serde(rename = "nextRun", with = "azure_core::date::rfc3339::option")]
    pub next_run: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the next run time's offset in minutes."]
    #[serde(rename = "nextRunOffsetMinutes", default, skip_serializing_if = "Option::is_none")]
    pub next_run_offset_minutes: Option<f64>,
    #[doc = "Gets or sets the interval of the schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[doc = "Gets or sets the frequency of the schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<ScheduleFrequency>,
    #[doc = "Gets or sets the time zone of the schedule."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "The properties of the create Advanced Schedule."]
    #[serde(rename = "advancedSchedule", default, skip_serializing_if = "Option::is_none")]
    pub advanced_schedule: Option<AdvancedSchedule>,
    #[doc = "Gets or sets the creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl SucScheduleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Schedule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Definition of schedule parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduleProperties>,
}
impl Schedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The schedule property associated with the entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleAssociationProperty {
    #[doc = "Gets or sets the name of the Schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ScheduleAssociationProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create or update schedule operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleCreateOrUpdateParameters {
    #[doc = "Gets or sets the name of the Schedule."]
    pub name: String,
    #[doc = "The parameters supplied to the create or update schedule operation."]
    pub properties: ScheduleCreateOrUpdateProperties,
}
impl ScheduleCreateOrUpdateParameters {
    pub fn new(name: String, properties: ScheduleCreateOrUpdateProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "The parameters supplied to the create or update schedule operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleCreateOrUpdateProperties {
    #[doc = "Gets or sets the description of the schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets the start time of the schedule."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "Gets or sets the end time of the schedule."]
    #[serde(rename = "expiryTime", with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the interval of the schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<serde_json::Value>,
    #[doc = "Gets or sets the frequency of the schedule."]
    pub frequency: ScheduleFrequency,
    #[doc = "Gets or sets the time zone of the schedule."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "The properties of the create Advanced Schedule."]
    #[serde(rename = "advancedSchedule", default, skip_serializing_if = "Option::is_none")]
    pub advanced_schedule: Option<AdvancedSchedule>,
}
impl ScheduleCreateOrUpdateProperties {
    pub fn new(start_time: time::OffsetDateTime, frequency: ScheduleFrequency) -> Self {
        Self {
            description: None,
            start_time,
            expiry_time: None,
            interval: None,
            frequency,
            time_zone: None,
            advanced_schedule: None,
        }
    }
}
#[doc = "The response model for the list schedule operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleListResult {
    #[doc = "Gets or sets a list of schedules."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Schedule>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScheduleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScheduleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of schedule parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleProperties {
    #[doc = "Gets or sets the start time of the schedule."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the start time's offset in minutes."]
    #[serde(rename = "startTimeOffsetMinutes", default, skip_serializing_if = "Option::is_none")]
    pub start_time_offset_minutes: Option<f64>,
    #[doc = "Gets or sets the end time of the schedule."]
    #[serde(rename = "expiryTime", with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the expiry time's offset in minutes."]
    #[serde(rename = "expiryTimeOffsetMinutes", default, skip_serializing_if = "Option::is_none")]
    pub expiry_time_offset_minutes: Option<f64>,
    #[doc = "Gets or sets a value indicating whether this schedule is enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Gets or sets the next run time of the schedule."]
    #[serde(rename = "nextRun", with = "azure_core::date::rfc3339::option")]
    pub next_run: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the next run time's offset in minutes."]
    #[serde(rename = "nextRunOffsetMinutes", default, skip_serializing_if = "Option::is_none")]
    pub next_run_offset_minutes: Option<f64>,
    #[doc = "Gets or sets the interval of the schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<serde_json::Value>,
    #[doc = "Gets or sets the frequency of the schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<ScheduleFrequency>,
    #[doc = "Gets or sets the time zone of the schedule."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "The properties of the create Advanced Schedule."]
    #[serde(rename = "advancedSchedule", default, skip_serializing_if = "Option::is_none")]
    pub advanced_schedule: Option<AdvancedSchedule>,
    #[doc = "Gets or sets the creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ScheduleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the update schedule operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleUpdateParameters {
    #[doc = "Gets or sets the name of the Schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The parameters supplied to the update schedule operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduleUpdateProperties>,
}
impl ScheduleUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the update schedule operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleUpdateProperties {
    #[doc = "Gets or sets the description of the schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets a value indicating whether this schedule is enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}
impl ScheduleUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The account SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "Gets or sets the SKU name of the account."]
    pub name: sku::Name,
    #[doc = "Gets or sets the SKU family."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "Gets or sets the SKU capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self {
            name,
            family: None,
            capacity: None,
        }
    }
}
pub mod sku {
    use super::*;
    #[doc = "Gets or sets the SKU name of the account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Free,
        Basic,
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
                Self::Free => serializer.serialize_unit_variant("Name", 0u32, "Free"),
                Self::Basic => serializer.serialize_unit_variant("Name", 1u32, "Basic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Definition of the source control."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControl {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Definition of the source control properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SourceControlProperties>,
}
impl SourceControl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create or update source control operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceControlCreateOrUpdateParameters {
    #[doc = "The properties of the create source control operation."]
    pub properties: SourceControlCreateOrUpdateProperties,
}
impl SourceControlCreateOrUpdateParameters {
    pub fn new(properties: SourceControlCreateOrUpdateProperties) -> Self {
        Self { properties }
    }
}
#[doc = "The properties of the create source control operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlCreateOrUpdateProperties {
    #[doc = "The repo url of the source control."]
    #[serde(rename = "repoUrl", default, skip_serializing_if = "Option::is_none")]
    pub repo_url: Option<String>,
    #[doc = "The repo branch of the source control. Include branch as empty string for VsoTfvc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[doc = "The folder path of the source control. Path must be relative."]
    #[serde(rename = "folderPath", default, skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
    #[doc = "The auto async of the source control. Default is false."]
    #[serde(rename = "autoSync", default, skip_serializing_if = "Option::is_none")]
    pub auto_sync: Option<bool>,
    #[doc = "The auto publish of the source control. Default is true."]
    #[serde(rename = "publishRunbook", default, skip_serializing_if = "Option::is_none")]
    pub publish_runbook: Option<bool>,
    #[doc = "The source type. Must be one of VsoGit, VsoTfvc, GitHub, case sensitive."]
    #[serde(rename = "sourceType", default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<source_control_create_or_update_properties::SourceType>,
    #[serde(rename = "securityToken", default, skip_serializing_if = "Option::is_none")]
    pub security_token: Option<SourceControlSecurityTokenProperties>,
    #[doc = "The user description of the source control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl SourceControlCreateOrUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod source_control_create_or_update_properties {
    use super::*;
    #[doc = "The source type. Must be one of VsoGit, VsoTfvc, GitHub, case sensitive."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceType")]
    pub enum SourceType {
        VsoGit,
        VsoTfvc,
        GitHub,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SourceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SourceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SourceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::VsoGit => serializer.serialize_unit_variant("SourceType", 0u32, "VsoGit"),
                Self::VsoTfvc => serializer.serialize_unit_variant("SourceType", 1u32, "VsoTfvc"),
                Self::GitHub => serializer.serialize_unit_variant("SourceType", 2u32, "GitHub"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response model for the list source controls operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlListResult {
    #[doc = "The list of source controls."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SourceControl>,
    #[doc = "The next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SourceControlListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SourceControlListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the source control properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlProperties {
    #[doc = "The repo url of the source control."]
    #[serde(rename = "repoUrl", default, skip_serializing_if = "Option::is_none")]
    pub repo_url: Option<String>,
    #[doc = "The repo branch of the source control. Include branch as empty string for VsoTfvc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[doc = "The folder path of the source control."]
    #[serde(rename = "folderPath", default, skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
    #[doc = "The auto sync of the source control. Default is false."]
    #[serde(rename = "autoSync", default, skip_serializing_if = "Option::is_none")]
    pub auto_sync: Option<bool>,
    #[doc = "The auto publish of the source control. Default is true."]
    #[serde(rename = "publishRunbook", default, skip_serializing_if = "Option::is_none")]
    pub publish_runbook: Option<bool>,
    #[doc = "The source type. Must be one of VsoGit, VsoTfvc, GitHub."]
    #[serde(rename = "sourceType", default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<source_control_properties::SourceType>,
    #[doc = "The description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
}
impl SourceControlProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod source_control_properties {
    use super::*;
    #[doc = "The source type. Must be one of VsoGit, VsoTfvc, GitHub."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceType")]
    pub enum SourceType {
        VsoGit,
        VsoTfvc,
        GitHub,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SourceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SourceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SourceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::VsoGit => serializer.serialize_unit_variant("SourceType", 0u32, "VsoGit"),
                Self::VsoTfvc => serializer.serialize_unit_variant("SourceType", 1u32, "VsoTfvc"),
                Self::GitHub => serializer.serialize_unit_variant("SourceType", 2u32, "GitHub"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlSecurityTokenProperties {
    #[doc = "The access token."]
    #[serde(rename = "accessToken", default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[doc = "The refresh token."]
    #[serde(rename = "refreshToken", default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[doc = "The token type. Must be either PersonalAccessToken or Oauth."]
    #[serde(rename = "tokenType", default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<source_control_security_token_properties::TokenType>,
}
impl SourceControlSecurityTokenProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod source_control_security_token_properties {
    use super::*;
    #[doc = "The token type. Must be either PersonalAccessToken or Oauth."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TokenType")]
    pub enum TokenType {
        PersonalAccessToken,
        Oauth,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TokenType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TokenType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TokenType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PersonalAccessToken => serializer.serialize_unit_variant("TokenType", 0u32, "PersonalAccessToken"),
                Self::Oauth => serializer.serialize_unit_variant("TokenType", 1u32, "Oauth"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Definition of the source control sync job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlSyncJob {
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Definition of source control sync job properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SourceControlSyncJobProperties>,
}
impl SourceControlSyncJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the source control sync job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlSyncJobById {
    #[doc = "The id of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Definition of source control sync job properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SourceControlSyncJobByIdProperties>,
}
impl SourceControlSyncJobById {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of source control sync job properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlSyncJobByIdProperties {
    #[doc = "The source control sync job id."]
    #[serde(rename = "sourceControlSyncJobId", default, skip_serializing_if = "Option::is_none")]
    pub source_control_sync_job_id: Option<String>,
    #[doc = "The creation time of the job."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the job."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<source_control_sync_job_by_id_properties::ProvisioningState>,
    #[doc = "The start time of the job."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the job."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The sync type."]
    #[serde(rename = "syncType", default, skip_serializing_if = "Option::is_none")]
    pub sync_type: Option<source_control_sync_job_by_id_properties::SyncType>,
    #[doc = "The exceptions that occurred while running the sync job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exception: Option<String>,
}
impl SourceControlSyncJobByIdProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod source_control_sync_job_by_id_properties {
    use super::*;
    #[doc = "The provisioning state of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Completed,
        Failed,
        Running,
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
                Self::Completed => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Running => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Running"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The sync type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SyncType")]
    pub enum SyncType {
        PartialSync,
        FullSync,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SyncType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SyncType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SyncType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PartialSync => serializer.serialize_unit_variant("SyncType", 0u32, "PartialSync"),
                Self::FullSync => serializer.serialize_unit_variant("SyncType", 1u32, "FullSync"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters supplied to the create source control sync job operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceControlSyncJobCreateParameters {
    #[doc = "Definition of create source control sync job properties."]
    pub properties: SourceControlSyncJobCreateProperties,
}
impl SourceControlSyncJobCreateParameters {
    pub fn new(properties: SourceControlSyncJobCreateProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Definition of create source control sync job properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceControlSyncJobCreateProperties {
    #[doc = "The commit id of the source control sync job. If not syncing to a commitId, enter an empty string."]
    #[serde(rename = "commitId")]
    pub commit_id: String,
}
impl SourceControlSyncJobCreateProperties {
    pub fn new(commit_id: String) -> Self {
        Self { commit_id }
    }
}
#[doc = "The response model for the list source control sync jobs operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlSyncJobListResult {
    #[doc = "The list of source control sync jobs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SourceControlSyncJob>,
    #[doc = "The next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SourceControlSyncJobListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SourceControlSyncJobListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of source control sync job properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlSyncJobProperties {
    #[doc = "The source control sync job id."]
    #[serde(rename = "sourceControlSyncJobId", default, skip_serializing_if = "Option::is_none")]
    pub source_control_sync_job_id: Option<String>,
    #[doc = "The creation time of the job."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the job."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<source_control_sync_job_properties::ProvisioningState>,
    #[doc = "The start time of the job."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the job."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The sync type."]
    #[serde(rename = "syncType", default, skip_serializing_if = "Option::is_none")]
    pub sync_type: Option<source_control_sync_job_properties::SyncType>,
}
impl SourceControlSyncJobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod source_control_sync_job_properties {
    use super::*;
    #[doc = "The provisioning state of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Completed,
        Failed,
        Running,
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
                Self::Completed => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Running => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Running"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The sync type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SyncType")]
    pub enum SyncType {
        PartialSync,
        FullSync,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SyncType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SyncType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SyncType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PartialSync => serializer.serialize_unit_variant("SyncType", 0u32, "PartialSync"),
                Self::FullSync => serializer.serialize_unit_variant("SyncType", 1u32, "FullSync"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Definition of the source control sync job stream."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlSyncJobStream {
    #[doc = "Resource id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Definition of source control sync job stream properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SourceControlSyncJobStreamProperties>,
}
impl SourceControlSyncJobStream {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the source control sync job stream by id."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlSyncJobStreamById {
    #[doc = "Resource id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Definition of source control sync job stream by id properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SourceControlSyncJobStreamByIdProperties>,
}
impl SourceControlSyncJobStreamById {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of source control sync job stream by id properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlSyncJobStreamByIdProperties {
    #[doc = "The sync job stream id."]
    #[serde(rename = "sourceControlSyncJobStreamId", default, skip_serializing_if = "Option::is_none")]
    pub source_control_sync_job_stream_id: Option<String>,
    #[doc = "The summary of the sync job stream."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "The time of the sync job stream."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
    #[doc = "The type of the sync job stream."]
    #[serde(rename = "streamType", default, skip_serializing_if = "Option::is_none")]
    pub stream_type: Option<source_control_sync_job_stream_by_id_properties::StreamType>,
    #[doc = "The text of the sync job stream."]
    #[serde(rename = "streamText", default, skip_serializing_if = "Option::is_none")]
    pub stream_text: Option<String>,
    #[doc = "The values of the job stream."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
impl SourceControlSyncJobStreamByIdProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod source_control_sync_job_stream_by_id_properties {
    use super::*;
    #[doc = "The type of the sync job stream."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StreamType")]
    pub enum StreamType {
        Error,
        Output,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StreamType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StreamType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StreamType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("StreamType", 0u32, "Error"),
                Self::Output => serializer.serialize_unit_variant("StreamType", 1u32, "Output"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Definition of source control sync job stream properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlSyncJobStreamProperties {
    #[doc = "The sync job stream id."]
    #[serde(rename = "sourceControlSyncJobStreamId", default, skip_serializing_if = "Option::is_none")]
    pub source_control_sync_job_stream_id: Option<String>,
    #[doc = "The summary of the sync job stream."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "The time of the sync job stream."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
    #[doc = "The type of the sync job stream."]
    #[serde(rename = "streamType", default, skip_serializing_if = "Option::is_none")]
    pub stream_type: Option<source_control_sync_job_stream_properties::StreamType>,
}
impl SourceControlSyncJobStreamProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod source_control_sync_job_stream_properties {
    use super::*;
    #[doc = "The type of the sync job stream."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StreamType")]
    pub enum StreamType {
        Error,
        Output,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StreamType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StreamType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StreamType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("StreamType", 0u32, "Error"),
                Self::Output => serializer.serialize_unit_variant("StreamType", 1u32, "Output"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response model for the list source control sync job streams operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlSyncJobStreamsListBySyncJob {
    #[doc = "The list of source control sync job streams."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SourceControlSyncJobStream>,
    #[doc = "The next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SourceControlSyncJobStreamsListBySyncJob {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SourceControlSyncJobStreamsListBySyncJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the update source control operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlUpdateParameters {
    #[doc = "The properties of the update source control"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SourceControlUpdateProperties>,
}
impl SourceControlUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the update source control"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceControlUpdateProperties {
    #[doc = "The repo branch of the source control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[doc = "The folder path of the source control. Path must be relative."]
    #[serde(rename = "folderPath", default, skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
    #[doc = "The auto sync of the source control. Default is false."]
    #[serde(rename = "autoSync", default, skip_serializing_if = "Option::is_none")]
    pub auto_sync: Option<bool>,
    #[doc = "The auto publish of the source control. Default is true."]
    #[serde(rename = "publishRunbook", default, skip_serializing_if = "Option::is_none")]
    pub publish_runbook: Option<bool>,
    #[serde(rename = "securityToken", default, skip_serializing_if = "Option::is_none")]
    pub security_token: Option<SourceControlSecurityTokenProperties>,
    #[doc = "The user description of the source control."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl SourceControlUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the statistic."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Statistics {
    #[doc = "Gets the property value of the statistic."]
    #[serde(rename = "counterProperty", default, skip_serializing_if = "Option::is_none")]
    pub counter_property: Option<String>,
    #[doc = "Gets the value of the statistic."]
    #[serde(rename = "counterValue", default, skip_serializing_if = "Option::is_none")]
    pub counter_value: Option<i64>,
    #[doc = "Gets the startTime of the statistic."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the endTime of the statistic."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl Statistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for the list statistics operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatisticsListResult {
    #[doc = "Gets or sets a list of statistics."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Statistics>,
}
impl azure_core::Continuable for StatisticsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl StatisticsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag filter information for the VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagSettingsProperties {
    #[doc = "Dictionary of tags with its list of values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Filter VMs by Any or All specified tags."]
    #[serde(rename = "filterOperator", default, skip_serializing_if = "Option::is_none")]
    pub filter_operator: Option<tag_settings_properties::FilterOperator>,
}
impl TagSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod tag_settings_properties {
    use super::*;
    #[doc = "Filter VMs by Any or All specified tags."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FilterOperator {
        All,
        Any,
    }
}
#[doc = "Group specific to the update configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetProperties {
    #[doc = "List of Azure queries in the software update configuration."]
    #[serde(rename = "azureQueries", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_queries: Vec<AzureQueryProperties>,
    #[doc = "List of non Azure queries in the software update configuration."]
    #[serde(rename = "nonAzureQueries", default, skip_serializing_if = "Vec::is_empty")]
    pub non_azure_queries: Vec<NonAzureQueryProperties>,
}
impl TargetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the test job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestJob {
    #[doc = "Gets or sets the creation time of the test job."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the status of the test job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets the status details of the test job."]
    #[serde(rename = "statusDetails", default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[doc = "Gets or sets the runOn which specifies the group name where the job is to be executed."]
    #[serde(rename = "runOn", default, skip_serializing_if = "Option::is_none")]
    pub run_on: Option<String>,
    #[doc = "Gets or sets the start time of the test job."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the end time of the test job."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the exception of the test job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exception: Option<String>,
    #[doc = "Gets or sets the last modified time of the test job."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last status modified time of the test job."]
    #[serde(rename = "lastStatusModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_status_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the parameters of the test job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "The activity-level tracing options of the runbook."]
    #[serde(rename = "logActivityTrace", default, skip_serializing_if = "Option::is_none")]
    pub log_activity_trace: Option<i32>,
}
impl TestJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create test job operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestJobCreateParameters {
    #[doc = "Gets or sets the parameters of the test job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Gets or sets the runOn which specifies the group name where the job is to be executed."]
    #[serde(rename = "runOn", default, skip_serializing_if = "Option::is_none")]
    pub run_on: Option<String>,
}
impl TestJobCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a ARM tracked top level resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The Azure Region where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a field of a type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TypeField {
    #[doc = "Gets or sets the name of the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the type of the field."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl TypeField {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for the list fields operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TypeFieldListResult {
    #[doc = "Gets or sets a list of fields."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TypeField>,
}
impl azure_core::Continuable for TypeFieldListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl TypeFieldListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of Usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[doc = "Gets or sets the id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Definition of usage counter name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<UsageCounterName>,
    #[doc = "Gets or sets the usage unit name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Gets or sets the current usage value."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[doc = "Gets or sets max limit. -1 for unlimited"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "Gets or sets the throttle status."]
    #[serde(rename = "throttleStatus", default, skip_serializing_if = "Option::is_none")]
    pub throttle_status: Option<String>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of usage counter name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageCounterName {
    #[doc = "Gets or sets the usage counter name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Gets or sets the localized usage counter name."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl UsageCounterName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for the get usage operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageListResult {
    #[doc = "Gets or sets usage."]
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
#[doc = "Definition of the variable."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Variable {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Definition of the variable properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VariableProperties>,
}
impl Variable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create or update variable operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VariableCreateOrUpdateParameters {
    #[doc = "Gets or sets the name of the variable."]
    pub name: String,
    #[doc = "The properties of the create variable operation."]
    pub properties: VariableCreateOrUpdateProperties,
}
impl VariableCreateOrUpdateParameters {
    pub fn new(name: String, properties: VariableCreateOrUpdateProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "The properties of the create variable operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableCreateOrUpdateProperties {
    #[doc = "Gets or sets the value of the variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Gets or sets the description of the variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets the encrypted flag of the variable."]
    #[serde(rename = "isEncrypted", default, skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
}
impl VariableCreateOrUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for the list variables operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableListResult {
    #[doc = "Gets or sets a list of variables."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Variable>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VariableListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VariableListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the variable properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableProperties {
    #[doc = "Gets or sets the value of the variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Gets or sets the encrypted flag of the variable."]
    #[serde(rename = "isEncrypted", default, skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
    #[doc = "Gets or sets the creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl VariableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the update variable operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableUpdateParameters {
    #[doc = "Gets or sets the name of the variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The properties of the update variable"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VariableUpdateProperties>,
}
impl VariableUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the update variable"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VariableUpdateProperties {
    #[doc = "Gets or sets the value of the variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Gets or sets the description of the variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl VariableUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the watcher type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Watcher {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Definition of the watcher properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WatcherProperties>,
    #[doc = "Gets or sets the etag of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl Watcher {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for the list watcher operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WatcherListResult {
    #[doc = "Gets or sets a list of watchers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Watcher>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WatcherListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WatcherListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the watcher properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WatcherProperties {
    #[doc = "Gets or sets the frequency at which the watcher is invoked."]
    #[serde(rename = "executionFrequencyInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub execution_frequency_in_seconds: Option<i64>,
    #[doc = "Gets or sets the name of the script the watcher is attached to, i.e. the name of an existing runbook."]
    #[serde(rename = "scriptName", default, skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,
    #[doc = "Gets or sets the parameters of the script."]
    #[serde(rename = "scriptParameters", default, skip_serializing_if = "Option::is_none")]
    pub script_parameters: Option<serde_json::Value>,
    #[doc = "Gets or sets the name of the hybrid worker group the watcher will run on."]
    #[serde(rename = "scriptRunOn", default, skip_serializing_if = "Option::is_none")]
    pub script_run_on: Option<String>,
    #[doc = "Gets the current status of the watcher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets the creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Details of the user who last modified the watcher."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl WatcherProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WatcherUpdateParameters {
    #[doc = "The properties of the update watcher operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WatcherUpdateProperties>,
    #[doc = "Gets or sets the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl WatcherUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the update watcher operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WatcherUpdateProperties {
    #[doc = "Gets or sets the frequency at which the watcher is invoked."]
    #[serde(rename = "executionFrequencyInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub execution_frequency_in_seconds: Option<i64>,
}
impl WatcherUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the webhook type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Webhook {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Definition of the webhook properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebhookProperties>,
}
impl Webhook {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the create or update webhook operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookCreateOrUpdateParameters {
    #[doc = "Gets or sets the name of the webhook."]
    pub name: String,
    #[doc = "The properties of the create webhook operation."]
    pub properties: WebhookCreateOrUpdateProperties,
}
impl WebhookCreateOrUpdateParameters {
    pub fn new(name: String, properties: WebhookCreateOrUpdateProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "The properties of the create webhook operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebhookCreateOrUpdateProperties {
    #[doc = "Gets or sets the value of the enabled flag of webhook."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Gets or sets the uri."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Gets or sets the expiry time."]
    #[serde(rename = "expiryTime", with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the parameters of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "The runbook property associated with the entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runbook: Option<RunbookAssociationProperty>,
    #[doc = "Gets or sets the name of the hybrid worker group the webhook job will run on."]
    #[serde(rename = "runOn", default, skip_serializing_if = "Option::is_none")]
    pub run_on: Option<String>,
}
impl WebhookCreateOrUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response model for the list webhook operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebhookListResult {
    #[doc = "Gets or sets a list of webhooks."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Webhook>,
    #[doc = "Gets or sets the next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WebhookListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WebhookListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the webhook properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebhookProperties {
    #[doc = "Gets or sets the value of the enabled flag of the webhook."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Gets or sets the webhook uri."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Gets or sets the expiry time."]
    #[serde(rename = "expiryTime", with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last invoked time."]
    #[serde(rename = "lastInvokedTime", with = "azure_core::date::rfc3339::option")]
    pub last_invoked_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the parameters of the job that is created when the webhook calls the runbook it is associated with."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "The runbook property associated with the entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runbook: Option<RunbookAssociationProperty>,
    #[doc = "Gets or sets the name of the hybrid worker group the webhook job will run on."]
    #[serde(rename = "runOn", default, skip_serializing_if = "Option::is_none")]
    pub run_on: Option<String>,
    #[doc = "Gets or sets the creation time."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets the last modified time."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Details of the user who last modified the Webhook"]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "Gets or sets the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl WebhookProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters supplied to the update webhook operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebhookUpdateParameters {
    #[doc = "Gets or sets the name of the webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The properties of the update webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebhookUpdateProperties>,
}
impl WebhookUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the update webhook."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebhookUpdateProperties {
    #[doc = "Gets or sets the value of the enabled flag of webhook."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Gets or sets the name of the hybrid worker group the webhook job will run on."]
    #[serde(rename = "runOn", default, skip_serializing_if = "Option::is_none")]
    pub run_on: Option<String>,
    #[doc = "Gets or sets the parameters of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Gets or sets the description of the webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl WebhookUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Windows specific update configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsProperties {
    #[doc = "Update classification included in the software update configuration. A comma separated string with required values"]
    #[serde(rename = "includedUpdateClassifications", default, skip_serializing_if = "Option::is_none")]
    pub included_update_classifications: Option<windows_properties::IncludedUpdateClassifications>,
    #[doc = "KB numbers excluded from the software update configuration."]
    #[serde(rename = "excludedKbNumbers", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_kb_numbers: Vec<String>,
    #[doc = "KB numbers included from the software update configuration."]
    #[serde(rename = "includedKbNumbers", default, skip_serializing_if = "Vec::is_empty")]
    pub included_kb_numbers: Vec<String>,
    #[doc = "Reboot setting for the software update configuration."]
    #[serde(rename = "rebootSetting", default, skip_serializing_if = "Option::is_none")]
    pub reboot_setting: Option<String>,
}
impl WindowsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod windows_properties {
    use super::*;
    #[doc = "Update classification included in the software update configuration. A comma separated string with required values"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IncludedUpdateClassifications")]
    pub enum IncludedUpdateClassifications {
        Unclassified,
        Critical,
        Security,
        UpdateRollup,
        FeaturePack,
        ServicePack,
        Definition,
        Tools,
        Updates,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IncludedUpdateClassifications {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IncludedUpdateClassifications {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IncludedUpdateClassifications {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unclassified => serializer.serialize_unit_variant("IncludedUpdateClassifications", 0u32, "Unclassified"),
                Self::Critical => serializer.serialize_unit_variant("IncludedUpdateClassifications", 1u32, "Critical"),
                Self::Security => serializer.serialize_unit_variant("IncludedUpdateClassifications", 2u32, "Security"),
                Self::UpdateRollup => serializer.serialize_unit_variant("IncludedUpdateClassifications", 3u32, "UpdateRollup"),
                Self::FeaturePack => serializer.serialize_unit_variant("IncludedUpdateClassifications", 4u32, "FeaturePack"),
                Self::ServicePack => serializer.serialize_unit_variant("IncludedUpdateClassifications", 5u32, "ServicePack"),
                Self::Definition => serializer.serialize_unit_variant("IncludedUpdateClassifications", 6u32, "Definition"),
                Self::Tools => serializer.serialize_unit_variant("IncludedUpdateClassifications", 7u32, "Tools"),
                Self::Updates => serializer.serialize_unit_variant("IncludedUpdateClassifications", 8u32, "Updates"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Software update configuration machine run job navigation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobNavigation {
    #[doc = "Id of the job associated with the software update configuration run"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl JobNavigation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Target operating system for the software update configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperatingSystemType {
    Windows,
    Linux,
}
#[doc = "Gets or sets the frequency of the schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduleFrequency")]
pub enum ScheduleFrequency {
    OneTime,
    Day,
    Hour,
    Week,
    Month,
    Minute,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduleFrequency {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduleFrequency {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduleFrequency {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::OneTime => serializer.serialize_unit_variant("ScheduleFrequency", 0u32, "OneTime"),
            Self::Day => serializer.serialize_unit_variant("ScheduleFrequency", 1u32, "Day"),
            Self::Hour => serializer.serialize_unit_variant("ScheduleFrequency", 2u32, "Hour"),
            Self::Week => serializer.serialize_unit_variant("ScheduleFrequency", 3u32, "Week"),
            Self::Month => serializer.serialize_unit_variant("ScheduleFrequency", 4u32, "Month"),
            Self::Minute => serializer.serialize_unit_variant("ScheduleFrequency", 5u32, "Minute"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Software update configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoftwareUpdateConfiguration {
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Software update configuration properties."]
    pub properties: SoftwareUpdateConfigurationProperties,
}
impl SoftwareUpdateConfiguration {
    pub fn new(properties: SoftwareUpdateConfigurationProperties) -> Self {
        Self {
            name: None,
            id: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "Software update configuration collection item properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoftwareUpdateConfigurationCollectionItem {
    #[doc = "Name of the software update configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource Id of the software update configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Software update configuration collection item properties."]
    pub properties: SoftwareUpdateConfigurationCollectionItemProperties,
}
impl SoftwareUpdateConfigurationCollectionItem {
    pub fn new(properties: SoftwareUpdateConfigurationCollectionItemProperties) -> Self {
        Self {
            name: None,
            id: None,
            properties,
        }
    }
}
#[doc = "Software update configuration collection item properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SoftwareUpdateConfigurationCollectionItemProperties {
    #[doc = "Update specific properties of the software update configuration."]
    #[serde(rename = "updateConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub update_configuration: Option<UpdateConfiguration>,
    #[doc = "Task properties of the software update configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<SoftwareUpdateConfigurationTasks>,
    #[doc = "Gets or sets the frequency of the schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<ScheduleFrequency>,
    #[doc = "the start time of the update."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Creation time of the software update configuration, which only appears in the response."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Last time software update configuration was modified, which only appears in the response."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Provisioning state for the software update configuration, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "ext run time of the update."]
    #[serde(rename = "nextRun", with = "azure_core::date::rfc3339::option")]
    pub next_run: Option<time::OffsetDateTime>,
}
impl SoftwareUpdateConfigurationCollectionItemProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "result of listing all software update configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SoftwareUpdateConfigurationListResult {
    #[doc = "outer object returned when listing all software update configurations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SoftwareUpdateConfigurationCollectionItem>,
}
impl SoftwareUpdateConfigurationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Software update configuration machine run model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SoftwareUpdateConfigurationMachineRun {
    #[doc = "Name of the software update configuration machine run"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource Id of the software update configuration machine run"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Software update configuration machine run properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateConfigurationMachineRunProperties>,
}
impl SoftwareUpdateConfigurationMachineRun {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "result of listing all software update configuration machine runs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SoftwareUpdateConfigurationMachineRunListResult {
    #[doc = "outer object returned when listing all software update configuration machine runs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SoftwareUpdateConfigurationMachineRun>,
    #[doc = "link to next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SoftwareUpdateConfigurationMachineRunListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Software update configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoftwareUpdateConfigurationProperties {
    #[doc = "Update specific properties of the software update configuration."]
    #[serde(rename = "updateConfiguration")]
    pub update_configuration: UpdateConfiguration,
    #[doc = "Definition of schedule parameters."]
    #[serde(rename = "scheduleInfo")]
    pub schedule_info: SucScheduleProperties,
    #[doc = "Provisioning state for the software update configuration, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Error response of an operation failure"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
    #[doc = "Creation time of the resource, which only appears in the response."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "CreatedBy property, which only appears in the response."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Last time resource was modified, which only appears in the response."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "LastModifiedBy property, which only appears in the response."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "Task properties of the software update configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<SoftwareUpdateConfigurationTasks>,
}
impl SoftwareUpdateConfigurationProperties {
    pub fn new(update_configuration: UpdateConfiguration, schedule_info: SucScheduleProperties) -> Self {
        Self {
            update_configuration,
            schedule_info,
            provisioning_state: None,
            error: None,
            creation_time: None,
            created_by: None,
            last_modified_time: None,
            last_modified_by: None,
            tasks: None,
        }
    }
}
#[doc = "Software update configuration Run properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SoftwareUpdateConfigurationRun {
    #[doc = "Name of the software update configuration run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource Id of the software update configuration run"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Software update configuration properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SoftwareUpdateConfigurationRunProperties>,
}
impl SoftwareUpdateConfigurationRun {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "result of listing all software update configuration runs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SoftwareUpdateConfigurationRunListResult {
    #[doc = "outer object returned when listing all software update configuration runs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SoftwareUpdateConfigurationRun>,
    #[doc = "link to next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SoftwareUpdateConfigurationRunListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Software update configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SoftwareUpdateConfigurationRunProperties {
    #[doc = "Software update configuration Run Navigation model."]
    #[serde(rename = "softwareUpdateConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub software_update_configuration: Option<UpdateConfigurationNavigation>,
    #[doc = "Status of the software update configuration run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Configured duration for the software update configuration run."]
    #[serde(rename = "configuredDuration", default, skip_serializing_if = "Option::is_none")]
    pub configured_duration: Option<String>,
    #[doc = "Operating system target of the software update configuration triggered this run"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Start time of the software update configuration run."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the software update configuration run."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Number of computers in the software update configuration run."]
    #[serde(rename = "computerCount", default, skip_serializing_if = "Option::is_none")]
    pub computer_count: Option<i64>,
    #[doc = "Number of computers with failed status."]
    #[serde(rename = "failedCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i64>,
    #[doc = "Creation time of the resource, which only appears in the response."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "CreatedBy property, which only appears in the response."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Last time resource was modified, which only appears in the response."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "LastModifiedBy property, which only appears in the response."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "Software update configuration run tasks model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<SoftwareUpdateConfigurationRunTasks>,
}
impl SoftwareUpdateConfigurationRunProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Task properties of the software update configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SoftwareUpdateConfigurationRunTaskProperties {
    #[doc = "The status of the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The name of the source of the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "The job id of the task."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}
impl SoftwareUpdateConfigurationRunTaskProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Software update configuration run tasks model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SoftwareUpdateConfigurationRunTasks {
    #[doc = "Task properties of the software update configuration."]
    #[serde(rename = "preTask", default, skip_serializing_if = "Option::is_none")]
    pub pre_task: Option<SoftwareUpdateConfigurationRunTaskProperties>,
    #[doc = "Task properties of the software update configuration."]
    #[serde(rename = "postTask", default, skip_serializing_if = "Option::is_none")]
    pub post_task: Option<SoftwareUpdateConfigurationRunTaskProperties>,
}
impl SoftwareUpdateConfigurationRunTasks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Task properties of the software update configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SoftwareUpdateConfigurationTasks {
    #[doc = "Task properties of the software update configuration."]
    #[serde(rename = "preTask", default, skip_serializing_if = "Option::is_none")]
    pub pre_task: Option<TaskProperties>,
    #[doc = "Task properties of the software update configuration."]
    #[serde(rename = "postTask", default, skip_serializing_if = "Option::is_none")]
    pub post_task: Option<TaskProperties>,
}
impl SoftwareUpdateConfigurationTasks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Task properties of the software update configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskProperties {
    #[doc = "Gets or sets the parameters of the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Gets or sets the name of the runbook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
impl TaskProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update specific properties of the software update configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateConfiguration {
    #[doc = "Target operating system for the software update configuration."]
    #[serde(rename = "operatingSystem")]
    pub operating_system: OperatingSystemType,
    #[doc = "Windows specific update configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows: Option<WindowsProperties>,
    #[doc = "Linux specific update configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linux: Option<LinuxProperties>,
    #[doc = "Maximum time allowed for the software update configuration run. Duration needs to be specified using the format PT[n]H[n]M[n]S as per ISO8601"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "List of azure resource Ids for azure virtual machines targeted by the software update configuration."]
    #[serde(rename = "azureVirtualMachines", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_virtual_machines: Vec<String>,
    #[doc = "List of names of non-azure machines targeted by the software update configuration."]
    #[serde(rename = "nonAzureComputerNames", default, skip_serializing_if = "Vec::is_empty")]
    pub non_azure_computer_names: Vec<String>,
    #[doc = "Group specific to the update configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<TargetProperties>,
}
impl UpdateConfiguration {
    pub fn new(operating_system: OperatingSystemType) -> Self {
        Self {
            operating_system,
            windows: None,
            linux: None,
            duration: None,
            azure_virtual_machines: Vec::new(),
            non_azure_computer_names: Vec::new(),
            targets: None,
        }
    }
}
#[doc = "Software update configuration machine run properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateConfigurationMachineRunProperties {
    #[doc = "name of the updated computer"]
    #[serde(rename = "targetComputer", default, skip_serializing_if = "Option::is_none")]
    pub target_computer: Option<String>,
    #[doc = "type of the updated computer."]
    #[serde(rename = "targetComputerType", default, skip_serializing_if = "Option::is_none")]
    pub target_computer_type: Option<String>,
    #[doc = "Software update configuration Run Navigation model."]
    #[serde(rename = "softwareUpdateConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub software_update_configuration: Option<UpdateConfigurationNavigation>,
    #[doc = "Status of the software update configuration machine run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Operating system target of the software update configuration triggered this run"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "correlation id of the software update configuration machine run"]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "source computer id of the software update configuration machine run"]
    #[serde(rename = "sourceComputerId", default, skip_serializing_if = "Option::is_none")]
    pub source_computer_id: Option<String>,
    #[doc = "Start time of the software update configuration machine run."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the software update configuration machine run."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "configured duration for the software update configuration run."]
    #[serde(rename = "configuredDuration", default, skip_serializing_if = "Option::is_none")]
    pub configured_duration: Option<String>,
    #[doc = "Software update configuration machine run job navigation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<JobNavigation>,
    #[doc = "Creation time of the resource, which only appears in the response."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "createdBy property, which only appears in the response."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Last time resource was modified, which only appears in the response."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "lastModifiedBy property, which only appears in the response."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "Error response of an operation failure"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl UpdateConfigurationMachineRunProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Software update configuration Run Navigation model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateConfigurationNavigation {
    #[doc = "Name of the software update configuration triggered the software update configuration run"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl UpdateConfigurationNavigation {
    pub fn new() -> Self {
        Self::default()
    }
}
