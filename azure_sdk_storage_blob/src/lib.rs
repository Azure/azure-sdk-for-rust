#![recursion_limit = "128"]
#![allow(clippy::needless_lifetimes)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_sdk_core;
pub mod blob;
pub mod container;
pub mod prelude;
use azure_sdk_core::No;
use azure_sdk_storage_core::Client;
use std::borrow::Borrow;

pub trait Blob<C>
where
    C: Client,
{
    fn get_blob<'a>(&'a self) -> blob::requests::GetBlobBuilder<'a, C, No, No>;
    fn put_block_blob<'a>(&'a self) -> blob::requests::PutBlockBlobBuilder<'a, C, No, No, No>;
    fn put_page_blob<'a>(&'a self) -> blob::requests::PutPageBlobBuilder<'a, C, No, No, No>;
    fn put_append_blob<'a>(&'a self) -> blob::requests::PutAppendBlobBuilder<'a, C, No, No>;
    fn put_append_block<'a>(&'a self) -> blob::requests::PutAppendBlockBuilder<'a, C, No, No, No>;
    fn update_page<'a>(&'a self) -> blob::requests::UpdatePageBuilder<'a, C, No, No, No, No>;
    fn clear_page<'a>(&'a self) -> blob::requests::ClearPageBuilder<'a, C, No, No, No>;
    fn put_block<'a>(&'a self) -> blob::requests::PutBlockBuilder<'a, C, No, No, No, No>;
    fn get_block_list<'a>(&'a self) -> blob::requests::GetBlockListBuilder<'a, C, No, No, No>;
    fn put_block_list<'a, T: Borrow<[u8]> + 'a>(
        &'a self,
    ) -> blob::requests::PutBlockListBuilder<'a, C, T, No, No, No>;
    fn acquire_blob_lease<'a>(
        &'a self,
    ) -> blob::requests::AcquireBlobLeaseBuilder<'a, C, No, No, No>;
    fn renew_blob_lease<'a>(&'a self) -> blob::requests::RenewBlobLeaseBuilder<'a, C, No, No, No>;
    fn change_blob_lease<'a>(
        &'a self,
    ) -> blob::requests::ChangeBlobLeaseBuilder<'a, C, No, No, No, No>;
    fn release_blob_lease<'a>(
        &'a self,
    ) -> blob::requests::ReleaseBlobLeaseBuilder<'a, C, No, No, No>;
    fn break_blob_lease<'a>(&'a self) -> blob::requests::BreakBlobLeaseBuilder<'a, C, No, No, No>;
    fn delete_blob_snapshot<'a>(
        &'a self,
    ) -> blob::requests::DeleteBlobSnapshotBuilder<'a, C, No, No, No>;
    fn delete_blob<'a>(&'a self) -> blob::requests::DeleteBlobBuilder<'a, C, No, No, No>;
    fn stream_blob<'a>(&'a self) -> blob::requests::BlobStreamBuilder<'a, C, No, No, No>;
    fn copy_blob_from_url<'a>(
        &'a self,
    ) -> blob::requests::CopyBlobFromUrlBuilder<'a, C, No, No, No>;
    fn generate_signed_blob_url<'a>(
        &'a self,
    ) -> blob::requests::SignedUrlBuilder<'a, C, No, No, No>;
}

pub trait Container<C>
where
    C: Client,
{
    fn create_container<'a>(&'a self) -> container::requests::CreateBuilder<'a, C, No, No>;
    fn delete_container<'a>(&'a self) -> container::requests::DeleteBuilder<'a, C, No>;
    fn list_blobs<'a>(&'a self) -> container::requests::ListBlobBuilder<'a, C, No>;
    fn list_containers<'a>(&'a self) -> container::requests::ListBuilder<'a, C>;
    fn get_container_acl<'a>(&'a self) -> container::requests::GetACLBuilder<'a, C, No>;
    fn set_container_acl<'a>(&'a self) -> container::requests::SetACLBuilder<'a, C, No, No>;
    fn get_container_properties<'a>(
        &'a self,
    ) -> container::requests::GetPropertiesBuilder<'a, C, No>;
    fn acquire_container_lease<'a>(
        &'a self,
    ) -> container::requests::AcquireLeaseBuilder<'a, C, No, No>;
    fn renew_container_lease<'a>(&'a self)
        -> container::requests::RenewLeaseBuilder<'a, C, No, No>;
    fn release_container_lease<'a>(
        &'a self,
    ) -> container::requests::ReleaseLeaseBuilder<'a, C, No, No>;
    fn break_container_lease<'a>(&'a self) -> container::requests::BreakLeaseBuilder<'a, C, No>;
}

impl<C> Blob<C> for C
where
    C: Client,
{
    fn get_blob<'a>(&'a self) -> blob::requests::GetBlobBuilder<'a, C, No, No> {
        blob::requests::GetBlobBuilder::new(self)
    }

    fn put_block_blob<'a>(&'a self) -> blob::requests::PutBlockBlobBuilder<'a, C, No, No, No> {
        blob::requests::PutBlockBlobBuilder::new(self)
    }

    fn put_page_blob<'a>(&'a self) -> blob::requests::PutPageBlobBuilder<'a, C, No, No, No> {
        blob::requests::PutPageBlobBuilder::new(self)
    }

    fn put_append_blob<'a>(&'a self) -> blob::requests::PutAppendBlobBuilder<'a, C, No, No> {
        blob::requests::PutAppendBlobBuilder::new(self)
    }

    fn put_append_block<'a>(&'a self) -> blob::requests::PutAppendBlockBuilder<'a, C, No, No, No> {
        blob::requests::PutAppendBlockBuilder::new(self)
    }

    fn update_page<'a>(&'a self) -> blob::requests::UpdatePageBuilder<'a, C, No, No, No, No> {
        blob::requests::UpdatePageBuilder::new(self)
    }

    fn clear_page<'a>(&'a self) -> blob::requests::ClearPageBuilder<'a, C, No, No, No> {
        blob::requests::ClearPageBuilder::new(self)
    }

    fn put_block<'a>(&'a self) -> blob::requests::PutBlockBuilder<'a, C, No, No, No, No> {
        blob::requests::PutBlockBuilder::new(self)
    }

    fn get_block_list<'a>(&'a self) -> blob::requests::GetBlockListBuilder<'a, C, No, No, No> {
        blob::requests::GetBlockListBuilder::new(self)
    }

    fn put_block_list<'a, T: Borrow<[u8]> + 'a>(
        &'a self,
    ) -> blob::requests::PutBlockListBuilder<'a, C, T, No, No, No> {
        blob::requests::PutBlockListBuilder::new(self)
    }

    fn acquire_blob_lease<'a>(
        &'a self,
    ) -> blob::requests::AcquireBlobLeaseBuilder<'a, C, No, No, No> {
        blob::requests::AcquireBlobLeaseBuilder::new(self)
    }

    fn renew_blob_lease<'a>(&'a self) -> blob::requests::RenewBlobLeaseBuilder<'a, C, No, No, No> {
        blob::requests::RenewBlobLeaseBuilder::new(self)
    }

    fn change_blob_lease<'a>(
        &'a self,
    ) -> blob::requests::ChangeBlobLeaseBuilder<'a, C, No, No, No, No> {
        blob::requests::ChangeBlobLeaseBuilder::new(self)
    }

    fn release_blob_lease<'a>(
        &'a self,
    ) -> blob::requests::ReleaseBlobLeaseBuilder<'a, C, No, No, No> {
        blob::requests::ReleaseBlobLeaseBuilder::new(self)
    }

    fn break_blob_lease<'a>(&'a self) -> blob::requests::BreakBlobLeaseBuilder<'a, C, No, No, No> {
        blob::requests::BreakBlobLeaseBuilder::new(self)
    }

    fn delete_blob_snapshot<'a>(
        &'a self,
    ) -> blob::requests::DeleteBlobSnapshotBuilder<'a, C, No, No, No> {
        blob::requests::DeleteBlobSnapshotBuilder::new(self)
    }

    fn delete_blob<'a>(&'a self) -> blob::requests::DeleteBlobBuilder<'a, C, No, No, No> {
        blob::requests::DeleteBlobBuilder::new(self)
    }

    fn stream_blob<'a>(&'a self) -> blob::requests::BlobStreamBuilder<'a, C, No, No, No> {
        blob::requests::BlobStreamBuilder::new(self)
    }

    fn copy_blob_from_url<'a>(
        &'a self,
    ) -> blob::requests::CopyBlobFromUrlBuilder<'a, C, No, No, No> {
        blob::requests::CopyBlobFromUrlBuilder::new(self)
    }

    fn generate_signed_blob_url<'a>(
        &'a self,
    ) -> blob::requests::SignedUrlBuilder<'a, C, No, No, No> {
        blob::requests::SignedUrlBuilder::new(self)
    }
}

impl<C> Container<C> for C
where
    C: Client,
{
    fn list_blobs<'a>(&'a self) -> container::requests::ListBlobBuilder<'a, C, No> {
        container::requests::ListBlobBuilder::new(self)
    }

    fn create_container<'a>(&'a self) -> container::requests::CreateBuilder<'a, C, No, No> {
        container::requests::CreateBuilder::new(self)
    }

    fn delete_container<'a>(&'a self) -> container::requests::DeleteBuilder<'a, C, No> {
        container::requests::DeleteBuilder::new(self)
    }

    fn list_containers<'a>(&'a self) -> container::requests::ListBuilder<'a, C> {
        container::requests::ListBuilder::new(self)
    }

    fn get_container_acl<'a>(&'a self) -> container::requests::GetACLBuilder<'a, C, No> {
        container::requests::GetACLBuilder::new(self)
    }

    fn set_container_acl<'a>(&'a self) -> container::requests::SetACLBuilder<'a, C, No, No> {
        container::requests::SetACLBuilder::new(self)
    }

    fn get_container_properties<'a>(
        &'a self,
    ) -> container::requests::GetPropertiesBuilder<'a, C, No> {
        container::requests::GetPropertiesBuilder::new(self)
    }

    fn acquire_container_lease<'a>(
        &'a self,
    ) -> container::requests::AcquireLeaseBuilder<'a, C, No, No> {
        container::requests::AcquireLeaseBuilder::new(self)
    }

    fn renew_container_lease<'a>(
        &'a self,
    ) -> container::requests::RenewLeaseBuilder<'a, C, No, No> {
        container::requests::RenewLeaseBuilder::new(self)
    }

    fn release_container_lease<'a>(
        &'a self,
    ) -> container::requests::ReleaseLeaseBuilder<'a, C, No, No> {
        container::requests::ReleaseLeaseBuilder::new(self)
    }

    fn break_container_lease<'a>(&'a self) -> container::requests::BreakLeaseBuilder<'a, C, No> {
        container::requests::BreakLeaseBuilder::new(self)
    }
}
