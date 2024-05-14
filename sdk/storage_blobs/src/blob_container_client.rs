use std::marker::PhantomData;

use azure_core::{error::HttpError, Pageable, Url};

use crate::{units::*, BlobClient, BlobClientOptions};

pub struct BlobContainerClient<T>
where
    T: AccountStructure,
{
    account_structure: PhantomData<T>,
    endpoint: Url,
}

pub struct BlobItem {}

impl<T: AccountStructure> BlobContainerClient<T> {
    pub fn new(endpoint: Url, options: &BlobClientOptions) -> BlobContainerClient<Unset> {
        todo!()
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    // pub fn get_blob_client(&self, blob_name: &str) -> BlobClient<Unset, Unset> {
    //     todo!()
    // }

    pub fn get_blobs(&self) -> Pageable<BlobItem, HttpError> {
        todo!()
    }
}
