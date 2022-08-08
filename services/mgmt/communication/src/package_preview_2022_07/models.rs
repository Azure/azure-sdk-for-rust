#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The check availability request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityRequest {
    #[doc = "The name of the resource for which availability needs to be checked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckNameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The check availability result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResponse {
    #[doc = "Indicates if the resource name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the given name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_response::Reason>,
    #[doc = "Detailed reason why the given name is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_response {
    use super::*;
    #[doc = "The reason why the given name is not available."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        Invalid,
        AlreadyExists,
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
                Self::Invalid => serializer.serialize_unit_variant("Reason", 0u32, "Invalid"),
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 1u32, "AlreadyExists"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A class representing the access keys of a CommunicationService."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunicationServiceKeys {
    #[doc = "The primary access key."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "The secondary access key."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "CommunicationService connection string constructed via the primaryKey"]
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[doc = "CommunicationService connection string constructed via the secondaryKey"]
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
}
impl CommunicationServiceKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class that describes the properties of the CommunicationService."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationServiceProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<communication_service_properties::ProvisioningState>,
    #[doc = "FQDN of the CommunicationService instance."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "The location where the communication service stores its data at rest."]
    #[serde(rename = "dataLocation")]
    pub data_location: String,
    #[doc = "Resource ID of an Azure Notification Hub linked to this resource."]
    #[serde(rename = "notificationHubId", default, skip_serializing_if = "Option::is_none")]
    pub notification_hub_id: Option<String>,
    #[doc = "Version of the CommunicationService resource. Probably you need the same or higher version of client SDKs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The immutable resource Id of the communication service."]
    #[serde(rename = "immutableResourceId", default, skip_serializing_if = "Option::is_none")]
    pub immutable_resource_id: Option<String>,
    #[doc = "List of email Domain resource Ids."]
    #[serde(rename = "linkedDomains", default, skip_serializing_if = "Option::is_none")]
    pub linked_domains: Option<DomainsResourceList>,
}
impl CommunicationServiceProperties {
    pub fn new(data_location: String) -> Self {
        Self {
            provisioning_state: None,
            host_name: None,
            data_location,
            notification_hub_id: None,
            version: None,
            immutable_resource_id: None,
            linked_domains: None,
        }
    }
}
pub mod communication_service_properties {
    use super::*;
    #[doc = "Provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Unknown,
        Succeeded,
        Failed,
        Canceled,
        Running,
        Creating,
        Updating,
        Deleting,
        Moving,
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
                Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Unknown"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
                Self::Running => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Running"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Moving"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A class representing a CommunicationService resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunicationServiceResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "A class that describes the properties of the CommunicationService."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CommunicationServiceProperties>,
}
impl CommunicationServiceResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Object that includes an array of CommunicationServices and a possible link for next set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunicationServiceResourceList {
    #[doc = "List of CommunicationService"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CommunicationServiceResource>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CommunicationServiceResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CommunicationServiceResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class representing update parameters for CommunicationService resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunicationServiceResourceUpdate {
    #[serde(flatten)]
    pub tagged_resource: TaggedResource,
    #[doc = "A class that describes the properties that can be updated for CommunicationService resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CommunicationServiceUpdateProperties>,
}
impl CommunicationServiceResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class that describes the properties that can be updated for CommunicationService resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommunicationServiceUpdateProperties {
    #[doc = "List of email Domain resource Ids."]
    #[serde(rename = "linkedDomains", default, skip_serializing_if = "Option::is_none")]
    pub linked_domains: Option<DomainsResourceList>,
}
impl CommunicationServiceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class that represents a VerificationStatus record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DnsRecord {
    #[doc = "Type of the DNS record. Example: TXT"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Name of the DNS record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Value of the DNS record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Represents an expiry time in seconds to represent how long this entry can be cached by the resolver, default = 3600sec."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
}
impl DnsRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes how a Domains resource is being managed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DomainManagement")]
pub enum DomainManagement {
    AzureManaged,
    CustomerManaged,
    CustomerManagedInExchangeOnline,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DomainManagement {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DomainManagement {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DomainManagement {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureManaged => serializer.serialize_unit_variant("DomainManagement", 0u32, "AzureManaged"),
            Self::CustomerManaged => serializer.serialize_unit_variant("DomainManagement", 1u32, "CustomerManaged"),
            Self::CustomerManagedInExchangeOnline => {
                serializer.serialize_unit_variant("DomainManagement", 2u32, "CustomerManagedInExchangeOnline")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A class that describes the properties of a Domains resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<domain_properties::ProvisioningState>,
    #[doc = "The location where the Domains resource data is stored at rest."]
    #[serde(rename = "dataLocation", default, skip_serializing_if = "Option::is_none")]
    pub data_location: Option<String>,
    #[doc = "P2 sender domain that is displayed to the email recipients [RFC 5322]."]
    #[serde(rename = "fromSenderDomain", default, skip_serializing_if = "Option::is_none")]
    pub from_sender_domain: Option<String>,
    #[doc = "P1 sender domain that is present on the email envelope [RFC 5321]."]
    #[serde(rename = "mailFromSenderDomain", default, skip_serializing_if = "Option::is_none")]
    pub mail_from_sender_domain: Option<String>,
    #[doc = "Describes how a Domains resource is being managed."]
    #[serde(rename = "domainManagement")]
    pub domain_management: DomainManagement,
    #[doc = "List of VerificationStatusRecord"]
    #[serde(rename = "verificationStates", default, skip_serializing_if = "Option::is_none")]
    pub verification_states: Option<domain_properties::VerificationStates>,
    #[doc = "List of DnsRecord"]
    #[serde(rename = "verificationRecords", default, skip_serializing_if = "Option::is_none")]
    pub verification_records: Option<domain_properties::VerificationRecords>,
    #[doc = "Collection of valid sender usernames. This is a key-value pair where key=username and value=display name."]
    #[serde(rename = "validSenderUsernames", default, skip_serializing_if = "Option::is_none")]
    pub valid_sender_usernames: Option<ValidSenderUsernameCollection>,
    #[doc = "Describes whether user engagement tracking is enabled or disabled."]
    #[serde(rename = "userEngagementTracking", default, skip_serializing_if = "Option::is_none")]
    pub user_engagement_tracking: Option<UserEngagementTracking>,
}
impl DomainProperties {
    pub fn new(domain_management: DomainManagement) -> Self {
        Self {
            provisioning_state: None,
            data_location: None,
            from_sender_domain: None,
            mail_from_sender_domain: None,
            domain_management,
            verification_states: None,
            verification_records: None,
            valid_sender_usernames: None,
            user_engagement_tracking: None,
        }
    }
}
pub mod domain_properties {
    use super::*;
    #[doc = "Provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Unknown,
        Succeeded,
        Failed,
        Canceled,
        Running,
        Creating,
        Updating,
        Deleting,
        Moving,
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
                Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Unknown"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
                Self::Running => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Running"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Moving"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "List of VerificationStatusRecord"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct VerificationStates {
        #[doc = "A class that represents a VerificationStatus record."]
        #[serde(rename = "Domain", default, skip_serializing_if = "Option::is_none")]
        pub domain: Option<VerificationStatusRecord>,
        #[doc = "A class that represents a VerificationStatus record."]
        #[serde(rename = "SPF", default, skip_serializing_if = "Option::is_none")]
        pub spf: Option<VerificationStatusRecord>,
        #[doc = "A class that represents a VerificationStatus record."]
        #[serde(rename = "DKIM", default, skip_serializing_if = "Option::is_none")]
        pub dkim: Option<VerificationStatusRecord>,
        #[doc = "A class that represents a VerificationStatus record."]
        #[serde(rename = "DKIM2", default, skip_serializing_if = "Option::is_none")]
        pub dkim2: Option<VerificationStatusRecord>,
        #[doc = "A class that represents a VerificationStatus record."]
        #[serde(rename = "DMARC", default, skip_serializing_if = "Option::is_none")]
        pub dmarc: Option<VerificationStatusRecord>,
    }
    impl VerificationStates {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "List of DnsRecord"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct VerificationRecords {
        #[doc = "A class that represents a VerificationStatus record."]
        #[serde(rename = "Domain", default, skip_serializing_if = "Option::is_none")]
        pub domain: Option<DnsRecord>,
        #[doc = "A class that represents a VerificationStatus record."]
        #[serde(rename = "SPF", default, skip_serializing_if = "Option::is_none")]
        pub spf: Option<DnsRecord>,
        #[doc = "A class that represents a VerificationStatus record."]
        #[serde(rename = "DKIM", default, skip_serializing_if = "Option::is_none")]
        pub dkim: Option<DnsRecord>,
        #[doc = "A class that represents a VerificationStatus record."]
        #[serde(rename = "DKIM2", default, skip_serializing_if = "Option::is_none")]
        pub dkim2: Option<DnsRecord>,
        #[doc = "A class that represents a VerificationStatus record."]
        #[serde(rename = "DMARC", default, skip_serializing_if = "Option::is_none")]
        pub dmarc: Option<DnsRecord>,
    }
    impl VerificationRecords {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A class representing a Domains resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "A class that describes the properties of a Domains resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DomainProperties>,
}
impl DomainResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Object that includes an array of Domains resource and a possible link for next set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainResourceList {
    #[doc = "List of Domains resource"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DomainResource>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DomainResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DomainResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type DomainsResourceList = Vec<String>;
#[doc = "A class that describes the properties of the EmailService."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailServiceProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<email_service_properties::ProvisioningState>,
    #[doc = "The location where the email service stores its data at rest."]
    #[serde(rename = "dataLocation")]
    pub data_location: String,
}
impl EmailServiceProperties {
    pub fn new(data_location: String) -> Self {
        Self {
            provisioning_state: None,
            data_location,
        }
    }
}
pub mod email_service_properties {
    use super::*;
    #[doc = "Provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Unknown,
        Succeeded,
        Failed,
        Canceled,
        Running,
        Creating,
        Updating,
        Deleting,
        Moving,
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
                Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Unknown"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Canceled"),
                Self::Running => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Running"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Moving"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A class representing an EmailService resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailServiceResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "A class that describes the properties of the EmailService."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EmailServiceProperties>,
}
impl EmailServiceResource {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Object that includes an array of EmailServices and a possible link for next set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailServiceResourceList {
    #[doc = "List of EmailService"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EmailServiceResource>,
    #[doc = "The URL the client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EmailServiceResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EmailServiceResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class representing update parameters for EmailService resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmailServiceResourceUpdate {
    #[serde(flatten)]
    pub tagged_resource: TaggedResource,
}
impl EmailServiceResourceUpdate {
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
#[doc = "Description of an Azure Notification Hub to link to the communication service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkNotificationHubParameters {
    #[doc = "The resource ID of the notification hub"]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "Connection string for the notification hub"]
    #[serde(rename = "connectionString")]
    pub connection_string: String,
}
impl LinkNotificationHubParameters {
    pub fn new(resource_id: String, connection_string: String) -> Self {
        Self {
            resource_id,
            connection_string,
        }
    }
}
#[doc = "A notification hub that has been linked to the communication service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedNotificationHub {
    #[doc = "The resource ID of the notification hub"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl LinkedNotificationHub {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data POST-ed to the nameAvailability action"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailabilityParameters {
    #[serde(flatten)]
    pub check_name_availability_request: CheckNameAvailabilityRequest,
}
impl NameAvailabilityParameters {
    pub fn new() -> Self {
        Self {
            check_name_availability_request: CheckNameAvailabilityRequest::default(),
        }
    }
}
#[doc = "Details of a REST API operation, returned from the Resource Provider Operations API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation, as per Resource-Based Access Control (RBAC). Examples: \"Microsoft.Compute/virtualMachines/write\", \"Microsoft.Compute/virtualMachines/capture/action\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Localized display information for this particular operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name, e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly name of the resource type related to this operation. E.g. \"Virtual Machines\" or \"Job Schedule Collections\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The concise, localized friendly name for the operation; suitable for dropdowns. E.g. \"Create or Update Virtual Machine\", \"Restart Virtual Machine\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The short, localized friendly description of the operation; suitable for tool tips and detailed views."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Origin {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Origin {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Origin {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
                Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        Internal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
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
#[doc = "Parameters describes the request to regenerate access keys"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegenerateKeyParameters {
    #[doc = "The keyType to regenerate. Must be either 'primary' or 'secondary'(case-insensitive)."]
    #[serde(rename = "keyType", default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<regenerate_key_parameters::KeyType>,
}
impl RegenerateKeyParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod regenerate_key_parameters {
    use super::*;
    #[doc = "The keyType to regenerate. Must be either 'primary' or 'secondary'(case-insensitive)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        Primary,
        Secondary,
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
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
#[doc = "An ARM resource with that can accept tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TaggedResource {
    #[doc = "Tags of the service which is a list of key value pairs that describe the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TaggedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[doc = "A class that describes the updatable properties of a Domains resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateDomainProperties {
    #[doc = "Collection of valid sender usernames. This is a key-value pair where key=username and value=display name."]
    #[serde(rename = "validSenderUsernames", default, skip_serializing_if = "Option::is_none")]
    pub valid_sender_usernames: Option<ValidSenderUsernameCollection>,
    #[doc = "Describes whether user engagement tracking is enabled or disabled."]
    #[serde(rename = "userEngagementTracking", default, skip_serializing_if = "Option::is_none")]
    pub user_engagement_tracking: Option<UserEngagementTracking>,
}
impl UpdateDomainProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A class that describes the PATCH request parameters of a Domains resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateDomainRequestParameters {
    #[serde(flatten)]
    pub tagged_resource: TaggedResource,
    #[doc = "A class that describes the updatable properties of a Domains resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateDomainProperties>,
}
impl UpdateDomainRequestParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes whether user engagement tracking is enabled or disabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UserEngagementTracking")]
pub enum UserEngagementTracking {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UserEngagementTracking {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UserEngagementTracking {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UserEngagementTracking {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("UserEngagementTracking", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("UserEngagementTracking", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Collection of valid sender usernames. This is a key-value pair where key=username and value=display name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidSenderUsernameCollection {}
impl ValidSenderUsernameCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input parameter for verification APIs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerificationParameter {
    #[doc = "Type of verification."]
    #[serde(rename = "verificationType")]
    pub verification_type: verification_parameter::VerificationType,
}
impl VerificationParameter {
    pub fn new(verification_type: verification_parameter::VerificationType) -> Self {
        Self { verification_type }
    }
}
pub mod verification_parameter {
    use super::*;
    #[doc = "Type of verification."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VerificationType")]
    pub enum VerificationType {
        Domain,
        #[serde(rename = "SPF")]
        Spf,
        #[serde(rename = "DKIM")]
        Dkim,
        #[serde(rename = "DKIM2")]
        Dkim2,
        #[serde(rename = "DMARC")]
        Dmarc,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VerificationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VerificationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VerificationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Domain => serializer.serialize_unit_variant("VerificationType", 0u32, "Domain"),
                Self::Spf => serializer.serialize_unit_variant("VerificationType", 1u32, "SPF"),
                Self::Dkim => serializer.serialize_unit_variant("VerificationType", 2u32, "DKIM"),
                Self::Dkim2 => serializer.serialize_unit_variant("VerificationType", 3u32, "DKIM2"),
                Self::Dmarc => serializer.serialize_unit_variant("VerificationType", 4u32, "DMARC"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A class that represents a VerificationStatus record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VerificationStatusRecord {
    #[doc = "Status of the verification operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<verification_status_record::Status>,
    #[doc = "Error code. This property will only be present if the status is UnableToVerify."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}
impl VerificationStatusRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod verification_status_record {
    use super::*;
    #[doc = "Status of the verification operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        NotStarted,
        VerificationRequested,
        VerificationInProgress,
        VerificationFailed,
        Verified,
        CancellationRequested,
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
                Self::VerificationRequested => serializer.serialize_unit_variant("Status", 1u32, "VerificationRequested"),
                Self::VerificationInProgress => serializer.serialize_unit_variant("Status", 2u32, "VerificationInProgress"),
                Self::VerificationFailed => serializer.serialize_unit_variant("Status", 3u32, "VerificationFailed"),
                Self::Verified => serializer.serialize_unit_variant("Status", 4u32, "Verified"),
                Self::CancellationRequested => serializer.serialize_unit_variant("Status", 5u32, "CancellationRequested"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type VerifiedExchangeOnlineDomainList = Vec<String>;
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
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
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
