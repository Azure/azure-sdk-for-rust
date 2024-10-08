#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Request parameters to accept transfer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcceptTransferProperties {
    #[doc = "Request parameters to accept transfer."]
    #[serde(
        rename = "productDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub product_details: Vec<ProductDetails>,
}
impl AcceptTransferProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request parameters to accept transfer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcceptTransferRequest {
    #[doc = "Request parameters to accept transfer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AcceptTransferProperties>,
}
impl AcceptTransferRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Address details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressDetails {
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
    #[doc = "Company name. Optional for MCA Individual (Pay-as-you-go)."]
    #[serde(rename = "companyName", default, skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[doc = "Country code uses ISO 3166-1 Alpha-2 format."]
    pub country: String,
    #[doc = "Address district."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[doc = "Email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "First name. Optional for MCA Enterprise."]
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Last name. Optional for MCA Enterprise."]
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Middle name."]
    #[serde(rename = "middleName", default, skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[doc = "Phone number."]
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[doc = "Postal code."]
    #[serde(rename = "postalCode", default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "Address region."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "Indicates if the address is incomplete."]
    #[serde(rename = "isValidAddress", default, skip_serializing_if = "Option::is_none")]
    pub is_valid_address: Option<bool>,
}
impl AddressDetails {
    pub fn new(address_line1: String, country: String) -> Self {
        Self {
            address_line1,
            address_line2: None,
            address_line3: None,
            city: None,
            company_name: None,
            country,
            district: None,
            email: None,
            first_name: None,
            last_name: None,
            middle_name: None,
            phone_number: None,
            postal_code: None,
            region: None,
            is_valid_address: None,
        }
    }
}
#[doc = "Result of the address validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddressValidationResponse {
    #[doc = "Status of the address validation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<address_validation_response::Status>,
    #[doc = "The list of suggested addresses."]
    #[serde(
        rename = "suggestedAddresses",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub suggested_addresses: Vec<AddressDetails>,
    #[doc = "Validation error message."]
    #[serde(rename = "validationMessage", default, skip_serializing_if = "Option::is_none")]
    pub validation_message: Option<String>,
}
impl AddressValidationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod address_validation_response {
    use super::*;
    #[doc = "Status of the address validation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Other,
        Valid,
        Invalid,
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
                Self::Other => serializer.serialize_unit_variant("Status", 0u32, "Other"),
                Self::Valid => serializer.serialize_unit_variant("Status", 1u32, "Valid"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 2u32, "Invalid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An agreement."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Agreement {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "An agreement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AgreementProperties>,
}
impl Agreement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgreementListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Agreement>,
}
impl azure_core::Continuable for AgreementListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AgreementListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An agreement."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgreementProperties {
    #[doc = "The mode of acceptance for an agreement."]
    #[serde(rename = "acceptanceMode", default, skip_serializing_if = "Option::is_none")]
    pub acceptance_mode: Option<agreement_properties::AcceptanceMode>,
    #[doc = "The URL to download the agreement."]
    #[serde(rename = "agreementLink", default, skip_serializing_if = "Option::is_none")]
    pub agreement_link: Option<String>,
    #[doc = "The list of billing profiles associated with agreement and present only for specific agreements."]
    #[serde(
        rename = "billingProfileInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub billing_profile_info: Vec<BillingProfileInfo>,
    #[doc = "The category of the agreement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<agreement_properties::Category>,
    #[doc = "The name of the agreement signed by a customer."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The date from which the agreement is effective."]
    #[serde(rename = "effectiveDate", default, with = "azure_core::date::rfc3339::option")]
    pub effective_date: Option<::time::OffsetDateTime>,
    #[doc = "The date when the agreement expires."]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<::time::OffsetDateTime>,
    #[doc = "The list of participants that participates in acceptance of an agreement."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub participants: Vec<Participant>,
    #[doc = "The current status of the agreement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The ID of the lead billing account if this agreement is part of the Customer Affiliate Purchase Terms."]
    #[serde(rename = "leadBillingAccountName", default, skip_serializing_if = "Option::is_none")]
    pub lead_billing_account_name: Option<String>,
}
impl AgreementProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod agreement_properties {
    use super::*;
    #[doc = "The mode of acceptance for an agreement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AcceptanceMode")]
    pub enum AcceptanceMode {
        Other,
        ClickToAccept,
        ESignEmbedded,
        ESignOffline,
        Implicit,
        Offline,
        PhysicalSign,
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
                Self::Other => serializer.serialize_unit_variant("AcceptanceMode", 0u32, "Other"),
                Self::ClickToAccept => serializer.serialize_unit_variant("AcceptanceMode", 1u32, "ClickToAccept"),
                Self::ESignEmbedded => serializer.serialize_unit_variant("AcceptanceMode", 2u32, "ESignEmbedded"),
                Self::ESignOffline => serializer.serialize_unit_variant("AcceptanceMode", 3u32, "ESignOffline"),
                Self::Implicit => serializer.serialize_unit_variant("AcceptanceMode", 4u32, "Implicit"),
                Self::Offline => serializer.serialize_unit_variant("AcceptanceMode", 5u32, "Offline"),
                Self::PhysicalSign => serializer.serialize_unit_variant("AcceptanceMode", 6u32, "PhysicalSign"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The category of the agreement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Category")]
    pub enum Category {
        Other,
        AffiliatePurchaseTerms,
        IndirectForGovernmentAgreement,
        MicrosoftCustomerAgreement,
        MicrosoftPartnerAgreement,
        #[serde(rename = "UKCloudComputeFramework")]
        UkCloudComputeFramework,
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
                Self::Other => serializer.serialize_unit_variant("Category", 0u32, "Other"),
                Self::AffiliatePurchaseTerms => serializer.serialize_unit_variant("Category", 1u32, "AffiliatePurchaseTerms"),
                Self::IndirectForGovernmentAgreement => {
                    serializer.serialize_unit_variant("Category", 2u32, "IndirectForGovernmentAgreement")
                }
                Self::MicrosoftCustomerAgreement => serializer.serialize_unit_variant("Category", 3u32, "MicrosoftCustomerAgreement"),
                Self::MicrosoftPartnerAgreement => serializer.serialize_unit_variant("Category", 4u32, "MicrosoftPartnerAgreement"),
                Self::UkCloudComputeFramework => serializer.serialize_unit_variant("Category", 5u32, "UKCloudComputeFramework"),
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
    #[doc = "The amount value. For example, if the currency is USD, then a value of 600 would be $600.00."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl Amount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties specific to applied scope type. Not required if not applicable."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedScopeProperties {
    #[doc = "Tenant ID where the savings plan where the benefit is applied."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<TenantId>,
    #[doc = "Fully-qualified identifier of the management group where the benefit is applied."]
    #[serde(rename = "managementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub management_group_id: Option<ManagementGroupId>,
    #[doc = "Fully-qualified identifier of the subscription where the benefit is applied."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<SubscriptionId>,
    #[doc = "Fully-qualified identifier of the resource group where the benefit is applied."]
    #[serde(rename = "resourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<ResourceGroupId>,
    #[doc = "Display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl AppliedScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the Applied Scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AppliedScopeType")]
pub enum AppliedScopeType {
    Single,
    Shared,
    ManagementGroup,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AppliedScopeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AppliedScopeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AppliedScopeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Single => serializer.serialize_unit_variant("AppliedScopeType", 0u32, "Single"),
            Self::Shared => serializer.serialize_unit_variant("AppliedScopeType", 1u32, "Shared"),
            Self::ManagementGroup => serializer.serialize_unit_variant("AppliedScopeType", 2u32, "ManagementGroup"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type AppliedScopes = Vec<String>;
#[doc = "An associated tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssociatedTenant {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "An associated tenant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssociatedTenantProperties>,
}
impl AssociatedTenant {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssociatedTenantListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AssociatedTenant>,
}
impl azure_core::Continuable for AssociatedTenantListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AssociatedTenantListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An associated tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssociatedTenantProperties {
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<associated_tenant_properties::ProvisioningState>,
    #[doc = "The name of the associated tenant."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The ID that uniquely identifies a tenant."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The state determines whether users from the associated tenant can be assigned roles for commerce activities like viewing and downloading invoices, managing payments, and making purchases."]
    #[serde(rename = "billingManagementState", default, skip_serializing_if = "Option::is_none")]
    pub billing_management_state: Option<associated_tenant_properties::BillingManagementState>,
    #[doc = "The state determines whether subscriptions and licenses can be provisioned in the associated tenant. It can be set to 'Pending' to initiate a billing request."]
    #[serde(rename = "provisioningManagementState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_management_state: Option<associated_tenant_properties::ProvisioningManagementState>,
    #[doc = "The unique identifier for the billing request that is created when enabling provisioning for an associated tenant."]
    #[serde(rename = "provisioningBillingRequestId", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_billing_request_id: Option<String>,
}
impl AssociatedTenantProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod associated_tenant_properties {
    use super::*;
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Canceled,
        Failed,
        New,
        Pending,
        Provisioning,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::New => serializer.serialize_unit_variant("ProvisioningState", 3u32, "New"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The state determines whether users from the associated tenant can be assigned roles for commerce activities like viewing and downloading invoices, managing payments, and making purchases."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingManagementState")]
    pub enum BillingManagementState {
        Other,
        NotAllowed,
        Active,
        Revoked,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingManagementState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingManagementState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingManagementState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("BillingManagementState", 0u32, "Other"),
                Self::NotAllowed => serializer.serialize_unit_variant("BillingManagementState", 1u32, "NotAllowed"),
                Self::Active => serializer.serialize_unit_variant("BillingManagementState", 2u32, "Active"),
                Self::Revoked => serializer.serialize_unit_variant("BillingManagementState", 3u32, "Revoked"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The state determines whether subscriptions and licenses can be provisioned in the associated tenant. It can be set to 'Pending' to initiate a billing request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningManagementState")]
    pub enum ProvisioningManagementState {
        Other,
        NotRequested,
        Active,
        Pending,
        BillingRequestExpired,
        BillingRequestDeclined,
        Revoked,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningManagementState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningManagementState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningManagementState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("ProvisioningManagementState", 0u32, "Other"),
                Self::NotRequested => serializer.serialize_unit_variant("ProvisioningManagementState", 1u32, "NotRequested"),
                Self::Active => serializer.serialize_unit_variant("ProvisioningManagementState", 2u32, "Active"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningManagementState", 3u32, "Pending"),
                Self::BillingRequestExpired => {
                    serializer.serialize_unit_variant("ProvisioningManagementState", 4u32, "BillingRequestExpired")
                }
                Self::BillingRequestDeclined => {
                    serializer.serialize_unit_variant("ProvisioningManagementState", 5u32, "BillingRequestDeclined")
                }
                Self::Revoked => serializer.serialize_unit_variant("ProvisioningManagementState", 6u32, "Revoked"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Available Credit or Payment on Account Balance. The credit balance can be used to settle due or past due invoices."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableBalance {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "The Available Credit or Payment on Account Balance. The credit balance can be used to settle due or past due invoices."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailableBalanceProperties>,
}
impl AvailableBalance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Available Credit or Payment on Account Balance. The credit balance can be used to settle due or past due invoices."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableBalanceProperties {
    #[doc = "Credit amount for immediate payment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<serde_json::Value>,
    #[doc = "The list of payments on accounts."]
    #[serde(
        rename = "paymentsOnAccount",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub payments_on_account: Vec<PaymentOnAccount>,
    #[doc = "Total amount of payments on accounts."]
    #[serde(rename = "totalPaymentsOnAccount", default, skip_serializing_if = "Option::is_none")]
    pub total_payments_on_account: Option<serde_json::Value>,
}
impl AvailableBalanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the Azure plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzurePlan {
    #[doc = "The ID that uniquely identifies a product."]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[doc = "The ID that uniquely identifies a sku."]
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
#[doc = "Details of the beneficiary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Beneficiary {
    #[doc = "The ID that uniquely identifies a tenant."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The ID that uniquely identifies a user in a tenant."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}
impl Beneficiary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the Savings plan term in ISO 8601 format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BenefitTerm")]
pub enum BenefitTerm {
    #[serde(rename = "P1Y")]
    P1y,
    #[serde(rename = "P3Y")]
    P3y,
    #[serde(rename = "P5Y")]
    P5y,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BenefitTerm {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BenefitTerm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BenefitTerm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::P1y => serializer.serialize_unit_variant("BenefitTerm", 0u32, "P1Y"),
            Self::P3y => serializer.serialize_unit_variant("BenefitTerm", 1u32, "P3Y"),
            Self::P5y => serializer.serialize_unit_variant("BenefitTerm", 2u32, "P5Y"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A billing account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingAccount {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "A billing account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingAccountProperties>,
}
impl BillingAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type BillingAccountId = String;
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingAccountListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BillingAccount>,
}
impl azure_core::Continuable for BillingAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BillingAccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A billing account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingAccountPatch {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "A billing account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingAccountProperties>,
}
impl BillingAccountPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A policy at billing account scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingAccountPolicy {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "A policy at billing account scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingAccountPolicyProperties>,
}
impl BillingAccountPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A policy at billing account scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingAccountPolicyProperties {
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<billing_account_policy_properties::ProvisioningState>,
    #[doc = "The policies for Enterprise Agreement enrollments."]
    #[serde(rename = "enterpriseAgreementPolicies", default, skip_serializing_if = "Option::is_none")]
    pub enterprise_agreement_policies: Option<serde_json::Value>,
    #[doc = "The policy that controls whether Azure marketplace purchases are allowed."]
    #[serde(rename = "marketplacePurchases", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_purchases: Option<billing_account_policy_properties::MarketplacePurchases>,
    #[doc = "The policy that controls whether Azure reservation purchases are allowed."]
    #[serde(rename = "reservationPurchases", default, skip_serializing_if = "Option::is_none")]
    pub reservation_purchases: Option<billing_account_policy_properties::ReservationPurchases>,
    #[doc = "The policy that controls whether users with Azure savings plan purchase are allowed."]
    #[serde(rename = "savingsPlanPurchases", default, skip_serializing_if = "Option::is_none")]
    pub savings_plan_purchases: Option<billing_account_policy_properties::SavingsPlanPurchases>,
    #[doc = "List of all policies defined at the billing scope."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policies: Vec<PolicySummary>,
}
impl BillingAccountPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_account_policy_properties {
    use super::*;
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Canceled,
        Failed,
        New,
        Pending,
        Provisioning,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::New => serializer.serialize_unit_variant("ProvisioningState", 3u32, "New"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The policy that controls whether Azure marketplace purchases are allowed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MarketplacePurchases")]
    pub enum MarketplacePurchases {
        Other,
        AllAllowed,
        Disabled,
        NotAllowed,
        OnlyFreeAllowed,
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
                Self::Other => serializer.serialize_unit_variant("MarketplacePurchases", 0u32, "Other"),
                Self::AllAllowed => serializer.serialize_unit_variant("MarketplacePurchases", 1u32, "AllAllowed"),
                Self::Disabled => serializer.serialize_unit_variant("MarketplacePurchases", 2u32, "Disabled"),
                Self::NotAllowed => serializer.serialize_unit_variant("MarketplacePurchases", 3u32, "NotAllowed"),
                Self::OnlyFreeAllowed => serializer.serialize_unit_variant("MarketplacePurchases", 4u32, "OnlyFreeAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The policy that controls whether Azure reservation purchases are allowed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReservationPurchases")]
    pub enum ReservationPurchases {
        Other,
        Allowed,
        Disabled,
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
                Self::Other => serializer.serialize_unit_variant("ReservationPurchases", 0u32, "Other"),
                Self::Allowed => serializer.serialize_unit_variant("ReservationPurchases", 1u32, "Allowed"),
                Self::Disabled => serializer.serialize_unit_variant("ReservationPurchases", 2u32, "Disabled"),
                Self::NotAllowed => serializer.serialize_unit_variant("ReservationPurchases", 3u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The policy that controls whether users with Azure savings plan purchase are allowed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SavingsPlanPurchases")]
    pub enum SavingsPlanPurchases {
        Other,
        Allowed,
        Disabled,
        NotAllowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SavingsPlanPurchases {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SavingsPlanPurchases {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SavingsPlanPurchases {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("SavingsPlanPurchases", 0u32, "Other"),
                Self::Allowed => serializer.serialize_unit_variant("SavingsPlanPurchases", 1u32, "Allowed"),
                Self::Disabled => serializer.serialize_unit_variant("SavingsPlanPurchases", 2u32, "Disabled"),
                Self::NotAllowed => serializer.serialize_unit_variant("SavingsPlanPurchases", 3u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A billing account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingAccountProperties {
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<billing_account_properties::ProvisioningState>,
    #[doc = "The current status of the billing account."]
    #[serde(rename = "accountStatus", default, skip_serializing_if = "Option::is_none")]
    pub account_status: Option<billing_account_properties::AccountStatus>,
    #[doc = "The type of customer."]
    #[serde(rename = "accountType", default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<billing_account_properties::AccountType>,
    #[doc = "The tier of the account."]
    #[serde(rename = "accountSubType", default, skip_serializing_if = "Option::is_none")]
    pub account_sub_type: Option<billing_account_properties::AccountSubType>,
    #[doc = "Reason for the specified billing account status."]
    #[serde(rename = "accountStatusReasonCode", default, skip_serializing_if = "Option::is_none")]
    pub account_status_reason_code: Option<billing_account_properties::AccountStatusReasonCode>,
    #[doc = "The type of agreement."]
    #[serde(rename = "agreementType", default, skip_serializing_if = "Option::is_none")]
    pub agreement_type: Option<billing_account_properties::AgreementType>,
    #[doc = "The billing account name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The properties of an enrollment."]
    #[serde(rename = "enrollmentDetails", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_details: Option<serde_json::Value>,
    #[doc = "Indicates whether user has read access to the billing account."]
    #[serde(rename = "hasReadAccess", default, skip_serializing_if = "Option::is_none")]
    pub has_read_access: Option<bool>,
    #[doc = "Indicates whether or not the billing account has any billing profiles."]
    #[serde(rename = "hasNoBillingProfiles", default, skip_serializing_if = "Option::is_none")]
    pub has_no_billing_profiles: Option<bool>,
    #[doc = "Notification email address for legacy account. Available for agreement type Microsoft Online Services Program."]
    #[serde(rename = "notificationEmailAddress", default, skip_serializing_if = "Option::is_none")]
    pub notification_email_address: Option<String>,
    #[doc = "The tenant that was used to set up the billing account. By default, only users from this tenant can get role assignments on the billing account and all purchases are provisioned in this tenant."]
    #[serde(rename = "primaryBillingTenantId", default, skip_serializing_if = "Option::is_none")]
    pub primary_billing_tenant_id: Option<String>,
    #[doc = "The address of the individual or organization that is responsible for the billing account."]
    #[serde(rename = "soldTo", default, skip_serializing_if = "Option::is_none")]
    pub sold_to: Option<serde_json::Value>,
    #[doc = "Describes the registration number of the organization linked with the billing account."]
    #[serde(rename = "registrationNumber", default, skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<serde_json::Value>,
    #[doc = "Identifies the billing relationships represented by a billing account. The billing relationship may be between Microsoft, the customer, and/or a third-party."]
    #[serde(
        rename = "billingRelationshipTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub billing_relationship_types: Vec<String>,
    #[doc = "Qualifications for pricing on a billing account. Values may be Commercial, Education, Charity or Government."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub qualifications: Vec<String>,
    #[doc = "A list of tax identifiers for the billing account."]
    #[serde(
        rename = "taxIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tax_ids: Vec<TaxIdentifier>,
}
impl BillingAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_account_properties {
    use super::*;
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Canceled,
        Failed,
        New,
        Pending,
        Provisioning,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::New => serializer.serialize_unit_variant("ProvisioningState", 3u32, "New"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current status of the billing account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccountStatus")]
    pub enum AccountStatus {
        Other,
        Active,
        UnderReview,
        Disabled,
        Deleted,
        Extended,
        Pending,
        New,
        Expired,
        Terminated,
        Transferred,
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
                Self::Other => serializer.serialize_unit_variant("AccountStatus", 0u32, "Other"),
                Self::Active => serializer.serialize_unit_variant("AccountStatus", 1u32, "Active"),
                Self::UnderReview => serializer.serialize_unit_variant("AccountStatus", 2u32, "UnderReview"),
                Self::Disabled => serializer.serialize_unit_variant("AccountStatus", 3u32, "Disabled"),
                Self::Deleted => serializer.serialize_unit_variant("AccountStatus", 4u32, "Deleted"),
                Self::Extended => serializer.serialize_unit_variant("AccountStatus", 5u32, "Extended"),
                Self::Pending => serializer.serialize_unit_variant("AccountStatus", 6u32, "Pending"),
                Self::New => serializer.serialize_unit_variant("AccountStatus", 7u32, "New"),
                Self::Expired => serializer.serialize_unit_variant("AccountStatus", 8u32, "Expired"),
                Self::Terminated => serializer.serialize_unit_variant("AccountStatus", 9u32, "Terminated"),
                Self::Transferred => serializer.serialize_unit_variant("AccountStatus", 10u32, "Transferred"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of customer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccountType")]
    pub enum AccountType {
        Other,
        Enterprise,
        Individual,
        Partner,
        Reseller,
        ClassicPartner,
        Internal,
        Tenant,
        Business,
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
                Self::Other => serializer.serialize_unit_variant("AccountType", 0u32, "Other"),
                Self::Enterprise => serializer.serialize_unit_variant("AccountType", 1u32, "Enterprise"),
                Self::Individual => serializer.serialize_unit_variant("AccountType", 2u32, "Individual"),
                Self::Partner => serializer.serialize_unit_variant("AccountType", 3u32, "Partner"),
                Self::Reseller => serializer.serialize_unit_variant("AccountType", 4u32, "Reseller"),
                Self::ClassicPartner => serializer.serialize_unit_variant("AccountType", 5u32, "ClassicPartner"),
                Self::Internal => serializer.serialize_unit_variant("AccountType", 6u32, "Internal"),
                Self::Tenant => serializer.serialize_unit_variant("AccountType", 7u32, "Tenant"),
                Self::Business => serializer.serialize_unit_variant("AccountType", 8u32, "Business"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The tier of the account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccountSubType")]
    pub enum AccountSubType {
        Other,
        None,
        Individual,
        Professional,
        Enterprise,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccountSubType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccountSubType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccountSubType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("AccountSubType", 0u32, "Other"),
                Self::None => serializer.serialize_unit_variant("AccountSubType", 1u32, "None"),
                Self::Individual => serializer.serialize_unit_variant("AccountSubType", 2u32, "Individual"),
                Self::Professional => serializer.serialize_unit_variant("AccountSubType", 3u32, "Professional"),
                Self::Enterprise => serializer.serialize_unit_variant("AccountSubType", 4u32, "Enterprise"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Reason for the specified billing account status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccountStatusReasonCode")]
    pub enum AccountStatusReasonCode {
        Other,
        UnusualActivity,
        ManuallyTerminated,
        Expired,
        Transferred,
        TerminateProcessing,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccountStatusReasonCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccountStatusReasonCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccountStatusReasonCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("AccountStatusReasonCode", 0u32, "Other"),
                Self::UnusualActivity => serializer.serialize_unit_variant("AccountStatusReasonCode", 1u32, "UnusualActivity"),
                Self::ManuallyTerminated => serializer.serialize_unit_variant("AccountStatusReasonCode", 2u32, "ManuallyTerminated"),
                Self::Expired => serializer.serialize_unit_variant("AccountStatusReasonCode", 3u32, "Expired"),
                Self::Transferred => serializer.serialize_unit_variant("AccountStatusReasonCode", 4u32, "Transferred"),
                Self::TerminateProcessing => serializer.serialize_unit_variant("AccountStatusReasonCode", 5u32, "TerminateProcessing"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of agreement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AgreementType")]
    pub enum AgreementType {
        Other,
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
                Self::Other => serializer.serialize_unit_variant("AgreementType", 0u32, "Other"),
                Self::MicrosoftCustomerAgreement => serializer.serialize_unit_variant("AgreementType", 1u32, "MicrosoftCustomerAgreement"),
                Self::EnterpriseAgreement => serializer.serialize_unit_variant("AgreementType", 2u32, "EnterpriseAgreement"),
                Self::MicrosoftOnlineServicesProgram => {
                    serializer.serialize_unit_variant("AgreementType", 3u32, "MicrosoftOnlineServicesProgram")
                }
                Self::MicrosoftPartnerAgreement => serializer.serialize_unit_variant("AgreementType", 4u32, "MicrosoftPartnerAgreement"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The set of allowed action and not allowed actions a caller has on a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingPermission {
    #[doc = "The set of actions that the caller is allowed to perform."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<String>,
    #[doc = "The set of actions that the caller is not allowed to perform."]
    #[serde(
        rename = "notActions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub not_actions: Vec<String>,
}
impl BillingPermission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingPermissionListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BillingPermission>,
}
impl azure_core::Continuable for BillingPermissionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BillingPermissionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the billing plan in ISO 8601 format. Required only for monthly purchases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BillingPlan")]
pub enum BillingPlan {
    #[serde(rename = "P1M")]
    P1m,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BillingPlan {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BillingPlan {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BillingPlan {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::P1m => serializer.serialize_unit_variant("BillingPlan", 0u32, "P1M"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information describing the type of billing plan for this savings plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingPlanInformation {
    #[doc = "The price."]
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<Price>,
    #[doc = "Date when the billing plan has started."]
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[doc = "For recurring billing plans, indicates the date when next payment will be processed. Null when total is paid off."]
    #[serde(rename = "nextPaymentDueDate", default, skip_serializing_if = "Option::is_none")]
    pub next_payment_due_date: Option<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transactions: Vec<PaymentDetail>,
}
impl BillingPlanInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A billing profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProfile {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "A billing profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingProfileProperties>,
}
impl BillingProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type BillingProfileId = String;
#[doc = "Details about billing profile associated with agreement and available only for specific agreements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProfileInfo {
    #[doc = "The fully qualified ID that uniquely identifies a billing account."]
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<String>,
    #[doc = "The name of the billing profile."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The friendly ID that uniquely identifies a billing profile."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies a billing profile."]
    #[serde(rename = "billingProfileSystemId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_system_id: Option<String>,
    #[doc = "Billing account name. Available for a specific type of agreement."]
    #[serde(rename = "indirectRelationshipOrganizationName", default, skip_serializing_if = "Option::is_none")]
    pub indirect_relationship_organization_name: Option<String>,
}
impl BillingProfileInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProfileListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BillingProfile>,
}
impl azure_core::Continuable for BillingProfileListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BillingProfileListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A policy at billing profile scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProfilePolicy {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "A policy at billing profile scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingProfilePolicyProperties>,
}
impl BillingProfilePolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A policy at billing profile scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProfilePolicyProperties {
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<billing_profile_policy_properties::ProvisioningState>,
    #[doc = "The policies for Enterprise Agreement enrollments."]
    #[serde(rename = "enterpriseAgreementPolicies", default, skip_serializing_if = "Option::is_none")]
    pub enterprise_agreement_policies: Option<serde_json::Value>,
    #[doc = "The policy that controls invoice section label management at invoice section scope. This is allowed by default."]
    #[serde(rename = "invoiceSectionLabelManagement", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_label_management: Option<billing_profile_policy_properties::InvoiceSectionLabelManagement>,
    #[doc = "The policy that controls whether Azure marketplace purchases are allowed."]
    #[serde(rename = "marketplacePurchases", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_purchases: Option<billing_profile_policy_properties::MarketplacePurchases>,
    #[doc = "The policy that controls whether Azure reservation purchases are allowed."]
    #[serde(rename = "reservationPurchases", default, skip_serializing_if = "Option::is_none")]
    pub reservation_purchases: Option<billing_profile_policy_properties::ReservationPurchases>,
    #[doc = "The policy that controls whether users with Azure savings plan purchase are allowed."]
    #[serde(rename = "savingsPlanPurchases", default, skip_serializing_if = "Option::is_none")]
    pub savings_plan_purchases: Option<billing_profile_policy_properties::SavingsPlanPurchases>,
    #[doc = "The policy that controls whether the users in customer's organization can view charges at pay-as-you-go prices."]
    #[serde(rename = "viewCharges", default, skip_serializing_if = "Option::is_none")]
    pub view_charges: Option<billing_profile_policy_properties::ViewCharges>,
    #[doc = "List of all policies defined at the billing scope."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policies: Vec<PolicySummary>,
}
impl BillingProfilePolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_profile_policy_properties {
    use super::*;
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Canceled,
        Failed,
        New,
        Pending,
        Provisioning,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::New => serializer.serialize_unit_variant("ProvisioningState", 3u32, "New"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The policy that controls invoice section label management at invoice section scope. This is allowed by default."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InvoiceSectionLabelManagement")]
    pub enum InvoiceSectionLabelManagement {
        Other,
        Allowed,
        NotAllowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InvoiceSectionLabelManagement {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InvoiceSectionLabelManagement {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InvoiceSectionLabelManagement {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("InvoiceSectionLabelManagement", 0u32, "Other"),
                Self::Allowed => serializer.serialize_unit_variant("InvoiceSectionLabelManagement", 1u32, "Allowed"),
                Self::NotAllowed => serializer.serialize_unit_variant("InvoiceSectionLabelManagement", 2u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The policy that controls whether Azure marketplace purchases are allowed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MarketplacePurchases")]
    pub enum MarketplacePurchases {
        Other,
        AllAllowed,
        Disabled,
        NotAllowed,
        OnlyFreeAllowed,
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
                Self::Other => serializer.serialize_unit_variant("MarketplacePurchases", 0u32, "Other"),
                Self::AllAllowed => serializer.serialize_unit_variant("MarketplacePurchases", 1u32, "AllAllowed"),
                Self::Disabled => serializer.serialize_unit_variant("MarketplacePurchases", 2u32, "Disabled"),
                Self::NotAllowed => serializer.serialize_unit_variant("MarketplacePurchases", 3u32, "NotAllowed"),
                Self::OnlyFreeAllowed => serializer.serialize_unit_variant("MarketplacePurchases", 4u32, "OnlyFreeAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The policy that controls whether Azure reservation purchases are allowed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReservationPurchases")]
    pub enum ReservationPurchases {
        Other,
        Allowed,
        Disabled,
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
                Self::Other => serializer.serialize_unit_variant("ReservationPurchases", 0u32, "Other"),
                Self::Allowed => serializer.serialize_unit_variant("ReservationPurchases", 1u32, "Allowed"),
                Self::Disabled => serializer.serialize_unit_variant("ReservationPurchases", 2u32, "Disabled"),
                Self::NotAllowed => serializer.serialize_unit_variant("ReservationPurchases", 3u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The policy that controls whether users with Azure savings plan purchase are allowed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SavingsPlanPurchases")]
    pub enum SavingsPlanPurchases {
        Other,
        Allowed,
        Disabled,
        NotAllowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SavingsPlanPurchases {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SavingsPlanPurchases {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SavingsPlanPurchases {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("SavingsPlanPurchases", 0u32, "Other"),
                Self::Allowed => serializer.serialize_unit_variant("SavingsPlanPurchases", 1u32, "Allowed"),
                Self::Disabled => serializer.serialize_unit_variant("SavingsPlanPurchases", 2u32, "Disabled"),
                Self::NotAllowed => serializer.serialize_unit_variant("SavingsPlanPurchases", 3u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The policy that controls whether the users in customer's organization can view charges at pay-as-you-go prices."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ViewCharges")]
    pub enum ViewCharges {
        Other,
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
                Self::Other => serializer.serialize_unit_variant("ViewCharges", 0u32, "Other"),
                Self::Allowed => serializer.serialize_unit_variant("ViewCharges", 1u32, "Allowed"),
                Self::NotAllowed => serializer.serialize_unit_variant("ViewCharges", 2u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A billing profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProfileProperties {
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<billing_profile_properties::ProvisioningState>,
    #[doc = "Identifies the billing relationship represented by the billing profile. The billing relationship may be between Microsoft, the customer, and/or a third-party."]
    #[serde(rename = "billingRelationshipType", default, skip_serializing_if = "Option::is_none")]
    pub billing_relationship_type: Option<billing_profile_properties::BillingRelationshipType>,
    #[doc = "Billing address."]
    #[serde(rename = "billTo", default, skip_serializing_if = "Option::is_none")]
    pub bill_to: Option<serde_json::Value>,
    #[doc = "The currency in which the charges for the billing profile are billed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The name of the billing profile."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Information about the enabled azure plans."]
    #[serde(
        rename = "enabledAzurePlans",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enabled_azure_plans: Vec<AzurePlan>,
    #[doc = "Indicates whether user has read access to the billing profile."]
    #[serde(rename = "hasReadAccess", default, skip_serializing_if = "Option::is_none")]
    pub has_read_access: Option<bool>,
    #[doc = "Identifies the billing profile that is linked to another billing profile in indirect purchase motion."]
    #[serde(rename = "indirectRelationshipInfo", default, skip_serializing_if = "Option::is_none")]
    pub indirect_relationship_info: Option<serde_json::Value>,
    #[doc = "The day of the month when the invoice for the billing profile is generated."]
    #[serde(rename = "invoiceDay", default, skip_serializing_if = "Option::is_none")]
    pub invoice_day: Option<i32>,
    #[doc = "Flag controlling whether the invoices for the billing profile are sent through email."]
    #[serde(rename = "invoiceEmailOptIn", default, skip_serializing_if = "Option::is_none")]
    pub invoice_email_opt_in: Option<bool>,
    #[doc = "The list of email addresses to receive invoices by email for the billing profile."]
    #[serde(
        rename = "invoiceRecipients",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub invoice_recipients: Vec<String>,
    #[doc = "The default purchase order number that will appear on the invoices generated for the billing profile."]
    #[serde(rename = "poNumber", default, skip_serializing_if = "Option::is_none")]
    pub po_number: Option<String>,
    #[doc = "The default address where the products are shipped, or the services are being used. If a ship to is not specified for a product or a subscription, then this address will be used."]
    #[serde(rename = "shipTo", default, skip_serializing_if = "Option::is_none")]
    pub ship_to: Option<serde_json::Value>,
    #[doc = "The address of the individual or organization that is responsible for the billing account."]
    #[serde(rename = "soldTo", default, skip_serializing_if = "Option::is_none")]
    pub sold_to: Option<serde_json::Value>,
    #[doc = "The billing profile spending limit."]
    #[serde(rename = "spendingLimit", default, skip_serializing_if = "Option::is_none")]
    pub spending_limit: Option<billing_profile_properties::SpendingLimit>,
    #[doc = "The details of billing profile spending limit."]
    #[serde(
        rename = "spendingLimitDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub spending_limit_details: Vec<SpendingLimitDetails>,
    #[doc = "The status of the billing profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<billing_profile_properties::Status>,
    #[doc = "Reason for the specified billing profile status."]
    #[serde(rename = "statusReasonCode", default, skip_serializing_if = "Option::is_none")]
    pub status_reason_code: Option<billing_profile_properties::StatusReasonCode>,
    #[doc = "The system generated unique identifier for a billing profile."]
    #[serde(rename = "systemId", default, skip_serializing_if = "Option::is_none")]
    pub system_id: Option<String>,
    #[doc = "Dictionary of metadata associated with the resource. Maximum key/value length supported of 256 characters. Keys/value should not empty value nor null. Keys can not contain < > % & \\ ? /"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Identifies the cloud environments that are associated with a billing profile. This is a system managed optional field and gets updated as the billing profile gets associated with accounts in various clouds."]
    #[serde(
        rename = "targetClouds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub target_clouds: Vec<String>,
    #[doc = "The current payment term of the billing profile."]
    #[serde(rename = "currentPaymentTerm", default, skip_serializing_if = "Option::is_none")]
    pub current_payment_term: Option<serde_json::Value>,
    #[doc = "The other payment terms of the billing profile."]
    #[serde(
        rename = "otherPaymentTerms",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub other_payment_terms: Vec<PaymentTerm>,
}
impl BillingProfileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_profile_properties {
    use super::*;
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Canceled,
        Failed,
        New,
        Pending,
        Provisioning,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::New => serializer.serialize_unit_variant("ProvisioningState", 3u32, "New"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Identifies the billing relationship represented by the billing profile. The billing relationship may be between Microsoft, the customer, and/or a third-party."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingRelationshipType")]
    pub enum BillingRelationshipType {
        Other,
        Direct,
        IndirectCustomer,
        IndirectPartner,
        #[serde(rename = "CSPPartner")]
        CspPartner,
        #[serde(rename = "CSPCustomer")]
        CspCustomer,
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
                Self::Other => serializer.serialize_unit_variant("BillingRelationshipType", 0u32, "Other"),
                Self::Direct => serializer.serialize_unit_variant("BillingRelationshipType", 1u32, "Direct"),
                Self::IndirectCustomer => serializer.serialize_unit_variant("BillingRelationshipType", 2u32, "IndirectCustomer"),
                Self::IndirectPartner => serializer.serialize_unit_variant("BillingRelationshipType", 3u32, "IndirectPartner"),
                Self::CspPartner => serializer.serialize_unit_variant("BillingRelationshipType", 4u32, "CSPPartner"),
                Self::CspCustomer => serializer.serialize_unit_variant("BillingRelationshipType", 5u32, "CSPCustomer"),
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
    #[doc = "The status of the billing profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Other,
        Active,
        Disabled,
        Warned,
        Deleted,
        UnderReview,
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
                Self::Other => serializer.serialize_unit_variant("Status", 0u32, "Other"),
                Self::Active => serializer.serialize_unit_variant("Status", 1u32, "Active"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 2u32, "Disabled"),
                Self::Warned => serializer.serialize_unit_variant("Status", 3u32, "Warned"),
                Self::Deleted => serializer.serialize_unit_variant("Status", 4u32, "Deleted"),
                Self::UnderReview => serializer.serialize_unit_variant("Status", 5u32, "UnderReview"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Reason for the specified billing profile status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StatusReasonCode")]
    pub enum StatusReasonCode {
        Other,
        PastDue,
        UnusualActivity,
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
                Self::Other => serializer.serialize_unit_variant("StatusReasonCode", 0u32, "Other"),
                Self::PastDue => serializer.serialize_unit_variant("StatusReasonCode", 1u32, "PastDue"),
                Self::UnusualActivity => serializer.serialize_unit_variant("StatusReasonCode", 2u32, "UnusualActivity"),
                Self::SpendingLimitReached => serializer.serialize_unit_variant("StatusReasonCode", 3u32, "SpendingLimitReached"),
                Self::SpendingLimitExpired => serializer.serialize_unit_variant("StatusReasonCode", 4u32, "SpendingLimitExpired"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A billing property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingProperty {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "A billing property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingPropertyProperties>,
}
impl BillingProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A billing property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingPropertyProperties {
    #[doc = "The type of agreement."]
    #[serde(rename = "billingAccountAgreementType", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_agreement_type: Option<billing_property_properties::BillingAccountAgreementType>,
    #[doc = "The name of the billing account."]
    #[serde(rename = "billingAccountDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_display_name: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies a billing account."]
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<String>,
    #[doc = "Notification email address for legacy account. Available for agreement type Microsoft Online Services Program."]
    #[serde(rename = "accountAdminNotificationEmailAddress", default, skip_serializing_if = "Option::is_none")]
    pub account_admin_notification_email_address: Option<String>,
    #[doc = "The country of the individual or organization that is responsible for the billing account."]
    #[serde(rename = "billingAccountSoldToCountry", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_sold_to_country: Option<String>,
    #[doc = "The current status of the billing account."]
    #[serde(rename = "billingAccountStatus", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_status: Option<billing_property_properties::BillingAccountStatus>,
    #[doc = "Reason for the specified billing account status."]
    #[serde(rename = "billingAccountStatusReasonCode", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_status_reason_code: Option<billing_property_properties::BillingAccountStatusReasonCode>,
    #[doc = "The type of customer."]
    #[serde(rename = "billingAccountType", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_type: Option<billing_property_properties::BillingAccountType>,
    #[doc = "The tier of the account."]
    #[serde(rename = "billingAccountSubType", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_sub_type: Option<billing_property_properties::BillingAccountSubType>,
    #[doc = "The billing currency for the subscription. Available for billing accounts with agreement type Enterprise Agreement"]
    #[serde(rename = "billingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency: Option<String>,
    #[doc = "The name of the billing profile."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies a billing profile."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The billing profile spending limit."]
    #[serde(rename = "billingProfileSpendingLimit", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_spending_limit: Option<billing_property_properties::BillingProfileSpendingLimit>,
    #[doc = "The details of billing profile spending limit."]
    #[serde(
        rename = "billingProfileSpendingLimitDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub billing_profile_spending_limit_details: Vec<SpendingLimitDetails>,
    #[doc = "The status of the billing profile."]
    #[serde(rename = "billingProfileStatus", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_status: Option<billing_property_properties::BillingProfileStatus>,
    #[doc = "Reason for the specified billing profile status."]
    #[serde(rename = "billingProfileStatusReasonCode", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_status_reason_code: Option<billing_property_properties::BillingProfileStatusReasonCode>,
    #[doc = "The payment method family of the primary payment method for the billing profile."]
    #[serde(rename = "billingProfilePaymentMethodFamily", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_payment_method_family: Option<billing_property_properties::BillingProfilePaymentMethodFamily>,
    #[doc = "The payment method type of the primary payment method for the billing profile."]
    #[serde(rename = "billingProfilePaymentMethodType", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_payment_method_type: Option<String>,
    #[doc = "The Azure AD tenant ID of the billing account for the subscription."]
    #[serde(rename = "billingTenantId", default, skip_serializing_if = "Option::is_none")]
    pub billing_tenant_id: Option<String>,
    #[doc = "The cost center applied to the subscription. Available for agreement type Microsoft Customer Agreement and Microsoft Partner Agreement. This property can be updated via patch."]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "The name of the customer."]
    #[serde(rename = "customerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub customer_display_name: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies a customer."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "Identifies the status of an customer. This is an upcoming property that will be populated in the future."]
    #[serde(rename = "customerStatus", default, skip_serializing_if = "Option::is_none")]
    pub customer_status: Option<billing_property_properties::CustomerStatus>,
    #[doc = "The name of the invoice section."]
    #[serde(rename = "invoiceSectionDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_display_name: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies an invoice section."]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "Identifies the status of an invoice section."]
    #[serde(rename = "invoiceSectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_status: Option<billing_property_properties::InvoiceSectionStatus>,
    #[doc = "Reason for the specified invoice section status."]
    #[serde(rename = "invoiceSectionStatusReasonCode", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_status_reason_code: Option<billing_property_properties::InvoiceSectionStatusReasonCode>,
    #[doc = "Specifies if the billing account for the subscription is transitioned from a Microsoft Online Service Program to a Microsoft Customer Agreement (MCA) account. Will be present and value will be true if its a transitioned billing account."]
    #[serde(rename = "isTransitionedBillingAccount", default, skip_serializing_if = "Option::is_none")]
    pub is_transitioned_billing_account: Option<bool>,
    #[doc = "The sku description."]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[doc = "The ID that uniquely identifies a sku."]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "The subscription status."]
    #[serde(rename = "subscriptionBillingStatus", default, skip_serializing_if = "Option::is_none")]
    pub subscription_billing_status: Option<billing_property_properties::SubscriptionBillingStatus>,
    #[doc = "The reason codes for the subscription status."]
    #[serde(
        rename = "subscriptionBillingStatusDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subscription_billing_status_details: Vec<BillingSubscriptionStatusDetails>,
    #[doc = "The type of billing subscription."]
    #[serde(rename = "subscriptionBillingType", default, skip_serializing_if = "Option::is_none")]
    pub subscription_billing_type: Option<billing_property_properties::SubscriptionBillingType>,
    #[doc = "The address of the individual or organization where service subscription is being used. Available for agreement type Microsoft Online Services Program. This property can be updated via patch."]
    #[serde(rename = "subscriptionServiceUsageAddress", default, skip_serializing_if = "Option::is_none")]
    pub subscription_service_usage_address: Option<serde_json::Value>,
    #[doc = "The Azure workload type of the subscription."]
    #[serde(rename = "subscriptionWorkloadType", default, skip_serializing_if = "Option::is_none")]
    pub subscription_workload_type: Option<billing_property_properties::SubscriptionWorkloadType>,
    #[doc = "The enrollment details for the subscription. Available for billing accounts with agreement type Enterprise Agreement."]
    #[serde(rename = "enrollmentDetails", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_details: Option<serde_json::Value>,
    #[doc = "Indicates whether user is the account admin."]
    #[serde(rename = "isAccountAdmin", default, skip_serializing_if = "Option::is_none")]
    pub is_account_admin: Option<bool>,
    #[doc = "The ID that uniquely identifies a product."]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[doc = "The ID that uniquely identifies a product."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
}
impl BillingPropertyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_property_properties {
    use super::*;
    #[doc = "The type of agreement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingAccountAgreementType")]
    pub enum BillingAccountAgreementType {
        Other,
        MicrosoftCustomerAgreement,
        EnterpriseAgreement,
        MicrosoftOnlineServicesProgram,
        MicrosoftPartnerAgreement,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingAccountAgreementType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingAccountAgreementType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingAccountAgreementType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("BillingAccountAgreementType", 0u32, "Other"),
                Self::MicrosoftCustomerAgreement => {
                    serializer.serialize_unit_variant("BillingAccountAgreementType", 1u32, "MicrosoftCustomerAgreement")
                }
                Self::EnterpriseAgreement => serializer.serialize_unit_variant("BillingAccountAgreementType", 2u32, "EnterpriseAgreement"),
                Self::MicrosoftOnlineServicesProgram => {
                    serializer.serialize_unit_variant("BillingAccountAgreementType", 3u32, "MicrosoftOnlineServicesProgram")
                }
                Self::MicrosoftPartnerAgreement => {
                    serializer.serialize_unit_variant("BillingAccountAgreementType", 4u32, "MicrosoftPartnerAgreement")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current status of the billing account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingAccountStatus")]
    pub enum BillingAccountStatus {
        Other,
        Active,
        UnderReview,
        Disabled,
        Deleted,
        Extended,
        Pending,
        New,
        Expired,
        Terminated,
        Transferred,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingAccountStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingAccountStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingAccountStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("BillingAccountStatus", 0u32, "Other"),
                Self::Active => serializer.serialize_unit_variant("BillingAccountStatus", 1u32, "Active"),
                Self::UnderReview => serializer.serialize_unit_variant("BillingAccountStatus", 2u32, "UnderReview"),
                Self::Disabled => serializer.serialize_unit_variant("BillingAccountStatus", 3u32, "Disabled"),
                Self::Deleted => serializer.serialize_unit_variant("BillingAccountStatus", 4u32, "Deleted"),
                Self::Extended => serializer.serialize_unit_variant("BillingAccountStatus", 5u32, "Extended"),
                Self::Pending => serializer.serialize_unit_variant("BillingAccountStatus", 6u32, "Pending"),
                Self::New => serializer.serialize_unit_variant("BillingAccountStatus", 7u32, "New"),
                Self::Expired => serializer.serialize_unit_variant("BillingAccountStatus", 8u32, "Expired"),
                Self::Terminated => serializer.serialize_unit_variant("BillingAccountStatus", 9u32, "Terminated"),
                Self::Transferred => serializer.serialize_unit_variant("BillingAccountStatus", 10u32, "Transferred"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Reason for the specified billing account status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingAccountStatusReasonCode")]
    pub enum BillingAccountStatusReasonCode {
        Other,
        UnusualActivity,
        ManuallyTerminated,
        Expired,
        Transferred,
        TerminateProcessing,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingAccountStatusReasonCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingAccountStatusReasonCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingAccountStatusReasonCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("BillingAccountStatusReasonCode", 0u32, "Other"),
                Self::UnusualActivity => serializer.serialize_unit_variant("BillingAccountStatusReasonCode", 1u32, "UnusualActivity"),
                Self::ManuallyTerminated => serializer.serialize_unit_variant("BillingAccountStatusReasonCode", 2u32, "ManuallyTerminated"),
                Self::Expired => serializer.serialize_unit_variant("BillingAccountStatusReasonCode", 3u32, "Expired"),
                Self::Transferred => serializer.serialize_unit_variant("BillingAccountStatusReasonCode", 4u32, "Transferred"),
                Self::TerminateProcessing => {
                    serializer.serialize_unit_variant("BillingAccountStatusReasonCode", 5u32, "TerminateProcessing")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of customer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingAccountType")]
    pub enum BillingAccountType {
        Other,
        Enterprise,
        Individual,
        Partner,
        Reseller,
        ClassicPartner,
        Internal,
        Tenant,
        Business,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingAccountType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingAccountType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingAccountType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("BillingAccountType", 0u32, "Other"),
                Self::Enterprise => serializer.serialize_unit_variant("BillingAccountType", 1u32, "Enterprise"),
                Self::Individual => serializer.serialize_unit_variant("BillingAccountType", 2u32, "Individual"),
                Self::Partner => serializer.serialize_unit_variant("BillingAccountType", 3u32, "Partner"),
                Self::Reseller => serializer.serialize_unit_variant("BillingAccountType", 4u32, "Reseller"),
                Self::ClassicPartner => serializer.serialize_unit_variant("BillingAccountType", 5u32, "ClassicPartner"),
                Self::Internal => serializer.serialize_unit_variant("BillingAccountType", 6u32, "Internal"),
                Self::Tenant => serializer.serialize_unit_variant("BillingAccountType", 7u32, "Tenant"),
                Self::Business => serializer.serialize_unit_variant("BillingAccountType", 8u32, "Business"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The tier of the account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingAccountSubType")]
    pub enum BillingAccountSubType {
        Other,
        None,
        Individual,
        Professional,
        Enterprise,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingAccountSubType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingAccountSubType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingAccountSubType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("BillingAccountSubType", 0u32, "Other"),
                Self::None => serializer.serialize_unit_variant("BillingAccountSubType", 1u32, "None"),
                Self::Individual => serializer.serialize_unit_variant("BillingAccountSubType", 2u32, "Individual"),
                Self::Professional => serializer.serialize_unit_variant("BillingAccountSubType", 3u32, "Professional"),
                Self::Enterprise => serializer.serialize_unit_variant("BillingAccountSubType", 4u32, "Enterprise"),
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
    #[doc = "The status of the billing profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingProfileStatus")]
    pub enum BillingProfileStatus {
        Other,
        Active,
        Disabled,
        Warned,
        Deleted,
        UnderReview,
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
                Self::Other => serializer.serialize_unit_variant("BillingProfileStatus", 0u32, "Other"),
                Self::Active => serializer.serialize_unit_variant("BillingProfileStatus", 1u32, "Active"),
                Self::Disabled => serializer.serialize_unit_variant("BillingProfileStatus", 2u32, "Disabled"),
                Self::Warned => serializer.serialize_unit_variant("BillingProfileStatus", 3u32, "Warned"),
                Self::Deleted => serializer.serialize_unit_variant("BillingProfileStatus", 4u32, "Deleted"),
                Self::UnderReview => serializer.serialize_unit_variant("BillingProfileStatus", 5u32, "UnderReview"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Reason for the specified billing profile status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingProfileStatusReasonCode")]
    pub enum BillingProfileStatusReasonCode {
        Other,
        PastDue,
        UnusualActivity,
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
                Self::Other => serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 0u32, "Other"),
                Self::PastDue => serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 1u32, "PastDue"),
                Self::UnusualActivity => serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 2u32, "UnusualActivity"),
                Self::SpendingLimitReached => {
                    serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 3u32, "SpendingLimitReached")
                }
                Self::SpendingLimitExpired => {
                    serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 4u32, "SpendingLimitExpired")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The payment method family of the primary payment method for the billing profile."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingProfilePaymentMethodFamily")]
    pub enum BillingProfilePaymentMethodFamily {
        Other,
        None,
        CreditCard,
        Credits,
        CheckWire,
        EWallet,
        TaskOrder,
        DirectDebit,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingProfilePaymentMethodFamily {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingProfilePaymentMethodFamily {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingProfilePaymentMethodFamily {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("BillingProfilePaymentMethodFamily", 0u32, "Other"),
                Self::None => serializer.serialize_unit_variant("BillingProfilePaymentMethodFamily", 1u32, "None"),
                Self::CreditCard => serializer.serialize_unit_variant("BillingProfilePaymentMethodFamily", 2u32, "CreditCard"),
                Self::Credits => serializer.serialize_unit_variant("BillingProfilePaymentMethodFamily", 3u32, "Credits"),
                Self::CheckWire => serializer.serialize_unit_variant("BillingProfilePaymentMethodFamily", 4u32, "CheckWire"),
                Self::EWallet => serializer.serialize_unit_variant("BillingProfilePaymentMethodFamily", 5u32, "EWallet"),
                Self::TaskOrder => serializer.serialize_unit_variant("BillingProfilePaymentMethodFamily", 6u32, "TaskOrder"),
                Self::DirectDebit => serializer.serialize_unit_variant("BillingProfilePaymentMethodFamily", 7u32, "DirectDebit"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Identifies the status of an customer. This is an upcoming property that will be populated in the future."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CustomerStatus")]
    pub enum CustomerStatus {
        Other,
        Active,
        Pending,
        Disabled,
        Warned,
        Deleted,
        UnderReview,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CustomerStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CustomerStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CustomerStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("CustomerStatus", 0u32, "Other"),
                Self::Active => serializer.serialize_unit_variant("CustomerStatus", 1u32, "Active"),
                Self::Pending => serializer.serialize_unit_variant("CustomerStatus", 2u32, "Pending"),
                Self::Disabled => serializer.serialize_unit_variant("CustomerStatus", 3u32, "Disabled"),
                Self::Warned => serializer.serialize_unit_variant("CustomerStatus", 4u32, "Warned"),
                Self::Deleted => serializer.serialize_unit_variant("CustomerStatus", 5u32, "Deleted"),
                Self::UnderReview => serializer.serialize_unit_variant("CustomerStatus", 6u32, "UnderReview"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Identifies the status of an invoice section."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InvoiceSectionStatus")]
    pub enum InvoiceSectionStatus {
        Other,
        Active,
        Deleted,
        Disabled,
        UnderReview,
        Warned,
        Restricted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InvoiceSectionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InvoiceSectionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InvoiceSectionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("InvoiceSectionStatus", 0u32, "Other"),
                Self::Active => serializer.serialize_unit_variant("InvoiceSectionStatus", 1u32, "Active"),
                Self::Deleted => serializer.serialize_unit_variant("InvoiceSectionStatus", 2u32, "Deleted"),
                Self::Disabled => serializer.serialize_unit_variant("InvoiceSectionStatus", 3u32, "Disabled"),
                Self::UnderReview => serializer.serialize_unit_variant("InvoiceSectionStatus", 4u32, "UnderReview"),
                Self::Warned => serializer.serialize_unit_variant("InvoiceSectionStatus", 5u32, "Warned"),
                Self::Restricted => serializer.serialize_unit_variant("InvoiceSectionStatus", 6u32, "Restricted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Reason for the specified invoice section status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InvoiceSectionStatusReasonCode")]
    pub enum InvoiceSectionStatusReasonCode {
        Other,
        PastDue,
        UnusualActivity,
        SpendingLimitReached,
        SpendingLimitExpired,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InvoiceSectionStatusReasonCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InvoiceSectionStatusReasonCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InvoiceSectionStatusReasonCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("InvoiceSectionStatusReasonCode", 0u32, "Other"),
                Self::PastDue => serializer.serialize_unit_variant("InvoiceSectionStatusReasonCode", 1u32, "PastDue"),
                Self::UnusualActivity => serializer.serialize_unit_variant("InvoiceSectionStatusReasonCode", 2u32, "UnusualActivity"),
                Self::SpendingLimitReached => {
                    serializer.serialize_unit_variant("InvoiceSectionStatusReasonCode", 3u32, "SpendingLimitReached")
                }
                Self::SpendingLimitExpired => {
                    serializer.serialize_unit_variant("InvoiceSectionStatusReasonCode", 4u32, "SpendingLimitExpired")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The subscription status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SubscriptionBillingStatus")]
    pub enum SubscriptionBillingStatus {
        Other,
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
        Failed,
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
                Self::Other => serializer.serialize_unit_variant("SubscriptionBillingStatus", 0u32, "Other"),
                Self::Unknown => serializer.serialize_unit_variant("SubscriptionBillingStatus", 1u32, "Unknown"),
                Self::Active => serializer.serialize_unit_variant("SubscriptionBillingStatus", 2u32, "Active"),
                Self::Disabled => serializer.serialize_unit_variant("SubscriptionBillingStatus", 3u32, "Disabled"),
                Self::Deleted => serializer.serialize_unit_variant("SubscriptionBillingStatus", 4u32, "Deleted"),
                Self::Warned => serializer.serialize_unit_variant("SubscriptionBillingStatus", 5u32, "Warned"),
                Self::Expiring => serializer.serialize_unit_variant("SubscriptionBillingStatus", 6u32, "Expiring"),
                Self::Expired => serializer.serialize_unit_variant("SubscriptionBillingStatus", 7u32, "Expired"),
                Self::AutoRenew => serializer.serialize_unit_variant("SubscriptionBillingStatus", 8u32, "AutoRenew"),
                Self::Cancelled => serializer.serialize_unit_variant("SubscriptionBillingStatus", 9u32, "Cancelled"),
                Self::Suspended => serializer.serialize_unit_variant("SubscriptionBillingStatus", 10u32, "Suspended"),
                Self::Failed => serializer.serialize_unit_variant("SubscriptionBillingStatus", 11u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of billing subscription."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SubscriptionBillingType")]
    pub enum SubscriptionBillingType {
        None,
        Benefit,
        Free,
        Paid,
        PrePaid,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SubscriptionBillingType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SubscriptionBillingType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SubscriptionBillingType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("SubscriptionBillingType", 0u32, "None"),
                Self::Benefit => serializer.serialize_unit_variant("SubscriptionBillingType", 1u32, "Benefit"),
                Self::Free => serializer.serialize_unit_variant("SubscriptionBillingType", 2u32, "Free"),
                Self::Paid => serializer.serialize_unit_variant("SubscriptionBillingType", 3u32, "Paid"),
                Self::PrePaid => serializer.serialize_unit_variant("SubscriptionBillingType", 4u32, "PrePaid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The Azure workload type of the subscription."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SubscriptionWorkloadType")]
    pub enum SubscriptionWorkloadType {
        None,
        Production,
        DevTest,
        Internal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SubscriptionWorkloadType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SubscriptionWorkloadType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SubscriptionWorkloadType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("SubscriptionWorkloadType", 0u32, "None"),
                Self::Production => serializer.serialize_unit_variant("SubscriptionWorkloadType", 1u32, "Production"),
                Self::DevTest => serializer.serialize_unit_variant("SubscriptionWorkloadType", 2u32, "DevTest"),
                Self::Internal => serializer.serialize_unit_variant("SubscriptionWorkloadType", 3u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A request submitted by a user to manage billing. Users with an owner role on the scope can approve or decline these requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingRequest {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "A request submitted by a user to manage billing. Users with an owner role on the scope can approve or decline these requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingRequestProperties>,
}
impl BillingRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingRequestListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BillingRequest>,
}
impl azure_core::Continuable for BillingRequestListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BillingRequestListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A request submitted by a user to manage billing. Users with an owner role on the scope can approve or decline these requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingRequestProperties {
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<billing_request_properties::ProvisioningState>,
    #[doc = "Additional information for the billing request."]
    #[serde(rename = "additionalInformation", default, skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<serde_json::Value>,
    #[doc = "The principal of the request reviewer. Will only be set if request is approved."]
    #[serde(rename = "reviewedBy", default, skip_serializing_if = "Option::is_none")]
    pub reviewed_by: Option<serde_json::Value>,
    #[doc = "The date and time when the request was reviewed."]
    #[serde(rename = "reviewalDate", default, with = "azure_core::date::rfc3339::option")]
    pub reviewal_date: Option<::time::OffsetDateTime>,
    #[doc = "The fully qualified ID that uniquely identifies a billing account."]
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<String>,
    #[doc = "The ID that uniquely identifies a billing account."]
    #[serde(rename = "billingAccountName", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_name: Option<String>,
    #[doc = "The name of the billing account."]
    #[serde(rename = "billingAccountDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_display_name: Option<String>,
    #[doc = "The primary tenant ID of the billing account for which the billing request was submitted."]
    #[serde(rename = "billingAccountPrimaryBillingTenantId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_primary_billing_tenant_id: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies a billing profile."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The ID that uniquely identifies a billing profile."]
    #[serde(rename = "billingProfileName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_name: Option<String>,
    #[doc = "The name of the billing profile."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The principal of the entity who created the request."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<serde_json::Value>,
    #[doc = "The date and time when the request was created."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<::time::OffsetDateTime>,
    #[doc = "The date and time when the request expires."]
    #[serde(rename = "expirationDate", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<::time::OffsetDateTime>,
    #[doc = "The reason to approve or decline the request."]
    #[serde(rename = "decisionReason", default, skip_serializing_if = "Option::is_none")]
    pub decision_reason: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies an invoice section."]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "The ID that uniquely identifies an invoice section."]
    #[serde(rename = "invoiceSectionName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_name: Option<String>,
    #[doc = "The name of the invoice section."]
    #[serde(rename = "invoiceSectionDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_display_name: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies a customer."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "The ID that uniquely identifies a customer."]
    #[serde(rename = "customerName", default, skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[doc = "The name of the customer."]
    #[serde(rename = "customerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub customer_display_name: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies a billing subscription."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The ID that uniquely identifies a billing subscription."]
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
    #[doc = "The name of the billing subscription."]
    #[serde(rename = "subscriptionDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_display_name: Option<String>,
    #[doc = "Justification for submitting request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    #[doc = "The recipients of the billing request."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub recipients: Vec<Principal>,
    #[doc = "The billing scope for which the request was submitted (ex. '/providers/Microsoft.Billing/billingAccounts/{billingAccountName}/billingProfiles/{billingProfileName}')."]
    #[serde(rename = "requestScope", default, skip_serializing_if = "Option::is_none")]
    pub request_scope: Option<String>,
    #[doc = "The billing scope for which the request will be applied. This is a read only property derived by the service."]
    #[serde(rename = "billingScope", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope: Option<String>,
    #[doc = "Status of billing request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<billing_request_properties::Status>,
    #[doc = "Type of billing request."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<billing_request_properties::Type>,
    #[doc = "The principal of the entity who last updated the request."]
    #[serde(rename = "lastUpdatedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<serde_json::Value>,
    #[doc = "Date and time of last update."]
    #[serde(rename = "lastUpdatedDate", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_date: Option<::time::OffsetDateTime>,
}
impl BillingRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_request_properties {
    use super::*;
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Canceled,
        Failed,
        New,
        Pending,
        Provisioning,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::New => serializer.serialize_unit_variant("ProvisioningState", 3u32, "New"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Status of billing request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Other,
        Pending,
        Approved,
        Declined,
        Cancelled,
        Completed,
        Expired,
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
                Self::Other => serializer.serialize_unit_variant("Status", 0u32, "Other"),
                Self::Pending => serializer.serialize_unit_variant("Status", 1u32, "Pending"),
                Self::Approved => serializer.serialize_unit_variant("Status", 2u32, "Approved"),
                Self::Declined => serializer.serialize_unit_variant("Status", 3u32, "Declined"),
                Self::Cancelled => serializer.serialize_unit_variant("Status", 4u32, "Cancelled"),
                Self::Completed => serializer.serialize_unit_variant("Status", 5u32, "Completed"),
                Self::Expired => serializer.serialize_unit_variant("Status", 6u32, "Expired"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of billing request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Other,
        InvoiceAccess,
        ProvisioningAccess,
        RoleAssignment,
        UpdateBillingPolicy,
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
                Self::Other => serializer.serialize_unit_variant("Type", 0u32, "Other"),
                Self::InvoiceAccess => serializer.serialize_unit_variant("Type", 1u32, "InvoiceAccess"),
                Self::ProvisioningAccess => serializer.serialize_unit_variant("Type", 2u32, "ProvisioningAccess"),
                Self::RoleAssignment => serializer.serialize_unit_variant("Type", 3u32, "RoleAssignment"),
                Self::UpdateBillingPolicy => serializer.serialize_unit_variant("Type", 4u32, "UpdateBillingPolicy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of the billing role assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingRoleAssignment {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "The properties of the billing role assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingRoleAssignmentProperties>,
}
impl BillingRoleAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingRoleAssignmentListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BillingRoleAssignment>,
}
impl azure_core::Continuable for BillingRoleAssignmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BillingRoleAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the billing role assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingRoleAssignmentProperties {
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<billing_role_assignment_properties::ProvisioningState>,
    #[doc = "The date the role assignment was created."]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<::time::OffsetDateTime>,
    #[doc = "The tenant Id of the user who created the role assignment."]
    #[serde(rename = "createdByPrincipalTenantId", default, skip_serializing_if = "Option::is_none")]
    pub created_by_principal_tenant_id: Option<String>,
    #[doc = "The object ID of the user who created the role assignment."]
    #[serde(rename = "createdByPrincipalId", default, skip_serializing_if = "Option::is_none")]
    pub created_by_principal_id: Option<String>,
    #[doc = "The principal PUID of the user who created the role assignment."]
    #[serde(rename = "createdByPrincipalPuid", default, skip_serializing_if = "Option::is_none")]
    pub created_by_principal_puid: Option<String>,
    #[doc = "The email address of the user who created the role assignment. This is supported only for billing accounts with agreement type Enterprise Agreement."]
    #[serde(rename = "createdByUserEmailAddress", default, skip_serializing_if = "Option::is_none")]
    pub created_by_user_email_address: Option<String>,
    #[doc = "The date the role assignment was modified."]
    #[serde(rename = "modifiedOn", default, with = "azure_core::date::rfc3339::option")]
    pub modified_on: Option<::time::OffsetDateTime>,
    #[doc = "The principal PUID of the user who modified the role assignment."]
    #[serde(rename = "modifiedByPrincipalPuid", default, skip_serializing_if = "Option::is_none")]
    pub modified_by_principal_puid: Option<String>,
    #[doc = "The email address of the user who modified the role assignment. This is supported only for billing accounts with agreement type Enterprise Agreement."]
    #[serde(rename = "modifiedByUserEmailAddress", default, skip_serializing_if = "Option::is_none")]
    pub modified_by_user_email_address: Option<String>,
    #[doc = "The principal PUID of the user who modified the role assignment."]
    #[serde(rename = "modifiedByPrincipalId", default, skip_serializing_if = "Option::is_none")]
    pub modified_by_principal_id: Option<String>,
    #[doc = "The tenant Id of the user who modified the role assignment."]
    #[serde(rename = "modifiedByPrincipalTenantId", default, skip_serializing_if = "Option::is_none")]
    pub modified_by_principal_tenant_id: Option<String>,
    #[doc = "The principal PUID of the user to whom the role was assigned."]
    #[serde(rename = "principalPuid", default, skip_serializing_if = "Option::is_none")]
    pub principal_puid: Option<String>,
    #[doc = "The object id of the user to whom the role was assigned."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The principal tenant id of the user to whom the role was assigned."]
    #[serde(rename = "principalTenantId", default, skip_serializing_if = "Option::is_none")]
    pub principal_tenant_id: Option<String>,
    #[doc = "The ID of the role definition."]
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[doc = "The scope at which the role was assigned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The authentication type of the user, whether Organization or MSA, of the user to whom the role was assigned. This is supported only for billing accounts with agreement type Enterprise Agreement."]
    #[serde(rename = "userAuthenticationType", default, skip_serializing_if = "Option::is_none")]
    pub user_authentication_type: Option<String>,
    #[doc = "The email address of the user to whom the role was assigned. This is supported only for billing accounts with agreement type Enterprise Agreement."]
    #[serde(rename = "userEmailAddress", default, skip_serializing_if = "Option::is_none")]
    pub user_email_address: Option<String>,
    #[doc = "The friendly name of the tenant of the user to whom the role was assigned. This will be 'Primary Tenant' for the primary tenant of the billing account."]
    #[serde(rename = "principalTenantName", default, skip_serializing_if = "Option::is_none")]
    pub principal_tenant_name: Option<String>,
    #[doc = "The display name of the principal to whom the role was assigned."]
    #[serde(rename = "principalDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub principal_display_name: Option<String>,
    #[doc = "The type of a role Assignment."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<billing_role_assignment_properties::PrincipalType>,
    #[doc = "The ID of the billing request that was created for the role assignment. This is only applicable to cross tenant role assignments or role assignments created through the billing request."]
    #[serde(rename = "billingRequestId", default, skip_serializing_if = "Option::is_none")]
    pub billing_request_id: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies a billing account."]
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<String>,
    #[doc = "The name of the billing account."]
    #[serde(rename = "billingAccountDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_display_name: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies a billing profile."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The name of the billing profile."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies an invoice section."]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "The name of the invoice section."]
    #[serde(rename = "invoiceSectionDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_display_name: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies a customer."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "The name of the customer."]
    #[serde(rename = "customerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub customer_display_name: Option<String>,
}
impl BillingRoleAssignmentProperties {
    pub fn new(role_definition_id: String) -> Self {
        Self {
            provisioning_state: None,
            created_on: None,
            created_by_principal_tenant_id: None,
            created_by_principal_id: None,
            created_by_principal_puid: None,
            created_by_user_email_address: None,
            modified_on: None,
            modified_by_principal_puid: None,
            modified_by_user_email_address: None,
            modified_by_principal_id: None,
            modified_by_principal_tenant_id: None,
            principal_puid: None,
            principal_id: None,
            principal_tenant_id: None,
            role_definition_id,
            scope: None,
            user_authentication_type: None,
            user_email_address: None,
            principal_tenant_name: None,
            principal_display_name: None,
            principal_type: None,
            billing_request_id: None,
            billing_account_id: None,
            billing_account_display_name: None,
            billing_profile_id: None,
            billing_profile_display_name: None,
            invoice_section_id: None,
            invoice_section_display_name: None,
            customer_id: None,
            customer_display_name: None,
        }
    }
}
pub mod billing_role_assignment_properties {
    use super::*;
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Canceled,
        Failed,
        New,
        Pending,
        Provisioning,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::New => serializer.serialize_unit_variant("ProvisioningState", 3u32, "New"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of a role Assignment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        Unknown,
        None,
        User,
        Group,
        DirectoryRole,
        ServicePrincipal,
        Everyone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("PrincipalType", 0u32, "Unknown"),
                Self::None => serializer.serialize_unit_variant("PrincipalType", 1u32, "None"),
                Self::User => serializer.serialize_unit_variant("PrincipalType", 2u32, "User"),
                Self::Group => serializer.serialize_unit_variant("PrincipalType", 3u32, "Group"),
                Self::DirectoryRole => serializer.serialize_unit_variant("PrincipalType", 4u32, "DirectoryRole"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("PrincipalType", 5u32, "ServicePrincipal"),
                Self::Everyone => serializer.serialize_unit_variant("PrincipalType", 6u32, "Everyone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of a role definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingRoleDefinition {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "The properties of a role definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingRoleDefinitionProperties>,
}
impl BillingRoleDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingRoleDefinitionListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BillingRoleDefinition>,
}
impl azure_core::Continuable for BillingRoleDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BillingRoleDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a role definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingRoleDefinitionProperties {
    #[doc = "The role description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The billingPermissions the role has."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub permissions: Vec<BillingPermission>,
    #[doc = "The name of the role."]
    #[serde(rename = "roleName")]
    pub role_name: String,
}
impl BillingRoleDefinitionProperties {
    pub fn new(role_name: String) -> Self {
        Self {
            description: None,
            permissions: Vec::new(),
            role_name,
        }
    }
}
pub type BillingScopeId = String;
#[doc = "The billing properties of a subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscription {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
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
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "A billing subscription alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingSubscriptionAliasProperties>,
}
impl BillingSubscriptionAlias {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscriptionAliasListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BillingSubscriptionAlias>,
}
impl azure_core::Continuable for BillingSubscriptionAliasListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BillingSubscriptionAliasListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A billing subscription alias."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscriptionAliasProperties {
    #[serde(flatten)]
    pub billing_subscription_properties: BillingSubscriptionProperties,
    #[doc = "The ID of the billing subscription with the subscription alias."]
    #[serde(rename = "billingSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub billing_subscription_id: Option<String>,
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<billing_subscription_alias_properties::ProvisioningState>,
}
impl BillingSubscriptionAliasProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_subscription_alias_properties {
    use super::*;
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Canceled,
        Failed,
        New,
        Pending,
        Provisioning,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::New => serializer.serialize_unit_variant("ProvisioningState", 3u32, "New"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscriptionListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Total number of records."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<f64>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BillingSubscription>,
}
impl azure_core::Continuable for BillingSubscriptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BillingSubscriptionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request parameters that are provided to merge the two billing subscriptions."]
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
pub struct BillingSubscriptionPatch {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "The billing properties of a subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingSubscriptionProperties>,
}
impl BillingSubscriptionPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The billing properties of a subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscriptionProperties {
    #[doc = "Indicates whether auto renewal is turned on or off for a product."]
    #[serde(rename = "autoRenew", default, skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<billing_subscription_properties::AutoRenew>,
    #[doc = "The provisioning tenant of the subscription."]
    #[serde(rename = "beneficiaryTenantId", default, skip_serializing_if = "Option::is_none")]
    pub beneficiary_tenant_id: Option<String>,
    #[doc = "Details of the beneficiary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<Beneficiary>,
    #[doc = "The billing frequency in ISO8601 format of product in the subscription. Example: P1M, P3M, P1Y"]
    #[serde(rename = "billingFrequency", default, skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies a billing profile."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "Dictionary of billing policies associated with the subscription."]
    #[serde(rename = "billingPolicies", default, skip_serializing_if = "Option::is_none")]
    pub billing_policies: Option<serde_json::Value>,
    #[doc = "The name of the billing profile."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The ID that uniquely identifies a billing profile."]
    #[serde(rename = "billingProfileName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_name: Option<String>,
    #[doc = "The cost center applied to the subscription. This field is only available for consumption subscriptions of Microsoft Customer Agreement or Enterprise Agreement Type billing accounts."]
    #[serde(rename = "consumptionCostCenter", default, skip_serializing_if = "Option::is_none")]
    pub consumption_cost_center: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies a customer."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "The name of the customer."]
    #[serde(rename = "customerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub customer_display_name: Option<String>,
    #[doc = "The ID that uniquely identifies a customer."]
    #[serde(rename = "customerName", default, skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[doc = "The name of the billing subscription."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The enrollment Account ID associated with the subscription. This field is available only for the Enterprise Agreement Type billing accounts."]
    #[serde(rename = "enrollmentAccountId", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_account_id: Option<String>,
    #[doc = "The enrollment Account name associated with the subscription. This field is available only for the Enterprise Agreement Type billing accounts."]
    #[serde(rename = "enrollmentAccountDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_account_display_name: Option<String>,
    #[doc = "The billing properties that can be modified. Available only for the Enterprise Agreement Type."]
    #[serde(rename = "enrollmentAccountSubscriptionDetails", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_account_subscription_details: Option<EnrollmentAccountSubscriptionDetails>,
    #[doc = "The fully qualified ID that uniquely identifies an invoice section."]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "The name of the invoice section."]
    #[serde(rename = "invoiceSectionDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_display_name: Option<String>,
    #[doc = "The ID that uniquely identifies an invoice section."]
    #[serde(rename = "invoiceSectionName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_name: Option<String>,
    #[doc = "The amount."]
    #[serde(rename = "lastMonthCharges", default, skip_serializing_if = "Option::is_none")]
    pub last_month_charges: Option<Amount>,
    #[doc = "The amount."]
    #[serde(rename = "monthToDateCharges", default, skip_serializing_if = "Option::is_none")]
    pub month_to_date_charges: Option<Amount>,
    #[doc = "Billing cycle details of the product."]
    #[serde(rename = "nextBillingCycleDetails", default, skip_serializing_if = "Option::is_none")]
    pub next_billing_cycle_details: Option<NextBillingCycleDetails>,
    #[doc = "The offer ID for the subscription. This field is only available for the Microsoft Online Services Program billing accounts or billing accounts with agreement type Enterprise Agreement."]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "The category of the product for which the subscription is purchased. Possible values include: AzureSupport, Hardware, ReservationOrder, SaaS, SavingsPlanOrder, Software, UsageBased, Other."]
    #[serde(rename = "productCategory", default, skip_serializing_if = "Option::is_none")]
    pub product_category: Option<String>,
    #[doc = "Type of the product for which the subscription is purchased."]
    #[serde(rename = "productType", default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[doc = "Id of the product for which the subscription is purchased."]
    #[serde(rename = "productTypeId", default, skip_serializing_if = "Option::is_none")]
    pub product_type_id: Option<String>,
    #[doc = "Purchase date of the product in UTC time."]
    #[serde(rename = "purchaseDate", default, with = "azure_core::date::rfc3339::option")]
    pub purchase_date: Option<::time::OffsetDateTime>,
    #[doc = "The quantity of licenses or fulfillment units for the subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "Details of the reseller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reseller: Option<Reseller>,
    #[doc = "Details for the next renewal term of a subscription."]
    #[serde(rename = "renewalTermDetails", default, skip_serializing_if = "Option::is_none")]
    pub renewal_term_details: Option<RenewalTermDetails>,
    #[doc = "The SKU ID of the product for which the subscription is purchased. This field is is only available  for Microsoft Customer Agreement billing accounts."]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "The SKU description of the product for which the subscription is purchased. This field is is only available for billing accounts with agreement type Microsoft Customer Agreement and Microsoft Partner Agreement."]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[doc = "System imposed policies that regulate behavior of the subscription."]
    #[serde(rename = "systemOverrides", default, skip_serializing_if = "Option::is_none")]
    pub system_overrides: Option<SystemOverrides>,
    #[doc = "Unique identifier of the linked resource."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "The duration in ISO8601 format for which you can use the subscription. Example: P1M, P3M, P1Y"]
    #[serde(rename = "termDuration", default, skip_serializing_if = "Option::is_none")]
    pub term_duration: Option<String>,
    #[doc = "Start date of the term in UTC time."]
    #[serde(rename = "termStartDate", default, with = "azure_core::date::rfc3339::option")]
    pub term_start_date: Option<::time::OffsetDateTime>,
    #[doc = "End date of the term in UTC time."]
    #[serde(rename = "termEndDate", default, with = "azure_core::date::rfc3339::option")]
    pub term_end_date: Option<::time::OffsetDateTime>,
    #[doc = "The tenant in which the subscription is provisioned."]
    #[serde(rename = "provisioningTenantId", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_tenant_id: Option<String>,
    #[doc = "The status of the subscription. This field is not available for Enterprise Agreement billing accounts"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<billing_subscription_properties::Status>,
    #[doc = "The status of an operation on the subscription. When None, there is no ongoing operation. When LockedForUpdate, write operations will be blocked on the Billing Subscription. Other is the default value and you may need to refer to the latest API version for more details."]
    #[serde(rename = "operationStatus", default, skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<billing_subscription_properties::OperationStatus>,
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<billing_subscription_properties::ProvisioningState>,
    #[doc = "The ID of the subscription."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The suspension reason for a subscription. This field is not available for Enterprise Agreement billing accounts."]
    #[serde(
        rename = "suspensionReasons",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub suspension_reasons: Vec<String>,
    #[doc = "The suspension details for a subscription. This field is not available for Enterprise Agreement billing accounts."]
    #[serde(
        rename = "suspensionReasonDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub suspension_reason_details: Vec<BillingSubscriptionStatusDetails>,
}
impl BillingSubscriptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_subscription_properties {
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
    #[doc = "The status of the subscription. This field is not available for Enterprise Agreement billing accounts"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Other,
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
        Failed,
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
                Self::Other => serializer.serialize_unit_variant("Status", 0u32, "Other"),
                Self::Unknown => serializer.serialize_unit_variant("Status", 1u32, "Unknown"),
                Self::Active => serializer.serialize_unit_variant("Status", 2u32, "Active"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 3u32, "Disabled"),
                Self::Deleted => serializer.serialize_unit_variant("Status", 4u32, "Deleted"),
                Self::Warned => serializer.serialize_unit_variant("Status", 5u32, "Warned"),
                Self::Expiring => serializer.serialize_unit_variant("Status", 6u32, "Expiring"),
                Self::Expired => serializer.serialize_unit_variant("Status", 7u32, "Expired"),
                Self::AutoRenew => serializer.serialize_unit_variant("Status", 8u32, "AutoRenew"),
                Self::Cancelled => serializer.serialize_unit_variant("Status", 9u32, "Cancelled"),
                Self::Suspended => serializer.serialize_unit_variant("Status", 10u32, "Suspended"),
                Self::Failed => serializer.serialize_unit_variant("Status", 11u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of an operation on the subscription. When None, there is no ongoing operation. When LockedForUpdate, write operations will be blocked on the Billing Subscription. Other is the default value and you may need to refer to the latest API version for more details."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OperationStatus")]
    pub enum OperationStatus {
        Other,
        None,
        LockedForUpdate,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OperationStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OperationStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OperationStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("OperationStatus", 0u32, "Other"),
                Self::None => serializer.serialize_unit_variant("OperationStatus", 1u32, "None"),
                Self::LockedForUpdate => serializer.serialize_unit_variant("OperationStatus", 2u32, "LockedForUpdate"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Canceled,
        Failed,
        New,
        Pending,
        Provisioning,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::New => serializer.serialize_unit_variant("ProvisioningState", 3u32, "New"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Request parameters that are provided to split the billing subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscriptionSplitRequest {
    #[doc = "The ID of the target product to which the subscription needs to be split into. This value is not same as the value returned in Get API call and can be retrieved from Catalog API to know the product id to split into."]
    #[serde(rename = "targetProductTypeId", default, skip_serializing_if = "Option::is_none")]
    pub target_product_type_id: Option<String>,
    #[doc = "The ID of the target product to which the subscription needs to be split into. This value is not same as the value returned in Get API call and can be retrieved from Catalog API to know the sku id to split into."]
    #[serde(rename = "targetSkuId", default, skip_serializing_if = "Option::is_none")]
    pub target_sku_id: Option<String>,
    #[doc = "The quantity of the target product to which the subscription needs to be split into."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[doc = "The term duration of the target in ISO8601 format product to which the subscription needs to be split into. Example: P1M, P1Y"]
    #[serde(rename = "termDuration", default, skip_serializing_if = "Option::is_none")]
    pub term_duration: Option<String>,
    #[doc = "The billing frequency of the target subscription in the ISO8601 format. Example: P1M, P3M, P1Y\""]
    #[serde(rename = "billingFrequency", default, skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<String>,
}
impl BillingSubscriptionSplitRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The suspension details for a subscription. This field is not available for Enterprise Agreement billing accounts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingSubscriptionStatusDetails {
    #[doc = "The suspension effective date for a subscription. This field is not available for Enterprise Agreement billing accounts."]
    #[serde(rename = "effectiveDate", default, with = "azure_core::date::rfc3339::option")]
    pub effective_date: Option<::time::OffsetDateTime>,
    #[doc = "The suspension reason for a subscription. This field is not available for Enterprise Agreement billing accounts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<billing_subscription_status_details::Reason>,
}
impl BillingSubscriptionStatusDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod billing_subscription_status_details {
    use super::*;
    #[doc = "The suspension reason for a subscription. This field is not available for Enterprise Agreement billing accounts."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        None,
        Cancelled,
        PastDue,
        SuspiciousActivity,
        Other,
        Transferred,
        PolicyViolation,
        SpendingLimitReached,
        Expired,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Reason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Reason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Reason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Reason", 0u32, "None"),
                Self::Cancelled => serializer.serialize_unit_variant("Reason", 1u32, "Cancelled"),
                Self::PastDue => serializer.serialize_unit_variant("Reason", 2u32, "PastDue"),
                Self::SuspiciousActivity => serializer.serialize_unit_variant("Reason", 3u32, "SuspiciousActivity"),
                Self::Other => serializer.serialize_unit_variant("Reason", 4u32, "Other"),
                Self::Transferred => serializer.serialize_unit_variant("Reason", 5u32, "Transferred"),
                Self::PolicyViolation => serializer.serialize_unit_variant("Reason", 6u32, "PolicyViolation"),
                Self::SpendingLimitReached => serializer.serialize_unit_variant("Reason", 7u32, "SpendingLimitReached"),
                Self::Expired => serializer.serialize_unit_variant("Reason", 8u32, "Expired"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Request parameters for cancel customer subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CancelSubscriptionRequest {
    #[doc = "Cancellation reason."]
    #[serde(rename = "cancellationReason")]
    pub cancellation_reason: cancel_subscription_request::CancellationReason,
    #[doc = "The fully qualified ID that uniquely identifies a customer."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
}
impl CancelSubscriptionRequest {
    pub fn new(cancellation_reason: cancel_subscription_request::CancellationReason) -> Self {
        Self {
            cancellation_reason,
            customer_id: None,
        }
    }
}
pub mod cancel_subscription_request {
    use super::*;
    #[doc = "Cancellation reason."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CancellationReason")]
    pub enum CancellationReason {
        Other,
        Compromise,
        Dispute,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CancellationReason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CancellationReason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CancellationReason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("CancellationReason", 0u32, "Other"),
                Self::Compromise => serializer.serialize_unit_variant("CancellationReason", 1u32, "Compromise"),
                Self::Dispute => serializer.serialize_unit_variant("CancellationReason", 2u32, "Dispute"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Request to check access."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckAccessRequest {
    #[doc = "List of actions passed in the request body against which the permissions will be checked."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<String>,
}
impl CheckAccessRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a check access response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckAccessResponse {
    #[doc = "Access Decision, specifies access is allowed or not."]
    #[serde(rename = "accessDecision", default, skip_serializing_if = "Option::is_none")]
    pub access_decision: Option<check_access_response::AccessDecision>,
    #[doc = "Gets or sets an action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}
impl CheckAccessResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_access_response {
    use super::*;
    #[doc = "Access Decision, specifies access is allowed or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccessDecision")]
    pub enum AccessDecision {
        Other,
        Allowed,
        NotAllowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccessDecision {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccessDecision {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccessDecision {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("AccessDecision", 0u32, "Other"),
                Self::Allowed => serializer.serialize_unit_variant("AccessDecision", 1u32, "Allowed"),
                Self::NotAllowed => serializer.serialize_unit_variant("AccessDecision", 2u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Commitment towards the benefit."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Commitment {
    #[serde(flatten)]
    pub price: Price,
    #[doc = "Commitment grain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grain: Option<commitment::Grain>,
}
impl Commitment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod commitment {
    use super::*;
    #[doc = "Commitment grain."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Grain")]
    pub enum Grain {
        Hourly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Grain {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Grain {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Grain {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Hourly => serializer.serialize_unit_variant("Grain", 0u32, "Hourly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A partner's customer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Customer {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "A partner's customer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomerProperties>,
}
impl Customer {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type CustomerId = String;
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Customer>,
}
impl azure_core::Continuable for CustomerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CustomerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A policy at customer scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerPolicy {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "A policy at customer scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomerPolicyProperties>,
}
impl CustomerPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A policy at customer scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerPolicyProperties {
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<customer_policy_properties::ProvisioningState>,
    #[doc = "The policy that controls whether the users in customer's organization can view charges at pay-as-you-go prices."]
    #[serde(rename = "viewCharges")]
    pub view_charges: customer_policy_properties::ViewCharges,
    #[doc = "List of all policies defined at the billing scope."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policies: Vec<PolicySummary>,
}
impl CustomerPolicyProperties {
    pub fn new(view_charges: customer_policy_properties::ViewCharges) -> Self {
        Self {
            provisioning_state: None,
            view_charges,
            policies: Vec::new(),
        }
    }
}
pub mod customer_policy_properties {
    use super::*;
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Canceled,
        Failed,
        New,
        Pending,
        Provisioning,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::New => serializer.serialize_unit_variant("ProvisioningState", 3u32, "New"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The policy that controls whether the users in customer's organization can view charges at pay-as-you-go prices."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ViewCharges")]
    pub enum ViewCharges {
        Other,
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
                Self::Other => serializer.serialize_unit_variant("ViewCharges", 0u32, "Other"),
                Self::Allowed => serializer.serialize_unit_variant("ViewCharges", 1u32, "Allowed"),
                Self::NotAllowed => serializer.serialize_unit_variant("ViewCharges", 2u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A partner's customer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerProperties {
    #[doc = "The name of the billing profile."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies a billing profile."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The name of the customer."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The system generated unique identifier for a customer."]
    #[serde(rename = "systemId", default, skip_serializing_if = "Option::is_none")]
    pub system_id: Option<String>,
    #[doc = "Identifies the status of an customer. This is an upcoming property that will be populated in the future."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<customer_properties::Status>,
    #[doc = "Azure plans enabled for the customer."]
    #[serde(
        rename = "enabledAzurePlans",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enabled_azure_plans: Vec<AzurePlan>,
    #[doc = "The list of resellers for which an Azure plan is enabled for the customer."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resellers: Vec<Reseller>,
    #[doc = "Dictionary of metadata associated with the resource. Maximum key/value length supported of 256 characters. Keys/value should not empty value nor null. Keys can not contain < > % & \\ ? /"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CustomerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod customer_properties {
    use super::*;
    #[doc = "Identifies the status of an customer. This is an upcoming property that will be populated in the future."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Other,
        Active,
        Pending,
        Disabled,
        Warned,
        Deleted,
        UnderReview,
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
                Self::Other => serializer.serialize_unit_variant("Status", 0u32, "Other"),
                Self::Active => serializer.serialize_unit_variant("Status", 1u32, "Active"),
                Self::Pending => serializer.serialize_unit_variant("Status", 2u32, "Pending"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 3u32, "Disabled"),
                Self::Warned => serializer.serialize_unit_variant("Status", 4u32, "Warned"),
                Self::Deleted => serializer.serialize_unit_variant("Status", 5u32, "Deleted"),
                Self::UnderReview => serializer.serialize_unit_variant("Status", 6u32, "UnderReview"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Validation details of delete billing profile eligibility."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteBillingProfileEligibilityDetail {
    #[doc = "Code of the delete invoice section eligibility response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<delete_billing_profile_eligibility_detail::Code>,
    #[doc = "Validation message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl DeleteBillingProfileEligibilityDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod delete_billing_profile_eligibility_detail {
    use super::*;
    #[doc = "Code of the delete invoice section eligibility response."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Code")]
    pub enum Code {
        None,
        ActiveCredits,
        ActiveCreditCard,
        LastBillingProfile,
        NotSupported,
        OutstandingCharges,
        PendingCharges,
        ReservedInstances,
        ActiveBillingSubscriptions,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Code {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Code {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Code {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Code", 0u32, "None"),
                Self::ActiveCredits => serializer.serialize_unit_variant("Code", 1u32, "ActiveCredits"),
                Self::ActiveCreditCard => serializer.serialize_unit_variant("Code", 2u32, "ActiveCreditCard"),
                Self::LastBillingProfile => serializer.serialize_unit_variant("Code", 3u32, "LastBillingProfile"),
                Self::NotSupported => serializer.serialize_unit_variant("Code", 4u32, "NotSupported"),
                Self::OutstandingCharges => serializer.serialize_unit_variant("Code", 5u32, "OutstandingCharges"),
                Self::PendingCharges => serializer.serialize_unit_variant("Code", 6u32, "PendingCharges"),
                Self::ReservedInstances => serializer.serialize_unit_variant("Code", 7u32, "ReservedInstances"),
                Self::ActiveBillingSubscriptions => serializer.serialize_unit_variant("Code", 8u32, "ActiveBillingSubscriptions"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Eligibility to delete a billing profile result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteBillingProfileEligibilityResult {
    #[doc = "Status describing if billing profile is eligible to be deleted."]
    #[serde(rename = "eligibilityStatus", default, skip_serializing_if = "Option::is_none")]
    pub eligibility_status: Option<delete_billing_profile_eligibility_result::EligibilityStatus>,
    #[doc = "Validation details of delete billing profile eligibility."]
    #[serde(
        rename = "eligibilityDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub eligibility_details: Vec<DeleteBillingProfileEligibilityDetail>,
}
impl DeleteBillingProfileEligibilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod delete_billing_profile_eligibility_result {
    use super::*;
    #[doc = "Status describing if billing profile is eligible to be deleted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EligibilityStatus")]
    pub enum EligibilityStatus {
        Allowed,
        NotAllowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EligibilityStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EligibilityStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EligibilityStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allowed => serializer.serialize_unit_variant("EligibilityStatus", 0u32, "Allowed"),
                Self::NotAllowed => serializer.serialize_unit_variant("EligibilityStatus", 1u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The details of delete invoice section eligibility result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteInvoiceSectionEligibilityDetail {
    #[doc = "Code for the delete invoice section validation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<delete_invoice_section_eligibility_detail::Code>,
    #[doc = "Validation message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl DeleteInvoiceSectionEligibilityDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod delete_invoice_section_eligibility_detail {
    use super::*;
    #[doc = "Code for the delete invoice section validation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Code")]
    pub enum Code {
        Other,
        LastInvoiceSection,
        ActiveAzurePlans,
        ReservedInstances,
        ActiveBillingSubscriptions,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Code {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Code {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Code {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("Code", 0u32, "Other"),
                Self::LastInvoiceSection => serializer.serialize_unit_variant("Code", 1u32, "LastInvoiceSection"),
                Self::ActiveAzurePlans => serializer.serialize_unit_variant("Code", 2u32, "ActiveAzurePlans"),
                Self::ReservedInstances => serializer.serialize_unit_variant("Code", 3u32, "ReservedInstances"),
                Self::ActiveBillingSubscriptions => serializer.serialize_unit_variant("Code", 4u32, "ActiveBillingSubscriptions"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Eligibility to delete an invoice section result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteInvoiceSectionEligibilityResult {
    #[doc = "Status describing if invoice section is eligible to be deleted."]
    #[serde(rename = "eligibilityStatus", default, skip_serializing_if = "Option::is_none")]
    pub eligibility_status: Option<delete_invoice_section_eligibility_result::EligibilityStatus>,
    #[doc = "A list of delete invoice section eligibility result details."]
    #[serde(
        rename = "eligibilityDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub eligibility_details: Vec<DeleteInvoiceSectionEligibilityDetail>,
}
impl DeleteInvoiceSectionEligibilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod delete_invoice_section_eligibility_result {
    use super::*;
    #[doc = "Status describing if invoice section is eligible to be deleted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EligibilityStatus")]
    pub enum EligibilityStatus {
        Allowed,
        NotAllowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EligibilityStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EligibilityStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EligibilityStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allowed => serializer.serialize_unit_variant("EligibilityStatus", 0u32, "Allowed"),
                Self::NotAllowed => serializer.serialize_unit_variant("EligibilityStatus", 1u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Optional grouping of enrollment accounts to segment costs into logical groupings and set budgets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Department {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "Optional grouping of enrollment accounts to segment costs into logical groupings and set budgets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DepartmentProperties>,
}
impl Department {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DepartmentListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Department>,
}
impl azure_core::Continuable for DepartmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DepartmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Optional grouping of enrollment accounts to segment costs into logical groupings and set budgets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DepartmentProperties {
    #[doc = "The cost center associated with the department."]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "The name of the department."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The ID that uniquely identifies the department."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The status of the department."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl DepartmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Detailed transfer status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DetailedTransferStatus {
    #[doc = "The type of product that is transferred."]
    #[serde(rename = "productType", default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,
    #[doc = "The ID of the product that is transferred."]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[doc = "The name of the product that is transferred."]
    #[serde(rename = "productName", default, skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[doc = "The SKU of the product that is transferred."]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[doc = "The status of a transfer."]
    #[serde(rename = "transferStatus", default, skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<ProductTransferStatus>,
    #[doc = "Error details for transfer execution."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<TransferError>,
}
impl DetailedTransferStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type DisplayName = String;
#[doc = "A list of download details for individual documents."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DocumentDownloadRequest {
    #[doc = "The ID that uniquely identifies an invoice document. This ID may be an identifier for an invoice PDF, a credit note, or a tax receipt. If omitted, the most recent invoice PDF for the invoice will be returned."]
    #[serde(rename = "documentName", default, skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    #[doc = "The ID that uniquely identifies an invoice."]
    #[serde(rename = "invoiceName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_name: Option<String>,
}
impl DocumentDownloadRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A secure URL that can be used to download a an entity until the URL expires."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DocumentDownloadResult {
    #[doc = "The time in UTC when the download URL will expire."]
    #[serde(rename = "expiryTime", default, skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
    #[doc = "The URL to the PDF or .zip file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl DocumentDownloadResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the products that can be transferred."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EligibleProductType")]
pub enum EligibleProductType {
    DevTestAzureSubscription,
    StandardAzureSubscription,
    AzureReservation,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EligibleProductType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EligibleProductType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EligibleProductType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DevTestAzureSubscription => serializer.serialize_unit_variant("EligibleProductType", 0u32, "DevTestAzureSubscription"),
            Self::StandardAzureSubscription => serializer.serialize_unit_variant("EligibleProductType", 1u32, "StandardAzureSubscription"),
            Self::AzureReservation => serializer.serialize_unit_variant("EligibleProductType", 2u32, "AzureReservation"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "It is an organizational hierarchy within a billing account to administer and manage azure costs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrollmentAccount {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "It is an organizational hierarchy within a billing account to administer and manage azure costs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnrollmentAccountProperties>,
}
impl EnrollmentAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrollmentAccountListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EnrollmentAccount>,
}
impl azure_core::Continuable for EnrollmentAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EnrollmentAccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "It is an organizational hierarchy within a billing account to administer and manage azure costs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrollmentAccountProperties {
    #[doc = "The cost center associated with the enrollment account."]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "The name of the enrollment account."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The name of the department under which the enrollment account exists."]
    #[serde(rename = "departmentDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub department_display_name: Option<String>,
    #[doc = "The ID that uniquely identifies the department."]
    #[serde(rename = "departmentId", default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[doc = "Boolean flag which enables subscribers to run development and testing workloads on Azure at special Dev/Test rates."]
    #[serde(rename = "isDevTestEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_dev_test_enabled: Option<bool>,
    #[doc = "The owner of the enrollment account."]
    #[serde(rename = "accountOwner", default, skip_serializing_if = "Option::is_none")]
    pub account_owner: Option<String>,
    #[doc = "The authorization type of the enrollment account."]
    #[serde(rename = "authType", default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[doc = "The status of the enrollment account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The date from which the enrollment account became valid and functional."]
    #[serde(rename = "startDate", default, with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<::time::OffsetDateTime>,
    #[doc = "The date of expiration of the enrollment account."]
    #[serde(rename = "endDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<::time::OffsetDateTime>,
}
impl EnrollmentAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The billing properties that can be modified. Available only for the Enterprise Agreement Type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrollmentAccountSubscriptionDetails {
    #[doc = "The enrollment Account and the subscription association start date. This field is available only for the Enterprise Agreement Type."]
    #[serde(rename = "enrollmentAccountStartDate", default, with = "azure_core::date::rfc3339::option")]
    pub enrollment_account_start_date: Option<::time::OffsetDateTime>,
    #[doc = "The current enrollment account status of the subscription. This field is available only for the Enterprise Agreement Type."]
    #[serde(rename = "subscriptionEnrollmentAccountStatus", default, skip_serializing_if = "Option::is_none")]
    pub subscription_enrollment_account_status: Option<enrollment_account_subscription_details::SubscriptionEnrollmentAccountStatus>,
}
impl EnrollmentAccountSubscriptionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod enrollment_account_subscription_details {
    use super::*;
    #[doc = "The current enrollment account status of the subscription. This field is available only for the Enterprise Agreement Type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SubscriptionEnrollmentAccountStatus")]
    pub enum SubscriptionEnrollmentAccountStatus {
        Active,
        Cancelled,
        Expired,
        Deleted,
        TransferredOut,
        Transferring,
        Inactive,
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
                Self::Inactive => serializer.serialize_unit_variant("SubscriptionEnrollmentAccountStatus", 6u32, "Inactive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of an enrollment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnrollmentDetails {
    #[doc = "The start date of the enrollment."]
    #[serde(rename = "startDate", default, with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<::time::OffsetDateTime>,
    #[doc = "The end date of the enrollment."]
    #[serde(rename = "endDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<::time::OffsetDateTime>,
    #[doc = "The billing currency for the enrollment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The channel type of the enrollment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[doc = "The language for the enrollment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[doc = "The country code of the enrollment."]
    #[serde(rename = "countryCode", default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[doc = "The billing cycle for the enrollment."]
    #[serde(rename = "billingCycle", default, skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[doc = "The billing account extension opted by the company."]
    #[serde(rename = "extendedTermOption", default, skip_serializing_if = "Option::is_none")]
    pub extended_term_option: Option<enrollment_details::ExtendedTermOption>,
    #[doc = "The support level offer associated with an enrollment."]
    #[serde(rename = "supportLevel", default, skip_serializing_if = "Option::is_none")]
    pub support_level: Option<enrollment_details::SupportLevel>,
    #[doc = "The support coverage period for the enrollment."]
    #[serde(rename = "supportCoverage", default, skip_serializing_if = "Option::is_none")]
    pub support_coverage: Option<String>,
    #[doc = "The cloud of the enrollment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<String>,
    #[doc = "The purchase order number of the enrollment."]
    #[serde(rename = "poNumber", default, skip_serializing_if = "Option::is_none")]
    pub po_number: Option<String>,
    #[doc = "Markup status of enrollment, applicable only for indirect enrollments."]
    #[serde(rename = "markupStatus", default, skip_serializing_if = "Option::is_none")]
    pub markup_status: Option<enrollment_details::MarkupStatus>,
    #[doc = "The properties of an enrollment which are applicable only for indirect enrollments."]
    #[serde(rename = "indirectRelationshipInfo", default, skip_serializing_if = "Option::is_none")]
    pub indirect_relationship_info: Option<serde_json::Value>,
    #[doc = "The contact who receives invoices of the enrollment."]
    #[serde(rename = "invoiceRecipient", default, skip_serializing_if = "Option::is_none")]
    pub invoice_recipient: Option<String>,
}
impl EnrollmentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod enrollment_details {
    use super::*;
    #[doc = "The billing account extension opted by the company."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ExtendedTermOption")]
    pub enum ExtendedTermOption {
        Other,
        #[serde(rename = "Opted-In")]
        OptedIn,
        #[serde(rename = "Opted-Out")]
        OptedOut,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ExtendedTermOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ExtendedTermOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ExtendedTermOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("ExtendedTermOption", 0u32, "Other"),
                Self::OptedIn => serializer.serialize_unit_variant("ExtendedTermOption", 1u32, "Opted-In"),
                Self::OptedOut => serializer.serialize_unit_variant("ExtendedTermOption", 2u32, "Opted-Out"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The support level offer associated with an enrollment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SupportLevel")]
    pub enum SupportLevel {
        Other,
        Standard,
        #[serde(rename = "Pro-Direct")]
        ProDirect,
        Developer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SupportLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SupportLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SupportLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("SupportLevel", 0u32, "Other"),
                Self::Standard => serializer.serialize_unit_variant("SupportLevel", 1u32, "Standard"),
                Self::ProDirect => serializer.serialize_unit_variant("SupportLevel", 2u32, "Pro-Direct"),
                Self::Developer => serializer.serialize_unit_variant("SupportLevel", 3u32, "Developer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Markup status of enrollment, applicable only for indirect enrollments."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MarkupStatus")]
    pub enum MarkupStatus {
        Other,
        Disabled,
        Preview,
        Published,
        Locked,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MarkupStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MarkupStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MarkupStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("MarkupStatus", 0u32, "Other"),
                Self::Disabled => serializer.serialize_unit_variant("MarkupStatus", 1u32, "Disabled"),
                Self::Preview => serializer.serialize_unit_variant("MarkupStatus", 2u32, "Preview"),
                Self::Published => serializer.serialize_unit_variant("MarkupStatus", 3u32, "Published"),
                Self::Locked => serializer.serialize_unit_variant("MarkupStatus", 4u32, "Locked"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The policies for Enterprise Agreement enrollments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnterpriseAgreementPolicies {
    #[doc = "The state showing the enrollment auth level."]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<enterprise_agreement_policies::AuthenticationType>,
    #[doc = "The policy that controls whether account owner can view charges."]
    #[serde(rename = "accountOwnerViewCharges", default, skip_serializing_if = "Option::is_none")]
    pub account_owner_view_charges: Option<enterprise_agreement_policies::AccountOwnerViewCharges>,
    #[doc = "The policy that controls whether department admin can view charges."]
    #[serde(rename = "departmentAdminViewCharges", default, skip_serializing_if = "Option::is_none")]
    pub department_admin_view_charges: Option<enterprise_agreement_policies::DepartmentAdminViewCharges>,
}
impl EnterpriseAgreementPolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod enterprise_agreement_policies {
    use super::*;
    #[doc = "The state showing the enrollment auth level."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthenticationType")]
    pub enum AuthenticationType {
        Other,
        MicrosoftAccountOnly,
        MixedAccount,
        OrganizationalAccountCrossTenant,
        OrganizationalAccountOnly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("AuthenticationType", 0u32, "Other"),
                Self::MicrosoftAccountOnly => serializer.serialize_unit_variant("AuthenticationType", 1u32, "MicrosoftAccountOnly"),
                Self::MixedAccount => serializer.serialize_unit_variant("AuthenticationType", 2u32, "MixedAccount"),
                Self::OrganizationalAccountCrossTenant => {
                    serializer.serialize_unit_variant("AuthenticationType", 3u32, "OrganizationalAccountCrossTenant")
                }
                Self::OrganizationalAccountOnly => {
                    serializer.serialize_unit_variant("AuthenticationType", 4u32, "OrganizationalAccountOnly")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The policy that controls whether account owner can view charges."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AccountOwnerViewCharges")]
    pub enum AccountOwnerViewCharges {
        Other,
        Allowed,
        Disabled,
        NotAllowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AccountOwnerViewCharges {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AccountOwnerViewCharges {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AccountOwnerViewCharges {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("AccountOwnerViewCharges", 0u32, "Other"),
                Self::Allowed => serializer.serialize_unit_variant("AccountOwnerViewCharges", 1u32, "Allowed"),
                Self::Disabled => serializer.serialize_unit_variant("AccountOwnerViewCharges", 2u32, "Disabled"),
                Self::NotAllowed => serializer.serialize_unit_variant("AccountOwnerViewCharges", 3u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The policy that controls whether department admin can view charges."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DepartmentAdminViewCharges")]
    pub enum DepartmentAdminViewCharges {
        Other,
        Allowed,
        Disabled,
        NotAllowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DepartmentAdminViewCharges {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DepartmentAdminViewCharges {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DepartmentAdminViewCharges {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("DepartmentAdminViewCharges", 0u32, "Other"),
                Self::Allowed => serializer.serialize_unit_variant("DepartmentAdminViewCharges", 1u32, "Allowed"),
                Self::Disabled => serializer.serialize_unit_variant("DepartmentAdminViewCharges", 2u32, "Disabled"),
                Self::NotAllowed => serializer.serialize_unit_variant("DepartmentAdminViewCharges", 3u32, "NotAllowed"),
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "Extended status definition properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedStatusDefinitionProperties {
    #[doc = "Subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
impl ExtendedStatusDefinitionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Extended status information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedStatusInfo {
    #[doc = "Status code providing additional information."]
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[doc = "The message giving detailed information about the status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Properties specific to credit line check failure"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<extended_status_info::Properties>,
}
impl ExtendedStatusInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extended_status_info {
    use super::*;
    #[doc = "Properties specific to credit line check failure"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The subscription that has failed credit line check."]
        #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
        pub subscription_id: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "An external reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalReference {
    #[doc = "The ID that uniquely identifies an external reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The URL of the external reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ExternalReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A failed payment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailedPayment {
    #[doc = "The date when the payment was attempted."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub date: Option<::time::OffsetDateTime>,
    #[doc = "The reason that the payment failed."]
    #[serde(rename = "failedPaymentReason", default, skip_serializing_if = "Option::is_none")]
    pub failed_payment_reason: Option<failed_payment::FailedPaymentReason>,
}
impl FailedPayment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod failed_payment {
    use super::*;
    #[doc = "The reason that the payment failed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FailedPaymentReason")]
    pub enum FailedPaymentReason {
        Other,
        BankDeclined,
        CardExpired,
        IncorrectCardDetails,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FailedPaymentReason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FailedPaymentReason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FailedPaymentReason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("FailedPaymentReason", 0u32, "Other"),
                Self::BankDeclined => serializer.serialize_unit_variant("FailedPaymentReason", 1u32, "BankDeclined"),
                Self::CardExpired => serializer.serialize_unit_variant("FailedPaymentReason", 2u32, "CardExpired"),
                Self::IncorrectCardDetails => serializer.serialize_unit_variant("FailedPaymentReason", 3u32, "IncorrectCardDetails"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Identifies the billing profile that is linked to another billing profile in indirect purchase motion."]
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
#[doc = "Request parameters to initiate transfer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InitiateTransferProperties {
    #[doc = "The email ID of the recipient to whom the transfer request is sent."]
    #[serde(rename = "recipientEmailId", default, skip_serializing_if = "Option::is_none")]
    pub recipient_email_id: Option<String>,
}
impl InitiateTransferProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request parameters to initiate transfer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InitiateTransferRequest {
    #[doc = "Request parameters to initiate transfer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InitiateTransferProperties>,
}
impl InitiateTransferRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of customer of the transfer initiator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InitiatorCustomerType")]
pub enum InitiatorCustomerType {
    Partner,
    #[serde(rename = "EA")]
    Ea,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InitiatorCustomerType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InitiatorCustomerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InitiatorCustomerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Partner => serializer.serialize_unit_variant("InitiatorCustomerType", 0u32, "Partner"),
            Self::Ea => serializer.serialize_unit_variant("InitiatorCustomerType", 1u32, "EA"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Turning this on will apply the reservation discount to other VMs in the same VM size group. Only specify for VirtualMachines reserved resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InstanceFlexibility")]
pub enum InstanceFlexibility {
    On,
    Off,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InstanceFlexibility {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InstanceFlexibility {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InstanceFlexibility {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::On => serializer.serialize_unit_variant("InstanceFlexibility", 0u32, "On"),
            Self::Off => serializer.serialize_unit_variant("InstanceFlexibility", 1u32, "Off"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An invoice."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Invoice {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "An invoice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InvoiceProperties>,
}
impl Invoice {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a document."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceDocument {
    #[doc = "The document numbers for the invoice document."]
    #[serde(
        rename = "documentNumbers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub document_numbers: Vec<String>,
    #[doc = "The URL to download the invoice document if the source is external to Microsoft.Billing."]
    #[serde(rename = "externalUrl", default, skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    #[doc = "The type of the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<invoice_document::Kind>,
    #[doc = "The ID that uniquely identifies an invoice document. This ID may be an identifier for an invoice PDF, a credit note, or a tax receipt."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The URL to download the invoice document if the source is internal to Microsoft.Billing."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The source of the document. ENF for Brazil and DRS for rest of the world."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<invoice_document::Source>,
}
impl InvoiceDocument {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod invoice_document {
    use super::*;
    #[doc = "The type of the document."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Other,
        Invoice,
        VoidNote,
        TaxReceipt,
        CreditNote,
        Summary,
        Transactions,
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
                Self::Other => serializer.serialize_unit_variant("Kind", 0u32, "Other"),
                Self::Invoice => serializer.serialize_unit_variant("Kind", 1u32, "Invoice"),
                Self::VoidNote => serializer.serialize_unit_variant("Kind", 2u32, "VoidNote"),
                Self::TaxReceipt => serializer.serialize_unit_variant("Kind", 3u32, "TaxReceipt"),
                Self::CreditNote => serializer.serialize_unit_variant("Kind", 4u32, "CreditNote"),
                Self::Summary => serializer.serialize_unit_variant("Kind", 5u32, "Summary"),
                Self::Transactions => serializer.serialize_unit_variant("Kind", 6u32, "Transactions"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The source of the document. ENF for Brazil and DRS for rest of the world."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Source")]
    pub enum Source {
        Other,
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
                Self::Other => serializer.serialize_unit_variant("Source", 0u32, "Other"),
                Self::Drs => serializer.serialize_unit_variant("Source", 1u32, "DRS"),
                Self::Enf => serializer.serialize_unit_variant("Source", 2u32, "ENF"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Invoice>,
}
impl azure_core::Continuable for InvoiceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl InvoiceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An invoice."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceProperties {
    #[doc = "The amount due as of now."]
    #[serde(rename = "amountDue", default, skip_serializing_if = "Option::is_none")]
    pub amount_due: Option<serde_json::Value>,
    #[doc = "The amount of Azure prepayment applied to the charges. This field is applicable to billing accounts with agreement type Microsoft Customer Agreement."]
    #[serde(rename = "azurePrepaymentApplied", default, skip_serializing_if = "Option::is_none")]
    pub azure_prepayment_applied: Option<serde_json::Value>,
    #[doc = "The total charges for the invoice billing period."]
    #[serde(rename = "billedAmount", default, skip_serializing_if = "Option::is_none")]
    pub billed_amount: Option<serde_json::Value>,
    #[doc = "The Id of the active invoice which is originally billed after this invoice was voided. This field is applicable to the void invoices only."]
    #[serde(rename = "billedDocumentId", default, skip_serializing_if = "Option::is_none")]
    pub billed_document_id: Option<String>,
    #[doc = "The name of the billing profile for which the invoice is generated."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The ID of the billing profile for which the invoice is generated."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The total refund for returns and cancellations during the invoice billing period. This field is applicable to billing accounts with agreement type Microsoft Customer Agreement."]
    #[serde(rename = "creditAmount", default, skip_serializing_if = "Option::is_none")]
    pub credit_amount: Option<serde_json::Value>,
    #[doc = "The Id of the invoice which got voided and this credit note was issued as a result. This field is applicable to the credit notes only."]
    #[serde(rename = "creditForDocumentId", default, skip_serializing_if = "Option::is_none")]
    pub credit_for_document_id: Option<String>,
    #[doc = "List of documents available to download and view such as invoice, credit note, or tax receipt."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub documents: Vec<InvoiceDocument>,
    #[doc = "The type of the document."]
    #[serde(rename = "documentType", default, skip_serializing_if = "Option::is_none")]
    pub document_type: Option<invoice_properties::DocumentType>,
    #[doc = "The due date for the invoice."]
    #[serde(rename = "dueDate", default, with = "azure_core::date::rfc3339::option")]
    pub due_date: Option<::time::OffsetDateTime>,
    #[doc = "List of failed payments."]
    #[serde(
        rename = "failedPayments",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub failed_payments: Vec<FailedPayment>,
    #[doc = "The amount of free Azure credits applied to the charges. This field is applicable to billing accounts with agreement type Microsoft Customer Agreement."]
    #[serde(rename = "freeAzureCreditApplied", default, skip_serializing_if = "Option::is_none")]
    pub free_azure_credit_applied: Option<serde_json::Value>,
    #[doc = "The date when the invoice was generated."]
    #[serde(rename = "invoiceDate", default, with = "azure_core::date::rfc3339::option")]
    pub invoice_date: Option<::time::OffsetDateTime>,
    #[doc = "The end date of the billing period for which the invoice is generated. The date is in MM-DD-YYYY format."]
    #[serde(rename = "invoicePeriodEndDate", default, with = "azure_core::date::rfc3339::option")]
    pub invoice_period_end_date: Option<::time::OffsetDateTime>,
    #[doc = "The start date of the billing period for which the invoice is generated. The date is in MM-DD-YYYY format."]
    #[serde(rename = "invoicePeriodStartDate", default, with = "azure_core::date::rfc3339::option")]
    pub invoice_period_start_date: Option<::time::OffsetDateTime>,
    #[doc = "Invoice type."]
    #[serde(rename = "invoiceType", default, skip_serializing_if = "Option::is_none")]
    pub invoice_type: Option<invoice_properties::InvoiceType>,
    #[doc = "Specifies if the invoice is generated as part of monthly invoicing cycle or not. This field is applicable to billing accounts with agreement type Microsoft Customer Agreement."]
    #[serde(rename = "isMonthlyInvoice", default, skip_serializing_if = "Option::is_none")]
    pub is_monthly_invoice: Option<bool>,
    #[doc = "List of payments."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub payments: Vec<Payment>,
    #[doc = "An optional purchase order number for the invoice."]
    #[serde(rename = "purchaseOrderNumber", default, skip_serializing_if = "Option::is_none")]
    pub purchase_order_number: Option<String>,
    #[doc = "Rebill details for an invoice."]
    #[serde(rename = "rebillDetails", default, skip_serializing_if = "Option::is_none")]
    pub rebill_details: Option<serde_json::Value>,
    #[doc = "The current status of the invoice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<invoice_properties::Status>,
    #[doc = "The name of the billing subscription for which the invoice is generated."]
    #[serde(rename = "subscriptionDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_display_name: Option<String>,
    #[doc = "The ID of the subscription for which the invoice is generated."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Identifies the type of tax calculation used for the invoice. The field is applicable only to invoices with special tax calculation logic."]
    #[serde(rename = "specialTaxationType", default, skip_serializing_if = "Option::is_none")]
    pub special_taxation_type: Option<invoice_properties::SpecialTaxationType>,
    #[doc = "The pre-tax amount due. This field is applicable to billing accounts with agreement type Microsoft Customer Agreement."]
    #[serde(rename = "subTotal", default, skip_serializing_if = "Option::is_none")]
    pub sub_total: Option<serde_json::Value>,
    #[doc = "The amount of tax charged for the billing period. This field is applicable to billing accounts with agreement type Microsoft Customer Agreement."]
    #[serde(rename = "taxAmount", default, skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<serde_json::Value>,
    #[doc = "The amount due when the invoice was generated. This field is applicable to billing accounts with agreement type Microsoft Customer Agreement."]
    #[serde(rename = "totalAmount", default, skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<serde_json::Value>,
    #[doc = "The details of a refund request."]
    #[serde(rename = "refundDetails", default, skip_serializing_if = "Option::is_none")]
    pub refund_details: Option<serde_json::Value>,
}
impl InvoiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod invoice_properties {
    use super::*;
    #[doc = "The type of the document."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DocumentType")]
    pub enum DocumentType {
        Other,
        Invoice,
        VoidNote,
        TaxReceipt,
        CreditNote,
        Summary,
        Transactions,
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
                Self::Other => serializer.serialize_unit_variant("DocumentType", 0u32, "Other"),
                Self::Invoice => serializer.serialize_unit_variant("DocumentType", 1u32, "Invoice"),
                Self::VoidNote => serializer.serialize_unit_variant("DocumentType", 2u32, "VoidNote"),
                Self::TaxReceipt => serializer.serialize_unit_variant("DocumentType", 3u32, "TaxReceipt"),
                Self::CreditNote => serializer.serialize_unit_variant("DocumentType", 4u32, "CreditNote"),
                Self::Summary => serializer.serialize_unit_variant("DocumentType", 5u32, "Summary"),
                Self::Transactions => serializer.serialize_unit_variant("DocumentType", 6u32, "Transactions"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Invoice type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InvoiceType")]
    pub enum InvoiceType {
        Other,
        AzureServices,
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
                Self::Other => serializer.serialize_unit_variant("InvoiceType", 0u32, "Other"),
                Self::AzureServices => serializer.serialize_unit_variant("InvoiceType", 1u32, "AzureServices"),
                Self::AzureMarketplace => serializer.serialize_unit_variant("InvoiceType", 2u32, "AzureMarketplace"),
                Self::AzureSupport => serializer.serialize_unit_variant("InvoiceType", 3u32, "AzureSupport"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current status of the invoice."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Other,
        Due,
        OverDue,
        Paid,
        Void,
        Locked,
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
                Self::Other => serializer.serialize_unit_variant("Status", 0u32, "Other"),
                Self::Due => serializer.serialize_unit_variant("Status", 1u32, "Due"),
                Self::OverDue => serializer.serialize_unit_variant("Status", 2u32, "OverDue"),
                Self::Paid => serializer.serialize_unit_variant("Status", 3u32, "Paid"),
                Self::Void => serializer.serialize_unit_variant("Status", 4u32, "Void"),
                Self::Locked => serializer.serialize_unit_variant("Status", 5u32, "Locked"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Identifies the type of tax calculation used for the invoice. The field is applicable only to invoices with special tax calculation logic."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SpecialTaxationType")]
    pub enum SpecialTaxationType {
        SubtotalLevel,
        InvoiceLevel,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SpecialTaxationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SpecialTaxationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SpecialTaxationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SubtotalLevel => serializer.serialize_unit_variant("SpecialTaxationType", 0u32, "SubtotalLevel"),
                Self::InvoiceLevel => serializer.serialize_unit_variant("SpecialTaxationType", 1u32, "InvoiceLevel"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An invoice section."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceSection {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "An invoice section."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InvoiceSectionProperties>,
}
impl InvoiceSection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceSectionListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<InvoiceSection>,
}
impl azure_core::Continuable for InvoiceSectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl InvoiceSectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An invoice section."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceSectionProperties {
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<invoice_section_properties::ProvisioningState>,
    #[doc = "The name of the invoice section."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Identifies the status of an invoice section."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<invoice_section_properties::State>,
    #[doc = "Reason for the specified invoice section status."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<invoice_section_properties::ReasonCode>,
    #[doc = "The system generated unique identifier for an invoice section."]
    #[serde(rename = "systemId", default, skip_serializing_if = "Option::is_none")]
    pub system_id: Option<String>,
    #[doc = "Identifies the cloud environments that are associated with an invoice section. This is a system managed optional field and gets updated as the invoice section gets associated with accounts in various clouds."]
    #[serde(rename = "targetCloud", default, skip_serializing_if = "Option::is_none")]
    pub target_cloud: Option<String>,
    #[doc = "Dictionary of metadata associated with the resource. Maximum key/value length supported of 256 characters. Keys/value should not empty value nor null. Keys can not contain < > % & \\ ? /"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl InvoiceSectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod invoice_section_properties {
    use super::*;
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Canceled,
        Failed,
        New,
        Pending,
        Provisioning,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::New => serializer.serialize_unit_variant("ProvisioningState", 3u32, "New"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Identifies the status of an invoice section."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Other,
        Active,
        Deleted,
        Disabled,
        UnderReview,
        Warned,
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
                Self::Other => serializer.serialize_unit_variant("State", 0u32, "Other"),
                Self::Active => serializer.serialize_unit_variant("State", 1u32, "Active"),
                Self::Deleted => serializer.serialize_unit_variant("State", 2u32, "Deleted"),
                Self::Disabled => serializer.serialize_unit_variant("State", 3u32, "Disabled"),
                Self::UnderReview => serializer.serialize_unit_variant("State", 4u32, "UnderReview"),
                Self::Warned => serializer.serialize_unit_variant("State", 5u32, "Warned"),
                Self::Restricted => serializer.serialize_unit_variant("State", 6u32, "Restricted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Reason for the specified invoice section status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReasonCode")]
    pub enum ReasonCode {
        Other,
        PastDue,
        UnusualActivity,
        SpendingLimitReached,
        SpendingLimitExpired,
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
                Self::Other => serializer.serialize_unit_variant("ReasonCode", 0u32, "Other"),
                Self::PastDue => serializer.serialize_unit_variant("ReasonCode", 1u32, "PastDue"),
                Self::UnusualActivity => serializer.serialize_unit_variant("ReasonCode", 2u32, "UnusualActivity"),
                Self::SpendingLimitReached => serializer.serialize_unit_variant("ReasonCode", 3u32, "SpendingLimitReached"),
                Self::SpendingLimitExpired => serializer.serialize_unit_variant("ReasonCode", 4u32, "SpendingLimitExpired"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Invoice section properties with create subscription permission."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceSectionWithCreateSubPermission {
    #[doc = "The name of the billing profile."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies a billing profile."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The system generated unique identifier for a billing profile."]
    #[serde(rename = "billingProfileSystemId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_system_id: Option<String>,
    #[doc = "The status of the billing profile."]
    #[serde(rename = "billingProfileStatus", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_status: Option<invoice_section_with_create_sub_permission::BillingProfileStatus>,
    #[doc = "Reason for the specified billing profile status."]
    #[serde(rename = "billingProfileStatusReasonCode", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_status_reason_code: Option<invoice_section_with_create_sub_permission::BillingProfileStatusReasonCode>,
    #[doc = "The billing profile spending limit."]
    #[serde(rename = "billingProfileSpendingLimit", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_spending_limit: Option<invoice_section_with_create_sub_permission::BillingProfileSpendingLimit>,
    #[doc = "Enabled azure plans for the associated billing profile."]
    #[serde(
        rename = "enabledAzurePlans",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enabled_azure_plans: Vec<AzurePlan>,
    #[doc = "The name of the invoice section."]
    #[serde(rename = "invoiceSectionDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_display_name: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies an invoice section."]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "The system generated unique identifier for an invoice section."]
    #[serde(rename = "invoiceSectionSystemId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_system_id: Option<String>,
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
        Other,
        Active,
        Disabled,
        Warned,
        Deleted,
        UnderReview,
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
                Self::Other => serializer.serialize_unit_variant("BillingProfileStatus", 0u32, "Other"),
                Self::Active => serializer.serialize_unit_variant("BillingProfileStatus", 1u32, "Active"),
                Self::Disabled => serializer.serialize_unit_variant("BillingProfileStatus", 2u32, "Disabled"),
                Self::Warned => serializer.serialize_unit_variant("BillingProfileStatus", 3u32, "Warned"),
                Self::Deleted => serializer.serialize_unit_variant("BillingProfileStatus", 4u32, "Deleted"),
                Self::UnderReview => serializer.serialize_unit_variant("BillingProfileStatus", 5u32, "UnderReview"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Reason for the specified billing profile status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingProfileStatusReasonCode")]
    pub enum BillingProfileStatusReasonCode {
        Other,
        PastDue,
        UnusualActivity,
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
                Self::Other => serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 0u32, "Other"),
                Self::PastDue => serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 1u32, "PastDue"),
                Self::UnusualActivity => serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 2u32, "UnusualActivity"),
                Self::SpendingLimitReached => {
                    serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 3u32, "SpendingLimitReached")
                }
                Self::SpendingLimitExpired => {
                    serializer.serialize_unit_variant("BillingProfileStatusReasonCode", 4u32, "SpendingLimitExpired")
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
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvoiceSectionWithCreateSubPermissionListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<InvoiceSectionWithCreateSubPermission>,
}
impl azure_core::Continuable for InvoiceSectionWithCreateSubPermissionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl InvoiceSectionWithCreateSubPermissionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ManagementGroupId = String;
#[doc = "Result of the transfer eligibility validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveBillingSubscriptionEligibilityResult {
    #[doc = "Specifies whether the subscription is eligible to be transferred."]
    #[serde(rename = "isMoveEligible", default, skip_serializing_if = "Option::is_none")]
    pub is_move_eligible: Option<bool>,
    #[doc = "Error details of the transfer eligibility validation."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<MoveBillingSubscriptionErrorDetails>,
}
impl MoveBillingSubscriptionEligibilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details of the transfer eligibility validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveBillingSubscriptionErrorDetails {
    #[doc = "Error code of the transfer validation response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<move_billing_subscription_error_details::Code>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Detailed error message explaining the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl MoveBillingSubscriptionErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod move_billing_subscription_error_details {
    use super::*;
    #[doc = "Error code of the transfer validation response."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Code")]
    pub enum Code {
        Other,
        BillingAccountInactive,
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
        ProductInactive,
        ProductNotFound,
        ProductTypeNotSupported,
        SourceBillingProfilePastDue,
        SourceInvoiceSectionInactive,
        AccountIsLocked,
        AssetHasCap,
        AssetNotActive,
        BillingProfilePastDue,
        CrossBillingAccountNotAllowed,
        NoActiveAzurePlan,
        None,
        SubscriptionNotActive,
        SubscriptionHasReservations,
        SubscriptionTypeNotSupported,
        InvoiceSectionIsRestricted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Code {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Code {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Code {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("Code", 0u32, "Other"),
                Self::BillingAccountInactive => serializer.serialize_unit_variant("Code", 1u32, "BillingAccountInactive"),
                Self::DestinationBillingProfileInactive => {
                    serializer.serialize_unit_variant("Code", 2u32, "DestinationBillingProfileInactive")
                }
                Self::DestinationBillingProfileNotFound => {
                    serializer.serialize_unit_variant("Code", 3u32, "DestinationBillingProfileNotFound")
                }
                Self::DestinationBillingProfilePastDue => {
                    serializer.serialize_unit_variant("Code", 4u32, "DestinationBillingProfilePastDue")
                }
                Self::DestinationInvoiceSectionInactive => {
                    serializer.serialize_unit_variant("Code", 5u32, "DestinationInvoiceSectionInactive")
                }
                Self::DestinationInvoiceSectionNotFound => {
                    serializer.serialize_unit_variant("Code", 6u32, "DestinationInvoiceSectionNotFound")
                }
                Self::InsufficientPermissionOnDestination => {
                    serializer.serialize_unit_variant("Code", 7u32, "InsufficientPermissionOnDestination")
                }
                Self::InsufficientPermissionOnSource => serializer.serialize_unit_variant("Code", 8u32, "InsufficientPermissionOnSource"),
                Self::InvalidDestination => serializer.serialize_unit_variant("Code", 9u32, "InvalidDestination"),
                Self::InvalidSource => serializer.serialize_unit_variant("Code", 10u32, "InvalidSource"),
                Self::MarketplaceNotEnabledOnDestination => {
                    serializer.serialize_unit_variant("Code", 11u32, "MarketplaceNotEnabledOnDestination")
                }
                Self::ProductInactive => serializer.serialize_unit_variant("Code", 12u32, "ProductInactive"),
                Self::ProductNotFound => serializer.serialize_unit_variant("Code", 13u32, "ProductNotFound"),
                Self::ProductTypeNotSupported => serializer.serialize_unit_variant("Code", 14u32, "ProductTypeNotSupported"),
                Self::SourceBillingProfilePastDue => serializer.serialize_unit_variant("Code", 15u32, "SourceBillingProfilePastDue"),
                Self::SourceInvoiceSectionInactive => serializer.serialize_unit_variant("Code", 16u32, "SourceInvoiceSectionInactive"),
                Self::AccountIsLocked => serializer.serialize_unit_variant("Code", 17u32, "AccountIsLocked"),
                Self::AssetHasCap => serializer.serialize_unit_variant("Code", 18u32, "AssetHasCap"),
                Self::AssetNotActive => serializer.serialize_unit_variant("Code", 19u32, "AssetNotActive"),
                Self::BillingProfilePastDue => serializer.serialize_unit_variant("Code", 20u32, "BillingProfilePastDue"),
                Self::CrossBillingAccountNotAllowed => serializer.serialize_unit_variant("Code", 21u32, "CrossBillingAccountNotAllowed"),
                Self::NoActiveAzurePlan => serializer.serialize_unit_variant("Code", 22u32, "NoActiveAzurePlan"),
                Self::None => serializer.serialize_unit_variant("Code", 23u32, "None"),
                Self::SubscriptionNotActive => serializer.serialize_unit_variant("Code", 24u32, "SubscriptionNotActive"),
                Self::SubscriptionHasReservations => serializer.serialize_unit_variant("Code", 25u32, "SubscriptionHasReservations"),
                Self::SubscriptionTypeNotSupported => serializer.serialize_unit_variant("Code", 26u32, "SubscriptionTypeNotSupported"),
                Self::InvoiceSectionIsRestricted => serializer.serialize_unit_variant("Code", 27u32, "InvoiceSectionIsRestricted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Request parameters to transfer billing subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveBillingSubscriptionRequest {
    #[doc = "The destination invoice section id."]
    #[serde(rename = "destinationInvoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub destination_invoice_section_id: Option<String>,
    #[doc = "The destination enrollment account id."]
    #[serde(rename = "destinationEnrollmentAccountId", default, skip_serializing_if = "Option::is_none")]
    pub destination_enrollment_account_id: Option<String>,
}
impl MoveBillingSubscriptionRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the transfer eligibility validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveProductEligibilityResult {
    #[doc = "Specifies whether the subscription is eligible to be transferred."]
    #[serde(rename = "isMoveEligible", default, skip_serializing_if = "Option::is_none")]
    pub is_move_eligible: Option<bool>,
    #[doc = "Error details of the transfer eligibility validation."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<serde_json::Value>,
}
impl MoveProductEligibilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details of the transfer eligibility validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveProductErrorDetails {
    #[doc = "Error code for the product transfer validation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<move_product_error_details::Code>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Error details of the transfer eligibility validation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl MoveProductErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod move_product_error_details {
    use super::*;
    #[doc = "Error code for the product transfer validation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Code")]
    pub enum Code {
        Other,
        BillingAccountInactive,
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
        ProductInactive,
        ProductNotFound,
        ProductTypeNotSupported,
        SourceBillingProfilePastDue,
        SourceInvoiceSectionInactive,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Code {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Code {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Code {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("Code", 0u32, "Other"),
                Self::BillingAccountInactive => serializer.serialize_unit_variant("Code", 1u32, "BillingAccountInactive"),
                Self::DestinationBillingProfileInactive => {
                    serializer.serialize_unit_variant("Code", 2u32, "DestinationBillingProfileInactive")
                }
                Self::DestinationBillingProfileNotFound => {
                    serializer.serialize_unit_variant("Code", 3u32, "DestinationBillingProfileNotFound")
                }
                Self::DestinationBillingProfilePastDue => {
                    serializer.serialize_unit_variant("Code", 4u32, "DestinationBillingProfilePastDue")
                }
                Self::DestinationInvoiceSectionInactive => {
                    serializer.serialize_unit_variant("Code", 5u32, "DestinationInvoiceSectionInactive")
                }
                Self::DestinationInvoiceSectionNotFound => {
                    serializer.serialize_unit_variant("Code", 6u32, "DestinationInvoiceSectionNotFound")
                }
                Self::InsufficientPermissionOnDestination => {
                    serializer.serialize_unit_variant("Code", 7u32, "InsufficientPermissionOnDestination")
                }
                Self::InsufficientPermissionOnSource => serializer.serialize_unit_variant("Code", 8u32, "InsufficientPermissionOnSource"),
                Self::InvalidDestination => serializer.serialize_unit_variant("Code", 9u32, "InvalidDestination"),
                Self::InvalidSource => serializer.serialize_unit_variant("Code", 10u32, "InvalidSource"),
                Self::MarketplaceNotEnabledOnDestination => {
                    serializer.serialize_unit_variant("Code", 11u32, "MarketplaceNotEnabledOnDestination")
                }
                Self::ProductInactive => serializer.serialize_unit_variant("Code", 12u32, "ProductInactive"),
                Self::ProductNotFound => serializer.serialize_unit_variant("Code", 13u32, "ProductNotFound"),
                Self::ProductTypeNotSupported => serializer.serialize_unit_variant("Code", 14u32, "ProductTypeNotSupported"),
                Self::SourceBillingProfilePastDue => serializer.serialize_unit_variant("Code", 15u32, "SourceBillingProfilePastDue"),
                Self::SourceInvoiceSectionInactive => serializer.serialize_unit_variant("Code", 16u32, "SourceInvoiceSectionInactive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The properties of the product to initiate a transfer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveProductRequest {
    #[doc = "The destination invoice section id."]
    #[serde(rename = "destinationInvoiceSectionId")]
    pub destination_invoice_section_id: String,
}
impl MoveProductRequest {
    pub fn new(destination_invoice_section_id: String) -> Self {
        Self {
            destination_invoice_section_id,
        }
    }
}
#[doc = "Billing cycle details of the product."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NextBillingCycleDetails {
    #[doc = "Billing frequency of the product under the subscription."]
    #[serde(rename = "billingFrequency", default, skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<String>,
}
impl NextBillingCycleDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of a REST API operation, returned from the Resource Provider Operations API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation, as per Resource-Based Access Control (RBAC). Examples: \"Microsoft.Compute/virtualMachines/write\", \"Microsoft.Compute/virtualMachines/capture/action\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Localized display information for this particular operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "The short, localized friendly description of the operation; suitable for tool tips and detailed views."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The concise, localized friendly name for the operation; suitable for dropdowns. E.g. \"Create or Update Virtual Machine\", \"Restart Virtual Machine\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The localized friendly form of the resource provider name, e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The localized friendly name of the resource type related to this operation. E.g. \"Virtual Machines\" or \"Job Schedule Collections\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Billing account name. Available for a specific type of agreement."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Participant {
    #[doc = "The email address of the participant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The acceptance status of the participant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The date when the status got changed."]
    #[serde(rename = "statusDate", default, with = "azure_core::date::rfc3339::option")]
    pub status_date: Option<::time::OffsetDateTime>,
}
impl Participant {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request parameters to initiate transfer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartnerInitiateTransferProperties {
    #[doc = "The email ID of the recipient to whom the transfer request is sent."]
    #[serde(rename = "recipientEmailId", default, skip_serializing_if = "Option::is_none")]
    pub recipient_email_id: Option<String>,
    #[doc = "Optional MPN ID of the reseller for transfer requests that are sent from a Microsoft Partner Agreement billing account."]
    #[serde(rename = "resellerId", default, skip_serializing_if = "Option::is_none")]
    pub reseller_id: Option<String>,
}
impl PartnerInitiateTransferProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request parameters to initiate partner transfer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartnerInitiateTransferRequest {
    #[doc = "Request parameters to initiate transfer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PartnerInitiateTransferProperties>,
}
impl PartnerInitiateTransferRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the transfer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartnerTransferDetails {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "Transfer Details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PartnerTransferProperties>,
}
impl PartnerTransferDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of transfers initiated by partner."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartnerTransferDetailsListResult {
    #[doc = "The list of transfers initiated by partner."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PartnerTransferDetails>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PartnerTransferDetailsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PartnerTransferDetailsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Transfer Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartnerTransferProperties {
    #[doc = "The time at which the transfer request expires."]
    #[serde(rename = "expirationTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_time: Option<::time::OffsetDateTime>,
    #[doc = "The status of a transfer."]
    #[serde(rename = "transferStatus", default, skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<TransferStatus>,
    #[doc = "The email ID of the user to whom the transfer request was sent."]
    #[serde(rename = "recipientEmailId", default, skip_serializing_if = "Option::is_none")]
    pub recipient_email_id: Option<String>,
    #[doc = "The type of customer of the transfer initiator."]
    #[serde(rename = "initiatorCustomerType", default, skip_serializing_if = "Option::is_none")]
    pub initiator_customer_type: Option<InitiatorCustomerType>,
    #[doc = "The email ID of the user who sent the transfer request."]
    #[serde(rename = "initiatorEmailId", default, skip_serializing_if = "Option::is_none")]
    pub initiator_email_id: Option<String>,
    #[doc = "Optional MPN ID of the reseller for transfer requests that are sent from a Microsoft Partner Agreement billing account."]
    #[serde(rename = "resellerId", default, skip_serializing_if = "Option::is_none")]
    pub reseller_id: Option<String>,
    #[doc = "Optional name of the reseller for transfer requests that are sent from Microsoft Partner Agreement billing account."]
    #[serde(rename = "resellerName", default, skip_serializing_if = "Option::is_none")]
    pub reseller_name: Option<String>,
    #[doc = "The email ID of the user who canceled the transfer request."]
    #[serde(rename = "canceledBy", default, skip_serializing_if = "Option::is_none")]
    pub canceled_by: Option<String>,
    #[doc = "Detailed transfer status."]
    #[serde(
        rename = "detailedTransferStatus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub detailed_transfer_status: Vec<DetailedTransferStatus>,
}
impl PartnerTransferProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The request for reservation patch"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Patch {
    #[doc = "Properties for reservation patch"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PatchProperties>,
    #[doc = "The property of reservation sku object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ReservationSkuProperty>,
    #[doc = "Resource Tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
impl Patch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for reservation patch"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchProperties {
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable. Required and need to provide tenantId and managementGroupId if AppliedScopeType is ManagementGroup"]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<ReservationAppliedScopeProperties>,
    #[doc = "Turning this on will apply the reservation discount to other VMs in the same VM size group. Only specify for VirtualMachines reserved resource type."]
    #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[doc = "Display name of the reservation"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Setting this to true will automatically purchase a new reservation on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[serde(rename = "renewProperties", default, skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<patch_properties::RenewProperties>,
    #[doc = "This is the date-time when the Azure hybrid benefit needs to be reviewed."]
    #[serde(rename = "reviewDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub review_date_time: Option<::time::OffsetDateTime>,
}
impl PatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RenewProperties {
        #[doc = "The request for reservation purchase"]
        #[serde(rename = "purchaseProperties", default, skip_serializing_if = "Option::is_none")]
        pub purchase_properties: Option<ReservationPurchaseRequest>,
    }
    impl RenewProperties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Common fields that are in the patch method request body of all Azure Resource Manager patch resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchResource {}
impl PatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An invoice payment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Payment {
    #[doc = "The paid amount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<serde_json::Value>,
    #[doc = "The date when the payment was made."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub date: Option<::time::OffsetDateTime>,
    #[doc = "The ID that uniquely identifies the payment method used for the invoice."]
    #[serde(rename = "paymentMethodId", default, skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[doc = "The family of payment method."]
    #[serde(rename = "paymentMethodFamily", default, skip_serializing_if = "Option::is_none")]
    pub payment_method_family: Option<payment::PaymentMethodFamily>,
    #[doc = "The type of payment method."]
    #[serde(rename = "paymentMethodType", default, skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    #[doc = "The type of payment."]
    #[serde(rename = "paymentType", default, skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
}
impl Payment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod payment {
    use super::*;
    #[doc = "The family of payment method."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PaymentMethodFamily")]
    pub enum PaymentMethodFamily {
        Other,
        None,
        CreditCard,
        Credits,
        CheckWire,
        EWallet,
        TaskOrder,
        DirectDebit,
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
                Self::Other => serializer.serialize_unit_variant("PaymentMethodFamily", 0u32, "Other"),
                Self::None => serializer.serialize_unit_variant("PaymentMethodFamily", 1u32, "None"),
                Self::CreditCard => serializer.serialize_unit_variant("PaymentMethodFamily", 2u32, "CreditCard"),
                Self::Credits => serializer.serialize_unit_variant("PaymentMethodFamily", 3u32, "Credits"),
                Self::CheckWire => serializer.serialize_unit_variant("PaymentMethodFamily", 4u32, "CheckWire"),
                Self::EWallet => serializer.serialize_unit_variant("PaymentMethodFamily", 5u32, "EWallet"),
                Self::TaskOrder => serializer.serialize_unit_variant("PaymentMethodFamily", 6u32, "TaskOrder"),
                Self::DirectDebit => serializer.serialize_unit_variant("PaymentMethodFamily", 7u32, "DirectDebit"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information about payment related to a savings plan order."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentDetail {
    #[doc = "Date when the payment needs to be done."]
    #[serde(rename = "dueDate", default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[doc = "Date when the transaction is completed. Null when it is scheduled."]
    #[serde(rename = "paymentDate", default, skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<String>,
    #[doc = "The price."]
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<Price>,
    #[doc = "The price."]
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<Price>,
    #[doc = "Describes whether the payment is completed, failed, pending, cancelled or scheduled in the future."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentStatus>,
    #[doc = "Extended status information"]
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
}
impl PaymentDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A payment method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentMethod {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
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
    pub proxy_resource_with_tags: ProxyResourceWithTags,
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
    #[doc = "The account holder name for the payment method. This is only supported for payment methods with family CreditCard."]
    #[serde(rename = "accountHolderName", default, skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    #[doc = "The display name of the payment method."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The expiration month and year of the payment method. This is only supported for payment methods with family CreditCard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[doc = "The family of payment method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<payment_method_link_properties::Family>,
    #[doc = "Last four digits of payment method."]
    #[serde(rename = "lastFourDigits", default, skip_serializing_if = "Option::is_none")]
    pub last_four_digits: Option<String>,
    #[doc = "The list of logos for the payment method."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub logos: Vec<PaymentMethodLogo>,
    #[doc = "The properties of a payment method."]
    #[serde(rename = "paymentMethod", default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodProperties>,
    #[doc = "Id of payment method. Example: /providers/Microsoft.Billing/paymentMethods/ABCDABCDABC0"]
    #[serde(rename = "paymentMethodId", default, skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    #[doc = "The type of payment method."]
    #[serde(rename = "paymentMethodType", default, skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    #[doc = "Status of the payment method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<payment_method_link_properties::Status>,
}
impl PaymentMethodLinkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod payment_method_link_properties {
    use super::*;
    #[doc = "The family of payment method."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Family")]
    pub enum Family {
        Other,
        None,
        CreditCard,
        Credits,
        CheckWire,
        EWallet,
        TaskOrder,
        DirectDebit,
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
                Self::Other => serializer.serialize_unit_variant("Family", 0u32, "Other"),
                Self::None => serializer.serialize_unit_variant("Family", 1u32, "None"),
                Self::CreditCard => serializer.serialize_unit_variant("Family", 2u32, "CreditCard"),
                Self::Credits => serializer.serialize_unit_variant("Family", 3u32, "Credits"),
                Self::CheckWire => serializer.serialize_unit_variant("Family", 4u32, "CheckWire"),
                Self::EWallet => serializer.serialize_unit_variant("Family", 5u32, "EWallet"),
                Self::TaskOrder => serializer.serialize_unit_variant("Family", 6u32, "TaskOrder"),
                Self::DirectDebit => serializer.serialize_unit_variant("Family", 7u32, "DirectDebit"),
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
#[doc = "The list of payment method links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentMethodLinksListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of payment method links."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PaymentMethodLink>,
}
impl azure_core::Continuable for PaymentMethodLinksListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
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
#[doc = "The properties of a payment method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentMethodProperties {
    #[doc = "Id of payment method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The account holder name for the payment method. This is only supported for payment methods with family CreditCard."]
    #[serde(rename = "accountHolderName", default, skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    #[doc = "The display name of the payment method."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The expiration month and year of the payment method. This is only supported for payment methods with family CreditCard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[doc = "The family of payment method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<payment_method_properties::Family>,
    #[doc = "Last four digits of payment method."]
    #[serde(rename = "lastFourDigits", default, skip_serializing_if = "Option::is_none")]
    pub last_four_digits: Option<String>,
    #[doc = "The list of logos for the payment method."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub logos: Vec<PaymentMethodLogo>,
    #[doc = "The type of payment method."]
    #[serde(rename = "paymentMethodType", default, skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
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
        Other,
        None,
        CreditCard,
        Credits,
        CheckWire,
        EWallet,
        TaskOrder,
        DirectDebit,
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
                Self::Other => serializer.serialize_unit_variant("Family", 0u32, "Other"),
                Self::None => serializer.serialize_unit_variant("Family", 1u32, "None"),
                Self::CreditCard => serializer.serialize_unit_variant("Family", 2u32, "CreditCard"),
                Self::Credits => serializer.serialize_unit_variant("Family", 3u32, "Credits"),
                Self::CheckWire => serializer.serialize_unit_variant("Family", 4u32, "CheckWire"),
                Self::EWallet => serializer.serialize_unit_variant("Family", 5u32, "EWallet"),
                Self::TaskOrder => serializer.serialize_unit_variant("Family", 6u32, "TaskOrder"),
                Self::DirectDebit => serializer.serialize_unit_variant("Family", 7u32, "DirectDebit"),
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
#[doc = "The list of payment methods."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentMethodsListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of payment methods."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PaymentMethod>,
}
impl azure_core::Continuable for PaymentMethodsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PaymentMethodsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Payment on Account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentOnAccount {
    #[doc = "Payment on Account amount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<serde_json::Value>,
    #[doc = "The ID of the billing profile for the payments on account."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The name of the billing profile for the payments on account."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The ID of the invoice for which the payments on account was generated."]
    #[serde(rename = "invoiceId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[doc = "The name of the invoice for the payments on account."]
    #[serde(rename = "invoiceName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_name: Option<String>,
    #[doc = "The date of the payments on account."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub date: Option<::time::OffsetDateTime>,
    #[doc = "Payment on Account type."]
    #[serde(rename = "paymentMethodType", default, skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<payment_on_account::PaymentMethodType>,
}
impl PaymentOnAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod payment_on_account {
    use super::*;
    #[doc = "Payment on Account type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PaymentMethodType")]
    pub enum PaymentMethodType {
        Other,
        None,
        CreditCard,
        Credits,
        CheckWire,
        EWallet,
        TaskOrder,
        DirectDebit,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PaymentMethodType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PaymentMethodType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PaymentMethodType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("PaymentMethodType", 0u32, "Other"),
                Self::None => serializer.serialize_unit_variant("PaymentMethodType", 1u32, "None"),
                Self::CreditCard => serializer.serialize_unit_variant("PaymentMethodType", 2u32, "CreditCard"),
                Self::Credits => serializer.serialize_unit_variant("PaymentMethodType", 3u32, "Credits"),
                Self::CheckWire => serializer.serialize_unit_variant("PaymentMethodType", 4u32, "CheckWire"),
                Self::EWallet => serializer.serialize_unit_variant("PaymentMethodType", 5u32, "EWallet"),
                Self::TaskOrder => serializer.serialize_unit_variant("PaymentMethodType", 6u32, "TaskOrder"),
                Self::DirectDebit => serializer.serialize_unit_variant("PaymentMethodType", 7u32, "DirectDebit"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes whether the payment is completed, failed, pending, cancelled or scheduled in the future."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PaymentStatus")]
pub enum PaymentStatus {
    Succeeded,
    Failed,
    Scheduled,
    Cancelled,
    Completed,
    Pending,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PaymentStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PaymentStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PaymentStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("PaymentStatus", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("PaymentStatus", 1u32, "Failed"),
            Self::Scheduled => serializer.serialize_unit_variant("PaymentStatus", 2u32, "Scheduled"),
            Self::Cancelled => serializer.serialize_unit_variant("PaymentStatus", 3u32, "Cancelled"),
            Self::Completed => serializer.serialize_unit_variant("PaymentStatus", 4u32, "Completed"),
            Self::Pending => serializer.serialize_unit_variant("PaymentStatus", 5u32, "Pending"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties of payment term."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentTerm {
    #[doc = "Represents duration in netXX format. Always in days."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[doc = "The date on when the defined 'Payment Term' will be effective from and is always in UTC."]
    #[serde(rename = "startDate", default, with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<::time::OffsetDateTime>,
    #[doc = "The date on when the defined 'Payment Term' will end and is always in UTC."]
    #[serde(rename = "endDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<::time::OffsetDateTime>,
    #[doc = "Indicates payment term is the standard payment term."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}
impl PaymentTerm {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the payment terms eligibility."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentTermsEligibilityDetail {
    #[doc = "Indicates the reason for the ineligibility of the payment terms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<payment_terms_eligibility_detail::Code>,
    #[doc = "Indicates the message for the ineligibility of the payment terms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl PaymentTermsEligibilityDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod payment_terms_eligibility_detail {
    use super::*;
    #[doc = "Indicates the reason for the ineligibility of the payment terms."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Code")]
    pub enum Code {
        Other,
        OverlappingPaymentTerms,
        InvalidDateFormat,
        InvalidDateRange,
        InactiveBillingAccount,
        InvalidBillingAccountType,
        NullOrEmptyPaymentTerms,
        BillingAccountNotFound,
        IneligibleBillingAccountStatus,
        InvalidTerms,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Code {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Code {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Code {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("Code", 0u32, "Other"),
                Self::OverlappingPaymentTerms => serializer.serialize_unit_variant("Code", 1u32, "OverlappingPaymentTerms"),
                Self::InvalidDateFormat => serializer.serialize_unit_variant("Code", 2u32, "InvalidDateFormat"),
                Self::InvalidDateRange => serializer.serialize_unit_variant("Code", 3u32, "InvalidDateRange"),
                Self::InactiveBillingAccount => serializer.serialize_unit_variant("Code", 4u32, "InactiveBillingAccount"),
                Self::InvalidBillingAccountType => serializer.serialize_unit_variant("Code", 5u32, "InvalidBillingAccountType"),
                Self::NullOrEmptyPaymentTerms => serializer.serialize_unit_variant("Code", 6u32, "NullOrEmptyPaymentTerms"),
                Self::BillingAccountNotFound => serializer.serialize_unit_variant("Code", 7u32, "BillingAccountNotFound"),
                Self::IneligibleBillingAccountStatus => serializer.serialize_unit_variant("Code", 8u32, "IneligibleBillingAccountStatus"),
                Self::InvalidTerms => serializer.serialize_unit_variant("Code", 9u32, "InvalidTerms"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of the payment terms eligibility."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentTermsEligibilityResult {
    #[doc = "Indicates the eligibility status of the payment terms."]
    #[serde(rename = "eligibilityStatus", default, skip_serializing_if = "Option::is_none")]
    pub eligibility_status: Option<payment_terms_eligibility_result::EligibilityStatus>,
    #[doc = "Details of the payment terms eligibility."]
    #[serde(
        rename = "eligibilityDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub eligibility_details: Vec<PaymentTermsEligibilityDetail>,
}
impl PaymentTermsEligibilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod payment_terms_eligibility_result {
    use super::*;
    #[doc = "Indicates the eligibility status of the payment terms."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EligibilityStatus")]
    pub enum EligibilityStatus {
        Other,
        Valid,
        Invalid,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EligibilityStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EligibilityStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EligibilityStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("EligibilityStatus", 0u32, "Other"),
                Self::Valid => serializer.serialize_unit_variant("EligibilityStatus", 1u32, "Valid"),
                Self::Invalid => serializer.serialize_unit_variant("EligibilityStatus", 2u32, "Invalid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The summary of the policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicySummary {
    #[doc = "The name of the policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The type of the policy."]
    #[serde(rename = "policyType", default, skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<policy_summary::PolicyType>,
    #[doc = "The scope at which the policy is defined."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
impl PolicySummary {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_summary {
    use super::*;
    #[doc = "The type of the policy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PolicyType")]
    pub enum PolicyType {
        Other,
        UserControlled,
        SystemControlled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PolicyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PolicyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PolicyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("PolicyType", 0u32, "Other"),
                Self::UserControlled => serializer.serialize_unit_variant("PolicyType", 1u32, "UserControlled"),
                Self::SystemControlled => serializer.serialize_unit_variant("PolicyType", 2u32, "SystemControlled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The price."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Price {
    #[doc = "The ISO 4217 3-letter currency code for the currency used by this purchase record."]
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}
impl Price {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A principal who has interacted with a billing entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Principal {
    #[doc = "The tenant id of the principal who has interacted with a billing entity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The object id of the principal who has interacted with a billing entity."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "The user principal name of the principal who has interacted with a billing entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upn: Option<String>,
}
impl Principal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A product."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Product {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "A product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductProperties>,
}
impl Product {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ProductCode = String;
#[doc = "Details of the product that is transferred."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductDetails {
    #[doc = "The type of product that is transferred."]
    #[serde(rename = "productType", default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,
    #[doc = "The ID of the product that is transferred."]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}
impl ProductDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Product>,
}
impl azure_core::Continuable for ProductListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProductListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A product."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductPatch {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "A product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProductProperties>,
}
impl ProductPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A product."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductProperties {
    #[doc = "Indicates whether auto renewal is turned on or off for a product."]
    #[serde(rename = "autoRenew", default, skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<product_properties::AutoRenew>,
    #[doc = "The availability of the product."]
    #[serde(rename = "availabilityId", default, skip_serializing_if = "Option::is_none")]
    pub availability_id: Option<String>,
    #[doc = "The frequency at which the product will be billed."]
    #[serde(rename = "billingFrequency", default, skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<String>,
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
    #[doc = "The display name of the product."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The date when the product will be renewed or canceled."]
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[doc = "The ID of the invoice section to which the product is billed."]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "The name of the invoice section to which the product is billed."]
    #[serde(rename = "invoiceSectionDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_display_name: Option<String>,
    #[doc = "The last month charges."]
    #[serde(rename = "lastCharge", default, skip_serializing_if = "Option::is_none")]
    pub last_charge: Option<serde_json::Value>,
    #[doc = "The date of the last charge."]
    #[serde(rename = "lastChargeDate", default, skip_serializing_if = "Option::is_none")]
    pub last_charge_date: Option<String>,
    #[doc = "The description of the type of product."]
    #[serde(rename = "productType", default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[doc = "The ID of the type of product."]
    #[serde(rename = "productTypeId", default, skip_serializing_if = "Option::is_none")]
    pub product_type_id: Option<String>,
    #[doc = "The sku ID of the product."]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "The sku description of the product."]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[doc = "The date when the product was purchased."]
    #[serde(rename = "purchaseDate", default, skip_serializing_if = "Option::is_none")]
    pub purchase_date: Option<String>,
    #[doc = "The quantity purchased for the product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "The status of the product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<product_properties::Status>,
    #[doc = "The id of the tenant in which the product is used."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Reseller for this product. The fields is not available for Microsoft Partner Agreement products."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reseller: Option<serde_json::Value>,
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
    #[doc = "The status of the product."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Other,
        Active,
        Disabled,
        Deleted,
        PastDue,
        Expiring,
        Expired,
        AutoRenew,
        Canceled,
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
                Self::Other => serializer.serialize_unit_variant("Status", 0u32, "Other"),
                Self::Active => serializer.serialize_unit_variant("Status", 1u32, "Active"),
                Self::Disabled => serializer.serialize_unit_variant("Status", 2u32, "Disabled"),
                Self::Deleted => serializer.serialize_unit_variant("Status", 3u32, "Deleted"),
                Self::PastDue => serializer.serialize_unit_variant("Status", 4u32, "PastDue"),
                Self::Expiring => serializer.serialize_unit_variant("Status", 5u32, "Expiring"),
                Self::Expired => serializer.serialize_unit_variant("Status", 6u32, "Expired"),
                Self::AutoRenew => serializer.serialize_unit_variant("Status", 7u32, "AutoRenew"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 8u32, "Canceled"),
                Self::Suspended => serializer.serialize_unit_variant("Status", 9u32, "Suspended"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The status of a transfer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProductTransferStatus")]
pub enum ProductTransferStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProductTransferStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProductTransferStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProductTransferStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotStarted => serializer.serialize_unit_variant("ProductTransferStatus", 0u32, "NotStarted"),
            Self::InProgress => serializer.serialize_unit_variant("ProductTransferStatus", 1u32, "InProgress"),
            Self::Completed => serializer.serialize_unit_variant("ProductTransferStatus", 2u32, "Completed"),
            Self::Failed => serializer.serialize_unit_variant("ProductTransferStatus", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of product that is transferred."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProductType")]
pub enum ProductType {
    AzureSubscription,
    AzureReservation,
    Department,
    SavingsPlan,
    #[serde(rename = "SAAS")]
    Saas,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProductType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProductType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProductType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureSubscription => serializer.serialize_unit_variant("ProductType", 0u32, "AzureSubscription"),
            Self::AzureReservation => serializer.serialize_unit_variant("ProductType", 1u32, "AzureReservation"),
            Self::Department => serializer.serialize_unit_variant("ProductType", 2u32, "Department"),
            Self::SavingsPlan => serializer.serialize_unit_variant("ProductType", 3u32, "SavingsPlan"),
            Self::Saas => serializer.serialize_unit_variant("ProductType", 4u32, "SAAS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    PendingBilling,
    ConfirmedBilling,
    Creating,
    Failed,
    Created,
    Succeeded,
    Canceled,
    Expired,
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
            Self::PendingBilling => serializer.serialize_unit_variant("ProvisioningState", 0u32, "PendingBilling"),
            Self::ConfirmedBilling => serializer.serialize_unit_variant("ProvisioningState", 1u32, "ConfirmedBilling"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Creating"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
            Self::Created => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Created"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Succeeded"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Canceled"),
            Self::Expired => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Expired"),
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
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResourceWithTags {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Dictionary of metadata associated with the resource. It may not be populated for all resource types. Maximum key/value length supported of 256 characters. Keys/value should not empty value nor null. Keys can not contain < > % & \\ ? /"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ProxyResourceWithTags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Purchase request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PurchaseRequest {
    #[doc = "The SKU to be applied for this resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Purchase request properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PurchaseRequestProperties>,
}
impl PurchaseRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Purchase request properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PurchaseRequestProperties {
    #[doc = "Friendly name of the savings plan"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Subscription that will be charged for purchasing SavingsPlan"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[doc = "Represents the Savings plan term in ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<BenefitTerm>,
    #[doc = "Represents the billing plan in ISO 8601 format. Required only for monthly purchases."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<BillingPlan>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "Commitment towards the benefit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[doc = "Setting this to true will automatically purchase a new benefit on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable."]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<AppliedScopeProperties>,
}
impl PurchaseRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The rebill details of an invoice."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RebillDetails {
    #[doc = "The ID of invoice."]
    #[serde(rename = "invoiceDocumentId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_document_id: Option<String>,
    #[doc = "The ID of credit note."]
    #[serde(rename = "creditNoteDocumentId", default, skip_serializing_if = "Option::is_none")]
    pub credit_note_document_id: Option<String>,
    #[doc = "The rebill details of an invoice."]
    #[serde(rename = "rebillDetails", default, skip_serializing_if = "Option::is_none")]
    pub rebill_details: Option<Box<RebillDetails>>,
}
impl RebillDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the transfer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecipientTransferDetails {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "Transfer Details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecipientTransferProperties>,
}
impl RecipientTransferDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of transfers received by caller."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecipientTransferDetailsListResult {
    #[doc = "The list of transfers received by caller."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RecipientTransferDetails>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RecipientTransferDetailsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RecipientTransferDetailsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Transfer Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecipientTransferProperties {
    #[doc = "The time at which the transfer request expires."]
    #[serde(rename = "expirationTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_time: Option<::time::OffsetDateTime>,
    #[doc = "Type of subscriptions that can be transferred."]
    #[serde(
        rename = "allowedProductType",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_product_type: Vec<EligibleProductType>,
    #[doc = "The status of a transfer."]
    #[serde(rename = "transferStatus", default, skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<TransferStatus>,
    #[doc = "The email ID of the user to whom the transfer request was sent."]
    #[serde(rename = "recipientEmailId", default, skip_serializing_if = "Option::is_none")]
    pub recipient_email_id: Option<String>,
    #[doc = "The email ID of the user who sent the transfer request."]
    #[serde(rename = "initiatorEmailId", default, skip_serializing_if = "Option::is_none")]
    pub initiator_email_id: Option<String>,
    #[doc = "Optional MPN ID of the reseller for transfer requests that are sent from a Microsoft Partner Agreement billing account."]
    #[serde(rename = "resellerId", default, skip_serializing_if = "Option::is_none")]
    pub reseller_id: Option<String>,
    #[doc = "Optional name of the reseller for transfer requests that are sent from Microsoft Partner Agreement billing account."]
    #[serde(rename = "resellerName", default, skip_serializing_if = "Option::is_none")]
    pub reseller_name: Option<String>,
    #[doc = "The type of customer of the transfer initiator."]
    #[serde(rename = "initiatorCustomerType", default, skip_serializing_if = "Option::is_none")]
    pub initiator_customer_type: Option<InitiatorCustomerType>,
    #[doc = "The email ID of the user who canceled the transfer request."]
    #[serde(rename = "canceledBy", default, skip_serializing_if = "Option::is_none")]
    pub canceled_by: Option<String>,
    #[doc = "Detailed transfer status."]
    #[serde(
        rename = "detailedTransferStatus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub detailed_transfer_status: Vec<DetailedTransferStatus>,
    #[doc = "The customer tenant id."]
    #[serde(rename = "customerTenantId", default, skip_serializing_if = "Option::is_none")]
    pub customer_tenant_id: Option<String>,
    #[doc = "List of supported account types."]
    #[serde(
        rename = "supportedAccounts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_accounts: Vec<SupportedAccountType>,
}
impl RecipientTransferProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of refund request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefundDetailsSummary {
    #[doc = "Date when the refund was requested."]
    #[serde(rename = "requestedOn", default, with = "azure_core::date::rfc3339::option")]
    pub requested_on: Option<::time::OffsetDateTime>,
    #[doc = "Date when the refund was approved."]
    #[serde(rename = "approvedOn", default, with = "azure_core::date::rfc3339::option")]
    pub approved_on: Option<::time::OffsetDateTime>,
    #[doc = "Date when the refund was completed."]
    #[serde(rename = "completedOn", default, with = "azure_core::date::rfc3339::option")]
    pub completed_on: Option<::time::OffsetDateTime>,
    #[doc = "The amount of refund requested."]
    #[serde(rename = "amountRequested", default, skip_serializing_if = "Option::is_none")]
    pub amount_requested: Option<serde_json::Value>,
    #[doc = "The amount refunded."]
    #[serde(rename = "amountRefunded", default, skip_serializing_if = "Option::is_none")]
    pub amount_refunded: Option<serde_json::Value>,
    #[doc = "The invoice ID of the rebill invoice for a refund."]
    #[serde(rename = "rebillInvoiceId", default, skip_serializing_if = "Option::is_none")]
    pub rebill_invoice_id: Option<String>,
    #[doc = "The number of transactions refunded."]
    #[serde(rename = "transactionCount", default, skip_serializing_if = "Option::is_none")]
    pub transaction_count: Option<i32>,
    #[doc = "The status of refund request."]
    #[serde(rename = "refundStatus", default, skip_serializing_if = "Option::is_none")]
    pub refund_status: Option<refund_details_summary::RefundStatus>,
    #[doc = "The ID of refund operation."]
    #[serde(rename = "refundOperationId", default, skip_serializing_if = "Option::is_none")]
    pub refund_operation_id: Option<String>,
    #[doc = "The reason for refund."]
    #[serde(rename = "refundReason", default, skip_serializing_if = "Option::is_none")]
    pub refund_reason: Option<refund_details_summary::RefundReason>,
}
impl RefundDetailsSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod refund_details_summary {
    use super::*;
    #[doc = "The status of refund request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RefundStatus")]
    pub enum RefundStatus {
        Other,
        Pending,
        Approved,
        Declined,
        Cancelled,
        Completed,
        Expired,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RefundStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RefundStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RefundStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("RefundStatus", 0u32, "Other"),
                Self::Pending => serializer.serialize_unit_variant("RefundStatus", 1u32, "Pending"),
                Self::Approved => serializer.serialize_unit_variant("RefundStatus", 2u32, "Approved"),
                Self::Declined => serializer.serialize_unit_variant("RefundStatus", 3u32, "Declined"),
                Self::Cancelled => serializer.serialize_unit_variant("RefundStatus", 4u32, "Cancelled"),
                Self::Completed => serializer.serialize_unit_variant("RefundStatus", 5u32, "Completed"),
                Self::Expired => serializer.serialize_unit_variant("RefundStatus", 6u32, "Expired"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The reason for refund."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RefundReason")]
    pub enum RefundReason {
        Other,
        AccidentalConversion,
        UnclearPricing,
        AccidentalPurchase,
        ForgotToCancel,
        UnclearDocumentation,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RefundReason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RefundReason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RefundReason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("RefundReason", 0u32, "Other"),
                Self::AccidentalConversion => serializer.serialize_unit_variant("RefundReason", 1u32, "AccidentalConversion"),
                Self::UnclearPricing => serializer.serialize_unit_variant("RefundReason", 2u32, "UnclearPricing"),
                Self::AccidentalPurchase => serializer.serialize_unit_variant("RefundReason", 3u32, "AccidentalPurchase"),
                Self::ForgotToCancel => serializer.serialize_unit_variant("RefundReason", 4u32, "ForgotToCancel"),
                Self::UnclearDocumentation => serializer.serialize_unit_variant("RefundReason", 5u32, "UnclearDocumentation"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The refund details of a transaction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefundTransactionDetails {
    #[doc = "The amount of refund requested."]
    #[serde(rename = "amountRequested", default, skip_serializing_if = "Option::is_none")]
    pub amount_requested: Option<serde_json::Value>,
    #[doc = "The amount refunded."]
    #[serde(rename = "amountRefunded", default, skip_serializing_if = "Option::is_none")]
    pub amount_refunded: Option<serde_json::Value>,
    #[doc = "The ID of refund operation."]
    #[serde(rename = "refundOperationId", default, skip_serializing_if = "Option::is_none")]
    pub refund_operation_id: Option<String>,
}
impl RefundTransactionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the registration number of the organization linked with the billing account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationNumber {
    #[doc = "The unique identification number of the organization linked with the billing account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Identifies if the registration number is required for the billing account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[doc = "The types of registration number allowed based on the country of the billing account."]
    #[serde(
        rename = "type",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub type_: Vec<String>,
}
impl RegistrationNumber {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Renew = bool;
pub type RenewDestination = String;
#[doc = "Properties specific to renew."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RenewProperties {
    #[doc = "Purchase request."]
    #[serde(rename = "purchaseProperties", default, skip_serializing_if = "Option::is_none")]
    pub purchase_properties: Option<PurchaseRequest>,
}
impl RenewProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The renew properties for a reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RenewPropertiesResponse {
    #[doc = "The request for reservation purchase"]
    #[serde(rename = "purchaseProperties", default, skip_serializing_if = "Option::is_none")]
    pub purchase_properties: Option<ReservationPurchaseRequest>,
    #[doc = "The price."]
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<Price>,
    #[doc = "The price."]
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<Price>,
}
impl RenewPropertiesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type RenewSource = String;
#[doc = "Details for the next renewal term of a subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RenewalTermDetails {
    #[doc = "The billing frequency in ISO8601 format of product in the subscription. Example: P1M, P3M, P1Y"]
    #[serde(rename = "billingFrequency", default, skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<String>,
    #[doc = "Id of the product for which the subscription is purchased."]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[doc = "Type Id of the product for which the subscription is purchased."]
    #[serde(rename = "productTypeId", default, skip_serializing_if = "Option::is_none")]
    pub product_type_id: Option<String>,
    #[doc = "The SKU ID of the product for which the subscription is purchased. This field is is only available  for Microsoft Customer Agreement billing accounts."]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "The duration in ISO8601 format for which you can use the subscription. Example: P1M, P3M, P1Y"]
    #[serde(rename = "termDuration", default, skip_serializing_if = "Option::is_none")]
    pub term_duration: Option<String>,
    #[doc = "The quantity of licenses or fulfillment units for the subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "End date of the term in UTC time."]
    #[serde(rename = "termEndDate", default, with = "azure_core::date::rfc3339::option")]
    pub term_end_date: Option<::time::OffsetDateTime>,
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
#[doc = "The definition of the reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Reservation {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The location of the reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i32>,
    #[doc = "The property of reservation object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationProperty>,
    #[doc = "Resource Tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "The property of reservation sku object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ReservationSkuProperty>,
}
impl Reservation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ReservationAppliedScope = String;
#[doc = "Properties specific to applied scope type. Not required if not applicable. Required and need to provide tenantId and managementGroupId if AppliedScopeType is ManagementGroup"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationAppliedScopeProperties {
    #[doc = "Tenant ID where the reservation should apply benefit."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Fully-qualified identifier of the management group where the benefit must be applied."]
    #[serde(rename = "managementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub management_group_id: Option<String>,
    #[doc = "Fully-qualified identifier of the subscription."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Fully-qualified identifier of the resource group."]
    #[serde(rename = "resourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<String>,
    #[doc = "Display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl ReservationAppliedScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represent the billing plans."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReservationBillingPlan")]
pub enum ReservationBillingPlan {
    Upfront,
    Monthly,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReservationBillingPlan {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReservationBillingPlan {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReservationBillingPlan {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Upfront => serializer.serialize_unit_variant("ReservationBillingPlan", 0u32, "Upfront"),
            Self::Monthly => serializer.serialize_unit_variant("ReservationBillingPlan", 1u32, "Monthly"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Extended status information for the reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationExtendedStatusInfo {
    #[doc = "The status of the reservation."]
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<ReservationStatusCode>,
    #[doc = "The message giving detailed information about the status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Extended status definition properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExtendedStatusDefinitionProperties>,
}
impl ReservationExtendedStatusInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of `Reservations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Reservation>,
    #[doc = "Url to get the next page of reservations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReservationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ReservationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of reservation merge"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationMergeProperties {
    #[doc = "Reservation resource id Created due to the merge. Format of the resource id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "mergeDestination", default, skip_serializing_if = "Option::is_none")]
    pub merge_destination: Option<String>,
    #[doc = "Resource ids of the source reservation's merged to form this reservation. Format of the resource id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(
        rename = "mergeSources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub merge_sources: Vec<String>,
}
impl ReservationMergeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of a reservation order being returned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrder {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i32>,
    #[doc = "Properties of a reservation order."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationOrderProperty>,
    #[doc = "Resource Tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
impl ReservationOrder {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information describing the type of billing plan for this reservation order."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderBillingPlanInformation {
    #[doc = "The price."]
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<Price>,
    #[doc = "Date when the billing plan has started."]
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[doc = "For recurring billing plans, indicates the date when next payment will be processed. Null when total is paid off."]
    #[serde(rename = "nextPaymentDueDate", default, skip_serializing_if = "Option::is_none")]
    pub next_payment_due_date: Option<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transactions: Vec<ReservationPaymentDetail>,
}
impl ReservationOrderBillingPlanInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of ReservationOrders"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ReservationOrder>,
    #[doc = "Url to get the next page of reservationOrders."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReservationOrderList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ReservationOrderList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a reservation order."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderProperty {
    #[doc = "Friendly name for user to easily identified the reservation order."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Enrollment id of the reservation order."]
    #[serde(rename = "enrollmentId", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_id: Option<String>,
    #[doc = "Fully-qualified identifier of the customerId where the benefit is applied. Present only for Enterprise Agreement PartnerLed customers."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "Billing profile Id associated to this reservation order."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "Billing account Id associated to this reservation order."]
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<String>,
    #[doc = "This is the DateTime when the reservation order was initially requested for purchase."]
    #[serde(rename = "requestDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub request_date_time: Option<::time::OffsetDateTime>,
    #[doc = "This is the DateTime when the reservation order was created."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<::time::OffsetDateTime>,
    #[doc = "This is the date when the reservation order will expire."]
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[doc = "This is the date-time when the reservation order will expire."]
    #[serde(rename = "expiryDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiry_date_time: Option<::time::OffsetDateTime>,
    #[doc = "This is the DateTime when the reservation benefit started."]
    #[serde(rename = "benefitStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub benefit_start_time: Option<::time::OffsetDateTime>,
    #[doc = "Total original quantity of the skus purchased in the reservation order."]
    #[serde(rename = "originalQuantity", default, skip_serializing_if = "Option::is_none")]
    pub original_quantity: Option<i32>,
    #[doc = "The term of the reservation, e.g. P1Y"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[doc = "The provisioning state of the reservation, e.g. Succeeded"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Represent the billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[doc = "Information describing the type of billing plan for this reservation order."]
    #[serde(rename = "planInformation", default, skip_serializing_if = "Option::is_none")]
    pub plan_information: Option<ReservationOrderBillingPlanInformation>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reservations: Vec<Reservation>,
    #[doc = "This is the date-time when the Azure Hybrid Benefit needs to be reviewed."]
    #[serde(rename = "reviewDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub review_date_time: Option<::time::OffsetDateTime>,
    #[doc = "Extended status information for the reservation."]
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ReservationExtendedStatusInfo>,
    #[doc = "Represents UPN"]
    #[serde(rename = "productCode", default, skip_serializing_if = "Option::is_none")]
    pub product_code: Option<ProductCode>,
}
impl ReservationOrderProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about payment related to a reservation order."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationPaymentDetail {
    #[doc = "Date when the payment needs to be done."]
    #[serde(rename = "dueDate", default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[doc = "Date when the transaction is completed. Is null when it is scheduled."]
    #[serde(rename = "paymentDate", default, skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<String>,
    #[doc = "The price."]
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<Price>,
    #[doc = "The price."]
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<Price>,
    #[doc = "Shows the Account that is charged for this payment."]
    #[serde(rename = "billingAccount", default, skip_serializing_if = "Option::is_none")]
    pub billing_account: Option<String>,
    #[doc = "Describes whether the payment is completed, failed, cancelled or scheduled in the future."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentStatus>,
    #[doc = "Extended status information for the reservation."]
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ReservationExtendedStatusInfo>,
}
impl ReservationPaymentDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The property of reservation object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationProperty {
    #[doc = "The reserved source type of the reservation, e.g. virtual machine."]
    #[serde(rename = "reservedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<String>,
    #[doc = "Turning this on will apply the reservation discount to other VMs in the same VM size group. Only specify for VirtualMachines reserved resource type."]
    #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[doc = "The display name of the reservation"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The array of applied scopes of a reservation. Will be null if the reservation is in Shared scope"]
    #[serde(
        rename = "appliedScopes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub applied_scopes: Vec<ReservationAppliedScope>,
    #[doc = "The applied scope type of the reservation."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<String>,
    #[doc = "Indicates if the reservation is archived"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[doc = "Capabilities of the reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<String>,
    #[doc = "The number of the reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[doc = "The provisioning state of the reservation, e.g. Succeeded"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The effective date time of the reservation"]
    #[serde(rename = "effectiveDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub effective_date_time: Option<::time::OffsetDateTime>,
    #[doc = "This is the DateTime when the reservation benefit started."]
    #[serde(rename = "benefitStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub benefit_start_time: Option<::time::OffsetDateTime>,
    #[doc = "DateTime of the last time the reservation was updated."]
    #[serde(rename = "lastUpdatedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The expiry date of the reservation"]
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[doc = "This is the date-time when the reservation will expire."]
    #[serde(rename = "expiryDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiry_date_time: Option<::time::OffsetDateTime>,
    #[doc = "This is the date-time when the Azure Hybrid Benefit needs to be reviewed."]
    #[serde(rename = "reviewDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub review_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The sku description of the reservation"]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[doc = "Extended status information for the reservation."]
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ReservationExtendedStatusInfo>,
    #[doc = "Represent the billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[doc = "The provisioning state of the reservation for display, e.g. Succeeded"]
    #[serde(rename = "displayProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub display_provisioning_state: Option<String>,
    #[doc = "The provisioning state of the reservation, e.g. Succeeded"]
    #[serde(rename = "provisioningSubState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_sub_state: Option<String>,
    #[doc = "This is the date when the reservation was purchased."]
    #[serde(rename = "purchaseDate", default, skip_serializing_if = "Option::is_none")]
    pub purchase_date: Option<String>,
    #[doc = "This is the date-time when the reservation was purchased."]
    #[serde(rename = "purchaseDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub purchase_date_time: Option<::time::OffsetDateTime>,
    #[doc = "Properties of reservation split"]
    #[serde(rename = "splitProperties", default, skip_serializing_if = "Option::is_none")]
    pub split_properties: Option<ReservationSplitProperties>,
    #[doc = "Properties of reservation merge"]
    #[serde(rename = "mergeProperties", default, skip_serializing_if = "Option::is_none")]
    pub merge_properties: Option<ReservationMergeProperties>,
    #[doc = "Properties of reservation swap"]
    #[serde(rename = "swapProperties", default, skip_serializing_if = "Option::is_none")]
    pub swap_properties: Option<ReservationSwapProperties>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable. Required and need to provide tenantId and managementGroupId if AppliedScopeType is ManagementGroup"]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<ReservationAppliedScopeProperties>,
    #[doc = "Subscription that will be charged for purchasing reservation or savings plan"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<String>,
    #[doc = "The renew state of the reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<bool>,
    #[doc = "The renew source of the reservation"]
    #[serde(rename = "renewSource", default, skip_serializing_if = "Option::is_none")]
    pub renew_source: Option<String>,
    #[doc = "Reservation Id of the reservation which is purchased because of renew. Format of the resource Id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}."]
    #[serde(rename = "renewDestination", default, skip_serializing_if = "Option::is_none")]
    pub renew_destination: Option<String>,
    #[doc = "The renew properties for a reservation."]
    #[serde(rename = "renewProperties", default, skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<RenewPropertiesResponse>,
    #[doc = "The term of the reservation, e.g. P1Y"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[doc = "The applied scope type of the reservation for display, e.g. Shared"]
    #[serde(rename = "userFriendlyAppliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub user_friendly_applied_scope_type: Option<String>,
    #[doc = "The renew state of the reservation for display, e.g. On"]
    #[serde(rename = "userFriendlyRenewState", default, skip_serializing_if = "Option::is_none")]
    pub user_friendly_renew_state: Option<String>,
    #[doc = "Reservation utilization"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utilization: Option<reservation_property::Utilization>,
    #[doc = "Represents UPN"]
    #[serde(rename = "productCode", default, skip_serializing_if = "Option::is_none")]
    pub product_code: Option<ProductCode>,
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
        #[doc = "last 7 day utilization trend for a reservation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub trend: Option<String>,
        #[doc = "The array of aggregates of a reservation's utilization"]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub aggregates: Vec<ReservationUtilizationAggregates>,
    }
    impl Utilization {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The request for reservation purchase"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationPurchaseRequest {
    #[doc = "The name of sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuName>,
    #[doc = "The Azure region where the reserved resource lives."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties of reservation purchase request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationPurchaseRequestProperties>,
}
impl ReservationPurchaseRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of reservation purchase request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationPurchaseRequestProperties {
    #[doc = "The reserved source type of the reservation, e.g. virtual machine."]
    #[serde(rename = "reservedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<String>,
    #[doc = "Subscription that will be charged for purchasing reservation or savings plan"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<String>,
    #[doc = "The term of the reservation, e.g. P1Y"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[doc = "Represent the billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[doc = "Quantity of the skus that are part of the reservation. Must be greater than zero."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[doc = "Friendly name of the reservation"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "List of the subscriptions that the benefit will be applied. Do not specify if AppliedScopeType is Shared."]
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable. Required and need to provide tenantId and managementGroupId if AppliedScopeType is ManagementGroup"]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<ReservationAppliedScopeProperties>,
    #[doc = "Setting this to true will automatically purchase a new reservation on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[doc = "Properties specific to each reserved resource type. Not required if not applicable."]
    #[serde(rename = "reservedResourceProperties", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_properties: Option<reservation_purchase_request_properties::ReservedResourceProperties>,
    #[doc = "Turning this on will apply the reservation discount to other VMs in the same VM size group. Only specify for VirtualMachines reserved resource type."]
    #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[doc = "This is the date-time when the Azure hybrid benefit needs to be reviewed."]
    #[serde(rename = "reviewDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub review_date_time: Option<::time::OffsetDateTime>,
}
impl ReservationPurchaseRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod reservation_purchase_request_properties {
    use super::*;
    #[doc = "Properties specific to each reserved resource type. Not required if not applicable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ReservedResourceProperties {
        #[doc = "Turning this on will apply the reservation discount to other VMs in the same VM size group. Only specify for VirtualMachines reserved resource type."]
        #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
        pub instance_flexibility: Option<InstanceFlexibility>,
    }
    impl ReservedResourceProperties {
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
#[doc = "Properties of reservation split"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSplitProperties {
    #[doc = "List of destination resource id that are created due to split. Format of the resource id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(
        rename = "splitDestinations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub split_destinations: Vec<String>,
    #[doc = "Resource id of the reservation from which this is split. Format of the resource id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "splitSource", default, skip_serializing_if = "Option::is_none")]
    pub split_source: Option<String>,
}
impl ReservationSplitProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of the reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReservationStatusCode")]
pub enum ReservationStatusCode {
    None,
    Pending,
    Processing,
    Active,
    PurchaseError,
    PaymentInstrumentError,
    Split,
    Merged,
    Expired,
    Succeeded,
    CapacityError,
    CapacityRestricted,
    Exchanged,
    UnknownError,
    RiskCheckFailed,
    CreditLineCheckFailed,
    Warning,
    NoBenefitDueToSubscriptionTransfer,
    NoBenefitDueToSubscriptionDeletion,
    NoBenefit,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReservationStatusCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReservationStatusCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReservationStatusCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ReservationStatusCode", 0u32, "None"),
            Self::Pending => serializer.serialize_unit_variant("ReservationStatusCode", 1u32, "Pending"),
            Self::Processing => serializer.serialize_unit_variant("ReservationStatusCode", 2u32, "Processing"),
            Self::Active => serializer.serialize_unit_variant("ReservationStatusCode", 3u32, "Active"),
            Self::PurchaseError => serializer.serialize_unit_variant("ReservationStatusCode", 4u32, "PurchaseError"),
            Self::PaymentInstrumentError => serializer.serialize_unit_variant("ReservationStatusCode", 5u32, "PaymentInstrumentError"),
            Self::Split => serializer.serialize_unit_variant("ReservationStatusCode", 6u32, "Split"),
            Self::Merged => serializer.serialize_unit_variant("ReservationStatusCode", 7u32, "Merged"),
            Self::Expired => serializer.serialize_unit_variant("ReservationStatusCode", 8u32, "Expired"),
            Self::Succeeded => serializer.serialize_unit_variant("ReservationStatusCode", 9u32, "Succeeded"),
            Self::CapacityError => serializer.serialize_unit_variant("ReservationStatusCode", 10u32, "CapacityError"),
            Self::CapacityRestricted => serializer.serialize_unit_variant("ReservationStatusCode", 11u32, "CapacityRestricted"),
            Self::Exchanged => serializer.serialize_unit_variant("ReservationStatusCode", 12u32, "Exchanged"),
            Self::UnknownError => serializer.serialize_unit_variant("ReservationStatusCode", 13u32, "UnknownError"),
            Self::RiskCheckFailed => serializer.serialize_unit_variant("ReservationStatusCode", 14u32, "RiskCheckFailed"),
            Self::CreditLineCheckFailed => serializer.serialize_unit_variant("ReservationStatusCode", 15u32, "CreditLineCheckFailed"),
            Self::Warning => serializer.serialize_unit_variant("ReservationStatusCode", 16u32, "Warning"),
            Self::NoBenefitDueToSubscriptionTransfer => {
                serializer.serialize_unit_variant("ReservationStatusCode", 17u32, "NoBenefitDueToSubscriptionTransfer")
            }
            Self::NoBenefitDueToSubscriptionDeletion => {
                serializer.serialize_unit_variant("ReservationStatusCode", 18u32, "NoBenefitDueToSubscriptionDeletion")
            }
            Self::NoBenefit => serializer.serialize_unit_variant("ReservationStatusCode", 19u32, "NoBenefit"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The roll up count summary of reservations in each state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSummary {
    #[doc = "The number of reservation in Cancelled state"]
    #[serde(rename = "cancelledCount", default, skip_serializing_if = "Option::is_none")]
    pub cancelled_count: Option<f64>,
    #[doc = "The number of reservation in Expired state"]
    #[serde(rename = "expiredCount", default, skip_serializing_if = "Option::is_none")]
    pub expired_count: Option<f64>,
    #[doc = "The number of reservation in Expiring state"]
    #[serde(rename = "expiringCount", default, skip_serializing_if = "Option::is_none")]
    pub expiring_count: Option<f64>,
    #[doc = "The number of reservation in Failed state"]
    #[serde(rename = "failedCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<f64>,
    #[doc = "The number of reservation in Pending state"]
    #[serde(rename = "pendingCount", default, skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<f64>,
    #[doc = "The number of reservation in Succeeded state"]
    #[serde(rename = "succeededCount", default, skip_serializing_if = "Option::is_none")]
    pub succeeded_count: Option<f64>,
    #[doc = "The number of reservation in 'No Benefit' state"]
    #[serde(rename = "noBenefitCount", default, skip_serializing_if = "Option::is_none")]
    pub no_benefit_count: Option<f64>,
    #[doc = "The number of reservation in Warning state"]
    #[serde(rename = "warningCount", default, skip_serializing_if = "Option::is_none")]
    pub warning_count: Option<f64>,
    #[doc = "The number of reservation in Processing state"]
    #[serde(rename = "processingCount", default, skip_serializing_if = "Option::is_none")]
    pub processing_count: Option<f64>,
}
impl ReservationSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of reservation swap"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSwapProperties {
    #[doc = "Resource id of the source reservation that gets swapped. Format of the resource id is /providers/microsoft.capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "swapSource", default, skip_serializing_if = "Option::is_none")]
    pub swap_source: Option<String>,
    #[doc = "Reservation resource id that the original resource gets swapped to. Format of the resource id is /providers/microsoft.capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "swapDestination", default, skip_serializing_if = "Option::is_none")]
    pub swap_destination: Option<String>,
}
impl ReservationSwapProperties {
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
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The roll up count summary of reservations in each state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReservationSummary>,
    #[doc = "The list of reservations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Reservation>,
}
impl azure_core::Continuable for ReservationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ReservationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ResourceGroupId = String;
pub type SavingsPlanId = String;
#[doc = "Savings plan"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavingsPlanModel {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "The SKU to be applied for this resource"]
    pub sku: Sku,
    #[doc = "Savings plan properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SavingsPlanModelProperties>,
}
impl SavingsPlanModel {
    pub fn new(sku: Sku) -> Self {
        Self {
            proxy_resource_with_tags: ProxyResourceWithTags::default(),
            sku,
            properties: None,
        }
    }
}
#[doc = "List of savings plans"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanModelList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SavingsPlanModel>,
    #[doc = "Url to get the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SavingsPlanModelList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SavingsPlanModelList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of savings plans"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanModelListResult {
    #[serde(flatten)]
    pub savings_plan_model_list: SavingsPlanModelList,
    #[doc = "The roll up count summary of savings plans in each state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<SavingsPlanSummaryCount>,
}
impl azure_core::Continuable for SavingsPlanModelListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SavingsPlanModelListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanModelProperties {
    #[doc = "Display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<DisplayName>,
    #[doc = "Provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The provisioning state of the savings plan for display, e.g. Succeeded"]
    #[serde(rename = "displayProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub display_provisioning_state: Option<String>,
    #[doc = "The applied scope type of the savings plan for display, e.g. Shared"]
    #[serde(rename = "userFriendlyAppliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub user_friendly_applied_scope_type: Option<String>,
    #[doc = "Subscription that will be charged for purchasing SavingsPlan"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[doc = "Fully-qualified identifier of the billing profile where the savings plan is applied. Present only for Field-led or Customer-led customers."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<BillingProfileId>,
    #[doc = "Fully-qualified identifier of the customer where the savings plan is applied. Present only for Partner-led customers."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    #[doc = "Fully-qualified identifier of the billing account where the savings plan is applied."]
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<BillingAccountId>,
    #[doc = "Represents the Savings plan term in ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<BenefitTerm>,
    #[doc = "Setting this to true will automatically purchase a new benefit on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[doc = "SavingsPlan Id of the SavingsPlan from which this SavingsPlan is renewed."]
    #[serde(rename = "renewSource", default, skip_serializing_if = "Option::is_none")]
    pub renew_source: Option<RenewSource>,
    #[doc = "SavingsPlan Id of the SavingsPlan which is purchased because of renew."]
    #[serde(rename = "renewDestination", default, skip_serializing_if = "Option::is_none")]
    pub renew_destination: Option<RenewDestination>,
    #[doc = "Properties specific to renew."]
    #[serde(rename = "renewProperties", default, skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<RenewProperties>,
    #[doc = "Represents the billing plan in ISO 8601 format. Required only for monthly purchases."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<BillingPlan>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable."]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<AppliedScopeProperties>,
    #[doc = "Commitment towards the benefit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[doc = "DateTime of the savings plan starting when this version is effective from."]
    #[serde(rename = "effectiveDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub effective_date_time: Option<::time::OffsetDateTime>,
    #[doc = "This is the DateTime when the savings plan benefit starts."]
    #[serde(rename = "benefitStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub benefit_start_time: Option<::time::OffsetDateTime>,
    #[doc = "This is the date-time when the savings plan will expire."]
    #[serde(rename = "expiryDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiry_date_time: Option<::time::OffsetDateTime>,
    #[doc = "Date time when the savings plan was purchased."]
    #[serde(rename = "purchaseDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub purchase_date_time: Option<::time::OffsetDateTime>,
    #[doc = "Savings plan utilization"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utilization: Option<Utilization>,
    #[doc = "Extended status information"]
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
    #[doc = "Represents UPN"]
    #[serde(rename = "productCode", default, skip_serializing_if = "Option::is_none")]
    pub product_code: Option<ProductCode>,
}
impl SavingsPlanModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan order"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavingsPlanOrderModel {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "The SKU to be applied for this resource"]
    pub sku: Sku,
    #[doc = "Savings plan order properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SavingsPlanOrderModelProperties>,
}
impl SavingsPlanOrderModel {
    pub fn new(sku: Sku) -> Self {
        Self {
            proxy_resource_with_tags: ProxyResourceWithTags::default(),
            sku,
            properties: None,
        }
    }
}
#[doc = "List of savings plan orders"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanOrderModelList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SavingsPlanOrderModel>,
    #[doc = "Url to get the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SavingsPlanOrderModelList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SavingsPlanOrderModelList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan order properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanOrderModelProperties {
    #[doc = "Display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<DisplayName>,
    #[doc = "The provisioning state of the savings plan, e.g. Succeeded"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Subscription that will be charged for purchasing SavingsPlan"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[doc = "Fully-qualified identifier of the billing profile where the savings plan is applied. Present only for Field-led or Customer-led customers."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<BillingProfileId>,
    #[doc = "Fully-qualified identifier of the customer where the savings plan is applied. Present only for Partner-led customers."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    #[doc = "Fully-qualified identifier of the billing account where the savings plan is applied."]
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<BillingAccountId>,
    #[doc = "Represents the Savings plan term in ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<BenefitTerm>,
    #[doc = "Represents the billing plan in ISO 8601 format. Required only for monthly purchases."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<BillingPlan>,
    #[doc = "DateTime when the savings plan benefit started."]
    #[serde(rename = "benefitStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub benefit_start_time: Option<::time::OffsetDateTime>,
    #[doc = "DateTime when the savings plan will expire."]
    #[serde(rename = "expiryDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiry_date_time: Option<::time::OffsetDateTime>,
    #[doc = "Information describing the type of billing plan for this savings plan."]
    #[serde(rename = "planInformation", default, skip_serializing_if = "Option::is_none")]
    pub plan_information: Option<BillingPlanInformation>,
    #[serde(
        rename = "savingsPlans",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub savings_plans: Vec<SavingsPlanId>,
    #[doc = "Extended status information"]
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
    #[doc = "Represents UPN"]
    #[serde(rename = "productCode", default, skip_serializing_if = "Option::is_none")]
    pub product_code: Option<ProductCode>,
}
impl SavingsPlanOrderModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The roll up count summary of savings plans in each state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanSummaryCount {
    #[doc = "The number of savings plans in Succeeded state"]
    #[serde(rename = "succeededCount", default, skip_serializing_if = "Option::is_none")]
    pub succeeded_count: Option<f64>,
    #[doc = "The number of savings plans in Failed state"]
    #[serde(rename = "failedCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<f64>,
    #[doc = "The number of savings plans in Expiring state"]
    #[serde(rename = "expiringCount", default, skip_serializing_if = "Option::is_none")]
    pub expiring_count: Option<f64>,
    #[doc = "The number of savings plans in Expired state"]
    #[serde(rename = "expiredCount", default, skip_serializing_if = "Option::is_none")]
    pub expired_count: Option<f64>,
    #[doc = "The number of savings plans in Pending state"]
    #[serde(rename = "pendingCount", default, skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<f64>,
    #[doc = "The number of savings plans in Cancelled state"]
    #[serde(rename = "cancelledCount", default, skip_serializing_if = "Option::is_none")]
    pub cancelled_count: Option<f64>,
    #[doc = "The number of savings plans in Processing state"]
    #[serde(rename = "processingCount", default, skip_serializing_if = "Option::is_none")]
    pub processing_count: Option<f64>,
    #[doc = "The number of savings plans in No Benefit state"]
    #[serde(rename = "noBenefitCount", default, skip_serializing_if = "Option::is_none")]
    pub no_benefit_count: Option<f64>,
    #[doc = "The number of savings plans in Warning state"]
    #[serde(rename = "warningCount", default, skip_serializing_if = "Option::is_none")]
    pub warning_count: Option<f64>,
}
impl SavingsPlanSummaryCount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan patch request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanUpdateRequest {
    #[doc = "Savings plan patch request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SavingsPlanUpdateRequestProperties>,
    #[doc = "The SKU to be applied for this resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Resource Tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
impl SavingsPlanUpdateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan patch request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanUpdateRequestProperties {
    #[doc = "Display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<DisplayName>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable."]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<AppliedScopeProperties>,
    #[doc = "Setting this to true will automatically purchase a new benefit on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[doc = "Properties specific to renew."]
    #[serde(rename = "renewProperties", default, skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<RenewProperties>,
}
impl SavingsPlanUpdateRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan update validate request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanUpdateValidateRequest {
    #[doc = "The benefits of a savings plan."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub benefits: Vec<SavingsPlanUpdateRequestProperties>,
}
impl SavingsPlanUpdateValidateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Benefit scope response property"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanValidResponseProperty {
    #[doc = "Indicates if the provided input is valid"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[doc = "Failure reason code if the provided input is invalid"]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[doc = "Failure reason if the provided input is invalid"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl SavingsPlanValidResponseProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan update validate response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanValidateResponse {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub benefits: Vec<SavingsPlanValidResponseProperty>,
    #[doc = "Url to get the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SavingsPlanValidateResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SKU to be applied for this resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "Name of the SKU to be applied"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The name of sku"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl SkuName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The billing profile spending limit."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpendingLimitDetails {
    #[doc = "The initial amount for the billing profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "The currency in which the charges for the billing profile are billed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The date when this spending limit goes into effect."]
    #[serde(rename = "startDate", default, with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<::time::OffsetDateTime>,
    #[doc = "The date when this spending limit is no longer in effect."]
    #[serde(rename = "endDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<::time::OffsetDateTime>,
    #[doc = "The type of spending limit."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<spending_limit_details::Type>,
    #[doc = "The status of current spending limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<spending_limit_details::Status>,
}
impl SpendingLimitDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod spending_limit_details {
    use super::*;
    #[doc = "The type of spending limit."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Other,
        None,
        FreeAccount,
        Sandbox,
        AzureForStudents,
        AcademicSponsorship,
        AzureConsumptionCredit,
        AzurePassSponsorship,
        MpnSponsorship,
        #[serde(rename = "MSDN")]
        Msdn,
        NonProfitSponsorship,
        Sponsorship,
        StartupSponsorship,
        AzureForStudentsStarter,
        VisualStudio,
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
                Self::Other => serializer.serialize_unit_variant("Type", 0u32, "Other"),
                Self::None => serializer.serialize_unit_variant("Type", 1u32, "None"),
                Self::FreeAccount => serializer.serialize_unit_variant("Type", 2u32, "FreeAccount"),
                Self::Sandbox => serializer.serialize_unit_variant("Type", 3u32, "Sandbox"),
                Self::AzureForStudents => serializer.serialize_unit_variant("Type", 4u32, "AzureForStudents"),
                Self::AcademicSponsorship => serializer.serialize_unit_variant("Type", 5u32, "AcademicSponsorship"),
                Self::AzureConsumptionCredit => serializer.serialize_unit_variant("Type", 6u32, "AzureConsumptionCredit"),
                Self::AzurePassSponsorship => serializer.serialize_unit_variant("Type", 7u32, "AzurePassSponsorship"),
                Self::MpnSponsorship => serializer.serialize_unit_variant("Type", 8u32, "MpnSponsorship"),
                Self::Msdn => serializer.serialize_unit_variant("Type", 9u32, "MSDN"),
                Self::NonProfitSponsorship => serializer.serialize_unit_variant("Type", 10u32, "NonProfitSponsorship"),
                Self::Sponsorship => serializer.serialize_unit_variant("Type", 11u32, "Sponsorship"),
                Self::StartupSponsorship => serializer.serialize_unit_variant("Type", 12u32, "StartupSponsorship"),
                Self::AzureForStudentsStarter => serializer.serialize_unit_variant("Type", 13u32, "AzureForStudentsStarter"),
                Self::VisualStudio => serializer.serialize_unit_variant("Type", 14u32, "VisualStudio"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of current spending limit."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Other,
        None,
        Active,
        Expired,
        LimitReached,
        LimitRemoved,
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
                Self::Other => serializer.serialize_unit_variant("Status", 0u32, "Other"),
                Self::None => serializer.serialize_unit_variant("Status", 1u32, "None"),
                Self::Active => serializer.serialize_unit_variant("Status", 2u32, "Active"),
                Self::Expired => serializer.serialize_unit_variant("Status", 3u32, "Expired"),
                Self::LimitReached => serializer.serialize_unit_variant("Status", 4u32, "LimitReached"),
                Self::LimitRemoved => serializer.serialize_unit_variant("Status", 5u32, "LimitRemoved"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The enrollment details for the subscription. Available for billing accounts with agreement type Enterprise Agreement."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionEnrollmentDetails {
    #[doc = "The name of the department"]
    #[serde(rename = "departmentDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub department_display_name: Option<String>,
    #[doc = "The ID that uniquely identifies the department."]
    #[serde(rename = "departmentId", default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[doc = "The status of the enrollment account."]
    #[serde(rename = "enrollmentAccountStatus", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_account_status: Option<String>,
    #[doc = "The name of the enrollment account."]
    #[serde(rename = "enrollmentAccountDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_account_display_name: Option<String>,
    #[doc = "The ID that uniquely identifies an enrollment account."]
    #[serde(rename = "enrollmentAccountId", default, skip_serializing_if = "Option::is_none")]
    pub enrollment_account_id: Option<String>,
}
impl SubscriptionEnrollmentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type SubscriptionId = String;
#[doc = "A policy at subscription scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionPolicy {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "A policy at subscription scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionPolicyProperties>,
}
impl SubscriptionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A policy at subscription scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionPolicyProperties {
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<subscription_policy_properties::ProvisioningState>,
    #[doc = "List of all policies defined at the billing scope."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policies: Vec<PolicySummary>,
}
impl SubscriptionPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_policy_properties {
    use super::*;
    #[doc = "The provisioning state of the resource during a long-running operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Canceled,
        Failed,
        New,
        Pending,
        Provisioning,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::New => serializer.serialize_unit_variant("ProvisioningState", 3u32, "New"),
                Self::Pending => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Pending"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The supported account types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SupportedAccountType")]
pub enum SupportedAccountType {
    None,
    Partner,
    Individual,
    Enterprise,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SupportedAccountType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SupportedAccountType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SupportedAccountType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("SupportedAccountType", 0u32, "None"),
            Self::Partner => serializer.serialize_unit_variant("SupportedAccountType", 1u32, "Partner"),
            Self::Individual => serializer.serialize_unit_variant("SupportedAccountType", 2u32, "Individual"),
            Self::Enterprise => serializer.serialize_unit_variant("SupportedAccountType", 3u32, "Enterprise"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "System imposed policies that regulate behavior of the subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemOverrides {
    #[doc = "The policy override for the subscription indicates whether the self-serve cancellation or seat reduction is allowed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation: Option<system_overrides::Cancellation>,
    #[doc = "The end date in UTC time by when the self-serve cancellation ends."]
    #[serde(rename = "cancellationAllowedEndDate", default, with = "azure_core::date::rfc3339::option")]
    pub cancellation_allowed_end_date: Option<::time::OffsetDateTime>,
}
impl SystemOverrides {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_overrides {
    use super::*;
    #[doc = "The policy override for the subscription indicates whether the self-serve cancellation or seat reduction is allowed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Cancellation")]
    pub enum Cancellation {
        NotAllowed,
        Allowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Cancellation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Cancellation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Cancellation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotAllowed => serializer.serialize_unit_variant("Cancellation", 0u32, "NotAllowed"),
                Self::Allowed => serializer.serialize_unit_variant("Cancellation", 1u32, "Allowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Resource Tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A tax identifier for the billing account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaxIdentifier {
    #[doc = "The id of the tax identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the tax identifier."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<tax_identifier::Type>,
    #[doc = "The scope of the tax identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The country of the tax identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The status of the tax identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<tax_identifier::Status>,
}
impl TaxIdentifier {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod tax_identifier {
    use super::*;
    #[doc = "The type of the tax identifier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Other,
        BrazilCcmId,
        BrazilCnpjId,
        BrazilCpfId,
        CanadianFederalExempt,
        CanadianProvinceExempt,
        ExternalTaxation,
        IndiaFederalTanId,
        IndiaFederalServiceTaxId,
        IndiaPanId,
        IndiaStateCstId,
        #[serde(rename = "IndiaStateGstINId")]
        IndiaStateGstInId,
        IndiaStateVatId,
        IntlExempt,
        #[serde(rename = "USExempt")]
        UsExempt,
        VatId,
        LoveCode,
        MobileBarCode,
        NationalIdentificationNumber,
        PublicSectorId,
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
                Self::Other => serializer.serialize_unit_variant("Type", 0u32, "Other"),
                Self::BrazilCcmId => serializer.serialize_unit_variant("Type", 1u32, "BrazilCcmId"),
                Self::BrazilCnpjId => serializer.serialize_unit_variant("Type", 2u32, "BrazilCnpjId"),
                Self::BrazilCpfId => serializer.serialize_unit_variant("Type", 3u32, "BrazilCpfId"),
                Self::CanadianFederalExempt => serializer.serialize_unit_variant("Type", 4u32, "CanadianFederalExempt"),
                Self::CanadianProvinceExempt => serializer.serialize_unit_variant("Type", 5u32, "CanadianProvinceExempt"),
                Self::ExternalTaxation => serializer.serialize_unit_variant("Type", 6u32, "ExternalTaxation"),
                Self::IndiaFederalTanId => serializer.serialize_unit_variant("Type", 7u32, "IndiaFederalTanId"),
                Self::IndiaFederalServiceTaxId => serializer.serialize_unit_variant("Type", 8u32, "IndiaFederalServiceTaxId"),
                Self::IndiaPanId => serializer.serialize_unit_variant("Type", 9u32, "IndiaPanId"),
                Self::IndiaStateCstId => serializer.serialize_unit_variant("Type", 10u32, "IndiaStateCstId"),
                Self::IndiaStateGstInId => serializer.serialize_unit_variant("Type", 11u32, "IndiaStateGstINId"),
                Self::IndiaStateVatId => serializer.serialize_unit_variant("Type", 12u32, "IndiaStateVatId"),
                Self::IntlExempt => serializer.serialize_unit_variant("Type", 13u32, "IntlExempt"),
                Self::UsExempt => serializer.serialize_unit_variant("Type", 14u32, "USExempt"),
                Self::VatId => serializer.serialize_unit_variant("Type", 15u32, "VatId"),
                Self::LoveCode => serializer.serialize_unit_variant("Type", 16u32, "LoveCode"),
                Self::MobileBarCode => serializer.serialize_unit_variant("Type", 17u32, "MobileBarCode"),
                Self::NationalIdentificationNumber => serializer.serialize_unit_variant("Type", 18u32, "NationalIdentificationNumber"),
                Self::PublicSectorId => serializer.serialize_unit_variant("Type", 19u32, "PublicSectorId"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the tax identifier."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Other,
        Valid,
        Invalid,
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
                Self::Other => serializer.serialize_unit_variant("Status", 0u32, "Other"),
                Self::Valid => serializer.serialize_unit_variant("Status", 1u32, "Valid"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 2u32, "Invalid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type TenantId = String;
#[doc = "A transaction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Transaction {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "A transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TransactionProperties>,
}
impl Transaction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container for a list of resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransactionListResult {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of resources."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Transaction>,
}
impl azure_core::Continuable for TransactionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TransactionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A transaction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransactionProperties {
    #[doc = "The amount of any Azure credits automatically applied to this transaction."]
    #[serde(rename = "azureCreditApplied", default, skip_serializing_if = "Option::is_none")]
    pub azure_credit_applied: Option<serde_json::Value>,
    #[doc = "Details of the Azure plan."]
    #[serde(rename = "azurePlan", default, skip_serializing_if = "Option::is_none")]
    pub azure_plan: Option<String>,
    #[doc = "The ISO 4217 code for the currency in which this transaction is billed."]
    #[serde(rename = "billingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency: Option<String>,
    #[doc = "The name of the billing profile."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<serde_json::Value>,
    #[doc = "The fully qualified ID that uniquely identifies a billing profile."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The amount of Microsoft Azure Consumption Commitment(MACC) decrement through the transaction."]
    #[serde(rename = "consumptionCommitmentDecremented", default, skip_serializing_if = "Option::is_none")]
    pub consumption_commitment_decremented: Option<serde_json::Value>,
    #[doc = "The name of the customer."]
    #[serde(rename = "customerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub customer_display_name: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies a customer."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "The credit type of the transaction. Applies only to credited transactions."]
    #[serde(rename = "creditType", default, skip_serializing_if = "Option::is_none")]
    pub credit_type: Option<transaction_properties::CreditType>,
    #[doc = "The date of transaction."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub date: Option<::time::OffsetDateTime>,
    #[doc = "The percentage discount, if any, applied to this transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    #[doc = "The price of the product after applying any discounts."]
    #[serde(rename = "effectivePrice", default, skip_serializing_if = "Option::is_none")]
    pub effective_price: Option<serde_json::Value>,
    #[doc = "The exchange rate used to convert charged amount to billing currency, if applicable."]
    #[serde(rename = "exchangeRate", default, skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<f64>,
    #[doc = "Invoice name on which the transaction was billed or 'Pending' if the transaction is not billed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    #[doc = "The fully qualified ID of the invoice on which the transaction was billed. This field is only applicable for transactions which are billed."]
    #[serde(rename = "invoiceId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[doc = "The name of the invoice section."]
    #[serde(rename = "invoiceSectionDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_display_name: Option<String>,
    #[doc = "The fully qualified ID that uniquely identifies an invoice section."]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "Whether or not the transaction is third party."]
    #[serde(rename = "isThirdParty", default, skip_serializing_if = "Option::is_none")]
    pub is_third_party: Option<bool>,
    #[doc = "Type of the transaction, billed or unbilled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<transaction_properties::Kind>,
    #[doc = "The retail price of the product."]
    #[serde(rename = "marketPrice", default, skip_serializing_if = "Option::is_none")]
    pub market_price: Option<serde_json::Value>,
    #[doc = "The part number of the product for which the transaction took place. The field is only applicable for Enterprise Agreement invoices."]
    #[serde(rename = "partNumber", default, skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[doc = "The ISO 4217 code for the currency in which the product is priced."]
    #[serde(rename = "pricingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency: Option<String>,
    #[doc = "The description of the product for which the transaction took place."]
    #[serde(rename = "productDescription", default, skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[doc = "The family of the product for which the transaction took place."]
    #[serde(rename = "productFamily", default, skip_serializing_if = "Option::is_none")]
    pub product_family: Option<String>,
    #[doc = "The ID of the product type for which the transaction took place."]
    #[serde(rename = "productTypeId", default, skip_serializing_if = "Option::is_none")]
    pub product_type_id: Option<String>,
    #[doc = "The type of the product for which the transaction took place."]
    #[serde(rename = "productType", default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[doc = "The quantity purchased in the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[doc = "There reason code for the transaction."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[doc = "The date of the purchase of the product, or the start date of the month in which usage started."]
    #[serde(rename = "servicePeriodStartDate", default, with = "azure_core::date::rfc3339::option")]
    pub service_period_start_date: Option<::time::OffsetDateTime>,
    #[doc = "The end date of the product term, or the end date of the month in which usage ended."]
    #[serde(rename = "servicePeriodEndDate", default, with = "azure_core::date::rfc3339::option")]
    pub service_period_end_date: Option<::time::OffsetDateTime>,
    #[doc = "The pre-tax charged amount for the transaction."]
    #[serde(rename = "subTotal", default, skip_serializing_if = "Option::is_none")]
    pub sub_total: Option<serde_json::Value>,
    #[doc = "The tax amount applied to the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<serde_json::Value>,
    #[doc = "The charge associated with the transaction."]
    #[serde(rename = "transactionAmount", default, skip_serializing_if = "Option::is_none")]
    pub transaction_amount: Option<serde_json::Value>,
    #[doc = "The type of transaction."]
    #[serde(rename = "transactionType", default, skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
    #[doc = "The number of units used for a given product."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub units: Option<f64>,
    #[doc = "The unit of measure used to bill for the product. For example, compute services are billed per hour."]
    #[serde(rename = "unitOfMeasure", default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
    #[doc = "The description for the unit of measure for a given product."]
    #[serde(rename = "unitType", default, skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<String>,
    #[doc = "Identifies the type of tax calculation used for the invoice. The field is applicable only to invoices with special tax calculation logic."]
    #[serde(rename = "specialTaxationType", default, skip_serializing_if = "Option::is_none")]
    pub special_taxation_type: Option<transaction_properties::SpecialTaxationType>,
    #[doc = "The refund details of a transaction."]
    #[serde(rename = "refundTransactionDetails", default, skip_serializing_if = "Option::is_none")]
    pub refund_transaction_details: Option<serde_json::Value>,
}
impl TransactionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod transaction_properties {
    use super::*;
    #[doc = "The credit type of the transaction. Applies only to credited transactions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreditType")]
    pub enum CreditType {
        Other,
        AzureFreeCredit,
        AzureCreditOffer,
        ServiceInterruption,
        Refund,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreditType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreditType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreditType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Other => serializer.serialize_unit_variant("CreditType", 0u32, "Other"),
                Self::AzureFreeCredit => serializer.serialize_unit_variant("CreditType", 1u32, "AzureFreeCredit"),
                Self::AzureCreditOffer => serializer.serialize_unit_variant("CreditType", 2u32, "AzureCreditOffer"),
                Self::ServiceInterruption => serializer.serialize_unit_variant("CreditType", 3u32, "ServiceInterruption"),
                Self::Refund => serializer.serialize_unit_variant("CreditType", 4u32, "Refund"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Type of the transaction, billed or unbilled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Other,
        All,
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
                Self::Other => serializer.serialize_unit_variant("Kind", 0u32, "Other"),
                Self::All => serializer.serialize_unit_variant("Kind", 1u32, "All"),
                Self::Reservation => serializer.serialize_unit_variant("Kind", 2u32, "Reservation"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Identifies the type of tax calculation used for the invoice. The field is applicable only to invoices with special tax calculation logic."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SpecialTaxationType")]
    pub enum SpecialTaxationType {
        SubtotalLevel,
        InvoiceLevel,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SpecialTaxationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SpecialTaxationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SpecialTaxationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SubtotalLevel => serializer.serialize_unit_variant("SpecialTaxationType", 0u32, "SubtotalLevel"),
                Self::InvoiceLevel => serializer.serialize_unit_variant("SpecialTaxationType", 1u32, "InvoiceLevel"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A transaction summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransactionSummary {
    #[doc = "The total amount of any Azure credits applied."]
    #[serde(rename = "azureCreditApplied", default, skip_serializing_if = "Option::is_none")]
    pub azure_credit_applied: Option<f64>,
    #[doc = "The ISO 4217 code for the currency in which the transactions are billed."]
    #[serde(rename = "billingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency: Option<String>,
    #[doc = "The total Microsoft Azure Consumption Commitment (MACC) decrement through the invoice."]
    #[serde(rename = "consumptionCommitmentDecremented", default, skip_serializing_if = "Option::is_none")]
    pub consumption_commitment_decremented: Option<f64>,
    #[doc = "The total pre-tax charged amount."]
    #[serde(rename = "subTotal", default, skip_serializing_if = "Option::is_none")]
    pub sub_total: Option<f64>,
    #[doc = "The total tax amount applied."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    #[doc = "The total charges."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
}
impl TransactionSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the transfer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransferDetails {
    #[serde(flatten)]
    pub proxy_resource_with_tags: ProxyResourceWithTags,
    #[doc = "Transfer details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TransferProperties>,
}
impl TransferDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of transfers initiated by caller."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransferDetailsListResult {
    #[doc = "The list of transfers initiated by caller."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TransferDetails>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TransferDetailsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TransferDetailsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details for transfer execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransferError {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl TransferError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query parameter to enumerate transfer requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransferItemQueryParameter {
    #[doc = "State of the transfer request query filter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl TransferItemQueryParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Transfer details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransferProperties {
    #[doc = "The time at which the transfer request expires."]
    #[serde(rename = "expirationTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiration_time: Option<::time::OffsetDateTime>,
    #[doc = "The status of a transfer."]
    #[serde(rename = "transferStatus", default, skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<TransferStatus>,
    #[doc = "The email ID of the user to whom the transfer request was sent."]
    #[serde(rename = "recipientEmailId", default, skip_serializing_if = "Option::is_none")]
    pub recipient_email_id: Option<String>,
    #[doc = "The email ID of the user who sent the transfer request."]
    #[serde(rename = "initiatorEmailId", default, skip_serializing_if = "Option::is_none")]
    pub initiator_email_id: Option<String>,
    #[doc = "The email ID of the user who canceled the transfer request."]
    #[serde(rename = "canceledBy", default, skip_serializing_if = "Option::is_none")]
    pub canceled_by: Option<String>,
    #[doc = "Detailed transfer status."]
    #[serde(
        rename = "detailedTransferStatus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub detailed_transfer_status: Vec<DetailedTransferStatus>,
}
impl TransferProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of a transfer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TransferStatus")]
pub enum TransferStatus {
    Expired,
    Pending,
    InProgress,
    Completed,
    CompletedWithErrors,
    Failed,
    Canceled,
    Declined,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TransferStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TransferStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TransferStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Expired => serializer.serialize_unit_variant("TransferStatus", 0u32, "Expired"),
            Self::Pending => serializer.serialize_unit_variant("TransferStatus", 1u32, "Pending"),
            Self::InProgress => serializer.serialize_unit_variant("TransferStatus", 2u32, "InProgress"),
            Self::Completed => serializer.serialize_unit_variant("TransferStatus", 3u32, "Completed"),
            Self::CompletedWithErrors => serializer.serialize_unit_variant("TransferStatus", 4u32, "CompletedWithErrors"),
            Self::Failed => serializer.serialize_unit_variant("TransferStatus", 5u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("TransferStatus", 6u32, "Canceled"),
            Self::Declined => serializer.serialize_unit_variant("TransferStatus", 7u32, "Declined"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The details for a billing account transitioned from agreement type Microsoft Online Services Program to agreement type Microsoft Customer Agreement."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransitionDetails {
    #[doc = "The transition completion date."]
    #[serde(rename = "transitionDate", default, with = "azure_core::date::rfc3339::option")]
    pub transition_date: Option<::time::OffsetDateTime>,
    #[doc = "The anniversary day of the pre-transitioned account of type Microsoft Online Services Program."]
    #[serde(rename = "anniversaryDay", default, skip_serializing_if = "Option::is_none")]
    pub anniversary_day: Option<i32>,
}
impl TransitionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan utilization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Utilization {
    #[doc = "The trend for a savings plan's utilization"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trend: Option<String>,
    #[doc = "The array of aggregates of a savings plan's utilization"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub aggregates: Vec<UtilizationAggregates>,
}
impl Utilization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The aggregate values of savings plan utilization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UtilizationAggregates {
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
impl UtilizationAggregates {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of transfer validation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateTransferListResponse {
    #[doc = "The list of transfer validation results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ValidateTransferResponse>,
}
impl ValidateTransferListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Transfer validation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateTransferResponse {
    #[doc = "The properties of transfer validation response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ValidateTransferResponseProperties>,
}
impl ValidateTransferResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of transfer validation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateTransferResponseProperties {
    #[doc = "The status of validation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The product id for which this result applies."]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[doc = "The array of validation results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub results: Vec<ValidationResultProperties>,
}
impl ValidateTransferResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the validation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationResultProperties {
    #[doc = "Result Level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[doc = "Result Code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The validation message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ValidationResultProperties {
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
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
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
