#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The reference to the Azure stack edge device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStackEdgeFormat {
    #[serde(flatten)]
    pub device_properties_format: DevicePropertiesFormat,
    #[doc = "Reference to another sub resource."]
    #[serde(rename = "azureStackEdge")]
    pub azure_stack_edge: SubResource,
}
impl AzureStackEdgeFormat {
    pub fn new(device_properties_format: DevicePropertiesFormat, azure_stack_edge: SubResource) -> Self {
        Self {
            device_properties_format,
            azure_stack_edge,
        }
    }
}
#[doc = "Specifies the custom settings for the virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomProfile {
    #[doc = "Path for metadata configuration."]
    #[serde(rename = "metadataConfigurationPath", default, skip_serializing_if = "Option::is_none")]
    pub metadata_configuration_path: Option<String>,
}
impl CustomProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information about the operating system disk used by the virtual machine. <br><br> For more information about disks, see [About disks and VHDs for Azure virtual machines](https://docs.microsoft.com/azure/virtual-machines/virtual-machines-windows-about-disks-vhds?toc=%2fazure%2fvirtual-machines%2fwindows%2ftoc.json)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataDisk {
    #[doc = "Specifies how the virtual machine should be created."]
    #[serde(rename = "createOption", default, skip_serializing_if = "Option::is_none")]
    pub create_option: Option<data_disk::CreateOption>,
    #[doc = "The name of data disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the size of an empty disk in gigabytes. This element can be used to overwrite the size of the disk in a virtual machine image."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
}
impl DataDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_disk {
    use super::*;
    #[doc = "Specifies how the virtual machine should be created."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateOption")]
    pub enum CreateOption {
        Unknown,
        Empty,
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
                Self::Unknown => serializer.serialize_unit_variant("CreateOption", 0u32, "Unknown"),
                Self::Empty => serializer.serialize_unit_variant("CreateOption", 1u32, "Empty"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Device resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Device properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DevicePropertiesFormat>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Device {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
        }
    }
}
#[doc = "Response for devices API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceListResult {
    #[doc = "A list of devices."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Device>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Device properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevicePropertiesFormat {
    #[doc = "The current device status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<device_properties_format::Status>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The type of the device."]
    #[serde(rename = "deviceType")]
    pub device_type: device_properties_format::DeviceType,
    #[doc = "The list of network functions deployed on the device."]
    #[serde(rename = "networkFunctions", default, skip_serializing_if = "Vec::is_empty")]
    pub network_functions: Vec<SubResource>,
}
impl DevicePropertiesFormat {
    pub fn new(device_type: device_properties_format::DeviceType) -> Self {
        Self {
            status: None,
            provisioning_state: None,
            device_type,
            network_functions: Vec::new(),
        }
    }
}
pub mod device_properties_format {
    use super::*;
    #[doc = "The current device status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Unknown,
        NotRegistered,
        Registered,
        Deleted,
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
                Self::NotRegistered => serializer.serialize_unit_variant("Status", 1u32, "NotRegistered"),
                Self::Registered => serializer.serialize_unit_variant("Status", 2u32, "Registered"),
                Self::Deleted => serializer.serialize_unit_variant("Status", 3u32, "Deleted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of the device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeviceType")]
    pub enum DeviceType {
        Unknown,
        AzureStackEdge,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeviceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeviceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeviceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("DeviceType", 0u32, "Unknown"),
                Self::AzureStackEdge => serializer.serialize_unit_variant("DeviceType", 1u32, "AzureStackEdge"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The device registration key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceRegistrationKey {
    #[doc = "The registration key for the device."]
    #[serde(rename = "registrationKey", default, skip_serializing_if = "Option::is_none")]
    pub registration_key: Option<String>,
}
impl DeviceRegistrationKey {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Payload for execute request post call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecuteRequestParameters {
    #[doc = "The endpoint of service to call."]
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
    #[doc = "Request metadata of execute request post call payload."]
    #[serde(rename = "requestMetadata")]
    pub request_metadata: RequestMetadata,
}
impl ExecuteRequestParameters {
    pub fn new(service_endpoint: String, request_metadata: RequestMetadata) -> Self {
        Self {
            service_endpoint,
            request_metadata,
        }
    }
}
#[doc = "The image reference properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageReference {
    #[doc = "The image publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the offer of the image used to create the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "The image SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "Specifies the version of the image used to create the virtual machine. The allowed formats are Major.Minor.Build or 'latest'. Major, Minor, and Build are decimal numbers. Specify 'latest' to use the latest version of an image available at deploy time. Even if you use 'latest', the VM image will not automatically update after deploy time even if a new version becomes available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Specifies in decimal numbers, the exact version of image used to create the virtual machine."]
    #[serde(rename = "exactVersion", default, skip_serializing_if = "Option::is_none")]
    pub exact_version: Option<String>,
}
impl ImageReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the Linux operating system settings on the virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxConfiguration {
    #[doc = "SSH configuration for Linux based VMs running on Azure"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh: Option<SshConfiguration>,
}
impl LinuxConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for the managed application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedApplicationParameters {}
impl ManagedApplicationParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network function resource response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkFunction {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Network function properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkFunctionPropertiesFormat>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl NetworkFunction {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            etag: None,
            system_data: None,
        }
    }
}
#[doc = "Response for network function API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionListResult {
    #[doc = "A list of network function resources in a subscription or resource group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetworkFunction>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkFunctionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkFunctionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network function properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Reference to another sub resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<SubResource>,
    #[doc = "The sku name for the network function. Once set, it cannot be updated."]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
    #[doc = "Sku type."]
    #[serde(rename = "skuType", default, skip_serializing_if = "Option::is_none")]
    pub sku_type: Option<SkuType>,
    #[doc = "The vendor name for the network function. Once set, it cannot be updated."]
    #[serde(rename = "vendorName", default, skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[doc = "The service key for the network function resource."]
    #[serde(rename = "serviceKey", default, skip_serializing_if = "Option::is_none")]
    pub service_key: Option<String>,
    #[doc = "The current vendor provisioning state."]
    #[serde(rename = "vendorProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub vendor_provisioning_state: Option<VendorProvisioningState>,
    #[doc = "Reference to another sub resource."]
    #[serde(rename = "managedApplication", default, skip_serializing_if = "Option::is_none")]
    pub managed_application: Option<SubResource>,
    #[doc = "The parameters for the managed application."]
    #[serde(rename = "managedApplicationParameters", default, skip_serializing_if = "Option::is_none")]
    pub managed_application_parameters: Option<ManagedApplicationParameters>,
    #[doc = "The network function container configurations from the user."]
    #[serde(
        rename = "networkFunctionContainerConfigurations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_function_container_configurations: Option<serde_json::Value>,
    #[doc = "The network function configurations from the user."]
    #[serde(rename = "networkFunctionUserConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub network_function_user_configurations: Vec<NetworkFunctionUserConfiguration>,
}
impl NetworkFunctionPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network function role configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionRoleConfiguration {
    #[doc = "The name of the network function role."]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "Role type."]
    #[serde(rename = "roleType", default, skip_serializing_if = "Option::is_none")]
    pub role_type: Option<network_function_role_configuration::RoleType>,
    #[doc = "The size of the virtual machine."]
    #[serde(rename = "virtualMachineSize", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_size: Option<network_function_role_configuration::VirtualMachineSize>,
    #[doc = "Specifies the operating system settings for the role instance."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "The user data template."]
    #[serde(rename = "userDataTemplate", default, skip_serializing_if = "Option::is_none")]
    pub user_data_template: Option<UserDataTemplate>,
    #[doc = "The user data parameters."]
    #[serde(rename = "userDataParameters", default, skip_serializing_if = "Option::is_none")]
    pub user_data_parameters: Option<UserDataParameters>,
    #[doc = "The network interface configurations."]
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<NetworkInterface>,
    #[doc = "Specifies the storage settings for the virtual machine disks."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "Specifies the custom settings for the virtual machine."]
    #[serde(rename = "customProfile", default, skip_serializing_if = "Option::is_none")]
    pub custom_profile: Option<CustomProfile>,
}
impl NetworkFunctionRoleConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_function_role_configuration {
    use super::*;
    #[doc = "Role type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RoleType")]
    pub enum RoleType {
        Unknown,
        VirtualMachine,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RoleType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RoleType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RoleType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("RoleType", 0u32, "Unknown"),
                Self::VirtualMachine => serializer.serialize_unit_variant("RoleType", 1u32, "VirtualMachine"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The size of the virtual machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VirtualMachineSize")]
    pub enum VirtualMachineSize {
        Unknown,
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
        #[serde(rename = "Standard_D11_v2")]
        StandardD11V2,
        #[serde(rename = "Standard_D12_v2")]
        StandardD12V2,
        #[serde(rename = "Standard_D13_v2")]
        StandardD13V2,
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
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VirtualMachineSize {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VirtualMachineSize {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VirtualMachineSize {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("VirtualMachineSize", 0u32, "Unknown"),
                Self::StandardD1V2 => serializer.serialize_unit_variant("VirtualMachineSize", 1u32, "Standard_D1_v2"),
                Self::StandardD2V2 => serializer.serialize_unit_variant("VirtualMachineSize", 2u32, "Standard_D2_v2"),
                Self::StandardD3V2 => serializer.serialize_unit_variant("VirtualMachineSize", 3u32, "Standard_D3_v2"),
                Self::StandardD4V2 => serializer.serialize_unit_variant("VirtualMachineSize", 4u32, "Standard_D4_v2"),
                Self::StandardD5V2 => serializer.serialize_unit_variant("VirtualMachineSize", 5u32, "Standard_D5_v2"),
                Self::StandardD11V2 => serializer.serialize_unit_variant("VirtualMachineSize", 6u32, "Standard_D11_v2"),
                Self::StandardD12V2 => serializer.serialize_unit_variant("VirtualMachineSize", 7u32, "Standard_D12_v2"),
                Self::StandardD13V2 => serializer.serialize_unit_variant("VirtualMachineSize", 8u32, "Standard_D13_v2"),
                Self::StandardDs1V2 => serializer.serialize_unit_variant("VirtualMachineSize", 9u32, "Standard_DS1_v2"),
                Self::StandardDs2V2 => serializer.serialize_unit_variant("VirtualMachineSize", 10u32, "Standard_DS2_v2"),
                Self::StandardDs3V2 => serializer.serialize_unit_variant("VirtualMachineSize", 11u32, "Standard_DS3_v2"),
                Self::StandardDs4V2 => serializer.serialize_unit_variant("VirtualMachineSize", 12u32, "Standard_DS4_v2"),
                Self::StandardDs5V2 => serializer.serialize_unit_variant("VirtualMachineSize", 13u32, "Standard_DS5_v2"),
                Self::StandardDs11V2 => serializer.serialize_unit_variant("VirtualMachineSize", 14u32, "Standard_DS11_v2"),
                Self::StandardDs12V2 => serializer.serialize_unit_variant("VirtualMachineSize", 15u32, "Standard_DS12_v2"),
                Self::StandardDs13V2 => serializer.serialize_unit_variant("VirtualMachineSize", 16u32, "Standard_DS13_v2"),
                Self::StandardF1 => serializer.serialize_unit_variant("VirtualMachineSize", 17u32, "Standard_F1"),
                Self::StandardF2 => serializer.serialize_unit_variant("VirtualMachineSize", 18u32, "Standard_F2"),
                Self::StandardF4 => serializer.serialize_unit_variant("VirtualMachineSize", 19u32, "Standard_F4"),
                Self::StandardF8 => serializer.serialize_unit_variant("VirtualMachineSize", 20u32, "Standard_F8"),
                Self::StandardF16 => serializer.serialize_unit_variant("VirtualMachineSize", 21u32, "Standard_F16"),
                Self::StandardF1s => serializer.serialize_unit_variant("VirtualMachineSize", 22u32, "Standard_F1s"),
                Self::StandardF2s => serializer.serialize_unit_variant("VirtualMachineSize", 23u32, "Standard_F2s"),
                Self::StandardF4s => serializer.serialize_unit_variant("VirtualMachineSize", 24u32, "Standard_F4s"),
                Self::StandardF8s => serializer.serialize_unit_variant("VirtualMachineSize", 25u32, "Standard_F8s"),
                Self::StandardF16s => serializer.serialize_unit_variant("VirtualMachineSize", 26u32, "Standard_F16s"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of role instances of vendor network function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionRoleInstanceListResult {
    #[doc = "A list of role instances."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleInstance>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkFunctionRoleInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkFunctionRoleInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network function sku details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionSkuDetails {
    #[doc = "Sku type."]
    #[serde(rename = "skuType", default, skip_serializing_if = "Option::is_none")]
    pub sku_type: Option<SkuType>,
    #[doc = "The network function sku role details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetworkFunctionSkuRoleDetails>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkFunctionSkuDetails {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkFunctionSkuDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of available network function skus."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionSkuListResult {
    #[doc = "The network function vendor sku overview properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuOverview>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkFunctionSkuListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkFunctionSkuListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network function user configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionSkuRoleDetails {
    #[doc = "The name of the network function role."]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "The user data template."]
    #[serde(rename = "userDataTemplate", default, skip_serializing_if = "Option::is_none")]
    pub user_data_template: Option<UserDataTemplate>,
    #[doc = "The user data parameters."]
    #[serde(rename = "userDataParameters", default, skip_serializing_if = "Option::is_none")]
    pub user_data_parameters: Option<UserDataParameters>,
    #[doc = "The network interface configuration."]
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<NetworkInterface>,
}
impl NetworkFunctionSkuRoleDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network function template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionTemplate {
    #[doc = "An array of network function role definitions."]
    #[serde(rename = "networkFunctionRoleConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub network_function_role_configurations: Vec<NetworkFunctionRoleConfiguration>,
}
impl NetworkFunctionTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network function user configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionUserConfiguration {
    #[doc = "The name of the network function role."]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "The user data parameters."]
    #[serde(rename = "userDataParameters", default, skip_serializing_if = "Option::is_none")]
    pub user_data_parameters: Option<UserDataParameters>,
    #[doc = "The network interface configuration."]
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<NetworkInterface>,
    #[doc = "Specifies the operating system settings for the role instance."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<network_function_user_configuration::OsProfile>,
}
impl NetworkFunctionUserConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_function_user_configuration {
    use super::*;
    #[doc = "Specifies the operating system settings for the role instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct OsProfile {
        #[doc = "Specifies a base-64 encoded string of custom data. The base-64 encoded string is decoded to a binary array that is saved as a file on the virtual machine. The maximum length of the binary array is 65535 bytes. <br><br> **Note: Do not pass any secrets or passwords in customData property** <br><br> This property cannot be updated after the VM is created. <br><br> customData is passed to the VM to be saved as a file. For more information see [Custom Data on Azure VMs](https://azure.microsoft.com/en-us/blog/custom-data-and-cloud-init-on-windows-azure/) <br><br> For using cloud-init for your Linux VM, see [Using cloud-init to customize a Linux VM during creation](https://docs.microsoft.com/azure/virtual-machines/virtual-machines-linux-using-cloud-init?toc=%2fazure%2fvirtual-machines%2flinux%2ftoc.json)"]
        #[serde(rename = "customData", default, skip_serializing_if = "Option::is_none")]
        pub custom_data: Option<String>,
    }
    impl OsProfile {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The network function vendor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionVendor {
    #[doc = "The network function vendor details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VendorDetails>,
}
impl NetworkFunctionVendor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network function vendor configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionVendorConfiguration {
    #[doc = "The name of the vendor network function role."]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "Specifies the operating system settings for the role instance."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "The user data parameters."]
    #[serde(rename = "userDataParameters", default, skip_serializing_if = "Option::is_none")]
    pub user_data_parameters: Option<UserDataParameters>,
    #[doc = "The network interface configurations."]
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<NetworkInterface>,
}
impl NetworkFunctionVendorConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network function vendor list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionVendorListResult {
    #[doc = "A list of available network function vendors and skus."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetworkFunctionVendor>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkFunctionVendorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkFunctionVendorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network interface properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterface {
    #[doc = "The name of the network interface."]
    #[serde(rename = "networkInterfaceName", default, skip_serializing_if = "Option::is_none")]
    pub network_interface_name: Option<String>,
    #[doc = "The MAC address of the network interface."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "A list of IP configurations of the network interface."]
    #[serde(rename = "ipConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_configurations: Vec<NetworkInterfaceIpConfiguration>,
    #[doc = "The type of the VM switch."]
    #[serde(rename = "vmSwitchType", default, skip_serializing_if = "Option::is_none")]
    pub vm_switch_type: Option<network_interface::VmSwitchType>,
}
impl NetworkInterface {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_interface {
    use super::*;
    #[doc = "The type of the VM switch."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VmSwitchType")]
    pub enum VmSwitchType {
        Unknown,
        Management,
        Wan,
        Lan,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VmSwitchType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VmSwitchType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VmSwitchType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("VmSwitchType", 0u32, "Unknown"),
                Self::Management => serializer.serialize_unit_variant("VmSwitchType", 1u32, "Management"),
                Self::Wan => serializer.serialize_unit_variant("VmSwitchType", 2u32, "Wan"),
                Self::Lan => serializer.serialize_unit_variant("VmSwitchType", 3u32, "Lan"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Network interface IP configuration properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaceIpConfiguration {
    #[doc = "IP address allocation method."]
    #[serde(rename = "ipAllocationMethod", default, skip_serializing_if = "Option::is_none")]
    pub ip_allocation_method: Option<network_interface_ip_configuration::IpAllocationMethod>,
    #[doc = "The value of the IP address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The value of the subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[doc = "The value of the gateway."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[doc = "IP address version."]
    #[serde(rename = "ipVersion", default, skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<network_interface_ip_configuration::IpVersion>,
    #[doc = "The list of DNS servers IP addresses."]
    #[serde(rename = "dnsServers", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_servers: Vec<String>,
}
impl NetworkInterfaceIpConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_interface_ip_configuration {
    use super::*;
    #[doc = "IP address allocation method."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IpAllocationMethod")]
    pub enum IpAllocationMethod {
        Unknown,
        Static,
        Dynamic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IpAllocationMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IpAllocationMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IpAllocationMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("IpAllocationMethod", 0u32, "Unknown"),
                Self::Static => serializer.serialize_unit_variant("IpAllocationMethod", 1u32, "Static"),
                Self::Dynamic => serializer.serialize_unit_variant("IpAllocationMethod", 2u32, "Dynamic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "IP address version."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IpVersion")]
    pub enum IpVersion {
        Unknown,
        IPv4,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IpVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IpVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IpVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("IpVersion", 0u32, "Unknown"),
                Self::IPv4 => serializer.serialize_unit_variant("IpVersion", 1u32, "IPv4"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Details of a REST API operation, returned from the Resource Provider Operations API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation, as per Resource-Based Access Control (RBAC). Examples: \"Microsoft.Compute/virtualMachines/write\", \"Microsoft.Compute/virtualMachines/capture/action\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Localized display information for this particular operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name, e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly name of the resource type related to this operation. E.g. \"Virtual Machines\" or \"Job Schedule Collections\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The concise, localized friendly name for the operation; suitable for dropdowns. E.g. \"Create or Update Virtual Machine\", \"Restart Virtual Machine\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The short, localized friendly description of the operation; suitable for tool tips and detailed views."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Origin {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Origin {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Origin {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
                Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        Internal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
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
#[doc = "Specifies information about the operating system disk used by the virtual machine. <br><br> For more information about disks, see [About disks and VHDs for Azure virtual machines](https://docs.microsoft.com/azure/virtual-machines/virtual-machines-windows-about-disks-vhds?toc=%2fazure%2fvirtual-machines%2fwindows%2ftoc.json)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsDisk {
    #[doc = "The OS type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<os_disk::OsType>,
    #[doc = "The VHD name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Describes the uri of a disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhd: Option<VirtualHardDisk>,
    #[doc = "Specifies the size of os disk in gigabytes. This is the fully expanded disk size needed of the VHD image on the ASE. This disk size should be greater than the size of the VHD provided in vhdUri."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
}
impl OsDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod os_disk {
    use super::*;
    #[doc = "The OS type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OsType")]
    pub enum OsType {
        Unknown,
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
                Self::Unknown => serializer.serialize_unit_variant("OsType", 0u32, "Unknown"),
                Self::Windows => serializer.serialize_unit_variant("OsType", 1u32, "Windows"),
                Self::Linux => serializer.serialize_unit_variant("OsType", 2u32, "Linux"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the operating system settings for the role instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[doc = "Specifies the name of the administrator account. <br><br> **Windows-only restriction:** Cannot end in \".\" <br><br> **Disallowed values:** \"administrator\", \"admin\", \"user\", \"user1\", \"test\", \"user2\", \"test1\", \"user3\", \"admin1\", \"1\", \"123\", \"a\", \"actuser\", \"adm\", \"admin2\", \"aspnet\", \"backup\", \"console\", \"david\", \"guest\", \"john\", \"owner\", \"root\", \"server\", \"sql\", \"support\", \"support_388945a0\", \"sys\", \"test2\", \"test3\", \"user4\", \"user5\". <br><br> **Minimum-length (Linux):** 1  character <br><br> **Max-length (Linux):** 64 characters <br><br> **Max-length (Windows):** 20 characters  <br><br><li> For root access to the Linux VM, see [Using root privileges on Linux virtual machines in Azure](https://docs.microsoft.com/azure/virtual-machines/virtual-machines-linux-use-root-privileges?toc=%2fazure%2fvirtual-machines%2flinux%2ftoc.json)<br><li> For a list of built-in system users on Linux that should not be used in this field, see [Selecting User Names for Linux on Azure](https://docs.microsoft.com/azure/virtual-machines/virtual-machines-linux-usernames?toc=%2fazure%2fvirtual-machines%2flinux%2ftoc.json)."]
    #[serde(rename = "adminUsername", default, skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,
    #[doc = "Specifies the Linux operating system settings on the virtual machine."]
    #[serde(rename = "linuxConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub linux_configuration: Option<LinuxConfiguration>,
    #[doc = "Specifies a base-64 encoded string of custom data. The base-64 encoded string is decoded to a binary array that is saved as a file on the virtual machine. The maximum length of the binary array is 65535 bytes. <br><br> **Note: Do not pass any secrets or passwords in customData property** <br><br> This property cannot be updated after the VM is created. <br><br> customData is passed to the VM to be saved as a file. For more information see [Custom Data on Azure VMs](https://azure.microsoft.com/en-us/blog/custom-data-and-cloud-init-on-windows-azure/) <br><br> For using cloud-init for your Linux VM, see [Using cloud-init to customize a Linux VM during creation](https://docs.microsoft.com/azure/virtual-machines/virtual-machines-linux-using-cloud-init?toc=%2fazure%2fvirtual-machines%2flinux%2ftoc.json)"]
    #[serde(rename = "customData", default, skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<String>,
    #[doc = "Indicates if custom data is required to deploy this role."]
    #[serde(rename = "customDataRequired", default, skip_serializing_if = "Option::is_none")]
    pub custom_data_required: Option<bool>,
}
impl OsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Customer subscription which can use a sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreviewSubscription {
    #[doc = "The preview subscription ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The ARM ID of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "PreviewSubscription properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PreviewSubscriptionProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PreviewSubscription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "PreviewSubscription properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreviewSubscriptionProperties {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl PreviewSubscriptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of customer subscriptions which can use a sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreviewSubscriptionsList {
    #[doc = "A list of preview subscriptions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PreviewSubscription>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PreviewSubscriptionsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PreviewSubscriptionsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Unknown,
    Succeeded,
    Accepted,
    Deleting,
    Failed,
    Canceled,
    Deleted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Unknown"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Accepted"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
            Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "Request metadata of execute request post call payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestMetadata {
    #[doc = "The relative path of the request."]
    #[serde(rename = "relativePath")]
    pub relative_path: String,
    #[doc = "The http method of the request."]
    #[serde(rename = "httpMethod")]
    pub http_method: request_metadata::HttpMethod,
    #[doc = "The serialized body of the request."]
    #[serde(rename = "serializedBody")]
    pub serialized_body: String,
    #[doc = "The api version of the request."]
    #[serde(rename = "apiVersion", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}
impl RequestMetadata {
    pub fn new(relative_path: String, http_method: request_metadata::HttpMethod, serialized_body: String) -> Self {
        Self {
            relative_path,
            http_method,
            serialized_body,
            api_version: None,
        }
    }
}
pub mod request_metadata {
    use super::*;
    #[doc = "The http method of the request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HttpMethod")]
    pub enum HttpMethod {
        Unknown,
        Post,
        Put,
        Get,
        Patch,
        Delete,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HttpMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HttpMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HttpMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("HttpMethod", 0u32, "Unknown"),
                Self::Post => serializer.serialize_unit_variant("HttpMethod", 1u32, "Post"),
                Self::Put => serializer.serialize_unit_variant("HttpMethod", 2u32, "Put"),
                Self::Get => serializer.serialize_unit_variant("HttpMethod", 3u32, "Get"),
                Self::Patch => serializer.serialize_unit_variant("HttpMethod", 4u32, "Patch"),
                Self::Delete => serializer.serialize_unit_variant("HttpMethod", 5u32, "Delete"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "The role instance sub resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleInstance {
    #[doc = "The role instance name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The ARM ID of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The role instance properties of the network function."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleInstanceProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl RoleInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The role instance properties of the network function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleInstanceProperties {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The operational state of the role instance."]
    #[serde(rename = "operationalState", default, skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<role_instance_properties::OperationalState>,
}
impl RoleInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod role_instance_properties {
    use super::*;
    #[doc = "The operational state of the role instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationalState")]
    pub enum OperationalState {
        Unknown,
        Stopped,
        Running,
        Stopping,
        Starting,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OperationalState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OperationalState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OperationalState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("OperationalState", 0u32, "Unknown"),
                Self::Stopped => serializer.serialize_unit_variant("OperationalState", 1u32, "Stopped"),
                Self::Running => serializer.serialize_unit_variant("OperationalState", 2u32, "Running"),
                Self::Stopping => serializer.serialize_unit_variant("OperationalState", 3u32, "Stopping"),
                Self::Starting => serializer.serialize_unit_variant("OperationalState", 4u32, "Starting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Sku credential definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCredential {
    #[doc = "The username of the sku credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The credential value."]
    #[serde(rename = "acrToken", default, skip_serializing_if = "Option::is_none")]
    pub acr_token: Option<String>,
    #[doc = "The Acr server url"]
    #[serde(rename = "acrServerUrl", default, skip_serializing_if = "Option::is_none")]
    pub acr_server_url: Option<String>,
    #[doc = "The repositories that could be accessed using the current credential."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<String>,
    #[doc = "The UTC time when credential will expire."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expiry: Option<time::OffsetDateTime>,
}
impl SkuCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network function sku overview."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuOverview {
    #[doc = "The vendor sku name."]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
    #[doc = "Sku type."]
    #[serde(rename = "skuType", default, skip_serializing_if = "Option::is_none")]
    pub sku_type: Option<SkuType>,
}
impl SkuOverview {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sku type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuType")]
pub enum SkuType {
    Unknown,
    EvolvedPacketCore,
    #[serde(rename = "SDWAN")]
    Sdwan,
    Firewall,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SkuType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SkuType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SkuType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("SkuType", 0u32, "Unknown"),
            Self::EvolvedPacketCore => serializer.serialize_unit_variant("SkuType", 1u32, "EvolvedPacketCore"),
            Self::Sdwan => serializer.serialize_unit_variant("SkuType", 2u32, "SDWAN"),
            Self::Firewall => serializer.serialize_unit_variant("SkuType", 3u32, "Firewall"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
    #[doc = "SSH public key certificate used to authenticate with the VM through ssh. The key needs to be at least 2048-bit and in ssh-rsa format. <br><br> For creating ssh keys, see [Create SSH keys on Linux and Mac for Linux VMs in Azure](https://docs.microsoft.com/azure/virtual-machines/virtual-machines-linux-mac-create-ssh-keys?toc=%2fazure%2fvirtual-machines%2flinux%2ftoc.json)."]
    #[serde(rename = "keyData", default, skip_serializing_if = "Option::is_none")]
    pub key_data: Option<String>,
}
impl SshPublicKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the storage settings for the virtual machine disks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[doc = "The image reference properties."]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[doc = "Specifies information about the operating system disk used by the virtual machine. <br><br> For more information about disks, see [About disks and VHDs for Azure virtual machines](https://docs.microsoft.com/azure/virtual-machines/virtual-machines-windows-about-disks-vhds?toc=%2fazure%2fvirtual-machines%2fwindows%2ftoc.json)."]
    #[serde(rename = "osDisk", default, skip_serializing_if = "Option::is_none")]
    pub os_disk: Option<OsDisk>,
    #[doc = "Specifies the parameters that are used to add a data disk to a virtual machine."]
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disks: Vec<DataDisk>,
}
impl StorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to another sub resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tags object for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsObject {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsObject {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The user data parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserDataParameters {}
impl UserDataParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The user data template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserDataTemplate {}
impl UserDataTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vendor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Vendor {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Vendor properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VendorPropertiesFormat>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Vendor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network function vendor details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VendorDetails {
    #[doc = "The network function vendor name."]
    #[serde(rename = "vendorName", default, skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[doc = "The network function sku list."]
    #[serde(rename = "skuList", default, skip_serializing_if = "Vec::is_empty")]
    pub sku_list: Vec<SkuOverview>,
}
impl VendorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for vendors API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VendorListResult {
    #[doc = "A list of vendors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Vendor>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VendorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VendorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vendor network function sub resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VendorNetworkFunction {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Vendor network function properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VendorNetworkFunctionPropertiesFormat>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl VendorNetworkFunction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for vendors API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VendorNetworkFunctionListResult {
    #[doc = "A list of vendor network functions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VendorNetworkFunction>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VendorNetworkFunctionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VendorNetworkFunctionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vendor network function properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VendorNetworkFunctionPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The current vendor provisioning state."]
    #[serde(rename = "vendorProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub vendor_provisioning_state: Option<VendorProvisioningState>,
    #[doc = "The name of the sku. Once set, it cannot be updated."]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
    #[doc = "Sku type."]
    #[serde(rename = "skuType", default, skip_serializing_if = "Option::is_none")]
    pub sku_type: Option<SkuType>,
    #[doc = "An array of network function vendor configurations."]
    #[serde(rename = "networkFunctionVendorConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub network_function_vendor_configurations: Vec<NetworkFunctionVendorConfiguration>,
}
impl VendorNetworkFunctionPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vendor properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VendorPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "A list of IDs of the vendor skus offered by the vendor."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skus: Vec<SubResource>,
}
impl VendorPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current vendor provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VendorProvisioningState")]
pub enum VendorProvisioningState {
    Unknown,
    NotProvisioned,
    Provisioning,
    Provisioned,
    Deprovisioned,
    UserDataValidationFailed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VendorProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VendorProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VendorProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("VendorProvisioningState", 0u32, "Unknown"),
            Self::NotProvisioned => serializer.serialize_unit_variant("VendorProvisioningState", 1u32, "NotProvisioned"),
            Self::Provisioning => serializer.serialize_unit_variant("VendorProvisioningState", 2u32, "Provisioning"),
            Self::Provisioned => serializer.serialize_unit_variant("VendorProvisioningState", 3u32, "Provisioned"),
            Self::Deprovisioned => serializer.serialize_unit_variant("VendorProvisioningState", 4u32, "Deprovisioned"),
            Self::UserDataValidationFailed => {
                serializer.serialize_unit_variant("VendorProvisioningState", 5u32, "UserDataValidationFailed")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Sku sub resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VendorSku {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Sku properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VendorSkuPropertiesFormat>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl VendorSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for list vendor sku API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VendorSkuListResult {
    #[doc = "A list of vendor skus offered by the vendor."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VendorSku>,
    #[doc = "The URI to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VendorSkuListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VendorSkuListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sku properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VendorSkuPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Sku type."]
    #[serde(rename = "skuType", default, skip_serializing_if = "Option::is_none")]
    pub sku_type: Option<SkuType>,
    #[doc = "The sku deployment mode."]
    #[serde(rename = "deploymentMode", default, skip_serializing_if = "Option::is_none")]
    pub deployment_mode: Option<vendor_sku_properties_format::DeploymentMode>,
    #[doc = "The network function type."]
    #[serde(rename = "networkFunctionType", default, skip_serializing_if = "Option::is_none")]
    pub network_function_type: Option<vendor_sku_properties_format::NetworkFunctionType>,
    #[doc = "Indicates if the vendor sku is in preview mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    #[doc = "The parameters for the managed application."]
    #[serde(rename = "managedApplicationParameters", default, skip_serializing_if = "Option::is_none")]
    pub managed_application_parameters: Option<ManagedApplicationParameters>,
    #[doc = "The template for the managed application deployment."]
    #[serde(rename = "managedApplicationTemplate", default, skip_serializing_if = "Option::is_none")]
    pub managed_application_template: Option<serde_json::Value>,
    #[doc = "The network function template."]
    #[serde(rename = "networkFunctionTemplate", default, skip_serializing_if = "Option::is_none")]
    pub network_function_template: Option<NetworkFunctionTemplate>,
}
impl VendorSkuPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod vendor_sku_properties_format {
    use super::*;
    #[doc = "The sku deployment mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeploymentMode")]
    pub enum DeploymentMode {
        Unknown,
        Azure,
        PrivateEdgeZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeploymentMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeploymentMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeploymentMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("DeploymentMode", 0u32, "Unknown"),
                Self::Azure => serializer.serialize_unit_variant("DeploymentMode", 1u32, "Azure"),
                Self::PrivateEdgeZone => serializer.serialize_unit_variant("DeploymentMode", 2u32, "PrivateEdgeZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The network function type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NetworkFunctionType")]
    pub enum NetworkFunctionType {
        Unknown,
        VirtualNetworkFunction,
        ContainerizedNetworkFunction,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NetworkFunctionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NetworkFunctionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NetworkFunctionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("NetworkFunctionType", 0u32, "Unknown"),
                Self::VirtualNetworkFunction => serializer.serialize_unit_variant("NetworkFunctionType", 1u32, "VirtualNetworkFunction"),
                Self::ContainerizedNetworkFunction => {
                    serializer.serialize_unit_variant("NetworkFunctionType", 2u32, "ContainerizedNetworkFunction")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
