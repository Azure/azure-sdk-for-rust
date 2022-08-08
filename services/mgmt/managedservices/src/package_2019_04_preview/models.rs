#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Authorization tuple containing principal Id (of user/service principal/security group) and role definition id."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Authorization {
    #[doc = "Principal Id of the security group/service principal/user that would be assigned permissions to the projected subscription"]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "Display name of the principal Id."]
    #[serde(rename = "principalIdDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub principal_id_display_name: Option<String>,
    #[doc = "The role definition identifier. This role will define all the permissions that the security group/service principal/user must have on the projected subscription. This role cannot be an owner role."]
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[doc = "The delegatedRoleDefinitionIds field is required when the roleDefinitionId refers to the User Access Administrator Role. It is the list of role definition ids which define all the permissions that the user in the authorization can assign to other security groups/service principals/users."]
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
#[doc = "Error response indicates Azure Resource Manager is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    #[doc = "Error code."]
    pub code: String,
    #[doc = "Error message indicating why the operation failed."]
    pub message: String,
    #[doc = "Internal error details."]
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
    #[doc = "Error response indicates Azure Resource Manager is not able to process the incoming request. The reason is provided in the error message."]
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
#[doc = "Object that describes a single Microsoft.ManagedServices operation."]
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
        #[doc = "Service provider: Microsoft.ManagedServices"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Registration definition, registration assignment etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "List of the operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of Microsoft.ManagedServices operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Plan details for the managed services."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    #[doc = "The plan name."]
    pub name: String,
    #[doc = "The publisher ID."]
    pub publisher: String,
    #[doc = "The product code."]
    pub product: String,
    #[doc = "The plan's version."]
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
#[doc = "Registration assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationAssignment {
    #[doc = "Properties of a registration assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistrationAssignmentProperties>,
    #[doc = "The fully qualified path of the registration assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Name of the registration assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl RegistrationAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of registration assignments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationAssignmentList {
    #[doc = "List of registration assignments."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RegistrationAssignment>,
    #[doc = "Link to next page of registration assignments."]
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
#[doc = "Properties of a registration assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationAssignmentProperties {
    #[doc = "Fully qualified path of the registration definition."]
    #[serde(rename = "registrationDefinitionId")]
    pub registration_definition_id: String,
    #[doc = "Current state of the registration assignment."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<registration_assignment_properties::ProvisioningState>,
    #[doc = "Registration definition inside registration assignment."]
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
    #[doc = "Current state of the registration assignment."]
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
    #[doc = "Registration definition inside registration assignment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RegistrationDefinition {
        #[doc = "Properties of registration definition inside registration assignment."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub properties: Option<registration_definition::Properties>,
        #[doc = "Plan details for the managed services."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub plan: Option<Plan>,
        #[doc = "Fully qualified path of the registration definition."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Type of the resource (Microsoft.ManagedServices/registrationDefinitions)."]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[doc = "Name of the registration definition."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
    impl RegistrationDefinition {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod registration_definition {
        use super::*;
        #[doc = "Properties of registration definition inside registration assignment."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Properties {
            #[doc = "Description of the registration definition."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub description: Option<String>,
            #[doc = "Authorization tuple containing principal id of the user/security group or service principal and id of the build-in role."]
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub authorizations: Vec<Authorization>,
            #[doc = "Name of the registration definition."]
            #[serde(rename = "registrationDefinitionName", default, skip_serializing_if = "Option::is_none")]
            pub registration_definition_name: Option<String>,
            #[doc = "Current state of the registration definition."]
            #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
            pub provisioning_state: Option<properties::ProvisioningState>,
            #[doc = "Id of the home tenant."]
            #[serde(rename = "manageeTenantId", default, skip_serializing_if = "Option::is_none")]
            pub managee_tenant_id: Option<String>,
            #[doc = "Name of the home tenant."]
            #[serde(rename = "manageeTenantName", default, skip_serializing_if = "Option::is_none")]
            pub managee_tenant_name: Option<String>,
            #[doc = "Id of the managedBy tenant."]
            #[serde(rename = "managedByTenantId", default, skip_serializing_if = "Option::is_none")]
            pub managed_by_tenant_id: Option<String>,
            #[doc = "Name of the managedBy tenant."]
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
            #[doc = "Current state of the registration definition."]
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
#[doc = "Registration definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationDefinition {
    #[doc = "Properties of a registration definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegistrationDefinitionProperties>,
    #[doc = "Plan details for the managed services."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[doc = "Fully qualified path of the registration definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Name of the registration definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl RegistrationDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of registration definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegistrationDefinitionList {
    #[doc = "List of registration definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RegistrationDefinition>,
    #[doc = "Link to next page of registration definitions."]
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
#[doc = "Properties of a registration definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistrationDefinitionProperties {
    #[doc = "Description of the registration definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Authorization tuple containing principal id of the user/security group or service principal and id of the build-in role."]
    pub authorizations: Vec<Authorization>,
    #[doc = "Name of the registration definition."]
    #[serde(rename = "registrationDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub registration_definition_name: Option<String>,
    #[doc = "Id of the managedBy tenant."]
    #[serde(rename = "managedByTenantId")]
    pub managed_by_tenant_id: String,
    #[doc = "Current state of the registration definition."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<registration_definition_properties::ProvisioningState>,
    #[doc = "Name of the managedBy tenant."]
    #[serde(rename = "managedByTenantName", default, skip_serializing_if = "Option::is_none")]
    pub managed_by_tenant_name: Option<String>,
}
impl RegistrationDefinitionProperties {
    pub fn new(authorizations: Vec<Authorization>, managed_by_tenant_id: String) -> Self {
        Self {
            description: None,
            authorizations,
            registration_definition_name: None,
            managed_by_tenant_id,
            provisioning_state: None,
            managed_by_tenant_name: None,
        }
    }
}
pub mod registration_definition_properties {
    use super::*;
    #[doc = "Current state of the registration definition."]
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
