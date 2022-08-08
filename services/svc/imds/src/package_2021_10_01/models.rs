#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
pub type ApplicationResponse = String;
#[doc = "This is the response from the Attested_GetDocument operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttestedData {
    #[doc = "This is the encoded string containing the VM ID, SKU, plan information, public key, timestamp, and nonce value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[doc = "This is the encoding scheme of the signature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}
impl AttestedData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Compute Metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Compute {
    #[doc = "This is the name of the environment in which the VM is running."]
    #[serde(rename = "azEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub az_environment: Option<String>,
    #[doc = "Describes how the VM will be evicted if space needs to be freed up. Only applicable to Spot VMs. For a non-spot VM, this will be an empty string."]
    #[serde(rename = "evictionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub eviction_policy: Option<String>,
    #[doc = "Describes the extended location of the VM"]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocationProperties>,
    #[doc = "Identifies if the VM runs on the Host Compatibility Layer."]
    #[serde(rename = "isHostCompatibilityLayerVm", default, skip_serializing_if = "Option::is_none")]
    pub is_host_compatibility_layer_vm: Option<String>,
    #[doc = "Type of license for Azure Hybrid Benefit. Note that this is only present for AHB-enabled VMs."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[doc = "This is the Azure Region in which the VM is running."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "This is the name of the VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "This is the offer information for the VM image. This value is only present for images deployed from the Azure Image Gallery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "This contains the data about the OS."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "This value indicates the type of OS the VM is running, either Linux or Windows."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "This is the placement group of your Virtual Machine Scale Set."]
    #[serde(rename = "placementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub placement_group_id: Option<String>,
    #[doc = "This contains the data about the plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanProperties>,
    #[doc = "This is information about the SSH certificate"]
    #[serde(rename = "publicKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub public_keys: Vec<PublicKeysProperties>,
    #[doc = "This is the fault domain the VM resides in."]
    #[serde(rename = "platformFaultDomain", default, skip_serializing_if = "Option::is_none")]
    pub platform_fault_domain: Option<String>,
    #[doc = "This is the sub fault domain the VM resides in, if applicable."]
    #[serde(rename = "platformSubFaultDomain", default, skip_serializing_if = "Option::is_none")]
    pub platform_sub_fault_domain: Option<String>,
    #[doc = "This is the update domain the VM resides in."]
    #[serde(rename = "platformUpdateDomain", default, skip_serializing_if = "Option::is_none")]
    pub platform_update_domain: Option<String>,
    #[doc = "This is the priority of the VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[doc = "This is the provider of the VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "This is the publisher of the VM image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "This is the resource group for the VM."]
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[doc = "This is the fully qualified ID for the VM."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "This contains the data about the security profile associated with the VM."]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<SecurityProfile>,
    #[doc = "This is the specific SKU for the VM image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "This contains the data about the storage disks associated with the VM."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "This is the Azure subscription for the VM."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "This is the list of tags for your VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[doc = "This is the list of tags for your VM formatted as a JSON array for easier programmatic parsing."]
    #[serde(rename = "tagsList", default, skip_serializing_if = "Vec::is_empty")]
    pub tags_list: Vec<TagsProperties>,
    #[doc = "The set of data specified when the VM was created for use during or after provisioning (Base64 encoded)"]
    #[serde(rename = "userData", default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[doc = "This is the version of the VM image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Information about the VMSS the VM belongs to (if applicable)"]
    #[serde(rename = "virtualMachineScaleSet", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_scale_set: Option<VirtualMachineScaleSet>,
    #[doc = "This is the unique identifier for the VM."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "This is the resource name of the VMSS."]
    #[serde(rename = "vmScaleSetName", default, skip_serializing_if = "Option::is_none")]
    pub vm_scale_set_name: Option<String>,
    #[doc = "This is the size of the VM."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
    #[doc = "This is the availability zone of the VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}
impl Compute {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This contains information about the data disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataDisk {
    #[doc = "Disk read/write quota in bytes; only populated for Ultra Disks"]
    #[serde(rename = "bytesPerSecondThrottle", default, skip_serializing_if = "Option::is_none")]
    pub bytes_per_second_throttle: Option<String>,
    #[doc = "This is the caching requirements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<String>,
    #[doc = "This is information about how the VM was created."]
    #[serde(rename = "createOption", default, skip_serializing_if = "Option::is_none")]
    pub create_option: Option<String>,
    #[doc = "Size of disk in bytes; only populated for Ultra Disks"]
    #[serde(rename = "diskCapacityBytes", default, skip_serializing_if = "Option::is_none")]
    pub disk_capacity_bytes: Option<String>,
    #[doc = "This is the size of the disk in GB."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<String>,
    #[doc = "This is the source user image virtual hard disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<DiskImage>,
    #[doc = "Identifies if the disk is shared between resources; only populated for Ultra Disks"]
    #[serde(rename = "isSharedDisk", default, skip_serializing_if = "Option::is_none")]
    pub is_shared_disk: Option<String>,
    #[doc = "Identifies if the data disk is an Ultra Disk"]
    #[serde(rename = "isUltraDisk", default, skip_serializing_if = "Option::is_none")]
    pub is_ultra_disk: Option<String>,
    #[doc = "This is the logical unit number of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<String>,
    #[doc = "This is managed disk parameters."]
    #[serde(rename = "managedDisk", default, skip_serializing_if = "Option::is_none")]
    pub managed_disk: Option<ManagedDisk>,
    #[doc = "This is the disk name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Disk read/write quota in IOPS; only populated for Ultra Disks"]
    #[serde(rename = "opsPerSecondThrottle", default, skip_serializing_if = "Option::is_none")]
    pub ops_per_second_throttle: Option<String>,
    #[doc = "This is the virtual hard disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhd: Option<VirtualHardDisk>,
    #[doc = "This specifies whether or not writeAccelerator is enabled on the disk."]
    #[serde(rename = "writeAcceleratorEnabled", default, skip_serializing_if = "Option::is_none")]
    pub write_accelerator_enabled: Option<String>,
}
impl DataDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the ephemeral disk settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiffDiskSettings {
    #[doc = "This specifies the ephemeral disk settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option: Option<String>,
}
impl DiffDiskSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the source user image virtual hard disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskImage {
    #[doc = "This is the uri of the virtual hard disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl DiskImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the encryption settings for the disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionSettings {
    #[doc = "This specifies whether or not disk encryption is enabled on the VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
}
impl EncryptionSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the response from an operation in the case an error occurs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the extended location of the VM"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocationProperties {
    #[doc = "The type of the extended location of the VM"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the extended location of the VM"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ExtendedLocationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the response from an Identity operation in the case an error occurs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityErrorResponse {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<identity_error_response::Error>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}
impl IdentityErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity_error_response {
    use super::*;
    #[doc = "Error code"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Error")]
    pub enum Error {
        #[serde(rename = "invalid_request")]
        InvalidRequest,
        #[serde(rename = "unauthorized_client")]
        UnauthorizedClient,
        #[serde(rename = "access_denied")]
        AccessDenied,
        #[serde(rename = "unsupported_response_type")]
        UnsupportedResponseType,
        #[serde(rename = "invalid_scope")]
        InvalidScope,
        #[serde(rename = "server_error")]
        ServerError,
        #[serde(rename = "service_unavailable")]
        ServiceUnavailable,
        #[serde(rename = "bad_request")]
        BadRequest,
        #[serde(rename = "forbidden")]
        Forbidden,
        #[serde(rename = "not_found")]
        NotFound,
        #[serde(rename = "method_not_allowed")]
        MethodNotAllowed,
        #[serde(rename = "too_many_requests")]
        TooManyRequests,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Error {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Error {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Error {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::InvalidRequest => serializer.serialize_unit_variant("Error", 0u32, "invalid_request"),
                Self::UnauthorizedClient => serializer.serialize_unit_variant("Error", 1u32, "unauthorized_client"),
                Self::AccessDenied => serializer.serialize_unit_variant("Error", 2u32, "access_denied"),
                Self::UnsupportedResponseType => serializer.serialize_unit_variant("Error", 3u32, "unsupported_response_type"),
                Self::InvalidScope => serializer.serialize_unit_variant("Error", 4u32, "invalid_scope"),
                Self::ServerError => serializer.serialize_unit_variant("Error", 5u32, "server_error"),
                Self::ServiceUnavailable => serializer.serialize_unit_variant("Error", 6u32, "service_unavailable"),
                Self::BadRequest => serializer.serialize_unit_variant("Error", 7u32, "bad_request"),
                Self::Forbidden => serializer.serialize_unit_variant("Error", 8u32, "forbidden"),
                Self::NotFound => serializer.serialize_unit_variant("Error", 9u32, "not_found"),
                Self::MethodNotAllowed => serializer.serialize_unit_variant("Error", 10u32, "method_not_allowed"),
                Self::TooManyRequests => serializer.serialize_unit_variant("Error", 11u32, "too_many_requests"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "This is the response from the Identity_GetInfo operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityInfoResponse {
    #[doc = "This is the AAD tenantId of the identity of the system assigned managed identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl IdentityInfoResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the response from the Identity_GetToken operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityTokenResponse {
    #[doc = "This is the requested access token. The app can use this token to authenticate to the sink resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[doc = "This is how long the access token is valid (in seconds)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<String>,
    #[doc = "This is the time when the access token expires. The date is represented as the number of seconds from 1970-01-01T0:0:0Z UTC until the expiration time. This value is used to determine the lifetime of cached tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_on: Option<String>,
    #[doc = "This indicates the extended lifetime of the token (in seconds)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext_expires_in: Option<String>,
    #[doc = "This is the time when the access token becomes effective. The date is represented as the number of seconds from 1970-01-01T0:0:0Z UTC until the expiration time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[doc = "This is the app ID URI of the sink resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "This indicates the token type value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[doc = "This is the client_id specified in the request, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "This is the object_id specified in the request, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "This is the msi_res_id specified in the request, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msi_res_id: Option<String>,
}
impl IdentityTokenResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This contains information about the OS image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageReference {
    #[doc = "This is the resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "This is the offer of the platform or marketplace image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "This is the image publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "This is the image SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "This is the version of the platform or marketplace image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ImageReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the response from the Instance_GetMetadata operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Instance {
    #[doc = "Compute Metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute: Option<Compute>,
    #[doc = "Network Metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
}
impl Instance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This contains the IPv4 properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Ipv4Properties {
    #[doc = "This is the private IP address assigned to the interface."]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "This is the public IP address assigned to the interface."]
    #[serde(rename = "publicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
}
impl Ipv4Properties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This contains the IPv6 properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Ipv6Properties {
    #[doc = "This is the private IPv6 address assigned to the interface."]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
}
impl Ipv6Properties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is managed disk parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedDisk {
    #[doc = "This is the resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "This is the storage account type for the managed disk."]
    #[serde(rename = "storageAccountType", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_type: Option<String>,
}
impl ManagedDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network Metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Network {
    #[doc = "This contains data about the network interface."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interface: Vec<NetworkInterface>,
}
impl Network {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This contains data about the network interface."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterface {
    #[doc = "This contains the IPv4 address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<network_interface::Ipv4>,
    #[doc = "This contains the IPv6 address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<network_interface::Ipv6>,
    #[doc = "This is the MAC address of the interface."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
}
impl NetworkInterface {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_interface {
    use super::*;
    #[doc = "This contains the IPv4 address."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Ipv4 {
        #[doc = "This is the IP address"]
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Vec::is_empty")]
        pub ip_address: Vec<Ipv4Properties>,
        #[doc = "This is the subnet"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub subnet: Vec<SubnetProperties>,
    }
    impl Ipv4 {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "This contains the IPv6 address."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Ipv6 {
        #[doc = "This is the IP address"]
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Vec::is_empty")]
        pub ip_address: Vec<Ipv6Properties>,
    }
    impl Ipv6 {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "This contains information about the OS disk used by the VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsDisk {
    #[doc = "This is the caching requirements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caching: Option<String>,
    #[doc = "This is information about how the VM was created."]
    #[serde(rename = "createOption", default, skip_serializing_if = "Option::is_none")]
    pub create_option: Option<String>,
    #[doc = "This is the ephemeral disk settings."]
    #[serde(rename = "diffDiskSettings", default, skip_serializing_if = "Option::is_none")]
    pub diff_disk_settings: Option<DiffDiskSettings>,
    #[doc = "This is the size of the disk in GB."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<String>,
    #[doc = "This is the encryption settings for the disk."]
    #[serde(rename = "encryptionSettings", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings: Option<EncryptionSettings>,
    #[doc = "This is the source user image virtual hard disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<DiskImage>,
    #[doc = "This is managed disk parameters."]
    #[serde(rename = "managedDisk", default, skip_serializing_if = "Option::is_none")]
    pub managed_disk: Option<ManagedDisk>,
    #[doc = "This is the disk name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "This is the type of OS included in the disk."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "This is the virtual hard disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhd: Option<VirtualHardDisk>,
    #[doc = "This specifies whether or not writeAccelerator is enabled on the disk."]
    #[serde(rename = "writeAcceleratorEnabled", default, skip_serializing_if = "Option::is_none")]
    pub write_accelerator_enabled: Option<String>,
}
impl OsDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This contains the data about the OS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[doc = "This is admin account."]
    #[serde(rename = "adminUsername", default, skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,
    #[doc = "This is the name of the VM."]
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[doc = "This specifies whether or not password authentication is disabled. Note that this is present only for Linux VMs. For a Windows VM, this value will be the empty string."]
    #[serde(rename = "disablePasswordAuthentication", default, skip_serializing_if = "Option::is_none")]
    pub disable_password_authentication: Option<String>,
}
impl OsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This contains the data about the plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanProperties {
    #[doc = "This is the Plan ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "This is the publisher ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "This is the product of the image from the Marketplace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}
impl PlanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This contains the data about the public key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicKeysProperties {
    #[doc = "This specifies the full path on the VM where the SSH public key is stored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "This is the SSH public key certificate used to authenticate with the VM."]
    #[serde(rename = "keyData", default, skip_serializing_if = "Option::is_none")]
    pub key_data: Option<String>,
}
impl PublicKeysProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This contains the data about the security profile associated with the VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityProfile {
    #[doc = "Identifies if UEFI secure boot is enabled on the VM"]
    #[serde(rename = "secureBootEnabled", default, skip_serializing_if = "Option::is_none")]
    pub secure_boot_enabled: Option<String>,
    #[doc = "Identifies if the virtual Trusted Platform Module (TPM) is enabled on the VM"]
    #[serde(rename = "virtualTpmEnabled", default, skip_serializing_if = "Option::is_none")]
    pub virtual_tpm_enabled: Option<String>,
}
impl SecurityProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This contains the data about the storage disks associated with the VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[doc = "This contains information about the OS image."]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[doc = "This contains information about the OS disk used by the VM."]
    #[serde(rename = "osDisk", default, skip_serializing_if = "Option::is_none")]
    pub os_disk: Option<OsDisk>,
    #[doc = "Data disk information"]
    #[serde(rename = "dataDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub data_disks: Vec<DataDisk>,
    #[doc = "This contains data for the size of local temp disk of the VM, if it exists."]
    #[serde(rename = "resourceDisk", default, skip_serializing_if = "Option::is_none")]
    pub resource_disk: Option<storage_profile::ResourceDisk>,
}
impl StorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_profile {
    use super::*;
    #[doc = "This contains data for the size of local temp disk of the VM, if it exists."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ResourceDisk {
        #[doc = "The size of the local temp disk of the VM if it exists, in kilobytes. If the VM has no local temp disk, this value is 0."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size: Option<String>,
    }
    impl ResourceDisk {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "This contains the properties of the subnet."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubnetProperties {
    #[doc = "This is the address range of the subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "This is the prefix of the subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}
impl SubnetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This contains the properties of the tags in a tagsList."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsProperties {
    #[doc = "This is the name of the tag. It is equivalent to the key in the key-value pair format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "This is the value of the tag. It is, as expected, equivalent to the value in the key-value pair format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl TagsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This is the virtual hard disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualHardDisk {
    #[doc = "This is the uri of the virtual hard disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl VirtualHardDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the VMSS the VM belongs to (if applicable)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineScaleSet {
    #[doc = "This is the ID of the VMSS the VM belongs to (if applicable)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl VirtualMachineScaleSet {
    pub fn new() -> Self {
        Self::default()
    }
}
