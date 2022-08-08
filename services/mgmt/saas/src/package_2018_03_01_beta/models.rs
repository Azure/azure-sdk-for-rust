#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "the ISV access token result response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessTokenResult {
    #[doc = "The Publisher Offer Base Uri"]
    #[serde(rename = "publisherOfferBaseUri", default, skip_serializing_if = "Option::is_none")]
    pub publisher_offer_base_uri: Option<String>,
    #[doc = "The generated token"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl AccessTokenResult {
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
#[doc = "Sample result definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Result {
    #[doc = "Sample property of type string"]
    #[serde(rename = "sampleProperty", default, skip_serializing_if = "Option::is_none")]
    pub sample_property: Option<String>,
}
impl Result {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "the saasApp resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaasApp {
    #[doc = "the resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "the resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "the resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Saas resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SaasAppProperties>,
    #[doc = "the resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
impl SaasApp {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "saas app operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaasAppOperation {
    #[doc = "the operation name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Saas app operation display"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<SaasAppOperationDisplay>,
    #[doc = "the operation origin"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "whether the operation is a data action or not."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl SaasAppOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Saas app operation display"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaasAppOperationDisplay {
    #[doc = "Name of the provider for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Name of the resource type for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Name of the operation for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation for display purposes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl SaasAppOperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "saas app operation response with continuation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaasAppOperationsResponseWithContinuation {
    #[doc = "the next link to query to get the remaining results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "the value of response."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SaasAppOperation>,
}
impl azure_core::Continuable for SaasAppOperationsResponseWithContinuation {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SaasAppOperationsResponseWithContinuation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Saas resource plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaasAppPlan {
    #[doc = "the publisher id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "the offer id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "the plan id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl SaasAppPlan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Saas resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaasAppProperties {
    #[doc = "the Saas resource status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<saas_app_properties::Status>,
    #[doc = "Saas resource plan."]
    #[serde(rename = "saasAppPlan", default, skip_serializing_if = "Option::is_none")]
    pub saas_app_plan: Option<SaasAppPlan>,
}
impl SaasAppProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod saas_app_properties {
    use super::*;
    #[doc = "the Saas resource status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Pending,
        Subscribed,
        Unsubscribed,
        Suspended,
        Deactivated,
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
                Self::Pending => serializer.serialize_unit_variant("Status", 0u32, "Pending"),
                Self::Subscribed => serializer.serialize_unit_variant("Status", 1u32, "Subscribed"),
                Self::Unsubscribed => serializer.serialize_unit_variant("Status", 2u32, "Unsubscribed"),
                Self::Suspended => serializer.serialize_unit_variant("Status", 3u32, "Suspended"),
                Self::Deactivated => serializer.serialize_unit_variant("Status", 4u32, "Deactivated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "saas app response with continuation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaasAppResponseWithContinuation {
    #[doc = "the next link to query to get the remaining results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "the value of response."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SaasApp>,
}
impl azure_core::Continuable for SaasAppResponseWithContinuation {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SaasAppResponseWithContinuation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "properties for creation saas"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaasCreationProperties {
    #[doc = "The offer id."]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "The publisher id."]
    #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[doc = "The seat count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[doc = "The plan id."]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "The Payment channel for the SaasSubscription."]
    #[serde(rename = "paymentChannelType", default, skip_serializing_if = "Option::is_none")]
    pub payment_channel_type: Option<saas_creation_properties::PaymentChannelType>,
    #[doc = "The metadata about the SaaS subscription such as the AzureSubscriptionId and ResourceUri."]
    #[serde(rename = "paymentChannelMetadata", default, skip_serializing_if = "Option::is_none")]
    pub payment_channel_metadata: Option<serde_json::Value>,
    #[doc = "The SaaS resource name."]
    #[serde(rename = "saasResourceName", default, skip_serializing_if = "Option::is_none")]
    pub saas_resource_name: Option<String>,
    #[doc = "The current Term id."]
    #[serde(rename = "termId", default, skip_serializing_if = "Option::is_none")]
    pub term_id: Option<String>,
    #[doc = "Whether the SaaS subscription will auto renew upon term end."]
    #[serde(rename = "autoRenew", default, skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    #[doc = "The environment in the publisher side for this resource."]
    #[serde(rename = "publisherTestEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub publisher_test_environment: Option<String>,
    #[doc = "The saas subscription id used for tenant to subscription level migration request."]
    #[serde(rename = "saasSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub saas_subscription_id: Option<String>,
    #[doc = "The saas session id used for dev service migration request."]
    #[serde(rename = "saasSessionId", default, skip_serializing_if = "Option::is_none")]
    pub saas_session_id: Option<String>,
}
impl SaasCreationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod saas_creation_properties {
    use super::*;
    #[doc = "The Payment channel for the SaasSubscription."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PaymentChannelType")]
    pub enum PaymentChannelType {
        SubscriptionDelegated,
        CustomerDelegated,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PaymentChannelType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PaymentChannelType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PaymentChannelType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SubscriptionDelegated => serializer.serialize_unit_variant("PaymentChannelType", 0u32, "SubscriptionDelegated"),
                Self::CustomerDelegated => serializer.serialize_unit_variant("PaymentChannelType", 1u32, "CustomerDelegated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "saas resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaasProperties {
    #[doc = "The SaaS Subscription Status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<saas_properties::Status>,
    #[doc = "The current Term object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<saas_properties::Term>,
    #[doc = "Whether the current term is a Free Trial term"]
    #[serde(rename = "isFreeTrial", default, skip_serializing_if = "Option::is_none")]
    pub is_free_trial: Option<bool>,
    #[doc = "The created date of this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[doc = "The last modifier date if this resource."]
    #[serde(rename = "lastModified", default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
}
impl SaasProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod saas_properties {
    use super::*;
    #[doc = "The SaaS Subscription Status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        NotStarted,
        PendingFulfillmentStart,
        Subscribed,
        Unsubscribed,
        Suspended,
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
                Self::NotStarted => serializer.serialize_unit_variant("Status", 0u32, "NotStarted"),
                Self::PendingFulfillmentStart => serializer.serialize_unit_variant("Status", 1u32, "PendingFulfillmentStart"),
                Self::Subscribed => serializer.serialize_unit_variant("Status", 2u32, "Subscribed"),
                Self::Unsubscribed => serializer.serialize_unit_variant("Status", 3u32, "Unsubscribed"),
                Self::Suspended => serializer.serialize_unit_variant("Status", 4u32, "Suspended"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current Term object."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Term {
        #[doc = "The unit indicating Monthly / Yearly"]
        #[serde(rename = "termUnit", default, skip_serializing_if = "Option::is_none")]
        pub term_unit: Option<String>,
        #[doc = "The start date of the current term"]
        #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
        pub start_date: Option<String>,
        #[doc = "The end date of the current term"]
        #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
        pub end_date: Option<String>,
    }
    impl Term {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "SaaS REST API resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaasResource {
    #[doc = "saas properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource uri"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "the resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
impl SaasResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SaaS REST API resource definition for creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaasResourceCreation {
    #[doc = "The resource uri"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "the resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "Resource location. Only value allowed for SaaS is 'global'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "properties for creation saas"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SaasCreationProperties>,
}
impl SaasResourceCreation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "saas resources response with continuation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SaasResourceResponseWithContinuation {
    #[doc = "the next link to query to get the remaining results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "the value of response."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SaasResource>,
}
impl azure_core::Continuable for SaasResourceResponseWithContinuation {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SaasResourceResponseWithContinuation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "the resource tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "delete Options"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteOptions {
    #[doc = "whether it is unsubscribeOnly"]
    #[serde(rename = "unsubscribeOnly", default, skip_serializing_if = "Option::is_none")]
    pub unsubscribe_only: Option<bool>,
    #[doc = "The reasonCode"]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<f64>,
    #[doc = "the feedback"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
}
impl DeleteOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource Move Options"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveResource {
    #[doc = "The target resource group uri for the move"]
    #[serde(rename = "targetResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
    #[doc = "The resource uris to move"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
}
impl MoveResource {
    pub fn new() -> Self {
        Self::default()
    }
}
