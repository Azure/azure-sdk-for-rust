#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Schedules applicable to a virtual machine. The schedules may have been defined on a VM or on lab level."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicableSchedule {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a schedules applicable to a virtual machine."]
    pub properties: ApplicableScheduleProperties,
}
impl ApplicableSchedule {
    pub fn new(properties: ApplicableScheduleProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "Schedules applicable to a virtual machine. The schedules may have been defined on a VM or on lab level."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicableScheduleFragment {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl ApplicableScheduleFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a schedules applicable to a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicableScheduleProperties {
    #[doc = "A schedule."]
    #[serde(rename = "labVmsShutdown", default, skip_serializing_if = "Option::is_none")]
    pub lab_vms_shutdown: Option<Schedule>,
    #[doc = "A schedule."]
    #[serde(rename = "labVmsStartup", default, skip_serializing_if = "Option::is_none")]
    pub lab_vms_startup: Option<Schedule>,
}
impl ApplicableScheduleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a schedules applicable to a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicableSchedulePropertiesFragment {}
impl ApplicableSchedulePropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request body for applying artifacts to a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplyArtifactsRequest {
    #[doc = "The list of artifacts to apply."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ArtifactInstallProperties>,
}
impl ApplyArtifactsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Resource Manager template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmTemplate {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an Azure Resource Manager template."]
    pub properties: ArmTemplateProperties,
}
impl ArmTemplate {
    pub fn new(properties: ArmTemplateProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "Information about a generated ARM template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmTemplateInfo {
    #[doc = "The template's contents."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[doc = "The parameters of the ARM template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl ArmTemplateInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmTemplateList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ArmTemplate>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ArmTemplateList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ArmTemplateList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an Azure Resource Manager template parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmTemplateParameterProperties {
    #[doc = "The name of the template parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the template parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ArmTemplateParameterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an Azure Resource Manager template parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmTemplateParameterPropertiesFragment {}
impl ArmTemplateParameterPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an Azure Resource Manager template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmTemplateProperties {
    #[doc = "The display name of the ARM template."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the ARM template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The publisher of the ARM template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The URI to the icon of the ARM template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "The contents of the ARM template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<serde_json::Value>,
    #[doc = "The creation date of the armTemplate."]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "File name and parameter values information from all azuredeploy.*.parameters.json for the ARM template."]
    #[serde(rename = "parametersValueFilesInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub parameters_value_files_info: Vec<ParametersValueFileInfo>,
    #[doc = "Whether or not ARM template is enabled for use by lab user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ArmTemplateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an artifact."]
    pub properties: ArtifactProperties,
}
impl Artifact {
    pub fn new(properties: ArtifactProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "Properties of an artifact deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactDeploymentStatusProperties {
    #[doc = "The deployment status of the artifact."]
    #[serde(rename = "deploymentStatus", default, skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    #[doc = "The total count of the artifacts that were successfully applied."]
    #[serde(rename = "artifactsApplied", default, skip_serializing_if = "Option::is_none")]
    pub artifacts_applied: Option<i32>,
    #[doc = "The total count of the artifacts that were tentatively applied."]
    #[serde(rename = "totalArtifacts", default, skip_serializing_if = "Option::is_none")]
    pub total_artifacts: Option<i32>,
}
impl ArtifactDeploymentStatusProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an artifact deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactDeploymentStatusPropertiesFragment {}
impl ArtifactDeploymentStatusPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactInstallProperties {
    #[doc = "The artifact's identifier."]
    #[serde(rename = "artifactId", default, skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    #[doc = "The artifact's title."]
    #[serde(rename = "artifactTitle", default, skip_serializing_if = "Option::is_none")]
    pub artifact_title: Option<String>,
    #[doc = "The parameters of the artifact."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ArtifactParameterProperties>,
    #[doc = "The status of the artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The status message from the deployment."]
    #[serde(rename = "deploymentStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub deployment_status_message: Option<String>,
    #[doc = "The status message from the virtual machine extension."]
    #[serde(rename = "vmExtensionStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub vm_extension_status_message: Option<String>,
    #[doc = "The time that the artifact starts to install on the virtual machine."]
    #[serde(rename = "installTime", with = "azure_core::date::rfc3339::option")]
    pub install_time: Option<time::OffsetDateTime>,
}
impl ArtifactInstallProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactInstallPropertiesFragment {}
impl ArtifactInstallPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Artifact>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ArtifactList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ArtifactList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an artifact parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactParameterProperties {
    #[doc = "The name of the artifact parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the artifact parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ArtifactParameterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an artifact parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactParameterPropertiesFragment {}
impl ArtifactParameterPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactProperties {
    #[doc = "The artifact's title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The artifact's description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The artifact's publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The file path to the artifact."]
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[doc = "The URI to the artifact icon."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "The artifact's target OS."]
    #[serde(rename = "targetOsType", default, skip_serializing_if = "Option::is_none")]
    pub target_os_type: Option<String>,
    #[doc = "The artifact's parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "The artifact's creation date."]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
}
impl ArtifactProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an artifact source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactSource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an artifact source."]
    pub properties: ArtifactSourceProperties,
}
impl ArtifactSource {
    pub fn new(properties: ArtifactSourceProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "Properties of an artifact source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactSourceFragment {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl ArtifactSourceFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactSourceList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ArtifactSource>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ArtifactSourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ArtifactSourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an artifact source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactSourceProperties {
    #[doc = "The artifact source's display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The artifact source's URI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The artifact source's type."]
    #[serde(rename = "sourceType", default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<artifact_source_properties::SourceType>,
    #[doc = "The folder containing artifacts."]
    #[serde(rename = "folderPath", default, skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
    #[doc = "The folder containing Azure Resource Manager templates."]
    #[serde(rename = "armTemplateFolderPath", default, skip_serializing_if = "Option::is_none")]
    pub arm_template_folder_path: Option<String>,
    #[doc = "The artifact source's branch reference."]
    #[serde(rename = "branchRef", default, skip_serializing_if = "Option::is_none")]
    pub branch_ref: Option<String>,
    #[doc = "The security token to authenticate to the artifact source."]
    #[serde(rename = "securityToken", default, skip_serializing_if = "Option::is_none")]
    pub security_token: Option<String>,
    #[doc = "Indicates if the artifact source is enabled (values: Enabled, Disabled)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<artifact_source_properties::Status>,
    #[doc = "The artifact source's creation date."]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl ArtifactSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod artifact_source_properties {
    use super::*;
    #[doc = "The artifact source's type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceType")]
    pub enum SourceType {
        VsoGit,
        GitHub,
        StorageAccount,
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
                Self::GitHub => serializer.serialize_unit_variant("SourceType", 1u32, "GitHub"),
                Self::StorageAccount => serializer.serialize_unit_variant("SourceType", 2u32, "StorageAccount"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates if the artifact source is enabled (values: Enabled, Disabled)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of an artifact source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactSourcePropertiesFragment {}
impl ArtifactSourcePropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the disk to attach."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachDiskProperties {
    #[doc = "The resource ID of the Lab virtual machine to which the disk is attached."]
    #[serde(rename = "leasedByLabVmId", default, skip_serializing_if = "Option::is_none")]
    pub leased_by_lab_vm_id: Option<String>,
}
impl AttachDiskProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties to attach new disk to the Virtual Machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachNewDataDiskOptions {
    #[doc = "Size of the disk to be attached in Gibibytes."]
    #[serde(rename = "diskSizeGiB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gi_b: Option<i32>,
    #[doc = "The name of the disk to be attached."]
    #[serde(rename = "diskName", default, skip_serializing_if = "Option::is_none")]
    pub disk_name: Option<String>,
    #[doc = "The storage type for the disk (i.e. Standard, Premium)."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<attach_new_data_disk_options::DiskType>,
}
impl AttachNewDataDiskOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod attach_new_data_disk_options {
    use super::*;
    #[doc = "The storage type for the disk (i.e. Standard, Premium)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskType")]
    pub enum DiskType {
        Standard,
        Premium,
        #[serde(rename = "StandardSSD")]
        StandardSsd,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("DiskType", 0u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("DiskType", 1u32, "Premium"),
                Self::StandardSsd => serializer.serialize_unit_variant("DiskType", 2u32, "StandardSSD"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties to attach new disk to the Virtual Machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachNewDataDiskOptionsFragment {}
impl AttachNewDataDiskOptionsFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for creating multiple virtual machines as a single action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BulkCreationParameters {
    #[doc = "The number of virtual machine instances to create."]
    #[serde(rename = "instanceCount", default, skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
}
impl BulkCreationParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for creating multiple virtual machines as a single action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BulkCreationParametersFragment {}
impl BulkCreationParametersFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error from a REST request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Body of an error from a REST request."]
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
#[doc = "Body of an error from a REST request."]
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
    #[doc = "Inner errors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A data disks attached to a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeDataDisk {
    #[doc = "Gets data disk name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "When backed by a blob, the URI of underlying blob."]
    #[serde(rename = "diskUri", default, skip_serializing_if = "Option::is_none")]
    pub disk_uri: Option<String>,
    #[doc = "When backed by managed disk, this is the ID of the compute disk resource."]
    #[serde(rename = "managedDiskId", default, skip_serializing_if = "Option::is_none")]
    pub managed_disk_id: Option<String>,
    #[doc = "Gets data disk size in GiB."]
    #[serde(rename = "diskSizeGiB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gi_b: Option<i32>,
}
impl ComputeDataDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A data disks attached to a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeDataDiskFragment {}
impl ComputeDataDiskFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status information about a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeVmInstanceViewStatus {
    #[doc = "Gets the status Code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Gets the short localizable label for the status."]
    #[serde(rename = "displayStatus", default, skip_serializing_if = "Option::is_none")]
    pub display_status: Option<String>,
    #[doc = "Gets the message associated with the status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ComputeVmInstanceViewStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status information about a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeVmInstanceViewStatusFragment {}
impl ComputeVmInstanceViewStatusFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a virtual machine returned by the Microsoft.Compute API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeVmProperties {
    #[doc = "Gets the statuses of the virtual machine."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<ComputeVmInstanceViewStatus>,
    #[doc = "Gets the OS type of the virtual machine."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Gets the size of the virtual machine."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[doc = "Gets the network interface ID of the virtual machine."]
    #[serde(rename = "networkInterfaceId", default, skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[doc = "Gets OS disk blob uri for the virtual machine."]
    #[serde(rename = "osDiskId", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_id: Option<String>,
    #[doc = "Gets data disks blob uri for the virtual machine."]
    #[serde(rename = "dataDiskIds", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disk_ids: Vec<String>,
    #[doc = "Gets all data disks attached to the virtual machine."]
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disks: Vec<ComputeDataDisk>,
}
impl ComputeVmProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a virtual machine returned by the Microsoft.Compute API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeVmPropertiesFragment {}
impl ComputeVmPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a cost threshold item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CostThresholdProperties {
    #[doc = "The ID of the cost threshold item."]
    #[serde(rename = "thresholdId", default, skip_serializing_if = "Option::is_none")]
    pub threshold_id: Option<String>,
    #[doc = "Properties of a percentage cost threshold."]
    #[serde(rename = "percentageThreshold", default, skip_serializing_if = "Option::is_none")]
    pub percentage_threshold: Option<PercentageCostThresholdProperties>,
    #[doc = "Indicates whether this threshold will be displayed on cost charts."]
    #[serde(rename = "displayOnChart", default, skip_serializing_if = "Option::is_none")]
    pub display_on_chart: Option<cost_threshold_properties::DisplayOnChart>,
    #[doc = "Indicates whether notifications will be sent when this threshold is exceeded."]
    #[serde(rename = "sendNotificationWhenExceeded", default, skip_serializing_if = "Option::is_none")]
    pub send_notification_when_exceeded: Option<cost_threshold_properties::SendNotificationWhenExceeded>,
    #[doc = "Indicates the datetime when notifications were last sent for this threshold."]
    #[serde(rename = "notificationSent", default, skip_serializing_if = "Option::is_none")]
    pub notification_sent: Option<String>,
}
impl CostThresholdProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cost_threshold_properties {
    use super::*;
    #[doc = "Indicates whether this threshold will be displayed on cost charts."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DisplayOnChart")]
    pub enum DisplayOnChart {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DisplayOnChart {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DisplayOnChart {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DisplayOnChart {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("DisplayOnChart", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("DisplayOnChart", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates whether notifications will be sent when this threshold is exceeded."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SendNotificationWhenExceeded")]
    pub enum SendNotificationWhenExceeded {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SendNotificationWhenExceeded {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SendNotificationWhenExceeded {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SendNotificationWhenExceeded {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("SendNotificationWhenExceeded", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("SendNotificationWhenExceeded", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A custom image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomImage {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a custom image."]
    pub properties: CustomImageProperties,
}
impl CustomImage {
    pub fn new(properties: CustomImageProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A custom image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomImageFragment {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl CustomImageFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomImageList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomImage>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CustomImageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CustomImageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a custom image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomImageProperties {
    #[doc = "Properties for creating a custom image from a virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm: Option<CustomImagePropertiesFromVm>,
    #[doc = "Properties for creating a custom image from a VHD."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhd: Option<CustomImagePropertiesCustom>,
    #[doc = "The description of the custom image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The author of the custom image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "The creation date of the custom image."]
    #[serde(rename = "creationDate", with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The Managed Image Id backing the custom image."]
    #[serde(rename = "managedImageId", default, skip_serializing_if = "Option::is_none")]
    pub managed_image_id: Option<String>,
    #[doc = "The Managed Snapshot Id backing the custom image."]
    #[serde(rename = "managedSnapshotId", default, skip_serializing_if = "Option::is_none")]
    pub managed_snapshot_id: Option<String>,
    #[doc = "Storage information about the data disks present in the custom image"]
    #[serde(rename = "dataDiskStorageInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disk_storage_info: Vec<DataDiskStorageTypeInfo>,
    #[doc = "Properties for plan on a custom image."]
    #[serde(rename = "customImagePlan", default, skip_serializing_if = "Option::is_none")]
    pub custom_image_plan: Option<CustomImagePropertiesFromPlan>,
    #[doc = "Whether or not the custom images underlying offer/plan has been enabled for programmatic deployment"]
    #[serde(rename = "isPlanAuthorized", default, skip_serializing_if = "Option::is_none")]
    pub is_plan_authorized: Option<bool>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl CustomImageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for creating a custom image from a VHD."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomImagePropertiesCustom {
    #[doc = "The image name."]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "Indicates whether sysprep has been run on the VHD."]
    #[serde(rename = "sysPrep", default, skip_serializing_if = "Option::is_none")]
    pub sys_prep: Option<bool>,
    #[doc = "The OS type of the custom image (i.e. Windows, Linux)"]
    #[serde(rename = "osType")]
    pub os_type: custom_image_properties_custom::OsType,
}
impl CustomImagePropertiesCustom {
    pub fn new(os_type: custom_image_properties_custom::OsType) -> Self {
        Self {
            image_name: None,
            sys_prep: None,
            os_type,
        }
    }
}
pub mod custom_image_properties_custom {
    use super::*;
    #[doc = "The OS type of the custom image (i.e. Windows, Linux)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OsType")]
    pub enum OsType {
        Windows,
        Linux,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OsType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OsType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OsType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Windows => serializer.serialize_unit_variant("OsType", 0u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("OsType", 1u32, "Linux"),
                Self::None => serializer.serialize_unit_variant("OsType", 2u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties for creating a custom image from a VHD."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomImagePropertiesCustomFragment {}
impl CustomImagePropertiesCustomFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a custom image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomImagePropertiesFragment {}
impl CustomImagePropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for plan on a custom image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomImagePropertiesFromPlan {
    #[doc = "The id of the plan, equivalent to name of the plan"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The publisher for the plan from the marketplace image the custom image is derived from"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The offer for the plan from the marketplace image the custom image is derived from"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
}
impl CustomImagePropertiesFromPlan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for plan on a custom image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomImagePropertiesFromPlanFragment {}
impl CustomImagePropertiesFromPlanFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for creating a custom image from a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomImagePropertiesFromVm {
    #[doc = "The source vm identifier."]
    #[serde(rename = "sourceVmId", default, skip_serializing_if = "Option::is_none")]
    pub source_vm_id: Option<String>,
    #[doc = "Information about a Windows OS."]
    #[serde(rename = "windowsOsInfo", default, skip_serializing_if = "Option::is_none")]
    pub windows_os_info: Option<WindowsOsInfo>,
    #[doc = "Information about a Linux OS."]
    #[serde(rename = "linuxOsInfo", default, skip_serializing_if = "Option::is_none")]
    pub linux_os_info: Option<LinuxOsInfo>,
}
impl CustomImagePropertiesFromVm {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for creating a custom image from a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomImagePropertiesFromVmFragment {}
impl CustomImagePropertiesFromVmFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request body for adding a new or existing data disk to a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataDiskProperties {
    #[doc = "Properties to attach new disk to the Virtual Machine."]
    #[serde(rename = "attachNewDataDiskOptions", default, skip_serializing_if = "Option::is_none")]
    pub attach_new_data_disk_options: Option<AttachNewDataDiskOptions>,
    #[doc = "Specifies the existing lab disk id to attach to virtual machine."]
    #[serde(rename = "existingLabDiskId", default, skip_serializing_if = "Option::is_none")]
    pub existing_lab_disk_id: Option<String>,
    #[doc = "Caching option for a data disk (i.e. None, ReadOnly, ReadWrite)."]
    #[serde(rename = "hostCaching", default, skip_serializing_if = "Option::is_none")]
    pub host_caching: Option<data_disk_properties::HostCaching>,
}
impl DataDiskProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_disk_properties {
    use super::*;
    #[doc = "Caching option for a data disk (i.e. None, ReadOnly, ReadWrite)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HostCaching")]
    pub enum HostCaching {
        None,
        ReadOnly,
        ReadWrite,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HostCaching {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HostCaching {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HostCaching {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("HostCaching", 0u32, "None"),
                Self::ReadOnly => serializer.serialize_unit_variant("HostCaching", 1u32, "ReadOnly"),
                Self::ReadWrite => serializer.serialize_unit_variant("HostCaching", 2u32, "ReadWrite"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Request body for adding a new or existing data disk to a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataDiskPropertiesFragment {}
impl DataDiskPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage information about the data disks present in the custom image"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataDiskStorageTypeInfo {
    #[doc = "Disk Lun"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<String>,
    #[doc = "Disk Storage Type"]
    #[serde(rename = "storageType", default, skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<data_disk_storage_type_info::StorageType>,
}
impl DataDiskStorageTypeInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_disk_storage_type_info {
    use super::*;
    #[doc = "Disk Storage Type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageType")]
    pub enum StorageType {
        Standard,
        Premium,
        #[serde(rename = "StandardSSD")]
        StandardSsd,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("StorageType", 0u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("StorageType", 1u32, "Premium"),
                Self::StandardSsd => serializer.serialize_unit_variant("StorageType", 2u32, "StandardSSD"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Storage information about the data disks present in the custom image"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataDiskStorageTypeInfoFragment {}
impl DataDiskStorageTypeInfoFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a daily schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DayDetails {
    #[doc = "The time of day the schedule will occur."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}
impl DayDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a daily schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DayDetailsFragment {}
impl DayDetailsFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request body for detaching data disk from a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DetachDataDiskProperties {
    #[doc = "Specifies the disk resource ID to detach from virtual machine."]
    #[serde(rename = "existingLabDiskId", default, skip_serializing_if = "Option::is_none")]
    pub existing_lab_disk_id: Option<String>,
}
impl DetachDataDiskProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the disk to detach."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DetachDiskProperties {
    #[doc = "The resource ID of the Lab VM to which the disk is attached."]
    #[serde(rename = "leasedByLabVmId", default, skip_serializing_if = "Option::is_none")]
    pub leased_by_lab_vm_id: Option<String>,
}
impl DetachDiskProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Disk {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a disk."]
    pub properties: DiskProperties,
}
impl Disk {
    pub fn new(properties: DiskProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A Disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskFragment {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl DiskFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Disk>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiskList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DiskList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskProperties {
    #[doc = "The storage type for the disk (i.e. Standard, Premium)."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<disk_properties::DiskType>,
    #[doc = "The size of the disk in Gibibytes."]
    #[serde(rename = "diskSizeGiB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gi_b: Option<i32>,
    #[doc = "The resource ID of the VM to which this disk is leased."]
    #[serde(rename = "leasedByLabVmId", default, skip_serializing_if = "Option::is_none")]
    pub leased_by_lab_vm_id: Option<String>,
    #[doc = "When backed by a blob, the name of the VHD blob without extension."]
    #[serde(rename = "diskBlobName", default, skip_serializing_if = "Option::is_none")]
    pub disk_blob_name: Option<String>,
    #[doc = "When backed by a blob, the URI of underlying blob."]
    #[serde(rename = "diskUri", default, skip_serializing_if = "Option::is_none")]
    pub disk_uri: Option<String>,
    #[doc = "When backed by a blob, the storage account where the blob is."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "The creation date of the disk."]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The host caching policy of the disk (i.e. None, ReadOnly, ReadWrite)."]
    #[serde(rename = "hostCaching", default, skip_serializing_if = "Option::is_none")]
    pub host_caching: Option<String>,
    #[doc = "When backed by managed disk, this is the ID of the compute disk resource."]
    #[serde(rename = "managedDiskId", default, skip_serializing_if = "Option::is_none")]
    pub managed_disk_id: Option<String>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl DiskProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod disk_properties {
    use super::*;
    #[doc = "The storage type for the disk (i.e. Standard, Premium)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskType")]
    pub enum DiskType {
        Standard,
        Premium,
        #[serde(rename = "StandardSSD")]
        StandardSsd,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("DiskType", 0u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("DiskType", 1u32, "Premium"),
                Self::StandardSsd => serializer.serialize_unit_variant("DiskType", 2u32, "StandardSSD"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskPropertiesFragment {}
impl DiskPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An environment, which is essentially an ARM template deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DtlEnvironment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an environment."]
    pub properties: EnvironmentProperties,
}
impl DtlEnvironment {
    pub fn new(properties: EnvironmentProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "An environment, which is essentially an ARM template deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DtlEnvironmentFragment {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl DtlEnvironmentFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DtlEnvironmentList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DtlEnvironment>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DtlEnvironmentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DtlEnvironmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an environment deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentDeploymentProperties {
    #[doc = "The Azure Resource Manager template's identifier."]
    #[serde(rename = "armTemplateId", default, skip_serializing_if = "Option::is_none")]
    pub arm_template_id: Option<String>,
    #[doc = "The parameters of the Azure Resource Manager template."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ArmTemplateParameterProperties>,
}
impl EnvironmentDeploymentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an environment deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentDeploymentPropertiesFragment {}
impl EnvironmentDeploymentPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentProperties {
    #[doc = "Properties of an environment deployment."]
    #[serde(rename = "deploymentProperties", default, skip_serializing_if = "Option::is_none")]
    pub deployment_properties: Option<EnvironmentDeploymentProperties>,
    #[doc = "The display name of the Azure Resource Manager template that produced the environment."]
    #[serde(rename = "armTemplateDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub arm_template_display_name: Option<String>,
    #[doc = "The identifier of the resource group containing the environment's resources."]
    #[serde(rename = "resourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    #[doc = "The creator of the environment."]
    #[serde(rename = "createdByUser", default, skip_serializing_if = "Option::is_none")]
    pub created_by_user: Option<String>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl EnvironmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentPropertiesFragment {}
impl EnvironmentPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for evaluating a policy set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EvaluatePoliciesProperties {
    #[doc = "The fact name."]
    #[serde(rename = "factName", default, skip_serializing_if = "Option::is_none")]
    pub fact_name: Option<String>,
    #[doc = "The fact data."]
    #[serde(rename = "factData", default, skip_serializing_if = "Option::is_none")]
    pub fact_data: Option<String>,
    #[doc = "The value offset."]
    #[serde(rename = "valueOffset", default, skip_serializing_if = "Option::is_none")]
    pub value_offset: Option<String>,
    #[doc = "The user for which policies will be evaluated"]
    #[serde(rename = "userObjectId", default, skip_serializing_if = "Option::is_none")]
    pub user_object_id: Option<String>,
}
impl EvaluatePoliciesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request body for evaluating a policy set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EvaluatePoliciesRequest {
    #[doc = "Policies to evaluate."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policies: Vec<EvaluatePoliciesProperties>,
}
impl EvaluatePoliciesRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response body for evaluating a policy set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EvaluatePoliciesResponse {
    #[doc = "Results of evaluating a policy set."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<PolicySetResult>,
}
impl EvaluatePoliciesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An event to be notified for."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Event {
    #[doc = "The event type for which this notification is enabled (i.e. AutoShutdown, Cost)"]
    #[serde(rename = "eventName", default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<event::EventName>,
}
impl Event {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod event {
    use super::*;
    #[doc = "The event type for which this notification is enabled (i.e. AutoShutdown, Cost)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EventName")]
    pub enum EventName {
        AutoShutdown,
        Cost,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EventName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EventName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EventName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AutoShutdown => serializer.serialize_unit_variant("EventName", 0u32, "AutoShutdown"),
                Self::Cost => serializer.serialize_unit_variant("EventName", 1u32, "Cost"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An event to be notified for."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventFragment {}
impl EventFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters of the export operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportResourceUsageParameters {
    #[doc = "The blob storage absolute sas uri with write permission to the container which the usage data needs to be uploaded to."]
    #[serde(rename = "blobStorageAbsoluteSasUri", default, skip_serializing_if = "Option::is_none")]
    pub blob_storage_absolute_sas_uri: Option<String>,
    #[doc = "The start time of the usage. If not provided, usage will be reported since the beginning of data collection."]
    #[serde(rename = "usageStartDate", with = "azure_core::date::rfc3339::option")]
    pub usage_start_date: Option<time::OffsetDateTime>,
}
impl ExportResourceUsageParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subnet information as returned by the Microsoft.Network API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalSubnet {
    #[doc = "Gets or sets the identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ExternalSubnet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subnet information as returned by the Microsoft.Network API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalSubnetFragment {}
impl ExternalSubnetFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A formula for creating a VM, specifying an image base and other parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Formula {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a formula."]
    pub properties: FormulaProperties,
}
impl Formula {
    pub fn new(properties: FormulaProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A formula for creating a VM, specifying an image base and other parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FormulaFragment {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl FormulaFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FormulaList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Formula>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FormulaList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FormulaList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a formula."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FormulaProperties {
    #[doc = "The description of the formula."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The author of the formula."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "The OS type of the formula."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The creation date of the formula."]
    #[serde(rename = "creationDate", with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "Properties for creating a virtual machine."]
    #[serde(rename = "formulaContent", default, skip_serializing_if = "Option::is_none")]
    pub formula_content: Option<LabVirtualMachineCreationParameter>,
    #[doc = "Information about a VM from which a formula is to be created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm: Option<FormulaPropertiesFromVm>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl FormulaProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a formula."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FormulaPropertiesFragment {}
impl FormulaPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a VM from which a formula is to be created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FormulaPropertiesFromVm {
    #[doc = "The identifier of the VM from which a formula is to be created."]
    #[serde(rename = "labVmId", default, skip_serializing_if = "Option::is_none")]
    pub lab_vm_id: Option<String>,
}
impl FormulaPropertiesFromVm {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a VM from which a formula is to be created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FormulaPropertiesFromVmFragment {}
impl FormulaPropertiesFromVmFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A gallery image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImage {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a gallery image."]
    pub properties: GalleryImageProperties,
}
impl GalleryImage {
    pub fn new(properties: GalleryImageProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImageList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GalleryImage>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GalleryImageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GalleryImageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a gallery image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImageProperties {
    #[doc = "The author of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "The creation date of the gallery image."]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The description of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The reference information for an Azure Marketplace image."]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<GalleryImageReference>,
    #[doc = "The icon of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "Indicates whether this gallery image is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The third party plan that applies to this image"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Indicates if the plan has been authorized for programmatic deployment."]
    #[serde(rename = "isPlanAuthorized", default, skip_serializing_if = "Option::is_none")]
    pub is_plan_authorized: Option<bool>,
}
impl GalleryImageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The reference information for an Azure Marketplace image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImageReference {
    #[doc = "The offer of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "The publisher of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The SKU of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "The OS type of the gallery image."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The version of the gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl GalleryImageReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The reference information for an Azure Marketplace image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImageReferenceFragment {}
impl GalleryImageReferenceFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for generating an ARM template for deploying artifacts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateArmTemplateRequest {
    #[doc = "The resource name of the virtual machine."]
    #[serde(rename = "virtualMachineName", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_name: Option<String>,
    #[doc = "The parameters of the ARM template."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterInfo>,
    #[doc = "The location of the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Options for uploading the files for the artifact. UploadFilesAndGenerateSasTokens is the default value."]
    #[serde(rename = "fileUploadOptions", default, skip_serializing_if = "Option::is_none")]
    pub file_upload_options: Option<generate_arm_template_request::FileUploadOptions>,
}
impl GenerateArmTemplateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod generate_arm_template_request {
    use super::*;
    #[doc = "Options for uploading the files for the artifact. UploadFilesAndGenerateSasTokens is the default value."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FileUploadOptions")]
    pub enum FileUploadOptions {
        UploadFilesAndGenerateSasTokens,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FileUploadOptions {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FileUploadOptions {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FileUploadOptions {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::UploadFilesAndGenerateSasTokens => {
                    serializer.serialize_unit_variant("FileUploadOptions", 0u32, "UploadFilesAndGenerateSasTokens")
                }
                Self::None => serializer.serialize_unit_variant("FileUploadOptions", 1u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties for generating an upload URI."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateUploadUriParameter {
    #[doc = "The blob name of the upload URI."]
    #[serde(rename = "blobName", default, skip_serializing_if = "Option::is_none")]
    pub blob_name: Option<String>,
}
impl GenerateUploadUriParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response body for generating an upload URI."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateUploadUriResponse {
    #[doc = "The upload URI for the VHD."]
    #[serde(rename = "uploadUri", default, skip_serializing_if = "Option::is_none")]
    pub upload_uri: Option<String>,
}
impl GenerateUploadUriResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an hourly schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HourDetails {
    #[doc = "Minutes of the hour the schedule will run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<i32>,
}
impl HourDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an hourly schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HourDetailsFragment {}
impl HourDetailsFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a managed identity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityProperties {
    #[doc = "Managed identity."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity_properties::Type>,
    #[doc = "The principal id of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant identifier of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The client secret URL of the identity."]
    #[serde(rename = "clientSecretUrl", default, skip_serializing_if = "Option::is_none")]
    pub client_secret_url: Option<String>,
}
impl IdentityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity_properties {
    use super::*;
    #[doc = "Managed identity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned,UserAssigned")]
        SystemAssignedUserAssigned,
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
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 2u32, "UserAssigned"),
                Self::SystemAssignedUserAssigned => serializer.serialize_unit_variant("Type", 3u32, "SystemAssigned,UserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "This represents the payload required to import a virtual machine from a different lab into the current one"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportLabVirtualMachineRequest {
    #[doc = "The full resource ID of the virtual machine to be imported."]
    #[serde(rename = "sourceVirtualMachineResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_virtual_machine_resource_id: Option<String>,
    #[doc = "The name of the virtual machine in the destination lab"]
    #[serde(rename = "destinationVirtualMachineName", default, skip_serializing_if = "Option::is_none")]
    pub destination_virtual_machine_name: Option<String>,
}
impl ImportLabVirtualMachineRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A rule for NAT - exposing a VM's port (backendPort) on the public IP address using a load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundNatRule {
    #[doc = "The transport protocol for the endpoint."]
    #[serde(rename = "transportProtocol", default, skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<inbound_nat_rule::TransportProtocol>,
    #[doc = "The external endpoint port of the inbound connection. Possible values range between 1 and 65535, inclusive. If unspecified, a value will be allocated automatically."]
    #[serde(rename = "frontendPort", default, skip_serializing_if = "Option::is_none")]
    pub frontend_port: Option<i32>,
    #[doc = "The port to which the external traffic will be redirected."]
    #[serde(rename = "backendPort", default, skip_serializing_if = "Option::is_none")]
    pub backend_port: Option<i32>,
}
impl InboundNatRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod inbound_nat_rule {
    use super::*;
    #[doc = "The transport protocol for the endpoint."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TransportProtocol")]
    pub enum TransportProtocol {
        Tcp,
        Udp,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TransportProtocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TransportProtocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TransportProtocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tcp => serializer.serialize_unit_variant("TransportProtocol", 0u32, "Tcp"),
                Self::Udp => serializer.serialize_unit_variant("TransportProtocol", 1u32, "Udp"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A rule for NAT - exposing a VM's port (backendPort) on the public IP address using a load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InboundNatRuleFragment {}
impl InboundNatRuleFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A lab."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Lab {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a lab."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabProperties>,
}
impl Lab {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a lab's announcement banner"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabAnnouncementProperties {
    #[doc = "The plain text title for the lab announcement"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The markdown text (if any) that this lab displays in the UI. If left empty/null, nothing will be shown."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,
    #[doc = "Is the lab announcement active/enabled at this time?"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<lab_announcement_properties::Enabled>,
    #[doc = "The time at which the announcement expires (null for never)"]
    #[serde(rename = "expirationDate", with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Has this announcement expired?"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl LabAnnouncementProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod lab_announcement_properties {
    use super::*;
    #[doc = "Is the lab announcement active/enabled at this time?"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Enabled")]
    pub enum Enabled {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Enabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Enabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Enabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("Enabled", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Enabled", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a lab's announcement banner"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabAnnouncementPropertiesFragment {}
impl LabAnnouncementPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A cost item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabCost {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a cost item."]
    pub properties: LabCostProperties,
}
impl LabCost {
    pub fn new(properties: LabCostProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "The properties of a lab cost item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabCostDetailsProperties {
    #[doc = "The date of the cost item."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub date: Option<time::OffsetDateTime>,
    #[doc = "The cost component of the cost item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    #[doc = "The type of the cost."]
    #[serde(rename = "costType", default, skip_serializing_if = "Option::is_none")]
    pub cost_type: Option<lab_cost_details_properties::CostType>,
}
impl LabCostDetailsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod lab_cost_details_properties {
    use super::*;
    #[doc = "The type of the cost."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CostType")]
    pub enum CostType {
        Unavailable,
        Reported,
        Projected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CostType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CostType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CostType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unavailable => serializer.serialize_unit_variant("CostType", 0u32, "Unavailable"),
                Self::Reported => serializer.serialize_unit_variant("CostType", 1u32, "Reported"),
                Self::Projected => serializer.serialize_unit_variant("CostType", 2u32, "Projected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a cost item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabCostProperties {
    #[doc = "Properties of a cost target."]
    #[serde(rename = "targetCost", default, skip_serializing_if = "Option::is_none")]
    pub target_cost: Option<TargetCostProperties>,
    #[doc = "The properties of the cost summary."]
    #[serde(rename = "labCostSummary", default, skip_serializing_if = "Option::is_none")]
    pub lab_cost_summary: Option<LabCostSummaryProperties>,
    #[doc = "The lab cost details component of the cost data."]
    #[serde(rename = "labCostDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub lab_cost_details: Vec<LabCostDetailsProperties>,
    #[doc = "The resource cost component of the cost data."]
    #[serde(rename = "resourceCosts", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_costs: Vec<LabResourceCostProperties>,
    #[doc = "The currency code of the cost."]
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[doc = "The start time of the cost data."]
    #[serde(rename = "startDateTime", with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the cost data."]
    #[serde(rename = "endDateTime", with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "The creation date of the cost."]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl LabCostProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the cost summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabCostSummaryProperties {
    #[doc = "The cost component of the cost item."]
    #[serde(rename = "estimatedLabCost", default, skip_serializing_if = "Option::is_none")]
    pub estimated_lab_cost: Option<f64>,
}
impl LabCostSummaryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A lab."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabFragment {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl LabFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Lab>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LabList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl LabList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a lab."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabProperties {
    #[doc = "The lab's default storage account."]
    #[serde(rename = "defaultStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub default_storage_account: Option<String>,
    #[doc = "The lab's default premium storage account."]
    #[serde(rename = "defaultPremiumStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub default_premium_storage_account: Option<String>,
    #[doc = "The lab's artifact storage account."]
    #[serde(rename = "artifactsStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub artifacts_storage_account: Option<String>,
    #[doc = "The lab's premium data disk storage account."]
    #[serde(rename = "premiumDataDiskStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub premium_data_disk_storage_account: Option<String>,
    #[doc = "The lab's Key vault."]
    #[serde(rename = "vaultName", default, skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
    #[doc = "Type of storage used by the lab. It can be either Premium or Standard. Default is Premium."]
    #[serde(rename = "labStorageType", default, skip_serializing_if = "Option::is_none")]
    pub lab_storage_type: Option<lab_properties::LabStorageType>,
    #[doc = "The ordered list of artifact resource IDs that should be applied on all Linux VM creations by default, prior to the artifacts specified by the user."]
    #[serde(rename = "mandatoryArtifactsResourceIdsLinux", default, skip_serializing_if = "Vec::is_empty")]
    pub mandatory_artifacts_resource_ids_linux: Vec<String>,
    #[doc = "The ordered list of artifact resource IDs that should be applied on all Windows VM creations by default, prior to the artifacts specified by the user."]
    #[serde(rename = "mandatoryArtifactsResourceIdsWindows", default, skip_serializing_if = "Vec::is_empty")]
    pub mandatory_artifacts_resource_ids_windows: Vec<String>,
    #[doc = "The creation date of the lab."]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The setting to enable usage of premium data disks.\r\nWhen its value is 'Enabled', creation of standard or premium data disks is allowed.\r\nWhen its value is 'Disabled', only creation of standard data disks is allowed."]
    #[serde(rename = "premiumDataDisks", default, skip_serializing_if = "Option::is_none")]
    pub premium_data_disks: Option<lab_properties::PremiumDataDisks>,
    #[doc = "The access rights to be granted to the user when provisioning an environment"]
    #[serde(rename = "environmentPermission", default, skip_serializing_if = "Option::is_none")]
    pub environment_permission: Option<lab_properties::EnvironmentPermission>,
    #[doc = "Properties of a lab's announcement banner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub announcement: Option<LabAnnouncementProperties>,
    #[doc = "Properties of a lab's support banner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support: Option<LabSupportProperties>,
    #[doc = "The resource group in which all new lab virtual machines will be created. To let DevTest Labs manage resource group creation, set this value to null."]
    #[serde(rename = "vmCreationResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub vm_creation_resource_group: Option<String>,
    #[doc = "The public IP address for the lab's load balancer."]
    #[serde(rename = "publicIpId", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_id: Option<String>,
    #[doc = "The load balancer used to for lab VMs that use shared IP address."]
    #[serde(rename = "loadBalancerId", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<String>,
    #[doc = "The Network Security Group attached to the lab VMs Network interfaces to restrict open ports."]
    #[serde(rename = "networkSecurityGroupId", default, skip_serializing_if = "Option::is_none")]
    pub network_security_group_id: Option<String>,
    #[doc = "Extended properties of the lab used for experimental features"]
    #[serde(rename = "extendedProperties", default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<serde_json::Value>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl LabProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod lab_properties {
    use super::*;
    #[doc = "Type of storage used by the lab. It can be either Premium or Standard. Default is Premium."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LabStorageType")]
    pub enum LabStorageType {
        Standard,
        Premium,
        #[serde(rename = "StandardSSD")]
        StandardSsd,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LabStorageType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LabStorageType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LabStorageType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("LabStorageType", 0u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("LabStorageType", 1u32, "Premium"),
                Self::StandardSsd => serializer.serialize_unit_variant("LabStorageType", 2u32, "StandardSSD"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for LabStorageType {
        fn default() -> Self {
            Self::Premium
        }
    }
    #[doc = "The setting to enable usage of premium data disks.\r\nWhen its value is 'Enabled', creation of standard or premium data disks is allowed.\r\nWhen its value is 'Disabled', only creation of standard data disks is allowed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PremiumDataDisks")]
    pub enum PremiumDataDisks {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PremiumDataDisks {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PremiumDataDisks {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PremiumDataDisks {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("PremiumDataDisks", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("PremiumDataDisks", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The access rights to be granted to the user when provisioning an environment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnvironmentPermission")]
    pub enum EnvironmentPermission {
        Reader,
        Contributor,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnvironmentPermission {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnvironmentPermission {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnvironmentPermission {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Reader => serializer.serialize_unit_variant("EnvironmentPermission", 0u32, "Reader"),
                Self::Contributor => serializer.serialize_unit_variant("EnvironmentPermission", 1u32, "Contributor"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a lab."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabPropertiesFragment {}
impl LabPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a resource cost item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabResourceCostProperties {
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resourcename: Option<String>,
    #[doc = "The unique identifier of the resource."]
    #[serde(rename = "resourceUId", default, skip_serializing_if = "Option::is_none")]
    pub resource_u_id: Option<String>,
    #[doc = "The cost component of the resource cost item."]
    #[serde(rename = "resourceCost", default, skip_serializing_if = "Option::is_none")]
    pub resource_cost: Option<f64>,
    #[doc = "The logical resource type (ex. virtualmachine, storageaccount)"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The owner of the resource (ex. janedoe@microsoft.com)"]
    #[serde(rename = "resourceOwner", default, skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[doc = "The category of the resource (ex. Premium_LRS, Standard_DS1)"]
    #[serde(rename = "resourcePricingTier", default, skip_serializing_if = "Option::is_none")]
    pub resource_pricing_tier: Option<String>,
    #[doc = "The status of the resource (ex. Active)"]
    #[serde(rename = "resourceStatus", default, skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<String>,
    #[doc = "The ID of the resource"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The ID of the external resource"]
    #[serde(rename = "externalResourceId", default, skip_serializing_if = "Option::is_none")]
    pub external_resource_id: Option<String>,
}
impl LabResourceCostProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a lab's support banner"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabSupportProperties {
    #[doc = "Is the lab support banner active/enabled at this time?"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<lab_support_properties::Enabled>,
    #[doc = "The markdown text (if any) that this lab displays in the UI. If left empty/null, nothing will be shown."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,
}
impl LabSupportProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod lab_support_properties {
    use super::*;
    #[doc = "Is the lab support banner active/enabled at this time?"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Enabled")]
    pub enum Enabled {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Enabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Enabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Enabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("Enabled", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Enabled", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a lab's support banner"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabSupportPropertiesFragment {}
impl LabSupportPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a VHD in the lab."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabVhd {
    #[doc = "The URI to the VHD."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl LabVhd {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabVhdList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LabVhd>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LabVhdList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl LabVhdList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabVirtualMachine {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a virtual machine."]
    pub properties: LabVirtualMachineProperties,
}
impl LabVirtualMachine {
    pub fn new(properties: LabVirtualMachineProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "Properties for creating a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabVirtualMachineCreationParameter {
    #[doc = "Properties for virtual machine creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabVirtualMachineCreationParameterProperties>,
    #[doc = "The name of the virtual machine or environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The location of the new virtual machine or environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl LabVirtualMachineCreationParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for creating a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabVirtualMachineCreationParameterFragment {}
impl LabVirtualMachineCreationParameterFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for virtual machine creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabVirtualMachineCreationParameterProperties {
    #[doc = "Parameters for creating multiple virtual machines as a single action."]
    #[serde(rename = "bulkCreationParameters", default, skip_serializing_if = "Option::is_none")]
    pub bulk_creation_parameters: Option<BulkCreationParameters>,
    #[doc = "The notes of the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[doc = "The object identifier of the owner of the virtual machine."]
    #[serde(rename = "ownerObjectId", default, skip_serializing_if = "Option::is_none")]
    pub owner_object_id: Option<String>,
    #[doc = "The user principal name of the virtual machine owner."]
    #[serde(rename = "ownerUserPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub owner_user_principal_name: Option<String>,
    #[doc = "The creation date of the virtual machine."]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The custom image identifier of the virtual machine."]
    #[serde(rename = "customImageId", default, skip_serializing_if = "Option::is_none")]
    pub custom_image_id: Option<String>,
    #[doc = "The size of the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The user name of the virtual machine."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "The password of the virtual machine administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The SSH key of the virtual machine administrator."]
    #[serde(rename = "sshKey", default, skip_serializing_if = "Option::is_none")]
    pub ssh_key: Option<String>,
    #[doc = "Indicates whether this virtual machine uses an SSH key for authentication."]
    #[serde(rename = "isAuthenticationWithSshKey", default, skip_serializing_if = "Option::is_none")]
    pub is_authentication_with_ssh_key: Option<bool>,
    #[doc = "The lab subnet name of the virtual machine."]
    #[serde(rename = "labSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub lab_subnet_name: Option<String>,
    #[doc = "The lab virtual network identifier of the virtual machine."]
    #[serde(rename = "labVirtualNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub lab_virtual_network_id: Option<String>,
    #[doc = "Indicates whether the virtual machine is to be created without a public IP address."]
    #[serde(rename = "disallowPublicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub disallow_public_ip_address: Option<bool>,
    #[doc = "The artifacts to be installed on the virtual machine."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ArtifactInstallProperties>,
    #[doc = "The reference information for an Azure Marketplace image."]
    #[serde(rename = "galleryImageReference", default, skip_serializing_if = "Option::is_none")]
    pub gallery_image_reference: Option<GalleryImageReference>,
    #[doc = "The id of the plan associated with the virtual machine image"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Properties of a network interface."]
    #[serde(rename = "networkInterface", default, skip_serializing_if = "Option::is_none")]
    pub network_interface: Option<NetworkInterfaceProperties>,
    #[doc = "The expiration date for VM."]
    #[serde(rename = "expirationDate", with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Indicates whether another user can take ownership of the virtual machine"]
    #[serde(rename = "allowClaim", default, skip_serializing_if = "Option::is_none")]
    pub allow_claim: Option<bool>,
    #[doc = "Storage type to use for virtual machine (i.e. Standard, Premium)."]
    #[serde(rename = "storageType", default, skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[doc = "The resource ID of the environment that contains this virtual machine, if any."]
    #[serde(rename = "environmentId", default, skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[doc = "New or existing data disks to attach to the virtual machine after creation"]
    #[serde(rename = "dataDiskParameters", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disk_parameters: Vec<DataDiskProperties>,
    #[doc = "Virtual Machine schedules to be created"]
    #[serde(rename = "scheduleParameters", default, skip_serializing_if = "Vec::is_empty")]
    pub schedule_parameters: Vec<ScheduleCreationParameter>,
}
impl LabVirtualMachineCreationParameterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for virtual machine creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabVirtualMachineCreationParameterPropertiesFragment {}
impl LabVirtualMachineCreationParameterPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabVirtualMachineFragment {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl LabVirtualMachineFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabVirtualMachineList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LabVirtualMachine>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LabVirtualMachineList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl LabVirtualMachineList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabVirtualMachineProperties {
    #[doc = "The notes of the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[doc = "The object identifier of the owner of the virtual machine."]
    #[serde(rename = "ownerObjectId", default, skip_serializing_if = "Option::is_none")]
    pub owner_object_id: Option<String>,
    #[doc = "The user principal name of the virtual machine owner."]
    #[serde(rename = "ownerUserPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub owner_user_principal_name: Option<String>,
    #[doc = "The object identifier of the creator of the virtual machine."]
    #[serde(rename = "createdByUserId", default, skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
    #[doc = "The email address of creator of the virtual machine."]
    #[serde(rename = "createdByUser", default, skip_serializing_if = "Option::is_none")]
    pub created_by_user: Option<String>,
    #[doc = "The creation date of the virtual machine."]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The resource identifier (Microsoft.Compute) of the virtual machine."]
    #[serde(rename = "computeId", default, skip_serializing_if = "Option::is_none")]
    pub compute_id: Option<String>,
    #[doc = "The custom image identifier of the virtual machine."]
    #[serde(rename = "customImageId", default, skip_serializing_if = "Option::is_none")]
    pub custom_image_id: Option<String>,
    #[doc = "The OS type of the virtual machine."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The size of the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The user name of the virtual machine."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "The password of the virtual machine administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The SSH key of the virtual machine administrator."]
    #[serde(rename = "sshKey", default, skip_serializing_if = "Option::is_none")]
    pub ssh_key: Option<String>,
    #[doc = "Indicates whether this virtual machine uses an SSH key for authentication."]
    #[serde(rename = "isAuthenticationWithSshKey", default, skip_serializing_if = "Option::is_none")]
    pub is_authentication_with_ssh_key: Option<bool>,
    #[doc = "The fully-qualified domain name of the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "The lab subnet name of the virtual machine."]
    #[serde(rename = "labSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub lab_subnet_name: Option<String>,
    #[doc = "The lab virtual network identifier of the virtual machine."]
    #[serde(rename = "labVirtualNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub lab_virtual_network_id: Option<String>,
    #[doc = "Indicates whether the virtual machine is to be created without a public IP address."]
    #[serde(rename = "disallowPublicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub disallow_public_ip_address: Option<bool>,
    #[doc = "The artifacts to be installed on the virtual machine."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<ArtifactInstallProperties>,
    #[doc = "Properties of an artifact deployment."]
    #[serde(rename = "artifactDeploymentStatus", default, skip_serializing_if = "Option::is_none")]
    pub artifact_deployment_status: Option<ArtifactDeploymentStatusProperties>,
    #[doc = "The reference information for an Azure Marketplace image."]
    #[serde(rename = "galleryImageReference", default, skip_serializing_if = "Option::is_none")]
    pub gallery_image_reference: Option<GalleryImageReference>,
    #[doc = "The id of the plan associated with the virtual machine image"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Properties of a virtual machine returned by the Microsoft.Compute API."]
    #[serde(rename = "computeVm", default, skip_serializing_if = "Option::is_none")]
    pub compute_vm: Option<ComputeVmProperties>,
    #[doc = "Properties of a network interface."]
    #[serde(rename = "networkInterface", default, skip_serializing_if = "Option::is_none")]
    pub network_interface: Option<NetworkInterfaceProperties>,
    #[doc = "Schedules applicable to a virtual machine. The schedules may have been defined on a VM or on lab level."]
    #[serde(rename = "applicableSchedule", default, skip_serializing_if = "Option::is_none")]
    pub applicable_schedule: Option<ApplicableSchedule>,
    #[doc = "The expiration date for VM."]
    #[serde(rename = "expirationDate", with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "Indicates whether another user can take ownership of the virtual machine"]
    #[serde(rename = "allowClaim", default, skip_serializing_if = "Option::is_none")]
    pub allow_claim: Option<bool>,
    #[doc = "Storage type to use for virtual machine (i.e. Standard, Premium)."]
    #[serde(rename = "storageType", default, skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[doc = "Tells source of creation of lab virtual machine. Output property only."]
    #[serde(rename = "virtualMachineCreationSource", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_creation_source: Option<lab_virtual_machine_properties::VirtualMachineCreationSource>,
    #[doc = "The resource ID of the environment that contains this virtual machine, if any."]
    #[serde(rename = "environmentId", default, skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[doc = "New or existing data disks to attach to the virtual machine after creation"]
    #[serde(rename = "dataDiskParameters", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disk_parameters: Vec<DataDiskProperties>,
    #[doc = "Virtual Machine schedules to be created"]
    #[serde(rename = "scheduleParameters", default, skip_serializing_if = "Vec::is_empty")]
    pub schedule_parameters: Vec<ScheduleCreationParameter>,
    #[doc = "Last known compute power state captured in DTL"]
    #[serde(rename = "lastKnownPowerState", default, skip_serializing_if = "Option::is_none")]
    pub last_known_power_state: Option<String>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl LabVirtualMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod lab_virtual_machine_properties {
    use super::*;
    #[doc = "Tells source of creation of lab virtual machine. Output property only."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VirtualMachineCreationSource")]
    pub enum VirtualMachineCreationSource {
        FromCustomImage,
        FromGalleryImage,
        FromSharedGalleryImage,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VirtualMachineCreationSource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VirtualMachineCreationSource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VirtualMachineCreationSource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::FromCustomImage => serializer.serialize_unit_variant("VirtualMachineCreationSource", 0u32, "FromCustomImage"),
                Self::FromGalleryImage => serializer.serialize_unit_variant("VirtualMachineCreationSource", 1u32, "FromGalleryImage"),
                Self::FromSharedGalleryImage => {
                    serializer.serialize_unit_variant("VirtualMachineCreationSource", 2u32, "FromSharedGalleryImage")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabVirtualMachinePropertiesFragment {}
impl LabVirtualMachinePropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a Linux OS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxOsInfo {
    #[doc = "The state of the Linux OS (i.e. NonDeprovisioned, DeprovisionRequested, DeprovisionApplied)."]
    #[serde(rename = "linuxOsState", default, skip_serializing_if = "Option::is_none")]
    pub linux_os_state: Option<linux_os_info::LinuxOsState>,
}
impl LinuxOsInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod linux_os_info {
    use super::*;
    #[doc = "The state of the Linux OS (i.e. NonDeprovisioned, DeprovisionRequested, DeprovisionApplied)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LinuxOsState")]
    pub enum LinuxOsState {
        NonDeprovisioned,
        DeprovisionRequested,
        DeprovisionApplied,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LinuxOsState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LinuxOsState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LinuxOsState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NonDeprovisioned => serializer.serialize_unit_variant("LinuxOsState", 0u32, "NonDeprovisioned"),
                Self::DeprovisionRequested => serializer.serialize_unit_variant("LinuxOsState", 1u32, "DeprovisionRequested"),
                Self::DeprovisionApplied => serializer.serialize_unit_variant("LinuxOsState", 2u32, "DeprovisionApplied"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information about a Linux OS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxOsInfoFragment {}
impl LinuxOsInfoFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a network interface."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceProperties {
    #[doc = "The resource ID of the virtual network."]
    #[serde(rename = "virtualNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_id: Option<String>,
    #[doc = "The resource ID of the sub net."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "The resource ID of the public IP address."]
    #[serde(rename = "publicIpAddressId", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address_id: Option<String>,
    #[doc = "The public IP address."]
    #[serde(rename = "publicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    #[doc = "The private IP address."]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "The DNS name."]
    #[serde(rename = "dnsName", default, skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[doc = "The RdpAuthority property is a server DNS host name or IP address followed by the service port number for RDP (Remote Desktop Protocol)."]
    #[serde(rename = "rdpAuthority", default, skip_serializing_if = "Option::is_none")]
    pub rdp_authority: Option<String>,
    #[doc = "The SshAuthority property is a server DNS host name or IP address followed by the service port number for SSH."]
    #[serde(rename = "sshAuthority", default, skip_serializing_if = "Option::is_none")]
    pub ssh_authority: Option<String>,
    #[doc = "Properties of a virtual machine that determine how it is connected to a load balancer."]
    #[serde(rename = "sharedPublicIpAddressConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub shared_public_ip_address_configuration: Option<SharedPublicIpAddressConfiguration>,
}
impl NetworkInterfaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a network interface."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfacePropertiesFragment {}
impl NetworkInterfacePropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A notification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationChannel {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a schedule."]
    pub properties: NotificationChannelProperties,
}
impl NotificationChannel {
    pub fn new(properties: NotificationChannelProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A notification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationChannelFragment {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl NotificationChannelFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationChannelList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NotificationChannel>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NotificationChannelList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NotificationChannelList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationChannelProperties {
    #[doc = "The webhook URL to send notifications to."]
    #[serde(rename = "webHookUrl", default, skip_serializing_if = "Option::is_none")]
    pub web_hook_url: Option<String>,
    #[doc = "The email recipient to send notifications to (can be a list of semi-colon separated email addresses)."]
    #[serde(rename = "emailRecipient", default, skip_serializing_if = "Option::is_none")]
    pub email_recipient: Option<String>,
    #[doc = "The locale to use when sending a notification (fallback for unsupported languages is EN)."]
    #[serde(rename = "notificationLocale", default, skip_serializing_if = "Option::is_none")]
    pub notification_locale: Option<String>,
    #[doc = "Description of notification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The list of event for which this notification is enabled."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<Event>,
    #[doc = "The creation date of the notification channel."]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl NotificationChannelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationChannelPropertiesFragment {}
impl NotificationChannelPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Notification settings for a schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationSettings {
    #[doc = "If notifications are enabled for this schedule (i.e. Enabled, Disabled)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<notification_settings::Status>,
    #[doc = "Time in minutes before event at which notification will be sent."]
    #[serde(rename = "timeInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub time_in_minutes: Option<i32>,
    #[doc = "The webhook URL to which the notification will be sent."]
    #[serde(rename = "webhookUrl", default, skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[doc = "The email recipient to send notifications to (can be a list of semi-colon separated email addresses)."]
    #[serde(rename = "emailRecipient", default, skip_serializing_if = "Option::is_none")]
    pub email_recipient: Option<String>,
    #[doc = "The locale to use when sending a notification (fallback for unsupported languages is EN)."]
    #[serde(rename = "notificationLocale", default, skip_serializing_if = "Option::is_none")]
    pub notification_locale: Option<String>,
}
impl NotificationSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod notification_settings {
    use super::*;
    #[doc = "If notifications are enabled for this schedule (i.e. Enabled, Disabled)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[doc = "Notification settings for a schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationSettingsFragment {}
impl NotificationSettingsFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for generating a Notification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotifyParameters {
    #[doc = "The type of event (i.e. AutoShutdown, Cost)"]
    #[serde(rename = "eventName", default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<notify_parameters::EventName>,
    #[doc = "Properties for the notification in json format."]
    #[serde(rename = "jsonPayload", default, skip_serializing_if = "Option::is_none")]
    pub json_payload: Option<String>,
}
impl NotifyParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod notify_parameters {
    use super::*;
    #[doc = "The type of event (i.e. AutoShutdown, Cost)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EventName")]
    pub enum EventName {
        AutoShutdown,
        Cost,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EventName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EventName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EventName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AutoShutdown => serializer.serialize_unit_variant("EventName", 0u32, "AutoShutdown"),
                Self::Cost => serializer.serialize_unit_variant("EventName", 1u32, "Cost"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Error details for the operation in case of a failure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationError {
    #[doc = "The error code of the operation error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message of the operation error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl OperationError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The REST API operation supported by DevTestLab ResourceProvider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetadata {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that describes the operations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationMetadataDisplay>,
}
impl OperationMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that describes the operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetadataDisplay {
    #[doc = "Friendly name of the resource provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource type on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation type: read, write, delete, listKeys/action, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Friendly name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationMetadataDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Operation Result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "The operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The status code for the operation."]
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<operation_result::StatusCode>,
    #[doc = "Error details for the operation in case of a failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationError>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_result {
    use super::*;
    #[doc = "The status code for the operation."]
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
#[doc = "Information about an artifact's parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterInfo {
    #[doc = "The name of the artifact parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the artifact parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ParameterInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A file containing a set of parameter values for an ARM template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParametersValueFileInfo {
    #[doc = "File name."]
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "Contents of the file."]
    #[serde(rename = "parametersValueInfo", default, skip_serializing_if = "Option::is_none")]
    pub parameters_value_info: Option<serde_json::Value>,
}
impl ParametersValueFileInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a percentage cost threshold."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PercentageCostThresholdProperties {
    #[doc = "The cost threshold value."]
    #[serde(rename = "thresholdValue", default, skip_serializing_if = "Option::is_none")]
    pub threshold_value: Option<f64>,
}
impl PercentageCostThresholdProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Policy {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a Policy."]
    pub properties: PolicyProperties,
}
impl Policy {
    pub fn new(properties: PolicyProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A Policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyFragment {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl PolicyFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Policy>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PolicyList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PolicyList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyProperties {
    #[doc = "The description of the policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The status of the policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<policy_properties::Status>,
    #[doc = "The fact name of the policy (e.g. LabVmCount, LabVmSize, MaxVmsAllowedPerLab, etc."]
    #[serde(rename = "factName", default, skip_serializing_if = "Option::is_none")]
    pub fact_name: Option<policy_properties::FactName>,
    #[doc = "The fact data of the policy."]
    #[serde(rename = "factData", default, skip_serializing_if = "Option::is_none")]
    pub fact_data: Option<String>,
    #[doc = "The threshold of the policy (i.e. a number for MaxValuePolicy, and a JSON array of values for AllowedValuesPolicy)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<String>,
    #[doc = "The evaluator type of the policy (i.e. AllowedValuesPolicy, MaxValuePolicy)."]
    #[serde(rename = "evaluatorType", default, skip_serializing_if = "Option::is_none")]
    pub evaluator_type: Option<policy_properties::EvaluatorType>,
    #[doc = "The creation date of the policy."]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl PolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_properties {
    use super::*;
    #[doc = "The status of the policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The fact name of the policy (e.g. LabVmCount, LabVmSize, MaxVmsAllowedPerLab, etc."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FactName")]
    pub enum FactName {
        UserOwnedLabVmCount,
        UserOwnedLabPremiumVmCount,
        LabVmCount,
        LabPremiumVmCount,
        LabVmSize,
        GalleryImage,
        UserOwnedLabVmCountInSubnet,
        LabTargetCost,
        EnvironmentTemplate,
        ScheduleEditPermission,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FactName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FactName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FactName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::UserOwnedLabVmCount => serializer.serialize_unit_variant("FactName", 0u32, "UserOwnedLabVmCount"),
                Self::UserOwnedLabPremiumVmCount => serializer.serialize_unit_variant("FactName", 1u32, "UserOwnedLabPremiumVmCount"),
                Self::LabVmCount => serializer.serialize_unit_variant("FactName", 2u32, "LabVmCount"),
                Self::LabPremiumVmCount => serializer.serialize_unit_variant("FactName", 3u32, "LabPremiumVmCount"),
                Self::LabVmSize => serializer.serialize_unit_variant("FactName", 4u32, "LabVmSize"),
                Self::GalleryImage => serializer.serialize_unit_variant("FactName", 5u32, "GalleryImage"),
                Self::UserOwnedLabVmCountInSubnet => serializer.serialize_unit_variant("FactName", 6u32, "UserOwnedLabVmCountInSubnet"),
                Self::LabTargetCost => serializer.serialize_unit_variant("FactName", 7u32, "LabTargetCost"),
                Self::EnvironmentTemplate => serializer.serialize_unit_variant("FactName", 8u32, "EnvironmentTemplate"),
                Self::ScheduleEditPermission => serializer.serialize_unit_variant("FactName", 9u32, "ScheduleEditPermission"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The evaluator type of the policy (i.e. AllowedValuesPolicy, MaxValuePolicy)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EvaluatorType")]
    pub enum EvaluatorType {
        AllowedValuesPolicy,
        MaxValuePolicy,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EvaluatorType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EvaluatorType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EvaluatorType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AllowedValuesPolicy => serializer.serialize_unit_variant("EvaluatorType", 0u32, "AllowedValuesPolicy"),
                Self::MaxValuePolicy => serializer.serialize_unit_variant("EvaluatorType", 1u32, "MaxValuePolicy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a Policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyPropertiesFragment {}
impl PolicyPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of a policy set evaluation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicySetResult {
    #[doc = "A value indicating whether this policy set evaluation has discovered violations."]
    #[serde(rename = "hasError", default, skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    #[doc = "The list of policy violations."]
    #[serde(rename = "policyViolations", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_violations: Vec<PolicyViolation>,
}
impl PolicySetResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy violation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyViolation {
    #[doc = "The code of the policy violation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The message of the policy violation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl PolicyViolation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a network port."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Port {
    #[doc = "Protocol type of the port."]
    #[serde(rename = "transportProtocol", default, skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<port::TransportProtocol>,
    #[doc = "Backend port of the target virtual machine."]
    #[serde(rename = "backendPort", default, skip_serializing_if = "Option::is_none")]
    pub backend_port: Option<i32>,
}
impl Port {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod port {
    use super::*;
    #[doc = "Protocol type of the port."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TransportProtocol")]
    pub enum TransportProtocol {
        Tcp,
        Udp,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TransportProtocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TransportProtocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TransportProtocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Tcp => serializer.serialize_unit_variant("TransportProtocol", 0u32, "Tcp"),
                Self::Udp => serializer.serialize_unit_variant("TransportProtocol", 1u32, "Udp"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of a network port."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortFragment {}
impl PortFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list REST API operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderOperationResult {
    #[doc = "List of operations supported by the resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationMetadata>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProviderOperationResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProviderOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a .rdp file"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RdpConnection {
    #[doc = "The contents of the .rdp file"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
}
impl RdpConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request body for resizing a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResizeLabVirtualMachineProperties {
    #[doc = "Specifies the size of the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}
impl ResizeLabVirtualMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The identifier of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for retargeting a virtual machine schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetargetScheduleProperties {
    #[doc = "The resource Id of the virtual machine on which the schedule operates"]
    #[serde(rename = "currentResourceId", default, skip_serializing_if = "Option::is_none")]
    pub current_resource_id: Option<String>,
    #[doc = "The resource Id of the virtual machine that the schedule should be retargeted to"]
    #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
}
impl RetargetScheduleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a schedule."]
    pub properties: ScheduleProperties,
}
impl Schedule {
    pub fn new(properties: ScheduleProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "Properties for creating a schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleCreationParameter {
    #[doc = "Properties for schedule creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduleCreationParameterProperties>,
    #[doc = "The name of the virtual machine or environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The location of the new virtual machine or environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ScheduleCreationParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for creating a schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleCreationParameterFragment {}
impl ScheduleCreationParameterFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for schedule creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleCreationParameterProperties {
    #[doc = "The status of the schedule (i.e. Enabled, Disabled)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<schedule_creation_parameter_properties::Status>,
    #[doc = "The task type of the schedule (e.g. LabVmsShutdownTask, LabVmAutoStart)."]
    #[serde(rename = "taskType", default, skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    #[doc = "Properties of a weekly schedule."]
    #[serde(rename = "weeklyRecurrence", default, skip_serializing_if = "Option::is_none")]
    pub weekly_recurrence: Option<WeekDetails>,
    #[doc = "Properties of a daily schedule."]
    #[serde(rename = "dailyRecurrence", default, skip_serializing_if = "Option::is_none")]
    pub daily_recurrence: Option<DayDetails>,
    #[doc = "Properties of an hourly schedule."]
    #[serde(rename = "hourlyRecurrence", default, skip_serializing_if = "Option::is_none")]
    pub hourly_recurrence: Option<HourDetails>,
    #[doc = "The time zone ID (e.g. Pacific Standard time)."]
    #[serde(rename = "timeZoneId", default, skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    #[doc = "Notification settings for a schedule."]
    #[serde(rename = "notificationSettings", default, skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<NotificationSettings>,
    #[doc = "The resource ID to which the schedule belongs"]
    #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
}
impl ScheduleCreationParameterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod schedule_creation_parameter_properties {
    use super::*;
    #[doc = "The status of the schedule (i.e. Enabled, Disabled)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[doc = "Properties for schedule creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleCreationParameterPropertiesFragment {}
impl ScheduleCreationParameterPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleFragment {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl ScheduleFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Schedule>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScheduleList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScheduleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleProperties {
    #[doc = "The status of the schedule (i.e. Enabled, Disabled)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<schedule_properties::Status>,
    #[doc = "The task type of the schedule (e.g. LabVmsShutdownTask, LabVmAutoStart)."]
    #[serde(rename = "taskType", default, skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    #[doc = "Properties of a weekly schedule."]
    #[serde(rename = "weeklyRecurrence", default, skip_serializing_if = "Option::is_none")]
    pub weekly_recurrence: Option<WeekDetails>,
    #[doc = "Properties of a daily schedule."]
    #[serde(rename = "dailyRecurrence", default, skip_serializing_if = "Option::is_none")]
    pub daily_recurrence: Option<DayDetails>,
    #[doc = "Properties of an hourly schedule."]
    #[serde(rename = "hourlyRecurrence", default, skip_serializing_if = "Option::is_none")]
    pub hourly_recurrence: Option<HourDetails>,
    #[doc = "The time zone ID (e.g. Pacific Standard time)."]
    #[serde(rename = "timeZoneId", default, skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    #[doc = "Notification settings for a schedule."]
    #[serde(rename = "notificationSettings", default, skip_serializing_if = "Option::is_none")]
    pub notification_settings: Option<NotificationSettings>,
    #[doc = "The creation date of the schedule."]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The resource ID to which the schedule belongs"]
    #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl ScheduleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod schedule_properties {
    use super::*;
    #[doc = "The status of the schedule (i.e. Enabled, Disabled)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Status {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[doc = "Properties of a schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchedulePropertiesFragment {}
impl SchedulePropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Secret {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a secret."]
    pub properties: SecretProperties,
}
impl Secret {
    pub fn new(properties: SecretProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretFragment {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl SecretFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Secret>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SecretList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SecretList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretProperties {
    #[doc = "The value of the secret for secret creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl SecretProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretPropertiesFragment {}
impl SecretPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Service Fabric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceFabric {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a service fabric."]
    pub properties: ServiceFabricProperties,
}
impl ServiceFabric {
    pub fn new(properties: ServiceFabricProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A Service Fabric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceFabricFragment {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl ServiceFabricFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceFabricList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceFabric>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServiceFabricList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServiceFabricList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a service fabric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceFabricProperties {
    #[doc = "The backing service fabric resource's id"]
    #[serde(rename = "externalServiceFabricId", default, skip_serializing_if = "Option::is_none")]
    pub external_service_fabric_id: Option<String>,
    #[doc = "The resource id of the environment under which the service fabric resource is present"]
    #[serde(rename = "environmentId", default, skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[doc = "Schedules applicable to a virtual machine. The schedules may have been defined on a VM or on lab level."]
    #[serde(rename = "applicableSchedule", default, skip_serializing_if = "Option::is_none")]
    pub applicable_schedule: Option<ApplicableSchedule>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl ServiceFabricProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a service fabric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceFabricPropertiesFragment {}
impl ServiceFabricPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for a managed identity to execute DevTest lab services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceRunner {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a managed identity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
}
impl ServiceRunner {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceRunnerList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceRunner>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ServiceRunnerList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a virtual machine that determine how it is connected to a load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPublicIpAddressConfiguration {
    #[doc = "The incoming NAT rules"]
    #[serde(rename = "inboundNatRules", default, skip_serializing_if = "Vec::is_empty")]
    pub inbound_nat_rules: Vec<InboundNatRule>,
}
impl SharedPublicIpAddressConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a virtual machine that determine how it is connected to a load balancer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedPublicIpAddressConfigurationFragment {}
impl SharedPublicIpAddressConfigurationFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The contents of a shutdown notification. Webhooks can use this type to deserialize the request body when they get notified of an imminent shutdown."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShutdownNotificationContent {
    #[doc = "The URL to skip auto-shutdown."]
    #[serde(rename = "skipUrl", default, skip_serializing_if = "Option::is_none")]
    pub skip_url: Option<String>,
    #[doc = "The URL to delay shutdown by 60 minutes."]
    #[serde(rename = "delayUrl60", default, skip_serializing_if = "Option::is_none")]
    pub delay_url60: Option<String>,
    #[doc = "The URL to delay shutdown by 2 hours."]
    #[serde(rename = "delayUrl120", default, skip_serializing_if = "Option::is_none")]
    pub delay_url120: Option<String>,
    #[doc = "The virtual machine to be shut down."]
    #[serde(rename = "vmName", default, skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<String>,
    #[doc = "The GUID for the virtual machine to be shut down."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The owner of the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[doc = "The URL of the virtual machine."]
    #[serde(rename = "vmUrl", default, skip_serializing_if = "Option::is_none")]
    pub vm_url: Option<String>,
    #[doc = "Minutes remaining until shutdown"]
    #[serde(rename = "minutesUntilShutdown", default, skip_serializing_if = "Option::is_none")]
    pub minutes_until_shutdown: Option<String>,
    #[doc = "The event for which a notification will be sent."]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[doc = "The text for the notification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "The subscription ID for the schedule."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resource group name for the schedule."]
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[doc = "The lab for the schedule."]
    #[serde(rename = "labName", default, skip_serializing_if = "Option::is_none")]
    pub lab_name: Option<String>,
}
impl ShutdownNotificationContent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subnet information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Subnet {
    #[doc = "The resource ID of the subnet."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The name of the subnet as seen in the lab."]
    #[serde(rename = "labSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub lab_subnet_name: Option<String>,
    #[doc = "The permission policy of the subnet for allowing public IP addresses (i.e. Allow, Deny))."]
    #[serde(rename = "allowPublicIp", default, skip_serializing_if = "Option::is_none")]
    pub allow_public_ip: Option<subnet::AllowPublicIp>,
}
impl Subnet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subnet {
    use super::*;
    #[doc = "The permission policy of the subnet for allowing public IP addresses (i.e. Allow, Deny))."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AllowPublicIp")]
    pub enum AllowPublicIp {
        Default,
        Deny,
        Allow,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AllowPublicIp {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AllowPublicIp {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AllowPublicIp {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("AllowPublicIp", 0u32, "Default"),
                Self::Deny => serializer.serialize_unit_variant("AllowPublicIp", 1u32, "Deny"),
                Self::Allow => serializer.serialize_unit_variant("AllowPublicIp", 2u32, "Allow"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Subnet information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubnetFragment {}
impl SubnetFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Property overrides on a subnet of a virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubnetOverride {
    #[doc = "The resource ID of the subnet."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The name given to the subnet within the lab."]
    #[serde(rename = "labSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub lab_subnet_name: Option<String>,
    #[doc = "Indicates whether this subnet can be used during virtual machine creation (i.e. Allow, Deny)."]
    #[serde(rename = "useInVmCreationPermission", default, skip_serializing_if = "Option::is_none")]
    pub use_in_vm_creation_permission: Option<subnet_override::UseInVmCreationPermission>,
    #[doc = "Indicates whether public IP addresses can be assigned to virtual machines on this subnet (i.e. Allow, Deny)."]
    #[serde(rename = "usePublicIpAddressPermission", default, skip_serializing_if = "Option::is_none")]
    pub use_public_ip_address_permission: Option<subnet_override::UsePublicIpAddressPermission>,
    #[doc = "Configuration for public IP address sharing."]
    #[serde(rename = "sharedPublicIpAddressConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub shared_public_ip_address_configuration: Option<SubnetSharedPublicIpAddressConfiguration>,
    #[doc = "The virtual network pool associated with this subnet."]
    #[serde(rename = "virtualNetworkPoolName", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_pool_name: Option<String>,
}
impl SubnetOverride {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subnet_override {
    use super::*;
    #[doc = "Indicates whether this subnet can be used during virtual machine creation (i.e. Allow, Deny)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UseInVmCreationPermission")]
    pub enum UseInVmCreationPermission {
        Default,
        Deny,
        Allow,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UseInVmCreationPermission {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UseInVmCreationPermission {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UseInVmCreationPermission {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("UseInVmCreationPermission", 0u32, "Default"),
                Self::Deny => serializer.serialize_unit_variant("UseInVmCreationPermission", 1u32, "Deny"),
                Self::Allow => serializer.serialize_unit_variant("UseInVmCreationPermission", 2u32, "Allow"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Indicates whether public IP addresses can be assigned to virtual machines on this subnet (i.e. Allow, Deny)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UsePublicIpAddressPermission")]
    pub enum UsePublicIpAddressPermission {
        Default,
        Deny,
        Allow,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UsePublicIpAddressPermission {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UsePublicIpAddressPermission {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UsePublicIpAddressPermission {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("UsePublicIpAddressPermission", 0u32, "Default"),
                Self::Deny => serializer.serialize_unit_variant("UsePublicIpAddressPermission", 1u32, "Deny"),
                Self::Allow => serializer.serialize_unit_variant("UsePublicIpAddressPermission", 2u32, "Allow"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Property overrides on a subnet of a virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubnetOverrideFragment {}
impl SubnetOverrideFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration for public IP address sharing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubnetSharedPublicIpAddressConfiguration {
    #[doc = "Backend ports that virtual machines on this subnet are allowed to expose"]
    #[serde(rename = "allowedPorts", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_ports: Vec<Port>,
}
impl SubnetSharedPublicIpAddressConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration for public IP address sharing."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubnetSharedPublicIpAddressConfigurationFragment {}
impl SubnetSharedPublicIpAddressConfigurationFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a cost target."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetCostProperties {
    #[doc = "Target cost status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<target_cost_properties::Status>,
    #[doc = "Lab target cost"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<i32>,
    #[doc = "Cost thresholds."]
    #[serde(rename = "costThresholds", default, skip_serializing_if = "Vec::is_empty")]
    pub cost_thresholds: Vec<CostThresholdProperties>,
    #[doc = "Reporting cycle start date."]
    #[serde(rename = "cycleStartDateTime", with = "azure_core::date::rfc3339::option")]
    pub cycle_start_date_time: Option<time::OffsetDateTime>,
    #[doc = "Reporting cycle end date."]
    #[serde(rename = "cycleEndDateTime", with = "azure_core::date::rfc3339::option")]
    pub cycle_end_date_time: Option<time::OffsetDateTime>,
    #[doc = "Reporting cycle type."]
    #[serde(rename = "cycleType", default, skip_serializing_if = "Option::is_none")]
    pub cycle_type: Option<target_cost_properties::CycleType>,
}
impl TargetCostProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod target_cost_properties {
    use super::*;
    #[doc = "Target cost status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Enabled,
        Disabled,
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
                Self::Enabled => serializer.serialize_unit_variant("Status", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Reporting cycle type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CycleType")]
    pub enum CycleType {
        CalendarMonth,
        Custom,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CycleType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CycleType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CycleType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CalendarMonth => serializer.serialize_unit_variant("CycleType", 0u32, "CalendarMonth"),
                Self::Custom => serializer.serialize_unit_variant("CycleType", 1u32, "Custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents an update resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateResource {
    #[doc = "The tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl UpdateResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Profile of a lab user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct User {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a lab user profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserProperties>,
}
impl User {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Profile of a lab user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserFragment {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl UserFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity attributes of a lab user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserIdentity {
    #[doc = "Set to the principal name / UPN of the client JWT making the request."]
    #[serde(rename = "principalName", default, skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
    #[doc = "Set to the principal Id of the client JWT making the request. Service principal will not have the principal Id."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Set to the tenant ID of the client JWT making the request."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Set to the object Id of the client JWT making the request. Not all users have object Id. For CSP (reseller) scenarios for example, object Id is not available."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Set to the app Id of the client JWT making the request."]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}
impl UserIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity attributes of a lab user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserIdentityFragment {}
impl UserIdentityFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<User>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UserList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UserList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a lab user profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserProperties {
    #[doc = "Identity attributes of a lab user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<UserIdentity>,
    #[doc = "Properties of a user's secret store."]
    #[serde(rename = "secretStore", default, skip_serializing_if = "Option::is_none")]
    pub secret_store: Option<UserSecretStore>,
    #[doc = "The creation date of the user profile."]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl UserProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a lab user profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserPropertiesFragment {}
impl UserPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a user's secret store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserSecretStore {
    #[doc = "The URI of the user's Key vault."]
    #[serde(rename = "keyVaultUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
    #[doc = "The ID of the user's Key vault."]
    #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_id: Option<String>,
}
impl UserSecretStore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a user's secret store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserSecretStoreFragment {}
impl UserSecretStoreFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetwork {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a virtual network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualNetworkProperties>,
}
impl VirtualNetwork {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkFragment {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl VirtualNetworkFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkList {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetwork>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualNetworkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkProperties {
    #[doc = "The allowed subnets of the virtual network."]
    #[serde(rename = "allowedSubnets", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_subnets: Vec<Subnet>,
    #[doc = "The description of the virtual network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Microsoft.Network resource identifier of the virtual network."]
    #[serde(rename = "externalProviderResourceId", default, skip_serializing_if = "Option::is_none")]
    pub external_provider_resource_id: Option<String>,
    #[doc = "The external subnet properties."]
    #[serde(rename = "externalSubnets", default, skip_serializing_if = "Vec::is_empty")]
    pub external_subnets: Vec<ExternalSubnet>,
    #[doc = "The subnet overrides of the virtual network."]
    #[serde(rename = "subnetOverrides", default, skip_serializing_if = "Vec::is_empty")]
    pub subnet_overrides: Vec<SubnetOverride>,
    #[doc = "The creation date of the virtual network."]
    #[serde(rename = "createdDate", with = "azure_core::date::rfc3339::option")]
    pub created_date: Option<time::OffsetDateTime>,
    #[doc = "The provisioning status of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique immutable identifier of a resource (Guid)."]
    #[serde(rename = "uniqueIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}
impl VirtualNetworkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkPropertiesFragment {}
impl VirtualNetworkPropertiesFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a weekly schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WeekDetails {
    #[doc = "The days of the week for which the schedule is set (e.g. Sunday, Monday, Tuesday, etc.)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weekdays: Vec<String>,
    #[doc = "The time of the day the schedule will occur."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}
impl WeekDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a weekly schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WeekDetailsFragment {}
impl WeekDetailsFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a Windows OS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsOsInfo {
    #[doc = "The state of the Windows OS (i.e. NonSysprepped, SysprepRequested, SysprepApplied)."]
    #[serde(rename = "windowsOsState", default, skip_serializing_if = "Option::is_none")]
    pub windows_os_state: Option<windows_os_info::WindowsOsState>,
}
impl WindowsOsInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod windows_os_info {
    use super::*;
    #[doc = "The state of the Windows OS (i.e. NonSysprepped, SysprepRequested, SysprepApplied)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "WindowsOsState")]
    pub enum WindowsOsState {
        NonSysprepped,
        SysprepRequested,
        SysprepApplied,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for WindowsOsState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for WindowsOsState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for WindowsOsState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NonSysprepped => serializer.serialize_unit_variant("WindowsOsState", 0u32, "NonSysprepped"),
                Self::SysprepRequested => serializer.serialize_unit_variant("WindowsOsState", 1u32, "SysprepRequested"),
                Self::SysprepApplied => serializer.serialize_unit_variant("WindowsOsState", 2u32, "SysprepApplied"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information about a Windows OS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsOsInfoFragment {}
impl WindowsOsInfoFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
