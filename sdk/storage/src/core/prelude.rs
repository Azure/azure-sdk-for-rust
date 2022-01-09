pub use crate::core::{
    clients::{
        AsStorageClient, ServiceType, StorageAccountClient, StorageAccountOptions, StorageClient,
        StorageCredentials,
    },
    shared_access_signature::{
        account_sas::{
            AccountSasPermissions, AccountSasResource, AccountSasResourceType,
            ClientAccountSharedAccessSignature, SasExpirySupport, SasPermissionsSupport,
            SasProtocolSupport, SasResourceSupport, SasResourceTypeSupport, SasStartSupport,
        },
        service_sas::{BlobSasPermissions, BlobSignedResource},
        SasProtocol, SasToken,
    },
    {ConsistencyCRC64, ConsistencyMD5, CopyId, IPRange},
};
