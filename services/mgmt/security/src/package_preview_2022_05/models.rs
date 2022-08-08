#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The aws connector environment data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsEnvironmentData {
    #[serde(flatten)]
    pub environment_data: EnvironmentData,
    #[doc = "The awsOrganization data "]
    #[serde(rename = "organizationalData", default, skip_serializing_if = "Option::is_none")]
    pub organizational_data: Option<AwsOrganizationalData>,
}
impl AwsEnvironmentData {
    pub fn new(environment_data: EnvironmentData) -> Self {
        Self {
            environment_data,
            organizational_data: None,
        }
    }
}
#[doc = "The awsOrganization data "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsOrganizationalData {
    #[doc = "The multi cloud account's membership type in the organization"]
    #[serde(rename = "organizationMembershipType")]
    pub organization_membership_type: aws_organizational_data::OrganizationMembershipType,
}
impl AwsOrganizationalData {
    pub fn new(organization_membership_type: aws_organizational_data::OrganizationMembershipType) -> Self {
        Self {
            organization_membership_type,
        }
    }
}
pub mod aws_organizational_data {
    use super::*;
    #[doc = "The multi cloud account's membership type in the organization"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OrganizationMembershipType")]
    pub enum OrganizationMembershipType {
        Member,
        Organization,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OrganizationMembershipType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OrganizationMembershipType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OrganizationMembershipType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Member => serializer.serialize_unit_variant("OrganizationMembershipType", 0u32, "Member"),
                Self::Organization => serializer.serialize_unit_variant("OrganizationMembershipType", 1u32, "Organization"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The awsOrganization data for the master account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsOrganizationalDataMaster {
    #[serde(flatten)]
    pub aws_organizational_data: AwsOrganizationalData,
    #[doc = "If the multi cloud account is of membership type organization, this will be the name of the onboarding stackset"]
    #[serde(rename = "stacksetName", default, skip_serializing_if = "Option::is_none")]
    pub stackset_name: Option<String>,
    #[doc = "If the multi cloud account is of membership type organization, list of accounts excluded from offering"]
    #[serde(rename = "excludedAccountIds", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_account_ids: Vec<String>,
}
impl AwsOrganizationalDataMaster {
    pub fn new(aws_organizational_data: AwsOrganizationalData) -> Self {
        Self {
            aws_organizational_data,
            stackset_name: None,
            excluded_account_ids: Vec::new(),
        }
    }
}
#[doc = "The awsOrganization data for the member account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsOrganizationalDataMember {
    #[serde(flatten)]
    pub aws_organizational_data: AwsOrganizationalData,
    #[doc = "If the multi cloud account is not of membership type organization, this will be the ID of the account's parent"]
    #[serde(rename = "parentHierarchyId", default, skip_serializing_if = "Option::is_none")]
    pub parent_hierarchy_id: Option<String>,
}
impl AwsOrganizationalDataMember {
    pub fn new(aws_organizational_data: AwsOrganizationalData) -> Self {
        Self {
            aws_organizational_data,
            parent_hierarchy_id: None,
        }
    }
}
#[doc = "The AzureDevOps scope connector's environment data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureDevOpsScopeEnvironmentData {
    #[serde(flatten)]
    pub environment_data: EnvironmentData,
}
impl AzureDevOpsScopeEnvironmentData {
    pub fn new(environment_data: EnvironmentData) -> Self {
        Self { environment_data }
    }
}
#[doc = "Describes an Azure resource with location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureTrackedResourceLocation {
    #[doc = "Location where the resource is stored"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl AzureTrackedResourceLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl azure_core::Continuable for CloudError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
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
    pub details: Vec<CloudErrorBody>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Entity tag is used for comparing two or more entities from the same requested resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ETag {
    #[doc = "Entity tag is used for comparing two or more entities from the same requested resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ETag {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The security connector environment data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentData {
    #[doc = "The type of the environment data."]
    #[serde(rename = "environmentType")]
    pub environment_type: environment_data::EnvironmentType,
}
impl EnvironmentData {
    pub fn new(environment_type: environment_data::EnvironmentType) -> Self {
        Self { environment_type }
    }
}
pub mod environment_data {
    use super::*;
    #[doc = "The type of the environment data."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnvironmentType")]
    pub enum EnvironmentType {
        AwsAccount,
        GcpProject,
        GithubScope,
        AzureDevOpsScope,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnvironmentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnvironmentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnvironmentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AwsAccount => serializer.serialize_unit_variant("EnvironmentType", 0u32, "AwsAccount"),
                Self::GcpProject => serializer.serialize_unit_variant("EnvironmentType", 1u32, "GcpProject"),
                Self::GithubScope => serializer.serialize_unit_variant("EnvironmentType", 2u32, "GithubScope"),
                Self::AzureDevOpsScope => serializer.serialize_unit_variant("EnvironmentType", 3u32, "AzureDevOpsScope"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "The gcpOrganization data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcpOrganizationalData {
    #[doc = "The multi cloud account's membership type in the organization"]
    #[serde(rename = "organizationMembershipType")]
    pub organization_membership_type: gcp_organizational_data::OrganizationMembershipType,
}
impl GcpOrganizationalData {
    pub fn new(organization_membership_type: gcp_organizational_data::OrganizationMembershipType) -> Self {
        Self {
            organization_membership_type,
        }
    }
}
pub mod gcp_organizational_data {
    use super::*;
    #[doc = "The multi cloud account's membership type in the organization"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OrganizationMembershipType")]
    pub enum OrganizationMembershipType {
        Member,
        Organization,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OrganizationMembershipType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OrganizationMembershipType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OrganizationMembershipType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Member => serializer.serialize_unit_variant("OrganizationMembershipType", 0u32, "Member"),
                Self::Organization => serializer.serialize_unit_variant("OrganizationMembershipType", 1u32, "Organization"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The gcpOrganization data for the member account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcpOrganizationalDataMember {
    #[serde(flatten)]
    pub gcp_organizational_data: GcpOrganizationalData,
    #[doc = "If the multi cloud account is not of membership type organization, this will be the ID of the project's parent"]
    #[serde(rename = "parentHierarchyId", default, skip_serializing_if = "Option::is_none")]
    pub parent_hierarchy_id: Option<String>,
    #[doc = "The GCP management project number from organizational onboarding"]
    #[serde(rename = "managementProjectNumber", default, skip_serializing_if = "Option::is_none")]
    pub management_project_number: Option<String>,
}
impl GcpOrganizationalDataMember {
    pub fn new(gcp_organizational_data: GcpOrganizationalData) -> Self {
        Self {
            gcp_organizational_data,
            parent_hierarchy_id: None,
            management_project_number: None,
        }
    }
}
#[doc = "The gcpOrganization data for the parent account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcpOrganizationalDataOrganization {
    #[serde(flatten)]
    pub gcp_organizational_data: GcpOrganizationalData,
    #[doc = "If the multi cloud account is of membership type organization, list of accounts excluded from offering"]
    #[serde(rename = "excludedProjectNumbers", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_project_numbers: Vec<String>,
    #[doc = "The service account email address which represents the organization level permissions container."]
    #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
    pub service_account_email_address: Option<String>,
    #[doc = "The GCP workload identity provider id which represents the permissions required to auto provision security connectors"]
    #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
    pub workload_identity_provider_id: Option<String>,
}
impl GcpOrganizationalDataOrganization {
    pub fn new(gcp_organizational_data: GcpOrganizationalData) -> Self {
        Self {
            gcp_organizational_data,
            excluded_project_numbers: Vec::new(),
            service_account_email_address: None,
            workload_identity_provider_id: None,
        }
    }
}
#[doc = "The details about the project represented by the security connector"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GcpProjectDetails {
    #[doc = "The unique GCP Project number"]
    #[serde(rename = "projectNumber", default, skip_serializing_if = "Option::is_none")]
    pub project_number: Option<String>,
    #[doc = "The GCP Project id"]
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[doc = "The GCP workload identity federation pool id"]
    #[serde(rename = "workloadIdentityPoolId", default, skip_serializing_if = "Option::is_none")]
    pub workload_identity_pool_id: Option<String>,
}
impl GcpProjectDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The GCP project connector environment data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GcpProjectEnvironmentData {
    #[serde(flatten)]
    pub environment_data: EnvironmentData,
    #[doc = "The gcpOrganization data"]
    #[serde(rename = "organizationalData", default, skip_serializing_if = "Option::is_none")]
    pub organizational_data: Option<GcpOrganizationalData>,
    #[doc = "The details about the project represented by the security connector"]
    #[serde(rename = "projectDetails", default, skip_serializing_if = "Option::is_none")]
    pub project_details: Option<GcpProjectDetails>,
}
impl GcpProjectEnvironmentData {
    pub fn new(environment_data: EnvironmentData) -> Self {
        Self {
            environment_data,
            organizational_data: None,
            project_details: None,
        }
    }
}
#[doc = "The github scope connector's environment data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GithubScopeEnvironmentData {
    #[serde(flatten)]
    pub environment_data: EnvironmentData,
}
impl GithubScopeEnvironmentData {
    pub fn new(environment_data: EnvironmentData) -> Self {
        Self { environment_data }
    }
}
#[doc = "Describes an Azure resource with kind"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Kind {
    #[doc = "Kind of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl Kind {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The security connector resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityConnector {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "A set of properties that defines the security connector configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityConnectorProperties>,
}
impl SecurityConnector {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A set of properties that defines the security connector configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityConnectorProperties {
    #[doc = "The multi cloud resource identifier (account id in case of AWS connector, project number in case of GCP connector)."]
    #[serde(rename = "hierarchyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_identifier: Option<String>,
    #[doc = "The date on which the trial period will end, if applicable. Trial period exists for 30 days after upgrading to payed offerings."]
    #[serde(rename = "hierarchyIdentifierTrialEndDate", with = "azure_core::date::rfc3339::option")]
    pub hierarchy_identifier_trial_end_date: Option<time::OffsetDateTime>,
    #[doc = "The multi cloud resource's cloud name."]
    #[serde(rename = "environmentName", default, skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<security_connector_properties::EnvironmentName>,
    #[doc = "A collection of offerings for the security connector."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub offerings: Vec<CloudOffering>,
    #[doc = "The security connector environment data."]
    #[serde(rename = "environmentData", default, skip_serializing_if = "Option::is_none")]
    pub environment_data: Option<EnvironmentData>,
}
impl SecurityConnectorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod security_connector_properties {
    use super::*;
    #[doc = "The multi cloud resource's cloud name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnvironmentName")]
    pub enum EnvironmentName {
        Azure,
        #[serde(rename = "AWS")]
        Aws,
        #[serde(rename = "GCP")]
        Gcp,
        Github,
        AzureDevOps,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnvironmentName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnvironmentName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnvironmentName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Azure => serializer.serialize_unit_variant("EnvironmentName", 0u32, "Azure"),
                Self::Aws => serializer.serialize_unit_variant("EnvironmentName", 1u32, "AWS"),
                Self::Gcp => serializer.serialize_unit_variant("EnvironmentName", 2u32, "GCP"),
                Self::Github => serializer.serialize_unit_variant("EnvironmentName", 3u32, "Github"),
                Self::AzureDevOps => serializer.serialize_unit_variant("EnvironmentName", 4u32, "AzureDevOps"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of security connectors response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityConnectorsList {
    #[doc = "The list of security connectors under the given scope."]
    pub value: Vec<SecurityConnector>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecurityConnectorsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SecurityConnectorsList {
    pub fn new(value: Vec<SecurityConnector>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A list of key value pairs that describe the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {
    #[doc = "A list of key value pairs that describe the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure tracked resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub azure_tracked_resource_location: AzureTrackedResourceLocation,
    #[serde(flatten)]
    pub kind: Kind,
    #[serde(flatten)]
    pub e_tag: ETag,
    #[serde(flatten)]
    pub tags: Tags,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The security offering details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudOffering {
    #[doc = "The type of the security offering."]
    #[serde(rename = "offeringType")]
    pub offering_type: cloud_offering::OfferingType,
    #[doc = "The offering description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl CloudOffering {
    pub fn new(offering_type: cloud_offering::OfferingType) -> Self {
        Self {
            offering_type,
            description: None,
        }
    }
}
pub mod cloud_offering {
    use super::*;
    #[doc = "The type of the security offering."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OfferingType")]
    pub enum OfferingType {
        CspmMonitorAws,
        DefenderForContainersAws,
        DefenderForServersAws,
        DefenderForDatabasesAws,
        InformationProtectionAws,
        CspmMonitorGcp,
        CspmMonitorGithub,
        CspmMonitorAzureDevOps,
        DefenderForServersGcp,
        DefenderForContainersGcp,
        DefenderForDatabasesGcp,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OfferingType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OfferingType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OfferingType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CspmMonitorAws => serializer.serialize_unit_variant("OfferingType", 0u32, "CspmMonitorAws"),
                Self::DefenderForContainersAws => serializer.serialize_unit_variant("OfferingType", 1u32, "DefenderForContainersAws"),
                Self::DefenderForServersAws => serializer.serialize_unit_variant("OfferingType", 2u32, "DefenderForServersAws"),
                Self::DefenderForDatabasesAws => serializer.serialize_unit_variant("OfferingType", 3u32, "DefenderForDatabasesAws"),
                Self::InformationProtectionAws => serializer.serialize_unit_variant("OfferingType", 4u32, "InformationProtectionAws"),
                Self::CspmMonitorGcp => serializer.serialize_unit_variant("OfferingType", 5u32, "CspmMonitorGcp"),
                Self::CspmMonitorGithub => serializer.serialize_unit_variant("OfferingType", 6u32, "CspmMonitorGithub"),
                Self::CspmMonitorAzureDevOps => serializer.serialize_unit_variant("OfferingType", 7u32, "CspmMonitorAzureDevOps"),
                Self::DefenderForServersGcp => serializer.serialize_unit_variant("OfferingType", 8u32, "DefenderForServersGcp"),
                Self::DefenderForContainersGcp => serializer.serialize_unit_variant("OfferingType", 9u32, "DefenderForContainersGcp"),
                Self::DefenderForDatabasesGcp => serializer.serialize_unit_variant("OfferingType", 10u32, "DefenderForDatabasesGcp"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The CSPM monitoring for AWS offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CspmMonitorAwsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The native cloud connection configuration"]
    #[serde(rename = "nativeCloudConnection", default, skip_serializing_if = "Option::is_none")]
    pub native_cloud_connection: Option<cspm_monitor_aws_offering::NativeCloudConnection>,
}
impl CspmMonitorAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            native_cloud_connection: None,
        }
    }
}
pub mod cspm_monitor_aws_offering {
    use super::*;
    #[doc = "The native cloud connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct NativeCloudConnection {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl NativeCloudConnection {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The CSPM monitoring for AzureDevOps offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CspmMonitorAzureDevOpsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
}
impl CspmMonitorAzureDevOpsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self { cloud_offering }
    }
}
#[doc = "The CSPM monitoring for GCP offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CspmMonitorGcpOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The native cloud connection configuration"]
    #[serde(rename = "nativeCloudConnection", default, skip_serializing_if = "Option::is_none")]
    pub native_cloud_connection: Option<cspm_monitor_gcp_offering::NativeCloudConnection>,
}
impl CspmMonitorGcpOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            native_cloud_connection: None,
        }
    }
}
pub mod cspm_monitor_gcp_offering {
    use super::*;
    #[doc = "The native cloud connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct NativeCloudConnection {
        #[doc = "The GCP workload identity provider id for the offering"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
        #[doc = "The service account email address in GCP for this offering"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
    }
    impl NativeCloudConnection {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The CSPM monitoring for github offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CspmMonitorGithubOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
}
impl CspmMonitorGithubOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self { cloud_offering }
    }
}
#[doc = "The Defender for Databases AWS offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderFoDatabasesAwsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The ARC autoprovisioning configuration"]
    #[serde(rename = "arcAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub arc_auto_provisioning: Option<defender_fo_databases_aws_offering::ArcAutoProvisioning>,
}
impl DefenderFoDatabasesAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            arc_auto_provisioning: None,
        }
    }
}
pub mod defender_fo_databases_aws_offering {
    use super::*;
    #[doc = "The ARC autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ArcAutoProvisioning {
        #[doc = "Is arc auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
        #[doc = "Metadata of Service Principal secret for autoprovisioning"]
        #[serde(rename = "servicePrincipalSecretMetadata", default, skip_serializing_if = "Option::is_none")]
        pub service_principal_secret_metadata: Option<arc_auto_provisioning::ServicePrincipalSecretMetadata>,
    }
    impl ArcAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod arc_auto_provisioning {
        use super::*;
        #[doc = "Metadata of Service Principal secret for autoprovisioning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct ServicePrincipalSecretMetadata {
            #[doc = "expiration date of service principal secret"]
            #[serde(rename = "expiryDate", with = "azure_core::date::rfc3339::option")]
            pub expiry_date: Option<time::OffsetDateTime>,
            #[doc = "region of parameter store where secret is kept"]
            #[serde(rename = "parameterStoreRegion", default, skip_serializing_if = "Option::is_none")]
            pub parameter_store_region: Option<String>,
            #[doc = "name of secret resource in parameter store"]
            #[serde(rename = "parameterNameInStore", default, skip_serializing_if = "Option::is_none")]
            pub parameter_name_in_store: Option<String>,
        }
        impl ServicePrincipalSecretMetadata {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[doc = "The Defender for Containers AWS offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderForContainersAwsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The kubernetes service connection configuration"]
    #[serde(rename = "kubernetesService", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_service: Option<defender_for_containers_aws_offering::KubernetesService>,
    #[doc = "The kubernetes to scuba connection configuration"]
    #[serde(rename = "kubernetesScubaReader", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_scuba_reader: Option<defender_for_containers_aws_offering::KubernetesScubaReader>,
    #[doc = "The cloudwatch to kinesis connection configuration"]
    #[serde(rename = "cloudWatchToKinesis", default, skip_serializing_if = "Option::is_none")]
    pub cloud_watch_to_kinesis: Option<defender_for_containers_aws_offering::CloudWatchToKinesis>,
    #[doc = "The kinesis to s3 connection configuration"]
    #[serde(rename = "kinesisToS3", default, skip_serializing_if = "Option::is_none")]
    pub kinesis_to_s3: Option<defender_for_containers_aws_offering::KinesisToS3>,
    #[doc = "The container vulnerability assessment configuration"]
    #[serde(rename = "containerVulnerabilityAssessment", default, skip_serializing_if = "Option::is_none")]
    pub container_vulnerability_assessment: Option<defender_for_containers_aws_offering::ContainerVulnerabilityAssessment>,
    #[doc = "The container vulnerability assessment task configuration"]
    #[serde(rename = "containerVulnerabilityAssessmentTask", default, skip_serializing_if = "Option::is_none")]
    pub container_vulnerability_assessment_task: Option<defender_for_containers_aws_offering::ContainerVulnerabilityAssessmentTask>,
    #[doc = "Enable container vulnerability assessment feature"]
    #[serde(
        rename = "enableContainerVulnerabilityAssessment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_container_vulnerability_assessment: Option<bool>,
    #[doc = "Is audit logs pipeline auto provisioning enabled"]
    #[serde(rename = "autoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub auto_provisioning: Option<bool>,
    #[doc = "The retention time in days of kube audit logs set on the CloudWatch log group"]
    #[serde(rename = "kubeAuditRetentionTime", default, skip_serializing_if = "Option::is_none")]
    pub kube_audit_retention_time: Option<i64>,
    #[doc = "The externalId used by the data reader to prevent the confused deputy attack"]
    #[serde(rename = "scubaExternalId", default, skip_serializing_if = "Option::is_none")]
    pub scuba_external_id: Option<String>,
}
impl DefenderForContainersAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            kubernetes_service: None,
            kubernetes_scuba_reader: None,
            cloud_watch_to_kinesis: None,
            kinesis_to_s3: None,
            container_vulnerability_assessment: None,
            container_vulnerability_assessment_task: None,
            enable_container_vulnerability_assessment: None,
            auto_provisioning: None,
            kube_audit_retention_time: None,
            scuba_external_id: None,
        }
    }
}
pub mod defender_for_containers_aws_offering {
    use super::*;
    #[doc = "The kubernetes service connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct KubernetesService {
        #[doc = "The cloud role ARN in AWS for this feature used for provisioning resources"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl KubernetesService {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The kubernetes to scuba connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct KubernetesScubaReader {
        #[doc = "The cloud role ARN in AWS for this feature used for reading data"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl KubernetesScubaReader {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The cloudwatch to kinesis connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct CloudWatchToKinesis {
        #[doc = "The cloud role ARN in AWS used by CloudWatch to transfer data into Kinesis"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl CloudWatchToKinesis {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The kinesis to s3 connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct KinesisToS3 {
        #[doc = "The cloud role ARN in AWS used by Kinesis to transfer data into S3"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl KinesisToS3 {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The container vulnerability assessment configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ContainerVulnerabilityAssessment {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl ContainerVulnerabilityAssessment {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The container vulnerability assessment task configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ContainerVulnerabilityAssessmentTask {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl ContainerVulnerabilityAssessmentTask {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The containers GCP offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderForContainersGcpOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The native cloud connection configuration"]
    #[serde(rename = "nativeCloudConnection", default, skip_serializing_if = "Option::is_none")]
    pub native_cloud_connection: Option<defender_for_containers_gcp_offering::NativeCloudConnection>,
    #[doc = "The native cloud connection configuration"]
    #[serde(rename = "dataPipelineNativeCloudConnection", default, skip_serializing_if = "Option::is_none")]
    pub data_pipeline_native_cloud_connection: Option<defender_for_containers_gcp_offering::DataPipelineNativeCloudConnection>,
    #[doc = "Is audit logs data collection enabled"]
    #[serde(rename = "auditLogsAutoProvisioningFlag", default, skip_serializing_if = "Option::is_none")]
    pub audit_logs_auto_provisioning_flag: Option<bool>,
    #[doc = "Is Microsoft Defender for Cloud Kubernetes agent auto provisioning enabled"]
    #[serde(rename = "defenderAgentAutoProvisioningFlag", default, skip_serializing_if = "Option::is_none")]
    pub defender_agent_auto_provisioning_flag: Option<bool>,
    #[doc = "Is Policy Kubernetes agent auto provisioning enabled"]
    #[serde(rename = "policyAgentAutoProvisioningFlag", default, skip_serializing_if = "Option::is_none")]
    pub policy_agent_auto_provisioning_flag: Option<bool>,
}
impl DefenderForContainersGcpOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            native_cloud_connection: None,
            data_pipeline_native_cloud_connection: None,
            audit_logs_auto_provisioning_flag: None,
            defender_agent_auto_provisioning_flag: None,
            policy_agent_auto_provisioning_flag: None,
        }
    }
}
pub mod defender_for_containers_gcp_offering {
    use super::*;
    #[doc = "The native cloud connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct NativeCloudConnection {
        #[doc = "The service account email address in GCP for this offering"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
        #[doc = "The GCP workload identity provider id for this offering"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
    }
    impl NativeCloudConnection {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The native cloud connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DataPipelineNativeCloudConnection {
        #[doc = "The data collection service account email address in GCP for this offering"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
        #[doc = "The data collection GCP workload identity provider id for this offering"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
    }
    impl DataPipelineNativeCloudConnection {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The Defender for Databases GCP offering configurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderForDatabasesGcpOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The ARC autoprovisioning configuration"]
    #[serde(rename = "arcAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub arc_auto_provisioning: Option<defender_for_databases_gcp_offering::ArcAutoProvisioning>,
    #[doc = "The native cloud connection configuration"]
    #[serde(
        rename = "defenderForDatabasesArcAutoProvisioning",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub defender_for_databases_arc_auto_provisioning: Option<defender_for_databases_gcp_offering::DefenderForDatabasesArcAutoProvisioning>,
}
impl DefenderForDatabasesGcpOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            arc_auto_provisioning: None,
            defender_for_databases_arc_auto_provisioning: None,
        }
    }
}
pub mod defender_for_databases_gcp_offering {
    use super::*;
    #[doc = "The ARC autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ArcAutoProvisioning {
        #[doc = "Is arc auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "Configuration for ARC autoprovisioning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<arc_auto_provisioning::Configuration>,
    }
    impl ArcAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod arc_auto_provisioning {
        use super::*;
        #[doc = "Configuration for ARC autoprovisioning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Configuration {
            #[doc = "The Azure service principal client id for agent onboarding"]
            #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
            pub client_id: Option<String>,
            #[doc = "The agent onboarding service account numeric id"]
            #[serde(
                rename = "agentOnboardingServiceAccountNumericId",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub agent_onboarding_service_account_numeric_id: Option<String>,
        }
        impl Configuration {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
    #[doc = "The native cloud connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DefenderForDatabasesArcAutoProvisioning {
        #[doc = "The service account email address in GCP for this offering"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
        #[doc = "The GCP workload identity provider id for this offering"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
    }
    impl DefenderForDatabasesArcAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The Defender for Servers AWS offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderForServersAwsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The Defender for servers connection configuration"]
    #[serde(rename = "defenderForServers", default, skip_serializing_if = "Option::is_none")]
    pub defender_for_servers: Option<defender_for_servers_aws_offering::DefenderForServers>,
    #[doc = "The ARC autoprovisioning configuration"]
    #[serde(rename = "arcAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub arc_auto_provisioning: Option<defender_for_servers_aws_offering::ArcAutoProvisioning>,
    #[doc = "The Vulnerability Assessment autoprovisioning configuration"]
    #[serde(rename = "vaAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub va_auto_provisioning: Option<defender_for_servers_aws_offering::VaAutoProvisioning>,
    #[doc = "The Microsoft Defender for Endpoint autoprovisioning configuration"]
    #[serde(rename = "mdeAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub mde_auto_provisioning: Option<defender_for_servers_aws_offering::MdeAutoProvisioning>,
    #[doc = "configuration for the servers offering subPlan"]
    #[serde(rename = "subPlan", default, skip_serializing_if = "Option::is_none")]
    pub sub_plan: Option<defender_for_servers_aws_offering::SubPlan>,
    #[doc = "The Microsoft Defender for Server VM scanning configuration"]
    #[serde(rename = "vmScanners", default, skip_serializing_if = "Option::is_none")]
    pub vm_scanners: Option<defender_for_servers_aws_offering::VmScanners>,
}
impl DefenderForServersAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            defender_for_servers: None,
            arc_auto_provisioning: None,
            va_auto_provisioning: None,
            mde_auto_provisioning: None,
            sub_plan: None,
            vm_scanners: None,
        }
    }
}
pub mod defender_for_servers_aws_offering {
    use super::*;
    #[doc = "The Defender for servers connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DefenderForServers {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl DefenderForServers {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The ARC autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ArcAutoProvisioning {
        #[doc = "Is arc auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
        #[doc = "Metadata of Service Principal secret for autoprovisioning"]
        #[serde(rename = "servicePrincipalSecretMetadata", default, skip_serializing_if = "Option::is_none")]
        pub service_principal_secret_metadata: Option<arc_auto_provisioning::ServicePrincipalSecretMetadata>,
    }
    impl ArcAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod arc_auto_provisioning {
        use super::*;
        #[doc = "Metadata of Service Principal secret for autoprovisioning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct ServicePrincipalSecretMetadata {
            #[doc = "expiration date of service principal secret"]
            #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
            pub expiry_date: Option<String>,
            #[doc = "region of parameter store where secret is kept"]
            #[serde(rename = "parameterStoreRegion", default, skip_serializing_if = "Option::is_none")]
            pub parameter_store_region: Option<String>,
            #[doc = "name of secret resource in parameter store"]
            #[serde(rename = "parameterNameInStore", default, skip_serializing_if = "Option::is_none")]
            pub parameter_name_in_store: Option<String>,
        }
        impl ServicePrincipalSecretMetadata {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
    #[doc = "The Vulnerability Assessment autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct VaAutoProvisioning {
        #[doc = "Is Vulnerability Assessment auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "configuration for Vulnerability Assessment autoprovisioning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<va_auto_provisioning::Configuration>,
    }
    impl VaAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod va_auto_provisioning {
        use super::*;
        #[doc = "configuration for Vulnerability Assessment autoprovisioning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Configuration {
            #[doc = "The Vulnerability Assessment solution to be provisioned. Can be either 'TVM' or 'Qualys'"]
            #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
            pub type_: Option<configuration::Type>,
        }
        impl Configuration {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod configuration {
            use super::*;
            #[doc = "The Vulnerability Assessment solution to be provisioned. Can be either 'TVM' or 'Qualys'"]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(remote = "Type")]
            pub enum Type {
                Qualys,
                #[serde(rename = "TVM")]
                Tvm,
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
                        Self::Qualys => serializer.serialize_unit_variant("Type", 0u32, "Qualys"),
                        Self::Tvm => serializer.serialize_unit_variant("Type", 1u32, "TVM"),
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
    }
    #[doc = "The Microsoft Defender for Endpoint autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct MdeAutoProvisioning {
        #[doc = "Is Microsoft Defender for Endpoint auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "configuration for Microsoft Defender for Endpoint autoprovisioning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<serde_json::Value>,
    }
    impl MdeAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "configuration for the servers offering subPlan"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct SubPlan {
        #[doc = "The available sub plans"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<sub_plan::Type>,
    }
    impl SubPlan {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod sub_plan {
        use super::*;
        #[doc = "The available sub plans"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Type")]
        pub enum Type {
            P1,
            P2,
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
                    Self::P1 => serializer.serialize_unit_variant("Type", 0u32, "P1"),
                    Self::P2 => serializer.serialize_unit_variant("Type", 1u32, "P2"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
    #[doc = "The Microsoft Defender for Server VM scanning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct VmScanners {
        #[doc = "Is Microsoft Defender for Server VM scanning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "configuration for Microsoft Defender for Server VM scanning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<vm_scanners::Configuration>,
    }
    impl VmScanners {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod vm_scanners {
        use super::*;
        #[doc = "configuration for Microsoft Defender for Server VM scanning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Configuration {
            #[doc = "The cloud role ARN in AWS for this feature"]
            #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
            pub cloud_role_arn: Option<String>,
            #[doc = "The scanning mode for the vm scan."]
            #[serde(rename = "scanningMode", default, skip_serializing_if = "Option::is_none")]
            pub scanning_mode: Option<configuration::ScanningMode>,
            #[doc = "VM tags that indicates that VM should not be scanned"]
            #[serde(rename = "exclusionTags", default, skip_serializing_if = "Option::is_none")]
            pub exclusion_tags: Option<serde_json::Value>,
        }
        impl Configuration {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod configuration {
            use super::*;
            #[doc = "The scanning mode for the vm scan."]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(remote = "ScanningMode")]
            pub enum ScanningMode {
                Default,
                #[serde(skip_deserializing)]
                UnknownValue(String),
            }
            impl FromStr for ScanningMode {
                type Err = value::Error;
                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Self::deserialize(s.into_deserializer())
                }
            }
            impl<'de> Deserialize<'de> for ScanningMode {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let s = String::deserialize(deserializer)?;
                    let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                    Ok(deserialized)
                }
            }
            impl Serialize for ScanningMode {
                fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    match self {
                        Self::Default => serializer.serialize_unit_variant("ScanningMode", 0u32, "Default"),
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
    }
}
#[doc = "The Defender for Servers GCP offering configurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefenderForServersGcpOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The Defender for servers connection configuration"]
    #[serde(rename = "defenderForServers", default, skip_serializing_if = "Option::is_none")]
    pub defender_for_servers: Option<defender_for_servers_gcp_offering::DefenderForServers>,
    #[doc = "The ARC autoprovisioning configuration"]
    #[serde(rename = "arcAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub arc_auto_provisioning: Option<defender_for_servers_gcp_offering::ArcAutoProvisioning>,
    #[doc = "The Vulnerability Assessment autoprovisioning configuration"]
    #[serde(rename = "vaAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub va_auto_provisioning: Option<defender_for_servers_gcp_offering::VaAutoProvisioning>,
    #[doc = "The Microsoft Defender for Endpoint autoprovisioning configuration"]
    #[serde(rename = "mdeAutoProvisioning", default, skip_serializing_if = "Option::is_none")]
    pub mde_auto_provisioning: Option<defender_for_servers_gcp_offering::MdeAutoProvisioning>,
    #[doc = "configuration for the servers offering subPlan"]
    #[serde(rename = "subPlan", default, skip_serializing_if = "Option::is_none")]
    pub sub_plan: Option<defender_for_servers_gcp_offering::SubPlan>,
}
impl DefenderForServersGcpOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            defender_for_servers: None,
            arc_auto_provisioning: None,
            va_auto_provisioning: None,
            mde_auto_provisioning: None,
            sub_plan: None,
        }
    }
}
pub mod defender_for_servers_gcp_offering {
    use super::*;
    #[doc = "The Defender for servers connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DefenderForServers {
        #[doc = "The workload identity provider id in GCP for this feature"]
        #[serde(rename = "workloadIdentityProviderId", default, skip_serializing_if = "Option::is_none")]
        pub workload_identity_provider_id: Option<String>,
        #[doc = "The service account email address in GCP for this feature"]
        #[serde(rename = "serviceAccountEmailAddress", default, skip_serializing_if = "Option::is_none")]
        pub service_account_email_address: Option<String>,
    }
    impl DefenderForServers {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The ARC autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ArcAutoProvisioning {
        #[doc = "Is arc auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "Configuration for ARC autoprovisioning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<arc_auto_provisioning::Configuration>,
    }
    impl ArcAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod arc_auto_provisioning {
        use super::*;
        #[doc = "Configuration for ARC autoprovisioning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Configuration {
            #[doc = "The Azure service principal client id for agent onboarding"]
            #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
            pub client_id: Option<String>,
            #[doc = "The agent onboarding service account numeric id"]
            #[serde(
                rename = "agentOnboardingServiceAccountNumericId",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub agent_onboarding_service_account_numeric_id: Option<String>,
        }
        impl Configuration {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
    #[doc = "The Vulnerability Assessment autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct VaAutoProvisioning {
        #[doc = "Is Vulnerability Assessment auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "configuration for Vulnerability Assessment autoprovisioning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<va_auto_provisioning::Configuration>,
    }
    impl VaAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod va_auto_provisioning {
        use super::*;
        #[doc = "configuration for Vulnerability Assessment autoprovisioning"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Configuration {
            #[doc = "The Vulnerability Assessment solution to be provisioned. Can be either 'TVM' or 'Qualys'"]
            #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
            pub type_: Option<configuration::Type>,
        }
        impl Configuration {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod configuration {
            use super::*;
            #[doc = "The Vulnerability Assessment solution to be provisioned. Can be either 'TVM' or 'Qualys'"]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(remote = "Type")]
            pub enum Type {
                Qualys,
                #[serde(rename = "TVM")]
                Tvm,
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
                        Self::Qualys => serializer.serialize_unit_variant("Type", 0u32, "Qualys"),
                        Self::Tvm => serializer.serialize_unit_variant("Type", 1u32, "TVM"),
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
    }
    #[doc = "The Microsoft Defender for Endpoint autoprovisioning configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct MdeAutoProvisioning {
        #[doc = "Is Microsoft Defender for Endpoint auto provisioning enabled"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "configuration for Microsoft Defender for Endpoint autoprovisioning"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<serde_json::Value>,
    }
    impl MdeAutoProvisioning {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "configuration for the servers offering subPlan"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct SubPlan {
        #[doc = "The available sub plans"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<sub_plan::Type>,
    }
    impl SubPlan {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod sub_plan {
        use super::*;
        #[doc = "The available sub plans"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Type")]
        pub enum Type {
            P1,
            P2,
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
                    Self::P1 => serializer.serialize_unit_variant("Type", 0u32, "P1"),
                    Self::P2 => serializer.serialize_unit_variant("Type", 1u32, "P2"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "The information protection for AWS offering"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InformationProtectionAwsOffering {
    #[serde(flatten)]
    pub cloud_offering: CloudOffering,
    #[doc = "The native cloud connection configuration"]
    #[serde(rename = "informationProtection", default, skip_serializing_if = "Option::is_none")]
    pub information_protection: Option<information_protection_aws_offering::InformationProtection>,
}
impl InformationProtectionAwsOffering {
    pub fn new(cloud_offering: CloudOffering) -> Self {
        Self {
            cloud_offering,
            information_protection: None,
        }
    }
}
pub mod information_protection_aws_offering {
    use super::*;
    #[doc = "The native cloud connection configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct InformationProtection {
        #[doc = "The cloud role ARN in AWS for this feature"]
        #[serde(rename = "cloudRoleArn", default, skip_serializing_if = "Option::is_none")]
        pub cloud_role_arn: Option<String>,
    }
    impl InformationProtection {
        pub fn new() -> Self {
            Self::default()
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
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
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
