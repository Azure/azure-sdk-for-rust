#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A disk access SAS uri."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessUri {
    #[doc = "A SAS uri for accessing a disk."]
    #[serde(rename = "accessSAS", default, skip_serializing_if = "Option::is_none")]
    pub access_sas: Option<String>,
    #[doc = "A SAS uri for accessing a VM guest state."]
    #[serde(rename = "securityDataAccessSAS", default, skip_serializing_if = "Option::is_none")]
    pub security_data_access_sas: Option<String>,
}
impl AccessUri {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enables or disables a capability on the virtual machine or virtual machine scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdditionalCapabilities {
    #[doc = "The flag that enables or disables a capability to have one or more managed data disks with UltraSSD_LRS storage account type on the VM or VMSS. Managed disks with storage account type UltraSSD_LRS can be added to a virtual machine or virtual machine scale set only if this property is enabled."]
    #[serde(rename = "ultraSSDEnabled", default, skip_serializing_if = "Option::is_none")]
    pub ultra_ssd_enabled: Option<bool>,
    #[doc = "The flag that enables or disables hibernation capability on the VM."]
    #[serde(rename = "hibernationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub hibernation_enabled: Option<bool>,
}
impl AdditionalCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies additional XML formatted information that can be included in the Unattend.xml file, which is used by Windows Setup. Contents are defined by setting name, component name, and the pass in which the content is applied."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdditionalUnattendContent {
    #[doc = "The pass name. Currently, the only allowable value is OobeSystem."]
    #[serde(rename = "passName", default, skip_serializing_if = "Option::is_none")]
    pub pass_name: Option<additional_unattend_content::PassName>,
    #[doc = "The component name. Currently, the only allowable value is Microsoft-Windows-Shell-Setup."]
    #[serde(rename = "componentName", default, skip_serializing_if = "Option::is_none")]
    pub component_name: Option<additional_unattend_content::ComponentName>,
    #[doc = "Specifies the name of the setting to which the content applies. Possible values are: FirstLogonCommands and AutoLogon."]
    #[serde(rename = "settingName", default, skip_serializing_if = "Option::is_none")]
    pub setting_name: Option<additional_unattend_content::SettingName>,
    #[doc = "Specifies the XML formatted content that is added to the unattend.xml file for the specified path and component. The XML must be less than 4KB and must include the root element for the setting or feature that is being inserted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
impl AdditionalUnattendContent {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod additional_unattend_content {
    use super::*;
    #[doc = "The pass name. Currently, the only allowable value is OobeSystem."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PassName {
        OobeSystem,
    }
    #[doc = "The component name. Currently, the only allowable value is Microsoft-Windows-Shell-Setup."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ComponentName {
        #[serde(rename = "Microsoft-Windows-Shell-Setup")]
        MicrosoftWindowsShellSetup,
    }
    #[doc = "Specifies the name of the setting to which the content applies. Possible values are: FirstLogonCommands and AutoLogon."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SettingName {
        AutoLogon,
        FirstLogonCommands,
    }
}
#[doc = "The API entity reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiEntityReference {
    #[doc = "The ARM resource id in the form of /subscriptions/{SubscriptionId}/resourceGroups/{ResourceGroupName}/..."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ApiEntityReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Api error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiError {
    #[doc = "The Api error details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ApiErrorBase>,
    #[doc = "Inner error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<InnerError>,
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
#[doc = "Contains the list of gallery applications that should be made available to the VM/VMSS"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationProfile {
    #[doc = "Specifies the gallery applications that should be made available to the VM/VMSS"]
    #[serde(rename = "galleryApplications", default, skip_serializing_if = "Vec::is_empty")]
    pub gallery_applications: Vec<VmGalleryApplication>,
}
impl ApplicationProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The architecture of the image. Applicable to OS disks only."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Architecture")]
pub enum Architecture {
    #[serde(rename = "x64")]
    X64,
    Arm64,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Architecture {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Architecture {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Architecture {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::X64 => serializer.serialize_unit_variant("Architecture", 0u32, "x64"),
            Self::Arm64 => serializer.serialize_unit_variant("Architecture", 1u32, "Arm64"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies the Architecture Type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ArchitectureType")]
pub enum ArchitectureType {
    #[serde(rename = "x64")]
    X64,
    Arm64,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ArchitectureType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ArchitectureType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ArchitectureType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::X64 => serializer.serialize_unit_variant("ArchitectureType", 0u32, "x64"),
            Self::Arm64 => serializer.serialize_unit_variant("ArchitectureType", 1u32, "Arm64"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The configuration parameters used for performing automatic OS upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomaticOsUpgradePolicy {
    #[doc = "Indicates whether OS upgrades should automatically be applied to scale set instances in a rolling fashion when a newer version of the OS image becomes available. Default value is false. <br><br> If this is set to true for Windows based scale sets, [enableAutomaticUpdates](https://docs.microsoft.com/dotnet/api/microsoft.azure.management.compute.models.windowsconfiguration.enableautomaticupdates?view=azure-dotnet) is automatically set to false and cannot be set to true."]
    #[serde(rename = "enableAutomaticOSUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_os_upgrade: Option<bool>,
    #[doc = "Whether OS image rollback feature should be disabled. Default value is false."]
    #[serde(rename = "disableAutomaticRollback", default, skip_serializing_if = "Option::is_none")]
    pub disable_automatic_rollback: Option<bool>,
    #[doc = "Indicates whether rolling upgrade policy should be used during Auto OS Upgrade. Default value is false. Auto OS Upgrade will fallback to the default policy if no policy is defined on the VMSS."]
    #[serde(rename = "useRollingUpgradePolicy", default, skip_serializing_if = "Option::is_none")]
    pub use_rolling_upgrade_policy: Option<bool>,
}
impl AutomaticOsUpgradePolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes automatic OS upgrade properties on the image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomaticOsUpgradeProperties {
    #[doc = "Specifies whether automatic OS upgrade is supported on the image."]
    #[serde(rename = "automaticOSUpgradeSupported")]
    pub automatic_os_upgrade_supported: bool,
}
impl AutomaticOsUpgradeProperties {
    pub fn new(automatic_os_upgrade_supported: bool) -> Self {
        Self {
            automatic_os_upgrade_supported,
        }
    }
}
#[doc = "Specifies the configuration parameters for automatic repairs on the virtual machine scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutomaticRepairsPolicy {
    #[doc = "Specifies whether automatic repairs should be enabled on the virtual machine scale set. The default value is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The amount of time for which automatic repairs are suspended due to a state change on VM. The grace time starts after the state change has completed. This helps avoid premature or accidental repairs. The time duration should be specified in ISO 8601 format. The minimum allowed grace period is 10 minutes (PT10M), which is also the default value. The maximum allowed grace period is 90 minutes (PT90M)."]
    #[serde(rename = "gracePeriod", default, skip_serializing_if = "Option::is_none")]
    pub grace_period: Option<String>,
    #[doc = "Type of repair action (replace, restart, reimage) that will be used for repairing unhealthy virtual machines in the scale set. Default value is replace."]
    #[serde(rename = "repairAction", default, skip_serializing_if = "Option::is_none")]
    pub repair_action: Option<automatic_repairs_policy::RepairAction>,
}
impl AutomaticRepairsPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod automatic_repairs_policy {
    use super::*;
    #[doc = "Type of repair action (replace, restart, reimage) that will be used for repairing unhealthy virtual machines in the scale set. Default value is replace."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RepairAction")]
    pub enum RepairAction {
        Replace,
        Restart,
        Reimage,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RepairAction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RepairAction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RepairAction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Replace => serializer.serialize_unit_variant("RepairAction", 0u32, "Replace"),
                Self::Restart => serializer.serialize_unit_variant("RepairAction", 1u32, "Restart"),
                Self::Reimage => serializer.serialize_unit_variant("RepairAction", 2u32, "Reimage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies information about the availability set that the virtual machine should be assigned to. Virtual machines specified in the same availability set are allocated to different nodes to maximize availability. For more information about availability sets, see [Availability sets overview](https://docs.microsoft.com/azure/virtual-machines/availability-set-overview). <br><br> For more information on Azure planned maintenance, see [Maintenance and updates for Virtual Machines in Azure](https://docs.microsoft.com/azure/virtual-machines/maintenance-and-updates) <br><br> Currently, a VM can only be added to availability set at creation time. An existing VM cannot be added to an availability set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailabilitySet {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The instance view of a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailabilitySetProperties>,
    #[doc = "Describes a virtual machine scale set sku. NOTE: If the new VM SKU is not supported on the hardware the scale set is currently on, you need to deallocate the VMs in the scale set before you modify the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl AvailabilitySet {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
            sku: None,
        }
    }
}
#[doc = "The List Availability Set operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailabilitySetListResult {
    #[doc = "The list of availability sets"]
    pub value: Vec<AvailabilitySet>,
    #[doc = "The URI to fetch the next page of AvailabilitySets. Call ListNext() with this URI to fetch the next page of AvailabilitySets."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvailabilitySetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AvailabilitySetListResult {
    pub fn new(value: Vec<AvailabilitySet>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The instance view of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilitySetProperties {
    #[doc = "Update Domain count."]
    #[serde(rename = "platformUpdateDomainCount", default, skip_serializing_if = "Option::is_none")]
    pub platform_update_domain_count: Option<i32>,
    #[doc = "Fault Domain count."]
    #[serde(rename = "platformFaultDomainCount", default, skip_serializing_if = "Option::is_none")]
    pub platform_fault_domain_count: Option<i32>,
    #[doc = "A list of references to all virtual machines in the availability set."]
    #[serde(rename = "virtualMachines", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_machines: Vec<SubResource>,
    #[serde(rename = "proximityPlacementGroup", default, skip_serializing_if = "Option::is_none")]
    pub proximity_placement_group: Option<SubResource>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<InstanceViewStatus>,
}
impl AvailabilitySetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the sku of an Availability Set. Use 'Aligned' for virtual machines with managed disks and 'Classic' for virtual machines with unmanaged disks. Default value is 'Classic'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AvailabilitySetSkuType")]
pub enum AvailabilitySetSkuType {
    Classic,
    Aligned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AvailabilitySetSkuType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AvailabilitySetSkuType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AvailabilitySetSkuType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Classic => serializer.serialize_unit_variant("AvailabilitySetSkuType", 0u32, "Classic"),
            Self::Aligned => serializer.serialize_unit_variant("AvailabilitySetSkuType", 1u32, "Aligned"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies information about the availability set that the virtual machine should be assigned to. Only tags may be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilitySetUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "The instance view of a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailabilitySetProperties>,
    #[doc = "Describes a virtual machine scale set sku. NOTE: If the new VM SKU is not supported on the hardware the scale set is currently on, you need to deallocate the VMs in the scale set before you modify the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl AvailabilitySetUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of an virtual machine instance view for available patch summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailablePatchSummary {
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Unknown\", \"Failed\", \"Succeeded\", or \"CompletedWithWarnings.\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<available_patch_summary::Status>,
    #[doc = "The activity ID of the operation that produced this result. It is used to correlate across CRP and extension logs."]
    #[serde(rename = "assessmentActivityId", default, skip_serializing_if = "Option::is_none")]
    pub assessment_activity_id: Option<String>,
    #[doc = "The overall reboot status of the VM. It will be true when partially installed patches require a reboot to complete installation but the reboot has not yet occurred."]
    #[serde(rename = "rebootPending", default, skip_serializing_if = "Option::is_none")]
    pub reboot_pending: Option<bool>,
    #[doc = "The number of critical or security patches that have been detected as available and not yet installed."]
    #[serde(rename = "criticalAndSecurityPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub critical_and_security_patch_count: Option<i32>,
    #[doc = "The number of all available patches excluding critical and security."]
    #[serde(rename = "otherPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub other_patch_count: Option<i32>,
    #[doc = "The UTC timestamp when the operation began."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The UTC timestamp when the operation began."]
    #[serde(rename = "lastModifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Api error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}
impl AvailablePatchSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod available_patch_summary {
    use super::*;
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Unknown\", \"Failed\", \"Succeeded\", or \"CompletedWithWarnings.\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Unknown,
        InProgress,
        Failed,
        Succeeded,
        CompletedWithWarnings,
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
                Self::Unknown => serializer.serialize_unit_variant("Status", 0u32, "Unknown"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 3u32, "Succeeded"),
                Self::CompletedWithWarnings => serializer.serialize_unit_variant("Status", 4u32, "CompletedWithWarnings"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the billing related details of a Azure Spot VM or VMSS. <br><br>Minimum api-version: 2019-03-01."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProfile {
    #[doc = "Specifies the maximum price you are willing to pay for a Azure Spot VM/VMSS. This price is in US Dollars. <br><br> This price will be compared with the current Azure Spot price for the VM size. Also, the prices are compared at the time of create/update of Azure Spot VM/VMSS and the operation will only succeed if  the maxPrice is greater than the current Azure Spot price. <br><br> The maxPrice will also be used for evicting a Azure Spot VM/VMSS if the current Azure Spot price goes beyond the maxPrice after creation of VM/VMSS. <br><br> Possible values are: <br><br> - Any decimal value greater than zero. Example: 0.01538 <br><br> -1 – indicates default price to be up-to on-demand. <br><br> You can set the maxPrice to -1 to indicate that the Azure Spot VM/VMSS should not be evicted for price reasons. Also, the default max price is -1 if it is not provided by you. <br><br>Minimum api-version: 2019-03-01."]
    #[serde(rename = "maxPrice", default, skip_serializing_if = "Option::is_none")]
    pub max_price: Option<f64>,
}
impl BillingProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Boot Diagnostics is a debugging feature which allows you to view Console Output and Screenshot to diagnose VM status. <br><br> You can easily view the output of your console log. <br><br> Azure also enables you to see a screenshot of the VM from the hypervisor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BootDiagnostics {
    #[doc = "Whether boot diagnostics should be enabled on the Virtual Machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Uri of the storage account to use for placing the console output and screenshot. <br><br>If storageUri is not specified while enabling boot diagnostics, managed storage will be used."]
    #[serde(rename = "storageUri", default, skip_serializing_if = "Option::is_none")]
    pub storage_uri: Option<String>,
}
impl BootDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The instance view of a virtual machine boot diagnostics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BootDiagnosticsInstanceView {
    #[doc = "The console screenshot blob URI. <br><br>NOTE: This will **not** be set if boot diagnostics is currently enabled with managed storage."]
    #[serde(rename = "consoleScreenshotBlobUri", default, skip_serializing_if = "Option::is_none")]
    pub console_screenshot_blob_uri: Option<String>,
    #[doc = "The serial console log blob Uri. <br><br>NOTE: This will **not** be set if boot diagnostics is currently enabled with managed storage."]
    #[serde(rename = "serialConsoleLogBlobUri", default, skip_serializing_if = "Option::is_none")]
    pub serial_console_log_blob_uri: Option<String>,
    #[doc = "Instance view status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceViewStatus>,
}
impl BootDiagnosticsInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the caching requirements. <br><br> Possible values are: <br><br> **None** <br><br> **ReadOnly** <br><br> **ReadWrite** <br><br> Default: **None for Standard storage. ReadOnly for Premium storage**"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Caching {
    None,
    ReadOnly,
    ReadWrite,
}
#[doc = "Specifies information about the capacity reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapacityReservation {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the Capacity reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CapacityReservationProperties>,
    #[doc = "Describes a virtual machine scale set sku. NOTE: If the new VM SKU is not supported on the hardware the scale set is currently on, you need to deallocate the VMs in the scale set before you modify the SKU name."]
    pub sku: Sku,
    #[doc = "Availability Zone to use for this capacity reservation. The zone has to be single value and also should be part for the list of zones specified during the capacity reservation group creation. The zone can be assigned only during creation. If not provided, the reservation supports only non-zonal deployments. If provided, enforces VM/VMSS using this capacity reservation to be in same zone."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl CapacityReservation {
    pub fn new(resource: Resource, sku: Sku) -> Self {
        Self {
            resource,
            properties: None,
            sku,
            zones: Vec::new(),
        }
    }
}
#[doc = "Specifies information about the capacity reservation group that the capacity reservations should be assigned to. <br><br> Currently, a capacity reservation can only be added to a capacity reservation group at creation time. An existing capacity reservation cannot be added or moved to another capacity reservation group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapacityReservationGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "capacity reservation group Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CapacityReservationGroupProperties>,
    #[doc = "Availability Zones to use for this capacity reservation group. The zones can be assigned only during creation. If not provided, the group supports only regional resources in the region. If provided, enforces each capacity reservation in the group to be in one of the zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl CapacityReservationGroup {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
            zones: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityReservationGroupInstanceView {
    #[doc = "List of instance view of the capacity reservations under the capacity reservation group."]
    #[serde(rename = "capacityReservations", default, skip_serializing_if = "Vec::is_empty")]
    pub capacity_reservations: Vec<CapacityReservationInstanceViewWithName>,
}
impl CapacityReservationGroupInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List capacity reservation group with resource group response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapacityReservationGroupListResult {
    #[doc = "The list of capacity reservation groups"]
    pub value: Vec<CapacityReservationGroup>,
    #[doc = "The URI to fetch the next page of capacity reservation groups. Call ListNext() with this URI to fetch the next page of capacity reservation groups."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CapacityReservationGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CapacityReservationGroupListResult {
    pub fn new(value: Vec<CapacityReservationGroup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "capacity reservation group Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityReservationGroupProperties {
    #[doc = "A list of all capacity reservation resource ids that belong to capacity reservation group."]
    #[serde(rename = "capacityReservations", default, skip_serializing_if = "Vec::is_empty")]
    pub capacity_reservations: Vec<SubResourceReadOnly>,
    #[doc = "A list of references to all virtual machines associated to the capacity reservation group."]
    #[serde(rename = "virtualMachinesAssociated", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_machines_associated: Vec<SubResourceReadOnly>,
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<CapacityReservationGroupInstanceView>,
}
impl CapacityReservationGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the capacity reservation group. Only tags can be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityReservationGroupUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "capacity reservation group Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CapacityReservationGroupProperties>,
}
impl CapacityReservationGroupUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The instance view of a capacity reservation that provides as snapshot of the runtime properties of the capacity reservation that is managed by the platform and can change outside of control plane operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityReservationInstanceView {
    #[doc = "Represents the capacity reservation utilization in terms of resources allocated."]
    #[serde(rename = "utilizationInfo", default, skip_serializing_if = "Option::is_none")]
    pub utilization_info: Option<CapacityReservationUtilization>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<InstanceViewStatus>,
}
impl CapacityReservationInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The instance view of a capacity reservation that includes the name of the capacity reservation. It is used for the response to the instance view of a capacity reservation group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityReservationInstanceViewWithName {
    #[serde(flatten)]
    pub capacity_reservation_instance_view: CapacityReservationInstanceView,
    #[doc = "The name of the capacity reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CapacityReservationInstanceViewWithName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list capacity reservation operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapacityReservationListResult {
    #[doc = "The list of capacity reservations"]
    pub value: Vec<CapacityReservation>,
    #[doc = "The URI to fetch the next page of capacity reservations. Call ListNext() with this URI to fetch the next page of capacity reservations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CapacityReservationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CapacityReservationListResult {
    pub fn new(value: Vec<CapacityReservation>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The parameters of a capacity reservation Profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityReservationProfile {
    #[serde(rename = "capacityReservationGroup", default, skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_group: Option<SubResource>,
}
impl CapacityReservationProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Capacity reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityReservationProperties {
    #[doc = "A unique id generated and assigned to the capacity reservation by the platform which does not change throughout the lifetime of the resource."]
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[doc = "A list of all virtual machine resource ids that are associated with the capacity reservation."]
    #[serde(rename = "virtualMachinesAssociated", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_machines_associated: Vec<SubResourceReadOnly>,
    #[doc = "The date time when the capacity reservation was last updated."]
    #[serde(rename = "provisioningTime", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_time: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The instance view of a capacity reservation that provides as snapshot of the runtime properties of the capacity reservation that is managed by the platform and can change outside of control plane operations."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<CapacityReservationInstanceView>,
    #[doc = "Specifies the time at which the Capacity Reservation resource was created.<br><br>Minimum api-version: 2022-03-01."]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<time::OffsetDateTime>,
}
impl CapacityReservationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the capacity reservation. Only tags and sku.capacity can be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityReservationUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "Properties of the Capacity reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CapacityReservationProperties>,
    #[doc = "Describes a virtual machine scale set sku. NOTE: If the new VM SKU is not supported on the hardware the scale set is currently on, you need to deallocate the VMs in the scale set before you modify the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl CapacityReservationUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the capacity reservation utilization in terms of resources allocated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapacityReservationUtilization {
    #[doc = "A list of all virtual machines resource ids allocated against the capacity reservation."]
    #[serde(rename = "virtualMachinesAllocated", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_machines_allocated: Vec<SubResourceReadOnly>,
}
impl CapacityReservationUtilization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the Compute service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Api error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
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
#[doc = "Describes the cloud service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudService {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location."]
    pub location: String,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Cloud service properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudServiceProperties>,
}
impl CloudService {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
            properties: None,
        }
    }
}
#[doc = "Describes a cloud service extension profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServiceExtensionProfile {
    #[doc = "List of extensions for the cloud service."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<Extension>,
}
impl CloudServiceExtensionProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Extension Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServiceExtensionProperties {
    #[doc = "The name of the extension handler publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the type of the extension."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the extension. Specifies the version of the extension. If this element is not specified or an asterisk (*) is used as the value, the latest version of the extension is used. If the value is specified with a major version number and an asterisk as the minor version number (X.), the latest minor version of the specified major version is selected. If a major version number and a minor version number are specified (X.Y), the specific extension version is selected. If a version is specified, an auto-upgrade is performed on the role instance."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Explicitly specify whether platform can automatically upgrade typeHandlerVersion to higher minor versions when they become available."]
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[doc = "Public settings for the extension. For JSON extensions, this is the JSON settings for the extension. For XML Extension (like RDP), this is the XML setting for the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,
    #[doc = "Protected settings for the extension which are encrypted before sent to the role instance."]
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<String>,
    #[serde(rename = "protectedSettingsFromKeyVault", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings_from_key_vault: Option<CloudServiceVaultAndSecretReference>,
    #[doc = "Tag to force apply the provided public and protected settings.\r\nChanging the tag value allows for re-running the extension without changing any of the public or protected settings.\r\nIf forceUpdateTag is not changed, updates to public or protected settings would still be applied by the handler.\r\nIf neither forceUpdateTag nor any of public or protected settings change, extension would flow to the role instance with the same sequence-number, and\r\nit is up to handler implementation whether to re-run it or not"]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Optional list of roles to apply this extension. If property is not specified or '*' is specified, extension is applied to all roles in the cloud service."]
    #[serde(rename = "rolesAppliedTo", default, skip_serializing_if = "Vec::is_empty")]
    pub roles_applied_to: Vec<String>,
}
impl CloudServiceExtensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "InstanceView of CloudService as a whole"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServiceInstanceView {
    #[doc = "Instance view statuses."]
    #[serde(rename = "roleInstance", default, skip_serializing_if = "Option::is_none")]
    pub role_instance: Option<InstanceViewStatusesSummary>,
    #[doc = "The version of the SDK that was used to generate the package for the cloud service."]
    #[serde(rename = "sdkVersion", default, skip_serializing_if = "Option::is_none")]
    pub sdk_version: Option<String>,
    #[doc = "Specifies a list of unique identifiers generated internally for the cloud service. <br /><br /> NOTE: If you are using Azure Diagnostics extension, this property can be used as 'DeploymentId' for querying details."]
    #[serde(rename = "privateIds", default, skip_serializing_if = "Vec::is_empty")]
    pub private_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<ResourceInstanceViewStatus>,
}
impl CloudServiceInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudServiceListResult {
    pub value: Vec<CloudService>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CloudServiceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CloudServiceListResult {
    pub fn new(value: Vec<CloudService>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Network Profile for the cloud service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServiceNetworkProfile {
    #[doc = "List of Load balancer configurations. Cloud service can have up to two load balancer configurations, corresponding to a Public Load Balancer and an Internal Load Balancer."]
    #[serde(rename = "loadBalancerConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_configurations: Vec<LoadBalancerConfiguration>,
    #[serde(rename = "swappableCloudService", default, skip_serializing_if = "Option::is_none")]
    pub swappable_cloud_service: Option<SubResource>,
}
impl CloudServiceNetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the OS profile for the cloud service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServiceOsProfile {
    #[doc = "Specifies set of certificates that should be installed onto the role instances."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<CloudServiceVaultSecretGroup>,
}
impl CloudServiceOsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cloud service properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServiceProperties {
    #[doc = "Specifies a URL that refers to the location of the service package in the Blob service. The service package URL can be Shared Access Signature (SAS) URI from any storage account.\r\nThis is a write-only property and is not returned in GET calls."]
    #[serde(rename = "packageUrl", default, skip_serializing_if = "Option::is_none")]
    pub package_url: Option<String>,
    #[doc = "Specifies the XML service configuration (.cscfg) for the cloud service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[doc = "Specifies a URL that refers to the location of the service configuration in the Blob service. The service package URL  can be Shared Access Signature (SAS) URI from any storage account.\r\nThis is a write-only property and is not returned in GET calls."]
    #[serde(rename = "configurationUrl", default, skip_serializing_if = "Option::is_none")]
    pub configuration_url: Option<String>,
    #[doc = "(Optional) Indicates whether to start the cloud service immediately after it is created. The default value is `true`.\r\nIf false, the service model is still deployed, but the code is not run immediately. Instead, the service is PoweredOff until you call Start, at which time the service will be started. A deployed service still incurs charges, even if it is poweredoff."]
    #[serde(rename = "startCloudService", default, skip_serializing_if = "Option::is_none")]
    pub start_cloud_service: Option<bool>,
    #[doc = "(Optional) Indicates whether the role sku properties (roleProfile.roles.sku) specified in the model/template should override the role instance count and vm size specified in the .cscfg and .csdef respectively.\r\nThe default value is `false`."]
    #[serde(rename = "allowModelOverride", default, skip_serializing_if = "Option::is_none")]
    pub allow_model_override: Option<bool>,
    #[doc = "Update mode for the cloud service. Role instances are allocated to update domains when the service is deployed. Updates can be initiated manually in each update domain or initiated automatically in all update domains.\r\nPossible Values are <br /><br />**Auto**<br /><br />**Manual** <br /><br />**Simultaneous**<br /><br />\r\nIf not specified, the default value is Auto. If set to Manual, PUT UpdateDomain must be called to apply the update. If set to Auto, the update is automatically applied to each update domain in sequence."]
    #[serde(rename = "upgradeMode", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_mode: Option<CloudServiceUpgradeMode>,
    #[doc = "Describes the role profile for the cloud service."]
    #[serde(rename = "roleProfile", default, skip_serializing_if = "Option::is_none")]
    pub role_profile: Option<CloudServiceRoleProfile>,
    #[doc = "Describes the OS profile for the cloud service."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<CloudServiceOsProfile>,
    #[doc = "Network Profile for the cloud service."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<CloudServiceNetworkProfile>,
    #[doc = "Describes a cloud service extension profile."]
    #[serde(rename = "extensionProfile", default, skip_serializing_if = "Option::is_none")]
    pub extension_profile: Option<CloudServiceExtensionProfile>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique identifier for the cloud service."]
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}
impl CloudServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a role of the cloud service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServiceRole {
    #[doc = "Resource id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Describes the cloud service role sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<CloudServiceRoleSku>,
    #[doc = ""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudServiceRoleProperties>,
}
impl CloudServiceRole {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudServiceRoleListResult {
    pub value: Vec<CloudServiceRole>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CloudServiceRoleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CloudServiceRoleListResult {
    pub fn new(value: Vec<CloudServiceRole>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the role profile for the cloud service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServiceRoleProfile {
    #[doc = "List of roles for the cloud service."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<CloudServiceRoleProfileProperties>,
}
impl CloudServiceRoleProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the role properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServiceRoleProfileProperties {
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes the cloud service role sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<CloudServiceRoleSku>,
}
impl CloudServiceRoleProfileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServiceRoleProperties {
    #[doc = "Specifies the ID which uniquely identifies a cloud service role."]
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}
impl CloudServiceRoleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the cloud service role sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServiceRoleSku {
    #[doc = "The sku name. NOTE: If the new SKU is not supported on the hardware the cloud service is currently on, you need to delete and recreate the cloud service or move back to the old sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the tier of the cloud service. Possible Values are <br /><br /> **Standard** <br /><br /> **Basic**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "Specifies the number of role instances in the cloud service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}
impl CloudServiceRoleSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServiceUpdate {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CloudServiceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update mode for the cloud service. Role instances are allocated to update domains when the service is deployed. Updates can be initiated manually in each update domain or initiated automatically in all update domains.\r\nPossible Values are <br /><br />**Auto**<br /><br />**Manual** <br /><br />**Simultaneous**<br /><br />\r\nIf not specified, the default value is Auto. If set to Manual, PUT UpdateDomain must be called to apply the update. If set to Auto, the update is automatically applied to each update domain in sequence."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CloudServiceUpgradeMode")]
pub enum CloudServiceUpgradeMode {
    Auto,
    Manual,
    Simultaneous,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CloudServiceUpgradeMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CloudServiceUpgradeMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CloudServiceUpgradeMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Auto => serializer.serialize_unit_variant("CloudServiceUpgradeMode", 0u32, "Auto"),
            Self::Manual => serializer.serialize_unit_variant("CloudServiceUpgradeMode", 1u32, "Manual"),
            Self::Simultaneous => serializer.serialize_unit_variant("CloudServiceUpgradeMode", 2u32, "Simultaneous"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServiceVaultAndSecretReference {
    #[serde(rename = "sourceVault", default, skip_serializing_if = "Option::is_none")]
    pub source_vault: Option<SubResource>,
    #[serde(rename = "secretUrl", default, skip_serializing_if = "Option::is_none")]
    pub secret_url: Option<String>,
}
impl CloudServiceVaultAndSecretReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a single certificate reference in a Key Vault, and where the certificate should reside on the role instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServiceVaultCertificate {
    #[doc = "This is the URL of a certificate that has been uploaded to Key Vault as a secret."]
    #[serde(rename = "certificateUrl", default, skip_serializing_if = "Option::is_none")]
    pub certificate_url: Option<String>,
}
impl CloudServiceVaultCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a set of certificates which are all in the same Key Vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServiceVaultSecretGroup {
    #[serde(rename = "sourceVault", default, skip_serializing_if = "Option::is_none")]
    pub source_vault: Option<SubResource>,
    #[doc = "The list of key vault references in SourceVault which contain certificates."]
    #[serde(rename = "vaultCertificates", default, skip_serializing_if = "Vec::is_empty")]
    pub vault_certificates: Vec<CloudServiceVaultCertificate>,
}
impl CloudServiceVaultSecretGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the Community Gallery that you want to create or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunityGallery {
    #[serde(flatten)]
    pub pir_community_gallery_resource: PirCommunityGalleryResource,
}
impl CommunityGallery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The identifier information of community gallery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunityGalleryIdentifier {
    #[doc = "The unique id of this community gallery."]
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}
impl CommunityGalleryIdentifier {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the gallery image definition that you want to create or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunityGalleryImage {
    #[serde(flatten)]
    pub pir_community_gallery_resource: PirCommunityGalleryResource,
    #[doc = "Describes the properties of a gallery image definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CommunityGalleryImageProperties>,
}
impl CommunityGalleryImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Community Gallery Images operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunityGalleryImageList {
    #[doc = "A list of community gallery images."]
    pub value: Vec<CommunityGalleryImage>,
    #[doc = "The uri to fetch the next page of community gallery images. Call ListNext() with this to fetch the next page of community gallery images."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CommunityGalleryImageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CommunityGalleryImageList {
    pub fn new(value: Vec<CommunityGalleryImage>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the properties of a gallery image definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunityGalleryImageProperties {
    #[doc = "This property allows you to specify the type of the OS that is included in the disk when creating a VM from a managed image. <br><br> Possible values are: <br><br> **Windows** <br><br> **Linux**"]
    #[serde(rename = "osType")]
    pub os_type: community_gallery_image_properties::OsType,
    #[doc = "This property allows the user to specify whether the virtual machines created under this image are 'Generalized' or 'Specialized'."]
    #[serde(rename = "osState")]
    pub os_state: community_gallery_image_properties::OsState,
    #[doc = "The end of life date of the gallery image definition. This property can be used for decommissioning purposes. This property is updatable."]
    #[serde(rename = "endOfLifeDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_of_life_date: Option<time::OffsetDateTime>,
    #[doc = "This is the gallery image definition identifier."]
    pub identifier: GalleryImageIdentifier,
    #[doc = "The properties describe the recommended machine configuration for this Image Definition. These properties are updatable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommended: Option<RecommendedMachineConfiguration>,
    #[doc = "Describes the disallowed disk types."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disallowed: Option<Disallowed>,
    #[doc = "The hypervisor generation of the Virtual Machine. Applicable to OS disks only."]
    #[serde(rename = "hyperVGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<community_gallery_image_properties::HyperVGeneration>,
    #[doc = "A list of gallery image features."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<GalleryImageFeature>,
    #[doc = "Describes the gallery image definition purchase plan. This is used by marketplace images."]
    #[serde(rename = "purchasePlan", default, skip_serializing_if = "Option::is_none")]
    pub purchase_plan: Option<ImagePurchasePlan>,
    #[doc = "The architecture of the image. Applicable to OS disks only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<Architecture>,
    #[doc = "Privacy statement uri for the current community gallery image."]
    #[serde(rename = "privacyStatementUri", default, skip_serializing_if = "Option::is_none")]
    pub privacy_statement_uri: Option<String>,
    #[doc = "End-user license agreement for the current community gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eula: Option<String>,
}
impl CommunityGalleryImageProperties {
    pub fn new(
        os_type: community_gallery_image_properties::OsType,
        os_state: community_gallery_image_properties::OsState,
        identifier: GalleryImageIdentifier,
    ) -> Self {
        Self {
            os_type,
            os_state,
            end_of_life_date: None,
            identifier,
            recommended: None,
            disallowed: None,
            hyper_v_generation: None,
            features: Vec::new(),
            purchase_plan: None,
            architecture: None,
            privacy_statement_uri: None,
            eula: None,
        }
    }
}
pub mod community_gallery_image_properties {
    use super::*;
    #[doc = "This property allows you to specify the type of the OS that is included in the disk when creating a VM from a managed image. <br><br> Possible values are: <br><br> **Windows** <br><br> **Linux**"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[doc = "This property allows the user to specify whether the virtual machines created under this image are 'Generalized' or 'Specialized'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsState {
        Generalized,
        Specialized,
    }
    #[doc = "The hypervisor generation of the Virtual Machine. Applicable to OS disks only."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HyperVGeneration")]
    pub enum HyperVGeneration {
        V1,
        V2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HyperVGeneration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HyperVGeneration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HyperVGeneration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::V1 => serializer.serialize_unit_variant("HyperVGeneration", 0u32, "V1"),
                Self::V2 => serializer.serialize_unit_variant("HyperVGeneration", 1u32, "V2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies information about the gallery image version that you want to create or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunityGalleryImageVersion {
    #[serde(flatten)]
    pub pir_community_gallery_resource: PirCommunityGalleryResource,
    #[doc = "Describes the properties of a gallery image version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CommunityGalleryImageVersionProperties>,
}
impl CommunityGalleryImageVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Community Gallery Image versions operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunityGalleryImageVersionList {
    #[doc = "A list of community gallery image versions."]
    pub value: Vec<CommunityGalleryImageVersion>,
    #[doc = "The uri to fetch the next page of community gallery image versions. Call ListNext() with this to fetch the next page of community gallery image versions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CommunityGalleryImageVersionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CommunityGalleryImageVersionList {
    pub fn new(value: Vec<CommunityGalleryImageVersion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the properties of a gallery image version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunityGalleryImageVersionProperties {
    #[doc = "The published date of the gallery image version Definition. This property can be used for decommissioning purposes. This property is updatable."]
    #[serde(rename = "publishedDate", default, with = "azure_core::date::rfc3339::option")]
    pub published_date: Option<time::OffsetDateTime>,
    #[doc = "The end of life date of the gallery image version Definition. This property can be used for decommissioning purposes. This property is updatable."]
    #[serde(rename = "endOfLifeDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_of_life_date: Option<time::OffsetDateTime>,
    #[doc = "If set to true, Virtual Machines deployed from the latest version of the Image Definition won't use this Image Version."]
    #[serde(rename = "excludeFromLatest", default, skip_serializing_if = "Option::is_none")]
    pub exclude_from_latest: Option<bool>,
    #[doc = "This is the storage profile of a Gallery Image Version."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<SharedGalleryImageVersionStorageProfile>,
}
impl CommunityGalleryImageVersionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information of community gallery if current gallery is shared to community"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunityGalleryInfo {
    #[doc = "The link to the publisher website. Visible to all users."]
    #[serde(rename = "publisherUri", default, skip_serializing_if = "Option::is_none")]
    pub publisher_uri: Option<String>,
    #[doc = "Community gallery publisher support email. The email address of the publisher. Visible to all users."]
    #[serde(rename = "publisherContact", default, skip_serializing_if = "Option::is_none")]
    pub publisher_contact: Option<String>,
    #[doc = "End-user license agreement for community gallery image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eula: Option<String>,
    #[doc = "The prefix of the gallery name that will be displayed publicly. Visible to all users."]
    #[serde(rename = "publicNamePrefix", default, skip_serializing_if = "Option::is_none")]
    pub public_name_prefix: Option<String>,
    #[doc = "Contains info about whether community gallery sharing is enabled."]
    #[serde(rename = "communityGalleryEnabled", default, skip_serializing_if = "Option::is_none")]
    pub community_gallery_enabled: Option<bool>,
    #[doc = "Community gallery public name list."]
    #[serde(rename = "publicNames", default, skip_serializing_if = "Vec::is_empty")]
    pub public_names: Vec<String>,
}
impl CommunityGalleryInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Compute Operation operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeOperationListResult {
    #[doc = "The list of compute operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ComputeOperationValue>,
}
impl azure_core::Continuable for ComputeOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ComputeOperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Compute Operation value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeOperationValue {
    #[doc = "The origin of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The name of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes the properties of a Compute Operation Value Display."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ComputeOperationValueDisplay>,
}
impl ComputeOperationValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Compute Operation Value Display."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeOperationValueDisplay {
    #[doc = "The display name of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The display name of the resource the operation applies to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The resource provider for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
impl ComputeOperationValueDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies how the virtual machine should be created.<br><br> Possible values are:<br><br> **Attach** \\u2013 This value is used when you are using a specialized disk to create the virtual machine.<br><br> **FromImage** \\u2013 This value is used when you are using an image to create the virtual machine. If you are using a platform image, you also use the imageReference element described above. If you are using a marketplace image, you  also use the plan element previously described."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CreateOption")]
pub enum CreateOption {
    FromImage,
    Empty,
    Attach,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CreateOption {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CreateOption {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CreateOption {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::FromImage => serializer.serialize_unit_variant("CreateOption", 0u32, "FromImage"),
            Self::Empty => serializer.serialize_unit_variant("CreateOption", 1u32, "Empty"),
            Self::Attach => serializer.serialize_unit_variant("CreateOption", 2u32, "Attach"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Data used when creating a disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreationData {
    #[doc = "This enumerates the possible sources of a disk's creation."]
    #[serde(rename = "createOption")]
    pub create_option: creation_data::CreateOption,
    #[doc = "Required if createOption is Import. The Azure Resource Manager identifier of the storage account containing the blob to import as a disk."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "The source image used for creating the disk."]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageDiskReference>,
    #[doc = "The source image used for creating the disk."]
    #[serde(rename = "galleryImageReference", default, skip_serializing_if = "Option::is_none")]
    pub gallery_image_reference: Option<ImageDiskReference>,
    #[doc = "If createOption is Import, this is the URI of a blob to be imported into a managed disk."]
    #[serde(rename = "sourceUri", default, skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[doc = "If createOption is Copy, this is the ARM id of the source snapshot or disk."]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[doc = "If this field is set, this is the unique id identifying the source of this resource."]
    #[serde(rename = "sourceUniqueId", default, skip_serializing_if = "Option::is_none")]
    pub source_unique_id: Option<String>,
    #[doc = "If createOption is Upload, this is the size of the contents of the upload including the VHD footer. This value should be between 20972032 (20 MiB + 512 bytes for the VHD footer) and 35183298347520 bytes (32 TiB + 512 bytes for the VHD footer)."]
    #[serde(rename = "uploadSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub upload_size_bytes: Option<i64>,
    #[doc = "Logical sector size in bytes for Ultra disks. Supported values are 512 ad 4096. 4096 is the default."]
    #[serde(rename = "logicalSectorSize", default, skip_serializing_if = "Option::is_none")]
    pub logical_sector_size: Option<i32>,
    #[doc = "If createOption is ImportSecure, this is the URI of a blob to be imported into VM guest state."]
    #[serde(rename = "securityDataUri", default, skip_serializing_if = "Option::is_none")]
    pub security_data_uri: Option<String>,
}
impl CreationData {
    pub fn new(create_option: creation_data::CreateOption) -> Self {
        Self {
            create_option,
            storage_account_id: None,
            image_reference: None,
            gallery_image_reference: None,
            source_uri: None,
            source_resource_id: None,
            source_unique_id: None,
            upload_size_bytes: None,
            logical_sector_size: None,
            security_data_uri: None,
        }
    }
}
pub mod creation_data {
    use super::*;
    #[doc = "This enumerates the possible sources of a disk's creation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateOption")]
    pub enum CreateOption {
        Empty,
        Attach,
        FromImage,
        Import,
        Copy,
        Restore,
        Upload,
        CopyStart,
        ImportSecure,
        UploadPreparedSecure,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreateOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreateOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreateOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Empty => serializer.serialize_unit_variant("CreateOption", 0u32, "Empty"),
                Self::Attach => serializer.serialize_unit_variant("CreateOption", 1u32, "Attach"),
                Self::FromImage => serializer.serialize_unit_variant("CreateOption", 2u32, "FromImage"),
                Self::Import => serializer.serialize_unit_variant("CreateOption", 3u32, "Import"),
                Self::Copy => serializer.serialize_unit_variant("CreateOption", 4u32, "Copy"),
                Self::Restore => serializer.serialize_unit_variant("CreateOption", 5u32, "Restore"),
                Self::Upload => serializer.serialize_unit_variant("CreateOption", 6u32, "Upload"),
                Self::CopyStart => serializer.serialize_unit_variant("CreateOption", 7u32, "CopyStart"),
                Self::ImportSecure => serializer.serialize_unit_variant("CreateOption", 8u32, "ImportSecure"),
                Self::UploadPreparedSecure => serializer.serialize_unit_variant("CreateOption", 9u32, "UploadPreparedSecure"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Additional authentication requirements when exporting or uploading to a disk or snapshot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DataAccessAuthMode")]
pub enum DataAccessAuthMode {
    AzureActiveDirectory,
    None,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DataAccessAuthMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DataAccessAuthMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DataAccessAuthMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureActiveDirectory => serializer.serialize_unit_variant("DataAccessAuthMode", 0u32, "AzureActiveDirectory"),
            Self::None => serializer.serialize_unit_variant("DataAccessAuthMode", 1u32, "None"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes a data disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataDisk {
    #[doc = "Specifies the logical unit number of the data disk. This value is used to identify data disks within the VM and therefore must be unique for each data disk attached to a VM."]
    pub lun: i32,
    #[doc = "The disk name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes the uri of a disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhd: Option<VirtualHardDisk>,
    #[doc = "Describes the uri of a disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<VirtualHardDisk>,
    #[doc = "Specifies the caching requirements. <br><br> Possible values are: <br><br> **None** <br><br> **ReadOnly** <br><br> **ReadWrite** <br><br> Default: **None for Standard storage. ReadOnly for Premium storage**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<Caching>,
    #[doc = "Specifies whether writeAccelerator should be enabled or disabled on the disk."]
    #[serde(rename = "writeAcceleratorEnabled", default, skip_serializing_if = "Option::is_none")]
    pub write_accelerator_enabled: Option<bool>,
    #[doc = "Specifies how the virtual machine should be created.<br><br> Possible values are:<br><br> **Attach** \\u2013 This value is used when you are using a specialized disk to create the virtual machine.<br><br> **FromImage** \\u2013 This value is used when you are using an image to create the virtual machine. If you are using a platform image, you also use the imageReference element described above. If you are using a marketplace image, you  also use the plan element previously described."]
    #[serde(rename = "createOption")]
    pub create_option: CreateOption,
    #[doc = "Specifies the size of an empty data disk in gigabytes. This element can be used to overwrite the size of the disk in a virtual machine image. <br><br> This value cannot be larger than 1023 GB"]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "The parameters of a managed disk."]
    #[serde(rename = "managedDisk", default, skip_serializing_if = "Option::is_none")]
    pub managed_disk: Option<ManagedDiskParameters>,
    #[doc = "Specifies whether the data disk is in process of detachment from the VirtualMachine/VirtualMachineScaleset"]
    #[serde(rename = "toBeDetached", default, skip_serializing_if = "Option::is_none")]
    pub to_be_detached: Option<bool>,
    #[doc = "Specifies the Read-Write IOPS for the managed disk when StorageAccountType is UltraSSD_LRS. Returned only for VirtualMachine ScaleSet VM disks. Can be updated only via updates to the VirtualMachine Scale Set."]
    #[serde(rename = "diskIOPSReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_write: Option<i64>,
    #[doc = "Specifies the bandwidth in MB per second for the managed disk when StorageAccountType is UltraSSD_LRS. Returned only for VirtualMachine ScaleSet VM disks. Can be updated only via updates to the VirtualMachine Scale Set."]
    #[serde(rename = "diskMBpsReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_write: Option<i64>,
    #[doc = "Specifies the detach behavior to be used while detaching a disk or which is already in the process of detachment from the virtual machine. Supported values: **ForceDetach**. <br><br> detachOption: **ForceDetach** is applicable only for managed data disks. If a previous detachment attempt of the data disk did not complete due to an unexpected failure from the virtual machine and the disk is still not released then use force-detach as a last resort option to detach the disk forcibly from the VM. All writes might not have been flushed when using this detach behavior. <br><br> This feature is still in preview mode and is not supported for VirtualMachineScaleSet. To force-detach a data disk update toBeDetached to 'true' along with setting detachOption: 'ForceDetach'."]
    #[serde(rename = "detachOption", default, skip_serializing_if = "Option::is_none")]
    pub detach_option: Option<DetachOption>,
    #[doc = "Specifies the behavior of the managed disk when the VM gets deleted i.e whether the managed disk is deleted or detached. Supported values:<br><br> **Delete** If this value is used, the managed disk is deleted when VM gets deleted.<br><br> **Detach** If this value is used, the managed disk is retained after VM gets deleted.<br><br> Minimum api-version: 2021-03-01"]
    #[serde(rename = "deleteOption", default, skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<DeleteOption>,
}
impl DataDisk {
    pub fn new(lun: i32, create_option: CreateOption) -> Self {
        Self {
            lun,
            name: None,
            vhd: None,
            image: None,
            caching: None,
            write_accelerator_enabled: None,
            create_option,
            disk_size_gb: None,
            managed_disk: None,
            to_be_detached: None,
            disk_iops_read_write: None,
            disk_m_bps_read_write: None,
            detach_option: None,
            delete_option: None,
        }
    }
}
#[doc = "Contains the data disk images information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataDiskImage {
    #[doc = "Specifies the logical unit number of the data disk. This value is used to identify data disks within the VM and therefore must be unique for each data disk attached to a VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
}
impl DataDiskImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains encryption settings for a data disk image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataDiskImageEncryption {
    #[serde(flatten)]
    pub disk_image_encryption: DiskImageEncryption,
    #[doc = "This property specifies the logical unit number of the data disk. This value is used to identify data disks within the Virtual Machine and therefore must be unique for each data disk attached to the Virtual Machine."]
    pub lun: i32,
}
impl DataDiskImageEncryption {
    pub fn new(lun: i32) -> Self {
        Self {
            disk_image_encryption: DiskImageEncryption::default(),
            lun,
        }
    }
}
#[doc = "Specifies information about the Dedicated host."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedHost {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the dedicated host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DedicatedHostProperties>,
    #[doc = "Describes a virtual machine scale set sku. NOTE: If the new VM SKU is not supported on the hardware the scale set is currently on, you need to deallocate the VMs in the scale set before you modify the SKU name."]
    pub sku: Sku,
}
impl DedicatedHost {
    pub fn new(resource: Resource, sku: Sku) -> Self {
        Self {
            resource,
            properties: None,
            sku,
        }
    }
}
#[doc = "Represents the dedicated host unutilized capacity in terms of a specific VM size."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedHostAllocatableVm {
    #[doc = "VM size in terms of which the unutilized capacity is represented."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[doc = "Maximum number of VMs of size vmSize that can fit in the dedicated host's remaining capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
}
impl DedicatedHostAllocatableVm {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dedicated host unutilized capacity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedHostAvailableCapacity {
    #[doc = "The unutilized capacity of the dedicated host represented in terms of each VM size that is allowed to be deployed to the dedicated host."]
    #[serde(rename = "allocatableVMs", default, skip_serializing_if = "Vec::is_empty")]
    pub allocatable_v_ms: Vec<DedicatedHostAllocatableVm>,
}
impl DedicatedHostAvailableCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the dedicated host group that the dedicated hosts should be assigned to. <br><br> Currently, a dedicated host can only be added to a dedicated host group at creation time. An existing dedicated host cannot be added to another dedicated host group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedHostGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Dedicated Host Group Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DedicatedHostGroupProperties>,
    #[doc = "Availability Zone to use for this host group. Only single zone is supported. The zone can be assigned only during creation. If not provided, the group supports all zones in the region. If provided, enforces each host in the group to be in the same zone."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl DedicatedHostGroup {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
            zones: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedHostGroupInstanceView {
    #[doc = "List of instance view of the dedicated hosts under the dedicated host group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hosts: Vec<DedicatedHostInstanceViewWithName>,
}
impl DedicatedHostGroupInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Dedicated Host Group with resource group response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedHostGroupListResult {
    #[doc = "The list of dedicated host groups"]
    pub value: Vec<DedicatedHostGroup>,
    #[doc = "The URI to fetch the next page of Dedicated Host Groups. Call ListNext() with this URI to fetch the next page of Dedicated Host Groups."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DedicatedHostGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DedicatedHostGroupListResult {
    pub fn new(value: Vec<DedicatedHostGroup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Dedicated Host Group Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedHostGroupProperties {
    #[doc = "Number of fault domains that the host group can span."]
    #[serde(rename = "platformFaultDomainCount")]
    pub platform_fault_domain_count: i32,
    #[doc = "A list of references to all dedicated hosts in the dedicated host group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hosts: Vec<SubResourceReadOnly>,
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<DedicatedHostGroupInstanceView>,
    #[doc = "Specifies whether virtual machines or virtual machine scale sets can be placed automatically on the dedicated host group. Automatic placement means resources are allocated on dedicated hosts, that are chosen by Azure, under the dedicated host group. The value is defaulted to 'false' when not provided. <br><br>Minimum api-version: 2020-06-01."]
    #[serde(rename = "supportAutomaticPlacement", default, skip_serializing_if = "Option::is_none")]
    pub support_automatic_placement: Option<bool>,
    #[doc = "Enables or disables a capability on the dedicated host group.<br><br>Minimum api-version: 2022-03-01."]
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub additional_capabilities: Option<dedicated_host_group_properties::AdditionalCapabilities>,
}
impl DedicatedHostGroupProperties {
    pub fn new(platform_fault_domain_count: i32) -> Self {
        Self {
            platform_fault_domain_count,
            hosts: Vec::new(),
            instance_view: None,
            support_automatic_placement: None,
            additional_capabilities: None,
        }
    }
}
pub mod dedicated_host_group_properties {
    use super::*;
    #[doc = "Enables or disables a capability on the dedicated host group.<br><br>Minimum api-version: 2022-03-01."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct AdditionalCapabilities {
        #[doc = "The flag that enables or disables a capability to have UltraSSD Enabled Virtual Machines on Dedicated Hosts of the Dedicated Host Group. For the Virtual Machines to be UltraSSD Enabled, UltraSSDEnabled flag for the resource needs to be set true as well. The value is defaulted to 'false' when not provided. Please refer to https://docs.microsoft.com/en-us/azure/virtual-machines/disks-enable-ultra-ssd for more details on Ultra SSD feature. <br><br>NOTE: The ultraSSDEnabled setting can only be enabled for Host Groups that are created as zonal. <br><br>Minimum api-version: 2022-03-01."]
        #[serde(rename = "ultraSSDEnabled", default, skip_serializing_if = "Option::is_none")]
        pub ultra_ssd_enabled: Option<bool>,
    }
    impl AdditionalCapabilities {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Specifies information about the dedicated host group that the dedicated host should be assigned to. Only tags may be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedHostGroupUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "Dedicated Host Group Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DedicatedHostGroupProperties>,
    #[doc = "Availability Zone to use for this host group. Only single zone is supported. The zone can be assigned only during creation. If not provided, the group supports all zones in the region. If provided, enforces each host in the group to be in the same zone."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl DedicatedHostGroupUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The instance view of a dedicated host."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedHostInstanceView {
    #[doc = "Specifies the unique id of the dedicated physical machine on which the dedicated host resides."]
    #[serde(rename = "assetId", default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[doc = "Dedicated host unutilized capacity."]
    #[serde(rename = "availableCapacity", default, skip_serializing_if = "Option::is_none")]
    pub available_capacity: Option<DedicatedHostAvailableCapacity>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<InstanceViewStatus>,
}
impl DedicatedHostInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The instance view of a dedicated host that includes the name of the dedicated host. It is used for the response to the instance view of a dedicated host group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedHostInstanceViewWithName {
    #[serde(flatten)]
    pub dedicated_host_instance_view: DedicatedHostInstanceView,
    #[doc = "The name of the dedicated host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl DedicatedHostInstanceViewWithName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the software license type that will be applied to the VMs deployed on the dedicated host. <br><br> Possible values are: <br><br> **None** <br><br> **Windows_Server_Hybrid** <br><br> **Windows_Server_Perpetual** <br><br> Default: **None**"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DedicatedHostLicenseType {
    None,
    #[serde(rename = "Windows_Server_Hybrid")]
    WindowsServerHybrid,
    #[serde(rename = "Windows_Server_Perpetual")]
    WindowsServerPerpetual,
}
#[doc = "The list dedicated host operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DedicatedHostListResult {
    #[doc = "The list of dedicated hosts"]
    pub value: Vec<DedicatedHost>,
    #[doc = "The URI to fetch the next page of dedicated hosts. Call ListNext() with this URI to fetch the next page of dedicated hosts."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DedicatedHostListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DedicatedHostListResult {
    pub fn new(value: Vec<DedicatedHost>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties of the dedicated host."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedHostProperties {
    #[doc = "Fault domain of the dedicated host within a dedicated host group."]
    #[serde(rename = "platformFaultDomain", default, skip_serializing_if = "Option::is_none")]
    pub platform_fault_domain: Option<i32>,
    #[doc = "Specifies whether the dedicated host should be replaced automatically in case of a failure. The value is defaulted to 'true' when not provided."]
    #[serde(rename = "autoReplaceOnFailure", default, skip_serializing_if = "Option::is_none")]
    pub auto_replace_on_failure: Option<bool>,
    #[doc = "A unique id generated and assigned to the dedicated host by the platform. <br><br> Does not change throughout the lifetime of the host."]
    #[serde(rename = "hostId", default, skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,
    #[doc = "A list of references to all virtual machines in the Dedicated Host."]
    #[serde(rename = "virtualMachines", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_machines: Vec<SubResourceReadOnly>,
    #[doc = "Specifies the software license type that will be applied to the VMs deployed on the dedicated host. <br><br> Possible values are: <br><br> **None** <br><br> **Windows_Server_Hybrid** <br><br> **Windows_Server_Perpetual** <br><br> Default: **None**"]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<DedicatedHostLicenseType>,
    #[doc = "The date when the host was first provisioned."]
    #[serde(rename = "provisioningTime", default, with = "azure_core::date::rfc3339::option")]
    pub provisioning_time: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The instance view of a dedicated host."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<DedicatedHostInstanceView>,
    #[doc = "Specifies the time at which the Dedicated Host resource was created.<br><br>Minimum api-version: 2022-03-01."]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<time::OffsetDateTime>,
}
impl DedicatedHostProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the dedicated host. Only tags, autoReplaceOnFailure and licenseType may be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DedicatedHostUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "Properties of the dedicated host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DedicatedHostProperties>,
}
impl DedicatedHostUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the behavior of the managed disk when the VM gets deleted i.e whether the managed disk is deleted or detached. Supported values:<br><br> **Delete** If this value is used, the managed disk is deleted when VM gets deleted.<br><br> **Detach** If this value is used, the managed disk is retained after VM gets deleted.<br><br> Minimum api-version: 2021-03-01"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeleteOption")]
pub enum DeleteOption {
    Delete,
    Detach,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeleteOption {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeleteOption {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeleteOption {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Delete => serializer.serialize_unit_variant("DeleteOption", 0u32, "Delete"),
            Self::Detach => serializer.serialize_unit_variant("DeleteOption", 1u32, "Detach"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies the detach behavior to be used while detaching a disk or which is already in the process of detachment from the virtual machine. Supported values: **ForceDetach**. <br><br> detachOption: **ForceDetach** is applicable only for managed data disks. If a previous detachment attempt of the data disk did not complete due to an unexpected failure from the virtual machine and the disk is still not released then use force-detach as a last resort option to detach the disk forcibly from the VM. All writes might not have been flushed when using this detach behavior. <br><br> This feature is still in preview mode and is not supported for VirtualMachineScaleSet. To force-detach a data disk update toBeDetached to 'true' along with setting detachOption: 'ForceDetach'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DetachOption")]
pub enum DetachOption {
    ForceDetach,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DetachOption {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DetachOption {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DetachOption {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ForceDetach => serializer.serialize_unit_variant("DetachOption", 0u32, "ForceDetach"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies the boot diagnostic settings state. <br><br>Minimum api-version: 2015-06-15."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticsProfile {
    #[doc = "Boot Diagnostics is a debugging feature which allows you to view Console Output and Screenshot to diagnose VM status. <br><br> You can easily view the output of your console log. <br><br> Azure also enables you to see a screenshot of the VM from the hypervisor."]
    #[serde(rename = "bootDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub boot_diagnostics: Option<BootDiagnostics>,
}
impl DiagnosticsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the ephemeral disk option for operating system disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiffDiskOption")]
pub enum DiffDiskOption {
    Local,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DiffDiskOption {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DiffDiskOption {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DiffDiskOption {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Local => serializer.serialize_unit_variant("DiffDiskOption", 0u32, "Local"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies the ephemeral disk placement for operating system disk. This property can be used by user in the request to choose the location i.e, cache disk or resource disk space for Ephemeral OS disk provisioning. For more information on Ephemeral OS disk size requirements, please refer Ephemeral OS disk size requirements for Windows VM at https://docs.microsoft.com/azure/virtual-machines/windows/ephemeral-os-disks#size-requirements and Linux VM at https://docs.microsoft.com/azure/virtual-machines/linux/ephemeral-os-disks#size-requirements"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiffDiskPlacement")]
pub enum DiffDiskPlacement {
    CacheDisk,
    ResourceDisk,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DiffDiskPlacement {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DiffDiskPlacement {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DiffDiskPlacement {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CacheDisk => serializer.serialize_unit_variant("DiffDiskPlacement", 0u32, "CacheDisk"),
            Self::ResourceDisk => serializer.serialize_unit_variant("DiffDiskPlacement", 1u32, "ResourceDisk"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the parameters of ephemeral disk settings that can be specified for operating system disk. <br><br> NOTE: The ephemeral disk settings can only be specified for managed disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiffDiskSettings {
    #[doc = "Specifies the ephemeral disk option for operating system disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option: Option<DiffDiskOption>,
    #[doc = "Specifies the ephemeral disk placement for operating system disk. This property can be used by user in the request to choose the location i.e, cache disk or resource disk space for Ephemeral OS disk provisioning. For more information on Ephemeral OS disk size requirements, please refer Ephemeral OS disk size requirements for Windows VM at https://docs.microsoft.com/azure/virtual-machines/windows/ephemeral-os-disks#size-requirements and Linux VM at https://docs.microsoft.com/azure/virtual-machines/linux/ephemeral-os-disks#size-requirements"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<DiffDiskPlacement>,
}
impl DiffDiskSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the disallowed disk types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Disallowed {
    #[doc = "A list of disk types."]
    #[serde(rename = "diskTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub disk_types: Vec<String>,
}
impl Disallowed {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the disallowed configuration for a virtual machine image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DisallowedConfiguration {
    #[doc = "VM disk types which are disallowed."]
    #[serde(rename = "vmDiskType", default, skip_serializing_if = "Option::is_none")]
    pub vm_disk_type: Option<disallowed_configuration::VmDiskType>,
}
impl DisallowedConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod disallowed_configuration {
    use super::*;
    #[doc = "VM disk types which are disallowed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VmDiskType")]
    pub enum VmDiskType {
        None,
        Unmanaged,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VmDiskType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VmDiskType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VmDiskType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("VmDiskType", 0u32, "None"),
                Self::Unmanaged => serializer.serialize_unit_variant("VmDiskType", 1u32, "Unmanaged"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Disk resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Disk {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "A relative URI containing the ID of the VM that has the disk attached."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[doc = "List of relative URIs containing the IDs of the VMs that have the disk attached. maxShares should be set to a value greater than one for disks to allow attaching them to multiple VMs."]
    #[serde(rename = "managedByExtended", default, skip_serializing_if = "Vec::is_empty")]
    pub managed_by_extended: Vec<String>,
    #[doc = "The disks sku name. Can be Standard_LRS, Premium_LRS, StandardSSD_LRS, UltraSSD_LRS, Premium_ZRS, or StandardSSD_ZRS."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<DiskSku>,
    #[doc = "The Logical zone list for Disk."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "Disk resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskProperties>,
}
impl Disk {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            managed_by: None,
            managed_by_extended: Vec::new(),
            sku: None,
            zones: Vec::new(),
            extended_location: None,
            properties: None,
        }
    }
}
#[doc = "disk access resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskAccess {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskAccessProperties>,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
}
impl DiskAccess {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
            extended_location: None,
        }
    }
}
#[doc = "The List disk access operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskAccessList {
    #[doc = "A list of disk access resources."]
    pub value: Vec<DiskAccess>,
    #[doc = "The uri to fetch the next page of disk access resources. Call ListNext() with this to fetch the next page of disk access resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiskAccessList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DiskAccessList {
    pub fn new(value: Vec<DiskAccess>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskAccessProperties {
    #[doc = "A readonly collection of private endpoint connections created on the disk. Currently only one endpoint connection is supported."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "The disk access resource provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The time when the disk access was created."]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<time::OffsetDateTime>,
}
impl DiskAccessProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Used for updating a disk access resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskAccessUpdate {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DiskAccessUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "disk encryption set resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskEncryptionSet {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The managed identity for the disk encryption set. It should be given permission on the key vault before it can be used to encrypt disks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<EncryptionSetIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EncryptionSetProperties>,
}
impl DiskEncryptionSet {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            identity: None,
            properties: None,
        }
    }
}
#[doc = "The List disk encryption set operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskEncryptionSetList {
    #[doc = "A list of disk encryption sets."]
    pub value: Vec<DiskEncryptionSet>,
    #[doc = "The uri to fetch the next page of disk encryption sets. Call ListNext() with this to fetch the next page of disk encryption sets."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiskEncryptionSetList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DiskEncryptionSetList {
    pub fn new(value: Vec<DiskEncryptionSet>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the parameter of customer managed disk encryption set resource id that can be specified for disk. <br><br> NOTE: The disk encryption set resource id can only be specified for managed disk. Please refer https://aka.ms/mdssewithcmkoverview for more details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskEncryptionSetParameters {
    #[serde(flatten)]
    pub sub_resource: SubResource,
}
impl DiskEncryptionSetParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of key used to encrypt the data of the disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiskEncryptionSetType")]
pub enum DiskEncryptionSetType {
    EncryptionAtRestWithCustomerKey,
    EncryptionAtRestWithPlatformAndCustomerKeys,
    ConfidentialVmEncryptedWithCustomerKey,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DiskEncryptionSetType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DiskEncryptionSetType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DiskEncryptionSetType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EncryptionAtRestWithCustomerKey => {
                serializer.serialize_unit_variant("DiskEncryptionSetType", 0u32, "EncryptionAtRestWithCustomerKey")
            }
            Self::EncryptionAtRestWithPlatformAndCustomerKeys => {
                serializer.serialize_unit_variant("DiskEncryptionSetType", 1u32, "EncryptionAtRestWithPlatformAndCustomerKeys")
            }
            Self::ConfidentialVmEncryptedWithCustomerKey => {
                serializer.serialize_unit_variant("DiskEncryptionSetType", 2u32, "ConfidentialVmEncryptedWithCustomerKey")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "disk encryption set update resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskEncryptionSetUpdate {
    #[doc = "disk encryption set resource update properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskEncryptionSetUpdateProperties>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The managed identity for the disk encryption set. It should be given permission on the key vault before it can be used to encrypt disks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<EncryptionSetIdentity>,
}
impl DiskEncryptionSetUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "disk encryption set resource update properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskEncryptionSetUpdateProperties {
    #[doc = "The type of key used to encrypt the data of the disk."]
    #[serde(rename = "encryptionType", default, skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<DiskEncryptionSetType>,
    #[doc = "Key Vault Key Url to be used for server side encryption of Managed Disks and Snapshots"]
    #[serde(rename = "activeKey", default, skip_serializing_if = "Option::is_none")]
    pub active_key: Option<KeyForDiskEncryptionSet>,
    #[doc = "Set this flag to true to enable auto-updating of this disk encryption set to the latest key version."]
    #[serde(rename = "rotationToLatestKeyVersionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub rotation_to_latest_key_version_enabled: Option<bool>,
}
impl DiskEncryptionSetUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Encryption Settings for a Disk"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskEncryptionSettings {
    #[doc = "Describes a reference to Key Vault Secret"]
    #[serde(rename = "diskEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_key: Option<KeyVaultSecretReference>,
    #[doc = "Describes a reference to Key Vault Key"]
    #[serde(rename = "keyEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub key_encryption_key: Option<KeyVaultKeyReference>,
    #[doc = "Specifies whether disk encryption should be enabled on the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl DiskEncryptionSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the disk image encryption base class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskImageEncryption {
    #[doc = "A relative URI containing the resource ID of the disk encryption set."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
}
impl DiskImageEncryption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The instance view of the disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskInstanceView {
    #[doc = "The disk name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the encryption settings for the OS Disk. <br><br> Minimum api-version: 2015-06-15"]
    #[serde(rename = "encryptionSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub encryption_settings: Vec<DiskEncryptionSettings>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<InstanceViewStatus>,
}
impl DiskInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Disks operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskList {
    #[doc = "A list of disks."]
    pub value: Vec<Disk>,
    #[doc = "The uri to fetch the next page of disks. Call ListNext() with this to fetch the next page of disks."]
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
    pub fn new(value: Vec<Disk>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Disk resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskProperties {
    #[doc = "The time when the disk was created."]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<time::OffsetDateTime>,
    #[doc = "The Operating System type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<disk_properties::OsType>,
    #[doc = "The hypervisor generation of the Virtual Machine. Applicable to OS disks only."]
    #[serde(rename = "hyperVGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<disk_properties::HyperVGeneration>,
    #[doc = "Used for establishing the purchase context of any 3rd Party artifact through MarketPlace."]
    #[serde(rename = "purchasePlan", default, skip_serializing_if = "Option::is_none")]
    pub purchase_plan: Option<PurchasePlan>,
    #[doc = "List of supported capabilities persisted on the disk resource for VM use."]
    #[serde(rename = "supportedCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub supported_capabilities: Option<SupportedCapabilities>,
    #[doc = "Data used when creating a disk."]
    #[serde(rename = "creationData")]
    pub creation_data: CreationData,
    #[doc = "If creationData.createOption is Empty, this field is mandatory and it indicates the size of the disk to create. If this field is present for updates or creation with other options, it indicates a resize. Resizes are only allowed if the disk is not attached to a running VM, and can only increase the disk's size."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "The size of the disk in bytes. This field is read only."]
    #[serde(rename = "diskSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_bytes: Option<i64>,
    #[doc = "Unique Guid identifying the resource."]
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[doc = "Encryption settings for disk or snapshot"]
    #[serde(rename = "encryptionSettingsCollection", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
    #[doc = "The disk provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The number of IOPS allowed for this disk; only settable for UltraSSD disks. One operation can transfer between 4k and 256k bytes."]
    #[serde(rename = "diskIOPSReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_write: Option<i64>,
    #[doc = "The bandwidth allowed for this disk; only settable for UltraSSD disks. MBps means millions of bytes per second - MB here uses the ISO notation, of powers of 10."]
    #[serde(rename = "diskMBpsReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_write: Option<i64>,
    #[doc = "The total number of IOPS that will be allowed across all VMs mounting the shared disk as ReadOnly. One operation can transfer between 4k and 256k bytes."]
    #[serde(rename = "diskIOPSReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_only: Option<i64>,
    #[doc = "The total throughput (MBps) that will be allowed across all VMs mounting the shared disk as ReadOnly. MBps means millions of bytes per second - MB here uses the ISO notation, of powers of 10."]
    #[serde(rename = "diskMBpsReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_only: Option<i64>,
    #[doc = "This enumerates the possible state of the disk."]
    #[serde(rename = "diskState", default, skip_serializing_if = "Option::is_none")]
    pub disk_state: Option<DiskState>,
    #[doc = "Encryption at rest settings for disk or snapshot"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "The maximum number of VMs that can attach to the disk at the same time. Value greater than one indicates a disk that can be mounted on multiple VMs at the same time."]
    #[serde(rename = "maxShares", default, skip_serializing_if = "Option::is_none")]
    pub max_shares: Option<i32>,
    #[doc = "Details of the list of all VMs that have the disk attached. maxShares should be set to a value greater than one for disks to allow attaching them to multiple VMs."]
    #[serde(rename = "shareInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub share_info: Vec<ShareInfoElement>,
    #[doc = "Policy for accessing the disk via network."]
    #[serde(rename = "networkAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_access_policy: Option<NetworkAccessPolicy>,
    #[doc = "ARM id of the DiskAccess resource for using private endpoints on disks."]
    #[serde(rename = "diskAccessId", default, skip_serializing_if = "Option::is_none")]
    pub disk_access_id: Option<String>,
    #[doc = "Performance tier of the disk (e.g, P4, S10) as described here: https://azure.microsoft.com/en-us/pricing/details/managed-disks/. Does not apply to Ultra disks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "Set to true to enable bursting beyond the provisioned performance target of the disk. Bursting is disabled by default. Does not apply to Ultra disks."]
    #[serde(rename = "burstingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub bursting_enabled: Option<bool>,
    #[doc = "Properties of the disk for which update is pending."]
    #[serde(rename = "propertyUpdatesInProgress", default, skip_serializing_if = "Option::is_none")]
    pub property_updates_in_progress: Option<PropertyUpdatesInProgress>,
    #[doc = "Indicates the OS on a disk supports hibernation."]
    #[serde(rename = "supportsHibernation", default, skip_serializing_if = "Option::is_none")]
    pub supports_hibernation: Option<bool>,
    #[doc = "Contains the security related information for the resource."]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<DiskSecurityProfile>,
    #[doc = "Percentage complete for the background copy when a resource is created via the CopyStart operation."]
    #[serde(rename = "completionPercent", default, skip_serializing_if = "Option::is_none")]
    pub completion_percent: Option<f64>,
    #[doc = "Policy for controlling export on the disk."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
    #[doc = "Additional authentication requirements when exporting or uploading to a disk or snapshot."]
    #[serde(rename = "dataAccessAuthMode", default, skip_serializing_if = "Option::is_none")]
    pub data_access_auth_mode: Option<DataAccessAuthMode>,
}
impl DiskProperties {
    pub fn new(creation_data: CreationData) -> Self {
        Self {
            time_created: None,
            os_type: None,
            hyper_v_generation: None,
            purchase_plan: None,
            supported_capabilities: None,
            creation_data,
            disk_size_gb: None,
            disk_size_bytes: None,
            unique_id: None,
            encryption_settings_collection: None,
            provisioning_state: None,
            disk_iops_read_write: None,
            disk_m_bps_read_write: None,
            disk_iops_read_only: None,
            disk_m_bps_read_only: None,
            disk_state: None,
            encryption: None,
            max_shares: None,
            share_info: Vec::new(),
            network_access_policy: None,
            disk_access_id: None,
            tier: None,
            bursting_enabled: None,
            property_updates_in_progress: None,
            supports_hibernation: None,
            security_profile: None,
            completion_percent: None,
            public_network_access: None,
            data_access_auth_mode: None,
        }
    }
}
pub mod disk_properties {
    use super::*;
    #[doc = "The Operating System type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[doc = "The hypervisor generation of the Virtual Machine. Applicable to OS disks only."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HyperVGeneration")]
    pub enum HyperVGeneration {
        V1,
        V2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HyperVGeneration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HyperVGeneration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HyperVGeneration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::V1 => serializer.serialize_unit_variant("HyperVGeneration", 0u32, "V1"),
                Self::V2 => serializer.serialize_unit_variant("HyperVGeneration", 1u32, "V2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of disk restore point"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskRestorePoint {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[doc = "Properties of an incremental disk restore point"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskRestorePointProperties>,
}
impl DiskRestorePoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The instance view of a disk restore point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskRestorePointInstanceView {
    #[doc = "Disk restore point Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The instance view of a disk restore point."]
    #[serde(rename = "replicationStatus", default, skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<DiskRestorePointReplicationStatus>,
}
impl DiskRestorePointInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Disk Restore Points operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskRestorePointList {
    #[doc = "A list of disk restore points."]
    pub value: Vec<DiskRestorePoint>,
    #[doc = "The uri to fetch the next page of disk restore points. Call ListNext() with this to fetch the next page of disk restore points."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiskRestorePointList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DiskRestorePointList {
    pub fn new(value: Vec<DiskRestorePoint>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Properties of an incremental disk restore point"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskRestorePointProperties {
    #[doc = "The timestamp of restorePoint creation"]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<time::OffsetDateTime>,
    #[doc = "arm id of source disk or source disk restore point."]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
    #[doc = "The Operating System type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<disk_restore_point_properties::OsType>,
    #[doc = "The hypervisor generation of the Virtual Machine. Applicable to OS disks only."]
    #[serde(rename = "hyperVGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<disk_restore_point_properties::HyperVGeneration>,
    #[doc = "Used for establishing the purchase context of any 3rd Party artifact through MarketPlace."]
    #[serde(rename = "purchasePlan", default, skip_serializing_if = "Option::is_none")]
    pub purchase_plan: Option<PurchasePlan>,
    #[doc = "List of supported capabilities persisted on the disk resource for VM use."]
    #[serde(rename = "supportedCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub supported_capabilities: Option<SupportedCapabilities>,
    #[doc = "id of the backing snapshot's MIS family"]
    #[serde(rename = "familyId", default, skip_serializing_if = "Option::is_none")]
    pub family_id: Option<String>,
    #[doc = "unique incarnation id of the source disk"]
    #[serde(rename = "sourceUniqueId", default, skip_serializing_if = "Option::is_none")]
    pub source_unique_id: Option<String>,
    #[doc = "Encryption at rest settings for disk or snapshot"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "Indicates the OS on a disk supports hibernation."]
    #[serde(rename = "supportsHibernation", default, skip_serializing_if = "Option::is_none")]
    pub supports_hibernation: Option<bool>,
    #[doc = "Policy for accessing the disk via network."]
    #[serde(rename = "networkAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_access_policy: Option<NetworkAccessPolicy>,
    #[doc = "Policy for controlling export on the disk."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
    #[doc = "ARM id of the DiskAccess resource for using private endpoints on disks."]
    #[serde(rename = "diskAccessId", default, skip_serializing_if = "Option::is_none")]
    pub disk_access_id: Option<String>,
    #[doc = "Percentage complete for the background copy of disk restore point when source resource is from a different region."]
    #[serde(rename = "completionPercent", default, skip_serializing_if = "Option::is_none")]
    pub completion_percent: Option<f64>,
    #[doc = "Replication state of disk restore point when source resource is from a different region."]
    #[serde(rename = "replicationState", default, skip_serializing_if = "Option::is_none")]
    pub replication_state: Option<String>,
    #[doc = "Location of source disk or source disk restore point when source resource is from a different region."]
    #[serde(rename = "sourceResourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_location: Option<String>,
}
impl DiskRestorePointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod disk_restore_point_properties {
    use super::*;
    #[doc = "The Operating System type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[doc = "The hypervisor generation of the Virtual Machine. Applicable to OS disks only."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HyperVGeneration")]
    pub enum HyperVGeneration {
        V1,
        V2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HyperVGeneration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HyperVGeneration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HyperVGeneration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::V1 => serializer.serialize_unit_variant("HyperVGeneration", 0u32, "V1"),
                Self::V2 => serializer.serialize_unit_variant("HyperVGeneration", 1u32, "V2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The instance view of a disk restore point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskRestorePointReplicationStatus {
    #[doc = "Instance view status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceViewStatus>,
    #[doc = "Replication completion percentage."]
    #[serde(rename = "completionPercent", default, skip_serializing_if = "Option::is_none")]
    pub completion_percent: Option<i32>,
}
impl DiskRestorePointReplicationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the security related information for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskSecurityProfile {
    #[doc = "Specifies the SecurityType of the VM. Applicable for OS disks only."]
    #[serde(rename = "securityType", default, skip_serializing_if = "Option::is_none")]
    pub security_type: Option<DiskSecurityType>,
    #[doc = "ResourceId of the disk encryption set associated to Confidential VM supported disk encrypted with customer managed key"]
    #[serde(rename = "secureVMDiskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub secure_vm_disk_encryption_set_id: Option<String>,
}
impl DiskSecurityProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the SecurityType of the VM. Applicable for OS disks only."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiskSecurityType")]
pub enum DiskSecurityType {
    TrustedLaunch,
    #[serde(rename = "ConfidentialVM_VMGuestStateOnlyEncryptedWithPlatformKey")]
    ConfidentialVmVmGuestStateOnlyEncryptedWithPlatformKey,
    #[serde(rename = "ConfidentialVM_DiskEncryptedWithPlatformKey")]
    ConfidentialVmDiskEncryptedWithPlatformKey,
    #[serde(rename = "ConfidentialVM_DiskEncryptedWithCustomerKey")]
    ConfidentialVmDiskEncryptedWithCustomerKey,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DiskSecurityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DiskSecurityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DiskSecurityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::TrustedLaunch => serializer.serialize_unit_variant("DiskSecurityType", 0u32, "TrustedLaunch"),
            Self::ConfidentialVmVmGuestStateOnlyEncryptedWithPlatformKey => {
                serializer.serialize_unit_variant("DiskSecurityType", 1u32, "ConfidentialVM_VMGuestStateOnlyEncryptedWithPlatformKey")
            }
            Self::ConfidentialVmDiskEncryptedWithPlatformKey => {
                serializer.serialize_unit_variant("DiskSecurityType", 2u32, "ConfidentialVM_DiskEncryptedWithPlatformKey")
            }
            Self::ConfidentialVmDiskEncryptedWithCustomerKey => {
                serializer.serialize_unit_variant("DiskSecurityType", 3u32, "ConfidentialVM_DiskEncryptedWithCustomerKey")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The disks sku name. Can be Standard_LRS, Premium_LRS, StandardSSD_LRS, UltraSSD_LRS, Premium_ZRS, or StandardSSD_ZRS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskSku {
    #[doc = "The sku name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<disk_sku::Name>,
    #[doc = "The sku tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl DiskSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod disk_sku {
    use super::*;
    #[doc = "The sku name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
        #[serde(rename = "UltraSSD_LRS")]
        UltraSsdLrs,
        #[serde(rename = "Premium_ZRS")]
        PremiumZrs,
        #[serde(rename = "StandardSSD_ZRS")]
        StandardSsdZrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("Name", 0u32, "Standard_LRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("Name", 1u32, "Premium_LRS"),
                Self::StandardSsdLrs => serializer.serialize_unit_variant("Name", 2u32, "StandardSSD_LRS"),
                Self::UltraSsdLrs => serializer.serialize_unit_variant("Name", 3u32, "UltraSSD_LRS"),
                Self::PremiumZrs => serializer.serialize_unit_variant("Name", 4u32, "Premium_ZRS"),
                Self::StandardSsdZrs => serializer.serialize_unit_variant("Name", 5u32, "StandardSSD_ZRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "This enumerates the possible state of the disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiskState")]
pub enum DiskState {
    Unattached,
    Attached,
    Reserved,
    Frozen,
    #[serde(rename = "ActiveSAS")]
    ActiveSas,
    #[serde(rename = "ActiveSASFrozen")]
    ActiveSasFrozen,
    ReadyToUpload,
    ActiveUpload,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DiskState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DiskState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DiskState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unattached => serializer.serialize_unit_variant("DiskState", 0u32, "Unattached"),
            Self::Attached => serializer.serialize_unit_variant("DiskState", 1u32, "Attached"),
            Self::Reserved => serializer.serialize_unit_variant("DiskState", 2u32, "Reserved"),
            Self::Frozen => serializer.serialize_unit_variant("DiskState", 3u32, "Frozen"),
            Self::ActiveSas => serializer.serialize_unit_variant("DiskState", 4u32, "ActiveSAS"),
            Self::ActiveSasFrozen => serializer.serialize_unit_variant("DiskState", 5u32, "ActiveSASFrozen"),
            Self::ReadyToUpload => serializer.serialize_unit_variant("DiskState", 6u32, "ReadyToUpload"),
            Self::ActiveUpload => serializer.serialize_unit_variant("DiskState", 7u32, "ActiveUpload"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Disk update resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskUpdate {
    #[doc = "Disk resource update properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskUpdateProperties>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The disks sku name. Can be Standard_LRS, Premium_LRS, StandardSSD_LRS, UltraSSD_LRS, Premium_ZRS, or StandardSSD_ZRS."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<DiskSku>,
}
impl DiskUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Disk resource update properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskUpdateProperties {
    #[doc = "the Operating System type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<disk_update_properties::OsType>,
    #[doc = "If creationData.createOption is Empty, this field is mandatory and it indicates the size of the disk to create. If this field is present for updates or creation with other options, it indicates a resize. Resizes are only allowed if the disk is not attached to a running VM, and can only increase the disk's size."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "Encryption settings for disk or snapshot"]
    #[serde(rename = "encryptionSettingsCollection", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
    #[doc = "The number of IOPS allowed for this disk; only settable for UltraSSD disks. One operation can transfer between 4k and 256k bytes."]
    #[serde(rename = "diskIOPSReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_write: Option<i64>,
    #[doc = "The bandwidth allowed for this disk; only settable for UltraSSD disks. MBps means millions of bytes per second - MB here uses the ISO notation, of powers of 10."]
    #[serde(rename = "diskMBpsReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_write: Option<i64>,
    #[doc = "The total number of IOPS that will be allowed across all VMs mounting the shared disk as ReadOnly. One operation can transfer between 4k and 256k bytes."]
    #[serde(rename = "diskIOPSReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_only: Option<i64>,
    #[doc = "The total throughput (MBps) that will be allowed across all VMs mounting the shared disk as ReadOnly. MBps means millions of bytes per second - MB here uses the ISO notation, of powers of 10."]
    #[serde(rename = "diskMBpsReadOnly", default, skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_only: Option<i64>,
    #[doc = "The maximum number of VMs that can attach to the disk at the same time. Value greater than one indicates a disk that can be mounted on multiple VMs at the same time."]
    #[serde(rename = "maxShares", default, skip_serializing_if = "Option::is_none")]
    pub max_shares: Option<i32>,
    #[doc = "Encryption at rest settings for disk or snapshot"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "Policy for accessing the disk via network."]
    #[serde(rename = "networkAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_access_policy: Option<NetworkAccessPolicy>,
    #[doc = "ARM id of the DiskAccess resource for using private endpoints on disks."]
    #[serde(rename = "diskAccessId", default, skip_serializing_if = "Option::is_none")]
    pub disk_access_id: Option<String>,
    #[doc = "Performance tier of the disk (e.g, P4, S10) as described here: https://azure.microsoft.com/en-us/pricing/details/managed-disks/. Does not apply to Ultra disks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "Set to true to enable bursting beyond the provisioned performance target of the disk. Bursting is disabled by default. Does not apply to Ultra disks."]
    #[serde(rename = "burstingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub bursting_enabled: Option<bool>,
    #[doc = "Used for establishing the purchase context of any 3rd Party artifact through MarketPlace."]
    #[serde(rename = "purchasePlan", default, skip_serializing_if = "Option::is_none")]
    pub purchase_plan: Option<PurchasePlan>,
    #[doc = "List of supported capabilities persisted on the disk resource for VM use."]
    #[serde(rename = "supportedCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub supported_capabilities: Option<SupportedCapabilities>,
    #[doc = "Properties of the disk for which update is pending."]
    #[serde(rename = "propertyUpdatesInProgress", default, skip_serializing_if = "Option::is_none")]
    pub property_updates_in_progress: Option<PropertyUpdatesInProgress>,
    #[doc = "Indicates the OS on a disk supports hibernation."]
    #[serde(rename = "supportsHibernation", default, skip_serializing_if = "Option::is_none")]
    pub supports_hibernation: Option<bool>,
    #[doc = "Policy for controlling export on the disk."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
    #[doc = "Additional authentication requirements when exporting or uploading to a disk or snapshot."]
    #[serde(rename = "dataAccessAuthMode", default, skip_serializing_if = "Option::is_none")]
    pub data_access_auth_mode: Option<DataAccessAuthMode>,
}
impl DiskUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod disk_update_properties {
    use super::*;
    #[doc = "the Operating System type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[doc = "Encryption at rest settings for disk or snapshot"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Encryption {
    #[doc = "ResourceId of the disk encryption set to use for enabling encryption at rest."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
    #[doc = "The type of key used to encrypt the data of the disk."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<EncryptionType>,
}
impl Encryption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Optional. Allows users to provide customer managed keys for encrypting the OS and data disks in the gallery artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionImages {
    #[doc = "Contains encryption settings for an OS disk image."]
    #[serde(rename = "osDiskImage", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_image: Option<OsDiskImageEncryption>,
    #[doc = "A list of encryption specifications for data disk images."]
    #[serde(rename = "dataDiskImages", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disk_images: Vec<DataDiskImageEncryption>,
}
impl EncryptionImages {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed identity for the disk encryption set. It should be given permission on the key vault before it can be used to encrypt disks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionSetIdentity {
    #[doc = "The type of Managed Identity used by the DiskEncryptionSet. Only SystemAssigned is supported for new creations. Disk Encryption Sets can be updated with Identity type None during migration of subscription to a new Azure Active Directory tenant; it will cause the encrypted resources to lose access to the keys."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<encryption_set_identity::Type>,
    #[doc = "The object id of the Managed Identity Resource. This will be sent to the RP from ARM via the x-ms-identity-principal-id header in the PUT request if the resource has a systemAssigned(implicit) identity"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id of the Managed Identity Resource. This will be sent to the RP from ARM via the x-ms-client-tenant-id header in the PUT request if the resource has a systemAssigned(implicit) identity"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl EncryptionSetIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption_set_identity {
    use super::*;
    #[doc = "The type of Managed Identity used by the DiskEncryptionSet. Only SystemAssigned is supported for new creations. Disk Encryption Sets can be updated with Identity type None during migration of subscription to a new Azure Active Directory tenant; it will cause the encrypted resources to lose access to the keys."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        SystemAssigned,
        None,
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
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 0u32, "SystemAssigned"),
                Self::None => serializer.serialize_unit_variant("Type", 1u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionSetProperties {
    #[doc = "The type of key used to encrypt the data of the disk."]
    #[serde(rename = "encryptionType", default, skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<DiskEncryptionSetType>,
    #[doc = "Key Vault Key Url to be used for server side encryption of Managed Disks and Snapshots"]
    #[serde(rename = "activeKey", default, skip_serializing_if = "Option::is_none")]
    pub active_key: Option<KeyForDiskEncryptionSet>,
    #[doc = "A readonly collection of key vault keys previously used by this disk encryption set while a key rotation is in progress. It will be empty if there is no ongoing key rotation."]
    #[serde(rename = "previousKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub previous_keys: Vec<KeyForDiskEncryptionSet>,
    #[doc = "The disk encryption set provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Set this flag to true to enable auto-updating of this disk encryption set to the latest key version."]
    #[serde(rename = "rotationToLatestKeyVersionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub rotation_to_latest_key_version_enabled: Option<bool>,
    #[doc = "The time when the active key of this disk encryption set was updated."]
    #[serde(rename = "lastKeyRotationTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub last_key_rotation_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Api error."]
    #[serde(rename = "autoKeyRotationError", default, skip_serializing_if = "Option::is_none")]
    pub auto_key_rotation_error: Option<ApiError>,
}
impl EncryptionSetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Encryption settings for disk or snapshot"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionSettingsCollection {
    #[doc = "Set this flag to true and provide DiskEncryptionKey and optional KeyEncryptionKey to enable encryption. Set this flag to false and remove DiskEncryptionKey and KeyEncryptionKey to disable encryption. If EncryptionSettings is null in the request object, the existing settings remain unchanged."]
    pub enabled: bool,
    #[doc = "A collection of encryption settings, one for each disk volume."]
    #[serde(rename = "encryptionSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub encryption_settings: Vec<EncryptionSettingsElement>,
    #[doc = "Describes what type of encryption is used for the disks. Once this field is set, it cannot be overwritten. '1.0' corresponds to Azure Disk Encryption with AAD app.'1.1' corresponds to Azure Disk Encryption."]
    #[serde(rename = "encryptionSettingsVersion", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings_version: Option<String>,
}
impl EncryptionSettingsCollection {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            encryption_settings: Vec::new(),
            encryption_settings_version: None,
        }
    }
}
#[doc = "Encryption settings for one disk volume."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionSettingsElement {
    #[doc = "Key Vault Secret Url and vault id of the encryption key "]
    #[serde(rename = "diskEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_key: Option<KeyVaultAndSecretReference>,
    #[doc = "Key Vault Key Url and vault id of KeK, KeK is optional and when provided is used to unwrap the encryptionKey"]
    #[serde(rename = "keyEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub key_encryption_key: Option<KeyVaultAndKeyReference>,
}
impl EncryptionSettingsElement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of key used to encrypt the data of the disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EncryptionType")]
pub enum EncryptionType {
    EncryptionAtRestWithPlatformKey,
    EncryptionAtRestWithCustomerKey,
    EncryptionAtRestWithPlatformAndCustomerKeys,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EncryptionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EncryptionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EncryptionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EncryptionAtRestWithPlatformKey => {
                serializer.serialize_unit_variant("EncryptionType", 0u32, "EncryptionAtRestWithPlatformKey")
            }
            Self::EncryptionAtRestWithCustomerKey => {
                serializer.serialize_unit_variant("EncryptionType", 1u32, "EncryptionAtRestWithCustomerKey")
            }
            Self::EncryptionAtRestWithPlatformAndCustomerKeys => {
                serializer.serialize_unit_variant("EncryptionType", 2u32, "EncryptionAtRestWithPlatformAndCustomerKeys")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The complex type of the extended location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocation {
    #[doc = "The name of the extended location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of extendedLocation."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ExtendedLocationType>,
}
impl ExtendedLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of extendedLocation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ExtendedLocationType")]
pub enum ExtendedLocationType {
    EdgeZone,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ExtendedLocationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ExtendedLocationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ExtendedLocationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EdgeZone => serializer.serialize_unit_variant("ExtendedLocationType", 0u32, "EdgeZone"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes a cloud service Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Extension {
    #[doc = "The name of the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Extension Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudServiceExtensionProperties>,
}
impl Extension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the Shared Image Gallery that you want to create or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gallery {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of a Shared Image Gallery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryProperties>,
}
impl Gallery {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "Specifies information about the gallery Application Definition that you want to create or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplication {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of a gallery Application Definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryApplicationProperties>,
}
impl GalleryApplication {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "The List Gallery Applications operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplicationList {
    #[doc = "A list of Gallery Applications."]
    pub value: Vec<GalleryApplication>,
    #[doc = "The uri to fetch the next page of Application Definitions in the Application Gallery. Call ListNext() with this to fetch the next page of gallery Application Definitions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GalleryApplicationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GalleryApplicationList {
    pub fn new(value: Vec<GalleryApplication>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the properties of a gallery Application Definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplicationProperties {
    #[doc = "The description of this gallery Application Definition resource. This property is updatable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Eula agreement for the gallery Application Definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eula: Option<String>,
    #[doc = "The privacy statement uri."]
    #[serde(rename = "privacyStatementUri", default, skip_serializing_if = "Option::is_none")]
    pub privacy_statement_uri: Option<String>,
    #[doc = "The release note uri."]
    #[serde(rename = "releaseNoteUri", default, skip_serializing_if = "Option::is_none")]
    pub release_note_uri: Option<String>,
    #[doc = "The end of life date of the gallery Application Definition. This property can be used for decommissioning purposes. This property is updatable."]
    #[serde(rename = "endOfLifeDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_of_life_date: Option<time::OffsetDateTime>,
    #[doc = "This property allows you to specify the supported type of the OS that application is built for. <br><br> Possible values are: <br><br> **Windows** <br><br> **Linux**"]
    #[serde(rename = "supportedOSType")]
    pub supported_os_type: gallery_application_properties::SupportedOsType,
}
impl GalleryApplicationProperties {
    pub fn new(supported_os_type: gallery_application_properties::SupportedOsType) -> Self {
        Self {
            description: None,
            eula: None,
            privacy_statement_uri: None,
            release_note_uri: None,
            end_of_life_date: None,
            supported_os_type,
        }
    }
}
pub mod gallery_application_properties {
    use super::*;
    #[doc = "This property allows you to specify the supported type of the OS that application is built for. <br><br> Possible values are: <br><br> **Windows** <br><br> **Linux**"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SupportedOsType {
        Windows,
        Linux,
    }
}
#[doc = "Specifies information about the gallery Application Definition that you want to update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryApplicationUpdate {
    #[serde(flatten)]
    pub update_resource_definition: UpdateResourceDefinition,
    #[doc = "Describes the properties of a gallery Application Definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryApplicationProperties>,
}
impl GalleryApplicationUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the gallery Application Version that you want to create or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplicationVersion {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of a gallery image version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryApplicationVersionProperties>,
}
impl GalleryApplicationVersion {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "The List Gallery Application version operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplicationVersionList {
    #[doc = "A list of gallery Application Versions."]
    pub value: Vec<GalleryApplicationVersion>,
    #[doc = "The uri to fetch the next page of gallery Application Versions. Call ListNext() with this to fetch the next page of gallery Application Versions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GalleryApplicationVersionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GalleryApplicationVersionList {
    pub fn new(value: Vec<GalleryApplicationVersion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the properties of a gallery image version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplicationVersionProperties {
    #[doc = "The publishing profile of a gallery image version."]
    #[serde(rename = "publishingProfile")]
    pub publishing_profile: GalleryApplicationVersionPublishingProfile,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<GalleryProvisioningState>,
    #[doc = "This is the replication status of the gallery image version."]
    #[serde(rename = "replicationStatus", default, skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<ReplicationStatus>,
}
impl GalleryApplicationVersionProperties {
    pub fn new(publishing_profile: GalleryApplicationVersionPublishingProfile) -> Self {
        Self {
            publishing_profile,
            provisioning_state: None,
            replication_status: None,
        }
    }
}
#[doc = "The publishing profile of a gallery image version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryApplicationVersionPublishingProfile {
    #[serde(flatten)]
    pub gallery_artifact_publishing_profile_base: GalleryArtifactPublishingProfileBase,
    #[doc = "The source image from which the Image Version is going to be created."]
    pub source: UserArtifactSource,
    #[serde(rename = "manageActions", default, skip_serializing_if = "Option::is_none")]
    pub manage_actions: Option<UserArtifactManage>,
    #[doc = "Additional settings for the VM app that contains the target package and config file name when it is deployed to target VM or VM scale set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<UserArtifactSettings>,
    #[doc = "Optional. Additional settings to pass to the vm-application-manager extension. For advanced use only."]
    #[serde(rename = "advancedSettings", default, skip_serializing_if = "Option::is_none")]
    pub advanced_settings: Option<serde_json::Value>,
    #[doc = "Optional. Whether or not this application reports health."]
    #[serde(rename = "enableHealthCheck", default, skip_serializing_if = "Option::is_none")]
    pub enable_health_check: Option<bool>,
}
impl GalleryApplicationVersionPublishingProfile {
    pub fn new(source: UserArtifactSource) -> Self {
        Self {
            gallery_artifact_publishing_profile_base: GalleryArtifactPublishingProfileBase::default(),
            source,
            manage_actions: None,
            settings: None,
            advanced_settings: None,
            enable_health_check: None,
        }
    }
}
#[doc = "Specifies information about the gallery Application Version that you want to update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryApplicationVersionUpdate {
    #[serde(flatten)]
    pub update_resource_definition: UpdateResourceDefinition,
    #[doc = "Describes the properties of a gallery image version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryApplicationVersionProperties>,
}
impl GalleryApplicationVersionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the basic gallery artifact publishing profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryArtifactPublishingProfileBase {
    #[doc = "The target regions where the Image Version is going to be replicated to. This property is updatable."]
    #[serde(rename = "targetRegions", default, skip_serializing_if = "Vec::is_empty")]
    pub target_regions: Vec<TargetRegion>,
    #[doc = "The number of replicas of the Image Version to be created per region. This property would take effect for a region when regionalReplicaCount is not specified. This property is updatable."]
    #[serde(rename = "replicaCount", default, skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
    #[doc = "If set to true, Virtual Machines deployed from the latest version of the Image Definition won't use this Image Version."]
    #[serde(rename = "excludeFromLatest", default, skip_serializing_if = "Option::is_none")]
    pub exclude_from_latest: Option<bool>,
    #[doc = "The timestamp for when the gallery image version is published."]
    #[serde(rename = "publishedDate", default, with = "azure_core::date::rfc3339::option")]
    pub published_date: Option<time::OffsetDateTime>,
    #[doc = "The end of life date of the gallery image version. This property can be used for decommissioning purposes. This property is updatable."]
    #[serde(rename = "endOfLifeDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_of_life_date: Option<time::OffsetDateTime>,
    #[doc = "Specifies the storage account type to be used to store the image. This property is not updatable."]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<gallery_artifact_publishing_profile_base::StorageAccountType>,
    #[doc = "Optional parameter which specifies the mode to be used for replication. This property is not updatable."]
    #[serde(rename = "replicationMode", default, skip_serializing_if = "Option::is_none")]
    pub replication_mode: Option<gallery_artifact_publishing_profile_base::ReplicationMode>,
    #[doc = "The target extended locations where the Image Version is going to be replicated to. This property is updatable."]
    #[serde(rename = "targetExtendedLocations", default, skip_serializing_if = "Vec::is_empty")]
    pub target_extended_locations: Vec<GalleryTargetExtendedLocation>,
}
impl GalleryArtifactPublishingProfileBase {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod gallery_artifact_publishing_profile_base {
    use super::*;
    #[doc = "Specifies the storage account type to be used to store the image. This property is not updatable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageAccountType")]
    pub enum StorageAccountType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageAccountType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageAccountType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageAccountType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("StorageAccountType", 0u32, "Standard_LRS"),
                Self::StandardZrs => serializer.serialize_unit_variant("StorageAccountType", 1u32, "Standard_ZRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("StorageAccountType", 2u32, "Premium_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Optional parameter which specifies the mode to be used for replication. This property is not updatable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReplicationMode")]
    pub enum ReplicationMode {
        Full,
        Shallow,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReplicationMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReplicationMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReplicationMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Full => serializer.serialize_unit_variant("ReplicationMode", 0u32, "Full"),
                Self::Shallow => serializer.serialize_unit_variant("ReplicationMode", 1u32, "Shallow"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The source image from which the Image Version is going to be created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryArtifactSource {
    #[doc = "The managed artifact."]
    #[serde(rename = "managedImage")]
    pub managed_image: ManagedArtifact,
}
impl GalleryArtifactSource {
    pub fn new(managed_image: ManagedArtifact) -> Self {
        Self { managed_image }
    }
}
#[doc = "The gallery artifact version source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryArtifactVersionSource {
    #[doc = "The id of the gallery artifact version source. Can specify a disk uri, snapshot uri, user image or storage account resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The uri of the gallery artifact version source. Currently used to specify vhd/blob source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl GalleryArtifactVersionSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the data disk image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryDataDiskImage {
    #[serde(flatten)]
    pub gallery_disk_image: GalleryDiskImage,
    #[doc = "This property specifies the logical unit number of the data disk. This value is used to identify data disks within the Virtual Machine and therefore must be unique for each data disk attached to the Virtual Machine."]
    pub lun: i32,
}
impl GalleryDataDiskImage {
    pub fn new(lun: i32) -> Self {
        Self {
            gallery_disk_image: GalleryDiskImage::default(),
            lun,
        }
    }
}
#[doc = "This is the disk image base class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryDiskImage {
    #[doc = "This property indicates the size of the VHD to be created."]
    #[serde(rename = "sizeInGB", default, skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i32>,
    #[doc = "The host caching of the disk. Valid values are 'None', 'ReadOnly', and 'ReadWrite'"]
    #[serde(rename = "hostCaching", default, skip_serializing_if = "Option::is_none")]
    pub host_caching: Option<gallery_disk_image::HostCaching>,
    #[doc = "The gallery artifact version source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<GalleryArtifactVersionSource>,
}
impl GalleryDiskImage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod gallery_disk_image {
    use super::*;
    #[doc = "The host caching of the disk. Valid values are 'None', 'ReadOnly', and 'ReadWrite'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HostCaching {
        None,
        ReadOnly,
        ReadWrite,
    }
}
#[doc = "The name of the extended location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryExtendedLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "It is type of the extended location."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<GalleryExtendedLocationType>,
}
impl GalleryExtendedLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "It is type of the extended location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "GalleryExtendedLocationType")]
pub enum GalleryExtendedLocationType {
    EdgeZone,
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for GalleryExtendedLocationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for GalleryExtendedLocationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for GalleryExtendedLocationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EdgeZone => serializer.serialize_unit_variant("GalleryExtendedLocationType", 0u32, "EdgeZone"),
            Self::Unknown => serializer.serialize_unit_variant("GalleryExtendedLocationType", 1u32, "Unknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the gallery unique name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryIdentifier {
    #[doc = "The unique name of the Shared Image Gallery. This name is generated automatically by Azure."]
    #[serde(rename = "uniqueName", default, skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
}
impl GalleryIdentifier {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the gallery image definition that you want to create or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImage {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of a gallery image definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryImageProperties>,
}
impl GalleryImage {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "A feature for gallery image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImageFeature {
    #[doc = "The name of the gallery image feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the gallery image feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl GalleryImageFeature {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the gallery image definition identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageIdentifier {
    #[doc = "The name of the gallery image definition publisher."]
    pub publisher: String,
    #[doc = "The name of the gallery image definition offer."]
    pub offer: String,
    #[doc = "The name of the gallery image definition SKU."]
    pub sku: String,
}
impl GalleryImageIdentifier {
    pub fn new(publisher: String, offer: String, sku: String) -> Self {
        Self { publisher, offer, sku }
    }
}
#[doc = "The List Gallery Images operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageList {
    #[doc = "A list of Shared Image Gallery images."]
    pub value: Vec<GalleryImage>,
    #[doc = "The uri to fetch the next page of Image Definitions in the Shared Image Gallery. Call ListNext() with this to fetch the next page of gallery image definitions."]
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
    pub fn new(value: Vec<GalleryImage>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the properties of a gallery image definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageProperties {
    #[doc = "The description of this gallery image definition resource. This property is updatable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Eula agreement for the gallery image definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eula: Option<String>,
    #[doc = "The privacy statement uri."]
    #[serde(rename = "privacyStatementUri", default, skip_serializing_if = "Option::is_none")]
    pub privacy_statement_uri: Option<String>,
    #[doc = "The release note uri."]
    #[serde(rename = "releaseNoteUri", default, skip_serializing_if = "Option::is_none")]
    pub release_note_uri: Option<String>,
    #[doc = "This property allows you to specify the type of the OS that is included in the disk when creating a VM from a managed image. <br><br> Possible values are: <br><br> **Windows** <br><br> **Linux**"]
    #[serde(rename = "osType")]
    pub os_type: gallery_image_properties::OsType,
    #[doc = "This property allows the user to specify whether the virtual machines created under this image are 'Generalized' or 'Specialized'."]
    #[serde(rename = "osState")]
    pub os_state: gallery_image_properties::OsState,
    #[doc = "The hypervisor generation of the Virtual Machine. Applicable to OS disks only."]
    #[serde(rename = "hyperVGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<gallery_image_properties::HyperVGeneration>,
    #[doc = "The end of life date of the gallery image definition. This property can be used for decommissioning purposes. This property is updatable."]
    #[serde(rename = "endOfLifeDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_of_life_date: Option<time::OffsetDateTime>,
    #[doc = "This is the gallery image definition identifier."]
    pub identifier: GalleryImageIdentifier,
    #[doc = "The properties describe the recommended machine configuration for this Image Definition. These properties are updatable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommended: Option<RecommendedMachineConfiguration>,
    #[doc = "Describes the disallowed disk types."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disallowed: Option<Disallowed>,
    #[doc = "Describes the gallery image definition purchase plan. This is used by marketplace images."]
    #[serde(rename = "purchasePlan", default, skip_serializing_if = "Option::is_none")]
    pub purchase_plan: Option<ImagePurchasePlan>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<GalleryProvisioningState>,
    #[doc = "A list of gallery image features."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<GalleryImageFeature>,
    #[doc = "The architecture of the image. Applicable to OS disks only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<Architecture>,
}
impl GalleryImageProperties {
    pub fn new(
        os_type: gallery_image_properties::OsType,
        os_state: gallery_image_properties::OsState,
        identifier: GalleryImageIdentifier,
    ) -> Self {
        Self {
            description: None,
            eula: None,
            privacy_statement_uri: None,
            release_note_uri: None,
            os_type,
            os_state,
            hyper_v_generation: None,
            end_of_life_date: None,
            identifier,
            recommended: None,
            disallowed: None,
            purchase_plan: None,
            provisioning_state: None,
            features: Vec::new(),
            architecture: None,
        }
    }
}
pub mod gallery_image_properties {
    use super::*;
    #[doc = "This property allows you to specify the type of the OS that is included in the disk when creating a VM from a managed image. <br><br> Possible values are: <br><br> **Windows** <br><br> **Linux**"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[doc = "This property allows the user to specify whether the virtual machines created under this image are 'Generalized' or 'Specialized'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsState {
        Generalized,
        Specialized,
    }
    #[doc = "The hypervisor generation of the Virtual Machine. Applicable to OS disks only."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HyperVGeneration")]
    pub enum HyperVGeneration {
        V1,
        V2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HyperVGeneration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HyperVGeneration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HyperVGeneration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::V1 => serializer.serialize_unit_variant("HyperVGeneration", 0u32, "V1"),
                Self::V2 => serializer.serialize_unit_variant("HyperVGeneration", 1u32, "V2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies information about the gallery image definition that you want to update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImageUpdate {
    #[serde(flatten)]
    pub update_resource_definition: UpdateResourceDefinition,
    #[doc = "Describes the properties of a gallery image definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryImageProperties>,
}
impl GalleryImageUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the gallery image version that you want to create or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageVersion {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of a gallery image version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryImageVersionProperties>,
}
impl GalleryImageVersion {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "The List Gallery Image version operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageVersionList {
    #[doc = "A list of gallery image versions."]
    pub value: Vec<GalleryImageVersion>,
    #[doc = "The uri to fetch the next page of gallery image versions. Call ListNext() with this to fetch the next page of gallery image versions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GalleryImageVersionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GalleryImageVersionList {
    pub fn new(value: Vec<GalleryImageVersion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the properties of a gallery image version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryImageVersionProperties {
    #[doc = "The publishing profile of a gallery image Version."]
    #[serde(rename = "publishingProfile", default, skip_serializing_if = "Option::is_none")]
    pub publishing_profile: Option<GalleryImageVersionPublishingProfile>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<GalleryProvisioningState>,
    #[doc = "This is the storage profile of a Gallery Image Version."]
    #[serde(rename = "storageProfile")]
    pub storage_profile: GalleryImageVersionStorageProfile,
    #[doc = "This is the replication status of the gallery image version."]
    #[serde(rename = "replicationStatus", default, skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<ReplicationStatus>,
}
impl GalleryImageVersionProperties {
    pub fn new(storage_profile: GalleryImageVersionStorageProfile) -> Self {
        Self {
            publishing_profile: None,
            provisioning_state: None,
            storage_profile,
            replication_status: None,
        }
    }
}
#[doc = "The publishing profile of a gallery image Version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImageVersionPublishingProfile {
    #[serde(flatten)]
    pub gallery_artifact_publishing_profile_base: GalleryArtifactPublishingProfileBase,
}
impl GalleryImageVersionPublishingProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the storage profile of a Gallery Image Version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImageVersionStorageProfile {
    #[doc = "The gallery artifact version source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<GalleryArtifactVersionSource>,
    #[doc = "This is the OS disk image."]
    #[serde(rename = "osDiskImage", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_image: Option<GalleryOsDiskImage>,
    #[doc = "A list of data disk images."]
    #[serde(rename = "dataDiskImages", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disk_images: Vec<GalleryDataDiskImage>,
}
impl GalleryImageVersionStorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the gallery image version that you want to update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryImageVersionUpdate {
    #[serde(flatten)]
    pub update_resource_definition: UpdateResourceDefinition,
    #[doc = "Describes the properties of a gallery image version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryImageVersionProperties>,
}
impl GalleryImageVersionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Galleries operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryList {
    #[doc = "A list of galleries."]
    pub value: Vec<Gallery>,
    #[doc = "The uri to fetch the next page of galleries. Call ListNext() with this to fetch the next page of galleries."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GalleryList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GalleryList {
    pub fn new(value: Vec<Gallery>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "This is the OS disk image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryOsDiskImage {
    #[serde(flatten)]
    pub gallery_disk_image: GalleryDiskImage,
}
impl GalleryOsDiskImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Shared Image Gallery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryProperties {
    #[doc = "The description of this Shared Image Gallery resource. This property is updatable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Describes the gallery unique name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<GalleryIdentifier>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<GalleryProvisioningState>,
    #[doc = "Profile for gallery sharing to subscription or tenant"]
    #[serde(rename = "sharingProfile", default, skip_serializing_if = "Option::is_none")]
    pub sharing_profile: Option<SharingProfile>,
    #[doc = "Contains information about the soft deletion policy of the gallery."]
    #[serde(rename = "softDeletePolicy", default, skip_serializing_if = "Option::is_none")]
    pub soft_delete_policy: Option<SoftDeletePolicy>,
    #[doc = "Sharing status of current gallery."]
    #[serde(rename = "sharingStatus", default, skip_serializing_if = "Option::is_none")]
    pub sharing_status: Option<SharingStatus>,
}
impl GalleryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provisioning state, which only appears in the response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "GalleryProvisioningState")]
pub enum GalleryProvisioningState {
    Creating,
    Updating,
    Failed,
    Succeeded,
    Deleting,
    Migrating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for GalleryProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for GalleryProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for GalleryProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Creating => serializer.serialize_unit_variant("GalleryProvisioningState", 0u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("GalleryProvisioningState", 1u32, "Updating"),
            Self::Failed => serializer.serialize_unit_variant("GalleryProvisioningState", 2u32, "Failed"),
            Self::Succeeded => serializer.serialize_unit_variant("GalleryProvisioningState", 3u32, "Succeeded"),
            Self::Deleting => serializer.serialize_unit_variant("GalleryProvisioningState", 4u32, "Deleting"),
            Self::Migrating => serializer.serialize_unit_variant("GalleryProvisioningState", 5u32, "Migrating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryTargetExtendedLocation {
    #[doc = "The name of the region."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The name of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<GalleryExtendedLocation>,
    #[doc = "The number of replicas of the Image Version to be created per extended location. This property is updatable."]
    #[serde(rename = "extendedLocationReplicaCount", default, skip_serializing_if = "Option::is_none")]
    pub extended_location_replica_count: Option<i32>,
    #[doc = "Specifies the storage account type to be used to store the image. This property is not updatable."]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<gallery_target_extended_location::StorageAccountType>,
    #[doc = "Optional. Allows users to provide customer managed keys for encrypting the OS and data disks in the gallery artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionImages>,
}
impl GalleryTargetExtendedLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod gallery_target_extended_location {
    use super::*;
    #[doc = "Specifies the storage account type to be used to store the image. This property is not updatable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageAccountType")]
    pub enum StorageAccountType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageAccountType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageAccountType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageAccountType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("StorageAccountType", 0u32, "Standard_LRS"),
                Self::StandardZrs => serializer.serialize_unit_variant("StorageAccountType", 1u32, "Standard_ZRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("StorageAccountType", 2u32, "Premium_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies information about the Shared Image Gallery that you want to update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryUpdate {
    #[serde(flatten)]
    pub update_resource_definition: UpdateResourceDefinition,
    #[doc = "Describes the properties of a Shared Image Gallery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryProperties>,
}
impl GalleryUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data used for requesting a SAS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GrantAccessData {
    pub access: grant_access_data::Access,
    #[doc = "Time duration in seconds until the SAS access expires."]
    #[serde(rename = "durationInSeconds")]
    pub duration_in_seconds: i32,
    #[doc = "Set this flag to true to get additional SAS for VM guest state"]
    #[serde(rename = "getSecureVMGuestStateSAS", default, skip_serializing_if = "Option::is_none")]
    pub get_secure_vm_guest_state_sas: Option<bool>,
}
impl GrantAccessData {
    pub fn new(access: grant_access_data::Access, duration_in_seconds: i32) -> Self {
        Self {
            access,
            duration_in_seconds,
            get_secure_vm_guest_state_sas: None,
        }
    }
}
pub mod grant_access_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Access")]
    pub enum Access {
        None,
        Read,
        Write,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Access {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Access {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Access {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Access", 0u32, "None"),
                Self::Read => serializer.serialize_unit_variant("Access", 1u32, "Read"),
                Self::Write => serializer.serialize_unit_variant("Access", 2u32, "Write"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the hardware settings for the virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HardwareProfile {
    #[doc = "Specifies the size of the virtual machine. <br><br> The enum data type is currently deprecated and will be removed by December 23rd 2023. <br><br> Recommended way to get the list of available sizes is using these APIs: <br><br> [List all available virtual machine sizes in an availability set](https://docs.microsoft.com/rest/api/compute/availabilitysets/listavailablesizes) <br><br> [List all available virtual machine sizes in a region]( https://docs.microsoft.com/rest/api/compute/resourceskus/list) <br><br> [List all available virtual machine sizes for resizing](https://docs.microsoft.com/rest/api/compute/virtualmachines/listavailablesizes). For more information about virtual machine sizes, see [Sizes for virtual machines](https://docs.microsoft.com/azure/virtual-machines/sizes). <br><br> The available VM sizes depend on region and availability set."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<hardware_profile::VmSize>,
    #[doc = "Specifies VM Size Property settings on the virtual machine."]
    #[serde(rename = "vmSizeProperties", default, skip_serializing_if = "Option::is_none")]
    pub vm_size_properties: Option<VmSizeProperties>,
}
impl HardwareProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hardware_profile {
    use super::*;
    #[doc = "Specifies the size of the virtual machine. <br><br> The enum data type is currently deprecated and will be removed by December 23rd 2023. <br><br> Recommended way to get the list of available sizes is using these APIs: <br><br> [List all available virtual machine sizes in an availability set](https://docs.microsoft.com/rest/api/compute/availabilitysets/listavailablesizes) <br><br> [List all available virtual machine sizes in a region]( https://docs.microsoft.com/rest/api/compute/resourceskus/list) <br><br> [List all available virtual machine sizes for resizing](https://docs.microsoft.com/rest/api/compute/virtualmachines/listavailablesizes). For more information about virtual machine sizes, see [Sizes for virtual machines](https://docs.microsoft.com/azure/virtual-machines/sizes). <br><br> The available VM sizes depend on region and availability set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VmSize")]
    pub enum VmSize {
        #[serde(rename = "Basic_A0")]
        BasicA0,
        #[serde(rename = "Basic_A1")]
        BasicA1,
        #[serde(rename = "Basic_A2")]
        BasicA2,
        #[serde(rename = "Basic_A3")]
        BasicA3,
        #[serde(rename = "Basic_A4")]
        BasicA4,
        #[serde(rename = "Standard_A0")]
        StandardA0,
        #[serde(rename = "Standard_A1")]
        StandardA1,
        #[serde(rename = "Standard_A2")]
        StandardA2,
        #[serde(rename = "Standard_A3")]
        StandardA3,
        #[serde(rename = "Standard_A4")]
        StandardA4,
        #[serde(rename = "Standard_A5")]
        StandardA5,
        #[serde(rename = "Standard_A6")]
        StandardA6,
        #[serde(rename = "Standard_A7")]
        StandardA7,
        #[serde(rename = "Standard_A8")]
        StandardA8,
        #[serde(rename = "Standard_A9")]
        StandardA9,
        #[serde(rename = "Standard_A10")]
        StandardA10,
        #[serde(rename = "Standard_A11")]
        StandardA11,
        #[serde(rename = "Standard_A1_v2")]
        StandardA1V2,
        #[serde(rename = "Standard_A2_v2")]
        StandardA2V2,
        #[serde(rename = "Standard_A4_v2")]
        StandardA4V2,
        #[serde(rename = "Standard_A8_v2")]
        StandardA8V2,
        #[serde(rename = "Standard_A2m_v2")]
        StandardA2mV2,
        #[serde(rename = "Standard_A4m_v2")]
        StandardA4mV2,
        #[serde(rename = "Standard_A8m_v2")]
        StandardA8mV2,
        #[serde(rename = "Standard_B1s")]
        StandardB1s,
        #[serde(rename = "Standard_B1ms")]
        StandardB1ms,
        #[serde(rename = "Standard_B2s")]
        StandardB2s,
        #[serde(rename = "Standard_B2ms")]
        StandardB2ms,
        #[serde(rename = "Standard_B4ms")]
        StandardB4ms,
        #[serde(rename = "Standard_B8ms")]
        StandardB8ms,
        #[serde(rename = "Standard_D1")]
        StandardD1,
        #[serde(rename = "Standard_D2")]
        StandardD2,
        #[serde(rename = "Standard_D3")]
        StandardD3,
        #[serde(rename = "Standard_D4")]
        StandardD4,
        #[serde(rename = "Standard_D11")]
        StandardD11,
        #[serde(rename = "Standard_D12")]
        StandardD12,
        #[serde(rename = "Standard_D13")]
        StandardD13,
        #[serde(rename = "Standard_D14")]
        StandardD14,
        #[serde(rename = "Standard_D1_v2")]
        StandardD1V2,
        #[serde(rename = "Standard_D2_v2")]
        StandardD2V2,
        #[serde(rename = "Standard_D3_v2")]
        StandardD3V2,
        #[serde(rename = "Standard_D4_v2")]
        StandardD4V2,
        #[serde(rename = "Standard_D5_v2")]
        StandardD5V2,
        #[serde(rename = "Standard_D2_v3")]
        StandardD2V3,
        #[serde(rename = "Standard_D4_v3")]
        StandardD4V3,
        #[serde(rename = "Standard_D8_v3")]
        StandardD8V3,
        #[serde(rename = "Standard_D16_v3")]
        StandardD16V3,
        #[serde(rename = "Standard_D32_v3")]
        StandardD32V3,
        #[serde(rename = "Standard_D64_v3")]
        StandardD64V3,
        #[serde(rename = "Standard_D2s_v3")]
        StandardD2sV3,
        #[serde(rename = "Standard_D4s_v3")]
        StandardD4sV3,
        #[serde(rename = "Standard_D8s_v3")]
        StandardD8sV3,
        #[serde(rename = "Standard_D16s_v3")]
        StandardD16sV3,
        #[serde(rename = "Standard_D32s_v3")]
        StandardD32sV3,
        #[serde(rename = "Standard_D64s_v3")]
        StandardD64sV3,
        #[serde(rename = "Standard_D11_v2")]
        StandardD11V2,
        #[serde(rename = "Standard_D12_v2")]
        StandardD12V2,
        #[serde(rename = "Standard_D13_v2")]
        StandardD13V2,
        #[serde(rename = "Standard_D14_v2")]
        StandardD14V2,
        #[serde(rename = "Standard_D15_v2")]
        StandardD15V2,
        #[serde(rename = "Standard_DS1")]
        StandardDs1,
        #[serde(rename = "Standard_DS2")]
        StandardDs2,
        #[serde(rename = "Standard_DS3")]
        StandardDs3,
        #[serde(rename = "Standard_DS4")]
        StandardDs4,
        #[serde(rename = "Standard_DS11")]
        StandardDs11,
        #[serde(rename = "Standard_DS12")]
        StandardDs12,
        #[serde(rename = "Standard_DS13")]
        StandardDs13,
        #[serde(rename = "Standard_DS14")]
        StandardDs14,
        #[serde(rename = "Standard_DS1_v2")]
        StandardDs1V2,
        #[serde(rename = "Standard_DS2_v2")]
        StandardDs2V2,
        #[serde(rename = "Standard_DS3_v2")]
        StandardDs3V2,
        #[serde(rename = "Standard_DS4_v2")]
        StandardDs4V2,
        #[serde(rename = "Standard_DS5_v2")]
        StandardDs5V2,
        #[serde(rename = "Standard_DS11_v2")]
        StandardDs11V2,
        #[serde(rename = "Standard_DS12_v2")]
        StandardDs12V2,
        #[serde(rename = "Standard_DS13_v2")]
        StandardDs13V2,
        #[serde(rename = "Standard_DS14_v2")]
        StandardDs14V2,
        #[serde(rename = "Standard_DS15_v2")]
        StandardDs15V2,
        #[serde(rename = "Standard_DS13-4_v2")]
        StandardDs134V2,
        #[serde(rename = "Standard_DS13-2_v2")]
        StandardDs132V2,
        #[serde(rename = "Standard_DS14-8_v2")]
        StandardDs148V2,
        #[serde(rename = "Standard_DS14-4_v2")]
        StandardDs144V2,
        #[serde(rename = "Standard_E2_v3")]
        StandardE2V3,
        #[serde(rename = "Standard_E4_v3")]
        StandardE4V3,
        #[serde(rename = "Standard_E8_v3")]
        StandardE8V3,
        #[serde(rename = "Standard_E16_v3")]
        StandardE16V3,
        #[serde(rename = "Standard_E32_v3")]
        StandardE32V3,
        #[serde(rename = "Standard_E64_v3")]
        StandardE64V3,
        #[serde(rename = "Standard_E2s_v3")]
        StandardE2sV3,
        #[serde(rename = "Standard_E4s_v3")]
        StandardE4sV3,
        #[serde(rename = "Standard_E8s_v3")]
        StandardE8sV3,
        #[serde(rename = "Standard_E16s_v3")]
        StandardE16sV3,
        #[serde(rename = "Standard_E32s_v3")]
        StandardE32sV3,
        #[serde(rename = "Standard_E64s_v3")]
        StandardE64sV3,
        #[serde(rename = "Standard_E32-16_v3")]
        StandardE3216V3,
        #[serde(rename = "Standard_E32-8s_v3")]
        StandardE328sV3,
        #[serde(rename = "Standard_E64-32s_v3")]
        StandardE6432sV3,
        #[serde(rename = "Standard_E64-16s_v3")]
        StandardE6416sV3,
        #[serde(rename = "Standard_F1")]
        StandardF1,
        #[serde(rename = "Standard_F2")]
        StandardF2,
        #[serde(rename = "Standard_F4")]
        StandardF4,
        #[serde(rename = "Standard_F8")]
        StandardF8,
        #[serde(rename = "Standard_F16")]
        StandardF16,
        #[serde(rename = "Standard_F1s")]
        StandardF1s,
        #[serde(rename = "Standard_F2s")]
        StandardF2s,
        #[serde(rename = "Standard_F4s")]
        StandardF4s,
        #[serde(rename = "Standard_F8s")]
        StandardF8s,
        #[serde(rename = "Standard_F16s")]
        StandardF16s,
        #[serde(rename = "Standard_F2s_v2")]
        StandardF2sV2,
        #[serde(rename = "Standard_F4s_v2")]
        StandardF4sV2,
        #[serde(rename = "Standard_F8s_v2")]
        StandardF8sV2,
        #[serde(rename = "Standard_F16s_v2")]
        StandardF16sV2,
        #[serde(rename = "Standard_F32s_v2")]
        StandardF32sV2,
        #[serde(rename = "Standard_F64s_v2")]
        StandardF64sV2,
        #[serde(rename = "Standard_F72s_v2")]
        StandardF72sV2,
        #[serde(rename = "Standard_G1")]
        StandardG1,
        #[serde(rename = "Standard_G2")]
        StandardG2,
        #[serde(rename = "Standard_G3")]
        StandardG3,
        #[serde(rename = "Standard_G4")]
        StandardG4,
        #[serde(rename = "Standard_G5")]
        StandardG5,
        #[serde(rename = "Standard_GS1")]
        StandardGs1,
        #[serde(rename = "Standard_GS2")]
        StandardGs2,
        #[serde(rename = "Standard_GS3")]
        StandardGs3,
        #[serde(rename = "Standard_GS4")]
        StandardGs4,
        #[serde(rename = "Standard_GS5")]
        StandardGs5,
        #[serde(rename = "Standard_GS4-8")]
        StandardGs48,
        #[serde(rename = "Standard_GS4-4")]
        StandardGs44,
        #[serde(rename = "Standard_GS5-16")]
        StandardGs516,
        #[serde(rename = "Standard_GS5-8")]
        StandardGs58,
        #[serde(rename = "Standard_H8")]
        StandardH8,
        #[serde(rename = "Standard_H16")]
        StandardH16,
        #[serde(rename = "Standard_H8m")]
        StandardH8m,
        #[serde(rename = "Standard_H16m")]
        StandardH16m,
        #[serde(rename = "Standard_H16r")]
        StandardH16r,
        #[serde(rename = "Standard_H16mr")]
        StandardH16mr,
        #[serde(rename = "Standard_L4s")]
        StandardL4s,
        #[serde(rename = "Standard_L8s")]
        StandardL8s,
        #[serde(rename = "Standard_L16s")]
        StandardL16s,
        #[serde(rename = "Standard_L32s")]
        StandardL32s,
        #[serde(rename = "Standard_M64s")]
        StandardM64s,
        #[serde(rename = "Standard_M64ms")]
        StandardM64ms,
        #[serde(rename = "Standard_M128s")]
        StandardM128s,
        #[serde(rename = "Standard_M128ms")]
        StandardM128ms,
        #[serde(rename = "Standard_M64-32ms")]
        StandardM6432ms,
        #[serde(rename = "Standard_M64-16ms")]
        StandardM6416ms,
        #[serde(rename = "Standard_M128-64ms")]
        StandardM12864ms,
        #[serde(rename = "Standard_M128-32ms")]
        StandardM12832ms,
        #[serde(rename = "Standard_NC6")]
        StandardNc6,
        #[serde(rename = "Standard_NC12")]
        StandardNc12,
        #[serde(rename = "Standard_NC24")]
        StandardNc24,
        #[serde(rename = "Standard_NC24r")]
        StandardNc24r,
        #[serde(rename = "Standard_NC6s_v2")]
        StandardNc6sV2,
        #[serde(rename = "Standard_NC12s_v2")]
        StandardNc12sV2,
        #[serde(rename = "Standard_NC24s_v2")]
        StandardNc24sV2,
        #[serde(rename = "Standard_NC24rs_v2")]
        StandardNc24rsV2,
        #[serde(rename = "Standard_NC6s_v3")]
        StandardNc6sV3,
        #[serde(rename = "Standard_NC12s_v3")]
        StandardNc12sV3,
        #[serde(rename = "Standard_NC24s_v3")]
        StandardNc24sV3,
        #[serde(rename = "Standard_NC24rs_v3")]
        StandardNc24rsV3,
        #[serde(rename = "Standard_ND6s")]
        StandardNd6s,
        #[serde(rename = "Standard_ND12s")]
        StandardNd12s,
        #[serde(rename = "Standard_ND24s")]
        StandardNd24s,
        #[serde(rename = "Standard_ND24rs")]
        StandardNd24rs,
        #[serde(rename = "Standard_NV6")]
        StandardNv6,
        #[serde(rename = "Standard_NV12")]
        StandardNv12,
        #[serde(rename = "Standard_NV24")]
        StandardNv24,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VmSize {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VmSize {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VmSize {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BasicA0 => serializer.serialize_unit_variant("VmSize", 0u32, "Basic_A0"),
                Self::BasicA1 => serializer.serialize_unit_variant("VmSize", 1u32, "Basic_A1"),
                Self::BasicA2 => serializer.serialize_unit_variant("VmSize", 2u32, "Basic_A2"),
                Self::BasicA3 => serializer.serialize_unit_variant("VmSize", 3u32, "Basic_A3"),
                Self::BasicA4 => serializer.serialize_unit_variant("VmSize", 4u32, "Basic_A4"),
                Self::StandardA0 => serializer.serialize_unit_variant("VmSize", 5u32, "Standard_A0"),
                Self::StandardA1 => serializer.serialize_unit_variant("VmSize", 6u32, "Standard_A1"),
                Self::StandardA2 => serializer.serialize_unit_variant("VmSize", 7u32, "Standard_A2"),
                Self::StandardA3 => serializer.serialize_unit_variant("VmSize", 8u32, "Standard_A3"),
                Self::StandardA4 => serializer.serialize_unit_variant("VmSize", 9u32, "Standard_A4"),
                Self::StandardA5 => serializer.serialize_unit_variant("VmSize", 10u32, "Standard_A5"),
                Self::StandardA6 => serializer.serialize_unit_variant("VmSize", 11u32, "Standard_A6"),
                Self::StandardA7 => serializer.serialize_unit_variant("VmSize", 12u32, "Standard_A7"),
                Self::StandardA8 => serializer.serialize_unit_variant("VmSize", 13u32, "Standard_A8"),
                Self::StandardA9 => serializer.serialize_unit_variant("VmSize", 14u32, "Standard_A9"),
                Self::StandardA10 => serializer.serialize_unit_variant("VmSize", 15u32, "Standard_A10"),
                Self::StandardA11 => serializer.serialize_unit_variant("VmSize", 16u32, "Standard_A11"),
                Self::StandardA1V2 => serializer.serialize_unit_variant("VmSize", 17u32, "Standard_A1_v2"),
                Self::StandardA2V2 => serializer.serialize_unit_variant("VmSize", 18u32, "Standard_A2_v2"),
                Self::StandardA4V2 => serializer.serialize_unit_variant("VmSize", 19u32, "Standard_A4_v2"),
                Self::StandardA8V2 => serializer.serialize_unit_variant("VmSize", 20u32, "Standard_A8_v2"),
                Self::StandardA2mV2 => serializer.serialize_unit_variant("VmSize", 21u32, "Standard_A2m_v2"),
                Self::StandardA4mV2 => serializer.serialize_unit_variant("VmSize", 22u32, "Standard_A4m_v2"),
                Self::StandardA8mV2 => serializer.serialize_unit_variant("VmSize", 23u32, "Standard_A8m_v2"),
                Self::StandardB1s => serializer.serialize_unit_variant("VmSize", 24u32, "Standard_B1s"),
                Self::StandardB1ms => serializer.serialize_unit_variant("VmSize", 25u32, "Standard_B1ms"),
                Self::StandardB2s => serializer.serialize_unit_variant("VmSize", 26u32, "Standard_B2s"),
                Self::StandardB2ms => serializer.serialize_unit_variant("VmSize", 27u32, "Standard_B2ms"),
                Self::StandardB4ms => serializer.serialize_unit_variant("VmSize", 28u32, "Standard_B4ms"),
                Self::StandardB8ms => serializer.serialize_unit_variant("VmSize", 29u32, "Standard_B8ms"),
                Self::StandardD1 => serializer.serialize_unit_variant("VmSize", 30u32, "Standard_D1"),
                Self::StandardD2 => serializer.serialize_unit_variant("VmSize", 31u32, "Standard_D2"),
                Self::StandardD3 => serializer.serialize_unit_variant("VmSize", 32u32, "Standard_D3"),
                Self::StandardD4 => serializer.serialize_unit_variant("VmSize", 33u32, "Standard_D4"),
                Self::StandardD11 => serializer.serialize_unit_variant("VmSize", 34u32, "Standard_D11"),
                Self::StandardD12 => serializer.serialize_unit_variant("VmSize", 35u32, "Standard_D12"),
                Self::StandardD13 => serializer.serialize_unit_variant("VmSize", 36u32, "Standard_D13"),
                Self::StandardD14 => serializer.serialize_unit_variant("VmSize", 37u32, "Standard_D14"),
                Self::StandardD1V2 => serializer.serialize_unit_variant("VmSize", 38u32, "Standard_D1_v2"),
                Self::StandardD2V2 => serializer.serialize_unit_variant("VmSize", 39u32, "Standard_D2_v2"),
                Self::StandardD3V2 => serializer.serialize_unit_variant("VmSize", 40u32, "Standard_D3_v2"),
                Self::StandardD4V2 => serializer.serialize_unit_variant("VmSize", 41u32, "Standard_D4_v2"),
                Self::StandardD5V2 => serializer.serialize_unit_variant("VmSize", 42u32, "Standard_D5_v2"),
                Self::StandardD2V3 => serializer.serialize_unit_variant("VmSize", 43u32, "Standard_D2_v3"),
                Self::StandardD4V3 => serializer.serialize_unit_variant("VmSize", 44u32, "Standard_D4_v3"),
                Self::StandardD8V3 => serializer.serialize_unit_variant("VmSize", 45u32, "Standard_D8_v3"),
                Self::StandardD16V3 => serializer.serialize_unit_variant("VmSize", 46u32, "Standard_D16_v3"),
                Self::StandardD32V3 => serializer.serialize_unit_variant("VmSize", 47u32, "Standard_D32_v3"),
                Self::StandardD64V3 => serializer.serialize_unit_variant("VmSize", 48u32, "Standard_D64_v3"),
                Self::StandardD2sV3 => serializer.serialize_unit_variant("VmSize", 49u32, "Standard_D2s_v3"),
                Self::StandardD4sV3 => serializer.serialize_unit_variant("VmSize", 50u32, "Standard_D4s_v3"),
                Self::StandardD8sV3 => serializer.serialize_unit_variant("VmSize", 51u32, "Standard_D8s_v3"),
                Self::StandardD16sV3 => serializer.serialize_unit_variant("VmSize", 52u32, "Standard_D16s_v3"),
                Self::StandardD32sV3 => serializer.serialize_unit_variant("VmSize", 53u32, "Standard_D32s_v3"),
                Self::StandardD64sV3 => serializer.serialize_unit_variant("VmSize", 54u32, "Standard_D64s_v3"),
                Self::StandardD11V2 => serializer.serialize_unit_variant("VmSize", 55u32, "Standard_D11_v2"),
                Self::StandardD12V2 => serializer.serialize_unit_variant("VmSize", 56u32, "Standard_D12_v2"),
                Self::StandardD13V2 => serializer.serialize_unit_variant("VmSize", 57u32, "Standard_D13_v2"),
                Self::StandardD14V2 => serializer.serialize_unit_variant("VmSize", 58u32, "Standard_D14_v2"),
                Self::StandardD15V2 => serializer.serialize_unit_variant("VmSize", 59u32, "Standard_D15_v2"),
                Self::StandardDs1 => serializer.serialize_unit_variant("VmSize", 60u32, "Standard_DS1"),
                Self::StandardDs2 => serializer.serialize_unit_variant("VmSize", 61u32, "Standard_DS2"),
                Self::StandardDs3 => serializer.serialize_unit_variant("VmSize", 62u32, "Standard_DS3"),
                Self::StandardDs4 => serializer.serialize_unit_variant("VmSize", 63u32, "Standard_DS4"),
                Self::StandardDs11 => serializer.serialize_unit_variant("VmSize", 64u32, "Standard_DS11"),
                Self::StandardDs12 => serializer.serialize_unit_variant("VmSize", 65u32, "Standard_DS12"),
                Self::StandardDs13 => serializer.serialize_unit_variant("VmSize", 66u32, "Standard_DS13"),
                Self::StandardDs14 => serializer.serialize_unit_variant("VmSize", 67u32, "Standard_DS14"),
                Self::StandardDs1V2 => serializer.serialize_unit_variant("VmSize", 68u32, "Standard_DS1_v2"),
                Self::StandardDs2V2 => serializer.serialize_unit_variant("VmSize", 69u32, "Standard_DS2_v2"),
                Self::StandardDs3V2 => serializer.serialize_unit_variant("VmSize", 70u32, "Standard_DS3_v2"),
                Self::StandardDs4V2 => serializer.serialize_unit_variant("VmSize", 71u32, "Standard_DS4_v2"),
                Self::StandardDs5V2 => serializer.serialize_unit_variant("VmSize", 72u32, "Standard_DS5_v2"),
                Self::StandardDs11V2 => serializer.serialize_unit_variant("VmSize", 73u32, "Standard_DS11_v2"),
                Self::StandardDs12V2 => serializer.serialize_unit_variant("VmSize", 74u32, "Standard_DS12_v2"),
                Self::StandardDs13V2 => serializer.serialize_unit_variant("VmSize", 75u32, "Standard_DS13_v2"),
                Self::StandardDs14V2 => serializer.serialize_unit_variant("VmSize", 76u32, "Standard_DS14_v2"),
                Self::StandardDs15V2 => serializer.serialize_unit_variant("VmSize", 77u32, "Standard_DS15_v2"),
                Self::StandardDs134V2 => serializer.serialize_unit_variant("VmSize", 78u32, "Standard_DS13-4_v2"),
                Self::StandardDs132V2 => serializer.serialize_unit_variant("VmSize", 79u32, "Standard_DS13-2_v2"),
                Self::StandardDs148V2 => serializer.serialize_unit_variant("VmSize", 80u32, "Standard_DS14-8_v2"),
                Self::StandardDs144V2 => serializer.serialize_unit_variant("VmSize", 81u32, "Standard_DS14-4_v2"),
                Self::StandardE2V3 => serializer.serialize_unit_variant("VmSize", 82u32, "Standard_E2_v3"),
                Self::StandardE4V3 => serializer.serialize_unit_variant("VmSize", 83u32, "Standard_E4_v3"),
                Self::StandardE8V3 => serializer.serialize_unit_variant("VmSize", 84u32, "Standard_E8_v3"),
                Self::StandardE16V3 => serializer.serialize_unit_variant("VmSize", 85u32, "Standard_E16_v3"),
                Self::StandardE32V3 => serializer.serialize_unit_variant("VmSize", 86u32, "Standard_E32_v3"),
                Self::StandardE64V3 => serializer.serialize_unit_variant("VmSize", 87u32, "Standard_E64_v3"),
                Self::StandardE2sV3 => serializer.serialize_unit_variant("VmSize", 88u32, "Standard_E2s_v3"),
                Self::StandardE4sV3 => serializer.serialize_unit_variant("VmSize", 89u32, "Standard_E4s_v3"),
                Self::StandardE8sV3 => serializer.serialize_unit_variant("VmSize", 90u32, "Standard_E8s_v3"),
                Self::StandardE16sV3 => serializer.serialize_unit_variant("VmSize", 91u32, "Standard_E16s_v3"),
                Self::StandardE32sV3 => serializer.serialize_unit_variant("VmSize", 92u32, "Standard_E32s_v3"),
                Self::StandardE64sV3 => serializer.serialize_unit_variant("VmSize", 93u32, "Standard_E64s_v3"),
                Self::StandardE3216V3 => serializer.serialize_unit_variant("VmSize", 94u32, "Standard_E32-16_v3"),
                Self::StandardE328sV3 => serializer.serialize_unit_variant("VmSize", 95u32, "Standard_E32-8s_v3"),
                Self::StandardE6432sV3 => serializer.serialize_unit_variant("VmSize", 96u32, "Standard_E64-32s_v3"),
                Self::StandardE6416sV3 => serializer.serialize_unit_variant("VmSize", 97u32, "Standard_E64-16s_v3"),
                Self::StandardF1 => serializer.serialize_unit_variant("VmSize", 98u32, "Standard_F1"),
                Self::StandardF2 => serializer.serialize_unit_variant("VmSize", 99u32, "Standard_F2"),
                Self::StandardF4 => serializer.serialize_unit_variant("VmSize", 100u32, "Standard_F4"),
                Self::StandardF8 => serializer.serialize_unit_variant("VmSize", 101u32, "Standard_F8"),
                Self::StandardF16 => serializer.serialize_unit_variant("VmSize", 102u32, "Standard_F16"),
                Self::StandardF1s => serializer.serialize_unit_variant("VmSize", 103u32, "Standard_F1s"),
                Self::StandardF2s => serializer.serialize_unit_variant("VmSize", 104u32, "Standard_F2s"),
                Self::StandardF4s => serializer.serialize_unit_variant("VmSize", 105u32, "Standard_F4s"),
                Self::StandardF8s => serializer.serialize_unit_variant("VmSize", 106u32, "Standard_F8s"),
                Self::StandardF16s => serializer.serialize_unit_variant("VmSize", 107u32, "Standard_F16s"),
                Self::StandardF2sV2 => serializer.serialize_unit_variant("VmSize", 108u32, "Standard_F2s_v2"),
                Self::StandardF4sV2 => serializer.serialize_unit_variant("VmSize", 109u32, "Standard_F4s_v2"),
                Self::StandardF8sV2 => serializer.serialize_unit_variant("VmSize", 110u32, "Standard_F8s_v2"),
                Self::StandardF16sV2 => serializer.serialize_unit_variant("VmSize", 111u32, "Standard_F16s_v2"),
                Self::StandardF32sV2 => serializer.serialize_unit_variant("VmSize", 112u32, "Standard_F32s_v2"),
                Self::StandardF64sV2 => serializer.serialize_unit_variant("VmSize", 113u32, "Standard_F64s_v2"),
                Self::StandardF72sV2 => serializer.serialize_unit_variant("VmSize", 114u32, "Standard_F72s_v2"),
                Self::StandardG1 => serializer.serialize_unit_variant("VmSize", 115u32, "Standard_G1"),
                Self::StandardG2 => serializer.serialize_unit_variant("VmSize", 116u32, "Standard_G2"),
                Self::StandardG3 => serializer.serialize_unit_variant("VmSize", 117u32, "Standard_G3"),
                Self::StandardG4 => serializer.serialize_unit_variant("VmSize", 118u32, "Standard_G4"),
                Self::StandardG5 => serializer.serialize_unit_variant("VmSize", 119u32, "Standard_G5"),
                Self::StandardGs1 => serializer.serialize_unit_variant("VmSize", 120u32, "Standard_GS1"),
                Self::StandardGs2 => serializer.serialize_unit_variant("VmSize", 121u32, "Standard_GS2"),
                Self::StandardGs3 => serializer.serialize_unit_variant("VmSize", 122u32, "Standard_GS3"),
                Self::StandardGs4 => serializer.serialize_unit_variant("VmSize", 123u32, "Standard_GS4"),
                Self::StandardGs5 => serializer.serialize_unit_variant("VmSize", 124u32, "Standard_GS5"),
                Self::StandardGs48 => serializer.serialize_unit_variant("VmSize", 125u32, "Standard_GS4-8"),
                Self::StandardGs44 => serializer.serialize_unit_variant("VmSize", 126u32, "Standard_GS4-4"),
                Self::StandardGs516 => serializer.serialize_unit_variant("VmSize", 127u32, "Standard_GS5-16"),
                Self::StandardGs58 => serializer.serialize_unit_variant("VmSize", 128u32, "Standard_GS5-8"),
                Self::StandardH8 => serializer.serialize_unit_variant("VmSize", 129u32, "Standard_H8"),
                Self::StandardH16 => serializer.serialize_unit_variant("VmSize", 130u32, "Standard_H16"),
                Self::StandardH8m => serializer.serialize_unit_variant("VmSize", 131u32, "Standard_H8m"),
                Self::StandardH16m => serializer.serialize_unit_variant("VmSize", 132u32, "Standard_H16m"),
                Self::StandardH16r => serializer.serialize_unit_variant("VmSize", 133u32, "Standard_H16r"),
                Self::StandardH16mr => serializer.serialize_unit_variant("VmSize", 134u32, "Standard_H16mr"),
                Self::StandardL4s => serializer.serialize_unit_variant("VmSize", 135u32, "Standard_L4s"),
                Self::StandardL8s => serializer.serialize_unit_variant("VmSize", 136u32, "Standard_L8s"),
                Self::StandardL16s => serializer.serialize_unit_variant("VmSize", 137u32, "Standard_L16s"),
                Self::StandardL32s => serializer.serialize_unit_variant("VmSize", 138u32, "Standard_L32s"),
                Self::StandardM64s => serializer.serialize_unit_variant("VmSize", 139u32, "Standard_M64s"),
                Self::StandardM64ms => serializer.serialize_unit_variant("VmSize", 140u32, "Standard_M64ms"),
                Self::StandardM128s => serializer.serialize_unit_variant("VmSize", 141u32, "Standard_M128s"),
                Self::StandardM128ms => serializer.serialize_unit_variant("VmSize", 142u32, "Standard_M128ms"),
                Self::StandardM6432ms => serializer.serialize_unit_variant("VmSize", 143u32, "Standard_M64-32ms"),
                Self::StandardM6416ms => serializer.serialize_unit_variant("VmSize", 144u32, "Standard_M64-16ms"),
                Self::StandardM12864ms => serializer.serialize_unit_variant("VmSize", 145u32, "Standard_M128-64ms"),
                Self::StandardM12832ms => serializer.serialize_unit_variant("VmSize", 146u32, "Standard_M128-32ms"),
                Self::StandardNc6 => serializer.serialize_unit_variant("VmSize", 147u32, "Standard_NC6"),
                Self::StandardNc12 => serializer.serialize_unit_variant("VmSize", 148u32, "Standard_NC12"),
                Self::StandardNc24 => serializer.serialize_unit_variant("VmSize", 149u32, "Standard_NC24"),
                Self::StandardNc24r => serializer.serialize_unit_variant("VmSize", 150u32, "Standard_NC24r"),
                Self::StandardNc6sV2 => serializer.serialize_unit_variant("VmSize", 151u32, "Standard_NC6s_v2"),
                Self::StandardNc12sV2 => serializer.serialize_unit_variant("VmSize", 152u32, "Standard_NC12s_v2"),
                Self::StandardNc24sV2 => serializer.serialize_unit_variant("VmSize", 153u32, "Standard_NC24s_v2"),
                Self::StandardNc24rsV2 => serializer.serialize_unit_variant("VmSize", 154u32, "Standard_NC24rs_v2"),
                Self::StandardNc6sV3 => serializer.serialize_unit_variant("VmSize", 155u32, "Standard_NC6s_v3"),
                Self::StandardNc12sV3 => serializer.serialize_unit_variant("VmSize", 156u32, "Standard_NC12s_v3"),
                Self::StandardNc24sV3 => serializer.serialize_unit_variant("VmSize", 157u32, "Standard_NC24s_v3"),
                Self::StandardNc24rsV3 => serializer.serialize_unit_variant("VmSize", 158u32, "Standard_NC24rs_v3"),
                Self::StandardNd6s => serializer.serialize_unit_variant("VmSize", 159u32, "Standard_ND6s"),
                Self::StandardNd12s => serializer.serialize_unit_variant("VmSize", 160u32, "Standard_ND12s"),
                Self::StandardNd24s => serializer.serialize_unit_variant("VmSize", 161u32, "Standard_ND24s"),
                Self::StandardNd24rs => serializer.serialize_unit_variant("VmSize", 162u32, "Standard_ND24rs"),
                Self::StandardNv6 => serializer.serialize_unit_variant("VmSize", 163u32, "Standard_NV6"),
                Self::StandardNv12 => serializer.serialize_unit_variant("VmSize", 164u32, "Standard_NV12"),
                Self::StandardNv24 => serializer.serialize_unit_variant("VmSize", 165u32, "Standard_NV24"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the HyperVGeneration Type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HyperVGenerationType")]
pub enum HyperVGenerationType {
    V1,
    V2,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HyperVGenerationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HyperVGenerationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HyperVGenerationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::V1 => serializer.serialize_unit_variant("HyperVGenerationType", 0u32, "V1"),
            Self::V2 => serializer.serialize_unit_variant("HyperVGenerationType", 1u32, "V2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The source user image virtual hard disk. The virtual hard disk will be copied before being attached to the virtual machine. If SourceImage is provided, the destination virtual hard drive must not exist."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of an Image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImageProperties>,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
}
impl Image {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
            extended_location: None,
        }
    }
}
#[doc = "Describes a data disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageDataDisk {
    #[serde(flatten)]
    pub image_disk: ImageDisk,
    #[doc = "Specifies the logical unit number of the data disk. This value is used to identify data disks within the VM and therefore must be unique for each data disk attached to a VM."]
    pub lun: i32,
}
impl ImageDataDisk {
    pub fn new(lun: i32) -> Self {
        Self {
            image_disk: ImageDisk::default(),
            lun,
        }
    }
}
#[doc = "Describes a image disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageDisk {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<SubResource>,
    #[serde(rename = "managedDisk", default, skip_serializing_if = "Option::is_none")]
    pub managed_disk: Option<SubResource>,
    #[doc = "The Virtual Hard Disk."]
    #[serde(rename = "blobUri", default, skip_serializing_if = "Option::is_none")]
    pub blob_uri: Option<String>,
    #[doc = "Specifies the caching requirements. <br><br> Possible values are: <br><br> **None** <br><br> **ReadOnly** <br><br> **ReadWrite** <br><br> Default: **None for Standard storage. ReadOnly for Premium storage**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<image_disk::Caching>,
    #[doc = "Specifies the size of empty data disks in gigabytes. This element can be used to overwrite the name of the disk in a virtual machine image. <br><br> This value cannot be larger than 1023 GB"]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "Specifies the storage account type for the managed disk. Managed OS disk storage account type can only be set when you create the scale set. NOTE: UltraSSD_LRS can only be used with data disks. It cannot be used with OS Disk. Standard_LRS uses Standard HDD. StandardSSD_LRS uses Standard SSD. Premium_LRS uses Premium SSD. UltraSSD_LRS uses Ultra disk. Premium_ZRS uses Premium SSD zone redundant storage. StandardSSD_ZRS uses Standard SSD zone redundant storage. For more information regarding disks supported for Windows Virtual Machines, refer to https://docs.microsoft.com/azure/virtual-machines/windows/disks-types and, for Linux Virtual Machines, refer to https://docs.microsoft.com/azure/virtual-machines/linux/disks-types"]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<StorageAccountType>,
    #[doc = "Describes the parameter of customer managed disk encryption set resource id that can be specified for disk. <br><br> NOTE: The disk encryption set resource id can only be specified for managed disk. Please refer https://aka.ms/mdssewithcmkoverview for more details."]
    #[serde(rename = "diskEncryptionSet", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set: Option<DiskEncryptionSetParameters>,
}
impl ImageDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod image_disk {
    use super::*;
    #[doc = "Specifies the caching requirements. <br><br> Possible values are: <br><br> **None** <br><br> **ReadOnly** <br><br> **ReadWrite** <br><br> Default: **None for Standard storage. ReadOnly for Premium storage**"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Caching {
        None,
        ReadOnly,
        ReadWrite,
    }
}
#[doc = "The source image used for creating the disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageDiskReference {
    #[doc = "A relative uri containing either a Platform Image Repository or user image reference."]
    pub id: String,
    #[doc = "If the disk is created from an image's data disk, this is an index that indicates which of the data disks in the image to use. For OS disks, this field is null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
}
impl ImageDiskReference {
    pub fn new(id: String) -> Self {
        Self { id, lun: None }
    }
}
#[doc = "The List Image operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageListResult {
    #[doc = "The list of Images."]
    pub value: Vec<Image>,
    #[doc = "The uri to fetch the next page of Images. Call ListNext() with this to fetch the next page of Images."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ImageListResult {
    pub fn new(value: Vec<Image>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes an Operating System disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageOsDisk {
    #[serde(flatten)]
    pub image_disk: ImageDisk,
    #[doc = "This property allows you to specify the type of the OS that is included in the disk if creating a VM from a custom image. <br><br> Possible values are: <br><br> **Windows** <br><br> **Linux**"]
    #[serde(rename = "osType")]
    pub os_type: image_os_disk::OsType,
    #[doc = "The OS State. For managed images, use Generalized."]
    #[serde(rename = "osState")]
    pub os_state: image_os_disk::OsState,
}
impl ImageOsDisk {
    pub fn new(os_type: image_os_disk::OsType, os_state: image_os_disk::OsState) -> Self {
        Self {
            image_disk: ImageDisk::default(),
            os_type,
            os_state,
        }
    }
}
pub mod image_os_disk {
    use super::*;
    #[doc = "This property allows you to specify the type of the OS that is included in the disk if creating a VM from a custom image. <br><br> Possible values are: <br><br> **Windows** <br><br> **Linux**"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[doc = "The OS State. For managed images, use Generalized."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsState {
        Generalized,
        Specialized,
    }
}
#[doc = "Describes the properties of an Image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageProperties {
    #[serde(rename = "sourceVirtualMachine", default, skip_serializing_if = "Option::is_none")]
    pub source_virtual_machine: Option<SubResource>,
    #[doc = "Describes a storage profile."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<ImageStorageProfile>,
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Specifies the HyperVGeneration Type"]
    #[serde(rename = "hyperVGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<HyperVGenerationType>,
}
impl ImageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the gallery image definition purchase plan. This is used by marketplace images."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImagePurchasePlan {
    #[doc = "The plan ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The publisher ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The product ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}
impl ImagePurchasePlan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the image to use. You can specify information about platform images, marketplace images, or virtual machine images. This element is required when you want to use a platform image, marketplace image, or virtual machine image, but is not used in other creation operations. NOTE: Image reference publisher and offer can only be set when you create the scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageReference {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The image publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the offer of the platform image or marketplace image used to create the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "The image SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "Specifies the version of the platform image or marketplace image used to create the virtual machine. The allowed formats are Major.Minor.Build or 'latest'. Major, Minor, and Build are decimal numbers. Specify 'latest' to use the latest version of an image available at deploy time. Even if you use 'latest', the VM image will not automatically update after deploy time even if a new version becomes available. Please do not use field 'version' for gallery image deployment, gallery image should always use 'id' field for deployment, to use 'latest' version of gallery image, just set '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.Compute/galleries/{galleryName}/images/{imageName}' in the 'id' field without version input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Specifies in decimal numbers, the version of platform image or marketplace image used to create the virtual machine. This readonly field differs from 'version', only if the value specified in 'version' field is 'latest'."]
    #[serde(rename = "exactVersion", default, skip_serializing_if = "Option::is_none")]
    pub exact_version: Option<String>,
    #[doc = "Specified the shared gallery image unique id for vm deployment. This can be fetched from shared gallery image GET call."]
    #[serde(rename = "sharedGalleryImageId", default, skip_serializing_if = "Option::is_none")]
    pub shared_gallery_image_id: Option<String>,
    #[doc = "Specified the community gallery image unique id for vm deployment. This can be fetched from community gallery image GET call."]
    #[serde(rename = "communityGalleryImageId", default, skip_serializing_if = "Option::is_none")]
    pub community_gallery_image_id: Option<String>,
}
impl ImageReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a storage profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageStorageProfile {
    #[doc = "Describes an Operating System disk."]
    #[serde(rename = "osDisk", default, skip_serializing_if = "Option::is_none")]
    pub os_disk: Option<ImageOsDisk>,
    #[doc = "Specifies the parameters that are used to add a data disk to a virtual machine. <br><br> For more information about disks, see [About disks and VHDs for Azure virtual machines](https://docs.microsoft.com/azure/virtual-machines/managed-disks-overview)."]
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disks: Vec<ImageDataDisk>,
    #[doc = "Specifies whether an image is zone resilient or not. Default is false. Zone resilient images can be created only in regions that provide Zone Redundant Storage (ZRS)."]
    #[serde(rename = "zoneResilient", default, skip_serializing_if = "Option::is_none")]
    pub zone_resilient: Option<bool>,
}
impl ImageStorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The source user image virtual hard disk. Only tags may be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "Describes the properties of an Image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImageProperties>,
}
impl ImageUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Inner error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[doc = "The exception type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exceptiontype: Option<String>,
    #[doc = "The internal error message or exception dump."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errordetail: Option<String>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceSku {
    #[doc = "The sku name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The tier of the cloud service role instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl InstanceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Instance view status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceViewStatus {
    #[doc = "The status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The level code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<instance_view_status::Level>,
    #[doc = "The short localizable label for the status."]
    #[serde(rename = "displayStatus", default, skip_serializing_if = "Option::is_none")]
    pub display_status: Option<String>,
    #[doc = "The detailed status message, including for alerts and error messages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The time of the status."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
}
impl InstanceViewStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod instance_view_status {
    use super::*;
    #[doc = "The level code."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Level {
        Info,
        Warning,
        Error,
    }
}
#[doc = "Instance view statuses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstanceViewStatusesSummary {
    #[serde(rename = "statusesSummary", default, skip_serializing_if = "Vec::is_empty")]
    pub statuses_summary: Vec<StatusCodeCount>,
}
impl InstanceViewStatusesSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Key Vault Key Url to be used for server side encryption of Managed Disks and Snapshots"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyForDiskEncryptionSet {
    #[doc = "The vault id is an Azure Resource Manager Resource id in the form /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}"]
    #[serde(rename = "sourceVault", default, skip_serializing_if = "Option::is_none")]
    pub source_vault: Option<SourceVault>,
    #[doc = "Fully versioned Key Url pointing to a key in KeyVault. Version segment of the Url is required regardless of rotationToLatestKeyVersionEnabled value."]
    #[serde(rename = "keyUrl")]
    pub key_url: String,
}
impl KeyForDiskEncryptionSet {
    pub fn new(key_url: String) -> Self {
        Self {
            source_vault: None,
            key_url,
        }
    }
}
#[doc = "Key Vault Key Url and vault id of KeK, KeK is optional and when provided is used to unwrap the encryptionKey"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultAndKeyReference {
    #[doc = "The vault id is an Azure Resource Manager Resource id in the form /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}"]
    #[serde(rename = "sourceVault")]
    pub source_vault: SourceVault,
    #[doc = "Url pointing to a key or secret in KeyVault"]
    #[serde(rename = "keyUrl")]
    pub key_url: String,
}
impl KeyVaultAndKeyReference {
    pub fn new(source_vault: SourceVault, key_url: String) -> Self {
        Self { source_vault, key_url }
    }
}
#[doc = "Key Vault Secret Url and vault id of the encryption key "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultAndSecretReference {
    #[doc = "The vault id is an Azure Resource Manager Resource id in the form /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}"]
    #[serde(rename = "sourceVault")]
    pub source_vault: SourceVault,
    #[doc = "Url pointing to a key or secret in KeyVault"]
    #[serde(rename = "secretUrl")]
    pub secret_url: String,
}
impl KeyVaultAndSecretReference {
    pub fn new(source_vault: SourceVault, secret_url: String) -> Self {
        Self { source_vault, secret_url }
    }
}
#[doc = "Describes a reference to Key Vault Key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultKeyReference {
    #[doc = "The URL referencing a key encryption key in Key Vault."]
    #[serde(rename = "keyUrl")]
    pub key_url: String,
    #[serde(rename = "sourceVault")]
    pub source_vault: SubResource,
}
impl KeyVaultKeyReference {
    pub fn new(key_url: String, source_vault: SubResource) -> Self {
        Self { key_url, source_vault }
    }
}
#[doc = "Describes a reference to Key Vault Secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultSecretReference {
    #[doc = "The URL referencing a secret in a Key Vault."]
    #[serde(rename = "secretUrl")]
    pub secret_url: String,
    #[serde(rename = "sourceVault")]
    pub source_vault: SubResource,
}
impl KeyVaultSecretReference {
    pub fn new(secret_url: String, source_vault: SubResource) -> Self {
        Self { secret_url, source_vault }
    }
}
#[doc = "Describes the properties of the last installed patch summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LastPatchInstallationSummary {
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Unknown\", \"Failed\", \"Succeeded\", or \"CompletedWithWarnings.\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<last_patch_installation_summary::Status>,
    #[doc = "The activity ID of the operation that produced this result. It is used to correlate across CRP and extension logs."]
    #[serde(rename = "installationActivityId", default, skip_serializing_if = "Option::is_none")]
    pub installation_activity_id: Option<String>,
    #[doc = "Describes whether the operation ran out of time before it completed all its intended actions"]
    #[serde(rename = "maintenanceWindowExceeded", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window_exceeded: Option<bool>,
    #[doc = "The number of all available patches but not going to be installed because it didn't match a classification or inclusion list entry."]
    #[serde(rename = "notSelectedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub not_selected_patch_count: Option<i32>,
    #[doc = "The number of all available patches but excluded explicitly by a customer-specified exclusion list match."]
    #[serde(rename = "excludedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub excluded_patch_count: Option<i32>,
    #[doc = "The number of all available patches expected to be installed over the course of the patch installation operation."]
    #[serde(rename = "pendingPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub pending_patch_count: Option<i32>,
    #[doc = "The count of patches that successfully installed."]
    #[serde(rename = "installedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub installed_patch_count: Option<i32>,
    #[doc = "The count of patches that failed installation."]
    #[serde(rename = "failedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_patch_count: Option<i32>,
    #[doc = "The UTC timestamp when the operation began."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The UTC timestamp when the operation began."]
    #[serde(rename = "lastModifiedTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "Api error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}
impl LastPatchInstallationSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod last_patch_installation_summary {
    use super::*;
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Unknown\", \"Failed\", \"Succeeded\", or \"CompletedWithWarnings.\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Unknown,
        InProgress,
        Failed,
        Succeeded,
        CompletedWithWarnings,
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
                Self::Unknown => serializer.serialize_unit_variant("Status", 0u32, "Unknown"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 3u32, "Succeeded"),
                Self::CompletedWithWarnings => serializer.serialize_unit_variant("Status", 4u32, "CompletedWithWarnings"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the Linux operating system settings on the virtual machine. <br><br>For a list of supported Linux distributions, see [Linux on Azure-Endorsed Distributions](https://docs.microsoft.com/azure/virtual-machines/linux/endorsed-distros)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxConfiguration {
    #[doc = "Specifies whether password authentication should be disabled."]
    #[serde(rename = "disablePasswordAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub disable_password_authentication: Option<bool>,
    #[doc = "SSH configuration for Linux based VMs running on Azure"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh: Option<SshConfiguration>,
    #[doc = "Indicates whether virtual machine agent should be provisioned on the virtual machine. <br><br> When this property is not specified in the request body, default behavior is to set it to true.  This will ensure that VM Agent is installed on the VM so that extensions can be added to the VM later."]
    #[serde(rename = "provisionVMAgent", default, skip_serializing_if = "Option::is_none")]
    pub provision_vm_agent: Option<bool>,
    #[doc = "Specifies settings related to VM Guest Patching on Linux."]
    #[serde(rename = "patchSettings", default, skip_serializing_if = "Option::is_none")]
    pub patch_settings: Option<LinuxPatchSettings>,
}
impl LinuxConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input for InstallPatches on a Linux VM, as directly received by the API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxParameters {
    #[doc = "The update classifications to select when installing patches for Linux."]
    #[serde(rename = "classificationsToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub classifications_to_include: Vec<String>,
    #[doc = "packages to include in the patch operation. Format: packageName_packageVersion"]
    #[serde(rename = "packageNameMasksToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub package_name_masks_to_include: Vec<String>,
    #[doc = "packages to exclude in the patch operation. Format: packageName_packageVersion"]
    #[serde(rename = "packageNameMasksToExclude", default, skip_serializing_if = "Vec::is_empty")]
    pub package_name_masks_to_exclude: Vec<String>,
    #[doc = "This is used as a maintenance run identifier for Auto VM Guest Patching in Linux."]
    #[serde(rename = "maintenanceRunId", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_run_id: Option<String>,
}
impl LinuxParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies settings related to VM Guest Patching on Linux."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxPatchSettings {
    #[doc = "Specifies the mode of VM Guest Patching to IaaS virtual machine or virtual machines associated to virtual machine scale set with OrchestrationMode as Flexible.<br /><br /> Possible values are:<br /><br /> **ImageDefault** - The virtual machine's default patching configuration is used. <br /><br /> **AutomaticByPlatform** - The virtual machine will be automatically updated by the platform. The property provisionVMAgent must be true"]
    #[serde(rename = "patchMode", default, skip_serializing_if = "Option::is_none")]
    pub patch_mode: Option<linux_patch_settings::PatchMode>,
    #[doc = "Specifies the mode of VM Guest Patch Assessment for the IaaS virtual machine.<br /><br /> Possible values are:<br /><br /> **ImageDefault** - You control the timing of patch assessments on a virtual machine. <br /><br /> **AutomaticByPlatform** - The platform will trigger periodic patch assessments. The property provisionVMAgent must be true."]
    #[serde(rename = "assessmentMode", default, skip_serializing_if = "Option::is_none")]
    pub assessment_mode: Option<linux_patch_settings::AssessmentMode>,
    #[doc = "Specifies additional settings to be applied when patch mode AutomaticByPlatform is selected in Linux patch settings."]
    #[serde(rename = "automaticByPlatformSettings", default, skip_serializing_if = "Option::is_none")]
    pub automatic_by_platform_settings: Option<LinuxVmGuestPatchAutomaticByPlatformSettings>,
}
impl LinuxPatchSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod linux_patch_settings {
    use super::*;
    #[doc = "Specifies the mode of VM Guest Patching to IaaS virtual machine or virtual machines associated to virtual machine scale set with OrchestrationMode as Flexible.<br /><br /> Possible values are:<br /><br /> **ImageDefault** - The virtual machine's default patching configuration is used. <br /><br /> **AutomaticByPlatform** - The virtual machine will be automatically updated by the platform. The property provisionVMAgent must be true"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PatchMode")]
    pub enum PatchMode {
        ImageDefault,
        AutomaticByPlatform,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PatchMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PatchMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PatchMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ImageDefault => serializer.serialize_unit_variant("PatchMode", 0u32, "ImageDefault"),
                Self::AutomaticByPlatform => serializer.serialize_unit_variant("PatchMode", 1u32, "AutomaticByPlatform"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the mode of VM Guest Patch Assessment for the IaaS virtual machine.<br /><br /> Possible values are:<br /><br /> **ImageDefault** - You control the timing of patch assessments on a virtual machine. <br /><br /> **AutomaticByPlatform** - The platform will trigger periodic patch assessments. The property provisionVMAgent must be true."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AssessmentMode")]
    pub enum AssessmentMode {
        ImageDefault,
        AutomaticByPlatform,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AssessmentMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AssessmentMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AssessmentMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ImageDefault => serializer.serialize_unit_variant("AssessmentMode", 0u32, "ImageDefault"),
                Self::AutomaticByPlatform => serializer.serialize_unit_variant("AssessmentMode", 1u32, "AutomaticByPlatform"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies additional settings to be applied when patch mode AutomaticByPlatform is selected in Linux patch settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxVmGuestPatchAutomaticByPlatformSettings {
    #[doc = "Specifies the reboot setting for all AutomaticByPlatform patch installation operations."]
    #[serde(rename = "rebootSetting", default, skip_serializing_if = "Option::is_none")]
    pub reboot_setting: Option<linux_vm_guest_patch_automatic_by_platform_settings::RebootSetting>,
}
impl LinuxVmGuestPatchAutomaticByPlatformSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod linux_vm_guest_patch_automatic_by_platform_settings {
    use super::*;
    #[doc = "Specifies the reboot setting for all AutomaticByPlatform patch installation operations."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RebootSetting")]
    pub enum RebootSetting {
        Unknown,
        IfRequired,
        Never,
        Always,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RebootSetting {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RebootSetting {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RebootSetting {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("RebootSetting", 0u32, "Unknown"),
                Self::IfRequired => serializer.serialize_unit_variant("RebootSetting", 1u32, "IfRequired"),
                Self::Never => serializer.serialize_unit_variant("RebootSetting", 2u32, "Never"),
                Self::Always => serializer.serialize_unit_variant("RebootSetting", 3u32, "Always"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The List Usages operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListUsagesResult {
    #[doc = "The list of compute resource usages."]
    pub value: Vec<Usage>,
    #[doc = "The URI to fetch the next page of compute resource usage information. Call ListNext() with this to fetch the next page of compute resource usage information."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListUsagesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListUsagesResult {
    pub fn new(value: Vec<Usage>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the load balancer configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerConfiguration {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the Load balancer"]
    pub name: String,
    pub properties: LoadBalancerConfigurationProperties,
}
impl LoadBalancerConfiguration {
    pub fn new(name: String, properties: LoadBalancerConfigurationProperties) -> Self {
        Self {
            id: None,
            name,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerConfigurationProperties {
    #[doc = "Specifies the frontend IP to be used for the load balancer. Only IPv4 frontend IP address is supported. Each load balancer configuration must have exactly one frontend IP configuration."]
    #[serde(rename = "frontendIPConfigurations")]
    pub frontend_ip_configurations: Vec<LoadBalancerFrontendIpConfiguration>,
}
impl LoadBalancerConfigurationProperties {
    pub fn new(frontend_ip_configurations: Vec<LoadBalancerFrontendIpConfiguration>) -> Self {
        Self {
            frontend_ip_configurations,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancerFrontendIpConfiguration {
    #[doc = "The name of the resource that is unique within the set of frontend IP configurations used by the load balancer. This name can be used to access the resource."]
    pub name: String,
    #[doc = "Describes a cloud service IP Configuration"]
    pub properties: LoadBalancerFrontendIpConfigurationProperties,
}
impl LoadBalancerFrontendIpConfiguration {
    pub fn new(name: String, properties: LoadBalancerFrontendIpConfigurationProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "Describes a cloud service IP Configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancerFrontendIpConfigurationProperties {
    #[serde(rename = "publicIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<SubResource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<SubResource>,
    #[doc = "The virtual network private IP address of the IP configuration."]
    #[serde(rename = "privateIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
}
impl LoadBalancerFrontendIpConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Api input base class for LogAnalytics Api."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogAnalyticsInputBase {
    #[doc = "SAS Uri of the logging blob container to which LogAnalytics Api writes output logs to."]
    #[serde(rename = "blobContainerSasUri")]
    pub blob_container_sas_uri: String,
    #[doc = "From time of the query"]
    #[serde(rename = "fromTime", with = "azure_core::date::rfc3339")]
    pub from_time: time::OffsetDateTime,
    #[doc = "To time of the query"]
    #[serde(rename = "toTime", with = "azure_core::date::rfc3339")]
    pub to_time: time::OffsetDateTime,
    #[doc = "Group query result by Throttle Policy applied."]
    #[serde(rename = "groupByThrottlePolicy", default, skip_serializing_if = "Option::is_none")]
    pub group_by_throttle_policy: Option<bool>,
    #[doc = "Group query result by Operation Name."]
    #[serde(rename = "groupByOperationName", default, skip_serializing_if = "Option::is_none")]
    pub group_by_operation_name: Option<bool>,
    #[doc = "Group query result by Resource Name."]
    #[serde(rename = "groupByResourceName", default, skip_serializing_if = "Option::is_none")]
    pub group_by_resource_name: Option<bool>,
    #[doc = "Group query result by Client Application ID."]
    #[serde(rename = "groupByClientApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub group_by_client_application_id: Option<bool>,
    #[doc = "Group query result by User Agent."]
    #[serde(rename = "groupByUserAgent", default, skip_serializing_if = "Option::is_none")]
    pub group_by_user_agent: Option<bool>,
}
impl LogAnalyticsInputBase {
    pub fn new(blob_container_sas_uri: String, from_time: time::OffsetDateTime, to_time: time::OffsetDateTime) -> Self {
        Self {
            blob_container_sas_uri,
            from_time,
            to_time,
            group_by_throttle_policy: None,
            group_by_operation_name: None,
            group_by_resource_name: None,
            group_by_client_application_id: None,
            group_by_user_agent: None,
        }
    }
}
#[doc = "LogAnalytics operation status response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogAnalyticsOperationResult {
    #[doc = "LogAnalytics output properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LogAnalyticsOutput>,
}
impl LogAnalyticsOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "LogAnalytics output properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogAnalyticsOutput {
    #[doc = "Output file Uri path to blob container."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}
impl LogAnalyticsOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Maintenance Operation Status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceRedeployStatus {
    #[doc = "True, if customer is allowed to perform Maintenance."]
    #[serde(rename = "isCustomerInitiatedMaintenanceAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_customer_initiated_maintenance_allowed: Option<bool>,
    #[doc = "Start Time for the Pre Maintenance Window."]
    #[serde(rename = "preMaintenanceWindowStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub pre_maintenance_window_start_time: Option<time::OffsetDateTime>,
    #[doc = "End Time for the Pre Maintenance Window."]
    #[serde(rename = "preMaintenanceWindowEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub pre_maintenance_window_end_time: Option<time::OffsetDateTime>,
    #[doc = "Start Time for the Maintenance Window."]
    #[serde(rename = "maintenanceWindowStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub maintenance_window_start_time: Option<time::OffsetDateTime>,
    #[doc = "End Time for the Maintenance Window."]
    #[serde(rename = "maintenanceWindowEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub maintenance_window_end_time: Option<time::OffsetDateTime>,
    #[doc = "The Last Maintenance Operation Result Code."]
    #[serde(rename = "lastOperationResultCode", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_result_code: Option<maintenance_redeploy_status::LastOperationResultCode>,
    #[doc = "Message returned for the last Maintenance Operation."]
    #[serde(rename = "lastOperationMessage", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_message: Option<String>,
}
impl MaintenanceRedeployStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod maintenance_redeploy_status {
    use super::*;
    #[doc = "The Last Maintenance Operation Result Code."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastOperationResultCode {
        None,
        RetryLater,
        MaintenanceAborted,
        MaintenanceCompleted,
    }
}
#[doc = "The managed artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedArtifact {
    #[doc = "The managed artifact id."]
    pub id: String,
}
impl ManagedArtifact {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "The parameters of a managed disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDiskParameters {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Specifies the storage account type for the managed disk. Managed OS disk storage account type can only be set when you create the scale set. NOTE: UltraSSD_LRS can only be used with data disks. It cannot be used with OS Disk. Standard_LRS uses Standard HDD. StandardSSD_LRS uses Standard SSD. Premium_LRS uses Premium SSD. UltraSSD_LRS uses Ultra disk. Premium_ZRS uses Premium SSD zone redundant storage. StandardSSD_ZRS uses Standard SSD zone redundant storage. For more information regarding disks supported for Windows Virtual Machines, refer to https://docs.microsoft.com/azure/virtual-machines/windows/disks-types and, for Linux Virtual Machines, refer to https://docs.microsoft.com/azure/virtual-machines/linux/disks-types"]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<StorageAccountType>,
    #[doc = "Describes the parameter of customer managed disk encryption set resource id that can be specified for disk. <br><br> NOTE: The disk encryption set resource id can only be specified for managed disk. Please refer https://aka.ms/mdssewithcmkoverview for more details."]
    #[serde(rename = "diskEncryptionSet", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set: Option<DiskEncryptionSetParameters>,
    #[doc = "Specifies the security profile settings for the managed disk. <br><br> NOTE: It can only be set for Confidential VMs"]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<VmDiskSecurityProfile>,
}
impl ManagedDiskParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy for accessing the disk via network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NetworkAccessPolicy")]
pub enum NetworkAccessPolicy {
    AllowAll,
    AllowPrivate,
    DenyAll,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NetworkAccessPolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NetworkAccessPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NetworkAccessPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AllowAll => serializer.serialize_unit_variant("NetworkAccessPolicy", 0u32, "AllowAll"),
            Self::AllowPrivate => serializer.serialize_unit_variant("NetworkAccessPolicy", 1u32, "AllowPrivate"),
            Self::DenyAll => serializer.serialize_unit_variant("NetworkAccessPolicy", 2u32, "DenyAll"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes a network interface reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceReference {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Describes a network interface reference properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkInterfaceReferenceProperties>,
}
impl NetworkInterfaceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a network interface reference properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceReferenceProperties {
    #[doc = "Specifies the primary network interface in case the virtual machine has more than 1 network interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[doc = "Specify what happens to the network interface when the VM is deleted"]
    #[serde(rename = "deleteOption", default, skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<network_interface_reference_properties::DeleteOption>,
}
impl NetworkInterfaceReferenceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_interface_reference_properties {
    use super::*;
    #[doc = "Specify what happens to the network interface when the VM is deleted"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeleteOption")]
    pub enum DeleteOption {
        Delete,
        Detach,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeleteOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeleteOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeleteOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Delete => serializer.serialize_unit_variant("DeleteOption", 0u32, "Delete"),
                Self::Detach => serializer.serialize_unit_variant("DeleteOption", 1u32, "Detach"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the network interfaces or the networking configuration of the virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfile {
    #[doc = "Specifies the list of resource Ids for the network interfaces associated with the virtual machine."]
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<NetworkInterfaceReference>,
    #[doc = "specifies the Microsoft.Network API version used when creating networking resources in the Network Interface Configurations"]
    #[serde(rename = "networkApiVersion", default, skip_serializing_if = "Option::is_none")]
    pub network_api_version: Option<network_profile::NetworkApiVersion>,
    #[doc = "Specifies the networking configurations that will be used to create the virtual machine networking resources."]
    #[serde(rename = "networkInterfaceConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interface_configurations: Vec<VirtualMachineNetworkInterfaceConfiguration>,
}
impl NetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_profile {
    use super::*;
    #[doc = "specifies the Microsoft.Network API version used when creating networking resources in the Network Interface Configurations"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NetworkApiVersion")]
    pub enum NetworkApiVersion {
        #[serde(rename = "2020-11-01")]
        N2020_11_01,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NetworkApiVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NetworkApiVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NetworkApiVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N2020_11_01 => serializer.serialize_unit_variant("NetworkApiVersion", 0u32, "2020-11-01"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies information about the operating system disk used by the virtual machine. <br><br> For more information about disks, see [About disks and VHDs for Azure virtual machines](https://docs.microsoft.com/azure/virtual-machines/managed-disks-overview)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsDisk {
    #[doc = "This property allows you to specify the type of the OS that is included in the disk if creating a VM from user-image or a specialized VHD. <br><br> Possible values are: <br><br> **Windows** <br><br> **Linux**"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<os_disk::OsType>,
    #[doc = "Describes a Encryption Settings for a Disk"]
    #[serde(rename = "encryptionSettings", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings: Option<DiskEncryptionSettings>,
    #[doc = "The disk name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes the uri of a disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhd: Option<VirtualHardDisk>,
    #[doc = "Describes the uri of a disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<VirtualHardDisk>,
    #[doc = "Specifies the caching requirements. <br><br> Possible values are: <br><br> **None** <br><br> **ReadOnly** <br><br> **ReadWrite** <br><br> Default: **None for Standard storage. ReadOnly for Premium storage**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<Caching>,
    #[doc = "Specifies whether writeAccelerator should be enabled or disabled on the disk."]
    #[serde(rename = "writeAcceleratorEnabled", default, skip_serializing_if = "Option::is_none")]
    pub write_accelerator_enabled: Option<bool>,
    #[doc = "Describes the parameters of ephemeral disk settings that can be specified for operating system disk. <br><br> NOTE: The ephemeral disk settings can only be specified for managed disk."]
    #[serde(rename = "diffDiskSettings", default, skip_serializing_if = "Option::is_none")]
    pub diff_disk_settings: Option<DiffDiskSettings>,
    #[doc = "Specifies how the virtual machine should be created.<br><br> Possible values are:<br><br> **Attach** \\u2013 This value is used when you are using a specialized disk to create the virtual machine.<br><br> **FromImage** \\u2013 This value is used when you are using an image to create the virtual machine. If you are using a platform image, you also use the imageReference element described above. If you are using a marketplace image, you  also use the plan element previously described."]
    #[serde(rename = "createOption")]
    pub create_option: CreateOption,
    #[doc = "Specifies the size of an empty data disk in gigabytes. This element can be used to overwrite the size of the disk in a virtual machine image. <br><br> This value cannot be larger than 1023 GB"]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "The parameters of a managed disk."]
    #[serde(rename = "managedDisk", default, skip_serializing_if = "Option::is_none")]
    pub managed_disk: Option<ManagedDiskParameters>,
    #[doc = "Specifies the behavior of the managed disk when the VM gets deleted i.e whether the managed disk is deleted or detached. Supported values:<br><br> **Delete** If this value is used, the managed disk is deleted when VM gets deleted.<br><br> **Detach** If this value is used, the managed disk is retained after VM gets deleted.<br><br> Minimum api-version: 2021-03-01"]
    #[serde(rename = "deleteOption", default, skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<DeleteOption>,
}
impl OsDisk {
    pub fn new(create_option: CreateOption) -> Self {
        Self {
            os_type: None,
            encryption_settings: None,
            name: None,
            vhd: None,
            image: None,
            caching: None,
            write_accelerator_enabled: None,
            diff_disk_settings: None,
            create_option,
            disk_size_gb: None,
            managed_disk: None,
            delete_option: None,
        }
    }
}
pub mod os_disk {
    use super::*;
    #[doc = "This property allows you to specify the type of the OS that is included in the disk if creating a VM from user-image or a specialized VHD. <br><br> Possible values are: <br><br> **Windows** <br><br> **Linux**"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[doc = "Contains the os disk image information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsDiskImage {
    #[doc = "The operating system of the osDiskImage."]
    #[serde(rename = "operatingSystem")]
    pub operating_system: os_disk_image::OperatingSystem,
}
impl OsDiskImage {
    pub fn new(operating_system: os_disk_image::OperatingSystem) -> Self {
        Self { operating_system }
    }
}
pub mod os_disk_image {
    use super::*;
    #[doc = "The operating system of the osDiskImage."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OperatingSystem {
        Windows,
        Linux,
    }
}
#[doc = "Contains encryption settings for an OS disk image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsDiskImageEncryption {
    #[serde(flatten)]
    pub disk_image_encryption: DiskImageEncryption,
    #[doc = "Contains security profile for an OS disk image."]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<OsDiskImageSecurityProfile>,
}
impl OsDiskImageEncryption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains security profile for an OS disk image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsDiskImageSecurityProfile {
    #[doc = "confidential VM encryption types"]
    #[serde(rename = "confidentialVMEncryptionType", default, skip_serializing_if = "Option::is_none")]
    pub confidential_vm_encryption_type: Option<os_disk_image_security_profile::ConfidentialVmEncryptionType>,
    #[doc = "secure VM disk encryption set id"]
    #[serde(rename = "secureVMDiskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub secure_vm_disk_encryption_set_id: Option<String>,
}
impl OsDiskImageSecurityProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod os_disk_image_security_profile {
    use super::*;
    #[doc = "confidential VM encryption types"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConfidentialVmEncryptionType")]
    pub enum ConfidentialVmEncryptionType {
        #[serde(rename = "EncryptedVMGuestStateOnlyWithPmk")]
        EncryptedVmGuestStateOnlyWithPmk,
        EncryptedWithPmk,
        EncryptedWithCmk,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConfidentialVmEncryptionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConfidentialVmEncryptionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConfidentialVmEncryptionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::EncryptedVmGuestStateOnlyWithPmk => {
                    serializer.serialize_unit_variant("ConfidentialVmEncryptionType", 0u32, "EncryptedVMGuestStateOnlyWithPmk")
                }
                Self::EncryptedWithPmk => serializer.serialize_unit_variant("ConfidentialVmEncryptionType", 1u32, "EncryptedWithPmk"),
                Self::EncryptedWithCmk => serializer.serialize_unit_variant("ConfidentialVmEncryptionType", 2u32, "EncryptedWithCmk"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a cloud service OS family."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsFamily {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "OS family properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OsFamilyProperties>,
}
impl OsFamily {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsFamilyListResult {
    pub value: Vec<OsFamily>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OsFamilyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OsFamilyListResult {
    pub fn new(value: Vec<OsFamily>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "OS family properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsFamilyProperties {
    #[doc = "The OS family name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The OS family label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "List of OS versions belonging to this family."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub versions: Vec<OsVersionPropertiesBase>,
}
impl OsFamilyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the operating system settings for the virtual machine. Some of the settings cannot be changed once VM is provisioned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[doc = "Specifies the host OS name of the virtual machine. <br><br> This name cannot be updated after the VM is created. <br><br> **Max-length (Windows):** 15 characters <br><br> **Max-length (Linux):** 64 characters. <br><br> For naming conventions and restrictions see [Azure infrastructure services implementation guidelines](https://docs.microsoft.com/azure/azure-resource-manager/management/resource-name-rules)."]
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[doc = "Specifies the name of the administrator account. <br><br> This property cannot be updated after the VM is created. <br><br> **Windows-only restriction:** Cannot end in \".\" <br><br> **Disallowed values:** \"administrator\", \"admin\", \"user\", \"user1\", \"test\", \"user2\", \"test1\", \"user3\", \"admin1\", \"1\", \"123\", \"a\", \"actuser\", \"adm\", \"admin2\", \"aspnet\", \"backup\", \"console\", \"david\", \"guest\", \"john\", \"owner\", \"root\", \"server\", \"sql\", \"support\", \"support_388945a0\", \"sys\", \"test2\", \"test3\", \"user4\", \"user5\". <br><br> **Minimum-length (Linux):** 1  character <br><br> **Max-length (Linux):** 64 characters <br><br> **Max-length (Windows):** 20 characters."]
    #[serde(rename = "adminUsername", default, skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,
    #[doc = "Specifies the password of the administrator account. <br><br> **Minimum-length (Windows):** 8 characters <br><br> **Minimum-length (Linux):** 6 characters <br><br> **Max-length (Windows):** 123 characters <br><br> **Max-length (Linux):** 72 characters <br><br> **Complexity requirements:** 3 out of 4 conditions below need to be fulfilled <br> Has lower characters <br>Has upper characters <br> Has a digit <br> Has a special character (Regex match [\\W_]) <br><br> **Disallowed values:** \"abc@123\", \"P@$$w0rd\", \"P@ssw0rd\", \"P@ssword123\", \"Pa$$word\", \"pass@word1\", \"Password!\", \"Password1\", \"Password22\", \"iloveyou!\" <br><br> For resetting the password, see [How to reset the Remote Desktop service or its login password in a Windows VM](https://docs.microsoft.com/troubleshoot/azure/virtual-machines/reset-rdp) <br><br> For resetting root password, see [Manage users, SSH, and check or repair disks on Azure Linux VMs using the VMAccess Extension](https://docs.microsoft.com/troubleshoot/azure/virtual-machines/troubleshoot-ssh-connection)"]
    #[serde(rename = "adminPassword", default, skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<String>,
    #[doc = "Specifies a base-64 encoded string of custom data. The base-64 encoded string is decoded to a binary array that is saved as a file on the Virtual Machine. The maximum length of the binary array is 65535 bytes. <br><br> **Note: Do not pass any secrets or passwords in customData property** <br><br> This property cannot be updated after the VM is created. <br><br> customData is passed to the VM to be saved as a file, for more information see [Custom Data on Azure VMs](https://azure.microsoft.com/blog/custom-data-and-cloud-init-on-windows-azure/) <br><br> For using cloud-init for your Linux VM, see [Using cloud-init to customize a Linux VM during creation](https://docs.microsoft.com/azure/virtual-machines/linux/using-cloud-init)"]
    #[serde(rename = "customData", default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<String>,
    #[doc = "Specifies Windows operating system settings on the virtual machine."]
    #[serde(rename = "windowsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<WindowsConfiguration>,
    #[doc = "Specifies the Linux operating system settings on the virtual machine. <br><br>For a list of supported Linux distributions, see [Linux on Azure-Endorsed Distributions](https://docs.microsoft.com/azure/virtual-machines/linux/endorsed-distros)."]
    #[serde(rename = "linuxConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub linux_configuration: Option<LinuxConfiguration>,
    #[doc = "Specifies set of certificates that should be installed onto the virtual machine. To install certificates on a virtual machine it is recommended to use the [Azure Key Vault virtual machine extension for Linux](https://docs.microsoft.com/azure/virtual-machines/extensions/key-vault-linux) or the [Azure Key Vault virtual machine extension for Windows](https://docs.microsoft.com/azure/virtual-machines/extensions/key-vault-windows)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<VaultSecretGroup>,
    #[doc = "Specifies whether extension operations should be allowed on the virtual machine. <br><br>This may only be set to False when no extensions are present on the virtual machine."]
    #[serde(rename = "allowExtensionOperations", default, skip_serializing_if = "Option::is_none")]
    pub allow_extension_operations: Option<bool>,
    #[doc = "Optional property which must either be set to True or omitted."]
    #[serde(rename = "requireGuestProvisionSignal", default, skip_serializing_if = "Option::is_none")]
    pub require_guest_provision_signal: Option<bool>,
}
impl OsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a cloud service OS version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsVersion {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "OS version properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OsVersionProperties>,
}
impl OsVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsVersionListResult {
    pub value: Vec<OsVersion>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OsVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OsVersionListResult {
    pub fn new(value: Vec<OsVersion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "OS version properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsVersionProperties {
    #[doc = "The family of this OS version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The family label of this OS version."]
    #[serde(rename = "familyLabel", default, skip_serializing_if = "Option::is_none")]
    pub family_label: Option<String>,
    #[doc = "The OS version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The OS version label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Specifies whether this is the default OS version for its family."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "Specifies whether this OS version is active."]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}
impl OsVersionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration view of an OS version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsVersionPropertiesBase {
    #[doc = "The OS version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The OS version label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Specifies whether this is the default OS version for its family."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "Specifies whether this OS version is active."]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}
impl OsVersionPropertiesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the orchestration mode for the virtual machine scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OrchestrationMode")]
pub enum OrchestrationMode {
    Uniform,
    Flexible,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OrchestrationMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OrchestrationMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OrchestrationMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Uniform => serializer.serialize_unit_variant("OrchestrationMode", 0u32, "Uniform"),
            Self::Flexible => serializer.serialize_unit_variant("OrchestrationMode", 1u32, "Flexible"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The input for OrchestrationServiceState"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrchestrationServiceStateInput {
    #[doc = "The name of the service."]
    #[serde(rename = "serviceName")]
    pub service_name: orchestration_service_state_input::ServiceName,
    #[doc = "The action to be performed."]
    pub action: orchestration_service_state_input::Action,
}
impl OrchestrationServiceStateInput {
    pub fn new(service_name: orchestration_service_state_input::ServiceName, action: orchestration_service_state_input::Action) -> Self {
        Self { service_name, action }
    }
}
pub mod orchestration_service_state_input {
    use super::*;
    #[doc = "The name of the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServiceName")]
    pub enum ServiceName {
        AutomaticRepairs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServiceName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServiceName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServiceName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AutomaticRepairs => serializer.serialize_unit_variant("ServiceName", 0u32, "AutomaticRepairs"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The action to be performed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        Resume,
        Suspend,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Resume => serializer.serialize_unit_variant("Action", 0u32, "Resume"),
                Self::Suspend => serializer.serialize_unit_variant("Action", 1u32, "Suspend"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Summary for an orchestration service of a virtual machine scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrchestrationServiceSummary {
    #[doc = "The name of the service."]
    #[serde(rename = "serviceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<orchestration_service_summary::ServiceName>,
    #[doc = "The current state of the service."]
    #[serde(rename = "serviceState", default, skip_serializing_if = "Option::is_none")]
    pub service_state: Option<orchestration_service_summary::ServiceState>,
}
impl OrchestrationServiceSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod orchestration_service_summary {
    use super::*;
    #[doc = "The name of the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServiceName")]
    pub enum ServiceName {
        AutomaticRepairs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServiceName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServiceName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServiceName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AutomaticRepairs => serializer.serialize_unit_variant("ServiceName", 0u32, "AutomaticRepairs"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current state of the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServiceState")]
    pub enum ServiceState {
        NotRunning,
        Running,
        Suspended,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServiceState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServiceState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServiceState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotRunning => serializer.serialize_unit_variant("ServiceState", 0u32, "NotRunning"),
                Self::Running => serializer.serialize_unit_variant("ServiceState", 1u32, "Running"),
                Self::Suspended => serializer.serialize_unit_variant("ServiceState", 2u32, "Suspended"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information about a specific patch that was encountered during an installation action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchInstallationDetail {
    #[doc = "A unique identifier for the patch."]
    #[serde(rename = "patchId", default, skip_serializing_if = "Option::is_none")]
    pub patch_id: Option<String>,
    #[doc = "The friendly name of the patch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The version string of the package. It may conform to Semantic Versioning. Only applies to Linux."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The KBID of the patch. Only applies to Windows patches."]
    #[serde(rename = "kbId", default, skip_serializing_if = "Option::is_none")]
    pub kb_id: Option<String>,
    #[doc = "The classification(s) of the patch as provided by the patch publisher."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifications: Vec<String>,
    #[doc = "The state of the patch after the installation operation completed."]
    #[serde(rename = "installationState", default, skip_serializing_if = "Option::is_none")]
    pub installation_state: Option<patch_installation_detail::InstallationState>,
}
impl PatchInstallationDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod patch_installation_detail {
    use super::*;
    #[doc = "The state of the patch after the installation operation completed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InstallationState")]
    pub enum InstallationState {
        Unknown,
        Installed,
        Failed,
        Excluded,
        NotSelected,
        Pending,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InstallationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InstallationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InstallationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("InstallationState", 0u32, "Unknown"),
                Self::Installed => serializer.serialize_unit_variant("InstallationState", 1u32, "Installed"),
                Self::Failed => serializer.serialize_unit_variant("InstallationState", 2u32, "Failed"),
                Self::Excluded => serializer.serialize_unit_variant("InstallationState", 3u32, "Excluded"),
                Self::NotSelected => serializer.serialize_unit_variant("InstallationState", 4u32, "NotSelected"),
                Self::Pending => serializer.serialize_unit_variant("InstallationState", 5u32, "Pending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies settings related to VM Guest Patching on Windows."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchSettings {
    #[doc = "Specifies the mode of VM Guest Patching to IaaS virtual machine or virtual machines associated to virtual machine scale set with OrchestrationMode as Flexible.<br /><br /> Possible values are:<br /><br /> **Manual** - You  control the application of patches to a virtual machine. You do this by applying patches manually inside the VM. In this mode, automatic updates are disabled; the property WindowsConfiguration.enableAutomaticUpdates must be false<br /><br /> **AutomaticByOS** - The virtual machine will automatically be updated by the OS. The property WindowsConfiguration.enableAutomaticUpdates must be true. <br /><br /> **AutomaticByPlatform** - the virtual machine will automatically updated by the platform. The properties provisionVMAgent and WindowsConfiguration.enableAutomaticUpdates must be true "]
    #[serde(rename = "patchMode", default, skip_serializing_if = "Option::is_none")]
    pub patch_mode: Option<patch_settings::PatchMode>,
    #[doc = "Enables customers to patch their Azure VMs without requiring a reboot. For enableHotpatching, the 'provisionVMAgent' must be set to true and 'patchMode' must be set to 'AutomaticByPlatform'."]
    #[serde(rename = "enableHotpatching", default, skip_serializing_if = "Option::is_none")]
    pub enable_hotpatching: Option<bool>,
    #[doc = "Specifies the mode of VM Guest patch assessment for the IaaS virtual machine.<br /><br /> Possible values are:<br /><br /> **ImageDefault** - You control the timing of patch assessments on a virtual machine.<br /><br /> **AutomaticByPlatform** - The platform will trigger periodic patch assessments. The property provisionVMAgent must be true. "]
    #[serde(rename = "assessmentMode", default, skip_serializing_if = "Option::is_none")]
    pub assessment_mode: Option<patch_settings::AssessmentMode>,
    #[doc = "Specifies additional settings to be applied when patch mode AutomaticByPlatform is selected in Windows patch settings."]
    #[serde(rename = "automaticByPlatformSettings", default, skip_serializing_if = "Option::is_none")]
    pub automatic_by_platform_settings: Option<WindowsVmGuestPatchAutomaticByPlatformSettings>,
}
impl PatchSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod patch_settings {
    use super::*;
    #[doc = "Specifies the mode of VM Guest Patching to IaaS virtual machine or virtual machines associated to virtual machine scale set with OrchestrationMode as Flexible.<br /><br /> Possible values are:<br /><br /> **Manual** - You  control the application of patches to a virtual machine. You do this by applying patches manually inside the VM. In this mode, automatic updates are disabled; the property WindowsConfiguration.enableAutomaticUpdates must be false<br /><br /> **AutomaticByOS** - The virtual machine will automatically be updated by the OS. The property WindowsConfiguration.enableAutomaticUpdates must be true. <br /><br /> **AutomaticByPlatform** - the virtual machine will automatically updated by the platform. The properties provisionVMAgent and WindowsConfiguration.enableAutomaticUpdates must be true "]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PatchMode")]
    pub enum PatchMode {
        Manual,
        #[serde(rename = "AutomaticByOS")]
        AutomaticByOs,
        AutomaticByPlatform,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PatchMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PatchMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PatchMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Manual => serializer.serialize_unit_variant("PatchMode", 0u32, "Manual"),
                Self::AutomaticByOs => serializer.serialize_unit_variant("PatchMode", 1u32, "AutomaticByOS"),
                Self::AutomaticByPlatform => serializer.serialize_unit_variant("PatchMode", 2u32, "AutomaticByPlatform"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the mode of VM Guest patch assessment for the IaaS virtual machine.<br /><br /> Possible values are:<br /><br /> **ImageDefault** - You control the timing of patch assessments on a virtual machine.<br /><br /> **AutomaticByPlatform** - The platform will trigger periodic patch assessments. The property provisionVMAgent must be true. "]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AssessmentMode")]
    pub enum AssessmentMode {
        ImageDefault,
        AutomaticByPlatform,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AssessmentMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AssessmentMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AssessmentMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ImageDefault => serializer.serialize_unit_variant("AssessmentMode", 0u32, "ImageDefault"),
                Self::AutomaticByPlatform => serializer.serialize_unit_variant("AssessmentMode", 1u32, "AutomaticByPlatform"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base information about the community gallery resource in pir."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PirCommunityGalleryResource {
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The identifier information of community gallery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<CommunityGalleryIdentifier>,
}
impl PirCommunityGalleryResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PirResource {
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl PirResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base information about the shared gallery resource in pir."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PirSharedGalleryResource {
    #[serde(flatten)]
    pub pir_resource: PirResource,
    #[doc = "The identifier information of shared gallery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<SharedGalleryIdentifier>,
}
impl PirSharedGalleryResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the marketplace image used to create the virtual machine. This element is only used for marketplace images. Before you can use a marketplace image from an API, you must enable the image for programmatic use.  In the Azure portal, find the marketplace image that you want to use and then click **Want to deploy programmatically, Get Started ->**. Enter any required information and then click **Save**."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Plan {
    #[doc = "The plan ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The publisher ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the product of the image from the marketplace. This is the same value as Offer under the imageReference element."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "The promotion code."]
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}
impl Plan {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Properties of the PrivateEndpointConnectProperties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
    #[doc = "private endpoint connection Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "private endpoint connection name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "private endpoint connection type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of private endpoint connections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "The uri to fetch the next page of snapshots. Call ListNext() with this to fetch the next page of snapshots."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
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
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
    #[doc = "private link resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "private link resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "private link resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The private link resource DNS zone name."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Properties of the disk for which update is pending."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PropertyUpdatesInProgress {
    #[doc = "The target performance tier of the disk if a tier change operation is in progress."]
    #[serde(rename = "targetTier", default, skip_serializing_if = "Option::is_none")]
    pub target_tier: Option<String>,
}
impl PropertyUpdatesInProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the proximity placement group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProximityPlacementGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of a Proximity Placement Group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProximityPlacementGroupProperties>,
    #[doc = "Specifies the Availability Zone where virtual machine, virtual machine scale set or availability set associated with the  proximity placement group can be created."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl ProximityPlacementGroup {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
            zones: Vec::new(),
        }
    }
}
#[doc = "The List Proximity Placement Group operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProximityPlacementGroupListResult {
    #[doc = "The list of proximity placement groups"]
    pub value: Vec<ProximityPlacementGroup>,
    #[doc = "The URI to fetch the next page of proximity placement groups."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProximityPlacementGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProximityPlacementGroupListResult {
    pub fn new(value: Vec<ProximityPlacementGroup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the properties of a Proximity Placement Group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProximityPlacementGroupProperties {
    #[doc = "Specifies the type of the proximity placement group. <br><br> Possible values are: <br><br> **Standard** : Co-locate resources within an Azure region or Availability Zone. <br><br> **Ultra** : For future use."]
    #[serde(rename = "proximityPlacementGroupType", default, skip_serializing_if = "Option::is_none")]
    pub proximity_placement_group_type: Option<proximity_placement_group_properties::ProximityPlacementGroupType>,
    #[doc = "A list of references to all virtual machines in the proximity placement group."]
    #[serde(rename = "virtualMachines", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_machines: Vec<SubResourceWithColocationStatus>,
    #[doc = "A list of references to all virtual machine scale sets in the proximity placement group."]
    #[serde(rename = "virtualMachineScaleSets", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_machine_scale_sets: Vec<SubResourceWithColocationStatus>,
    #[doc = "A list of references to all availability sets in the proximity placement group."]
    #[serde(rename = "availabilitySets", default, skip_serializing_if = "Vec::is_empty")]
    pub availability_sets: Vec<SubResourceWithColocationStatus>,
    #[doc = "Instance view status."]
    #[serde(rename = "colocationStatus", default, skip_serializing_if = "Option::is_none")]
    pub colocation_status: Option<InstanceViewStatus>,
    #[doc = "Specifies the user intent of the proximity placement group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<proximity_placement_group_properties::Intent>,
}
impl ProximityPlacementGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod proximity_placement_group_properties {
    use super::*;
    #[doc = "Specifies the type of the proximity placement group. <br><br> Possible values are: <br><br> **Standard** : Co-locate resources within an Azure region or Availability Zone. <br><br> **Ultra** : For future use."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProximityPlacementGroupType")]
    pub enum ProximityPlacementGroupType {
        Standard,
        Ultra,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProximityPlacementGroupType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProximityPlacementGroupType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProximityPlacementGroupType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("ProximityPlacementGroupType", 0u32, "Standard"),
                Self::Ultra => serializer.serialize_unit_variant("ProximityPlacementGroupType", 1u32, "Ultra"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the user intent of the proximity placement group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Intent {
        #[doc = "Specifies possible sizes of virtual machines that can be created in the proximity placement group."]
        #[serde(rename = "vmSizes", default, skip_serializing_if = "Vec::is_empty")]
        pub vm_sizes: Vec<String>,
    }
    impl Intent {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Specifies information about the proximity placement group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProximityPlacementGroupUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
}
impl ProximityPlacementGroupUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ProxyOnly Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyOnlyResource {
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
impl ProxyOnlyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for an Azure Resource Manager proxy resource. It will not have tags and a location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
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
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the public IP Sku. It can only be set with OrchestrationMode as Flexible."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicIpAddressSku {
    #[doc = "Specify public IP sku name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<public_ip_address_sku::Name>,
    #[doc = "Specify public IP sku tier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<public_ip_address_sku::Tier>,
}
impl PublicIpAddressSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod public_ip_address_sku {
    use super::*;
    #[doc = "Specify public IP sku name"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Basic,
        Standard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("Name", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Name", 1u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specify public IP sku tier"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Regional,
        Global,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Regional => serializer.serialize_unit_variant("Tier", 0u32, "Regional"),
                Self::Global => serializer.serialize_unit_variant("Tier", 1u32, "Global"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Policy for controlling export on the disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PublicNetworkAccess")]
pub enum PublicNetworkAccess {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PublicNetworkAccess {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PublicNetworkAccess {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PublicNetworkAccess {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Used for establishing the purchase context of any 3rd Party artifact through MarketPlace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurchasePlan {
    #[doc = "The publisher ID."]
    pub publisher: String,
    #[doc = "The plan ID."]
    pub name: String,
    #[doc = "Specifies the product of the image from the marketplace. This is the same value as Offer under the imageReference element."]
    pub product: String,
}
impl PurchasePlan {
    pub fn new(publisher: String, name: String, product: String) -> Self {
        Self { publisher, name, product }
    }
}
#[doc = "The properties describe the recommended machine configuration for this Image Definition. These properties are updatable."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedMachineConfiguration {
    #[doc = "Describes the resource range."]
    #[serde(rename = "vCPUs", default, skip_serializing_if = "Option::is_none")]
    pub v_cp_us: Option<ResourceRange>,
    #[doc = "Describes the resource range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<ResourceRange>,
}
impl RecommendedMachineConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response after calling a manual recovery walk"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryWalkResponse {
    #[doc = "Whether the recovery walk was performed"]
    #[serde(rename = "walkPerformed", default, skip_serializing_if = "Option::is_none")]
    pub walk_performed: Option<bool>,
    #[doc = "The next update domain that needs to be walked. Null means walk spanning all update domains has been completed"]
    #[serde(rename = "nextPlatformUpdateDomain", default, skip_serializing_if = "Option::is_none")]
    pub next_platform_update_domain: Option<i64>,
}
impl RecoveryWalkResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the regional replication status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegionalReplicationStatus {
    #[doc = "The region to which the gallery image version is being replicated to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "This is the regional replication state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<regional_replication_status::State>,
    #[doc = "The details of the replication status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[doc = "It indicates progress of the replication job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
}
impl RegionalReplicationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod regional_replication_status {
    use super::*;
    #[doc = "This is the regional replication state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Unknown,
        Replicating,
        Completed,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("State", 0u32, "Unknown"),
                Self::Replicating => serializer.serialize_unit_variant("State", 1u32, "Replicating"),
                Self::Completed => serializer.serialize_unit_variant("State", 2u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("State", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Gallery regional sharing status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegionalSharingStatus {
    #[doc = "Region name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "The sharing state of the gallery, which only appears in the response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<SharingState>,
    #[doc = "Details of gallery regional sharing failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl RegionalSharingStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the replication status of the gallery image version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationStatus {
    #[doc = "This is the aggregated replication status based on all the regional replication status flags."]
    #[serde(rename = "aggregatedState", default, skip_serializing_if = "Option::is_none")]
    pub aggregated_state: Option<replication_status::AggregatedState>,
    #[doc = "This is a summary of replication status for each region."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub summary: Vec<RegionalReplicationStatus>,
}
impl ReplicationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod replication_status {
    use super::*;
    #[doc = "This is the aggregated replication status based on all the regional replication status flags."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AggregatedState")]
    pub enum AggregatedState {
        Unknown,
        InProgress,
        Completed,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AggregatedState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AggregatedState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AggregatedState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("AggregatedState", 0u32, "Unknown"),
                Self::InProgress => serializer.serialize_unit_variant("AggregatedState", 1u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("AggregatedState", 2u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("AggregatedState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Api request input for LogAnalytics getRequestRateByInterval Api."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestRateByIntervalInput {
    #[serde(flatten)]
    pub log_analytics_input_base: LogAnalyticsInputBase,
    #[doc = "Interval value in minutes used to create LogAnalytics call rate logs."]
    #[serde(rename = "intervalLength")]
    pub interval_length: request_rate_by_interval_input::IntervalLength,
}
impl RequestRateByIntervalInput {
    pub fn new(log_analytics_input_base: LogAnalyticsInputBase, interval_length: request_rate_by_interval_input::IntervalLength) -> Self {
        Self {
            log_analytics_input_base,
            interval_length,
        }
    }
}
pub mod request_rate_by_interval_input {
    use super::*;
    #[doc = "Interval value in minutes used to create LogAnalytics call rate logs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IntervalLength {
        ThreeMins,
        FiveMins,
        ThirtyMins,
        SixtyMins,
    }
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
#[doc = "Instance view status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceInstanceViewStatus {
    #[doc = "The status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The short localizable label for the status."]
    #[serde(rename = "displayStatus", default, skip_serializing_if = "Option::is_none")]
    pub display_status: Option<String>,
    #[doc = "The detailed status message, including for alerts and error messages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The time of the status."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub time: Option<time::OffsetDateTime>,
    #[doc = "The level code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<resource_instance_view_status::Level>,
}
impl ResourceInstanceViewStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_instance_view_status {
    use super::*;
    #[doc = "The level code."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Level {
        Info,
        Warning,
        Error,
    }
}
#[doc = "Describes the resource range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceRange {
    #[doc = "The minimum number of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    #[doc = "The maximum number of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
}
impl ResourceRange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an available Compute SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSku {
    #[doc = "The type of resource the SKU applies to."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The name of SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the tier of virtual machines in a scale set.<br /><br /> Possible Values:<br /><br /> **Standard**<br /><br /> **Basic**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The Size of the SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The Family of this particular SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The Kind of resources that are supported in this SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Describes scaling information of a SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<ResourceSkuCapacity>,
    #[doc = "The set of locations that the SKU is available."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "A list of locations and availability zones in those locations where the SKU is available."]
    #[serde(rename = "locationInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub location_info: Vec<ResourceSkuLocationInfo>,
    #[doc = "The api versions that support this SKU."]
    #[serde(rename = "apiVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
    #[doc = "Metadata for retrieving price info."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub costs: Vec<ResourceSkuCosts>,
    #[doc = "A name value pair to describe the capability."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<ResourceSkuCapabilities>,
    #[doc = "The restrictions because of which SKU cannot be used. This is empty if there are no restrictions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<ResourceSkuRestrictions>,
}
impl ResourceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes The SKU capabilities object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuCapabilities {
    #[doc = "An invariant to describe the feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "An invariant if the feature is measured by quantity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ResourceSkuCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes scaling information of a SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuCapacity {
    #[doc = "The minimum capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[doc = "The maximum capacity that can be set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[doc = "The default capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,
    #[doc = "The scale type applicable to the sku."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<resource_sku_capacity::ScaleType>,
}
impl ResourceSkuCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_sku_capacity {
    use super::*;
    #[doc = "The scale type applicable to the sku."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScaleType {
        Automatic,
        Manual,
        None,
    }
}
#[doc = "Describes metadata for retrieving price info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuCosts {
    #[doc = "Used for querying price from commerce."]
    #[serde(rename = "meterID", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "The multiplier is needed to extend the base metered cost."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "An invariant to show the extended unit."]
    #[serde(rename = "extendedUnit", default, skip_serializing_if = "Option::is_none")]
    pub extended_unit: Option<String>,
}
impl ResourceSkuCosts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an available Compute SKU Location Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuLocationInfo {
    #[doc = "Location of the SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "List of availability zones where the SKU is supported."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "Details of capabilities available to a SKU in specific zones."]
    #[serde(rename = "zoneDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub zone_details: Vec<ResourceSkuZoneDetails>,
    #[doc = "The names of extended locations."]
    #[serde(rename = "extendedLocations", default, skip_serializing_if = "Vec::is_empty")]
    pub extended_locations: Vec<String>,
    #[doc = "The type of the extended location."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<resource_sku_location_info::Type>,
}
impl ResourceSkuLocationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_sku_location_info {
    use super::*;
    #[doc = "The type of the extended location."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        EdgeZone,
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
                Self::EdgeZone => serializer.serialize_unit_variant("Type", 0u32, "EdgeZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes an available Compute SKU Restriction Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuRestrictionInfo {
    #[doc = "Locations where the SKU is restricted"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "List of availability zones where the SKU is restricted."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl ResourceSkuRestrictionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes scaling information of a SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuRestrictions {
    #[doc = "The type of restrictions."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<resource_sku_restrictions::Type>,
    #[doc = "The value of restrictions. If the restriction type is set to location. This would be different locations where the SKU is restricted."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[doc = "Describes an available Compute SKU Restriction Information."]
    #[serde(rename = "restrictionInfo", default, skip_serializing_if = "Option::is_none")]
    pub restriction_info: Option<ResourceSkuRestrictionInfo>,
    #[doc = "The reason for restriction."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<resource_sku_restrictions::ReasonCode>,
}
impl ResourceSkuRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_sku_restrictions {
    use super::*;
    #[doc = "The type of restrictions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Location,
        Zone,
    }
    #[doc = "The reason for restriction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
    }
}
#[doc = "Describes The zonal capabilities of a SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuZoneDetails {
    #[doc = "The set of zones that the SKU is available in with the specified capabilities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    #[doc = "A list of capabilities that are available for the SKU in the specified list of zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<ResourceSkuCapabilities>,
}
impl ResourceSkuZoneDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Resource Skus operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkusResult {
    #[doc = "The list of skus available for the subscription."]
    pub value: Vec<ResourceSku>,
    #[doc = "The URI to fetch the next page of Resource Skus. Call ListNext() with this URI to fetch the next page of Resource Skus"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceSkusResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceSkusResult {
    pub fn new(value: Vec<ResourceSku>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The List resources which are encrypted with the disk encryption set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceUriList {
    #[doc = "A list of IDs or Owner IDs of resources which are encrypted with the disk encryption set."]
    pub value: Vec<String>,
    #[doc = "The uri to fetch the next page of encrypted resources. Call ListNext() with this to fetch the next page of encrypted resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceUriList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceUriList {
    pub fn new(value: Vec<String>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The Resource model definition with location property as optional."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceWithOptionalLocation {
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceWithOptionalLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Restore Point details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The restore point properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestorePointProperties>,
}
impl RestorePoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Create or update Restore Point collection parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestorePointCollection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The restore point collection properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestorePointCollectionProperties>,
}
impl RestorePointCollection {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "The List restore point collection operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePointCollectionListResult {
    #[doc = "Gets the list of restore point collections."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RestorePointCollection>,
    #[doc = "The uri to fetch the next page of RestorePointCollections. Call ListNext() with this to fetch the next page of RestorePointCollections"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RestorePointCollectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RestorePointCollectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The restore point collection properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePointCollectionProperties {
    #[doc = "The properties of the source resource that this restore point collection is created from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<RestorePointCollectionSourceProperties>,
    #[doc = "The provisioning state of the restore point collection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The unique id of the restore point collection."]
    #[serde(rename = "restorePointCollectionId", default, skip_serializing_if = "Option::is_none")]
    pub restore_point_collection_id: Option<String>,
    #[doc = "A list containing all restore points created under this restore point collection."]
    #[serde(rename = "restorePoints", default, skip_serializing_if = "Vec::is_empty")]
    pub restore_points: Vec<RestorePoint>,
}
impl RestorePointCollectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the source resource that this restore point collection is created from."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePointCollectionSourceProperties {
    #[doc = "Location of the source resource used to create this restore point collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource Id of the source resource used to create this restore point collection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl RestorePointCollectionSourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update Restore Point collection parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePointCollectionUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "The restore point collection properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestorePointCollectionProperties>,
}
impl RestorePointCollectionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The instance view of a restore point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePointInstanceView {
    #[doc = "The disk restore points information."]
    #[serde(rename = "diskRestorePoints", default, skip_serializing_if = "Vec::is_empty")]
    pub disk_restore_points: Vec<DiskRestorePointInstanceView>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<InstanceViewStatus>,
}
impl RestorePointInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The restore point properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePointProperties {
    #[doc = "List of disk resource ids that the customer wishes to exclude from the restore point. If no disks are specified, all disks will be included."]
    #[serde(rename = "excludeDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_disks: Vec<ApiEntityReference>,
    #[doc = "Describes the properties of the Virtual Machine for which the restore point was created. The properties provided are a subset and the snapshot of the overall Virtual Machine properties captured at the time of the restore point creation."]
    #[serde(rename = "sourceMetadata", default, skip_serializing_if = "Option::is_none")]
    pub source_metadata: Option<RestorePointSourceMetadata>,
    #[doc = "Gets the provisioning state of the restore point."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "ConsistencyMode of the RestorePoint. Can be specified in the input while creating a restore point. For now, only CrashConsistent is accepted as a valid input. Please refer to https://aka.ms/RestorePoints for more details."]
    #[serde(rename = "consistencyMode", default, skip_serializing_if = "Option::is_none")]
    pub consistency_mode: Option<restore_point_properties::ConsistencyMode>,
    #[doc = "Gets the creation time of the restore point."]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<time::OffsetDateTime>,
    #[doc = "The API entity reference."]
    #[serde(rename = "sourceRestorePoint", default, skip_serializing_if = "Option::is_none")]
    pub source_restore_point: Option<ApiEntityReference>,
    #[doc = "The instance view of a restore point."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<RestorePointInstanceView>,
}
impl RestorePointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod restore_point_properties {
    use super::*;
    #[doc = "ConsistencyMode of the RestorePoint. Can be specified in the input while creating a restore point. For now, only CrashConsistent is accepted as a valid input. Please refer to https://aka.ms/RestorePoints for more details."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConsistencyMode")]
    pub enum ConsistencyMode {
        CrashConsistent,
        FileSystemConsistent,
        ApplicationConsistent,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConsistencyMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConsistencyMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConsistencyMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CrashConsistent => serializer.serialize_unit_variant("ConsistencyMode", 0u32, "CrashConsistent"),
                Self::FileSystemConsistent => serializer.serialize_unit_variant("ConsistencyMode", 1u32, "FileSystemConsistent"),
                Self::ApplicationConsistent => serializer.serialize_unit_variant("ConsistencyMode", 2u32, "ApplicationConsistent"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the properties of the Virtual Machine for which the restore point was created. The properties provided are a subset and the snapshot of the overall Virtual Machine properties captured at the time of the restore point creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePointSourceMetadata {
    #[doc = "Specifies the hardware settings for the virtual machine."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[doc = "Describes the storage profile."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<RestorePointSourceVmStorageProfile>,
    #[doc = "Specifies the operating system settings for the virtual machine. Some of the settings cannot be changed once VM is provisioned."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "Specifies the boot diagnostic settings state. <br><br>Minimum api-version: 2015-06-15."]
    #[serde(rename = "diagnosticsProfile", default, skip_serializing_if = "Option::is_none")]
    pub diagnostics_profile: Option<DiagnosticsProfile>,
    #[doc = "Gets the license type, which is for bring your own license scenario."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[doc = "Gets the virtual machine unique id."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "Specifies the Security profile settings for the virtual machine or virtual machine scale set."]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<SecurityProfile>,
    #[doc = "Location of the VM from which the restore point was created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl RestorePointSourceMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a data disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePointSourceVmDataDisk {
    #[doc = "Gets the logical unit number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[doc = "Gets the disk name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the caching requirements. <br><br> Possible values are: <br><br> **None** <br><br> **ReadOnly** <br><br> **ReadWrite** <br><br> Default: **None for Standard storage. ReadOnly for Premium storage**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<Caching>,
    #[doc = "Gets the initial disk size in GB for blank data disks, and the new desired size for existing OS and Data disks."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "The parameters of a managed disk."]
    #[serde(rename = "managedDisk", default, skip_serializing_if = "Option::is_none")]
    pub managed_disk: Option<ManagedDiskParameters>,
    #[doc = "The API entity reference."]
    #[serde(rename = "diskRestorePoint", default, skip_serializing_if = "Option::is_none")]
    pub disk_restore_point: Option<ApiEntityReference>,
}
impl RestorePointSourceVmDataDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Operating System disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePointSourceVmosDisk {
    #[doc = "Gets the Operating System type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<restore_point_source_vmos_disk::OsType>,
    #[doc = "Describes a Encryption Settings for a Disk"]
    #[serde(rename = "encryptionSettings", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings: Option<DiskEncryptionSettings>,
    #[doc = "Gets the disk name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the caching requirements. <br><br> Possible values are: <br><br> **None** <br><br> **ReadOnly** <br><br> **ReadWrite** <br><br> Default: **None for Standard storage. ReadOnly for Premium storage**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<Caching>,
    #[doc = "Gets the disk size in GB."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "The parameters of a managed disk."]
    #[serde(rename = "managedDisk", default, skip_serializing_if = "Option::is_none")]
    pub managed_disk: Option<ManagedDiskParameters>,
    #[doc = "The API entity reference."]
    #[serde(rename = "diskRestorePoint", default, skip_serializing_if = "Option::is_none")]
    pub disk_restore_point: Option<ApiEntityReference>,
}
impl RestorePointSourceVmosDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod restore_point_source_vmos_disk {
    use super::*;
    #[doc = "Gets the Operating System type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OsType")]
    pub enum OsType {
        Windows,
        Linux,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the storage profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorePointSourceVmStorageProfile {
    #[doc = "Describes an Operating System disk."]
    #[serde(rename = "osDisk", default, skip_serializing_if = "Option::is_none")]
    pub os_disk: Option<RestorePointSourceVmosDisk>,
    #[doc = "Gets the data disks of the VM captured at the time of the restore point creation."]
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disks: Vec<RestorePointSourceVmDataDisk>,
}
impl RestorePointSourceVmStorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SAS URIs of the console screenshot and serial log blobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetrieveBootDiagnosticsDataResult {
    #[doc = "The console screenshot blob URI"]
    #[serde(rename = "consoleScreenshotBlobUri", default, skip_serializing_if = "Option::is_none")]
    pub console_screenshot_blob_uri: Option<String>,
    #[doc = "The serial console log blob URI."]
    #[serde(rename = "serialConsoleLogBlobUri", default, skip_serializing_if = "Option::is_none")]
    pub serial_console_log_blob_uri: Option<String>,
}
impl RetrieveBootDiagnosticsDataResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleInstance {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource Type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource Location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<InstanceSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleInstanceProperties>,
}
impl RoleInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The instance view of the role instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleInstanceInstanceView {
    #[doc = "The Update Domain."]
    #[serde(rename = "platformUpdateDomain", default, skip_serializing_if = "Option::is_none")]
    pub platform_update_domain: Option<i32>,
    #[doc = "The Fault Domain."]
    #[serde(rename = "platformFaultDomain", default, skip_serializing_if = "Option::is_none")]
    pub platform_fault_domain: Option<i32>,
    #[doc = "Specifies a unique identifier generated internally for the cloud service associated with this role instance. <br /><br /> NOTE: If you are using Azure Diagnostics extension, this property can be used as 'DeploymentId' for querying details."]
    #[serde(rename = "privateId", default, skip_serializing_if = "Option::is_none")]
    pub private_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<ResourceInstanceViewStatus>,
}
impl RoleInstanceInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleInstanceListResult {
    pub value: Vec<RoleInstance>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleInstanceListResult {
    pub fn new(value: Vec<RoleInstance>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the network profile for the role instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleInstanceNetworkProfile {
    #[doc = "Specifies the list of resource Ids for the network interfaces associated with the role instance."]
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<SubResource>,
}
impl RoleInstanceNetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleInstanceProperties {
    #[doc = "Describes the network profile for the role instance."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<RoleInstanceNetworkProfile>,
    #[doc = "The instance view of the role instance."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<RoleInstanceInstanceView>,
}
impl RoleInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies a list of role instances from the cloud service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleInstances {
    #[doc = "List of cloud service role instance names. Value of '*' will signify all role instances of the cloud service."]
    #[serde(rename = "roleInstances")]
    pub role_instances: Vec<String>,
}
impl RoleInstances {
    pub fn new(role_instances: Vec<String>) -> Self {
        Self { role_instances }
    }
}
#[doc = "Information about rollback on failed VM instances after a OS Upgrade operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RollbackStatusInfo {
    #[doc = "The number of instances which have been successfully rolled back."]
    #[serde(rename = "successfullyRolledbackInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub successfully_rolledback_instance_count: Option<i32>,
    #[doc = "The number of instances which failed to rollback."]
    #[serde(rename = "failedRolledbackInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_rolledback_instance_count: Option<i32>,
    #[doc = "Api error."]
    #[serde(rename = "rollbackError", default, skip_serializing_if = "Option::is_none")]
    pub rollback_error: Option<ApiError>,
}
impl RollbackStatusInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration parameters used while performing a rolling upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RollingUpgradePolicy {
    #[doc = "The maximum percent of total virtual machine instances that will be upgraded simultaneously by the rolling upgrade in one batch. As this is a maximum, unhealthy instances in previous or future batches can cause the percentage of instances in a batch to decrease to ensure higher reliability. The default value for this parameter is 20%."]
    #[serde(rename = "maxBatchInstancePercent", default, skip_serializing_if = "Option::is_none")]
    pub max_batch_instance_percent: Option<i32>,
    #[doc = "The maximum percentage of the total virtual machine instances in the scale set that can be simultaneously unhealthy, either as a result of being upgraded, or by being found in an unhealthy state by the virtual machine health checks before the rolling upgrade aborts. This constraint will be checked prior to starting any batch. The default value for this parameter is 20%."]
    #[serde(rename = "maxUnhealthyInstancePercent", default, skip_serializing_if = "Option::is_none")]
    pub max_unhealthy_instance_percent: Option<i32>,
    #[doc = "The maximum percentage of upgraded virtual machine instances that can be found to be in an unhealthy state. This check will happen after each batch is upgraded. If this percentage is ever exceeded, the rolling update aborts. The default value for this parameter is 20%."]
    #[serde(rename = "maxUnhealthyUpgradedInstancePercent", default, skip_serializing_if = "Option::is_none")]
    pub max_unhealthy_upgraded_instance_percent: Option<i32>,
    #[doc = "The wait time between completing the update for all virtual machines in one batch and starting the next batch. The time duration should be specified in ISO 8601 format. The default value is 0 seconds (PT0S)."]
    #[serde(rename = "pauseTimeBetweenBatches", default, skip_serializing_if = "Option::is_none")]
    pub pause_time_between_batches: Option<String>,
    #[doc = "Allow VMSS to ignore AZ boundaries when constructing upgrade batches. Take into consideration the Update Domain and maxBatchInstancePercent to determine the batch size."]
    #[serde(rename = "enableCrossZoneUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub enable_cross_zone_upgrade: Option<bool>,
    #[doc = "Upgrade all unhealthy instances in a scale set before any healthy instances."]
    #[serde(rename = "prioritizeUnhealthyInstances", default, skip_serializing_if = "Option::is_none")]
    pub prioritize_unhealthy_instances: Option<bool>,
}
impl RollingUpgradePolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the number of virtual machine instances in each upgrade state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RollingUpgradeProgressInfo {
    #[doc = "The number of instances that have been successfully upgraded."]
    #[serde(rename = "successfulInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub successful_instance_count: Option<i32>,
    #[doc = "The number of instances that have failed to be upgraded successfully."]
    #[serde(rename = "failedInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_instance_count: Option<i32>,
    #[doc = "The number of instances that are currently being upgraded."]
    #[serde(rename = "inProgressInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub in_progress_instance_count: Option<i32>,
    #[doc = "The number of instances that have not yet begun to be upgraded."]
    #[serde(rename = "pendingInstanceCount", default, skip_serializing_if = "Option::is_none")]
    pub pending_instance_count: Option<i32>,
}
impl RollingUpgradeProgressInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the current running state of the overall upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RollingUpgradeRunningStatus {
    #[doc = "Code indicating the current status of the upgrade."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<rolling_upgrade_running_status::Code>,
    #[doc = "Start time of the upgrade."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The last action performed on the rolling upgrade."]
    #[serde(rename = "lastAction", default, skip_serializing_if = "Option::is_none")]
    pub last_action: Option<rolling_upgrade_running_status::LastAction>,
    #[doc = "Last action time of the upgrade."]
    #[serde(rename = "lastActionTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_action_time: Option<time::OffsetDateTime>,
}
impl RollingUpgradeRunningStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod rolling_upgrade_running_status {
    use super::*;
    #[doc = "Code indicating the current status of the upgrade."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Code {
        RollingForward,
        Cancelled,
        Completed,
        Faulted,
    }
    #[doc = "The last action performed on the rolling upgrade."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastAction {
        Start,
        Cancel,
    }
}
#[doc = "The status of the latest virtual machine scale set rolling upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RollingUpgradeStatusInfo {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The status of the latest virtual machine scale set rolling upgrade."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RollingUpgradeStatusInfoProperties>,
}
impl RollingUpgradeStatusInfo {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "The status of the latest virtual machine scale set rolling upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RollingUpgradeStatusInfoProperties {
    #[doc = "The configuration parameters used while performing a rolling upgrade."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<RollingUpgradePolicy>,
    #[doc = "Information about the current running state of the overall upgrade."]
    #[serde(rename = "runningStatus", default, skip_serializing_if = "Option::is_none")]
    pub running_status: Option<RollingUpgradeRunningStatus>,
    #[doc = "Information about the number of virtual machine instances in each upgrade state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<RollingUpgradeProgressInfo>,
    #[doc = "Api error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}
impl RollingUpgradeStatusInfoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Run Command."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunCommandDocument {
    #[serde(flatten)]
    pub run_command_document_base: RunCommandDocumentBase,
    #[doc = "The script to be executed."]
    pub script: Vec<String>,
    #[doc = "The parameters used by the script."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<RunCommandParameterDefinition>,
}
impl RunCommandDocument {
    pub fn new(run_command_document_base: RunCommandDocumentBase, script: Vec<String>) -> Self {
        Self {
            run_command_document_base,
            script,
            parameters: Vec::new(),
        }
    }
}
#[doc = "Describes the properties of a Run Command metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunCommandDocumentBase {
    #[doc = "The VM run command schema."]
    #[serde(rename = "$schema")]
    pub schema: String,
    #[doc = "The VM run command id."]
    pub id: String,
    #[doc = "The Operating System type."]
    #[serde(rename = "osType")]
    pub os_type: run_command_document_base::OsType,
    #[doc = "The VM run command label."]
    pub label: String,
    #[doc = "The VM run command description."]
    pub description: String,
}
impl RunCommandDocumentBase {
    pub fn new(schema: String, id: String, os_type: run_command_document_base::OsType, label: String, description: String) -> Self {
        Self {
            schema,
            id,
            os_type,
            label,
            description,
        }
    }
}
pub mod run_command_document_base {
    use super::*;
    #[doc = "The Operating System type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[doc = "Capture Virtual Machine parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunCommandInput {
    #[doc = "The run command id."]
    #[serde(rename = "commandId")]
    pub command_id: String,
    #[doc = "Optional. The script to be executed.  When this value is given, the given script will override the default script of the command."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub script: Vec<String>,
    #[doc = "The run command parameters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<RunCommandInputParameter>,
}
impl RunCommandInput {
    pub fn new(command_id: String) -> Self {
        Self {
            command_id,
            script: Vec::new(),
            parameters: Vec::new(),
        }
    }
}
#[doc = "Describes the properties of a run command parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunCommandInputParameter {
    #[doc = "The run command parameter name."]
    pub name: String,
    #[doc = "The run command parameter value."]
    pub value: String,
}
impl RunCommandInputParameter {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
#[doc = "The List Virtual Machine operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunCommandListResult {
    #[doc = "The list of virtual machine run commands."]
    pub value: Vec<RunCommandDocumentBase>,
    #[doc = "The uri to fetch the next page of run commands. Call ListNext() with this to fetch the next page of run commands."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RunCommandListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RunCommandListResult {
    pub fn new(value: Vec<RunCommandDocumentBase>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the properties of a run command parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunCommandParameterDefinition {
    #[doc = "The run command parameter name."]
    pub name: String,
    #[doc = "The run command parameter type."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The run command parameter default value."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "The run command parameter required."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}
impl RunCommandParameterDefinition {
    pub fn new(name: String, type_: String) -> Self {
        Self {
            name,
            type_,
            default_value: None,
            required: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunCommandResult {
    #[doc = "Run command operation response."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<InstanceViewStatus>,
}
impl RunCommandResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a scale-in policy for a virtual machine scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScaleInPolicy {
    #[doc = "The rules to be followed when scaling-in a virtual machine scale set. <br><br> Possible values are: <br><br> **Default** When a virtual machine scale set is scaled in, the scale set will first be balanced across zones if it is a zonal scale set. Then, it will be balanced across Fault Domains as far as possible. Within each Fault Domain, the virtual machines chosen for removal will be the newest ones that are not protected from scale-in. <br><br> **OldestVM** When a virtual machine scale set is being scaled-in, the oldest virtual machines that are not protected from scale-in will be chosen for removal. For zonal virtual machine scale sets, the scale set will first be balanced across zones. Within each zone, the oldest virtual machines that are not protected will be chosen for removal. <br><br> **NewestVM** When a virtual machine scale set is being scaled-in, the newest virtual machines that are not protected from scale-in will be chosen for removal. For zonal virtual machine scale sets, the scale set will first be balanced across zones. Within each zone, the newest virtual machines that are not protected will be chosen for removal. <br><br>"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<String>,
    #[doc = "This property allows you to specify if virtual machines chosen for removal have to be force deleted when a virtual machine scale set is being scaled-in.(Feature in Preview)"]
    #[serde(rename = "forceDeletion", default, skip_serializing_if = "Option::is_none")]
    pub force_deletion: Option<bool>,
}
impl ScaleInPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduledEventsProfile {
    #[serde(rename = "terminateNotificationProfile", default, skip_serializing_if = "Option::is_none")]
    pub terminate_notification_profile: Option<TerminateNotificationProfile>,
}
impl ScheduledEventsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the Security profile settings for the virtual machine or virtual machine scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityProfile {
    #[doc = "Specifies the security settings like secure boot and vTPM used while creating the virtual machine. <br><br>Minimum api-version: 2020-12-01"]
    #[serde(rename = "uefiSettings", default, skip_serializing_if = "Option::is_none")]
    pub uefi_settings: Option<UefiSettings>,
    #[doc = "This property can be used by user in the request to enable or disable the Host Encryption for the virtual machine or virtual machine scale set. This will enable the encryption for all the disks including Resource/Temp disk at host itself. <br><br> Default: The Encryption at host will be disabled unless this property is set to true for the resource."]
    #[serde(rename = "encryptionAtHost", default, skip_serializing_if = "Option::is_none")]
    pub encryption_at_host: Option<bool>,
    #[doc = "Specifies the SecurityType of the virtual machine. It has to be set to any specified value to enable UefiSettings. <br><br> Default: UefiSettings will not be enabled unless this property is set."]
    #[serde(rename = "securityType", default, skip_serializing_if = "Option::is_none")]
    pub security_type: Option<security_profile::SecurityType>,
}
impl SecurityProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod security_profile {
    use super::*;
    #[doc = "Specifies the SecurityType of the virtual machine. It has to be set to any specified value to enable UefiSettings. <br><br> Default: UefiSettings will not be enabled unless this property is set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SecurityType")]
    pub enum SecurityType {
        TrustedLaunch,
        #[serde(rename = "ConfidentialVM")]
        ConfidentialVm,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SecurityType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SecurityType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SecurityType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::TrustedLaunch => serializer.serialize_unit_variant("SecurityType", 0u32, "TrustedLaunch"),
                Self::ConfidentialVm => serializer.serialize_unit_variant("SecurityType", 1u32, "ConfidentialVM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareInfoElement {
    #[doc = "A relative URI containing the ID of the VM that has the disk attached."]
    #[serde(rename = "vmUri", default, skip_serializing_if = "Option::is_none")]
    pub vm_uri: Option<String>,
}
impl ShareInfoElement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the Shared Gallery that you want to create or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedGallery {
    #[serde(flatten)]
    pub pir_shared_gallery_resource: PirSharedGalleryResource,
}
impl SharedGallery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the data disk image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedGalleryDataDiskImage {
    #[serde(flatten)]
    pub shared_gallery_disk_image: SharedGalleryDiskImage,
    #[doc = "This property specifies the logical unit number of the data disk. This value is used to identify data disks within the Virtual Machine and therefore must be unique for each data disk attached to the Virtual Machine."]
    pub lun: i32,
}
impl SharedGalleryDataDiskImage {
    pub fn new(lun: i32) -> Self {
        Self {
            shared_gallery_disk_image: SharedGalleryDiskImage::default(),
            lun,
        }
    }
}
#[doc = "This is the disk image base class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedGalleryDiskImage {
    #[doc = "This property indicates the size of the VHD to be created."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "The host caching of the disk. Valid values are 'None', 'ReadOnly', and 'ReadWrite'"]
    #[serde(rename = "hostCaching", default, skip_serializing_if = "Option::is_none")]
    pub host_caching: Option<shared_gallery_disk_image::HostCaching>,
}
impl SharedGalleryDiskImage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod shared_gallery_disk_image {
    use super::*;
    #[doc = "The host caching of the disk. Valid values are 'None', 'ReadOnly', and 'ReadWrite'"]
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
#[doc = "The identifier information of shared gallery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedGalleryIdentifier {
    #[doc = "The unique id of this shared gallery."]
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}
impl SharedGalleryIdentifier {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the gallery image definition that you want to create or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedGalleryImage {
    #[serde(flatten)]
    pub pir_shared_gallery_resource: PirSharedGalleryResource,
    #[doc = "Describes the properties of a gallery image definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedGalleryImageProperties>,
}
impl SharedGalleryImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Shared Gallery Images operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedGalleryImageList {
    #[doc = "A list of shared gallery images."]
    pub value: Vec<SharedGalleryImage>,
    #[doc = "The uri to fetch the next page of shared gallery images. Call ListNext() with this to fetch the next page of shared gallery images."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SharedGalleryImageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SharedGalleryImageList {
    pub fn new(value: Vec<SharedGalleryImage>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the properties of a gallery image definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedGalleryImageProperties {
    #[doc = "This property allows you to specify the type of the OS that is included in the disk when creating a VM from a managed image. <br><br> Possible values are: <br><br> **Windows** <br><br> **Linux**"]
    #[serde(rename = "osType")]
    pub os_type: shared_gallery_image_properties::OsType,
    #[doc = "This property allows the user to specify whether the virtual machines created under this image are 'Generalized' or 'Specialized'."]
    #[serde(rename = "osState")]
    pub os_state: shared_gallery_image_properties::OsState,
    #[doc = "The end of life date of the gallery image definition. This property can be used for decommissioning purposes. This property is updatable."]
    #[serde(rename = "endOfLifeDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_of_life_date: Option<time::OffsetDateTime>,
    #[doc = "This is the gallery image definition identifier."]
    pub identifier: GalleryImageIdentifier,
    #[doc = "The properties describe the recommended machine configuration for this Image Definition. These properties are updatable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommended: Option<RecommendedMachineConfiguration>,
    #[doc = "Describes the disallowed disk types."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disallowed: Option<Disallowed>,
    #[doc = "The hypervisor generation of the Virtual Machine. Applicable to OS disks only."]
    #[serde(rename = "hyperVGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<shared_gallery_image_properties::HyperVGeneration>,
    #[doc = "A list of gallery image features."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<GalleryImageFeature>,
    #[doc = "Describes the gallery image definition purchase plan. This is used by marketplace images."]
    #[serde(rename = "purchasePlan", default, skip_serializing_if = "Option::is_none")]
    pub purchase_plan: Option<ImagePurchasePlan>,
    #[doc = "The architecture of the image. Applicable to OS disks only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<Architecture>,
}
impl SharedGalleryImageProperties {
    pub fn new(
        os_type: shared_gallery_image_properties::OsType,
        os_state: shared_gallery_image_properties::OsState,
        identifier: GalleryImageIdentifier,
    ) -> Self {
        Self {
            os_type,
            os_state,
            end_of_life_date: None,
            identifier,
            recommended: None,
            disallowed: None,
            hyper_v_generation: None,
            features: Vec::new(),
            purchase_plan: None,
            architecture: None,
        }
    }
}
pub mod shared_gallery_image_properties {
    use super::*;
    #[doc = "This property allows you to specify the type of the OS that is included in the disk when creating a VM from a managed image. <br><br> Possible values are: <br><br> **Windows** <br><br> **Linux**"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[doc = "This property allows the user to specify whether the virtual machines created under this image are 'Generalized' or 'Specialized'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsState {
        Generalized,
        Specialized,
    }
    #[doc = "The hypervisor generation of the Virtual Machine. Applicable to OS disks only."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HyperVGeneration")]
    pub enum HyperVGeneration {
        V1,
        V2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HyperVGeneration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HyperVGeneration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HyperVGeneration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::V1 => serializer.serialize_unit_variant("HyperVGeneration", 0u32, "V1"),
                Self::V2 => serializer.serialize_unit_variant("HyperVGeneration", 1u32, "V2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies information about the gallery image version that you want to create or update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedGalleryImageVersion {
    #[serde(flatten)]
    pub pir_shared_gallery_resource: PirSharedGalleryResource,
    #[doc = "Describes the properties of a gallery image version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedGalleryImageVersionProperties>,
}
impl SharedGalleryImageVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Shared Gallery Image versions operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedGalleryImageVersionList {
    #[doc = "A list of shared gallery images versions."]
    pub value: Vec<SharedGalleryImageVersion>,
    #[doc = "The uri to fetch the next page of shared gallery image versions. Call ListNext() with this to fetch the next page of shared gallery image versions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SharedGalleryImageVersionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SharedGalleryImageVersionList {
    pub fn new(value: Vec<SharedGalleryImageVersion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the properties of a gallery image version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedGalleryImageVersionProperties {
    #[doc = "The published date of the gallery image version Definition. This property can be used for decommissioning purposes. This property is updatable."]
    #[serde(rename = "publishedDate", default, with = "azure_core::date::rfc3339::option")]
    pub published_date: Option<time::OffsetDateTime>,
    #[doc = "The end of life date of the gallery image version Definition. This property can be used for decommissioning purposes. This property is updatable."]
    #[serde(rename = "endOfLifeDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_of_life_date: Option<time::OffsetDateTime>,
    #[doc = "If set to true, Virtual Machines deployed from the latest version of the Image Definition won't use this Image Version."]
    #[serde(rename = "excludeFromLatest", default, skip_serializing_if = "Option::is_none")]
    pub exclude_from_latest: Option<bool>,
    #[doc = "This is the storage profile of a Gallery Image Version."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<SharedGalleryImageVersionStorageProfile>,
}
impl SharedGalleryImageVersionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the storage profile of a Gallery Image Version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedGalleryImageVersionStorageProfile {
    #[doc = "This is the OS disk image."]
    #[serde(rename = "osDiskImage", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_image: Option<SharedGalleryOsDiskImage>,
    #[doc = "A list of data disk images."]
    #[serde(rename = "dataDiskImages", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disk_images: Vec<SharedGalleryDataDiskImage>,
}
impl SharedGalleryImageVersionStorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Shared Galleries operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedGalleryList {
    #[doc = "A list of shared galleries."]
    pub value: Vec<SharedGallery>,
    #[doc = "The uri to fetch the next page of shared galleries. Call ListNext() with this to fetch the next page of shared galleries."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SharedGalleryList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SharedGalleryList {
    pub fn new(value: Vec<SharedGallery>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "This is the OS disk image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedGalleryOsDiskImage {
    #[serde(flatten)]
    pub shared_gallery_disk_image: SharedGalleryDiskImage,
}
impl SharedGalleryOsDiskImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Profile for gallery sharing to subscription or tenant"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharingProfile {
    #[doc = "This property allows you to specify the permission of sharing gallery. <br><br> Possible values are: <br><br> **Private** <br><br> **Groups** <br><br> **Community**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<sharing_profile::Permissions>,
    #[doc = "A list of sharing profile groups."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<SharingProfileGroup>,
    #[doc = "Information of community gallery if current gallery is shared to community"]
    #[serde(rename = "communityGalleryInfo", default, skip_serializing_if = "Option::is_none")]
    pub community_gallery_info: Option<CommunityGalleryInfo>,
}
impl SharingProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sharing_profile {
    use super::*;
    #[doc = "This property allows you to specify the permission of sharing gallery. <br><br> Possible values are: <br><br> **Private** <br><br> **Groups** <br><br> **Community**"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Permissions")]
    pub enum Permissions {
        Private,
        Groups,
        Community,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Permissions {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Permissions {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Permissions {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Private => serializer.serialize_unit_variant("Permissions", 0u32, "Private"),
                Self::Groups => serializer.serialize_unit_variant("Permissions", 1u32, "Groups"),
                Self::Community => serializer.serialize_unit_variant("Permissions", 2u32, "Community"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Group of the gallery sharing profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharingProfileGroup {
    #[doc = "This property allows you to specify the type of sharing group. <br><br> Possible values are: <br><br> **Subscriptions** <br><br> **AADTenants**"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<sharing_profile_group::Type>,
    #[doc = "A list of subscription/tenant ids the gallery is aimed to be shared to."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ids: Vec<String>,
}
impl SharingProfileGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sharing_profile_group {
    use super::*;
    #[doc = "This property allows you to specify the type of sharing group. <br><br> Possible values are: <br><br> **Subscriptions** <br><br> **AADTenants**"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Subscriptions,
        #[serde(rename = "AADTenants")]
        AadTenants,
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
                Self::Subscriptions => serializer.serialize_unit_variant("Type", 0u32, "Subscriptions"),
                Self::AadTenants => serializer.serialize_unit_variant("Type", 1u32, "AADTenants"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The sharing state of the gallery, which only appears in the response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SharingState")]
pub enum SharingState {
    Succeeded,
    InProgress,
    Failed,
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SharingState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SharingState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SharingState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("SharingState", 0u32, "Succeeded"),
            Self::InProgress => serializer.serialize_unit_variant("SharingState", 1u32, "InProgress"),
            Self::Failed => serializer.serialize_unit_variant("SharingState", 2u32, "Failed"),
            Self::Unknown => serializer.serialize_unit_variant("SharingState", 3u32, "Unknown"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Sharing status of current gallery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharingStatus {
    #[doc = "The sharing state of the gallery, which only appears in the response."]
    #[serde(rename = "aggregatedState", default, skip_serializing_if = "Option::is_none")]
    pub aggregated_state: Option<SharingState>,
    #[doc = "Summary of all regional sharing status."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub summary: Vec<RegionalSharingStatus>,
}
impl SharingStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the gallery sharing profile update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharingUpdate {
    #[doc = "This property allows you to specify the operation type of gallery sharing update. <br><br> Possible values are: <br><br> **Add** <br><br> **Remove** <br><br> **Reset**"]
    #[serde(rename = "operationType")]
    pub operation_type: sharing_update::OperationType,
    #[doc = "A list of sharing profile groups."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<SharingProfileGroup>,
}
impl SharingUpdate {
    pub fn new(operation_type: sharing_update::OperationType) -> Self {
        Self {
            operation_type,
            groups: Vec::new(),
        }
    }
}
pub mod sharing_update {
    use super::*;
    #[doc = "This property allows you to specify the operation type of gallery sharing update. <br><br> Possible values are: <br><br> **Add** <br><br> **Remove** <br><br> **Reset**"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationType")]
    pub enum OperationType {
        Add,
        Remove,
        Reset,
        EnableCommunity,
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
                Self::Add => serializer.serialize_unit_variant("OperationType", 0u32, "Add"),
                Self::Remove => serializer.serialize_unit_variant("OperationType", 1u32, "Remove"),
                Self::Reset => serializer.serialize_unit_variant("OperationType", 2u32, "Reset"),
                Self::EnableCommunity => serializer.serialize_unit_variant("OperationType", 3u32, "EnableCommunity"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a virtual machine scale set sku. NOTE: If the new VM SKU is not supported on the hardware the scale set is currently on, you need to deallocate the VMs in the scale set before you modify the SKU name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "The sku name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the tier of virtual machines in a scale set.<br /><br /> Possible Values:<br /><br /> **Standard**<br /><br /> **Basic**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "Specifies the number of virtual machines in the scale set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Snapshot resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Unused. Always Null."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[doc = "The snapshots sku name. Can be Standard_LRS, Premium_LRS, or Standard_ZRS. This is an optional parameter for incremental snapshot and the default behavior is the SKU will be set to the same sku as the previous snapshot"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SnapshotSku>,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "Snapshot resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotProperties>,
}
impl Snapshot {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            managed_by: None,
            sku: None,
            extended_location: None,
            properties: None,
        }
    }
}
#[doc = "The List Snapshots operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotList {
    #[doc = "A list of snapshots."]
    pub value: Vec<Snapshot>,
    #[doc = "The uri to fetch the next page of snapshots. Call ListNext() with this to fetch the next page of snapshots."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SnapshotList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SnapshotList {
    pub fn new(value: Vec<Snapshot>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Snapshot resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotProperties {
    #[doc = "The time when the snapshot was created."]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<time::OffsetDateTime>,
    #[doc = "The Operating System type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<snapshot_properties::OsType>,
    #[doc = "The hypervisor generation of the Virtual Machine. Applicable to OS disks only."]
    #[serde(rename = "hyperVGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<snapshot_properties::HyperVGeneration>,
    #[doc = "Used for establishing the purchase context of any 3rd Party artifact through MarketPlace."]
    #[serde(rename = "purchasePlan", default, skip_serializing_if = "Option::is_none")]
    pub purchase_plan: Option<PurchasePlan>,
    #[doc = "List of supported capabilities persisted on the disk resource for VM use."]
    #[serde(rename = "supportedCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub supported_capabilities: Option<SupportedCapabilities>,
    #[doc = "Data used when creating a disk."]
    #[serde(rename = "creationData")]
    pub creation_data: CreationData,
    #[doc = "If creationData.createOption is Empty, this field is mandatory and it indicates the size of the disk to create. If this field is present for updates or creation with other options, it indicates a resize. Resizes are only allowed if the disk is not attached to a running VM, and can only increase the disk's size."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "The size of the disk in bytes. This field is read only."]
    #[serde(rename = "diskSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_bytes: Option<i64>,
    #[doc = "This enumerates the possible state of the disk."]
    #[serde(rename = "diskState", default, skip_serializing_if = "Option::is_none")]
    pub disk_state: Option<DiskState>,
    #[doc = "Unique Guid identifying the resource."]
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[doc = "Encryption settings for disk or snapshot"]
    #[serde(rename = "encryptionSettingsCollection", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
    #[doc = "The disk provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Whether a snapshot is incremental. Incremental snapshots on the same disk occupy less space than full snapshots and can be diffed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incremental: Option<bool>,
    #[doc = "Encryption at rest settings for disk or snapshot"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "Policy for accessing the disk via network."]
    #[serde(rename = "networkAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_access_policy: Option<NetworkAccessPolicy>,
    #[doc = "ARM id of the DiskAccess resource for using private endpoints on disks."]
    #[serde(rename = "diskAccessId", default, skip_serializing_if = "Option::is_none")]
    pub disk_access_id: Option<String>,
    #[doc = "Contains the security related information for the resource."]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<DiskSecurityProfile>,
    #[doc = "Indicates the OS on a snapshot supports hibernation."]
    #[serde(rename = "supportsHibernation", default, skip_serializing_if = "Option::is_none")]
    pub supports_hibernation: Option<bool>,
    #[doc = "Policy for controlling export on the disk."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
    #[doc = "Percentage complete for the background copy when a resource is created via the CopyStart operation."]
    #[serde(rename = "completionPercent", default, skip_serializing_if = "Option::is_none")]
    pub completion_percent: Option<f64>,
    #[doc = "Additional authentication requirements when exporting or uploading to a disk or snapshot."]
    #[serde(rename = "dataAccessAuthMode", default, skip_serializing_if = "Option::is_none")]
    pub data_access_auth_mode: Option<DataAccessAuthMode>,
}
impl SnapshotProperties {
    pub fn new(creation_data: CreationData) -> Self {
        Self {
            time_created: None,
            os_type: None,
            hyper_v_generation: None,
            purchase_plan: None,
            supported_capabilities: None,
            creation_data,
            disk_size_gb: None,
            disk_size_bytes: None,
            disk_state: None,
            unique_id: None,
            encryption_settings_collection: None,
            provisioning_state: None,
            incremental: None,
            encryption: None,
            network_access_policy: None,
            disk_access_id: None,
            security_profile: None,
            supports_hibernation: None,
            public_network_access: None,
            completion_percent: None,
            data_access_auth_mode: None,
        }
    }
}
pub mod snapshot_properties {
    use super::*;
    #[doc = "The Operating System type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[doc = "The hypervisor generation of the Virtual Machine. Applicable to OS disks only."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HyperVGeneration")]
    pub enum HyperVGeneration {
        V1,
        V2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HyperVGeneration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HyperVGeneration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HyperVGeneration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::V1 => serializer.serialize_unit_variant("HyperVGeneration", 0u32, "V1"),
                Self::V2 => serializer.serialize_unit_variant("HyperVGeneration", 1u32, "V2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The snapshots sku name. Can be Standard_LRS, Premium_LRS, or Standard_ZRS. This is an optional parameter for incremental snapshot and the default behavior is the SKU will be set to the same sku as the previous snapshot"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotSku {
    #[doc = "The sku name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<snapshot_sku::Name>,
    #[doc = "The sku tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl SnapshotSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod snapshot_sku {
    use super::*;
    #[doc = "The sku name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("Name", 0u32, "Standard_LRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("Name", 1u32, "Premium_LRS"),
                Self::StandardZrs => serializer.serialize_unit_variant("Name", 2u32, "Standard_ZRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Snapshot update resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotUpdate {
    #[doc = "Snapshot resource update properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotUpdateProperties>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The snapshots sku name. Can be Standard_LRS, Premium_LRS, or Standard_ZRS. This is an optional parameter for incremental snapshot and the default behavior is the SKU will be set to the same sku as the previous snapshot"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SnapshotSku>,
}
impl SnapshotUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Snapshot resource update properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotUpdateProperties {
    #[doc = "the Operating System type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<snapshot_update_properties::OsType>,
    #[doc = "If creationData.createOption is Empty, this field is mandatory and it indicates the size of the disk to create. If this field is present for updates or creation with other options, it indicates a resize. Resizes are only allowed if the disk is not attached to a running VM, and can only increase the disk's size."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "Encryption settings for disk or snapshot"]
    #[serde(rename = "encryptionSettingsCollection", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings_collection: Option<EncryptionSettingsCollection>,
    #[doc = "Encryption at rest settings for disk or snapshot"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "Policy for accessing the disk via network."]
    #[serde(rename = "networkAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub network_access_policy: Option<NetworkAccessPolicy>,
    #[doc = "ARM id of the DiskAccess resource for using private endpoints on disks."]
    #[serde(rename = "diskAccessId", default, skip_serializing_if = "Option::is_none")]
    pub disk_access_id: Option<String>,
    #[doc = "Indicates the OS on a snapshot supports hibernation."]
    #[serde(rename = "supportsHibernation", default, skip_serializing_if = "Option::is_none")]
    pub supports_hibernation: Option<bool>,
    #[doc = "Policy for controlling export on the disk."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
    #[doc = "Additional authentication requirements when exporting or uploading to a disk or snapshot."]
    #[serde(rename = "dataAccessAuthMode", default, skip_serializing_if = "Option::is_none")]
    pub data_access_auth_mode: Option<DataAccessAuthMode>,
    #[doc = "List of supported capabilities persisted on the disk resource for VM use."]
    #[serde(rename = "supportedCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub supported_capabilities: Option<SupportedCapabilities>,
}
impl SnapshotUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod snapshot_update_properties {
    use super::*;
    #[doc = "the Operating System type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[doc = "Contains information about the soft deletion policy of the gallery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SoftDeletePolicy {
    #[doc = "Enables soft-deletion for resources in this gallery, allowing them to be recovered within retention time."]
    #[serde(rename = "isSoftDeleteEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_soft_delete_enabled: Option<bool>,
}
impl SoftDeletePolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The vault id is an Azure Resource Manager Resource id in the form /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceVault {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SourceVault {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the Spot-Try-Restore properties for the virtual machine scale set. <br><br> With this property customer can enable or disable automatic restore of the evicted Spot VMSS VM instances opportunistically based on capacity availability and pricing constraint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpotRestorePolicy {
    #[doc = "Enables the Spot-Try-Restore feature where evicted VMSS SPOT instances will be tried to be restored opportunistically based on capacity availability and pricing constraints"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Timeout value expressed as an ISO 8601 time duration after which the platform will not try to restore the VMSS SPOT instances"]
    #[serde(rename = "restoreTimeout", default, skip_serializing_if = "Option::is_none")]
    pub restore_timeout: Option<String>,
}
impl SpotRestorePolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SSH configuration for Linux based VMs running on Azure"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SshConfiguration {
    #[doc = "The list of SSH public keys used to authenticate with linux based VMs."]
    #[serde(rename = "publicKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub public_keys: Vec<SshPublicKey>,
}
impl SshConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains information about SSH certificate public key and the path on the Linux VM where the public key is placed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SshPublicKey {
    #[doc = "Specifies the full path on the created VM where ssh public key is stored. If the file already exists, the specified key is appended to the file. Example: /home/user/.ssh/authorized_keys"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "SSH public key certificate used to authenticate with the VM through ssh. The key needs to be at least 2048-bit and in ssh-rsa format. <br><br> For creating ssh keys, see [Create SSH keys on Linux and Mac for Linux VMs in Azure]https://docs.microsoft.com/azure/virtual-machines/linux/create-ssh-keys-detailed)."]
    #[serde(rename = "keyData", default, skip_serializing_if = "Option::is_none")]
    pub key_data: Option<String>,
}
impl SshPublicKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response from generation of an SSH key pair."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SshPublicKeyGenerateKeyPairResult {
    #[doc = "Private key portion of the key pair used to authenticate to a virtual machine through ssh. The private key is returned in RFC3447 format and should be treated as a secret."]
    #[serde(rename = "privateKey")]
    pub private_key: String,
    #[doc = "Public key portion of the key pair used to authenticate to a virtual machine through ssh. The public key is in ssh-rsa format."]
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[doc = "The ARM resource id in the form of /subscriptions/{SubscriptionId}/resourceGroups/{ResourceGroupName}/providers/Microsoft.Compute/sshPublicKeys/{SshPublicKeyName}"]
    pub id: String,
}
impl SshPublicKeyGenerateKeyPairResult {
    pub fn new(private_key: String, public_key: String, id: String) -> Self {
        Self {
            private_key,
            public_key,
            id,
        }
    }
}
#[doc = "Specifies information about the SSH public key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SshPublicKeyResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the SSH public key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SshPublicKeyResourceProperties>,
}
impl SshPublicKeyResource {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "Properties of the SSH public key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SshPublicKeyResourceProperties {
    #[doc = "SSH public key used to authenticate to a virtual machine through ssh. If this property is not initially provided when the resource is created, the publicKey property will be populated when generateKeyPair is called. If the public key is provided upon resource creation, the provided public key needs to be at least 2048-bit and in ssh-rsa format."]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}
impl SshPublicKeyResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the SSH public key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SshPublicKeyUpdateResource {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "Properties of the SSH public key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SshPublicKeyResourceProperties>,
}
impl SshPublicKeyUpdateResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list SSH public keys operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SshPublicKeysGroupListResult {
    #[doc = "The list of SSH public keys"]
    pub value: Vec<SshPublicKeyResource>,
    #[doc = "The URI to fetch the next page of SSH public keys. Call ListNext() with this URI to fetch the next page of SSH public keys."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SshPublicKeysGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SshPublicKeysGroupListResult {
    pub fn new(value: Vec<SshPublicKeyResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusCodeCount {
    #[doc = "The instance view status code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Number of instances having this status code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
impl StatusCodeCount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the storage account type for the managed disk. Managed OS disk storage account type can only be set when you create the scale set. NOTE: UltraSSD_LRS can only be used with data disks. It cannot be used with OS Disk. Standard_LRS uses Standard HDD. StandardSSD_LRS uses Standard SSD. Premium_LRS uses Premium SSD. UltraSSD_LRS uses Ultra disk. Premium_ZRS uses Premium SSD zone redundant storage. StandardSSD_ZRS uses Standard SSD zone redundant storage. For more information regarding disks supported for Windows Virtual Machines, refer to https://docs.microsoft.com/azure/virtual-machines/windows/disks-types and, for Linux Virtual Machines, refer to https://docs.microsoft.com/azure/virtual-machines/linux/disks-types"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StorageAccountType")]
pub enum StorageAccountType {
    #[serde(rename = "Standard_LRS")]
    StandardLrs,
    #[serde(rename = "Premium_LRS")]
    PremiumLrs,
    #[serde(rename = "StandardSSD_LRS")]
    StandardSsdLrs,
    #[serde(rename = "UltraSSD_LRS")]
    UltraSsdLrs,
    #[serde(rename = "Premium_ZRS")]
    PremiumZrs,
    #[serde(rename = "StandardSSD_ZRS")]
    StandardSsdZrs,
    #[serde(rename = "PremiumV2_LRS")]
    PremiumV2Lrs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StorageAccountType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StorageAccountType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StorageAccountType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StandardLrs => serializer.serialize_unit_variant("StorageAccountType", 0u32, "Standard_LRS"),
            Self::PremiumLrs => serializer.serialize_unit_variant("StorageAccountType", 1u32, "Premium_LRS"),
            Self::StandardSsdLrs => serializer.serialize_unit_variant("StorageAccountType", 2u32, "StandardSSD_LRS"),
            Self::UltraSsdLrs => serializer.serialize_unit_variant("StorageAccountType", 3u32, "UltraSSD_LRS"),
            Self::PremiumZrs => serializer.serialize_unit_variant("StorageAccountType", 4u32, "Premium_ZRS"),
            Self::StandardSsdZrs => serializer.serialize_unit_variant("StorageAccountType", 5u32, "StandardSSD_ZRS"),
            Self::PremiumV2Lrs => serializer.serialize_unit_variant("StorageAccountType", 6u32, "PremiumV2_LRS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies the storage settings for the virtual machine disks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[doc = "Specifies information about the image to use. You can specify information about platform images, marketplace images, or virtual machine images. This element is required when you want to use a platform image, marketplace image, or virtual machine image, but is not used in other creation operations. NOTE: Image reference publisher and offer can only be set when you create the scale set."]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[doc = "Specifies information about the operating system disk used by the virtual machine. <br><br> For more information about disks, see [About disks and VHDs for Azure virtual machines](https://docs.microsoft.com/azure/virtual-machines/managed-disks-overview)."]
    #[serde(rename = "osDisk", default, skip_serializing_if = "Option::is_none")]
    pub os_disk: Option<OsDisk>,
    #[doc = "Specifies the parameters that are used to add a data disk to a virtual machine. <br><br> For more information about disks, see [About disks and VHDs for Azure virtual machines](https://docs.microsoft.com/azure/virtual-machines/managed-disks-overview)."]
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disks: Vec<DataDisk>,
}
impl StorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResourceReadOnly {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResourceReadOnly {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResourceWithColocationStatus {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Instance view status."]
    #[serde(rename = "colocationStatus", default, skip_serializing_if = "Option::is_none")]
    pub colocation_status: Option<InstanceViewStatus>,
}
impl SubResourceWithColocationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of supported capabilities persisted on the disk resource for VM use."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedCapabilities {
    #[doc = "True if the image from which the OS disk is created supports accelerated networking."]
    #[serde(rename = "acceleratedNetwork", default, skip_serializing_if = "Option::is_none")]
    pub accelerated_network: Option<bool>,
    #[doc = "CPU architecture supported by an OS disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<supported_capabilities::Architecture>,
}
impl SupportedCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod supported_capabilities {
    use super::*;
    #[doc = "CPU architecture supported by an OS disk."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Architecture")]
    pub enum Architecture {
        #[serde(rename = "x64")]
        X64,
        Arm64,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Architecture {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Architecture {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Architecture {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::X64 => serializer.serialize_unit_variant("Architecture", 0u32, "x64"),
                Self::Arm64 => serializer.serialize_unit_variant("Architecture", 1u32, "Arm64"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the target region information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetRegion {
    #[doc = "The name of the region."]
    pub name: String,
    #[doc = "The number of replicas of the Image Version to be created per region. This property is updatable."]
    #[serde(rename = "regionalReplicaCount", default, skip_serializing_if = "Option::is_none")]
    pub regional_replica_count: Option<i32>,
    #[doc = "Specifies the storage account type to be used to store the image. This property is not updatable."]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<target_region::StorageAccountType>,
    #[doc = "Optional. Allows users to provide customer managed keys for encrypting the OS and data disks in the gallery artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionImages>,
}
impl TargetRegion {
    pub fn new(name: String) -> Self {
        Self {
            name,
            regional_replica_count: None,
            storage_account_type: None,
            encryption: None,
        }
    }
}
pub mod target_region {
    use super::*;
    #[doc = "Specifies the storage account type to be used to store the image. This property is not updatable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageAccountType")]
    pub enum StorageAccountType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageAccountType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageAccountType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageAccountType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("StorageAccountType", 0u32, "Standard_LRS"),
                Self::StandardZrs => serializer.serialize_unit_variant("StorageAccountType", 1u32, "Standard_ZRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("StorageAccountType", 2u32, "Premium_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TerminateNotificationProfile {
    #[doc = "Configurable length of time a Virtual Machine being deleted will have to potentially approve the Terminate Scheduled Event before the event is auto approved (timed out). The configuration must be specified in ISO 8601 format, the default value is 5 minutes (PT5M)"]
    #[serde(rename = "notBeforeTimeout", default, skip_serializing_if = "Option::is_none")]
    pub not_before_timeout: Option<String>,
    #[doc = "Specifies whether the Terminate Scheduled event is enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}
impl TerminateNotificationProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Api request input for LogAnalytics getThrottledRequests Api."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThrottledRequestsInput {
    #[serde(flatten)]
    pub log_analytics_input_base: LogAnalyticsInputBase,
}
impl ThrottledRequestsInput {
    pub fn new(log_analytics_input_base: LogAnalyticsInputBase) -> Self {
        Self { log_analytics_input_base }
    }
}
#[doc = "Specifies the security settings like secure boot and vTPM used while creating the virtual machine. <br><br>Minimum api-version: 2020-12-01"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UefiSettings {
    #[doc = "Specifies whether secure boot should be enabled on the virtual machine. <br><br>Minimum api-version: 2020-12-01"]
    #[serde(rename = "secureBootEnabled", default, skip_serializing_if = "Option::is_none")]
    pub secure_boot_enabled: Option<bool>,
    #[doc = "Specifies whether vTPM should be enabled on the virtual machine. <br><br>Minimum api-version: 2020-12-01"]
    #[serde(rename = "vTpmEnabled", default, skip_serializing_if = "Option::is_none")]
    pub v_tpm_enabled: Option<bool>,
}
impl UefiSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines an update domain for the cloud service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateDomain {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl UpdateDomain {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDomainListResult {
    pub value: Vec<UpdateDomain>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UpdateDomainListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UpdateDomainListResult {
    pub fn new(value: Vec<UpdateDomain>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The Update Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateResource {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl UpdateResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Update Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateResourceDefinition {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl UpdateResourceDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual Machine Scale Set OS Upgrade History operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpgradeOperationHistoricalStatusInfo {
    #[doc = "Describes each OS upgrade on the Virtual Machine Scale Set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpgradeOperationHistoricalStatusInfoProperties>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl UpgradeOperationHistoricalStatusInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes each OS upgrade on the Virtual Machine Scale Set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpgradeOperationHistoricalStatusInfoProperties {
    #[doc = "Information about the current running state of the overall upgrade."]
    #[serde(rename = "runningStatus", default, skip_serializing_if = "Option::is_none")]
    pub running_status: Option<UpgradeOperationHistoryStatus>,
    #[doc = "Information about the number of virtual machine instances in each upgrade state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<RollingUpgradeProgressInfo>,
    #[doc = "Api error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
    #[doc = "Invoker of the Upgrade Operation"]
    #[serde(rename = "startedBy", default, skip_serializing_if = "Option::is_none")]
    pub started_by: Option<upgrade_operation_historical_status_info_properties::StartedBy>,
    #[doc = "Specifies information about the image to use. You can specify information about platform images, marketplace images, or virtual machine images. This element is required when you want to use a platform image, marketplace image, or virtual machine image, but is not used in other creation operations. NOTE: Image reference publisher and offer can only be set when you create the scale set."]
    #[serde(rename = "targetImageReference", default, skip_serializing_if = "Option::is_none")]
    pub target_image_reference: Option<ImageReference>,
    #[doc = "Information about rollback on failed VM instances after a OS Upgrade operation."]
    #[serde(rename = "rollbackInfo", default, skip_serializing_if = "Option::is_none")]
    pub rollback_info: Option<RollbackStatusInfo>,
}
impl UpgradeOperationHistoricalStatusInfoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod upgrade_operation_historical_status_info_properties {
    use super::*;
    #[doc = "Invoker of the Upgrade Operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StartedBy {
        Unknown,
        User,
        Platform,
    }
}
#[doc = "Information about the current running state of the overall upgrade."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpgradeOperationHistoryStatus {
    #[doc = "Code indicating the current status of the upgrade."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<upgrade_operation_history_status::Code>,
    #[doc = "Start time of the upgrade."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the upgrade."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl UpgradeOperationHistoryStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod upgrade_operation_history_status {
    use super::*;
    #[doc = "Code indicating the current status of the upgrade."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Code {
        RollingForward,
        Cancelled,
        Completed,
        Faulted,
    }
}
#[doc = "Describes an upgrade policy - automatic, manual, or rolling."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpgradePolicy {
    #[doc = "Specifies the mode of an upgrade to virtual machines in the scale set.<br /><br /> Possible values are:<br /><br /> **Manual** - You  control the application of updates to virtual machines in the scale set. You do this by using the manualUpgrade action.<br /><br /> **Automatic** - All virtual machines in the scale set are  automatically updated at the same time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<upgrade_policy::Mode>,
    #[doc = "The configuration parameters used while performing a rolling upgrade."]
    #[serde(rename = "rollingUpgradePolicy", default, skip_serializing_if = "Option::is_none")]
    pub rolling_upgrade_policy: Option<RollingUpgradePolicy>,
    #[doc = "The configuration parameters used for performing automatic OS upgrade."]
    #[serde(rename = "automaticOSUpgradePolicy", default, skip_serializing_if = "Option::is_none")]
    pub automatic_os_upgrade_policy: Option<AutomaticOsUpgradePolicy>,
}
impl UpgradePolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod upgrade_policy {
    use super::*;
    #[doc = "Specifies the mode of an upgrade to virtual machines in the scale set.<br /><br /> Possible values are:<br /><br /> **Manual** - You  control the application of updates to virtual machines in the scale set. You do this by using the manualUpgrade action.<br /><br /> **Automatic** - All virtual machines in the scale set are  automatically updated at the same time."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        Automatic,
        Manual,
        Rolling,
    }
}
#[doc = "Describes Compute Resource Usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    #[doc = "An enum describing the unit of usage measurement."]
    pub unit: usage::Unit,
    #[doc = "The current usage of the resource."]
    #[serde(rename = "currentValue")]
    pub current_value: i32,
    #[doc = "The maximum permitted usage of the resource."]
    pub limit: i64,
    #[doc = "The Usage Names."]
    pub name: UsageName,
}
impl Usage {
    pub fn new(unit: usage::Unit, current_value: i32, limit: i64, name: UsageName) -> Self {
        Self {
            unit,
            current_value,
            limit,
            name,
        }
    }
}
pub mod usage {
    use super::*;
    #[doc = "An enum describing the unit of usage measurement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Unit {
        Count,
    }
}
#[doc = "The Usage Names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageName {
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The localized name of the resource."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl UsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserArtifactManage {
    #[doc = "Required. The path and arguments to install the gallery application. This is limited to 4096 characters."]
    pub install: String,
    #[doc = "Required. The path and arguments to remove the gallery application. This is limited to 4096 characters."]
    pub remove: String,
    #[doc = "Optional. The path and arguments to update the gallery application. If not present, then update operation will invoke remove command on the previous version and install command on the current version of the gallery application. This is limited to 4096 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<String>,
}
impl UserArtifactManage {
    pub fn new(install: String, remove: String) -> Self {
        Self {
            install,
            remove,
            update: None,
        }
    }
}
#[doc = "Additional settings for the VM app that contains the target package and config file name when it is deployed to target VM or VM scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserArtifactSettings {
    #[doc = "Optional. The name to assign the downloaded package file on the VM. This is limited to 4096 characters. If not specified, the package file will be named the same as the Gallery Application name."]
    #[serde(rename = "packageFileName", default, skip_serializing_if = "Option::is_none")]
    pub package_file_name: Option<String>,
    #[doc = "Optional. The name to assign the downloaded config file on the VM. This is limited to 4096 characters. If not specified, the config file will be named the Gallery Application name appended with \"_config\"."]
    #[serde(rename = "configFileName", default, skip_serializing_if = "Option::is_none")]
    pub config_file_name: Option<String>,
}
impl UserArtifactSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The source image from which the Image Version is going to be created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserArtifactSource {
    #[doc = "Required. The mediaLink of the artifact, must be a readable storage page blob."]
    #[serde(rename = "mediaLink")]
    pub media_link: String,
    #[doc = "Optional. The defaultConfigurationLink of the artifact, must be a readable storage page blob."]
    #[serde(rename = "defaultConfigurationLink", default, skip_serializing_if = "Option::is_none")]
    pub default_configuration_link: Option<String>,
}
impl UserArtifactSource {
    pub fn new(media_link: String) -> Self {
        Self {
            media_link,
            default_configuration_link: None,
        }
    }
}
#[doc = "The list of user identities associated with the Virtual Machine. The user identity dictionary key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the security profile settings for the managed disk. <br><br> NOTE: It can only be set for Confidential VMs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmDiskSecurityProfile {
    #[doc = "Specifies the EncryptionType of the managed disk. <br> It is set to DiskWithVMGuestState for encryption of the managed disk along with VMGuestState blob, and VMGuestStateOnly for encryption of just the VMGuestState blob. <br><br> NOTE: It can be set for only Confidential VMs."]
    #[serde(rename = "securityEncryptionType", default, skip_serializing_if = "Option::is_none")]
    pub security_encryption_type: Option<vm_disk_security_profile::SecurityEncryptionType>,
    #[doc = "Describes the parameter of customer managed disk encryption set resource id that can be specified for disk. <br><br> NOTE: The disk encryption set resource id can only be specified for managed disk. Please refer https://aka.ms/mdssewithcmkoverview for more details."]
    #[serde(rename = "diskEncryptionSet", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set: Option<DiskEncryptionSetParameters>,
}
impl VmDiskSecurityProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vm_disk_security_profile {
    use super::*;
    #[doc = "Specifies the EncryptionType of the managed disk. <br> It is set to DiskWithVMGuestState for encryption of the managed disk along with VMGuestState blob, and VMGuestStateOnly for encryption of just the VMGuestState blob. <br><br> NOTE: It can be set for only Confidential VMs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SecurityEncryptionType")]
    pub enum SecurityEncryptionType {
        #[serde(rename = "VMGuestStateOnly")]
        VmGuestStateOnly,
        #[serde(rename = "DiskWithVMGuestState")]
        DiskWithVmGuestState,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SecurityEncryptionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SecurityEncryptionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SecurityEncryptionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::VmGuestStateOnly => serializer.serialize_unit_variant("SecurityEncryptionType", 0u32, "VMGuestStateOnly"),
                Self::DiskWithVmGuestState => serializer.serialize_unit_variant("SecurityEncryptionType", 1u32, "DiskWithVMGuestState"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the required information to reference a compute gallery application version"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmGalleryApplication {
    #[doc = "Optional, Specifies a passthrough value for more generic context."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[doc = "Optional, Specifies the order in which the packages have to be installed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "Specifies the GalleryApplicationVersion resource id on the form of /subscriptions/{SubscriptionId}/resourceGroups/{ResourceGroupName}/providers/Microsoft.Compute/galleries/{galleryName}/applications/{application}/versions/{version}"]
    #[serde(rename = "packageReferenceId")]
    pub package_reference_id: String,
    #[doc = "Optional, Specifies the uri to an azure blob that will replace the default configuration for the package if provided"]
    #[serde(rename = "configurationReference", default, skip_serializing_if = "Option::is_none")]
    pub configuration_reference: Option<String>,
    #[doc = "Optional, If true, any failure for any operation in the VmApplication will fail the deployment"]
    #[serde(rename = "treatFailureAsDeploymentFailure", default, skip_serializing_if = "Option::is_none")]
    pub treat_failure_as_deployment_failure: Option<bool>,
    #[doc = "If set to true, when a new Gallery Application version is available in PIR/SIG, it will be automatically updated for the VM/VMSS"]
    #[serde(rename = "enableAutomaticUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_upgrade: Option<bool>,
}
impl VmGalleryApplication {
    pub fn new(package_reference_id: String) -> Self {
        Self {
            tags: None,
            order: None,
            package_reference_id,
            configuration_reference: None,
            treat_failure_as_deployment_failure: None,
            enable_automatic_upgrade: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmScaleSetConvertToSinglePlacementGroupInput {
    #[doc = "Id of the placement group in which you want future virtual machine instances to be placed. To query placement group Id, please use Virtual Machine Scale Set VMs - Get API. If not provided, the platform will choose one with maximum number of virtual machine instances."]
    #[serde(rename = "activePlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub active_placement_group_id: Option<String>,
}
impl VmScaleSetConvertToSinglePlacementGroupInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies VM Size Property settings on the virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmSizeProperties {
    #[doc = "Specifies the number of vCPUs available for the VM. <br><br> When this property is not specified in the request body the default behavior is to set it to the value of vCPUs available for that VM size exposed in api response of [List all available virtual machine sizes in a region](https://docs.microsoft.com/en-us/rest/api/compute/resource-skus/list) ."]
    #[serde(rename = "vCPUsAvailable", default, skip_serializing_if = "Option::is_none")]
    pub v_cp_us_available: Option<i32>,
    #[doc = "Specifies the vCPU to physical core ratio. <br><br> When this property is not specified in the request body the default behavior is set to the value of vCPUsPerCore for the VM Size exposed in api response of [List all available virtual machine sizes in a region](https://docs.microsoft.com/en-us/rest/api/compute/resource-skus/list) <br><br> Setting this property to 1 also means that hyper-threading is disabled."]
    #[serde(rename = "vCPUsPerCore", default, skip_serializing_if = "Option::is_none")]
    pub v_cp_us_per_core: Option<i32>,
}
impl VmSizeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a single certificate reference in a Key Vault, and where the certificate should reside on the VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultCertificate {
    #[doc = "This is the URL of a certificate that has been uploaded to Key Vault as a secret. For adding a secret to the Key Vault, see [Add a key or secret to the key vault](https://docs.microsoft.com/azure/key-vault/key-vault-get-started/#add). In this case, your certificate needs to be It is the Base64 encoding of the following JSON Object which is encoded in UTF-8: <br><br> {<br>  \"data\":\"<Base64-encoded-certificate>\",<br>  \"dataType\":\"pfx\",<br>  \"password\":\"<pfx-file-password>\"<br>} <br> To install certificates on a virtual machine it is recommended to use the [Azure Key Vault virtual machine extension for Linux](https://docs.microsoft.com/azure/virtual-machines/extensions/key-vault-linux) or the [Azure Key Vault virtual machine extension for Windows](https://docs.microsoft.com/azure/virtual-machines/extensions/key-vault-windows)."]
    #[serde(rename = "certificateUrl", default, skip_serializing_if = "Option::is_none")]
    pub certificate_url: Option<String>,
    #[doc = "For Windows VMs, specifies the certificate store on the Virtual Machine to which the certificate should be added. The specified certificate store is implicitly in the LocalMachine account. <br><br>For Linux VMs, the certificate file is placed under the /var/lib/waagent directory, with the file name &lt;UppercaseThumbprint&gt;.crt for the X509 certificate file and &lt;UppercaseThumbprint&gt;.prv for private key. Both of these files are .pem formatted."]
    #[serde(rename = "certificateStore", default, skip_serializing_if = "Option::is_none")]
    pub certificate_store: Option<String>,
}
impl VaultCertificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a set of certificates which are all in the same Key Vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultSecretGroup {
    #[serde(rename = "sourceVault", default, skip_serializing_if = "Option::is_none")]
    pub source_vault: Option<SubResource>,
    #[doc = "The list of key vault references in SourceVault which contain certificates."]
    #[serde(rename = "vaultCertificates", default, skip_serializing_if = "Vec::is_empty")]
    pub vault_certificates: Vec<VaultCertificate>,
}
impl VaultSecretGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the uri of a disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualHardDisk {
    #[doc = "Specifies the virtual hard disk's uri."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl VirtualHardDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Virtual Machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachine {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Specifies information about the marketplace image used to create the virtual machine. This element is only used for marketplace images. Before you can use a marketplace image from an API, you must enable the image for programmatic use.  In the Azure portal, find the marketplace image that you want to use and then click **Want to deploy programmatically, Get Started ->**. Enter any required information and then click **Save**."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[doc = "Describes the properties of a Virtual Machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineProperties>,
    #[doc = "The virtual machine child extension resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<VirtualMachineExtension>,
    #[doc = "Identity for the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<VirtualMachineIdentity>,
    #[doc = "The virtual machine zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
}
impl VirtualMachine {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            plan: None,
            properties: None,
            resources: Vec::new(),
            identity: None,
            zones: Vec::new(),
            extended_location: None,
        }
    }
}
#[doc = "The instance view of the VM Agent running on the virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineAgentInstanceView {
    #[doc = "The VM Agent full version."]
    #[serde(rename = "vmAgentVersion", default, skip_serializing_if = "Option::is_none")]
    pub vm_agent_version: Option<String>,
    #[doc = "The virtual machine extension handler instance view."]
    #[serde(rename = "extensionHandlers", default, skip_serializing_if = "Vec::is_empty")]
    pub extension_handlers: Vec<VirtualMachineExtensionHandlerInstanceView>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<InstanceViewStatus>,
}
impl VirtualMachineAgentInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of an AssessPatches result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineAssessPatchesResult {
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Unknown\", \"Failed\", \"Succeeded\", or \"CompletedWithWarnings.\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<virtual_machine_assess_patches_result::Status>,
    #[doc = "The activity ID of the operation that produced this result. It is used to correlate across CRP and extension logs."]
    #[serde(rename = "assessmentActivityId", default, skip_serializing_if = "Option::is_none")]
    pub assessment_activity_id: Option<String>,
    #[doc = "The overall reboot status of the VM. It will be true when partially installed patches require a reboot to complete installation but the reboot has not yet occurred."]
    #[serde(rename = "rebootPending", default, skip_serializing_if = "Option::is_none")]
    pub reboot_pending: Option<bool>,
    #[doc = "The number of critical or security patches that have been detected as available and not yet installed."]
    #[serde(rename = "criticalAndSecurityPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub critical_and_security_patch_count: Option<i32>,
    #[doc = "The number of all available patches excluding critical and security."]
    #[serde(rename = "otherPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub other_patch_count: Option<i32>,
    #[doc = "The UTC timestamp when the operation began."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "The list of patches that have been detected as available for installation."]
    #[serde(rename = "availablePatches", default, skip_serializing_if = "Vec::is_empty")]
    pub available_patches: Vec<VirtualMachineSoftwarePatchProperties>,
    #[doc = "Api error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}
impl VirtualMachineAssessPatchesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_assess_patches_result {
    use super::*;
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Unknown\", \"Failed\", \"Succeeded\", or \"CompletedWithWarnings.\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Unknown,
        InProgress,
        Failed,
        Succeeded,
        CompletedWithWarnings,
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
                Self::Unknown => serializer.serialize_unit_variant("Status", 0u32, "Unknown"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 3u32, "Succeeded"),
                Self::CompletedWithWarnings => serializer.serialize_unit_variant("Status", 4u32, "CompletedWithWarnings"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Capture Virtual Machine parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineCaptureParameters {
    #[doc = "The captured virtual hard disk's name prefix."]
    #[serde(rename = "vhdPrefix")]
    pub vhd_prefix: String,
    #[doc = "The destination container name."]
    #[serde(rename = "destinationContainerName")]
    pub destination_container_name: String,
    #[doc = "Specifies whether to overwrite the destination virtual hard disk, in case of conflict."]
    #[serde(rename = "overwriteVhds")]
    pub overwrite_vhds: bool,
}
impl VirtualMachineCaptureParameters {
    pub fn new(vhd_prefix: String, destination_container_name: String, overwrite_vhds: bool) -> Self {
        Self {
            vhd_prefix,
            destination_container_name,
            overwrite_vhds,
        }
    }
}
#[doc = "Output of virtual machine capture operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineCaptureResult {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "the schema of the captured virtual machine"]
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[doc = "the version of the content"]
    #[serde(rename = "contentVersion", default, skip_serializing_if = "Option::is_none")]
    pub content_version: Option<String>,
    #[doc = "parameters of the captured virtual machine"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "a list of resource items of the captured virtual machine"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<serde_json::Value>,
}
impl VirtualMachineCaptureResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Virtual Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineExtension {
    #[serde(flatten)]
    pub resource_with_optional_location: ResourceWithOptionalLocation,
    #[doc = "Describes the properties of a Virtual Machine Extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineExtensionProperties>,
}
impl VirtualMachineExtension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The instance view of a virtual machine extension handler."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineExtensionHandlerInstanceView {
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Instance view status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceViewStatus>,
}
impl VirtualMachineExtensionHandlerInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Virtual Machine Extension Image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineExtensionImage {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of a Virtual Machine Extension Image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineExtensionImageProperties>,
}
impl VirtualMachineExtensionImage {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "Describes the properties of a Virtual Machine Extension Image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineExtensionImageProperties {
    #[doc = "The operating system this extension supports."]
    #[serde(rename = "operatingSystem")]
    pub operating_system: String,
    #[doc = "The type of role (IaaS or PaaS) this extension supports."]
    #[serde(rename = "computeRole")]
    pub compute_role: String,
    #[doc = "The schema defined by publisher, where extension consumers should provide settings in a matching schema."]
    #[serde(rename = "handlerSchema")]
    pub handler_schema: String,
    #[doc = "Whether the extension can be used on xRP VMScaleSets. By default existing extensions are usable on scalesets, but there might be cases where a publisher wants to explicitly indicate the extension is only enabled for CRP VMs but not VMSS."]
    #[serde(rename = "vmScaleSetEnabled", default, skip_serializing_if = "Option::is_none")]
    pub vm_scale_set_enabled: Option<bool>,
    #[doc = "Whether the handler can support multiple extensions."]
    #[serde(rename = "supportsMultipleExtensions", default, skip_serializing_if = "Option::is_none")]
    pub supports_multiple_extensions: Option<bool>,
}
impl VirtualMachineExtensionImageProperties {
    pub fn new(operating_system: String, compute_role: String, handler_schema: String) -> Self {
        Self {
            operating_system,
            compute_role,
            handler_schema,
            vm_scale_set_enabled: None,
            supports_multiple_extensions: None,
        }
    }
}
#[doc = "The instance view of a virtual machine extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineExtensionInstanceView {
    #[doc = "The virtual machine extension name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub substatuses: Vec<InstanceViewStatus>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<InstanceViewStatus>,
}
impl VirtualMachineExtensionInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Virtual Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineExtensionProperties {
    #[doc = "How the extension handler should be forced to update even if the extension configuration has not changed."]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[doc = "The name of the extension handler publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true."]
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[doc = "Indicates whether the extension should be automatically upgraded by the platform if there is a newer version of the extension available."]
    #[serde(rename = "enableAutomaticUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_upgrade: Option<bool>,
    #[doc = "Json formatted public settings for the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[doc = "The extension can contain either protectedSettings or protectedSettingsFromKeyVault or no protected settings at all."]
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The instance view of a virtual machine extension."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<VirtualMachineExtensionInstanceView>,
    #[doc = "Indicates whether failures stemming from the extension will be suppressed (Operational failures such as not connecting to the VM will not be suppressed regardless of this value). The default is false."]
    #[serde(rename = "suppressFailures", default, skip_serializing_if = "Option::is_none")]
    pub suppress_failures: Option<bool>,
    #[doc = "The extensions protected settings that are passed by reference, and consumed from key vault"]
    #[serde(rename = "protectedSettingsFromKeyVault", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings_from_key_vault: Option<serde_json::Value>,
}
impl VirtualMachineExtensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Virtual Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineExtensionUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "Describes the properties of a Virtual Machine Extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineExtensionUpdateProperties>,
}
impl VirtualMachineExtensionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Virtual Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineExtensionUpdateProperties {
    #[doc = "How the extension handler should be forced to update even if the extension configuration has not changed."]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[doc = "The name of the extension handler publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true."]
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[doc = "Indicates whether the extension should be automatically upgraded by the platform if there is a newer version of the extension available."]
    #[serde(rename = "enableAutomaticUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_upgrade: Option<bool>,
    #[doc = "Json formatted public settings for the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[doc = "The extension can contain either protectedSettings or protectedSettingsFromKeyVault or no protected settings at all."]
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
    #[doc = "Indicates whether failures stemming from the extension will be suppressed (Operational failures such as not connecting to the VM will not be suppressed regardless of this value). The default is false."]
    #[serde(rename = "suppressFailures", default, skip_serializing_if = "Option::is_none")]
    pub suppress_failures: Option<bool>,
    #[doc = "The extensions protected settings that are passed by reference, and consumed from key vault"]
    #[serde(rename = "protectedSettingsFromKeyVault", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings_from_key_vault: Option<serde_json::Value>,
}
impl VirtualMachineExtensionUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Extension operation response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineExtensionsListResult {
    #[doc = "The list of extensions"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualMachineExtension>,
}
impl VirtualMachineExtensionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The health status of the VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineHealthStatus {
    #[doc = "Instance view status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceViewStatus>,
}
impl VirtualMachineHealthStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineIdentity {
    #[doc = "The principal id of virtual machine identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id associated with the virtual machine. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of identity used for the virtual machine. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the virtual machine."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<virtual_machine_identity::Type>,
    #[doc = "The list of user identities associated with the Virtual Machine. The user identity dictionary key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl VirtualMachineIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_identity {
    use super::*;
    #[doc = "The type of identity used for the virtual machine. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the virtual machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[doc = "Describes a Virtual Machine Image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineImage {
    #[serde(flatten)]
    pub virtual_machine_image_resource: VirtualMachineImageResource,
    #[doc = "Describes the properties of a Virtual Machine Image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineImageProperties>,
}
impl VirtualMachineImage {
    pub fn new(virtual_machine_image_resource: VirtualMachineImageResource) -> Self {
        Self {
            virtual_machine_image_resource,
            properties: None,
        }
    }
}
#[doc = "Specifies additional capabilities supported by the image"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineImageFeature {
    #[doc = "The name of the feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The corresponding value for the feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl VirtualMachineImageFeature {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Virtual Machine Image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineImageProperties {
    #[doc = "Used for establishing the purchase context of any 3rd Party artifact through MarketPlace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<PurchasePlan>,
    #[doc = "Contains the os disk image information."]
    #[serde(rename = "osDiskImage", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_image: Option<OsDiskImage>,
    #[serde(rename = "dataDiskImages", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disk_images: Vec<DataDiskImage>,
    #[doc = "Describes automatic OS upgrade properties on the image."]
    #[serde(rename = "automaticOSUpgradeProperties", default, skip_serializing_if = "Option::is_none")]
    pub automatic_os_upgrade_properties: Option<AutomaticOsUpgradeProperties>,
    #[doc = "Specifies the HyperVGeneration Type"]
    #[serde(rename = "hyperVGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<HyperVGenerationType>,
    #[doc = "Specifies the disallowed configuration for a virtual machine image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disallowed: Option<DisallowedConfiguration>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<VirtualMachineImageFeature>,
    #[doc = "Specifies the Architecture Type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<ArchitectureType>,
}
impl VirtualMachineImageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual machine image resource information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineImageResource {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The name of the resource."]
    pub name: String,
    #[doc = "The supported Azure location of the resource."]
    pub location: String,
    #[doc = "Specifies the tags that are assigned to the virtual machine. For more information about using tags, see [Using tags to organize your Azure resources](https://docs.microsoft.com/azure/azure-resource-manager/resource-group-using-tags.md)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
}
impl VirtualMachineImageResource {
    pub fn new(name: String, location: String) -> Self {
        Self {
            sub_resource: SubResource::default(),
            name,
            location,
            tags: None,
            extended_location: None,
        }
    }
}
#[doc = "Input for InstallPatches as directly received by the API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineInstallPatchesParameters {
    #[doc = "Specifies the maximum amount of time that the operation will run. It must be an ISO 8601-compliant duration string such as PT4H (4 hours)"]
    #[serde(rename = "maximumDuration", default, skip_serializing_if = "Option::is_none")]
    pub maximum_duration: Option<String>,
    #[doc = "Defines when it is acceptable to reboot a VM during a software update operation."]
    #[serde(rename = "rebootSetting")]
    pub reboot_setting: virtual_machine_install_patches_parameters::RebootSetting,
    #[doc = "Input for InstallPatches on a Windows VM, as directly received by the API"]
    #[serde(rename = "windowsParameters", default, skip_serializing_if = "Option::is_none")]
    pub windows_parameters: Option<WindowsParameters>,
    #[doc = "Input for InstallPatches on a Linux VM, as directly received by the API"]
    #[serde(rename = "linuxParameters", default, skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,
}
impl VirtualMachineInstallPatchesParameters {
    pub fn new(reboot_setting: virtual_machine_install_patches_parameters::RebootSetting) -> Self {
        Self {
            maximum_duration: None,
            reboot_setting,
            windows_parameters: None,
            linux_parameters: None,
        }
    }
}
pub mod virtual_machine_install_patches_parameters {
    use super::*;
    #[doc = "Defines when it is acceptable to reboot a VM during a software update operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RebootSetting")]
    pub enum RebootSetting {
        IfRequired,
        Never,
        Always,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RebootSetting {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RebootSetting {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RebootSetting {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IfRequired => serializer.serialize_unit_variant("RebootSetting", 0u32, "IfRequired"),
                Self::Never => serializer.serialize_unit_variant("RebootSetting", 1u32, "Never"),
                Self::Always => serializer.serialize_unit_variant("RebootSetting", 2u32, "Always"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The result summary of an installation operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineInstallPatchesResult {
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Failed\", \"Succeeded\", \"Unknown\" or \"CompletedWithWarnings.\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<virtual_machine_install_patches_result::Status>,
    #[doc = "The activity ID of the operation that produced this result. It is used to correlate across CRP and extension logs."]
    #[serde(rename = "installationActivityId", default, skip_serializing_if = "Option::is_none")]
    pub installation_activity_id: Option<String>,
    #[doc = "The reboot state of the VM following completion of the operation."]
    #[serde(rename = "rebootStatus", default, skip_serializing_if = "Option::is_none")]
    pub reboot_status: Option<virtual_machine_install_patches_result::RebootStatus>,
    #[doc = "Whether the operation ran out of time before it completed all its intended actions."]
    #[serde(rename = "maintenanceWindowExceeded", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window_exceeded: Option<bool>,
    #[doc = "The number of patches that were not installed due to the user blocking their installation."]
    #[serde(rename = "excludedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub excluded_patch_count: Option<i32>,
    #[doc = "The number of patches that were detected as available for install, but did not meet the operation's criteria."]
    #[serde(rename = "notSelectedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub not_selected_patch_count: Option<i32>,
    #[doc = "The number of patches that were identified as meeting the installation criteria, but were not able to be installed. Typically this happens when maintenanceWindowExceeded == true."]
    #[serde(rename = "pendingPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub pending_patch_count: Option<i32>,
    #[doc = "The number of patches successfully installed."]
    #[serde(rename = "installedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub installed_patch_count: Option<i32>,
    #[doc = "The number of patches that could not be installed due to some issue. See errors for details."]
    #[serde(rename = "failedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_patch_count: Option<i32>,
    #[doc = "The patches that were installed during the operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patches: Vec<PatchInstallationDetail>,
    #[doc = "The UTC timestamp when the operation began."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "Api error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}
impl VirtualMachineInstallPatchesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_install_patches_result {
    use super::*;
    #[doc = "The overall success or failure status of the operation. It remains \"InProgress\" until the operation completes. At that point it will become \"Failed\", \"Succeeded\", \"Unknown\" or \"CompletedWithWarnings.\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Unknown,
        InProgress,
        Failed,
        Succeeded,
        CompletedWithWarnings,
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
                Self::Unknown => serializer.serialize_unit_variant("Status", 0u32, "Unknown"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 3u32, "Succeeded"),
                Self::CompletedWithWarnings => serializer.serialize_unit_variant("Status", 4u32, "CompletedWithWarnings"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The reboot state of the VM following completion of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RebootStatus")]
    pub enum RebootStatus {
        Unknown,
        NotNeeded,
        Required,
        Started,
        Failed,
        Completed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RebootStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RebootStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RebootStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("RebootStatus", 0u32, "Unknown"),
                Self::NotNeeded => serializer.serialize_unit_variant("RebootStatus", 1u32, "NotNeeded"),
                Self::Required => serializer.serialize_unit_variant("RebootStatus", 2u32, "Required"),
                Self::Started => serializer.serialize_unit_variant("RebootStatus", 3u32, "Started"),
                Self::Failed => serializer.serialize_unit_variant("RebootStatus", 4u32, "Failed"),
                Self::Completed => serializer.serialize_unit_variant("RebootStatus", 5u32, "Completed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The instance view of a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineInstanceView {
    #[doc = "Specifies the update domain of the virtual machine."]
    #[serde(rename = "platformUpdateDomain", default, skip_serializing_if = "Option::is_none")]
    pub platform_update_domain: Option<i32>,
    #[doc = "Specifies the fault domain of the virtual machine."]
    #[serde(rename = "platformFaultDomain", default, skip_serializing_if = "Option::is_none")]
    pub platform_fault_domain: Option<i32>,
    #[doc = "The computer name assigned to the virtual machine."]
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[doc = "The Operating System running on the virtual machine."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "The version of Operating System running on the virtual machine."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "Specifies the HyperVGeneration Type associated with a resource"]
    #[serde(rename = "hyperVGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_generation: Option<virtual_machine_instance_view::HyperVGeneration>,
    #[doc = "The Remote desktop certificate thumbprint."]
    #[serde(rename = "rdpThumbPrint", default, skip_serializing_if = "Option::is_none")]
    pub rdp_thumb_print: Option<String>,
    #[doc = "The instance view of the VM Agent running on the virtual machine."]
    #[serde(rename = "vmAgent", default, skip_serializing_if = "Option::is_none")]
    pub vm_agent: Option<VirtualMachineAgentInstanceView>,
    #[doc = "Maintenance Operation Status."]
    #[serde(rename = "maintenanceRedeployStatus", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_redeploy_status: Option<MaintenanceRedeployStatus>,
    #[doc = "The virtual machine disk information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<DiskInstanceView>,
    #[doc = "The extensions information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<VirtualMachineExtensionInstanceView>,
    #[doc = "The health status of the VM."]
    #[serde(rename = "vmHealth", default, skip_serializing_if = "Option::is_none")]
    pub vm_health: Option<VirtualMachineHealthStatus>,
    #[doc = "The instance view of a virtual machine boot diagnostics."]
    #[serde(rename = "bootDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub boot_diagnostics: Option<BootDiagnosticsInstanceView>,
    #[doc = "Resource id of the dedicated host, on which the virtual machine is allocated through automatic placement, when the virtual machine is associated with a dedicated host group that has automatic placement enabled. <br><br>Minimum api-version: 2020-06-01."]
    #[serde(rename = "assignedHost", default, skip_serializing_if = "Option::is_none")]
    pub assigned_host: Option<String>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<InstanceViewStatus>,
    #[doc = "The status of virtual machine patch operations."]
    #[serde(rename = "patchStatus", default, skip_serializing_if = "Option::is_none")]
    pub patch_status: Option<VirtualMachinePatchStatus>,
}
impl VirtualMachineInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_instance_view {
    use super::*;
    #[doc = "Specifies the HyperVGeneration Type associated with a resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HyperVGeneration")]
    pub enum HyperVGeneration {
        V1,
        V2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HyperVGeneration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HyperVGeneration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HyperVGeneration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::V1 => serializer.serialize_unit_variant("HyperVGeneration", 0u32, "V1"),
                Self::V2 => serializer.serialize_unit_variant("HyperVGeneration", 1u32, "V2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Contains the IP tag associated with the public IP address."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineIpTag {
    #[doc = "IP tag type. Example: FirstPartyUsage."]
    #[serde(rename = "ipTagType", default, skip_serializing_if = "Option::is_none")]
    pub ip_tag_type: Option<String>,
    #[doc = "IP tag associated with the public IP. Example: SQL, Storage etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}
impl VirtualMachineIpTag {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Virtual Machine operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineListResult {
    #[doc = "The list of virtual machines."]
    pub value: Vec<VirtualMachine>,
    #[doc = "The URI to fetch the next page of VMs. Call ListNext() with this URI to fetch the next page of Virtual Machines."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualMachineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualMachineListResult {
    pub fn new(value: Vec<VirtualMachine>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes a virtual machine network interface configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineNetworkInterfaceConfiguration {
    #[doc = "The network interface configuration name."]
    pub name: String,
    #[doc = "Describes a virtual machine network profile's IP configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineNetworkInterfaceConfigurationProperties>,
}
impl VirtualMachineNetworkInterfaceConfiguration {
    pub fn new(name: String) -> Self {
        Self { name, properties: None }
    }
}
#[doc = "Describes a virtual machine network profile's IP configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineNetworkInterfaceConfigurationProperties {
    #[doc = "Specifies the primary network interface in case the virtual machine has more than 1 network interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[doc = "Specify what happens to the network interface when the VM is deleted"]
    #[serde(rename = "deleteOption", default, skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<virtual_machine_network_interface_configuration_properties::DeleteOption>,
    #[doc = "Specifies whether the network interface is accelerated networking-enabled."]
    #[serde(rename = "enableAcceleratedNetworking", default, skip_serializing_if = "Option::is_none")]
    pub enable_accelerated_networking: Option<bool>,
    #[doc = "Specifies whether the network interface is FPGA networking-enabled."]
    #[serde(rename = "enableFpga", default, skip_serializing_if = "Option::is_none")]
    pub enable_fpga: Option<bool>,
    #[doc = "Whether IP forwarding enabled on this NIC."]
    #[serde(rename = "enableIPForwarding", default, skip_serializing_if = "Option::is_none")]
    pub enable_ip_forwarding: Option<bool>,
    #[serde(rename = "networkSecurityGroup", default, skip_serializing_if = "Option::is_none")]
    pub network_security_group: Option<SubResource>,
    #[doc = "Describes a virtual machines network configuration's DNS settings."]
    #[serde(rename = "dnsSettings", default, skip_serializing_if = "Option::is_none")]
    pub dns_settings: Option<VirtualMachineNetworkInterfaceDnsSettingsConfiguration>,
    #[doc = "Specifies the IP configurations of the network interface."]
    #[serde(rename = "ipConfigurations")]
    pub ip_configurations: Vec<VirtualMachineNetworkInterfaceIpConfiguration>,
    #[serde(rename = "dscpConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub dscp_configuration: Option<SubResource>,
}
impl VirtualMachineNetworkInterfaceConfigurationProperties {
    pub fn new(ip_configurations: Vec<VirtualMachineNetworkInterfaceIpConfiguration>) -> Self {
        Self {
            primary: None,
            delete_option: None,
            enable_accelerated_networking: None,
            enable_fpga: None,
            enable_ip_forwarding: None,
            network_security_group: None,
            dns_settings: None,
            ip_configurations,
            dscp_configuration: None,
        }
    }
}
pub mod virtual_machine_network_interface_configuration_properties {
    use super::*;
    #[doc = "Specify what happens to the network interface when the VM is deleted"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeleteOption")]
    pub enum DeleteOption {
        Delete,
        Detach,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeleteOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeleteOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeleteOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Delete => serializer.serialize_unit_variant("DeleteOption", 0u32, "Delete"),
                Self::Detach => serializer.serialize_unit_variant("DeleteOption", 1u32, "Detach"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a virtual machines network configuration's DNS settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineNetworkInterfaceDnsSettingsConfiguration {
    #[doc = "List of DNS servers IP addresses"]
    #[serde(rename = "dnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<String>,
}
impl VirtualMachineNetworkInterfaceDnsSettingsConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual machine network profile's IP configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineNetworkInterfaceIpConfiguration {
    #[doc = "The IP configuration name."]
    pub name: String,
    #[doc = "Describes a virtual machine network interface IP configuration properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineNetworkInterfaceIpConfigurationProperties>,
}
impl VirtualMachineNetworkInterfaceIpConfiguration {
    pub fn new(name: String) -> Self {
        Self { name, properties: None }
    }
}
#[doc = "Describes a virtual machine network interface IP configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineNetworkInterfaceIpConfigurationProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<SubResource>,
    #[doc = "Specifies the primary network interface in case the virtual machine has more than 1 network interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[doc = "Describes a virtual machines IP Configuration's PublicIPAddress configuration"]
    #[serde(rename = "publicIPAddressConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address_configuration: Option<VirtualMachinePublicIpAddressConfiguration>,
    #[doc = "Available from Api-Version 2017-03-30 onwards, it represents whether the specific ipconfiguration is IPv4 or IPv6. Default is taken as IPv4.  Possible values are: 'IPv4' and 'IPv6'."]
    #[serde(rename = "privateIPAddressVersion", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address_version: Option<virtual_machine_network_interface_ip_configuration_properties::PrivateIpAddressVersion>,
    #[doc = "Specifies an array of references to application security group."]
    #[serde(rename = "applicationSecurityGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub application_security_groups: Vec<SubResource>,
    #[doc = "Specifies an array of references to backend address pools of application gateways. A virtual machine can reference backend address pools of multiple application gateways. Multiple virtual machines cannot use the same application gateway."]
    #[serde(rename = "applicationGatewayBackendAddressPools", default, skip_serializing_if = "Vec::is_empty")]
    pub application_gateway_backend_address_pools: Vec<SubResource>,
    #[doc = "Specifies an array of references to backend address pools of load balancers. A virtual machine can reference backend address pools of one public and one internal load balancer. [Multiple virtual machines cannot use the same basic sku load balancer]."]
    #[serde(rename = "loadBalancerBackendAddressPools", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_backend_address_pools: Vec<SubResource>,
}
impl VirtualMachineNetworkInterfaceIpConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_network_interface_ip_configuration_properties {
    use super::*;
    #[doc = "Available from Api-Version 2017-03-30 onwards, it represents whether the specific ipconfiguration is IPv4 or IPv6. Default is taken as IPv4.  Possible values are: 'IPv4' and 'IPv6'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivateIpAddressVersion")]
    pub enum PrivateIpAddressVersion {
        IPv4,
        IPv6,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivateIpAddressVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivateIpAddressVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivateIpAddressVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IPv4 => serializer.serialize_unit_variant("PrivateIpAddressVersion", 0u32, "IPv4"),
                Self::IPv6 => serializer.serialize_unit_variant("PrivateIpAddressVersion", 1u32, "IPv6"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The status of virtual machine patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachinePatchStatus {
    #[doc = "Describes the properties of an virtual machine instance view for available patch summary."]
    #[serde(rename = "availablePatchSummary", default, skip_serializing_if = "Option::is_none")]
    pub available_patch_summary: Option<AvailablePatchSummary>,
    #[doc = "Describes the properties of the last installed patch summary."]
    #[serde(rename = "lastPatchInstallationSummary", default, skip_serializing_if = "Option::is_none")]
    pub last_patch_installation_summary: Option<LastPatchInstallationSummary>,
    #[doc = "The enablement status of the specified patchMode"]
    #[serde(rename = "configurationStatuses", default, skip_serializing_if = "Vec::is_empty")]
    pub configuration_statuses: Vec<InstanceViewStatus>,
}
impl VirtualMachinePatchStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Virtual Machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineProperties {
    #[doc = "Specifies the hardware settings for the virtual machine."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[doc = "Specifies the storage settings for the virtual machine disks."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "Enables or disables a capability on the virtual machine or virtual machine scale set."]
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub additional_capabilities: Option<AdditionalCapabilities>,
    #[doc = "Specifies the operating system settings for the virtual machine. Some of the settings cannot be changed once VM is provisioned."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "Specifies the network interfaces or the networking configuration of the virtual machine."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[doc = "Specifies the Security profile settings for the virtual machine or virtual machine scale set."]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<SecurityProfile>,
    #[doc = "Specifies the boot diagnostic settings state. <br><br>Minimum api-version: 2015-06-15."]
    #[serde(rename = "diagnosticsProfile", default, skip_serializing_if = "Option::is_none")]
    pub diagnostics_profile: Option<DiagnosticsProfile>,
    #[serde(rename = "availabilitySet", default, skip_serializing_if = "Option::is_none")]
    pub availability_set: Option<SubResource>,
    #[serde(rename = "virtualMachineScaleSet", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_scale_set: Option<SubResource>,
    #[serde(rename = "proximityPlacementGroup", default, skip_serializing_if = "Option::is_none")]
    pub proximity_placement_group: Option<SubResource>,
    #[doc = "Specifies the priority for a standalone virtual machine or the virtual machines in the scale set. <br><br> 'Low' enum will be deprecated in the future, please use 'Spot' as the enum to deploy Azure Spot VM/VMSS."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[doc = "Specifies the eviction policy for the Azure Spot VM/VMSS"]
    #[serde(rename = "evictionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub eviction_policy: Option<EvictionPolicy>,
    #[doc = "Specifies the billing related details of a Azure Spot VM or VMSS. <br><br>Minimum api-version: 2019-03-01."]
    #[serde(rename = "billingProfile", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile: Option<BillingProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<SubResource>,
    #[serde(rename = "hostGroup", default, skip_serializing_if = "Option::is_none")]
    pub host_group: Option<SubResource>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The instance view of a virtual machine."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<VirtualMachineInstanceView>,
    #[doc = "Specifies that the image or disk that is being used was licensed on-premises. <br><br> Possible values for Windows Server operating system are: <br><br> Windows_Client <br><br> Windows_Server <br><br> Possible values for Linux Server operating system are: <br><br> RHEL_BYOS (for RHEL) <br><br> SLES_BYOS (for SUSE) <br><br> For more information, see [Azure Hybrid Use Benefit for Windows Server](https://docs.microsoft.com/azure/virtual-machines/windows/hybrid-use-benefit-licensing) <br><br> [Azure Hybrid Use Benefit for Linux Server](https://docs.microsoft.com/azure/virtual-machines/linux/azure-hybrid-benefit-linux) <br><br> Minimum api-version: 2015-06-15"]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[doc = "Specifies the VM unique ID which is a 128-bits identifier that is encoded and stored in all Azure IaaS VMs SMBIOS and can be read using platform BIOS commands."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "Specifies the time alloted for all extensions to start. The time duration should be between 15 minutes and 120 minutes (inclusive) and should be specified in ISO 8601 format. The default value is 90 minutes (PT1H30M). <br><br> Minimum api-version: 2020-06-01"]
    #[serde(rename = "extensionsTimeBudget", default, skip_serializing_if = "Option::is_none")]
    pub extensions_time_budget: Option<String>,
    #[doc = "Specifies the scale set logical fault domain into which the Virtual Machine will be created. By default, the Virtual Machine will by automatically assigned to a fault domain that best maintains balance across available fault domains.<br><li>This is applicable only if the 'virtualMachineScaleSet' property of this Virtual Machine is set.<li>The Virtual Machine Scale Set that is referenced, must have 'platformFaultDomainCount' &gt; 1.<li>This property cannot be updated once the Virtual Machine is created.<li>Fault domain assignment can be viewed in the Virtual Machine Instance View.<br><br>Minimum api‐version: 2020‐12‐01"]
    #[serde(rename = "platformFaultDomain", default, skip_serializing_if = "Option::is_none")]
    pub platform_fault_domain: Option<i32>,
    #[serde(rename = "scheduledEventsProfile", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_events_profile: Option<ScheduledEventsProfile>,
    #[doc = "UserData for the VM, which must be base-64 encoded. Customer should not pass any secrets in here. <br><br>Minimum api-version: 2021-03-01"]
    #[serde(rename = "userData", default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[doc = "The parameters of a capacity reservation Profile."]
    #[serde(rename = "capacityReservation", default, skip_serializing_if = "Option::is_none")]
    pub capacity_reservation: Option<CapacityReservationProfile>,
    #[doc = "Contains the list of gallery applications that should be made available to the VM/VMSS"]
    #[serde(rename = "applicationProfile", default, skip_serializing_if = "Option::is_none")]
    pub application_profile: Option<ApplicationProfile>,
    #[doc = "Specifies the time at which the Virtual Machine resource was created.<br><br>Minimum api-version: 2022-03-01."]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<time::OffsetDateTime>,
}
impl VirtualMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual machines IP Configuration's PublicIPAddress configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachinePublicIpAddressConfiguration {
    #[doc = "The publicIP address configuration name."]
    pub name: String,
    #[doc = "Describes a virtual machines IP Configuration's PublicIPAddress configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachinePublicIpAddressConfigurationProperties>,
    #[doc = "Describes the public IP Sku. It can only be set with OrchestrationMode as Flexible."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PublicIpAddressSku>,
}
impl VirtualMachinePublicIpAddressConfiguration {
    pub fn new(name: String) -> Self {
        Self {
            name,
            properties: None,
            sku: None,
        }
    }
}
#[doc = "Describes a virtual machines IP Configuration's PublicIPAddress configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachinePublicIpAddressConfigurationProperties {
    #[doc = "The idle timeout of the public IP address."]
    #[serde(rename = "idleTimeoutInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_in_minutes: Option<i32>,
    #[doc = "Specify what happens to the public IP address when the VM is deleted"]
    #[serde(rename = "deleteOption", default, skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<virtual_machine_public_ip_address_configuration_properties::DeleteOption>,
    #[doc = "Describes a virtual machines network configuration's DNS settings."]
    #[serde(rename = "dnsSettings", default, skip_serializing_if = "Option::is_none")]
    pub dns_settings: Option<VirtualMachinePublicIpAddressDnsSettingsConfiguration>,
    #[doc = "The list of IP tags associated with the public IP address."]
    #[serde(rename = "ipTags", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_tags: Vec<VirtualMachineIpTag>,
    #[serde(rename = "publicIPPrefix", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_prefix: Option<SubResource>,
    #[doc = "Available from Api-Version 2019-07-01 onwards, it represents whether the specific ipconfiguration is IPv4 or IPv6. Default is taken as IPv4. Possible values are: 'IPv4' and 'IPv6'."]
    #[serde(rename = "publicIPAddressVersion", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address_version: Option<virtual_machine_public_ip_address_configuration_properties::PublicIpAddressVersion>,
    #[doc = "Specify the public IP allocation type"]
    #[serde(rename = "publicIPAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_allocation_method: Option<virtual_machine_public_ip_address_configuration_properties::PublicIpAllocationMethod>,
}
impl VirtualMachinePublicIpAddressConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_public_ip_address_configuration_properties {
    use super::*;
    #[doc = "Specify what happens to the public IP address when the VM is deleted"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeleteOption")]
    pub enum DeleteOption {
        Delete,
        Detach,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeleteOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeleteOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeleteOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Delete => serializer.serialize_unit_variant("DeleteOption", 0u32, "Delete"),
                Self::Detach => serializer.serialize_unit_variant("DeleteOption", 1u32, "Detach"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Available from Api-Version 2019-07-01 onwards, it represents whether the specific ipconfiguration is IPv4 or IPv6. Default is taken as IPv4. Possible values are: 'IPv4' and 'IPv6'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicIpAddressVersion")]
    pub enum PublicIpAddressVersion {
        IPv4,
        IPv6,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicIpAddressVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicIpAddressVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicIpAddressVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IPv4 => serializer.serialize_unit_variant("PublicIpAddressVersion", 0u32, "IPv4"),
                Self::IPv6 => serializer.serialize_unit_variant("PublicIpAddressVersion", 1u32, "IPv6"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specify the public IP allocation type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicIpAllocationMethod")]
    pub enum PublicIpAllocationMethod {
        Dynamic,
        Static,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicIpAllocationMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicIpAllocationMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicIpAllocationMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dynamic => serializer.serialize_unit_variant("PublicIpAllocationMethod", 0u32, "Dynamic"),
                Self::Static => serializer.serialize_unit_variant("PublicIpAllocationMethod", 1u32, "Static"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a virtual machines network configuration's DNS settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachinePublicIpAddressDnsSettingsConfiguration {
    #[doc = "The Domain name label prefix of the PublicIPAddress resources that will be created. The generated name label is the concatenation of the domain name label and vm network profile unique ID."]
    #[serde(rename = "domainNameLabel")]
    pub domain_name_label: String,
}
impl VirtualMachinePublicIpAddressDnsSettingsConfiguration {
    pub fn new(domain_name_label: String) -> Self {
        Self { domain_name_label }
    }
}
#[doc = "Parameters for Reimaging Virtual Machine. NOTE: Virtual Machine OS disk will always be reimaged"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineReimageParameters {
    #[doc = "Specifies whether to reimage temp disk. Default value: false. Note: This temp disk reimage parameter is only supported for VM/VMSS with Ephemeral OS disk."]
    #[serde(rename = "tempDisk", default, skip_serializing_if = "Option::is_none")]
    pub temp_disk: Option<bool>,
}
impl VirtualMachineReimageParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Virtual Machine run command."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineRunCommand {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of a Virtual Machine run command."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineRunCommandProperties>,
}
impl VirtualMachineRunCommand {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[doc = "The instance view of a virtual machine run command."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineRunCommandInstanceView {
    #[doc = "Script execution status."]
    #[serde(rename = "executionState", default, skip_serializing_if = "Option::is_none")]
    pub execution_state: Option<virtual_machine_run_command_instance_view::ExecutionState>,
    #[doc = "Communicate script configuration errors or execution messages."]
    #[serde(rename = "executionMessage", default, skip_serializing_if = "Option::is_none")]
    pub execution_message: Option<String>,
    #[doc = "Exit code returned from script execution."]
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[doc = "Script output stream."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[doc = "Script error stream."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "Script start time."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Script end time."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<InstanceViewStatus>,
}
impl VirtualMachineRunCommandInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_run_command_instance_view {
    use super::*;
    #[doc = "Script execution status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ExecutionState")]
    pub enum ExecutionState {
        Unknown,
        Pending,
        Running,
        Failed,
        Succeeded,
        TimedOut,
        Canceled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ExecutionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ExecutionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ExecutionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("ExecutionState", 0u32, "Unknown"),
                Self::Pending => serializer.serialize_unit_variant("ExecutionState", 1u32, "Pending"),
                Self::Running => serializer.serialize_unit_variant("ExecutionState", 2u32, "Running"),
                Self::Failed => serializer.serialize_unit_variant("ExecutionState", 3u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ExecutionState", 4u32, "Succeeded"),
                Self::TimedOut => serializer.serialize_unit_variant("ExecutionState", 5u32, "TimedOut"),
                Self::Canceled => serializer.serialize_unit_variant("ExecutionState", 6u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes the properties of a Virtual Machine run command."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineRunCommandProperties {
    #[doc = "Describes the script sources for run command."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<VirtualMachineRunCommandScriptSource>,
    #[doc = "The parameters used by the script."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<RunCommandInputParameter>,
    #[doc = "The parameters used by the script."]
    #[serde(rename = "protectedParameters", default, skip_serializing_if = "Vec::is_empty")]
    pub protected_parameters: Vec<RunCommandInputParameter>,
    #[doc = "Optional. If set to true, provisioning will complete as soon as the script starts and will not wait for script to complete."]
    #[serde(rename = "asyncExecution", default, skip_serializing_if = "Option::is_none")]
    pub async_execution: Option<bool>,
    #[doc = "Specifies the user account on the VM when executing the run command."]
    #[serde(rename = "runAsUser", default, skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<String>,
    #[doc = "Specifies the user account password on the VM when executing the run command."]
    #[serde(rename = "runAsPassword", default, skip_serializing_if = "Option::is_none")]
    pub run_as_password: Option<String>,
    #[doc = "The timeout in seconds to execute the run command."]
    #[serde(rename = "timeoutInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i32>,
    #[doc = "Specifies the Azure storage blob where script output stream will be uploaded."]
    #[serde(rename = "outputBlobUri", default, skip_serializing_if = "Option::is_none")]
    pub output_blob_uri: Option<String>,
    #[doc = "Specifies the Azure storage blob where script error stream will be uploaded."]
    #[serde(rename = "errorBlobUri", default, skip_serializing_if = "Option::is_none")]
    pub error_blob_uri: Option<String>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The instance view of a virtual machine run command."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<VirtualMachineRunCommandInstanceView>,
}
impl VirtualMachineRunCommandProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the script sources for run command."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineRunCommandScriptSource {
    #[doc = "Specifies the script content to be executed on the VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[doc = "Specifies the script download location."]
    #[serde(rename = "scriptUri", default, skip_serializing_if = "Option::is_none")]
    pub script_uri: Option<String>,
    #[doc = "Specifies a commandId of predefined built-in script."]
    #[serde(rename = "commandId", default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
}
impl VirtualMachineRunCommandScriptSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Virtual Machine run command."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineRunCommandUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "Describes the properties of a Virtual Machine run command."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineRunCommandProperties>,
}
impl VirtualMachineRunCommandUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List run command operation response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineRunCommandsListResult {
    #[doc = "The list of run commands"]
    pub value: Vec<VirtualMachineRunCommand>,
    #[doc = "The uri to fetch the next page of run commands."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualMachineRunCommandsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualMachineRunCommandsListResult {
    pub fn new(value: Vec<VirtualMachineRunCommand>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes a Virtual Machine Scale Set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSet {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes a virtual machine scale set sku. NOTE: If the new VM SKU is not supported on the hardware the scale set is currently on, you need to deallocate the VMs in the scale set before you modify the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Specifies information about the marketplace image used to create the virtual machine. This element is only used for marketplace images. Before you can use a marketplace image from an API, you must enable the image for programmatic use.  In the Azure portal, find the marketplace image that you want to use and then click **Want to deploy programmatically, Get Started ->**. Enter any required information and then click **Save**."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[doc = "Describes the properties of a Virtual Machine Scale Set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetProperties>,
    #[doc = "Identity for the virtual machine scale set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<VirtualMachineScaleSetIdentity>,
    #[doc = "The virtual machine scale set zones. NOTE: Availability zones can only be set when you create the scale set"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "The complex type of the extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
}
impl VirtualMachineScaleSet {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            sku: None,
            plan: None,
            properties: None,
            identity: None,
            zones: Vec::new(),
            extended_location: None,
        }
    }
}
#[doc = "Describes a virtual machine scale set data disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSetDataDisk {
    #[doc = "The disk name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the logical unit number of the data disk. This value is used to identify data disks within the VM and therefore must be unique for each data disk attached to a VM."]
    pub lun: i32,
    #[doc = "Specifies the caching requirements. <br><br> Possible values are: <br><br> **None** <br><br> **ReadOnly** <br><br> **ReadWrite** <br><br> Default: **None for Standard storage. ReadOnly for Premium storage**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<Caching>,
    #[doc = "Specifies whether writeAccelerator should be enabled or disabled on the disk."]
    #[serde(rename = "writeAcceleratorEnabled", default, skip_serializing_if = "Option::is_none")]
    pub write_accelerator_enabled: Option<bool>,
    #[doc = "Specifies how the virtual machine should be created.<br><br> Possible values are:<br><br> **Attach** \\u2013 This value is used when you are using a specialized disk to create the virtual machine.<br><br> **FromImage** \\u2013 This value is used when you are using an image to create the virtual machine. If you are using a platform image, you also use the imageReference element described above. If you are using a marketplace image, you  also use the plan element previously described."]
    #[serde(rename = "createOption")]
    pub create_option: CreateOption,
    #[doc = "Specifies the size of an empty data disk in gigabytes. This element can be used to overwrite the size of the disk in a virtual machine image. <br><br> This value cannot be larger than 1023 GB"]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "Describes the parameters of a ScaleSet managed disk."]
    #[serde(rename = "managedDisk", default, skip_serializing_if = "Option::is_none")]
    pub managed_disk: Option<VirtualMachineScaleSetManagedDiskParameters>,
    #[doc = "Specifies the Read-Write IOPS for the managed disk. Should be used only when StorageAccountType is UltraSSD_LRS. If not specified, a default value would be assigned based on diskSizeGB."]
    #[serde(rename = "diskIOPSReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_iops_read_write: Option<i64>,
    #[doc = "Specifies the bandwidth in MB per second for the managed disk. Should be used only when StorageAccountType is UltraSSD_LRS. If not specified, a default value would be assigned based on diskSizeGB."]
    #[serde(rename = "diskMBpsReadWrite", default, skip_serializing_if = "Option::is_none")]
    pub disk_m_bps_read_write: Option<i64>,
    #[doc = "Specifies the behavior of the managed disk when the VM gets deleted i.e whether the managed disk is deleted or detached. Supported values:<br><br> **Delete** If this value is used, the managed disk is deleted when VM gets deleted.<br><br> **Detach** If this value is used, the managed disk is retained after VM gets deleted.<br><br> Minimum api-version: 2021-03-01"]
    #[serde(rename = "deleteOption", default, skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<DeleteOption>,
}
impl VirtualMachineScaleSetDataDisk {
    pub fn new(lun: i32, create_option: CreateOption) -> Self {
        Self {
            name: None,
            lun,
            caching: None,
            write_accelerator_enabled: None,
            create_option,
            disk_size_gb: None,
            managed_disk: None,
            disk_iops_read_write: None,
            disk_m_bps_read_write: None,
            delete_option: None,
        }
    }
}
#[doc = "Describes a Virtual Machine Scale Set Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetExtension {
    #[serde(flatten)]
    pub sub_resource_read_only: SubResourceReadOnly,
    #[doc = "The name of the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Describes the properties of a Virtual Machine Scale Set Extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetExtensionProperties>,
}
impl VirtualMachineScaleSetExtension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List VM scale set extension operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSetExtensionListResult {
    #[doc = "The list of VM scale set extensions."]
    pub value: Vec<VirtualMachineScaleSetExtension>,
    #[doc = "The uri to fetch the next page of VM scale set extensions. Call ListNext() with this to fetch the next page of VM scale set extensions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualMachineScaleSetExtensionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualMachineScaleSetExtensionListResult {
    pub fn new(value: Vec<VirtualMachineScaleSetExtension>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes a virtual machine scale set extension profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetExtensionProfile {
    #[doc = "The virtual machine scale set child extension resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<VirtualMachineScaleSetExtension>,
    #[doc = "Specifies the time alloted for all extensions to start. The time duration should be between 15 minutes and 120 minutes (inclusive) and should be specified in ISO 8601 format. The default value is 90 minutes (PT1H30M). <br><br> Minimum api-version: 2020-06-01"]
    #[serde(rename = "extensionsTimeBudget", default, skip_serializing_if = "Option::is_none")]
    pub extensions_time_budget: Option<String>,
}
impl VirtualMachineScaleSetExtensionProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Virtual Machine Scale Set Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetExtensionProperties {
    #[doc = "If a value is provided and is different from the previous value, the extension handler will be forced to update even if the extension configuration has not changed."]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[doc = "The name of the extension handler publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true."]
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[doc = "Indicates whether the extension should be automatically upgraded by the platform if there is a newer version of the extension available."]
    #[serde(rename = "enableAutomaticUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_upgrade: Option<bool>,
    #[doc = "Json formatted public settings for the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[doc = "The extension can contain either protectedSettings or protectedSettingsFromKeyVault or no protected settings at all."]
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Collection of extension names after which this extension needs to be provisioned."]
    #[serde(rename = "provisionAfterExtensions", default, skip_serializing_if = "Vec::is_empty")]
    pub provision_after_extensions: Vec<String>,
    #[doc = "Indicates whether failures stemming from the extension will be suppressed (Operational failures such as not connecting to the VM will not be suppressed regardless of this value). The default is false."]
    #[serde(rename = "suppressFailures", default, skip_serializing_if = "Option::is_none")]
    pub suppress_failures: Option<bool>,
    #[doc = "The extensions protected settings that are passed by reference, and consumed from key vault"]
    #[serde(rename = "protectedSettingsFromKeyVault", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings_from_key_vault: Option<serde_json::Value>,
}
impl VirtualMachineScaleSetExtensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Virtual Machine Scale Set Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetExtensionUpdate {
    #[serde(flatten)]
    pub sub_resource_read_only: SubResourceReadOnly,
    #[doc = "The name of the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Describes the properties of a Virtual Machine Scale Set Extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetExtensionProperties>,
}
impl VirtualMachineScaleSetExtensionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the hardware settings for the virtual machine scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetHardwareProfile {
    #[doc = "Specifies VM Size Property settings on the virtual machine."]
    #[serde(rename = "vmSizeProperties", default, skip_serializing_if = "Option::is_none")]
    pub vm_size_properties: Option<VmSizeProperties>,
}
impl VirtualMachineScaleSetHardwareProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual machine scale set network profile's IP configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSetIpConfiguration {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The IP configuration name."]
    pub name: String,
    #[doc = "Describes a virtual machine scale set network profile's IP configuration properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetIpConfigurationProperties>,
}
impl VirtualMachineScaleSetIpConfiguration {
    pub fn new(name: String) -> Self {
        Self {
            sub_resource: SubResource::default(),
            name,
            properties: None,
        }
    }
}
#[doc = "Describes a virtual machine scale set network profile's IP configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetIpConfigurationProperties {
    #[doc = "The API entity reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<ApiEntityReference>,
    #[doc = "Specifies the primary network interface in case the virtual machine has more than 1 network interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[doc = "Describes a virtual machines scale set IP Configuration's PublicIPAddress configuration"]
    #[serde(rename = "publicIPAddressConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address_configuration: Option<VirtualMachineScaleSetPublicIpAddressConfiguration>,
    #[doc = "Available from Api-Version 2017-03-30 onwards, it represents whether the specific ipconfiguration is IPv4 or IPv6. Default is taken as IPv4.  Possible values are: 'IPv4' and 'IPv6'."]
    #[serde(rename = "privateIPAddressVersion", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address_version: Option<virtual_machine_scale_set_ip_configuration_properties::PrivateIpAddressVersion>,
    #[doc = "Specifies an array of references to backend address pools of application gateways. A scale set can reference backend address pools of multiple application gateways. Multiple scale sets cannot use the same application gateway."]
    #[serde(rename = "applicationGatewayBackendAddressPools", default, skip_serializing_if = "Vec::is_empty")]
    pub application_gateway_backend_address_pools: Vec<SubResource>,
    #[doc = "Specifies an array of references to application security group."]
    #[serde(rename = "applicationSecurityGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub application_security_groups: Vec<SubResource>,
    #[doc = "Specifies an array of references to backend address pools of load balancers. A scale set can reference backend address pools of one public and one internal load balancer. Multiple scale sets cannot use the same basic sku load balancer."]
    #[serde(rename = "loadBalancerBackendAddressPools", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_backend_address_pools: Vec<SubResource>,
    #[doc = "Specifies an array of references to inbound Nat pools of the load balancers. A scale set can reference inbound nat pools of one public and one internal load balancer. Multiple scale sets cannot use the same basic sku load balancer."]
    #[serde(rename = "loadBalancerInboundNatPools", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_inbound_nat_pools: Vec<SubResource>,
}
impl VirtualMachineScaleSetIpConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_scale_set_ip_configuration_properties {
    use super::*;
    #[doc = "Available from Api-Version 2017-03-30 onwards, it represents whether the specific ipconfiguration is IPv4 or IPv6. Default is taken as IPv4.  Possible values are: 'IPv4' and 'IPv6'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivateIpAddressVersion")]
    pub enum PrivateIpAddressVersion {
        IPv4,
        IPv6,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivateIpAddressVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivateIpAddressVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivateIpAddressVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IPv4 => serializer.serialize_unit_variant("PrivateIpAddressVersion", 0u32, "IPv4"),
                Self::IPv6 => serializer.serialize_unit_variant("PrivateIpAddressVersion", 1u32, "IPv6"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Identity for the virtual machine scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetIdentity {
    #[doc = "The principal id of virtual machine scale set identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id associated with the virtual machine scale set. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of identity used for the virtual machine scale set. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the virtual machine scale set."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<virtual_machine_scale_set_identity::Type>,
    #[doc = "The list of user identities associated with the Virtual Machine. The user identity dictionary key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl VirtualMachineScaleSetIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_scale_set_identity {
    use super::*;
    #[doc = "The type of identity used for the virtual machine scale set. The type 'SystemAssigned, UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the virtual machine scale set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[doc = "The instance view of a virtual machine scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetInstanceView {
    #[doc = "Instance view statuses summary for virtual machines of a virtual machine scale set."]
    #[serde(rename = "virtualMachine", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine: Option<VirtualMachineScaleSetInstanceViewStatusesSummary>,
    #[doc = "The extensions information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<VirtualMachineScaleSetVmExtensionsSummary>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<InstanceViewStatus>,
    #[doc = "The orchestration services information."]
    #[serde(rename = "orchestrationServices", default, skip_serializing_if = "Vec::is_empty")]
    pub orchestration_services: Vec<OrchestrationServiceSummary>,
}
impl VirtualMachineScaleSetInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Instance view statuses summary for virtual machines of a virtual machine scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetInstanceViewStatusesSummary {
    #[doc = "The extensions information."]
    #[serde(rename = "statusesSummary", default, skip_serializing_if = "Vec::is_empty")]
    pub statuses_summary: Vec<VirtualMachineStatusCodeCount>,
}
impl VirtualMachineScaleSetInstanceViewStatusesSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the IP tag associated with the public IP address."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetIpTag {
    #[doc = "IP tag type. Example: FirstPartyUsage."]
    #[serde(rename = "ipTagType", default, skip_serializing_if = "Option::is_none")]
    pub ip_tag_type: Option<String>,
    #[doc = "IP tag associated with the public IP. Example: SQL, Storage etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}
impl VirtualMachineScaleSetIpTag {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Virtual Machine Scale Set OS Upgrade History operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSetListOsUpgradeHistory {
    #[doc = "The list of OS upgrades performed on the virtual machine scale set."]
    pub value: Vec<UpgradeOperationHistoricalStatusInfo>,
    #[doc = "The uri to fetch the next page of OS Upgrade History. Call ListNext() with this to fetch the next page of history of upgrades."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualMachineScaleSetListOsUpgradeHistory {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualMachineScaleSetListOsUpgradeHistory {
    pub fn new(value: Vec<UpgradeOperationHistoricalStatusInfo>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The List Virtual Machine operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSetListResult {
    #[doc = "The list of virtual machine scale sets."]
    pub value: Vec<VirtualMachineScaleSet>,
    #[doc = "The uri to fetch the next page of Virtual Machine Scale Sets. Call ListNext() with this to fetch the next page of VMSS."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualMachineScaleSetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualMachineScaleSetListResult {
    pub fn new(value: Vec<VirtualMachineScaleSet>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The Virtual Machine Scale Set List Skus operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSetListSkusResult {
    #[doc = "The list of skus available for the virtual machine scale set."]
    pub value: Vec<VirtualMachineScaleSetSku>,
    #[doc = "The uri to fetch the next page of Virtual Machine Scale Set Skus. Call ListNext() with this to fetch the next page of VMSS Skus."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualMachineScaleSetListSkusResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualMachineScaleSetListSkusResult {
    pub fn new(value: Vec<VirtualMachineScaleSetSku>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The List Virtual Machine operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSetListWithLinkResult {
    #[doc = "The list of virtual machine scale sets."]
    pub value: Vec<VirtualMachineScaleSet>,
    #[doc = "The uri to fetch the next page of Virtual Machine Scale Sets. Call ListNext() with this to fetch the next page of Virtual Machine Scale Sets."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualMachineScaleSetListWithLinkResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualMachineScaleSetListWithLinkResult {
    pub fn new(value: Vec<VirtualMachineScaleSet>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the parameters of a ScaleSet managed disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetManagedDiskParameters {
    #[doc = "Specifies the storage account type for the managed disk. Managed OS disk storage account type can only be set when you create the scale set. NOTE: UltraSSD_LRS can only be used with data disks. It cannot be used with OS Disk. Standard_LRS uses Standard HDD. StandardSSD_LRS uses Standard SSD. Premium_LRS uses Premium SSD. UltraSSD_LRS uses Ultra disk. Premium_ZRS uses Premium SSD zone redundant storage. StandardSSD_ZRS uses Standard SSD zone redundant storage. For more information regarding disks supported for Windows Virtual Machines, refer to https://docs.microsoft.com/azure/virtual-machines/windows/disks-types and, for Linux Virtual Machines, refer to https://docs.microsoft.com/azure/virtual-machines/linux/disks-types"]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<StorageAccountType>,
    #[doc = "Describes the parameter of customer managed disk encryption set resource id that can be specified for disk. <br><br> NOTE: The disk encryption set resource id can only be specified for managed disk. Please refer https://aka.ms/mdssewithcmkoverview for more details."]
    #[serde(rename = "diskEncryptionSet", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set: Option<DiskEncryptionSetParameters>,
    #[doc = "Specifies the security profile settings for the managed disk. <br><br> NOTE: It can only be set for Confidential VMs"]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<VmDiskSecurityProfile>,
}
impl VirtualMachineScaleSetManagedDiskParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual machine scale set network profile's network configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSetNetworkConfiguration {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The network configuration name."]
    pub name: String,
    #[doc = "Describes a virtual machine scale set network profile's IP configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetNetworkConfigurationProperties>,
}
impl VirtualMachineScaleSetNetworkConfiguration {
    pub fn new(name: String) -> Self {
        Self {
            sub_resource: SubResource::default(),
            name,
            properties: None,
        }
    }
}
#[doc = "Describes a virtual machines scale sets network configuration's DNS settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetNetworkConfigurationDnsSettings {
    #[doc = "List of DNS servers IP addresses"]
    #[serde(rename = "dnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<String>,
}
impl VirtualMachineScaleSetNetworkConfigurationDnsSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual machine scale set network profile's IP configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSetNetworkConfigurationProperties {
    #[doc = "Specifies the primary network interface in case the virtual machine has more than 1 network interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[doc = "Specifies whether the network interface is accelerated networking-enabled."]
    #[serde(rename = "enableAcceleratedNetworking", default, skip_serializing_if = "Option::is_none")]
    pub enable_accelerated_networking: Option<bool>,
    #[doc = "Specifies whether the network interface is FPGA networking-enabled."]
    #[serde(rename = "enableFpga", default, skip_serializing_if = "Option::is_none")]
    pub enable_fpga: Option<bool>,
    #[serde(rename = "networkSecurityGroup", default, skip_serializing_if = "Option::is_none")]
    pub network_security_group: Option<SubResource>,
    #[doc = "Describes a virtual machines scale sets network configuration's DNS settings."]
    #[serde(rename = "dnsSettings", default, skip_serializing_if = "Option::is_none")]
    pub dns_settings: Option<VirtualMachineScaleSetNetworkConfigurationDnsSettings>,
    #[doc = "Specifies the IP configurations of the network interface."]
    #[serde(rename = "ipConfigurations")]
    pub ip_configurations: Vec<VirtualMachineScaleSetIpConfiguration>,
    #[doc = "Whether IP forwarding enabled on this NIC."]
    #[serde(rename = "enableIPForwarding", default, skip_serializing_if = "Option::is_none")]
    pub enable_ip_forwarding: Option<bool>,
    #[doc = "Specify what happens to the network interface when the VM is deleted"]
    #[serde(rename = "deleteOption", default, skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<virtual_machine_scale_set_network_configuration_properties::DeleteOption>,
}
impl VirtualMachineScaleSetNetworkConfigurationProperties {
    pub fn new(ip_configurations: Vec<VirtualMachineScaleSetIpConfiguration>) -> Self {
        Self {
            primary: None,
            enable_accelerated_networking: None,
            enable_fpga: None,
            network_security_group: None,
            dns_settings: None,
            ip_configurations,
            enable_ip_forwarding: None,
            delete_option: None,
        }
    }
}
pub mod virtual_machine_scale_set_network_configuration_properties {
    use super::*;
    #[doc = "Specify what happens to the network interface when the VM is deleted"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeleteOption")]
    pub enum DeleteOption {
        Delete,
        Detach,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeleteOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeleteOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeleteOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Delete => serializer.serialize_unit_variant("DeleteOption", 0u32, "Delete"),
                Self::Detach => serializer.serialize_unit_variant("DeleteOption", 1u32, "Detach"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a virtual machine scale set network profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetNetworkProfile {
    #[doc = "The API entity reference."]
    #[serde(rename = "healthProbe", default, skip_serializing_if = "Option::is_none")]
    pub health_probe: Option<ApiEntityReference>,
    #[doc = "The list of network configurations."]
    #[serde(rename = "networkInterfaceConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interface_configurations: Vec<VirtualMachineScaleSetNetworkConfiguration>,
    #[doc = "specifies the Microsoft.Network API version used when creating networking resources in the Network Interface Configurations for Virtual Machine Scale Set with orchestration mode 'Flexible'"]
    #[serde(rename = "networkApiVersion", default, skip_serializing_if = "Option::is_none")]
    pub network_api_version: Option<virtual_machine_scale_set_network_profile::NetworkApiVersion>,
}
impl VirtualMachineScaleSetNetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_scale_set_network_profile {
    use super::*;
    #[doc = "specifies the Microsoft.Network API version used when creating networking resources in the Network Interface Configurations for Virtual Machine Scale Set with orchestration mode 'Flexible'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NetworkApiVersion")]
    pub enum NetworkApiVersion {
        #[serde(rename = "2020-11-01")]
        N2020_11_01,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NetworkApiVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NetworkApiVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NetworkApiVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N2020_11_01 => serializer.serialize_unit_variant("NetworkApiVersion", 0u32, "2020-11-01"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a virtual machine scale set operating system disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSetOsDisk {
    #[doc = "The disk name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the caching requirements. <br><br> Possible values are: <br><br> **None** <br><br> **ReadOnly** <br><br> **ReadWrite** <br><br> Default: **None for Standard storage. ReadOnly for Premium storage**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<Caching>,
    #[doc = "Specifies whether writeAccelerator should be enabled or disabled on the disk."]
    #[serde(rename = "writeAcceleratorEnabled", default, skip_serializing_if = "Option::is_none")]
    pub write_accelerator_enabled: Option<bool>,
    #[doc = "Specifies how the virtual machine should be created.<br><br> Possible values are:<br><br> **Attach** \\u2013 This value is used when you are using a specialized disk to create the virtual machine.<br><br> **FromImage** \\u2013 This value is used when you are using an image to create the virtual machine. If you are using a platform image, you also use the imageReference element described above. If you are using a marketplace image, you  also use the plan element previously described."]
    #[serde(rename = "createOption")]
    pub create_option: CreateOption,
    #[doc = "Describes the parameters of ephemeral disk settings that can be specified for operating system disk. <br><br> NOTE: The ephemeral disk settings can only be specified for managed disk."]
    #[serde(rename = "diffDiskSettings", default, skip_serializing_if = "Option::is_none")]
    pub diff_disk_settings: Option<DiffDiskSettings>,
    #[doc = "Specifies the size of the operating system disk in gigabytes. This element can be used to overwrite the size of the disk in a virtual machine image. <br><br> This value cannot be larger than 1023 GB"]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "This property allows you to specify the type of the OS that is included in the disk if creating a VM from user-image or a specialized VHD. <br><br> Possible values are: <br><br> **Windows** <br><br> **Linux**"]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<virtual_machine_scale_set_os_disk::OsType>,
    #[doc = "Describes the uri of a disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<VirtualHardDisk>,
    #[doc = "Specifies the container urls that are used to store operating system disks for the scale set."]
    #[serde(rename = "vhdContainers", default, skip_serializing_if = "Vec::is_empty")]
    pub vhd_containers: Vec<String>,
    #[doc = "Describes the parameters of a ScaleSet managed disk."]
    #[serde(rename = "managedDisk", default, skip_serializing_if = "Option::is_none")]
    pub managed_disk: Option<VirtualMachineScaleSetManagedDiskParameters>,
    #[doc = "Specifies the behavior of the managed disk when the VM gets deleted i.e whether the managed disk is deleted or detached. Supported values:<br><br> **Delete** If this value is used, the managed disk is deleted when VM gets deleted.<br><br> **Detach** If this value is used, the managed disk is retained after VM gets deleted.<br><br> Minimum api-version: 2021-03-01"]
    #[serde(rename = "deleteOption", default, skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<DeleteOption>,
}
impl VirtualMachineScaleSetOsDisk {
    pub fn new(create_option: CreateOption) -> Self {
        Self {
            name: None,
            caching: None,
            write_accelerator_enabled: None,
            create_option,
            diff_disk_settings: None,
            disk_size_gb: None,
            os_type: None,
            image: None,
            vhd_containers: Vec::new(),
            managed_disk: None,
            delete_option: None,
        }
    }
}
pub mod virtual_machine_scale_set_os_disk {
    use super::*;
    #[doc = "This property allows you to specify the type of the OS that is included in the disk if creating a VM from user-image or a specialized VHD. <br><br> Possible values are: <br><br> **Windows** <br><br> **Linux**"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[doc = "Describes a virtual machine scale set OS profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetOsProfile {
    #[doc = "Specifies the computer name prefix for all of the virtual machines in the scale set. Computer name prefixes must be 1 to 15 characters long."]
    #[serde(rename = "computerNamePrefix", default, skip_serializing_if = "Option::is_none")]
    pub computer_name_prefix: Option<String>,
    #[doc = "Specifies the name of the administrator account. <br><br> **Windows-only restriction:** Cannot end in \".\" <br><br> **Disallowed values:** \"administrator\", \"admin\", \"user\", \"user1\", \"test\", \"user2\", \"test1\", \"user3\", \"admin1\", \"1\", \"123\", \"a\", \"actuser\", \"adm\", \"admin2\", \"aspnet\", \"backup\", \"console\", \"david\", \"guest\", \"john\", \"owner\", \"root\", \"server\", \"sql\", \"support\", \"support_388945a0\", \"sys\", \"test2\", \"test3\", \"user4\", \"user5\". <br><br> **Minimum-length (Linux):** 1  character <br><br> **Max-length (Linux):** 64 characters <br><br> **Max-length (Windows):** 20 characters"]
    #[serde(rename = "adminUsername", default, skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,
    #[doc = "Specifies the password of the administrator account. <br><br> **Minimum-length (Windows):** 8 characters <br><br> **Minimum-length (Linux):** 6 characters <br><br> **Max-length (Windows):** 123 characters <br><br> **Max-length (Linux):** 72 characters <br><br> **Complexity requirements:** 3 out of 4 conditions below need to be fulfilled <br> Has lower characters <br>Has upper characters <br> Has a digit <br> Has a special character (Regex match [\\W_]) <br><br> **Disallowed values:** \"abc@123\", \"P@$$w0rd\", \"P@ssw0rd\", \"P@ssword123\", \"Pa$$word\", \"pass@word1\", \"Password!\", \"Password1\", \"Password22\", \"iloveyou!\" <br><br> For resetting the password, see [How to reset the Remote Desktop service or its login password in a Windows VM](https://docs.microsoft.com/troubleshoot/azure/virtual-machines/reset-rdp) <br><br> For resetting root password, see [Manage users, SSH, and check or repair disks on Azure Linux VMs using the VMAccess Extension](https://docs.microsoft.com/troubleshoot/azure/virtual-machines/troubleshoot-ssh-connection)"]
    #[serde(rename = "adminPassword", default, skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<String>,
    #[doc = "Specifies a base-64 encoded string of custom data. The base-64 encoded string is decoded to a binary array that is saved as a file on the Virtual Machine. The maximum length of the binary array is 65535 bytes. <br><br> For using cloud-init for your VM, see [Using cloud-init to customize a Linux VM during creation](https://docs.microsoft.com/azure/virtual-machines/linux/using-cloud-init)"]
    #[serde(rename = "customData", default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<String>,
    #[doc = "Specifies Windows operating system settings on the virtual machine."]
    #[serde(rename = "windowsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<WindowsConfiguration>,
    #[doc = "Specifies the Linux operating system settings on the virtual machine. <br><br>For a list of supported Linux distributions, see [Linux on Azure-Endorsed Distributions](https://docs.microsoft.com/azure/virtual-machines/linux/endorsed-distros)."]
    #[serde(rename = "linuxConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub linux_configuration: Option<LinuxConfiguration>,
    #[doc = "Specifies set of certificates that should be installed onto the virtual machines in the scale set. To install certificates on a virtual machine it is recommended to use the [Azure Key Vault virtual machine extension for Linux](https://docs.microsoft.com/azure/virtual-machines/extensions/key-vault-linux) or the [Azure Key Vault virtual machine extension for Windows](https://docs.microsoft.com/azure/virtual-machines/extensions/key-vault-windows)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<VaultSecretGroup>,
    #[doc = "Specifies whether extension operations should be allowed on the virtual machine scale set. <br><br>This may only be set to False when no extensions are present on the virtual machine scale set."]
    #[serde(rename = "allowExtensionOperations", default, skip_serializing_if = "Option::is_none")]
    pub allow_extension_operations: Option<bool>,
}
impl VirtualMachineScaleSetOsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Virtual Machine Scale Set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetProperties {
    #[doc = "Describes an upgrade policy - automatic, manual, or rolling."]
    #[serde(rename = "upgradePolicy", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_policy: Option<UpgradePolicy>,
    #[doc = "Specifies the configuration parameters for automatic repairs on the virtual machine scale set."]
    #[serde(rename = "automaticRepairsPolicy", default, skip_serializing_if = "Option::is_none")]
    pub automatic_repairs_policy: Option<AutomaticRepairsPolicy>,
    #[doc = "Describes a virtual machine scale set virtual machine profile."]
    #[serde(rename = "virtualMachineProfile", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_profile: Option<VirtualMachineScaleSetVmProfile>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Specifies whether the Virtual Machine Scale Set should be overprovisioned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overprovision: Option<bool>,
    #[doc = "When Overprovision is enabled, extensions are launched only on the requested number of VMs which are finally kept. This property will hence ensure that the extensions do not run on the extra overprovisioned VMs."]
    #[serde(
        rename = "doNotRunExtensionsOnOverprovisionedVMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_run_extensions_on_overprovisioned_v_ms: Option<bool>,
    #[doc = "Specifies the ID which uniquely identifies a Virtual Machine Scale Set."]
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[doc = "When true this limits the scale set to a single placement group, of max size 100 virtual machines. NOTE: If singlePlacementGroup is true, it may be modified to false. However, if singlePlacementGroup is false, it may not be modified to true."]
    #[serde(rename = "singlePlacementGroup", default, skip_serializing_if = "Option::is_none")]
    pub single_placement_group: Option<bool>,
    #[doc = "Whether to force strictly even Virtual Machine distribution cross x-zones in case there is zone outage. zoneBalance property can only be set if the zones property of the scale set contains more than one zone. If there are no zones or only one zone specified, then zoneBalance property should not be set."]
    #[serde(rename = "zoneBalance", default, skip_serializing_if = "Option::is_none")]
    pub zone_balance: Option<bool>,
    #[doc = "Fault Domain count for each placement group."]
    #[serde(rename = "platformFaultDomainCount", default, skip_serializing_if = "Option::is_none")]
    pub platform_fault_domain_count: Option<i32>,
    #[serde(rename = "proximityPlacementGroup", default, skip_serializing_if = "Option::is_none")]
    pub proximity_placement_group: Option<SubResource>,
    #[serde(rename = "hostGroup", default, skip_serializing_if = "Option::is_none")]
    pub host_group: Option<SubResource>,
    #[doc = "Enables or disables a capability on the virtual machine or virtual machine scale set."]
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub additional_capabilities: Option<AdditionalCapabilities>,
    #[doc = "Describes a scale-in policy for a virtual machine scale set."]
    #[serde(rename = "scaleInPolicy", default, skip_serializing_if = "Option::is_none")]
    pub scale_in_policy: Option<ScaleInPolicy>,
    #[doc = "Specifies the orchestration mode for the virtual machine scale set."]
    #[serde(rename = "orchestrationMode", default, skip_serializing_if = "Option::is_none")]
    pub orchestration_mode: Option<OrchestrationMode>,
    #[doc = "Specifies the Spot-Try-Restore properties for the virtual machine scale set. <br><br> With this property customer can enable or disable automatic restore of the evicted Spot VMSS VM instances opportunistically based on capacity availability and pricing constraint."]
    #[serde(rename = "spotRestorePolicy", default, skip_serializing_if = "Option::is_none")]
    pub spot_restore_policy: Option<SpotRestorePolicy>,
    #[doc = "Specifies the time at which the Virtual Machine Scale Set resource was created.<br><br>Minimum api-version: 2022-03-01."]
    #[serde(rename = "timeCreated", default, with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<time::OffsetDateTime>,
}
impl VirtualMachineScaleSetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual machines scale set IP Configuration's PublicIPAddress configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSetPublicIpAddressConfiguration {
    #[doc = "The publicIP address configuration name."]
    pub name: String,
    #[doc = "Describes a virtual machines scale set IP Configuration's PublicIPAddress configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetPublicIpAddressConfigurationProperties>,
    #[doc = "Describes the public IP Sku. It can only be set with OrchestrationMode as Flexible."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PublicIpAddressSku>,
}
impl VirtualMachineScaleSetPublicIpAddressConfiguration {
    pub fn new(name: String) -> Self {
        Self {
            name,
            properties: None,
            sku: None,
        }
    }
}
#[doc = "Describes a virtual machines scale sets network configuration's DNS settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSetPublicIpAddressConfigurationDnsSettings {
    #[doc = "The Domain name label.The concatenation of the domain name label and vm index will be the domain name labels of the PublicIPAddress resources that will be created"]
    #[serde(rename = "domainNameLabel")]
    pub domain_name_label: String,
}
impl VirtualMachineScaleSetPublicIpAddressConfigurationDnsSettings {
    pub fn new(domain_name_label: String) -> Self {
        Self { domain_name_label }
    }
}
#[doc = "Describes a virtual machines scale set IP Configuration's PublicIPAddress configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetPublicIpAddressConfigurationProperties {
    #[doc = "The idle timeout of the public IP address."]
    #[serde(rename = "idleTimeoutInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_in_minutes: Option<i32>,
    #[doc = "Describes a virtual machines scale sets network configuration's DNS settings."]
    #[serde(rename = "dnsSettings", default, skip_serializing_if = "Option::is_none")]
    pub dns_settings: Option<VirtualMachineScaleSetPublicIpAddressConfigurationDnsSettings>,
    #[doc = "The list of IP tags associated with the public IP address."]
    #[serde(rename = "ipTags", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_tags: Vec<VirtualMachineScaleSetIpTag>,
    #[serde(rename = "publicIPPrefix", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_prefix: Option<SubResource>,
    #[doc = "Available from Api-Version 2019-07-01 onwards, it represents whether the specific ipconfiguration is IPv4 or IPv6. Default is taken as IPv4. Possible values are: 'IPv4' and 'IPv6'."]
    #[serde(rename = "publicIPAddressVersion", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address_version: Option<virtual_machine_scale_set_public_ip_address_configuration_properties::PublicIpAddressVersion>,
    #[doc = "Specify what happens to the public IP when the VM is deleted"]
    #[serde(rename = "deleteOption", default, skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<virtual_machine_scale_set_public_ip_address_configuration_properties::DeleteOption>,
}
impl VirtualMachineScaleSetPublicIpAddressConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_scale_set_public_ip_address_configuration_properties {
    use super::*;
    #[doc = "Available from Api-Version 2019-07-01 onwards, it represents whether the specific ipconfiguration is IPv4 or IPv6. Default is taken as IPv4. Possible values are: 'IPv4' and 'IPv6'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicIpAddressVersion")]
    pub enum PublicIpAddressVersion {
        IPv4,
        IPv6,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicIpAddressVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicIpAddressVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicIpAddressVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IPv4 => serializer.serialize_unit_variant("PublicIpAddressVersion", 0u32, "IPv4"),
                Self::IPv6 => serializer.serialize_unit_variant("PublicIpAddressVersion", 1u32, "IPv6"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specify what happens to the public IP when the VM is deleted"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeleteOption")]
    pub enum DeleteOption {
        Delete,
        Detach,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeleteOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeleteOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeleteOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Delete => serializer.serialize_unit_variant("DeleteOption", 0u32, "Delete"),
                Self::Detach => serializer.serialize_unit_variant("DeleteOption", 1u32, "Detach"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a Virtual Machine Scale Set VM Reimage Parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetReimageParameters {
    #[serde(flatten)]
    pub virtual_machine_scale_set_vm_reimage_parameters: VirtualMachineScaleSetVmReimageParameters,
    #[doc = "The virtual machine scale set instance ids. Omitting the virtual machine scale set instance ids will result in the operation being performed on all virtual machines in the virtual machine scale set."]
    #[serde(rename = "instanceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub instance_ids: Vec<String>,
}
impl VirtualMachineScaleSetReimageParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an available virtual machine scale set sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetSku {
    #[doc = "The type of resource the sku applies to."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Describes a virtual machine scale set sku. NOTE: If the new VM SKU is not supported on the hardware the scale set is currently on, you need to deallocate the VMs in the scale set before you modify the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Describes scaling information of a sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<VirtualMachineScaleSetSkuCapacity>,
}
impl VirtualMachineScaleSetSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes scaling information of a sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetSkuCapacity {
    #[doc = "The minimum capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[doc = "The maximum capacity that can be set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[doc = "The default capacity."]
    #[serde(rename = "defaultCapacity", default, skip_serializing_if = "Option::is_none")]
    pub default_capacity: Option<i64>,
    #[doc = "The scale type applicable to the sku."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<virtual_machine_scale_set_sku_capacity::ScaleType>,
}
impl VirtualMachineScaleSetSkuCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_scale_set_sku_capacity {
    use super::*;
    #[doc = "The scale type applicable to the sku."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScaleType {
        Automatic,
        None,
    }
}
#[doc = "Describes a virtual machine scale set storage profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetStorageProfile {
    #[doc = "Specifies information about the image to use. You can specify information about platform images, marketplace images, or virtual machine images. This element is required when you want to use a platform image, marketplace image, or virtual machine image, but is not used in other creation operations. NOTE: Image reference publisher and offer can only be set when you create the scale set."]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[doc = "Describes a virtual machine scale set operating system disk."]
    #[serde(rename = "osDisk", default, skip_serializing_if = "Option::is_none")]
    pub os_disk: Option<VirtualMachineScaleSetOsDisk>,
    #[doc = "Specifies the parameters that are used to add data disks to the virtual machines in the scale set. <br><br> For more information about disks, see [About disks and VHDs for Azure virtual machines](https://docs.microsoft.com/azure/virtual-machines/managed-disks-overview)."]
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disks: Vec<VirtualMachineScaleSetDataDisk>,
}
impl VirtualMachineScaleSetStorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Virtual Machine Scale Set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "Describes a virtual machine scale set sku. NOTE: If the new VM SKU is not supported on the hardware the scale set is currently on, you need to deallocate the VMs in the scale set before you modify the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Specifies information about the marketplace image used to create the virtual machine. This element is only used for marketplace images. Before you can use a marketplace image from an API, you must enable the image for programmatic use.  In the Azure portal, find the marketplace image that you want to use and then click **Want to deploy programmatically, Get Started ->**. Enter any required information and then click **Save**."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[doc = "Describes the properties of a Virtual Machine Scale Set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetUpdateProperties>,
    #[doc = "Identity for the virtual machine scale set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<VirtualMachineScaleSetIdentity>,
}
impl VirtualMachineScaleSetUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual machine scale set network profile's IP configuration. NOTE: The subnet of a scale set may be modified as long as the original subnet and the new subnet are in the same virtual network"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetUpdateIpConfiguration {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The IP configuration name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes a virtual machine scale set network profile's IP configuration properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetUpdateIpConfigurationProperties>,
}
impl VirtualMachineScaleSetUpdateIpConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual machine scale set network profile's IP configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetUpdateIpConfigurationProperties {
    #[doc = "The API entity reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<ApiEntityReference>,
    #[doc = "Specifies the primary IP Configuration in case the network interface has more than one IP Configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[doc = "Describes a virtual machines scale set IP Configuration's PublicIPAddress configuration"]
    #[serde(rename = "publicIPAddressConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address_configuration: Option<VirtualMachineScaleSetUpdatePublicIpAddressConfiguration>,
    #[doc = "Available from Api-Version 2017-03-30 onwards, it represents whether the specific ipconfiguration is IPv4 or IPv6. Default is taken as IPv4.  Possible values are: 'IPv4' and 'IPv6'."]
    #[serde(rename = "privateIPAddressVersion", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address_version: Option<virtual_machine_scale_set_update_ip_configuration_properties::PrivateIpAddressVersion>,
    #[doc = "The application gateway backend address pools."]
    #[serde(rename = "applicationGatewayBackendAddressPools", default, skip_serializing_if = "Vec::is_empty")]
    pub application_gateway_backend_address_pools: Vec<SubResource>,
    #[doc = "Specifies an array of references to application security group."]
    #[serde(rename = "applicationSecurityGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub application_security_groups: Vec<SubResource>,
    #[doc = "The load balancer backend address pools."]
    #[serde(rename = "loadBalancerBackendAddressPools", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_backend_address_pools: Vec<SubResource>,
    #[doc = "The load balancer inbound nat pools."]
    #[serde(rename = "loadBalancerInboundNatPools", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_inbound_nat_pools: Vec<SubResource>,
}
impl VirtualMachineScaleSetUpdateIpConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_scale_set_update_ip_configuration_properties {
    use super::*;
    #[doc = "Available from Api-Version 2017-03-30 onwards, it represents whether the specific ipconfiguration is IPv4 or IPv6. Default is taken as IPv4.  Possible values are: 'IPv4' and 'IPv6'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivateIpAddressVersion")]
    pub enum PrivateIpAddressVersion {
        IPv4,
        IPv6,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivateIpAddressVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivateIpAddressVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivateIpAddressVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::IPv4 => serializer.serialize_unit_variant("PrivateIpAddressVersion", 0u32, "IPv4"),
                Self::IPv6 => serializer.serialize_unit_variant("PrivateIpAddressVersion", 1u32, "IPv6"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a virtual machine scale set network profile's network configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetUpdateNetworkConfiguration {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The network configuration name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes a virtual machine scale set updatable network profile's IP configuration.Use this object for updating network profile's IP Configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetUpdateNetworkConfigurationProperties>,
}
impl VirtualMachineScaleSetUpdateNetworkConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual machine scale set updatable network profile's IP configuration.Use this object for updating network profile's IP Configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetUpdateNetworkConfigurationProperties {
    #[doc = "Whether this is a primary NIC on a virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[doc = "Specifies whether the network interface is accelerated networking-enabled."]
    #[serde(rename = "enableAcceleratedNetworking", default, skip_serializing_if = "Option::is_none")]
    pub enable_accelerated_networking: Option<bool>,
    #[doc = "Specifies whether the network interface is FPGA networking-enabled."]
    #[serde(rename = "enableFpga", default, skip_serializing_if = "Option::is_none")]
    pub enable_fpga: Option<bool>,
    #[serde(rename = "networkSecurityGroup", default, skip_serializing_if = "Option::is_none")]
    pub network_security_group: Option<SubResource>,
    #[doc = "Describes a virtual machines scale sets network configuration's DNS settings."]
    #[serde(rename = "dnsSettings", default, skip_serializing_if = "Option::is_none")]
    pub dns_settings: Option<VirtualMachineScaleSetNetworkConfigurationDnsSettings>,
    #[doc = "The virtual machine scale set IP Configuration."]
    #[serde(rename = "ipConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_configurations: Vec<VirtualMachineScaleSetUpdateIpConfiguration>,
    #[doc = "Whether IP forwarding enabled on this NIC."]
    #[serde(rename = "enableIPForwarding", default, skip_serializing_if = "Option::is_none")]
    pub enable_ip_forwarding: Option<bool>,
    #[doc = "Specify what happens to the network interface when the VM is deleted"]
    #[serde(rename = "deleteOption", default, skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<virtual_machine_scale_set_update_network_configuration_properties::DeleteOption>,
}
impl VirtualMachineScaleSetUpdateNetworkConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_scale_set_update_network_configuration_properties {
    use super::*;
    #[doc = "Specify what happens to the network interface when the VM is deleted"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeleteOption")]
    pub enum DeleteOption {
        Delete,
        Detach,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeleteOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeleteOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeleteOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Delete => serializer.serialize_unit_variant("DeleteOption", 0u32, "Delete"),
                Self::Detach => serializer.serialize_unit_variant("DeleteOption", 1u32, "Detach"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a virtual machine scale set network profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetUpdateNetworkProfile {
    #[doc = "The API entity reference."]
    #[serde(rename = "healthProbe", default, skip_serializing_if = "Option::is_none")]
    pub health_probe: Option<ApiEntityReference>,
    #[doc = "The list of network configurations."]
    #[serde(rename = "networkInterfaceConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interface_configurations: Vec<VirtualMachineScaleSetUpdateNetworkConfiguration>,
    #[doc = "specifies the Microsoft.Network API version used when creating networking resources in the Network Interface Configurations for Virtual Machine Scale Set with orchestration mode 'Flexible'"]
    #[serde(rename = "networkApiVersion", default, skip_serializing_if = "Option::is_none")]
    pub network_api_version: Option<virtual_machine_scale_set_update_network_profile::NetworkApiVersion>,
}
impl VirtualMachineScaleSetUpdateNetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_scale_set_update_network_profile {
    use super::*;
    #[doc = "specifies the Microsoft.Network API version used when creating networking resources in the Network Interface Configurations for Virtual Machine Scale Set with orchestration mode 'Flexible'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NetworkApiVersion")]
    pub enum NetworkApiVersion {
        #[serde(rename = "2020-11-01")]
        N2020_11_01,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NetworkApiVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NetworkApiVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NetworkApiVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N2020_11_01 => serializer.serialize_unit_variant("NetworkApiVersion", 0u32, "2020-11-01"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes virtual machine scale set operating system disk Update Object. This should be used for Updating VMSS OS Disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetUpdateOsDisk {
    #[doc = "Specifies the caching requirements. <br><br> Possible values are: <br><br> **None** <br><br> **ReadOnly** <br><br> **ReadWrite** <br><br> Default: **None for Standard storage. ReadOnly for Premium storage**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<Caching>,
    #[doc = "Specifies whether writeAccelerator should be enabled or disabled on the disk."]
    #[serde(rename = "writeAcceleratorEnabled", default, skip_serializing_if = "Option::is_none")]
    pub write_accelerator_enabled: Option<bool>,
    #[doc = "Specifies the size of the operating system disk in gigabytes. This element can be used to overwrite the size of the disk in a virtual machine image. <br><br> This value cannot be larger than 1023 GB"]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "Describes the uri of a disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<VirtualHardDisk>,
    #[doc = "The list of virtual hard disk container uris."]
    #[serde(rename = "vhdContainers", default, skip_serializing_if = "Vec::is_empty")]
    pub vhd_containers: Vec<String>,
    #[doc = "Describes the parameters of a ScaleSet managed disk."]
    #[serde(rename = "managedDisk", default, skip_serializing_if = "Option::is_none")]
    pub managed_disk: Option<VirtualMachineScaleSetManagedDiskParameters>,
    #[doc = "Specifies the behavior of the managed disk when the VM gets deleted i.e whether the managed disk is deleted or detached. Supported values:<br><br> **Delete** If this value is used, the managed disk is deleted when VM gets deleted.<br><br> **Detach** If this value is used, the managed disk is retained after VM gets deleted.<br><br> Minimum api-version: 2021-03-01"]
    #[serde(rename = "deleteOption", default, skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<DeleteOption>,
}
impl VirtualMachineScaleSetUpdateOsDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual machine scale set OS profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetUpdateOsProfile {
    #[doc = "A base-64 encoded string of custom data."]
    #[serde(rename = "customData", default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<String>,
    #[doc = "Specifies Windows operating system settings on the virtual machine."]
    #[serde(rename = "windowsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<WindowsConfiguration>,
    #[doc = "Specifies the Linux operating system settings on the virtual machine. <br><br>For a list of supported Linux distributions, see [Linux on Azure-Endorsed Distributions](https://docs.microsoft.com/azure/virtual-machines/linux/endorsed-distros)."]
    #[serde(rename = "linuxConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub linux_configuration: Option<LinuxConfiguration>,
    #[doc = "The List of certificates for addition to the VM."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<VaultSecretGroup>,
}
impl VirtualMachineScaleSetUpdateOsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Virtual Machine Scale Set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetUpdateProperties {
    #[doc = "Describes an upgrade policy - automatic, manual, or rolling."]
    #[serde(rename = "upgradePolicy", default, skip_serializing_if = "Option::is_none")]
    pub upgrade_policy: Option<UpgradePolicy>,
    #[doc = "Specifies the configuration parameters for automatic repairs on the virtual machine scale set."]
    #[serde(rename = "automaticRepairsPolicy", default, skip_serializing_if = "Option::is_none")]
    pub automatic_repairs_policy: Option<AutomaticRepairsPolicy>,
    #[doc = "Describes a virtual machine scale set virtual machine profile."]
    #[serde(rename = "virtualMachineProfile", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_profile: Option<VirtualMachineScaleSetUpdateVmProfile>,
    #[doc = "Specifies whether the Virtual Machine Scale Set should be overprovisioned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overprovision: Option<bool>,
    #[doc = "When Overprovision is enabled, extensions are launched only on the requested number of VMs which are finally kept. This property will hence ensure that the extensions do not run on the extra overprovisioned VMs."]
    #[serde(
        rename = "doNotRunExtensionsOnOverprovisionedVMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub do_not_run_extensions_on_overprovisioned_v_ms: Option<bool>,
    #[doc = "When true this limits the scale set to a single placement group, of max size 100 virtual machines. NOTE: If singlePlacementGroup is true, it may be modified to false. However, if singlePlacementGroup is false, it may not be modified to true."]
    #[serde(rename = "singlePlacementGroup", default, skip_serializing_if = "Option::is_none")]
    pub single_placement_group: Option<bool>,
    #[doc = "Enables or disables a capability on the virtual machine or virtual machine scale set."]
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub additional_capabilities: Option<AdditionalCapabilities>,
    #[doc = "Describes a scale-in policy for a virtual machine scale set."]
    #[serde(rename = "scaleInPolicy", default, skip_serializing_if = "Option::is_none")]
    pub scale_in_policy: Option<ScaleInPolicy>,
    #[serde(rename = "proximityPlacementGroup", default, skip_serializing_if = "Option::is_none")]
    pub proximity_placement_group: Option<SubResource>,
}
impl VirtualMachineScaleSetUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual machines scale set IP Configuration's PublicIPAddress configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetUpdatePublicIpAddressConfiguration {
    #[doc = "The publicIP address configuration name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes a virtual machines scale set IP Configuration's PublicIPAddress configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetUpdatePublicIpAddressConfigurationProperties>,
}
impl VirtualMachineScaleSetUpdatePublicIpAddressConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual machines scale set IP Configuration's PublicIPAddress configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetUpdatePublicIpAddressConfigurationProperties {
    #[doc = "The idle timeout of the public IP address."]
    #[serde(rename = "idleTimeoutInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_in_minutes: Option<i32>,
    #[doc = "Describes a virtual machines scale sets network configuration's DNS settings."]
    #[serde(rename = "dnsSettings", default, skip_serializing_if = "Option::is_none")]
    pub dns_settings: Option<VirtualMachineScaleSetPublicIpAddressConfigurationDnsSettings>,
    #[serde(rename = "publicIPPrefix", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_prefix: Option<SubResource>,
    #[doc = "Specify what happens to the public IP when the VM is deleted"]
    #[serde(rename = "deleteOption", default, skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<virtual_machine_scale_set_update_public_ip_address_configuration_properties::DeleteOption>,
}
impl VirtualMachineScaleSetUpdatePublicIpAddressConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_scale_set_update_public_ip_address_configuration_properties {
    use super::*;
    #[doc = "Specify what happens to the public IP when the VM is deleted"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeleteOption")]
    pub enum DeleteOption {
        Delete,
        Detach,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeleteOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeleteOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeleteOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Delete => serializer.serialize_unit_variant("DeleteOption", 0u32, "Delete"),
                Self::Detach => serializer.serialize_unit_variant("DeleteOption", 1u32, "Detach"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes a virtual machine scale set storage profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetUpdateStorageProfile {
    #[doc = "Specifies information about the image to use. You can specify information about platform images, marketplace images, or virtual machine images. This element is required when you want to use a platform image, marketplace image, or virtual machine image, but is not used in other creation operations. NOTE: Image reference publisher and offer can only be set when you create the scale set."]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[doc = "Describes virtual machine scale set operating system disk Update Object. This should be used for Updating VMSS OS Disk."]
    #[serde(rename = "osDisk", default, skip_serializing_if = "Option::is_none")]
    pub os_disk: Option<VirtualMachineScaleSetUpdateOsDisk>,
    #[doc = "The data disks."]
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disks: Vec<VirtualMachineScaleSetDataDisk>,
}
impl VirtualMachineScaleSetUpdateStorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual machine scale set virtual machine profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetUpdateVmProfile {
    #[doc = "Describes a virtual machine scale set OS profile."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<VirtualMachineScaleSetUpdateOsProfile>,
    #[doc = "Describes a virtual machine scale set storage profile."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<VirtualMachineScaleSetUpdateStorageProfile>,
    #[doc = "Describes a virtual machine scale set network profile."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<VirtualMachineScaleSetUpdateNetworkProfile>,
    #[doc = "Specifies the Security profile settings for the virtual machine or virtual machine scale set."]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<SecurityProfile>,
    #[doc = "Specifies the boot diagnostic settings state. <br><br>Minimum api-version: 2015-06-15."]
    #[serde(rename = "diagnosticsProfile", default, skip_serializing_if = "Option::is_none")]
    pub diagnostics_profile: Option<DiagnosticsProfile>,
    #[doc = "Describes a virtual machine scale set extension profile."]
    #[serde(rename = "extensionProfile", default, skip_serializing_if = "Option::is_none")]
    pub extension_profile: Option<VirtualMachineScaleSetExtensionProfile>,
    #[doc = "The license type, which is for bring your own license scenario."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[doc = "Specifies the billing related details of a Azure Spot VM or VMSS. <br><br>Minimum api-version: 2019-03-01."]
    #[serde(rename = "billingProfile", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile: Option<BillingProfile>,
    #[serde(rename = "scheduledEventsProfile", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_events_profile: Option<ScheduledEventsProfile>,
    #[doc = "UserData for the VM, which must be base-64 encoded. Customer should not pass any secrets in here. <br><br>Minimum api-version: 2021-03-01"]
    #[serde(rename = "userData", default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}
impl VirtualMachineScaleSetUpdateVmProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual machine scale set virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSetVm {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The virtual machine instance ID."]
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[doc = "Describes a virtual machine scale set sku. NOTE: If the new VM SKU is not supported on the hardware the scale set is currently on, you need to deallocate the VMs in the scale set before you modify the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Describes the properties of a virtual machine scale set virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineScaleSetVmProperties>,
    #[doc = "Specifies information about the marketplace image used to create the virtual machine. This element is only used for marketplace images. Before you can use a marketplace image from an API, you must enable the image for programmatic use.  In the Azure portal, find the marketplace image that you want to use and then click **Want to deploy programmatically, Get Started ->**. Enter any required information and then click **Save**."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[doc = "The virtual machine child extension resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<VirtualMachineExtension>,
    #[doc = "The virtual machine zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "Identity for the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<VirtualMachineIdentity>,
}
impl VirtualMachineScaleSetVm {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            instance_id: None,
            sku: None,
            properties: None,
            plan: None,
            resources: Vec::new(),
            zones: Vec::new(),
            identity: None,
        }
    }
}
#[doc = "Describes a VMSS VM Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetVmExtension {
    #[serde(flatten)]
    pub sub_resource_read_only: SubResourceReadOnly,
    #[doc = "The name of the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Describes the properties of a Virtual Machine Extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineExtensionProperties>,
}
impl VirtualMachineScaleSetVmExtension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a VMSS VM Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetVmExtensionUpdate {
    #[serde(flatten)]
    pub sub_resource_read_only: SubResourceReadOnly,
    #[doc = "The name of the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Describes the properties of a Virtual Machine Extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineExtensionUpdateProperties>,
}
impl VirtualMachineScaleSetVmExtensionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List VMSS VM Extension operation response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetVmExtensionsListResult {
    #[doc = "The list of VMSS VM extensions"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualMachineScaleSetVmExtension>,
}
impl VirtualMachineScaleSetVmExtensionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Extensions summary for virtual machines of a virtual machine scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetVmExtensionsSummary {
    #[doc = "The extension name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The extensions information."]
    #[serde(rename = "statusesSummary", default, skip_serializing_if = "Vec::is_empty")]
    pub statuses_summary: Vec<VirtualMachineStatusCodeCount>,
}
impl VirtualMachineScaleSetVmExtensionsSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies a list of virtual machine instance IDs from the VM scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetVmInstanceIDs {
    #[doc = "The virtual machine scale set instance ids. Omitting the virtual machine scale set instance ids will result in the operation being performed on all virtual machines in the virtual machine scale set."]
    #[serde(rename = "instanceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub instance_ids: Vec<String>,
}
impl VirtualMachineScaleSetVmInstanceIDs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies a list of virtual machine instance IDs from the VM scale set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSetVmInstanceRequiredIDs {
    #[doc = "The virtual machine scale set instance ids."]
    #[serde(rename = "instanceIds")]
    pub instance_ids: Vec<String>,
}
impl VirtualMachineScaleSetVmInstanceRequiredIDs {
    pub fn new(instance_ids: Vec<String>) -> Self {
        Self { instance_ids }
    }
}
#[doc = "The instance view of a virtual machine scale set VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetVmInstanceView {
    #[doc = "The Update Domain count."]
    #[serde(rename = "platformUpdateDomain", default, skip_serializing_if = "Option::is_none")]
    pub platform_update_domain: Option<i32>,
    #[doc = "The Fault Domain count."]
    #[serde(rename = "platformFaultDomain", default, skip_serializing_if = "Option::is_none")]
    pub platform_fault_domain: Option<i32>,
    #[doc = "The Remote desktop certificate thumbprint."]
    #[serde(rename = "rdpThumbPrint", default, skip_serializing_if = "Option::is_none")]
    pub rdp_thumb_print: Option<String>,
    #[doc = "The instance view of the VM Agent running on the virtual machine."]
    #[serde(rename = "vmAgent", default, skip_serializing_if = "Option::is_none")]
    pub vm_agent: Option<VirtualMachineAgentInstanceView>,
    #[doc = "Maintenance Operation Status."]
    #[serde(rename = "maintenanceRedeployStatus", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_redeploy_status: Option<MaintenanceRedeployStatus>,
    #[doc = "The disks information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<DiskInstanceView>,
    #[doc = "The extensions information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<VirtualMachineExtensionInstanceView>,
    #[doc = "The health status of the VM."]
    #[serde(rename = "vmHealth", default, skip_serializing_if = "Option::is_none")]
    pub vm_health: Option<VirtualMachineHealthStatus>,
    #[doc = "The instance view of a virtual machine boot diagnostics."]
    #[serde(rename = "bootDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub boot_diagnostics: Option<BootDiagnosticsInstanceView>,
    #[doc = "The resource status information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<InstanceViewStatus>,
    #[doc = "Resource id of the dedicated host, on which the virtual machine is allocated through automatic placement, when the virtual machine is associated with a dedicated host group that has automatic placement enabled. <br><br>Minimum api-version: 2020-06-01."]
    #[serde(rename = "assignedHost", default, skip_serializing_if = "Option::is_none")]
    pub assigned_host: Option<String>,
    #[doc = "The placement group in which the VM is running. If the VM is deallocated it will not have a placementGroupId."]
    #[serde(rename = "placementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub placement_group_id: Option<String>,
}
impl VirtualMachineScaleSetVmInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Virtual Machine Scale Set VMs operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineScaleSetVmListResult {
    #[doc = "The list of virtual machine scale sets VMs."]
    pub value: Vec<VirtualMachineScaleSetVm>,
    #[doc = "The uri to fetch the next page of Virtual Machine Scale Set VMs. Call ListNext() with this to fetch the next page of VMSS VMs"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualMachineScaleSetVmListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualMachineScaleSetVmListResult {
    pub fn new(value: Vec<VirtualMachineScaleSetVm>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes a virtual machine scale set VM network profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetVmNetworkProfileConfiguration {
    #[doc = "The list of network configurations."]
    #[serde(rename = "networkInterfaceConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interface_configurations: Vec<VirtualMachineScaleSetNetworkConfiguration>,
}
impl VirtualMachineScaleSetVmNetworkProfileConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a virtual machine scale set virtual machine profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetVmProfile {
    #[doc = "Describes a virtual machine scale set OS profile."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<VirtualMachineScaleSetOsProfile>,
    #[doc = "Describes a virtual machine scale set storage profile."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<VirtualMachineScaleSetStorageProfile>,
    #[doc = "Describes a virtual machine scale set network profile."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<VirtualMachineScaleSetNetworkProfile>,
    #[doc = "Specifies the Security profile settings for the virtual machine or virtual machine scale set."]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<SecurityProfile>,
    #[doc = "Specifies the boot diagnostic settings state. <br><br>Minimum api-version: 2015-06-15."]
    #[serde(rename = "diagnosticsProfile", default, skip_serializing_if = "Option::is_none")]
    pub diagnostics_profile: Option<DiagnosticsProfile>,
    #[doc = "Describes a virtual machine scale set extension profile."]
    #[serde(rename = "extensionProfile", default, skip_serializing_if = "Option::is_none")]
    pub extension_profile: Option<VirtualMachineScaleSetExtensionProfile>,
    #[doc = "Specifies that the image or disk that is being used was licensed on-premises. <br><br> Possible values for Windows Server operating system are: <br><br> Windows_Client <br><br> Windows_Server <br><br> Possible values for Linux Server operating system are: <br><br> RHEL_BYOS (for RHEL) <br><br> SLES_BYOS (for SUSE) <br><br> For more information, see [Azure Hybrid Use Benefit for Windows Server](https://docs.microsoft.com/azure/virtual-machines/windows/hybrid-use-benefit-licensing) <br><br> [Azure Hybrid Use Benefit for Linux Server](https://docs.microsoft.com/azure/virtual-machines/linux/azure-hybrid-benefit-linux) <br><br> Minimum api-version: 2015-06-15"]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[doc = "Specifies the priority for a standalone virtual machine or the virtual machines in the scale set. <br><br> 'Low' enum will be deprecated in the future, please use 'Spot' as the enum to deploy Azure Spot VM/VMSS."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[doc = "Specifies the eviction policy for the Azure Spot VM/VMSS"]
    #[serde(rename = "evictionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub eviction_policy: Option<EvictionPolicy>,
    #[doc = "Specifies the billing related details of a Azure Spot VM or VMSS. <br><br>Minimum api-version: 2019-03-01."]
    #[serde(rename = "billingProfile", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile: Option<BillingProfile>,
    #[serde(rename = "scheduledEventsProfile", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_events_profile: Option<ScheduledEventsProfile>,
    #[doc = "UserData for the virtual machines in the scale set, which must be base-64 encoded. Customer should not pass any secrets in here. <br><br>Minimum api-version: 2021-03-01"]
    #[serde(rename = "userData", default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[doc = "The parameters of a capacity reservation Profile."]
    #[serde(rename = "capacityReservation", default, skip_serializing_if = "Option::is_none")]
    pub capacity_reservation: Option<CapacityReservationProfile>,
    #[doc = "Contains the list of gallery applications that should be made available to the VM/VMSS"]
    #[serde(rename = "applicationProfile", default, skip_serializing_if = "Option::is_none")]
    pub application_profile: Option<ApplicationProfile>,
    #[doc = "Specifies the hardware settings for the virtual machine scale set."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<VirtualMachineScaleSetHardwareProfile>,
}
impl VirtualMachineScaleSetVmProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a virtual machine scale set virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetVmProperties {
    #[doc = "Specifies whether the latest model has been applied to the virtual machine."]
    #[serde(rename = "latestModelApplied", default, skip_serializing_if = "Option::is_none")]
    pub latest_model_applied: Option<bool>,
    #[doc = "Azure VM unique ID."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "The instance view of a virtual machine scale set VM."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<VirtualMachineScaleSetVmInstanceView>,
    #[doc = "Specifies the hardware settings for the virtual machine."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[doc = "Specifies the storage settings for the virtual machine disks."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "Enables or disables a capability on the virtual machine or virtual machine scale set."]
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub additional_capabilities: Option<AdditionalCapabilities>,
    #[doc = "Specifies the operating system settings for the virtual machine. Some of the settings cannot be changed once VM is provisioned."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "Specifies the Security profile settings for the virtual machine or virtual machine scale set."]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<SecurityProfile>,
    #[doc = "Specifies the network interfaces or the networking configuration of the virtual machine."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[doc = "Describes a virtual machine scale set VM network profile."]
    #[serde(rename = "networkProfileConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub network_profile_configuration: Option<VirtualMachineScaleSetVmNetworkProfileConfiguration>,
    #[doc = "Specifies the boot diagnostic settings state. <br><br>Minimum api-version: 2015-06-15."]
    #[serde(rename = "diagnosticsProfile", default, skip_serializing_if = "Option::is_none")]
    pub diagnostics_profile: Option<DiagnosticsProfile>,
    #[serde(rename = "availabilitySet", default, skip_serializing_if = "Option::is_none")]
    pub availability_set: Option<SubResource>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Specifies that the image or disk that is being used was licensed on-premises. <br><br> Possible values for Windows Server operating system are: <br><br> Windows_Client <br><br> Windows_Server <br><br> Possible values for Linux Server operating system are: <br><br> RHEL_BYOS (for RHEL) <br><br> SLES_BYOS (for SUSE) <br><br> For more information, see [Azure Hybrid Use Benefit for Windows Server](https://docs.microsoft.com/azure/virtual-machines/windows/hybrid-use-benefit-licensing) <br><br> [Azure Hybrid Use Benefit for Linux Server](https://docs.microsoft.com/azure/virtual-machines/linux/azure-hybrid-benefit-linux) <br><br> Minimum api-version: 2015-06-15"]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[doc = "Specifies whether the model applied to the virtual machine is the model of the virtual machine scale set or the customized model for the virtual machine."]
    #[serde(rename = "modelDefinitionApplied", default, skip_serializing_if = "Option::is_none")]
    pub model_definition_applied: Option<String>,
    #[doc = "The protection policy of a virtual machine scale set VM."]
    #[serde(rename = "protectionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub protection_policy: Option<VirtualMachineScaleSetVmProtectionPolicy>,
    #[doc = "UserData for the VM, which must be base-64 encoded. Customer should not pass any secrets in here. <br><br>Minimum api-version: 2021-03-01"]
    #[serde(rename = "userData", default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}
impl VirtualMachineScaleSetVmProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The protection policy of a virtual machine scale set VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetVmProtectionPolicy {
    #[doc = "Indicates that the virtual machine scale set VM shouldn't be considered for deletion during a scale-in operation."]
    #[serde(rename = "protectFromScaleIn", default, skip_serializing_if = "Option::is_none")]
    pub protect_from_scale_in: Option<bool>,
    #[doc = "Indicates that model updates or actions (including scale-in) initiated on the virtual machine scale set should not be applied to the virtual machine scale set VM."]
    #[serde(rename = "protectFromScaleSetActions", default, skip_serializing_if = "Option::is_none")]
    pub protect_from_scale_set_actions: Option<bool>,
}
impl VirtualMachineScaleSetVmProtectionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Virtual Machine Scale Set VM Reimage Parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSetVmReimageParameters {
    #[serde(flatten)]
    pub virtual_machine_reimage_parameters: VirtualMachineReimageParameters,
}
impl VirtualMachineScaleSetVmReimageParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a VM size."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineSize {
    #[doc = "The name of the virtual machine size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The number of cores supported by the virtual machine size. For Constrained vCPU capable VM sizes, this number represents the total vCPUs of quota that the VM uses. For accurate vCPU count, please refer to https://docs.microsoft.com/azure/virtual-machines/constrained-vcpu or https://docs.microsoft.com/rest/api/compute/resourceskus/list"]
    #[serde(rename = "numberOfCores", default, skip_serializing_if = "Option::is_none")]
    pub number_of_cores: Option<i32>,
    #[doc = "The OS disk size, in MB, allowed by the virtual machine size."]
    #[serde(rename = "osDiskSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_size_in_mb: Option<i32>,
    #[doc = "The resource disk size, in MB, allowed by the virtual machine size."]
    #[serde(rename = "resourceDiskSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub resource_disk_size_in_mb: Option<i32>,
    #[doc = "The amount of memory, in MB, supported by the virtual machine size."]
    #[serde(rename = "memoryInMB", default, skip_serializing_if = "Option::is_none")]
    pub memory_in_mb: Option<i32>,
    #[doc = "The maximum number of data disks that can be attached to the virtual machine size."]
    #[serde(rename = "maxDataDiskCount", default, skip_serializing_if = "Option::is_none")]
    pub max_data_disk_count: Option<i32>,
}
impl VirtualMachineSize {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Virtual Machine operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineSizeListResult {
    #[doc = "The list of virtual machine sizes."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualMachineSize>,
}
impl azure_core::Continuable for VirtualMachineSizeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl VirtualMachineSizeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Virtual Machine software patch."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineSoftwarePatchProperties {
    #[doc = "A unique identifier for the patch."]
    #[serde(rename = "patchId", default, skip_serializing_if = "Option::is_none")]
    pub patch_id: Option<String>,
    #[doc = "The friendly name of the patch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The version number of the patch. This property applies only to Linux patches."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The KBID of the patch. Only applies to Windows patches."]
    #[serde(rename = "kbId", default, skip_serializing_if = "Option::is_none")]
    pub kb_id: Option<String>,
    #[doc = "The classification(s) of the patch as provided by the patch publisher."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifications: Vec<String>,
    #[doc = "Describes the reboot requirements of the patch."]
    #[serde(rename = "rebootBehavior", default, skip_serializing_if = "Option::is_none")]
    pub reboot_behavior: Option<virtual_machine_software_patch_properties::RebootBehavior>,
    #[doc = "The activity ID of the operation that produced this result. It is used to correlate across CRP and extension logs."]
    #[serde(rename = "activityId", default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[doc = "The UTC timestamp when the repository published this patch."]
    #[serde(rename = "publishedDate", default, with = "azure_core::date::rfc3339::option")]
    pub published_date: Option<time::OffsetDateTime>,
    #[doc = "The UTC timestamp of the last update to this patch record."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "Describes the availability of a given patch."]
    #[serde(rename = "assessmentState", default, skip_serializing_if = "Option::is_none")]
    pub assessment_state: Option<virtual_machine_software_patch_properties::AssessmentState>,
}
impl VirtualMachineSoftwarePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_software_patch_properties {
    use super::*;
    #[doc = "Describes the reboot requirements of the patch."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RebootBehavior")]
    pub enum RebootBehavior {
        Unknown,
        NeverReboots,
        AlwaysRequiresReboot,
        CanRequestReboot,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RebootBehavior {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RebootBehavior {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RebootBehavior {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("RebootBehavior", 0u32, "Unknown"),
                Self::NeverReboots => serializer.serialize_unit_variant("RebootBehavior", 1u32, "NeverReboots"),
                Self::AlwaysRequiresReboot => serializer.serialize_unit_variant("RebootBehavior", 2u32, "AlwaysRequiresReboot"),
                Self::CanRequestReboot => serializer.serialize_unit_variant("RebootBehavior", 3u32, "CanRequestReboot"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Describes the availability of a given patch."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AssessmentState")]
    pub enum AssessmentState {
        Unknown,
        Available,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AssessmentState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AssessmentState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AssessmentState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("AssessmentState", 0u32, "Unknown"),
                Self::Available => serializer.serialize_unit_variant("AssessmentState", 1u32, "Available"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The status code and count of the virtual machine scale set instance view status summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineStatusCodeCount {
    #[doc = "The instance view status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The number of instances having a particular status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
impl VirtualMachineStatusCodeCount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Virtual Machine Update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "Specifies information about the marketplace image used to create the virtual machine. This element is only used for marketplace images. Before you can use a marketplace image from an API, you must enable the image for programmatic use.  In the Azure portal, find the marketplace image that you want to use and then click **Want to deploy programmatically, Get Started ->**. Enter any required information and then click **Save**."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[doc = "Describes the properties of a Virtual Machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineProperties>,
    #[doc = "Identity for the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<VirtualMachineIdentity>,
    #[doc = "The virtual machine zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl VirtualMachineUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List VmImages in EdgeZone operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmImagesInEdgeZoneListResult {
    #[doc = "The list of VMImages in EdgeZone"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualMachineImageResource>,
    #[doc = "The URI to fetch the next page of VMImages in EdgeZone. Call ListNext() with this URI to fetch the next page of VmImages."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl VmImagesInEdgeZoneListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes Windows Remote Management configuration of the VM"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WinRmConfiguration {
    #[doc = "The list of Windows Remote Management listeners"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub listeners: Vec<WinRmListener>,
}
impl WinRmConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes Protocol and thumbprint of Windows Remote Management listener"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WinRmListener {
    #[doc = "Specifies the protocol of WinRM listener. <br><br> Possible values are: <br>**http** <br><br> **https**"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<win_rm_listener::Protocol>,
    #[doc = "This is the URL of a certificate that has been uploaded to Key Vault as a secret. For adding a secret to the Key Vault, see [Add a key or secret to the key vault](https://docs.microsoft.com/azure/key-vault/key-vault-get-started/#add). In this case, your certificate needs to be It is the Base64 encoding of the following JSON Object which is encoded in UTF-8: <br><br> {<br>  \"data\":\"<Base64-encoded-certificate>\",<br>  \"dataType\":\"pfx\",<br>  \"password\":\"<pfx-file-password>\"<br>} <br> To install certificates on a virtual machine it is recommended to use the [Azure Key Vault virtual machine extension for Linux](https://docs.microsoft.com/azure/virtual-machines/extensions/key-vault-linux) or the [Azure Key Vault virtual machine extension for Windows](https://docs.microsoft.com/azure/virtual-machines/extensions/key-vault-windows)."]
    #[serde(rename = "certificateUrl", default, skip_serializing_if = "Option::is_none")]
    pub certificate_url: Option<String>,
}
impl WinRmListener {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod win_rm_listener {
    use super::*;
    #[doc = "Specifies the protocol of WinRM listener. <br><br> Possible values are: <br>**http** <br><br> **https**"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        Http,
        Https,
    }
}
#[doc = "Specifies Windows operating system settings on the virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsConfiguration {
    #[doc = "Indicates whether virtual machine agent should be provisioned on the virtual machine. <br><br> When this property is not specified in the request body, default behavior is to set it to true.  This will ensure that VM Agent is installed on the VM so that extensions can be added to the VM later."]
    #[serde(rename = "provisionVMAgent", default, skip_serializing_if = "Option::is_none")]
    pub provision_vm_agent: Option<bool>,
    #[doc = "Indicates whether Automatic Updates is enabled for the Windows virtual machine. Default value is true. <br><br> For virtual machine scale sets, this property can be updated and updates will take effect on OS reprovisioning."]
    #[serde(rename = "enableAutomaticUpdates", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_updates: Option<bool>,
    #[doc = "Specifies the time zone of the virtual machine. e.g. \"Pacific Standard Time\". <br><br> Possible values can be [TimeZoneInfo.Id](https://docs.microsoft.com/dotnet/api/system.timezoneinfo.id?#System_TimeZoneInfo_Id) value from time zones returned by [TimeZoneInfo.GetSystemTimeZones](https://docs.microsoft.com/dotnet/api/system.timezoneinfo.getsystemtimezones)."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "Specifies additional base-64 encoded XML formatted information that can be included in the Unattend.xml file, which is used by Windows Setup."]
    #[serde(rename = "additionalUnattendContent", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_unattend_content: Vec<AdditionalUnattendContent>,
    #[doc = "Specifies settings related to VM Guest Patching on Windows."]
    #[serde(rename = "patchSettings", default, skip_serializing_if = "Option::is_none")]
    pub patch_settings: Option<PatchSettings>,
    #[doc = "Describes Windows Remote Management configuration of the VM"]
    #[serde(rename = "winRM", default, skip_serializing_if = "Option::is_none")]
    pub win_rm: Option<WinRmConfiguration>,
}
impl WindowsConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input for InstallPatches on a Windows VM, as directly received by the API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsParameters {
    #[doc = "The update classifications to select when installing patches for Windows."]
    #[serde(rename = "classificationsToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub classifications_to_include: Vec<String>,
    #[doc = "Kbs to include in the patch operation"]
    #[serde(rename = "kbNumbersToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub kb_numbers_to_include: Vec<String>,
    #[doc = "Kbs to exclude in the patch operation"]
    #[serde(rename = "kbNumbersToExclude", default, skip_serializing_if = "Vec::is_empty")]
    pub kb_numbers_to_exclude: Vec<String>,
    #[doc = "Filters out Kbs that don't have an InstallationRebootBehavior of 'NeverReboots' when this is set to true."]
    #[serde(rename = "excludeKbsRequiringReboot", default, skip_serializing_if = "Option::is_none")]
    pub exclude_kbs_requiring_reboot: Option<bool>,
    #[doc = "This is used to install patches that were published on or before this given max published date."]
    #[serde(rename = "maxPatchPublishDate", default, with = "azure_core::date::rfc3339::option")]
    pub max_patch_publish_date: Option<time::OffsetDateTime>,
}
impl WindowsParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies additional settings to be applied when patch mode AutomaticByPlatform is selected in Windows patch settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsVmGuestPatchAutomaticByPlatformSettings {
    #[doc = "Specifies the reboot setting for all AutomaticByPlatform patch installation operations."]
    #[serde(rename = "rebootSetting", default, skip_serializing_if = "Option::is_none")]
    pub reboot_setting: Option<windows_vm_guest_patch_automatic_by_platform_settings::RebootSetting>,
}
impl WindowsVmGuestPatchAutomaticByPlatformSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod windows_vm_guest_patch_automatic_by_platform_settings {
    use super::*;
    #[doc = "Specifies the reboot setting for all AutomaticByPlatform patch installation operations."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RebootSetting")]
    pub enum RebootSetting {
        Unknown,
        IfRequired,
        Never,
        Always,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RebootSetting {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RebootSetting {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RebootSetting {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("RebootSetting", 0u32, "Unknown"),
                Self::IfRequired => serializer.serialize_unit_variant("RebootSetting", 1u32, "IfRequired"),
                Self::Never => serializer.serialize_unit_variant("RebootSetting", 2u32, "Never"),
                Self::Always => serializer.serialize_unit_variant("RebootSetting", 3u32, "Always"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the eviction policy for the Azure Spot VM/VMSS"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EvictionPolicy")]
pub enum EvictionPolicy {
    Deallocate,
    Delete,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EvictionPolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EvictionPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EvictionPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Deallocate => serializer.serialize_unit_variant("EvictionPolicy", 0u32, "Deallocate"),
            Self::Delete => serializer.serialize_unit_variant("EvictionPolicy", 1u32, "Delete"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Specifies the priority for a standalone virtual machine or the virtual machines in the scale set. <br><br> 'Low' enum will be deprecated in the future, please use 'Spot' as the enum to deploy Azure Spot VM/VMSS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Priority")]
pub enum Priority {
    Regular,
    Low,
    Spot,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Priority {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Priority {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Priority {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Regular => serializer.serialize_unit_variant("Priority", 0u32, "Regular"),
            Self::Low => serializer.serialize_unit_variant("Priority", 1u32, "Low"),
            Self::Spot => serializer.serialize_unit_variant("Priority", 2u32, "Spot"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
