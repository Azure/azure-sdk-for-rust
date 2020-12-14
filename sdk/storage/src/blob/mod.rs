pub mod blob;
pub mod container;
mod headers;
pub mod prelude;

use crate::core::Client;
use azure_core::No;
use http::request::Builder;
use std::borrow::Borrow;

create_enum!(RehydratePriority, (High, "High"), (Standard, "Standard"));

pub trait RehydratePrioritySupport {
    type O;
    fn with_rehydrate_priority(self, rehydrate_priority: RehydratePriority) -> Self::O;
}

pub trait RehydratePriorityOption {
    fn rehydrate_priority(&self) -> Option<RehydratePriority>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(rehydrate_priority) = self.rehydrate_priority() {
            builder = builder.header(headers::REHYDRATE_PRIORITY, rehydrate_priority.as_ref());
        }
        builder
    }
}

pub trait Blob<C>
where
    C: Client,
{
    fn get_blob<'a>(&'a self) -> blob::requests::GetBlobBuilder<'a, C, No, No>;
    fn get_blob_properties<'a>(&'a self)
        -> blob::requests::GetBlobPropertiesBuilder<'a, C, No, No>;
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
    fn copy_blob<'a>(&'a self) -> blob::requests::CopyBlobBuilder<'a, C, No, No, No>;
    fn generate_signed_blob_url<'a>(
        &'a self,
    ) -> blob::requests::SignedUrlBuilder<'a, C, No, No, No>;
}

impl<C> Blob<C> for C
where
    C: Client,
{
    fn get_blob<'a>(&'a self) -> blob::requests::GetBlobBuilder<'a, C, No, No> {
        blob::requests::GetBlobBuilder::new(self)
    }

    fn get_blob_properties<'a>(
        &'a self,
    ) -> blob::requests::GetBlobPropertiesBuilder<'a, C, No, No> {
        blob::requests::GetBlobPropertiesBuilder::new(self)
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

    fn copy_blob<'a>(&'a self) -> blob::requests::CopyBlobBuilder<'a, C, No, No, No> {
        blob::requests::CopyBlobBuilder::new(self)
    }

    fn generate_signed_blob_url<'a>(
        &'a self,
    ) -> blob::requests::SignedUrlBuilder<'a, C, No, No, No> {
        blob::requests::SignedUrlBuilder::new(self)
    }
}
