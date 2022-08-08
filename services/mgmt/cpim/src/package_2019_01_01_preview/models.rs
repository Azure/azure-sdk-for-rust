#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The async operation status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AsyncOperationStatus {
    #[doc = "Subscription ID that the resource belongs to."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The GET resource path for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The operation ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<async_operation_status::Status>,
    #[doc = "Start time of the async operation."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "End time of the async operation."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "Properties of the Azure AD B2C tenant Azure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<B2cTenantResourceProperties>,
    #[doc = "Error response if async operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<async_operation_status::Error>,
}
impl AsyncOperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod async_operation_status {
    use super::*;
    #[doc = "The status of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Succeeded,
        Pending,
        Failed,
    }
    #[doc = "Error response if async operation failed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Error code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Error message."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "SKU properties of the Azure AD B2C tenant. Learn more about Azure AD B2C billing at [aka.ms/b2cBilling](https://aka.ms/b2cBilling)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct B2cResourceSku {
    #[doc = "The name of the SKU for the tenant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<b2c_resource_sku::Name>,
    #[doc = "The tier of the tenant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<b2c_resource_sku::Tier>,
}
impl B2cResourceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod b2c_resource_sku {
    use super::*;
    #[doc = "The name of the SKU for the tenant."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Standard,
        PremiumP1,
        PremiumP2,
    }
    #[doc = "The tier of the tenant."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        A0,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct B2cTenantResource {
    #[doc = "The type of the B2C tenant resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<b2c_tenant_resource::Type>,
    #[doc = "SKU properties of the Azure AD B2C tenant. Learn more about Azure AD B2C billing at [aka.ms/b2cBilling](https://aka.ms/b2cBilling)."]
    pub sku: B2cResourceSku,
    #[doc = "Properties of the Azure AD B2C tenant Azure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<B2cTenantResourceProperties>,
    #[doc = "An identifier that represents the B2C tenant resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the B2C tenant resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The location in which the resource is hosted and data resides. Can be one of 'United States', 'Europe', 'Asia Pacific', or 'Australia' (preview). Refer to [this documentation](https://aka.ms/B2CDataResidency) for more information."]
    pub location: String,
    #[doc = "Resource Tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl B2cTenantResource {
    pub fn new(sku: B2cResourceSku, location: String) -> Self {
        Self {
            type_: None,
            sku,
            properties: None,
            id: None,
            name: None,
            location,
            tags: None,
        }
    }
}
pub mod b2c_tenant_resource {
    use super::*;
    #[doc = "The type of the B2C tenant resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.AzureActiveDirectory/b2cDirectories")]
        MicrosoftAzureActiveDirectoryB2cDirectories,
    }
}
#[doc = "The collection of Azure AD B2C tenant resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct B2cTenantResourceList {
    #[doc = "List of guest usages resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<B2cTenantResource>,
}
impl azure_core::Continuable for B2cTenantResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl B2cTenantResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Azure AD B2C tenant Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct B2cTenantResourceProperties {
    #[doc = "The billing configuration for the tenant."]
    #[serde(rename = "billingConfig", default, skip_serializing_if = "Option::is_none")]
    pub billing_config: Option<b2c_tenant_resource_properties::BillingConfig>,
    #[doc = "An identifier of the B2C tenant."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl B2cTenantResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod b2c_tenant_resource_properties {
    use super::*;
    #[doc = "The billing configuration for the tenant."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct BillingConfig {
        #[doc = "The type of billing. Will be MAU for all new customers. If 'Auths', it can be updated to 'MAU'. Cannot be changed if value is 'MAU'. Learn more about Azure AD B2C billing at [aka.ms/b2cBilling](https://aka.ms/b2cbilling)."]
        #[serde(rename = "billingType", default, skip_serializing_if = "Option::is_none")]
        pub billing_type: Option<billing_config::BillingType>,
        #[doc = "The data from which the billing type took effect"]
        #[serde(rename = "effectiveStartDateUtc", default, skip_serializing_if = "Option::is_none")]
        pub effective_start_date_utc: Option<String>,
    }
    impl BillingConfig {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod billing_config {
        use super::*;
        #[doc = "The type of billing. Will be MAU for all new customers. If 'Auths', it can be updated to 'MAU'. Cannot be changed if value is 'MAU'. Learn more about Azure AD B2C billing at [aka.ms/b2cBilling](https://aka.ms/b2cbilling)."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum BillingType {
            #[serde(rename = "MAU")]
            Mau,
            Auths,
        }
    }
}
#[doc = "The request body to update the Azure AD B2C tenant resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct B2cTenantUpdateRequest {
    #[doc = "SKU properties of the Azure AD B2C tenant. Learn more about Azure AD B2C billing at [aka.ms/b2cBilling](https://aka.ms/b2cBilling)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<B2cResourceSku>,
    #[doc = "Properties of the Azure AD B2C tenant Azure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<B2cTenantResourceProperties>,
    #[doc = "Resource Tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl B2cTenantUpdateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The information required to check the availability of the name for the tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityRequestBody {
    #[doc = "The domain name to check for availability."]
    pub name: String,
    #[doc = "Country code of Azure tenant (e.g. 'US'). Refer to [aka.ms/B2CDataResidency](https://aka.ms/B2CDataResidency) to see valid country codes and corresponding data residency locations. If you do not see a country code in an valid data residency location, choose one from the list."]
    #[serde(rename = "countryCode")]
    pub country_code: CountryCode,
}
impl CheckNameAvailabilityRequestBody {
    pub fn new(name: String, country_code: CountryCode) -> Self {
        Self { name, country_code }
    }
}
#[doc = "An error response for a resource management request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl azure_core::Continuable for CloudError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type CountryCode = String;
#[doc = "These properties are used to create the Azure AD B2C tenant. These properties are not part of the Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateTenantProperties {
    #[doc = "The display name of the B2C tenant."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Country code of Azure tenant (e.g. 'US'). Refer to [aka.ms/B2CDataResidency](https://aka.ms/B2CDataResidency) to see valid country codes and corresponding data residency locations. If you do not see a country code in an valid data residency location, choose one from the list."]
    #[serde(rename = "countryCode", default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<CountryCode>,
}
impl CreateTenantProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The information needed to create the Azure AD B2C tenant and corresponding Azure resource, which is used for billing purposes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTenantRequestBody {
    #[doc = "The location in which the resource is hosted and data resides. Can be one of 'United States', 'Europe', 'Asia Pacific', or 'Australia' (preview). Refer to [this documentation](https://aka.ms/B2CDataResidency) for more information."]
    pub location: String,
    pub properties: create_tenant_request_body::Properties,
    #[doc = "SKU properties of the Azure AD B2C tenant. Learn more about Azure AD B2C billing at [aka.ms/b2cBilling](https://aka.ms/b2cBilling)."]
    pub sku: B2cResourceSku,
    #[doc = "Resource Tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CreateTenantRequestBody {
    pub fn new(location: String, properties: create_tenant_request_body::Properties, sku: B2cResourceSku) -> Self {
        Self {
            location,
            properties,
            sku,
            tags: None,
        }
    }
}
pub mod create_tenant_request_body {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "These properties are used to create the Azure AD B2C tenant. These properties are not part of the Azure resource."]
        #[serde(rename = "createTenantProperties", default, skip_serializing_if = "Option::is_none")]
        pub create_tenant_properties: Option<CreateTenantProperties>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
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
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
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
    pub details: Vec<ErrorResponse>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the reason for the 'nameAvailable' value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NameAvailabilityReason {
    AlreadyExists,
    Invalid,
}
#[doc = "Response of the CheckNameAvailability operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailabilityResponse {
    #[doc = "Description of the reason if name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "True if the name is available and can be used to create a new tenant. Otherwise false."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Describes the reason for the 'nameAvailable' value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<NameAvailabilityReason>,
}
impl NameAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Microsoft.AzureActiveDirectory REST API operation."]
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
        #[doc = "Service provider: Microsoft.AzureActiveDirectory."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: GuestUsages, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Friendly name of the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of listing operations for the resourceProvider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of cpim service operations supported by the Microsoft.AzureActiveDirectory resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
