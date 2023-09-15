#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An attestation resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attestation {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of an attestation resource."]
    pub properties: AttestationProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Attestation {
    pub fn new(properties: AttestationProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "A piece of evidence supporting the compliance state set in the attestation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttestationEvidence {
    #[doc = "The description for this piece of evidence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The URI location of the evidence."]
    #[serde(rename = "sourceUri", default, skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
}
impl AttestationEvidence {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of attestations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttestationListResult {
    #[doc = "Array of attestation definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Attestation>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AttestationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AttestationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an attestation resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttestationProperties {
    #[doc = "The resource ID of the policy assignment that the attestation is setting the state for."]
    #[serde(rename = "policyAssignmentId")]
    pub policy_assignment_id: String,
    #[doc = "The policy definition reference ID from a policy set definition that the attestation is setting the state for. If the policy assignment assigns a policy set definition the attestation can choose a definition within the set definition with this property or omit this and set the state for the entire set definition."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[doc = "The compliance state that should be set on the resource."]
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<attestation_properties::ComplianceState>,
    #[doc = "The time the compliance state should expire."]
    #[serde(rename = "expiresOn", default, with = "azure_core::date::rfc3339::option")]
    pub expires_on: Option<time::OffsetDateTime>,
    #[doc = "The person responsible for setting the state of the resource. This value is typically an Azure Active Directory object ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[doc = "Comments describing why this attestation was created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[doc = "The evidence supporting the compliance state set in this attestation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub evidence: Vec<AttestationEvidence>,
    #[doc = "The status of the attestation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The time the compliance state was last changed in this attestation."]
    #[serde(rename = "lastComplianceStateChangeAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_compliance_state_change_at: Option<time::OffsetDateTime>,
    #[doc = "The time the evidence was assessed"]
    #[serde(rename = "assessmentDate", default, with = "azure_core::date::rfc3339::option")]
    pub assessment_date: Option<time::OffsetDateTime>,
    #[doc = "Additional metadata for this attestation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl AttestationProperties {
    pub fn new(policy_assignment_id: String) -> Self {
        Self {
            policy_assignment_id,
            policy_definition_reference_id: None,
            compliance_state: None,
            expires_on: None,
            owner: None,
            comments: None,
            evidence: Vec::new(),
            provisioning_state: None,
            last_compliance_state_change_at: None,
            assessment_date: None,
            metadata: None,
        }
    }
}
pub mod attestation_properties {
    use super::*;
    #[doc = "The compliance state that should be set on the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ComplianceState")]
    pub enum ComplianceState {
        Compliant,
        NonCompliant,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ComplianceState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ComplianceState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ComplianceState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Compliant => serializer.serialize_unit_variant("ComplianceState", 0u32, "Compliant"),
                Self::NonCompliant => serializer.serialize_unit_variant("ComplianceState", 1u32, "NonCompliant"),
                Self::Unknown => serializer.serialize_unit_variant("ComplianceState", 2u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The check policy restrictions parameters describing the resource that is being evaluated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckManagementGroupRestrictionsRequest {
    #[doc = "The information about the resource that will be evaluated."]
    #[serde(rename = "resourceDetails", default, skip_serializing_if = "Option::is_none")]
    pub resource_details: Option<CheckRestrictionsResourceDetails>,
    #[doc = "The list of fields and values that should be evaluated for potential restrictions."]
    #[serde(
        rename = "pendingFields",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pending_fields: Vec<PendingField>,
}
impl CheckManagementGroupRestrictionsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The check policy restrictions parameters describing the resource that is being evaluated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckRestrictionsRequest {
    #[doc = "The information about the resource that will be evaluated."]
    #[serde(rename = "resourceDetails")]
    pub resource_details: CheckRestrictionsResourceDetails,
    #[doc = "The list of fields and values that should be evaluated for potential restrictions."]
    #[serde(
        rename = "pendingFields",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pending_fields: Vec<PendingField>,
}
impl CheckRestrictionsRequest {
    pub fn new(resource_details: CheckRestrictionsResourceDetails) -> Self {
        Self {
            resource_details,
            pending_fields: Vec::new(),
        }
    }
}
#[doc = "The information about the resource that will be evaluated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckRestrictionsResourceDetails {
    #[doc = "The resource content. This should include whatever properties are already known and can be a partial set of all resource properties."]
    #[serde(rename = "resourceContent")]
    pub resource_content: serde_json::Value,
    #[doc = "The api-version of the resource content."]
    #[serde(rename = "apiVersion", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[doc = "The scope where the resource is being created. For example, if the resource is a child resource this would be the parent resource's resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
impl CheckRestrictionsResourceDetails {
    pub fn new(resource_content: serde_json::Value) -> Self {
        Self {
            resource_content,
            api_version: None,
            scope: None,
        }
    }
}
#[doc = "The result of a check policy restrictions evaluation on a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckRestrictionsResult {
    #[doc = "The restrictions that will be placed on various fields in the resource by policy."]
    #[serde(
        rename = "fieldRestrictions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub field_restrictions: Vec<FieldRestrictions>,
    #[doc = "Evaluation results for the provided partial resource content."]
    #[serde(rename = "contentEvaluationResult", default, skip_serializing_if = "Option::is_none")]
    pub content_evaluation_result: Option<check_restrictions_result::ContentEvaluationResult>,
}
impl CheckRestrictionsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_restrictions_result {
    use super::*;
    #[doc = "Evaluation results for the provided partial resource content."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ContentEvaluationResult {
        #[doc = "Policy evaluation results against the given resource content. This will indicate if the partial content that was provided will be denied as-is."]
        #[serde(
            rename = "policyEvaluations",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub policy_evaluations: Vec<PolicyEvaluationResult>,
    }
    impl ContentEvaluationResult {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The compliance state rollup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComplianceDetail {
    #[doc = "The compliance state."]
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<String>,
    #[doc = "Summarized count value for this compliance state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
impl ComplianceDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Component event details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentEventDetails {
    #[doc = "Component Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Component type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Component name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Timestamp for component policy event record."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Tenant ID for the policy event record."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Principal object ID for the user who initiated the resource component operation that triggered the policy event."]
    #[serde(rename = "principalOid", default, skip_serializing_if = "Option::is_none")]
    pub principal_oid: Option<String>,
    #[doc = "Policy definition action, i.e. effect."]
    #[serde(rename = "policyDefinitionAction", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_action: Option<String>,
}
impl ComponentEventDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Evaluation details of policy language expressions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentExpressionEvaluationDetails {
    #[doc = "Evaluation result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[doc = "Expression evaluated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[doc = "The kind of expression that was evaluated."]
    #[serde(rename = "expressionKind", default, skip_serializing_if = "Option::is_none")]
    pub expression_kind: Option<String>,
    #[doc = "Property path if the expression is a field or an alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Value of the expression."]
    #[serde(rename = "expressionValue", default, skip_serializing_if = "Option::is_none")]
    pub expression_value: Option<serde_json::Value>,
    #[doc = "Target value to be compared with the expression value."]
    #[serde(rename = "targetValue", default, skip_serializing_if = "Option::is_none")]
    pub target_value: Option<serde_json::Value>,
    #[doc = "Operator to compare the expression value and the target value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
}
impl ComponentExpressionEvaluationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy evaluation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentPolicyEvaluationDetails {
    #[doc = "Details of the evaluated expressions."]
    #[serde(
        rename = "evaluatedExpressions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub evaluated_expressions: Vec<ComponentExpressionEvaluationDetails>,
    #[doc = "Additional textual reason for the evaluation outcome."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ComponentPolicyEvaluationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Component Policy State record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentPolicyState {
    #[doc = "OData entity ID; always set to null since component policy state records do not have an entity ID."]
    #[serde(rename = "@odata.id", default, skip_serializing_if = "Option::is_none")]
    pub odata_id: Option<String>,
    #[doc = "OData context string; used by OData clients to resolve type information based on metadata."]
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[doc = "Timestamp for the component policy state record."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Component Id."]
    #[serde(rename = "componentId", default, skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    #[doc = "Component type."]
    #[serde(rename = "componentType", default, skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    #[doc = "Component name."]
    #[serde(rename = "componentName", default, skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    #[doc = "Resource ID."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Policy assignment ID."]
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[doc = "Policy definition ID."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "Subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Resource location."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "Resource group name."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "Policy assignment name."]
    #[serde(rename = "policyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_name: Option<String>,
    #[doc = "Policy assignment owner."]
    #[serde(rename = "policyAssignmentOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_owner: Option<String>,
    #[doc = "Policy assignment parameters."]
    #[serde(rename = "policyAssignmentParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_parameters: Option<String>,
    #[doc = "Policy assignment scope."]
    #[serde(rename = "policyAssignmentScope", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_scope: Option<String>,
    #[doc = "Policy definition name."]
    #[serde(rename = "policyDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_name: Option<String>,
    #[doc = "Policy definition action, i.e. effect."]
    #[serde(rename = "policyDefinitionAction", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_action: Option<String>,
    #[doc = "Policy definition category."]
    #[serde(rename = "policyDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_category: Option<String>,
    #[doc = "Policy set definition ID, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[doc = "Policy set definition name, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_name: Option<String>,
    #[doc = "Policy set definition owner, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_owner: Option<String>,
    #[doc = "Policy set definition category, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_category: Option<String>,
    #[doc = "Policy set definition parameters, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_parameters: Option<String>,
    #[doc = "Reference ID for the policy definition inside the policy set, if the policy assignment is for a policy set."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[doc = "Compliance state of the resource."]
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<String>,
    #[doc = "Policy evaluation details."]
    #[serde(rename = "policyEvaluationDetails", default, skip_serializing_if = "Option::is_none")]
    pub policy_evaluation_details: Option<ComponentPolicyEvaluationDetails>,
    #[doc = "Policy definition group names."]
    #[serde(
        rename = "policyDefinitionGroupNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policy_definition_group_names: Vec<String>,
    #[doc = "Evaluated policy definition version."]
    #[serde(rename = "policyDefinitionVersion", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_version: Option<String>,
    #[doc = "Evaluated policy set definition version."]
    #[serde(rename = "policySetDefinitionVersion", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_version: Option<String>,
    #[doc = "Evaluated policy assignment version."]
    #[serde(rename = "policyAssignmentVersion", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_version: Option<String>,
}
impl ComponentPolicyState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentPolicyStatesQueryResults {
    #[doc = "OData context string; used by OData clients to resolve type information based on metadata."]
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[doc = "OData entity count; represents the number of policy state records returned."]
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[doc = "Query results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ComponentPolicyState>,
}
impl ComponentPolicyStatesQueryResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Component state details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentStateDetails {
    #[doc = "Component Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Component type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Component name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Component compliance evaluation timestamp."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Component compliance state."]
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<String>,
}
impl ComponentStateDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Internal error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDefinition>,
    #[doc = "Additional scenario specific error details."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<TypedErrorInfo>,
}
impl ErrorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
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
#[doc = "Evaluation details of policy language expressions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpressionEvaluationDetails {
    #[doc = "Evaluation result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[doc = "Expression evaluated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[doc = "The kind of expression that was evaluated."]
    #[serde(rename = "expressionKind", default, skip_serializing_if = "Option::is_none")]
    pub expression_kind: Option<String>,
    #[doc = "Property path if the expression is a field or an alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Value of the expression."]
    #[serde(rename = "expressionValue", default, skip_serializing_if = "Option::is_none")]
    pub expression_value: Option<serde_json::Value>,
    #[doc = "Target value to be compared with the expression value."]
    #[serde(rename = "targetValue", default, skip_serializing_if = "Option::is_none")]
    pub target_value: Option<serde_json::Value>,
    #[doc = "Operator to compare the expression value and the target value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
}
impl ExpressionEvaluationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The restrictions on a field imposed by a specific policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FieldRestriction {
    #[doc = "The type of restriction that is imposed on the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<field_restriction::Result>,
    #[doc = "The value that policy will set for the field if the user does not provide a value."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "The values that policy either requires or denies for the field."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
    #[doc = "Resource identifiers for a policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<PolicyReference>,
}
impl FieldRestriction {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod field_restriction {
    use super::*;
    #[doc = "The type of restriction that is imposed on the field."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Result")]
    pub enum Result {
        Required,
        Removed,
        Deny,
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
                Self::Required => serializer.serialize_unit_variant("Result", 0u32, "Required"),
                Self::Removed => serializer.serialize_unit_variant("Result", 1u32, "Removed"),
                Self::Deny => serializer.serialize_unit_variant("Result", 2u32, "Deny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The restrictions that will be placed on a field in the resource by policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FieldRestrictions {
    #[doc = "The name of the field. This can be a top-level property like 'name' or 'type' or an Azure Policy field alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[doc = "The restrictions placed on that field by policy."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub restrictions: Vec<FieldRestriction>,
}
impl FieldRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Evaluation details of IfNotExists effect."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IfNotExistsEvaluationDetails {
    #[doc = "ID of the last evaluated resource for IfNotExists effect."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Total number of resources to which the existence condition is applicable."]
    #[serde(rename = "totalResources", default, skip_serializing_if = "Option::is_none")]
    pub total_resources: Option<i64>,
}
impl IfNotExistsEvaluationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type MetadataDocument = String;
#[doc = "Operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Display metadata associated with the operation."]
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
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Resource provider name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource name on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Operation description."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "List of available operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsListResults {
    #[doc = "OData entity count; represents the number of operations returned."]
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[doc = "List of available operations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
}
impl OperationsListResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A field that should be evaluated against Azure Policy to determine restrictions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PendingField {
    #[doc = "The name of the field. This can be a top-level property like 'name' or 'type' or an Azure Policy field alias."]
    pub field: String,
    #[doc = "The list of potential values for the field that should be evaluated against Azure Policy."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl PendingField {
    pub fn new(field: String) -> Self {
        Self { field, values: Vec::new() }
    }
}
#[doc = "Policy assignment summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyAssignmentSummary {
    #[doc = "Policy assignment ID."]
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[doc = "Policy set definition ID, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[doc = "Compliance summary on a particular summary level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<SummaryResults>,
    #[doc = "Policy definitions summary."]
    #[serde(
        rename = "policyDefinitions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policy_definitions: Vec<PolicyDefinitionSummary>,
    #[doc = "Policy definition group summary."]
    #[serde(
        rename = "policyGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policy_groups: Vec<PolicyGroupSummary>,
}
impl PolicyAssignmentSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy definition summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDefinitionSummary {
    #[doc = "Policy definition ID."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "Policy definition reference ID."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[doc = "Policy definition group names."]
    #[serde(
        rename = "policyDefinitionGroupNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policy_definition_group_names: Vec<String>,
    #[doc = "Policy effect, i.e. policy definition action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[doc = "Compliance summary on a particular summary level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<SummaryResults>,
}
impl PolicyDefinitionSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDetails {
    #[doc = "The ID of the policy definition."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "The ID of the policy assignment."]
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[doc = "The display name of the policy assignment."]
    #[serde(rename = "policyAssignmentDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_display_name: Option<String>,
    #[doc = "The scope of the policy assignment."]
    #[serde(rename = "policyAssignmentScope", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_scope: Option<String>,
    #[doc = "The ID of the policy set definition."]
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[doc = "The policy definition reference ID within the policy set definition."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
}
impl PolicyDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy evaluation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyEvaluationDetails {
    #[doc = "Details of the evaluated expressions."]
    #[serde(
        rename = "evaluatedExpressions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub evaluated_expressions: Vec<ExpressionEvaluationDetails>,
    #[doc = "Evaluation details of IfNotExists effect."]
    #[serde(rename = "ifNotExistsDetails", default, skip_serializing_if = "Option::is_none")]
    pub if_not_exists_details: Option<IfNotExistsEvaluationDetails>,
}
impl PolicyEvaluationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a non-compliant policy evaluation against the given resource content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyEvaluationResult {
    #[doc = "Resource identifiers for a policy."]
    #[serde(rename = "policyInfo", default, skip_serializing_if = "Option::is_none")]
    pub policy_info: Option<PolicyReference>,
    #[doc = "The result of the policy evaluation against the resource. This will typically be 'NonCompliant' but may contain other values if errors were encountered."]
    #[serde(rename = "evaluationResult", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_result: Option<String>,
    #[doc = "Policy evaluation details."]
    #[serde(rename = "evaluationDetails", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_details: Option<PolicyEvaluationDetails>,
}
impl PolicyEvaluationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy event record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyEvent {
    #[doc = "OData entity ID; always set to null since policy event records do not have an entity ID."]
    #[serde(rename = "@odata.id", default, skip_serializing_if = "Option::is_none")]
    pub odata_id: Option<String>,
    #[doc = "OData context string; used by OData clients to resolve type information based on metadata."]
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[doc = "Timestamp for the policy event record."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Resource ID."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Policy assignment ID."]
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[doc = "Policy definition ID."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "Effective parameters for the policy assignment."]
    #[serde(rename = "effectiveParameters", default, skip_serializing_if = "Option::is_none")]
    pub effective_parameters: Option<String>,
    #[doc = "Flag which states whether the resource is compliant against the policy assignment it was evaluated against."]
    #[serde(rename = "isCompliant", default, skip_serializing_if = "Option::is_none")]
    pub is_compliant: Option<bool>,
    #[doc = "Subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Resource location."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "Resource group name."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "List of resource tags."]
    #[serde(rename = "resourceTags", default, skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<String>,
    #[doc = "Policy assignment name."]
    #[serde(rename = "policyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_name: Option<String>,
    #[doc = "Policy assignment owner."]
    #[serde(rename = "policyAssignmentOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_owner: Option<String>,
    #[doc = "Policy assignment parameters."]
    #[serde(rename = "policyAssignmentParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_parameters: Option<String>,
    #[doc = "Policy assignment scope."]
    #[serde(rename = "policyAssignmentScope", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_scope: Option<String>,
    #[doc = "Policy definition name."]
    #[serde(rename = "policyDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_name: Option<String>,
    #[doc = "Policy definition action, i.e. effect."]
    #[serde(rename = "policyDefinitionAction", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_action: Option<String>,
    #[doc = "Policy definition category."]
    #[serde(rename = "policyDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_category: Option<String>,
    #[doc = "Policy set definition ID, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[doc = "Policy set definition name, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_name: Option<String>,
    #[doc = "Policy set definition owner, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_owner: Option<String>,
    #[doc = "Policy set definition category, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_category: Option<String>,
    #[doc = "Policy set definition parameters, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_parameters: Option<String>,
    #[doc = "Comma separated list of management group IDs, which represent the hierarchy of the management groups the resource is under."]
    #[serde(rename = "managementGroupIds", default, skip_serializing_if = "Option::is_none")]
    pub management_group_ids: Option<String>,
    #[doc = "Reference ID for the policy definition inside the policy set, if the policy assignment is for a policy set."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[doc = "Compliance state of the resource."]
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<String>,
    #[doc = "Tenant ID for the policy event record."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Principal object ID for the user who initiated the resource operation that triggered the policy event."]
    #[serde(rename = "principalOid", default, skip_serializing_if = "Option::is_none")]
    pub principal_oid: Option<String>,
    #[doc = "Components events records populated only when URL contains $expand=components clause."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub components: Vec<ComponentEventDetails>,
}
impl PolicyEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyEventsQueryResults {
    #[doc = "OData context string; used by OData clients to resolve type information based on metadata."]
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[doc = "OData entity count; represents the number of policy event records returned."]
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[doc = "Odata next link; URL to get the next set of results."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
    #[doc = "Query results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PolicyEvent>,
}
impl azure_core::Continuable for PolicyEventsQueryResults {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PolicyEventsQueryResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy definition group summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyGroupSummary {
    #[doc = "Policy group name."]
    #[serde(rename = "policyGroupName", default, skip_serializing_if = "Option::is_none")]
    pub policy_group_name: Option<String>,
    #[doc = "Compliance summary on a particular summary level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<SummaryResults>,
}
impl PolicyGroupSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy metadata resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyMetadata {
    #[doc = "The properties of the policy metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyMetadataProperties>,
    #[doc = "The ID of the policy metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the policy metadata."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the policy metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl PolicyMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of policy metadata resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyMetadataCollection {
    #[doc = "Array of policy metadata definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SlimPolicyMetadata>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PolicyMetadataCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PolicyMetadataCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the policy metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyMetadataProperties {
    #[serde(flatten)]
    pub policy_metadata_slim_properties: PolicyMetadataSlimProperties,
    #[doc = "The description of the policy metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The requirements of the policy metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requirements: Option<String>,
}
impl PolicyMetadataProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the policy metadata, excluding properties containing large strings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyMetadataSlimProperties {
    #[doc = "The policy metadata identifier."]
    #[serde(rename = "metadataId", default, skip_serializing_if = "Option::is_none")]
    pub metadata_id: Option<String>,
    #[doc = "The category of the policy metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The title of the policy metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The owner of the policy metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[doc = "Url for getting additional content about the resource metadata."]
    #[serde(rename = "additionalContentUrl", default, skip_serializing_if = "Option::is_none")]
    pub additional_content_url: Option<String>,
    #[doc = "Additional metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl PolicyMetadataSlimProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource identifiers for a policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyReference {
    #[doc = "The resource identifier of the policy definition."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "The resource identifier of the policy set definition."]
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[doc = "The reference identifier of a specific policy definition within a policy set definition."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[doc = "The resource identifier of the policy assignment."]
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
}
impl PolicyReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy state record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyState {
    #[doc = "OData entity ID; always set to null since policy state records do not have an entity ID."]
    #[serde(rename = "@odata.id", default, skip_serializing_if = "Option::is_none")]
    pub odata_id: Option<String>,
    #[doc = "OData context string; used by OData clients to resolve type information based on metadata."]
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[doc = "Timestamp for the policy state record."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Resource ID."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Policy assignment ID."]
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[doc = "Policy definition ID."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "Effective parameters for the policy assignment."]
    #[serde(rename = "effectiveParameters", default, skip_serializing_if = "Option::is_none")]
    pub effective_parameters: Option<String>,
    #[doc = "Flag which states whether the resource is compliant against the policy assignment it was evaluated against. This property is deprecated; please use ComplianceState instead."]
    #[serde(rename = "isCompliant", default, skip_serializing_if = "Option::is_none")]
    pub is_compliant: Option<bool>,
    #[doc = "Subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Resource location."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "Resource group name."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "List of resource tags."]
    #[serde(rename = "resourceTags", default, skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<String>,
    #[doc = "Policy assignment name."]
    #[serde(rename = "policyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_name: Option<String>,
    #[doc = "Policy assignment owner."]
    #[serde(rename = "policyAssignmentOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_owner: Option<String>,
    #[doc = "Policy assignment parameters."]
    #[serde(rename = "policyAssignmentParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_parameters: Option<String>,
    #[doc = "Policy assignment scope."]
    #[serde(rename = "policyAssignmentScope", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_scope: Option<String>,
    #[doc = "Policy definition name."]
    #[serde(rename = "policyDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_name: Option<String>,
    #[doc = "Policy definition action, i.e. effect."]
    #[serde(rename = "policyDefinitionAction", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_action: Option<String>,
    #[doc = "Policy definition category."]
    #[serde(rename = "policyDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_category: Option<String>,
    #[doc = "Policy set definition ID, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[doc = "Policy set definition name, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_name: Option<String>,
    #[doc = "Policy set definition owner, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_owner: Option<String>,
    #[doc = "Policy set definition category, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_category: Option<String>,
    #[doc = "Policy set definition parameters, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_parameters: Option<String>,
    #[doc = "Comma separated list of management group IDs, which represent the hierarchy of the management groups the resource is under."]
    #[serde(rename = "managementGroupIds", default, skip_serializing_if = "Option::is_none")]
    pub management_group_ids: Option<String>,
    #[doc = "Reference ID for the policy definition inside the policy set, if the policy assignment is for a policy set."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[doc = "Compliance state of the resource."]
    #[serde(rename = "complianceState", default, skip_serializing_if = "Option::is_none")]
    pub compliance_state: Option<String>,
    #[doc = "Policy evaluation details."]
    #[serde(rename = "policyEvaluationDetails", default, skip_serializing_if = "Option::is_none")]
    pub policy_evaluation_details: Option<PolicyEvaluationDetails>,
    #[doc = "Policy definition group names."]
    #[serde(
        rename = "policyDefinitionGroupNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policy_definition_group_names: Vec<String>,
    #[doc = "Components state compliance records populated only when URL contains $expand=components clause."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub components: Vec<ComponentStateDetails>,
    #[doc = "Evaluated policy definition version."]
    #[serde(rename = "policyDefinitionVersion", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_version: Option<String>,
    #[doc = "Evaluated policy set definition version."]
    #[serde(rename = "policySetDefinitionVersion", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_version: Option<String>,
    #[doc = "Evaluated policy assignment version."]
    #[serde(rename = "policyAssignmentVersion", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_version: Option<String>,
}
impl PolicyState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyStatesQueryResults {
    #[doc = "OData context string; used by OData clients to resolve type information based on metadata."]
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[doc = "OData entity count; represents the number of policy state records returned."]
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[doc = "Odata next link; URL to get the next set of results."]
    #[serde(rename = "@odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
    #[doc = "Query results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PolicyState>,
}
impl azure_core::Continuable for PolicyStatesQueryResults {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PolicyStatesQueryResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy tracked resource record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyTrackedResource {
    #[doc = "The ID of the policy tracked resource."]
    #[serde(rename = "trackedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub tracked_resource_id: Option<String>,
    #[doc = "The policy details."]
    #[serde(rename = "policyDetails", default, skip_serializing_if = "Option::is_none")]
    pub policy_details: Option<PolicyDetails>,
    #[doc = "The details of the policy triggered deployment that created or modified the tracked resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<TrackedResourceModificationDetails>,
    #[doc = "The details of the policy triggered deployment that created or modified the tracked resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<TrackedResourceModificationDetails>,
    #[doc = "Timestamp of the last update to the tracked resource."]
    #[serde(rename = "lastUpdateUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_update_utc: Option<time::OffsetDateTime>,
}
impl PolicyTrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyTrackedResourcesQueryResults {
    #[doc = "Query results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PolicyTrackedResource>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PolicyTrackedResourcesQueryResults {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PolicyTrackedResourcesQueryResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryFailure {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<query_failure::Error>,
}
impl azure_core::Continuable for QueryFailure {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl QueryFailure {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod query_failure {
    use super::*;
    #[doc = "Error definition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Description of the error."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The remediation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Remediation {
    #[doc = "The remediation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RemediationProperties>,
    #[doc = "The ID of the remediation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the remediation."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the remediation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Remediation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of a single deployment created by the remediation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemediationDeployment {
    #[doc = "Resource ID of the resource that is being remediated by the deployment."]
    #[serde(rename = "remediatedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub remediated_resource_id: Option<String>,
    #[doc = "Resource ID of the template deployment that will remediate the resource."]
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[doc = "Status of the remediation deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Location of the resource that is being remediated."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
    #[doc = "The time at which the remediation was created."]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "The time at which the remediation deployment was last updated."]
    #[serde(rename = "lastUpdatedOn", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_on: Option<time::OffsetDateTime>,
}
impl RemediationDeployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The deployment status summary for all deployments created by the remediation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemediationDeploymentSummary {
    #[doc = "The number of deployments required by the remediation."]
    #[serde(rename = "totalDeployments", default, skip_serializing_if = "Option::is_none")]
    pub total_deployments: Option<i64>,
    #[doc = "The number of deployments required by the remediation that have succeeded."]
    #[serde(rename = "successfulDeployments", default, skip_serializing_if = "Option::is_none")]
    pub successful_deployments: Option<i64>,
    #[doc = "The number of deployments required by the remediation that have failed."]
    #[serde(rename = "failedDeployments", default, skip_serializing_if = "Option::is_none")]
    pub failed_deployments: Option<i64>,
}
impl RemediationDeploymentSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of deployments for a remediation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemediationDeploymentsListResult {
    #[doc = "Array of deployments for the remediation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RemediationDeployment>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RemediationDeploymentsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RemediationDeploymentsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The filters that will be applied to determine which resources to remediate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemediationFilters {
    #[doc = "The resource locations that will be remediated."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub locations: Vec<String>,
}
impl RemediationFilters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of remediations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemediationListResult {
    #[doc = "Array of remediation definitions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Remediation>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RemediationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RemediationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The remediation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemediationProperties {
    #[doc = "The resource ID of the policy assignment that should be remediated."]
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[doc = "The policy definition reference ID of the individual definition that should be remediated. Required when the policy assignment being remediated assigns a policy set definition."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[doc = "The way resources to remediate are discovered. Defaults to ExistingNonCompliant if not specified."]
    #[serde(rename = "resourceDiscoveryMode", default, skip_serializing_if = "Option::is_none")]
    pub resource_discovery_mode: Option<remediation_properties::ResourceDiscoveryMode>,
    #[doc = "The status of the remediation. This refers to the entire remediation task, not individual deployments. Allowed values are Evaluating, Canceled, Cancelling, Failed, Complete, or Succeeded."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The time at which the remediation was created."]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "The time at which the remediation was last updated."]
    #[serde(rename = "lastUpdatedOn", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_on: Option<time::OffsetDateTime>,
    #[doc = "The filters that will be applied to determine which resources to remediate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<RemediationFilters>,
    #[doc = "The deployment status summary for all deployments created by the remediation."]
    #[serde(rename = "deploymentStatus", default, skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<RemediationDeploymentSummary>,
    #[doc = "The remediation status message. Provides additional details regarding the state of the remediation."]
    #[serde(rename = "statusMessage", default, skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[doc = "The remediation correlation Id. Can be used to find events related to the remediation in the activity log."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "Determines the max number of resources that can be remediated by the remediation job. If not provided, the default resource count is used."]
    #[serde(rename = "resourceCount", default, skip_serializing_if = "Option::is_none")]
    pub resource_count: Option<i32>,
    #[doc = "Determines how many resources to remediate at any given time. Can be used to increase or reduce the pace of the remediation. If not provided, the default parallel deployments value is used."]
    #[serde(rename = "parallelDeployments", default, skip_serializing_if = "Option::is_none")]
    pub parallel_deployments: Option<i32>,
    #[doc = "The remediation failure threshold settings"]
    #[serde(rename = "failureThreshold", default, skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<remediation_properties::FailureThreshold>,
}
impl RemediationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod remediation_properties {
    use super::*;
    #[doc = "The way resources to remediate are discovered. Defaults to ExistingNonCompliant if not specified."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResourceDiscoveryMode")]
    pub enum ResourceDiscoveryMode {
        ExistingNonCompliant,
        ReEvaluateCompliance,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResourceDiscoveryMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResourceDiscoveryMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResourceDiscoveryMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ExistingNonCompliant => serializer.serialize_unit_variant("ResourceDiscoveryMode", 0u32, "ExistingNonCompliant"),
                Self::ReEvaluateCompliance => serializer.serialize_unit_variant("ResourceDiscoveryMode", 1u32, "ReEvaluateCompliance"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The remediation failure threshold settings"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct FailureThreshold {
        #[doc = "A number between 0.0 to 1.0 representing the percentage failure threshold. The remediation will fail if the percentage of failed remediation operations (i.e. failed deployments) exceeds this threshold."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub percentage: Option<f32>,
    }
    impl FailureThreshold {
        pub fn new() -> Self {
            Self::default()
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
#[doc = "Slim version of policy metadata resource definition, excluding properties with large strings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SlimPolicyMetadata {
    #[doc = "The properties of the policy metadata, excluding properties containing large strings"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyMetadataSlimProperties>,
    #[doc = "The ID of the policy metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the policy metadata."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the policy metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl SlimPolicyMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summarize action results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SummarizeResults {
    #[doc = "OData context string; used by OData clients to resolve type information based on metadata."]
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[doc = "OData entity count; represents the number of summaries returned; always set to 1."]
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[doc = "Summarize action results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Summary>,
}
impl SummarizeResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summary results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Summary {
    #[doc = "OData entity ID; always set to null since summaries do not have an entity ID."]
    #[serde(rename = "@odata.id", default, skip_serializing_if = "Option::is_none")]
    pub odata_id: Option<String>,
    #[doc = "OData context string; used by OData clients to resolve type information based on metadata."]
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[doc = "Compliance summary on a particular summary level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<SummaryResults>,
    #[doc = "Policy assignments summary."]
    #[serde(
        rename = "policyAssignments",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policy_assignments: Vec<PolicyAssignmentSummary>,
}
impl Summary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Compliance summary on a particular summary level."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SummaryResults {
    #[doc = "HTTP POST URI for queryResults action on Microsoft.PolicyInsights to retrieve raw results for the compliance summary. This property will not be available by default in future API versions, but could be queried explicitly."]
    #[serde(rename = "queryResultsUri", default, skip_serializing_if = "Option::is_none")]
    pub query_results_uri: Option<String>,
    #[doc = "Number of non-compliant resources."]
    #[serde(rename = "nonCompliantResources", default, skip_serializing_if = "Option::is_none")]
    pub non_compliant_resources: Option<i32>,
    #[doc = "Number of non-compliant policies."]
    #[serde(rename = "nonCompliantPolicies", default, skip_serializing_if = "Option::is_none")]
    pub non_compliant_policies: Option<i32>,
    #[doc = "The resources summary at this level."]
    #[serde(
        rename = "resourceDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_details: Vec<ComplianceDetail>,
    #[doc = "The policy artifact summary at this level. For query scope level, it represents policy assignment summary. For policy assignment level, it represents policy definitions summary."]
    #[serde(
        rename = "policyDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policy_details: Vec<ComplianceDetail>,
    #[doc = "The policy definition group summary at this level."]
    #[serde(
        rename = "policyGroupDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policy_group_details: Vec<ComplianceDetail>,
}
impl SummaryResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the policy triggered deployment that created or modified the tracked resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResourceModificationDetails {
    #[doc = "The policy details."]
    #[serde(rename = "policyDetails", default, skip_serializing_if = "Option::is_none")]
    pub policy_details: Option<PolicyDetails>,
    #[doc = "The ID of the deployment that created or modified the tracked resource."]
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[doc = "Timestamp of the deployment that created or modified the tracked resource."]
    #[serde(rename = "deploymentTime", default, with = "azure_core::date::rfc3339::option")]
    pub deployment_time: Option<time::OffsetDateTime>,
}
impl TrackedResourceModificationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Scenario specific error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TypedErrorInfo {
    #[doc = "The type of included error details."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The scenario specific error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl TypedErrorInfo {
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
