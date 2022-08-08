#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The active directory group user definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdGroupUser {
    #[serde(flatten)]
    pub user: User,
    #[doc = "The AAD tenant ID of the AD Group."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[doc = "The AAD object ID of the AD Group."]
    #[serde(rename = "objectId")]
    pub object_id: String,
}
impl AdGroupUser {
    pub fn new(user: User, tenant_id: String, object_id: String) -> Self {
        Self {
            user,
            tenant_id,
            object_id,
        }
    }
}
#[doc = "Can be anything: string, number, array, object, etc. (except `null`)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AnyValue {}
impl AnyValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The access token definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiToken {
    #[serde(flatten)]
    pub permission: Permission,
    #[doc = "Unique ID of the API token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Value of the API token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[doc = "String-formatted date representing the time when the token expires."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub expiry: Option<time::OffsetDateTime>,
}
impl ApiToken {
    pub fn new(permission: Permission) -> Self {
        Self {
            permission,
            id: None,
            token: None,
            expiry: None,
        }
    }
}
#[doc = "The paged results of API tokens."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiTokenCollection {
    #[doc = "The collection of API tokens."]
    pub value: Vec<ApiToken>,
    #[doc = "URL to get the next page of API tokens."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiTokenCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApiTokenCollection {
    pub fn new(value: Vec<ApiToken>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The attestation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attestation {
    #[doc = "Type of the attestation."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl Attestation {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "Configuration specifying options for a bar chart tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BarChartConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[serde(flatten)]
    pub group_tile_configuration: GroupTileConfiguration,
    #[serde(flatten)]
    pub tile_capability_configuration: TileCapabilityConfiguration,
    #[doc = "Configuration specifying formatting options for a chart tile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<ChartFormatConfiguration>,
    #[doc = "Configuration specifying the time range and resolution of data to return for a tile."]
    #[serde(rename = "queryRange")]
    pub query_range: TimeQueryRangeConfiguration,
}
impl BarChartConfiguration {
    pub fn new(
        tile_configuration: TileConfiguration,
        group_tile_configuration: GroupTileConfiguration,
        query_range: TimeQueryRangeConfiguration,
    ) -> Self {
        Self {
            tile_configuration,
            group_tile_configuration,
            tile_capability_configuration: TileCapabilityConfiguration::default(),
            format: None,
            query_range,
        }
    }
}
#[doc = "The blob storage destination definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobStorageV1Destination {
    #[serde(flatten)]
    pub destination: Destination,
    #[doc = "The authentication definition for blob storage destination."]
    pub authorization: BlobStorageV1DestinationAuth,
}
impl BlobStorageV1Destination {
    pub fn new(destination: Destination, authorization: BlobStorageV1DestinationAuth) -> Self {
        Self {
            destination,
            authorization,
        }
    }
}
#[doc = "The authentication definition for blob storage destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobStorageV1DestinationAuth {
    #[doc = "The kind of authentication to use."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl BlobStorageV1DestinationAuth {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "The authentication definition with connection string for blob storage destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobStorageV1DestinationConnectionStringAuth {
    #[serde(flatten)]
    pub blob_storage_v1_destination_auth: BlobStorageV1DestinationAuth,
    #[doc = "The connection string for accessing the blob storage account."]
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    #[doc = "Name of the container where data should be written in the storage account."]
    #[serde(rename = "containerName")]
    pub container_name: String,
}
impl BlobStorageV1DestinationConnectionStringAuth {
    pub fn new(blob_storage_v1_destination_auth: BlobStorageV1DestinationAuth, connection_string: String, container_name: String) -> Self {
        Self {
            blob_storage_v1_destination_auth,
            connection_string,
            container_name,
        }
    }
}
#[doc = "The authentication definition with system assigned managed identity for blob storage destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobStorageV1DestinationSystemAssignedManagedIdentityAuth {
    #[serde(flatten)]
    pub blob_storage_v1_destination_auth: BlobStorageV1DestinationAuth,
    #[doc = "The storage account's blob service endpoint URL."]
    #[serde(rename = "endpointUri")]
    pub endpoint_uri: String,
    #[doc = "Name of the container where data should be written in the storage account."]
    #[serde(rename = "containerName")]
    pub container_name: String,
}
impl BlobStorageV1DestinationSystemAssignedManagedIdentityAuth {
    pub fn new(blob_storage_v1_destination_auth: BlobStorageV1DestinationAuth, endpoint_uri: String, container_name: String) -> Self {
        Self {
            blob_storage_v1_destination_auth,
            endpoint_uri,
            container_name,
        }
    }
}
#[doc = "The type of aggregation to be applied on capability data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CapabilityAggregateFunctionType {
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "avg")]
    Avg,
}
impl Default for CapabilityAggregateFunctionType {
    fn default() -> Self {
        Self::Count
    }
}
#[doc = "The capability job data definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilityJobData {
    #[doc = "The device template which defines the target capability for the job."]
    pub target: String,
    #[doc = "The path to the target capability within the device template."]
    pub path: String,
    #[doc = "Can be anything: string, number, array, object, etc. (except `null`)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<AnyValue>,
}
impl CapabilityJobData {
    pub fn new(target: String, path: String) -> Self {
        Self { target, path, value: None }
    }
}
#[doc = "Configuration specifying formatting options for a chart tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChartFormatConfiguration {
    #[doc = "Whether to display the x-axis"]
    #[serde(rename = "xAxisEnabled", default, skip_serializing_if = "Option::is_none")]
    pub x_axis_enabled: Option<bool>,
    #[doc = "Whether to display the y-axis"]
    #[serde(rename = "yAxisEnabled", default, skip_serializing_if = "Option::is_none")]
    pub y_axis_enabled: Option<bool>,
    #[doc = "Whether to display the legend"]
    #[serde(rename = "legendEnabled", default, skip_serializing_if = "Option::is_none")]
    pub legend_enabled: Option<bool>,
}
impl ChartFormatConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The cloud property job data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudPropertyJobData {
    #[serde(flatten)]
    pub job_data: JobData,
    #[serde(flatten)]
    pub capability_job_data: CapabilityJobData,
}
impl CloudPropertyJobData {
    pub fn new(job_data: JobData, capability_job_data: CapabilityJobData) -> Self {
        Self {
            job_data,
            capability_job_data,
        }
    }
}
#[doc = "The collection of entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    #[doc = "The collection of entities."]
    pub value: Vec<serde_json::Value>,
    #[doc = "URL to get the next page of entities."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl Collection {
    pub fn new(value: Vec<serde_json::Value>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Configuration specifying options for a command tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[doc = "The device id that the command is associated with"]
    pub device: String,
    #[doc = "The command id to associate the tile to"]
    pub command: String,
}
impl CommandConfiguration {
    pub fn new(tile_configuration: TileConfiguration, device: String, command: String) -> Self {
        Self {
            tile_configuration,
            device,
            command,
        }
    }
}
#[doc = "The command job data definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandJobData {
    #[serde(flatten)]
    pub job_data: JobData,
    #[serde(flatten)]
    pub capability_job_data: CapabilityJobData,
}
impl CommandJobData {
    pub fn new(job_data: JobData, capability_job_data: CapabilityJobData) -> Self {
        Self {
            job_data,
            capability_job_data,
        }
    }
}
#[doc = "Configuration specifying options for a command tile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandTileConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[doc = "The ID of the device group to display"]
    pub group: String,
    #[doc = "The command to reference in the tile"]
    pub command: String,
    #[doc = "The device to reference in the tile"]
    pub device: serde_json::Value,
}
impl CommandTileConfiguration {
    pub fn new(tile_configuration: TileConfiguration, group: String, command: String, device: serde_json::Value) -> Self {
        Self {
            tile_configuration,
            group,
            command,
            device,
        }
    }
}
#[doc = "Configuration specifying the number of data points to query for a tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CountQueryRangeConfiguration {
    #[serde(flatten)]
    pub query_range_configuration: QueryRangeConfiguration,
    #[doc = "The maximum number of data points to query for."]
    pub count: i64,
}
impl CountQueryRangeConfiguration {
    pub fn new(query_range_configuration: QueryRangeConfiguration, count: i64) -> Self {
        Self {
            query_range_configuration,
            count,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dashboard {
    #[doc = "Unique ID of the dashboard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the dashboard."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The tiles displayed by the dashboard."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tiles: Vec<Tile>,
    #[doc = "Whether the dashboard is personal and can only be viewed by the current user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal: Option<bool>,
    #[doc = "Whether the dashboard is favorited or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,
    #[doc = "Etag to prevent conflict when updating the dashboard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "The organization the dashboard belongs to. If not present, the dashboard is root-level or personal. only one organization is supported today, multiple organizations will be supported soon."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organizations: Vec<String>,
}
impl Dashboard {
    pub fn new(display_name: String) -> Self {
        Self {
            id: None,
            display_name,
            tiles: Vec::new(),
            personal: None,
            favorite: None,
            etag: None,
            organizations: Vec::new(),
        }
    }
}
#[doc = "The paged results of dashboards."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardCollection {
    #[doc = "The collection of dashboards."]
    pub value: Vec<Dashboard>,
    #[doc = "URL to get the next page of dashboards."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DashboardCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DashboardCollection {
    pub fn new(value: Vec<Dashboard>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Configuration specifying options for an image tile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExplorerTileConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[doc = "Configuration specifying the time range and resolution of data to return for a tile."]
    #[serde(rename = "queryRange")]
    pub query_range: TimeQueryRangeConfiguration,
    #[doc = "The id of the Data Explorer query to show in the tile"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}
impl DataExplorerTileConfiguration {
    pub fn new(tile_configuration: TileConfiguration, query_range: TimeQueryRangeConfiguration) -> Self {
        Self {
            tile_configuration,
            query_range,
            query: None,
        }
    }
}
#[doc = "The azure data explorer destination definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExplorerV1Destination {
    #[serde(flatten)]
    pub destination: Destination,
    #[doc = "The resource URI of the Data Explorer instance."]
    #[serde(rename = "clusterUrl")]
    pub cluster_url: String,
    #[doc = "Name Data Explorer database where data should be written."]
    pub database: String,
    #[doc = "The table within the Data Explorer database that will receive the data."]
    pub table: String,
    #[doc = "The authentication definition for azure data explorer destination."]
    pub authorization: DataExplorerV1DestinationAuth,
}
impl DataExplorerV1Destination {
    pub fn new(
        destination: Destination,
        cluster_url: String,
        database: String,
        table: String,
        authorization: DataExplorerV1DestinationAuth,
    ) -> Self {
        Self {
            destination,
            cluster_url,
            database,
            table,
            authorization,
        }
    }
}
#[doc = "The authentication definition for azure data explorer destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExplorerV1DestinationAuth {
    #[doc = "The kind of authentication to use."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl DataExplorerV1DestinationAuth {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "The authentication definition with service principal for azure data explorer destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExplorerV1DestinationServicePrincipalAuth {
    #[serde(flatten)]
    pub data_explorer_v1_destination_auth: DataExplorerV1DestinationAuth,
    #[doc = "Service Principal client ID."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "Service Principal tenant ID."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[doc = "Service Principal client secret."]
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
}
impl DataExplorerV1DestinationServicePrincipalAuth {
    pub fn new(
        data_explorer_v1_destination_auth: DataExplorerV1DestinationAuth,
        client_id: String,
        tenant_id: String,
        client_secret: String,
    ) -> Self {
        Self {
            data_explorer_v1_destination_auth,
            client_id,
            tenant_id,
            client_secret,
        }
    }
}
#[doc = "The authentication definition with system assigned managed identity for azure data explorer destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExplorerV1DestinationSystemAssignedManagedIdentityAuth {
    #[serde(flatten)]
    pub data_explorer_v1_destination_auth: DataExplorerV1DestinationAuth,
}
impl DataExplorerV1DestinationSystemAssignedManagedIdentityAuth {
    pub fn new(data_explorer_v1_destination_auth: DataExplorerV1DestinationAuth) -> Self {
        Self {
            data_explorer_v1_destination_auth,
        }
    }
}
#[doc = "The data export error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataExportError {
    #[doc = "The code for the error that occurred."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The description of the error that occurred."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl DataExportError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The data export status definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataExportStatus {
    #[doc = "Indication of the current health and operation of the export or destination."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Errors encountered by the export or destination."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<DataExportError>,
    #[doc = "The timestamp of the last message that was sent to the export or destination."]
    #[serde(rename = "lastExportTime", with = "azure_core::date::rfc3339::option")]
    pub last_export_time: Option<time::OffsetDateTime>,
}
impl DataExportStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The date based end definition of job schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateJobScheduleEnd {
    #[serde(flatten)]
    pub job_schedule_end: JobScheduleEnd,
    #[doc = "The date when to end the scheduled job."]
    pub date: String,
}
impl DateJobScheduleEnd {
    pub fn new(job_schedule_end: JobScheduleEnd, date: String) -> Self {
        Self { job_schedule_end, date }
    }
}
#[doc = "The destination definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Destination {
    #[serde(flatten)]
    pub data_export_status: DataExportStatus,
    #[doc = "Unique ID of the destination."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the destination."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The type of destination configuration."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl Destination {
    pub fn new(display_name: String, type_: String) -> Self {
        Self {
            data_export_status: DataExportStatus::default(),
            id: None,
            display_name,
            type_,
        }
    }
}
#[doc = "The paged results of destinations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DestinationCollection {
    #[doc = "The collection of destinations."]
    pub value: Vec<Destination>,
    #[doc = "URL to get the next page of destinations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DestinationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DestinationCollection {
    pub fn new(value: Vec<Destination>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The destination export definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DestinationExport {
    #[serde(flatten)]
    pub export: Export,
    #[doc = "Query for transforming the message structure to a particular output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<String>,
}
impl DestinationExport {
    pub fn new(export: Export) -> Self {
        Self { export, transform: None }
    }
}
#[doc = "The destination reference definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DestinationReference {
    #[doc = "The ID of the destination where data should be sent."]
    pub id: String,
    #[doc = "Query for transforming the message structure to a particular output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<String>,
}
impl DestinationReference {
    pub fn new(id: String) -> Self {
        Self { id, transform: None }
    }
}
#[doc = "The device definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Device {
    #[doc = "Unique ID of the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "ETag used to prevent conflict in device updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Display name of the device."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The device template definition for the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[doc = "Whether the device connection to IoT Central has been enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Whether resources have been allocated for the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioned: Option<bool>,
    #[doc = "Whether the device is simulated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simulated: Option<bool>,
    #[doc = "List of organization IDs that the device is a part of, only one organization is supported today, multiple organizations will be supported soon."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organizations: Vec<String>,
}
impl Device {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paged results of devices."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCollection {
    #[doc = "The collection of devices."]
    pub value: Vec<Device>,
    #[doc = "URL to get the next page of devices."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceCollection {
    pub fn new(value: Vec<Device>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The device command definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceCommand {
    #[doc = "The request ID of the device command execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Connection timeout in seconds to wait for a disconnected device to come online. Defaults to 0 seconds."]
    #[serde(rename = "connectionTimeout", default, skip_serializing_if = "Option::is_none")]
    pub connection_timeout: Option<i64>,
    #[doc = "Response timeout in seconds to wait for a command completion on a device. Defaults to 30 seconds."]
    #[serde(rename = "responseTimeout", default, skip_serializing_if = "Option::is_none")]
    pub response_timeout: Option<i64>,
    #[doc = "The payload for the device command, support any primitive types or object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<serde_json::Value>,
    #[doc = "The payload of the device command response, support any primitive types or object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<serde_json::Value>,
    #[doc = "The status code of the device command response."]
    #[serde(rename = "responseCode", default, skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i64>,
}
impl DeviceCommand {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paged results of device command executions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCommandCollection {
    #[doc = "The collection of device command executions."]
    pub value: Vec<DeviceCommand>,
    #[doc = "URL to get the next page of device command executions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceCommandCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceCommandCollection {
    pub fn new(value: Vec<DeviceCommand>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Configuration specifying options for a device count tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCountTileConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[doc = "The ID of the device group to display"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[doc = "Configuration specifying formatting options for a text based tile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<TextFormatConfiguration>,
}
impl DeviceCountTileConfiguration {
    pub fn new(tile_configuration: TileConfiguration) -> Self {
        Self {
            tile_configuration,
            group: None,
            format: None,
        }
    }
}
#[doc = "The device credentials definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceCredentials {
    #[doc = "ID scope for connecting to the IoT Central application."]
    #[serde(rename = "idScope")]
    pub id_scope: String,
    #[doc = "The symmetric key definition."]
    #[serde(rename = "symmetricKey", default, skip_serializing_if = "Option::is_none")]
    pub symmetric_key: Option<SymmetricKey>,
    #[doc = "The X509 definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x509: Option<X509>,
    #[doc = "The trusted platform module definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tpm: Option<Tpm>,
}
impl DeviceCredentials {
    pub fn new(id_scope: String) -> Self {
        Self {
            id_scope,
            symmetric_key: None,
            x509: None,
            tpm: None,
        }
    }
}
#[doc = "The device group definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceGroup {
    #[doc = "Unique ID of the device group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the device group."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Query defining which devices should be in this group, [Query Language Reference](https://aka.ms/iotcquery)."]
    pub filter: String,
    #[doc = "Short summary of device group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "ETag used to prevent conflict in device group updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "List of organization IDs of the device group, only one organization is supported today, multiple organizations will be supported soon."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organizations: Vec<String>,
}
impl DeviceGroup {
    pub fn new(display_name: String, filter: String) -> Self {
        Self {
            id: None,
            display_name,
            filter,
            description: None,
            etag: None,
            organizations: Vec::new(),
        }
    }
}
#[doc = "The paged results of device groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceGroupCollection {
    #[doc = "The collection of device groups."]
    pub value: Vec<DeviceGroup>,
    #[doc = "URL to get the next page of device groups."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceGroupCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceGroupCollection {
    pub fn new(value: Vec<DeviceGroup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The paged results of devices belonging to the device group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceGroupDeviceCollection {
    #[doc = "The collection of devices belonging to the device group."]
    pub value: Vec<Device>,
    #[doc = "URL to get the next page of devices in the group."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceGroupDeviceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceGroupDeviceCollection {
    pub fn new(value: Vec<Device>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Property values associated with the device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceProperties {}
impl DeviceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object representing the relationship between an upstream and a downstream device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceRelationship {
    #[doc = "The unique identifier of this relationship."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The device ID of the source (parent) device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "The device ID of the target (child) device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl DeviceRelationship {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paged results of device relationships."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceRelationshipCollection {
    #[doc = "The collection of device relationships."]
    pub value: Vec<DeviceRelationship>,
    #[doc = "URL to get the next page of device relationships."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DeviceRelationshipCollection {
    pub fn new(value: Vec<DeviceRelationship>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The device telemetry definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceTelemetry {
    #[doc = "The last known value of this device telemetry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[doc = "String-formatted date representing the time when the telemetry value was sent."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
}
impl DeviceTelemetry {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The device template definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceTemplate {
    #[doc = "Unique ID of the device template."]
    #[serde(rename = "@id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The JSON-LD types of this device template."]
    #[serde(rename = "@type")]
    pub type_: Vec<String>,
    #[doc = "ETag used to prevent conflict in device template updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Display name of the device template."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Detailed description of the device template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The capability model utilized by this device template."]
    #[serde(rename = "capabilityModel")]
    pub capability_model: serde_json::Value,
    #[doc = "Deployment manifest associated to this device template."]
    #[serde(rename = "deploymentManifest", default, skip_serializing_if = "Option::is_none")]
    pub deployment_manifest: Option<serde_json::Value>,
}
impl DeviceTemplate {
    pub fn new(type_: Vec<String>, capability_model: serde_json::Value) -> Self {
        Self {
            id: None,
            type_,
            etag: None,
            display_name: None,
            description: None,
            capability_model,
            deployment_manifest: None,
        }
    }
}
#[doc = "The paged results of device templates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceTemplateCollection {
    #[doc = "The collection of device templates."]
    pub value: Vec<DeviceTemplate>,
    #[doc = "URL to get the next page of device templates."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceTemplateCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceTemplateCollection {
    pub fn new(value: Vec<DeviceTemplate>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The device template migration job data definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceTemplateMigrationJobData {
    #[serde(flatten)]
    pub job_data: JobData,
    #[doc = "The target device template to which devices will be migrated."]
    pub template: String,
}
impl DeviceTemplateMigrationJobData {
    pub fn new(job_data: JobData, template: String) -> Self {
        Self { job_data, template }
    }
}
#[doc = "The email user definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailUser {
    #[serde(flatten)]
    pub user: User,
    #[doc = "Email address of the user."]
    pub email: String,
}
impl EmailUser {
    pub fn new(user: User, email: String) -> Self {
        Self { user, email }
    }
}
#[doc = "The enrichment definition for data export."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Enrichment {
    #[doc = "The device template or interface which defines the target capability for the enrichment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The path to the target capability within the device template or the system property to use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The raw value used for the enrichment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
impl Enrichment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The enrollment group definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrollmentGroup {
    #[doc = "Unique ID of the enrollment group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the enrollment group."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Whether the devices using the group are allowed to connect to IoT Central."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Type of devices that connect through the group."]
    #[serde(rename = "type")]
    pub type_: enrollment_group::Type,
    #[doc = "The attestation definition for an enrollment group."]
    pub attestation: GroupAttestation,
    #[doc = "ETag used to prevent conflict in enrollment group updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl EnrollmentGroup {
    pub fn new(display_name: String, type_: enrollment_group::Type, attestation: GroupAttestation) -> Self {
        Self {
            id: None,
            display_name,
            enabled: None,
            type_,
            attestation,
            etag: None,
        }
    }
}
pub mod enrollment_group {
    use super::*;
    #[doc = "Type of devices that connect through the group."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "iot")]
        Iot,
        #[serde(rename = "iotEdge")]
        IotEdge,
    }
}
#[doc = "The paged results of enrollment groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrollmentGroupCollection {
    #[doc = "The collection of enrollment groups."]
    pub value: Vec<EnrollmentGroup>,
    #[doc = "URL to get the next page of enrollment groups."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EnrollmentGroupCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EnrollmentGroupCollection {
    pub fn new(value: Vec<EnrollmentGroup>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The response error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[doc = "The detail information of the error."]
    pub error: ErrorDetails,
}
impl azure_core::Continuable for Error {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Error {
    pub fn new(error: ErrorDetails) -> Self {
        Self { error }
    }
}
#[doc = "The detail information of the error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetails {
    #[doc = "Error code."]
    pub code: String,
    #[doc = "Error message details."]
    pub message: String,
    #[doc = "Correlation Id for current request."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The time that error request failed."]
    #[serde(with = "azure_core::date::rfc1123::option")]
    pub time: Option<time::OffsetDateTime>,
}
impl ErrorDetails {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            request_id: None,
            time: None,
        }
    }
}
#[doc = "Configuration specifying options for a event chart tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventChartConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[serde(flatten)]
    pub group_tile_configuration: GroupTileConfiguration,
    #[serde(flatten)]
    pub tile_capability_configuration: TileCapabilityConfiguration,
    #[doc = "Configuration specifying the time range and resolution of data to return for a tile."]
    #[serde(rename = "queryRange")]
    pub query_range: TimeQueryRangeConfiguration,
}
impl EventChartConfiguration {
    pub fn new(
        tile_configuration: TileConfiguration,
        group_tile_configuration: GroupTileConfiguration,
        query_range: TimeQueryRangeConfiguration,
    ) -> Self {
        Self {
            tile_configuration,
            group_tile_configuration,
            tile_capability_configuration: TileCapabilityConfiguration::default(),
            query_range,
        }
    }
}
#[doc = "Configuration specifying options for a event history chart tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHistoryChartConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[serde(flatten)]
    pub group_tile_configuration: GroupTileConfiguration,
    #[serde(flatten)]
    pub tile_capability_configuration: TileCapabilityConfiguration,
    #[doc = "Configuration specifying the time range and resolution of data to return for a tile."]
    #[serde(rename = "queryRange")]
    pub query_range: TimeQueryRangeConfiguration,
    #[doc = "Configuration specifying formatting options for a text based tile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<TextFormatConfiguration>,
}
impl EventHistoryChartConfiguration {
    pub fn new(
        tile_configuration: TileConfiguration,
        group_tile_configuration: GroupTileConfiguration,
        query_range: TimeQueryRangeConfiguration,
    ) -> Self {
        Self {
            tile_configuration,
            group_tile_configuration,
            tile_capability_configuration: TileCapabilityConfiguration::default(),
            query_range,
            format: None,
        }
    }
}
#[doc = "the event hub destination definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubsV1Destination {
    #[serde(flatten)]
    pub destination: Destination,
    #[doc = "The authentication definition for event hub destination."]
    pub authorization: EventHubsV1DestinationAuth,
}
impl EventHubsV1Destination {
    pub fn new(destination: Destination, authorization: EventHubsV1DestinationAuth) -> Self {
        Self {
            destination,
            authorization,
        }
    }
}
#[doc = "The authentication definition for event hub destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubsV1DestinationAuth {
    #[doc = "The kind of authentication to use."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl EventHubsV1DestinationAuth {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "The authentication definition with connection string for event hub destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubsV1DestinationConnectionStringAuth {
    #[serde(flatten)]
    pub event_hubs_v1_destination_auth: EventHubsV1DestinationAuth,
    #[doc = "The connection string for accessing the Event Hubs namespace, including the `EntityPath` of the event hub."]
    #[serde(rename = "connectionString")]
    pub connection_string: String,
}
impl EventHubsV1DestinationConnectionStringAuth {
    pub fn new(event_hubs_v1_destination_auth: EventHubsV1DestinationAuth, connection_string: String) -> Self {
        Self {
            event_hubs_v1_destination_auth,
            connection_string,
        }
    }
}
#[doc = "The authentication definition with system assigned managed identity for event hub destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubsV1DestinationSystemAssignedManagedIdentityAuth {
    #[serde(flatten)]
    pub event_hubs_v1_destination_auth: EventHubsV1DestinationAuth,
    #[doc = "The host name of the Event Hubs namespace."]
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[doc = "The Event Hubs instance name."]
    #[serde(rename = "eventHubName")]
    pub event_hub_name: String,
}
impl EventHubsV1DestinationSystemAssignedManagedIdentityAuth {
    pub fn new(event_hubs_v1_destination_auth: EventHubsV1DestinationAuth, host_name: String, event_hub_name: String) -> Self {
        Self {
            event_hubs_v1_destination_auth,
            host_name,
            event_hub_name,
        }
    }
}
#[doc = "The data export definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Export {
    #[serde(flatten)]
    pub data_export_status: DataExportStatus,
    #[doc = "Unique ID of the export."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the export."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Toggle to start/stop an export from sending data."]
    pub enabled: bool,
    #[doc = "The type of data to export."]
    pub source: export::Source,
    #[doc = "Query defining which events from the source should be exported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[doc = "Additional pieces of information to include with each sent message. Data is represented as a set of key/value pairs, where the key is the name of the enrichment that will appear in the output message and the value identifies the data to send."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enrichments: Option<serde_json::Value>,
    #[doc = "The list of destinations to which the export should send data."]
    pub destinations: Vec<DestinationReference>,
}
impl Export {
    pub fn new(display_name: String, enabled: bool, source: export::Source, destinations: Vec<DestinationReference>) -> Self {
        Self {
            data_export_status: DataExportStatus::default(),
            id: None,
            display_name,
            enabled,
            source,
            filter: None,
            enrichments: None,
            destinations,
        }
    }
}
pub mod export {
    use super::*;
    #[doc = "The type of data to export."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Source {
        #[serde(rename = "telemetry")]
        Telemetry,
        #[serde(rename = "properties")]
        Properties,
        #[serde(rename = "deviceLifecycle")]
        DeviceLifecycle,
        #[serde(rename = "deviceTemplateLifecycle")]
        DeviceTemplateLifecycle,
        #[serde(rename = "deviceConnectivity")]
        DeviceConnectivity,
    }
}
#[doc = "The paged results of exports."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportCollection {
    #[doc = "The collection of exports."]
    pub value: Vec<Export>,
    #[doc = "URL to get the next page of exports."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ExportCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExportCollection {
    pub fn new(value: Vec<Export>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The export destination definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDestination {
    #[serde(flatten)]
    pub destination: Destination,
    #[doc = "Query for transforming the message structure to a particular output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<String>,
}
impl ExportDestination {
    pub fn new(destination: Destination) -> Self {
        Self {
            destination,
            transform: None,
        }
    }
}
#[doc = "Configuration specifying options for an external content tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalContentTileConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[doc = "URL of the website to render inside the tile. Must be a valid HTTPS URL."]
    #[serde(rename = "sourceUrl")]
    pub source_url: String,
}
impl ExternalContentTileConfiguration {
    pub fn new(tile_configuration: TileConfiguration, source_url: String) -> Self {
        Self {
            tile_configuration,
            source_url,
        }
    }
}
#[doc = "The file upload configuration definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileUpload {
    #[doc = "The storage account name where to upload the file to"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[doc = "The connection string used to configure the storage account"]
    #[serde(rename = "connectionString")]
    pub connection_string: String,
    #[doc = "The name of the container inside the storage account"]
    pub container: String,
    #[doc = "ISO 8601 duration standard, The amount of time the device’s request to upload a file is valid before it expires."]
    #[serde(rename = "sasTtl", default, skip_serializing_if = "Option::is_none")]
    pub sas_ttl: Option<String>,
    #[doc = "The state of the file upload configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<file_upload::State>,
    #[doc = "ETag used to prevent conflict with multiple uploads"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl FileUpload {
    pub fn new(connection_string: String, container: String) -> Self {
        Self {
            account: None,
            connection_string,
            container,
            sas_ttl: None,
            state: None,
            etag: None,
        }
    }
}
pub mod file_upload {
    use super::*;
    #[doc = "The state of the file upload configuration"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "updating")]
        Updating,
        #[serde(rename = "deleting")]
        Deleting,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
    }
}
#[doc = "The attestation definition for an enrollment group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAttestation {
    #[doc = "Type of the attestation."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl GroupAttestation {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "The symmetric key attestation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupSymmetricKeyAttestation {
    #[serde(flatten)]
    pub group_attestation: GroupAttestation,
    #[doc = "The symmetric key definition."]
    #[serde(rename = "symmetricKey", default, skip_serializing_if = "Option::is_none")]
    pub symmetric_key: Option<SymmetricKey>,
}
impl GroupSymmetricKeyAttestation {
    pub fn new(group_attestation: GroupAttestation) -> Self {
        Self {
            group_attestation,
            symmetric_key: None,
        }
    }
}
#[doc = "Configuration specifying a set of devices to display data for in a tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupTileConfiguration {
    #[doc = "The ID of the device group to display"]
    pub group: String,
    #[doc = "The list of associated devices to display"]
    pub devices: Vec<String>,
}
impl GroupTileConfiguration {
    pub fn new(group: String, devices: Vec<String>) -> Self {
        Self { group, devices }
    }
}
#[doc = "The X509 attestation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupX509Attestation {
    #[serde(flatten)]
    pub group_attestation: GroupAttestation,
    #[doc = "The X509 definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x509: Option<SigningX509>,
}
impl GroupX509Attestation {
    pub fn new(group_attestation: GroupAttestation) -> Self {
        Self {
            group_attestation,
            x509: None,
        }
    }
}
#[doc = "Configuration specifying options for a heat map tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeatMapConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[serde(flatten)]
    pub group_tile_configuration: GroupTileConfiguration,
    #[serde(flatten)]
    pub tile_capability_configuration: TileCapabilityConfiguration,
    #[doc = "Configuration specifying the time range and resolution of data to return for a tile."]
    #[serde(rename = "queryRange")]
    pub query_range: TimeQueryRangeConfiguration,
    #[doc = "Configuration specifying formatting options for a chart tile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<ChartFormatConfiguration>,
}
impl HeatMapConfiguration {
    pub fn new(
        tile_configuration: TileConfiguration,
        group_tile_configuration: GroupTileConfiguration,
        query_range: TimeQueryRangeConfiguration,
    ) -> Self {
        Self {
            tile_configuration,
            group_tile_configuration,
            tile_capability_configuration: TileCapabilityConfiguration::default(),
            query_range,
            format: None,
        }
    }
}
#[doc = "Configuration specifying options for an image tile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageTileConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[doc = "The asset id of the image to display"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[doc = "The URL the tile links to when clicked"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[doc = "Format options for the image tile"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<image_tile_configuration::Format>,
}
impl ImageTileConfiguration {
    pub fn new(tile_configuration: TileConfiguration) -> Self {
        Self {
            tile_configuration,
            image: None,
            href: None,
            format: None,
        }
    }
}
pub mod image_tile_configuration {
    use super::*;
    #[doc = "Format options for the image tile"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Format {
        #[doc = "The background color to show behind the image"]
        #[serde(rename = "backgroundColor", default, skip_serializing_if = "Option::is_none")]
        pub background_color: Option<String>,
        #[doc = "Whether to stretch the image to fit the aspect ratio of the tile or display in the image's native aspect ratio"]
        #[serde(rename = "fitImage", default, skip_serializing_if = "Option::is_none")]
        pub fit_image: Option<bool>,
        #[doc = "The color of the text in the tile"]
        #[serde(rename = "textColor", default, skip_serializing_if = "Option::is_none")]
        pub text_color: Option<String>,
        #[doc = "Size of the test in the tile"]
        #[serde(rename = "textSize", default, skip_serializing_if = "Option::is_none")]
        pub text_size: Option<f64>,
        #[doc = "The unit of size for the text in the tile"]
        #[serde(rename = "textSizeUnit", default, skip_serializing_if = "Option::is_none")]
        pub text_size_unit: Option<format::TextSizeUnit>,
        #[doc = "Whether or not to show the display name text on the tile"]
        #[serde(rename = "showTitle", default, skip_serializing_if = "Option::is_none")]
        pub show_title: Option<bool>,
    }
    impl Format {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod format {
        use super::*;
        #[doc = "The unit of size for the text in the tile"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum TextSizeUnit {
            #[serde(rename = "px")]
            Px,
        }
        impl Default for TextSizeUnit {
            fn default() -> Self {
                Self::Px
            }
        }
    }
}
#[doc = "The job definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Job {
    #[doc = "Unique ID of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Id of the scheduled job definition that created this job."]
    #[serde(rename = "scheduledJobId", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_job_id: Option<String>,
    #[doc = "Display name of the job."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Detailed description of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The ID of the device group on which to execute the job."]
    pub group: String,
    #[doc = "The job batch definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch: Option<JobBatch>,
    #[doc = "The job cancellation threshold definition."]
    #[serde(rename = "cancellationThreshold", default, skip_serializing_if = "Option::is_none")]
    pub cancellation_threshold: Option<JobCancellationThreshold>,
    #[doc = "The capabilities being updated by the job and the values with which they are being updated."]
    pub data: Vec<JobData>,
    #[doc = "The start time of the job"]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub start: Option<time::OffsetDateTime>,
    #[doc = "The end time of the job"]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub end: Option<time::OffsetDateTime>,
    #[doc = "progress summary for a scheduled job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<JobProgress>,
    #[doc = "Indicates whether the job is starting, running, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "List of organizations of the job, only one organization is supported today, multiple organizations will be supported soon."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organizations: Vec<String>,
}
impl Job {
    pub fn new(group: String, data: Vec<JobData>) -> Self {
        Self {
            id: None,
            scheduled_job_id: None,
            display_name: None,
            description: None,
            group,
            batch: None,
            cancellation_threshold: None,
            data,
            start: None,
            end: None,
            progress: None,
            status: None,
            organizations: Vec::new(),
        }
    }
}
#[doc = "The job batch definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobBatch {
    #[doc = "Whether batching is done on a specified number of devices or a percentage of the total devices."]
    #[serde(rename = "type")]
    pub type_: job_batch::Type,
    #[doc = "The number or percentage of devices on which batching is done."]
    pub value: f64,
}
impl JobBatch {
    pub fn new(type_: job_batch::Type, value: f64) -> Self {
        Self { type_, value }
    }
}
pub mod job_batch {
    use super::*;
    #[doc = "Whether batching is done on a specified number of devices or a percentage of the total devices."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "number")]
        Number,
        #[serde(rename = "percentage")]
        Percentage,
    }
}
#[doc = "The job cancellation threshold definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobCancellationThreshold {
    #[doc = "Whether the cancellation threshold is per a specified number of devices or a percentage of the total devices."]
    #[serde(rename = "type")]
    pub type_: job_cancellation_threshold::Type,
    #[doc = "The number or percentage of devices on which the cancellation threshold is applied."]
    pub value: f64,
    #[doc = "Whether the cancellation threshold applies per-batch or to the overall job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch: Option<bool>,
}
impl JobCancellationThreshold {
    pub fn new(type_: job_cancellation_threshold::Type, value: f64) -> Self {
        Self { type_, value, batch: None }
    }
}
pub mod job_cancellation_threshold {
    use super::*;
    #[doc = "Whether the cancellation threshold is per a specified number of devices or a percentage of the total devices."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "number")]
        Number,
        #[serde(rename = "percentage")]
        Percentage,
    }
}
#[doc = "The paged results of jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobCollection {
    #[doc = "The collection of jobs."]
    pub value: Vec<Job>,
    #[doc = "URL to get the next page of jobs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobCollection {
    pub fn new(value: Vec<Job>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The job data definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobData {
    #[doc = "Type of the job data."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl JobData {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "The job device status definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDeviceStatus {
    #[doc = "ID of the device whose job status is being provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Indicates whether the job is starting, running, etc. for the given device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl JobDeviceStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paged results of job device statuses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDeviceStatusCollection {
    #[doc = "The collection of job device statuses."]
    pub value: Vec<JobDeviceStatus>,
    #[doc = "URL to get the next page of job device statuses."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobDeviceStatusCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobDeviceStatusCollection {
    pub fn new(value: Vec<JobDeviceStatus>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "progress summary for a scheduled job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobProgress {
    #[doc = "The total number of entities targeted by the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[doc = "The number of entities for which the job is not yet running."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending: Option<i64>,
    #[doc = "The number of entities for which the job has completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<i64>,
    #[doc = "The number of entities for which the job has failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
}
impl JobProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The schedule definition of job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobSchedule {
    #[doc = "The recurrence of the scheduled job. If not provided, the job will run once at the specified start time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<job_schedule::Recurrence>,
    #[doc = "The start time for the scheduled job"]
    #[serde(with = "azure_core::date::rfc3339")]
    pub start: time::OffsetDateTime,
    #[doc = "The end definition of job schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<JobScheduleEnd>,
}
impl JobSchedule {
    pub fn new(start: time::OffsetDateTime) -> Self {
        Self {
            recurrence: None,
            start,
            end: None,
        }
    }
}
pub mod job_schedule {
    use super::*;
    #[doc = "The recurrence of the scheduled job. If not provided, the job will run once at the specified start time."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Recurrence {
        #[serde(rename = "daily")]
        Daily,
        #[serde(rename = "weekly")]
        Weekly,
        #[serde(rename = "monthly")]
        Monthly,
    }
}
#[doc = "The end definition of job schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobScheduleEnd {
    #[doc = "Type of the job schedule end."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl JobScheduleEnd {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "Configuration specifying options for kpi tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KpiTileConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[serde(flatten)]
    pub group_tile_configuration: GroupTileConfiguration,
    #[serde(flatten)]
    pub tile_capability_configuration: TileCapabilityConfiguration,
    #[doc = "Configuration specifying the time range and resolution of data to return for a tile."]
    #[serde(rename = "queryRange")]
    pub query_range: TimeQueryRangeConfiguration,
    #[doc = "Configuration specifying formatting options for a text based tile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<TextFormatConfiguration>,
}
impl KpiTileConfiguration {
    pub fn new(
        tile_configuration: TileConfiguration,
        group_tile_configuration: GroupTileConfiguration,
        query_range: TimeQueryRangeConfiguration,
    ) -> Self {
        Self {
            tile_configuration,
            group_tile_configuration,
            tile_capability_configuration: TileCapabilityConfiguration::default(),
            query_range,
            format: None,
        }
    }
}
#[doc = "Configuration specifying options for a label tile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LabelTileConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[doc = "The text to display in the tile"]
    pub text: String,
    #[doc = "The font size of the text being displayed"]
    #[serde(rename = "textSize", default, skip_serializing_if = "Option::is_none")]
    pub text_size: Option<f64>,
    #[doc = "The unit of size for the text in the tile"]
    #[serde(rename = "textSizeUnit", default, skip_serializing_if = "Option::is_none")]
    pub text_size_unit: Option<TileTextSizeUnit>,
    #[doc = "Whether to wrap the text being displayed"]
    #[serde(rename = "wordWrap", default, skip_serializing_if = "Option::is_none")]
    pub word_wrap: Option<bool>,
}
impl LabelTileConfiguration {
    pub fn new(tile_configuration: TileConfiguration, text: String) -> Self {
        Self {
            tile_configuration,
            text,
            text_size: None,
            text_size_unit: None,
            word_wrap: None,
        }
    }
}
#[doc = "Configuration specifying options for a line chart tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineChartConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[serde(flatten)]
    pub group_tile_configuration: GroupTileConfiguration,
    #[serde(flatten)]
    pub tile_capability_configuration: TileCapabilityConfiguration,
    #[doc = "Configuration specifying how much data to return for a tile."]
    #[serde(rename = "queryRange")]
    pub query_range: QueryRangeConfiguration,
    #[doc = "Configuration specifying formatting options for a chart tile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<ChartFormatConfiguration>,
}
impl LineChartConfiguration {
    pub fn new(
        tile_configuration: TileConfiguration,
        group_tile_configuration: GroupTileConfiguration,
        query_range: QueryRangeConfiguration,
    ) -> Self {
        Self {
            tile_configuration,
            group_tile_configuration,
            tile_capability_configuration: TileCapabilityConfiguration::default(),
            query_range,
            format: None,
        }
    }
}
#[doc = "Configuration specifying options for a last known value tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LkvTileConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[serde(flatten)]
    pub group_tile_configuration: GroupTileConfiguration,
    #[serde(flatten)]
    pub tile_capability_configuration: TileCapabilityConfiguration,
    #[doc = "Show the trend between the last known value and the value before that"]
    #[serde(rename = "showTrend", default, skip_serializing_if = "Option::is_none")]
    pub show_trend: Option<bool>,
    #[doc = "Configuration specifying formatting options for a text based tile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<TextFormatConfiguration>,
}
impl LkvTileConfiguration {
    pub fn new(tile_configuration: TileConfiguration, group_tile_configuration: GroupTileConfiguration) -> Self {
        Self {
            tile_configuration,
            group_tile_configuration,
            tile_capability_configuration: TileCapabilityConfiguration::default(),
            show_trend: None,
            format: None,
        }
    }
}
#[doc = "Configuration specifying formatting options for a map tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapFormatConfiguration {
    #[doc = "The zoom level of the map"]
    #[serde(rename = "zoomLevel", default, skip_serializing_if = "Option::is_none")]
    pub zoom_level: Option<i64>,
}
impl MapFormatConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration specifying options for a map property tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapPropertyConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[serde(flatten)]
    pub map_tile_configuration: MapTileConfiguration,
}
impl MapPropertyConfiguration {
    pub fn new(tile_configuration: TileConfiguration, map_tile_configuration: MapTileConfiguration) -> Self {
        Self {
            tile_configuration,
            map_tile_configuration,
        }
    }
}
#[doc = "Configuration specifying options for a map telemetry tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapTelemetryConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[serde(flatten)]
    pub map_tile_configuration: MapTileConfiguration,
}
impl MapTelemetryConfiguration {
    pub fn new(tile_configuration: TileConfiguration, map_tile_configuration: MapTileConfiguration) -> Self {
        Self {
            tile_configuration,
            map_tile_configuration,
        }
    }
}
#[doc = "Configuration specifying options for a map tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapTileConfiguration {
    #[serde(flatten)]
    pub group_tile_configuration: GroupTileConfiguration,
    #[serde(flatten)]
    pub tile_capability_configuration: TileCapabilityConfiguration,
    #[doc = "The zoom level of the map"]
    #[serde(rename = "zoomLevel", default, skip_serializing_if = "Option::is_none")]
    pub zoom_level: Option<f64>,
}
impl MapTileConfiguration {
    pub fn new(group_tile_configuration: GroupTileConfiguration) -> Self {
        Self {
            group_tile_configuration,
            tile_capability_configuration: TileCapabilityConfiguration::default(),
            zoom_level: None,
        }
    }
}
#[doc = "Configuration specifying options for a markdown tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkdownTileConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[doc = "Link to visit when tile is clicked"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[doc = "Markdown string to render inside the tile"]
    pub description: String,
    #[doc = "Base64 encoded"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}
impl MarkdownTileConfiguration {
    pub fn new(tile_configuration: TileConfiguration, description: String) -> Self {
        Self {
            tile_configuration,
            href: None,
            description,
            image: None,
        }
    }
}
#[doc = "The occurences based end definition of job schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OccurrencesJobScheduleEnd {
    #[serde(flatten)]
    pub job_schedule_end: JobScheduleEnd,
    #[doc = "The number of occurrences after which to end the scheduled job."]
    pub occurrences: i64,
}
impl OccurrencesJobScheduleEnd {
    pub fn new(job_schedule_end: JobScheduleEnd, occurrences: i64) -> Self {
        Self {
            job_schedule_end,
            occurrences,
        }
    }
}
#[doc = "The organization definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Organization {
    #[doc = "Unique ID of the organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the organization."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "ID of the parent of the organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
}
impl Organization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The paged results of organizations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationCollection {
    #[doc = "The collection of organizations."]
    pub value: Vec<Organization>,
    #[doc = "URL to get the next page of organizations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OrganizationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OrganizationCollection {
    pub fn new(value: Vec<Organization>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The permission definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Permission {
    #[doc = "List of role assignments that specify the permissions to access the application."]
    pub roles: Vec<RoleAssignment>,
}
impl Permission {
    pub fn new(roles: Vec<RoleAssignment>) -> Self {
        Self { roles }
    }
}
#[doc = "Configuration specifying options for a pie chart tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PieChartConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[serde(flatten)]
    pub group_tile_configuration: GroupTileConfiguration,
    #[serde(flatten)]
    pub tile_capability_configuration: TileCapabilityConfiguration,
    #[doc = "Configuration specifying the time range and resolution of data to return for a tile."]
    #[serde(rename = "queryRange")]
    pub query_range: TimeQueryRangeConfiguration,
    #[doc = "Configuration specifying formatting options for a chart tile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<ChartFormatConfiguration>,
}
impl PieChartConfiguration {
    pub fn new(
        tile_configuration: TileConfiguration,
        group_tile_configuration: GroupTileConfiguration,
        query_range: TimeQueryRangeConfiguration,
    ) -> Self {
        Self {
            tile_configuration,
            group_tile_configuration,
            tile_capability_configuration: TileCapabilityConfiguration::default(),
            query_range,
            format: None,
        }
    }
}
#[doc = "The property job data definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyJobData {
    #[serde(flatten)]
    pub job_data: JobData,
    #[serde(flatten)]
    pub capability_job_data: CapabilityJobData,
}
impl PropertyJobData {
    pub fn new(job_data: JobData, capability_job_data: CapabilityJobData) -> Self {
        Self {
            job_data,
            capability_job_data,
        }
    }
}
#[doc = "Configuration specifying options for a property tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyTileConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[serde(flatten)]
    pub group_tile_configuration: GroupTileConfiguration,
    #[serde(flatten)]
    pub tile_capability_configuration: TileCapabilityConfiguration,
    #[doc = "Configuration specifying formatting options for a text based tile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<TextFormatConfiguration>,
}
impl PropertyTileConfiguration {
    pub fn new(tile_configuration: TileConfiguration, group_tile_configuration: GroupTileConfiguration) -> Self {
        Self {
            tile_configuration,
            group_tile_configuration,
            tile_capability_configuration: TileCapabilityConfiguration::default(),
            format: None,
        }
    }
}
#[doc = "Configuration specifying how much data to return for a tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryRangeConfiguration {
    #[doc = "The type of time range - 'count' or 'time'."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl QueryRangeConfiguration {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "The query request payload definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryRequest {
    #[doc = "Query to be executed."]
    pub query: String,
}
impl QueryRequest {
    pub fn new(query: String) -> Self {
        Self { query }
    }
}
#[doc = "The query response payload definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResponse {
    pub results: Vec<serde_json::Value>,
}
impl QueryResponse {
    pub fn new(results: Vec<serde_json::Value>) -> Self {
        Self { results }
    }
}
#[doc = "The role definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Role {
    #[doc = "Unique ID of the role."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the role."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl Role {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The role assignment definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignment {
    #[doc = "ID of the role for this role assignment."]
    pub role: String,
    #[doc = "ID of the organization for this role assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
}
impl RoleAssignment {
    pub fn new(role: String) -> Self {
        Self { role, organization: None }
    }
}
#[doc = "The paged results of roles."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleCollection {
    #[doc = "The collection of roles."]
    pub value: Vec<Role>,
    #[doc = "URL to get the next page of roles."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleCollection {
    pub fn new(value: Vec<Role>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The scheduled job definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledJob {
    #[doc = "ETag used to prevent conflict in scheduled job updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Unique ID of the scheduled job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the scheduled job."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Detailed description of the scheduled job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The ID of the device group on which to execute the scheduled job."]
    pub group: String,
    #[doc = "The job batch definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch: Option<JobBatch>,
    #[doc = "The job cancellation threshold definition."]
    #[serde(rename = "cancellationThreshold", default, skip_serializing_if = "Option::is_none")]
    pub cancellation_threshold: Option<JobCancellationThreshold>,
    #[doc = "Data related to the operation being performed by this job. All entries must be of the same type."]
    pub data: Vec<JobData>,
    #[doc = "List of organizations of the job, only one organization is supported today, multiple organizations will be supported soon."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organizations: Vec<String>,
    #[doc = "The schedule definition of job."]
    pub schedule: JobSchedule,
    #[doc = "Whether the scheduled job is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Whether the scheduled job has completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
}
impl ScheduledJob {
    pub fn new(group: String, data: Vec<JobData>, schedule: JobSchedule) -> Self {
        Self {
            etag: None,
            id: None,
            display_name: None,
            description: None,
            group,
            batch: None,
            cancellation_threshold: None,
            data,
            organizations: Vec::new(),
            schedule,
            enabled: None,
            completed: None,
        }
    }
}
#[doc = "The paged results of scheduled job definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledJobCollection {
    #[doc = "The collection of scheduled jobs."]
    pub value: Vec<ScheduledJob>,
    #[doc = "URL to get the next page of scheduled jobs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScheduledJobCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScheduledJobCollection {
    pub fn new(value: Vec<ScheduledJob>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The service bus queue destination definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusQueueV1Destination {
    #[serde(flatten)]
    pub destination: Destination,
    #[doc = "The authentication definition for service bus queue definition."]
    pub authorization: ServiceBusQueueV1DestinationAuth,
}
impl ServiceBusQueueV1Destination {
    pub fn new(destination: Destination, authorization: ServiceBusQueueV1DestinationAuth) -> Self {
        Self {
            destination,
            authorization,
        }
    }
}
#[doc = "The authentication definition for service bus queue definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusQueueV1DestinationAuth {
    #[doc = "The kind of authentication to use."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl ServiceBusQueueV1DestinationAuth {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "The authentication definition with connection string for service bus queue definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusQueueV1DestinationConnectionStringAuth {
    #[serde(flatten)]
    pub service_bus_queue_v1_destination_auth: ServiceBusQueueV1DestinationAuth,
    #[doc = "The connection string for accessing the Service Bus namespace, including the `EntityPath` of the queue."]
    #[serde(rename = "connectionString")]
    pub connection_string: String,
}
impl ServiceBusQueueV1DestinationConnectionStringAuth {
    pub fn new(service_bus_queue_v1_destination_auth: ServiceBusQueueV1DestinationAuth, connection_string: String) -> Self {
        Self {
            service_bus_queue_v1_destination_auth,
            connection_string,
        }
    }
}
#[doc = "The authentication definition with system assigned managed identity for service bus queue definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusQueueV1DestinationSystemAssignedManagedIdentityAuth {
    #[serde(flatten)]
    pub service_bus_queue_v1_destination_auth: ServiceBusQueueV1DestinationAuth,
    #[doc = "The host name of the Service Bus namespace."]
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[doc = "The Service Bus queue name."]
    #[serde(rename = "queueName")]
    pub queue_name: String,
}
impl ServiceBusQueueV1DestinationSystemAssignedManagedIdentityAuth {
    pub fn new(service_bus_queue_v1_destination_auth: ServiceBusQueueV1DestinationAuth, host_name: String, queue_name: String) -> Self {
        Self {
            service_bus_queue_v1_destination_auth,
            host_name,
            queue_name,
        }
    }
}
#[doc = "The service bus topic destination definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusTopicV1Destination {
    #[serde(flatten)]
    pub destination: Destination,
    #[doc = "The authentication definition for service bus topic destination."]
    pub authorization: ServiceBusTopicV1DestinationAuth,
}
impl ServiceBusTopicV1Destination {
    pub fn new(destination: Destination, authorization: ServiceBusTopicV1DestinationAuth) -> Self {
        Self {
            destination,
            authorization,
        }
    }
}
#[doc = "The authentication definition for service bus topic destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusTopicV1DestinationAuth {
    #[doc = "The kind of authentication to use."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl ServiceBusTopicV1DestinationAuth {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "The authentication definition with connection string for service bus topic destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusTopicV1DestinationConnectionStringAuth {
    #[serde(flatten)]
    pub service_bus_topic_v1_destination_auth: ServiceBusTopicV1DestinationAuth,
    #[doc = "The connection string for accessing the Service Bus namespace, including the `EntityPath` of the topic."]
    #[serde(rename = "connectionString")]
    pub connection_string: String,
}
impl ServiceBusTopicV1DestinationConnectionStringAuth {
    pub fn new(service_bus_topic_v1_destination_auth: ServiceBusTopicV1DestinationAuth, connection_string: String) -> Self {
        Self {
            service_bus_topic_v1_destination_auth,
            connection_string,
        }
    }
}
#[doc = "The authentication definition with system assigned managed identity for service bus topic destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBusTopicV1DestinationSystemAssignedManagedIdentityAuth {
    #[serde(flatten)]
    pub service_bus_topic_v1_destination_auth: ServiceBusTopicV1DestinationAuth,
    #[doc = "The host name of the Service Bus namespace."]
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[doc = "The Service Bus topic name."]
    #[serde(rename = "topicName")]
    pub topic_name: String,
}
impl ServiceBusTopicV1DestinationSystemAssignedManagedIdentityAuth {
    pub fn new(service_bus_topic_v1_destination_auth: ServiceBusTopicV1DestinationAuth, host_name: String, topic_name: String) -> Self {
        Self {
            service_bus_topic_v1_destination_auth,
            host_name,
            topic_name,
        }
    }
}
#[doc = "The service principal user definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalUser {
    #[serde(flatten)]
    pub user: User,
    #[doc = "The AAD tenant ID of the service principal."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[doc = "The AAD object ID of the service principal."]
    #[serde(rename = "objectId")]
    pub object_id: String,
}
impl ServicePrincipalUser {
    pub fn new(user: User, tenant_id: String, object_id: String) -> Self {
        Self {
            user,
            tenant_id,
            object_id,
        }
    }
}
#[doc = "The X509 definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SigningX509 {
    #[doc = "The X509 certificates definition."]
    #[serde(rename = "signingCertificates", default, skip_serializing_if = "Option::is_none")]
    pub signing_certificates: Option<SigningX509Certificates>,
}
impl SigningX509 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The X509 certificate definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SigningX509Certificate {
    #[doc = "Whether the certificate has been verified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    #[doc = "The string representation of this certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[doc = "The X509 certificate info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<X509CertificateInfo>,
    #[doc = "ETag used to prevent conflict across multiple updates"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl SigningX509Certificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The X509 certificates definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SigningX509Certificates {
    #[doc = "The X509 certificate definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<SigningX509Certificate>,
    #[doc = "The X509 certificate definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<SigningX509Certificate>,
}
impl SigningX509Certificates {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration specifying options for a state chart tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateChartConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[serde(flatten)]
    pub group_tile_configuration: GroupTileConfiguration,
    #[serde(flatten)]
    pub tile_capability_configuration: TileCapabilityConfiguration,
    #[doc = "Configuration specifying the time range and resolution of data to return for a tile."]
    #[serde(rename = "queryRange")]
    pub query_range: TimeQueryRangeConfiguration,
}
impl StateChartConfiguration {
    pub fn new(
        tile_configuration: TileConfiguration,
        group_tile_configuration: GroupTileConfiguration,
        query_range: TimeQueryRangeConfiguration,
    ) -> Self {
        Self {
            tile_configuration,
            group_tile_configuration,
            tile_capability_configuration: TileCapabilityConfiguration::default(),
            query_range,
        }
    }
}
#[doc = "Configuration specifying options for a state history chart tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateHistoryChartConfiguration {
    #[serde(flatten)]
    pub tile_configuration: TileConfiguration,
    #[serde(flatten)]
    pub group_tile_configuration: GroupTileConfiguration,
    #[serde(flatten)]
    pub tile_capability_configuration: TileCapabilityConfiguration,
    #[doc = "Configuration specifying the time range and resolution of data to return for a tile."]
    #[serde(rename = "queryRange")]
    pub query_range: TimeQueryRangeConfiguration,
    #[doc = "Configuration specifying formatting options for a text based tile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<TextFormatConfiguration>,
}
impl StateHistoryChartConfiguration {
    pub fn new(
        tile_configuration: TileConfiguration,
        group_tile_configuration: GroupTileConfiguration,
        query_range: TimeQueryRangeConfiguration,
    ) -> Self {
        Self {
            tile_configuration,
            group_tile_configuration,
            tile_capability_configuration: TileCapabilityConfiguration::default(),
            query_range,
            format: None,
        }
    }
}
#[doc = "The symmetric key definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SymmetricKey {
    #[doc = "The primary key for this credential."]
    #[serde(rename = "primaryKey")]
    pub primary_key: String,
    #[doc = "The secondary key for this credential."]
    #[serde(rename = "secondaryKey")]
    pub secondary_key: String,
}
impl SymmetricKey {
    pub fn new(primary_key: String, secondary_key: String) -> Self {
        Self {
            primary_key,
            secondary_key,
        }
    }
}
#[doc = "The symmetric key attestation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SymmetricKeyAttestation {
    #[serde(flatten)]
    pub attestation: Attestation,
    #[doc = "The symmetric key definition."]
    #[serde(rename = "symmetricKey")]
    pub symmetric_key: SymmetricKey,
}
impl SymmetricKeyAttestation {
    pub fn new(attestation: Attestation, symmetric_key: SymmetricKey) -> Self {
        Self {
            attestation,
            symmetric_key,
        }
    }
}
#[doc = "Configuration specifying formatting options for a text based tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TextFormatConfiguration {
    #[doc = "Whether to abbreviate the value"]
    #[serde(rename = "abbreviateValue", default, skip_serializing_if = "Option::is_none")]
    pub abbreviate_value: Option<bool>,
    #[doc = "The number of decimal places being displayed"]
    #[serde(rename = "decimalPlaces", default, skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<i64>,
    #[doc = "The font size of the text being displayed"]
    #[serde(rename = "textSize", default, skip_serializing_if = "Option::is_none")]
    pub text_size: Option<f64>,
    #[doc = "The unit of size for the text in the tile"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<TileTextSizeUnit>,
    #[doc = "Whether to wrap the text being displayed"]
    #[serde(rename = "wordWrap", default, skip_serializing_if = "Option::is_none")]
    pub word_wrap: Option<bool>,
}
impl TextFormatConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration specifying tile object, including the layout, display name and configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    #[doc = "Display name of the tile."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Configuration specifying information about a tile."]
    pub configuration: TileConfiguration,
    #[doc = "Height of the tile"]
    pub height: f64,
    #[doc = "Width of the tile"]
    pub width: f64,
    #[doc = "Horizontal position of the tile"]
    pub x: f64,
    #[doc = "Vertical position of the tile"]
    pub y: f64,
}
impl Tile {
    pub fn new(display_name: String, configuration: TileConfiguration, height: f64, width: f64, x: f64, y: f64) -> Self {
        Self {
            display_name,
            configuration,
            height,
            width,
            x,
            y,
        }
    }
}
#[doc = "Specifies the capability to be displayed in the tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TileCapability {
    #[doc = "The path of the capability associated with data to be rendered in the tile."]
    pub capability: String,
    #[doc = "The type of aggregation to be applied on capability data."]
    #[serde(rename = "aggregateFunction")]
    pub aggregate_function: CapabilityAggregateFunctionType,
}
impl TileCapability {
    pub fn new(capability: String, aggregate_function: CapabilityAggregateFunctionType) -> Self {
        Self {
            capability,
            aggregate_function,
        }
    }
}
#[doc = "Configuration specifying an array of capabilities to be displayed in the tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TileCapabilityConfiguration {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<TileCapability>,
}
impl TileCapabilityConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration specifying information about a tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TileConfiguration {
    #[doc = "The type of widget the tile renders"]
    #[serde(rename = "type")]
    pub type_: String,
}
impl TileConfiguration {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "The unit of size for the text in the tile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TileTextSizeUnit {
    #[serde(rename = "pt")]
    Pt,
}
impl Default for TileTextSizeUnit {
    fn default() -> Self {
        Self::Pt
    }
}
#[doc = "Configuration specifying the time range and resolution of data to return for a tile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeQueryRangeConfiguration {
    #[serde(flatten)]
    pub query_range_configuration: QueryRangeConfiguration,
    #[doc = "The duration of time to look back when querying data."]
    pub duration: TimeRangeDuration,
    #[doc = "The resolution to aggregate data over for each data point."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<TimeRangeResolution>,
}
impl TimeQueryRangeConfiguration {
    pub fn new(query_range_configuration: QueryRangeConfiguration, duration: TimeRangeDuration) -> Self {
        Self {
            query_range_configuration,
            duration,
            resolution: None,
        }
    }
}
#[doc = "The duration of time to look back when querying data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TimeRangeDuration {
    #[serde(rename = "PT30M")]
    Pt30m,
    #[serde(rename = "PT1H")]
    Pt1h,
    #[serde(rename = "PT12H")]
    Pt12h,
    #[serde(rename = "P1D")]
    P1d,
    #[serde(rename = "P1W")]
    P1w,
    #[serde(rename = "P30D")]
    P30d,
}
#[doc = "The resolution to aggregate data over for each data point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TimeRangeResolution {
    #[serde(rename = "PT1M")]
    Pt1m,
    #[serde(rename = "PT5M")]
    Pt5m,
    #[serde(rename = "PT10M")]
    Pt10m,
    #[serde(rename = "PT30M")]
    Pt30m,
    #[serde(rename = "PT1H")]
    Pt1h,
    #[serde(rename = "PT12H")]
    Pt12h,
    #[serde(rename = "P1D")]
    P1d,
    #[serde(rename = "P1W")]
    P1w,
}
impl Default for TimeRangeResolution {
    fn default() -> Self {
        Self::Pt5m
    }
}
#[doc = "The trusted platform module definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tpm {
    #[doc = "The TPM endorsement key for this credential."]
    #[serde(rename = "endorsementKey")]
    pub endorsement_key: String,
}
impl Tpm {
    pub fn new(endorsement_key: String) -> Self {
        Self { endorsement_key }
    }
}
#[doc = "The TPM attestation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TpmAttestation {
    #[serde(flatten)]
    pub attestation: Attestation,
    #[doc = "The trusted platform module definition."]
    pub tpm: Tpm,
}
impl TpmAttestation {
    pub fn new(attestation: Attestation, tpm: Tpm) -> Self {
        Self { attestation, tpm }
    }
}
#[doc = "The user definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub permission: Permission,
    #[doc = "Unique ID of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Type of the user."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl User {
    pub fn new(permission: Permission, type_: String) -> Self {
        Self {
            permission,
            id: None,
            type_,
        }
    }
}
#[doc = "The paged results of users."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCollection {
    #[doc = "The collection of users."]
    pub value: Vec<User>,
    #[doc = "URL to get the next page of users."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UserCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UserCollection {
    pub fn new(value: Vec<User>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The webhook destination definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookV1Destination {
    #[serde(flatten)]
    pub destination: Destination,
    #[doc = "The URL to invoke when exporting data."]
    pub url: String,
    #[doc = "Additional query parameters that should be added to each request."]
    #[serde(rename = "queryCustomizations", default, skip_serializing_if = "Option::is_none")]
    pub query_customizations: Option<serde_json::Value>,
    #[doc = "Additional headers that should be added to each request."]
    #[serde(rename = "headerCustomizations", default, skip_serializing_if = "Option::is_none")]
    pub header_customizations: Option<serde_json::Value>,
    #[doc = "The authentication definition for webhook destination."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<WebhookV1DestinationAuth>,
}
impl WebhookV1Destination {
    pub fn new(destination: Destination, url: String) -> Self {
        Self {
            destination,
            url,
            query_customizations: None,
            header_customizations: None,
            authorization: None,
        }
    }
}
#[doc = "The authentication definition for webhook destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookV1DestinationAuth {
    #[doc = "The kind of authentication to use."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl WebhookV1DestinationAuth {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "The customization definition for webhook destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookV1DestinationCustomization {
    #[doc = "The value to use for this webhook customization."]
    pub value: String,
    #[doc = "Whether to consider the value to be a secret and hide it when retrieving the destination configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<bool>,
}
impl WebhookV1DestinationCustomization {
    pub fn new(value: String) -> Self {
        Self { value, secret: None }
    }
}
#[doc = "The authentication definition with header for webhook destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookV1DestinationHeaderAuth {
    #[serde(flatten)]
    pub webhook_v1_destination_auth: WebhookV1DestinationAuth,
    #[doc = "Value to use for the Authorization header when making requests."]
    pub value: String,
}
impl WebhookV1DestinationHeaderAuth {
    pub fn new(webhook_v1_destination_auth: WebhookV1DestinationAuth, value: String) -> Self {
        Self {
            webhook_v1_destination_auth,
            value,
        }
    }
}
#[doc = "The authentication definition with OAuth for webhook destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookV1DestinationOAuthAuth {
    #[serde(flatten)]
    pub webhook_v1_destination_auth: WebhookV1DestinationAuth,
    #[doc = "URL where an access token can be retrieved."]
    #[serde(rename = "tokenUrl")]
    pub token_url: String,
    #[doc = "OAuth2 client ID used when retrieving the token."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "OAuth2 client secret used to retrieve the token."]
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    #[doc = "OAuth2 audience."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[doc = "OAuth2 scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Content-Type for the token request."]
    #[serde(rename = "requestType", default, skip_serializing_if = "Option::is_none")]
    pub request_type: Option<webhook_v1_destination_o_auth_auth::RequestType>,
}
impl WebhookV1DestinationOAuthAuth {
    pub fn new(webhook_v1_destination_auth: WebhookV1DestinationAuth, token_url: String, client_id: String, client_secret: String) -> Self {
        Self {
            webhook_v1_destination_auth,
            token_url,
            client_id,
            client_secret,
            audience: None,
            scope: None,
            request_type: None,
        }
    }
}
pub mod webhook_v1_destination_o_auth_auth {
    use super::*;
    #[doc = "Content-Type for the token request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RequestType {
        #[serde(rename = "auto")]
        Auto,
        #[serde(rename = "json")]
        Json,
        #[serde(rename = "urlencoded")]
        Urlencoded,
    }
}
#[doc = "The X509 definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct X509 {
    #[doc = "The X509 certificates definition."]
    #[serde(rename = "clientCertificates", default, skip_serializing_if = "Option::is_none")]
    pub client_certificates: Option<X509Certificates>,
}
impl X509 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The X509 attestation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X509Attestation {
    #[serde(flatten)]
    pub attestation: Attestation,
    #[doc = "The X509 definition."]
    pub x509: X509,
}
impl X509Attestation {
    pub fn new(attestation: Attestation, x509: X509) -> Self {
        Self { attestation, x509 }
    }
}
#[doc = "The X509 certificate definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct X509Certificate {
    #[doc = "The string representation of this certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[doc = "The X509 certificate info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<X509CertificateInfo>,
}
impl X509Certificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The X509 certificate info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X509CertificateInfo {
    #[doc = "The SHA-1 hash value of the certificate."]
    #[serde(rename = "sha1Thumbprint")]
    pub sha1_thumbprint: String,
}
impl X509CertificateInfo {
    pub fn new(sha1_thumbprint: String) -> Self {
        Self { sha1_thumbprint }
    }
}
#[doc = "The X509 certificates definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X509Certificates {
    #[doc = "The X509 certificate definition."]
    pub primary: X509Certificate,
    #[doc = "The X509 certificate definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary: Option<X509Certificate>,
}
impl X509Certificates {
    pub fn new(primary: X509Certificate) -> Self {
        Self { primary, secondary: None }
    }
}
#[doc = "The X509 verification certificate definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X509VerificationCertificate {
    #[doc = "The string representation of this certificate."]
    pub certificate: String,
}
impl X509VerificationCertificate {
    pub fn new(certificate: String) -> Self {
        Self { certificate }
    }
}
#[doc = "The X509 certificate verification code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct X509VerificationCode {
    #[doc = "The X509 certificate verification code."]
    #[serde(rename = "verificationCode", default, skip_serializing_if = "Option::is_none")]
    pub verification_code: Option<String>,
}
impl X509VerificationCode {
    pub fn new() -> Self {
        Self::default()
    }
}
