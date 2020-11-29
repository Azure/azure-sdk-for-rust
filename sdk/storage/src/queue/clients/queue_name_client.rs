use crate::core::Client;
use crate::queue::clients::QueueServiceClient;
use crate::requests;
use crate::{HasStorageClient, IntoQueueNameClient, QueueNameService, WithQueueNameClient};
use azure_core::No;
use std::borrow::Cow;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct QueueNameClient<'a, 'b, C>
where
    C: Client + Clone,
{
    pub storage_client: Cow<'a, C>,
    pub queue_name: Cow<'b, str>,
}

impl<'a, 'b, C> HasStorageClient for QueueNameClient<'a, 'b, C>
where
    C: Client + Clone,
{
    type StorageClient = C;

    fn storage_client(&self) -> &C {
        self.storage_client.as_ref()
    }
}

impl<'a, 'b, C> WithQueueNameClient<'a, 'b> for QueueServiceClient<'a, C>
where
    C: Client + Clone,
{
    type QueueNameClient = QueueNameClient<'a, 'b, C>;

    fn with_queue_name_client<NAME>(&'a self, queue_name: NAME) -> Self::QueueNameClient
    where
        NAME: Into<Cow<'b, str>>,
    {
        QueueNameClient {
            storage_client: Cow::Borrowed(&self.storage_client),
            queue_name: queue_name.into(),
        }
    }
}

impl<'a, 'b, C> IntoQueueNameClient<'b> for QueueServiceClient<'a, C>
where
    C: Client + Clone,
{
    type QueueNameClient = QueueNameClient<'a, 'b, C>;

    fn into_queue_name_client<NAME>(self, queue_name: NAME) -> Self::QueueNameClient
    where
        NAME: Into<Cow<'b, str>>,
    {
        QueueNameClient {
            storage_client: Cow::Owned(self.storage_client.into_owned()),
            queue_name: queue_name.into(),
        }
    }
}

impl<'a, 'b, C> QueueNameService for QueueNameClient<'a, 'b, C>
where
    C: Client + Clone,
{
    fn queue_name(&self) -> &str {
        self.queue_name.as_ref()
    }
}

impl<'a, 'b, C> QueueNameClient<'a, 'b, C>
where
    C: Client + Clone,
{
    pub fn put_message(&self) -> requests::PutMessageBuilder<'_, '_, C, No> {
        requests::PutMessageBuilder::new(self)
    }

    pub fn get_messages(&self) -> requests::GetMessagesBuilder<'_, C> {
        requests::GetMessagesBuilder::new(self)
    }

    pub fn peek_messages(&self) -> requests::PeekMessagesBuilder<'_, C> {
        requests::PeekMessagesBuilder::new(self)
    }

    pub fn delete_message(&self) -> requests::DeleteMessageBuilder<'_, C, No> {
        requests::DeleteMessageBuilder::new(self)
    }

    pub fn clear_messages(&self) -> requests::ClearMessagesBuilder<'_, C> {
        requests::ClearMessagesBuilder::new(self)
    }
}
