#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Class represents provider operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableProviderOperation {
    #[doc = "Gets or Sets Name of the operations"]
    pub name: String,
    #[doc = "Contains the localized display information for this particular operation / action. \r\nThese value will be used by several clients for \r\n(1) custom role definitions for RBAC; \r\n(2) complex query filters for the event service; and (3) audit history / records for management operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<AvailableProviderOperationDisplay>,
    #[doc = "Gets or sets Origin\r\nThe intended executor of the operation; governs the display of the operation in the RBAC UX and the audit logs UX.\r\nDefault value is “user,system”"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Class represents Properties in AvailableProviderOperations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailableProviderOperationProperties>,
}
impl AvailableProviderOperation {
    pub fn new(name: String) -> Self {
        Self {
            name,
            display: None,
            origin: None,
            properties: None,
        }
    }
}
#[doc = "Contains the localized display information for this particular operation / action. \r\nThese value will be used by several clients for \r\n(1) custom role definitions for RBAC; \r\n(2) complex query filters for the event service; and (3) audit history / records for management operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableProviderOperationDisplay {
    #[doc = "Gets or sets Provider\r\nThe localized friendly form of the resource provider name – it is expected to also include the publisher/company responsible. \r\nIt should use Title Casing and begin with “Microsoft” for 1st party services."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Gets or sets Resource\r\nThe localized friendly form of the resource type related to this action/operation – it should match the public documentation for the resource provider. \r\nIt should use Title Casing – for examples, please refer to the “name” section."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Gets or sets Operation\r\nThe localized friendly name for the operation, as it should be shown to the user. \r\nIt should be concise (to fit in drop downs) but clear (i.e. self-documenting). It should use Title Casing and include the entity/resource to which it applies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Gets or sets Description\r\nThe localized friendly description for the operation, as it should be shown to the user. \r\nIt should be thorough, yet concise – it will be used in tool tips and detailed views."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AvailableProviderOperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class represents Properties in AvailableProviderOperations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableProviderOperationProperties {}
impl AvailableProviderOperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class for set of operations used for discovery of available provider operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableProviderOperations {
    #[doc = "List of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AvailableProviderOperation>,
    #[doc = "Link for the next set of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvailableProviderOperations {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AvailableProviderOperations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The pair of customer secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerSecret {
    #[doc = "The identifier to the data service input object which this secret corresponds to."]
    #[serde(rename = "keyIdentifier")]
    pub key_identifier: String,
    #[doc = "It contains the encrypted customer secret."]
    #[serde(rename = "keyValue")]
    pub key_value: String,
    #[doc = "The encryption algorithm used to encrypt data."]
    pub algorithm: customer_secret::Algorithm,
}
impl CustomerSecret {
    pub fn new(key_identifier: String, key_value: String, algorithm: customer_secret::Algorithm) -> Self {
        Self {
            key_identifier,
            key_value,
            algorithm,
        }
    }
}
pub mod customer_secret {
    use super::*;
    #[doc = "The encryption algorithm used to encrypt data."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Algorithm {
        None,
        #[serde(rename = "RSA1_5")]
        Rsa15,
        #[serde(rename = "RSA_OAEP")]
        RsaOaep,
        PlainText,
    }
}
#[doc = "The DataManager resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataManager {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Etag of the Resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl DataManager {
    pub fn new(resource: Resource) -> Self {
        Self { resource, etag: None }
    }
}
#[doc = "DataManager resources Collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataManagerList {
    #[doc = "List of data manager resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataManager>,
    #[doc = "Link for the next set of data stores."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataManagerList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DataManagerList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The DataManagerUpdateParameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataManagerUpdateParameter {
    #[doc = "The sku type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource\r\n(across resource groups)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DataManagerUpdateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataService {
    #[serde(flatten)]
    pub dms_base_object: DmsBaseObject,
    #[doc = "Data Service properties."]
    pub properties: DataServiceProperties,
}
impl DataService {
    pub fn new(properties: DataServiceProperties) -> Self {
        Self {
            dms_base_object: DmsBaseObject::default(),
            properties,
        }
    }
}
#[doc = "Data Service Collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataServiceList {
    #[doc = "List of data services."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataService>,
    #[doc = "Link for the next set of data services."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataServiceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataServiceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Service properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataServiceProperties {
    #[doc = "State of the data service."]
    pub state: data_service_properties::State,
    #[doc = "Supported data store types which can be used as a sink."]
    #[serde(rename = "supportedDataSinkTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_data_sink_types: Vec<String>,
    #[doc = "Supported data store types which can be used as a source."]
    #[serde(rename = "supportedDataSourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_data_source_types: Vec<String>,
}
impl DataServiceProperties {
    pub fn new(state: data_service_properties::State) -> Self {
        Self {
            state,
            supported_data_sink_types: Vec::new(),
            supported_data_source_types: Vec::new(),
        }
    }
}
pub mod data_service_properties {
    use super::*;
    #[doc = "State of the data service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Disabled,
        Enabled,
        Supported,
    }
}
#[doc = "Data store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataStore {
    #[serde(flatten)]
    pub dms_base_object: DmsBaseObject,
    #[doc = "Data Store for sources and sinks"]
    pub properties: DataStoreProperties,
}
impl DataStore {
    pub fn new(properties: DataStoreProperties) -> Self {
        Self {
            dms_base_object: DmsBaseObject::default(),
            properties,
        }
    }
}
#[doc = "Contains the information about the filters for the DataStore."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataStoreFilter {
    #[doc = "The data store type id."]
    #[serde(rename = "dataStoreTypeId", default, skip_serializing_if = "Option::is_none")]
    pub data_store_type_id: Option<String>,
}
impl DataStoreFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Store Collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataStoreList {
    #[doc = "List of data stores."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataStore>,
    #[doc = "Link for the next set of data stores."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataStoreList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataStoreList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Store for sources and sinks"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataStoreProperties {
    #[doc = "Arm Id for the manager resource to which the data source is associated. This is optional."]
    #[serde(rename = "repositoryId", default, skip_serializing_if = "Option::is_none")]
    pub repository_id: Option<String>,
    #[doc = "State of the data source."]
    pub state: data_store_properties::State,
    #[doc = "A generic json used differently by each data source type."]
    #[serde(rename = "extendedProperties", default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<serde_json::Value>,
    #[doc = "The arm id of the data store type."]
    #[serde(rename = "dataStoreTypeId")]
    pub data_store_type_id: String,
    #[doc = "List of customer secrets containing a key identifier and key value. The key identifier is a way for the specific data source to understand the key. Value contains customer secret encrypted by the encryptionKeys."]
    #[serde(rename = "customerSecrets", default, skip_serializing_if = "Vec::is_empty")]
    pub customer_secrets: Vec<CustomerSecret>,
}
impl DataStoreProperties {
    pub fn new(state: data_store_properties::State, data_store_type_id: String) -> Self {
        Self {
            repository_id: None,
            state,
            extended_properties: None,
            data_store_type_id,
            customer_secrets: Vec::new(),
        }
    }
}
pub mod data_store_properties {
    use super::*;
    #[doc = "State of the data source."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Disabled,
        Enabled,
        Supported,
    }
}
#[doc = "Data Store Type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataStoreType {
    #[serde(flatten)]
    pub dms_base_object: DmsBaseObject,
    #[doc = "Data Store Type properties."]
    pub properties: DataStoreTypeProperties,
}
impl DataStoreType {
    pub fn new(properties: DataStoreTypeProperties) -> Self {
        Self {
            dms_base_object: DmsBaseObject::default(),
            properties,
        }
    }
}
#[doc = "Data Store Type Collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataStoreTypeList {
    #[doc = "List of DataStoreType."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataStoreType>,
    #[doc = "Link for the next set of data store types."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataStoreTypeList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DataStoreTypeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Store Type properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataStoreTypeProperties {
    #[doc = "Arm type for the manager resource to which the data source type is associated. This is optional."]
    #[serde(rename = "repositoryType", default, skip_serializing_if = "Option::is_none")]
    pub repository_type: Option<String>,
    #[doc = "State of the data store type."]
    pub state: data_store_type_properties::State,
    #[doc = "Supported data services where it can be used as a sink."]
    #[serde(rename = "supportedDataServicesAsSink", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_data_services_as_sink: Vec<String>,
    #[doc = "Supported data services where it can be used as a source."]
    #[serde(rename = "supportedDataServicesAsSource", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_data_services_as_source: Vec<String>,
}
impl DataStoreTypeProperties {
    pub fn new(state: data_store_type_properties::State) -> Self {
        Self {
            repository_type: None,
            state,
            supported_data_services_as_sink: Vec::new(),
            supported_data_services_as_source: Vec::new(),
        }
    }
}
pub mod data_store_type_properties {
    use super::*;
    #[doc = "State of the data store type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Disabled,
        Enabled,
        Supported,
    }
}
#[doc = "Base class for all objects under DataManager Service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DmsBaseObject {
    #[doc = "Name of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Id of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Type of the object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl DmsBaseObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Top level error for the job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[doc = "Error code that can be used to programmatically identify the error."]
    pub code: String,
    #[doc = "Describes the error in detail and provides debugging information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl Error {
    pub fn new(code: String) -> Self {
        Self { code, message: None }
    }
}
#[doc = "Error Details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "Recommended action for the error."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "Contains the non localized exception message"]
    #[serde(rename = "exceptionMessage", default, skip_serializing_if = "Option::is_none")]
    pub exception_message: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data service job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Job {
    #[serde(flatten)]
    pub dms_base_object: DmsBaseObject,
    #[doc = "Status of the job."]
    pub status: job::Status,
    #[doc = "Time at which the job was started in UTC ISO 8601 format."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "Time at which the job ended in UTC ISO 8601 format."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Job Properties"]
    pub properties: JobProperties,
    #[doc = "Top level error for the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
impl Job {
    pub fn new(status: job::Status, start_time: time::OffsetDateTime, properties: JobProperties) -> Self {
        Self {
            dms_base_object: DmsBaseObject::default(),
            status,
            start_time,
            end_time: None,
            properties,
            error: None,
        }
    }
}
pub mod job {
    use super::*;
    #[doc = "Status of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        None,
        InProgress,
        Succeeded,
        WaitingForAction,
        Failed,
        Cancelled,
        Cancelling,
    }
}
#[doc = "Job Definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDefinition {
    #[serde(flatten)]
    pub dms_base_object: DmsBaseObject,
    #[doc = "Job Definition"]
    pub properties: JobDefinitionProperties,
}
impl JobDefinition {
    pub fn new(properties: JobDefinitionProperties) -> Self {
        Self {
            dms_base_object: DmsBaseObject::default(),
            properties,
        }
    }
}
#[doc = "Contains the supported job definition filters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDefinitionFilter {
    #[doc = "The state of the job definition."]
    pub state: job_definition_filter::State,
    #[doc = "The data source associated with the job definition"]
    #[serde(rename = "dataSource", default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    #[doc = "The last modified date time of the data source."]
    #[serde(rename = "lastModified", with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
}
impl JobDefinitionFilter {
    pub fn new(state: job_definition_filter::State) -> Self {
        Self {
            state,
            data_source: None,
            last_modified: None,
        }
    }
}
pub mod job_definition_filter {
    use super::*;
    #[doc = "The state of the job definition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Disabled,
        Enabled,
        Supported,
    }
}
#[doc = "Job Definition Collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDefinitionList {
    #[doc = "List of job definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobDefinition>,
    #[doc = "Link for the next set of job definitions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobDefinitionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobDefinitionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job Definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDefinitionProperties {
    #[doc = "Data Source Id associated to the job definition."]
    #[serde(rename = "dataSourceId")]
    pub data_source_id: String,
    #[doc = "Data Sink Id associated to the job definition."]
    #[serde(rename = "dataSinkId")]
    pub data_sink_id: String,
    #[doc = "Schedule for running the job definition"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedules: Vec<Schedule>,
    #[doc = "State of the job definition."]
    pub state: job_definition_properties::State,
    #[doc = "Last modified time of the job definition."]
    #[serde(rename = "lastModifiedTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_time: Option<time::OffsetDateTime>,
    #[doc = "This is the preferred geo location for the job to run."]
    #[serde(rename = "runLocation", default, skip_serializing_if = "Option::is_none")]
    pub run_location: Option<job_definition_properties::RunLocation>,
    #[doc = "Enum to detect if user confirmation is required. If not passed will default to NotRequired."]
    #[serde(rename = "userConfirmation", default, skip_serializing_if = "Option::is_none")]
    pub user_confirmation: Option<job_definition_properties::UserConfirmation>,
    #[doc = "A generic json used differently by each data service type."]
    #[serde(rename = "dataServiceInput", default, skip_serializing_if = "Option::is_none")]
    pub data_service_input: Option<serde_json::Value>,
    #[doc = "List of customer secrets containing a key identifier and key value. The key identifier is a way for the specific data source to understand the key. Value contains customer secret encrypted by the encryptionKeys."]
    #[serde(rename = "customerSecrets", default, skip_serializing_if = "Vec::is_empty")]
    pub customer_secrets: Vec<CustomerSecret>,
}
impl JobDefinitionProperties {
    pub fn new(data_source_id: String, data_sink_id: String, state: job_definition_properties::State) -> Self {
        Self {
            data_source_id,
            data_sink_id,
            schedules: Vec::new(),
            state,
            last_modified_time: None,
            run_location: None,
            user_confirmation: None,
            data_service_input: None,
            customer_secrets: Vec::new(),
        }
    }
}
pub mod job_definition_properties {
    use super::*;
    #[doc = "State of the job definition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Disabled,
        Enabled,
        Supported,
    }
    #[doc = "This is the preferred geo location for the job to run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RunLocation {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "australiaeast")]
        Australiaeast,
        #[serde(rename = "australiasoutheast")]
        Australiasoutheast,
        #[serde(rename = "brazilsouth")]
        Brazilsouth,
        #[serde(rename = "canadacentral")]
        Canadacentral,
        #[serde(rename = "canadaeast")]
        Canadaeast,
        #[serde(rename = "centralindia")]
        Centralindia,
        #[serde(rename = "centralus")]
        Centralus,
        #[serde(rename = "eastasia")]
        Eastasia,
        #[serde(rename = "eastus")]
        Eastus,
        #[serde(rename = "eastus2")]
        Eastus2,
        #[serde(rename = "japaneast")]
        Japaneast,
        #[serde(rename = "japanwest")]
        Japanwest,
        #[serde(rename = "koreacentral")]
        Koreacentral,
        #[serde(rename = "koreasouth")]
        Koreasouth,
        #[serde(rename = "southeastasia")]
        Southeastasia,
        #[serde(rename = "southcentralus")]
        Southcentralus,
        #[serde(rename = "southindia")]
        Southindia,
        #[serde(rename = "northcentralus")]
        Northcentralus,
        #[serde(rename = "northeurope")]
        Northeurope,
        #[serde(rename = "uksouth")]
        Uksouth,
        #[serde(rename = "ukwest")]
        Ukwest,
        #[serde(rename = "westcentralus")]
        Westcentralus,
        #[serde(rename = "westeurope")]
        Westeurope,
        #[serde(rename = "westindia")]
        Westindia,
        #[serde(rename = "westus")]
        Westus,
        #[serde(rename = "westus2")]
        Westus2,
    }
    #[doc = "Enum to detect if user confirmation is required. If not passed will default to NotRequired."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UserConfirmation {
        NotRequired,
        Required,
    }
    impl Default for UserConfirmation {
        fn default() -> Self {
            Self::NotRequired
        }
    }
}
#[doc = "Job details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDetails {
    #[doc = "List of stages that ran in the job"]
    #[serde(rename = "jobStages", default, skip_serializing_if = "Vec::is_empty")]
    pub job_stages: Vec<JobStages>,
    #[doc = "Job Definition."]
    #[serde(rename = "jobDefinition", default, skip_serializing_if = "Option::is_none")]
    pub job_definition: Option<JobDefinition>,
    #[doc = "Error details for failure. This is optional."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<ErrorDetails>,
    #[doc = "Item Details Link to download files or see details"]
    #[serde(rename = "itemDetailsLink", default, skip_serializing_if = "Option::is_none")]
    pub item_details_link: Option<String>,
}
impl JobDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the information about the filters for the job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobFilter {
    #[doc = "The status of the job."]
    pub status: job_filter::Status,
    #[doc = "The start time of the job."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
}
impl JobFilter {
    pub fn new(status: job_filter::Status) -> Self {
        Self { status, start_time: None }
    }
}
pub mod job_filter {
    use super::*;
    #[doc = "The status of the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        None,
        InProgress,
        Succeeded,
        WaitingForAction,
        Failed,
        Cancelled,
        Cancelling,
    }
}
#[doc = "Job Collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobList {
    #[doc = "List of jobs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Job>,
    #[doc = "Link for the next set of jobs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobProperties {
    #[doc = "Describes whether the job is cancellable."]
    #[serde(rename = "isCancellable")]
    pub is_cancellable: job_properties::IsCancellable,
    #[doc = "Number of bytes processed by the job as of now."]
    #[serde(rename = "bytesProcessed", default, skip_serializing_if = "Option::is_none")]
    pub bytes_processed: Option<i64>,
    #[doc = "Number of items processed by the job as of now"]
    #[serde(rename = "itemsProcessed", default, skip_serializing_if = "Option::is_none")]
    pub items_processed: Option<i64>,
    #[doc = "Number of bytes to be processed by the job in total."]
    #[serde(rename = "totalBytesToProcess", default, skip_serializing_if = "Option::is_none")]
    pub total_bytes_to_process: Option<i64>,
    #[doc = "Number of items to be processed by the job in total"]
    #[serde(rename = "totalItemsToProcess", default, skip_serializing_if = "Option::is_none")]
    pub total_items_to_process: Option<i64>,
    #[doc = "Job details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<JobDetails>,
    #[doc = "Name of the data source on which the job was triggered."]
    #[serde(rename = "dataSourceName", default, skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    #[doc = "Name of the data sink on which the job was triggered."]
    #[serde(rename = "dataSinkName", default, skip_serializing_if = "Option::is_none")]
    pub data_sink_name: Option<String>,
}
impl JobProperties {
    pub fn new(is_cancellable: job_properties::IsCancellable) -> Self {
        Self {
            is_cancellable,
            bytes_processed: None,
            items_processed: None,
            total_bytes_to_process: None,
            total_items_to_process: None,
            details: None,
            data_source_name: None,
            data_sink_name: None,
        }
    }
}
pub mod job_properties {
    use super::*;
    #[doc = "Describes whether the job is cancellable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IsCancellable {
        NotCancellable,
        Cancellable,
    }
}
#[doc = "Job stages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStages {
    #[doc = "Name of the job stage."]
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[doc = "Status of the job stage."]
    #[serde(rename = "stageStatus")]
    pub stage_status: job_stages::StageStatus,
    #[doc = "Job Stage Details"]
    #[serde(rename = "jobStageDetails", default, skip_serializing_if = "Option::is_none")]
    pub job_stage_details: Option<serde_json::Value>,
    #[doc = "Error details for the stage. This is optional"]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<ErrorDetails>,
}
impl JobStages {
    pub fn new(stage_status: job_stages::StageStatus) -> Self {
        Self {
            stage_name: None,
            stage_status,
            job_stage_details: None,
            error_details: Vec::new(),
        }
    }
}
pub mod job_stages {
    use super::*;
    #[doc = "Status of the job stage."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageStatus {
        None,
        InProgress,
        Succeeded,
        WaitingForAction,
        Failed,
        Cancelled,
        Cancelling,
    }
}
#[doc = "Encryption Key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Key {
    #[doc = "Modulus of the encryption key."]
    #[serde(rename = "keyModulus")]
    pub key_modulus: String,
    #[doc = "Exponent of the encryption key."]
    #[serde(rename = "keyExponent")]
    pub key_exponent: String,
    #[doc = "The maximum byte size that can be encrypted by the key. For a key size larger than the size, break into chunks and encrypt each chunk, append each encrypted chunk with : to mark the end of the chunk."]
    #[serde(rename = "encryptionChunkSizeInBytes")]
    pub encryption_chunk_size_in_bytes: i32,
}
impl Key {
    pub fn new(key_modulus: String, key_exponent: String, encryption_chunk_size_in_bytes: i32) -> Self {
        Self {
            key_modulus,
            key_exponent,
            encryption_chunk_size_in_bytes,
        }
    }
}
#[doc = "Public key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicKey {
    #[serde(flatten)]
    pub dms_base_object: DmsBaseObject,
    #[doc = "PublicKey Properties"]
    pub properties: PublicKeyProperties,
}
impl PublicKey {
    pub fn new(properties: PublicKeyProperties) -> Self {
        Self {
            dms_base_object: DmsBaseObject::default(),
            properties,
        }
    }
}
#[doc = "PublicKey Collection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicKeyList {
    #[doc = "List of public keys."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PublicKey>,
    #[doc = "Link for the next set of public keys."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PublicKeyList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PublicKeyList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "PublicKey Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicKeyProperties {
    #[doc = "Encryption Key."]
    #[serde(rename = "dataServiceLevel1Key")]
    pub data_service_level1_key: Key,
    #[doc = "Encryption Key."]
    #[serde(rename = "dataServiceLevel2Key")]
    pub data_service_level2_key: Key,
}
impl PublicKeyProperties {
    pub fn new(data_service_level1_key: Key, data_service_level2_key: Key) -> Self {
        Self {
            data_service_level1_key,
            data_service_level2_key,
        }
    }
}
#[doc = "Model of the Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "The Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The Resource Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The location of the resource. This will be one of the supported and registered Azure Geo Regions (e.g. West US, East\r\nUS, Southeast Asia, etc.). The geo region of a resource cannot be changed once it is created, but if an identical geo\r\nregion is specified on update the request will succeed."]
    pub location: String,
    #[doc = "The list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource\r\n(across resource groups)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The sku type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
            sku: None,
        }
    }
}
#[doc = "Run parameters for a job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunParameters {
    #[doc = "Enum to detect if user confirmation is required. If not passed will default to NotRequired."]
    #[serde(rename = "userConfirmation", default, skip_serializing_if = "Option::is_none")]
    pub user_confirmation: Option<run_parameters::UserConfirmation>,
    #[doc = "A generic json used differently by each data service type."]
    #[serde(rename = "dataServiceInput", default, skip_serializing_if = "Option::is_none")]
    pub data_service_input: Option<serde_json::Value>,
    #[doc = "List of customer secrets containing a key identifier and key value. The key identifier is a way for the specific data source to understand the key. Value contains customer secret encrypted by the encryptionKeys."]
    #[serde(rename = "customerSecrets", default, skip_serializing_if = "Vec::is_empty")]
    pub customer_secrets: Vec<CustomerSecret>,
}
impl RunParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod run_parameters {
    use super::*;
    #[doc = "Enum to detect if user confirmation is required. If not passed will default to NotRequired."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UserConfirmation {
        NotRequired,
        Required,
    }
    impl Default for UserConfirmation {
        fn default() -> Self {
            Self::NotRequired
        }
    }
}
#[doc = "Schedule for the job run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Schedule {
    #[doc = "Name of the schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A list of repetition intervals in ISO 8601 format."]
    #[serde(rename = "policyList", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_list: Vec<String>,
}
impl Schedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The sku type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "The sku name. Required for data manager creation, optional for update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The sku tier. This is based on the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
