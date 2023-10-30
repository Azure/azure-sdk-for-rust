#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The workflow approval properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Approval {
    #[serde(flatten)]
    pub workflow_task: WorkflowTask,
    #[doc = "The approval details"]
    #[serde(rename = "approvalDetail", default, skip_serializing_if = "Option::is_none")]
    pub approval_detail: Option<ApprovalDetail>,
}
impl Approval {
    pub fn new(workflow_task: WorkflowTask) -> Self {
        Self {
            workflow_task,
            approval_detail: None,
        }
    }
}
#[doc = "The approval details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApprovalDetail {
    #[doc = "The approval type of an approval."]
    #[serde(rename = "approvalType")]
    pub approval_type: approval_detail::ApprovalType,
    #[doc = "The status of an approval."]
    pub status: approval_detail::Status,
    #[doc = "The list of approvers with reply."]
    pub approvers: serde_json::Value,
}
impl ApprovalDetail {
    pub fn new(approval_type: approval_detail::ApprovalType, status: approval_detail::Status, approvers: serde_json::Value) -> Self {
        Self {
            approval_type,
            status,
            approvers,
        }
    }
}
pub mod approval_detail {
    use super::*;
    #[doc = "The approval type of an approval."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ApprovalType")]
    pub enum ApprovalType {
        PendingOnAny,
        PendingOnAll,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ApprovalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ApprovalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ApprovalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PendingOnAny => serializer.serialize_unit_variant("ApprovalType", 0u32, "PendingOnAny"),
                Self::PendingOnAll => serializer.serialize_unit_variant("ApprovalType", 1u32, "PendingOnAll"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of an approval."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Pending,
        Approved,
        Rejected,
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
                Self::Pending => serializer.serialize_unit_variant("Status", 0u32, "Pending"),
                Self::Approved => serializer.serialize_unit_variant("Status", 1u32, "Approved"),
                Self::Rejected => serializer.serialize_unit_variant("Status", 2u32, "Rejected"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 3u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalResponseComment {
    #[doc = "The comment of approving or rejecting an approval type of workflow task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}
impl ApprovalResponseComment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of approvers for an approval type of workflow task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApproverResponse {
    #[doc = "The response for an approval type of workflow task."]
    pub reply: approver_response::Reply,
    #[doc = "The comment of approving or rejecting an approval type of workflow task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "The reply time of approver to a workflow task."]
    #[serde(rename = "responseTime", default, with = "azure_core::date::rfc3339::option")]
    pub response_time: Option<time::OffsetDateTime>,
}
impl ApproverResponse {
    pub fn new(reply: approver_response::Reply) -> Self {
        Self {
            reply,
            comment: None,
            response_time: None,
        }
    }
}
pub mod approver_response {
    use super::*;
    #[doc = "The response for an approval type of workflow task."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reply")]
    pub enum Reply {
        Approved,
        Rejected,
        Pending,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Reply {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Reply {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Reply {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Approved => serializer.serialize_unit_variant("Reply", 0u32, "Approved"),
                Self::Rejected => serializer.serialize_unit_variant("Reply", 1u32, "Rejected"),
                Self::Pending => serializer.serialize_unit_variant("Reply", 2u32, "Pending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type CreatedTime = time::OffsetDateTime;
#[doc = "Default error model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorModel {
    #[doc = "Gets or sets the code."]
    pub code: String,
    #[doc = "Gets or sets the details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorModel>,
    #[doc = "Gets or sets the messages."]
    pub message: String,
    #[doc = "Gets or sets the target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl ErrorModel {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            details: Vec::new(),
            message,
            target: None,
        }
    }
}
#[doc = "Default error response model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[doc = "Default error model"]
    pub error: ErrorModel,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new(error: ErrorModel) -> Self {
        Self { error }
    }
}
pub type LastUpdateTime = time::OffsetDateTime;
#[doc = "The operation user wants to perform."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[doc = "The operation type."]
    #[serde(rename = "type")]
    pub type_: OperationType,
    #[doc = "The payload of each operation which user want to submit."]
    pub payload: serde_json::Value,
}
impl Operation {
    pub fn new(type_: OperationType, payload: serde_json::Value) -> Self {
        Self { type_, payload }
    }
}
#[doc = "The operation type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationType")]
pub enum OperationType {
    CreateTerm,
    UpdateTerm,
    DeleteTerm,
    ImportTerms,
    UpdateAsset,
    GrantDataAccess,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CreateTerm => serializer.serialize_unit_variant("OperationType", 0u32, "CreateTerm"),
            Self::UpdateTerm => serializer.serialize_unit_variant("OperationType", 1u32, "UpdateTerm"),
            Self::DeleteTerm => serializer.serialize_unit_variant("OperationType", 2u32, "DeleteTerm"),
            Self::ImportTerms => serializer.serialize_unit_variant("OperationType", 3u32, "ImportTerms"),
            Self::UpdateAsset => serializer.serialize_unit_variant("OperationType", 4u32, "UpdateAsset"),
            Self::GrantDataAccess => serializer.serialize_unit_variant("OperationType", 5u32, "GrantDataAccess"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The request payload of reassigning a workflow task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReassignCommand {
    #[doc = "The request body of reassigning a workflow task."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reassignments: Vec<serde_json::Value>,
}
impl ReassignCommand {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type RequestorId = String;
#[doc = "The workflow simple task properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleTask {
    #[serde(flatten)]
    pub workflow_task: WorkflowTask,
    #[doc = "Workflow simple task details."]
    #[serde(rename = "taskDetail", default, skip_serializing_if = "Option::is_none")]
    pub task_detail: Option<SimpleTaskDetail>,
}
impl SimpleTask {
    pub fn new(workflow_task: WorkflowTask) -> Self {
        Self {
            workflow_task,
            task_detail: None,
        }
    }
}
#[doc = "Workflow simple task details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleTaskDetail {
    #[doc = "The simple task body."]
    #[serde(rename = "taskBody")]
    pub task_body: String,
    #[doc = "The users or groups were assigned the simple task."]
    #[serde(rename = "assignedTo")]
    pub assigned_to: Vec<String>,
    #[doc = "Simple task status."]
    pub status: simple_task_detail::Status,
    #[serde(rename = "changeHistory")]
    pub change_history: Vec<TaskChangeEvent>,
}
impl SimpleTaskDetail {
    pub fn new(
        task_body: String,
        assigned_to: Vec<String>,
        status: simple_task_detail::Status,
        change_history: Vec<TaskChangeEvent>,
    ) -> Self {
        Self {
            task_body,
            assigned_to,
            status,
            change_history,
        }
    }
}
pub mod simple_task_detail {
    use super::*;
    #[doc = "Simple task status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        NotStarted,
        InProgress,
        Completed,
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
                Self::NotStarted => serializer.serialize_unit_variant("Status", 0u32, "NotStarted"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("Status", 2u32, "Completed"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 3u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Status")]
pub enum Status {
    NotStarted,
    InProgress,
    Failed,
    Completed,
    Canceling,
    CancellationFailed,
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
            Self::NotStarted => serializer.serialize_unit_variant("Status", 0u32, "NotStarted"),
            Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
            Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
            Self::Completed => serializer.serialize_unit_variant("Status", 3u32, "Completed"),
            Self::Canceling => serializer.serialize_unit_variant("Status", 4u32, "Canceling"),
            Self::CancellationFailed => serializer.serialize_unit_variant("Status", 5u32, "CancellationFailed"),
            Self::Canceled => serializer.serialize_unit_variant("Status", 6u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type SubmittedTime = time::OffsetDateTime;
#[doc = "History of changes made on task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaskChangeEvent {}
impl TaskChangeEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Info and material that helps assignees to take action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskPayload {
    #[doc = "The task payload type."]
    #[serde(rename = "type")]
    pub type_: TaskPayloadType,
    #[doc = "The target value of entity which user want to involve workflow to update."]
    #[serde(rename = "targetValue")]
    pub target_value: String,
    #[doc = "The payload of the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}
impl TaskPayload {
    pub fn new(type_: TaskPayloadType, target_value: String) -> Self {
        Self {
            type_,
            target_value,
            payload: None,
        }
    }
}
#[doc = "The task payload type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TaskPayloadType")]
pub enum TaskPayloadType {
    CreateTerm,
    UpdateTerm,
    DeleteTerm,
    ImportTerms,
    UpdateAsset,
    GrantDataAccess,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TaskPayloadType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TaskPayloadType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TaskPayloadType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CreateTerm => serializer.serialize_unit_variant("TaskPayloadType", 0u32, "CreateTerm"),
            Self::UpdateTerm => serializer.serialize_unit_variant("TaskPayloadType", 1u32, "UpdateTerm"),
            Self::DeleteTerm => serializer.serialize_unit_variant("TaskPayloadType", 2u32, "DeleteTerm"),
            Self::ImportTerms => serializer.serialize_unit_variant("TaskPayloadType", 3u32, "ImportTerms"),
            Self::UpdateAsset => serializer.serialize_unit_variant("TaskPayloadType", 4u32, "UpdateAsset"),
            Self::GrantDataAccess => serializer.serialize_unit_variant("TaskPayloadType", 5u32, "GrantDataAccess"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskUpdateCommand {
    #[doc = "The new status will be used to update the task."]
    #[serde(rename = "newStatus")]
    pub new_status: task_update_command::NewStatus,
    #[doc = "The comment when update a task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}
impl TaskUpdateCommand {
    pub fn new(new_status: task_update_command::NewStatus) -> Self {
        Self { new_status, comment: None }
    }
}
pub mod task_update_command {
    use super::*;
    #[doc = "The new status will be used to update the task."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NewStatus")]
    pub enum NewStatus {
        NotStarted,
        InProgress,
        Completed,
        Canceled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NewStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NewStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NewStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotStarted => serializer.serialize_unit_variant("NewStatus", 0u32, "NotStarted"),
                Self::InProgress => serializer.serialize_unit_variant("NewStatus", 1u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("NewStatus", 2u32, "Completed"),
                Self::Canceled => serializer.serialize_unit_variant("NewStatus", 3u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TasksList {
    #[doc = "The value of workflow tasks list."]
    pub value: Vec<WorkflowTaskUnion>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TasksList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TasksList {
    pub fn new(value: Vec<WorkflowTaskUnion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes under what condition a workflow will run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trigger {
    #[serde(rename = "type")]
    pub type_: trigger::Type,
    #[doc = "Glossary term hierarchy path."]
    #[serde(rename = "underGlossaryHierarchy", default, skip_serializing_if = "Option::is_none")]
    pub under_glossary_hierarchy: Option<String>,
    #[doc = "The collection name."]
    #[serde(rename = "underCollection", default, skip_serializing_if = "Option::is_none")]
    pub under_collection: Option<String>,
    #[doc = "The glossary guid."]
    #[serde(rename = "underGlossary", default, skip_serializing_if = "Option::is_none")]
    pub under_glossary: Option<String>,
}
impl Trigger {
    pub fn new(type_: trigger::Type) -> Self {
        Self {
            type_,
            under_glossary_hierarchy: None,
            under_collection: None,
            under_glossary: None,
        }
    }
}
pub mod trigger {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "when_term_creation_is_requested")]
        WhenTermCreationIsRequested,
        #[serde(rename = "when_term_deletion_is_requested")]
        WhenTermDeletionIsRequested,
        #[serde(rename = "when_term_update_is_requested")]
        WhenTermUpdateIsRequested,
        #[serde(rename = "when_terms_import_is_requested")]
        WhenTermsImportIsRequested,
        #[serde(rename = "when_data_access_grant_is_requested")]
        WhenDataAccessGrantIsRequested,
        #[serde(rename = "when_asset_update_is_requested")]
        WhenAssetUpdateIsRequested,
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
                Self::WhenTermCreationIsRequested => serializer.serialize_unit_variant("Type", 0u32, "when_term_creation_is_requested"),
                Self::WhenTermDeletionIsRequested => serializer.serialize_unit_variant("Type", 1u32, "when_term_deletion_is_requested"),
                Self::WhenTermUpdateIsRequested => serializer.serialize_unit_variant("Type", 2u32, "when_term_update_is_requested"),
                Self::WhenTermsImportIsRequested => serializer.serialize_unit_variant("Type", 3u32, "when_terms_import_is_requested"),
                Self::WhenDataAccessGrantIsRequested => {
                    serializer.serialize_unit_variant("Type", 4u32, "when_data_access_grant_is_requested")
                }
                Self::WhenAssetUpdateIsRequested => serializer.serialize_unit_variant("Type", 5u32, "when_asset_update_is_requested"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type Triggers = Vec<Trigger>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRequestPayload {
    #[doc = "The list of operations user want to submit, each operation matches one Purview API call and will do the operation directly."]
    pub operations: Vec<Operation>,
    #[doc = "The comment when submit a user request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}
impl UserRequestPayload {
    pub fn new(operations: Vec<Operation>) -> Self {
        Self { operations, comment: None }
    }
}
#[doc = "Describes user ask to do operation(s) on Purview."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRequestResponse {
    #[doc = "The user request id."]
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[doc = "The person who submitted the user request."]
    pub requestor: RequestorId,
    #[doc = "The list of operations user want to submit, each operation matches one Purview API call and will do the operation directly."]
    pub operations: Vec<serde_json::Value>,
    #[doc = "The comment when submit a user request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "The status."]
    pub status: Status,
}
impl UserRequestResponse {
    pub fn new(request_id: String, requestor: RequestorId, operations: Vec<serde_json::Value>, status: Status) -> Self {
        Self {
            request_id,
            requestor,
            operations,
            comment: None,
            status,
        }
    }
}
#[doc = "The workflow properties. It includes the triggers, actual flow and other properties of a workflow."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    #[serde(flatten)]
    pub workflow_metadata: WorkflowMetadata,
    #[doc = "The action DAG(Directed Acyclic Graph), it defines steps to be executed in a workflow run and their order."]
    #[serde(rename = "actionDag", default, skip_serializing_if = "Option::is_none")]
    pub action_dag: Option<serde_json::Value>,
}
impl Workflow {
    pub fn new(workflow_metadata: WorkflowMetadata) -> Self {
        Self {
            workflow_metadata,
            action_dag: None,
        }
    }
}
#[doc = "Create or update workflow payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowCreateOrUpdateCommand {
    #[doc = "It describes under what condition a workflow will run.  "]
    pub triggers: Triggers,
    #[doc = "The workflow name."]
    pub name: String,
    #[doc = "Whether the workflow enabled or not."]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[doc = "Description of a workflow."]
    pub description: String,
    #[doc = "The action DAG(Directed Acyclic Graph), it defines actual flow."]
    #[serde(rename = "actionDag", default, skip_serializing_if = "Option::is_none")]
    pub action_dag: Option<serde_json::Value>,
}
impl WorkflowCreateOrUpdateCommand {
    pub fn new(triggers: Triggers, name: String, is_enabled: bool, description: String) -> Self {
        Self {
            triggers,
            name,
            is_enabled,
            description,
            action_dag: None,
        }
    }
}
pub type WorkflowId = String;
#[doc = "The workflow metadata, action DAGs are not included."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowMetadata {
    #[doc = "The id of workflow."]
    pub id: String,
    #[doc = "It describes under what condition a workflow will run.  "]
    pub triggers: Triggers,
    #[doc = "The created time of workflow."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The person who created the workflow."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last update time."]
    #[serde(rename = "lastUpdateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_update_time: Option<time::OffsetDateTime>,
    #[doc = "The person who updated the workflow."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The name of a workflow."]
    pub name: String,
    #[doc = "Whether the workflow is enabled or not."]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[doc = "Description of a workflow."]
    pub description: String,
}
impl WorkflowMetadata {
    pub fn new(id: String, triggers: Triggers, name: String, is_enabled: bool, description: String) -> Self {
        Self {
            id,
            triggers,
            created_time: None,
            created_by: None,
            last_update_time: None,
            updated_by: None,
            name,
            is_enabled,
            description,
        }
    }
}
#[doc = "The workflow list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowMetadataList {
    #[doc = "The value of workflow list."]
    pub value: Vec<WorkflowMetadata>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkflowMetadataList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkflowMetadataList {
    pub fn new(value: Vec<WorkflowMetadata>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The execution of a workflow. It includes workflow action DAG at run time (action DAG snapshot), run payload, status of the entire run and other properties of a run. "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRun {
    #[doc = "The workflow run id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<WorkflowRunId>,
    #[doc = "The workflow id."]
    #[serde(rename = "workflowId", default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<WorkflowId>,
    #[doc = "Workflow run start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The person who submitted the user request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requestor: Option<RequestorId>,
    #[doc = "The user request id."]
    #[serde(rename = "userRequestId", default, skip_serializing_if = "Option::is_none")]
    pub user_request_id: Option<String>,
    #[doc = "The input of a workflow run. Align with operation in user request. "]
    #[serde(rename = "runPayload", default, skip_serializing_if = "Option::is_none")]
    pub run_payload: Option<workflow_run::RunPayload>,
    #[doc = "The status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[doc = "The time of workflow run completed."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The time of workflow run be canceled."]
    #[serde(rename = "cancelTime", default, with = "azure_core::date::rfc3339::option")]
    pub cancel_time: Option<time::OffsetDateTime>,
    #[doc = "The comment when cancel a workflow run."]
    #[serde(rename = "cancelComment", default, skip_serializing_if = "Option::is_none")]
    pub cancel_comment: Option<String>,
    #[doc = "The action DAG(Directed Acyclic Graph), it defines actual flow."]
    #[serde(rename = "actionDag")]
    pub action_dag: serde_json::Value,
    #[doc = "It refers to the \"detail\" property of a workflow run object, which contains run context and runtime information of actions."]
    pub detail: WorkflowRunDetail,
}
impl WorkflowRun {
    pub fn new(action_dag: serde_json::Value, detail: WorkflowRunDetail) -> Self {
        Self {
            id: None,
            workflow_id: None,
            start_time: None,
            requestor: None,
            user_request_id: None,
            run_payload: None,
            status: None,
            end_time: None,
            cancel_time: None,
            cancel_comment: None,
            action_dag,
            detail,
        }
    }
}
pub mod workflow_run {
    use super::*;
    #[doc = "The input of a workflow run. Align with operation in user request. "]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RunPayload {
        #[doc = "The workflow run payload type."]
        #[serde(rename = "type")]
        pub type_: WorkflowRunPayloadType,
        #[doc = "The target value which need involve workflow to update."]
        #[serde(rename = "targetValue")]
        pub target_value: String,
        #[doc = "The payload of each operation which user want to submit."]
        pub payload: serde_json::Value,
    }
    impl RunPayload {
        pub fn new(type_: WorkflowRunPayloadType, target_value: String, payload: serde_json::Value) -> Self {
            Self {
                type_,
                target_value,
                payload,
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowRunCancelRequest {
    #[doc = "The comment of canceling a workflow run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}
impl WorkflowRunCancelRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "It refers to the \"detail\" property of a workflow run object, which contains run context and runtime information of actions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRunDetail {
    #[doc = "Built-in variables starts with @runInput. Its properties are determined by trigger type at workflow run time. "]
    #[serde(rename = "runInput")]
    pub run_input: serde_json::Value,
    pub actions: serde_json::Value,
}
impl WorkflowRunDetail {
    pub fn new(run_input: serde_json::Value, actions: serde_json::Value) -> Self {
        Self { run_input, actions }
    }
}
pub type WorkflowRunId = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRunList {
    #[doc = "The value of workflow run list."]
    pub value: Vec<WorkflowRunMetadata>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkflowRunList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkflowRunList {
    pub fn new(value: Vec<WorkflowRunMetadata>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The execution of a workflow. It includes status of the entire run and other properties of a run. "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRunMetadata {
    #[doc = "The workflow run id."]
    pub id: WorkflowRunId,
    #[doc = "The workflow id."]
    #[serde(rename = "workflowId")]
    pub workflow_id: WorkflowId,
    #[doc = "Workflow run start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "The person who submitted the user request."]
    pub requestor: RequestorId,
    #[doc = "The user request id."]
    #[serde(rename = "userRequestId", default, skip_serializing_if = "Option::is_none")]
    pub user_request_id: Option<String>,
    #[doc = "The input of a workflow run. Align with operation in user request. "]
    #[serde(rename = "runPayload")]
    pub run_payload: WorkflowRunPayload,
    #[doc = "The status."]
    pub status: Status,
    #[doc = "The time of workflow run completed."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The time of workflow run be canceled."]
    #[serde(rename = "cancelTime", default, with = "azure_core::date::rfc3339::option")]
    pub cancel_time: Option<time::OffsetDateTime>,
    #[doc = "The comment when cancel a workflow run."]
    #[serde(rename = "cancelComment", default, skip_serializing_if = "Option::is_none")]
    pub cancel_comment: Option<String>,
}
impl WorkflowRunMetadata {
    pub fn new(
        id: WorkflowRunId,
        workflow_id: WorkflowId,
        start_time: time::OffsetDateTime,
        requestor: RequestorId,
        run_payload: WorkflowRunPayload,
        status: Status,
    ) -> Self {
        Self {
            id,
            workflow_id,
            start_time,
            requestor,
            user_request_id: None,
            run_payload,
            status,
            end_time: None,
            cancel_time: None,
            cancel_comment: None,
        }
    }
}
#[doc = "The input of a workflow run. Align with operation in user request. "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRunPayload {
    #[doc = "The workflow run payload type."]
    #[serde(rename = "type")]
    pub type_: WorkflowRunPayloadType,
    #[doc = "The target value which need involve workflow to update."]
    #[serde(rename = "targetValue")]
    pub target_value: String,
}
impl WorkflowRunPayload {
    pub fn new(type_: WorkflowRunPayloadType, target_value: String) -> Self {
        Self { type_, target_value }
    }
}
#[doc = "The workflow run payload type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkflowRunPayloadType")]
pub enum WorkflowRunPayloadType {
    CreateTerm,
    UpdateTerm,
    DeleteTerm,
    ImportTerms,
    UpdateAsset,
    GrantDataAccess,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkflowRunPayloadType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkflowRunPayloadType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkflowRunPayloadType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CreateTerm => serializer.serialize_unit_variant("WorkflowRunPayloadType", 0u32, "CreateTerm"),
            Self::UpdateTerm => serializer.serialize_unit_variant("WorkflowRunPayloadType", 1u32, "UpdateTerm"),
            Self::DeleteTerm => serializer.serialize_unit_variant("WorkflowRunPayloadType", 2u32, "DeleteTerm"),
            Self::ImportTerms => serializer.serialize_unit_variant("WorkflowRunPayloadType", 3u32, "ImportTerms"),
            Self::UpdateAsset => serializer.serialize_unit_variant("WorkflowRunPayloadType", 4u32, "UpdateAsset"),
            Self::GrantDataAccess => serializer.serialize_unit_variant("WorkflowRunPayloadType", 5u32, "GrantDataAccess"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An actionable item assigned to assignees. It is created when approval or task action starts to execute. Approval is one kind of task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTask {
    #[doc = "The workflow task id."]
    pub id: String,
    #[doc = "The workflow task title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The workflow run id."]
    #[serde(rename = "workflowRunId")]
    pub workflow_run_id: WorkflowRunId,
    #[doc = "The workflow id."]
    #[serde(rename = "workflowId")]
    pub workflow_id: WorkflowId,
    #[doc = "The person who submitted the user request."]
    pub requestor: RequestorId,
    #[doc = "The created time."]
    #[serde(rename = "createdTime")]
    pub created_time: CreatedTime,
    #[doc = "The last update time."]
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: LastUpdateTime,
    #[doc = "Info and material that helps assignees to take action."]
    pub payload: TaskPayload,
    #[doc = "Info of task reminder."]
    #[serde(rename = "reminderInfo", default, skip_serializing_if = "Option::is_none")]
    pub reminder_info: Option<workflow_task::ReminderInfo>,
    #[doc = "Info of task expiry."]
    #[serde(rename = "expiryInfo", default, skip_serializing_if = "Option::is_none")]
    pub expiry_info: Option<workflow_task::ExpiryInfo>,
}
impl WorkflowTask {
    pub fn new(
        id: String,
        workflow_run_id: WorkflowRunId,
        workflow_id: WorkflowId,
        requestor: RequestorId,
        created_time: CreatedTime,
        last_update_time: LastUpdateTime,
        payload: TaskPayload,
    ) -> Self {
        Self {
            id,
            title: None,
            workflow_run_id,
            workflow_id,
            requestor,
            created_time,
            last_update_time,
            payload,
            reminder_info: None,
            expiry_info: None,
        }
    }
}
pub mod workflow_task {
    use super::*;
    #[doc = "Info of task reminder."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ReminderInfo {
        #[doc = "The last update time."]
        #[serde(rename = "lastRemindTime", default, with = "azure_core::date::rfc3339::option")]
        pub last_remind_time: Option<time::OffsetDateTime>,
        #[doc = "The next remind time."]
        #[serde(rename = "nextRemindTime", with = "azure_core::date::rfc3339")]
        pub next_remind_time: time::OffsetDateTime,
        #[doc = "The reminder settings."]
        #[serde(rename = "reminderSettings")]
        pub reminder_settings: serde_json::Value,
    }
    impl ReminderInfo {
        pub fn new(next_remind_time: time::OffsetDateTime, reminder_settings: serde_json::Value) -> Self {
            Self {
                last_remind_time: None,
                next_remind_time,
                reminder_settings,
            }
        }
    }
    #[doc = "Info of task expiry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ExpiryInfo {
        #[doc = "The last expiry notification time."]
        #[serde(rename = "lastExpiryNotificationTime", default, with = "azure_core::date::rfc3339::option")]
        pub last_expiry_notification_time: Option<time::OffsetDateTime>,
        #[doc = "The next expiry notification time."]
        #[serde(rename = "nextExpiryNotificationTime", with = "azure_core::date::rfc3339")]
        pub next_expiry_notification_time: time::OffsetDateTime,
        #[doc = "The expiry time."]
        #[serde(rename = "expiryTime", with = "azure_core::date::rfc3339")]
        pub expiry_time: time::OffsetDateTime,
        #[serde(rename = "expirySettings")]
        pub expiry_settings: expiry_info::ExpirySettings,
    }
    impl ExpiryInfo {
        pub fn new(
            next_expiry_notification_time: time::OffsetDateTime,
            expiry_time: time::OffsetDateTime,
            expiry_settings: expiry_info::ExpirySettings,
        ) -> Self {
            Self {
                last_expiry_notification_time: None,
                next_expiry_notification_time,
                expiry_time,
                expiry_settings,
            }
        }
    }
    pub mod expiry_info {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct ExpirySettings {
            #[doc = "The time of expiry."]
            #[serde(rename = "expireAfter")]
            pub expire_after: serde_json::Value,
            #[serde(
                rename = "notifyOnExpiration",
                default,
                deserialize_with = "azure_core::util::deserialize_null_as_default",
                skip_serializing_if = "Vec::is_empty"
            )]
            pub notify_on_expiration: Vec<String>,
        }
        impl ExpirySettings {
            pub fn new(expire_after: serde_json::Value) -> Self {
                Self {
                    expire_after,
                    notify_on_expiration: Vec::new(),
                }
            }
        }
    }
}
#[doc = "The workflow task type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WorkflowTaskUnion {
    Approval(Approval),
    SimpleTask(SimpleTask),
}
#[doc = "The detail of validation rule violated in a workflow."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowValidationRuleViolation {
    #[doc = "The severity of the validation rule."]
    pub severity: workflow_validation_rule_violation::Severity,
    #[doc = "The location where the violation happens."]
    pub location: workflow_validation_rule_violation::Location,
    #[doc = "The detail about how the validation rule is violated."]
    pub message: String,
}
impl WorkflowValidationRuleViolation {
    pub fn new(
        severity: workflow_validation_rule_violation::Severity,
        location: workflow_validation_rule_violation::Location,
        message: String,
    ) -> Self {
        Self {
            severity,
            location,
            message,
        }
    }
}
pub mod workflow_validation_rule_violation {
    use super::*;
    #[doc = "The severity of the validation rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "warning")]
        Warning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Severity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Severity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Severity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("Severity", 0u32, "error"),
                Self::Warning => serializer.serialize_unit_variant("Severity", 1u32, "warning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The location where the violation happens."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Location {
        #[doc = "The validation violation location type."]
        #[serde(rename = "type")]
        pub type_: location::Type,
        #[doc = "The name of the action where the violation happens."]
        #[serde(rename = "actionName", default, skip_serializing_if = "Option::is_none")]
        pub action_name: Option<String>,
        #[doc = "The key of the action parameter where the violation happens."]
        #[serde(rename = "parameterKey", default, skip_serializing_if = "Option::is_none")]
        pub parameter_key: Option<String>,
    }
    impl Location {
        pub fn new(type_: location::Type) -> Self {
            Self {
                type_,
                action_name: None,
                parameter_key: None,
            }
        }
    }
    pub mod location {
        use super::*;
        #[doc = "The validation violation location type."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Type")]
        pub enum Type {
            #[serde(rename = "workflow")]
            Workflow,
            #[serde(rename = "action")]
            Action,
            #[serde(rename = "actionParameter")]
            ActionParameter,
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
                    Self::Workflow => serializer.serialize_unit_variant("Type", 0u32, "workflow"),
                    Self::Action => serializer.serialize_unit_variant("Type", 1u32, "action"),
                    Self::ActionParameter => serializer.serialize_unit_variant("Type", 2u32, "actionParameter"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowValidationRuleViolationsList {
    #[doc = "The value of violated validation rule list."]
    pub value: Vec<WorkflowValidationRuleViolation>,
}
impl WorkflowValidationRuleViolationsList {
    pub fn new(value: Vec<WorkflowValidationRuleViolation>) -> Self {
        Self { value }
    }
}
