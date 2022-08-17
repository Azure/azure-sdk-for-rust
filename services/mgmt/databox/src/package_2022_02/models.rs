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
#[doc = "This class represents additional info which Resource Providers pass when an error occurs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdditionalErrorInfo {
    #[doc = "Additional information of the type of error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
    #[doc = "Type of error (e.g. CustomerIntervention, PolicyViolation, SecurityViolation)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
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
#[doc = "Provides additional information about an http error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Gets or sets additional error info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<AdditionalErrorInfo>,
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Gets or sets details for the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudError>,
    #[doc = "The error message parsed from the body of the http error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Gets or sets the target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
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
        DataBoxCustomerDisk,
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
        DataBoxCustomerDisk,
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
#[doc = "The secrets related to customer disk job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerDiskJobSecrets {
    #[serde(flatten)]
    pub job_secrets: JobSecrets,
    #[doc = "Contains the list of secrets object for that device."]
    #[serde(rename = "diskSecrets", default, skip_serializing_if = "Vec::is_empty")]
    pub disk_secrets: Vec<DiskSecret>,
    #[doc = "Carrier Account Number of the customer"]
    #[serde(rename = "carrierAccountNumber", default, skip_serializing_if = "Option::is_none")]
    pub carrier_account_number: Option<String>,
}
impl CustomerDiskJobSecrets {
    pub fn new(job_secrets: JobSecrets) -> Self {
        Self {
            job_secrets,
            disk_secrets: Vec::new(),
            carrier_account_number: None,
        }
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
#[doc = "Copy Log Details for customer disk"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxCustomerDiskCopyLogDetails {
    #[serde(flatten)]
    pub copy_log_details: CopyLogDetails,
    #[doc = "Disk Serial Number."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Link for copy error logs."]
    #[serde(rename = "errorLogLink", default, skip_serializing_if = "Option::is_none")]
    pub error_log_link: Option<String>,
    #[doc = "Link for copy verbose logs."]
    #[serde(rename = "verboseLogLink", default, skip_serializing_if = "Option::is_none")]
    pub verbose_log_link: Option<String>,
}
impl DataBoxCustomerDiskCopyLogDetails {
    pub fn new(copy_log_details: CopyLogDetails) -> Self {
        Self {
            copy_log_details,
            serial_number: None,
            error_log_link: None,
            verbose_log_link: None,
        }
    }
}
#[doc = "DataBox CustomerDisk Copy Progress"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxCustomerDiskCopyProgress {
    #[serde(flatten)]
    pub copy_progress: CopyProgress,
    #[doc = "Disk Serial Number."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "The Status of the copy"]
    #[serde(rename = "copyStatus", default, skip_serializing_if = "Option::is_none")]
    pub copy_status: Option<data_box_customer_disk_copy_progress::CopyStatus>,
}
impl DataBoxCustomerDiskCopyProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_box_customer_disk_copy_progress {
    use super::*;
    #[doc = "The Status of the copy"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CopyStatus")]
    pub enum CopyStatus {
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
        DriveNotReceived,
        UnsupportedDrive,
        OtherServiceError,
        OtherUserError,
        DriveNotDetected,
        DriveCorrupted,
        MetadataFilesModifiedOrRemoved,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CopyStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CopyStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CopyStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotStarted => serializer.serialize_unit_variant("CopyStatus", 0u32, "NotStarted"),
                Self::InProgress => serializer.serialize_unit_variant("CopyStatus", 1u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("CopyStatus", 2u32, "Completed"),
                Self::CompletedWithErrors => serializer.serialize_unit_variant("CopyStatus", 3u32, "CompletedWithErrors"),
                Self::Failed => serializer.serialize_unit_variant("CopyStatus", 4u32, "Failed"),
                Self::NotReturned => serializer.serialize_unit_variant("CopyStatus", 5u32, "NotReturned"),
                Self::HardwareError => serializer.serialize_unit_variant("CopyStatus", 6u32, "HardwareError"),
                Self::DeviceFormatted => serializer.serialize_unit_variant("CopyStatus", 7u32, "DeviceFormatted"),
                Self::DeviceMetadataModified => serializer.serialize_unit_variant("CopyStatus", 8u32, "DeviceMetadataModified"),
                Self::StorageAccountNotAccessible => serializer.serialize_unit_variant("CopyStatus", 9u32, "StorageAccountNotAccessible"),
                Self::UnsupportedData => serializer.serialize_unit_variant("CopyStatus", 10u32, "UnsupportedData"),
                Self::DriveNotReceived => serializer.serialize_unit_variant("CopyStatus", 11u32, "DriveNotReceived"),
                Self::UnsupportedDrive => serializer.serialize_unit_variant("CopyStatus", 12u32, "UnsupportedDrive"),
                Self::OtherServiceError => serializer.serialize_unit_variant("CopyStatus", 13u32, "OtherServiceError"),
                Self::OtherUserError => serializer.serialize_unit_variant("CopyStatus", 14u32, "OtherUserError"),
                Self::DriveNotDetected => serializer.serialize_unit_variant("CopyStatus", 15u32, "DriveNotDetected"),
                Self::DriveCorrupted => serializer.serialize_unit_variant("CopyStatus", 16u32, "DriveCorrupted"),
                Self::MetadataFilesModifiedOrRemoved => {
                    serializer.serialize_unit_variant("CopyStatus", 17u32, "MetadataFilesModifiedOrRemoved")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Customer disk job details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxCustomerDiskJobDetails {
    #[serde(flatten)]
    pub job_details: JobDetails,
    #[doc = "Contains the map of disk serial number to the disk details for import jobs."]
    #[serde(rename = "importDiskDetailsCollection", default, skip_serializing_if = "Option::is_none")]
    pub import_disk_details_collection: Option<serde_json::Value>,
    #[doc = "Contains the map of disk serial number to the disk details for export jobs."]
    #[serde(rename = "exportDiskDetailsCollection", default, skip_serializing_if = "Option::is_none")]
    pub export_disk_details_collection: Option<serde_json::Value>,
    #[doc = "Copy progress per disk."]
    #[serde(rename = "copyProgress", default, skip_serializing_if = "Vec::is_empty")]
    pub copy_progress: Vec<DataBoxCustomerDiskCopyProgress>,
    #[doc = "package carrier info"]
    #[serde(rename = "deliverToDcPackageDetails", default, skip_serializing_if = "Option::is_none")]
    pub deliver_to_dc_package_details: Option<PackageCarrierInfo>,
    #[doc = "Package carrier details."]
    #[serde(rename = "returnToCustomerPackageDetails")]
    pub return_to_customer_package_details: PackageCarrierDetails,
    #[doc = "Flag to indicate if disk manifest should be backed-up in the Storage Account."]
    #[serde(rename = "enableManifestBackup", default, skip_serializing_if = "Option::is_none")]
    pub enable_manifest_backup: Option<bool>,
}
impl DataBoxCustomerDiskJobDetails {
    pub fn new(job_details: JobDetails, return_to_customer_package_details: PackageCarrierDetails) -> Self {
        Self {
            job_details,
            import_disk_details_collection: None,
            export_disk_details_collection: None,
            copy_progress: Vec::new(),
            deliver_to_dc_package_details: None,
            return_to_customer_package_details,
            enable_manifest_backup: None,
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
    #[serde(remote = "Status")]
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
        DriveNotReceived,
        UnsupportedDrive,
        OtherServiceError,
        OtherUserError,
        DriveNotDetected,
        DriveCorrupted,
        MetadataFilesModifiedOrRemoved,
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
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("Status", 2u32, "Completed"),
                Self::CompletedWithErrors => serializer.serialize_unit_variant("Status", 3u32, "CompletedWithErrors"),
                Self::Failed => serializer.serialize_unit_variant("Status", 4u32, "Failed"),
                Self::NotReturned => serializer.serialize_unit_variant("Status", 5u32, "NotReturned"),
                Self::HardwareError => serializer.serialize_unit_variant("Status", 6u32, "HardwareError"),
                Self::DeviceFormatted => serializer.serialize_unit_variant("Status", 7u32, "DeviceFormatted"),
                Self::DeviceMetadataModified => serializer.serialize_unit_variant("Status", 8u32, "DeviceMetadataModified"),
                Self::StorageAccountNotAccessible => serializer.serialize_unit_variant("Status", 9u32, "StorageAccountNotAccessible"),
                Self::UnsupportedData => serializer.serialize_unit_variant("Status", 10u32, "UnsupportedData"),
                Self::DriveNotReceived => serializer.serialize_unit_variant("Status", 11u32, "DriveNotReceived"),
                Self::UnsupportedDrive => serializer.serialize_unit_variant("Status", 12u32, "UnsupportedDrive"),
                Self::OtherServiceError => serializer.serialize_unit_variant("Status", 13u32, "OtherServiceError"),
                Self::OtherUserError => serializer.serialize_unit_variant("Status", 14u32, "OtherUserError"),
                Self::DriveNotDetected => serializer.serialize_unit_variant("Status", 15u32, "DriveNotDetected"),
                Self::DriveCorrupted => serializer.serialize_unit_variant("Status", 16u32, "DriveCorrupted"),
                Self::MetadataFilesModifiedOrRemoved => {
                    serializer.serialize_unit_variant("Status", 17u32, "MetadataFilesModifiedOrRemoved")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Granular Copy Log Details for customer disk"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataBoxDiskGranularCopyLogDetails {
    #[serde(flatten)]
    pub granular_copy_log_details: GranularCopyLogDetails,
    #[doc = "Disk Serial Number."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "Account name."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "Link for copy error logs."]
    #[serde(rename = "errorLogLink", default, skip_serializing_if = "Option::is_none")]
    pub error_log_link: Option<String>,
    #[doc = "Link for copy verbose logs."]
    #[serde(rename = "verboseLogLink", default, skip_serializing_if = "Option::is_none")]
    pub verbose_log_link: Option<String>,
}
impl DataBoxDiskGranularCopyLogDetails {
    pub fn new(granular_copy_log_details: GranularCopyLogDetails) -> Self {
        Self {
            granular_copy_log_details,
            serial_number: None,
            account_name: None,
            error_log_link: None,
            verbose_log_link: None,
        }
    }
}
#[doc = "DataBox Disk Granular Copy Progress"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataBoxDiskGranularCopyProgress {
    #[serde(flatten)]
    pub granular_copy_progress: GranularCopyProgress,
    #[doc = "Disk Serial Number."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "The Status of the copy"]
    #[serde(rename = "copyStatus", default, skip_serializing_if = "Option::is_none")]
    pub copy_status: Option<data_box_disk_granular_copy_progress::CopyStatus>,
}
impl DataBoxDiskGranularCopyProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_box_disk_granular_copy_progress {
    use super::*;
    #[doc = "The Status of the copy"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CopyStatus")]
    pub enum CopyStatus {
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
        DriveNotReceived,
        UnsupportedDrive,
        OtherServiceError,
        OtherUserError,
        DriveNotDetected,
        DriveCorrupted,
        MetadataFilesModifiedOrRemoved,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CopyStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CopyStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CopyStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotStarted => serializer.serialize_unit_variant("CopyStatus", 0u32, "NotStarted"),
                Self::InProgress => serializer.serialize_unit_variant("CopyStatus", 1u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("CopyStatus", 2u32, "Completed"),
                Self::CompletedWithErrors => serializer.serialize_unit_variant("CopyStatus", 3u32, "CompletedWithErrors"),
                Self::Failed => serializer.serialize_unit_variant("CopyStatus", 4u32, "Failed"),
                Self::NotReturned => serializer.serialize_unit_variant("CopyStatus", 5u32, "NotReturned"),
                Self::HardwareError => serializer.serialize_unit_variant("CopyStatus", 6u32, "HardwareError"),
                Self::DeviceFormatted => serializer.serialize_unit_variant("CopyStatus", 7u32, "DeviceFormatted"),
                Self::DeviceMetadataModified => serializer.serialize_unit_variant("CopyStatus", 8u32, "DeviceMetadataModified"),
                Self::StorageAccountNotAccessible => serializer.serialize_unit_variant("CopyStatus", 9u32, "StorageAccountNotAccessible"),
                Self::UnsupportedData => serializer.serialize_unit_variant("CopyStatus", 10u32, "UnsupportedData"),
                Self::DriveNotReceived => serializer.serialize_unit_variant("CopyStatus", 11u32, "DriveNotReceived"),
                Self::UnsupportedDrive => serializer.serialize_unit_variant("CopyStatus", 12u32, "UnsupportedDrive"),
                Self::OtherServiceError => serializer.serialize_unit_variant("CopyStatus", 13u32, "OtherServiceError"),
                Self::OtherUserError => serializer.serialize_unit_variant("CopyStatus", 14u32, "OtherUserError"),
                Self::DriveNotDetected => serializer.serialize_unit_variant("CopyStatus", 15u32, "DriveNotDetected"),
                Self::DriveCorrupted => serializer.serialize_unit_variant("CopyStatus", 16u32, "DriveCorrupted"),
                Self::MetadataFilesModifiedOrRemoved => {
                    serializer.serialize_unit_variant("CopyStatus", 17u32, "MetadataFilesModifiedOrRemoved")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
    #[doc = "Copy progress per disk."]
    #[serde(rename = "granularCopyProgress", default, skip_serializing_if = "Vec::is_empty")]
    pub granular_copy_progress: Vec<DataBoxDiskGranularCopyProgress>,
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
            granular_copy_progress: Vec::new(),
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
    #[doc = "Level of the logs to be collected."]
    #[serde(rename = "logCollectionLevel", default, skip_serializing_if = "Option::is_none")]
    pub log_collection_level: Option<data_import_details::LogCollectionLevel>,
}
impl DataImportDetails {
    pub fn new(account_details: DataAccountDetails) -> Self {
        Self {
            account_details,
            log_collection_level: None,
        }
    }
}
pub mod data_import_details {
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
        DataBoxCustomerDisk,
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
#[doc = "Datacenter instruction for given storage location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatacenterAddressInstructionResponse {
    #[serde(flatten)]
    pub datacenter_address_response: DatacenterAddressResponse,
    #[doc = "Data center communication instruction"]
    #[serde(rename = "communicationInstruction", default, skip_serializing_if = "Option::is_none")]
    pub communication_instruction: Option<String>,
}
impl DatacenterAddressInstructionResponse {
    pub fn new(datacenter_address_response: DatacenterAddressResponse) -> Self {
        Self {
            datacenter_address_response,
            communication_instruction: None,
        }
    }
}
#[doc = "Datacenter address for given storage location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatacenterAddressLocationResponse {
    #[serde(flatten)]
    pub datacenter_address_response: DatacenterAddressResponse,
    #[doc = "Contact person name"]
    #[serde(rename = "contactPersonName", default, skip_serializing_if = "Option::is_none")]
    pub contact_person_name: Option<String>,
    #[doc = "Company name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[doc = "Street address line 1"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street1: Option<String>,
    #[doc = "Street address line 2"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,
    #[doc = "Street address line 3"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street3: Option<String>,
    #[doc = "City name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "name of the state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Zip code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[doc = "name of the country"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Phone number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "Phone extension"]
    #[serde(rename = "phoneExtension", default, skip_serializing_if = "Option::is_none")]
    pub phone_extension: Option<String>,
    #[doc = "Address type"]
    #[serde(rename = "addressType", default, skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    #[doc = "Special instruction for shipping"]
    #[serde(rename = "additionalShippingInformation", default, skip_serializing_if = "Option::is_none")]
    pub additional_shipping_information: Option<String>,
}
impl DatacenterAddressLocationResponse {
    pub fn new(datacenter_address_response: DatacenterAddressResponse) -> Self {
        Self {
            datacenter_address_response,
            contact_person_name: None,
            company: None,
            street1: None,
            street2: None,
            street3: None,
            city: None,
            state: None,
            zip: None,
            country: None,
            phone: None,
            phone_extension: None,
            address_type: None,
            additional_shipping_information: None,
        }
    }
}
#[doc = "Request body to get the datacenter address."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatacenterAddressRequest {
    #[doc = "Storage location. For locations check: https://management.azure.com/subscriptions/SUBSCRIPTIONID/locations?api-version=2018-01-01"]
    #[serde(rename = "storageLocation")]
    pub storage_location: String,
    #[doc = "Sku Name for which the data center address requested."]
    #[serde(rename = "skuName")]
    pub sku_name: datacenter_address_request::SkuName,
}
impl DatacenterAddressRequest {
    pub fn new(storage_location: String, sku_name: datacenter_address_request::SkuName) -> Self {
        Self {
            storage_location,
            sku_name,
        }
    }
}
pub mod datacenter_address_request {
    use super::*;
    #[doc = "Sku Name for which the data center address requested."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SkuName {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
        DataBoxCustomerDisk,
    }
}
#[doc = "Datacenter address for given storage location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatacenterAddressResponse {
    #[doc = "Data center address type"]
    #[serde(rename = "datacenterAddressType")]
    pub datacenter_address_type: datacenter_address_response::DatacenterAddressType,
    #[doc = "List of supported carriers for return shipment."]
    #[serde(rename = "supportedCarriersForReturnShipment", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_carriers_for_return_shipment: Vec<String>,
    #[doc = "Azure Location where the Data Center serves primarily."]
    #[serde(rename = "dataCenterAzureLocation", default, skip_serializing_if = "Option::is_none")]
    pub data_center_azure_location: Option<String>,
}
impl DatacenterAddressResponse {
    pub fn new(datacenter_address_type: datacenter_address_response::DatacenterAddressType) -> Self {
        Self {
            datacenter_address_type,
            supported_carriers_for_return_shipment: Vec::new(),
            data_center_azure_location: None,
        }
    }
}
pub mod datacenter_address_response {
    use super::*;
    #[doc = "Data center address type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DatacenterAddressType {
        DatacenterAddressLocation,
        DatacenterAddressInstruction,
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
#[doc = "Device erasure details with erasure completion status and erasureordestructionlog sas key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceErasureDetails {
    #[doc = "Holds the device erasure completion status"]
    #[serde(rename = "deviceErasureStatus", default, skip_serializing_if = "Option::is_none")]
    pub device_erasure_status: Option<device_erasure_details::DeviceErasureStatus>,
    #[doc = "Shared access key to download cleanup or destruction certificate for device"]
    #[serde(rename = "erasureOrDestructionCertificateSasKey", default, skip_serializing_if = "Option::is_none")]
    pub erasure_or_destruction_certificate_sas_key: Option<String>,
}
impl DeviceErasureDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod device_erasure_details {
    use super::*;
    #[doc = "Holds the device erasure completion status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeviceErasureStatus {
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
        CustomerActionPerformed,
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
#[doc = "Export disk details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportDiskDetails {
    #[doc = "The relative path of the manifest file on the disk."]
    #[serde(rename = "manifestFile", default, skip_serializing_if = "Option::is_none")]
    pub manifest_file: Option<String>,
    #[doc = "The Base16-encoded MD5 hash of the manifest file on the disk."]
    #[serde(rename = "manifestHash", default, skip_serializing_if = "Option::is_none")]
    pub manifest_hash: Option<String>,
    #[doc = "Path to backed up manifest, only returned if enableManifestBackup is true."]
    #[serde(rename = "backupManifestCloudPath", default, skip_serializing_if = "Option::is_none")]
    pub backup_manifest_cloud_path: Option<String>,
}
impl ExportDiskDetails {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Granular Details for log generated during copy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GranularCopyLogDetails {
    #[doc = "Indicates the type of job details."]
    #[serde(rename = "copyLogDetailsType")]
    pub copy_log_details_type: granular_copy_log_details::CopyLogDetailsType,
}
impl GranularCopyLogDetails {
    pub fn new(copy_log_details_type: granular_copy_log_details::CopyLogDetailsType) -> Self {
        Self { copy_log_details_type }
    }
}
pub mod granular_copy_log_details {
    use super::*;
    #[doc = "Indicates the type of job details."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CopyLogDetailsType {
        DataBox,
        DataBoxDisk,
        DataBoxHeavy,
        DataBoxCustomerDisk,
    }
}
#[doc = "Granular Copy progress."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GranularCopyProgress {
    #[doc = "Name of the storage account. This will be empty for data account types other than storage account."]
    #[serde(rename = "storageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_name: Option<String>,
    #[doc = "Transfer type of data"]
    #[serde(rename = "transferType", default, skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<granular_copy_progress::TransferType>,
    #[doc = "Data Account Type."]
    #[serde(rename = "dataAccountType", default, skip_serializing_if = "Option::is_none")]
    pub data_account_type: Option<granular_copy_progress::DataAccountType>,
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
impl GranularCopyProgress {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod granular_copy_progress {
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
#[doc = "Import disk details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportDiskDetails {
    #[doc = "The relative path of the manifest file on the disk."]
    #[serde(rename = "manifestFile")]
    pub manifest_file: String,
    #[doc = "The Base16-encoded MD5 hash of the manifest file on the disk."]
    #[serde(rename = "manifestHash")]
    pub manifest_hash: String,
    #[doc = "BitLocker key used to encrypt the disk."]
    #[serde(rename = "bitLockerKey")]
    pub bit_locker_key: String,
    #[doc = "Path to backed up manifest, only returned if enableManifestBackup is true."]
    #[serde(rename = "backupManifestCloudPath", default, skip_serializing_if = "Option::is_none")]
    pub backup_manifest_cloud_path: Option<String>,
}
impl ImportDiskDetails {
    pub fn new(manifest_file: String, manifest_hash: String, bit_locker_key: String) -> Self {
        Self {
            manifest_file,
            manifest_hash,
            bit_locker_key,
            backup_manifest_cloud_path: None,
        }
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
    #[doc = "package shipping details"]
    #[serde(rename = "deliveryPackage", default, skip_serializing_if = "Option::is_none")]
    pub delivery_package: Option<PackageShippingDetails>,
    #[doc = "package shipping details"]
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
    #[doc = "Device erasure details with erasure completion status and erasureordestructionlog sas key"]
    #[serde(rename = "deviceErasureDetails", default, skip_serializing_if = "Option::is_none")]
    pub device_erasure_details: Option<DeviceErasureDetails>,
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
    #[doc = "Datacenter address for given storage location."]
    #[serde(rename = "datacenterAddress", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_address: Option<DatacenterAddressResponse>,
    #[doc = "DataCenter code."]
    #[serde(rename = "dataCenterCode", default, skip_serializing_if = "Option::is_none")]
    pub data_center_code: Option<job_details::DataCenterCode>,
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
            device_erasure_details: None,
            key_encryption_key: None,
            expected_data_size_in_tera_bytes: None,
            actions: Vec::new(),
            last_mitigation_action_on_job: None,
            datacenter_address: None,
            data_center_code: None,
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
        DataBoxCustomerDisk,
    }
    #[doc = "DataCenter code."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataCenterCode")]
    pub enum DataCenterCode {
        Invalid,
        #[serde(rename = "BY2")]
        By2,
        #[serde(rename = "BY1")]
        By1,
        #[serde(rename = "ORK70")]
        Ork70,
        #[serde(rename = "AM2")]
        Am2,
        #[serde(rename = "AMS20")]
        Ams20,
        #[serde(rename = "BY21")]
        By21,
        #[serde(rename = "BY24")]
        By24,
        #[serde(rename = "MWH01")]
        Mwh01,
        #[serde(rename = "AMS06")]
        Ams06,
        #[serde(rename = "SSE90")]
        Sse90,
        #[serde(rename = "SYD03")]
        Syd03,
        #[serde(rename = "SYD23")]
        Syd23,
        #[serde(rename = "CBR20")]
        Cbr20,
        #[serde(rename = "YTO20")]
        Yto20,
        #[serde(rename = "CWL20")]
        Cwl20,
        #[serde(rename = "LON24")]
        Lon24,
        #[serde(rename = "BOM01")]
        Bom01,
        #[serde(rename = "BL20")]
        Bl20,
        #[serde(rename = "BL7")]
        Bl7,
        #[serde(rename = "SEL20")]
        Sel20,
        #[serde(rename = "TYO01")]
        Tyo01,
        #[serde(rename = "BN1")]
        Bn1,
        #[serde(rename = "SN5")]
        Sn5,
        #[serde(rename = "CYS04")]
        Cys04,
        #[serde(rename = "TYO22")]
        Tyo22,
        #[serde(rename = "YTO21")]
        Yto21,
        #[serde(rename = "YQB20")]
        Yqb20,
        #[serde(rename = "FRA22")]
        Fra22,
        #[serde(rename = "MAA01")]
        Maa01,
        #[serde(rename = "CPQ02")]
        Cpq02,
        #[serde(rename = "CPQ20")]
        Cpq20,
        #[serde(rename = "SIN20")]
        Sin20,
        #[serde(rename = "HKG20")]
        Hkg20,
        #[serde(rename = "SG2")]
        Sg2,
        #[serde(rename = "MEL23")]
        Mel23,
        #[serde(rename = "SEL21")]
        Sel21,
        #[serde(rename = "OSA20")]
        Osa20,
        #[serde(rename = "SHA03")]
        Sha03,
        #[serde(rename = "BJB")]
        Bjb,
        #[serde(rename = "JNB22")]
        Jnb22,
        #[serde(rename = "JNB21")]
        Jnb21,
        #[serde(rename = "MNZ21")]
        Mnz21,
        #[serde(rename = "SN8")]
        Sn8,
        #[serde(rename = "AUH20")]
        Auh20,
        #[serde(rename = "ZRH20")]
        Zrh20,
        #[serde(rename = "PUS20")]
        Pus20,
        AdHoc,
        #[serde(rename = "CH1")]
        Ch1,
        #[serde(rename = "DSM05")]
        Dsm05,
        #[serde(rename = "DUB07")]
        Dub07,
        #[serde(rename = "PNQ01")]
        Pnq01,
        #[serde(rename = "SVG20")]
        Svg20,
        #[serde(rename = "OSA02")]
        Osa02,
        #[serde(rename = "OSA22")]
        Osa22,
        #[serde(rename = "PAR22")]
        Par22,
        #[serde(rename = "BN7")]
        Bn7,
        #[serde(rename = "SN6")]
        Sn6,
        #[serde(rename = "BJS20")]
        Bjs20,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataCenterCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataCenterCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataCenterCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("DataCenterCode", 0u32, "Invalid"),
                Self::By2 => serializer.serialize_unit_variant("DataCenterCode", 1u32, "BY2"),
                Self::By1 => serializer.serialize_unit_variant("DataCenterCode", 2u32, "BY1"),
                Self::Ork70 => serializer.serialize_unit_variant("DataCenterCode", 3u32, "ORK70"),
                Self::Am2 => serializer.serialize_unit_variant("DataCenterCode", 4u32, "AM2"),
                Self::Ams20 => serializer.serialize_unit_variant("DataCenterCode", 5u32, "AMS20"),
                Self::By21 => serializer.serialize_unit_variant("DataCenterCode", 6u32, "BY21"),
                Self::By24 => serializer.serialize_unit_variant("DataCenterCode", 7u32, "BY24"),
                Self::Mwh01 => serializer.serialize_unit_variant("DataCenterCode", 8u32, "MWH01"),
                Self::Ams06 => serializer.serialize_unit_variant("DataCenterCode", 9u32, "AMS06"),
                Self::Sse90 => serializer.serialize_unit_variant("DataCenterCode", 10u32, "SSE90"),
                Self::Syd03 => serializer.serialize_unit_variant("DataCenterCode", 11u32, "SYD03"),
                Self::Syd23 => serializer.serialize_unit_variant("DataCenterCode", 12u32, "SYD23"),
                Self::Cbr20 => serializer.serialize_unit_variant("DataCenterCode", 13u32, "CBR20"),
                Self::Yto20 => serializer.serialize_unit_variant("DataCenterCode", 14u32, "YTO20"),
                Self::Cwl20 => serializer.serialize_unit_variant("DataCenterCode", 15u32, "CWL20"),
                Self::Lon24 => serializer.serialize_unit_variant("DataCenterCode", 16u32, "LON24"),
                Self::Bom01 => serializer.serialize_unit_variant("DataCenterCode", 17u32, "BOM01"),
                Self::Bl20 => serializer.serialize_unit_variant("DataCenterCode", 18u32, "BL20"),
                Self::Bl7 => serializer.serialize_unit_variant("DataCenterCode", 19u32, "BL7"),
                Self::Sel20 => serializer.serialize_unit_variant("DataCenterCode", 20u32, "SEL20"),
                Self::Tyo01 => serializer.serialize_unit_variant("DataCenterCode", 21u32, "TYO01"),
                Self::Bn1 => serializer.serialize_unit_variant("DataCenterCode", 22u32, "BN1"),
                Self::Sn5 => serializer.serialize_unit_variant("DataCenterCode", 23u32, "SN5"),
                Self::Cys04 => serializer.serialize_unit_variant("DataCenterCode", 24u32, "CYS04"),
                Self::Tyo22 => serializer.serialize_unit_variant("DataCenterCode", 25u32, "TYO22"),
                Self::Yto21 => serializer.serialize_unit_variant("DataCenterCode", 26u32, "YTO21"),
                Self::Yqb20 => serializer.serialize_unit_variant("DataCenterCode", 27u32, "YQB20"),
                Self::Fra22 => serializer.serialize_unit_variant("DataCenterCode", 28u32, "FRA22"),
                Self::Maa01 => serializer.serialize_unit_variant("DataCenterCode", 29u32, "MAA01"),
                Self::Cpq02 => serializer.serialize_unit_variant("DataCenterCode", 30u32, "CPQ02"),
                Self::Cpq20 => serializer.serialize_unit_variant("DataCenterCode", 31u32, "CPQ20"),
                Self::Sin20 => serializer.serialize_unit_variant("DataCenterCode", 32u32, "SIN20"),
                Self::Hkg20 => serializer.serialize_unit_variant("DataCenterCode", 33u32, "HKG20"),
                Self::Sg2 => serializer.serialize_unit_variant("DataCenterCode", 34u32, "SG2"),
                Self::Mel23 => serializer.serialize_unit_variant("DataCenterCode", 35u32, "MEL23"),
                Self::Sel21 => serializer.serialize_unit_variant("DataCenterCode", 36u32, "SEL21"),
                Self::Osa20 => serializer.serialize_unit_variant("DataCenterCode", 37u32, "OSA20"),
                Self::Sha03 => serializer.serialize_unit_variant("DataCenterCode", 38u32, "SHA03"),
                Self::Bjb => serializer.serialize_unit_variant("DataCenterCode", 39u32, "BJB"),
                Self::Jnb22 => serializer.serialize_unit_variant("DataCenterCode", 40u32, "JNB22"),
                Self::Jnb21 => serializer.serialize_unit_variant("DataCenterCode", 41u32, "JNB21"),
                Self::Mnz21 => serializer.serialize_unit_variant("DataCenterCode", 42u32, "MNZ21"),
                Self::Sn8 => serializer.serialize_unit_variant("DataCenterCode", 43u32, "SN8"),
                Self::Auh20 => serializer.serialize_unit_variant("DataCenterCode", 44u32, "AUH20"),
                Self::Zrh20 => serializer.serialize_unit_variant("DataCenterCode", 45u32, "ZRH20"),
                Self::Pus20 => serializer.serialize_unit_variant("DataCenterCode", 46u32, "PUS20"),
                Self::AdHoc => serializer.serialize_unit_variant("DataCenterCode", 47u32, "AdHoc"),
                Self::Ch1 => serializer.serialize_unit_variant("DataCenterCode", 48u32, "CH1"),
                Self::Dsm05 => serializer.serialize_unit_variant("DataCenterCode", 49u32, "DSM05"),
                Self::Dub07 => serializer.serialize_unit_variant("DataCenterCode", 50u32, "DUB07"),
                Self::Pnq01 => serializer.serialize_unit_variant("DataCenterCode", 51u32, "PNQ01"),
                Self::Svg20 => serializer.serialize_unit_variant("DataCenterCode", 52u32, "SVG20"),
                Self::Osa02 => serializer.serialize_unit_variant("DataCenterCode", 53u32, "OSA02"),
                Self::Osa22 => serializer.serialize_unit_variant("DataCenterCode", 54u32, "OSA22"),
                Self::Par22 => serializer.serialize_unit_variant("DataCenterCode", 55u32, "PAR22"),
                Self::Bn7 => serializer.serialize_unit_variant("DataCenterCode", 56u32, "BN7"),
                Self::Sn6 => serializer.serialize_unit_variant("DataCenterCode", 57u32, "SN6"),
                Self::Bjs20 => serializer.serialize_unit_variant("DataCenterCode", 58u32, "BJS20"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
    #[doc = "Provides additional information about an http error response."]
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
    #[serde(remote = "Status")]
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
        Created,
        #[serde(rename = "ShippedToAzureDC")]
        ShippedToAzureDc,
        AwaitingShipmentDetails,
        #[serde(rename = "PreparingToShipFromAzureDC")]
        PreparingToShipFromAzureDc,
        ShippedToCustomer,
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
                Self::DeviceOrdered => serializer.serialize_unit_variant("Status", 0u32, "DeviceOrdered"),
                Self::DevicePrepared => serializer.serialize_unit_variant("Status", 1u32, "DevicePrepared"),
                Self::Dispatched => serializer.serialize_unit_variant("Status", 2u32, "Dispatched"),
                Self::Delivered => serializer.serialize_unit_variant("Status", 3u32, "Delivered"),
                Self::PickedUp => serializer.serialize_unit_variant("Status", 4u32, "PickedUp"),
                Self::AtAzureDc => serializer.serialize_unit_variant("Status", 5u32, "AtAzureDC"),
                Self::DataCopy => serializer.serialize_unit_variant("Status", 6u32, "DataCopy"),
                Self::Completed => serializer.serialize_unit_variant("Status", 7u32, "Completed"),
                Self::CompletedWithErrors => serializer.serialize_unit_variant("Status", 8u32, "CompletedWithErrors"),
                Self::Cancelled => serializer.serialize_unit_variant("Status", 9u32, "Cancelled"),
                Self::FailedIssueReportedAtCustomer => serializer.serialize_unit_variant("Status", 10u32, "Failed_IssueReportedAtCustomer"),
                Self::FailedIssueDetectedAtAzureDc => serializer.serialize_unit_variant("Status", 11u32, "Failed_IssueDetectedAtAzureDC"),
                Self::Aborted => serializer.serialize_unit_variant("Status", 12u32, "Aborted"),
                Self::CompletedWithWarnings => serializer.serialize_unit_variant("Status", 13u32, "CompletedWithWarnings"),
                Self::ReadyToDispatchFromAzureDc => serializer.serialize_unit_variant("Status", 14u32, "ReadyToDispatchFromAzureDC"),
                Self::ReadyToReceiveAtAzureDc => serializer.serialize_unit_variant("Status", 15u32, "ReadyToReceiveAtAzureDC"),
                Self::Created => serializer.serialize_unit_variant("Status", 16u32, "Created"),
                Self::ShippedToAzureDc => serializer.serialize_unit_variant("Status", 17u32, "ShippedToAzureDC"),
                Self::AwaitingShipmentDetails => serializer.serialize_unit_variant("Status", 18u32, "AwaitingShipmentDetails"),
                Self::PreparingToShipFromAzureDc => serializer.serialize_unit_variant("Status", 19u32, "PreparingToShipFromAzureDC"),
                Self::ShippedToCustomer => serializer.serialize_unit_variant("Status", 20u32, "ShippedToCustomer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
    #[doc = "Provides additional information about an http error response."]
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
        DataBoxCustomerDisk,
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
    #[serde(remote = "StageName")]
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
        Created,
        #[serde(rename = "ShippedToAzureDC")]
        ShippedToAzureDc,
        AwaitingShipmentDetails,
        #[serde(rename = "PreparingToShipFromAzureDC")]
        PreparingToShipFromAzureDc,
        ShippedToCustomer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StageName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StageName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StageName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::DeviceOrdered => serializer.serialize_unit_variant("StageName", 0u32, "DeviceOrdered"),
                Self::DevicePrepared => serializer.serialize_unit_variant("StageName", 1u32, "DevicePrepared"),
                Self::Dispatched => serializer.serialize_unit_variant("StageName", 2u32, "Dispatched"),
                Self::Delivered => serializer.serialize_unit_variant("StageName", 3u32, "Delivered"),
                Self::PickedUp => serializer.serialize_unit_variant("StageName", 4u32, "PickedUp"),
                Self::AtAzureDc => serializer.serialize_unit_variant("StageName", 5u32, "AtAzureDC"),
                Self::DataCopy => serializer.serialize_unit_variant("StageName", 6u32, "DataCopy"),
                Self::Completed => serializer.serialize_unit_variant("StageName", 7u32, "Completed"),
                Self::CompletedWithErrors => serializer.serialize_unit_variant("StageName", 8u32, "CompletedWithErrors"),
                Self::Cancelled => serializer.serialize_unit_variant("StageName", 9u32, "Cancelled"),
                Self::FailedIssueReportedAtCustomer => {
                    serializer.serialize_unit_variant("StageName", 10u32, "Failed_IssueReportedAtCustomer")
                }
                Self::FailedIssueDetectedAtAzureDc => {
                    serializer.serialize_unit_variant("StageName", 11u32, "Failed_IssueDetectedAtAzureDC")
                }
                Self::Aborted => serializer.serialize_unit_variant("StageName", 12u32, "Aborted"),
                Self::CompletedWithWarnings => serializer.serialize_unit_variant("StageName", 13u32, "CompletedWithWarnings"),
                Self::ReadyToDispatchFromAzureDc => serializer.serialize_unit_variant("StageName", 14u32, "ReadyToDispatchFromAzureDC"),
                Self::ReadyToReceiveAtAzureDc => serializer.serialize_unit_variant("StageName", 15u32, "ReadyToReceiveAtAzureDC"),
                Self::Created => serializer.serialize_unit_variant("StageName", 16u32, "Created"),
                Self::ShippedToAzureDc => serializer.serialize_unit_variant("StageName", 17u32, "ShippedToAzureDC"),
                Self::AwaitingShipmentDetails => serializer.serialize_unit_variant("StageName", 18u32, "AwaitingShipmentDetails"),
                Self::PreparingToShipFromAzureDc => serializer.serialize_unit_variant("StageName", 19u32, "PreparingToShipFromAzureDC"),
                Self::ShippedToCustomer => serializer.serialize_unit_variant("StageName", 20u32, "ShippedToCustomer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
        CustomerActionPerformed,
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
        Restart,
        ReachOutToOperation,
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
#[doc = "The request body to provide the delivery package details of job"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkDevicesShippedRequest {
    #[doc = "package carrier info"]
    #[serde(rename = "deliverToDcPackageDetails")]
    pub deliver_to_dc_package_details: PackageCarrierInfo,
}
impl MarkDevicesShippedRequest {
    pub fn new(deliver_to_dc_package_details: PackageCarrierInfo) -> Self {
        Self {
            deliver_to_dc_package_details,
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
        Restart,
        ReachOutToOperation,
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
    #[serde(remote = "StageName")]
    pub enum StageName {
        DevicePrepared,
        Dispatched,
        Delivered,
        PickedUp,
        #[serde(rename = "AtAzureDC")]
        AtAzureDc,
        DataCopy,
        Created,
        ShippedToCustomer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StageName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StageName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StageName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::DevicePrepared => serializer.serialize_unit_variant("StageName", 0u32, "DevicePrepared"),
                Self::Dispatched => serializer.serialize_unit_variant("StageName", 1u32, "Dispatched"),
                Self::Delivered => serializer.serialize_unit_variant("StageName", 2u32, "Delivered"),
                Self::PickedUp => serializer.serialize_unit_variant("StageName", 3u32, "PickedUp"),
                Self::AtAzureDc => serializer.serialize_unit_variant("StageName", 4u32, "AtAzureDC"),
                Self::DataCopy => serializer.serialize_unit_variant("StageName", 5u32, "DataCopy"),
                Self::Created => serializer.serialize_unit_variant("StageName", 6u32, "Created"),
                Self::ShippedToCustomer => serializer.serialize_unit_variant("StageName", 7u32, "ShippedToCustomer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
#[doc = "Package carrier details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageCarrierDetails {
    #[doc = "Carrier Account Number of customer for customer disk."]
    #[serde(rename = "carrierAccountNumber", default, skip_serializing_if = "Option::is_none")]
    pub carrier_account_number: Option<String>,
    #[doc = "Name of the carrier."]
    #[serde(rename = "carrierName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<String>,
    #[doc = "Tracking Id of shipment."]
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
}
impl PackageCarrierDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "package carrier info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageCarrierInfo {
    #[doc = "Name of the carrier."]
    #[serde(rename = "carrierName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<String>,
    #[doc = "Tracking Id of shipment."]
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
}
impl PackageCarrierInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "package shipping details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PackageShippingDetails {
    #[doc = "Url where shipment can be tracked."]
    #[serde(rename = "trackingUrl", default, skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
    #[doc = "Name of the carrier."]
    #[serde(rename = "carrierName", default, skip_serializing_if = "Option::is_none")]
    pub carrier_name: Option<String>,
    #[doc = "Tracking Id of shipment."]
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
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
    #[doc = "Preferences related to the Access Tier of storage accounts."]
    #[serde(rename = "storageAccountAccessTierPreferences", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_account_access_tier_preferences: Vec<String>,
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
        DataBoxCustomerDisk,
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
    #[doc = "Request body to get the datacenter address."]
    #[serde(rename = "datacenterAddressRequest", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_address_request: Option<DatacenterAddressRequest>,
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
    #[doc = "Datacenter address for given storage location."]
    #[serde(rename = "datacenterAddressResponse", default, skip_serializing_if = "Option::is_none")]
    pub datacenter_address_response: Option<DatacenterAddressResponse>,
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
        DataBoxCustomerDisk,
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
        DataBoxCustomerDisk,
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
        DataBoxCustomerDisk,
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
        DataBoxCustomerDisk,
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
    #[doc = "Package carrier details."]
    #[serde(rename = "returnToCustomerPackageDetails", default, skip_serializing_if = "Option::is_none")]
    pub return_to_customer_package_details: Option<PackageCarrierDetails>,
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
        DataBoxCustomerDisk,
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
    #[doc = "Provides additional information about an http error response."]
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
