use crate::blob::blob::generate_blob_uri;
use crate::core::prelude::*;
use crate::core::{shared_access_signature::SharedAccessSignature, No, ToAssign, Yes};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct SignedUrlBuilder<'a, C, ContainerNameSet, BlobNameSet, SignatureSet>
where
    C: Client,
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_signature: PhantomData<SignatureSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    signature: Option<&'a SharedAccessSignature>,
}

impl<'a, C> SignedUrlBuilder<'a, C, No, No, No>
where
    C: Client,
{
    pub fn new(client: &'a C) -> Self {
        Self {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_signature: PhantomData {},
            signature: None,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SignatureSet>
    SignedUrlBuilder<'a, C, ContainerNameSet, BlobNameSet, SignatureSet>
where
    C: Client,
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SignatureSet: ToAssign,
{
    pub fn with_container_name(
        self,
        container_name: &'a str,
    ) -> SignedUrlBuilder<'a, C, Yes, BlobNameSet, SignatureSet> {
        SignedUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_signature: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            signature: self.signature,
        }
    }

    pub fn with_blob_name(
        self,
        blob_name: &'a str,
    ) -> SignedUrlBuilder<'a, C, ContainerNameSet, Yes, SignatureSet> {
        SignedUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_signature: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            signature: self.signature,
        }
    }

    pub fn with_shared_access_signature(
        self,
        signature: &'a SharedAccessSignature,
    ) -> SignedUrlBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes> {
        SignedUrlBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_signature: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            signature: Some(signature),
        }
    }
}

impl<'a, C> SignedUrlBuilder<'a, C, Yes, Yes, Yes>
where
    C: Client,
{
    #[inline]
    pub fn finalize(self) -> String {
        // the following unwraps
        // are statically guaranteed to be Some
        // because of the type signature.
        // Unfortunately
        // Rust cannot reason about this on its own.
        generate_blob_uri(
            self.client,
            self.container_name.unwrap(),
            self.blob_name.unwrap(),
            Some(&self.signature.unwrap().token()),
        )
    }
}
