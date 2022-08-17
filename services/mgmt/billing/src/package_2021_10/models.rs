#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The amount."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Amount {
    #[doc = "The currency for the amount value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "Amount value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl Amount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A billing subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscription {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The billing properties of a subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingSubscriptionProperties>,
}
impl BillingSubscription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A billing subscription alias."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscriptionAlias {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Billing subscription alias properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingSubscriptionAliasProperties>,
}
impl BillingSubscriptionAlias {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of billing subscription aliases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscriptionAliasListResult {
    #[doc = "The list of billing subscription aliases."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BillingSubscriptionAlias>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BillingSubscriptionAliasListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BillingSubscriptionAliasListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Billing subscription alias properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscriptionAliasProperties {
    #[serde(flatten)]
    pub billing_subscription_properties: BillingSubscriptionProperties,
    #[doc = "The ID of the billing subscription with the subscription alias."]
    #[serde(rename = "billingSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub billing_subscription_id: Option<String>,
}
impl BillingSubscriptionAliasProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request to merge two billing subscriptions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscriptionMergeRequest {
    #[doc = "The ID of the target billing subscription that will be merged with the source subscription provided in the request."]
    #[serde(rename = "targetBillingSubscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub target_billing_subscription_name: Option<String>,
    #[doc = "The quantity of the source billing subscription that will be merged with the target billing subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}
impl BillingSubscriptionMergeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The billing properties of a subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscriptionProperties {
    #[doc = "Indicates whether auto renewal is turned on or off for a subscription."]
    #[serde(rename = "autoRenew", default, skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<billing_subscription_properties::AutoRenew>,
    #[doc = "The provisioning tenant of the subscription."]
    #[serde(rename = "beneficiaryTenantId", default, skip_serializing_if = "Option::is_none")]
    pub beneficiary_tenant_id: Option<String>,
    #[doc = "The billing frequency of the subscription in the ISO8601 format. Example: P1M, P3M, P1Y"]
    #[serde(rename = "billingFrequency", default, skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<String>,
    #[doc = "The ID of the billing profile to which the subscription is billed. This field is only applicable for Microsoft Customer Agreement billing accounts."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "Dictionary of billing policies associated with the subscription."]
    #[serde(rename = "billingPolicies", default, skip_serializing_if = "Option::is_none")]
    pub billing_policies: Option<serde_json::Value>,
    #[doc = "The display name of the billing profile to which the subscription is billed. This field is only applicable for Microsoft Customer Agreement billing accounts."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The name of the billing profile to which the subscription is billed. This field is only applicable for Microsoft Customer Agreement billing accounts."]
    #[serde(rename = "billingProfileName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_name: Option<String>,
    #[doc = "The cost center applied to the subscription. This field is only available for consumption subscriptions of Microsoft Customer Agreement Type billing accounts."]
    #[serde(rename = "consumptionCostCenter", default, skip_serializing_if = "Option::is_none")]
    pub consumption_cost_center: Option<String>,
    #[doc = "The ID of the customer for whom the subscription was created. The field is applicable only for Microsoft Partner Agreement billing accounts."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "The name of the customer for whom the subscription was created. The field is applicable only for Microsoft Partner Agreement billing accounts."]
    #[serde(rename = "customerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub customer_display_name: Option<String>,
    #[doc = "The name of the subscription."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The enrollment Account ID associated with the subscription. This field is available only for the Enterprise Agreement billing accounts."]
    #[serde(rename = "enrollmentAccountId", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_account_id: Option<String>,
    #[doc = "The enrollment Account name associated with the subscription. This field is available only for the Enterprise Agreement billing accounts."]
    #[serde(rename = "enrollmentAccountDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_account_display_name: Option<String>,
    #[doc = "The billing properties that can be modified. This field is available only for the Enterprise Agreement billing accounts."]
    #[serde(rename = "enrollmentAccountSubscriptionDetails", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_account_subscription_details: Option<EnrollmentAccountSubscriptionDetails>,
    #[doc = "The ID of the invoice section to which the subscription is billed. The field is applicable only for Microsoft Partner Agreement billing accounts."]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "The display name of the invoice section to which the subscription is billed. The field is applicable only for Microsoft Partner Agreement billing accounts."]
    #[serde(rename = "invoiceSectionDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_display_name: Option<String>,
    #[doc = "The name of the invoice section to which the subscription is billed. The field is applicable only for Microsoft Partner Agreement billing accounts."]
    #[serde(rename = "invoiceSectionName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_name: Option<String>,
    #[doc = "The amount."]
    #[serde(rename = "lastMonthCharges", default, skip_serializing_if = "Option::is_none")]
    pub last_month_charges: Option<Amount>,
    #[doc = "The amount."]
    #[serde(rename = "monthToDateCharges", default, skip_serializing_if = "Option::is_none")]
    pub month_to_date_charges: Option<Amount>,
    #[doc = "The next billing cycle details of the subscription."]
    #[serde(rename = "nextBillingCycleDetails", default, skip_serializing_if = "Option::is_none")]
    pub next_billing_cycle_details: Option<NextBillingCycleDetails>,
    #[doc = "The offer ID for the subscription. This field is only available for the Microsoft Online Services Program billing accounts."]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "The category of the product for which the subscription is purchased. Possible values include: AzureSupport, Hardware, ReservationOrder, SaaS, SavingsPlanOrder, Software, UsageBased, Other"]
    #[serde(rename = "productCategory", default, skip_serializing_if = "Option::is_none")]
    pub product_category: Option<String>,
    #[doc = "The type of the product for which the subscription is purchased"]
    #[serde(rename = "productType", default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[doc = "The ID of the product for which the subscription is purchased"]
    #[serde(rename = "productTypeId", default, skip_serializing_if = "Option::is_none")]
    pub product_type_id: Option<String>,
    #[doc = "The purchase date of the subscription in UTC time."]
    #[serde(rename = "purchaseDate", default, with = "azure_core::date::rfc3339::option")]
    pub purchase_date: Option<time::OffsetDateTime>,
    #[doc = "The number of licenses purchased for the subscription"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "Details of the reseller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reseller: Option<Reseller>,
    #[doc = "The term details of the subscription at renewal."]
    #[serde(rename = "renewalTermDetails", default, skip_serializing_if = "Option::is_none")]
    pub renewal_term_details: Option<RenewalTermDetails>,
    #[doc = "The SKU description of the product for which the subscription is purchased. This field is only available for Microsoft Customer Agreement billing accounts."]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[doc = "The SKU ID of the product for which the subscription is purchased. This field is only available for Microsoft Customer Agreement billing accounts."]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "The status of the subscription. This field is not available for Enterprise Agreement billing accounts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<billing_subscription_properties::Status>,
    #[doc = "The ID of the usage-based subscription. This field is only available for usage-based subscriptions of Microsoft Customer Agreement billing accounts."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The suspension reason for the subscription. This field is not available for Enterprise Agreement billing accounts."]
    #[serde(rename = "suspensionReasons", default, skip_serializing_if = "Vec::is_empty")]
    pub suspension_reasons: Vec<String>,
    #[doc = "The duration for which you can use the subscription. Example P1Y and P1M"]
    #[serde(rename = "termDuration", default, skip_serializing_if = "Option::is_none")]
    pub term_duration: Option<String>,
    #[doc = "The start date of the term in UTC time."]
    #[serde(rename = "termStartDate", default, with = "azure_core::date::rfc3339::option")]
    pub term_start_date: Option<time::OffsetDateTime>,
    #[doc = "The end date of the term in UTC time."]
    #[serde(rename = "termEndDate", default, with = "azure_core::date::rfc3339::option")]
    pub term_end_date: Option<time::OffsetDateTime>,
}
impl BillingSubscriptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_subscription_properties {
    use super::*;
    #[doc = "Indicates whether auto renewal is turned on or off for a subscription."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AutoRenew")]
    pub enum AutoRenew {
        Off,
        On,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AutoRenew {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AutoRenew {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AutoRenew {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Off => serializer.serialize_unit_variant("AutoRenew", 0u32, "Off"),
                Self::On => serializer.serialize_unit_variant("AutoRenew", 1u32, "On"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the subscription. This field is not available for Enterprise Agreement billing accounts."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Unknown,
        Active,
        Disabled,
        Deleted,
        Warned,
        Expiring,
        Expired,
        AutoRenew,
        Cancelled,
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
                Self::Unknown => serializer.serialize_unit_variant("Status", 0u32, "Unknown"),
                Self::Active => serializer.serialize_unit_variant("Status", 1u32, "Active"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 2u32, "Disabled"),
                Self::Deleted => serializer.serialize_unit_variant("Status", 3u32, "Deleted"),
                Self::Warned => serializer.serialize_unit_variant("Status", 4u32, "Warned"),
                Self::Expiring => serializer.serialize_unit_variant("Status", 5u32, "Expiring"),
                Self::Expired => serializer.serialize_unit_variant("Status", 6u32, "Expired"),
                Self::AutoRenew => serializer.serialize_unit_variant("Status", 7u32, "AutoRenew"),
                Self::Cancelled => serializer.serialize_unit_variant("Status", 8u32, "Cancelled"),
                Self::Suspended => serializer.serialize_unit_variant("Status", 9u32, "Suspended"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Request to split a billing subscription"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscriptionSplitRequest {
    #[doc = "The billing frequency of the target subscription in the ISO8601 format. Example: P1M, P3M, P1Y"]
    #[serde(rename = "billingFrequency", default, skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<String>,
    #[doc = "The quantity of the target product to which the subscription needs to be split into."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[doc = "The ID of the target product to which the subscription needs to be split into. This value is not same as the value returned in Get API call and can be retrieved from Catalog API to know the product id to split into."]
    #[serde(rename = "targetProductTypeId", default, skip_serializing_if = "Option::is_none")]
    pub target_product_type_id: Option<String>,
    #[doc = "The ID of the target product to which the subscription needs to be split into. This value is not same as the value returned in Get API call and can be retrieved from Catalog API to know the sku id to split into."]
    #[serde(rename = "targetSkuId", default, skip_serializing_if = "Option::is_none")]
    pub target_sku_id: Option<String>,
    #[doc = "The term duration of the target in ISO8601 format product to which the subscription needs to be split into. Example: P1M, P1Y"]
    #[serde(rename = "termDuration", default, skip_serializing_if = "Option::is_none")]
    pub term_duration: Option<String>,
}
impl BillingSubscriptionSplitRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of billing subscriptions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscriptionsListResult {
    #[doc = "The list of billing subscriptions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BillingSubscription>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BillingSubscriptionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BillingSubscriptionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DetachPaymentMethodEligibilityError {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The list of detach payment method eligibility errors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<DetachPaymentMethodErrorDetails>,
}
impl DetachPaymentMethodEligibilityError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error code of the detach payment method eligibility validation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DetachPaymentMethodEligibilityErrorCode")]
pub enum DetachPaymentMethodEligibilityErrorCode {
    AzureSubscriptions,
    RecurringCharges,
    ReservedInstances,
    OutstandingCharges,
    PendingCharges,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DetachPaymentMethodEligibilityErrorCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DetachPaymentMethodEligibilityErrorCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DetachPaymentMethodEligibilityErrorCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureSubscriptions => {
                serializer.serialize_unit_variant("DetachPaymentMethodEligibilityErrorCode", 0u32, "AzureSubscriptions")
            }
            Self::RecurringCharges => {
                serializer.serialize_unit_variant("DetachPaymentMethodEligibilityErrorCode", 1u32, "RecurringCharges")
            }
            Self::ReservedInstances => {
                serializer.serialize_unit_variant("DetachPaymentMethodEligibilityErrorCode", 2u32, "ReservedInstances")
            }
            Self::OutstandingCharges => {
                serializer.serialize_unit_variant("DetachPaymentMethodEligibilityErrorCode", 3u32, "OutstandingCharges")
            }
            Self::PendingCharges => serializer.serialize_unit_variant("DetachPaymentMethodEligibilityErrorCode", 4u32, "PendingCharges"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Error response indicates that payment method cannot be detached from billing profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DetachPaymentMethodEligibilityResult {
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<DetachPaymentMethodEligibilityError>,
}
impl DetachPaymentMethodEligibilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details of the detach payment method eligibility validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DetachPaymentMethodErrorDetails {
    #[doc = "Error code of the detach payment method eligibility validation response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<DetachPaymentMethodEligibilityErrorCode>,
    #[doc = "Error message for the detach payment method eligibility validation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl DetachPaymentMethodErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The billing properties that can be modified. This field is available only for the Enterprise Agreement billing accounts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrollmentAccountSubscriptionDetails {
    #[doc = "The current enrollment account status of the subscription. This field is available only for the Enterprise Agreement billing accounts."]
    #[serde(rename = "subscriptionEnrollmentAccountStatus", default, skip_serializing_if = "Option::is_none")]
    pub subscription_enrollment_account_status: Option<enrollment_account_subscription_details::SubscriptionEnrollmentAccountStatus>,
    #[doc = "The enrollment Account and the subscription association start date. This field is available only for the Enterprise Agreement billing accounts."]
    #[serde(rename = "enrollmentAccountStartDate", default, with = "azure_core::date::rfc3339::option")]
    pub enrollment_account_start_date: Option<time::OffsetDateTime>,
}
impl EnrollmentAccountSubscriptionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod enrollment_account_subscription_details {
    use super::*;
    #[doc = "The current enrollment account status of the subscription. This field is available only for the Enterprise Agreement billing accounts."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SubscriptionEnrollmentAccountStatus")]
    pub enum SubscriptionEnrollmentAccountStatus {
        Active,
        Cancelled,
        Expired,
        Deleted,
        TransferredOut,
        Transferring,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SubscriptionEnrollmentAccountStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SubscriptionEnrollmentAccountStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SubscriptionEnrollmentAccountStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("SubscriptionEnrollmentAccountStatus", 0u32, "Active"),
                Self::Cancelled => serializer.serialize_unit_variant("SubscriptionEnrollmentAccountStatus", 1u32, "Cancelled"),
                Self::Expired => serializer.serialize_unit_variant("SubscriptionEnrollmentAccountStatus", 2u32, "Expired"),
                Self::Deleted => serializer.serialize_unit_variant("SubscriptionEnrollmentAccountStatus", 3u32, "Deleted"),
                Self::TransferredOut => serializer.serialize_unit_variant("SubscriptionEnrollmentAccountStatus", 4u32, "TransferredOut"),
                Self::Transferring => serializer.serialize_unit_variant("SubscriptionEnrollmentAccountStatus", 5u32, "Transferring"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The details of the error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The sub details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<ErrorSubDetails>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
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
pub type ErrorSubDetails = Vec<serde_json::Value>;
#[doc = "Error code of the transfer validation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MoveBillingSubscriptionEligibilityErrorCode")]
pub enum MoveBillingSubscriptionEligibilityErrorCode {
    AccountIsLocked,
    AssetNotActive,
    AssetHasCap,
    BillingAccountInactive,
    BillingProfilePastDue,
    CrossBillingAccountNotAllowed,
    DestinationBillingProfileNotFound,
    DestinationBillingProfileInactive,
    DestinationBillingProfilePastDue,
    DestinationInvoiceSectionNotFound,
    DestinationInvoiceSectionInactive,
    InvalidDestination,
    InvalidSource,
    InvoiceSectionIsRestricted,
    InsufficientPermissionOnDestination,
    InsufficientPermissionOnSource,
    MarketplaceNotEnabledOnDestination,
    ProductNotFound,
    ProductInactive,
    ProductTypeNotSupported,
    SourceBillingProfilePastDue,
    SourceInvoiceSectionInactive,
    SubscriptionNotActive,
    SubscriptionTypeNotSupported,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MoveBillingSubscriptionEligibilityErrorCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MoveBillingSubscriptionEligibilityErrorCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MoveBillingSubscriptionEligibilityErrorCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AccountIsLocked => {
                serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 0u32, "AccountIsLocked")
            }
            Self::AssetNotActive => {
                serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 1u32, "AssetNotActive")
            }
            Self::AssetHasCap => serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 2u32, "AssetHasCap"),
            Self::BillingAccountInactive => {
                serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 3u32, "BillingAccountInactive")
            }
            Self::BillingProfilePastDue => {
                serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 4u32, "BillingProfilePastDue")
            }
            Self::CrossBillingAccountNotAllowed => {
                serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 5u32, "CrossBillingAccountNotAllowed")
            }
            Self::DestinationBillingProfileNotFound => serializer.serialize_unit_variant(
                "MoveBillingSubscriptionEligibilityErrorCode",
                6u32,
                "DestinationBillingProfileNotFound",
            ),
            Self::DestinationBillingProfileInactive => serializer.serialize_unit_variant(
                "MoveBillingSubscriptionEligibilityErrorCode",
                7u32,
                "DestinationBillingProfileInactive",
            ),
            Self::DestinationBillingProfilePastDue => serializer.serialize_unit_variant(
                "MoveBillingSubscriptionEligibilityErrorCode",
                8u32,
                "DestinationBillingProfilePastDue",
            ),
            Self::DestinationInvoiceSectionNotFound => serializer.serialize_unit_variant(
                "MoveBillingSubscriptionEligibilityErrorCode",
                9u32,
                "DestinationInvoiceSectionNotFound",
            ),
            Self::DestinationInvoiceSectionInactive => serializer.serialize_unit_variant(
                "MoveBillingSubscriptionEligibilityErrorCode",
                10u32,
                "DestinationInvoiceSectionInactive",
            ),
            Self::InvalidDestination => {
                serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 11u32, "InvalidDestination")
            }
            Self::InvalidSource => serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 12u32, "InvalidSource"),
            Self::InvoiceSectionIsRestricted => {
                serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 13u32, "InvoiceSectionIsRestricted")
            }
            Self::InsufficientPermissionOnDestination => serializer.serialize_unit_variant(
                "MoveBillingSubscriptionEligibilityErrorCode",
                14u32,
                "InsufficientPermissionOnDestination",
            ),
            Self::InsufficientPermissionOnSource => serializer.serialize_unit_variant(
                "MoveBillingSubscriptionEligibilityErrorCode",
                15u32,
                "InsufficientPermissionOnSource",
            ),
            Self::MarketplaceNotEnabledOnDestination => serializer.serialize_unit_variant(
                "MoveBillingSubscriptionEligibilityErrorCode",
                16u32,
                "MarketplaceNotEnabledOnDestination",
            ),
            Self::ProductNotFound => {
                serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 17u32, "ProductNotFound")
            }
            Self::ProductInactive => {
                serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 18u32, "ProductInactive")
            }
            Self::ProductTypeNotSupported => {
                serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 19u32, "ProductTypeNotSupported")
            }
            Self::SourceBillingProfilePastDue => {
                serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 20u32, "SourceBillingProfilePastDue")
            }
            Self::SourceInvoiceSectionInactive => {
                serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 21u32, "SourceInvoiceSectionInactive")
            }
            Self::SubscriptionNotActive => {
                serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 22u32, "SubscriptionNotActive")
            }
            Self::SubscriptionTypeNotSupported => {
                serializer.serialize_unit_variant("MoveBillingSubscriptionEligibilityErrorCode", 23u32, "SubscriptionTypeNotSupported")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Request parameters to transfer billing subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveBillingSubscriptionRequest {
    #[doc = "The destination enrollment account id."]
    #[serde(rename = "destinationEnrollmentAccountId", default, skip_serializing_if = "Option::is_none")]
    pub destination_enrollment_account_id: Option<String>,
    #[doc = "The destination invoice section id."]
    #[serde(rename = "destinationInvoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub destination_invoice_section_id: Option<String>,
}
impl MoveBillingSubscriptionRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The next billing cycle details of the subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NextBillingCycleDetails {
    #[doc = "The billing frequency of the subscription in the next billing cycle."]
    #[serde(rename = "billingFrequency", default, skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<String>,
}
impl NextBillingCycleDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Billing REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Identifies if the operation is a data operation."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
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
        #[doc = "Service provider: Microsoft.Billing."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed such as invoice and billing subscription."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type such as read, write and delete."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The list of billing operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The list of billing operations supported by the Microsoft.Billing resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
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
#[doc = "A payment method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentMethod {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a payment method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PaymentMethodProperties>,
}
impl PaymentMethod {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A payment method link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentMethodLink {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a payment method link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PaymentMethodLinkProperties>,
}
impl PaymentMethodLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a payment method link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentMethodLinkProperties {
    #[doc = "The properties of a payment method projection."]
    #[serde(rename = "paymentMethod", default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodProjectionProperties>,
}
impl PaymentMethodLinkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of payment method links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentMethodLinksListResult {
    #[doc = "The list of payment method links."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PaymentMethodLink>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaymentMethodLinksListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PaymentMethodLinksListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Logo of payment method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentMethodLogo {
    #[doc = "MIME type of the logo."]
    #[serde(rename = "mimeType", default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[doc = "Public URL of image of the logo."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl PaymentMethodLogo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a payment method projection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentMethodProjectionProperties {
    #[doc = "Id of payment method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The family of payment method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<payment_method_projection_properties::Family>,
    #[doc = "The type of payment method."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The account holder name for the payment method. This is only supported for payment methods with family CreditCard."]
    #[serde(rename = "accountHolderName", default, skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    #[doc = "The expiration month and year of the payment method. This is only supported for payment methods with family CreditCard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[doc = "Last four digits of payment method."]
    #[serde(rename = "lastFourDigits", default, skip_serializing_if = "Option::is_none")]
    pub last_four_digits: Option<String>,
    #[doc = "The display name of the payment method."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The list of logos for the payment method."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub logos: Vec<PaymentMethodLogo>,
    #[doc = "Status of the payment method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<payment_method_projection_properties::Status>,
}
impl PaymentMethodProjectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod payment_method_projection_properties {
    use super::*;
    #[doc = "The family of payment method."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Family")]
    pub enum Family {
        CreditCard,
        CheckWire,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Family {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Family {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Family {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CreditCard => serializer.serialize_unit_variant("Family", 0u32, "CreditCard"),
                Self::CheckWire => serializer.serialize_unit_variant("Family", 1u32, "CheckWire"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Status of the payment method."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "inactive")]
        Inactive,
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
                Self::Active => serializer.serialize_unit_variant("Status", 0u32, "active"),
                Self::Inactive => serializer.serialize_unit_variant("Status", 1u32, "inactive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of a payment method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentMethodProperties {
    #[doc = "The family of payment method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<payment_method_properties::Family>,
    #[doc = "The type of payment method."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The account holder name for the payment method. This is only supported for payment methods with family CreditCard."]
    #[serde(rename = "accountHolderName", default, skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    #[doc = "The expiration month and year of the payment method. This is only supported for payment methods with family CreditCard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[doc = "Last four digits of payment method."]
    #[serde(rename = "lastFourDigits", default, skip_serializing_if = "Option::is_none")]
    pub last_four_digits: Option<String>,
    #[doc = "The display name of the payment method."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The list of logos for the payment method."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub logos: Vec<PaymentMethodLogo>,
    #[doc = "Status of the payment method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<payment_method_properties::Status>,
}
impl PaymentMethodProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod payment_method_properties {
    use super::*;
    #[doc = "The family of payment method."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Family")]
    pub enum Family {
        CreditCard,
        CheckWire,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Family {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Family {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Family {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CreditCard => serializer.serialize_unit_variant("Family", 0u32, "CreditCard"),
                Self::CheckWire => serializer.serialize_unit_variant("Family", 1u32, "CheckWire"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Status of the payment method."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "inactive")]
        Inactive,
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
                Self::Active => serializer.serialize_unit_variant("Status", 0u32, "active"),
                Self::Inactive => serializer.serialize_unit_variant("Status", 1u32, "inactive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The payment method resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentMethodResource {
    #[doc = "The ID that uniquely identifies a payment method."]
    #[serde(rename = "paymentMethodId", default, skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
}
impl PaymentMethodResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of payment methods."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentMethodsListResult {
    #[doc = "The list of payment methods."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PaymentMethod>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PaymentMethodsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PaymentMethodsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The term details of the subscription at renewal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RenewalTermDetails {
    #[doc = "The billing frequency of the subscription."]
    #[serde(rename = "billingFrequency", default, skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<String>,
    #[doc = "The ID of the product."]
    #[serde(rename = "productTypeId", default, skip_serializing_if = "Option::is_none")]
    pub product_type_id: Option<String>,
    #[doc = "The number of licenses"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "The SKU ID of the product"]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "The term duration of the subscription. Example P1M and P1Y"]
    #[serde(rename = "termDuration", default, skip_serializing_if = "Option::is_none")]
    pub term_duration: Option<String>,
}
impl RenewalTermDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the reseller."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Reseller {
    #[doc = "The MPN ID of the reseller."]
    #[serde(rename = "resellerId", default, skip_serializing_if = "Option::is_none")]
    pub reseller_id: Option<String>,
    #[doc = "The name of the reseller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl Reseller {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details of the transfer eligibility validation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateMoveBillingSubscriptionEligibilityError {
    #[doc = "Error code of the transfer validation response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<MoveBillingSubscriptionEligibilityErrorCode>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Detailed error message explaining the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl ValidateMoveBillingSubscriptionEligibilityError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the transfer eligibility validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateMoveBillingSubscriptionEligibilityResult {
    #[doc = "Specifies whether the subscription is eligible to move."]
    #[serde(rename = "isMoveEligible", default, skip_serializing_if = "Option::is_none")]
    pub is_move_eligible: Option<bool>,
    #[doc = "Error details of the transfer eligibility validation"]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ValidateMoveBillingSubscriptionEligibilityError>,
}
impl ValidateMoveBillingSubscriptionEligibilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
