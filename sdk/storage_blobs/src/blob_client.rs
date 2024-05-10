use std::marker::PhantomData;

use azure_core::{Response, Url};

use crate::units::*;

pub struct BlobClient<T, U>
where
    T: AccountStructure,
    U: BlobKind,
{
    account_structure: PhantomData<T>,
    blob_type: PhantomData<U>,
    endpoint: Url,
}

impl<U: BlobKind> BlobClient<Unset, U> {
    pub fn using_flat_namespace(&self) -> BlobClient<Flat, U> {
        todo!()
    }

    pub fn using_hierarchichal_namespace(&self) -> BlobClient<Hierarchichal, U> {
        todo!()
    }
}

impl<T: AccountStructure> BlobClient<T, Unset> {
    pub fn as_block_blob(&self) -> BlobClient<T, Block> {
        todo!()
    }

    pub fn as_page_blob(&self) -> BlobClient<T, Page> {
        todo!()
    }

    pub fn as_append_blob(&self) -> BlobClient<T, Append> {
        todo!()
    }
}

impl<T: AccountStructure, U: BlobKind> BlobClient<T, U> {
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub fn download(&self) -> Response {
        todo!()
    }
}

impl<T: AccountStructure> BlobClient<T, Block> {
    pub fn put_block(&self) -> Response {
        todo!()
    }

    pub fn put_block_list(&self) -> Response {
        todo!()
    }
}
