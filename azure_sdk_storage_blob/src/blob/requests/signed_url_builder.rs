use crate::blob::generate_blob_uri;
use azure_sdk_core::{
    BlobNameRequired, BlobNameSupport, ContainerNameRequired, ContainerNameSupport, No, ToAssign,
    Yes,
};
use azure_sdk_storage_core::prelude::*;
use azure_sdk_storage_core::{
    shared_access_signature::SharedAccessSignature, SharedAccessSignatureRequired,
    SharedAccessSignatureSupport,
};
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

impl<'a, C, ContainerNameSet, BlobNameSet, SignatureSet> ClientRequired<'a, C>
    for SignedUrlBuilder<'a, C, ContainerNameSet, BlobNameSet, SignatureSet>
where
    C: Client,
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SignatureSet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a C {
        &self.client
    }
}

impl<'a, C, BlobNameSet, SignatureSet> ContainerNameRequired<'a>
    for SignedUrlBuilder<'a, C, Yes, BlobNameSet, SignatureSet>
where
    C: Client,
    BlobNameSet: ToAssign,
    SignatureSet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, SignatureSet> BlobNameRequired<'a>
    for SignedUrlBuilder<'a, C, ContainerNameSet, Yes, SignatureSet>
where
    C: Client,
    ContainerNameSet: ToAssign,
    SignatureSet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> SharedAccessSignatureRequired<'a>
    for SignedUrlBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>
where
    C: Client,
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn shared_access_signature(&self) -> &'a SharedAccessSignature {
        self.signature.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SignatureSet> ContainerNameSupport<'a>
    for SignedUrlBuilder<'a, C, ContainerNameSet, BlobNameSet, SignatureSet>
where
    C: Client,
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SignatureSet: ToAssign,
{
    type O = SignedUrlBuilder<'a, C, Yes, BlobNameSet, SignatureSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
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
}

impl<'a, C, ContainerNameSet, BlobNameSet, SignatureSet> BlobNameSupport<'a>
    for SignedUrlBuilder<'a, C, ContainerNameSet, BlobNameSet, SignatureSet>
where
    C: Client,
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SignatureSet: ToAssign,
{
    type O = SignedUrlBuilder<'a, C, ContainerNameSet, Yes, SignatureSet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
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
}

impl<'a, C, ContainerNameSet, BlobNameSet, SignatureSet> SharedAccessSignatureSupport<'a>
    for SignedUrlBuilder<'a, C, ContainerNameSet, BlobNameSet, SignatureSet>
where
    C: Client,
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SignatureSet: ToAssign,
{
    type O = SignedUrlBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_shared_access_signature(self, signature: &'a SharedAccessSignature) -> Self::O {
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
        generate_blob_uri(
            self.client(),
            self.container_name(),
            self.blob_name(),
            Some(&self.signature.unwrap().token()),
        )
    }
}
