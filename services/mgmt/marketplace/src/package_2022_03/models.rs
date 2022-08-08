#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Notification update request payload details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcknowledgeOfferNotificationDetails {
    #[doc = "Gets or sets a value indicating whether acknowledge action flag is enabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acknowledge: Option<bool>,
    #[doc = "Gets or sets a value indicating whether dismiss action flag is enabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismiss: Option<bool>,
    #[doc = "Gets or sets a value indicating whether remove offer action flag is enabled"]
    #[serde(rename = "removeOffer", default, skip_serializing_if = "Option::is_none")]
    pub remove_offer: Option<bool>,
    #[doc = "Gets or sets added plans"]
    #[serde(rename = "addPlans", default, skip_serializing_if = "Vec::is_empty")]
    pub add_plans: Vec<String>,
    #[doc = "Gets or sets remove plans"]
    #[serde(rename = "removePlans", default, skip_serializing_if = "Vec::is_empty")]
    pub remove_plans: Vec<String>,
}
impl AcknowledgeOfferNotificationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Notification update request payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcknowledgeOfferNotificationProperties {
    #[doc = "Notification update request payload details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AcknowledgeOfferNotificationDetails>,
}
impl AcknowledgeOfferNotificationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Admin approval request resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdminRequestApprovalProperties {
    #[doc = "Gets or sets offer Id"]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "Gets display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets publisher Id"]
    #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[doc = "Gets or sets admin action"]
    #[serde(rename = "adminAction", default, skip_serializing_if = "Option::is_none")]
    pub admin_action: Option<admin_request_approval_properties::AdminAction>,
    #[doc = "Gets or sets Approved plans ids, empty in case of rejected"]
    #[serde(rename = "approvedPlans", default, skip_serializing_if = "Vec::is_empty")]
    pub approved_plans: Vec<String>,
    #[doc = "Gets or sets admin comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[doc = "Gets or sets admin details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrator: Option<String>,
    #[doc = "Gets list of plans with requesters details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plans: Vec<PlanRequesterDetails>,
    #[doc = "Gets or sets list of associated collection ids"]
    #[serde(rename = "collectionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub collection_ids: Vec<String>,
    #[doc = "The offer icon url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}
impl AdminRequestApprovalProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod admin_request_approval_properties {
    use super::*;
    #[doc = "Gets or sets admin action"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AdminAction")]
    pub enum AdminAction {
        Approved,
        Rejected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AdminAction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AdminAction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AdminAction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Approved => serializer.serialize_unit_variant("AdminAction", 0u32, "Approved"),
                Self::Rejected => serializer.serialize_unit_variant("AdminAction", 1u32, "Rejected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of admin request approval resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdminRequestApprovalsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AdminRequestApprovalsResource>,
    #[doc = "URL to get the next set of notifications list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AdminRequestApprovalsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Admin request approval resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdminRequestApprovalsResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Admin approval request resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdminRequestApprovalProperties>,
}
impl AdminRequestApprovalsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response object of query if are there existing offers in the collections."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnyExistingOffersInTheCollectionsResponse {
    #[doc = "Boolean answer, true if exists at least a single offer in an enabled collection, otherwise, false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}
impl AnyExistingOffersInTheCollectionsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Billing accounts response object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingAccountsResponse {
    #[doc = "Billing accounts list"]
    #[serde(rename = "billingAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub billing_accounts: Vec<String>,
}
impl BillingAccountsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Bulk collection details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BulkCollectionsDetails {
    #[doc = "collection ids list that the action is performed on"]
    #[serde(rename = "collectionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub collection_ids: Vec<String>,
    #[doc = "Action to perform (For example: EnableCollections, DisableCollections)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}
impl BulkCollectionsDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Bulk collections action properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BulkCollectionsPayload {
    #[doc = "Bulk collection details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BulkCollectionsDetails>,
}
impl BulkCollectionsPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The bulk collections response. The response contains two lists that indicate for each collection whether the operation succeeded or failed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BulkCollectionsResponse {
    #[doc = "Succeeded collections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub succeeded: Vec<CollectionsDetails>,
    #[doc = "Failed collections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub failed: Vec<CollectionsDetails>,
}
impl BulkCollectionsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Collection data structure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Collection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The collection details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CollectionProperties>,
}
impl Collection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The collection details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectionProperties {
    #[doc = "Gets collection Id."]
    #[serde(rename = "collectionId", default, skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[doc = "Gets or sets collection name."]
    #[serde(rename = "collectionName", default, skip_serializing_if = "Option::is_none")]
    pub collection_name: Option<String>,
    #[doc = "Gets or sets the association with Commercial's Billing Account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim: Option<String>,
    #[doc = "Indicating whether all subscriptions are selected (=true) or not (=false)."]
    #[serde(rename = "allSubscriptions", default, skip_serializing_if = "Option::is_none")]
    pub all_subscriptions: Option<bool>,
    #[doc = "Indicating whether all items are approved for this collection (=true) or not (=false)."]
    #[serde(rename = "approveAllItems", default, skip_serializing_if = "Option::is_none")]
    pub approve_all_items: Option<bool>,
    #[doc = "Gets the modified date of all items approved."]
    #[serde(rename = "approveAllItemsModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub approve_all_items_modified_at: Option<time::OffsetDateTime>,
    #[doc = "Gets or sets subscription ids list. Empty list indicates all subscriptions are selected, null indicates no update is done, explicit list indicates the explicit selected subscriptions. On insert, null is considered as bad request"]
    #[serde(rename = "subscriptionsList", default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions_list: Vec<String>,
    #[doc = "Indicating whether the collection is enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Gets the number of offers associated with the collection."]
    #[serde(rename = "numberOfOffers", default, skip_serializing_if = "Option::is_none")]
    pub number_of_offers: Option<i64>,
}
impl CollectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection name and id."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectionsDetails {
    #[doc = "Collection name."]
    #[serde(rename = "collectionName", default, skip_serializing_if = "Option::is_none")]
    pub collection_name: Option<String>,
    #[doc = "Collection id."]
    #[serde(rename = "collectionId", default, skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
}
impl CollectionsDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectionsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Collection>,
    #[doc = "URL to get the next set of offer list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl CollectionsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The subscriptions list to get the related collections"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectionsToSubscriptionsMappingPayload {
    #[doc = "The subscriptions list to get the related collections"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CollectionsToSubscriptionsMappingProperties>,
}
impl CollectionsToSubscriptionsMappingPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The subscriptions list to get the related collections"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectionsToSubscriptionsMappingProperties {
    #[doc = "Subscriptions ids list"]
    #[serde(rename = "subscriptionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub subscription_ids: Vec<String>,
}
impl CollectionsToSubscriptionsMappingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A map of collections subscriptions details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectionsToSubscriptionsMappingResponse {
    #[doc = "Collections - subscriptions mapping details, map key is collectionId. In case subscriptions list is null or empty: no subscription from the list is related to that collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<CollectionsToSubscriptionsMappingResponseProperties>,
}
impl CollectionsToSubscriptionsMappingResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collections - subscriptions mapping details, map key is collectionId. In case subscriptions list is null or empty: no subscription from the list is related to that collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectionsToSubscriptionsMappingResponseProperties {}
impl CollectionsToSubscriptionsMappingResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object of plans per context."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContextAndPlansDetails {
    #[doc = "Plan's context, e.g. subscription ID, tenant ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "List of plan IDs."]
    #[serde(rename = "planIds", default, skip_serializing_if = "Vec::is_empty")]
    pub plan_ids: Vec<String>,
}
impl ContextAndPlansDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates Microsoft.Marketplace service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
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
pub mod error_response {
    use super::*;
    #[doc = "The details of the error."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Error code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Error message indicating why the operation failed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The type of identity that creates/modifies resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IdentityType")]
pub enum IdentityType {
    User,
    Application,
    ManagedIdentity,
    Key,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::User => serializer.serialize_unit_variant("IdentityType", 0u32, "User"),
            Self::Application => serializer.serialize_unit_variant("IdentityType", 1u32, "Application"),
            Self::ManagedIdentity => serializer.serialize_unit_variant("IdentityType", 2u32, "ManagedIdentity"),
            Self::Key => serializer.serialize_unit_variant("IdentityType", 3u32, "Key"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Payload object for upsert offer with multiple context and plans."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MultiContextAndPlansPayload {
    #[doc = "Object describes multiple context and plans."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MultiContextAndPlansProperties>,
}
impl MultiContextAndPlansPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object describes multiple context and plans."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MultiContextAndPlansProperties {
    #[doc = "The offer ID which contains the plans."]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "The offer's eTag."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "plansContext", default, skip_serializing_if = "Vec::is_empty")]
    pub plans_context: Vec<ContextAndPlansDetails>,
}
impl MultiContextAndPlansProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "New plans notification details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NewNotifications {
    #[doc = "Gets offer id"]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "Gets offer display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets a value indicating whether future plans is enabled."]
    #[serde(rename = "isFuturePlansEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_future_plans_enabled: Option<bool>,
    #[doc = "Gets or sets the notification message id"]
    #[serde(rename = "messageCode", default, skip_serializing_if = "Option::is_none")]
    pub message_code: Option<i64>,
    #[doc = "Gets or sets the icon url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "Gets or sets removed plans notifications"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plans: Vec<PlanNotificationDetails>,
}
impl NewNotifications {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all new plans notifications for public offers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NewPlansNotificationsList {
    #[serde(rename = "newPlansNotifications", default, skip_serializing_if = "Vec::is_empty")]
    pub new_plans_notifications: Vec<NewNotifications>,
}
impl NewPlansNotificationsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the json payload for notifications settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationsSettingsProperties {
    #[doc = "Gets or sets list of notified recipients for new requests"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipients: Vec<Recipient>,
    #[doc = "Gets or sets whether to send email to all marketplace admins for new requests"]
    #[serde(rename = "sendToAllMarketplaceAdmins", default, skip_serializing_if = "Option::is_none")]
    pub send_to_all_marketplace_admins: Option<bool>,
}
impl NotificationsSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The privateStore offer data structure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Offer {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfferProperties>,
}
impl Offer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OfferListResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Offer>,
    #[doc = "URL to get the next set of offer list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OfferListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OfferListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OfferProperties {
    #[doc = "Offers unique id"]
    #[serde(rename = "uniqueOfferId", default, skip_serializing_if = "Option::is_none")]
    pub unique_offer_id: Option<String>,
    #[doc = "It will be displayed prominently in the marketplace"]
    #[serde(rename = "offerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub offer_display_name: Option<String>,
    #[doc = "Publisher name that will be displayed prominently in the marketplace"]
    #[serde(rename = "publisherDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_display_name: Option<String>,
    #[doc = "Identifier for purposes of race condition"]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Private store unique id"]
    #[serde(rename = "privateStoreId", default, skip_serializing_if = "Option::is_none")]
    pub private_store_id: Option<String>,
    #[doc = "Private store offer creation date"]
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[doc = "Private store offer modification date"]
    #[serde(rename = "modifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[doc = "Plan ids limitation for this offer"]
    #[serde(rename = "specificPlanIdsLimitation", default, skip_serializing_if = "Vec::is_empty")]
    pub specific_plan_ids_limitation: Vec<String>,
    #[doc = "Indicating whether the offer was not updated to db (true = not updated). If the allow list is identical to the existed one in db, the offer would not be updated."]
    #[serde(rename = "updateSuppressedDueIdempotence", default, skip_serializing_if = "Option::is_none")]
    pub update_suppressed_due_idempotence: Option<bool>,
    #[doc = "Icon File Uris"]
    #[serde(rename = "iconFileUris", default, skip_serializing_if = "Option::is_none")]
    pub icon_file_uris: Option<serde_json::Value>,
    #[doc = "Offer plans"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plans: Vec<Plan>,
}
impl OfferProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to list Marketplace operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Microsoft.Marketplace operations supported by the Microsoft.Marketplace resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SingleOperation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Plan {
    #[doc = "Identifier for this plan"]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "Text identifier for this plan"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Friendly name for the plan for display in the marketplace"]
    #[serde(rename = "planDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub plan_display_name: Option<String>,
    #[doc = "Plan accessibility"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<plan::Accessibility>,
    #[doc = "Alternative stack type"]
    #[serde(rename = "altStackReference", default, skip_serializing_if = "Option::is_none")]
    pub alt_stack_reference: Option<String>,
    #[doc = "Stack type (classic or arm)"]
    #[serde(rename = "stackType", default, skip_serializing_if = "Option::is_none")]
    pub stack_type: Option<String>,
}
impl Plan {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod plan {
    use super::*;
    #[doc = "Plan accessibility"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Accessibility")]
    pub enum Accessibility {
        Unknown,
        Public,
        PrivateTenantOnLevel,
        PrivateSubscriptionOnLevel,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Accessibility {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Accessibility {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Accessibility {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("Accessibility", 0u32, "Unknown"),
                Self::Public => serializer.serialize_unit_variant("Accessibility", 1u32, "Public"),
                Self::PrivateTenantOnLevel => serializer.serialize_unit_variant("Accessibility", 2u32, "PrivateTenantOnLevel"),
                Self::PrivateSubscriptionOnLevel => serializer.serialize_unit_variant("Accessibility", 3u32, "PrivateSubscriptionOnLevel"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Plan notification details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanNotificationDetails {
    #[doc = "Gets or sets the plan id"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Gets or sets the plan display name"]
    #[serde(rename = "planDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub plan_display_name: Option<String>,
}
impl PlanNotificationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Plan with requesters details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanRequesterDetails {
    #[doc = "Gets the plan id"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Gets the plan display name"]
    #[serde(rename = "planDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub plan_display_name: Option<String>,
    #[doc = "Gets requesters details list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requesters: Vec<UserRequestDetails>,
}
impl PlanRequesterDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The PrivateStore data structure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateStore {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the json payload on whether or not the private store is enabled for a given tenant"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateStoreProperties>,
}
impl PrivateStore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the json payload for the list of available private stores (between zero and one, inclusive)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateStoreList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateStore>,
    #[doc = "URL to get the next set of PrivateStore list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateStoreList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateStoreList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Get private store notifications state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateStoreNotificationsState {
    #[serde(rename = "stopSellNotifications", default, skip_serializing_if = "Vec::is_empty")]
    pub stop_sell_notifications: Vec<StopSellNotifications>,
    #[serde(rename = "newNotifications", default, skip_serializing_if = "Vec::is_empty")]
    pub new_notifications: Vec<NewNotifications>,
    #[serde(rename = "approvalRequests", default, skip_serializing_if = "Vec::is_empty")]
    pub approval_requests: Vec<RequestApprovalsDetails>,
}
impl PrivateStoreNotificationsState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Set the Operation for the POST method. Ping or Delete"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateStoreOperation")]
pub enum PrivateStoreOperation {
    DeletePrivateStoreOffer,
    DeletePrivateStoreCollection,
    DeletePrivateStoreCollectionOffer,
    Ping,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateStoreOperation {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateStoreOperation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateStoreOperation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DeletePrivateStoreOffer => serializer.serialize_unit_variant("PrivateStoreOperation", 0u32, "DeletePrivateStoreOffer"),
            Self::DeletePrivateStoreCollection => {
                serializer.serialize_unit_variant("PrivateStoreOperation", 1u32, "DeletePrivateStoreCollection")
            }
            Self::DeletePrivateStoreCollectionOffer => {
                serializer.serialize_unit_variant("PrivateStoreOperation", 2u32, "DeletePrivateStoreCollectionOffer")
            }
            Self::Ping => serializer.serialize_unit_variant("PrivateStoreOperation", 3u32, "Ping"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Describes the json payload on whether or not the private store is enabled for a given tenant"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateStoreProperties {
    #[doc = "Indicates private store availability"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability: Option<private_store_properties::Availability>,
    #[doc = "Private Store id"]
    #[serde(rename = "privateStoreId", default, skip_serializing_if = "Option::is_none")]
    pub private_store_id: Option<String>,
    #[doc = "Identifier for purposes of race condition"]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Private Store Name"]
    #[serde(rename = "privateStoreName", default, skip_serializing_if = "Option::is_none")]
    pub private_store_name: Option<String>,
    #[doc = "Tenant id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Is government"]
    #[serde(rename = "isGov", default, skip_serializing_if = "Option::is_none")]
    pub is_gov: Option<bool>,
    #[doc = "Gets list of associated collection ids"]
    #[serde(rename = "collectionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub collection_ids: Vec<String>,
    #[doc = "Gets or sets list of branding characteristics"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branding: Option<serde_json::Value>,
    #[doc = "Describes the json payload for notifications settings"]
    #[serde(rename = "notificationsSettings", default, skip_serializing_if = "Option::is_none")]
    pub notifications_settings: Option<NotificationsSettingsProperties>,
}
impl PrivateStoreProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_store_properties {
    use super::*;
    #[doc = "Indicates private store availability"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Availability")]
    pub enum Availability {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Availability {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Availability {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Availability {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("Availability", 0u32, "enabled"),
                Self::Disabled => serializer.serialize_unit_variant("Availability", 1u32, "disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Query approved plans details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryApprovedPlans {
    #[doc = "Offer id"]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "Offer plan ids"]
    #[serde(rename = "planIds", default, skip_serializing_if = "Vec::is_empty")]
    pub plan_ids: Vec<String>,
    #[doc = "List of subscription IDs"]
    #[serde(rename = "subscriptionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub subscription_ids: Vec<String>,
}
impl QueryApprovedPlans {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query approved plans response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryApprovedPlansDetails {
    #[doc = "Plan id"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Approved subscription ids list. In case all subscriptions are approved for a plan, allSubscriptions flag is true and list is empty ( else flag is set to false). In case both subscriptions list is empty and allSubscriptions flag is false, the plan is not approved for any subscription."]
    #[serde(rename = "subscriptionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub subscription_ids: Vec<String>,
    #[doc = "Indicates whether all subscriptions are approved for this plan"]
    #[serde(rename = "allSubscriptions", default, skip_serializing_if = "Option::is_none")]
    pub all_subscriptions: Option<bool>,
}
impl QueryApprovedPlansDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query approved plans payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryApprovedPlansPayload {
    #[doc = "Query approved plans details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueryApprovedPlans>,
}
impl QueryApprovedPlansPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query approved plans response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryApprovedPlansResponse {
    #[doc = "A list indicating for each plan which subscriptions are approved. Plan Id is unique"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<QueryApprovedPlansResponseDetails>,
}
impl QueryApprovedPlansResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type QueryApprovedPlansResponseDetails = Vec<QueryApprovedPlansDetails>;
#[doc = "List of offers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryOffers {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OfferProperties>,
    #[doc = "URL to get the next set of PrivateStore list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl QueryOffers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets the request plans with indication on each plan whether is approved by the admin, has pending request or not requested yet"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryRequestApproval {
    #[doc = "Gets or sets unique offer id."]
    #[serde(rename = "uniqueOfferId", default, skip_serializing_if = "Option::is_none")]
    pub unique_offer_id: Option<String>,
    #[doc = "Gets or sets the plans details"]
    #[serde(rename = "plansDetails", default, skip_serializing_if = "Option::is_none")]
    pub plans_details: Option<serde_json::Value>,
    #[doc = "Gets or sets e-tag field "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Gets or sets the notification message id"]
    #[serde(rename = "messageCode", default, skip_serializing_if = "Option::is_none")]
    pub message_code: Option<i64>,
}
impl QueryRequestApproval {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details to get the request plans statuses"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryRequestApprovalProperties {
    #[doc = "Request details needed to get the plans statuses"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RequestDetails>,
}
impl QueryRequestApprovalProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of offers IDs and list of user's subscriptions IDs to query the user's approved offers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryUserOffersDetails {
    #[doc = "List of offer IDs"]
    #[serde(rename = "offerIds", default, skip_serializing_if = "Vec::is_empty")]
    pub offer_ids: Vec<String>,
    #[doc = "List of subscription IDs"]
    #[serde(rename = "subscriptionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub subscription_ids: Vec<String>,
}
impl QueryUserOffersDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query user's offers properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryUserOffersProperties {
    #[doc = "List of offers IDs and list of user's subscriptions IDs to query the user's approved offers"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueryUserOffersDetails>,
}
impl QueryUserOffersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the json payload for a notified recipient for new requests"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Recipient {
    #[doc = "Principal ID"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Email Address"]
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[doc = "Display Name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl Recipient {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Approval request resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestApprovalProperties {
    #[doc = "Gets or sets unique offer id."]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "Gets offer display name"]
    #[serde(rename = "offerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub offer_display_name: Option<String>,
    #[doc = "The offer's publisher id"]
    #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[doc = "Gets or sets the plans details"]
    #[serde(rename = "plansDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub plans_details: Vec<PlanDetails>,
    #[doc = "Gets a value indicating whether the request is closed"]
    #[serde(rename = "isClosed", default, skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,
    #[doc = "Gets or sets the request approval message code"]
    #[serde(rename = "messageCode", default, skip_serializing_if = "Option::is_none")]
    pub message_code: Option<i64>,
}
impl RequestApprovalProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request approval resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestApprovalResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Approval request resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RequestApprovalProperties>,
}
impl RequestApprovalResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request approvals details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestApprovalsDetails {
    #[doc = "Gets offer id"]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "Gets offer display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets publisher id"]
    #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[doc = "Gets or sets the notification message id"]
    #[serde(rename = "messageCode", default, skip_serializing_if = "Option::is_none")]
    pub message_code: Option<i64>,
    #[doc = "Gets or sets the icon url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "Gets or sets removed plans notifications"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plans: Vec<PlanNotificationDetails>,
}
impl RequestApprovalsDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of admin request approval resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestApprovalsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RequestApprovalResource>,
    #[doc = "URL to get the next set of notifications list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl RequestApprovalsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request details needed to get the plans statuses"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestDetails {
    #[doc = "The offer's publisher id"]
    #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[doc = "Current plans list"]
    #[serde(rename = "planIds", default, skip_serializing_if = "Vec::is_empty")]
    pub plan_ids: Vec<String>,
    #[doc = "Gets or sets the subscription id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
impl RequestDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Read only system data"]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Microsoft.Marketplace REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SingleOperation {
    #[doc = "Operation ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<single_operation::Display>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Operation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl SingleOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod single_operation {
    use super::*;
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.Marketplace"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Friendly description for the operation,"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Stop sell notification details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StopSellNotifications {
    #[doc = "Gets offer id"]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "Gets offer display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets a value indicating whether entire offer is in stop sell or only few of its plans"]
    #[serde(rename = "isEntire", default, skip_serializing_if = "Option::is_none")]
    pub is_entire: Option<bool>,
    #[doc = "Gets or sets the notification message id"]
    #[serde(rename = "messageCode", default, skip_serializing_if = "Option::is_none")]
    pub message_code: Option<i64>,
    #[doc = "Gets or sets the icon url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "Gets or sets removed plans notifications"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plans: Vec<PlanNotificationDetails>,
}
impl StopSellNotifications {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of stop sell offers and plans notifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StopSellOffersPlansNotificationsList {
    #[serde(rename = "stopSellNotifications", default, skip_serializing_if = "Vec::is_empty")]
    pub stop_sell_notifications: Vec<StopSellOffersPlansNotificationsListProperties>,
}
impl StopSellOffersPlansNotificationsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of stop sell offers and plans notifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StopSellOffersPlansNotificationsListProperties {
    #[doc = "The offer id"]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "The offer display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "A value indicating whether entire offer is in stop sell or only few of its plans"]
    #[serde(rename = "isEntire", default, skip_serializing_if = "Option::is_none")]
    pub is_entire: Option<bool>,
    #[doc = "The notification message code"]
    #[serde(rename = "messageCode", default, skip_serializing_if = "Option::is_none")]
    pub message_code: Option<i64>,
    #[doc = "The icon url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "The list of removed plans notifications"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plans: Vec<PlanNotificationDetails>,
    #[doc = "True if the offer has public plans"]
    #[serde(rename = "publicContext", default, skip_serializing_if = "Option::is_none")]
    pub public_context: Option<bool>,
    #[doc = "The subscriptions related to private plans"]
    #[serde(rename = "subscriptionsIds", default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions_ids: Vec<String>,
}
impl StopSellOffersPlansNotificationsListProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private plans subscriptions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StopSellSubscriptions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<String>,
}
impl StopSellSubscriptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Subscription {
    #[doc = "The fully qualified ID for the subscription. For example, /subscriptions/00000000-0000-0000-0000-000000000000."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The subscription display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The subscription state. Possible values are Enabled, Warned, PastDue, Disabled, and Deleted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<subscription::State>,
}
impl Subscription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription {
    use super::*;
    #[doc = "The subscription state. Possible values are Enabled, Warned, PastDue, Disabled, and Deleted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Enabled,
        Warned,
        PastDue,
        Disabled,
        Deleted,
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
                Self::Enabled => serializer.serialize_unit_variant("State", 0u32, "Enabled"),
                Self::Warned => serializer.serialize_unit_variant("State", 1u32, "Warned"),
                Self::PastDue => serializer.serialize_unit_variant("State", 2u32, "PastDue"),
                Self::Disabled => serializer.serialize_unit_variant("State", 3u32, "Disabled"),
                Self::Deleted => serializer.serialize_unit_variant("State", 4u32, "Deleted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of subscription Ids in the private store"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionsContextList {
    #[serde(rename = "subscriptionsIds", default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions_ids: Vec<String>,
}
impl SubscriptionsContextList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription list operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionsResponse {
    #[doc = "An array of subscriptions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Subscription>,
    #[doc = "The skip token to retrieve the next page."]
    #[serde(rename = "skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
    #[doc = "Number of subscriptions on the page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl SubscriptionsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Read only system data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that creates/modifies resources"]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<IdentityType>,
    #[doc = "The timestamp of resource creation (UTC)"]
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that creates/modifies resources"]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<IdentityType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Transfer offers response details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransferOffersDetails {
    #[doc = "Target collections ids"]
    #[serde(rename = "targetCollections", default, skip_serializing_if = "Vec::is_empty")]
    pub target_collections: Vec<String>,
    #[doc = "Operation to perform (For example: Copy or Move)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Offers ids list to transfer from source collection to target collection(s)"]
    #[serde(rename = "offerIdsList", default, skip_serializing_if = "Vec::is_empty")]
    pub offer_ids_list: Vec<String>,
}
impl TransferOffersDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Transfer offers properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransferOffersProperties {
    #[doc = "Transfer offers response details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TransferOffersDetails>,
}
impl TransferOffersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The transfer items response. The response contains two lists that indicate for each collection whether the operation succeeded or failed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransferOffersResponse {
    #[doc = "Succeeded collections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub succeeded: Vec<CollectionsDetails>,
    #[doc = "Failed collections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub failed: Vec<CollectionsDetails>,
}
impl TransferOffersResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "user request details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserRequestDetails {
    #[doc = "Gets user id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[doc = "Gets request date"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[doc = "Gets justification"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    #[doc = "Gets the subscription id that the user is requesting to add the plan to"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Gets the subscription name that the user is requesting to add the plan to"]
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
}
impl UserRequestDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Withdraw properties details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WithdrawDetails {
    #[doc = "Gets or sets Plan Id "]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "The offer's publisher id"]
    #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
}
impl WithdrawDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Withdraw properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WithdrawProperties {
    #[doc = "Withdraw properties details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WithdrawDetails>,
}
impl WithdrawProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection name and related subscriptions list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CollectionsSubscriptionsMappingDetails {
    #[doc = "Collection name"]
    #[serde(rename = "collectionName", default, skip_serializing_if = "Option::is_none")]
    pub collection_name: Option<String>,
    #[doc = "Subscriptions ids list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<String>,
}
impl CollectionsSubscriptionsMappingDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Return plan with request details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanDetails {
    #[doc = "Gets or sets Plan Id "]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Gets the plan status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<plan_details::Status>,
    #[doc = "Gets request date"]
    #[serde(rename = "requestDate", default, skip_serializing_if = "Option::is_none")]
    pub request_date: Option<serde_json::Value>,
    #[doc = "Gets or sets user's justification for the plan's request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    #[doc = "Gets or sets the subscription id that the user is requesting to add the plan to"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Gets or sets the subscription name that the user is requesting to add the plan to"]
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
}
impl PlanDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod plan_details {
    use super::*;
    #[doc = "Gets the plan status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Pending,
        Rejected,
        Approved,
        None,
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
                Self::Rejected => serializer.serialize_unit_variant("Status", 1u32, "Rejected"),
                Self::Approved => serializer.serialize_unit_variant("Status", 2u32, "Approved"),
                Self::None => serializer.serialize_unit_variant("Status", 3u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
