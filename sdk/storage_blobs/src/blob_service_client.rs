use std::marker::PhantomData;

use azure_core::{error::HttpError, Pageable, Url};

use crate::{units::*, BlobContainerClient};

pub struct BlobServiceClient<T>
where
    T: AccountStructure,
{
    account_structure: PhantomData<T>,
    endpoint: Url,
}

pub struct ContainerItem {}

impl<T: AccountStructure> BlobServiceClient<T> {
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub fn get_blob_conatiner_client(&self, container_name: &str) -> BlobContainerClient<Unset> {
        todo!()
    }

    pub fn get_containers(&self) -> Pageable<ContainerItem, HttpError> {
        todo!()
    }
}
