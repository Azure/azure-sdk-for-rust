#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Credential details of the account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountCredentialDetails {
    #[doc = "Name of the account."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "Type of the account."]
    #[serde(rename = "dataAccountType", default, skip_serializing_if = "Option::is_none")]
    pub data_account_type: Option<account_credential_details::DataAccountType>,
    #[doc = "Connection string of the account endpoint to use the account as a storage endpoint on the device."]
    #[serde(rename = "accountConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub account_connection_string: Option<String>,
    #[doc = "Per share level unencrypted access credentials."]
    #[serde(rename = "shareCredentialDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub share_credential_details: Vec<ShareCredentialDetails>,
}
impl AccountCredentialDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod account_credential_details {
    use super::*;
    #[doc = "Type of the account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataAccountType {
        StorageAccount,
        ManagedDisk,
    }
}
#[doc = "Additional error info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdditionalErrorInfo {
    #[doc = "Additional error type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Additional error info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl AdditionalErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Output of the address validation api."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddressValidationOutput {
    #[doc = "The address validation output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AddressValidationProperties>,
}
impl AddressValidationOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The address validation output."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressValidationProperties {
    #[serde(flatten)]
    pub validation_input_response: ValidationInputResponse,
    #[doc = "The address validation status."]
    #[serde(rename = "validationStatus", default, skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<address_validation_properties::ValidationStatus>,
    #[doc = "List of alternate addresses."]
    #[serde(rename = "alternateAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub alternate_addresses: Vec<ShippingAddress>,
}
impl AddressValidationProperties {
    pub fn new(validation_input_response: ValidationInputResponse) -> Self {
        Self {
            validation_input_response,
            validation_status: None,
            alternate_addresses: Vec::new(),
        }
    }
}
pub mod address_validation_properties {
    use super::*;
    #[doc = "The address validation status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ValidationStatus {
        Valid,
        Invalid,
        Ambiguous,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiError {
    pub error: ErrorDetail,
}
impl azure_core::Continuable for ApiError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ApiError {
    pub fn new(error: ErrorDetail) -> Self {
        Self { error }
    }
}
#[doc = "The Network Adapter configuration of a DataBox."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplianceNetworkConfiguration {
    #[doc = "Name of the network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Mac Address."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
}
impl ApplianceNetworkConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for all objects under resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmBaseObject {
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
impl ArmBaseObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The filters for showing the available skus."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableSkuRequest {
    #[doc = "Type of the transfer."]
    #[serde(rename = "transferType")]
    pub transfer_type: available_sku_request::TransferType,
    #[doc = "ISO country code. Country for hardware shipment. For codes check: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2#Officially_assigned_code_elements"]
    pub country: String,
    #[doc = "Location for data transfer. For locations check: https://management.azure.com/subscriptions/SUBSCRIPTIONID/locations?api-version=2018-01-01"]
    pub location: String,
    #[doc = "Sku Names to filter for available skus"]
    #[serde(rename = "skuNames", default, skip_serializing_if = "Vec::is_empty")]
    pub sku_names: Vec<String>,
}
impl AvailableSkuRequest {
    pub fn new(transfer_type: available_sku_request::TransferType, country: String, location: String) -> Self {
        Self {
            transfer_type,
            country,
            location,
            sku_names: Vec::new(),
        }
    }
}
pub mod available_sku_request {
    use super::*;
    #[doc = "Type of the transfer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TransferType {
        ImportToAzure,
        ExportFromAzure,
    }
}
#[doc = "The available skus operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableSkusResult {
    #[doc = "List of available skus."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuInformation>,
    #[doc = "Link for the next set of skus."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvailableSkusResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AvailableSkusResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Filter details to transfer Azure files"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureFileFilterDetails {
    #[doc = "Prefix list of the Azure files to be transferred."]
    #[serde(rename = "filePrefixList", default, skip_serializing_if = "Vec::is_empty")]
    pub file_prefix_list: Vec<String>,
    #[doc = "List of full path of the files to be transferred."]
    #[serde(rename = "filePathList", default, skip_serializing_if = "Vec::is_empty")]
    pub file_path_list: Vec<String>,
    #[doc = "List of file shares to be transferred."]
    #[serde(rename = "fileShareList", default, skip_serializing_if = "Vec::is_empty")]
    pub file_share_list: Vec<String>,
}
impl AzureFileFilterDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Filter details to transfer Azure Blobs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobFilterDetails {
    #[doc = "Prefix list of the Azure blobs to be transferred."]
    #[serde(rename = "blobPrefixList", default, skip_serializing_if = "Vec::is_empty")]
    pub blob_prefix_list: Vec<String>,
    #[doc = "List of full path of the blobs to be transferred."]
    #[serde(rename = "blobPathList", default, skip_serializing_if = "Vec::is_empty")]
    pub blob_path_list: Vec<String>,
    #[doc = "List of blob containers to be transferred."]
    #[serde(rename = "containerList", default, skip_serializing_if = "Vec::is_empty")]
    pub container_list: Vec<String>,
}
impl BlobFilterDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reason for cancellation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CancellationReason {
    #[doc = "Reason for cancellation."]
    pub reason: String,
}
impl CancellationReason {
    pub fn new(reason: String) -> Self {
        Self { reason }
    }
}
#[doc = "Cloud error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Cloud error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Cloud error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Cloud error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Cloud error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudError>,
    #[doc = "Cloud error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<AdditionalErrorInfo>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contact Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactDetails {
    #[doc = "Contact name of the person."]
    #[serde(rename = "contactName")]
    pub contact_name: String,
    #[doc = "Phone number of the contact person."]
    pub phone: String,
    #[doc = "Phone extension number of the contact person."]
    #[serde(rename = "phoneExtension", default, skip_serializing_if = "Option::is_none")]
    pub phone_extension: Option<String>,
    #[doc = "Mobile number of the contact person."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[doc = "List of Email-ids to be notified about job progress."]
    #[serde(rename = "emailList")]
    pub email_list: Vec<String>,
    #[doc = "Notification preference for a job stage."]
    #[serde(rename = "notificationPreference", default, skip_serializing_if = "Vec::is_empty")]
    pub notification_preference: Vec<NotificationPreference>,
}
impl ContactDetails {
    pub fn new(contact_name: String, phone: String, email_list: Vec<String>) -> Self {
        Self {
            contact_name,
            phone,
            phone_extension: None,
            mobile: None,
            email_list,
            notification_preference: Vec::new(),
        }
    }
}
#[doc = "Details for log generated during copy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CopyLogDetails {
    #[doc = "Indicates the type of job details."]
    #[serde(rename = "copyLogDetailsType")]
    pub copy_log_details_type: copy_log_details::CopyLogDetailsType,
}
impl CopyLogDetails {
    pub fn new(copy_log_details_type: copy_log_details::CopyLogDetailsType) -> Self {
        Self { copy_log_details_type }
    }
}
pub mod copy_log_details {
    use super::*;
    #[doc = "Indicates the type of job details."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CopyLogDetailsType {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
}
#[doc = "Copy progress."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CopyProgress {
    #[doc = "Name of the storage account. This will be empty for data account types other than storage account."]
    #[serde(rename = "storageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_name: Option<String>,
    #[doc = "Transfer type of data"]
    #[serde(rename = "transferType", default, skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<copy_progress::TransferType>,
    #[doc = "Data Account Type."]
    #[serde(rename = "dataAccountType", default, skip_serializing_if = "Option::is_none")]
    pub data_account_type: Option<copy_progress::DataAccountType>,
    #[doc = "Id of the account where the data needs to be uploaded."]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "To indicate bytes transferred."]
    #[serde(rename = "bytesProcessed", default, skip_serializing_if = "Option::is_none")]
    pub bytes_processed: Option<i64>,
    #[doc = "Total amount of data to be processed by the job."]
    #[serde(rename = "totalBytesToProcess", default, skip_serializing_if = "Option::is_none")]
    pub total_bytes_to_process: Option<i64>,
    #[doc = "Number of files processed"]
    #[serde(rename = "filesProcessed", default, skip_serializing_if = "Option::is_none")]
    pub files_processed: Option<i64>,
    #[doc = "Total files to process"]
    #[serde(rename = "totalFilesToProcess", default, skip_serializing_if = "Option::is_none")]
    pub total_files_to_process: Option<i64>,
    #[doc = "Number of files not adhering to azure naming conventions which were processed by automatic renaming"]
    #[serde(rename = "invalidFilesProcessed", default, skip_serializing_if = "Option::is_none")]
    pub invalid_files_processed: Option<i64>,
    #[doc = "Total amount of data not adhering to azure naming conventions which were processed by automatic renaming"]
    #[serde(rename = "invalidFileBytesUploaded", default, skip_serializing_if = "Option::is_none")]
    pub invalid_file_bytes_uploaded: Option<i64>,
    #[doc = "Number of folders not adhering to azure naming conventions which were processed by automatic renaming"]
    #[serde(rename = "renamedContainerCount", default, skip_serializing_if = "Option::is_none")]
    pub renamed_container_count: Option<i64>,
    #[doc = "Number of files which could not be copied"]
    #[serde(rename = "filesErroredOut", default, skip_serializing_if = "Option::is_none")]
    pub files_errored_out: Option<i64>,
    #[doc = "To indicate directories errored out in the job."]
    #[serde(rename = "directoriesErroredOut", default, skip_serializing_if = "Option::is_none")]
    pub directories_errored_out: Option<i64>,
    #[doc = "To indicate directories renamed"]
    #[serde(rename = "invalidDirectoriesProcessed", default, skip_serializing_if = "Option::is_none")]
    pub invalid_directories_processed: Option<i64>,
    #[doc = "To indicate if enumeration of data is in progress. \r\nUntil this is true, the TotalBytesToProcess may not be valid."]
    #[serde(rename = "isEnumerationInProgress", default, skip_serializing_if = "Option::is_none")]
    pub is_enumeration_in_progress: Option<bool>,
}
impl CopyProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod copy_progress {
    use super::*;
    #[doc = "Transfer type of data"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TransferType {
        ImportToAzure,
        ExportFromAzure,
    }
    #[doc = "Data Account Type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataAccountType {
        StorageAccount,
        ManagedDisk,
    }
}
#[doc = "It does all pre-job creation validations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateJobValidations {
    #[serde(flatten)]
    pub validation_request: ValidationRequest,
}
impl CreateJobValidations {
    pub fn new(validation_request: ValidationRequest) -> Self {
        Self { validation_request }
    }
}
#[doc = "Request to validate create order limit for current subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrderLimitForSubscriptionValidationRequest {
    #[serde(flatten)]
    pub validation_input_request: ValidationInputRequest,
    #[doc = "Device type to be used for the job."]
    #[serde(rename = "deviceType")]
    pub device_type: create_order_limit_for_subscription_validation_request::DeviceType,
}
impl CreateOrderLimitForSubscriptionValidationRequest {
    pub fn new(
        validation_input_request: ValidationInputRequest,
        device_type: create_order_limit_for_subscription_validation_request::DeviceType,
    ) -> Self {
        Self {
            validation_input_request,
            device_type,
        }
    }
}
pub mod create_order_limit_for_subscription_validation_request {
    use super::*;
    #[doc = "Device type to be used for the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeviceType {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
}
#[doc = "Properties of create order limit for subscription validation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateOrderLimitForSubscriptionValidationResponseProperties {
    #[serde(flatten)]
    pub validation_input_response: ValidationInputResponse,
    #[doc = "Create order limit validation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<create_order_limit_for_subscription_validation_response_properties::Status>,
}
impl CreateOrderLimitForSubscriptionValidationResponseProperties {
    pub fn new(validation_input_response: ValidationInputResponse) -> Self {
        Self {
            validation_input_response,
            status: None,
        }
    }
}
pub mod create_order_limit_for_subscription_validation_response_properties {
    use super::*;
    #[doc = "Create order limit validation status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Valid,
        Invalid,
        Skipped,
    }
}
#[doc = "Account details of the data to be transferred"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataAccountDetails {
    #[doc = "Account Type of the data to be transferred."]
    #[serde(rename = "dataAccountType")]
    pub data_account_type: data_account_details::DataAccountType,
    #[doc = "Password for all the shares to be created on the device. Should not be passed for TransferType:ExportFromAzure jobs. If this is not passed, the service will generate password itself. This will not be returned in Get Call. Password Requirements :  Password must be minimum of 12 and maximum of 64 characters. Password must have at least one uppercase alphabet, one number and one special character. Password cannot have the following characters : IilLoO0 Password can have only alphabets, numbers and these characters : @#\\-$%^!+=;:_()]+"]
    #[serde(rename = "sharePassword", default, skip_serializing_if = "Option::is_none")]
    pub share_password: Option<String>,
}
impl DataAccountDetails {
    pub fn new(data_account_type: data_account_details::DataAccountType) -> Self {
        Self {
            data_account_type,
            share_password: None,
        }
    }
}
pub mod data_account_details {
    use super::*;
    #[doc = "Account Type of the data to be transferred."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataAccountType {
        StorageAccount,
        ManagedDisk,
    }
    impl Default for DataAccountType {
        fn default() -> Self {
            Self::StorageAccount
        }
    }
}
#[doc = "Copy log details for a storage account of a DataBox job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxAccountCopyLogDetails {
    #[serde(flatten)]
    pub copy_log_details: CopyLogDetails,
    #[doc = "Account name."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "Link for copy logs."]
    #[serde(rename = "copyLogLink", default, skip_serializing_if = "Option::is_none")]
    pub copy_log_link: Option<String>,
    #[doc = "Link for copy verbose logs. This will be set only when LogCollectionLevel is set to Verbose."]
    #[serde(rename = "copyVerboseLogLink", default, skip_serializing_if = "Option::is_none")]
    pub copy_verbose_log_link: Option<String>,
}
impl DataBoxAccountCopyLogDetails {
    pub fn new(copy_log_details: CopyLogDetails) -> Self {
        Self {
            copy_log_details,
            account_name: None,
            copy_log_link: None,
            copy_verbose_log_link: None,
        }
    }
}
#[doc = "Copy Log Details for a disk"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxDiskCopyLogDetails {
    #[serde(flatten)]
    pub copy_log_details: CopyLogDetails,
    #[doc = "Disk Serial Number."]
    #[serde(rename = "diskSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub disk_serial_number: Option<String>,
    #[doc = "Link for copy error logs."]
    #[serde(rename = "errorLogLink", default, skip_serializing_if = "Option::is_none")]
    pub error_log_link: Option<String>,
    #[doc = "Link for copy verbose logs."]
    #[serde(rename = "verboseLogLink", default, skip_serializing_if = "Option::is_none")]
    pub verbose_log_link: Option<String>,
}
impl DataBoxDiskCopyLogDetails {
    pub fn new(copy_log_details: CopyLogDetails) -> Self {
        Self {
            copy_log_details,
            disk_serial_number: None,
            error_log_link: None,
            verbose_log_link: None,
        }
    }
}
#[doc = "DataBox Disk Copy Progress"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxDiskCopyProgress {
    #[doc = "The serial number of the disk"]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Bytes copied during the copy of disk."]
    #[serde(rename = "bytesCopied", default, skip_serializing_if = "Option::is_none")]
    pub bytes_copied: Option<i64>,
    #[doc = "Indicates the percentage completed for the copy of the disk."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[doc = "The Status of the copy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<data_box_disk_copy_progress::Status>,
}
impl DataBoxDiskCopyProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_box_disk_copy_progress {
    use super::*;
    #[doc = "The Status of the copy"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        NotStarted,
        InProgress,
        Completed,
        CompletedWithErrors,
        Failed,
        NotReturned,
        HardwareError,
        DeviceFormatted,
        DeviceMetadataModified,
        StorageAccountNotAccessible,
        UnsupportedData,
    }
}
#[doc = "DataBox Disk Job Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxDiskJobDetails {
    #[serde(flatten)]
    pub job_details: JobDetails,
    #[doc = "User preference on what size disks are needed for the job. The map is from the disk size in TB to the count. Eg. {2,5} means 5 disks of 2 TB size. Key is string but will be checked against an int."]
    #[serde(rename = "preferredDisks", default, skip_serializing_if = "Option::is_none")]
    pub preferred_disks: Option<serde_json::Value>,
    #[doc = "Copy progress per disk."]
    #[serde(rename = "copyProgress", default, skip_serializing_if = "Vec::is_empty")]
    pub copy_progress: Vec<DataBoxDiskCopyProgress>,
    #[doc = "Contains the map of disk serial number to the disk size being used for the job. Is returned only after the disks are shipped to the customer."]
    #[serde(rename = "disksAndSizeDetails", default, skip_serializing_if = "Option::is_none")]
    pub disks_and_size_details: Option<serde_json::Value>,
    #[doc = "User entered passkey for DataBox Disk job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passkey: Option<String>,
}
impl DataBoxDiskJobDetails {
    pub fn new(job_details: JobDetails) -> Self {
        Self {
            job_details,
            preferred_disks: None,
            copy_progress: Vec::new(),
            disks_and_size_details: None,
            passkey: None,
        }
    }
}
#[doc = "The secrets related to disk job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxDiskJobSecrets {
    #[serde(flatten)]
    pub job_secrets: JobSecrets,
    #[doc = "Contains the list of secrets object for that device."]
    #[serde(rename = "diskSecrets", default, skip_serializing_if = "Vec::is_empty")]
    pub disk_secrets: Vec<DiskSecret>,
    #[doc = "PassKey for the disk Job."]
    #[serde(rename = "passKey", default, skip_serializing_if = "Option::is_none")]
    pub pass_key: Option<String>,
    #[doc = "Whether passkey was provided by user."]
    #[serde(rename = "isPasskeyUserDefined", default, skip_serializing_if = "Option::is_none")]
    pub is_passkey_user_defined: Option<bool>,
}
impl DataBoxDiskJobSecrets {
    pub fn new(job_secrets: JobSecrets) -> Self {
        Self {
            job_secrets,
            disk_secrets: Vec::new(),
            pass_key: None,
            is_passkey_user_defined: None,
        }
    }
}
#[doc = "Copy log details for a storage account for Databox heavy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxHeavyAccountCopyLogDetails {
    #[serde(flatten)]
    pub copy_log_details: CopyLogDetails,
    #[doc = "Account name."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "Link for copy logs."]
    #[serde(rename = "copyLogLink", default, skip_serializing_if = "Vec::is_empty")]
    pub copy_log_link: Vec<String>,
    #[doc = "Link for copy verbose logs. This will be set only when the LogCollectionLevel is set to verbose."]
    #[serde(rename = "copyVerboseLogLink", default, skip_serializing_if = "Vec::is_empty")]
    pub copy_verbose_log_link: Vec<String>,
}
impl DataBoxHeavyAccountCopyLogDetails {
    pub fn new(copy_log_details: CopyLogDetails) -> Self {
        Self {
            copy_log_details,
            account_name: None,
            copy_log_link: Vec::new(),
            copy_verbose_log_link: Vec::new(),
        }
    }
}
#[doc = "Databox Heavy Device Job Details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxHeavyJobDetails {
    #[serde(flatten)]
    pub job_details: JobDetails,
    #[doc = "Copy progress per account."]
    #[serde(rename = "copyProgress", default, skip_serializing_if = "Vec::is_empty")]
    pub copy_progress: Vec<CopyProgress>,
    #[doc = "Set Device password for unlocking Databox Heavy. Should not be passed for TransferType:ExportFromAzure jobs. If this is not passed, the service will generate password itself. This will not be returned in Get Call. Password Requirements :  Password must be minimum of 12 and maximum of 64 characters. Password must have at least one uppercase alphabet, one number and one special character. Password cannot have the following characters : IilLoO0 Password can have only alphabets, numbers and these characters : @#\\-$%^!+=;:_()]+"]
    #[serde(rename = "devicePassword", default, skip_serializing_if = "Option::is_none")]
    pub device_password: Option<String>,
}
impl DataBoxHeavyJobDetails {
    pub fn new(job_details: JobDetails) -> Self {
        Self {
            job_details,
            copy_progress: Vec::new(),
            device_password: None,
        }
    }
}
#[doc = "The secrets related to a databox heavy job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxHeavyJobSecrets {
    #[serde(flatten)]
    pub job_secrets: JobSecrets,
    #[doc = "Contains the list of secret objects for a databox heavy job."]
    #[serde(rename = "cabinetPodSecrets", default, skip_serializing_if = "Vec::is_empty")]
    pub cabinet_pod_secrets: Vec<DataBoxHeavySecret>,
}
impl DataBoxHeavyJobSecrets {
    pub fn new(job_secrets: JobSecrets) -> Self {
        Self {
            job_secrets,
            cabinet_pod_secrets: Vec::new(),
        }
    }
}
#[doc = "The secrets related to a databox heavy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxHeavySecret {
    #[doc = "Serial number of the assigned device."]
    #[serde(rename = "deviceSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub device_serial_number: Option<String>,
    #[doc = "Password for out of the box experience on device."]
    #[serde(rename = "devicePassword", default, skip_serializing_if = "Option::is_none")]
    pub device_password: Option<String>,
    #[doc = "Network configuration of the appliance."]
    #[serde(rename = "networkConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub network_configurations: Vec<ApplianceNetworkConfiguration>,
    #[doc = "The base 64 encoded public key to authenticate with the device"]
    #[serde(rename = "encodedValidationCertPubKey", default, skip_serializing_if = "Option::is_none")]
    pub encoded_validation_cert_pub_key: Option<String>,
    #[doc = "Per account level access credentials."]
    #[serde(rename = "accountCredentialDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub account_credential_details: Vec<AccountCredentialDetails>,
}
impl DataBoxHeavySecret {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Databox Job Details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxJobDetails {
    #[serde(flatten)]
    pub job_details: JobDetails,
    #[doc = "Copy progress per storage account."]
    #[serde(rename = "copyProgress", default, skip_serializing_if = "Vec::is_empty")]
    pub copy_progress: Vec<CopyProgress>,
    #[doc = "Set Device password for unlocking Databox. Should not be passed for TransferType:ExportFromAzure jobs. If this is not passed, the service will generate password itself. This will not be returned in Get Call. Password Requirements :  Password must be minimum of 12 and maximum of 64 characters. Password must have at least one uppercase alphabet, one number and one special character. Password cannot have the following characters : IilLoO0 Password can have only alphabets, numbers and these characters : @#\\-$%^!+=;:_()]+"]
    #[serde(rename = "devicePassword", default, skip_serializing_if = "Option::is_none")]
    pub device_password: Option<String>,
}
impl DataBoxJobDetails {
    pub fn new(job_details: JobDetails) -> Self {
        Self {
            job_details,
            copy_progress: Vec::new(),
            device_password: None,
        }
    }
}
#[doc = "Request body to get the availability for scheduling data box orders orders."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxScheduleAvailabilityRequest {
    #[serde(flatten)]
    pub schedule_availability_request: ScheduleAvailabilityRequest,
}
impl DataBoxScheduleAvailabilityRequest {
    pub fn new(schedule_availability_request: ScheduleAvailabilityRequest) -> Self {
        Self {
            schedule_availability_request,
        }
    }
}
#[doc = "The secrets related to a DataBox."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxSecret {
    #[doc = "Serial number of the assigned device."]
    #[serde(rename = "deviceSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub device_serial_number: Option<String>,
    #[doc = "Password for out of the box experience on device."]
    #[serde(rename = "devicePassword", default, skip_serializing_if = "Option::is_none")]
    pub device_password: Option<String>,
    #[doc = "Network configuration of the appliance."]
    #[serde(rename = "networkConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub network_configurations: Vec<ApplianceNetworkConfiguration>,
    #[doc = "The base 64 encoded public key to authenticate with the device"]
    #[serde(rename = "encodedValidationCertPubKey", default, skip_serializing_if = "Option::is_none")]
    pub encoded_validation_cert_pub_key: Option<String>,
    #[doc = "Per account level access credentials."]
    #[serde(rename = "accountCredentialDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub account_credential_details: Vec<AccountCredentialDetails>,
}
impl DataBoxSecret {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the data to be used for exporting data from azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportDetails {
    #[doc = "Configuration for defining the transfer of data."]
    #[serde(rename = "transferConfiguration")]
    pub transfer_configuration: TransferConfiguration,
    #[doc = "Level of the logs to be collected."]
    #[serde(rename = "logCollectionLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_collection_level: Option<data_export_details::LogCollectionLevel>,
    #[doc = "Account details of the data to be transferred"]
    #[serde(rename = "accountDetails")]
    pub account_details: DataAccountDetails,
}
impl DataExportDetails {
    pub fn new(transfer_configuration: TransferConfiguration, account_details: DataAccountDetails) -> Self {
        Self {
            transfer_configuration,
            log_collection_level: None,
            account_details,
        }
    }
}
pub mod data_export_details {
    use super::*;
    #[doc = "Level of the logs to be collected."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LogCollectionLevel {
        Error,
        Verbose,
    }
    impl Default for LogCollectionLevel {
        fn default() -> Self {
            Self::Error
        }
    }
}
#[doc = "Details of the data to be used for importing data to azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataImportDetails {
    #[doc = "Account details of the data to be transferred"]
    #[serde(rename = "accountDetails")]
    pub account_details: DataAccountDetails,
}
impl DataImportDetails {
    pub fn new(account_details: DataAccountDetails) -> Self {
        Self { account_details }
    }
}
#[doc = "Map of data location to service location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLocationToServiceLocationMap {
    #[doc = "Location of the data."]
    #[serde(rename = "dataLocation", default, skip_serializing_if = "Option::is_none")]
    pub data_location: Option<String>,
    #[doc = "Location of the service."]
    #[serde(rename = "serviceLocation", default, skip_serializing_if = "Option::is_none")]
    pub service_location: Option<String>,
}
impl DataLocationToServiceLocationMap {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request to validate export and import data details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataTransferDetailsValidationRequest {
    #[serde(flatten)]
    pub validation_input_request: ValidationInputRequest,
    #[doc = "List of DataTransfer details to be used to export data from azure."]
    #[serde(rename = "dataExportDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub data_export_details: Vec<DataExportDetails>,
    #[doc = "List of DataTransfer details to be used to import data to azure."]
    #[serde(rename = "dataImportDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub data_import_details: Vec<DataImportDetails>,
    #[doc = "Device type."]
    #[serde(rename = "deviceType")]
    pub device_type: data_transfer_details_validation_request::DeviceType,
    #[doc = "Type of the transfer."]
    #[serde(rename = "transferType")]
    pub transfer_type: data_transfer_details_validation_request::TransferType,
}
impl DataTransferDetailsValidationRequest {
    pub fn new(
        validation_input_request: ValidationInputRequest,
        device_type: data_transfer_details_validation_request::DeviceType,
        transfer_type: data_transfer_details_validation_request::TransferType,
    ) -> Self {
        Self {
            validation_input_request,
            data_export_details: Vec::new(),
            data_import_details: Vec::new(),
            device_type,
            transfer_type,
        }
    }
}
pub mod data_transfer_details_validation_request {
    use super::*;
    #[doc = "Device type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeviceType {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
    #[doc = "Type of the transfer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TransferType {
        ImportToAzure,
        ExportFromAzure,
    }
}
#[doc = "Properties of data transfer details validation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataTransferDetailsValidationResponseProperties {
    #[serde(flatten)]
    pub validation_input_response: ValidationInputResponse,
    #[doc = "Data transfer details validation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<data_transfer_details_validation_response_properties::Status>,
}
impl DataTransferDetailsValidationResponseProperties {
    pub fn new(validation_input_response: ValidationInputResponse) -> Self {
        Self {
            validation_input_response,
            status: None,
        }
    }
}
pub mod data_transfer_details_validation_response_properties {
    use super::*;
    #[doc = "Data transfer details validation status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Valid,
        Invalid,
        Skipped,
    }
}
#[doc = "The secrets related to a databox job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataboxJobSecrets {
    #[serde(flatten)]
    pub job_secrets: JobSecrets,
    #[doc = "Contains the list of secret objects for a job."]
    #[serde(rename = "podSecrets", default, skip_serializing_if = "Vec::is_empty")]
    pub pod_secrets: Vec<DataBoxSecret>,
}
impl DataboxJobSecrets {
    pub fn new(job_secrets: JobSecrets) -> Self {
        Self {
            job_secrets,
            pod_secrets: Vec::new(),
        }
    }
}
#[doc = "Dc access security code"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DcAccessSecurityCode {
    #[doc = "Reverse Dc access security code."]
    #[serde(rename = "reverseDCAccessCode", default, skip_serializing_if = "Option::is_none")]
    pub reverse_dc_access_code: Option<String>,
    #[doc = "Forward Dc access security code."]
    #[serde(rename = "forwardDCAccessCode", default, skip_serializing_if = "Option::is_none")]
    pub forward_dc_access_code: Option<String>,
}
impl DcAccessSecurityCode {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Details {
    pub code: String,
    pub message: String,
}
impl Details {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[doc = "Request body to get the availability for scheduling disk orders."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskScheduleAvailabilityRequest {
    #[serde(flatten)]
    pub schedule_availability_request: ScheduleAvailabilityRequest,
    #[doc = "The expected size of the data, which needs to be transferred in this job, in terabytes."]
    #[serde(rename = "expectedDataSizeInTeraBytes")]
    pub expected_data_size_in_tera_bytes: i32,
}
impl DiskScheduleAvailabilityRequest {
    pub fn new(schedule_availability_request: ScheduleAvailabilityRequest, expected_data_size_in_tera_bytes: i32) -> Self {
        Self {
            schedule_availability_request,
            expected_data_size_in_tera_bytes,
        }
    }
}
#[doc = "Contains all the secrets of a Disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskSecret {
    #[doc = "Serial number of the assigned disk."]
    #[serde(rename = "diskSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub disk_serial_number: Option<String>,
    #[doc = "Bit Locker key of the disk which can be used to unlock the disk to copy data."]
    #[serde(rename = "bitLockerKey", default, skip_serializing_if = "Option::is_none")]
    pub bit_locker_key: Option<String>,
}
impl DiskSecret {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Preferences related to the Encryption."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionPreferences {
    #[doc = "Defines secondary layer of software-based encryption enablement."]
    #[serde(rename = "doubleEncryption", default, skip_serializing_if = "Option::is_none")]
    pub double_encryption: Option<encryption_preferences::DoubleEncryption>,
}
impl EncryptionPreferences {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption_preferences {
    use super::*;
    #[doc = "Defines secondary layer of software-based encryption enablement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DoubleEncryption {
        Enabled,
        Disabled,
    }
    impl Default for DoubleEncryption {
        fn default() -> Self {
            Self::Disabled
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<Details>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
            target: None,
        }
    }
}
#[doc = "Details of the filter files to be used for data transfer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterFileDetails {
    #[doc = "Type of the filter file."]
    #[serde(rename = "filterFileType")]
    pub filter_file_type: filter_file_details::FilterFileType,
    #[doc = "Path of the file that contains the details of all items to transfer."]
    #[serde(rename = "filterFilePath")]
    pub filter_file_path: String,
}
impl FilterFileDetails {
    pub fn new(filter_file_type: filter_file_details::FilterFileType, filter_file_path: String) -> Self {
        Self {
            filter_file_type,
            filter_file_path,
        }
    }
}
pub mod filter_file_details {
    use super::*;
    #[doc = "Type of the filter file."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FilterFileType {
        AzureBlob,
        AzureFile,
    }
}
#[doc = "Request body to get the availability for scheduling heavy orders."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeavyScheduleAvailabilityRequest {
    #[serde(flatten)]
    pub schedule_availability_request: ScheduleAvailabilityRequest,
}
impl HeavyScheduleAvailabilityRequest {
    pub fn new(schedule_availability_request: ScheduleAvailabilityRequest) -> Self {
        Self {
            schedule_availability_request,
        }
    }
}
#[doc = "Managed identity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityProperties {
    #[doc = "Managed service identity type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "User assigned identity properties."]
    #[serde(rename = "userAssigned", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned: Option<UserAssignedProperties>,
}
impl IdentityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional delivery info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDeliveryInfo {
    #[doc = "Scheduled date time."]
    #[serde(rename = "scheduledDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub scheduled_date_time: Option<time::OffsetDateTime>,
}
impl JobDeliveryInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDetails {
    #[doc = "List of stages that run in the job."]
    #[serde(rename = "jobStages", default, skip_serializing_if = "Vec::is_empty")]
    pub job_stages: Vec<JobStages>,
    #[doc = "Contact Details."]
    #[serde(rename = "contactDetails")]
    pub contact_details: ContactDetails,
    #[doc = "Shipping address where customer wishes to receive the device."]
    #[serde(rename = "shippingAddress", default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
    #[doc = "Shipping details."]
    #[serde(rename = "deliveryPackage", default, skip_serializing_if = "Option::is_none")]
    pub delivery_package: Option<PackageShippingDetails>,
    #[doc = "Shipping details."]
    #[serde(rename = "returnPackage", default, skip_serializing_if = "Option::is_none")]
    pub return_package: Option<PackageShippingDetails>,
    #[doc = "Details of the data to be imported into azure."]
    #[serde(rename = "dataImportDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub data_import_details: Vec<DataImportDetails>,
    #[doc = "Details of the data to be exported from azure."]
    #[serde(rename = "dataExportDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub data_export_details: Vec<DataExportDetails>,
    #[doc = "Indicates the type of job details."]
    #[serde(rename = "jobDetailsType")]
    pub job_details_type: job_details::JobDetailsType,
    #[doc = "Preferences related to the order"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Preferences>,
    #[doc = "List of copy log details."]
    #[serde(rename = "copyLogDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub copy_log_details: Vec<CopyLogDetails>,
    #[doc = "Shared access key to download the return shipment label"]
    #[serde(rename = "reverseShipmentLabelSasKey", default, skip_serializing_if = "Option::is_none")]
    pub reverse_shipment_label_sas_key: Option<String>,
    #[doc = "Shared access key to download the chain of custody logs"]
    #[serde(rename = "chainOfCustodySasKey", default, skip_serializing_if = "Option::is_none")]
    pub chain_of_custody_sas_key: Option<String>,
    #[doc = "Encryption key containing details about key to encrypt different keys."]
    #[serde(rename = "keyEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub key_encryption_key: Option<KeyEncryptionKey>,
    #[doc = "The expected size of the data, which needs to be transferred in this job, in terabytes."]
    #[serde(rename = "expectedDataSizeInTeraBytes", default, skip_serializing_if = "Option::is_none")]
    pub expected_data_size_in_tera_bytes: Option<i32>,
    #[doc = "Available actions on the job."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
    #[doc = "Last Mitigation Action Performed On Job"]
    #[serde(rename = "lastMitigationActionOnJob", default, skip_serializing_if = "Option::is_none")]
    pub last_mitigation_action_on_job: Option<LastMitigationActionOnJob>,
}
impl JobDetails {
    pub fn new(contact_details: ContactDetails, job_details_type: job_details::JobDetailsType) -> Self {
        Self {
            job_stages: Vec::new(),
            contact_details,
            shipping_address: None,
            delivery_package: None,
            return_package: None,
            data_import_details: Vec::new(),
            data_export_details: Vec::new(),
            job_details_type,
            preferences: None,
            copy_log_details: Vec::new(),
            reverse_shipment_label_sas_key: None,
            chain_of_custody_sas_key: None,
            key_encryption_key: None,
            expected_data_size_in_tera_bytes: None,
            actions: Vec::new(),
            last_mitigation_action_on_job: None,
        }
    }
}
pub mod job_details {
    use super::*;
    #[doc = "Indicates the type of job details."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum JobDetailsType {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
}
#[doc = "Job Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobProperties {
    #[doc = "Type of the data transfer."]
    #[serde(rename = "transferType")]
    pub transfer_type: job_properties::TransferType,
    #[doc = "Describes whether the job is cancellable or not."]
    #[serde(rename = "isCancellable", default, skip_serializing_if = "Option::is_none")]
    pub is_cancellable: Option<bool>,
    #[doc = "Describes whether the job is deletable or not."]
    #[serde(rename = "isDeletable", default, skip_serializing_if = "Option::is_none")]
    pub is_deletable: Option<bool>,
    #[doc = "Describes whether the shipping address is editable or not."]
    #[serde(rename = "isShippingAddressEditable", default, skip_serializing_if = "Option::is_none")]
    pub is_shipping_address_editable: Option<bool>,
    #[doc = "Is Prepare To Ship Enabled on this job"]
    #[serde(rename = "isPrepareToShipEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_prepare_to_ship_enabled: Option<bool>,
    #[doc = "Name of the stage which is in progress."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<job_properties::Status>,
    #[doc = "Time at which the job was started in UTC ISO 8601 format."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Cloud error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudError>,
    #[doc = "Job details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<JobDetails>,
    #[doc = "Reason for cancellation."]
    #[serde(rename = "cancellationReason", default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    #[doc = "Delivery type of Job."]
    #[serde(rename = "deliveryType", default, skip_serializing_if = "Option::is_none")]
    pub delivery_type: Option<job_properties::DeliveryType>,
    #[doc = "Additional delivery info."]
    #[serde(rename = "deliveryInfo", default, skip_serializing_if = "Option::is_none")]
    pub delivery_info: Option<JobDeliveryInfo>,
    #[doc = "Flag to indicate cancellation of scheduled job."]
    #[serde(rename = "isCancellableWithoutFee", default, skip_serializing_if = "Option::is_none")]
    pub is_cancellable_without_fee: Option<bool>,
}
impl JobProperties {
    pub fn new(transfer_type: job_properties::TransferType) -> Self {
        Self {
            transfer_type,
            is_cancellable: None,
            is_deletable: None,
            is_shipping_address_editable: None,
            is_prepare_to_ship_enabled: None,
            status: None,
            start_time: None,
            error: None,
            details: None,
            cancellation_reason: None,
            delivery_type: None,
            delivery_info: None,
            is_cancellable_without_fee: None,
        }
    }
}
pub mod job_properties {
    use super::*;
    #[doc = "Type of the data transfer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TransferType {
        ImportToAzure,
        ExportFromAzure,
    }
    #[doc = "Name of the stage which is in progress."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        DeviceOrdered,
        DevicePrepared,
        Dispatched,
        Delivered,
        PickedUp,
        #[serde(rename = "AtAzureDC")]
        AtAzureDc,
        DataCopy,
        Completed,
        CompletedWithErrors,
        Cancelled,
        #[serde(rename = "Failed_IssueReportedAtCustomer")]
        FailedIssueReportedAtCustomer,
        #[serde(rename = "Failed_IssueDetectedAtAzureDC")]
        FailedIssueDetectedAtAzureDc,
        Aborted,
        CompletedWithWarnings,
        #[serde(rename = "ReadyToDispatchFromAzureDC")]
        ReadyToDispatchFromAzureDc,
        #[serde(rename = "ReadyToReceiveAtAzureDC")]
        ReadyToReceiveAtAzureDc,
    }
    #[doc = "Delivery type of Job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeliveryType {
        NonScheduled,
        Scheduled,
    }
    impl Default for DeliveryType {
        fn default() -> Self {
            Self::NonScheduled
        }
    }
}
#[doc = "Job Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Job Properties"]
    pub properties: JobProperties,
    #[doc = "Name of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Id of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Type of the object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Provides details about resource creation and update time"]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl JobResource {
    pub fn new(resource: Resource, properties: JobProperties) -> Self {
        Self {
            resource,
            properties,
            name: None,
            id: None,
            type_: None,
            system_data: None,
        }
    }
}
#[doc = "Job Resource Collection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobResourceList {
    #[doc = "List of job resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobResource>,
    #[doc = "Link for the next set of job resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JobResourceUpdateParameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobResourceUpdateParameter {
    #[doc = "Job Properties for update"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateJobProperties>,
    #[doc = "The list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Msi identity details of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
}
impl JobResourceUpdateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The base class for the secrets"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobSecrets {
    #[doc = "Used to indicate what type of job secrets object."]
    #[serde(rename = "jobSecretsType")]
    pub job_secrets_type: job_secrets::JobSecretsType,
    #[doc = "Dc access security code"]
    #[serde(rename = "dcAccessSecurityCode", default, skip_serializing_if = "Option::is_none")]
    pub dc_access_security_code: Option<DcAccessSecurityCode>,
    #[doc = "Cloud error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudError>,
}
impl JobSecrets {
    pub fn new(job_secrets_type: job_secrets::JobSecretsType) -> Self {
        Self {
            job_secrets_type,
            dc_access_security_code: None,
            error: None,
        }
    }
}
pub mod job_secrets {
    use super::*;
    #[doc = "Used to indicate what type of job secrets object."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum JobSecretsType {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
}
#[doc = "Job stages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStages {
    #[doc = "Name of the job stage."]
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<job_stages::StageName>,
    #[doc = "Display name of the job stage."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Status of the job stage."]
    #[serde(rename = "stageStatus", default, skip_serializing_if = "Option::is_none")]
    pub stage_status: Option<job_stages::StageStatus>,
    #[doc = "Time for the job stage in UTC ISO 8601 format."]
    #[serde(rename = "stageTime", default, with = "azure_core::date::rfc3339::option")]
    pub stage_time: Option<time::OffsetDateTime>,
    #[doc = "Job Stage Details"]
    #[serde(rename = "jobStageDetails", default, skip_serializing_if = "Option::is_none")]
    pub job_stage_details: Option<serde_json::Value>,
}
impl JobStages {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_stages {
    use super::*;
    #[doc = "Name of the job stage."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageName {
        DeviceOrdered,
        DevicePrepared,
        Dispatched,
        Delivered,
        PickedUp,
        #[serde(rename = "AtAzureDC")]
        AtAzureDc,
        DataCopy,
        Completed,
        CompletedWithErrors,
        Cancelled,
        #[serde(rename = "Failed_IssueReportedAtCustomer")]
        FailedIssueReportedAtCustomer,
        #[serde(rename = "Failed_IssueDetectedAtAzureDC")]
        FailedIssueDetectedAtAzureDc,
        Aborted,
        CompletedWithWarnings,
        #[serde(rename = "ReadyToDispatchFromAzureDC")]
        ReadyToDispatchFromAzureDc,
        #[serde(rename = "ReadyToReceiveAtAzureDC")]
        ReadyToReceiveAtAzureDc,
    }
    #[doc = "Status of the job stage."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageStatus {
        None,
        InProgress,
        Succeeded,
        Failed,
        Cancelled,
        Cancelling,
        SucceededWithErrors,
        WaitingForCustomerAction,
        SucceededWithWarnings,
        WaitingForCustomerActionForKek,
        WaitingForCustomerActionForCleanUp,
        CustomerActionPerformedForCleanUp,
    }
}
#[doc = "Encryption key containing details about key to encrypt different keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyEncryptionKey {
    #[doc = "Type of encryption key used for key encryption."]
    #[serde(rename = "kekType")]
    pub kek_type: key_encryption_key::KekType,
    #[doc = "Managed identity properties."]
    #[serde(rename = "identityProperties", default, skip_serializing_if = "Option::is_none")]
    pub identity_properties: Option<IdentityProperties>,
    #[doc = "Key encryption key. It is required in case of Customer managed KekType."]
    #[serde(rename = "kekUrl", default, skip_serializing_if = "Option::is_none")]
    pub kek_url: Option<String>,
    #[doc = "Kek vault resource id. It is required in case of Customer managed KekType."]
    #[serde(rename = "kekVaultResourceID", default, skip_serializing_if = "Option::is_none")]
    pub kek_vault_resource_id: Option<String>,
}
impl KeyEncryptionKey {
    pub fn new(kek_type: key_encryption_key::KekType) -> Self {
        Self {
            kek_type,
            identity_properties: None,
            kek_url: None,
            kek_vault_resource_id: None,
        }
    }
}
pub mod key_encryption_key {
    use super::*;
    #[doc = "Type of encryption key used for key encryption."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KekType {
        MicrosoftManaged,
        CustomerManaged,
    }
    impl Default for KekType {
        fn default() -> Self {
            Self::MicrosoftManaged
        }
    }
}
#[doc = "Last Mitigation Action Performed On Job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LastMitigationActionOnJob {
    #[doc = "Action performed date time"]
    #[serde(rename = "actionDateTimeInUtc", default, with = "azure_core::date::rfc3339::option")]
    pub action_date_time_in_utc: Option<time::OffsetDateTime>,
    #[doc = "Action performed by customer,\r\npossibility is that mitigation might happen by customer or service or by ops"]
    #[serde(rename = "isPerformedByCustomer", default, skip_serializing_if = "Option::is_none")]
    pub is_performed_by_customer: Option<bool>,
    #[doc = "Resolution code provided by customer"]
    #[serde(rename = "customerResolution", default, skip_serializing_if = "Option::is_none")]
    pub customer_resolution: Option<last_mitigation_action_on_job::CustomerResolution>,
}
impl LastMitigationActionOnJob {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod last_mitigation_action_on_job {
    use super::*;
    #[doc = "Resolution code provided by customer"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CustomerResolution {
        None,
        MoveToCleanUpDevice,
        Resume,
    }
}
#[doc = "Details of the managed disks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedDiskDetails {
    #[serde(flatten)]
    pub data_account_details: DataAccountDetails,
    #[doc = "Resource Group Id of the compute disks."]
    #[serde(rename = "resourceGroupId")]
    pub resource_group_id: String,
    #[doc = "Resource Id of the storage account that can be used to copy the vhd for staging."]
    #[serde(rename = "stagingStorageAccountId")]
    pub staging_storage_account_id: String,
}
impl ManagedDiskDetails {
    pub fn new(data_account_details: DataAccountDetails, resource_group_id: String, staging_storage_account_id: String) -> Self {
        Self {
            data_account_details,
            resource_group_id,
            staging_storage_account_id,
        }
    }
}
#[doc = "The Mitigate Job captured from request body for Mitigate API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MitigateJobRequest {
    #[doc = "Resolution code for the job"]
    #[serde(rename = "customerResolutionCode")]
    pub customer_resolution_code: mitigate_job_request::CustomerResolutionCode,
}
impl MitigateJobRequest {
    pub fn new(customer_resolution_code: mitigate_job_request::CustomerResolutionCode) -> Self {
        Self { customer_resolution_code }
    }
}
pub mod mitigate_job_request {
    use super::*;
    #[doc = "Resolution code for the job"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CustomerResolutionCode {
        None,
        MoveToCleanUpDevice,
        Resume,
    }
}
#[doc = "Notification preference for a job stage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationPreference {
    #[doc = "Name of the stage."]
    #[serde(rename = "stageName")]
    pub stage_name: notification_preference::StageName,
    #[doc = "Notification is required or not."]
    #[serde(rename = "sendNotification")]
    pub send_notification: bool,
}
impl NotificationPreference {
    pub fn new(stage_name: notification_preference::StageName, send_notification: bool) -> Self {
        Self {
            stage_name,
            send_notification,
        }
    }
}
pub mod notification_preference {
    use super::*;
    #[doc = "Name of the stage."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StageName {
        DevicePrepared,
        Dispatched,
        Delivered,
        PickedUp,
        #[serde(rename = "AtAzureDC")]
        AtAzureDc,
        DataCopy,
    }
}
#[doc = "Operation entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation. Format: {resourceProviderNamespace}/{resourceType}/{read|write|delete|action}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Operation display"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Operation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
    #[doc = "Origin of the operation. Can be : user|system|user,system"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation display"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Provider name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Localized name of the operation for display purpose."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Localized description of the operation for display purpose."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation Collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "Link for the next set of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Shipping details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageShippingDetails {
    #[doc = "Name of the carrier."]
    #[serde(rename = "carrierName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<String>,
    #[doc = "Tracking Id of shipment."]
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[doc = "Url where shipment can be tracked."]
    #[serde(rename = "trackingUrl", default, skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
}
impl PackageShippingDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Preferences related to the order"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Preferences {
    #[doc = "Preferred data center region."]
    #[serde(rename = "preferredDataCenterRegion", default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_data_center_region: Vec<String>,
    #[doc = "Preferences related to the shipment logistics of the sku"]
    #[serde(rename = "transportPreferences", default, skip_serializing_if = "Option::is_none")]
    pub transport_preferences: Option<TransportPreferences>,
    #[doc = "Preferences related to the Encryption."]
    #[serde(rename = "encryptionPreferences", default, skip_serializing_if = "Option::is_none")]
    pub encryption_preferences: Option<EncryptionPreferences>,
}
impl Preferences {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request to validate preference of transport and data center."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreferencesValidationRequest {
    #[serde(flatten)]
    pub validation_input_request: ValidationInputRequest,
    #[doc = "Preferences related to the order"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preference: Option<Preferences>,
    #[doc = "Device type to be used for the job."]
    #[serde(rename = "deviceType")]
    pub device_type: preferences_validation_request::DeviceType,
}
impl PreferencesValidationRequest {
    pub fn new(validation_input_request: ValidationInputRequest, device_type: preferences_validation_request::DeviceType) -> Self {
        Self {
            validation_input_request,
            preference: None,
            device_type,
        }
    }
}
pub mod preferences_validation_request {
    use super::*;
    #[doc = "Device type to be used for the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeviceType {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
}
#[doc = "Properties of data center and transport preference validation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreferencesValidationResponseProperties {
    #[serde(flatten)]
    pub validation_input_response: ValidationInputResponse,
    #[doc = "Validation status of requested data center and transport."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<preferences_validation_response_properties::Status>,
}
impl PreferencesValidationResponseProperties {
    pub fn new(validation_input_response: ValidationInputResponse) -> Self {
        Self {
            validation_input_response,
            status: None,
        }
    }
}
pub mod preferences_validation_response_properties {
    use super::*;
    #[doc = "Validation status of requested data center and transport."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Valid,
        Invalid,
        Skipped,
    }
}
#[doc = "Request body to get the configuration for the region."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegionConfigurationRequest {
    #[doc = "Request body to get the availability for scheduling orders."]
    #[serde(rename = "scheduleAvailabilityRequest", default, skip_serializing_if = "Option::is_none")]
    pub schedule_availability_request: Option<ScheduleAvailabilityRequest>,
    #[doc = "Request body to get the transport availability for given sku."]
    #[serde(rename = "transportAvailabilityRequest", default, skip_serializing_if = "Option::is_none")]
    pub transport_availability_request: Option<TransportAvailabilityRequest>,
}
impl RegionConfigurationRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration response specific to a region."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegionConfigurationResponse {
    #[doc = "Schedule availability for given sku in a region."]
    #[serde(rename = "scheduleAvailabilityResponse", default, skip_serializing_if = "Option::is_none")]
    pub schedule_availability_response: Option<ScheduleAvailabilityResponse>,
    #[doc = "Transport options available for given sku in a region."]
    #[serde(rename = "transportAvailabilityResponse", default, skip_serializing_if = "Option::is_none")]
    pub transport_availability_response: Option<TransportAvailabilityResponse>,
}
impl RegionConfigurationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model of the Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "The location of the resource. This will be one of the supported and registered Azure Regions (e.g. West US, East US, Southeast Asia, etc.). The region of a resource cannot be changed once it is created, but if an identical region is specified on update the request will succeed."]
    pub location: String,
    #[doc = "The list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The Sku."]
    pub sku: Sku,
    #[doc = "Msi identity details of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
}
impl Resource {
    pub fn new(location: String, sku: Sku) -> Self {
        Self {
            location,
            tags: None,
            sku,
            identity: None,
        }
    }
}
#[doc = "Msi identity details of the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceIdentity {
    #[doc = "Identity type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Service Principal Id backing the Msi"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Home Tenant Id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "User Assigned Identities"]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ResourceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request body to get the availability for scheduling orders."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleAvailabilityRequest {
    #[doc = "Location for data transfer. For locations check: https://management.azure.com/subscriptions/SUBSCRIPTIONID/locations?api-version=2018-01-01"]
    #[serde(rename = "storageLocation")]
    pub storage_location: String,
    #[doc = "Sku Name for which the order is to be scheduled."]
    #[serde(rename = "skuName")]
    pub sku_name: schedule_availability_request::SkuName,
    #[doc = "Country in which storage location should be supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}
impl ScheduleAvailabilityRequest {
    pub fn new(storage_location: String, sku_name: schedule_availability_request::SkuName) -> Self {
        Self {
            storage_location,
            sku_name,
            country: None,
        }
    }
}
pub mod schedule_availability_request {
    use super::*;
    #[doc = "Sku Name for which the order is to be scheduled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SkuName {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
}
#[doc = "Schedule availability for given sku in a region."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleAvailabilityResponse {
    #[doc = "List of dates available to schedule"]
    #[serde(rename = "availableDates", default, skip_serializing_if = "Vec::is_empty")]
    pub available_dates: Vec<time::OffsetDateTime>,
}
impl ScheduleAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Credential details of the shares in account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareCredentialDetails {
    #[doc = "Name of the share."]
    #[serde(rename = "shareName", default, skip_serializing_if = "Option::is_none")]
    pub share_name: Option<String>,
    #[doc = "Type of the share."]
    #[serde(rename = "shareType", default, skip_serializing_if = "Option::is_none")]
    pub share_type: Option<share_credential_details::ShareType>,
    #[doc = "User name for the share."]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "Password for the share."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Access protocols supported on the device."]
    #[serde(rename = "supportedAccessProtocols", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_access_protocols: Vec<String>,
}
impl ShareCredentialDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod share_credential_details {
    use super::*;
    #[doc = "Type of the share."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ShareType {
        UnknownType,
        #[serde(rename = "HCS")]
        Hcs,
        BlockBlob,
        PageBlob,
        AzureFile,
        ManagedDisk,
    }
}
#[doc = "Shipment pick up request details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipmentPickUpRequest {
    #[doc = "Minimum date after which the pick up should commence, this must be in local time of pick up area."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "Maximum date before which the pick up should commence, this must be in local time of pick up area."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339")]
    pub end_time: time::OffsetDateTime,
    #[doc = "Shipment Location in the pickup place. Eg.front desk"]
    #[serde(rename = "shipmentLocation")]
    pub shipment_location: String,
}
impl ShipmentPickUpRequest {
    pub fn new(start_time: time::OffsetDateTime, end_time: time::OffsetDateTime, shipment_location: String) -> Self {
        Self {
            start_time,
            end_time,
            shipment_location,
        }
    }
}
#[doc = "Shipment pick up response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShipmentPickUpResponse {
    #[doc = "Confirmation number for the pick up request."]
    #[serde(rename = "confirmationNumber", default, skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<String>,
    #[doc = "Time by which shipment should be ready for pick up, this is in local time of pick up area."]
    #[serde(rename = "readyByTime", default, with = "azure_core::date::rfc3339::option")]
    pub ready_by_time: Option<time::OffsetDateTime>,
}
impl ShipmentPickUpResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Shipping address where customer wishes to receive the device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShippingAddress {
    #[doc = "Street Address line 1."]
    #[serde(rename = "streetAddress1")]
    pub street_address1: String,
    #[doc = "Street Address line 2."]
    #[serde(rename = "streetAddress2", default, skip_serializing_if = "Option::is_none")]
    pub street_address2: Option<String>,
    #[doc = "Street Address line 3."]
    #[serde(rename = "streetAddress3", default, skip_serializing_if = "Option::is_none")]
    pub street_address3: Option<String>,
    #[doc = "Name of the City."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Name of the State or Province."]
    #[serde(rename = "stateOrProvince", default, skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    #[doc = "Name of the Country."]
    pub country: String,
    #[doc = "Postal code."]
    #[serde(rename = "postalCode", default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "Extended Zip Code."]
    #[serde(rename = "zipExtendedCode", default, skip_serializing_if = "Option::is_none")]
    pub zip_extended_code: Option<String>,
    #[doc = "Name of the company."]
    #[serde(rename = "companyName", default, skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[doc = "Type of address."]
    #[serde(rename = "addressType", default, skip_serializing_if = "Option::is_none")]
    pub address_type: Option<shipping_address::AddressType>,
}
impl ShippingAddress {
    pub fn new(street_address1: String, country: String) -> Self {
        Self {
            street_address1,
            street_address2: None,
            street_address3: None,
            city: None,
            state_or_province: None,
            country,
            postal_code: None,
            zip_extended_code: None,
            company_name: None,
            address_type: None,
        }
    }
}
pub mod shipping_address {
    use super::*;
    #[doc = "Type of address."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AddressType {
        None,
        Residential,
        Commercial,
    }
    impl Default for AddressType {
        fn default() -> Self {
            Self::None
        }
    }
}
#[doc = "The Sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The sku name."]
    pub name: sku::Name,
    #[doc = "The display name of the sku."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The sku family."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self {
            name,
            display_name: None,
            family: None,
        }
    }
}
pub mod sku {
    use super::*;
    #[doc = "The sku name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
}
#[doc = "Request to validate sku availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuAvailabilityValidationRequest {
    #[serde(flatten)]
    pub validation_input_request: ValidationInputRequest,
    #[doc = "Device type to be used for the job."]
    #[serde(rename = "deviceType")]
    pub device_type: sku_availability_validation_request::DeviceType,
    #[doc = "Type of the transfer."]
    #[serde(rename = "transferType")]
    pub transfer_type: sku_availability_validation_request::TransferType,
    #[doc = "ISO country code. Country for hardware shipment. For codes check: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2#Officially_assigned_code_elements"]
    pub country: String,
    #[doc = "Location for data transfer. For locations check: https://management.azure.com/subscriptions/SUBSCRIPTIONID/locations?api-version=2018-01-01"]
    pub location: String,
}
impl SkuAvailabilityValidationRequest {
    pub fn new(
        validation_input_request: ValidationInputRequest,
        device_type: sku_availability_validation_request::DeviceType,
        transfer_type: sku_availability_validation_request::TransferType,
        country: String,
        location: String,
    ) -> Self {
        Self {
            validation_input_request,
            device_type,
            transfer_type,
            country,
            location,
        }
    }
}
pub mod sku_availability_validation_request {
    use super::*;
    #[doc = "Device type to be used for the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeviceType {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
    #[doc = "Type of the transfer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TransferType {
        ImportToAzure,
        ExportFromAzure,
    }
}
#[doc = "Properties of sku availability validation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuAvailabilityValidationResponseProperties {
    #[serde(flatten)]
    pub validation_input_response: ValidationInputResponse,
    #[doc = "Sku availability validation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<sku_availability_validation_response_properties::Status>,
}
impl SkuAvailabilityValidationResponseProperties {
    pub fn new(validation_input_response: ValidationInputResponse) -> Self {
        Self {
            validation_input_response,
            status: None,
        }
    }
}
pub mod sku_availability_validation_response_properties {
    use super::*;
    #[doc = "Sku availability validation status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Valid,
        Invalid,
        Skipped,
    }
}
#[doc = "Capacity of the sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapacity {
    #[doc = "Usable capacity in TB."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usable: Option<String>,
    #[doc = "Maximum capacity in TB."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,
}
impl SkuCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes metadata for retrieving price info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCost {
    #[doc = "Meter id of the Sku."]
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "The type of the meter."]
    #[serde(rename = "meterType", default, skip_serializing_if = "Option::is_none")]
    pub meter_type: Option<String>,
    #[doc = "Multiplier specifies the region specific value to be multiplied with 1$ guid. Eg: Our new regions will be using 1$ shipping guid with appropriate multiplier specific to region."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<f64>,
}
impl SkuCost {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information of the sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuInformation {
    #[doc = "The Sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The sku is enabled or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Properties of the sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SkuProperties>,
}
impl SkuInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuProperties {
    #[doc = "The map of data location to service location."]
    #[serde(rename = "dataLocationToServiceLocationMap", default, skip_serializing_if = "Vec::is_empty")]
    pub data_location_to_service_location_map: Vec<DataLocationToServiceLocationMap>,
    #[doc = "Capacity of the sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<SkuCapacity>,
    #[doc = "Cost of the Sku."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub costs: Vec<SkuCost>,
    #[doc = "Api versions that support this Sku."]
    #[serde(rename = "apiVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
    #[doc = "Reason why the Sku is disabled."]
    #[serde(rename = "disabledReason", default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<sku_properties::DisabledReason>,
    #[doc = "Message for why the Sku is disabled."]
    #[serde(rename = "disabledReasonMessage", default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason_message: Option<String>,
    #[doc = "Required feature to access the sku."]
    #[serde(rename = "requiredFeature", default, skip_serializing_if = "Option::is_none")]
    pub required_feature: Option<String>,
}
impl SkuProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku_properties {
    use super::*;
    #[doc = "Reason why the Sku is disabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DisabledReason {
        None,
        Country,
        Region,
        Feature,
        OfferType,
        NoSubscriptionInfo,
    }
}
#[doc = "Details for the storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountDetails {
    #[serde(flatten)]
    pub data_account_details: DataAccountDetails,
    #[doc = "Storage Account Resource Id."]
    #[serde(rename = "storageAccountId")]
    pub storage_account_id: String,
}
impl StorageAccountDetails {
    pub fn new(data_account_details: DataAccountDetails, storage_account_id: String) -> Self {
        Self {
            data_account_details,
            storage_account_id,
        }
    }
}
#[doc = "Request to validate subscription permission to create jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionIsAllowedToCreateJobValidationRequest {
    #[serde(flatten)]
    pub validation_input_request: ValidationInputRequest,
}
impl SubscriptionIsAllowedToCreateJobValidationRequest {
    pub fn new(validation_input_request: ValidationInputRequest) -> Self {
        Self { validation_input_request }
    }
}
#[doc = "Properties of subscription permission to create job validation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionIsAllowedToCreateJobValidationResponseProperties {
    #[serde(flatten)]
    pub validation_input_response: ValidationInputResponse,
    #[doc = "Validation status of subscription permission to create job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<subscription_is_allowed_to_create_job_validation_response_properties::Status>,
}
impl SubscriptionIsAllowedToCreateJobValidationResponseProperties {
    pub fn new(validation_input_response: ValidationInputResponse) -> Self {
        Self {
            validation_input_response,
            status: None,
        }
    }
}
pub mod subscription_is_allowed_to_create_job_validation_response_properties {
    use super::*;
    #[doc = "Validation status of subscription permission to create job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Valid,
        Invalid,
        Skipped,
    }
}
#[doc = "Details to transfer all data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferAllDetails {
    #[doc = "Type of the account of data"]
    #[serde(rename = "dataAccountType")]
    pub data_account_type: transfer_all_details::DataAccountType,
    #[doc = "To indicate if all Azure blobs have to be transferred"]
    #[serde(rename = "transferAllBlobs", default, skip_serializing_if = "Option::is_none")]
    pub transfer_all_blobs: Option<bool>,
    #[doc = "To indicate if all Azure Files have to be transferred"]
    #[serde(rename = "transferAllFiles", default, skip_serializing_if = "Option::is_none")]
    pub transfer_all_files: Option<bool>,
}
impl TransferAllDetails {
    pub fn new(data_account_type: transfer_all_details::DataAccountType) -> Self {
        Self {
            data_account_type,
            transfer_all_blobs: None,
            transfer_all_files: None,
        }
    }
}
pub mod transfer_all_details {
    use super::*;
    #[doc = "Type of the account of data"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataAccountType {
        StorageAccount,
        ManagedDisk,
    }
    impl Default for DataAccountType {
        fn default() -> Self {
            Self::StorageAccount
        }
    }
}
#[doc = "Configuration for defining the transfer of data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferConfiguration {
    #[doc = "Type of the configuration for transfer."]
    #[serde(rename = "transferConfigurationType")]
    pub transfer_configuration_type: transfer_configuration::TransferConfigurationType,
    #[doc = "Map of filter type and the details to filter. This field is required only if the TransferConfigurationType is given as TransferUsingFilter."]
    #[serde(rename = "transferFilterDetails", default, skip_serializing_if = "Option::is_none")]
    pub transfer_filter_details: Option<transfer_configuration::TransferFilterDetails>,
    #[doc = "Map of filter type and the details to transfer all data. This field is required only if the TransferConfigurationType is given as TransferAll"]
    #[serde(rename = "transferAllDetails", default, skip_serializing_if = "Option::is_none")]
    pub transfer_all_details: Option<transfer_configuration::TransferAllDetails>,
}
impl TransferConfiguration {
    pub fn new(transfer_configuration_type: transfer_configuration::TransferConfigurationType) -> Self {
        Self {
            transfer_configuration_type,
            transfer_filter_details: None,
            transfer_all_details: None,
        }
    }
}
pub mod transfer_configuration {
    use super::*;
    #[doc = "Type of the configuration for transfer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TransferConfigurationType {
        TransferAll,
        TransferUsingFilter,
    }
    #[doc = "Map of filter type and the details to filter. This field is required only if the TransferConfigurationType is given as TransferUsingFilter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct TransferFilterDetails {
        #[doc = "Details of the filtering the transfer of data."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub include: Box<Option<TransferFilterDetails>>,
    }
    impl TransferFilterDetails {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Map of filter type and the details to transfer all data. This field is required only if the TransferConfigurationType is given as TransferAll"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct TransferAllDetails {
        #[doc = "Details to transfer all data."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub include: Box<Option<TransferAllDetails>>,
    }
    impl TransferAllDetails {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Details of the filtering the transfer of data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferFilterDetails {
    #[doc = "Type of the account of data."]
    #[serde(rename = "dataAccountType")]
    pub data_account_type: transfer_filter_details::DataAccountType,
    #[doc = "Filter details to transfer Azure Blobs"]
    #[serde(rename = "blobFilterDetails", default, skip_serializing_if = "Option::is_none")]
    pub blob_filter_details: Option<BlobFilterDetails>,
    #[doc = "Filter details to transfer Azure files"]
    #[serde(rename = "azureFileFilterDetails", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_filter_details: Option<AzureFileFilterDetails>,
    #[doc = "Details of the filter files to be used for data transfer."]
    #[serde(rename = "filterFileDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub filter_file_details: Vec<FilterFileDetails>,
}
impl TransferFilterDetails {
    pub fn new(data_account_type: transfer_filter_details::DataAccountType) -> Self {
        Self {
            data_account_type,
            blob_filter_details: None,
            azure_file_filter_details: None,
            filter_file_details: Vec::new(),
        }
    }
}
pub mod transfer_filter_details {
    use super::*;
    #[doc = "Type of the account of data."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DataAccountType {
        StorageAccount,
        ManagedDisk,
    }
    impl Default for DataAccountType {
        fn default() -> Self {
            Self::StorageAccount
        }
    }
}
#[doc = "Transport options availability details for given region."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransportAvailabilityDetails {
    #[doc = "Transport Shipment Type supported for given region."]
    #[serde(rename = "shipmentType", default, skip_serializing_if = "Option::is_none")]
    pub shipment_type: Option<transport_availability_details::ShipmentType>,
}
impl TransportAvailabilityDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod transport_availability_details {
    use super::*;
    #[doc = "Transport Shipment Type supported for given region."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ShipmentType {
        CustomerManaged,
        MicrosoftManaged,
    }
}
#[doc = "Request body to get the transport availability for given sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransportAvailabilityRequest {
    #[doc = "Type of the device."]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<transport_availability_request::SkuName>,
}
impl TransportAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod transport_availability_request {
    use super::*;
    #[doc = "Type of the device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SkuName {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
}
#[doc = "Transport options available for given sku in a region."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransportAvailabilityResponse {
    #[doc = "List of transport availability details for given region"]
    #[serde(rename = "transportAvailabilityDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub transport_availability_details: Vec<TransportAvailabilityDetails>,
}
impl TransportAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Preferences related to the shipment logistics of the sku"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransportPreferences {
    #[doc = "Indicates Shipment Logistics type that the customer preferred."]
    #[serde(rename = "preferredShipmentType")]
    pub preferred_shipment_type: transport_preferences::PreferredShipmentType,
}
impl TransportPreferences {
    pub fn new(preferred_shipment_type: transport_preferences::PreferredShipmentType) -> Self {
        Self { preferred_shipment_type }
    }
}
pub mod transport_preferences {
    use super::*;
    #[doc = "Indicates Shipment Logistics type that the customer preferred."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PreferredShipmentType {
        CustomerManaged,
        MicrosoftManaged,
    }
}
#[doc = "Unencrypted credentials for accessing device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnencryptedCredentials {
    #[doc = "Name of the job."]
    #[serde(rename = "jobName", default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[doc = "The base class for the secrets"]
    #[serde(rename = "jobSecrets", default, skip_serializing_if = "Option::is_none")]
    pub job_secrets: Option<JobSecrets>,
}
impl UnencryptedCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of unencrypted credentials for accessing device."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnencryptedCredentialsList {
    #[doc = "List of unencrypted credentials."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UnencryptedCredentials>,
    #[doc = "Link for the next set of unencrypted credentials."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UnencryptedCredentialsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl UnencryptedCredentialsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job details for update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateJobDetails {
    #[doc = "Contact Details."]
    #[serde(rename = "contactDetails", default, skip_serializing_if = "Option::is_none")]
    pub contact_details: Option<ContactDetails>,
    #[doc = "Shipping address where customer wishes to receive the device."]
    #[serde(rename = "shippingAddress", default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
    #[doc = "Encryption key containing details about key to encrypt different keys."]
    #[serde(rename = "keyEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub key_encryption_key: Option<KeyEncryptionKey>,
}
impl UpdateJobDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job Properties for update"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateJobProperties {
    #[doc = "Job details for update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<UpdateJobDetails>,
}
impl UpdateJobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Class defining User assigned identity details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal id of user assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client id of user assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedProperties {
    #[doc = "Arm resource id for user assigned identity to be used to fetch MSI token."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl UserAssignedProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The requirements to validate customer address where the device needs to be shipped."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateAddress {
    #[serde(flatten)]
    pub validation_input_request: ValidationInputRequest,
    #[doc = "Shipping address where customer wishes to receive the device."]
    #[serde(rename = "shippingAddress")]
    pub shipping_address: ShippingAddress,
    #[doc = "Device type to be used for the job."]
    #[serde(rename = "deviceType")]
    pub device_type: validate_address::DeviceType,
    #[doc = "Preferences related to the shipment logistics of the sku"]
    #[serde(rename = "transportPreferences", default, skip_serializing_if = "Option::is_none")]
    pub transport_preferences: Option<TransportPreferences>,
}
impl ValidateAddress {
    pub fn new(
        validation_input_request: ValidationInputRequest,
        shipping_address: ShippingAddress,
        device_type: validate_address::DeviceType,
    ) -> Self {
        Self {
            validation_input_request,
            shipping_address,
            device_type,
            transport_preferences: None,
        }
    }
}
pub mod validate_address {
    use super::*;
    #[doc = "Device type to be used for the job."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeviceType {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
    }
}
#[doc = "Minimum fields that must be present in any type of validation request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationInputRequest {
    #[doc = "Identifies the type of validation request."]
    #[serde(rename = "validationType")]
    pub validation_type: validation_input_request::ValidationType,
}
impl ValidationInputRequest {
    pub fn new(validation_type: validation_input_request::ValidationType) -> Self {
        Self { validation_type }
    }
}
pub mod validation_input_request {
    use super::*;
    #[doc = "Identifies the type of validation request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ValidationType {
        ValidateAddress,
        ValidateSubscriptionIsAllowedToCreateJob,
        ValidatePreferences,
        ValidateCreateOrderLimit,
        ValidateSkuAvailability,
        ValidateDataTransferDetails,
    }
}
#[doc = "Minimum properties that should be present in each individual validation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationInputResponse {
    #[doc = "Identifies the type of validation response."]
    #[serde(rename = "validationType")]
    pub validation_type: validation_input_response::ValidationType,
    #[doc = "Cloud error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudError>,
}
impl ValidationInputResponse {
    pub fn new(validation_type: validation_input_response::ValidationType) -> Self {
        Self {
            validation_type,
            error: None,
        }
    }
}
pub mod validation_input_response {
    use super::*;
    #[doc = "Identifies the type of validation response."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ValidationType {
        ValidateAddress,
        ValidateSubscriptionIsAllowedToCreateJob,
        ValidatePreferences,
        ValidateCreateOrderLimit,
        ValidateSkuAvailability,
        ValidateDataTransferDetails,
    }
}
#[doc = "Minimum request requirement of any validation category."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationRequest {
    #[doc = "Identify the nature of validation."]
    #[serde(rename = "validationCategory")]
    pub validation_category: validation_request::ValidationCategory,
    #[doc = "List of request details contain validationType and its request as key and value respectively."]
    #[serde(rename = "individualRequestDetails")]
    pub individual_request_details: Vec<ValidationInputRequest>,
}
impl ValidationRequest {
    pub fn new(
        validation_category: validation_request::ValidationCategory,
        individual_request_details: Vec<ValidationInputRequest>,
    ) -> Self {
        Self {
            validation_category,
            individual_request_details,
        }
    }
}
pub mod validation_request {
    use super::*;
    #[doc = "Identify the nature of validation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ValidationCategory {
        JobCreationValidation,
    }
}
#[doc = "Response of pre job creation validations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationResponse {
    #[doc = "Properties of pre job creation validation response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ValidationResponseProperties>,
}
impl ValidationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of pre job creation validation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationResponseProperties {
    #[doc = "Overall validation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<validation_response_properties::Status>,
    #[doc = "List of response details contain validationType and its response as key and value respectively."]
    #[serde(rename = "individualResponseDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub individual_response_details: Vec<ValidationInputResponse>,
}
impl ValidationResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod validation_response_properties {
    use super::*;
    #[doc = "Overall validation status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        AllValidToProceed,
        InputsRevisitRequired,
        CertainInputValidationsSkipped,
    }
}
#[doc = "Provides details about resource creation and update time"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "A string identifier for the identity that created the resource"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource: user, application, managedIdentity"]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<String>,
    #[doc = "The timestamp of resource creation (UTC)"]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "A string identifier for the identity that last modified the resource"]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource: user, application, managedIdentity"]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<String>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
