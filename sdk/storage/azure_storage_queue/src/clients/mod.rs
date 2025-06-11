mod queue_client;

pub use queue_client::QueueClient;

pub use crate::generated::clients::{
    AzureQueueStorageClient, AzureQueueStorageClientOptions,
    AzureQueueStorageMessageIdOperationsClient, AzureQueueStorageMessagesOperationsClient,
    AzureQueueStorageQueueOperationsClient, AzureQueueStorageServiceOperationsClient,
    AzureQueueStorageServicePropertiesOperationsClient,
};
