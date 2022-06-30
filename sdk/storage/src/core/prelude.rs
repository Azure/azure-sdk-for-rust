pub use crate::core::{
    clients::StorageClient,
    shared_access_signature::{
        account_sas::{AccountSasPermissions, AccountSasResource, AccountSasResourceType},
        service_sas::{BlobSasPermissions, BlobSignedResource},
        SasProtocol, SasToken,
    },
    {ConsistencyCRC64, ConsistencyMD5, CopyId, IPRange},
};
