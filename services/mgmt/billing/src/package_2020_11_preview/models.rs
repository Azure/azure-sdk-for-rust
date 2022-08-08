#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
pub type Action = String;
#[doc = "Address details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressDetails {
    #[doc = "First name."]
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Middle name."]
    #[serde(rename = "middleName", default, skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[doc = "Last name."]
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Company name."]
    #[serde(rename = "companyName", default, skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[doc = "Address line 1."]
    #[serde(rename = "addressLine1")]
    pub address_line1: String,
    #[doc = "Address line 2."]
    #[serde(rename = "addressLine2", default, skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[doc = "Address line 3."]
    #[serde(rename = "addressLine3", default, skip_serializing_if = "Option::is_none")]
    pub address_line3: Option<String>,
    #[doc = "Address city."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Address district."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[doc = "Address region."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "Country code uses ISO2, 2-digit format."]
    pub country: String,
    #[doc = "Postal code."]
    #[serde(rename = "postalCode", default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "Email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Phone number."]
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl AddressDetails {
    pub fn new(address_line1: String, country: String) -> Self {
        Self {
            first_name: None,
            middle_name: None,
            last_name: None,
            company_name: None,
            address_line1,
            address_line2: None,
            address_line3: None,
            city: None,
            district: None,
            region: None,
            country,
            postal_code: None,
            email: None,
            phone_number: None,
        }
    }
}
#[doc = "Status of the address validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AddressValidationStatus")]
pub enum AddressValidationStatus {
    Valid,
    Invalid,
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
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An agreement."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Agreement {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of an agreement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AgreementProperties>,
}
impl Agreement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing agreements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgreementListResult {
    #[doc = "The list of agreements."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Agreement>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AgreementListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AgreementListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an agreement."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgreementProperties {
    #[doc = "The URL to download the agreement."]
    #[serde(rename = "agreementLink", default, skip_serializing_if = "Option::is_none")]
    pub agreement_link: Option<String>,
    #[doc = "The category of the agreement signed by a customer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<agreement_properties::Category>,
    #[doc = "The mode of acceptance for an agreement."]
    #[serde(rename = "acceptanceMode", default, skip_serializing_if = "Option::is_none")]
    pub acceptance_mode: Option<agreement_properties::AcceptanceMode>,
    #[doc = "Details about billing profile associated with agreement and available only for specific agreements."]
    #[serde(rename = "billingProfileInfo", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_info: Option<BillingProfileInfo>,
    #[doc = "The date from which the agreement is effective."]
    #[serde(rename = "effectiveDate", with = "azure_core::date::rfc3339::option")]
    pub effective_date: Option<time::OffsetDateTime>,
    #[doc = "The date when the agreement expires."]
    #[serde(rename = "expirationDate", with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "The list of participants that participates in acceptance of an agreement."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participants: Vec<Participants>,
    #[doc = "The current status of the agreement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl AgreementProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod agreement_properties {
    use super::*;
    #[doc = "The category of the agreement signed by a customer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Category")]
    pub enum Category {
        MicrosoftCustomerAgreement,
        AffiliatePurchaseTerms,
        Other,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Category {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Category {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Category {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MicrosoftCustomerAgreement => serializer.serialize_unit_variant("Category", 0u32, "MicrosoftCustomerAgreement"),
                Self::AffiliatePurchaseTerms => serializer.serialize_unit_variant("Category", 1u32, "AffiliatePurchaseTerms"),
                Self::Other => serializer.serialize_unit_variant("Category", 2u32, "Other"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The mode of acceptance for an agreement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AcceptanceMode")]
    pub enum AcceptanceMode {
        ClickToAccept,
        ESignEmbedded,
        ESignOffline,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AcceptanceMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AcceptanceMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AcceptanceMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ClickToAccept => serializer.serialize_unit_variant("AcceptanceMode", 0u32, "ClickToAccept"),
                Self::ESignEmbedded => serializer.serialize_unit_variant("AcceptanceMode", 1u32, "ESignEmbedded"),
                Self::ESignOffline => serializer.serialize_unit_variant("AcceptanceMode", 2u32, "ESignOffline"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
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
pub type AppliedScopes = Vec<String>;
#[doc = "The latest Azure credit balance. This is the balance available for pay now."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableBalance {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of available balance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailableBalanceProperties>,
}
impl AvailableBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of available balance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableBalanceProperties {
    #[doc = "The amount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
}
impl AvailableBalanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the Azure plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzurePlan {
    #[doc = "The sku id."]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "The sku description."]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
}
impl AzurePlan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A billing account."]
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
#[doc = "The list of billing accounts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingAccountListResult {
    #[doc = "The list of billing accounts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BillingAccount>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BillingAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BillingAccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the billing account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingAccountProperties {
    #[doc = "The billing account name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Address details."]
    #[serde(rename = "soldTo", default, skip_serializing_if = "Option::is_none")]
    pub sold_to: Option<AddressDetails>,
    #[doc = "The type of agreement."]
    #[serde(rename = "agreementType", default, skip_serializing_if = "Option::is_none")]
    pub agreement_type: Option<billing_account_properties::AgreementType>,
    #[doc = "The type of customer."]
    #[serde(rename = "accountType", default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<billing_account_properties::AccountType>,
    #[doc = "The current status of the billing account."]
    #[serde(rename = "accountStatus", default, skip_serializing_if = "Option::is_none")]
    pub account_status: Option<billing_account_properties::AccountStatus>,
    #[doc = "The billing profiles associated with the billing account. By default this is not populated, unless it's specified in $expand."]
    #[serde(rename = "billingProfiles", default, skip_serializing_if = "Option::is_none")]
    pub billing_profiles: Option<BillingProfilesOnExpand>,
    #[doc = "The properties of an enrollment."]
    #[serde(rename = "enrollmentDetails", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_details: Option<Enrollment>,
    #[doc = "The departments associated to the enrollment."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub departments: Vec<Department>,
    #[doc = "The accounts associated to the enrollment."]
    #[serde(rename = "enrollmentAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub enrollment_accounts: Vec<EnrollmentAccount>,
    #[doc = "Indicates whether user has read access to the billing account."]
    #[serde(rename = "hasReadAccess", default, skip_serializing_if = "Option::is_none")]
    pub has_read_access: Option<bool>,
    #[doc = "Notification email address, only for legacy accounts"]
    #[serde(rename = "notificationEmailAddress", default, skip_serializing_if = "Option::is_none")]
    pub notification_email_address: Option<String>,
}
impl BillingAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_account_properties {
    use super::*;
    #[doc = "The type of agreement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AgreementType")]
    pub enum AgreementType {
        MicrosoftCustomerAgreement,
        EnterpriseAgreement,
        MicrosoftOnlineServicesProgram,
        MicrosoftPartnerAgreement,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AgreementType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AgreementType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AgreementType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MicrosoftCustomerAgreement => serializer.serialize_unit_variant("AgreementType", 0u32, "MicrosoftCustomerAgreement"),
                Self::EnterpriseAgreement => serializer.serialize_unit_variant("AgreementType", 1u32, "EnterpriseAgreement"),
                Self::MicrosoftOnlineServicesProgram => {
                    serializer.serialize_unit_variant("AgreementType", 2u32, "MicrosoftOnlineServicesProgram")
                }
                Self::MicrosoftPartnerAgreement => serializer.serialize_unit_variant("AgreementType", 3u32, "MicrosoftPartnerAgreement"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of customer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccountType")]
    pub enum AccountType {
        Enterprise,
        Individual,
        Partner,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccountType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccountType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccountType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enterprise => serializer.serialize_unit_variant("AccountType", 0u32, "Enterprise"),
                Self::Individual => serializer.serialize_unit_variant("AccountType", 1u32, "Individual"),
                Self::Partner => serializer.serialize_unit_variant("AccountType", 2u32, "Partner"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current status of the billing account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccountStatus")]
    pub enum AccountStatus {
        Active,
        Deleted,
        Disabled,
        Expired,
        Transferred,
        Extended,
        Terminated,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccountStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccountStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccountStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("AccountStatus", 0u32, "Active"),
                Self::Deleted => serializer.serialize_unit_variant("AccountStatus", 1u32, "Deleted"),
                Self::Disabled => serializer.serialize_unit_variant("AccountStatus", 2u32, "Disabled"),
                Self::Expired => serializer.serialize_unit_variant("AccountStatus", 3u32, "Expired"),
                Self::Transferred => serializer.serialize_unit_variant("AccountStatus", 4u32, "Transferred"),
                Self::Extended => serializer.serialize_unit_variant("AccountStatus", 5u32, "Extended"),
                Self::Terminated => serializer.serialize_unit_variant("AccountStatus", 6u32, "Terminated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The request properties of the billing account that can be updated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingAccountUpdateRequest {
    #[doc = "The properties of the billing account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingAccountProperties>,
}
impl BillingAccountUpdateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of list billingPermissions a caller has on a billing account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingPermissionsListResult {
    #[doc = "The list of billingPermissions a caller has on a billing account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BillingPermissionsProperties>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BillingPermissionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BillingPermissionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The set of allowed action and not allowed actions a caller has on a billing account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingPermissionsProperties {
    #[doc = "The set of actions that the caller is allowed to perform."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
    #[doc = "The set of actions that the caller is not allowed to perform."]
    #[serde(rename = "notActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_actions: Vec<NotAction>,
}
impl BillingPermissionsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A billing profile."]
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
#[doc = "The request parameters for creating a new billing profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProfileCreationRequest {
    #[doc = "The name of the billing profile."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The purchase order name that will appear on the invoices generated for the billing profile."]
    #[serde(rename = "poNumber", default, skip_serializing_if = "Option::is_none")]
    pub po_number: Option<String>,
    #[doc = "Address details."]
    #[serde(rename = "billTo", default, skip_serializing_if = "Option::is_none")]
    pub bill_to: Option<AddressDetails>,
    #[doc = "Flag controlling whether the invoices for the billing profile are sent through email."]
    #[serde(rename = "invoiceEmailOptIn", default, skip_serializing_if = "Option::is_none")]
    pub invoice_email_opt_in: Option<bool>,
    #[doc = "Enabled azure plans for the billing profile."]
    #[serde(rename = "enabledAzurePlans", default, skip_serializing_if = "Vec::is_empty")]
    pub enabled_azure_plans: Vec<AzurePlan>,
}
impl BillingProfileCreationRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about billing profile associated with agreement and available only for specific agreements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProfileInfo {
    #[doc = "The unique identifier for the billing profile."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The name of the billing profile"]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "Billing account name. This property is available for a specific type of agreement."]
    #[serde(rename = "indirectRelationshipOrganizationName", default, skip_serializing_if = "Option::is_none")]
    pub indirect_relationship_organization_name: Option<String>,
}
impl BillingProfileInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of billing profiles."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProfileListResult {
    #[doc = "The list of billing profiles."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BillingProfile>,
    #[doc = "Total number of records."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<f64>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BillingProfileListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BillingProfileListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the billing profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProfileProperties {
    #[doc = "The name of the billing profile."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The purchase order name that will appear on the invoices generated for the billing profile."]
    #[serde(rename = "poNumber", default, skip_serializing_if = "Option::is_none")]
    pub po_number: Option<String>,
    #[doc = "Identifies which services and purchases are paid by a billing profile."]
    #[serde(rename = "billingRelationshipType", default, skip_serializing_if = "Option::is_none")]
    pub billing_relationship_type: Option<billing_profile_properties::BillingRelationshipType>,
    #[doc = "Address details."]
    #[serde(rename = "billTo", default, skip_serializing_if = "Option::is_none")]
    pub bill_to: Option<AddressDetails>,
    #[doc = "The billing profile details of the partner of the customer for an indirect motion."]
    #[serde(rename = "indirectRelationshipInfo", default, skip_serializing_if = "Option::is_none")]
    pub indirect_relationship_info: Option<IndirectRelationshipInfo>,
    #[doc = "Flag controlling whether the invoices for the billing profile are sent through email."]
    #[serde(rename = "invoiceEmailOptIn", default, skip_serializing_if = "Option::is_none")]
    pub invoice_email_opt_in: Option<bool>,
    #[doc = "The day of the month when the invoice for the billing profile is generated."]
    #[serde(rename = "invoiceDay", default, skip_serializing_if = "Option::is_none")]
    pub invoice_day: Option<i32>,
    #[doc = "The currency in which the charges for the billing profile are billed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "Information about the enabled azure plans."]
    #[serde(rename = "enabledAzurePlans", default, skip_serializing_if = "Vec::is_empty")]
    pub enabled_azure_plans: Vec<AzurePlan>,
    #[doc = "The invoice sections associated to the billing profile. By default this is not populated, unless it's specified in $expand."]
    #[serde(rename = "invoiceSections", default, skip_serializing_if = "Option::is_none")]
    pub invoice_sections: Option<InvoiceSectionsOnExpand>,
    #[doc = "Indicates whether user has read access to the billing profile."]
    #[serde(rename = "hasReadAccess", default, skip_serializing_if = "Option::is_none")]
    pub has_read_access: Option<bool>,
    #[doc = "The system generated unique identifier for a billing profile."]
    #[serde(rename = "systemId", default, skip_serializing_if = "Option::is_none")]
    pub system_id: Option<String>,
    #[doc = "The status of the billing profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<billing_profile_properties::Status>,
    #[doc = "Reason for the specified billing profile status."]
    #[serde(rename = "statusReasonCode", default, skip_serializing_if = "Option::is_none")]
    pub status_reason_code: Option<billing_profile_properties::StatusReasonCode>,
    #[doc = "The billing profile spending limit."]
    #[serde(rename = "spendingLimit", default, skip_serializing_if = "Option::is_none")]
    pub spending_limit: Option<billing_profile_properties::SpendingLimit>,
    #[doc = "Identifies the cloud environments that are associated with a billing profile. This is a system managed optional field and gets updated as the billing profile gets associated with accounts in various clouds."]
    #[serde(rename = "targetClouds", default, skip_serializing_if = "Vec::is_empty")]
    pub target_clouds: Vec<TargetCloud>,
    #[doc = "Tags of billing profiles."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl BillingProfileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_profile_properties {
    use super::*;
    #[doc = "Identifies which services and purchases are paid by a billing profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingRelationshipType")]
    pub enum BillingRelationshipType {
        Direct,
        IndirectCustomer,
        IndirectPartner,
        #[serde(rename = "CSPPartner")]
        CspPartner,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingRelationshipType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingRelationshipType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingRelationshipType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Direct => serializer.serialize_unit_variant("BillingRelationshipType", 0u32, "Direct"),
                Self::IndirectCustomer => serializer.serialize_unit_variant("BillingRelationshipType", 1u32, "IndirectCustomer"),
                Self::IndirectPartner => serializer.serialize_unit_variant("BillingRelationshipType", 2u32, "IndirectPartner"),
                Self::CspPartner => serializer.serialize_unit_variant("BillingRelationshipType", 3u32, "CSPPartner"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the billing profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Active,
        Disabled,
        Warned,
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
                Self::Active => serializer.serialize_unit_variant("Status", 0u32, "Active"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 1u32, "Disabled"),
                Self::Warned => serializer.serialize_unit_variant("Status", 2u32, "Warned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Reason for the specified billing profile status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StatusReasonCode")]
    pub enum StatusReasonCode {
        PastDue,
        SpendingLimitReached,
        SpendingLimitExpired,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StatusReasonCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StatusReasonCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StatusReasonCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PastDue => serializer.serialize_unit_variant("StatusReasonCode", 0u32, "PastDue"),
                Self::SpendingLimitReached => serializer.serialize_unit_variant("StatusReasonCode", 1u32, "SpendingLimitReached"),
                Self::SpendingLimitExpired => serializer.serialize_unit_variant("StatusReasonCode", 2u32, "SpendingLimitExpired"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The billing profile spending limit."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SpendingLimit")]
    pub enum SpendingLimit {
        Off,
        On,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SpendingLimit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SpendingLimit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SpendingLimit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Off => serializer.serialize_unit_variant("SpendingLimit", 0u32, "Off"),
                Self::On => serializer.serialize_unit_variant("SpendingLimit", 1u32, "On"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The billing profiles associated with the billing account. By default this is not populated, unless it's specified in $expand."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProfilesOnExpand {
    #[doc = "Indicates whether there are more billing profiles than the ones listed in this collection. The collection lists a maximum of 50 billing profiles. To get all billing profiles, use the list billing profiles API."]
    #[serde(rename = "hasMoreResults", default, skip_serializing_if = "Option::is_none")]
    pub has_more_results: Option<bool>,
    #[doc = "The billing profiles associated with the billing account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BillingProfile>,
}
impl BillingProfilesOnExpand {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A billing property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProperty {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The billing property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingPropertyProperties>,
}
impl BillingProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The billing property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingPropertyProperties {
    #[doc = "The email address on which the account admin gets all Azure notifications."]
    #[serde(rename = "accountAdminNotificationEmailAddress", default, skip_serializing_if = "Option::is_none")]
    pub account_admin_notification_email_address: Option<String>,
    #[doc = "The Azure AD tenant ID of the billing account for the subscription."]
    #[serde(rename = "billingTenantId", default, skip_serializing_if = "Option::is_none")]
    pub billing_tenant_id: Option<String>,
    #[doc = "The ID of the billing account to which the subscription is billed."]
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<String>,
    #[doc = "The name of the billing account to which the subscription is billed."]
    #[serde(rename = "billingAccountDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_display_name: Option<String>,
    #[doc = "The ID of the billing profile to which the subscription is billed."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The name of the billing profile to which the subscription is billed."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The status of the billing profile."]
    #[serde(rename = "billingProfileStatus", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_status: Option<billing_property_properties::BillingProfileStatus>,
    #[doc = "Reason for the specified billing profile status."]
    #[serde(rename = "billingProfileStatusReasonCode", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_status_reason_code: Option<billing_property_properties::BillingProfileStatusReasonCode>,
    #[doc = "The billing profile spending limit."]
    #[serde(rename = "billingProfileSpendingLimit", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_spending_limit: Option<billing_property_properties::BillingProfileSpendingLimit>,
    #[doc = "The cost center applied to the subscription."]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "The ID of the invoice section to which the subscription is billed."]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "The name of the invoice section to which the subscription is billed."]
    #[serde(rename = "invoiceSectionDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_display_name: Option<String>,
    #[doc = "Indicates whether user is the account admin."]
    #[serde(rename = "isAccountAdmin", default, skip_serializing_if = "Option::is_none")]
    pub is_account_admin: Option<bool>,
    #[doc = "The product ID of the Azure plan."]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[doc = "The product name of the Azure plan."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "The sku ID of the Azure plan for the subscription."]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "The sku description of the Azure plan for the subscription."]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
}
impl BillingPropertyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_property_properties {
    use super::*;
    #[doc = "The status of the billing profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingProfileStatus")]
    pub enum BillingProfileStatus {
        Active,
        Disabled,
        Warned,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingProfileStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingProfileStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingProfileStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("BillingProfileStatus", 0u32, "Active"),
                Self::Disabled => serializer.serialize_unit_variant("BillingProfileStatus", 1u32, "Disabled"),
                Self::Warned => serializer.serialize_unit_variant("BillingProfileStatus", 2u32, "Warned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Reason for the specified billing profile status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingProfileStatusReasonCode")]
    pub enum BillingProfileStatusReasonCode {
        PastDue,
        SpendingLimitReached,
        SpendingLimitExpired,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingProfileStatusReasonCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingProfileStatusReasonCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingProfileStatusReasonCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PastDue => serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 0u32, "PastDue"),
                Self::SpendingLimitReached => {
                    serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 1u32, "SpendingLimitReached")
                }
                Self::SpendingLimitExpired => {
                    serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 2u32, "SpendingLimitExpired")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The billing profile spending limit."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingProfileSpendingLimit")]
    pub enum BillingProfileSpendingLimit {
        Off,
        On,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingProfileSpendingLimit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingProfileSpendingLimit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingProfileSpendingLimit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Off => serializer.serialize_unit_variant("BillingProfileSpendingLimit", 0u32, "Off"),
                Self::On => serializer.serialize_unit_variant("BillingProfileSpendingLimit", 1u32, "On"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The role assignment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingRoleAssignment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the role assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingRoleAssignmentProperties>,
}
impl BillingRoleAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of role assignments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingRoleAssignmentListResult {
    #[doc = "The list of role assignments."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BillingRoleAssignment>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BillingRoleAssignmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BillingRoleAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the role assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingRoleAssignmentProperties {
    #[doc = "The date the role assignment was created."]
    #[serde(rename = "createdOn", default, skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[doc = "The tenant Id of the user who created the role assignment."]
    #[serde(rename = "createdByPrincipalTenantId", default, skip_serializing_if = "Option::is_none")]
    pub created_by_principal_tenant_id: Option<String>,
    #[doc = "The principal Id of the user who created the role assignment."]
    #[serde(rename = "createdByPrincipalId", default, skip_serializing_if = "Option::is_none")]
    pub created_by_principal_id: Option<String>,
    #[doc = "The email address of the user who created the role assignment."]
    #[serde(rename = "createdByUserEmailAddress", default, skip_serializing_if = "Option::is_none")]
    pub created_by_user_email_address: Option<String>,
    #[doc = "The principal id of the user to whom the role was assigned."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The principal tenant id of the user to whom the role was assigned."]
    #[serde(rename = "principalTenantId", default, skip_serializing_if = "Option::is_none")]
    pub principal_tenant_id: Option<String>,
    #[doc = "The ID of the role definition."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "The scope at which the role was assigned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The authentication type."]
    #[serde(rename = "userAuthenticationType", default, skip_serializing_if = "Option::is_none")]
    pub user_authentication_type: Option<String>,
    #[doc = "The email address of the user."]
    #[serde(rename = "userEmailAddress", default, skip_serializing_if = "Option::is_none")]
    pub user_email_address: Option<String>,
}
impl BillingRoleAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a role definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingRoleDefinition {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the a role definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingRoleDefinitionProperties>,
}
impl BillingRoleDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of role definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingRoleDefinitionListResult {
    #[doc = "The role definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BillingRoleDefinition>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BillingRoleDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BillingRoleDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the a role definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingRoleDefinitionProperties {
    #[doc = "The role description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The billingPermissions the role has"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<BillingPermissionsProperties>,
    #[doc = "The name of the role"]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}
impl BillingRoleDefinitionProperties {
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
#[doc = "The billing properties of a subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscriptionProperties {
    #[doc = "The name of the subscription."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The ID of the subscription."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The current billing status of the subscription."]
    #[serde(rename = "subscriptionBillingStatus", default, skip_serializing_if = "Option::is_none")]
    pub subscription_billing_status: Option<billing_subscription_properties::SubscriptionBillingStatus>,
    #[doc = "The amount."]
    #[serde(rename = "lastMonthCharges", default, skip_serializing_if = "Option::is_none")]
    pub last_month_charges: Option<Amount>,
    #[doc = "The amount."]
    #[serde(rename = "monthToDateCharges", default, skip_serializing_if = "Option::is_none")]
    pub month_to_date_charges: Option<Amount>,
    #[doc = "The ID of the billing profile to which the subscription is billed."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The name of the billing profile to which the subscription is billed."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The cost center applied to the subscription."]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "The ID of the customer for whom the subscription was created. The field is applicable only for Microsoft Partner Agreement billing account."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "The name of the customer for whom the subscription was created. The field is applicable only for Microsoft Partner Agreement billing account."]
    #[serde(rename = "customerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub customer_display_name: Option<String>,
    #[doc = "The ID of the invoice section to which the subscription is billed."]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "The name of the invoice section to which the subscription is billed."]
    #[serde(rename = "invoiceSectionDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_display_name: Option<String>,
    #[doc = "Details of the reseller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reseller: Option<Reseller>,
    #[doc = "The sku ID of the Azure plan for the subscription."]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "The sku description of the Azure plan for the subscription."]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[doc = "The suspension reason for a subscription. Applies only to subscriptions in Microsoft Online Services Program billing accounts."]
    #[serde(rename = "suspensionReasons", default, skip_serializing_if = "Vec::is_empty")]
    pub suspension_reasons: Vec<String>,
}
impl BillingSubscriptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_subscription_properties {
    use super::*;
    #[doc = "The current billing status of the subscription."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SubscriptionBillingStatus")]
    pub enum SubscriptionBillingStatus {
        Active,
        Inactive,
        Abandoned,
        Deleted,
        Warning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SubscriptionBillingStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SubscriptionBillingStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SubscriptionBillingStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("SubscriptionBillingStatus", 0u32, "Active"),
                Self::Inactive => serializer.serialize_unit_variant("SubscriptionBillingStatus", 1u32, "Inactive"),
                Self::Abandoned => serializer.serialize_unit_variant("SubscriptionBillingStatus", 2u32, "Abandoned"),
                Self::Deleted => serializer.serialize_unit_variant("SubscriptionBillingStatus", 3u32, "Deleted"),
                Self::Warning => serializer.serialize_unit_variant("SubscriptionBillingStatus", 4u32, "Warning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The list of billing subscriptions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscriptionsListResult {
    #[doc = "The list of billing subscriptions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BillingSubscription>,
    #[doc = "Total number of records."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<f64>,
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
#[doc = "A partner's customer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Customer {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a customer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomerProperties>,
}
impl Customer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of customers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerListResult {
    #[doc = "The list of customers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Customer>,
    #[doc = "Total number of records."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<f64>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CustomerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CustomerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The customer's Policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerPolicy {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a customer's policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomerPolicyProperties>,
}
impl CustomerPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a customer's policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerPolicyProperties {
    #[doc = "The policy that controls whether the users in customer's organization can view charges at pay-as-you-go prices."]
    #[serde(rename = "viewCharges", default, skip_serializing_if = "Option::is_none")]
    pub view_charges: Option<customer_policy_properties::ViewCharges>,
}
impl CustomerPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod customer_policy_properties {
    use super::*;
    #[doc = "The policy that controls whether the users in customer's organization can view charges at pay-as-you-go prices."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ViewCharges")]
    pub enum ViewCharges {
        Allowed,
        NotAllowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ViewCharges {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ViewCharges {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ViewCharges {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allowed => serializer.serialize_unit_variant("ViewCharges", 0u32, "Allowed"),
                Self::NotAllowed => serializer.serialize_unit_variant("ViewCharges", 1u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of a customer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerProperties {
    #[doc = "The ID of the billing profile for the invoice section."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The name of the billing profile for the invoice section."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The name of the customer."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Azure plans enabled for the customer."]
    #[serde(rename = "enabledAzurePlans", default, skip_serializing_if = "Vec::is_empty")]
    pub enabled_azure_plans: Vec<AzurePlan>,
    #[doc = "The list of resellers for which an Azure plan is enabled for the customer."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resellers: Vec<Reseller>,
}
impl CustomerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A department."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Department {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a department."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DepartmentProperties>,
}
impl Department {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a department."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DepartmentProperties {
    #[doc = "The name of the department."]
    #[serde(rename = "departmentName", default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[doc = "The cost center associated with the department."]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "The status of the department."]
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
#[doc = "The properties of a document."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Document {
    #[doc = "The type of the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<document::Kind>,
    #[doc = "Document URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The source of the document. ENF for Brazil and DRS for rest of the world."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<document::Source>,
}
impl Document {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod document {
    use super::*;
    #[doc = "The type of the document."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Invoice,
        VoidNote,
        TaxReceipt,
        CreditNote,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invoice => serializer.serialize_unit_variant("Kind", 0u32, "Invoice"),
                Self::VoidNote => serializer.serialize_unit_variant("Kind", 1u32, "VoidNote"),
                Self::TaxReceipt => serializer.serialize_unit_variant("Kind", 2u32, "TaxReceipt"),
                Self::CreditNote => serializer.serialize_unit_variant("Kind", 3u32, "CreditNote"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The source of the document. ENF for Brazil and DRS for rest of the world."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Source")]
    pub enum Source {
        #[serde(rename = "DRS")]
        Drs,
        #[serde(rename = "ENF")]
        Enf,
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
                Self::Drs => serializer.serialize_unit_variant("Source", 0u32, "DRS"),
                Self::Enf => serializer.serialize_unit_variant("Source", 1u32, "ENF"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A secure URL that can be used to download a an entity until the URL expires."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DownloadUrl {
    #[doc = "The time in UTC when the download URL will expire."]
    #[serde(rename = "expiryTime", with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[doc = "The URL to the PDF file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl DownloadUrl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an enrollment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Enrollment {
    #[doc = "The start date of the enrollment."]
    #[serde(rename = "startDate", with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "The end date of the enrollment."]
    #[serde(rename = "endDate", with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<time::OffsetDateTime>,
    #[doc = "The billing currency for the enrollment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The channel type of the enrollment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[doc = "The policies for Enterprise Agreement enrollments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<EnrollmentPolicies>,
    #[doc = "The language for the enrollment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[doc = "The country code of the enrollment."]
    #[serde(rename = "countryCode", default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[doc = "The current status of the enrollment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The billing cycle for the enrollment."]
    #[serde(rename = "billingCycle", default, skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
}
impl Enrollment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An enrollment account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrollmentAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of an enrollment account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnrollmentAccountProperties>,
}
impl EnrollmentAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The enrollment account context"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrollmentAccountContext {
    #[doc = "The cost center associated with the enrollment account."]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "The start date of the enrollment account."]
    #[serde(rename = "startDate", with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "The end date of the enrollment account."]
    #[serde(rename = "endDate", with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<time::OffsetDateTime>,
    #[doc = "The ID of the enrollment account."]
    #[serde(rename = "enrollmentAccountName", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_account_name: Option<String>,
}
impl EnrollmentAccountContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an enrollment account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrollmentAccountProperties {
    #[doc = "The name of the enrollment account."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The cost center associated with the enrollment account."]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "The owner of the enrollment account."]
    #[serde(rename = "accountOwner", default, skip_serializing_if = "Option::is_none")]
    pub account_owner: Option<String>,
    #[doc = "The enrollment account owner email address."]
    #[serde(rename = "accountOwnerEmail", default, skip_serializing_if = "Option::is_none")]
    pub account_owner_email: Option<String>,
    #[doc = "The status of the enrollment account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The start date of the enrollment account."]
    #[serde(rename = "startDate", with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "The end date of the enrollment account."]
    #[serde(rename = "endDate", with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<time::OffsetDateTime>,
    #[doc = "A department."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<Department>,
}
impl EnrollmentAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policies for Enterprise Agreement enrollments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrollmentPolicies {
    #[doc = "The policy that controls whether Account Owners can view charges."]
    #[serde(rename = "accountOwnerViewCharges", default, skip_serializing_if = "Option::is_none")]
    pub account_owner_view_charges: Option<bool>,
    #[doc = "The policy that controls whether Department Administrators can view charges."]
    #[serde(rename = "departmentAdminViewCharges", default, skip_serializing_if = "Option::is_none")]
    pub department_admin_view_charges: Option<bool>,
    #[doc = "The policy that controls whether Azure marketplace purchases are allowed in the enrollment."]
    #[serde(rename = "marketplaceEnabled", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_enabled: Option<bool>,
    #[doc = "The policy that controls whether Azure reservation purchases are allowed in the enrollment."]
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
    #[doc = "The target of the particular error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
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
#[doc = "The billing profile details of the partner of the customer for an indirect motion."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IndirectRelationshipInfo {
    #[doc = "The billing account name of the partner or the customer for an indirect motion."]
    #[serde(rename = "billingAccountName", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_name: Option<String>,
    #[doc = "The billing profile name of the partner or the customer for an indirect motion."]
    #[serde(rename = "billingProfileName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_name: Option<String>,
    #[doc = "The display name of the partner or customer for an indirect motion."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl IndirectRelationshipInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An instruction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Instruction {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "A billing instruction used during invoice generation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InstructionProperties>,
}
impl Instruction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of billing instructions used during invoice generation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InstructionListResult {
    #[doc = "The list of billing instructions used during invoice generation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Instruction>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InstructionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl InstructionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A billing instruction used during invoice generation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstructionProperties {
    #[doc = "The amount budgeted for this billing instruction."]
    pub amount: f64,
    #[doc = "The date this billing instruction goes into effect."]
    #[serde(rename = "startDate", with = "azure_core::date::rfc3339")]
    pub start_date: time::OffsetDateTime,
    #[doc = "The date this billing instruction is no longer in effect."]
    #[serde(rename = "endDate", with = "azure_core::date::rfc3339")]
    pub end_date: time::OffsetDateTime,
    #[doc = "The date this billing instruction was created."]
    #[serde(rename = "creationDate", with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
}
impl InstructionProperties {
    pub fn new(amount: f64, start_date: time::OffsetDateTime, end_date: time::OffsetDateTime) -> Self {
        Self {
            amount,
            start_date,
            end_date,
            creation_date: None,
        }
    }
}
#[doc = "An invoice."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Invoice {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the invoice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InvoiceProperties>,
}
impl Invoice {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of invoices."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceListResult {
    #[doc = "The list of invoices."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Invoice>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Total number of records."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<f64>,
}
impl azure_core::Continuable for InvoiceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl InvoiceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the invoice."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceProperties {
    #[doc = "The due date for the invoice."]
    #[serde(rename = "dueDate", with = "azure_core::date::rfc3339::option")]
    pub due_date: Option<time::OffsetDateTime>,
    #[doc = "The date when the invoice was generated."]
    #[serde(rename = "invoiceDate", with = "azure_core::date::rfc3339::option")]
    pub invoice_date: Option<time::OffsetDateTime>,
    #[doc = "The current status of the invoice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<invoice_properties::Status>,
    #[doc = "The amount."]
    #[serde(rename = "amountDue", default, skip_serializing_if = "Option::is_none")]
    pub amount_due: Option<Amount>,
    #[doc = "The amount."]
    #[serde(rename = "azurePrepaymentApplied", default, skip_serializing_if = "Option::is_none")]
    pub azure_prepayment_applied: Option<Amount>,
    #[doc = "The amount."]
    #[serde(rename = "billedAmount", default, skip_serializing_if = "Option::is_none")]
    pub billed_amount: Option<Amount>,
    #[doc = "The amount."]
    #[serde(rename = "creditAmount", default, skip_serializing_if = "Option::is_none")]
    pub credit_amount: Option<Amount>,
    #[doc = "The amount."]
    #[serde(rename = "freeAzureCreditApplied", default, skip_serializing_if = "Option::is_none")]
    pub free_azure_credit_applied: Option<Amount>,
    #[doc = "The amount."]
    #[serde(rename = "subTotal", default, skip_serializing_if = "Option::is_none")]
    pub sub_total: Option<Amount>,
    #[doc = "The amount."]
    #[serde(rename = "taxAmount", default, skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<Amount>,
    #[doc = "The amount."]
    #[serde(rename = "totalAmount", default, skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<Amount>,
    #[doc = "The start date of the billing period for which the invoice is generated."]
    #[serde(rename = "invoicePeriodStartDate", with = "azure_core::date::rfc3339::option")]
    pub invoice_period_start_date: Option<time::OffsetDateTime>,
    #[doc = "The end date of the billing period for which the invoice is generated."]
    #[serde(rename = "invoicePeriodEndDate", with = "azure_core::date::rfc3339::option")]
    pub invoice_period_end_date: Option<time::OffsetDateTime>,
    #[doc = "Invoice type."]
    #[serde(rename = "invoiceType", default, skip_serializing_if = "Option::is_none")]
    pub invoice_type: Option<invoice_properties::InvoiceType>,
    #[doc = "Specifies if the invoice is generated as part of monthly invoicing cycle or not. This field is applicable to billing accounts with agreement type Microsoft Customer Agreement."]
    #[serde(rename = "isMonthlyInvoice", default, skip_serializing_if = "Option::is_none")]
    pub is_monthly_invoice: Option<bool>,
    #[doc = "The ID of the billing profile for which the invoice is generated."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The name of the billing profile for which the invoice is generated."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "An optional purchase order number for the invoice."]
    #[serde(rename = "purchaseOrderNumber", default, skip_serializing_if = "Option::is_none")]
    pub purchase_order_number: Option<String>,
    #[doc = "List of documents available to download such as invoice and tax receipt."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub documents: Vec<Document>,
    #[doc = "List of payments."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payments: Vec<PaymentProperties>,
    #[doc = "Rebill details for an invoice."]
    #[serde(rename = "rebillDetails", default, skip_serializing_if = "Option::is_none")]
    pub rebill_details: Option<serde_json::Value>,
    #[doc = "The type of the document."]
    #[serde(rename = "documentType", default, skip_serializing_if = "Option::is_none")]
    pub document_type: Option<invoice_properties::DocumentType>,
    #[doc = "The Id of the active invoice which is originally billed after this invoice was voided. This field is applicable to the void invoices only."]
    #[serde(rename = "billedDocumentId", default, skip_serializing_if = "Option::is_none")]
    pub billed_document_id: Option<String>,
    #[doc = "The Id of the invoice which got voided and this credit note was issued as a result. This field is applicable to the credit notes only."]
    #[serde(rename = "creditForDocumentId", default, skip_serializing_if = "Option::is_none")]
    pub credit_for_document_id: Option<String>,
    #[doc = "The ID of the subscription for which the invoice is generated."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
impl InvoiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod invoice_properties {
    use super::*;
    #[doc = "The current status of the invoice."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Due,
        OverDue,
        Paid,
        Void,
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
                Self::Due => serializer.serialize_unit_variant("Status", 0u32, "Due"),
                Self::OverDue => serializer.serialize_unit_variant("Status", 1u32, "OverDue"),
                Self::Paid => serializer.serialize_unit_variant("Status", 2u32, "Paid"),
                Self::Void => serializer.serialize_unit_variant("Status", 3u32, "Void"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Invoice type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InvoiceType")]
    pub enum InvoiceType {
        AzureService,
        AzureMarketplace,
        AzureSupport,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InvoiceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InvoiceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InvoiceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureService => serializer.serialize_unit_variant("InvoiceType", 0u32, "AzureService"),
                Self::AzureMarketplace => serializer.serialize_unit_variant("InvoiceType", 1u32, "AzureMarketplace"),
                Self::AzureSupport => serializer.serialize_unit_variant("InvoiceType", 2u32, "AzureSupport"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of the document."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DocumentType")]
    pub enum DocumentType {
        Invoice,
        CreditNote,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DocumentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DocumentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DocumentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invoice => serializer.serialize_unit_variant("DocumentType", 0u32, "Invoice"),
                Self::CreditNote => serializer.serialize_unit_variant("DocumentType", 1u32, "CreditNote"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An invoice section."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceSection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of an invoice section."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InvoiceSectionProperties>,
}
impl InvoiceSection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the invoice section."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceSectionCreationRequest {
    #[doc = "The name of the invoice section."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl InvoiceSectionCreationRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of invoice sections."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceSectionListResult {
    #[doc = "The list of invoice sections."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<InvoiceSection>,
    #[doc = "Total number of records."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<f64>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InvoiceSectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl InvoiceSectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of invoice section properties with create subscription permission."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceSectionListWithCreateSubPermissionResult {
    #[doc = "The list of invoice section properties with create subscription permission."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<InvoiceSectionWithCreateSubPermission>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InvoiceSectionListWithCreateSubPermissionResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl InvoiceSectionListWithCreateSubPermissionResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an invoice section."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceSectionProperties {
    #[doc = "The name of the invoice section."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Dictionary of metadata associated with the invoice section."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<serde_json::Value>,
    #[doc = "Identifies the state of an invoice section."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<invoice_section_properties::State>,
    #[doc = "The system generated unique identifier for an invoice section."]
    #[serde(rename = "systemId", default, skip_serializing_if = "Option::is_none")]
    pub system_id: Option<String>,
    #[doc = "Dictionary of metadata associated with the invoice section. Maximum key/value length supported of 256 characters. Keys/value should not empty value nor null. Keys can not contain < > % & \\ ? /"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Possible cloud environments."]
    #[serde(rename = "targetCloud", default, skip_serializing_if = "Option::is_none")]
    pub target_cloud: Option<TargetCloud>,
}
impl InvoiceSectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod invoice_section_properties {
    use super::*;
    #[doc = "Identifies the state of an invoice section."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Active,
        Restricted,
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
                Self::Active => serializer.serialize_unit_variant("State", 0u32, "Active"),
                Self::Restricted => serializer.serialize_unit_variant("State", 1u32, "Restricted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Invoice section properties with create subscription permission."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceSectionWithCreateSubPermission {
    #[doc = "The ID of the invoice section."]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "The name of the invoice section."]
    #[serde(rename = "invoiceSectionDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_display_name: Option<String>,
    #[doc = "The system generated unique identifier for an invoice section."]
    #[serde(rename = "invoiceSectionSystemId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_system_id: Option<String>,
    #[doc = "The ID of the billing profile for the invoice section."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The name of the billing profile for the invoice section."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The status of the billing profile."]
    #[serde(rename = "billingProfileStatus", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_status: Option<invoice_section_with_create_sub_permission::BillingProfileStatus>,
    #[doc = "Reason for the specified billing profile status."]
    #[serde(rename = "billingProfileStatusReasonCode", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_status_reason_code: Option<invoice_section_with_create_sub_permission::BillingProfileStatusReasonCode>,
    #[doc = "The billing profile spending limit."]
    #[serde(rename = "billingProfileSpendingLimit", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_spending_limit: Option<invoice_section_with_create_sub_permission::BillingProfileSpendingLimit>,
    #[doc = "The system generated unique identifier for a billing profile."]
    #[serde(rename = "billingProfileSystemId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_system_id: Option<String>,
    #[doc = "Enabled azure plans for the associated billing profile."]
    #[serde(rename = "enabledAzurePlans", default, skip_serializing_if = "Vec::is_empty")]
    pub enabled_azure_plans: Vec<AzurePlan>,
}
impl InvoiceSectionWithCreateSubPermission {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod invoice_section_with_create_sub_permission {
    use super::*;
    #[doc = "The status of the billing profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingProfileStatus")]
    pub enum BillingProfileStatus {
        Active,
        Disabled,
        Warned,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingProfileStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingProfileStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingProfileStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("BillingProfileStatus", 0u32, "Active"),
                Self::Disabled => serializer.serialize_unit_variant("BillingProfileStatus", 1u32, "Disabled"),
                Self::Warned => serializer.serialize_unit_variant("BillingProfileStatus", 2u32, "Warned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Reason for the specified billing profile status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingProfileStatusReasonCode")]
    pub enum BillingProfileStatusReasonCode {
        PastDue,
        SpendingLimitReached,
        SpendingLimitExpired,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingProfileStatusReasonCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingProfileStatusReasonCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingProfileStatusReasonCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PastDue => serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 0u32, "PastDue"),
                Self::SpendingLimitReached => {
                    serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 1u32, "SpendingLimitReached")
                }
                Self::SpendingLimitExpired => {
                    serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 2u32, "SpendingLimitExpired")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The billing profile spending limit."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingProfileSpendingLimit")]
    pub enum BillingProfileSpendingLimit {
        Off,
        On,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingProfileSpendingLimit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingProfileSpendingLimit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingProfileSpendingLimit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Off => serializer.serialize_unit_variant("BillingProfileSpendingLimit", 0u32, "Off"),
                Self::On => serializer.serialize_unit_variant("BillingProfileSpendingLimit", 1u32, "On"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The invoice sections associated to the billing profile. By default this is not populated, unless it's specified in $expand."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceSectionsOnExpand {
    #[doc = "Indicates whether there are more invoice sections than the ones listed in this collection. The collection lists a maximum of 50 invoice sections. To get all invoice sections, use the list invoice sections API."]
    #[serde(rename = "hasMoreResults", default, skip_serializing_if = "Option::is_none")]
    pub has_more_results: Option<bool>,
    #[doc = "The invoice sections associated to the billing profile."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<InvoiceSection>,
}
impl InvoiceSectionsOnExpand {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type NotAction = String;
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
#[doc = "The details of the error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsErrorDetails {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl OperationsErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsErrorResponse {
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationsErrorDetails>,
}
impl azure_core::Continuable for OperationsErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationsErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details about a participant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Participants {
    #[doc = "The acceptance status of the participant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The date when the status got changed."]
    #[serde(rename = "statusDate", with = "azure_core::date::rfc3339::option")]
    pub status_date: Option<time::OffsetDateTime>,
    #[doc = "The email address of the participant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
impl Participants {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a payment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentProperties {
    #[doc = "The type of payment."]
    #[serde(rename = "paymentType", default, skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    #[doc = "The amount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[doc = "The date when the payment was made."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub date: Option<time::OffsetDateTime>,
    #[doc = "The family of payment method."]
    #[serde(rename = "paymentMethodFamily", default, skip_serializing_if = "Option::is_none")]
    pub payment_method_family: Option<payment_properties::PaymentMethodFamily>,
    #[doc = "The type of payment method."]
    #[serde(rename = "paymentMethodType", default, skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
}
impl PaymentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod payment_properties {
    use super::*;
    #[doc = "The family of payment method."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PaymentMethodFamily")]
    pub enum PaymentMethodFamily {
        Credits,
        CheckWire,
        CreditCard,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PaymentMethodFamily {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PaymentMethodFamily {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PaymentMethodFamily {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Credits => serializer.serialize_unit_variant("PaymentMethodFamily", 0u32, "Credits"),
                Self::CheckWire => serializer.serialize_unit_variant("PaymentMethodFamily", 1u32, "CheckWire"),
                Self::CreditCard => serializer.serialize_unit_variant("PaymentMethodFamily", 2u32, "CreditCard"),
                Self::None => serializer.serialize_unit_variant("PaymentMethodFamily", 3u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Policy {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyProperties>,
}
impl Policy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyProperties {
    #[doc = "The policy that controls whether Azure marketplace purchases are allowed for a billing profile."]
    #[serde(rename = "marketplacePurchases", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_purchases: Option<policy_properties::MarketplacePurchases>,
    #[doc = "The policy that controls whether Azure reservation purchases are allowed for a billing profile."]
    #[serde(rename = "reservationPurchases", default, skip_serializing_if = "Option::is_none")]
    pub reservation_purchases: Option<policy_properties::ReservationPurchases>,
    #[doc = "The policy that controls whether users with Azure RBAC access to a subscription can view its charges."]
    #[serde(rename = "viewCharges", default, skip_serializing_if = "Option::is_none")]
    pub view_charges: Option<policy_properties::ViewCharges>,
}
impl PolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_properties {
    use super::*;
    #[doc = "The policy that controls whether Azure marketplace purchases are allowed for a billing profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MarketplacePurchases")]
    pub enum MarketplacePurchases {
        AllAllowed,
        OnlyFreeAllowed,
        NotAllowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MarketplacePurchases {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MarketplacePurchases {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MarketplacePurchases {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AllAllowed => serializer.serialize_unit_variant("MarketplacePurchases", 0u32, "AllAllowed"),
                Self::OnlyFreeAllowed => serializer.serialize_unit_variant("MarketplacePurchases", 1u32, "OnlyFreeAllowed"),
                Self::NotAllowed => serializer.serialize_unit_variant("MarketplacePurchases", 2u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The policy that controls whether Azure reservation purchases are allowed for a billing profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReservationPurchases")]
    pub enum ReservationPurchases {
        Allowed,
        NotAllowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReservationPurchases {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReservationPurchases {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReservationPurchases {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allowed => serializer.serialize_unit_variant("ReservationPurchases", 0u32, "Allowed"),
                Self::NotAllowed => serializer.serialize_unit_variant("ReservationPurchases", 1u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The policy that controls whether users with Azure RBAC access to a subscription can view its charges."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ViewCharges")]
    pub enum ViewCharges {
        Allowed,
        NotAllowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ViewCharges {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ViewCharges {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ViewCharges {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allowed => serializer.serialize_unit_variant("ViewCharges", 0u32, "Allowed"),
                Self::NotAllowed => serializer.serialize_unit_variant("ViewCharges", 1u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A product."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Product {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductProperties>,
}
impl Product {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a product."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductProperties {
    #[doc = "Indicates whether auto renewal is turned on or off for a product."]
    #[serde(rename = "autoRenew", default, skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<product_properties::AutoRenew>,
    #[doc = "The display name of the product."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The date when the product was purchased."]
    #[serde(rename = "purchaseDate", with = "azure_core::date::rfc3339::option")]
    pub purchase_date: Option<time::OffsetDateTime>,
    #[doc = "The ID of the type of product."]
    #[serde(rename = "productTypeId", default, skip_serializing_if = "Option::is_none")]
    pub product_type_id: Option<String>,
    #[doc = "The description of the type of product."]
    #[serde(rename = "productType", default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[doc = "The current status of the product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<product_properties::Status>,
    #[doc = "The date when the product will be renewed or canceled."]
    #[serde(rename = "endDate", with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<time::OffsetDateTime>,
    #[doc = "The frequency at which the product will be billed."]
    #[serde(rename = "billingFrequency", default, skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<product_properties::BillingFrequency>,
    #[doc = "The amount."]
    #[serde(rename = "lastCharge", default, skip_serializing_if = "Option::is_none")]
    pub last_charge: Option<Amount>,
    #[doc = "The date of the last charge."]
    #[serde(rename = "lastChargeDate", with = "azure_core::date::rfc3339::option")]
    pub last_charge_date: Option<time::OffsetDateTime>,
    #[doc = "The quantity purchased for the product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[doc = "The sku ID of the product."]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "The sku description of the product."]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[doc = "The id of the tenant in which the product is used."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The availability of the product."]
    #[serde(rename = "availabilityId", default, skip_serializing_if = "Option::is_none")]
    pub availability_id: Option<String>,
    #[doc = "The ID of the invoice section to which the product is billed."]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "The name of the invoice section to which the product is billed."]
    #[serde(rename = "invoiceSectionDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_display_name: Option<String>,
    #[doc = "The ID of the billing profile to which the product is billed."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The name of the billing profile to which the product is billed."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The ID of the customer for whom the product was purchased. The field is applicable only for Microsoft Partner Agreement billing account."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "The name of the customer for whom the product was purchased. The field is applicable only for Microsoft Partner Agreement billing account."]
    #[serde(rename = "customerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub customer_display_name: Option<String>,
    #[doc = "Details of the reseller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reseller: Option<Reseller>,
}
impl ProductProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod product_properties {
    use super::*;
    #[doc = "Indicates whether auto renewal is turned on or off for a product."]
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
    #[doc = "The current status of the product."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Active,
        Inactive,
        PastDue,
        Expiring,
        Expired,
        Disabled,
        Cancelled,
        AutoRenew,
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
                Self::Active => serializer.serialize_unit_variant("Status", 0u32, "Active"),
                Self::Inactive => serializer.serialize_unit_variant("Status", 1u32, "Inactive"),
                Self::PastDue => serializer.serialize_unit_variant("Status", 2u32, "PastDue"),
                Self::Expiring => serializer.serialize_unit_variant("Status", 3u32, "Expiring"),
                Self::Expired => serializer.serialize_unit_variant("Status", 4u32, "Expired"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 5u32, "Disabled"),
                Self::Cancelled => serializer.serialize_unit_variant("Status", 6u32, "Cancelled"),
                Self::AutoRenew => serializer.serialize_unit_variant("Status", 7u32, "AutoRenew"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The frequency at which the product will be billed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingFrequency")]
    pub enum BillingFrequency {
        OneTime,
        Monthly,
        UsageBased,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingFrequency {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingFrequency {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingFrequency {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::OneTime => serializer.serialize_unit_variant("BillingFrequency", 0u32, "OneTime"),
                Self::Monthly => serializer.serialize_unit_variant("BillingFrequency", 1u32, "Monthly"),
                Self::UsageBased => serializer.serialize_unit_variant("BillingFrequency", 2u32, "UsageBased"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Error code of the transfer validation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProductTransferValidationErrorCode")]
pub enum ProductTransferValidationErrorCode {
    InvalidSource,
    ProductNotActive,
    InsufficientPermissionOnSource,
    InsufficientPermissionOnDestination,
    DestinationBillingProfilePastDue,
    ProductTypeNotSupported,
    CrossBillingAccountNotAllowed,
    NotAvailableForDestinationMarket,
    OneTimePurchaseProductTransferNotAllowed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProductTransferValidationErrorCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProductTransferValidationErrorCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProductTransferValidationErrorCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::InvalidSource => serializer.serialize_unit_variant("ProductTransferValidationErrorCode", 0u32, "InvalidSource"),
            Self::ProductNotActive => serializer.serialize_unit_variant("ProductTransferValidationErrorCode", 1u32, "ProductNotActive"),
            Self::InsufficientPermissionOnSource => {
                serializer.serialize_unit_variant("ProductTransferValidationErrorCode", 2u32, "InsufficientPermissionOnSource")
            }
            Self::InsufficientPermissionOnDestination => {
                serializer.serialize_unit_variant("ProductTransferValidationErrorCode", 3u32, "InsufficientPermissionOnDestination")
            }
            Self::DestinationBillingProfilePastDue => {
                serializer.serialize_unit_variant("ProductTransferValidationErrorCode", 4u32, "DestinationBillingProfilePastDue")
            }
            Self::ProductTypeNotSupported => {
                serializer.serialize_unit_variant("ProductTransferValidationErrorCode", 5u32, "ProductTypeNotSupported")
            }
            Self::CrossBillingAccountNotAllowed => {
                serializer.serialize_unit_variant("ProductTransferValidationErrorCode", 6u32, "CrossBillingAccountNotAllowed")
            }
            Self::NotAvailableForDestinationMarket => {
                serializer.serialize_unit_variant("ProductTransferValidationErrorCode", 7u32, "NotAvailableForDestinationMarket")
            }
            Self::OneTimePurchaseProductTransferNotAllowed => serializer.serialize_unit_variant(
                "ProductTransferValidationErrorCode",
                8u32,
                "OneTimePurchaseProductTransferNotAllowed",
            ),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The list of products. It contains a list of available product summaries in reverse chronological order by purchase date."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductsListResult {
    #[doc = "The list of products."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Product>,
    #[doc = "Total number of records."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<f64>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProductsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProductsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PromotionCheckEligibilityResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PromotionCheckEligibilityResponseProperties>,
}
impl PromotionCheckEligibilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PromotionCheckEligibilityResponseProperties {
    #[doc = "Name of the promotion"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the promotion"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies whether the subscription is eligible for the promotion"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eligible: Option<bool>,
    #[doc = "Reason in case the subscription is not eligible for promotion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Subscription Id on which the eligibility check was performed."]
    #[serde(rename = "eligibilitySubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub eligibility_subscription_id: Option<String>,
    #[doc = "Date till when the promotion is available."]
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
}
impl PromotionCheckEligibilityResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PromotionCreateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<PromotionCreateSkuNameRequestProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PromotionCreateRequestProperties>,
}
impl PromotionCreateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PromotionCreateRequestProperties {
    #[doc = "Friendly name for user to easily identified the promotion."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "List of the subscriptions that the benefit will be applied."]
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[doc = "Unique Id for the promotion order."]
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}
impl PromotionCreateRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PromotionCreateSkuNameRequestProperties {
    #[doc = "SKU Name of the promotion to activate"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl PromotionCreateSkuNameRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of promotions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PromotionList {
    #[doc = "The list of promotions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PromotionResponse>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PromotionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PromotionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PromotionResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i64>,
    #[doc = "Identifier of the promotion"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the promotion"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PromotionResponseProperties>,
    #[doc = "Type of resource. \"Microsoft.Billing/Promotions\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PromotionResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PromotionResponseProperties {
    #[doc = "Friendly name for user to easily identified the promotion."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "This is the DateTime when the promotion would come in effect"]
    #[serde(rename = "effectiveDateTime", with = "azure_core::date::rfc3339::option")]
    pub effective_date_time: Option<time::OffsetDateTime>,
    #[doc = "Last update time of the promotion resource."]
    #[serde(rename = "lastUpdatedDateTime", with = "azure_core::date::rfc3339::option")]
    pub last_updated_date_time: Option<time::OffsetDateTime>,
    #[doc = "This is the date when the Reservation will expire."]
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[doc = "Current state of the promotion."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "List of the subscriptions that the benefit will be applied."]
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
}
impl PromotionResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The rebill details of an invoice."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RebillDetails {
    #[doc = "The ID of credit note."]
    #[serde(rename = "creditNoteDocumentId", default, skip_serializing_if = "Option::is_none")]
    pub credit_note_document_id: Option<String>,
    #[doc = "The ID of invoice."]
    #[serde(rename = "invoiceDocumentId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_document_id: Option<String>,
    #[doc = "Rebill details for an invoice."]
    #[serde(rename = "rebillDetails", default, skip_serializing_if = "Option::is_none")]
    pub rebill_details: Option<serde_json::Value>,
}
impl RebillDetails {
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
#[doc = "The definition of the reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Reservation {
    #[doc = "The id of the reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the reservation."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The location of the reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The property of reservation sku object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ReservationSkuProperty>,
    #[doc = "The property of reservation object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationProperty>,
}
impl Reservation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ReservationAppliedScope = String;
#[doc = "The property of reservation object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationProperty {
    #[doc = "The array of applied scopes of a reservation. Will be null if the reservation is in Shared scope"]
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Vec::is_empty")]
    pub applied_scopes: Vec<ReservationAppliedScope>,
    #[doc = "The applied scope type of the reservation."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<String>,
    #[doc = "The reserved source type of the reservation, e.g. virtual machine."]
    #[serde(rename = "reservedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<String>,
    #[doc = "The number of the reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[doc = "The provisioning state of the reservation, e.g. Succeeded"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The expiry date of the reservation"]
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[doc = "The provisioning state of the reservation, e.g. Succeeded"]
    #[serde(rename = "provisioningSubState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_sub_state: Option<String>,
    #[doc = "The display name of the reservation"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The provisioning state of the reservation for display, e.g. Succeeded"]
    #[serde(rename = "displayProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub display_provisioning_state: Option<String>,
    #[doc = "The renew state of the reservation for display, e.g. On"]
    #[serde(rename = "userFriendlyRenewState", default, skip_serializing_if = "Option::is_none")]
    pub user_friendly_renew_state: Option<String>,
    #[doc = "The applied scope type of the reservation for display, e.g. Shared"]
    #[serde(rename = "userFriendlyAppliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub user_friendly_applied_scope_type: Option<String>,
    #[doc = "The effective date time of the reservation"]
    #[serde(rename = "effectiveDateTime", default, skip_serializing_if = "Option::is_none")]
    pub effective_date_time: Option<String>,
    #[doc = "The sku description of the reservation"]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[doc = "The term of the reservation, e.g. P1Y"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[doc = "The renew state of the reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<bool>,
    #[doc = "The renew source of the reservation"]
    #[serde(rename = "renewSource", default, skip_serializing_if = "Option::is_none")]
    pub renew_source: Option<String>,
    #[doc = "Reservation utilization"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utilization: Option<reservation_property::Utilization>,
}
impl ReservationProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod reservation_property {
    use super::*;
    #[doc = "Reservation utilization"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Utilization {
        #[doc = "The number of days trend for a reservation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub trend: Option<String>,
        #[doc = "The array of aggregates of a reservation's utilization"]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub aggregates: Vec<ReservationUtilizationAggregates>,
    }
    impl Utilization {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The property of reservation sku object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSkuProperty {
    #[doc = "The name of the reservation sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ReservationSkuProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The roll up count summary of reservations in each state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSummary {
    #[doc = "The number of reservation in Succeeded state"]
    #[serde(rename = "succeededCount", default, skip_serializing_if = "Option::is_none")]
    pub succeeded_count: Option<f64>,
    #[doc = "The number of reservation in Failed state"]
    #[serde(rename = "failedCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<f64>,
    #[doc = "The number of reservation in Expiring state"]
    #[serde(rename = "expiringCount", default, skip_serializing_if = "Option::is_none")]
    pub expiring_count: Option<f64>,
    #[doc = "The number of reservation in Expired state"]
    #[serde(rename = "expiredCount", default, skip_serializing_if = "Option::is_none")]
    pub expired_count: Option<f64>,
    #[doc = "The number of reservation in Pending state"]
    #[serde(rename = "pendingCount", default, skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<f64>,
    #[doc = "The number of reservation in Cancelled state"]
    #[serde(rename = "cancelledCount", default, skip_serializing_if = "Option::is_none")]
    pub cancelled_count: Option<f64>,
}
impl ReservationSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The aggregate values of reservation utilization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationUtilizationAggregates {
    #[doc = "The grain of the aggregate"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grain: Option<f64>,
    #[doc = "The grain unit of the aggregate"]
    #[serde(rename = "grainUnit", default, skip_serializing_if = "Option::is_none")]
    pub grain_unit: Option<String>,
    #[doc = "The aggregate value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[doc = "The aggregate value unit"]
    #[serde(rename = "valueUnit", default, skip_serializing_if = "Option::is_none")]
    pub value_unit: Option<String>,
}
impl ReservationUtilizationAggregates {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of reservations and summary of roll out count of reservations in each state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationsListResult {
    #[doc = "The list of reservations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Reservation>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The roll up count summary of reservations in each state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReservationSummary>,
}
impl azure_core::Continuable for ReservationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ReservationsListResult {
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error code of the transfer validation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SubscriptionTransferValidationErrorCode")]
pub enum SubscriptionTransferValidationErrorCode {
    BillingAccountInactive,
    CrossBillingAccountNotAllowed,
    DestinationBillingProfileInactive,
    DestinationBillingProfileNotFound,
    DestinationBillingProfilePastDue,
    DestinationInvoiceSectionInactive,
    DestinationInvoiceSectionNotFound,
    InsufficientPermissionOnDestination,
    InsufficientPermissionOnSource,
    InvalidDestination,
    InvalidSource,
    MarketplaceNotEnabledOnDestination,
    NotAvailableForDestinationMarket,
    ProductInactive,
    ProductNotFound,
    ProductTypeNotSupported,
    SourceBillingProfilePastDue,
    SourceInvoiceSectionInactive,
    SubscriptionNotActive,
    SubscriptionTypeNotSupported,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SubscriptionTransferValidationErrorCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SubscriptionTransferValidationErrorCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SubscriptionTransferValidationErrorCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::BillingAccountInactive => {
                serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 0u32, "BillingAccountInactive")
            }
            Self::CrossBillingAccountNotAllowed => {
                serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 1u32, "CrossBillingAccountNotAllowed")
            }
            Self::DestinationBillingProfileInactive => {
                serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 2u32, "DestinationBillingProfileInactive")
            }
            Self::DestinationBillingProfileNotFound => {
                serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 3u32, "DestinationBillingProfileNotFound")
            }
            Self::DestinationBillingProfilePastDue => {
                serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 4u32, "DestinationBillingProfilePastDue")
            }
            Self::DestinationInvoiceSectionInactive => {
                serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 5u32, "DestinationInvoiceSectionInactive")
            }
            Self::DestinationInvoiceSectionNotFound => {
                serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 6u32, "DestinationInvoiceSectionNotFound")
            }
            Self::InsufficientPermissionOnDestination => serializer.serialize_unit_variant(
                "SubscriptionTransferValidationErrorCode",
                7u32,
                "InsufficientPermissionOnDestination",
            ),
            Self::InsufficientPermissionOnSource => {
                serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 8u32, "InsufficientPermissionOnSource")
            }
            Self::InvalidDestination => {
                serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 9u32, "InvalidDestination")
            }
            Self::InvalidSource => serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 10u32, "InvalidSource"),
            Self::MarketplaceNotEnabledOnDestination => serializer.serialize_unit_variant(
                "SubscriptionTransferValidationErrorCode",
                11u32,
                "MarketplaceNotEnabledOnDestination",
            ),
            Self::NotAvailableForDestinationMarket => {
                serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 12u32, "NotAvailableForDestinationMarket")
            }
            Self::ProductInactive => serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 13u32, "ProductInactive"),
            Self::ProductNotFound => serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 14u32, "ProductNotFound"),
            Self::ProductTypeNotSupported => {
                serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 15u32, "ProductTypeNotSupported")
            }
            Self::SourceBillingProfilePastDue => {
                serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 16u32, "SourceBillingProfilePastDue")
            }
            Self::SourceInvoiceSectionInactive => {
                serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 17u32, "SourceInvoiceSectionInactive")
            }
            Self::SubscriptionNotActive => {
                serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 18u32, "SubscriptionNotActive")
            }
            Self::SubscriptionTypeNotSupported => {
                serializer.serialize_unit_variant("SubscriptionTransferValidationErrorCode", 19u32, "SubscriptionTypeNotSupported")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Possible cloud environments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TargetCloud")]
pub enum TargetCloud {
    #[serde(rename = "USGov")]
    UsGov,
    #[serde(rename = "USNat")]
    UsNat,
    #[serde(rename = "USSec")]
    UsSec,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TargetCloud {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TargetCloud {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TargetCloud {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UsGov => serializer.serialize_unit_variant("TargetCloud", 0u32, "USGov"),
            Self::UsNat => serializer.serialize_unit_variant("TargetCloud", 1u32, "USNat"),
            Self::UsSec => serializer.serialize_unit_variant("TargetCloud", 2u32, "USSec"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A transaction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Transaction {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TransactionProperties>,
}
impl Transaction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of transactions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransactionListResult {
    #[doc = "The list of transactions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Transaction>,
    #[doc = "Total number of records."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<f64>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TransactionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TransactionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a transaction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransactionProperties {
    #[doc = "The kind of transaction. Options are all or reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<transaction_properties::Kind>,
    #[doc = "The date of transaction."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub date: Option<time::OffsetDateTime>,
    #[doc = "Invoice on which the transaction was billed or 'pending' if the transaction is not billed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    #[doc = "The ID of the invoice on which the transaction was billed. This field is only applicable for transactions which are billed."]
    #[serde(rename = "invoiceId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[doc = "The order ID of the reservation. The field is only applicable for transaction of kind reservation."]
    #[serde(rename = "orderId", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[doc = "The name of the reservation order. The field is only applicable for transactions of kind reservation."]
    #[serde(rename = "orderName", default, skip_serializing_if = "Option::is_none")]
    pub order_name: Option<String>,
    #[doc = "The family of the product for which the transaction took place."]
    #[serde(rename = "productFamily", default, skip_serializing_if = "Option::is_none")]
    pub product_family: Option<String>,
    #[doc = "The ID of the product type for which the transaction took place."]
    #[serde(rename = "productTypeId", default, skip_serializing_if = "Option::is_none")]
    pub product_type_id: Option<String>,
    #[doc = "The type of the product for which the transaction took place."]
    #[serde(rename = "productType", default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[doc = "The description of the product for which the transaction took place."]
    #[serde(rename = "productDescription", default, skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[doc = "The type of transaction."]
    #[serde(rename = "transactionType", default, skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<transaction_properties::TransactionType>,
    #[doc = "The amount."]
    #[serde(rename = "transactionAmount", default, skip_serializing_if = "Option::is_none")]
    pub transaction_amount: Option<Amount>,
    #[doc = "The quantity purchased in the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[doc = "The ID of the invoice section which will be billed for the transaction."]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "The name of the invoice section which will be billed for the transaction."]
    #[serde(rename = "invoiceSectionDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_display_name: Option<String>,
    #[doc = "The ID of the billing profile which will be billed for the transaction."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The name of the billing profile which will be billed for the transaction."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The ID of the customer for which the transaction took place. The field is applicable only for Microsoft Partner Agreement billing account."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "The name of the customer for which the transaction took place. The field is applicable only for Microsoft Partner Agreement billing account."]
    #[serde(rename = "customerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub customer_display_name: Option<String>,
    #[doc = "The ID of the subscription that was used for the transaction. The field is only applicable for transaction of kind reservation."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The name of the subscription that was used for the transaction. The field is only applicable for transaction of kind reservation."]
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
    #[doc = "The type of azure plan of the subscription that was used for the transaction."]
    #[serde(rename = "azurePlan", default, skip_serializing_if = "Option::is_none")]
    pub azure_plan: Option<String>,
    #[doc = "The amount."]
    #[serde(rename = "azureCreditApplied", default, skip_serializing_if = "Option::is_none")]
    pub azure_credit_applied: Option<Amount>,
    #[doc = "The ISO 4217 code for the currency in which this transaction is billed."]
    #[serde(rename = "billingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency: Option<String>,
    #[doc = "The percentage discount, if any, applied to this transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    #[doc = "The amount."]
    #[serde(rename = "effectivePrice", default, skip_serializing_if = "Option::is_none")]
    pub effective_price: Option<Amount>,
    #[doc = "The exchange rate used to convert charged amount to billing currency, if applicable."]
    #[serde(rename = "exchangeRate", default, skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<f64>,
    #[doc = "The amount."]
    #[serde(rename = "marketPrice", default, skip_serializing_if = "Option::is_none")]
    pub market_price: Option<Amount>,
    #[doc = "The ISO 4217 code for the currency in which the product is priced."]
    #[serde(rename = "pricingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency: Option<String>,
    #[doc = "The date of the purchase of the product, or the start date of the month in which usage started."]
    #[serde(rename = "servicePeriodStartDate", with = "azure_core::date::rfc3339::option")]
    pub service_period_start_date: Option<time::OffsetDateTime>,
    #[doc = "The end date of the product term, or the end date of the month in which usage ended."]
    #[serde(rename = "servicePeriodEndDate", with = "azure_core::date::rfc3339::option")]
    pub service_period_end_date: Option<time::OffsetDateTime>,
    #[doc = "The amount."]
    #[serde(rename = "subTotal", default, skip_serializing_if = "Option::is_none")]
    pub sub_total: Option<Amount>,
    #[doc = "The amount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<Amount>,
    #[doc = "The unit of measure used to bill for the product. For example, compute services are billed per hour."]
    #[serde(rename = "unitOfMeasure", default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
    #[doc = "The number of units used for a given product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub units: Option<f64>,
    #[doc = "The description for the unit of measure for a given product."]
    #[serde(rename = "unitType", default, skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<String>,
}
impl TransactionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod transaction_properties {
    use super::*;
    #[doc = "The kind of transaction. Options are all or reservation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "reservation")]
        Reservation,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::All => serializer.serialize_unit_variant("Kind", 0u32, "all"),
                Self::Reservation => serializer.serialize_unit_variant("Kind", 1u32, "reservation"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of transaction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TransactionType")]
    pub enum TransactionType {
        Purchase,
        #[serde(rename = "Usage Charge")]
        UsageCharge,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TransactionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TransactionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TransactionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Purchase => serializer.serialize_unit_variant("TransactionType", 0u32, "Purchase"),
                Self::UsageCharge => serializer.serialize_unit_variant("TransactionType", 1u32, "Usage Charge"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Request parameters to transfer billing subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferBillingSubscriptionRequestProperties {
    #[doc = "The destination invoice section id."]
    #[serde(rename = "destinationInvoiceSectionId")]
    pub destination_invoice_section_id: String,
}
impl TransferBillingSubscriptionRequestProperties {
    pub fn new(destination_invoice_section_id: String) -> Self {
        Self {
            destination_invoice_section_id,
        }
    }
}
#[doc = "The properties of the product to initiate a transfer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransferProductRequestProperties {
    #[doc = "The destination invoice section id."]
    #[serde(rename = "destinationInvoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub destination_invoice_section_id: Option<String>,
}
impl TransferProductRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the address validation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateAddressResponse {
    #[doc = "Status of the address validation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AddressValidationStatus>,
    #[doc = "The list of suggested addresses."]
    #[serde(rename = "suggestedAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub suggested_addresses: Vec<AddressDetails>,
    #[doc = "Validation error message."]
    #[serde(rename = "validationMessage", default, skip_serializing_if = "Option::is_none")]
    pub validation_message: Option<String>,
}
impl ValidateAddressResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details of the product transfer eligibility validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateProductTransferEligibilityError {
    #[doc = "Error code of the transfer validation response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<ProductTransferValidationErrorCode>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Detailed error message explaining the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl ValidateProductTransferEligibilityError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the product transfer eligibility validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateProductTransferEligibilityResult {
    #[doc = "Specifies whether the transfer is eligible or not."]
    #[serde(rename = "isMoveEligible", default, skip_serializing_if = "Option::is_none")]
    pub is_move_eligible: Option<bool>,
    #[doc = "Error details of the product transfer eligibility validation."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ValidateProductTransferEligibilityError>,
}
impl ValidateProductTransferEligibilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details of the transfer eligibility validation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateSubscriptionTransferEligibilityError {
    #[doc = "Error code of the transfer validation response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<SubscriptionTransferValidationErrorCode>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Detailed error message explaining the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl ValidateSubscriptionTransferEligibilityError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the transfer eligibility validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateSubscriptionTransferEligibilityResult {
    #[doc = "Specifies whether the subscription is eligible to be transferred."]
    #[serde(rename = "isMoveEligible", default, skip_serializing_if = "Option::is_none")]
    pub is_move_eligible: Option<bool>,
    #[doc = "Error details of the transfer eligibility validation"]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ValidateSubscriptionTransferEligibilityError>,
}
impl ValidateSubscriptionTransferEligibilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
