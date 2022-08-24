#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The Azure Active Directory principal identifier and Azure built-in role that describes the access the principal will receive on the delegated resource in the managed tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Authorization {
    #[doc = "The identifier of the Azure Active Directory principal."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "The display name of the Azure Active Directory principal."]
    #[serde(rename = "principalIdDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub principal_id_display_name: Option<String>,
    #[doc = "The identifier of the Azure built-in role that defines the permissions that the Azure Active Directory principal will have on the projected scope."]
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[doc = "The delegatedRoleDefinitionIds field is required when the roleDefinitionId refers to the User Access Administrator Role. It is the list of role definition ids which define all the permissions that the user in the authorization can assign to other principals."]
    #[serde(rename = "delegatedRoleDefinitionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub delegated_role_definition_ids: Vec<String>,
}
impl Authorization {
    pub fn new(principal_id: String, role_definition_id: String) -> Self {
        Self {
            principal_id,
            principal_id_display_name: None,
            role_definition_id,
            delegated_role_definition_ids: Vec::new(),
        }
    }
}
#[doc = "Defines the Azure Active Directory principal that can approve any just-in-time access requests by the principal defined in the EligibleAuthorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EligibleApprover {
    #[doc = "The identifier of the Azure Active Directory principal."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "The display name of the Azure Active Directory principal."]
    #[serde(rename = "principalIdDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub principal_id_display_name: Option<String>,
}
impl EligibleApprover {
    pub fn new(principal_id: String) -> Self {
        Self {
            principal_id,
            principal_id_display_name: None,
        }
    }
}
#[doc = "The Azure Active Directory principal identifier, Azure built-in role, and just-in-time access policy that describes the just-in-time access the principal will receive on the delegated resource in the managed tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EligibleAuthorization {
    #[doc = "The identifier of the Azure Active Directory principal."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "The display name of the Azure Active Directory principal."]
    #[serde(rename = "principalIdDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub principal_id_display_name: Option<String>,
    #[doc = "The identifier of the Azure built-in role that defines the permissions that the Azure Active Directory principal will have on the projected scope."]
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[doc = "Just-in-time access policy setting."]
    #[serde(rename = "justInTimeAccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub just_in_time_access_policy: Option<JustInTimeAccessPolicy>,
}
impl EligibleAuthorization {
    pub fn new(principal_id: String, role_definition_id: String) -> Self {
        Self {
            principal_id,
            principal_id_display_name: None,
            role_definition_id,
            just_in_time_access_policy: None,
        }
    }
}
#[doc = "The error response indicating why the incoming request wasn’t able to be processed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    #[doc = "The error code."]
    pub code: String,
    #[doc = "The error message indicating why the operation failed."]
    pub message: String,
    #[doc = "The internal error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDefinition>,
}
impl ErrorDefinition {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
        }
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error response indicating why the incoming request wasn’t able to be processed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
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
#[doc = "Just-in-time access policy setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JustInTimeAccessPolicy {
    #[doc = "The multi-factor authorization provider to be used for just-in-time access requests."]
    #[serde(rename = "multiFactorAuthProvider")]
    pub multi_factor_auth_provider: just_in_time_access_policy::MultiFactorAuthProvider,
    #[doc = "The maximum access duration in ISO 8601 format for just-in-time access requests."]
    #[serde(rename = "maximumActivationDuration", default, skip_serializing_if = "Option::is_none")]
    pub maximum_activation_duration: Option<String>,
    #[doc = "The list of managedByTenant approvers for the eligible authorization."]
    #[serde(rename = "managedByTenantApprovers", default, skip_serializing_if = "Vec::is_empty")]
    pub managed_by_tenant_approvers: Vec<EligibleApprover>,
}
impl JustInTimeAccessPolicy {
    pub fn new(multi_factor_auth_provider: just_in_time_access_policy::MultiFactorAuthProvider) -> Self {
        Self {
            multi_factor_auth_provider,
            maximum_activation_duration: None,
            managed_by_tenant_approvers: Vec::new(),
        }
    }
}
pub mod just_in_time_access_policy {
    use super::*;
    #[doc = "The multi-factor authorization provider to be used for just-in-time access requests."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MultiFactorAuthProvider")]
    pub enum MultiFactorAuthProvider {
        Azure,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MultiFactorAuthProvider {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MultiFactorAuthProvider {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MultiFactorAuthProvider {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Azure => serializer.serialize_unit_variant("MultiFactorAuthProvider", 0u32, "Azure"),
                Self::None => serializer.serialize_unit_variant("MultiFactorAuthProvider", 1u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for MultiFactorAuthProvider {
        fn default() -> Self {
            Self::None
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketplaceRegistrationDefinition {
    #[doc = "The properties of the marketplace registration definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MarketplaceRegistrationDefinitionProperties>,
    #[doc = "The details for the Managed Services offer’s plan in Azure Marketplace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[doc = "The fully qualified path of the marketplace registration definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the Azure resource (Microsoft.ManagedServices/marketplaceRegistrationDefinitions)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the marketplace registration definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl MarketplaceRegistrationDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of marketplace registration definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketplaceRegistrationDefinitionList {
    #[doc = "The list of marketplace registration definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MarketplaceRegistrationDefinition>,
    #[doc = "The link to the next page of marketplace registration definitions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MarketplaceRegistrationDefinitionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MarketplaceRegistrationDefinitionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the marketplace registration definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceRegistrationDefinitionProperties {
    #[doc = "The identifier of the managedBy tenant."]
    #[serde(rename = "managedByTenantId")]
    pub managed_by_tenant_id: String,
    #[doc = "The collection of authorization objects describing the access Azure Active Directory principals in the managedBy tenant will receive on the delegated resource in the managed tenant."]
    pub authorizations: Vec<Authorization>,
    #[doc = "The collection of eligible authorization objects describing the just-in-time access Azure Active Directory principals in the managedBy tenant will receive on the delegated resource in the managed tenant."]
    #[serde(rename = "eligibleAuthorizations", default, skip_serializing_if = "Vec::is_empty")]
    pub eligible_authorizations: Vec<EligibleAuthorization>,
    #[doc = "The marketplace offer display name."]
    #[serde(rename = "offerDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub offer_display_name: Option<String>,
    #[doc = "The marketplace publisher display name."]
    #[serde(rename = "publisherDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_display_name: Option<String>,
    #[doc = "The marketplace plan display name."]
    #[serde(rename = "planDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub plan_display_name: Option<String>,
}
impl MarketplaceRegistrationDefinitionProperties {
    pub fn new(managed_by_tenant_id: String, authorizations: Vec<Authorization>) -> Self {
        Self {
            managed_by_tenant_id,
            authorizations,
            eligible_authorizations: Vec::new(),
            offer_display_name: None,
            publisher_display_name: None,
            plan_display_name: None,
        }
    }
}
#[doc = "The object that describes a single Microsoft.ManagedServices operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The operation name with the format: {provider}/{resource}/{operation}"]
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
        #[doc = "The service provider."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The resource on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The operation type."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The list of the operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "The list of Microsoft.ManagedServices operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details for the Managed Services offer’s plan in Azure Marketplace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    #[doc = "Azure Marketplace plan name."]
    pub name: String,
    #[doc = "Azure Marketplace publisher ID."]
    pub publisher: String,
    #[doc = "Azure Marketplace product code."]
    pub product: String,
    #[doc = "Azure Marketplace plan's version."]
    pub version: String,
}
impl Plan {
    pub fn new(name: String, publisher: String, product: String, version: String) -> Self {
        Self {
            name,
            publisher,
            product,
            version,
        }
    }
}
#[doc = "The registration assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationAssignment {
    #[doc = "The properties of the registration assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistrationAssignmentProperties>,
    #[doc = "The fully qualified path of the registration assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the Azure resource (Microsoft.ManagedServices/registrationAssignments)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the registration assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl RegistrationAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of registration assignments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationAssignmentList {
    #[doc = "The list of registration assignments."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RegistrationAssignment>,
    #[doc = "The link to the next page of registration assignments."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RegistrationAssignmentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RegistrationAssignmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the registration assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationAssignmentProperties {
    #[doc = "The fully qualified path of the registration definition."]
    #[serde(rename = "registrationDefinitionId")]
    pub registration_definition_id: String,
    #[doc = "The current provisioning state of the registration assignment."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<registration_assignment_properties::ProvisioningState>,
    #[doc = "The registration definition associated with the registration assignment."]
    #[serde(rename = "registrationDefinition", default, skip_serializing_if = "Option::is_none")]
    pub registration_definition: Option<registration_assignment_properties::RegistrationDefinition>,
}
impl RegistrationAssignmentProperties {
    pub fn new(registration_definition_id: String) -> Self {
        Self {
            registration_definition_id,
            provisioning_state: None,
            registration_definition: None,
        }
    }
}
pub mod registration_assignment_properties {
    use super::*;
    #[doc = "The current provisioning state of the registration assignment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        NotSpecified,
        Accepted,
        Running,
        Ready,
        Creating,
        Created,
        Deleting,
        Deleted,
        Canceled,
        Failed,
        Succeeded,
        Updating,
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
                Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 0u32, "NotSpecified"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Accepted"),
                Self::Running => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Running"),
                Self::Ready => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Ready"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Created"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleting"),
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleted"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 10u32, "Succeeded"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 11u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The registration definition associated with the registration assignment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RegistrationDefinition {
        #[doc = "The properties of the registration definition associated with the registration assignment."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub properties: Option<registration_definition::Properties>,
        #[doc = "The details for the Managed Services offer’s plan in Azure Marketplace."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub plan: Option<Plan>,
        #[doc = "The fully qualified path of the registration definition."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "The type of the Azure resource (Microsoft.ManagedServices/registrationDefinitions)."]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[doc = "The name of the registration definition."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[doc = "Metadata pertaining to creation and last modification of the resource."]
        #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
        pub system_data: Option<SystemData>,
    }
    impl RegistrationDefinition {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod registration_definition {
        use super::*;
        #[doc = "The properties of the registration definition associated with the registration assignment."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Properties {
            #[doc = "The description of the registration definition."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub description: Option<String>,
            #[doc = "The collection of authorization objects describing the access Azure Active Directory principals in the managedBy tenant will receive on the delegated resource in the managed tenant."]
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub authorizations: Vec<Authorization>,
            #[doc = "The collection of eligible authorization objects describing the just-in-time access Azure Active Directory principals in the managedBy tenant will receive on the delegated resource in the managed tenant."]
            #[serde(rename = "eligibleAuthorizations", default, skip_serializing_if = "Vec::is_empty")]
            pub eligible_authorizations: Vec<EligibleAuthorization>,
            #[doc = "The name of the registration definition."]
            #[serde(rename = "registrationDefinitionName", default, skip_serializing_if = "Option::is_none")]
            pub registration_definition_name: Option<String>,
            #[doc = "The current provisioning state of the registration definition."]
            #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
            pub provisioning_state: Option<properties::ProvisioningState>,
            #[doc = "The identifier of the managed tenant."]
            #[serde(rename = "manageeTenantId", default, skip_serializing_if = "Option::is_none")]
            pub managee_tenant_id: Option<String>,
            #[doc = "The name of the managed tenant."]
            #[serde(rename = "manageeTenantName", default, skip_serializing_if = "Option::is_none")]
            pub managee_tenant_name: Option<String>,
            #[doc = "The identifier of the managedBy tenant."]
            #[serde(rename = "managedByTenantId", default, skip_serializing_if = "Option::is_none")]
            pub managed_by_tenant_id: Option<String>,
            #[doc = "The name of the managedBy tenant."]
            #[serde(rename = "managedByTenantName", default, skip_serializing_if = "Option::is_none")]
            pub managed_by_tenant_name: Option<String>,
        }
        impl Properties {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod properties {
            use super::*;
            #[doc = "The current provisioning state of the registration definition."]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(remote = "ProvisioningState")]
            pub enum ProvisioningState {
                NotSpecified,
                Accepted,
                Running,
                Ready,
                Creating,
                Created,
                Deleting,
                Deleted,
                Canceled,
                Failed,
                Succeeded,
                Updating,
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
                        Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 0u32, "NotSpecified"),
                        Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Accepted"),
                        Self::Running => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Running"),
                        Self::Ready => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Ready"),
                        Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
                        Self::Created => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Created"),
                        Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleting"),
                        Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleted"),
                        Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Canceled"),
                        Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Failed"),
                        Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 10u32, "Succeeded"),
                        Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 11u32, "Updating"),
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
    }
}
#[doc = "The registration definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationDefinition {
    #[doc = "The properties of a registration definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistrationDefinitionProperties>,
    #[doc = "The details for the Managed Services offer’s plan in Azure Marketplace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[doc = "The fully qualified path of the registration definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the Azure resource (Microsoft.ManagedServices/registrationDefinitions)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the registration definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl RegistrationDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of registration definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationDefinitionList {
    #[doc = "The list of registration definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RegistrationDefinition>,
    #[doc = "The link to the next page of registration definitions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RegistrationDefinitionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RegistrationDefinitionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a registration definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationDefinitionProperties {
    #[doc = "The description of the registration definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The collection of authorization objects describing the access Azure Active Directory principals in the managedBy tenant will receive on the delegated resource in the managed tenant."]
    pub authorizations: Vec<Authorization>,
    #[doc = "The collection of eligible authorization objects describing the just-in-time access Azure Active Directory principals in the managedBy tenant will receive on the delegated resource in the managed tenant."]
    #[serde(rename = "eligibleAuthorizations", default, skip_serializing_if = "Vec::is_empty")]
    pub eligible_authorizations: Vec<EligibleAuthorization>,
    #[doc = "The name of the registration definition."]
    #[serde(rename = "registrationDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub registration_definition_name: Option<String>,
    #[doc = "The identifier of the managedBy tenant."]
    #[serde(rename = "managedByTenantId")]
    pub managed_by_tenant_id: String,
    #[doc = "The current provisioning state of the registration definition."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<registration_definition_properties::ProvisioningState>,
    #[doc = "The identifier of the managed tenant."]
    #[serde(rename = "manageeTenantId", default, skip_serializing_if = "Option::is_none")]
    pub managee_tenant_id: Option<String>,
    #[doc = "The name of the managed tenant."]
    #[serde(rename = "manageeTenantName", default, skip_serializing_if = "Option::is_none")]
    pub managee_tenant_name: Option<String>,
    #[doc = "The name of the managedBy tenant."]
    #[serde(rename = "managedByTenantName", default, skip_serializing_if = "Option::is_none")]
    pub managed_by_tenant_name: Option<String>,
}
impl RegistrationDefinitionProperties {
    pub fn new(authorizations: Vec<Authorization>, managed_by_tenant_id: String) -> Self {
        Self {
            description: None,
            authorizations,
            eligible_authorizations: Vec::new(),
            registration_definition_name: None,
            managed_by_tenant_id,
            provisioning_state: None,
            managee_tenant_id: None,
            managee_tenant_name: None,
            managed_by_tenant_name: None,
        }
    }
}
pub mod registration_definition_properties {
    use super::*;
    #[doc = "The current provisioning state of the registration definition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        NotSpecified,
        Accepted,
        Running,
        Ready,
        Creating,
        Created,
        Deleting,
        Deleted,
        Canceled,
        Failed,
        Succeeded,
        Updating,
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
                Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 0u32, "NotSpecified"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Accepted"),
                Self::Running => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Running"),
                Self::Ready => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Ready"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Created"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleting"),
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleted"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 10u32, "Succeeded"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 11u32, "Updating"),
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
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
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
