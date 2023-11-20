#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A pointer to an Azure Action Group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroup {
    #[doc = "The resource ID of the Action Group. This cannot be null or empty."]
    #[serde(rename = "actionGroupId")]
    pub action_group_id: String,
    #[doc = "the dictionary of custom properties to include with the post operation. These data are appended to the webhook payload."]
    #[serde(rename = "webhookProperties", default, skip_serializing_if = "Option::is_none")]
    pub webhook_properties: Option<serde_json::Value>,
    #[doc = "Predefined list of properties and configuration items for the action group."]
    #[serde(rename = "actionProperties", default, skip_serializing_if = "Option::is_none")]
    pub action_properties: Option<serde_json::Value>,
}
impl ActionGroup {
    pub fn new(action_group_id: String) -> Self {
        Self {
            action_group_id,
            webhook_properties: None,
            action_properties: None,
        }
    }
}
#[doc = "A list of Activity Log Alert rule actions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionList {
    #[doc = "The list of the Action Groups."]
    #[serde(
        rename = "actionGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub action_groups: Vec<ActionGroup>,
}
impl ActionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Activity Log Alert rule condition that is met when all its member conditions are met."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleAllOfCondition {
    #[doc = "The list of Activity Log Alert rule conditions."]
    #[serde(rename = "allOf")]
    pub all_of: Vec<AlertRuleAnyOfOrLeafCondition>,
}
impl AlertRuleAllOfCondition {
    pub fn new(all_of: Vec<AlertRuleAnyOfOrLeafCondition>) -> Self {
        Self { all_of }
    }
}
#[doc = "An Activity Log Alert rule condition that is met when all its member conditions are met.\nEach condition can be of one of the following types:\n__Important__: Each type has its unique subset of properties. Properties from different types CANNOT exist in one condition.\n   * __Leaf Condition -__ must contain 'field' and either 'equals' or 'containsAny'.\n  _Please note, 'anyOf' should __not__ be set in a Leaf Condition._\n  * __AnyOf Condition -__ must contain __only__ 'anyOf' (which is an array of Leaf Conditions).\n  _Please note, 'field', 'equals' and 'containsAny' should __not__ be set in an AnyOf Condition._\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleAnyOfOrLeafCondition {
    #[serde(flatten)]
    pub alert_rule_leaf_condition: AlertRuleLeafCondition,
    #[doc = "An Activity Log Alert rule condition that is met when at least one of its member leaf conditions are met."]
    #[serde(
        rename = "anyOf",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub any_of: Vec<AlertRuleLeafCondition>,
}
impl AlertRuleAnyOfOrLeafCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Activity Log Alert rule condition that is met by comparing the field and value of an Activity Log event.\nThis condition must contain 'field' and either 'equals' or 'containsAny'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleLeafCondition {
    #[doc = "The name of the Activity Log event's field that this condition will examine.\nThe possible values for this field are (case-insensitive): 'resourceId', 'category', 'caller', 'level', 'operationName', 'resourceGroup', 'resourceProvider', 'status', 'subStatus', 'resourceType', or anything beginning with 'properties'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[doc = "The value of the event's field will be compared to this value (case-insensitive) to determine if the condition is met."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,
    #[doc = "The value of the event's field will be compared to the values in this array (case-insensitive) to determine if the condition is met."]
    #[serde(
        rename = "containsAny",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contains_any: Vec<String>,
}
impl AlertRuleLeafCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Activity Log Alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleProperties {
    #[doc = "The tenant GUID. Must be provided for tenant-level and management group events rules."]
    #[serde(rename = "tenantScope", default, skip_serializing_if = "Option::is_none")]
    pub tenant_scope: Option<String>,
    #[doc = "A list of resource IDs that will be used as prefixes. The alert will only apply to Activity Log events with resource IDs that fall under one of these prefixes. This list must include at least one item."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<String>,
    #[doc = "An Activity Log Alert rule condition that is met when all its member conditions are met."]
    pub condition: AlertRuleAllOfCondition,
    #[doc = "A list of Activity Log Alert rule actions."]
    pub actions: ActionList,
    #[doc = "Indicates whether this Activity Log Alert rule is enabled. If an Activity Log Alert rule is not enabled, then none of its actions will be activated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "A description of this Activity Log Alert rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AlertRuleProperties {
    pub fn new(condition: AlertRuleAllOfCondition, actions: ActionList) -> Self {
        Self {
            tenant_scope: None,
            scopes: Vec::new(),
            condition,
            actions,
            enabled: None,
            description: None,
        }
    }
}
#[doc = "An Azure resource object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResource {
    #[doc = "The resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The location of the resource. Since Azure Activity Log Alerts is a global service, the location of the rules should always be 'global'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AzureResource {
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
#[doc = "A Tenant Activity Log Alert rule resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantActivityLogAlertResource {
    #[serde(flatten)]
    pub azure_resource: AzureResource,
    #[doc = "An Azure Activity Log Alert rule."]
    pub properties: AlertRuleProperties,
}
impl TenantActivityLogAlertResource {
    pub fn new(properties: AlertRuleProperties) -> Self {
        Self {
            azure_resource: AzureResource::default(),
            properties,
        }
    }
}
#[doc = "A list of Tenant Activity Log Alert rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantAlertRuleList {
    #[doc = "The list of Tenant Activity Log Alert rules."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TenantActivityLogAlertResource>,
    #[doc = "Provides the link to retrieve the next set of elements."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TenantAlertRuleList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TenantAlertRuleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Activity Log Alert rule object for the body of patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantAlertRulePatchObject {
    #[doc = "The resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "An Activity Log Alert rule properties for patch operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TenantAlertRulePatchProperties>,
}
impl TenantAlertRulePatchObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Activity Log Alert rule properties for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantAlertRulePatchProperties {
    #[doc = "Indicates whether this Activity Log Alert rule is enabled. If an Activity Log Alert rule is not enabled, then none of its actions will be activated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl TenantAlertRulePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
