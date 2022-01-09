use crate::clients::FileSystemClient;
use crate::operations::ListFileSystems;
use azure_core::Context;
use azure_storage::core::prelude::*;

#[derive(Debug, Clone)]
pub struct DataLakeClient {
    client: StorageAccountClient,
    context: Context,
}

impl DataLakeClient {
    pub fn new(client: StorageAccountClient) -> Self {
        let mut context = Context::new();
        context.insert(ServiceType::Blob);
        Self {
            client,
            context: context.clone(),
        }
    }

    pub fn new_with_credential<A>(account: A, storage_credentials: StorageCredentials) -> Self
    where
        A: Into<String>,
    {
        Self::new_with_options(
            account,
            storage_credentials,
            StorageAccountOptions::default(),
        )
    }

    pub(crate) fn new_with_options<A>(
        account: A,
        storage_credentials: StorageCredentials,
        options: StorageAccountOptions,
    ) -> Self
    where
        A: Into<String>,
    {
        let client = StorageAccountClient::new(account, storage_credentials, options);
        Self::new(client)
    }

    #[cfg(feature = "mock_transport_framework")]
    pub fn new_with_transaction<A, T>(
        account: A,
        storage_credentials: StorageCredentials,
        transaction_name: T,
    ) -> Self
    where
        A: Into<String>,
        T: Into<String>,
    {
        Self::new_with_options(
            account,
            storage_credentials,
            StorageAccountOptions::new_with_transaction_name(transaction_name.into()),
        )
    }

    pub fn list_file_systems(&self) -> ListFileSystems {
        ListFileSystems::new(self.client.clone(), Some(self.context.clone()))
    }

    pub fn into_file_system_client(self, file_system_name: String) -> FileSystemClient {
        FileSystemClient::new(self.client, file_system_name)
    }
}
