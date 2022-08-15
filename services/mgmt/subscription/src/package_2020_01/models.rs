#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Active Directory Principal who’ll get owner access on the new subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdPrincipal {
    #[doc = "Object id of the Principal"]
    #[serde(rename = "objectId")]
    pub object_id: String,
}
impl AdPrincipal {
    pub fn new(object_id: String) -> Self {
        Self { object_id }
    }
}
#[doc = "The ID of the canceled subscription"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CanceledSubscriptionId {
    #[doc = "The ID of the canceled subscription"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl CanceledSubscriptionId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ID of the subscriptions that is being enabled"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnabledSubscriptionId {
    #[doc = "The ID of the subscriptions that is being enabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl EnabledSubscriptionId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters required to create a new CSP subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModernCspSubscriptionCreationParameters {
    #[doc = "The friendly name of the subscription."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The SKU ID of the Azure plan. Azure plan determines the pricing and service-level agreement of the subscription.  Use 001 for Microsoft Azure Plan and 002 for Microsoft Azure Plan for DevTest."]
    #[serde(rename = "skuId")]
    pub sku_id: String,
    #[doc = "Reseller ID, basically MPN Id."]
    #[serde(rename = "resellerId", default, skip_serializing_if = "Option::is_none")]
    pub reseller_id: Option<String>,
}
impl ModernCspSubscriptionCreationParameters {
    pub fn new(display_name: String, sku_id: String) -> Self {
        Self {
            display_name,
            sku_id,
            reseller_id: None,
        }
    }
}
#[doc = "The parameters required to create a new subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModernSubscriptionCreationParameters {
    #[doc = "The friendly name of the subscription."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The SKU ID of the Azure plan. Azure plan determines the pricing and service-level agreement of the subscription.  Use 001 for Microsoft Azure Plan and 002 for Microsoft Azure Plan for DevTest."]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "If set, the cost center will show up on the Azure usage and charges file."]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "Active Directory Principal who’ll get owner access on the new subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<AdPrincipal>,
    #[doc = "The identifier of the management group to which this subscription will be associated."]
    #[serde(rename = "managementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub management_group_id: Option<String>,
}
impl ModernSubscriptionCreationParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.Subscription"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Profile, endpoint, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ID of the subscriptions that is being renamed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RenamedSubscriptionId {
    #[doc = "The ID of the subscriptions that is being renamed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl RenamedSubscriptionId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription Creation Parameters required to create a new Azure subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionCreationParameters {
    #[doc = "The display name of the subscription."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The Management Group Id."]
    #[serde(rename = "managementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub management_group_id: Option<String>,
    #[doc = "The list of principals that should be granted Owner access on the subscription. Principals should be of type User, Service Principal or Security Group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owners: Vec<AdPrincipal>,
    #[doc = "The offer type of the subscription. For example, MS-AZR-0017P (EnterpriseAgreement) and MS-AZR-0148P (EnterpriseAgreement devTest) are available. Only valid when creating a subscription in a enrollment account scope."]
    #[serde(rename = "offerType", default, skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<subscription_creation_parameters::OfferType>,
}
impl SubscriptionCreationParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_creation_parameters {
    use super::*;
    #[doc = "The offer type of the subscription. For example, MS-AZR-0017P (EnterpriseAgreement) and MS-AZR-0148P (EnterpriseAgreement devTest) are available. Only valid when creating a subscription in a enrollment account scope."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OfferType")]
    pub enum OfferType {
        #[serde(rename = "MS-AZR-0017P")]
        MsAzr0017p,
        #[serde(rename = "MS-AZR-0148P")]
        MsAzr0148p,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OfferType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OfferType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OfferType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MsAzr0017p => serializer.serialize_unit_variant("OfferType", 0u32, "MS-AZR-0017P"),
                Self::MsAzr0148p => serializer.serialize_unit_variant("OfferType", 1u32, "MS-AZR-0148P"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The created subscription object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionCreationResult {
    #[doc = "The link to the new subscription. Use this link to check the status of subscription creation operation."]
    #[serde(rename = "subscriptionLink", default, skip_serializing_if = "Option::is_none")]
    pub subscription_link: Option<String>,
}
impl SubscriptionCreationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The new name of the subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionName {
    #[doc = "New subscription name"]
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
}
impl SubscriptionName {
    pub fn new() -> Self {
        Self::default()
    }
}
