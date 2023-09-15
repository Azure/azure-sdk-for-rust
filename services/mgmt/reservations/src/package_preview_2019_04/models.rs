#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedReservationList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<String>,
    #[doc = "Url to get the next page of reservations"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AppliedReservationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedReservations {
    #[doc = "Identifier of the applied reservations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. \"Microsoft.Capacity/AppliedReservations\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppliedReservationsProperties>,
}
impl AppliedReservations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedReservationsProperties {
    #[serde(rename = "reservationOrderIds", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_ids: Option<AppliedReservationList>,
}
impl AppliedReservationsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedScopeProperties {
    #[doc = "Tenant ID of the applied scope type"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Management group ID of the format /providers/Microsoft.Management/managementGroups/{managementGroupId}"]
    #[serde(rename = "managementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub management_group_id: Option<String>,
    #[doc = "Management group display name"]
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
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type AppliedScopes = Vec<String>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableScopeRequest {
    #[doc = "List of scopes for which availability should be checked"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailableScopeRequestProperties>,
}
impl AvailableScopeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of scopes for which availability should be checked"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableScopeRequestProperties {
    #[doc = "Scopes to be checked for availability"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<String>,
}
impl AvailableScopeRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type BillingScopeId = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CalculatePriceResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CalculatePriceResponseProperties>,
}
impl CalculatePriceResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CalculatePriceResponseProperties {
    #[doc = "Currency and amount that customer will be charged in customer's local currency. Tax is not included."]
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<calculate_price_response_properties::BillingCurrencyTotal>,
    #[doc = "Net total"]
    #[serde(rename = "netTotal", default, skip_serializing_if = "Option::is_none")]
    pub net_total: Option<f64>,
    #[doc = "Tax total"]
    #[serde(rename = "taxTotal", default, skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<f64>,
    #[doc = "Grand Total"]
    #[serde(rename = "grandTotal", default, skip_serializing_if = "Option::is_none")]
    pub grand_total: Option<f64>,
    #[doc = "True if billing is managed by Microsoft Partner. Used only for CSP accounts."]
    #[serde(rename = "isBillingPartnerManaged", default, skip_serializing_if = "Option::is_none")]
    pub is_billing_partner_managed: Option<bool>,
    #[doc = "GUID that represents reservation order that can be placed after calculating price."]
    #[serde(rename = "reservationOrderId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[doc = "Title of SKU that is being purchased."]
    #[serde(rename = "skuTitle", default, skip_serializing_if = "Option::is_none")]
    pub sku_title: Option<String>,
    #[doc = "Description of SKU that is being purchased."]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[doc = "Amount that Microsoft uses for record. Used during refund for calculating refund limit. Tax is not included."]
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<calculate_price_response_properties::PricingCurrencyTotal>,
    #[serde(
        rename = "paymentSchedule",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub payment_schedule: Vec<PaymentDetail>,
}
impl CalculatePriceResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod calculate_price_response_properties {
    use super::*;
    #[doc = "Currency and amount that customer will be charged in customer's local currency. Tax is not included."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct BillingCurrencyTotal {
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    impl BillingCurrencyTotal {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Amount that Microsoft uses for record. Used during refund for calculating refund limit. Tax is not included."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct PricingCurrencyTotal {
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    impl PricingCurrencyTotal {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Catalog {
    #[doc = "The type of resource the SKU applies to."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The name of SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The billing plan options available for this SKU."]
    #[serde(rename = "billingPlans", default, skip_serializing_if = "Option::is_none")]
    pub billing_plans: Option<serde_json::Value>,
    #[doc = "The sku's MSRP values for each term"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msrp: Option<serde_json::Value>,
    #[doc = "Available reservation terms for this resource"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub terms: Vec<ReservationTerm>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub locations: Vec<String>,
    #[serde(
        rename = "skuProperties",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sku_properties: Vec<SkuProperty>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub restrictions: Vec<SkuRestriction>,
}
impl Catalog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ExtendedErrorInfo>,
}
impl azure_core::Continuable for Error {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ErrorResponseCode")]
pub enum ErrorResponseCode {
    NotSpecified,
    InternalServerError,
    ServerTimeout,
    AuthorizationFailed,
    BadRequest,
    ClientCertificateThumbprintNotSet,
    InvalidRequestContent,
    OperationFailed,
    HttpMethodNotSupported,
    InvalidRequestUri,
    MissingTenantId,
    InvalidTenantId,
    InvalidReservationOrderId,
    InvalidReservationId,
    ReservationIdNotInReservationOrder,
    ReservationOrderNotFound,
    InvalidSubscriptionId,
    InvalidAccessToken,
    InvalidLocationId,
    UnauthenticatedRequestsThrottled,
    InvalidHealthCheckType,
    Forbidden,
    BillingScopeIdCannotBeChanged,
    AppliedScopesNotAssociatedWithCommerceAccount,
    PatchValuesSameAsExisting,
    RoleAssignmentCreationFailed,
    ReservationOrderCreationFailed,
    ReservationOrderNotEnabled,
    CapacityUpdateScopesFailed,
    UnsupportedReservationTerm,
    ReservationOrderIdAlreadyExists,
    RiskCheckFailed,
    CreateQuoteFailed,
    ActivateQuoteFailed,
    NonsupportedAccountId,
    PaymentInstrumentNotFound,
    MissingAppliedScopesForSingle,
    NoValidReservationsToReRate,
    #[serde(rename = "ReRateOnlyAllowedForEA")]
    ReRateOnlyAllowedForEa,
    OperationCannotBePerformedInCurrentState,
    InvalidSingleAppliedScopesCount,
    InvalidFulfillmentRequestParameters,
    NotSupportedCountry,
    InvalidRefundQuantity,
    PurchaseError,
    BillingCustomerInputError,
    BillingPaymentInstrumentSoftError,
    BillingPaymentInstrumentHardError,
    BillingTransientError,
    BillingError,
    FulfillmentConfigurationError,
    FulfillmentOutOfStockError,
    FulfillmentTransientError,
    FulfillmentError,
    CalculatePriceFailed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ErrorResponseCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ErrorResponseCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ErrorResponseCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("ErrorResponseCode", 0u32, "NotSpecified"),
            Self::InternalServerError => serializer.serialize_unit_variant("ErrorResponseCode", 1u32, "InternalServerError"),
            Self::ServerTimeout => serializer.serialize_unit_variant("ErrorResponseCode", 2u32, "ServerTimeout"),
            Self::AuthorizationFailed => serializer.serialize_unit_variant("ErrorResponseCode", 3u32, "AuthorizationFailed"),
            Self::BadRequest => serializer.serialize_unit_variant("ErrorResponseCode", 4u32, "BadRequest"),
            Self::ClientCertificateThumbprintNotSet => {
                serializer.serialize_unit_variant("ErrorResponseCode", 5u32, "ClientCertificateThumbprintNotSet")
            }
            Self::InvalidRequestContent => serializer.serialize_unit_variant("ErrorResponseCode", 6u32, "InvalidRequestContent"),
            Self::OperationFailed => serializer.serialize_unit_variant("ErrorResponseCode", 7u32, "OperationFailed"),
            Self::HttpMethodNotSupported => serializer.serialize_unit_variant("ErrorResponseCode", 8u32, "HttpMethodNotSupported"),
            Self::InvalidRequestUri => serializer.serialize_unit_variant("ErrorResponseCode", 9u32, "InvalidRequestUri"),
            Self::MissingTenantId => serializer.serialize_unit_variant("ErrorResponseCode", 10u32, "MissingTenantId"),
            Self::InvalidTenantId => serializer.serialize_unit_variant("ErrorResponseCode", 11u32, "InvalidTenantId"),
            Self::InvalidReservationOrderId => serializer.serialize_unit_variant("ErrorResponseCode", 12u32, "InvalidReservationOrderId"),
            Self::InvalidReservationId => serializer.serialize_unit_variant("ErrorResponseCode", 13u32, "InvalidReservationId"),
            Self::ReservationIdNotInReservationOrder => {
                serializer.serialize_unit_variant("ErrorResponseCode", 14u32, "ReservationIdNotInReservationOrder")
            }
            Self::ReservationOrderNotFound => serializer.serialize_unit_variant("ErrorResponseCode", 15u32, "ReservationOrderNotFound"),
            Self::InvalidSubscriptionId => serializer.serialize_unit_variant("ErrorResponseCode", 16u32, "InvalidSubscriptionId"),
            Self::InvalidAccessToken => serializer.serialize_unit_variant("ErrorResponseCode", 17u32, "InvalidAccessToken"),
            Self::InvalidLocationId => serializer.serialize_unit_variant("ErrorResponseCode", 18u32, "InvalidLocationId"),
            Self::UnauthenticatedRequestsThrottled => {
                serializer.serialize_unit_variant("ErrorResponseCode", 19u32, "UnauthenticatedRequestsThrottled")
            }
            Self::InvalidHealthCheckType => serializer.serialize_unit_variant("ErrorResponseCode", 20u32, "InvalidHealthCheckType"),
            Self::Forbidden => serializer.serialize_unit_variant("ErrorResponseCode", 21u32, "Forbidden"),
            Self::BillingScopeIdCannotBeChanged => {
                serializer.serialize_unit_variant("ErrorResponseCode", 22u32, "BillingScopeIdCannotBeChanged")
            }
            Self::AppliedScopesNotAssociatedWithCommerceAccount => {
                serializer.serialize_unit_variant("ErrorResponseCode", 23u32, "AppliedScopesNotAssociatedWithCommerceAccount")
            }
            Self::PatchValuesSameAsExisting => serializer.serialize_unit_variant("ErrorResponseCode", 24u32, "PatchValuesSameAsExisting"),
            Self::RoleAssignmentCreationFailed => {
                serializer.serialize_unit_variant("ErrorResponseCode", 25u32, "RoleAssignmentCreationFailed")
            }
            Self::ReservationOrderCreationFailed => {
                serializer.serialize_unit_variant("ErrorResponseCode", 26u32, "ReservationOrderCreationFailed")
            }
            Self::ReservationOrderNotEnabled => serializer.serialize_unit_variant("ErrorResponseCode", 27u32, "ReservationOrderNotEnabled"),
            Self::CapacityUpdateScopesFailed => serializer.serialize_unit_variant("ErrorResponseCode", 28u32, "CapacityUpdateScopesFailed"),
            Self::UnsupportedReservationTerm => serializer.serialize_unit_variant("ErrorResponseCode", 29u32, "UnsupportedReservationTerm"),
            Self::ReservationOrderIdAlreadyExists => {
                serializer.serialize_unit_variant("ErrorResponseCode", 30u32, "ReservationOrderIdAlreadyExists")
            }
            Self::RiskCheckFailed => serializer.serialize_unit_variant("ErrorResponseCode", 31u32, "RiskCheckFailed"),
            Self::CreateQuoteFailed => serializer.serialize_unit_variant("ErrorResponseCode", 32u32, "CreateQuoteFailed"),
            Self::ActivateQuoteFailed => serializer.serialize_unit_variant("ErrorResponseCode", 33u32, "ActivateQuoteFailed"),
            Self::NonsupportedAccountId => serializer.serialize_unit_variant("ErrorResponseCode", 34u32, "NonsupportedAccountId"),
            Self::PaymentInstrumentNotFound => serializer.serialize_unit_variant("ErrorResponseCode", 35u32, "PaymentInstrumentNotFound"),
            Self::MissingAppliedScopesForSingle => {
                serializer.serialize_unit_variant("ErrorResponseCode", 36u32, "MissingAppliedScopesForSingle")
            }
            Self::NoValidReservationsToReRate => {
                serializer.serialize_unit_variant("ErrorResponseCode", 37u32, "NoValidReservationsToReRate")
            }
            Self::ReRateOnlyAllowedForEa => serializer.serialize_unit_variant("ErrorResponseCode", 38u32, "ReRateOnlyAllowedForEA"),
            Self::OperationCannotBePerformedInCurrentState => {
                serializer.serialize_unit_variant("ErrorResponseCode", 39u32, "OperationCannotBePerformedInCurrentState")
            }
            Self::InvalidSingleAppliedScopesCount => {
                serializer.serialize_unit_variant("ErrorResponseCode", 40u32, "InvalidSingleAppliedScopesCount")
            }
            Self::InvalidFulfillmentRequestParameters => {
                serializer.serialize_unit_variant("ErrorResponseCode", 41u32, "InvalidFulfillmentRequestParameters")
            }
            Self::NotSupportedCountry => serializer.serialize_unit_variant("ErrorResponseCode", 42u32, "NotSupportedCountry"),
            Self::InvalidRefundQuantity => serializer.serialize_unit_variant("ErrorResponseCode", 43u32, "InvalidRefundQuantity"),
            Self::PurchaseError => serializer.serialize_unit_variant("ErrorResponseCode", 44u32, "PurchaseError"),
            Self::BillingCustomerInputError => serializer.serialize_unit_variant("ErrorResponseCode", 45u32, "BillingCustomerInputError"),
            Self::BillingPaymentInstrumentSoftError => {
                serializer.serialize_unit_variant("ErrorResponseCode", 46u32, "BillingPaymentInstrumentSoftError")
            }
            Self::BillingPaymentInstrumentHardError => {
                serializer.serialize_unit_variant("ErrorResponseCode", 47u32, "BillingPaymentInstrumentHardError")
            }
            Self::BillingTransientError => serializer.serialize_unit_variant("ErrorResponseCode", 48u32, "BillingTransientError"),
            Self::BillingError => serializer.serialize_unit_variant("ErrorResponseCode", 49u32, "BillingError"),
            Self::FulfillmentConfigurationError => {
                serializer.serialize_unit_variant("ErrorResponseCode", 50u32, "FulfillmentConfigurationError")
            }
            Self::FulfillmentOutOfStockError => serializer.serialize_unit_variant("ErrorResponseCode", 51u32, "FulfillmentOutOfStockError"),
            Self::FulfillmentTransientError => serializer.serialize_unit_variant("ErrorResponseCode", 52u32, "FulfillmentTransientError"),
            Self::FulfillmentError => serializer.serialize_unit_variant("ErrorResponseCode", 53u32, "FulfillmentError"),
            Self::CalculatePriceFailed => serializer.serialize_unit_variant("ErrorResponseCode", 54u32, "CalculatePriceFailed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedErrorInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorResponseCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ExtendedErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedStatusInfo {
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<ReservationStatusCode>,
    #[doc = "The message giving detailed information about the status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ExtendedStatusInfo {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MergeProperties {
    #[doc = "Format of the resource id should be /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sources: Vec<String>,
}
impl MergeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MergeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MergeProperties>,
}
impl MergeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MsrpProperty {
    #[doc = "Represents the currency code - 3 character ISO."]
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[doc = "Represents the amount of money in the currency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}
impl MsrpProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OperationResponse>,
    #[doc = "Url to get the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl OperationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Patch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PatchProperties>,
}
impl Patch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchProperties {
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "List of the subscriptions that the benefit will be applied. Do not specify if AppliedScopeType is Shared."]
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[doc = "Turning this on will apply the reservation discount to other VMs in the same VM size group. Only specify for VirtualMachines reserved resource type."]
    #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[doc = "Name of the Reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Setting this to true will automatically purchase a new reservation on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[serde(rename = "renewProperties", default, skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<patch_properties::RenewProperties>,
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
        #[serde(rename = "purchaseProperties", default, skip_serializing_if = "Option::is_none")]
        pub purchase_properties: Option<PurchaseRequest>,
    }
    impl RenewProperties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Information about payment related to a reservation order."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentDetail {
    #[doc = "Date when the payment needs to be done."]
    #[serde(rename = "dueDate", default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[doc = "Date when the transaction is completed. Is null when it is scheduled."]
    #[serde(rename = "paymentDate", default, skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<String>,
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<Price>,
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<Price>,
    #[doc = "Shows the Account that is charged for this payment."]
    #[serde(rename = "billingAccount", default, skip_serializing_if = "Option::is_none")]
    pub billing_account: Option<String>,
    #[doc = "Describes whether the payment is completed, failed, cancelled or scheduled in the future."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentStatus>,
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
}
impl PaymentDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes whether the payment is completed, failed, cancelled or scheduled in the future."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PaymentStatus")]
pub enum PaymentStatus {
    Succeeded,
    Failed,
    Scheduled,
    Cancelled,
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
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Properties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionScopeProperties>,
}
impl Properties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PurchaseRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuName>,
    #[doc = "The Azure Region where the reserved resource lives."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PurchaseRequestProperties>,
}
impl PurchaseRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PurchaseRequestProperties {
    #[doc = "The type of the resource that is being reserved."]
    #[serde(rename = "reservedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<ReservedResourceType>,
    #[doc = "Turning this on will apply the reservation discount to other VMs in the same VM size group. Only specify for VirtualMachines reserved resource type."]
    #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[doc = "Subscription that will be charged for purchasing Reservation"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[doc = "Represent the term of Reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
    #[doc = "Represent the billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[doc = "Quantity of the SKUs that are part of the Reservation. Must be greater than zero."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<ReservationQuantity>,
    #[doc = "Friendly name of the Reservation"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "List of the subscriptions that the benefit will be applied. Do not specify if AppliedScopeType is Shared."]
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[doc = "Setting this to true will automatically purchase a new reservation on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[doc = "Properties specific to each reserved resource type. Not required if not applicable."]
    #[serde(rename = "reservedResourceProperties", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_properties: Option<purchase_request_properties::ReservedResourceProperties>,
}
impl PurchaseRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod purchase_request_properties {
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
pub type Renew = bool;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RenewPropertiesResponse {
    #[serde(rename = "purchaseProperties", default, skip_serializing_if = "Option::is_none")]
    pub purchase_properties: Option<PurchaseRequest>,
    #[doc = "Amount that Microsoft uses for record. Used during refund for calculating refund limit. Tax is not included. This is locked price 30 days before expiry."]
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<renew_properties_response::PricingCurrencyTotal>,
    #[doc = "Currency and amount that customer will be charged in customer's local currency for renewal purchase. Tax is not included."]
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<renew_properties_response::BillingCurrencyTotal>,
}
impl RenewPropertiesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod renew_properties_response {
    use super::*;
    #[doc = "Amount that Microsoft uses for record. Used during refund for calculating refund limit. Tax is not included. This is locked price 30 days before expiry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct PricingCurrencyTotal {
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    impl PricingCurrencyTotal {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Currency and amount that customer will be charged in customer's local currency for renewal purchase. Tax is not included."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct BillingCurrencyTotal {
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    impl BillingCurrencyTotal {
        pub fn new() -> Self {
            Self::default()
        }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ReservationResponse>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationMergeProperties {
    #[doc = "Reservation Resource Id Created due to the merge. Format of the resource Id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "mergeDestination", default, skip_serializing_if = "Option::is_none")]
    pub merge_destination: Option<String>,
    #[doc = "Resource Ids of the Source Reservation's merged to form this Reservation. Format of the resource Id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
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
#[doc = "Information describing the type of billing plan for this reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderBillingPlanInformation {
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
impl ReservationOrderBillingPlanInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ReservationOrderResponse>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderProperties {
    #[doc = "Friendly name for user to easily identified the reservation."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "This is the DateTime when the reservation was initially requested for purchase."]
    #[serde(rename = "requestDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub request_date_time: Option<time::OffsetDateTime>,
    #[doc = "This is the DateTime when the reservation was created."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "This is the date when the Reservation will expire."]
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[doc = "This is the DateTime when the reservation benefit started."]
    #[serde(rename = "benefitStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub benefit_start_time: Option<time::OffsetDateTime>,
    #[doc = "Quantity of the SKUs that are part of the Reservation. Must be greater than zero."]
    #[serde(rename = "originalQuantity", default, skip_serializing_if = "Option::is_none")]
    pub original_quantity: Option<ReservationQuantity>,
    #[doc = "Represent the term of Reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
    #[doc = "Current state of the reservation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Represent the billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[doc = "Information describing the type of billing plan for this reservation."]
    #[serde(rename = "planInformation", default, skip_serializing_if = "Option::is_none")]
    pub plan_information: Option<ReservationOrderBillingPlanInformation>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reservations: Vec<ReservationResponse>,
}
impl ReservationOrderProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i64>,
    #[doc = "Identifier of the reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationOrderProperties>,
    #[doc = "Type of resource. \"Microsoft.Capacity/reservations\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ReservationOrderResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationProperties {
    #[doc = "The type of the resource that is being reserved."]
    #[serde(rename = "reservedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<ReservedResourceType>,
    #[doc = "Turning this on will apply the reservation discount to other VMs in the same VM size group. Only specify for VirtualMachines reserved resource type."]
    #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[doc = "Friendly name for user to easily identify the reservation"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "List of the subscriptions that the benefit will be applied. Do not specify if AppliedScopeType is Shared."]
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "Quantity of the SKUs that are part of the Reservation. Must be greater than zero."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<ReservationQuantity>,
    #[doc = "Current state of the reservation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "DateTime of the Reservation starting when this version is effective from."]
    #[serde(rename = "effectiveDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub effective_date_time: Option<time::OffsetDateTime>,
    #[doc = "This is the DateTime when the reservation benefit started."]
    #[serde(rename = "benefitStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub benefit_start_time: Option<time::OffsetDateTime>,
    #[doc = "DateTime of the last time the Reservation was updated."]
    #[serde(rename = "lastUpdatedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_date_time: Option<time::OffsetDateTime>,
    #[doc = "This is the date when the Reservation will expire."]
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[doc = "Description of the SKU in english."]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
    #[doc = "Represent the billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[serde(rename = "splitProperties", default, skip_serializing_if = "Option::is_none")]
    pub split_properties: Option<ReservationSplitProperties>,
    #[serde(rename = "mergeProperties", default, skip_serializing_if = "Option::is_none")]
    pub merge_properties: Option<ReservationMergeProperties>,
    #[serde(rename = "swapProperties", default, skip_serializing_if = "Option::is_none")]
    pub swap_properties: Option<ReservationSwapProperties>,
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<AppliedScopeProperties>,
    #[doc = "Subscription that will be charged for purchasing Reservation"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[doc = "Setting this to true will automatically purchase a new reservation on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[doc = "Reservation Id of the reservation from which this reservation is renewed. Format of the resource Id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}."]
    #[serde(rename = "renewSource", default, skip_serializing_if = "Option::is_none")]
    pub renew_source: Option<String>,
    #[doc = "Reservation Id of the reservation which is purchased because of renew. Format of the resource Id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}."]
    #[serde(rename = "renewDestination", default, skip_serializing_if = "Option::is_none")]
    pub renew_destination: Option<String>,
    #[serde(rename = "renewProperties", default, skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<RenewPropertiesResponse>,
    #[doc = "Represent the term of Reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
    #[doc = "Property to determine if a reservation is archived or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[doc = "List the Capabilities of a reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<String>,
}
impl ReservationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ReservationQuantity = i32;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationResponse {
    #[doc = "The Azure Region where the reserved resource lives."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i64>,
    #[doc = "Identifier of the reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationProperties>,
    #[doc = "Type of resource. \"Microsoft.Capacity/reservationOrders/reservations\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ReservationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSplitProperties {
    #[doc = "List of destination Resource Id that are created due to split. Format of the resource Id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(
        rename = "splitDestinations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub split_destinations: Vec<String>,
    #[doc = "Resource Id of the Reservation from which this is split. Format of the resource Id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "splitSource", default, skip_serializing_if = "Option::is_none")]
    pub split_source: Option<String>,
}
impl ReservationSplitProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReservationStatusCode")]
pub enum ReservationStatusCode {
    None,
    Pending,
    Active,
    PurchaseError,
    PaymentInstrumentError,
    Split,
    Merged,
    Expired,
    Succeeded,
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
            Self::Active => serializer.serialize_unit_variant("ReservationStatusCode", 2u32, "Active"),
            Self::PurchaseError => serializer.serialize_unit_variant("ReservationStatusCode", 3u32, "PurchaseError"),
            Self::PaymentInstrumentError => serializer.serialize_unit_variant("ReservationStatusCode", 4u32, "PaymentInstrumentError"),
            Self::Split => serializer.serialize_unit_variant("ReservationStatusCode", 5u32, "Split"),
            Self::Merged => serializer.serialize_unit_variant("ReservationStatusCode", 6u32, "Merged"),
            Self::Expired => serializer.serialize_unit_variant("ReservationStatusCode", 7u32, "Expired"),
            Self::Succeeded => serializer.serialize_unit_variant("ReservationStatusCode", 8u32, "Succeeded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSwapProperties {
    #[doc = "Resource Id of the Source Reservation that gets swapped. Format of the resource Id is /providers/microsoft.capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "swapSource", default, skip_serializing_if = "Option::is_none")]
    pub swap_source: Option<String>,
    #[doc = "Reservation Resource Id that the original resource gets swapped to. Format of the resource Id is /providers/microsoft.capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "swapDestination", default, skip_serializing_if = "Option::is_none")]
    pub swap_destination: Option<String>,
}
impl ReservationSwapProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represent the term of Reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReservationTerm")]
pub enum ReservationTerm {
    #[serde(rename = "P1Y")]
    P1y,
    #[serde(rename = "P3Y")]
    P3y,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReservationTerm {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReservationTerm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReservationTerm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::P1y => serializer.serialize_unit_variant("ReservationTerm", 0u32, "P1Y"),
            Self::P3y => serializer.serialize_unit_variant("ReservationTerm", 1u32, "P3Y"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of the resource that is being reserved."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReservedResourceType")]
pub enum ReservedResourceType {
    VirtualMachines,
    SqlDatabases,
    SuseLinux,
    CosmosDb,
    RedHat,
    SqlDataWarehouse,
    VMwareCloudSimple,
    RedHatOsa,
    Databricks,
    AppService,
    ManagedDisk,
    BlockBlob,
    RedisCache,
    AzureDataExplorer,
    MySql,
    MariaDb,
    PostgreSql,
    DedicatedHost,
    SapHana,
    SqlAzureHybridBenefit,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReservedResourceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReservedResourceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReservedResourceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::VirtualMachines => serializer.serialize_unit_variant("ReservedResourceType", 0u32, "VirtualMachines"),
            Self::SqlDatabases => serializer.serialize_unit_variant("ReservedResourceType", 1u32, "SqlDatabases"),
            Self::SuseLinux => serializer.serialize_unit_variant("ReservedResourceType", 2u32, "SuseLinux"),
            Self::CosmosDb => serializer.serialize_unit_variant("ReservedResourceType", 3u32, "CosmosDb"),
            Self::RedHat => serializer.serialize_unit_variant("ReservedResourceType", 4u32, "RedHat"),
            Self::SqlDataWarehouse => serializer.serialize_unit_variant("ReservedResourceType", 5u32, "SqlDataWarehouse"),
            Self::VMwareCloudSimple => serializer.serialize_unit_variant("ReservedResourceType", 6u32, "VMwareCloudSimple"),
            Self::RedHatOsa => serializer.serialize_unit_variant("ReservedResourceType", 7u32, "RedHatOsa"),
            Self::Databricks => serializer.serialize_unit_variant("ReservedResourceType", 8u32, "Databricks"),
            Self::AppService => serializer.serialize_unit_variant("ReservedResourceType", 9u32, "AppService"),
            Self::ManagedDisk => serializer.serialize_unit_variant("ReservedResourceType", 10u32, "ManagedDisk"),
            Self::BlockBlob => serializer.serialize_unit_variant("ReservedResourceType", 11u32, "BlockBlob"),
            Self::RedisCache => serializer.serialize_unit_variant("ReservedResourceType", 12u32, "RedisCache"),
            Self::AzureDataExplorer => serializer.serialize_unit_variant("ReservedResourceType", 13u32, "AzureDataExplorer"),
            Self::MySql => serializer.serialize_unit_variant("ReservedResourceType", 14u32, "MySql"),
            Self::MariaDb => serializer.serialize_unit_variant("ReservedResourceType", 15u32, "MariaDb"),
            Self::PostgreSql => serializer.serialize_unit_variant("ReservedResourceType", 16u32, "PostgreSql"),
            Self::DedicatedHost => serializer.serialize_unit_variant("ReservedResourceType", 17u32, "DedicatedHost"),
            Self::SapHana => serializer.serialize_unit_variant("ReservedResourceType", 18u32, "SapHana"),
            Self::SqlAzureHybridBenefit => serializer.serialize_unit_variant("ReservedResourceType", 19u32, "SqlAzureHybridBenefit"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl ScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuProperty {
    #[doc = "An invariant to describe the feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "An invariant if the feature is measured by quantity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SkuProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuRestriction {
    #[doc = "The type of restrictions."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The value of restrictions. If the restriction type is set to location. This would be different locations where the SKU is restricted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
    #[doc = "The reason for restriction."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
}
impl SkuRestriction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SplitProperties {
    #[doc = "List of the quantities in the new reservations to create."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub quantities: Vec<i64>,
    #[doc = "Resource id of the reservation to be split. Format of the resource id should be /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
}
impl SplitProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SplitRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SplitProperties>,
}
impl SplitRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionScopeProperties {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<ScopeProperties>,
}
impl SubscriptionScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
