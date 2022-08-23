#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Profile for how to handle shutting down virtual machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoShutdownProfile {
    #[doc = "Property enabled state."]
    #[serde(rename = "shutdownOnDisconnect", default, skip_serializing_if = "Option::is_none")]
    pub shutdown_on_disconnect: Option<EnableState>,
    #[doc = "Property enabled state."]
    #[serde(rename = "shutdownWhenNotConnected", default, skip_serializing_if = "Option::is_none")]
    pub shutdown_when_not_connected: Option<EnableState>,
    #[doc = "Defines whether to shut down VM on idle and the criteria for idle detection."]
    #[serde(rename = "shutdownOnIdle", default, skip_serializing_if = "Option::is_none")]
    pub shutdown_on_idle: Option<ShutdownOnIdleMode>,
    #[doc = "The amount of time a VM will stay running after a user disconnects if this behavior is enabled."]
    #[serde(rename = "disconnectDelay", default, skip_serializing_if = "Option::is_none")]
    pub disconnect_delay: Option<String>,
    #[doc = "The amount of time a VM will stay running before it is shutdown if no connection is made and this behavior is enabled."]
    #[serde(rename = "noConnectDelay", default, skip_serializing_if = "Option::is_none")]
    pub no_connect_delay: Option<String>,
    #[doc = "The amount of time a VM will idle before it is shutdown if this behavior is enabled."]
    #[serde(rename = "idleDelay", default, skip_serializing_if = "Option::is_none")]
    pub idle_delay: Option<String>,
}
impl AutoShutdownProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Connection profile for how users connect to lab virtual machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionProfile {
    #[doc = "A connection type for access labs and VMs (Public, Private or None)."]
    #[serde(rename = "webSshAccess", default, skip_serializing_if = "Option::is_none")]
    pub web_ssh_access: Option<ConnectionType>,
    #[doc = "A connection type for access labs and VMs (Public, Private or None)."]
    #[serde(rename = "webRdpAccess", default, skip_serializing_if = "Option::is_none")]
    pub web_rdp_access: Option<ConnectionType>,
    #[doc = "A connection type for access labs and VMs (Public, Private or None)."]
    #[serde(rename = "clientSshAccess", default, skip_serializing_if = "Option::is_none")]
    pub client_ssh_access: Option<ConnectionType>,
    #[doc = "A connection type for access labs and VMs (Public, Private or None)."]
    #[serde(rename = "clientRdpAccess", default, skip_serializing_if = "Option::is_none")]
    pub client_rdp_access: Option<ConnectionType>,
}
impl ConnectionProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Credentials for a user on a lab VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Credentials {
    #[doc = "The username to use when signing in to lab VMs."]
    pub username: String,
    #[doc = "The password for the user. This is required for the TemplateVM createOption."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl Credentials {
    pub fn new(username: String) -> Self {
        Self { username, password: None }
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
#[doc = "Lab services virtual machine image"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of an image resource."]
    pub properties: ImageProperties,
}
impl Image {
    pub fn new(properties: ImageProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "Properties of an image resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageProperties {
    #[serde(flatten)]
    pub image_update_properties: ImageUpdateProperties,
    #[doc = "Resource provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The image display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "A description of the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "URL of the image icon."]
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[doc = "The image author."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[doc = "The operating system type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "The ID of marketplace plan associated with the image (optional)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[doc = "Property enabled state."]
    #[serde(rename = "termsStatus", default, skip_serializing_if = "Option::is_none")]
    pub terms_status: Option<EnableState>,
    #[doc = "The ID of an offer associated with the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "The ID of the publisher of the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The image SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "The image version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "A URL."]
    #[serde(rename = "sharedGalleryId", default, skip_serializing_if = "Option::is_none")]
    pub shared_gallery_id: Option<Url>,
    #[doc = "The available regions of the image in the shared gallery."]
    #[serde(rename = "availableRegions", default, skip_serializing_if = "Vec::is_empty")]
    pub available_regions: Vec<String>,
    #[doc = "The operating system state."]
    #[serde(rename = "osState", default, skip_serializing_if = "Option::is_none")]
    pub os_state: Option<OsState>,
}
impl ImageProperties {
    pub fn new() -> Self {
        Self {
            image_update_properties: ImageUpdateProperties::default(),
            provisioning_state: None,
            display_name: None,
            description: None,
            icon_url: None,
            author: None,
            os_type: None,
            plan: None,
            terms_status: None,
            offer: None,
            publisher: None,
            sku: None,
            version: None,
            shared_gallery_id: None,
            available_regions: Vec::new(),
            os_state: None,
        }
    }
}
#[doc = "Image reference information. Used in the virtual machine profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageReference {
    #[doc = "A URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Url>,
    #[doc = "The image offer if applicable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "The image publisher"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The image SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "The image version specified on creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The actual version of the image after use."]
    #[serde(rename = "exactVersion", default, skip_serializing_if = "Option::is_none")]
    pub exact_version: Option<String>,
}
impl ImageReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lab services virtual machine image for updates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageUpdate {
    #[doc = "Properties of an image resource update"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImageUpdateProperties>,
}
impl ImageUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an image resource update"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageUpdateProperties {
    #[doc = "Property enabled state."]
    #[serde(rename = "enabledState", default, skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<EnableState>,
}
impl ImageUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The lab user invitation state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InvitationState {
    NotSent,
    Sending,
    Sent,
    Failed,
}
#[doc = "Body for a user invite request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InviteBody {
    #[doc = "Custom text for the invite email."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
impl InviteBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The lab resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Lab {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Properties of a lab resource."]
    pub properties: LabProperties,
}
impl Lab {
    pub fn new(tracked_resource: TrackedResource, properties: LabProperties) -> Self {
        Self {
            tracked_resource,
            system_data: None,
            properties,
        }
    }
}
#[doc = "Profile for how to handle networking for Labs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabNetworkProfile {
    #[doc = "A URL."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<Url>,
    #[doc = "A URL."]
    #[serde(rename = "loadBalancerId", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<Url>,
    #[doc = "A URL."]
    #[serde(rename = "publicIpId", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_id: Option<Url>,
}
impl LabNetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lab Plans act as a permission container for creating labs via labs.azure.com. Additionally, they can provide a set of default configurations that will apply at the time of creating a lab, but these defaults can still be overwritten."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabPlan {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Lab plan resource properties"]
    pub properties: LabPlanProperties,
}
impl LabPlan {
    pub fn new(tracked_resource: TrackedResource, properties: LabPlanProperties) -> Self {
        Self {
            tracked_resource,
            system_data: None,
            properties,
        }
    }
}
#[doc = "Profile for how to handle networking for Lab Plans."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabPlanNetworkProfile {
    #[doc = "A URL."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<Url>,
}
impl LabPlanNetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lab plan resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabPlanProperties {
    #[serde(flatten)]
    pub lab_plan_update_properties: LabPlanUpdateProperties,
    #[doc = "Resource provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl LabPlanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains lab configuration and default settings. This variant is used for PATCH."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabPlanUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "Lab plan resource properties for updates"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabPlanUpdateProperties>,
}
impl LabPlanUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lab plan resource properties for updates"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabPlanUpdateProperties {
    #[doc = "Connection profile for how users connect to lab virtual machines."]
    #[serde(rename = "defaultConnectionProfile", default, skip_serializing_if = "Option::is_none")]
    pub default_connection_profile: Option<ConnectionProfile>,
    #[doc = "Profile for how to handle shutting down virtual machines."]
    #[serde(rename = "defaultAutoShutdownProfile", default, skip_serializing_if = "Option::is_none")]
    pub default_auto_shutdown_profile: Option<AutoShutdownProfile>,
    #[doc = "Profile for how to handle networking for Lab Plans."]
    #[serde(rename = "defaultNetworkProfile", default, skip_serializing_if = "Option::is_none")]
    pub default_network_profile: Option<LabPlanNetworkProfile>,
    #[doc = "The allowed regions for the lab creator to use when creating labs using this lab plan."]
    #[serde(rename = "allowedRegions", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_regions: Vec<String>,
    #[doc = "A URL."]
    #[serde(rename = "sharedGalleryId", default, skip_serializing_if = "Option::is_none")]
    pub shared_gallery_id: Option<Url>,
    #[doc = "Support contact information and instructions."]
    #[serde(rename = "supportInfo", default, skip_serializing_if = "Option::is_none")]
    pub support_info: Option<SupportInfo>,
    #[doc = "A URL."]
    #[serde(rename = "linkedLmsInstance", default, skip_serializing_if = "Option::is_none")]
    pub linked_lms_instance: Option<Url>,
}
impl LabPlanUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a lab resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabProperties {
    #[serde(flatten)]
    pub lab_update_properties: LabUpdateProperties,
    #[doc = "Resource provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Profile for how to handle networking for Labs."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<LabNetworkProfile>,
    #[doc = "The state of a virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<LabState>,
}
impl LabProperties {
    pub fn new() -> Self {
        Self {
            lab_update_properties: LabUpdateProperties::default(),
            provisioning_state: None,
            network_profile: None,
            state: None,
        }
    }
}
#[doc = "Azure Lab Services resource SKUs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabServicesSku {
    #[doc = "The lab services resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The name of the SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The tier of the SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<lab_services_sku::Tier>,
    #[doc = "The SKU size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The family of the SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The scale out/in options of the SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<LabServicesSkuCapacity>,
    #[doc = "The capabilities of the SKU."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<LabServicesSkuCapabilities>,
    #[doc = "List of locations that are available for a size."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "Metadata for retrieving price info of a lab services SKUs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub costs: Vec<LabServicesSkuCost>,
    #[doc = "Restrictions of a lab services SKUs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<LabServicesSkuRestrictions>,
}
impl LabServicesSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod lab_services_sku {
    use super::*;
    #[doc = "The tier of the SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Standard,
        Premium,
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
                Self::Standard => serializer.serialize_unit_variant("Tier", 0u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("Tier", 1u32, "Premium"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The array of capabilities of a lab services SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabServicesSkuCapabilities {
    #[doc = "The name of the capability for a SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the capability for a SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl LabServicesSkuCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The scale out/in options of the SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabServicesSkuCapacity {
    #[doc = "The default capacity for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,
    #[doc = "The lowest permitted capacity for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[doc = "The highest permitted capacity for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[doc = "The localized name of the resource."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<lab_services_sku_capacity::ScaleType>,
}
impl LabServicesSkuCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod lab_services_sku_capacity {
    use super::*;
    #[doc = "The localized name of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScaleType")]
    pub enum ScaleType {
        None,
        Manual,
        Automatic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScaleType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScaleType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScaleType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ScaleType", 0u32, "None"),
                Self::Manual => serializer.serialize_unit_variant("ScaleType", 1u32, "Manual"),
                Self::Automatic => serializer.serialize_unit_variant("ScaleType", 2u32, "Automatic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The array of costs of a lab services SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabServicesSkuCost {
    #[doc = "The meter id."]
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "The quantity of units charged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[doc = "The extended unit."]
    #[serde(rename = "extendedUnit", default, skip_serializing_if = "Option::is_none")]
    pub extended_unit: Option<String>,
}
impl LabServicesSkuCost {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The restriction details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabServicesSkuRestrictions {
    #[doc = "The type of restriction."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<lab_services_sku_restrictions::Type>,
    #[doc = "The values of the restriction."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[doc = "The reason for the restriction."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<lab_services_sku_restrictions::ReasonCode>,
}
impl LabServicesSkuRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod lab_services_sku_restrictions {
    use super::*;
    #[doc = "The type of restriction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Location,
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
                Self::Location => serializer.serialize_unit_variant("Type", 0u32, "Location"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The reason for the restriction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReasonCode")]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReasonCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReasonCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReasonCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::QuotaId => serializer.serialize_unit_variant("ReasonCode", 0u32, "QuotaId"),
                Self::NotAvailableForSubscription => serializer.serialize_unit_variant("ReasonCode", 1u32, "NotAvailableForSubscription"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The state of a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LabState {
    Draft,
    Publishing,
    Scaling,
    Syncing,
    Published,
}
#[doc = "The lab resource for updates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "Properties of a lab resource used for updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LabUpdateProperties>,
}
impl LabUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a lab resource used for updates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabUpdateProperties {
    #[doc = "Profile for how to handle shutting down virtual machines."]
    #[serde(rename = "autoShutdownProfile", default, skip_serializing_if = "Option::is_none")]
    pub auto_shutdown_profile: Option<AutoShutdownProfile>,
    #[doc = "Connection profile for how users connect to lab virtual machines."]
    #[serde(rename = "connectionProfile", default, skip_serializing_if = "Option::is_none")]
    pub connection_profile: Option<ConnectionProfile>,
    #[doc = "The base virtual machine configuration for a lab."]
    #[serde(rename = "virtualMachineProfile", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_profile: Option<VirtualMachineProfile>,
    #[doc = "The lab security profile."]
    #[serde(rename = "securityProfile", default, skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<SecurityProfile>,
    #[doc = "The lab user list management profile."]
    #[serde(rename = "rosterProfile", default, skip_serializing_if = "Option::is_none")]
    pub roster_profile: Option<RosterProfile>,
    #[doc = "A URL."]
    #[serde(rename = "labPlanId", default, skip_serializing_if = "Option::is_none")]
    pub lab_plan_id: Option<Url>,
    #[doc = "The title of the lab."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The description of the lab."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl LabUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Core Usages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListUsagesResult {
    #[doc = "The array page of Usages."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
    #[doc = "The link to get the next page of Usage result."]
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
    pub fn new() -> Self {
        Self::default()
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
#[doc = "A long running operation result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResult {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The operation status"]
    pub status: operation_result::Status,
    #[doc = "Start time"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time"]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Percent completion"]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl OperationResult {
    pub fn new(status: operation_result::Status) -> Self {
        Self {
            id: None,
            name: None,
            status,
            start_time: None,
            end_time: None,
            percent_complete: None,
            error: None,
        }
    }
}
pub mod operation_result {
    use super::*;
    #[doc = "The operation status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        NotStarted,
        InProgress,
        Succeeded,
        Failed,
        Canceled,
    }
}
#[doc = "Paged list of Lab services virtual machine images."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedImages {
    #[doc = "The array page of virtual machine images."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Image>,
    #[doc = "The link to get the next page of image results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedImages {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PagedImages {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged list of lab plans."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedLabPlans {
    #[doc = "The array page of lab plans."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LabPlan>,
    #[doc = "The link to get the next page of lab plan results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedLabPlans {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PagedLabPlans {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged list of lab services skus."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedLabServicesSkus {
    #[doc = "The array page of sku results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LabServicesSku>,
    #[doc = "The link to get the next page of sku results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedLabServicesSkus {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PagedLabServicesSkus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged list of labs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedLabs {
    #[doc = "The array page of lab results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Lab>,
    #[doc = "The link to get the next page of image results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedLabs {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PagedLabs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged list of schedules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedSchedules {
    #[doc = "The array page of schedule results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Schedule>,
    #[doc = "The link to get the next page of schedule results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedSchedules {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PagedSchedules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged list of users."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedUsers {
    #[doc = "The array page of user results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<User>,
    #[doc = "The link to get the next page of image results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedUsers {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PagedUsers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged list of lab services virtual machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedVirtualMachines {
    #[doc = "The array page of virtual machine results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualMachine>,
    #[doc = "The link to get the next page of virtual machine results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedVirtualMachines {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PagedVirtualMachines {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Schedule recurrence frequencies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RecurrenceFrequency {
    Daily,
    Weekly,
}
#[doc = "Recurrence pattern of a lab schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurrencePattern {
    #[doc = "Schedule recurrence frequencies."]
    pub frequency: RecurrenceFrequency,
    #[doc = "The week days the schedule runs. Used for when the Frequency is set to Weekly."]
    #[serde(rename = "weekDays", default, skip_serializing_if = "Vec::is_empty")]
    pub week_days: Vec<WeekDay>,
    #[doc = "The interval to invoke the schedule on. For example, interval = 2 and RecurrenceFrequency.Daily will run every 2 days. When no interval is supplied, an interval of 1 is used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[doc = "When the recurrence will expire. This date is inclusive."]
    #[serde(rename = "expirationDate")]
    pub expiration_date: String,
}
impl RecurrencePattern {
    pub fn new(frequency: RecurrenceFrequency, expiration_date: String) -> Self {
        Self {
            frequency,
            week_days: Vec::new(),
            interval: None,
            expiration_date,
        }
    }
}
#[doc = "The user lab registration state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RegistrationState {
    NotRegistered,
    Registered,
}
#[doc = "Body of a reset password request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResetPasswordBody {
    #[doc = "The user whose password is being reset"]
    pub username: String,
    #[doc = "The password"]
    pub password: String,
}
impl ResetPasswordBody {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
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
#[doc = "The lab user list management profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RosterProfile {
    #[doc = "The AAD group ID which this lab roster is populated from. Having this set enables AAD sync mode."]
    #[serde(rename = "activeDirectoryGroupId", default, skip_serializing_if = "Option::is_none")]
    pub active_directory_group_id: Option<String>,
    #[doc = "The unique context identifier for the lab in the lms."]
    #[serde(rename = "ltiContextId", default, skip_serializing_if = "Option::is_none")]
    pub lti_context_id: Option<String>,
    #[doc = "The base URI identifying the lms instance."]
    #[serde(rename = "lmsInstance", default, skip_serializing_if = "Option::is_none")]
    pub lms_instance: Option<String>,
    #[doc = "The unique id of the azure lab services tool in the lms."]
    #[serde(rename = "ltiClientId", default, skip_serializing_if = "Option::is_none")]
    pub lti_client_id: Option<String>,
    #[doc = "The uri of the names and roles service endpoint on the lms for the class attached to this lab."]
    #[serde(rename = "ltiRosterEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub lti_roster_endpoint: Option<String>,
}
impl RosterProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Body for the save image POST"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaveImageBody {
    #[doc = "The name for the image we create."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A URL."]
    #[serde(rename = "labVirtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub lab_virtual_machine_id: Option<Url>,
}
impl SaveImageBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schedule for automatically turning virtual machines in a lab on and off at specified times."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Schedule resource properties"]
    pub properties: ScheduleProperties,
}
impl Schedule {
    pub fn new(properties: ScheduleProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "Schedule resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleProperties {
    #[serde(flatten)]
    pub schedule_update_properties: ScheduleUpdateProperties,
    #[doc = "Resource provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ScheduleProperties {
    pub fn new() -> Self {
        Self {
            schedule_update_properties: ScheduleUpdateProperties::default(),
            provisioning_state: None,
        }
    }
}
#[doc = "Schedule for automatically turning virtual machines in a lab on and off at specified times. Used for updates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleUpdate {
    #[doc = "Schedule resource properties used for updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduleUpdateProperties>,
}
impl ScheduleUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schedule resource properties used for updates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleUpdateProperties {
    #[doc = "When lab user virtual machines will be started. Timestamp offsets will be ignored and timeZoneId is used instead."]
    #[serde(rename = "startAt", default, with = "azure_core::date::rfc3339::option")]
    pub start_at: Option<time::OffsetDateTime>,
    #[doc = "When lab user virtual machines will be stopped. Timestamp offsets will be ignored and timeZoneId is used instead."]
    #[serde(rename = "stopAt", default, with = "azure_core::date::rfc3339::option")]
    pub stop_at: Option<time::OffsetDateTime>,
    #[doc = "Recurrence pattern of a lab schedule."]
    #[serde(rename = "recurrencePattern", default, skip_serializing_if = "Option::is_none")]
    pub recurrence_pattern: Option<RecurrencePattern>,
    #[doc = "The IANA timezone id for the schedule."]
    #[serde(rename = "timeZoneId", default, skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    #[doc = "Notes for this schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
impl ScheduleUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The lab security profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecurityProfile {
    #[doc = "The registration code for the lab."]
    #[serde(rename = "registrationCode", default, skip_serializing_if = "Option::is_none")]
    pub registration_code: Option<String>,
    #[doc = "Property enabled state."]
    #[serde(rename = "openAccess", default, skip_serializing_if = "Option::is_none")]
    pub open_access: Option<EnableState>,
}
impl SecurityProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition representing SKU"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the SKU. Ex - P3. It is typically a letter+number code"]
    pub name: String,
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
    #[doc = "The SKU size. When the name field is the combination of tier and some other value, this would be the standalone code. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "If the service has different generations of hardware, for the same SKU, then that can be captured here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "If the SKU supports scale out/in then the capacity integer should be included. If scale out/in is not possible for the resource this may be omitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
pub mod sku {
    use super::*;
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Free,
        Basic,
        Standard,
        Premium,
    }
}
#[doc = "Support contact information and instructions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportInfo {
    #[doc = "A URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[doc = "An email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailAddress>,
    #[doc = "A phone number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<PhoneNumber>,
    #[doc = "Support instructions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}
impl SupportInfo {
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
#[doc = "Base tracked resource type for all PATCH updates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
impl TrackedResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The core usage details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[doc = "The current usage."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "The limit integer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "The unit details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<usage::Unit>,
    #[doc = "The Usage Names."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<UsageName>,
    #[doc = "The fully qualified arm resource id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod usage {
    use super::*;
    #[doc = "The unit details."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Count,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "Count"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Usage Names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageName {
    #[doc = "The localized name of the resource."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl UsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User of a lab that can register for and use virtual machines within the lab."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "User resource properties"]
    pub properties: UserProperties,
}
impl User {
    pub fn new(properties: UserProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "User resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserProperties {
    #[serde(flatten)]
    pub user_update_properties: UserUpdateProperties,
    #[doc = "Resource provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Display name of the user, for example user's full name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "An email address."]
    pub email: EmailAddress,
    #[doc = "The user lab registration state."]
    #[serde(rename = "registrationState", default, skip_serializing_if = "Option::is_none")]
    pub registration_state: Option<RegistrationState>,
    #[doc = "The lab user invitation state."]
    #[serde(rename = "invitationState", default, skip_serializing_if = "Option::is_none")]
    pub invitation_state: Option<InvitationState>,
    #[doc = "Date and time when the invitation message was sent to the user."]
    #[serde(rename = "invitationSent", default, with = "azure_core::date::rfc3339::option")]
    pub invitation_sent: Option<time::OffsetDateTime>,
    #[doc = "How long the user has used their virtual machines in this lab."]
    #[serde(rename = "totalUsage", default, skip_serializing_if = "Option::is_none")]
    pub total_usage: Option<String>,
}
impl UserProperties {
    pub fn new(email: EmailAddress) -> Self {
        Self {
            user_update_properties: UserUpdateProperties::default(),
            provisioning_state: None,
            display_name: None,
            email,
            registration_state: None,
            invitation_state: None,
            invitation_sent: None,
            total_usage: None,
        }
    }
}
#[doc = "User of a lab that can register for and use virtual machines within the lab. Used for updates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserUpdate {
    #[doc = "User resource properties used for updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserUpdateProperties>,
}
impl UserUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User resource properties used for updates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserUpdateProperties {
    #[doc = "The amount of usage quota time the user gets in addition to the lab usage quota."]
    #[serde(rename = "additionalUsageQuota", default, skip_serializing_if = "Option::is_none")]
    pub additional_usage_quota: Option<String>,
}
impl UserUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A lab virtual machine resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachine {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Virtual machine resource properties"]
    pub properties: VirtualMachineProperties,
}
impl VirtualMachine {
    pub fn new(properties: VirtualMachineProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            system_data: None,
            properties,
        }
    }
}
#[doc = "The additional capabilities for a lab VM."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineAdditionalCapabilities {
    #[doc = "Property enabled state."]
    #[serde(rename = "installGpuDrivers", default, skip_serializing_if = "Option::is_none")]
    pub install_gpu_drivers: Option<EnableState>,
}
impl VirtualMachineAdditionalCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The connection information for the virtual machine"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineConnectionProfile {
    #[doc = "The private IP address of the virtual machine."]
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[doc = "Port and host name separated by semicolon for connecting via SSH protocol to the virtual machine."]
    #[serde(rename = "sshAuthority", default, skip_serializing_if = "Option::is_none")]
    pub ssh_authority: Option<String>,
    #[doc = "A URL."]
    #[serde(rename = "sshInBrowserUrl", default, skip_serializing_if = "Option::is_none")]
    pub ssh_in_browser_url: Option<Url>,
    #[doc = "Port and host name separated by semicolon for connecting via RDP protocol to the virtual machine."]
    #[serde(rename = "rdpAuthority", default, skip_serializing_if = "Option::is_none")]
    pub rdp_authority: Option<String>,
    #[doc = "A URL."]
    #[serde(rename = "rdpInBrowserUrl", default, skip_serializing_if = "Option::is_none")]
    pub rdp_in_browser_url: Option<Url>,
    #[doc = "The username used to log on to the virtual machine as admin."]
    #[serde(rename = "adminUsername", default, skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,
    #[doc = "The username used to log on to the virtual machine as non-admin, if one exists."]
    #[serde(rename = "nonAdminUsername", default, skip_serializing_if = "Option::is_none")]
    pub non_admin_username: Option<String>,
}
impl VirtualMachineConnectionProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The base virtual machine configuration for a lab."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineProfile {
    #[doc = "Indicates what lab virtual machines are created from."]
    #[serde(rename = "createOption")]
    pub create_option: virtual_machine_profile::CreateOption,
    #[doc = "Image reference information. Used in the virtual machine profile."]
    #[serde(rename = "imageReference")]
    pub image_reference: ImageReference,
    #[doc = "The operating system type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "The resource model definition representing SKU"]
    pub sku: Sku,
    #[doc = "The additional capabilities for a lab VM."]
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Option::is_none")]
    pub additional_capabilities: Option<VirtualMachineAdditionalCapabilities>,
    #[doc = "The initial quota alloted to each lab user. Must be a time span between 0 and 9999 hours."]
    #[serde(rename = "usageQuota")]
    pub usage_quota: String,
    #[doc = "Property enabled state."]
    #[serde(rename = "useSharedPassword", default, skip_serializing_if = "Option::is_none")]
    pub use_shared_password: Option<EnableState>,
    #[doc = "Credentials for a user on a lab VM."]
    #[serde(rename = "adminUser")]
    pub admin_user: Credentials,
    #[doc = "Credentials for a user on a lab VM."]
    #[serde(rename = "nonAdminUser", default, skip_serializing_if = "Option::is_none")]
    pub non_admin_user: Option<Credentials>,
}
impl VirtualMachineProfile {
    pub fn new(
        create_option: virtual_machine_profile::CreateOption,
        image_reference: ImageReference,
        sku: Sku,
        usage_quota: String,
        admin_user: Credentials,
    ) -> Self {
        Self {
            create_option,
            image_reference,
            os_type: None,
            sku,
            additional_capabilities: None,
            usage_quota,
            use_shared_password: None,
            admin_user,
            non_admin_user: None,
        }
    }
}
pub mod virtual_machine_profile {
    use super::*;
    #[doc = "Indicates what lab virtual machines are created from."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateOption {
        Image,
        #[serde(rename = "TemplateVM")]
        TemplateVm,
    }
}
#[doc = "Virtual machine resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineProperties {
    #[doc = "Resource provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The state of a virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<VirtualMachineState>,
    #[doc = "The connection information for the virtual machine"]
    #[serde(rename = "connectionProfile", default, skip_serializing_if = "Option::is_none")]
    pub connection_profile: Option<VirtualMachineConnectionProfile>,
    #[doc = "The lab user ID (not the PUID!) of who claimed the virtual machine."]
    #[serde(rename = "claimedByUserId", default, skip_serializing_if = "Option::is_none")]
    pub claimed_by_user_id: Option<String>,
    #[doc = "The type of the lab virtual machine."]
    #[serde(rename = "vmType", default, skip_serializing_if = "Option::is_none")]
    pub vm_type: Option<VirtualMachineType>,
}
impl VirtualMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The state of a virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VirtualMachineState {
    Stopped,
    Starting,
    Running,
    Stopping,
    ResettingPassword,
    Reimaging,
    Redeploying,
}
#[doc = "The type of the lab virtual machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VirtualMachineType {
    User,
    Template,
}
#[doc = "Days of the week."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WeekDay {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
#[doc = "A connection type for access labs and VMs (Public, Private or None)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConnectionType {
    Public,
    Private,
    None,
}
pub type EmailAddress = String;
#[doc = "Property enabled state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EnableState {
    Enabled,
    Disabled,
}
#[doc = "The operating system state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OsState {
    Generalized,
    Specialized,
}
#[doc = "The operating system type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OsType {
    Windows,
    Linux,
}
pub type PhoneNumber = String;
#[doc = "Resource provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Creating,
    Updating,
    Deleting,
    Succeeded,
    Failed,
    Locked,
}
#[doc = "Defines whether to shut down VM on idle and the criteria for idle detection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ShutdownOnIdleMode {
    None,
    UserAbsence,
    LowUsage,
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
pub type Url = String;
