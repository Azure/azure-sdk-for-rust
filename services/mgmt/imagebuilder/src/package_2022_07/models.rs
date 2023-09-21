#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An error response from the Azure VM Image Builder service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the Azure VM Image Builder service."]
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
#[doc = "An error response from the Azure VM Image Builder service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes how to generate new x.y.z version number for distribution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributeVersioner {
    #[doc = "Version numbering scheme to be used."]
    pub scheme: String,
}
impl DistributeVersioner {
    pub fn new(scheme: String) -> Self {
        Self { scheme }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "scheme")]
pub enum DistributeVersionerUnion {
    Latest(DistributeVersionerLatest),
    Source(DistributeVersionerSource),
}
#[doc = "Generates version number that will be latest based on existing version numbers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributeVersionerLatest {
    #[serde(flatten)]
    pub distribute_versioner: DistributeVersioner,
    #[doc = "Major version for the generated version number. Determine what is \"latest\" based on versions with this value as the major version. -1 is equivalent to leaving it unset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub major: Option<i32>,
}
impl DistributeVersionerLatest {
    pub fn new(distribute_versioner: DistributeVersioner) -> Self {
        Self {
            distribute_versioner,
            major: None,
        }
    }
}
#[doc = "Generates version number based on version number of source image"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributeVersionerSource {
    #[serde(flatten)]
    pub distribute_versioner: DistributeVersioner,
}
impl DistributeVersionerSource {
    pub fn new(distribute_versioner: DistributeVersioner) -> Self {
        Self { distribute_versioner }
    }
}
#[doc = "Image template is an ARM resource managed by Microsoft.VirtualMachineImages provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplate {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describes the properties of an image template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImageTemplateProperties>,
    #[doc = "Identity for the image template."]
    pub identity: ImageTemplateIdentity,
}
impl ImageTemplate {
    pub fn new(tracked_resource: TrackedResource, identity: ImageTemplateIdentity) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ImageTemplateCustomizerUnion {
    File(ImageTemplateFileCustomizer),
    PowerShell(ImageTemplatePowerShellCustomizer),
    WindowsRestart(ImageTemplateRestartCustomizer),
    Shell(ImageTemplateShellCustomizer),
    WindowsUpdate(ImageTemplateWindowsUpdateCustomizer),
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ImageTemplateDistributorUnion {
    ManagedImage(ImageTemplateManagedImageDistributor),
    SharedImage(ImageTemplateSharedImageDistributor),
    #[serde(rename = "VHD")]
    Vhd(ImageTemplateVhdDistributor),
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
#[doc = "Uploads files required for validation to VMs (Linux, Windows). Corresponds to Packer file provisioner"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateFileValidator {
    #[serde(flatten)]
    pub image_template_in_vm_validator: ImageTemplateInVmValidator,
    #[doc = "The URI of the file to be uploaded to the VM for validation. It can be a github link, Azure Storage URI (authorized or SAS), etc"]
    #[serde(rename = "sourceUri", default, skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[doc = "SHA256 checksum of the file provided in the sourceUri field above"]
    #[serde(rename = "sha256Checksum", default, skip_serializing_if = "Option::is_none")]
    pub sha256_checksum: Option<String>,
    #[doc = "The absolute path to a file (with nested directory structures already created) where the file (from sourceUri) will be uploaded to in the VM"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}
impl ImageTemplateFileValidator {
    pub fn new(image_template_in_vm_validator: ImageTemplateInVmValidator) -> Self {
        Self {
            image_template_in_vm_validator,
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
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
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
#[doc = "Describes a unit of in-VM validation of image"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateInVmValidator {
    #[doc = "The type of validation you want to use on the Image. For example, \"Shell\" can be shell validation"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Friendly Name to provide context on what this validation step does"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ImageTemplateInVmValidator {
    pub fn new(type_: String) -> Self {
        Self { type_, name: None }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ImageTemplateInVmValidatorUnion {
    File(ImageTemplateFileValidator),
    PowerShell(ImageTemplatePowerShellValidator),
    Shell(ImageTemplateShellValidator),
}
#[doc = "Describes the latest status of running an image template"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageTemplateLastRunStatus {
    #[doc = "Start time of the last run (UTC)"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the last run (UTC)"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
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
        Canceling,
        Succeeded,
        PartiallySucceeded,
        Failed,
        Canceled,
    }
    #[doc = "Sub-state of the last run"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RunSubState {
        Queued,
        Building,
        Customizing,
        Optimizing,
        Validating,
        Distributing,
    }
}
#[doc = "The result of List image templates operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageTemplateListResult {
    #[doc = "An array of image templates"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ImageTemplate>,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImageTemplateListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
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
#[doc = "Describes an image source that is a managed image in customer subscription. This image must reside in the same subscription and region as the Image Builder template."]
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
    #[doc = "Image version from the [Azure Gallery Images](https://docs.microsoft.com/en-us/rest/api/compute/virtualmachineimages). If 'latest' is specified here, the version is evaluated when the image build takes place, not when the template is submitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Image version from the [Azure Gallery Images](https://docs.microsoft.com/en-us/rest/api/compute/virtualmachineimages). This readonly field differs from 'version', only if the value specified in 'version' field is 'latest'."]
    #[serde(rename = "exactVersion", default, skip_serializing_if = "Option::is_none")]
    pub exact_version: Option<String>,
    #[doc = "Purchase plan configuration for platform image."]
    #[serde(rename = "planInfo", default, skip_serializing_if = "Option::is_none")]
    pub plan_info: Option<PlatformImagePurchasePlan>,
}
impl ImageTemplatePlatformImageSource {
    pub fn new(image_template_source: ImageTemplateSource) -> Self {
        Self {
            image_template_source,
            publisher: None,
            offer: None,
            sku: None,
            version: None,
            exact_version: None,
            plan_info: None,
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inline: Vec<String>,
    #[doc = "If specified, the PowerShell script will be run with elevated privileges"]
    #[serde(rename = "runElevated", default, skip_serializing_if = "Option::is_none")]
    pub run_elevated: Option<bool>,
    #[doc = "If specified, the PowerShell script will be run with elevated privileges using the Local System user. Can only be true when the runElevated field above is set to true."]
    #[serde(rename = "runAsSystem", default, skip_serializing_if = "Option::is_none")]
    pub run_as_system: Option<bool>,
    #[doc = "Valid exit codes for the PowerShell script. [Default: 0]"]
    #[serde(
        rename = "validExitCodes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub valid_exit_codes: Vec<i32>,
}
impl ImageTemplatePowerShellCustomizer {
    pub fn new(image_template_customizer: ImageTemplateCustomizer) -> Self {
        Self {
            image_template_customizer,
            script_uri: None,
            sha256_checksum: None,
            inline: Vec::new(),
            run_elevated: None,
            run_as_system: None,
            valid_exit_codes: Vec::new(),
        }
    }
}
#[doc = "Runs the specified PowerShell script during the validation phase (Windows). Corresponds to Packer powershell provisioner. Exactly one of 'scriptUri' or 'inline' can be specified."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplatePowerShellValidator {
    #[serde(flatten)]
    pub image_template_in_vm_validator: ImageTemplateInVmValidator,
    #[doc = "URI of the PowerShell script to be run for validation. It can be a github link, Azure Storage URI, etc"]
    #[serde(rename = "scriptUri", default, skip_serializing_if = "Option::is_none")]
    pub script_uri: Option<String>,
    #[doc = "SHA256 checksum of the power shell script provided in the scriptUri field above"]
    #[serde(rename = "sha256Checksum", default, skip_serializing_if = "Option::is_none")]
    pub sha256_checksum: Option<String>,
    #[doc = "Array of PowerShell commands to execute"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inline: Vec<String>,
    #[doc = "If specified, the PowerShell script will be run with elevated privileges"]
    #[serde(rename = "runElevated", default, skip_serializing_if = "Option::is_none")]
    pub run_elevated: Option<bool>,
    #[doc = "If specified, the PowerShell script will be run with elevated privileges using the Local System user. Can only be true when the runElevated field above is set to true."]
    #[serde(rename = "runAsSystem", default, skip_serializing_if = "Option::is_none")]
    pub run_as_system: Option<bool>,
    #[doc = "Valid exit codes for the PowerShell script. [Default: 0]"]
    #[serde(
        rename = "validExitCodes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub valid_exit_codes: Vec<i32>,
}
impl ImageTemplatePowerShellValidator {
    pub fn new(image_template_in_vm_validator: ImageTemplateInVmValidator) -> Self {
        Self {
            image_template_in_vm_validator,
            script_uri: None,
            sha256_checksum: None,
            inline: Vec::new(),
            run_elevated: None,
            run_as_system: None,
            valid_exit_codes: Vec::new(),
        }
    }
}
#[doc = "Describes the properties of an image template"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateProperties {
    #[doc = "Describes a virtual machine image source for building, customizing and distributing"]
    pub source: ImageTemplateSourceUnion,
    #[doc = "Specifies the properties used to describe the customization steps of the image, like Image source etc"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub customize: Vec<ImageTemplateCustomizerUnion>,
    #[doc = "Specifies optimization to be performed on image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optimize: Option<image_template_properties::Optimize>,
    #[doc = "Configuration options and list of validations to be performed on the resulting image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validate: Option<image_template_properties::Validate>,
    #[doc = "The distribution targets where the image output needs to go to."]
    pub distribute: Vec<ImageTemplateDistributorUnion>,
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Describes the error happened when create or update an image template"]
    #[serde(rename = "provisioningError", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_error: Option<ProvisioningError>,
    #[doc = "Describes the latest status of running an image template"]
    #[serde(rename = "lastRunStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_run_status: Option<ImageTemplateLastRunStatus>,
    #[doc = "Maximum duration to wait while building the image template (includes all customizations, optimization, validations, and distributions). Omit or specify 0 to use the default (4 hours)."]
    #[serde(rename = "buildTimeoutInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub build_timeout_in_minutes: Option<i32>,
    #[doc = "Describes the virtual machines used to build and validate images"]
    #[serde(rename = "vmProfile", default, skip_serializing_if = "Option::is_none")]
    pub vm_profile: Option<ImageTemplateVmProfile>,
    #[doc = "The staging resource group id in the same subscription as the image template that will be used to build the image. If this field is empty, a resource group with a random name will be created. If the resource group specified in this field doesn't exist, it will be created with the same name. If the resource group specified exists, it must be empty and in the same region as the image template. The resource group created will be deleted during template deletion if this field is empty or the resource group specified doesn't exist, but if the resource group specified exists the resources created in the resource group will be deleted during template deletion and the resource group itself will remain."]
    #[serde(rename = "stagingResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub staging_resource_group: Option<String>,
    #[doc = "The staging resource group id in the same subscription as the image template that will be used to build the image. This read-only field differs from 'stagingResourceGroup' only if the value specified in the 'stagingResourceGroup' field is empty."]
    #[serde(rename = "exactStagingResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub exact_staging_resource_group: Option<String>,
}
impl ImageTemplateProperties {
    pub fn new(source: ImageTemplateSourceUnion, distribute: Vec<ImageTemplateDistributorUnion>) -> Self {
        Self {
            source,
            customize: Vec::new(),
            optimize: None,
            validate: None,
            distribute,
            provisioning_state: None,
            provisioning_error: None,
            last_run_status: None,
            build_timeout_in_minutes: None,
            vm_profile: None,
            staging_resource_group: None,
            exact_staging_resource_group: None,
        }
    }
}
pub mod image_template_properties {
    use super::*;
    #[doc = "Specifies optimization to be performed on image."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Optimize {
        #[doc = "Optimization is applied on the image for a faster VM boot."]
        #[serde(rename = "vmBoot", default, skip_serializing_if = "Option::is_none")]
        pub vm_boot: Option<optimize::VmBoot>,
    }
    impl Optimize {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod optimize {
        use super::*;
        #[doc = "Optimization is applied on the image for a faster VM boot."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct VmBoot {
            #[doc = "Enabling this field will improve VM boot time by optimizing the final customized image output."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub state: Option<vm_boot::State>,
        }
        impl VmBoot {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod vm_boot {
            use super::*;
            #[doc = "Enabling this field will improve VM boot time by optimizing the final customized image output."]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            pub enum State {
                Enabled,
                Disabled,
            }
        }
    }
    #[doc = "Configuration options and list of validations to be performed on the resulting image."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Validate {
        #[doc = "If validation fails and this field is set to false, output image(s) will not be distributed. This is the default behavior. If validation fails and this field is set to true, output image(s) will still be distributed. Please use this option with caution as it may result in bad images being distributed for use. In either case (true or false), the end to end image run will be reported as having failed in case of a validation failure. [Note: This field has no effect if validation succeeds.]"]
        #[serde(rename = "continueDistributeOnFailure", default, skip_serializing_if = "Option::is_none")]
        pub continue_distribute_on_failure: Option<bool>,
        #[doc = "If this field is set to true, the image specified in the 'source' section will directly be validated. No separate build will be run to generate and then validate a customized image."]
        #[serde(rename = "sourceValidationOnly", default, skip_serializing_if = "Option::is_none")]
        pub source_validation_only: Option<bool>,
        #[doc = "List of validations to be performed."]
        #[serde(
            rename = "inVMValidations",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub in_vm_validations: Vec<ImageTemplateInVmValidatorUnion>,
    }
    impl Validate {
        pub fn new() -> Self {
            Self::default()
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
#[doc = "Distribute via Azure Compute Gallery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateSharedImageDistributor {
    #[serde(flatten)]
    pub image_template_distributor: ImageTemplateDistributor,
    #[doc = "Resource Id of the Azure Compute Gallery image"]
    #[serde(rename = "galleryImageId")]
    pub gallery_image_id: String,
    #[doc = "[Deprecated] A list of regions that the image will be replicated to. This list can be specified only if targetRegions is not specified. This field is deprecated - use targetRegions instead."]
    #[serde(
        rename = "replicationRegions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub replication_regions: Vec<String>,
    #[doc = "Flag that indicates whether created image version should be excluded from latest. Omit to use the default (false)."]
    #[serde(rename = "excludeFromLatest", default, skip_serializing_if = "Option::is_none")]
    pub exclude_from_latest: Option<bool>,
    #[doc = "Specifies the storage account type to be used to store the Azure Compute Gallery image version in."]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<SharedImageStorageAccountType>,
    #[doc = "The target regions where the distributed Image Version is going to be replicated to. This object supersedes replicationRegions and can be specified only if replicationRegions is not specified."]
    #[serde(
        rename = "targetRegions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub target_regions: Vec<TargetRegion>,
    #[doc = "Describes how to generate new x.y.z version number for distribution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versioning: Option<DistributeVersionerUnion>,
}
impl ImageTemplateSharedImageDistributor {
    pub fn new(image_template_distributor: ImageTemplateDistributor, gallery_image_id: String) -> Self {
        Self {
            image_template_distributor,
            gallery_image_id,
            replication_regions: Vec::new(),
            exclude_from_latest: None,
            storage_account_type: None,
            target_regions: Vec::new(),
            versioning: None,
        }
    }
}
#[doc = "Describes an image source that is an image version in an Azure Compute Gallery or a Direct Shared Gallery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateSharedImageVersionSource {
    #[serde(flatten)]
    pub image_template_source: ImageTemplateSource,
    #[doc = "ARM resource id of the image version. When image version name is 'latest', the version is evaluated when the image build takes place."]
    #[serde(rename = "imageVersionId")]
    pub image_version_id: String,
    #[doc = "Exact ARM resource id of the image version. This readonly field differs from the image version Id in 'imageVersionId' only if the version name specified in 'imageVersionId' field is 'latest'."]
    #[serde(rename = "exactVersion", default, skip_serializing_if = "Option::is_none")]
    pub exact_version: Option<String>,
}
impl ImageTemplateSharedImageVersionSource {
    pub fn new(image_template_source: ImageTemplateSource, image_version_id: String) -> Self {
        Self {
            image_template_source,
            image_version_id,
            exact_version: None,
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "Runs the specified shell script during the validation phase (Linux). Corresponds to Packer shell provisioner. Exactly one of 'scriptUri' or 'inline' can be specified."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateShellValidator {
    #[serde(flatten)]
    pub image_template_in_vm_validator: ImageTemplateInVmValidator,
    #[doc = "URI of the shell script to be run for validation. It can be a github link, Azure Storage URI, etc"]
    #[serde(rename = "scriptUri", default, skip_serializing_if = "Option::is_none")]
    pub script_uri: Option<String>,
    #[doc = "SHA256 checksum of the shell script provided in the scriptUri field"]
    #[serde(rename = "sha256Checksum", default, skip_serializing_if = "Option::is_none")]
    pub sha256_checksum: Option<String>,
    #[doc = "Array of shell commands to execute"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inline: Vec<String>,
}
impl ImageTemplateShellValidator {
    pub fn new(image_template_in_vm_validator: ImageTemplateInVmValidator) -> Self {
        Self {
            image_template_in_vm_validator,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ImageTemplateSourceUnion {
    ManagedImage(ImageTemplateManagedImageSource),
    PlatformImage(ImageTemplatePlatformImageSource),
    SharedImageVersion(ImageTemplateSharedImageVersionSource),
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
    #[doc = "Optional Azure Storage URI for the distributed VHD blob. Omit to use the default (empty string) in which case VHD would be published to the storage account in the staging resource group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl ImageTemplateVhdDistributor {
    pub fn new(image_template_distributor: ImageTemplateDistributor) -> Self {
        Self {
            image_template_distributor,
            uri: None,
        }
    }
}
#[doc = "Describes the virtual machines used to build and validate images"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageTemplateVmProfile {
    #[doc = "Size of the virtual machine used to build, customize and capture images. Omit or specify empty string to use the default (Standard_D1_v2 for Gen1 images and Standard_D2ds_v4 for Gen2 images)."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[doc = "Size of the OS disk in GB. Omit or specify 0 to use Azure's default OS disk size."]
    #[serde(rename = "osDiskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_size_gb: Option<i32>,
    #[doc = "Optional array of resource IDs of user assigned managed identities to be configured on the build VM and validation VM. This may include the identity of the image template."]
    #[serde(
        rename = "userAssignedIdentities",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_assigned_identities: Vec<String>,
    #[doc = "Virtual Network configuration."]
    #[serde(rename = "vnetConfig", default, skip_serializing_if = "Option::is_none")]
    pub vnet_config: Option<VirtualNetworkConfig>,
}
impl ImageTemplateVmProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Installs Windows Updates. Corresponds to Packer Windows Update Provisioner (https://github.com/rgl/packer-provisioner-windows-update)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTemplateWindowsUpdateCustomizer {
    #[serde(flatten)]
    pub image_template_customizer: ImageTemplateCustomizer,
    #[doc = "Criteria to search updates. Omit or specify empty string to use the default (search all). Refer to above link for examples and detailed description of this field."]
    #[serde(rename = "searchCriteria", default, skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<String>,
    #[doc = "Array of filters to select updates to apply. Omit or specify empty array to use the default (no filter). Refer to above link for examples and detailed description of this field."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub filters: Vec<String>,
    #[doc = "Maximum number of updates to apply at a time. Omit or specify 0 to use the default (1000)"]
    #[serde(rename = "updateLimit", default, skip_serializing_if = "Option::is_none")]
    pub update_limit: Option<i32>,
}
impl ImageTemplateWindowsUpdateCustomizer {
    pub fn new(image_template_customizer: ImageTemplateCustomizer) -> Self {
        Self {
            image_template_customizer,
            search_criteria: None,
            filters: Vec::new(),
            update_limit: None,
        }
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
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Purchase plan configuration for platform image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlatformImagePurchasePlan {
    #[doc = "Name of the purchase plan."]
    #[serde(rename = "planName")]
    pub plan_name: String,
    #[doc = "Product of the purchase plan."]
    #[serde(rename = "planProduct")]
    pub plan_product: String,
    #[doc = "Publisher of the purchase plan."]
    #[serde(rename = "planPublisher")]
    pub plan_publisher: String,
}
impl PlatformImagePurchasePlan {
    pub fn new(plan_name: String, plan_product: String, plan_publisher: String) -> Self {
        Self {
            plan_name,
            plan_product,
            plan_publisher,
        }
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
        BadManagedImageSource,
        BadSharedImageVersionSource,
        BadCustomizerType,
        UnsupportedCustomizerType,
        NoCustomizerScript,
        BadValidatorType,
        UnsupportedValidatorType,
        NoValidatorScript,
        BadDistributeType,
        BadSharedImageDistribute,
        BadStagingResourceGroup,
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
                Self::BadManagedImageSource => serializer.serialize_unit_variant("ProvisioningErrorCode", 2u32, "BadManagedImageSource"),
                Self::BadSharedImageVersionSource => {
                    serializer.serialize_unit_variant("ProvisioningErrorCode", 3u32, "BadSharedImageVersionSource")
                }
                Self::BadCustomizerType => serializer.serialize_unit_variant("ProvisioningErrorCode", 4u32, "BadCustomizerType"),
                Self::UnsupportedCustomizerType => {
                    serializer.serialize_unit_variant("ProvisioningErrorCode", 5u32, "UnsupportedCustomizerType")
                }
                Self::NoCustomizerScript => serializer.serialize_unit_variant("ProvisioningErrorCode", 6u32, "NoCustomizerScript"),
                Self::BadValidatorType => serializer.serialize_unit_variant("ProvisioningErrorCode", 7u32, "BadValidatorType"),
                Self::UnsupportedValidatorType => {
                    serializer.serialize_unit_variant("ProvisioningErrorCode", 8u32, "UnsupportedValidatorType")
                }
                Self::NoValidatorScript => serializer.serialize_unit_variant("ProvisioningErrorCode", 9u32, "NoValidatorScript"),
                Self::BadDistributeType => serializer.serialize_unit_variant("ProvisioningErrorCode", 10u32, "BadDistributeType"),
                Self::BadSharedImageDistribute => {
                    serializer.serialize_unit_variant("ProvisioningErrorCode", 11u32, "BadSharedImageDistribute")
                }
                Self::BadStagingResourceGroup => {
                    serializer.serialize_unit_variant("ProvisioningErrorCode", 12u32, "BadStagingResourceGroup")
                }
                Self::ServerError => serializer.serialize_unit_variant("ProvisioningErrorCode", 13u32, "ServerError"),
                Self::Other => serializer.serialize_unit_variant("ProvisioningErrorCode", 14u32, "Other"),
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
    Canceled,
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an output that was created by running an image template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunOutput {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Describes the properties of a run output"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RunOutputProperties>,
}
impl RunOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of List run outputs operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunOutputCollection {
    #[doc = "An array of run outputs"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RunOutput>,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RunOutputCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
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
#[doc = "Specifies the storage account type to be used to store the Azure Compute Gallery image version in."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SharedImageStorageAccountType")]
pub enum SharedImageStorageAccountType {
    #[serde(rename = "Standard_LRS")]
    StandardLrs,
    #[serde(rename = "Standard_ZRS")]
    StandardZrs,
    #[serde(rename = "Premium_LRS")]
    PremiumLrs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SharedImageStorageAccountType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SharedImageStorageAccountType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SharedImageStorageAccountType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StandardLrs => serializer.serialize_unit_variant("SharedImageStorageAccountType", 0u32, "Standard_LRS"),
            Self::StandardZrs => serializer.serialize_unit_variant("SharedImageStorageAccountType", 1u32, "Standard_ZRS"),
            Self::PremiumLrs => serializer.serialize_unit_variant("SharedImageStorageAccountType", 2u32, "Premium_LRS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties of SourceImage kind of trigger"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceImageTriggerProperties {
    #[serde(flatten)]
    pub trigger_properties: TriggerProperties,
}
impl SourceImageTriggerProperties {
    pub fn new(trigger_properties: TriggerProperties) -> Self {
        Self { trigger_properties }
    }
}
#[doc = "Describes the target region information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetRegion {
    #[doc = "The name of the region."]
    pub name: String,
    #[doc = "The number of replicas of the Image Version to be created in this region. Omit to use the default (1)."]
    #[serde(rename = "replicaCount", default, skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
    #[doc = "Specifies the storage account type to be used to store the Azure Compute Gallery image version in."]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<SharedImageStorageAccountType>,
}
impl TargetRegion {
    pub fn new(name: String) -> Self {
        Self {
            name,
            replica_count: None,
            storage_account_type: None,
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
#[doc = "Represents a trigger that can invoke an image template build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Trigger {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Describes the properties of a trigger"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TriggerPropertiesUnion>,
}
impl Trigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of List triggers operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerCollection {
    #[doc = "An array of triggers"]
    pub value: Vec<Trigger>,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TriggerCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TriggerCollection {
    pub fn new(value: Vec<Trigger>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the properties of a trigger"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerProperties {
    #[doc = "The kind of trigger."]
    pub kind: String,
    #[doc = "Describes the status of a trigger"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TriggerStatus>,
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl TriggerProperties {
    pub fn new(kind: String) -> Self {
        Self {
            kind,
            status: None,
            provisioning_state: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum TriggerPropertiesUnion {
    SourceImage(SourceImageTriggerProperties),
}
#[doc = "Describes the status of a trigger"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerStatus {
    #[doc = "The status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The detailed status message, including for alerts and error messages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The time of the status."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
}
impl TriggerStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual Network configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkConfig {
    #[doc = "Resource id of a pre-existing subnet."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "Size of the proxy virtual machine used to pass traffic to the build VM and validation VM. Omit or specify empty string to use the default (Standard_A1_v2)."]
    #[serde(rename = "proxyVmSize", default, skip_serializing_if = "Option::is_none")]
    pub proxy_vm_size: Option<String>,
}
impl VirtualNetworkConfig {
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
