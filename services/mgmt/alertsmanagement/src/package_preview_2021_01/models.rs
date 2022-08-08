#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "API error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationErrorResponse {
    #[doc = "API error body."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<MigrationErrorResponseBody>,
}
impl MigrationErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API error body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationErrorResponseBody {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl MigrationErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for the quota submission request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationStatusResponse {
    #[doc = "The migration resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The migration process name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. \"Microsoft.AlertsManagement/migrateFromSmartDetection\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "A migration status response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MigrationStatusResponseProperties>,
}
impl MigrationStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A migration status response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrationStatusResponseProperties {
    #[doc = "The migration process unique id."]
    #[serde(rename = "migrationId")]
    pub migration_id: String,
    #[doc = "The current status of the migration process"]
    pub status: migration_status_response_properties::Status,
    #[doc = "The list of resource IDs that the requested migration should be performed on."]
    pub scope: Vec<String>,
    #[doc = "The name of the ARM deployment associated with the migration process."]
    #[serde(rename = "armDeploymentName", default, skip_serializing_if = "Option::is_none")]
    pub arm_deployment_name: Option<String>,
}
impl MigrationStatusResponseProperties {
    pub fn new(migration_id: String, status: migration_status_response_properties::Status, scope: Vec<String>) -> Self {
        Self {
            migration_id,
            status,
            scope,
            arm_deployment_name: None,
        }
    }
}
pub mod migration_status_response_properties {
    use super::*;
    #[doc = "The current status of the migration process"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Starting,
        InProcess,
        Deploying,
        Completed,
        Failed,
        Canceled,
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
                Self::Starting => serializer.serialize_unit_variant("Status", 0u32, "Starting"),
                Self::InProcess => serializer.serialize_unit_variant("Status", 1u32, "InProcess"),
                Self::Deploying => serializer.serialize_unit_variant("Status", 2u32, "Deploying"),
                Self::Completed => serializer.serialize_unit_variant("Status", 3u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("Status", 4u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Smart Detection migration request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartDetectionMigrationRequest {
    #[doc = "The list of resource IDs that the requested migration should be performed on."]
    pub scope: Vec<String>,
    #[doc = "The policy for migrating the email settings in the Smart Detection Rules into action groups. If not specified, 'Auto' policy is used."]
    #[serde(rename = "actionGroupCreationPolicy", default, skip_serializing_if = "Option::is_none")]
    pub action_group_creation_policy: Option<smart_detection_migration_request::ActionGroupCreationPolicy>,
    #[doc = "A custom name of an existing action group to attach to the created alert rules. Required only when actionGroupCreationPolicy is set to 'Custom'."]
    #[serde(rename = "customActionGroupName", default, skip_serializing_if = "Option::is_none")]
    pub custom_action_group_name: Option<String>,
}
impl SmartDetectionMigrationRequest {
    pub fn new(scope: Vec<String>) -> Self {
        Self {
            scope,
            action_group_creation_policy: None,
            custom_action_group_name: None,
        }
    }
}
pub mod smart_detection_migration_request {
    use super::*;
    #[doc = "The policy for migrating the email settings in the Smart Detection Rules into action groups. If not specified, 'Auto' policy is used."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionGroupCreationPolicy")]
    pub enum ActionGroupCreationPolicy {
        Custom,
        Auto,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionGroupCreationPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionGroupCreationPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionGroupCreationPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Custom => serializer.serialize_unit_variant("ActionGroupCreationPolicy", 0u32, "Custom"),
                Self::Auto => serializer.serialize_unit_variant("ActionGroupCreationPolicy", 1u32, "Auto"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Operation provided by provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties of the operation"]
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
    #[doc = "Properties of the operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Provider name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation name"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Lists the operations available in the AlertsManagement RP."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsList {
    #[doc = "URL to fetch the next set of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of operations"]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationsList {
    pub fn new(value: Vec<Operation>) -> Self {
        Self { next_link: None, value }
    }
}
