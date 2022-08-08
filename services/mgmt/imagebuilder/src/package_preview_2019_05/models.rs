#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Api error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiError {
    #[doc = "The Api error details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ApiErrorBase>,
    #[doc = "Inner error details."]
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<InnerError>,
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The target of the particular error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl azure_core::Continuable for ApiError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ApiError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Api error base."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiErrorBase {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The target of the particular error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ApiErrorBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image template is an ARM resource managed by Microsoft.VirtualMachineImages provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplate {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of an image template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImageTemplateProperties>,
    #[doc = "Identity for the image template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ImageTemplateIdentity>,
}
impl ImageTemplate {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "Describes a unit of image customization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateCustomizer {
    #[doc = "The type of customization tool you want to use on the Image. For example, \"Shell\" can be shell customizer"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Friendly Name to provide context on what this customization step does"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ImageTemplateCustomizer {
    pub fn new(type_: String) -> Self {
        Self { type_, name: None }
    }
}
#[doc = "Generic distribution object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateDistributor {
    #[doc = "Type of distribution."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The name to be used for the associated RunOutput."]
    #[serde(rename = "runOutputName")]
    pub run_output_name: String,
    #[doc = "Tags that will be applied to the artifact once it has been created/updated by the distributor."]
    #[serde(rename = "artifactTags", default, skip_serializing_if = "Option::is_none")]
    pub artifact_tags: Option<serde_json::Value>,
}
impl ImageTemplateDistributor {
    pub fn new(type_: String, run_output_name: String) -> Self {
        Self {
            type_,
            run_output_name,
            artifact_tags: None,
        }
    }
}
#[doc = "Uploads files to VMs (Linux, Windows). Corresponds to Packer file provisioner"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateFileCustomizer {
    #[serde(flatten)]
    pub image_template_customizer: ImageTemplateCustomizer,
    #[doc = "The URI of the file to be uploaded for customizing the VM. It can be a github link, SAS URI for Azure Storage, etc"]
    #[serde(rename = "sourceUri", default, skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[doc = "SHA256 checksum of the file provided in the sourceUri field above"]
    #[serde(rename = "sha256Checksum", default, skip_serializing_if = "Option::is_none")]
    pub sha256_checksum: Option<String>,
    #[doc = "The absolute path to a file (with nested directory structures already created) where the file (from sourceUri) will be uploaded to in the VM"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}
impl ImageTemplateFileCustomizer {
    pub fn new(image_template_customizer: ImageTemplateCustomizer) -> Self {
        Self {
            image_template_customizer,
            source_uri: None,
            sha256_checksum: None,
            destination: None,
        }
    }
}
#[doc = "Identity for the image template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageTemplateIdentity {
    #[doc = "The type of identity used for the image template. The type 'None' will remove any identities from the image template."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<image_template_identity::Type>,
    #[doc = "The list of user identities associated with the image template. The user identity dictionary key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ImageTemplateIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod image_template_identity {
    use super::*;
    #[doc = "The type of identity used for the image template. The type 'None' will remove any identities from the image template."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        UserAssigned,
        None,
    }
}
#[doc = "Describes an image source that is an installation ISO. Currently only supports Red Hat Enterprise Linux 7.2-7.5 ISO's."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateIsoSource {
    #[serde(flatten)]
    pub image_template_source: ImageTemplateSource,
    #[doc = "URI to get the ISO image. This URI has to be accessible to the resource provider at the time of the image template creation."]
    #[serde(rename = "sourceUri")]
    pub source_uri: String,
    #[doc = "SHA256 Checksum of the ISO image."]
    #[serde(rename = "sha256Checksum")]
    pub sha256_checksum: String,
}
impl ImageTemplateIsoSource {
    pub fn new(image_template_source: ImageTemplateSource, source_uri: String, sha256_checksum: String) -> Self {
        Self {
            image_template_source,
            source_uri,
            sha256_checksum,
        }
    }
}
#[doc = "Describes the latest status of running an image template"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageTemplateLastRunStatus {
    #[doc = "Start time of the last run (UTC)"]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the last run (UTC)"]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "State of the last run"]
    #[serde(rename = "runState", default, skip_serializing_if = "Option::is_none")]
    pub run_state: Option<image_template_last_run_status::RunState>,
    #[doc = "Sub-state of the last run"]
    #[serde(rename = "runSubState", default, skip_serializing_if = "Option::is_none")]
    pub run_sub_state: Option<image_template_last_run_status::RunSubState>,
    #[doc = "Verbose information about the last run state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ImageTemplateLastRunStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod image_template_last_run_status {
    use super::*;
    #[doc = "State of the last run"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RunState {
        Running,
        Succeeded,
        PartiallySucceeded,
        Failed,
    }
    #[doc = "Sub-state of the last run"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RunSubState {
        Queued,
        Building,
        Customizing,
        Distributing,
    }
}
#[doc = "The result of List image templates operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageTemplateListResult {
    #[doc = "An array of image templates"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ImageTemplate>,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImageTemplateListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ImageTemplateListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Distribute as a Managed Disk Image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateManagedImageDistributor {
    #[serde(flatten)]
    pub image_template_distributor: ImageTemplateDistributor,
    #[doc = "Resource Id of the Managed Disk Image"]
    #[serde(rename = "imageId")]
    pub image_id: String,
    #[doc = "Azure location for the image, should match if image already exists"]
    pub location: String,
}
impl ImageTemplateManagedImageDistributor {
    pub fn new(image_template_distributor: ImageTemplateDistributor, image_id: String, location: String) -> Self {
        Self {
            image_template_distributor,
            image_id,
            location,
        }
    }
}
#[doc = "Describes an image source that is a managed image in customer subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateManagedImageSource {
    #[serde(flatten)]
    pub image_template_source: ImageTemplateSource,
    #[doc = "ARM resource id of the managed image in customer subscription"]
    #[serde(rename = "imageId")]
    pub image_id: String,
}
impl ImageTemplateManagedImageSource {
    pub fn new(image_template_source: ImageTemplateSource, image_id: String) -> Self {
        Self {
            image_template_source,
            image_id,
        }
    }
}
#[doc = "Describes an image source from [Azure Gallery Images](https://docs.microsoft.com/en-us/rest/api/compute/virtualmachineimages)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplatePlatformImageSource {
    #[serde(flatten)]
    pub image_template_source: ImageTemplateSource,
    #[doc = "Image Publisher in [Azure Gallery Images](https://docs.microsoft.com/en-us/rest/api/compute/virtualmachineimages)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Image offer from the [Azure Gallery Images](https://docs.microsoft.com/en-us/rest/api/compute/virtualmachineimages)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "Image sku from the [Azure Gallery Images](https://docs.microsoft.com/en-us/rest/api/compute/virtualmachineimages)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "Image version from the [Azure Gallery Images](https://docs.microsoft.com/en-us/rest/api/compute/virtualmachineimages)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ImageTemplatePlatformImageSource {
    pub fn new(image_template_source: ImageTemplateSource) -> Self {
        Self {
            image_template_source,
            publisher: None,
            offer: None,
            sku: None,
            version: None,
        }
    }
}
#[doc = "Runs the specified PowerShell on the VM (Windows). Corresponds to Packer powershell provisioner. Exactly one of 'scriptUri' or 'inline' can be specified."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplatePowerShellCustomizer {
    #[serde(flatten)]
    pub image_template_customizer: ImageTemplateCustomizer,
    #[doc = "URI of the PowerShell script to be run for customizing. It can be a github link, SAS URI for Azure Storage, etc"]
    #[serde(rename = "scriptUri", default, skip_serializing_if = "Option::is_none")]
    pub script_uri: Option<String>,
    #[doc = "SHA256 checksum of the power shell script provided in the scriptUri field above"]
    #[serde(rename = "sha256Checksum", default, skip_serializing_if = "Option::is_none")]
    pub sha256_checksum: Option<String>,
    #[doc = "Array of PowerShell commands to execute"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inline: Vec<String>,
    #[doc = "If specified, the PowerShell script will be run with elevated privileges"]
    #[serde(rename = "runElevated", default, skip_serializing_if = "Option::is_none")]
    pub run_elevated: Option<bool>,
    #[doc = "Valid exit codes for the PowerShell script. [Default: 0]"]
    #[serde(rename = "validExitCodes", default, skip_serializing_if = "Vec::is_empty")]
    pub valid_exit_codes: Vec<i64>,
}
impl ImageTemplatePowerShellCustomizer {
    pub fn new(image_template_customizer: ImageTemplateCustomizer) -> Self {
        Self {
            image_template_customizer,
            script_uri: None,
            sha256_checksum: None,
            inline: Vec::new(),
            run_elevated: None,
            valid_exit_codes: Vec::new(),
        }
    }
}
#[doc = "Describes the properties of an image template"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateProperties {
    #[doc = "Describes a virtual machine image source for building, customizing and distributing"]
    pub source: ImageTemplateSource,
    #[doc = "Specifies the properties used to describe the customization steps of the image, like Image source etc"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub customize: Vec<ImageTemplateCustomizer>,
    #[doc = "The distribution targets where the image output needs to go to."]
    pub distribute: Vec<ImageTemplateDistributor>,
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Describes the error happened when create or update an image template"]
    #[serde(rename = "provisioningError", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_error: Option<ProvisioningError>,
    #[doc = "Describes the latest status of running an image template"]
    #[serde(rename = "lastRunStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_run_status: Option<ImageTemplateLastRunStatus>,
    #[doc = "Maximum duration to wait while building the image template. Omit or specify 0 to use the default (4 hours)."]
    #[serde(rename = "buildTimeoutInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub build_timeout_in_minutes: Option<i64>,
    #[doc = "Describes the virtual machine used to build, customize and capture images"]
    #[serde(rename = "vmProfile", default, skip_serializing_if = "Option::is_none")]
    pub vm_profile: Option<ImageTemplateVmProfile>,
}
impl ImageTemplateProperties {
    pub fn new(source: ImageTemplateSource, distribute: Vec<ImageTemplateDistributor>) -> Self {
        Self {
            source,
            customize: Vec::new(),
            distribute,
            provisioning_state: None,
            provisioning_error: None,
            last_run_status: None,
            build_timeout_in_minutes: None,
            vm_profile: None,
        }
    }
}
#[doc = "Reboots a VM and waits for it to come back online (Windows). Corresponds to Packer windows-restart provisioner"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateRestartCustomizer {
    #[serde(flatten)]
    pub image_template_customizer: ImageTemplateCustomizer,
    #[doc = "Command to execute the restart [Default: 'shutdown /r /f /t 0 /c \"packer restart\"']"]
    #[serde(rename = "restartCommand", default, skip_serializing_if = "Option::is_none")]
    pub restart_command: Option<String>,
    #[doc = "Command to check if restart succeeded [Default: '']"]
    #[serde(rename = "restartCheckCommand", default, skip_serializing_if = "Option::is_none")]
    pub restart_check_command: Option<String>,
    #[doc = "Restart timeout specified as a string of magnitude and unit, e.g. '5m' (5 minutes) or '2h' (2 hours) [Default: '5m']"]
    #[serde(rename = "restartTimeout", default, skip_serializing_if = "Option::is_none")]
    pub restart_timeout: Option<String>,
}
impl ImageTemplateRestartCustomizer {
    pub fn new(image_template_customizer: ImageTemplateCustomizer) -> Self {
        Self {
            image_template_customizer,
            restart_command: None,
            restart_check_command: None,
            restart_timeout: None,
        }
    }
}
#[doc = "Distribute via Shared Image Gallery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateSharedImageDistributor {
    #[serde(flatten)]
    pub image_template_distributor: ImageTemplateDistributor,
    #[doc = "Resource Id of the Shared Image Gallery image"]
    #[serde(rename = "galleryImageId")]
    pub gallery_image_id: String,
    #[doc = "A list of regions that the image will be replicated to"]
    #[serde(rename = "replicationRegions")]
    pub replication_regions: Vec<String>,
}
impl ImageTemplateSharedImageDistributor {
    pub fn new(image_template_distributor: ImageTemplateDistributor, gallery_image_id: String, replication_regions: Vec<String>) -> Self {
        Self {
            image_template_distributor,
            gallery_image_id,
            replication_regions,
        }
    }
}
#[doc = "Describes an image source that is an image version in a shared image gallery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateSharedImageVersionSource {
    #[serde(flatten)]
    pub image_template_source: ImageTemplateSource,
    #[doc = "ARM resource id of the image version in the shared image gallery"]
    #[serde(rename = "imageVersionId")]
    pub image_version_id: String,
}
impl ImageTemplateSharedImageVersionSource {
    pub fn new(image_template_source: ImageTemplateSource, image_version_id: String) -> Self {
        Self {
            image_template_source,
            image_version_id,
        }
    }
}
#[doc = "Runs a shell script during the customization phase (Linux). Corresponds to Packer shell provisioner. Exactly one of 'scriptUri' or 'inline' can be specified."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateShellCustomizer {
    #[serde(flatten)]
    pub image_template_customizer: ImageTemplateCustomizer,
    #[doc = "URI of the shell script to be run for customizing. It can be a github link, SAS URI for Azure Storage, etc"]
    #[serde(rename = "scriptUri", default, skip_serializing_if = "Option::is_none")]
    pub script_uri: Option<String>,
    #[doc = "SHA256 checksum of the shell script provided in the scriptUri field"]
    #[serde(rename = "sha256Checksum", default, skip_serializing_if = "Option::is_none")]
    pub sha256_checksum: Option<String>,
    #[doc = "Array of shell commands to execute"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inline: Vec<String>,
}
impl ImageTemplateShellCustomizer {
    pub fn new(image_template_customizer: ImageTemplateCustomizer) -> Self {
        Self {
            image_template_customizer,
            script_uri: None,
            sha256_checksum: None,
            inline: Vec::new(),
        }
    }
}
#[doc = "Describes a virtual machine image source for building, customizing and distributing"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateSource {
    #[doc = "Specifies the type of source image you want to start with."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl ImageTemplateSource {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "Parameters for updating an image template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageTemplateUpdateParameters {
    #[doc = "Identity for the image template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ImageTemplateIdentity>,
    #[doc = "The user-specified tags associated with the image template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ImageTemplateUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Distribute via VHD in a storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateVhdDistributor {
    #[serde(flatten)]
    pub image_template_distributor: ImageTemplateDistributor,
}
impl ImageTemplateVhdDistributor {
    pub fn new(image_template_distributor: ImageTemplateDistributor) -> Self {
        Self {
            image_template_distributor,
        }
    }
}
#[doc = "Describes the virtual machine used to build, customize and capture images"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageTemplateVmProfile {
    #[doc = "Size of the virtual machine used to build, customize and capture images. Omit or specify empty string to use the default (Standard_D1_v2)."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
}
impl ImageTemplateVmProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Inner error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[doc = "The exception type."]
    #[serde(rename = "exceptionType", default, skip_serializing_if = "Option::is_none")]
    pub exception_type: Option<String>,
    #[doc = "The internal error message or exception dump."]
    #[serde(rename = "errorDetail", default, skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<String>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "This is of the format {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "For example: read, write, delete, or listKeys/action"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the error happened when create or update an image template"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProvisioningError {
    #[doc = "Error code of the provisioning failure"]
    #[serde(rename = "provisioningErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_error_code: Option<provisioning_error::ProvisioningErrorCode>,
    #[doc = "Verbose error message about the provisioning failure"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ProvisioningError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod provisioning_error {
    use super::*;
    #[doc = "Error code of the provisioning failure"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningErrorCode")]
    pub enum ProvisioningErrorCode {
        BadSourceType,
        #[serde(rename = "BadPIRSource")]
        BadPirSource,
        #[serde(rename = "BadISOSource")]
        BadIsoSource,
        BadManagedImageSource,
        BadSharedImageVersionSource,
        BadCustomizerType,
        UnsupportedCustomizerType,
        NoCustomizerScript,
        BadDistributeType,
        BadSharedImageDistribute,
        ServerError,
        Other,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningErrorCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningErrorCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningErrorCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BadSourceType => serializer.serialize_unit_variant("ProvisioningErrorCode", 0u32, "BadSourceType"),
                Self::BadPirSource => serializer.serialize_unit_variant("ProvisioningErrorCode", 1u32, "BadPIRSource"),
                Self::BadIsoSource => serializer.serialize_unit_variant("ProvisioningErrorCode", 2u32, "BadISOSource"),
                Self::BadManagedImageSource => serializer.serialize_unit_variant("ProvisioningErrorCode", 3u32, "BadManagedImageSource"),
                Self::BadSharedImageVersionSource => {
                    serializer.serialize_unit_variant("ProvisioningErrorCode", 4u32, "BadSharedImageVersionSource")
                }
                Self::BadCustomizerType => serializer.serialize_unit_variant("ProvisioningErrorCode", 5u32, "BadCustomizerType"),
                Self::UnsupportedCustomizerType => {
                    serializer.serialize_unit_variant("ProvisioningErrorCode", 6u32, "UnsupportedCustomizerType")
                }
                Self::NoCustomizerScript => serializer.serialize_unit_variant("ProvisioningErrorCode", 7u32, "NoCustomizerScript"),
                Self::BadDistributeType => serializer.serialize_unit_variant("ProvisioningErrorCode", 8u32, "BadDistributeType"),
                Self::BadSharedImageDistribute => {
                    serializer.serialize_unit_variant("ProvisioningErrorCode", 9u32, "BadSharedImageDistribute")
                }
                Self::ServerError => serializer.serialize_unit_variant("ProvisioningErrorCode", 10u32, "ServerError"),
                Self::Other => serializer.serialize_unit_variant("ProvisioningErrorCode", 11u32, "Other"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Provisioning state of the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Creating,
    Updating,
    Succeeded,
    Failed,
    Deleting,
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
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
#[doc = "Represents an output that was created by running an image template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunOutput {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Describes the properties of a run output"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunOutputProperties>,
}
impl RunOutput {
    pub fn new(sub_resource: SubResource) -> Self {
        Self {
            sub_resource,
            properties: None,
        }
    }
}
#[doc = "The result of List run outputs operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunOutputCollection {
    #[doc = "An array of run outputs"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RunOutput>,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RunOutputCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RunOutputCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a run output"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunOutputProperties {
    #[doc = "The resource id of the artifact."]
    #[serde(rename = "artifactId", default, skip_serializing_if = "Option::is_none")]
    pub artifact_id: Option<String>,
    #[doc = "The location URI of the artifact."]
    #[serde(rename = "artifactUri", default, skip_serializing_if = "Option::is_none")]
    pub artifact_uri: Option<String>,
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl RunOutputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Sub Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubResource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    pub name: String,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl SubResource {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            name,
            type_: None,
        }
    }
}
