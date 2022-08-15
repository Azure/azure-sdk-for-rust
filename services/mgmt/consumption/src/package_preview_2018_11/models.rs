#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Address details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Address {
    #[doc = "Address Line1."]
    #[serde(rename = "addressLine1", default, skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[doc = "Address Line2."]
    #[serde(rename = "addressLine2", default, skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[doc = "Address Line3."]
    #[serde(rename = "addressLine3", default, skip_serializing_if = "Option::is_none")]
    pub address_line3: Option<String>,
    #[doc = "Address City."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Address Region."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "Country code uses ISO2, 2-digit format.."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Postal Code."]
    #[serde(rename = "postalCode", default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "Phone Number."]
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl Address {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object to represent monetary quantities."]
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
#[doc = "A billing account resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the billing account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingAccountProperties>,
}
impl BillingAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the billing account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingAccountProperties {
    #[doc = "The Company this billing account belongs to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[doc = "The billing account Type."]
    #[serde(rename = "accountType", default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<billing_account_properties::AccountType>,
    #[doc = "Address details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[doc = "The ISO currency, for example, USD."]
    #[serde(rename = "defaultCurrency", default, skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<String>,
    #[doc = "The country associated with billing account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Agreements associated with billing account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreements: Option<String>,
    #[doc = "The invoiceSections associated to the billing account."]
    #[serde(rename = "invoiceSections", default, skip_serializing_if = "Vec::is_empty")]
    pub invoice_sections: Vec<InvoiceSection>,
    #[doc = "The billing profiles associated to the billing account."]
    #[serde(rename = "billingProfiles", default, skip_serializing_if = "Vec::is_empty")]
    pub billing_profiles: Vec<BillingProfile>,
    #[doc = "Current entity level details"]
    #[serde(rename = "enrollmentDetails", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_details: Option<Enrollment>,
    #[doc = "The departments associated to the enrollment."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub departments: Vec<Department>,
    #[doc = "The accounts associated to the enrollment."]
    #[serde(rename = "enrollmentAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub enrollment_accounts: Vec<EnrollmentAccount>,
}
impl BillingAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_account_properties {
    use super::*;
    #[doc = "The billing account Type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccountType {
        CommerceRoot,
        Enrollment,
    }
}
#[doc = "A billing profile resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProfile {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the billing profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingProfileProperties>,
}
impl BillingProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the billing profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProfileProperties {
    #[doc = "The billing profile name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Purchase order number."]
    #[serde(rename = "poNumber", default, skip_serializing_if = "Option::is_none")]
    pub po_number: Option<String>,
    #[doc = "Address details."]
    #[serde(rename = "billingAddress", default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    #[doc = "Billing contact."]
    #[serde(rename = "billingContact", default, skip_serializing_if = "Option::is_none")]
    pub billing_contact: Option<String>,
    #[doc = "Email invoice."]
    #[serde(rename = "emailInvoice", default, skip_serializing_if = "Option::is_none")]
    pub email_invoice: Option<bool>,
    #[doc = "Invoice day."]
    #[serde(rename = "invoiceDay", default, skip_serializing_if = "Option::is_none")]
    pub invoice_day: Option<i64>,
    #[doc = "Currency on the billing profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}
impl BillingProfileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A charge summary resource by billing account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChargeSummaryByBillingAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the charge summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ChargeSummaryProperties>,
}
impl ChargeSummaryByBillingAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A charge summary resource by billing profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChargeSummaryByBillingProfile {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the charge summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ChargeSummaryProperties>,
}
impl ChargeSummaryByBillingProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A charge summary resource by invoiceSection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChargeSummaryByInvoiceSection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the charge summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ChargeSummaryProperties>,
}
impl ChargeSummaryByInvoiceSection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the charge summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChargeSummaryProperties {
    #[doc = "The id of the billing period resource that the usage belongs to."]
    #[serde(rename = "billingPeriodId", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_id: Option<String>,
    #[doc = "Billing period start date."]
    #[serde(rename = "usageStart", default, skip_serializing_if = "Option::is_none")]
    pub usage_start: Option<String>,
    #[doc = "Billing period end date."]
    #[serde(rename = "usageEnd", default, skip_serializing_if = "Option::is_none")]
    pub usage_end: Option<String>,
    #[doc = "Object to represent monetary quantities."]
    #[serde(rename = "azureCharges", default, skip_serializing_if = "Option::is_none")]
    pub azure_charges: Option<Amount>,
    #[doc = "Object to represent monetary quantities."]
    #[serde(rename = "chargesBilledSeparately", default, skip_serializing_if = "Option::is_none")]
    pub charges_billed_separately: Option<Amount>,
    #[doc = "Object to represent monetary quantities."]
    #[serde(rename = "marketplaceCharges", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_charges: Option<Amount>,
    #[doc = "The id of the billing account resource that the charge belongs to."]
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<String>,
    #[doc = "The id of the billing profile resource that the charge belongs to."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The id of the invoice section resource that the charge belongs to."]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
}
impl ChargeSummaryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing charge summary by billing account. It contains a list of available change summaries in reverse chronological order by billing period."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChargesListByBillingAccount {
    #[doc = "The list of charge summary by billing account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ChargeSummaryByBillingAccount>,
}
impl ChargesListByBillingAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing charge summary by billing profile. It contains a list of available change summaries in reverse chronological order by billing period."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChargesListByBillingProfile {
    #[doc = "The list of charge summary by billing profile."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ChargeSummaryByBillingProfile>,
}
impl ChargesListByBillingProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing charge summary by invoiceSection. It contains a list of available change summaries in reverse chronological order by billing period."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChargesListByInvoiceSection {
    #[doc = "The list of charge summary by invoiceSection."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ChargeSummaryByInvoiceSection>,
}
impl ChargesListByInvoiceSection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summary of credit balances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreditBalanceSummary {
    #[doc = "Object to represent monetary quantities."]
    #[serde(rename = "estimatedBalance", default, skip_serializing_if = "Option::is_none")]
    pub estimated_balance: Option<Amount>,
    #[doc = "Object to represent monetary quantities."]
    #[serde(rename = "currentBalance", default, skip_serializing_if = "Option::is_none")]
    pub current_balance: Option<Amount>,
}
impl CreditBalanceSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A credit summary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreditSummary {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the credit summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CreditSummaryProperties>,
}
impl CreditSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the credit summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreditSummaryProperties {
    #[doc = "Summary of credit balances."]
    #[serde(rename = "balanceSummary", default, skip_serializing_if = "Option::is_none")]
    pub balance_summary: Option<CreditBalanceSummary>,
    #[doc = "Object to represent monetary quantities."]
    #[serde(rename = "pendingCreditAdjustments", default, skip_serializing_if = "Option::is_none")]
    pub pending_credit_adjustments: Option<Amount>,
    #[doc = "Object to represent monetary quantities."]
    #[serde(rename = "expiredCredit", default, skip_serializing_if = "Option::is_none")]
    pub expired_credit: Option<Amount>,
    #[doc = "Object to represent monetary quantities."]
    #[serde(rename = "pendingEligibleCharges", default, skip_serializing_if = "Option::is_none")]
    pub pending_eligible_charges: Option<Amount>,
}
impl CreditSummaryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A department resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Department {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the department."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DepartmentProperties>,
}
impl Department {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the department."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DepartmentProperties {
    #[doc = "The name for department."]
    #[serde(rename = "departmentName", default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[doc = "The cost center name."]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "The status for department."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Associated enrollment accounts. By default this is not populated, unless it's specified in $expand."]
    #[serde(rename = "enrollmentAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub enrollment_accounts: Vec<EnrollmentAccount>,
}
impl DepartmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A secure URL that can be used to download a an entity until the URL expires."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DownloadUrl {
    #[doc = "The URL to the PDF file."]
    #[serde(rename = "downloadUrl", default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[doc = "The time in UTC at which this download URL will expire."]
    #[serde(rename = "expiryTime", default, skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
}
impl DownloadUrl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Current entity level details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Enrollment {
    #[doc = "Enrollment Start Date"]
    #[serde(rename = "startDate", with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "Enrollment End Date"]
    #[serde(rename = "endDate", with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<time::OffsetDateTime>,
    #[doc = "The currency associated with enrollment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The channel for Enrollment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[doc = "The attributes associated with legacy enrollment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<EnrollmentPolicies>,
    #[doc = "The language for Enrollment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[doc = "The countryCode for Enrollment"]
    #[serde(rename = "countryCode", default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[doc = "Enrollment status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Enrollment billing cycle"]
    #[serde(rename = "billingCycle", default, skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
}
impl Enrollment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An account resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrollmentAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnrollmentAccountProperties>,
}
impl EnrollmentAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrollmentAccountProperties {
    #[doc = "The account name."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The cost center name."]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "The account owner"]
    #[serde(rename = "accountOwner", default, skip_serializing_if = "Option::is_none")]
    pub account_owner: Option<String>,
    #[doc = "The status for account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Account Start Date"]
    #[serde(rename = "startDate", with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "Account End Date"]
    #[serde(rename = "endDate", with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<time::OffsetDateTime>,
    #[doc = "A department resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<Department>,
}
impl EnrollmentAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The attributes associated with legacy enrollment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrollmentPolicies {
    #[doc = "The accountOwnerViewCharges flag for Enrollment"]
    #[serde(rename = "accountOwnerViewCharges", default, skip_serializing_if = "Option::is_none")]
    pub account_owner_view_charges: Option<bool>,
    #[doc = "The departmentAdminViewCharges flag for Enrollment"]
    #[serde(rename = "departmentAdminViewCharges", default, skip_serializing_if = "Option::is_none")]
    pub department_admin_view_charges: Option<bool>,
    #[doc = "The marketplaces flag for Enrollment"]
    #[serde(rename = "marketplacesEnabled", default, skip_serializing_if = "Option::is_none")]
    pub marketplaces_enabled: Option<bool>,
    #[doc = "The reserved instances flag for Enrollment"]
    #[serde(rename = "reservedInstancesEnabled", default, skip_serializing_if = "Option::is_none")]
    pub reserved_instances_enabled: Option<bool>,
}
impl EnrollmentPolicies {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The event properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventProperties {
    #[doc = "Transaction Date."]
    #[serde(rename = "transactionDate", with = "azure_core::date::rfc3339::option")]
    pub transaction_date: Option<time::OffsetDateTime>,
    #[doc = "Transaction description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Object to represent monetary quantities."]
    #[serde(rename = "newCredit", default, skip_serializing_if = "Option::is_none")]
    pub new_credit: Option<Amount>,
    #[doc = "Object to represent monetary quantities."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjustments: Option<Amount>,
    #[doc = "Object to represent monetary quantities."]
    #[serde(rename = "creditExpired", default, skip_serializing_if = "Option::is_none")]
    pub credit_expired: Option<Amount>,
    #[doc = "Object to represent monetary quantities."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charges: Option<Amount>,
    #[doc = "Object to represent monetary quantities."]
    #[serde(rename = "closedBalance", default, skip_serializing_if = "Option::is_none")]
    pub closed_balance: Option<Amount>,
    #[doc = "The type of event."]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<event_properties::EventType>,
    #[doc = "Invoice Number."]
    #[serde(rename = "invoiceNumber", default, skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
}
impl EventProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod event_properties {
    use super::*;
    #[doc = "The type of event."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EventType")]
    pub enum EventType {
        NewCredit,
        ExpiredCredit,
        SettledCharges,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EventType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EventType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EventType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NewCredit => serializer.serialize_unit_variant("EventType", 0u32, "NewCredit"),
                Self::ExpiredCredit => serializer.serialize_unit_variant("EventType", 1u32, "ExpiredCredit"),
                Self::SettledCharges => serializer.serialize_unit_variant("EventType", 2u32, "SettledCharges"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An event summary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventSummary {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The event properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventProperties>,
}
impl EventSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing event summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Events {
    #[doc = "The list of event summary."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EventSummary>,
}
impl Events {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An InvoiceSection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceSection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of an InvoiceSection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InvoiceSectionProperties>,
}
impl InvoiceSection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an InvoiceSection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceSectionProperties {
    #[doc = "The name of the InvoiceSection."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The billing profiles associated to the billing account."]
    #[serde(rename = "billingProfiles", default, skip_serializing_if = "Vec::is_empty")]
    pub billing_profiles: Vec<BillingProfile>,
}
impl InvoiceSectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The lot properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LotProperties {
    #[doc = "Object to represent monetary quantities."]
    #[serde(rename = "originalAmount", default, skip_serializing_if = "Option::is_none")]
    pub original_amount: Option<Amount>,
    #[doc = "Object to represent monetary quantities."]
    #[serde(rename = "closedBalance", default, skip_serializing_if = "Option::is_none")]
    pub closed_balance: Option<Amount>,
    #[doc = "Lot source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<lot_properties::Source>,
    #[doc = "Start Date."]
    #[serde(rename = "startDate", with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "Expiration Date."]
    #[serde(rename = "expirationDate", with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "PO Number."]
    #[serde(rename = "poNumber", default, skip_serializing_if = "Option::is_none")]
    pub po_number: Option<String>,
}
impl LotProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod lot_properties {
    use super::*;
    #[doc = "Lot source."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Source")]
    pub enum Source {
        PurchasedCredit,
        PromotionalCredit,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Source {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Source {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Source {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PurchasedCredit => serializer.serialize_unit_variant("Source", 0u32, "PurchasedCredit"),
                Self::PromotionalCredit => serializer.serialize_unit_variant("Source", 1u32, "PromotionalCredit"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A lot summary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LotSummary {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The lot properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LotProperties>,
}
impl LotSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing lot summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Lots {
    #[doc = "The list of lot summary."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LotSummary>,
}
impl Lots {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Consumption REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
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
        #[doc = "Service provider: Microsoft.Consumption."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: UsageDetail, etc."]
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
#[doc = "Result of listing consumption operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of consumption operations supported by the Microsoft.Consumption resource provider."]
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
#[doc = "Download response of Pricesheets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PricesheetDownloadResponse {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "A secure URL that can be used to download a an entity until the URL expires."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DownloadUrl>,
}
impl PricesheetDownloadResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "eTag of the resource. To handle concurrent update scenario, this field will be used to determine whether the user is updating the latest version or not."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource model definition."]
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
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
