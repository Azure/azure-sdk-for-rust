mod queue_client;
mod queue_service_client;

pub use queue_client::QueueClient;
pub use queue_service_client::QueueServiceClient;

pub use crate::generated::clients::{
    AzureQueueStorageClient, AzureQueueStorageClientOptions,
    AzureQueueStorageMessageIdOperationsClient, AzureQueueStorageMessagesOperationsClient,
    AzureQueueStorageQueueOperationsClient, AzureQueueStorageServiceOperationsClient,
    AzureQueueStorageServicePropertiesOperationsClient,
};
