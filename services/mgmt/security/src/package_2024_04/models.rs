#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Configuration payload for PR Annotations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionableRemediation {
    #[doc = "ActionableRemediation Setting.\r\nNone - the setting was never set.\r\nEnabled - ActionableRemediation is enabled.\r\nDisabled - ActionableRemediation is disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ActionableRemediationState>,
    #[doc = "Gets or sets list of categories and severity levels."]
    #[serde(
        rename = "categoryConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub category_configurations: Vec<CategoryConfiguration>,
    #[doc = "Repository branch configuration for PR Annotations."]
    #[serde(rename = "branchConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub branch_configuration: Option<TargetBranchConfiguration>,
    #[doc = "Update Settings.\r\n\r\nEnabled - Resource should inherit configurations from parent.\r\nDisabled - Resource should not inherit configurations from parent."]
    #[serde(rename = "inheritFromParentState", default, skip_serializing_if = "Option::is_none")]
    pub inherit_from_parent_state: Option<InheritFromParentState>,
}
impl ActionableRemediation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ActionableRemediation Setting.\r\nNone - the setting was never set.\r\nEnabled - ActionableRemediation is enabled.\r\nDisabled - ActionableRemediation is disabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ActionableRemediationState")]
pub enum ActionableRemediationState {
    None,
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ActionableRemediationState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ActionableRemediationState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ActionableRemediationState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ActionableRemediationState", 0u32, "None"),
            Self::Disabled => serializer.serialize_unit_variant("ActionableRemediationState", 1u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("ActionableRemediationState", 2u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Configuration of PR Annotations on default branch.\r\n\r\nEnabled - PR Annotations are enabled on the resource's default branch.\r\nDisabled - PR Annotations are disabled on the resource's default branch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AnnotateDefaultBranchState")]
pub enum AnnotateDefaultBranchState {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AnnotateDefaultBranchState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AnnotateDefaultBranchState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AnnotateDefaultBranchState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("AnnotateDefaultBranchState", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("AnnotateDefaultBranchState", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Authorization payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Authorization {
    #[doc = "Gets or sets one-time OAuth code to exchange for refresh and access tokens.\r\n\r\nOnly used during PUT/PATCH operations. The secret is cleared during GET."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
impl Authorization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AutoDiscovery states."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutoDiscovery")]
pub enum AutoDiscovery {
    Disabled,
    Enabled,
    NotApplicable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutoDiscovery {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutoDiscovery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutoDiscovery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("AutoDiscovery", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("AutoDiscovery", 1u32, "Enabled"),
            Self::NotApplicable => serializer.serialize_unit_variant("AutoDiscovery", 2u32, "NotApplicable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Azure DevOps Organization resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsOrg {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Azure DevOps Organization properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureDevOpsOrgProperties>,
}
impl AzureDevOpsOrg {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsOrgListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AzureDevOpsOrg>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureDevOpsOrgListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AzureDevOpsOrgListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure DevOps Organization properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsOrgProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<::time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
    #[serde(rename = "onboardingState", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_state: Option<OnboardingState>,
    #[doc = "Configuration payload for PR Annotations."]
    #[serde(rename = "actionableRemediation", default, skip_serializing_if = "Option::is_none")]
    pub actionable_remediation: Option<ActionableRemediation>,
}
impl AzureDevOpsOrgProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AzureDevOps Org Inventory Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsOrganizationConfiguration {
    #[doc = "AutoDiscovery states."]
    #[serde(rename = "autoDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub auto_discovery: Option<AutoDiscovery>,
    #[doc = "AzureDevOps Project Inventory Configuration.\r\nDictionary of AzureDevOps project name to desired project configuration.\r\nIf AutoDiscovery is Enabled, this field should be empty or null."]
    #[serde(rename = "projectConfigs", default, skip_serializing_if = "Option::is_none")]
    pub project_configs: Option<serde_json::Value>,
}
impl AzureDevOpsOrganizationConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure DevOps Project resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsProject {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Azure DevOps Project properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureDevOpsProjectProperties>,
}
impl AzureDevOpsProject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AzureDevOps Project Inventory Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsProjectConfiguration {
    #[doc = "AutoDiscovery states."]
    #[serde(rename = "autoDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub auto_discovery: Option<AutoDiscovery>,
    #[doc = "AzureDevOps Repository Inventory Configuration.\r\nDictionary of AzureDevOps repository name to desired repository configuration.\r\nIf AutoDiscovery is Enabled, this field should be null or empty."]
    #[serde(rename = "repositoryConfigs", default, skip_serializing_if = "Option::is_none")]
    pub repository_configs: Option<serde_json::Value>,
}
impl AzureDevOpsProjectConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsProjectListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AzureDevOpsProject>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureDevOpsProjectListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AzureDevOpsProjectListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure DevOps Project properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsProjectProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<::time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Gets or sets parent Azure DevOps Organization name."]
    #[serde(rename = "parentOrgName", default, skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    #[doc = "Gets or sets Azure DevOps Project id."]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
    #[serde(rename = "onboardingState", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_state: Option<OnboardingState>,
    #[doc = "Configuration payload for PR Annotations."]
    #[serde(rename = "actionableRemediation", default, skip_serializing_if = "Option::is_none")]
    pub actionable_remediation: Option<ActionableRemediation>,
}
impl AzureDevOpsProjectProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure DevOps Repository resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsRepository {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Azure DevOps Repository properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureDevOpsRepositoryProperties>,
}
impl AzureDevOpsRepository {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsRepositoryListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AzureDevOpsRepository>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureDevOpsRepositoryListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AzureDevOpsRepositoryListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure DevOps Repository properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDevOpsRepositoryProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<::time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Gets or sets parent Azure DevOps Organization name."]
    #[serde(rename = "parentOrgName", default, skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    #[doc = "Gets or sets parent Azure DevOps Project name."]
    #[serde(rename = "parentProjectName", default, skip_serializing_if = "Option::is_none")]
    pub parent_project_name: Option<String>,
    #[doc = "Gets or sets Azure DevOps Repository id."]
    #[serde(rename = "repoId", default, skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    #[doc = "Gets or sets Azure DevOps Repository url."]
    #[serde(rename = "repoUrl", default, skip_serializing_if = "Option::is_none")]
    pub repo_url: Option<String>,
    #[doc = "Gets or sets Azure DevOps repository visibility, whether it is public or private etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
    #[serde(rename = "onboardingState", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_state: Option<OnboardingState>,
    #[doc = "Configuration payload for PR Annotations."]
    #[serde(rename = "actionableRemediation", default, skip_serializing_if = "Option::is_none")]
    pub actionable_remediation: Option<ActionableRemediation>,
}
impl AzureDevOpsRepositoryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base Resource Inventory configuration changes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaseResourceConfiguration {
    #[doc = "Onboarding states."]
    #[serde(rename = "desiredOnboardingState", default, skip_serializing_if = "Option::is_none")]
    pub desired_onboarding_state: Option<DesiredOnboardingState>,
}
impl BaseResourceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Severity level per category configuration for PR Annotations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CategoryConfiguration {
    #[doc = "Gets or sets minimum severity level for a given category."]
    #[serde(rename = "minimumSeverityLevel", default, skip_serializing_if = "Option::is_none")]
    pub minimum_severity_level: Option<String>,
    #[doc = "Rule categories.\r\nCode - code scanning results.\r\nArtifact scanning results.\r\nDependencies scanning results.\r\nIaC results.\r\nSecrets scanning results.\r\nContainer scanning results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<RuleCategory>,
}
impl CategoryConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Onboarding states."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DesiredOnboardingState")]
pub enum DesiredOnboardingState {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DesiredOnboardingState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DesiredOnboardingState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DesiredOnboardingState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("DesiredOnboardingState", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("DesiredOnboardingState", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Details about DevOps capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevOpsCapability {
    #[doc = "Gets the name of the DevOps capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the value of the DevOps capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl DevOpsCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DevOps Configuration resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevOpsConfiguration {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "DevOps Configuration properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DevOpsConfigurationProperties>,
}
impl DevOpsConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevOpsConfigurationListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DevOpsConfiguration>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DevOpsConfigurationListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DevOpsConfigurationListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DevOps Configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevOpsConfigurationProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<::time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Authorization payload."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    #[doc = "AutoDiscovery states."]
    #[serde(rename = "autoDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub auto_discovery: Option<AutoDiscovery>,
    #[doc = "List of top-level inventory to select when AutoDiscovery is disabled.\r\nThis field is ignored when AutoDiscovery is enabled."]
    #[serde(
        rename = "topLevelInventoryList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub top_level_inventory_list: Vec<String>,
    #[doc = "List of capabilities assigned to the DevOps configuration during the discovery process."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub capabilities: Vec<DevOpsCapability>,
}
impl DevOpsConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DevOpsProvisioningState")]
pub enum DevOpsProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Pending,
    PendingDeletion,
    DeletionSuccess,
    DeletionFailure,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DevOpsProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DevOpsProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DevOpsProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("DevOpsProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("DevOpsProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("DevOpsProvisioningState", 2u32, "Canceled"),
            Self::Pending => serializer.serialize_unit_variant("DevOpsProvisioningState", 3u32, "Pending"),
            Self::PendingDeletion => serializer.serialize_unit_variant("DevOpsProvisioningState", 4u32, "PendingDeletion"),
            Self::DeletionSuccess => serializer.serialize_unit_variant("DevOpsProvisioningState", 5u32, "DeletionSuccess"),
            Self::DeletionFailure => serializer.serialize_unit_variant("DevOpsProvisioningState", 6u32, "DeletionFailure"),
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
#[doc = "GitHub Owner resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubOwner {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "GitHub Owner properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GitHubOwnerProperties>,
}
impl GitHubOwner {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitHub Owner Inventory Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubOwnerConfiguration {
    #[doc = "AutoDiscovery states."]
    #[serde(rename = "autoDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub auto_discovery: Option<AutoDiscovery>,
    #[doc = "GitHub Repository Inventory Configuration.\r\nDictionary of GitHub repository name to desired repository configuration.\r\nIf AutoDiscovery is Enabled, this field should be null or empty."]
    #[serde(rename = "repositoryConfigs", default, skip_serializing_if = "Option::is_none")]
    pub repository_configs: Option<serde_json::Value>,
}
impl GitHubOwnerConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubOwnerListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitHubOwner>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GitHubOwnerListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GitHubOwnerListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitHub Owner properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubOwnerProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<::time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Gets or sets GitHub Owner url."]
    #[serde(rename = "ownerUrl", default, skip_serializing_if = "Option::is_none")]
    pub owner_url: Option<String>,
    #[doc = "Gets or sets internal GitHub id."]
    #[serde(rename = "gitHubInternalId", default, skip_serializing_if = "Option::is_none")]
    pub git_hub_internal_id: Option<String>,
    #[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
    #[serde(rename = "onboardingState", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_state: Option<OnboardingState>,
}
impl GitHubOwnerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitHub Repository resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubRepository {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "GitHub Repository properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GitHubRepositoryProperties>,
}
impl GitHubRepository {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubRepositoryListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitHubRepository>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GitHubRepositoryListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GitHubRepositoryListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitHub Repository properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubRepositoryProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<::time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Gets or sets GitHub Repository id.\r\n\r\nThis is a numeric id defined by Github.\r\nEg: \"123456\"."]
    #[serde(rename = "repoId", default, skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    #[doc = "Gets or sets GitHub Repository name.\r\nEg: \"new-repo-1\"."]
    #[serde(rename = "repoName", default, skip_serializing_if = "Option::is_none")]
    pub repo_name: Option<String>,
    #[doc = "Gets or sets GitHub Full Name.\r\nRepository name, prefixed with Owner name.\r\nEg: \"my-org/new-repo-1\"."]
    #[serde(rename = "repoFullName", default, skip_serializing_if = "Option::is_none")]
    pub repo_full_name: Option<String>,
    #[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
    #[serde(rename = "onboardingState", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_state: Option<OnboardingState>,
    #[doc = "Gets or sets GitHub Repository url."]
    #[serde(rename = "repoUrl", default, skip_serializing_if = "Option::is_none")]
    pub repo_url: Option<String>,
    #[doc = "Gets or sets parent GitHub Owner name."]
    #[serde(rename = "parentOwnerName", default, skip_serializing_if = "Option::is_none")]
    pub parent_owner_name: Option<String>,
}
impl GitHubRepositoryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitLab Group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLabGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "GitLab Group properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GitLabGroupProperties>,
}
impl GitLabGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitLab Group Inventory Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLabGroupConfiguration {
    #[doc = "AutoDiscovery states."]
    #[serde(rename = "autoDiscovery", default, skip_serializing_if = "Option::is_none")]
    pub auto_discovery: Option<AutoDiscovery>,
    #[doc = "GitLab Project Inventory Configuration.\r\nDictionary of GitLab fully-qualified project name to desired project configuration.\r\nIf AutoDiscovery is Enabled, this field should be null or empty."]
    #[serde(rename = "projectConfigs", default, skip_serializing_if = "Option::is_none")]
    pub project_configs: Option<serde_json::Value>,
}
impl GitLabGroupConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLabGroupListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitLabGroup>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GitLabGroupListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GitLabGroupListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitLab Group properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLabGroupProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<::time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Gets or sets the fully-qualified name of the Group object.\r\n\r\nThis contains the entire namespace hierarchy where namespaces are separated by the '$' character."]
    #[serde(rename = "fullyQualifiedName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_name: Option<String>,
    #[doc = "Gets or sets the human readable fully-qualified name of the Group object.\r\n\r\nThis contains the entire namespace hierarchy as seen on GitLab UI where namespaces are separated by the '/' character."]
    #[serde(rename = "fullyQualifiedFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_friendly_name: Option<String>,
    #[doc = "Gets or sets the url of the GitLab Group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
    #[serde(rename = "onboardingState", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_state: Option<OnboardingState>,
}
impl GitLabGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitLab Project resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLabProject {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "GitLab Project properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GitLabProjectProperties>,
}
impl GitLabProject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of RP resources which supports pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLabProjectListResponse {
    #[doc = "Gets or sets list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<GitLabProject>,
    #[doc = "Gets or sets next link to scroll over the results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GitLabProjectListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GitLabProjectListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitLab Project properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitLabProjectProperties {
    #[doc = "Gets or sets resource status message."]
    #[serde(rename = "provisioningStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_status_message: Option<String>,
    #[doc = "Gets or sets time when resource was last checked."]
    #[serde(rename = "provisioningStatusUpdateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_status_update_time_utc: Option<::time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource.\r\n\r\nPending - Provisioning pending.\r\nFailed - Provisioning failed.\r\nSucceeded - Successful provisioning.\r\nCanceled - Provisioning canceled.\r\nPendingDeletion - Deletion pending.\r\nDeletionSuccess - Deletion successful.\r\nDeletionFailure - Deletion failure."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<DevOpsProvisioningState>,
    #[doc = "Gets or sets the fully-qualified name of the project object.\r\n\r\nThis contains the entire hierarchy where entities are separated by the '$' character."]
    #[serde(rename = "fullyQualifiedName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_name: Option<String>,
    #[doc = "Gets or sets the human readable fully-qualified name of the Project object.\r\n\r\nThis contains the entire namespace hierarchy as seen on GitLab UI where entities are separated by the '/' character."]
    #[serde(rename = "fullyQualifiedFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_friendly_name: Option<String>,
    #[doc = "Gets or sets the fully-qualified name of the project's parent group object.\r\n\r\nThis contains the entire hierarchy where namespaces are separated by the '$' character."]
    #[serde(rename = "fullyQualifiedParentGroupName", default, skip_serializing_if = "Option::is_none")]
    pub fully_qualified_parent_group_name: Option<String>,
    #[doc = "Gets or sets the url of the GitLab Project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
    #[serde(rename = "onboardingState", default, skip_serializing_if = "Option::is_none")]
    pub onboarding_state: Option<OnboardingState>,
}
impl GitLabProjectProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update Settings.\r\n\r\nEnabled - Resource should inherit configurations from parent.\r\nDisabled - Resource should not inherit configurations from parent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InheritFromParentState")]
pub enum InheritFromParentState {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InheritFromParentState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InheritFromParentState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InheritFromParentState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("InheritFromParentState", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("InheritFromParentState", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Details about resource onboarding status across all connectors.\r\n\r\nOnboardedByOtherConnector - this resource has already been onboarded to another connector. This is only applicable to top-level resources.\r\nOnboarded - this resource has already been onboarded by the specified connector.\r\nNotOnboarded - this resource has not been onboarded to any connector.\r\nNotApplicable - the onboarding state is not applicable to the current endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OnboardingState")]
pub enum OnboardingState {
    NotApplicable,
    OnboardedByOtherConnector,
    Onboarded,
    NotOnboarded,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OnboardingState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OnboardingState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OnboardingState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotApplicable => serializer.serialize_unit_variant("OnboardingState", 0u32, "NotApplicable"),
            Self::OnboardedByOtherConnector => serializer.serialize_unit_variant("OnboardingState", 1u32, "OnboardedByOtherConnector"),
            Self::Onboarded => serializer.serialize_unit_variant("OnboardingState", 2u32, "Onboarded"),
            Self::NotOnboarded => serializer.serialize_unit_variant("OnboardingState", 3u32, "NotOnboarded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The current status of an async operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusResult {
    #[doc = "Fully qualified ID for the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Operation status."]
    pub status: String,
    #[doc = "Percent of the operation that is complete."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
    #[doc = "The operations list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operations: Vec<OperationStatusResult>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl OperationStatusResult {
    pub fn new(status: String) -> Self {
        Self {
            id: None,
            name: None,
            status,
            percent_complete: None,
            start_time: None,
            end_time: None,
            operations: Vec::new(),
            error: None,
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
#[doc = "Rule categories.\r\nCode - code scanning results.\r\nArtifact scanning results.\r\nDependencies scanning results.\r\nIaC results.\r\nSecrets scanning results.\r\nContainer scanning results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RuleCategory")]
pub enum RuleCategory {
    Code,
    Artifacts,
    Dependencies,
    Secrets,
    IaC,
    Containers,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RuleCategory {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RuleCategory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RuleCategory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Code => serializer.serialize_unit_variant("RuleCategory", 0u32, "Code"),
            Self::Artifacts => serializer.serialize_unit_variant("RuleCategory", 1u32, "Artifacts"),
            Self::Dependencies => serializer.serialize_unit_variant("RuleCategory", 2u32, "Dependencies"),
            Self::Secrets => serializer.serialize_unit_variant("RuleCategory", 3u32, "Secrets"),
            Self::IaC => serializer.serialize_unit_variant("RuleCategory", 4u32, "IaC"),
            Self::Containers => serializer.serialize_unit_variant("RuleCategory", 5u32, "Containers"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Repository branch configuration for PR Annotations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetBranchConfiguration {
    #[doc = "Gets or sets branches that should have annotations."]
    #[serde(
        rename = "branchNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub branch_names: Vec<String>,
    #[doc = "Configuration of PR Annotations on default branch.\r\n\r\nEnabled - PR Annotations are enabled on the resource's default branch.\r\nDisabled - PR Annotations are disabled on the resource's default branch."]
    #[serde(rename = "annotateDefaultBranch", default, skip_serializing_if = "Option::is_none")]
    pub annotate_default_branch: Option<AnnotateDefaultBranchState>,
}
impl TargetBranchConfiguration {
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
