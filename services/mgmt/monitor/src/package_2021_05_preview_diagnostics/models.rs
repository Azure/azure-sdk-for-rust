#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Access mode types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AccessMode")]
pub enum AccessMode {
    Open,
    PrivateOnly,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AccessMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AccessMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AccessMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Open => serializer.serialize_unit_variant("AccessMode", 0u32, "Open"),
            Self::PrivateOnly => serializer.serialize_unit_variant("AccessMode", 1u32, "PrivateOnly"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties that define the scope private link mode settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessModeSettings {
    #[doc = "Access mode types."]
    #[serde(rename = "queryAccessMode")]
    pub query_access_mode: AccessMode,
    #[doc = "Access mode types."]
    #[serde(rename = "ingestionAccessMode")]
    pub ingestion_access_mode: AccessMode,
    #[doc = "List of exclusions that override the default access mode settings for specific private endpoint connections."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exclusions: Vec<AccessModeSettingsExclusion>,
}
impl AccessModeSettings {
    pub fn new(query_access_mode: AccessMode, ingestion_access_mode: AccessMode) -> Self {
        Self {
            query_access_mode,
            ingestion_access_mode,
            exclusions: Vec::new(),
        }
    }
}
#[doc = "Properties that define the scope private link mode settings exclusion item. This setting applies to a specific private endpoint connection and overrides the default settings for that private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessModeSettingsExclusion {
    #[doc = "The private endpoint connection name associated to the private endpoint on which we want to apply the specific access mode settings."]
    #[serde(rename = "privateEndpointConnectionName", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint_connection_name: Option<String>,
    #[doc = "Access mode types."]
    #[serde(rename = "queryAccessMode", default, skip_serializing_if = "Option::is_none")]
    pub query_access_mode: Option<AccessMode>,
    #[doc = "Access mode types."]
    #[serde(rename = "ingestionAccessMode", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_access_mode: Option<AccessMode>,
}
impl AccessModeSettingsExclusion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The action detail"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionDetail {
    #[doc = "The mechanism type"]
    #[serde(rename = "MechanismType", default, skip_serializing_if = "Option::is_none")]
    pub mechanism_type: Option<String>,
    #[doc = "The name of the action"]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of the action"]
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The substatus of the action"]
    #[serde(rename = "SubState", default, skip_serializing_if = "Option::is_none")]
    pub sub_state: Option<String>,
    #[doc = "The send time"]
    #[serde(rename = "SendTime", default, skip_serializing_if = "Option::is_none")]
    pub send_time: Option<String>,
    #[doc = "The detail of the friendly error message"]
    #[serde(rename = "Detail", default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}
impl ActionDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure action group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroup {
    #[doc = "The short name of the action group. This will be used in SMS messages."]
    #[serde(rename = "groupShortName")]
    pub group_short_name: String,
    #[doc = "Indicates whether this action group is enabled. If an action group is not enabled, then none of its receivers will receive communications."]
    pub enabled: bool,
    #[doc = "The list of email receivers that are part of this action group."]
    #[serde(
        rename = "emailReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub email_receivers: Vec<EmailReceiver>,
    #[doc = "The list of SMS receivers that are part of this action group."]
    #[serde(
        rename = "smsReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sms_receivers: Vec<SmsReceiver>,
    #[doc = "The list of webhook receivers that are part of this action group."]
    #[serde(
        rename = "webhookReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub webhook_receivers: Vec<WebhookReceiver>,
    #[doc = "The list of ITSM receivers that are part of this action group."]
    #[serde(
        rename = "itsmReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub itsm_receivers: Vec<ItsmReceiver>,
    #[doc = "The list of AzureAppPush receivers that are part of this action group."]
    #[serde(
        rename = "azureAppPushReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub azure_app_push_receivers: Vec<AzureAppPushReceiver>,
    #[doc = "The list of AutomationRunbook receivers that are part of this action group."]
    #[serde(
        rename = "automationRunbookReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub automation_runbook_receivers: Vec<AutomationRunbookReceiver>,
    #[doc = "The list of voice receivers that are part of this action group."]
    #[serde(
        rename = "voiceReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub voice_receivers: Vec<VoiceReceiver>,
    #[doc = "The list of logic app receivers that are part of this action group."]
    #[serde(
        rename = "logicAppReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub logic_app_receivers: Vec<LogicAppReceiver>,
    #[doc = "The list of azure function receivers that are part of this action group."]
    #[serde(
        rename = "azureFunctionReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub azure_function_receivers: Vec<AzureFunctionReceiver>,
    #[doc = "The list of ARM role receivers that are part of this action group. Roles are Azure RBAC roles and only built-in roles are supported."]
    #[serde(
        rename = "armRoleReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub arm_role_receivers: Vec<ArmRoleReceiver>,
    #[doc = "The list of event hub receivers that are part of this action group."]
    #[serde(
        rename = "eventHubReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub event_hub_receivers: Vec<EventHubReceiver>,
}
impl ActionGroup {
    pub fn new(group_short_name: String, enabled: bool) -> Self {
        Self {
            group_short_name,
            enabled,
            email_receivers: Vec::new(),
            sms_receivers: Vec::new(),
            webhook_receivers: Vec::new(),
            itsm_receivers: Vec::new(),
            azure_app_push_receivers: Vec::new(),
            automation_runbook_receivers: Vec::new(),
            voice_receivers: Vec::new(),
            logic_app_receivers: Vec::new(),
            azure_function_receivers: Vec::new(),
            arm_role_receivers: Vec::new(),
            event_hub_receivers: Vec::new(),
        }
    }
}
#[doc = "A list of action groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionGroupList {
    #[doc = "The list of action groups."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ActionGroupResource>,
    #[doc = "Provides the link to retrieve the next set of elements."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ActionGroupList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ActionGroupList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure action group for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionGroupPatch {
    #[doc = "Indicates whether this action group is enabled. If an action group is not enabled, then none of its actions will be activated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ActionGroupPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An action group object for the body of patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionGroupPatchBody {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "An Azure action group for patch operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroupPatch>,
}
impl ActionGroupPatchBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An action group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupResource {
    #[serde(flatten)]
    pub azure_resource: AzureResource,
    #[doc = "An Azure action group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroup>,
}
impl ActionGroupResource {
    pub fn new(azure_resource: AzureResource) -> Self {
        Self {
            azure_resource,
            properties: None,
        }
    }
}
#[doc = "An arm role receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmRoleReceiver {
    #[doc = "The name of the arm role receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The arm role id."]
    #[serde(rename = "roleId")]
    pub role_id: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
impl ArmRoleReceiver {
    pub fn new(name: String, role_id: String) -> Self {
        Self {
            name,
            role_id,
            use_common_alert_schema: None,
        }
    }
}
#[doc = "The Azure Automation Runbook notification receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRunbookReceiver {
    #[doc = "The Azure automation account Id which holds this runbook and authenticate to Azure resource."]
    #[serde(rename = "automationAccountId")]
    pub automation_account_id: String,
    #[doc = "The name for this runbook."]
    #[serde(rename = "runbookName")]
    pub runbook_name: String,
    #[doc = "The resource id for webhook linked to this runbook."]
    #[serde(rename = "webhookResourceId")]
    pub webhook_resource_id: String,
    #[doc = "Indicates whether this instance is global runbook."]
    #[serde(rename = "isGlobalRunbook")]
    pub is_global_runbook: bool,
    #[doc = "Indicates name of the webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The URI where webhooks should be sent."]
    #[serde(rename = "serviceUri", default, skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
impl AutomationRunbookReceiver {
    pub fn new(automation_account_id: String, runbook_name: String, webhook_resource_id: String, is_global_runbook: bool) -> Self {
        Self {
            automation_account_id,
            runbook_name,
            webhook_resource_id,
            is_global_runbook,
            name: None,
            service_uri: None,
            use_common_alert_schema: None,
        }
    }
}
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoscaleErrorResponse {
    #[doc = "The error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<autoscale_error_response::Error>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl azure_core::Continuable for AutoscaleErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AutoscaleErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod autoscale_error_response {
    use super::*;
    #[doc = "The error object."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "One of a server-defined set of error codes."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "A human-readable representation of the error."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "The target of the particular error."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,
        #[doc = "A human-readable representation of the error's details."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub details: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Autoscale notification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleNotification {
    #[doc = "the operation associated with the notification and its value must be \"scale\""]
    pub operation: autoscale_notification::Operation,
    #[doc = "Email notification of an autoscale event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailNotification>,
    #[doc = "the collection of webhook notifications."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub webhooks: Vec<WebhookNotification>,
}
impl AutoscaleNotification {
    pub fn new(operation: autoscale_notification::Operation) -> Self {
        Self {
            operation,
            email: None,
            webhooks: Vec::new(),
        }
    }
}
pub mod autoscale_notification {
    use super::*;
    #[doc = "the operation associated with the notification and its value must be \"scale\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operation {
        Scale,
    }
}
#[doc = "Autoscale profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleProfile {
    #[doc = "the name of the profile."]
    pub name: String,
    #[doc = "The number of instances that can be used during this profile."]
    pub capacity: ScaleCapacity,
    #[doc = "the collection of rules that provide the triggers and parameters for the scaling action. A maximum of 10 rules can be specified."]
    pub rules: Vec<ScaleRule>,
    #[doc = "A specific date-time for the profile."]
    #[serde(rename = "fixedDate", default, skip_serializing_if = "Option::is_none")]
    pub fixed_date: Option<TimeWindow>,
    #[doc = "The repeating times at which this profile begins. This element is not used if the FixedDate element is used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<Recurrence>,
}
impl AutoscaleProfile {
    pub fn new(name: String, capacity: ScaleCapacity, rules: Vec<ScaleRule>) -> Self {
        Self {
            name,
            capacity,
            rules,
            fixed_date: None,
            recurrence: None,
        }
    }
}
#[doc = "A setting that contains all of the configuration for the automatic scaling of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSetting {
    #[doc = "the collection of automatic scaling profiles that specify different scaling parameters for different time periods. A maximum of 20 profiles can be specified."]
    pub profiles: Vec<AutoscaleProfile>,
    #[doc = "the collection of notifications."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub notifications: Vec<AutoscaleNotification>,
    #[doc = "the enabled flag. Specifies whether automatic scaling is enabled for the resource. The default value is 'false'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The parameters for enabling predictive autoscale."]
    #[serde(rename = "predictiveAutoscalePolicy", default, skip_serializing_if = "Option::is_none")]
    pub predictive_autoscale_policy: Option<PredictiveAutoscalePolicy>,
    #[doc = "the name of the autoscale setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "the resource identifier of the resource that the autoscale setting should be added to."]
    #[serde(rename = "targetResourceUri", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_uri: Option<String>,
    #[doc = "the location of the resource that the autoscale setting should be added to."]
    #[serde(rename = "targetResourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_location: Option<String>,
}
impl AutoscaleSetting {
    pub fn new(profiles: Vec<AutoscaleProfile>) -> Self {
        Self {
            profiles,
            notifications: Vec::new(),
            enabled: None,
            predictive_autoscale_policy: None,
            name: None,
            target_resource_uri: None,
            target_resource_location: None,
        }
    }
}
#[doc = "The autoscale setting resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSettingResource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Gets or sets a list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key no greater in length than 128 characters and a value no greater in length than 256 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "A setting that contains all of the configuration for the automatic scaling of a resource."]
    pub properties: AutoscaleSetting,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl AutoscaleSettingResource {
    pub fn new(location: String, properties: AutoscaleSetting) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
            properties,
            system_data: None,
        }
    }
}
#[doc = "Represents a collection of autoscale setting resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSettingResourceCollection {
    #[doc = "the values for the autoscale setting resources."]
    pub value: Vec<AutoscaleSettingResource>,
    #[doc = "URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AutoscaleSettingResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AutoscaleSettingResourceCollection {
    pub fn new(value: Vec<AutoscaleSettingResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The autoscale setting object for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoscaleSettingResourcePatch {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "A setting that contains all of the configuration for the automatic scaling of a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AutoscaleSetting>,
}
impl AutoscaleSettingResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Azure mobile App push notification receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureAppPushReceiver {
    #[doc = "The name of the Azure mobile app push receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The email address registered for the Azure mobile app."]
    #[serde(rename = "emailAddress")]
    pub email_address: String,
}
impl AzureAppPushReceiver {
    pub fn new(name: String, email_address: String) -> Self {
        Self { name, email_address }
    }
}
#[doc = "An azure function receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFunctionReceiver {
    #[doc = "The name of the azure function receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The azure resource id of the function app."]
    #[serde(rename = "functionAppResourceId")]
    pub function_app_resource_id: String,
    #[doc = "The function name in the function app."]
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[doc = "The http trigger url where http request sent to."]
    #[serde(rename = "httpTriggerUrl")]
    pub http_trigger_url: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
impl AzureFunctionReceiver {
    pub fn new(name: String, function_app_resource_id: String, function_name: String, http_trigger_url: String) -> Self {
        Self {
            name,
            function_app_resource_id,
            function_name,
            http_trigger_url,
            use_common_alert_schema: None,
        }
    }
}
#[doc = "An Azure Monitor PrivateLinkScope definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMonitorPrivateLinkScope {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties that define a Azure Monitor PrivateLinkScope resource."]
    pub properties: AzureMonitorPrivateLinkScopeProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl AzureMonitorPrivateLinkScope {
    pub fn new(tracked_resource: TrackedResource, properties: AzureMonitorPrivateLinkScopeProperties) -> Self {
        Self {
            tracked_resource,
            properties,
            system_data: None,
        }
    }
}
#[doc = "Describes the list of Azure Monitor PrivateLinkScope resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMonitorPrivateLinkScopeListResult {
    #[doc = "List of Azure Monitor PrivateLinkScope definitions."]
    pub value: Vec<AzureMonitorPrivateLinkScope>,
    #[doc = "The URI to get the next set of Azure Monitor PrivateLinkScope definitions if too many PrivateLinkScopes where returned in the result set."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureMonitorPrivateLinkScopeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AzureMonitorPrivateLinkScopeListResult {
    pub fn new(value: Vec<AzureMonitorPrivateLinkScope>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties that define a Azure Monitor PrivateLinkScope resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMonitorPrivateLinkScopeProperties {
    #[doc = "Current state of this PrivateLinkScope: whether or not is has been provisioned within the resource group it is defined. Users cannot change this value but are able to read from it. Values will include Provisioning ,Succeeded, Canceled and Failed."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "List of private endpoint connections."]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Properties that define the scope private link mode settings."]
    #[serde(rename = "accessModeSettings")]
    pub access_mode_settings: AccessModeSettings,
}
impl AzureMonitorPrivateLinkScopeProperties {
    pub fn new(access_mode_settings: AccessModeSettings) -> Self {
        Self {
            provisioning_state: None,
            private_endpoint_connections: Vec::new(),
            access_mode_settings,
        }
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AzureResource {
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
#[doc = "The context info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Context {
    #[doc = "The source of the notification request"]
    #[serde(rename = "notificationSource", default, skip_serializing_if = "Option::is_none")]
    pub notification_source: Option<String>,
    #[doc = "The context id type"]
    #[serde(rename = "contextType", default, skip_serializing_if = "Option::is_none")]
    pub context_type: Option<String>,
}
impl Context {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl azure_core::Continuable for DefaultErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DefaultErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The diagnostic settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettings {
    #[doc = "The resource ID of the storage account to which you would like to send Diagnostic Logs."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "The service bus rule Id of the diagnostic setting. This is here to maintain backwards compatibility."]
    #[serde(rename = "serviceBusRuleId", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_rule_id: Option<String>,
    #[doc = "The resource Id for the event hub authorization rule."]
    #[serde(rename = "eventHubAuthorizationRuleId", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_authorization_rule_id: Option<String>,
    #[doc = "The name of the event hub. If none is specified, the default event hub will be selected."]
    #[serde(rename = "eventHubName", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_name: Option<String>,
    #[doc = "The list of metric settings."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metrics: Vec<MetricSettings>,
    #[doc = "The list of logs settings."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub logs: Vec<LogSettings>,
    #[doc = "The full ARM resource ID of the Log Analytics workspace to which you would like to send Diagnostic Logs. Example: /subscriptions/4b9e8510-67ab-4e9a-95a9-e2f1e570ea9c/resourceGroups/insights-integration/providers/Microsoft.OperationalInsights/workspaces/viruela2"]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The full ARM resource ID of the Marketplace resource to which you would like to send Diagnostic Logs."]
    #[serde(rename = "marketplacePartnerId", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_partner_id: Option<String>,
    #[doc = "A string indicating whether the export to Log Analytics should use the default destination type, i.e. AzureDiagnostics, or use a destination type constructed as follows: <normalized service identity>_<normalized category name>. Possible values are: Dedicated and null (null is default.)"]
    #[serde(rename = "logAnalyticsDestinationType", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics_destination_type: Option<String>,
}
impl DiagnosticSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The diagnostic settings Category."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettingsCategory {
    #[doc = "The type of the diagnostic settings category."]
    #[serde(rename = "categoryType", default, skip_serializing_if = "Option::is_none")]
    pub category_type: Option<diagnostic_settings_category::CategoryType>,
    #[doc = "the collection of what category groups are supported."]
    #[serde(
        rename = "categoryGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub category_groups: Vec<String>,
}
impl DiagnosticSettingsCategory {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod diagnostic_settings_category {
    use super::*;
    #[doc = "The type of the diagnostic settings category."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CategoryType")]
    pub enum CategoryType {
        Metrics,
        Logs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CategoryType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CategoryType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CategoryType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Metrics => serializer.serialize_unit_variant("CategoryType", 0u32, "Metrics"),
                Self::Logs => serializer.serialize_unit_variant("CategoryType", 1u32, "Logs"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The diagnostic settings category resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettingsCategoryResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The diagnostic settings Category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticSettingsCategory>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DiagnosticSettingsCategoryResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a collection of diagnostic setting category resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettingsCategoryResourceCollection {
    #[doc = "The collection of diagnostic settings category resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DiagnosticSettingsCategoryResource>,
}
impl azure_core::Continuable for DiagnosticSettingsCategoryResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DiagnosticSettingsCategoryResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The diagnostic setting resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettingsResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The diagnostic settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiagnosticSettings>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DiagnosticSettingsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a collection of alert rule resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticSettingsResourceCollection {
    #[doc = "The collection of diagnostic settings resources;."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DiagnosticSettingsResource>,
}
impl azure_core::Continuable for DiagnosticSettingsResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DiagnosticSettingsResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Email notification of an autoscale event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailNotification {
    #[doc = "a value indicating whether to send email to subscription administrator."]
    #[serde(rename = "sendToSubscriptionAdministrator", default, skip_serializing_if = "Option::is_none")]
    pub send_to_subscription_administrator: Option<bool>,
    #[doc = "a value indicating whether to send email to subscription co-administrators."]
    #[serde(rename = "sendToSubscriptionCoAdministrators", default, skip_serializing_if = "Option::is_none")]
    pub send_to_subscription_co_administrators: Option<bool>,
    #[doc = "the custom e-mails list. This value can be null or empty, in which case this attribute will be ignored."]
    #[serde(
        rename = "customEmails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_emails: Vec<String>,
}
impl EmailNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An email receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailReceiver {
    #[doc = "The name of the email receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The email address of this receiver."]
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
    #[doc = "Indicates the status of the receiver. Receivers that are not Enabled will not receive any communications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
impl EmailReceiver {
    pub fn new(name: String, email_address: String) -> Self {
        Self {
            name,
            email_address,
            use_common_alert_schema: None,
            status: None,
        }
    }
}
#[doc = "Describes a receiver that should be resubscribed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableRequest {
    #[doc = "The name of the receiver to resubscribe."]
    #[serde(rename = "receiverName")]
    pub receiver_name: String,
}
impl EnableRequest {
    pub fn new(receiver_name: String) -> Self {
        Self { receiver_name }
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
#[doc = "Describes the format of Error response."]
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
#[doc = "An Event hub receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubReceiver {
    #[doc = "The name of the Event hub receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The Event Hub namespace"]
    #[serde(rename = "eventHubNameSpace")]
    pub event_hub_name_space: String,
    #[doc = "The name of the specific Event Hub queue"]
    #[serde(rename = "eventHubName")]
    pub event_hub_name: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
    #[doc = "The tenant Id for the subscription containing this event hub"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The Id for the subscription containing this event hub"]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl EventHubReceiver {
    pub fn new(name: String, event_hub_name_space: String, event_hub_name: String, subscription_id: String) -> Self {
        Self {
            name,
            event_hub_name_space,
            event_hub_name,
            use_common_alert_schema: None,
            tenant_id: None,
            subscription_id,
        }
    }
}
#[doc = "An Itsm receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItsmReceiver {
    #[doc = "The name of the Itsm receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "OMS LA instance identifier."]
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[doc = "Unique identification of ITSM connection among multiple defined in above workspace."]
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    #[doc = "JSON blob for the configurations of the ITSM action. CreateMultipleWorkItems option will be part of this blob as well."]
    #[serde(rename = "ticketConfiguration")]
    pub ticket_configuration: String,
    #[doc = "Region in which workspace resides. Supported values:'centralindia','japaneast','southeastasia','australiasoutheast','uksouth','westcentralus','canadacentral','eastus','westeurope'"]
    pub region: String,
}
impl ItsmReceiver {
    pub fn new(name: String, workspace_id: String, connection_id: String, ticket_configuration: String, region: String) -> Self {
        Self {
            name,
            workspace_id,
            connection_id,
            ticket_configuration,
            region,
        }
    }
}
#[doc = "Part of MultiTenantDiagnosticSettings. Specifies the settings for a particular log."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSettings {
    #[doc = "Name of a Diagnostic Log category for a resource type this setting is applied to. To obtain the list of Diagnostic Log categories for a resource, first perform a GET diagnostic settings operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Name of a Diagnostic Log category group for a resource type this setting is applied to. To obtain the list of Diagnostic Log categories for a resource, first perform a GET diagnostic settings operation."]
    #[serde(rename = "categoryGroup", default, skip_serializing_if = "Option::is_none")]
    pub category_group: Option<String>,
    #[doc = "a value indicating whether this log is enabled."]
    pub enabled: bool,
    #[doc = "Specifies the retention policy for the log."]
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
impl LogSettings {
    pub fn new(enabled: bool) -> Self {
        Self {
            category: None,
            category_group: None,
            enabled,
            retention_policy: None,
        }
    }
}
#[doc = "A logic app receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogicAppReceiver {
    #[doc = "The name of the logic app receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The azure resource id of the logic app receiver."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "The callback url where http request sent to."]
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
impl LogicAppReceiver {
    pub fn new(name: String, resource_id: String, callback_url: String) -> Self {
        Self {
            name,
            resource_id,
            callback_url,
            use_common_alert_schema: None,
        }
    }
}
#[doc = "The management group diagnostic settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupDiagnosticSettings {
    #[doc = "The resource ID of the storage account to which you would like to send Diagnostic Logs."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "The service bus rule Id of the diagnostic setting. This is here to maintain backwards compatibility."]
    #[serde(rename = "serviceBusRuleId", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_rule_id: Option<String>,
    #[doc = "The resource Id for the event hub authorization rule."]
    #[serde(rename = "eventHubAuthorizationRuleId", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_authorization_rule_id: Option<String>,
    #[doc = "The name of the event hub. If none is specified, the default event hub will be selected."]
    #[serde(rename = "eventHubName", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_name: Option<String>,
    #[doc = "The list of logs settings."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub logs: Vec<ManagementGroupLogSettings>,
    #[doc = "The full ARM resource ID of the Log Analytics workspace to which you would like to send Diagnostic Logs. Example: /subscriptions/4b9e8510-67ab-4e9a-95a9-e2f1e570ea9c/resourceGroups/insights-integration/providers/Microsoft.OperationalInsights/workspaces/viruela2"]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The full ARM resource ID of the Marketplace resource to which you would like to send Diagnostic Logs."]
    #[serde(rename = "marketplacePartnerId", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_partner_id: Option<String>,
}
impl ManagementGroupDiagnosticSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The management group diagnostic setting resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupDiagnosticSettingsResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The management group diagnostic settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementGroupDiagnosticSettings>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ManagementGroupDiagnosticSettingsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a collection of management group diagnostic settings resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupDiagnosticSettingsResourceCollection {
    #[doc = "The collection of management group diagnostic settings resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ManagementGroupDiagnosticSettingsResource>,
}
impl azure_core::Continuable for ManagementGroupDiagnosticSettingsResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ManagementGroupDiagnosticSettingsResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Part of Management Group diagnostic setting. Specifies the settings for a particular log."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementGroupLogSettings {
    #[doc = "Name of a Management Group Diagnostic Log category for a resource type this setting is applied to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Name of a Management Group Diagnostic Log category group for a resource type this setting is applied to."]
    #[serde(rename = "categoryGroup", default, skip_serializing_if = "Option::is_none")]
    pub category_group: Option<String>,
    #[doc = "a value indicating whether this log is enabled."]
    pub enabled: bool,
}
impl ManagementGroupLogSettings {
    pub fn new(enabled: bool) -> Self {
        Self {
            category: None,
            category_group: None,
            enabled,
        }
    }
}
#[doc = "Part of MultiTenantDiagnosticSettings. Specifies the settings for a particular metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSettings {
    #[doc = "the timegrain of the metric in ISO8601 format."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "Name of a Diagnostic Metric category for a resource type this setting is applied to. To obtain the list of Diagnostic metric categories for a resource, first perform a GET diagnostic settings operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "a value indicating whether this category is enabled."]
    pub enabled: bool,
    #[doc = "Specifies the retention policy for the log."]
    #[serde(rename = "retentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
impl MetricSettings {
    pub fn new(enabled: bool) -> Self {
        Self {
            time_grain: None,
            category: None,
            enabled,
            retention_policy: None,
        }
    }
}
#[doc = "The trigger that results in a scaling action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricTrigger {
    #[doc = "the name of the metric that defines what the rule monitors."]
    #[serde(rename = "metricName")]
    pub metric_name: String,
    #[doc = "the namespace of the metric that defines what the rule monitors."]
    #[serde(rename = "metricNamespace", default, skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    #[doc = "the resource identifier of the resource the rule monitors."]
    #[serde(rename = "metricResourceUri")]
    pub metric_resource_uri: String,
    #[doc = "the location of the resource the rule monitors."]
    #[serde(rename = "metricResourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub metric_resource_location: Option<String>,
    #[doc = "the granularity of metrics the rule monitors. Must be one of the predefined values returned from metric definitions for the metric. Must be between 12 hours and 1 minute."]
    #[serde(rename = "timeGrain")]
    pub time_grain: String,
    #[doc = "the metric statistic type. How the metrics from multiple instances are combined."]
    pub statistic: metric_trigger::Statistic,
    #[doc = "the range of time in which instance data is collected. This value must be greater than the delay in metric collection, which can vary from resource-to-resource. Must be between 12 hours and 5 minutes."]
    #[serde(rename = "timeWindow")]
    pub time_window: String,
    #[doc = "time aggregation type. How the data that is collected should be combined over time. The default value is Average."]
    #[serde(rename = "timeAggregation")]
    pub time_aggregation: metric_trigger::TimeAggregation,
    #[doc = "the operator that is used to compare the metric data and the threshold."]
    pub operator: metric_trigger::Operator,
    #[doc = "the threshold of the metric that triggers the scale action."]
    pub threshold: f64,
    #[doc = "List of dimension conditions. For example: [{\"DimensionName\":\"AppName\",\"Operator\":\"Equals\",\"Values\":[\"App1\"]},{\"DimensionName\":\"Deployment\",\"Operator\":\"Equals\",\"Values\":[\"default\"]}]."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<ScaleRuleMetricDimension>,
    #[doc = "a value indicating whether metric should divide per instance."]
    #[serde(rename = "dividePerInstance", default, skip_serializing_if = "Option::is_none")]
    pub divide_per_instance: Option<bool>,
}
impl MetricTrigger {
    pub fn new(
        metric_name: String,
        metric_resource_uri: String,
        time_grain: String,
        statistic: metric_trigger::Statistic,
        time_window: String,
        time_aggregation: metric_trigger::TimeAggregation,
        operator: metric_trigger::Operator,
        threshold: f64,
    ) -> Self {
        Self {
            metric_name,
            metric_namespace: None,
            metric_resource_uri,
            metric_resource_location: None,
            time_grain,
            statistic,
            time_window,
            time_aggregation,
            operator,
            threshold,
            dimensions: Vec::new(),
            divide_per_instance: None,
        }
    }
}
pub mod metric_trigger {
    use super::*;
    #[doc = "the metric statistic type. How the metrics from multiple instances are combined."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Statistic {
        Average,
        Min,
        Max,
        Sum,
        Count,
    }
    #[doc = "time aggregation type. How the data that is collected should be combined over time. The default value is Average."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TimeAggregation {
        Average,
        Minimum,
        Maximum,
        Total,
        Count,
        Last,
    }
    #[doc = "the operator that is used to compare the metric data and the threshold."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Equals,
        NotEquals,
        GreaterThan,
        GreaterThanOrEqual,
        LessThan,
        LessThanOrEqual,
    }
}
#[doc = "The request body which contain contact detail metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationRequestBody {
    #[doc = "The value of the supported alert type. Supported alert type values are: servicehealth, metricstaticthreshold, metricsdynamicthreshold, logalertv2, smartalert, webtestalert, logalertv1numresult, logalertv1metricmeasurement, resourcehealth, activitylog, actualcostbudget, forecastedbudget"]
    #[serde(rename = "alertType")]
    pub alert_type: String,
    #[doc = "The list of email receivers that are part of this action group."]
    #[serde(
        rename = "emailReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub email_receivers: Vec<EmailReceiver>,
    #[doc = "The list of SMS receivers that are part of this action group."]
    #[serde(
        rename = "smsReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sms_receivers: Vec<SmsReceiver>,
    #[doc = "The list of webhook receivers that are part of this action group."]
    #[serde(
        rename = "webhookReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub webhook_receivers: Vec<WebhookReceiver>,
    #[doc = "The list of ITSM receivers that are part of this action group."]
    #[serde(
        rename = "itsmReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub itsm_receivers: Vec<ItsmReceiver>,
    #[doc = "The list of AzureAppPush receivers that are part of this action group."]
    #[serde(
        rename = "azureAppPushReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub azure_app_push_receivers: Vec<AzureAppPushReceiver>,
    #[doc = "The list of AutomationRunbook receivers that are part of this action group."]
    #[serde(
        rename = "automationRunbookReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub automation_runbook_receivers: Vec<AutomationRunbookReceiver>,
    #[doc = "The list of voice receivers that are part of this action group."]
    #[serde(
        rename = "voiceReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub voice_receivers: Vec<VoiceReceiver>,
    #[doc = "The list of logic app receivers that are part of this action group."]
    #[serde(
        rename = "logicAppReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub logic_app_receivers: Vec<LogicAppReceiver>,
    #[doc = "The list of azure function receivers that are part of this action group."]
    #[serde(
        rename = "azureFunctionReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub azure_function_receivers: Vec<AzureFunctionReceiver>,
    #[doc = "The list of ARM role receivers that are part of this action group. Roles are Azure RBAC roles and only built-in roles are supported."]
    #[serde(
        rename = "armRoleReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub arm_role_receivers: Vec<ArmRoleReceiver>,
    #[doc = "The list of event hub receivers that are part of this action group."]
    #[serde(
        rename = "eventHubReceivers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub event_hub_receivers: Vec<EventHubReceiver>,
}
impl NotificationRequestBody {
    pub fn new(alert_type: String) -> Self {
        Self {
            alert_type,
            email_receivers: Vec::new(),
            sms_receivers: Vec::new(),
            webhook_receivers: Vec::new(),
            itsm_receivers: Vec::new(),
            azure_app_push_receivers: Vec::new(),
            automation_runbook_receivers: Vec::new(),
            voice_receivers: Vec::new(),
            logic_app_receivers: Vec::new(),
            azure_function_receivers: Vec::new(),
            arm_role_receivers: Vec::new(),
            event_hub_receivers: Vec::new(),
        }
    }
}
#[doc = "The status of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "The operation Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Start time of the job in standard ISO8601 format."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the job in standard ISO8601 format."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for enabling predictive autoscale."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PredictiveAutoscalePolicy {
    #[doc = "the predictive autoscale mode"]
    #[serde(rename = "scaleMode")]
    pub scale_mode: predictive_autoscale_policy::ScaleMode,
    #[doc = "the amount of time to specify by which instances are launched in advance. It must be between 1 minute and 60 minutes in ISO 8601 format."]
    #[serde(rename = "scaleLookAheadTime", default, skip_serializing_if = "Option::is_none")]
    pub scale_look_ahead_time: Option<String>,
}
impl PredictiveAutoscalePolicy {
    pub fn new(scale_mode: predictive_autoscale_policy::ScaleMode) -> Self {
        Self {
            scale_mode,
            scale_look_ahead_time: None,
        }
    }
}
pub mod predictive_autoscale_policy {
    use super::*;
    #[doc = "the predictive autoscale mode"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScaleMode {
        Disabled,
        ForecastOnly,
        Enabled,
    }
}
#[doc = "The response to a metrics query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PredictiveResponse {
    #[doc = "The timespan for which the data was retrieved. Its value consists of two datetimes concatenated, separated by '/'.  This may be adjusted in the future and returned back from what was originally requested."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timespan: Option<String>,
    #[doc = "The interval (window size) for which the metric data was returned in.  This may be adjusted in the future and returned back from what was originally requested.  This is not present if a metadata request was made."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[doc = "The metrics being queried"]
    #[serde(rename = "metricName", default, skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[doc = "resource of the predictive metric."]
    #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
    #[doc = "the value of the collection."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data: Vec<PredictiveValue>,
}
impl PredictiveResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a predictive metric value in the given bucket."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PredictiveValue {
    #[doc = "the timestamp for the metric value in ISO 8601 format."]
    #[serde(rename = "timeStamp", with = "azure_core::date::rfc3339")]
    pub time_stamp: time::OffsetDateTime,
    #[doc = "Predictive value in this time bucket."]
    pub value: f64,
}
impl PredictiveValue {
    pub fn new(time_stamp: time::OffsetDateTime, value: f64) -> Self {
        Self { time_stamp, value }
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
#[doc = "Indicates the status of the receiver. Receivers that are not Enabled will not receive any communications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReceiverStatus {
    NotSpecified,
    Enabled,
    Disabled,
}
#[doc = "The repeating times at which this profile begins. This element is not used if the FixedDate element is used."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recurrence {
    #[doc = "the recurrence frequency. How often the schedule profile should take effect. This value must be Week, meaning each week will have the same set of profiles. For example, to set a daily schedule, set **schedule** to every day of the week. The frequency property specifies that the schedule is repeated weekly."]
    pub frequency: recurrence::Frequency,
    #[doc = "The scheduling constraints for when the profile begins."]
    pub schedule: RecurrentSchedule,
}
impl Recurrence {
    pub fn new(frequency: recurrence::Frequency, schedule: RecurrentSchedule) -> Self {
        Self { frequency, schedule }
    }
}
pub mod recurrence {
    use super::*;
    #[doc = "the recurrence frequency. How often the schedule profile should take effect. This value must be Week, meaning each week will have the same set of profiles. For example, to set a daily schedule, set **schedule** to every day of the week. The frequency property specifies that the schedule is repeated weekly."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Frequency {
        None,
        Second,
        Minute,
        Hour,
        Day,
        Week,
        Month,
        Year,
    }
}
#[doc = "The scheduling constraints for when the profile begins."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurrentSchedule {
    #[doc = "the timezone for the hours of the profile. Some examples of valid time zones are: Dateline Standard Time, UTC-11, Hawaiian Standard Time, Alaskan Standard Time, Pacific Standard Time (Mexico), Pacific Standard Time, US Mountain Standard Time, Mountain Standard Time (Mexico), Mountain Standard Time, Central America Standard Time, Central Standard Time, Central Standard Time (Mexico), Canada Central Standard Time, SA Pacific Standard Time, Eastern Standard Time, US Eastern Standard Time, Venezuela Standard Time, Paraguay Standard Time, Atlantic Standard Time, Central Brazilian Standard Time, SA Western Standard Time, Pacific SA Standard Time, Newfoundland Standard Time, E. South America Standard Time, Argentina Standard Time, SA Eastern Standard Time, Greenland Standard Time, Montevideo Standard Time, Bahia Standard Time, UTC-02, Mid-Atlantic Standard Time, Azores Standard Time, Cape Verde Standard Time, Morocco Standard Time, UTC, GMT Standard Time, Greenwich Standard Time, W. Europe Standard Time, Central Europe Standard Time, Romance Standard Time, Central European Standard Time, W. Central Africa Standard Time, Namibia Standard Time, Jordan Standard Time, GTB Standard Time, Middle East Standard Time, Egypt Standard Time, Syria Standard Time, E. Europe Standard Time, South Africa Standard Time, FLE Standard Time, Turkey Standard Time, Israel Standard Time, Kaliningrad Standard Time, Libya Standard Time, Arabic Standard Time, Arab Standard Time, Belarus Standard Time, Russian Standard Time, E. Africa Standard Time, Iran Standard Time, Arabian Standard Time, Azerbaijan Standard Time, Russia Time Zone 3, Mauritius Standard Time, Georgian Standard Time, Caucasus Standard Time, Afghanistan Standard Time, West Asia Standard Time, Ekaterinburg Standard Time, Pakistan Standard Time, India Standard Time, Sri Lanka Standard Time, Nepal Standard Time, Central Asia Standard Time, Bangladesh Standard Time, N. Central Asia Standard Time, Myanmar Standard Time, SE Asia Standard Time, North Asia Standard Time, China Standard Time, North Asia East Standard Time, Singapore Standard Time, W. Australia Standard Time, Taipei Standard Time, Ulaanbaatar Standard Time, Tokyo Standard Time, Korea Standard Time, Yakutsk Standard Time, Cen. Australia Standard Time, AUS Central Standard Time, E. Australia Standard Time, AUS Eastern Standard Time, West Pacific Standard Time, Tasmania Standard Time, Magadan Standard Time, Vladivostok Standard Time, Russia Time Zone 10, Central Pacific Standard Time, Russia Time Zone 11, New Zealand Standard Time, UTC+12, Fiji Standard Time, Kamchatka Standard Time, Tonga Standard Time, Samoa Standard Time, Line Islands Standard Time"]
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[doc = "the collection of days that the profile takes effect on. Possible values are Sunday through Saturday."]
    pub days: Vec<String>,
    #[doc = "A collection of hours that the profile takes effect on. Values supported are 0 to 23 on the 24-hour clock (AM/PM times are not supported)."]
    pub hours: Vec<i32>,
    #[doc = "A collection of minutes at which the profile takes effect at."]
    pub minutes: Vec<i32>,
}
impl RecurrentSchedule {
    pub fn new(time_zone: String, days: Vec<String>, hours: Vec<i32>, minutes: Vec<i32>) -> Self {
        Self {
            time_zone,
            days,
            hours,
            minutes,
        }
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
#[doc = "Specifies the retention policy for the log."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionPolicy {
    #[doc = "a value indicating whether the retention policy is enabled."]
    pub enabled: bool,
    #[doc = "the number of days for the retention in days. A value of 0 will retain the events indefinitely."]
    pub days: i32,
}
impl RetentionPolicy {
    pub fn new(enabled: bool, days: i32) -> Self {
        Self { enabled, days }
    }
}
#[doc = "The parameters for the scaling action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleAction {
    #[doc = "the scale direction. Whether the scaling action increases or decreases the number of instances."]
    pub direction: scale_action::Direction,
    #[doc = "the type of action that should occur when the scale rule fires."]
    #[serde(rename = "type")]
    pub type_: scale_action::Type,
    #[doc = "the number of instances that are involved in the scaling action. This value must be 1 or greater. The default value is 1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "the amount of time to wait since the last scaling action before this action occurs. It must be between 1 week and 1 minute in ISO 8601 format."]
    pub cooldown: String,
}
impl ScaleAction {
    pub fn new(direction: scale_action::Direction, type_: scale_action::Type, cooldown: String) -> Self {
        Self {
            direction,
            type_,
            value: None,
            cooldown,
        }
    }
}
pub mod scale_action {
    use super::*;
    #[doc = "the scale direction. Whether the scaling action increases or decreases the number of instances."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Direction {
        None,
        Increase,
        Decrease,
    }
    #[doc = "the type of action that should occur when the scale rule fires."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        ChangeCount,
        PercentChangeCount,
        ExactCount,
        ServiceAllowedNextValue,
    }
}
#[doc = "The number of instances that can be used during this profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleCapacity {
    #[doc = "the minimum number of instances for the resource."]
    pub minimum: String,
    #[doc = "the maximum number of instances for the resource. The actual maximum number of instances is limited by the cores that are available in the subscription."]
    pub maximum: String,
    #[doc = "the number of instances that will be set if metrics are not available for evaluation. The default is only used if the current instance count is lower than the default."]
    pub default: String,
}
impl ScaleCapacity {
    pub fn new(minimum: String, maximum: String, default: String) -> Self {
        Self { minimum, maximum, default }
    }
}
#[doc = "A rule that provide the triggers and parameters for the scaling action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleRule {
    #[doc = "The trigger that results in a scaling action."]
    #[serde(rename = "metricTrigger")]
    pub metric_trigger: MetricTrigger,
    #[doc = "The parameters for the scaling action."]
    #[serde(rename = "scaleAction")]
    pub scale_action: ScaleAction,
}
impl ScaleRule {
    pub fn new(metric_trigger: MetricTrigger, scale_action: ScaleAction) -> Self {
        Self {
            metric_trigger,
            scale_action,
        }
    }
}
#[doc = "Specifies an auto scale rule metric dimension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleRuleMetricDimension {
    #[doc = "Name of the dimension."]
    #[serde(rename = "DimensionName")]
    pub dimension_name: String,
    #[doc = "the dimension operator. Only 'Equals' and 'NotEquals' are supported. 'Equals' being equal to any of the values. 'NotEquals' being not equal to all of the values"]
    #[serde(rename = "Operator")]
    pub operator: scale_rule_metric_dimension::Operator,
    #[doc = "list of dimension values. For example: [\"App1\",\"App2\"]."]
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}
impl ScaleRuleMetricDimension {
    pub fn new(dimension_name: String, operator: scale_rule_metric_dimension::Operator, values: Vec<String>) -> Self {
        Self {
            dimension_name,
            operator,
            values,
        }
    }
}
pub mod scale_rule_metric_dimension {
    use super::*;
    #[doc = "the dimension operator. Only 'Equals' and 'NotEquals' are supported. 'Equals' being equal to any of the values. 'NotEquals' being not equal to all of the values"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Equals,
        NotEquals,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Equals => serializer.serialize_unit_variant("Operator", 0u32, "Equals"),
                Self::NotEquals => serializer.serialize_unit_variant("Operator", 1u32, "NotEquals"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A private link scoped resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopedResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a private link scoped resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScopedResourceProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ScopedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of scoped resources in a private link scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopedResourceListResult {
    #[doc = "Array of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ScopedResource>,
    #[doc = "Link to retrieve next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScopedResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScopedResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link scoped resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopedResourceProperties {
    #[doc = "The resource id of the scoped Azure monitor resource."]
    #[serde(rename = "linkedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub linked_resource_id: Option<String>,
    #[doc = "State of the private endpoint connection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ScopedResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An SMS receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsReceiver {
    #[doc = "The name of the SMS receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The country code of the SMS receiver."]
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[doc = "The phone number of the SMS receiver."]
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[doc = "Indicates the status of the receiver. Receivers that are not Enabled will not receive any communications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
impl SmsReceiver {
    pub fn new(name: String, country_code: String, phone_number: String) -> Self {
        Self {
            name,
            country_code,
            phone_number,
            status: None,
        }
    }
}
#[doc = "The subscription diagnostic settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionDiagnosticSettings {
    #[doc = "The resource ID of the storage account to which you would like to send Diagnostic Logs."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "The service bus rule Id of the diagnostic setting. This is here to maintain backwards compatibility."]
    #[serde(rename = "serviceBusRuleId", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_rule_id: Option<String>,
    #[doc = "The resource Id for the event hub authorization rule."]
    #[serde(rename = "eventHubAuthorizationRuleId", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_authorization_rule_id: Option<String>,
    #[doc = "The name of the event hub. If none is specified, the default event hub will be selected."]
    #[serde(rename = "eventHubName", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_name: Option<String>,
    #[doc = "The list of logs settings."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub logs: Vec<SubscriptionLogSettings>,
    #[doc = "The full ARM resource ID of the Log Analytics workspace to which you would like to send Diagnostic Logs. Example: /subscriptions/4b9e8510-67ab-4e9a-95a9-e2f1e570ea9c/resourceGroups/insights-integration/providers/Microsoft.OperationalInsights/workspaces/viruela2"]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The full ARM resource ID of the Marketplace resource to which you would like to send Diagnostic Logs."]
    #[serde(rename = "marketplacePartnerId", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_partner_id: Option<String>,
}
impl SubscriptionDiagnosticSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The subscription diagnostic setting resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionDiagnosticSettingsResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The subscription diagnostic settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionDiagnosticSettings>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl SubscriptionDiagnosticSettingsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a collection of subscription diagnostic settings resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionDiagnosticSettingsResourceCollection {
    #[doc = "The collection of subscription diagnostic settings resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SubscriptionDiagnosticSettingsResource>,
}
impl azure_core::Continuable for SubscriptionDiagnosticSettingsResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SubscriptionDiagnosticSettingsResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Part of Subscription diagnostic setting. Specifies the settings for a particular log."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionLogSettings {
    #[doc = "Name of a Subscription Diagnostic Log category for a resource type this setting is applied to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Name of a Subscription Diagnostic Log category group for a resource type this setting is applied to."]
    #[serde(rename = "categoryGroup", default, skip_serializing_if = "Option::is_none")]
    pub category_group: Option<String>,
    #[doc = "a value indicating whether this log is enabled."]
    pub enabled: bool,
}
impl SubscriptionLogSettings {
    pub fn new(enabled: bool) -> Self {
        Self {
            category: None,
            category_group: None,
            enabled,
        }
    }
}
#[doc = "A container holding only the Tags for a resource, allowing the user to update the tags on a PrivateLinkScope instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsResource {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the test notification results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestNotificationDetailsResponse {
    #[doc = "The context info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,
    #[doc = "The overall state"]
    pub state: String,
    #[doc = "The completed time"]
    #[serde(rename = "completedTime", default, skip_serializing_if = "Option::is_none")]
    pub completed_time: Option<String>,
    #[doc = "The created time"]
    #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[doc = "The list of action detail"]
    #[serde(
        rename = "actionDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub action_details: Vec<ActionDetail>,
}
impl TestNotificationDetailsResponse {
    pub fn new(state: String) -> Self {
        Self {
            context: None,
            state,
            completed_time: None,
            created_time: None,
            action_details: Vec::new(),
        }
    }
}
#[doc = "A specific date-time for the profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeWindow {
    #[doc = "the timezone of the start and end times for the profile. Some examples of valid time zones are: Dateline Standard Time, UTC-11, Hawaiian Standard Time, Alaskan Standard Time, Pacific Standard Time (Mexico), Pacific Standard Time, US Mountain Standard Time, Mountain Standard Time (Mexico), Mountain Standard Time, Central America Standard Time, Central Standard Time, Central Standard Time (Mexico), Canada Central Standard Time, SA Pacific Standard Time, Eastern Standard Time, US Eastern Standard Time, Venezuela Standard Time, Paraguay Standard Time, Atlantic Standard Time, Central Brazilian Standard Time, SA Western Standard Time, Pacific SA Standard Time, Newfoundland Standard Time, E. South America Standard Time, Argentina Standard Time, SA Eastern Standard Time, Greenland Standard Time, Montevideo Standard Time, Bahia Standard Time, UTC-02, Mid-Atlantic Standard Time, Azores Standard Time, Cape Verde Standard Time, Morocco Standard Time, UTC, GMT Standard Time, Greenwich Standard Time, W. Europe Standard Time, Central Europe Standard Time, Romance Standard Time, Central European Standard Time, W. Central Africa Standard Time, Namibia Standard Time, Jordan Standard Time, GTB Standard Time, Middle East Standard Time, Egypt Standard Time, Syria Standard Time, E. Europe Standard Time, South Africa Standard Time, FLE Standard Time, Turkey Standard Time, Israel Standard Time, Kaliningrad Standard Time, Libya Standard Time, Arabic Standard Time, Arab Standard Time, Belarus Standard Time, Russian Standard Time, E. Africa Standard Time, Iran Standard Time, Arabian Standard Time, Azerbaijan Standard Time, Russia Time Zone 3, Mauritius Standard Time, Georgian Standard Time, Caucasus Standard Time, Afghanistan Standard Time, West Asia Standard Time, Ekaterinburg Standard Time, Pakistan Standard Time, India Standard Time, Sri Lanka Standard Time, Nepal Standard Time, Central Asia Standard Time, Bangladesh Standard Time, N. Central Asia Standard Time, Myanmar Standard Time, SE Asia Standard Time, North Asia Standard Time, China Standard Time, North Asia East Standard Time, Singapore Standard Time, W. Australia Standard Time, Taipei Standard Time, Ulaanbaatar Standard Time, Tokyo Standard Time, Korea Standard Time, Yakutsk Standard Time, Cen. Australia Standard Time, AUS Central Standard Time, E. Australia Standard Time, AUS Eastern Standard Time, West Pacific Standard Time, Tasmania Standard Time, Magadan Standard Time, Vladivostok Standard Time, Russia Time Zone 10, Central Pacific Standard Time, Russia Time Zone 11, New Zealand Standard Time, UTC+12, Fiji Standard Time, Kamchatka Standard Time, Tonga Standard Time, Samoa Standard Time, Line Islands Standard Time"]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "the start time for the profile in ISO 8601 format."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub start: time::OffsetDateTime,
    #[doc = "the end time for the profile in ISO 8601 format."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub end: time::OffsetDateTime,
}
impl TimeWindow {
    pub fn new(start: time::OffsetDateTime, end: time::OffsetDateTime) -> Self {
        Self {
            time_zone: None,
            start,
            end,
        }
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
#[doc = "A voice receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoiceReceiver {
    #[doc = "The name of the voice receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The country code of the voice receiver."]
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[doc = "The phone number of the voice receiver."]
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
}
impl VoiceReceiver {
    pub fn new(name: String, country_code: String, phone_number: String) -> Self {
        Self {
            name,
            country_code,
            phone_number,
        }
    }
}
#[doc = "Webhook notification of an autoscale event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebhookNotification {
    #[doc = "the service address to receive the notification."]
    #[serde(rename = "serviceUri", default, skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[doc = "a property bag of settings. This value can be empty."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl WebhookNotification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A webhook receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookReceiver {
    #[doc = "The name of the webhook receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The URI where webhooks should be sent."]
    #[serde(rename = "serviceUri")]
    pub service_uri: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
    #[doc = "Indicates whether or not use AAD authentication."]
    #[serde(rename = "useAadAuth", default, skip_serializing_if = "Option::is_none")]
    pub use_aad_auth: Option<bool>,
    #[doc = "Indicates the webhook app object Id for aad auth."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Indicates the identifier uri for aad auth."]
    #[serde(rename = "identifierUri", default, skip_serializing_if = "Option::is_none")]
    pub identifier_uri: Option<String>,
    #[doc = "Indicates the tenant id for aad auth."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl WebhookReceiver {
    pub fn new(name: String, service_uri: String) -> Self {
        Self {
            name,
            service_uri,
            use_common_alert_schema: None,
            use_aad_auth: None,
            object_id: None,
            identifier_uri: None,
            tenant_id: None,
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
