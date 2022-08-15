#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Address details for an order item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressDetails {
    #[doc = "Address Properties"]
    #[serde(rename = "forwardAddress")]
    pub forward_address: AddressProperties,
    #[doc = "Address Properties"]
    #[serde(rename = "returnAddress", default, skip_serializing_if = "Option::is_none")]
    pub return_address: Option<AddressProperties>,
}
impl AddressDetails {
    pub fn new(forward_address: AddressProperties) -> Self {
        Self {
            forward_address,
            return_address: None,
        }
    }
}
#[doc = "Address Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressProperties {
    #[doc = "Shipping address where customer wishes to receive the device."]
    #[serde(rename = "shippingAddress", default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
    #[doc = "Contact Details."]
    #[serde(rename = "contactDetails")]
    pub contact_details: ContactDetails,
    #[doc = "Status of address validation"]
    #[serde(rename = "addressValidationStatus", default, skip_serializing_if = "Option::is_none")]
    pub address_validation_status: Option<address_properties::AddressValidationStatus>,
}
impl AddressProperties {
    pub fn new(contact_details: ContactDetails) -> Self {
        Self {
            shipping_address: None,
            contact_details,
            address_validation_status: None,
        }
    }
}
pub mod address_properties {
    use super::*;
    #[doc = "Status of address validation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AddressValidationStatus")]
    pub enum AddressValidationStatus {
        Valid,
        Invalid,
        Ambiguous,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AddressValidationStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AddressValidationStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AddressValidationStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Valid => serializer.serialize_unit_variant("AddressValidationStatus", 0u32, "Valid"),
                Self::Invalid => serializer.serialize_unit_variant("AddressValidationStatus", 1u32, "Invalid"),
                Self::Ambiguous => serializer.serialize_unit_variant("AddressValidationStatus", 2u32, "Ambiguous"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Address Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Address Properties"]
    pub properties: AddressProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl AddressResource {
    pub fn new(tracked_resource: TrackedResource, properties: AddressProperties) -> Self {
        Self {
            tracked_resource,
            properties,
            system_data: None,
        }
    }
}
#[doc = "Address Resource Collection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddressResourceList {
    #[doc = "List of address resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AddressResource>,
    #[doc = "Link for the next set of job resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AddressResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AddressResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Address update parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddressUpdateParameter {
    #[doc = "Address Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AddressUpdateProperties>,
    #[doc = "The list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AddressUpdateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Address Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddressUpdateProperties {
    #[doc = "Shipping address where customer wishes to receive the device."]
    #[serde(rename = "shippingAddress", default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
    #[doc = "Contact Details."]
    #[serde(rename = "contactDetails", default, skip_serializing_if = "Option::is_none")]
    pub contact_details: Option<ContactDetails>,
}
impl AddressUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Availability information of a product system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityInformation {
    #[doc = "Current availability stage of the product. Availability stage"]
    #[serde(rename = "availabilityStage", default, skip_serializing_if = "Option::is_none")]
    pub availability_stage: Option<availability_information::AvailabilityStage>,
    #[doc = "Reason why the product is disabled."]
    #[serde(rename = "disabledReason", default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<availability_information::DisabledReason>,
    #[doc = "Message for why the product is disabled."]
    #[serde(rename = "disabledReasonMessage", default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason_message: Option<String>,
}
impl AvailabilityInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod availability_information {
    use super::*;
    #[doc = "Current availability stage of the product. Availability stage"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AvailabilityStage")]
    pub enum AvailabilityStage {
        Available,
        ComingSoon,
        Preview,
        Deprecated,
        Signup,
        Unavailable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AvailabilityStage {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AvailabilityStage {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AvailabilityStage {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Available => serializer.serialize_unit_variant("AvailabilityStage", 0u32, "Available"),
                Self::ComingSoon => serializer.serialize_unit_variant("AvailabilityStage", 1u32, "ComingSoon"),
                Self::Preview => serializer.serialize_unit_variant("AvailabilityStage", 2u32, "Preview"),
                Self::Deprecated => serializer.serialize_unit_variant("AvailabilityStage", 3u32, "Deprecated"),
                Self::Signup => serializer.serialize_unit_variant("AvailabilityStage", 4u32, "Signup"),
                Self::Unavailable => serializer.serialize_unit_variant("AvailabilityStage", 5u32, "Unavailable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Reason why the product is disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DisabledReason")]
    pub enum DisabledReason {
        None,
        Country,
        Region,
        Feature,
        OfferType,
        NoSubscriptionInfo,
        NotAvailable,
        OutOfStock,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DisabledReason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DisabledReason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DisabledReason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("DisabledReason", 0u32, "None"),
                Self::Country => serializer.serialize_unit_variant("DisabledReason", 1u32, "Country"),
                Self::Region => serializer.serialize_unit_variant("DisabledReason", 2u32, "Region"),
                Self::Feature => serializer.serialize_unit_variant("DisabledReason", 3u32, "Feature"),
                Self::OfferType => serializer.serialize_unit_variant("DisabledReason", 4u32, "OfferType"),
                Self::NoSubscriptionInfo => serializer.serialize_unit_variant("DisabledReason", 5u32, "NoSubscriptionInfo"),
                Self::NotAvailable => serializer.serialize_unit_variant("DisabledReason", 6u32, "NotAvailable"),
                Self::OutOfStock => serializer.serialize_unit_variant("DisabledReason", 7u32, "OutOfStock"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Basic information for any product system"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BasicInformation {
    #[doc = "Display Name for the product system."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description related properties of a product system."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[doc = "Image information for the product system."]
    #[serde(rename = "imageInformation", default, skip_serializing_if = "Vec::is_empty")]
    pub image_information: Vec<ImageInformation>,
    #[doc = "Cost information for the product system"]
    #[serde(rename = "costInformation", default, skip_serializing_if = "Option::is_none")]
    pub cost_information: Option<CostInformation>,
    #[doc = "Availability information of a product system."]
    #[serde(rename = "availabilityInformation", default, skip_serializing_if = "Option::is_none")]
    pub availability_information: Option<AvailabilityInformation>,
    #[doc = "Holds details about product hierarchy information"]
    #[serde(rename = "hierarchyInformation", default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_information: Option<HierarchyInformation>,
}
impl BasicInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Holds billing meter details for each type of billing"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingMeterDetails {
    #[doc = "Represents Billing type name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Holds details about billing type and its meter guids"]
    #[serde(rename = "meterDetails", default, skip_serializing_if = "Option::is_none")]
    pub meter_details: Option<MeterDetails>,
    #[doc = "Represents Metering type (eg one-time or recurrent)"]
    #[serde(rename = "meteringType", default, skip_serializing_if = "Option::is_none")]
    pub metering_type: Option<billing_meter_details::MeteringType>,
    #[doc = "Frequency of recurrence"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
}
impl BillingMeterDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_meter_details {
    use super::*;
    #[doc = "Represents Metering type (eg one-time or recurrent)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MeteringType")]
    pub enum MeteringType {
        OneTime,
        Recurring,
        Adhoc,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MeteringType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MeteringType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MeteringType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::OneTime => serializer.serialize_unit_variant("MeteringType", 0u32, "OneTime"),
                Self::Recurring => serializer.serialize_unit_variant("MeteringType", 1u32, "Recurring"),
                Self::Adhoc => serializer.serialize_unit_variant("MeteringType", 2u32, "Adhoc"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Reason for cancellation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CancellationReason {
    #[doc = "Reason for cancellation."]
    pub reason: String,
}
impl CancellationReason {
    pub fn new(reason: String) -> Self {
        Self { reason }
    }
}
#[doc = "Represents common properties across product hierarchy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommonProperties {
    #[serde(flatten)]
    pub basic_information: BasicInformation,
    #[doc = "list of filters supported for a product"]
    #[serde(rename = "filterableProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub filterable_properties: Vec<FilterableProperty>,
}
impl CommonProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Configuration {
    #[doc = "Properties of configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationProperties>,
}
impl Configuration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration filters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationFilters {
    #[doc = "Holds details about product hierarchy information"]
    #[serde(rename = "hierarchyInformation")]
    pub hierarchy_information: HierarchyInformation,
    #[doc = "Filters specific to product"]
    #[serde(rename = "filterableProperty", default, skip_serializing_if = "Vec::is_empty")]
    pub filterable_property: Vec<FilterableProperty>,
}
impl ConfigurationFilters {
    pub fn new(hierarchy_information: HierarchyInformation) -> Self {
        Self {
            hierarchy_information,
            filterable_property: Vec::new(),
        }
    }
}
#[doc = "Properties of configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationProperties {
    #[serde(flatten)]
    pub common_properties: CommonProperties,
    #[doc = "Specifications of the configuration"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specifications: Vec<Specification>,
    #[doc = "Dimensions of a configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
}
impl ConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Configurations {
    #[doc = "List of configurations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Configuration>,
    #[doc = "Link for the next set of configurations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for Configurations {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl Configurations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration request object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationsRequest {
    #[doc = "Holds details about product hierarchy information and filterable property."]
    #[serde(rename = "configurationFilters")]
    pub configuration_filters: Vec<ConfigurationFilters>,
    #[doc = "Holds Customer subscription details. Clients can display available products to unregistered customers by explicitly passing subscription details"]
    #[serde(rename = "customerSubscriptionDetails", default, skip_serializing_if = "Option::is_none")]
    pub customer_subscription_details: Option<CustomerSubscriptionDetails>,
}
impl ConfigurationsRequest {
    pub fn new(configuration_filters: Vec<ConfigurationFilters>) -> Self {
        Self {
            configuration_filters,
            customer_subscription_details: None,
        }
    }
}
#[doc = "Contact Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactDetails {
    #[doc = "Contact name of the person."]
    #[serde(rename = "contactName")]
    pub contact_name: String,
    #[doc = "Phone number of the contact person."]
    pub phone: String,
    #[doc = "Phone extension number of the contact person."]
    #[serde(rename = "phoneExtension", default, skip_serializing_if = "Option::is_none")]
    pub phone_extension: Option<String>,
    #[doc = "Mobile number of the contact person."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[doc = "List of Email-ids to be notified about job progress."]
    #[serde(rename = "emailList")]
    pub email_list: Vec<String>,
}
impl ContactDetails {
    pub fn new(contact_name: String, phone: String, email_list: Vec<String>) -> Self {
        Self {
            contact_name,
            phone,
            phone_extension: None,
            mobile: None,
            email_list,
        }
    }
}
#[doc = "Cost information for the product system"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CostInformation {
    #[doc = "Details on the various billing aspects for the product system."]
    #[serde(rename = "billingMeterDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub billing_meter_details: Vec<BillingMeterDetails>,
    #[doc = "Default url to display billing information"]
    #[serde(rename = "billingInfoUrl", default, skip_serializing_if = "Option::is_none")]
    pub billing_info_url: Option<String>,
}
impl CostInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Holds Customer subscription details. Clients can display available products to unregistered customers by explicitly passing subscription details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerSubscriptionDetails {
    #[doc = "List of registered feature flags for subscription"]
    #[serde(rename = "registeredFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub registered_features: Vec<CustomerSubscriptionRegisteredFeatures>,
    #[doc = "Location placement Id of a subscription"]
    #[serde(rename = "locationPlacementId", default, skip_serializing_if = "Option::is_none")]
    pub location_placement_id: Option<String>,
    #[doc = "Quota ID of a subscription"]
    #[serde(rename = "quotaId")]
    pub quota_id: String,
}
impl CustomerSubscriptionDetails {
    pub fn new(quota_id: String) -> Self {
        Self {
            registered_features: Vec::new(),
            location_placement_id: None,
            quota_id,
        }
    }
}
#[doc = "Represents subscription registered features"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerSubscriptionRegisteredFeatures {
    #[doc = "Name of subscription registered feature"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "State of subscription registered feature"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl CustomerSubscriptionRegisteredFeatures {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description related properties of a product system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Description {
    #[doc = "Type of description."]
    #[serde(rename = "descriptionType", default, skip_serializing_if = "Option::is_none")]
    pub description_type: Option<description::DescriptionType>,
    #[doc = "Short description of the product system."]
    #[serde(rename = "shortDescription", default, skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[doc = "Long description of the product system."]
    #[serde(rename = "longDescription", default, skip_serializing_if = "Option::is_none")]
    pub long_description: Option<String>,
    #[doc = "Keywords for the product system."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[doc = "Attributes for the product system."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<String>,
    #[doc = "Links for the product system."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link>,
}
impl Description {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod description {
    use super::*;
    #[doc = "Type of description."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DescriptionType")]
    pub enum DescriptionType {
        Base,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DescriptionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DescriptionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DescriptionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Base => serializer.serialize_unit_variant("DescriptionType", 0u32, "Base"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Device details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceDetails {
    #[doc = "device serial number"]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Management Resource Id"]
    #[serde(rename = "managementResourceId", default, skip_serializing_if = "Option::is_none")]
    pub management_resource_id: Option<String>,
    #[doc = "Management Resource Tenant ID"]
    #[serde(rename = "managementResourceTenantId", default, skip_serializing_if = "Option::is_none")]
    pub management_resource_tenant_id: Option<String>,
}
impl DeviceDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dimensions of a configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimensions {
    #[doc = "Length of the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    #[doc = "Height of the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[doc = "Width of the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[doc = "Unit for the dimensions of length, height and width."]
    #[serde(rename = "lengthHeightUnit", default, skip_serializing_if = "Option::is_none")]
    pub length_height_unit: Option<dimensions::LengthHeightUnit>,
    #[doc = "Weight of the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[doc = "Depth of the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<f64>,
    #[doc = "Unit for the dimensions of weight."]
    #[serde(rename = "weightUnit", default, skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<dimensions::WeightUnit>,
}
impl Dimensions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dimensions {
    use super::*;
    #[doc = "Unit for the dimensions of length, height and width."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LengthHeightUnit")]
    pub enum LengthHeightUnit {
        #[serde(rename = "IN")]
        In,
        #[serde(rename = "CM")]
        Cm,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LengthHeightUnit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LengthHeightUnit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LengthHeightUnit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::In => serializer.serialize_unit_variant("LengthHeightUnit", 0u32, "IN"),
                Self::Cm => serializer.serialize_unit_variant("LengthHeightUnit", 1u32, "CM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Unit for the dimensions of weight."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "WeightUnit")]
    pub enum WeightUnit {
        #[serde(rename = "LBS")]
        Lbs,
        #[serde(rename = "KGS")]
        Kgs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for WeightUnit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for WeightUnit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for WeightUnit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Lbs => serializer.serialize_unit_variant("WeightUnit", 0u32, "LBS"),
                Self::Kgs => serializer.serialize_unit_variant("WeightUnit", 1u32, "KGS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes product display information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DisplayInfo {
    #[doc = "Product family display name"]
    #[serde(rename = "productFamilyDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub product_family_display_name: Option<String>,
    #[doc = "Configuration display name"]
    #[serde(rename = "configurationDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub configuration_display_name: Option<String>,
}
impl DisplayInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Preferences related to the double encryption"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionPreferences {
    #[doc = "Double encryption status as entered by the customer. It is compulsory to give this parameter if the 'Deny' or 'Disabled' policy is configured."]
    #[serde(rename = "doubleEncryptionStatus", default, skip_serializing_if = "Option::is_none")]
    pub double_encryption_status: Option<encryption_preferences::DoubleEncryptionStatus>,
}
impl EncryptionPreferences {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption_preferences {
    use super::*;
    #[doc = "Double encryption status as entered by the customer. It is compulsory to give this parameter if the 'Deny' or 'Disabled' policy is configured."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DoubleEncryptionStatus")]
    pub enum DoubleEncryptionStatus {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DoubleEncryptionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DoubleEncryptionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DoubleEncryptionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("DoubleEncryptionStatus", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("DoubleEncryptionStatus", 1u32, "Enabled"),
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
#[doc = "Different types of filters supported and its values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterableProperty {
    #[doc = "Type of product filter."]
    #[serde(rename = "type")]
    pub type_: filterable_property::Type,
    #[doc = "Values to be filtered."]
    #[serde(rename = "supportedValues")]
    pub supported_values: Vec<String>,
}
impl FilterableProperty {
    pub fn new(type_: filterable_property::Type, supported_values: Vec<String>) -> Self {
        Self { type_, supported_values }
    }
}
pub mod filterable_property {
    use super::*;
    #[doc = "Type of product filter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        ShipToCountries,
        DoubleEncryptionStatus,
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
                Self::ShipToCountries => serializer.serialize_unit_variant("Type", 0u32, "ShipToCountries"),
                Self::DoubleEncryptionStatus => serializer.serialize_unit_variant("Type", 1u32, "DoubleEncryptionStatus"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Forward shipment details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForwardShippingDetails {
    #[doc = "Name of the carrier."]
    #[serde(rename = "carrierName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<String>,
    #[doc = "Carrier Name for display purpose. Not to be used for any processing."]
    #[serde(rename = "carrierDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_display_name: Option<String>,
    #[doc = "TrackingId of the package"]
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[doc = "TrackingUrl of the package."]
    #[serde(rename = "trackingUrl", default, skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
}
impl ForwardShippingDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Holds details about product hierarchy information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchyInformation {
    #[doc = "Represents product family name that uniquely identifies product family"]
    #[serde(rename = "productFamilyName", default, skip_serializing_if = "Option::is_none")]
    pub product_family_name: Option<String>,
    #[doc = "Represents product line name that uniquely identifies product line"]
    #[serde(rename = "productLineName", default, skip_serializing_if = "Option::is_none")]
    pub product_line_name: Option<String>,
    #[doc = "Represents product name that uniquely identifies product"]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "Represents configuration name that uniquely identifies configuration"]
    #[serde(rename = "configurationName", default, skip_serializing_if = "Option::is_none")]
    pub configuration_name: Option<String>,
}
impl HierarchyInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image for the product"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageInformation {
    #[doc = "Type of the image"]
    #[serde(rename = "imageType", default, skip_serializing_if = "Option::is_none")]
    pub image_type: Option<image_information::ImageType>,
    #[doc = "Url of the image"]
    #[serde(rename = "imageUrl", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}
impl ImageInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod image_information {
    use super::*;
    #[doc = "Type of the image"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ImageType")]
    pub enum ImageType {
        MainImage,
        BulletImage,
        GenericImage,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ImageType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ImageType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ImageType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MainImage => serializer.serialize_unit_variant("ImageType", 0u32, "MainImage"),
                Self::BulletImage => serializer.serialize_unit_variant("ImageType", 1u32, "BulletImage"),
                Self::GenericImage => serializer.serialize_unit_variant("ImageType", 2u32, "GenericImage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Returns link related to the product"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Link {
    #[doc = "Type of link"]
    #[serde(rename = "linkType", default, skip_serializing_if = "Option::is_none")]
    pub link_type: Option<link::LinkType>,
    #[doc = "Url of the link"]
    #[serde(rename = "linkUrl", default, skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
}
impl Link {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod link {
    use super::*;
    #[doc = "Type of link"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LinkType")]
    pub enum LinkType {
        Generic,
        TermsAndConditions,
        Specification,
        Documentation,
        KnowMore,
        SignUp,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LinkType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LinkType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LinkType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Generic => serializer.serialize_unit_variant("LinkType", 0u32, "Generic"),
                Self::TermsAndConditions => serializer.serialize_unit_variant("LinkType", 1u32, "TermsAndConditions"),
                Self::Specification => serializer.serialize_unit_variant("LinkType", 2u32, "Specification"),
                Self::Documentation => serializer.serialize_unit_variant("LinkType", 3u32, "Documentation"),
                Self::KnowMore => serializer.serialize_unit_variant("LinkType", 4u32, "KnowMore"),
                Self::SignUp => serializer.serialize_unit_variant("LinkType", 5u32, "SignUp"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Management resource preference to link device"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementResourcePreferences {
    #[doc = "Customer preferred Management resource ARM ID"]
    #[serde(rename = "preferredManagementResourceId", default, skip_serializing_if = "Option::is_none")]
    pub preferred_management_resource_id: Option<String>,
}
impl ManagementResourcePreferences {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Holds details about billing type and its meter guids"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MeterDetails {
    #[doc = "Represents billing type."]
    #[serde(rename = "billingType")]
    pub billing_type: meter_details::BillingType,
    #[doc = "Billing unit applicable for Pav2 billing"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<f64>,
    #[doc = "Charging type."]
    #[serde(rename = "chargingType", default, skip_serializing_if = "Option::is_none")]
    pub charging_type: Option<meter_details::ChargingType>,
}
impl MeterDetails {
    pub fn new(billing_type: meter_details::BillingType) -> Self {
        Self {
            billing_type,
            multiplier: None,
            charging_type: None,
        }
    }
}
pub mod meter_details {
    use super::*;
    #[doc = "Represents billing type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingType")]
    pub enum BillingType {
        Pav2,
        Purchase,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pav2 => serializer.serialize_unit_variant("BillingType", 0u32, "Pav2"),
                Self::Purchase => serializer.serialize_unit_variant("BillingType", 1u32, "Purchase"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Charging type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ChargingType")]
    pub enum ChargingType {
        PerOrder,
        PerDevice,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ChargingType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ChargingType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ChargingType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PerOrder => serializer.serialize_unit_variant("ChargingType", 0u32, "PerOrder"),
                Self::PerDevice => serializer.serialize_unit_variant("ChargingType", 1u32, "PerDevice"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Notification preference for a job stage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationPreference {
    #[doc = "Name of the stage."]
    #[serde(rename = "stageName")]
    pub stage_name: notification_preference::StageName,
    #[doc = "Notification is required or not."]
    #[serde(rename = "sendNotification")]
    pub send_notification: bool,
}
impl NotificationPreference {
    pub fn new(stage_name: notification_preference::StageName, send_notification: bool) -> Self {
        Self {
            stage_name,
            send_notification,
        }
    }
}
pub mod notification_preference {
    use super::*;
    #[doc = "Name of the stage."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StageName")]
    pub enum StageName {
        Shipped,
        Delivered,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StageName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StageName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StageName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Shipped => serializer.serialize_unit_variant("StageName", 0u32, "Shipped"),
                Self::Delivered => serializer.serialize_unit_variant("StageName", 1u32, "Delivered"),
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
#[doc = "Order item details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderItemDetails {
    #[doc = "Represents product details"]
    #[serde(rename = "productDetails")]
    pub product_details: ProductDetails,
    #[doc = "Order item type."]
    #[serde(rename = "orderItemType")]
    pub order_item_type: order_item_details::OrderItemType,
    #[doc = "Resource stage details."]
    #[serde(rename = "currentStage", default, skip_serializing_if = "Option::is_none")]
    pub current_stage: Option<StageDetails>,
    #[doc = "Order item status history"]
    #[serde(rename = "orderItemStageHistory", default, skip_serializing_if = "Vec::is_empty")]
    pub order_item_stage_history: Vec<StageDetails>,
    #[doc = "Preferences related to the order"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Preferences>,
    #[doc = "Forward shipment details."]
    #[serde(rename = "forwardShippingDetails", default, skip_serializing_if = "Option::is_none")]
    pub forward_shipping_details: Option<ForwardShippingDetails>,
    #[doc = "Reverse shipment details."]
    #[serde(rename = "reverseShippingDetails", default, skip_serializing_if = "Option::is_none")]
    pub reverse_shipping_details: Option<ReverseShippingDetails>,
    #[doc = "Additional notification email list"]
    #[serde(rename = "notificationEmailList", default, skip_serializing_if = "Vec::is_empty")]
    pub notification_email_list: Vec<String>,
    #[doc = "Cancellation reason."]
    #[serde(rename = "cancellationReason", default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    #[doc = "Describes whether the order item is cancellable or not."]
    #[serde(rename = "cancellationStatus", default, skip_serializing_if = "Option::is_none")]
    pub cancellation_status: Option<order_item_details::CancellationStatus>,
    #[doc = "Describes whether the order item is deletable or not."]
    #[serde(rename = "deletionStatus", default, skip_serializing_if = "Option::is_none")]
    pub deletion_status: Option<order_item_details::DeletionStatus>,
    #[doc = "Return reason."]
    #[serde(rename = "returnReason", default, skip_serializing_if = "Option::is_none")]
    pub return_reason: Option<String>,
    #[doc = "Describes whether the order item is returnable or not."]
    #[serde(rename = "returnStatus", default, skip_serializing_if = "Option::is_none")]
    pub return_status: Option<order_item_details::ReturnStatus>,
    #[doc = "Management RP details"]
    #[serde(rename = "managementRpDetails", default, skip_serializing_if = "Option::is_none")]
    pub management_rp_details: Option<ResourceProviderDetails>,
    #[doc = "List of parent RP details supported for configuration."]
    #[serde(rename = "managementRpDetailsList", default, skip_serializing_if = "Vec::is_empty")]
    pub management_rp_details_list: Vec<ResourceProviderDetails>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl OrderItemDetails {
    pub fn new(product_details: ProductDetails, order_item_type: order_item_details::OrderItemType) -> Self {
        Self {
            product_details,
            order_item_type,
            current_stage: None,
            order_item_stage_history: Vec::new(),
            preferences: None,
            forward_shipping_details: None,
            reverse_shipping_details: None,
            notification_email_list: Vec::new(),
            cancellation_reason: None,
            cancellation_status: None,
            deletion_status: None,
            return_reason: None,
            return_status: None,
            management_rp_details: None,
            management_rp_details_list: Vec::new(),
            error: None,
        }
    }
}
pub mod order_item_details {
    use super::*;
    #[doc = "Order item type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OrderItemType")]
    pub enum OrderItemType {
        Purchase,
        Rental,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OrderItemType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OrderItemType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OrderItemType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Purchase => serializer.serialize_unit_variant("OrderItemType", 0u32, "Purchase"),
                Self::Rental => serializer.serialize_unit_variant("OrderItemType", 1u32, "Rental"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Describes whether the order item is cancellable or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CancellationStatus")]
    pub enum CancellationStatus {
        Cancellable,
        CancellableWithFee,
        NotCancellable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CancellationStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CancellationStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CancellationStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Cancellable => serializer.serialize_unit_variant("CancellationStatus", 0u32, "Cancellable"),
                Self::CancellableWithFee => serializer.serialize_unit_variant("CancellationStatus", 1u32, "CancellableWithFee"),
                Self::NotCancellable => serializer.serialize_unit_variant("CancellationStatus", 2u32, "NotCancellable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Describes whether the order item is deletable or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeletionStatus")]
    pub enum DeletionStatus {
        Allowed,
        NotAllowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeletionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeletionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeletionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allowed => serializer.serialize_unit_variant("DeletionStatus", 0u32, "Allowed"),
                Self::NotAllowed => serializer.serialize_unit_variant("DeletionStatus", 1u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Describes whether the order item is returnable or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReturnStatus")]
    pub enum ReturnStatus {
        Returnable,
        ReturnableWithFee,
        NotReturnable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReturnStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReturnStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReturnStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Returnable => serializer.serialize_unit_variant("ReturnStatus", 0u32, "Returnable"),
                Self::ReturnableWithFee => serializer.serialize_unit_variant("ReturnStatus", 1u32, "ReturnableWithFee"),
                Self::NotReturnable => serializer.serialize_unit_variant("ReturnStatus", 2u32, "NotReturnable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents order item details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderItemProperties {
    #[doc = "Order item details"]
    #[serde(rename = "orderItemDetails")]
    pub order_item_details: OrderItemDetails,
    #[doc = "Address details for an order item."]
    #[serde(rename = "addressDetails")]
    pub address_details: AddressDetails,
    #[doc = "Start time of order item"]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Id of the order to which order item belongs to"]
    #[serde(rename = "orderId")]
    pub order_id: String,
}
impl OrderItemProperties {
    pub fn new(order_item_details: OrderItemDetails, address_details: AddressDetails, order_id: String) -> Self {
        Self {
            order_item_details,
            address_details,
            start_time: None,
            order_id,
        }
    }
}
#[doc = "Represents order item contract"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderItemResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Represents order item details."]
    pub properties: OrderItemProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl OrderItemResource {
    pub fn new(tracked_resource: TrackedResource, properties: OrderItemProperties) -> Self {
        Self {
            tracked_resource,
            properties,
            system_data: None,
        }
    }
}
#[doc = "List of orderItems."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderItemResourceList {
    #[doc = "List of order item resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OrderItemResource>,
    #[doc = "Link for the next set of order item resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OrderItemResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OrderItemResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Updates order item parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderItemUpdateParameter {
    #[doc = "Order item update properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OrderItemUpdateProperties>,
    #[doc = "The list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl OrderItemUpdateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Order item update properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderItemUpdateProperties {
    #[doc = "Address Properties"]
    #[serde(rename = "forwardAddress", default, skip_serializing_if = "Option::is_none")]
    pub forward_address: Option<AddressProperties>,
    #[doc = "Preferences related to the order"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Preferences>,
    #[doc = "Additional notification email list."]
    #[serde(rename = "notificationEmailList", default, skip_serializing_if = "Vec::is_empty")]
    pub notification_email_list: Vec<String>,
}
impl OrderItemUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents order details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderProperties {
    #[doc = "List of order item ARM Ids which are part of an order."]
    #[serde(rename = "orderItemIds", default, skip_serializing_if = "Vec::is_empty")]
    pub order_item_ids: Vec<String>,
    #[doc = "Resource stage details."]
    #[serde(rename = "currentStage", default, skip_serializing_if = "Option::is_none")]
    pub current_stage: Option<StageDetails>,
    #[doc = "Order status history."]
    #[serde(rename = "orderStageHistory", default, skip_serializing_if = "Vec::is_empty")]
    pub order_stage_history: Vec<StageDetails>,
}
impl OrderProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the properties or parameters for an order. Order is a grouping of one or more order items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Represents order details."]
    pub properties: OrderProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl OrderResource {
    pub fn new(properties: OrderProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "List of orders."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrderResourceList {
    #[doc = "List of order resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OrderResource>,
    #[doc = "Link for the next set of order resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OrderResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OrderResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Billing type PAV2 meter details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pav2MeterDetails {
    #[serde(flatten)]
    pub meter_details: MeterDetails,
    #[doc = "Validation status of requested data center and transport."]
    #[serde(rename = "meterGuid", default, skip_serializing_if = "Option::is_none")]
    pub meter_guid: Option<String>,
}
impl Pav2MeterDetails {
    pub fn new(meter_details: MeterDetails) -> Self {
        Self {
            meter_details,
            meter_guid: None,
        }
    }
}
#[doc = "Preferences related to the order"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Preferences {
    #[doc = "Notification preferences."]
    #[serde(rename = "notificationPreferences", default, skip_serializing_if = "Vec::is_empty")]
    pub notification_preferences: Vec<NotificationPreference>,
    #[doc = "Preferences related to the shipment logistics of the sku"]
    #[serde(rename = "transportPreferences", default, skip_serializing_if = "Option::is_none")]
    pub transport_preferences: Option<TransportPreferences>,
    #[doc = "Preferences related to the double encryption"]
    #[serde(rename = "encryptionPreferences", default, skip_serializing_if = "Option::is_none")]
    pub encryption_preferences: Option<EncryptionPreferences>,
    #[doc = "Management resource preference to link device"]
    #[serde(rename = "managementResourcePreferences", default, skip_serializing_if = "Option::is_none")]
    pub management_resource_preferences: Option<ManagementResourcePreferences>,
}
impl Preferences {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Products"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Product {
    #[doc = "Properties of products"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductProperties>,
}
impl Product {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents product details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductDetails {
    #[doc = "Describes product display information"]
    #[serde(rename = "displayInfo", default, skip_serializing_if = "Option::is_none")]
    pub display_info: Option<DisplayInfo>,
    #[doc = "Holds details about product hierarchy information"]
    #[serde(rename = "hierarchyInformation")]
    pub hierarchy_information: HierarchyInformation,
    #[doc = "Quantity of the product"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Double encryption status of the configuration. Read-only field."]
    #[serde(rename = "productDoubleEncryptionStatus", default, skip_serializing_if = "Option::is_none")]
    pub product_double_encryption_status: Option<product_details::ProductDoubleEncryptionStatus>,
    #[doc = "list of device details"]
    #[serde(rename = "deviceDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub device_details: Vec<DeviceDetails>,
}
impl ProductDetails {
    pub fn new(hierarchy_information: HierarchyInformation) -> Self {
        Self {
            display_info: None,
            hierarchy_information,
            count: None,
            product_double_encryption_status: None,
            device_details: Vec::new(),
        }
    }
}
pub mod product_details {
    use super::*;
    #[doc = "Double encryption status of the configuration. Read-only field."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProductDoubleEncryptionStatus")]
    pub enum ProductDoubleEncryptionStatus {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProductDoubleEncryptionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProductDoubleEncryptionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProductDoubleEncryptionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("ProductDoubleEncryptionStatus", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("ProductDoubleEncryptionStatus", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The list of product families."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductFamilies {
    #[doc = "List of product families."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProductFamily>,
    #[doc = "Link for the next set of product families."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProductFamilies {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProductFamilies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Holds details about product family metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductFamiliesMetadata {
    #[doc = "List of product family metadata details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProductFamiliesMetadataDetails>,
    #[doc = "Link for the next set of product families."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProductFamiliesMetadata {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProductFamiliesMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Product families metadata details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductFamiliesMetadataDetails {
    #[doc = "Properties of product family"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductFamilyProperties>,
}
impl ProductFamiliesMetadataDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The filters for showing the product families."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductFamiliesRequest {
    #[doc = "Dictionary of filterable properties on product family."]
    #[serde(rename = "filterableProperties")]
    pub filterable_properties: serde_json::Value,
    #[doc = "Holds Customer subscription details. Clients can display available products to unregistered customers by explicitly passing subscription details"]
    #[serde(rename = "customerSubscriptionDetails", default, skip_serializing_if = "Option::is_none")]
    pub customer_subscription_details: Option<CustomerSubscriptionDetails>,
}
impl ProductFamiliesRequest {
    pub fn new(filterable_properties: serde_json::Value) -> Self {
        Self {
            filterable_properties,
            customer_subscription_details: None,
        }
    }
}
#[doc = "Product Family"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductFamily {
    #[doc = "Properties of product family"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductFamilyProperties>,
}
impl ProductFamily {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of product family"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductFamilyProperties {
    #[serde(flatten)]
    pub common_properties: CommonProperties,
    #[doc = "List of product lines supported in the product family"]
    #[serde(rename = "productLines", default, skip_serializing_if = "Vec::is_empty")]
    pub product_lines: Vec<ProductLine>,
    #[doc = "Contains details related to resource provider"]
    #[serde(rename = "resourceProviderDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_provider_details: Vec<ResourceProviderDetails>,
}
impl ProductFamilyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Product line"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductLine {
    #[doc = "Properties of product line"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductLineProperties>,
}
impl ProductLine {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of product line"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductLineProperties {
    #[serde(flatten)]
    pub common_properties: CommonProperties,
    #[doc = "List of products in the product line"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub products: Vec<Product>,
}
impl ProductLineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of products"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductProperties {
    #[serde(flatten)]
    pub common_properties: CommonProperties,
    #[doc = "List of configurations for the product"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub configurations: Vec<Configuration>,
}
impl ProductProperties {
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
#[doc = "Billing type Purchase meter details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurchaseMeterDetails {
    #[serde(flatten)]
    pub meter_details: MeterDetails,
    #[doc = "Product Id"]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[doc = "Sku Id"]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "Term Id"]
    #[serde(rename = "termId", default, skip_serializing_if = "Option::is_none")]
    pub term_id: Option<String>,
}
impl PurchaseMeterDetails {
    pub fn new(meter_details: MeterDetails) -> Self {
        Self {
            meter_details,
            product_id: None,
            sku_id: None,
            term_id: None,
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
#[doc = "Msi identity details of the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceIdentity {
    #[doc = "Identity type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Service Principal Id backing the Msi"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Home Tenant Id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ResourceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Management RP details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderDetails {
    #[doc = "Resource provider namespace"]
    #[serde(rename = "resourceProviderNamespace", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider_namespace: Option<String>,
}
impl ResourceProviderDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Return order item request body"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReturnOrderItemDetails {
    #[doc = "Address Properties"]
    #[serde(rename = "returnAddress", default, skip_serializing_if = "Option::is_none")]
    pub return_address: Option<AddressProperties>,
    #[doc = "Return Reason."]
    #[serde(rename = "returnReason")]
    pub return_reason: String,
    #[doc = "Service tag (located on the bottom-right corner of the device)"]
    #[serde(rename = "serviceTag", default, skip_serializing_if = "Option::is_none")]
    pub service_tag: Option<String>,
    #[doc = "Shipping Box required"]
    #[serde(rename = "shippingBoxRequired", default, skip_serializing_if = "Option::is_none")]
    pub shipping_box_required: Option<bool>,
}
impl ReturnOrderItemDetails {
    pub fn new(return_reason: String) -> Self {
        Self {
            return_address: None,
            return_reason,
            service_tag: None,
            shipping_box_required: None,
        }
    }
}
#[doc = "Reverse shipment details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReverseShippingDetails {
    #[doc = "SAS key to download the reverse shipment label of the package."]
    #[serde(rename = "sasKeyForLabel", default, skip_serializing_if = "Option::is_none")]
    pub sas_key_for_label: Option<String>,
    #[doc = "Name of the carrier."]
    #[serde(rename = "carrierName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<String>,
    #[doc = "Carrier Name for display purpose. Not to be used for any processing."]
    #[serde(rename = "carrierDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_display_name: Option<String>,
    #[doc = "TrackingId of the package"]
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[doc = "TrackingUrl of the package."]
    #[serde(rename = "trackingUrl", default, skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
}
impl ReverseShippingDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Shipping address where customer wishes to receive the device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShippingAddress {
    #[doc = "Street Address line 1."]
    #[serde(rename = "streetAddress1")]
    pub street_address1: String,
    #[doc = "Street Address line 2."]
    #[serde(rename = "streetAddress2", default, skip_serializing_if = "Option::is_none")]
    pub street_address2: Option<String>,
    #[doc = "Street Address line 3."]
    #[serde(rename = "streetAddress3", default, skip_serializing_if = "Option::is_none")]
    pub street_address3: Option<String>,
    #[doc = "Name of the City."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Name of the State or Province."]
    #[serde(rename = "stateOrProvince", default, skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    #[doc = "Name of the Country."]
    pub country: String,
    #[doc = "Postal code."]
    #[serde(rename = "postalCode", default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "Extended Zip Code."]
    #[serde(rename = "zipExtendedCode", default, skip_serializing_if = "Option::is_none")]
    pub zip_extended_code: Option<String>,
    #[doc = "Name of the company."]
    #[serde(rename = "companyName", default, skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[doc = "Type of address."]
    #[serde(rename = "addressType", default, skip_serializing_if = "Option::is_none")]
    pub address_type: Option<shipping_address::AddressType>,
}
impl ShippingAddress {
    pub fn new(street_address1: String, country: String) -> Self {
        Self {
            street_address1,
            street_address2: None,
            street_address3: None,
            city: None,
            state_or_province: None,
            country,
            postal_code: None,
            zip_extended_code: None,
            company_name: None,
            address_type: None,
        }
    }
}
pub mod shipping_address {
    use super::*;
    #[doc = "Type of address."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AddressType")]
    pub enum AddressType {
        None,
        Residential,
        Commercial,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AddressType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AddressType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AddressType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("AddressType", 0u32, "None"),
                Self::Residential => serializer.serialize_unit_variant("AddressType", 1u32, "Residential"),
                Self::Commercial => serializer.serialize_unit_variant("AddressType", 2u32, "Commercial"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Package shipping details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShippingDetails {
    #[doc = "Name of the carrier."]
    #[serde(rename = "carrierName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<String>,
    #[doc = "Carrier Name for display purpose. Not to be used for any processing."]
    #[serde(rename = "carrierDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_display_name: Option<String>,
    #[doc = "TrackingId of the package"]
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[doc = "TrackingUrl of the package."]
    #[serde(rename = "trackingUrl", default, skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
}
impl ShippingDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifications of the configurations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Specification {
    #[doc = "Name of the specification"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Value of the specification"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Specification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource stage details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StageDetails {
    #[doc = "Stage status."]
    #[serde(rename = "stageStatus", default, skip_serializing_if = "Option::is_none")]
    pub stage_status: Option<stage_details::StageStatus>,
    #[doc = "Stage name"]
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<stage_details::StageName>,
    #[doc = "Display name of the resource stage."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Stage start time"]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
}
impl StageDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod stage_details {
    use super::*;
    #[doc = "Stage status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StageStatus")]
    pub enum StageStatus {
        None,
        InProgress,
        Succeeded,
        Failed,
        Cancelled,
        Cancelling,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StageStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StageStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StageStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("StageStatus", 0u32, "None"),
                Self::InProgress => serializer.serialize_unit_variant("StageStatus", 1u32, "InProgress"),
                Self::Succeeded => serializer.serialize_unit_variant("StageStatus", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("StageStatus", 3u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("StageStatus", 4u32, "Cancelled"),
                Self::Cancelling => serializer.serialize_unit_variant("StageStatus", 5u32, "Cancelling"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Stage name"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StageName")]
    pub enum StageName {
        Placed,
        InReview,
        Confirmed,
        ReadyToShip,
        Shipped,
        Delivered,
        InUse,
        ReturnInitiated,
        ReturnPickedUp,
        ReturnedToMicrosoft,
        ReturnCompleted,
        Cancelled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StageName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StageName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StageName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Placed => serializer.serialize_unit_variant("StageName", 0u32, "Placed"),
                Self::InReview => serializer.serialize_unit_variant("StageName", 1u32, "InReview"),
                Self::Confirmed => serializer.serialize_unit_variant("StageName", 2u32, "Confirmed"),
                Self::ReadyToShip => serializer.serialize_unit_variant("StageName", 3u32, "ReadyToShip"),
                Self::Shipped => serializer.serialize_unit_variant("StageName", 4u32, "Shipped"),
                Self::Delivered => serializer.serialize_unit_variant("StageName", 5u32, "Delivered"),
                Self::InUse => serializer.serialize_unit_variant("StageName", 6u32, "InUse"),
                Self::ReturnInitiated => serializer.serialize_unit_variant("StageName", 7u32, "ReturnInitiated"),
                Self::ReturnPickedUp => serializer.serialize_unit_variant("StageName", 8u32, "ReturnPickedUp"),
                Self::ReturnedToMicrosoft => serializer.serialize_unit_variant("StageName", 9u32, "ReturnedToMicrosoft"),
                Self::ReturnCompleted => serializer.serialize_unit_variant("StageName", 10u32, "ReturnCompleted"),
                Self::Cancelled => serializer.serialize_unit_variant("StageName", 11u32, "Cancelled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "Preferences related to the shipment logistics of the sku"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransportPreferences {
    #[doc = "Indicates Shipment Logistics type that the customer preferred."]
    #[serde(rename = "preferredShipmentType")]
    pub preferred_shipment_type: transport_preferences::PreferredShipmentType,
}
impl TransportPreferences {
    pub fn new(preferred_shipment_type: transport_preferences::PreferredShipmentType) -> Self {
        Self { preferred_shipment_type }
    }
}
pub mod transport_preferences {
    use super::*;
    #[doc = "Indicates Shipment Logistics type that the customer preferred."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PreferredShipmentType")]
    pub enum PreferredShipmentType {
        CustomerManaged,
        MicrosoftManaged,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PreferredShipmentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PreferredShipmentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PreferredShipmentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CustomerManaged => serializer.serialize_unit_variant("PreferredShipmentType", 0u32, "CustomerManaged"),
                Self::MicrosoftManaged => serializer.serialize_unit_variant("PreferredShipmentType", 1u32, "MicrosoftManaged"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
