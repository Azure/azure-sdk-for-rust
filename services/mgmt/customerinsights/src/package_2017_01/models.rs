#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The AssignmentPrincipal"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentPrincipal {
    #[doc = "The principal id being assigned to."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "The Type of the principal ID."]
    #[serde(rename = "principalType")]
    pub principal_type: String,
    #[doc = "Other metadata for the principal."]
    #[serde(rename = "principalMetadata", default, skip_serializing_if = "Option::is_none")]
    pub principal_metadata: Option<serde_json::Value>,
}
impl AssignmentPrincipal {
    pub fn new(principal_id: String, principal_type: String) -> Self {
        Self {
            principal_id,
            principal_type,
            principal_metadata: None,
        }
    }
}
#[doc = "The authorization policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationPolicy {
    #[doc = "Name of the policy."]
    #[serde(rename = "policyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[doc = "The permissions associated with the policy."]
    pub permissions: Vec<PermissionTypes>,
    #[doc = "Primary key associated with the policy."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "Secondary key associated with the policy."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}
impl AuthorizationPolicy {
    pub fn new(permissions: Vec<PermissionTypes>) -> Self {
        Self {
            policy_name: None,
            permissions,
            primary_key: None,
            secondary_key: None,
        }
    }
}
#[doc = "The response of list authorization policy operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationPolicyListResult {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AuthorizationPolicyResourceFormat>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AuthorizationPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AuthorizationPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The authorization policy resource format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationPolicyResourceFormat {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The authorization policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AuthorizationPolicy>,
}
impl AuthorizationPolicyResourceFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Azure Blob connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBlobConnectorProperties {
    #[doc = "The connection KeyVault URL."]
    #[serde(rename = "connectionKeyVaultUrl")]
    pub connection_key_vault_url: String,
}
impl AzureBlobConnectorProperties {
    pub fn new(connection_key_vault_url: String) -> Self {
        Self { connection_key_vault_url }
    }
}
#[doc = "Properties of connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Connector {
    #[doc = "ID of the connector."]
    #[serde(rename = "connectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i64>,
    #[doc = "Name of the connector."]
    #[serde(rename = "connectorName", default, skip_serializing_if = "Option::is_none")]
    pub connector_name: Option<String>,
    #[doc = "Type of connector."]
    #[serde(rename = "connectorType")]
    pub connector_type: ConnectorType,
    #[doc = "Display name of the connector."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description of the connector."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The connector properties."]
    #[serde(rename = "connectorProperties")]
    pub connector_properties: serde_json::Value,
    #[doc = "The created time."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The last modified time."]
    #[serde(rename = "lastModified", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
    #[doc = "State of connector."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<connector::State>,
    #[doc = "The hub name."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "If this is an internal connector."]
    #[serde(rename = "isInternal", default, skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
}
impl Connector {
    pub fn new(connector_type: ConnectorType, connector_properties: serde_json::Value) -> Self {
        Self {
            connector_id: None,
            connector_name: None,
            connector_type,
            display_name: None,
            description: None,
            connector_properties,
            created: None,
            last_modified: None,
            state: None,
            tenant_id: None,
            is_internal: None,
        }
    }
}
pub mod connector {
    use super::*;
    #[doc = "State of connector."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Creating,
        Created,
        Ready,
        Expiring,
        Deleting,
        Failed,
    }
}
#[doc = "The response of list connector operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectorListResult {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConnectorResourceFormat>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConnectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ConnectorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The connector mapping definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectorMapping {
    #[doc = "The connector name."]
    #[serde(rename = "connectorName", default, skip_serializing_if = "Option::is_none")]
    pub connector_name: Option<String>,
    #[doc = "Type of connector."]
    #[serde(rename = "connectorType", default, skip_serializing_if = "Option::is_none")]
    pub connector_type: Option<ConnectorType>,
    #[doc = "The created time."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
    #[doc = "The last modified time."]
    #[serde(rename = "lastModified", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
    #[doc = "Defines which entity type the file should map to."]
    #[serde(rename = "entityType")]
    pub entity_type: connector_mapping::EntityType,
    #[doc = "The mapping entity name."]
    #[serde(rename = "entityTypeName")]
    pub entity_type_name: String,
    #[doc = "The connector mapping name"]
    #[serde(rename = "connectorMappingName", default, skip_serializing_if = "Option::is_none")]
    pub connector_mapping_name: Option<String>,
    #[doc = "Display name for the connector mapping."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the connector mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The DataFormat ID."]
    #[serde(rename = "dataFormatId", default, skip_serializing_if = "Option::is_none")]
    pub data_format_id: Option<String>,
    #[doc = "The connector mapping properties."]
    #[serde(rename = "mappingProperties")]
    pub mapping_properties: ConnectorMappingProperties,
    #[doc = "The next run time based on customer's settings."]
    #[serde(rename = "nextRunTime", default, with = "azure_core::date::rfc3339::option")]
    pub next_run_time: Option<time::OffsetDateTime>,
    #[doc = "The RunId."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[doc = "State of connector mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<connector_mapping::State>,
    #[doc = "The hub name."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ConnectorMapping {
    pub fn new(
        entity_type: connector_mapping::EntityType,
        entity_type_name: String,
        mapping_properties: ConnectorMappingProperties,
    ) -> Self {
        Self {
            connector_name: None,
            connector_type: None,
            created: None,
            last_modified: None,
            entity_type,
            entity_type_name,
            connector_mapping_name: None,
            display_name: None,
            description: None,
            data_format_id: None,
            mapping_properties,
            next_run_time: None,
            run_id: None,
            state: None,
            tenant_id: None,
        }
    }
}
pub mod connector_mapping {
    use super::*;
    #[doc = "Defines which entity type the file should map to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EntityType {
        None,
        Profile,
        Interaction,
        Relationship,
    }
    #[doc = "State of connector mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Creating,
        Created,
        Failed,
        Ready,
        Running,
        Stopped,
        Expiring,
    }
}
#[doc = "Connector mapping property availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectorMappingAvailability {
    #[doc = "The frequency to update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<connector_mapping_availability::Frequency>,
    #[doc = "The interval of the given frequency to use."]
    pub interval: i64,
}
impl ConnectorMappingAvailability {
    pub fn new(interval: i64) -> Self {
        Self { frequency: None, interval }
    }
}
pub mod connector_mapping_availability {
    use super::*;
    #[doc = "The frequency to update."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Frequency {
        Minute,
        Hour,
        Day,
        Week,
        Month,
    }
}
#[doc = "The complete operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectorMappingCompleteOperation {
    #[doc = "The type of completion operation."]
    #[serde(rename = "completionOperationType", default, skip_serializing_if = "Option::is_none")]
    pub completion_operation_type: Option<connector_mapping_complete_operation::CompletionOperationType>,
    #[doc = "The destination folder where files will be moved to once the import is done."]
    #[serde(rename = "destinationFolder", default, skip_serializing_if = "Option::is_none")]
    pub destination_folder: Option<String>,
}
impl ConnectorMappingCompleteOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connector_mapping_complete_operation {
    use super::*;
    #[doc = "The type of completion operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CompletionOperationType {
        DoNothing,
        DeleteFile,
        MoveFile,
    }
}
#[doc = "The error management."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectorMappingErrorManagement {
    #[doc = "The type of error management to use for the mapping."]
    #[serde(rename = "errorManagementType")]
    pub error_management_type: connector_mapping_error_management::ErrorManagementType,
    #[doc = "The error limit allowed while importing data."]
    #[serde(rename = "errorLimit", default, skip_serializing_if = "Option::is_none")]
    pub error_limit: Option<i64>,
}
impl ConnectorMappingErrorManagement {
    pub fn new(error_management_type: connector_mapping_error_management::ErrorManagementType) -> Self {
        Self {
            error_management_type,
            error_limit: None,
        }
    }
}
pub mod connector_mapping_error_management {
    use super::*;
    #[doc = "The type of error management to use for the mapping."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ErrorManagementType {
        RejectAndContinue,
        StopImport,
        RejectUntilLimit,
    }
}
#[doc = "Connector mapping property format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectorMappingFormat {
    #[doc = "The type mapping format."]
    #[serde(rename = "formatType")]
    pub format_type: connector_mapping_format::FormatType,
    #[doc = "The character that signifies a break between columns."]
    #[serde(rename = "columnDelimiter", default, skip_serializing_if = "Option::is_none")]
    pub column_delimiter: Option<String>,
    #[doc = "The oData language."]
    #[serde(rename = "acceptLanguage", default, skip_serializing_if = "Option::is_none")]
    pub accept_language: Option<String>,
    #[doc = "Quote character, used to indicate enquoted fields."]
    #[serde(rename = "quoteCharacter", default, skip_serializing_if = "Option::is_none")]
    pub quote_character: Option<String>,
    #[doc = "Escape character for quotes, can be the same as the quoteCharacter."]
    #[serde(rename = "quoteEscapeCharacter", default, skip_serializing_if = "Option::is_none")]
    pub quote_escape_character: Option<String>,
    #[doc = "Character separating array elements."]
    #[serde(rename = "arraySeparator", default, skip_serializing_if = "Option::is_none")]
    pub array_separator: Option<String>,
}
impl ConnectorMappingFormat {
    pub fn new(format_type: connector_mapping_format::FormatType) -> Self {
        Self {
            format_type,
            column_delimiter: None,
            accept_language: None,
            quote_character: None,
            quote_escape_character: None,
            array_separator: None,
        }
    }
}
pub mod connector_mapping_format {
    use super::*;
    #[doc = "The type mapping format."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FormatType {
        TextFormat,
    }
}
#[doc = "The response of list connector mapping operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectorMappingListResult {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConnectorMappingResourceFormat>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConnectorMappingListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ConnectorMappingListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The connector mapping properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectorMappingProperties {
    #[doc = "The folder path for the mapping."]
    #[serde(rename = "folderPath", default, skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
    #[doc = "The file filter for the mapping."]
    #[serde(rename = "fileFilter", default, skip_serializing_if = "Option::is_none")]
    pub file_filter: Option<String>,
    #[doc = "If the file contains a header or not."]
    #[serde(rename = "hasHeader", default, skip_serializing_if = "Option::is_none")]
    pub has_header: Option<bool>,
    #[doc = "The error management."]
    #[serde(rename = "errorManagement")]
    pub error_management: ConnectorMappingErrorManagement,
    #[doc = "Connector mapping property format."]
    pub format: ConnectorMappingFormat,
    #[doc = "Connector mapping property availability."]
    pub availability: ConnectorMappingAvailability,
    #[doc = "Ingestion mapping information at property level."]
    pub structure: Vec<ConnectorMappingStructure>,
    #[doc = "The complete operation."]
    #[serde(rename = "completeOperation")]
    pub complete_operation: ConnectorMappingCompleteOperation,
}
impl ConnectorMappingProperties {
    pub fn new(
        error_management: ConnectorMappingErrorManagement,
        format: ConnectorMappingFormat,
        availability: ConnectorMappingAvailability,
        structure: Vec<ConnectorMappingStructure>,
        complete_operation: ConnectorMappingCompleteOperation,
    ) -> Self {
        Self {
            folder_path: None,
            file_filter: None,
            has_header: None,
            error_management,
            format,
            availability,
            structure,
            complete_operation,
        }
    }
}
#[doc = "The connector mapping resource format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectorMappingResourceFormat {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The connector mapping definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConnectorMapping>,
}
impl ConnectorMappingResourceFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Connector mapping property structure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectorMappingStructure {
    #[doc = "The property name of the mapping entity."]
    #[serde(rename = "propertyName")]
    pub property_name: String,
    #[doc = "The column name of the import file."]
    #[serde(rename = "columnName")]
    pub column_name: String,
    #[doc = "Custom format specifier for input parsing."]
    #[serde(rename = "customFormatSpecifier", default, skip_serializing_if = "Option::is_none")]
    pub custom_format_specifier: Option<String>,
    #[doc = "Indicates if the column is encrypted."]
    #[serde(rename = "isEncrypted", default, skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
}
impl ConnectorMappingStructure {
    pub fn new(property_name: String, column_name: String) -> Self {
        Self {
            property_name,
            column_name,
            custom_format_specifier: None,
            is_encrypted: None,
        }
    }
}
#[doc = "The connector resource format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectorResourceFormat {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of connector."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Connector>,
}
impl ConnectorResourceFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of connector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConnectorType")]
pub enum ConnectorType {
    None,
    #[serde(rename = "CRM")]
    Crm,
    AzureBlob,
    Salesforce,
    ExchangeOnline,
    Outbound,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConnectorType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConnectorType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConnectorType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ConnectorType", 0u32, "None"),
            Self::Crm => serializer.serialize_unit_variant("ConnectorType", 1u32, "CRM"),
            Self::AzureBlob => serializer.serialize_unit_variant("ConnectorType", 2u32, "AzureBlob"),
            Self::Salesforce => serializer.serialize_unit_variant("ConnectorType", 3u32, "Salesforce"),
            Self::ExchangeOnline => serializer.serialize_unit_variant("ConnectorType", 4u32, "ExchangeOnline"),
            Self::Outbound => serializer.serialize_unit_variant("ConnectorType", 5u32, "Outbound"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The CRM connector entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CrmConnectorEntities {
    #[doc = "The logical name."]
    #[serde(rename = "logicalName")]
    pub logical_name: String,
    #[doc = "The display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Indicating whether this is profile."]
    #[serde(rename = "isProfile", default, skip_serializing_if = "Option::is_none")]
    pub is_profile: Option<bool>,
}
impl CrmConnectorEntities {
    pub fn new(logical_name: String) -> Self {
        Self {
            logical_name,
            display_name: None,
            is_profile: None,
        }
    }
}
#[doc = "The CRM connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CrmConnectorProperties {
    #[doc = "The connection string."]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    #[doc = "The organization ID."]
    #[serde(rename = "organizationId")]
    pub organization_id: String,
    #[doc = "The organization URL."]
    #[serde(rename = "organizationUrl")]
    pub organization_url: String,
    #[doc = "The entities like account, contact, opportunity."]
    pub entities: Vec<CrmConnectorEntities>,
    #[doc = "The access token."]
    #[serde(rename = "accessToken", default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}
impl CrmConnectorProperties {
    pub fn new(organization_id: String, organization_url: String, entities: Vec<CrmConnectorEntities>) -> Self {
        Self {
            connection_string: None,
            organization_id,
            organization_url,
            entities,
            access_token: None,
        }
    }
}
#[doc = "Data Source is a way for us to know the source of instances. A single type can have data coming in from multiple places. In activities we use this to determine precedence rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSource {
    #[doc = "The data source name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The data source type."]
    #[serde(rename = "dataSourceType", default, skip_serializing_if = "Option::is_none")]
    pub data_source_type: Option<data_source::DataSourceType>,
    #[doc = "The data source status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<data_source::Status>,
    #[doc = "The data source ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The data source reference id."]
    #[serde(rename = "dataSourceReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub data_source_reference_id: Option<String>,
}
impl DataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_source {
    use super::*;
    #[doc = "The data source type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSourceType")]
    pub enum DataSourceType {
        Connector,
        LinkInteraction,
        SystemDefault,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSourceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSourceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSourceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Connector => serializer.serialize_unit_variant("DataSourceType", 0u32, "Connector"),
                Self::LinkInteraction => serializer.serialize_unit_variant("DataSourceType", 1u32, "LinkInteraction"),
                Self::SystemDefault => serializer.serialize_unit_variant("DataSourceType", 2u32, "SystemDefault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The data source status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        None,
        Active,
        Deleted,
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
                Self::None => serializer.serialize_unit_variant("Status", 0u32, "None"),
                Self::Active => serializer.serialize_unit_variant("Status", 1u32, "Active"),
                Self::Deleted => serializer.serialize_unit_variant("Status", 2u32, "Deleted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The data source precedence is a way to know the precedence of each data source."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourcePrecedence {
    #[doc = "Data Source is a way for us to know the source of instances. A single type can have data coming in from multiple places. In activities we use this to determine precedence rules."]
    #[serde(rename = "dataSource", default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[doc = "the precedence value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precedence: Option<i64>,
}
impl DataSourcePrecedence {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The enriching KPI definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichingKpi {
    #[serde(flatten)]
    pub kpi_definition: KpiDefinition,
}
impl EnrichingKpi {
    pub fn new(kpi_definition: KpiDefinition) -> Self {
        Self { kpi_definition }
    }
}
#[doc = "Describes an entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityTypeDefinition {
    #[serde(flatten)]
    pub metadata_definition_base: MetadataDefinitionBase,
    #[doc = "The api entity set name. This becomes the odata entity set name for the entity Type being referred in this object."]
    #[serde(rename = "apiEntitySetName", default, skip_serializing_if = "Option::is_none")]
    pub api_entity_set_name: Option<String>,
    #[doc = "Type of entity."]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<entity_type_definition::EntityType>,
    #[doc = "The properties of the Profile."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<PropertyDefinition>,
    #[doc = "The instance count."]
    #[serde(rename = "instancesCount", default, skip_serializing_if = "Option::is_none")]
    pub instances_count: Option<i64>,
    #[doc = "The last changed time for the type definition."]
    #[serde(rename = "lastChangedUtc", default, with = "azure_core::date::rfc3339::option")]
    pub last_changed_utc: Option<time::OffsetDateTime>,
    #[doc = "Provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The schema org link. This helps ACI identify and suggest semantic models."]
    #[serde(rename = "schemaItemTypeLink", default, skip_serializing_if = "Option::is_none")]
    pub schema_item_type_link: Option<String>,
    #[doc = "The hub name."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The timestamp property name. Represents the time when the interaction or profile update happened."]
    #[serde(rename = "timestampFieldName", default, skip_serializing_if = "Option::is_none")]
    pub timestamp_field_name: Option<String>,
    #[doc = "The name of the entity."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}
impl EntityTypeDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod entity_type_definition {
    use super::*;
    #[doc = "Type of entity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EntityType {
        None,
        Profile,
        Interaction,
        Relationship,
    }
}
#[doc = "Input type for getting image upload url."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetImageUploadUrlInput {
    #[doc = "Type of entity. Can be Profile or Interaction."]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[doc = "Name of the entity type."]
    #[serde(rename = "entityTypeName", default, skip_serializing_if = "Option::is_none")]
    pub entity_type_name: Option<String>,
    #[doc = "Relative path of the image."]
    #[serde(rename = "relativePath", default, skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}
impl GetImageUploadUrlInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Hub resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Hub {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of hub."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HubPropertiesFormat>,
}
impl Hub {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Hub billing info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HubBillingInfoFormat {
    #[doc = "The sku name."]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
    #[doc = "The minimum number of units will be billed. One unit is 10,000 Profiles and 100,000 Interactions."]
    #[serde(rename = "minUnits", default, skip_serializing_if = "Option::is_none")]
    pub min_units: Option<i64>,
    #[doc = "The maximum number of units can be used.  One unit is 10,000 Profiles and 100,000 Interactions."]
    #[serde(rename = "maxUnits", default, skip_serializing_if = "Option::is_none")]
    pub max_units: Option<i64>,
}
impl HubBillingInfoFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of list hub operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HubListResult {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Hub>,
    #[doc = "Link for next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HubListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HubListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of hub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HubPropertiesFormat {
    #[doc = "API endpoint URL of the hub."]
    #[serde(rename = "apiEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    #[doc = "Web endpoint URL of the hub."]
    #[serde(rename = "webEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub web_endpoint: Option<String>,
    #[doc = "Provisioning state of the hub."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The bit flags for enabled hub features. Bit 0 is set to 1 indicates graph is enabled, or disabled if set to 0. Bit 1 is set to 1 indicates the hub is disabled, or enabled if set to 0."]
    #[serde(rename = "tenantFeatures", default, skip_serializing_if = "Option::is_none")]
    pub tenant_features: Option<i64>,
    #[doc = "Hub billing info."]
    #[serde(rename = "hubBillingInfo", default, skip_serializing_if = "Option::is_none")]
    pub hub_billing_info: Option<HubBillingInfoFormat>,
}
impl HubPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The image definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageDefinition {
    #[doc = "Whether image exists already."]
    #[serde(rename = "imageExists", default, skip_serializing_if = "Option::is_none")]
    pub image_exists: Option<bool>,
    #[doc = "Content URL for the image blob."]
    #[serde(rename = "contentUrl", default, skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    #[doc = "Relative path of the image."]
    #[serde(rename = "relativePath", default, skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}
impl ImageDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of list interaction operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InteractionListResult {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<InteractionResourceFormat>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InteractionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl InteractionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The interaction resource format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InteractionResourceFormat {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The Interaction Type Definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InteractionTypeDefinition>,
}
impl InteractionResourceFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Interaction Type Definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InteractionTypeDefinition {
    #[serde(flatten)]
    pub entity_type_definition: EntityTypeDefinition,
    #[doc = "The id property names. Properties which uniquely identify an interaction instance."]
    #[serde(rename = "idPropertyNames", default, skip_serializing_if = "Vec::is_empty")]
    pub id_property_names: Vec<String>,
    #[doc = "Profiles that participated in the interaction."]
    #[serde(rename = "participantProfiles", default, skip_serializing_if = "Vec::is_empty")]
    pub participant_profiles: Vec<Participant>,
    #[doc = "The primary participant property name for an interaction ,This is used to logically represent the agent of the interaction, Specify the participant name here from ParticipantName."]
    #[serde(rename = "primaryParticipantProfilePropertyName", default, skip_serializing_if = "Option::is_none")]
    pub primary_participant_profile_property_name: Option<String>,
    #[doc = "This is specific to interactions modeled as activities. Data sources are used to determine where data is stored and also in precedence rules."]
    #[serde(rename = "dataSourcePrecedenceRules", default, skip_serializing_if = "Vec::is_empty")]
    pub data_source_precedence_rules: Vec<DataSourcePrecedence>,
    #[doc = "Data Source is a way for us to know the source of instances. A single type can have data coming in from multiple places. In activities we use this to determine precedence rules."]
    #[serde(rename = "defaultDataSource", default, skip_serializing_if = "Option::is_none")]
    pub default_data_source: Option<DataSource>,
    #[doc = "An interaction can be tagged as an activity only during create. This enables the interaction to be editable and can enable merging of properties from multiple data sources based on precedence, which is defined at a link level."]
    #[serde(rename = "isActivity", default, skip_serializing_if = "Option::is_none")]
    pub is_activity: Option<bool>,
}
impl InteractionTypeDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The KPI alias."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KpiAlias {
    #[doc = "KPI alias name."]
    #[serde(rename = "aliasName")]
    pub alias_name: String,
    #[doc = "The expression."]
    pub expression: String,
}
impl KpiAlias {
    pub fn new(alias_name: String, expression: String) -> Self {
        Self { alias_name, expression }
    }
}
#[doc = "Defines the KPI Threshold limits."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KpiDefinition {
    #[doc = "The mapping entity type."]
    #[serde(rename = "entityType")]
    pub entity_type: kpi_definition::EntityType,
    #[doc = "The mapping entity name."]
    #[serde(rename = "entityTypeName")]
    pub entity_type_name: String,
    #[doc = "The hub name."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The KPI name."]
    #[serde(rename = "kpiName", default, skip_serializing_if = "Option::is_none")]
    pub kpi_name: Option<String>,
    #[doc = "Localized display name for the KPI."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<serde_json::Value>,
    #[doc = "Localized description for the KPI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,
    #[doc = "The calculation window."]
    #[serde(rename = "calculationWindow")]
    pub calculation_window: kpi_definition::CalculationWindow,
    #[doc = "Name of calculation window field."]
    #[serde(rename = "calculationWindowFieldName", default, skip_serializing_if = "Option::is_none")]
    pub calculation_window_field_name: Option<String>,
    #[doc = "The computation function for the KPI."]
    pub function: kpi_definition::Function,
    #[doc = "The computation expression for the KPI."]
    pub expression: String,
    #[doc = "The unit of measurement for the KPI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The filter expression for the KPI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[doc = "the group by properties for the KPI."]
    #[serde(rename = "groupBy", default, skip_serializing_if = "Vec::is_empty")]
    pub group_by: Vec<String>,
    #[doc = "The KPI GroupByMetadata."]
    #[serde(rename = "groupByMetadata", default, skip_serializing_if = "Vec::is_empty")]
    pub group_by_metadata: Vec<KpiGroupByMetadata>,
    #[doc = "The participant profiles."]
    #[serde(rename = "participantProfilesMetadata", default, skip_serializing_if = "Vec::is_empty")]
    pub participant_profiles_metadata: Vec<KpiParticipantProfilesMetadata>,
    #[doc = "Provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Defines the KPI Threshold limits."]
    #[serde(rename = "thresHolds", default, skip_serializing_if = "Option::is_none")]
    pub thres_holds: Option<KpiThresholds>,
    #[doc = "The aliases."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<KpiAlias>,
    #[doc = "The KPI extracts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extracts: Vec<KpiExtract>,
}
impl KpiDefinition {
    pub fn new(
        entity_type: kpi_definition::EntityType,
        entity_type_name: String,
        calculation_window: kpi_definition::CalculationWindow,
        function: kpi_definition::Function,
        expression: String,
    ) -> Self {
        Self {
            entity_type,
            entity_type_name,
            tenant_id: None,
            kpi_name: None,
            display_name: None,
            description: None,
            calculation_window,
            calculation_window_field_name: None,
            function,
            expression,
            unit: None,
            filter: None,
            group_by: Vec::new(),
            group_by_metadata: Vec::new(),
            participant_profiles_metadata: Vec::new(),
            provisioning_state: None,
            thres_holds: None,
            aliases: Vec::new(),
            extracts: Vec::new(),
        }
    }
}
pub mod kpi_definition {
    use super::*;
    #[doc = "The mapping entity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EntityType {
        None,
        Profile,
        Interaction,
        Relationship,
    }
    #[doc = "The calculation window."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CalculationWindow {
        Lifetime,
        Hour,
        Day,
        Week,
        Month,
    }
    #[doc = "The computation function for the KPI."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Function {
        Sum,
        Avg,
        Min,
        Max,
        Last,
        Count,
        None,
        CountDistinct,
    }
}
#[doc = "The KPI extract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KpiExtract {
    #[doc = "KPI extract name."]
    #[serde(rename = "extractName")]
    pub extract_name: String,
    #[doc = "The expression."]
    pub expression: String,
}
impl KpiExtract {
    pub fn new(extract_name: String, expression: String) -> Self {
        Self { extract_name, expression }
    }
}
#[doc = "The KPI GroupBy field metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KpiGroupByMetadata {
    #[doc = "The display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<serde_json::Value>,
    #[doc = "The name of the field."]
    #[serde(rename = "fieldName", default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[doc = "The type of the field."]
    #[serde(rename = "fieldType", default, skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
}
impl KpiGroupByMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of list KPI operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KpiListResult {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<KpiResourceFormat>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for KpiListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl KpiListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The KPI participant profile metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KpiParticipantProfilesMetadata {
    #[doc = "Name of the type."]
    #[serde(rename = "typeName")]
    pub type_name: String,
}
impl KpiParticipantProfilesMetadata {
    pub fn new(type_name: String) -> Self {
        Self { type_name }
    }
}
#[doc = "The KPI resource format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KpiResourceFormat {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Defines the KPI Threshold limits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<KpiDefinition>,
}
impl KpiResourceFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the KPI Threshold limits."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KpiThresholds {
    #[doc = "The lower threshold limit."]
    #[serde(rename = "lowerLimit")]
    pub lower_limit: f64,
    #[doc = "The upper threshold limit."]
    #[serde(rename = "upperLimit")]
    pub upper_limit: f64,
    #[doc = "Whether or not the KPI is an increasing KPI."]
    #[serde(rename = "increasingKpi")]
    pub increasing_kpi: bool,
}
impl KpiThresholds {
    pub fn new(lower_limit: f64, upper_limit: f64, increasing_kpi: bool) -> Self {
        Self {
            lower_limit,
            upper_limit,
            increasing_kpi,
        }
    }
}
#[doc = "The definition of Link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkDefinition {
    #[doc = "The hub name."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The link name."]
    #[serde(rename = "linkName", default, skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
    #[doc = "Name of the source Interaction Type."]
    #[serde(rename = "sourceInteractionType")]
    pub source_interaction_type: String,
    #[doc = "Name of the target Profile Type."]
    #[serde(rename = "targetProfileType")]
    pub target_profile_type: String,
    #[doc = "Localized display name for the Link."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<serde_json::Value>,
    #[doc = "Localized descriptions for the Link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,
    #[doc = "The set of properties mappings between the source and target Types."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mappings: Vec<TypePropertiesMapping>,
    #[doc = "The properties that represent the participating profile."]
    #[serde(rename = "participantPropertyReferences")]
    pub participant_property_references: Vec<ParticipantPropertyReference>,
    #[doc = "Provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Indicating whether the link is reference only link. This flag is ignored if the Mappings are defined. If the mappings are not defined and it is set to true, links processing will not create or update profiles."]
    #[serde(rename = "referenceOnly", default, skip_serializing_if = "Option::is_none")]
    pub reference_only: Option<bool>,
    #[doc = "Determines whether this link is supposed to create or delete instances if Link is NOT Reference Only."]
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<link_definition::OperationType>,
}
impl LinkDefinition {
    pub fn new(
        source_interaction_type: String,
        target_profile_type: String,
        participant_property_references: Vec<ParticipantPropertyReference>,
    ) -> Self {
        Self {
            tenant_id: None,
            link_name: None,
            source_interaction_type,
            target_profile_type,
            display_name: None,
            description: None,
            mappings: Vec::new(),
            participant_property_references,
            provisioning_state: None,
            reference_only: None,
            operation_type: None,
        }
    }
}
pub mod link_definition {
    use super::*;
    #[doc = "Determines whether this link is supposed to create or delete instances if Link is NOT Reference Only."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OperationType {
        Upsert,
        Delete,
    }
}
#[doc = "The response of list link operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkListResult {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LinkResourceFormat>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LinkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl LinkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The link resource format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkResourceFormat {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The definition of Link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LinkDefinition>,
}
impl LinkResourceFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Metadata definition base."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataDefinitionBase {
    #[doc = "The attributes for the Type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[doc = "Localized descriptions for the property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,
    #[doc = "Localized display names for the property."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<serde_json::Value>,
    #[doc = "Any custom localized attributes for the Type."]
    #[serde(rename = "localizedAttributes", default, skip_serializing_if = "Option::is_none")]
    pub localized_attributes: Option<serde_json::Value>,
    #[doc = "Small Image associated with the Property or EntityType."]
    #[serde(rename = "smallImage", default, skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,
    #[doc = "Medium Image associated with the Property or EntityType."]
    #[serde(rename = "mediumImage", default, skip_serializing_if = "Option::is_none")]
    pub medium_image: Option<String>,
    #[doc = "Large Image associated with the Property or EntityType."]
    #[serde(rename = "largeImage", default, skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,
}
impl MetadataDefinitionBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Customer Insights REST API operation"]
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
        #[doc = "Service provider: Microsoft.CustomerInsights"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Invoice, etc."]
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
#[doc = "Result of the request to list Customer Insights operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Customer Insights operations supported by the Microsoft.CustomerInsights resource provider."]
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
#[doc = "Describes a profile type participating in an interaction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Participant {
    #[doc = "Profile type name."]
    #[serde(rename = "profileTypeName")]
    pub profile_type_name: String,
    #[doc = "The property references."]
    #[serde(rename = "participantPropertyReferences")]
    pub participant_property_references: Vec<ParticipantPropertyReference>,
    #[doc = "Participant name."]
    #[serde(rename = "participantName")]
    pub participant_name: String,
    #[doc = "Localized display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<serde_json::Value>,
    #[doc = "Localized descriptions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,
    #[doc = "The role that the participant is playing in the interaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl Participant {
    pub fn new(
        profile_type_name: String,
        participant_property_references: Vec<ParticipantPropertyReference>,
        participant_name: String,
    ) -> Self {
        Self {
            profile_type_name,
            participant_property_references,
            participant_name,
            display_name: None,
            description: None,
            role: None,
        }
    }
}
#[doc = "The participant property reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParticipantPropertyReference {
    #[doc = "The interaction property that maps to the profile property."]
    #[serde(rename = "interactionPropertyName")]
    pub interaction_property_name: String,
    #[doc = "The profile property that maps to the interaction property."]
    #[serde(rename = "profilePropertyName")]
    pub profile_property_name: String,
}
impl ParticipantPropertyReference {
    pub fn new(interaction_property_name: String, profile_property_name: String) -> Self {
        Self {
            interaction_property_name,
            profile_property_name,
        }
    }
}
#[doc = "Supported permission types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PermissionTypes {
    Read,
    Write,
    Manage,
}
#[doc = "Valid enum values in case of an enum property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileEnumValidValuesFormat {
    #[doc = "The integer value of the enum member."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    #[doc = "Localized names of the enum member."]
    #[serde(rename = "localizedValueNames", default, skip_serializing_if = "Option::is_none")]
    pub localized_value_names: Option<serde_json::Value>,
}
impl ProfileEnumValidValuesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of list profile operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileListResult {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProfileResourceFormat>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProfileListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProfileListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The profile resource format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileResourceFormat {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The profile type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProfileTypeDefinition>,
}
impl ProfileResourceFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The profile type definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileTypeDefinition {
    #[serde(flatten)]
    pub entity_type_definition: EntityTypeDefinition,
    #[doc = "The strong IDs."]
    #[serde(rename = "strongIds", default, skip_serializing_if = "Vec::is_empty")]
    pub strong_ids: Vec<StrongId>,
}
impl ProfileTypeDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Property definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyDefinition {
    #[doc = "Array value separator for properties with isArray set."]
    #[serde(rename = "arrayValueSeparator", default, skip_serializing_if = "Option::is_none")]
    pub array_value_separator: Option<String>,
    #[doc = "Describes valid values for an enum property."]
    #[serde(rename = "enumValidValues", default, skip_serializing_if = "Vec::is_empty")]
    pub enum_valid_values: Vec<ProfileEnumValidValuesFormat>,
    #[doc = "Name of the property."]
    #[serde(rename = "fieldName")]
    pub field_name: String,
    #[doc = "Type of the property."]
    #[serde(rename = "fieldType")]
    pub field_type: String,
    #[doc = "Indicates if the property is actually an array of the fieldType above on the data api."]
    #[serde(rename = "isArray", default, skip_serializing_if = "Option::is_none")]
    pub is_array: Option<bool>,
    #[doc = "Indicates if the property is an enum."]
    #[serde(rename = "isEnum", default, skip_serializing_if = "Option::is_none")]
    pub is_enum: Option<bool>,
    #[doc = "Indicates if the property is an flag enum."]
    #[serde(rename = "isFlagEnum", default, skip_serializing_if = "Option::is_none")]
    pub is_flag_enum: Option<bool>,
    #[doc = "Whether the property is an Image."]
    #[serde(rename = "isImage", default, skip_serializing_if = "Option::is_none")]
    pub is_image: Option<bool>,
    #[doc = "Whether the property is a localized string."]
    #[serde(rename = "isLocalizedString", default, skip_serializing_if = "Option::is_none")]
    pub is_localized_string: Option<bool>,
    #[doc = "Whether the property is a name or a part of name."]
    #[serde(rename = "isName", default, skip_serializing_if = "Option::is_none")]
    pub is_name: Option<bool>,
    #[doc = "Whether property value is required on instances, IsRequired field only for Interaction. Profile Instance will not check for required field."]
    #[serde(rename = "isRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[doc = "The ID associated with the property."]
    #[serde(rename = "propertyId", default, skip_serializing_if = "Option::is_none")]
    pub property_id: Option<String>,
    #[doc = "URL encoded schema.org item prop link for the property."]
    #[serde(rename = "schemaItemPropLink", default, skip_serializing_if = "Option::is_none")]
    pub schema_item_prop_link: Option<String>,
    #[doc = "Max length of string. Used only if type is string."]
    #[serde(rename = "maxLength", default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    #[doc = "Whether property is available in graph or not."]
    #[serde(rename = "isAvailableInGraph", default, skip_serializing_if = "Option::is_none")]
    pub is_available_in_graph: Option<bool>,
    #[doc = "This is specific to interactions modeled as activities. Data sources are used to determine where data is stored and also in precedence rules."]
    #[serde(rename = "dataSourcePrecedenceRules", default, skip_serializing_if = "Vec::is_empty")]
    pub data_source_precedence_rules: Vec<DataSourcePrecedence>,
}
impl PropertyDefinition {
    pub fn new(field_name: String, field_type: String) -> Self {
        Self {
            array_value_separator: None,
            enum_valid_values: Vec::new(),
            field_name,
            field_type,
            is_array: None,
            is_enum: None,
            is_flag_enum: None,
            is_image: None,
            is_localized_string: None,
            is_name: None,
            is_required: None,
            property_id: None,
            schema_item_prop_link: None,
            max_length: None,
            is_available_in_graph: None,
            data_source_precedence_rules: Vec::new(),
        }
    }
}
#[doc = "Provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Provisioning,
    Succeeded,
    Expiring,
    Deleting,
    HumanIntervention,
    Failed,
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
            Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Provisioning"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
            Self::Expiring => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Expiring"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
            Self::HumanIntervention => serializer.serialize_unit_variant("ProvisioningState", 4u32, "HumanIntervention"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Common properties of proxy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of Relationship."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipDefinition {
    #[doc = "The Relationship Cardinality."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<relationship_definition::Cardinality>,
    #[doc = "Localized display name for the Relationship."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<serde_json::Value>,
    #[doc = "Localized descriptions for the Relationship."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,
    #[doc = "The expiry date time in UTC."]
    #[serde(rename = "expiryDateTimeUtc", default, with = "azure_core::date::rfc3339::option")]
    pub expiry_date_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The properties of the Relationship."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<PropertyDefinition>,
    #[doc = "Optional property to be used to map fields in profile to their strong ids in related profile."]
    #[serde(rename = "lookupMappings", default, skip_serializing_if = "Vec::is_empty")]
    pub lookup_mappings: Vec<RelationshipTypeMapping>,
    #[doc = "Profile type."]
    #[serde(rename = "profileType")]
    pub profile_type: String,
    #[doc = "Provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The Relationship name."]
    #[serde(rename = "relationshipName", default, skip_serializing_if = "Option::is_none")]
    pub relationship_name: Option<String>,
    #[doc = "Related profile being referenced."]
    #[serde(rename = "relatedProfileType")]
    pub related_profile_type: String,
    #[doc = "The relationship guid id."]
    #[serde(rename = "relationshipGuidId", default, skip_serializing_if = "Option::is_none")]
    pub relationship_guid_id: Option<String>,
    #[doc = "The hub name."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl RelationshipDefinition {
    pub fn new(profile_type: String, related_profile_type: String) -> Self {
        Self {
            cardinality: None,
            display_name: None,
            description: None,
            expiry_date_time_utc: None,
            fields: Vec::new(),
            lookup_mappings: Vec::new(),
            profile_type,
            provisioning_state: None,
            relationship_name: None,
            related_profile_type,
            relationship_guid_id: None,
            tenant_id: None,
        }
    }
}
pub mod relationship_definition {
    use super::*;
    #[doc = "The Relationship Cardinality."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Cardinality {
        OneToOne,
        OneToMany,
        ManyToMany,
    }
}
#[doc = "The definition of relationship link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipLinkDefinition {
    #[doc = "Localized display name for the Relationship Link."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<serde_json::Value>,
    #[doc = "Localized descriptions for the Relationship Link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,
    #[doc = "The InteractionType associated with the Relationship Link."]
    #[serde(rename = "interactionType")]
    pub interaction_type: String,
    #[doc = "The name of the Relationship Link."]
    #[serde(rename = "linkName", default, skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
    #[doc = "The mappings between Interaction and Relationship fields."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mappings: Vec<RelationshipLinkFieldMapping>,
    #[doc = "The property references for the Profile of the Relationship."]
    #[serde(rename = "profilePropertyReferences")]
    pub profile_property_references: Vec<ParticipantPropertyReference>,
    #[doc = "Provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The property references for the Related Profile of the Relationship."]
    #[serde(rename = "relatedProfilePropertyReferences")]
    pub related_profile_property_references: Vec<ParticipantPropertyReference>,
    #[doc = "The Relationship associated with the Link."]
    #[serde(rename = "relationshipName")]
    pub relationship_name: String,
    #[doc = "The relationship guid id."]
    #[serde(rename = "relationshipGuidId", default, skip_serializing_if = "Option::is_none")]
    pub relationship_guid_id: Option<String>,
    #[doc = "The hub name."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl RelationshipLinkDefinition {
    pub fn new(
        interaction_type: String,
        profile_property_references: Vec<ParticipantPropertyReference>,
        related_profile_property_references: Vec<ParticipantPropertyReference>,
        relationship_name: String,
    ) -> Self {
        Self {
            display_name: None,
            description: None,
            interaction_type,
            link_name: None,
            mappings: Vec::new(),
            profile_property_references,
            provisioning_state: None,
            related_profile_property_references,
            relationship_name,
            relationship_guid_id: None,
            tenant_id: None,
        }
    }
}
#[doc = "The fields mapping for Relationships."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipLinkFieldMapping {
    #[doc = "The field name on the Interaction Type."]
    #[serde(rename = "interactionFieldName")]
    pub interaction_field_name: String,
    #[doc = "Link type."]
    #[serde(rename = "linkType", default, skip_serializing_if = "Option::is_none")]
    pub link_type: Option<relationship_link_field_mapping::LinkType>,
    #[doc = "The field name on the Relationship metadata."]
    #[serde(rename = "relationshipFieldName")]
    pub relationship_field_name: String,
}
impl RelationshipLinkFieldMapping {
    pub fn new(interaction_field_name: String, relationship_field_name: String) -> Self {
        Self {
            interaction_field_name,
            link_type: None,
            relationship_field_name,
        }
    }
}
pub mod relationship_link_field_mapping {
    use super::*;
    #[doc = "Link type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LinkType {
        UpdateAlways,
        CopyIfNull,
    }
}
#[doc = "The response of list relationship link operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelationshipLinkListResult {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RelationshipLinkResourceFormat>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RelationshipLinkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RelationshipLinkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The relationship link resource format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelationshipLinkResourceFormat {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The definition of relationship link."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RelationshipLinkDefinition>,
}
impl RelationshipLinkResourceFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of list relationship operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelationshipListResult {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RelationshipResourceFormat>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RelationshipListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RelationshipListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The relationship resource format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelationshipResourceFormat {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The definition of Relationship."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RelationshipDefinition>,
}
impl RelationshipResourceFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Map a field of profile to its corresponding StrongId in Related Profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipTypeFieldMapping {
    #[doc = "Specifies the fieldName in profile."]
    #[serde(rename = "profileFieldName")]
    pub profile_field_name: String,
    #[doc = "Specifies the KeyProperty (from StrongId) of the related profile."]
    #[serde(rename = "relatedProfileKeyProperty")]
    pub related_profile_key_property: String,
}
impl RelationshipTypeFieldMapping {
    pub fn new(profile_field_name: String, related_profile_key_property: String) -> Self {
        Self {
            profile_field_name,
            related_profile_key_property,
        }
    }
}
#[doc = "Maps fields in Profile to their corresponding StrongIds in Related Profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipTypeMapping {
    #[doc = "Maps a profile property with the StrongId of related profile. This is an array to support StrongIds that are composite key as well."]
    #[serde(rename = "fieldMappings")]
    pub field_mappings: Vec<RelationshipTypeFieldMapping>,
}
impl RelationshipTypeMapping {
    pub fn new(field_mappings: Vec<RelationshipTypeFieldMapping>) -> Self {
        Self { field_mappings }
    }
}
#[doc = "The definition of suggested relationship for the type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelationshipsLookup {
    #[doc = "The relationship profile."]
    #[serde(rename = "profileName", default, skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[doc = "The property references for the profile type."]
    #[serde(rename = "profilePropertyReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub profile_property_references: Vec<ParticipantPropertyReference>,
    #[doc = "The related profile."]
    #[serde(rename = "relatedProfileName", default, skip_serializing_if = "Option::is_none")]
    pub related_profile_name: Option<String>,
    #[doc = "The property references for the related profile type."]
    #[serde(rename = "relatedProfilePropertyReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub related_profile_property_references: Vec<ParticipantPropertyReference>,
    #[doc = "The name of existing Relationship."]
    #[serde(rename = "existingRelationshipName", default, skip_serializing_if = "Option::is_none")]
    pub existing_relationship_name: Option<String>,
}
impl RelationshipsLookup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common properties of Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource set description."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSetDescription {
    #[doc = "The elements included in the set."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub elements: Vec<String>,
    #[doc = "The elements that are not included in the set, in case elements contains '*' indicating 'all'."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exceptions: Vec<String>,
}
impl ResourceSetDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Role definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Role {
    #[doc = "The role name."]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "The description of the role."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl Role {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Role Assignment definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignment {
    #[doc = "The hub name."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The name of the metadata object."]
    #[serde(rename = "assignmentName", default, skip_serializing_if = "Option::is_none")]
    pub assignment_name: Option<String>,
    #[doc = "Localized display names for the metadata."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<serde_json::Value>,
    #[doc = "Localized description for the metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,
    #[doc = "Provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Type of roles."]
    pub role: role_assignment::Role,
    #[doc = "The principals being assigned to."]
    pub principals: Vec<AssignmentPrincipal>,
    #[doc = "The resource set description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profiles: Option<ResourceSetDescription>,
    #[doc = "The resource set description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interactions: Option<ResourceSetDescription>,
    #[doc = "The resource set description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<ResourceSetDescription>,
    #[doc = "The resource set description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kpis: Option<ResourceSetDescription>,
    #[doc = "The resource set description."]
    #[serde(rename = "sasPolicies", default, skip_serializing_if = "Option::is_none")]
    pub sas_policies: Option<ResourceSetDescription>,
    #[doc = "The resource set description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connectors: Option<ResourceSetDescription>,
    #[doc = "The resource set description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub views: Option<ResourceSetDescription>,
    #[doc = "The resource set description."]
    #[serde(rename = "relationshipLinks", default, skip_serializing_if = "Option::is_none")]
    pub relationship_links: Option<ResourceSetDescription>,
    #[doc = "The resource set description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationships: Option<ResourceSetDescription>,
    #[doc = "The resource set description."]
    #[serde(rename = "widgetTypes", default, skip_serializing_if = "Option::is_none")]
    pub widget_types: Option<ResourceSetDescription>,
    #[doc = "The resource set description."]
    #[serde(rename = "roleAssignments", default, skip_serializing_if = "Option::is_none")]
    pub role_assignments: Option<ResourceSetDescription>,
    #[doc = "The resource set description."]
    #[serde(rename = "conflationPolicies", default, skip_serializing_if = "Option::is_none")]
    pub conflation_policies: Option<ResourceSetDescription>,
    #[doc = "The resource set description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments: Option<ResourceSetDescription>,
}
impl RoleAssignment {
    pub fn new(role: role_assignment::Role, principals: Vec<AssignmentPrincipal>) -> Self {
        Self {
            tenant_id: None,
            assignment_name: None,
            display_name: None,
            description: None,
            provisioning_state: None,
            role,
            principals,
            profiles: None,
            interactions: None,
            links: None,
            kpis: None,
            sas_policies: None,
            connectors: None,
            views: None,
            relationship_links: None,
            relationships: None,
            widget_types: None,
            role_assignments: None,
            conflation_policies: None,
            segments: None,
        }
    }
}
pub mod role_assignment {
    use super::*;
    #[doc = "Type of roles."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Role {
        Admin,
        Reader,
        ManageAdmin,
        ManageReader,
        DataAdmin,
        DataReader,
    }
}
#[doc = "The response of list role assignment operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentListResult {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleAssignmentResourceFormat>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleAssignmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Role Assignment resource format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentResourceFormat {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The Role Assignment definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleAssignment>,
}
impl RoleAssignmentResourceFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of list role assignment operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleListResult {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleResourceFormat>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The role resource format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleResourceFormat {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The Role definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Role>,
}
impl RoleResourceFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Salesforce connector properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SalesforceConnectorProperties {
    #[doc = "Salesforce discover setting."]
    pub usersetting: SalesforceDiscoverSetting,
    #[doc = "The Salesforce tables."]
    pub salesforcetables: Vec<SalesforceTable>,
}
impl SalesforceConnectorProperties {
    pub fn new(usersetting: SalesforceDiscoverSetting, salesforcetables: Vec<SalesforceTable>) -> Self {
        Self {
            usersetting,
            salesforcetables,
        }
    }
}
#[doc = "Salesforce discover setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SalesforceDiscoverSetting {
    #[doc = "The salesforce connection string secret URL."]
    #[serde(rename = "salesforceConnectionStringSecretUrl")]
    pub salesforce_connection_string_secret_url: String,
}
impl SalesforceDiscoverSetting {
    pub fn new(salesforce_connection_string_secret_url: String) -> Self {
        Self {
            salesforce_connection_string_secret_url,
        }
    }
}
#[doc = "Salesforce table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SalesforceTable {
    #[doc = "Indicating whether this instance is profile."]
    #[serde(rename = "isProfile", default, skip_serializing_if = "Option::is_none")]
    pub is_profile: Option<String>,
    #[doc = "The table category."]
    #[serde(rename = "tableCategory")]
    pub table_category: String,
    #[doc = "The name of the table."]
    #[serde(rename = "tableName")]
    pub table_name: String,
    #[doc = "The table remarks."]
    #[serde(rename = "tableRemarks", default, skip_serializing_if = "Option::is_none")]
    pub table_remarks: Option<String>,
    #[doc = "The table schema."]
    #[serde(rename = "tableSchema")]
    pub table_schema: String,
}
impl SalesforceTable {
    pub fn new(table_category: String, table_name: String, table_schema: String) -> Self {
        Self {
            is_profile: None,
            table_category,
            table_name,
            table_remarks: None,
            table_schema,
        }
    }
}
#[doc = "Property/Properties which represent a unique ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StrongId {
    #[doc = "The properties which make up the unique ID."]
    #[serde(rename = "keyPropertyNames")]
    pub key_property_names: Vec<String>,
    #[doc = "The Name identifying the strong ID."]
    #[serde(rename = "strongIdName")]
    pub strong_id_name: String,
    #[doc = "Localized display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<serde_json::Value>,
    #[doc = "Localized descriptions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,
}
impl StrongId {
    pub fn new(key_property_names: Vec<String>, strong_id_name: String) -> Self {
        Self {
            key_property_names,
            strong_id_name,
            display_name: None,
            description: None,
        }
    }
}
#[doc = "The response of suggest relationship links operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SuggestRelationshipLinksResponse {
    #[doc = "The interaction name."]
    #[serde(rename = "interactionName", default, skip_serializing_if = "Option::is_none")]
    pub interaction_name: Option<String>,
    #[doc = "Suggested relationships for the type."]
    #[serde(rename = "suggestedRelationships", default, skip_serializing_if = "Vec::is_empty")]
    pub suggested_relationships: Vec<RelationshipsLookup>,
}
impl SuggestRelationshipLinksResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata for a Link's property mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypePropertiesMapping {
    #[doc = " Property name on the source Interaction Type."]
    #[serde(rename = "interactionTypePropertyName")]
    pub interaction_type_property_name: String,
    #[doc = "Property name on the target Profile Type."]
    #[serde(rename = "profileTypePropertyName")]
    pub profile_type_property_name: String,
    #[doc = "Flag to indicate whether the Profile Type property is an id on the Profile Type."]
    #[serde(rename = "isProfileTypeId", default, skip_serializing_if = "Option::is_none")]
    pub is_profile_type_id: Option<bool>,
    #[doc = "Link type."]
    #[serde(rename = "linkType", default, skip_serializing_if = "Option::is_none")]
    pub link_type: Option<type_properties_mapping::LinkType>,
}
impl TypePropertiesMapping {
    pub fn new(interaction_type_property_name: String, profile_type_property_name: String) -> Self {
        Self {
            interaction_type_property_name,
            profile_type_property_name,
            is_profile_type_id: None,
            link_type: None,
        }
    }
}
pub mod type_properties_mapping {
    use super::*;
    #[doc = "Link type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LinkType {
        UpdateAlways,
        CopyIfNull,
    }
}
#[doc = "The view in Customer 360 web application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct View {
    #[doc = "Name of the view."]
    #[serde(rename = "viewName", default, skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
    #[doc = "the user ID."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "the hub name."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Localized display name for the view."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<serde_json::Value>,
    #[doc = "View definition."]
    pub definition: String,
    #[doc = "Date time when view was last modified."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub changed: Option<time::OffsetDateTime>,
    #[doc = "Date time when view was created."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
}
impl View {
    pub fn new(definition: String) -> Self {
        Self {
            view_name: None,
            user_id: None,
            tenant_id: None,
            display_name: None,
            definition,
            changed: None,
            created: None,
        }
    }
}
#[doc = "The response of list view operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ViewListResult {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ViewResourceFormat>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ViewListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ViewListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The view resource format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ViewResourceFormat {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The view in Customer 360 web application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<View>,
}
impl ViewResourceFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of WidgetType."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetType {
    #[doc = "Name of the widget type."]
    #[serde(rename = "widgetTypeName", default, skip_serializing_if = "Option::is_none")]
    pub widget_type_name: Option<String>,
    #[doc = "Definition for widget type."]
    pub definition: String,
    #[doc = "Description for widget type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Localized display name for the widget type."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<serde_json::Value>,
    #[doc = "The image URL."]
    #[serde(rename = "imageUrl", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[doc = "The hub name."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The widget version."]
    #[serde(rename = "widgetVersion", default, skip_serializing_if = "Option::is_none")]
    pub widget_version: Option<String>,
    #[doc = "Date time when widget type was last modified."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub changed: Option<time::OffsetDateTime>,
    #[doc = "Date time when widget type was created."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub created: Option<time::OffsetDateTime>,
}
impl WidgetType {
    pub fn new(definition: String) -> Self {
        Self {
            widget_type_name: None,
            definition,
            description: None,
            display_name: None,
            image_url: None,
            tenant_id: None,
            widget_version: None,
            changed: None,
            created: None,
        }
    }
}
#[doc = "The response of list widget type operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WidgetTypeListResult {
    #[doc = "Results of the list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WidgetTypeResourceFormat>,
    #[doc = "Link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WidgetTypeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WidgetTypeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The WidgetTypeResourceFormat"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WidgetTypeResourceFormat {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Definition of WidgetType."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WidgetType>,
}
impl WidgetTypeResourceFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
