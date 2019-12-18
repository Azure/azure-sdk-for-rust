use crate::blob::generate_blob_uri;
use azure_sdk_core::{
    BlobNameRequired, BlobNameSupport, ContainerNameRequired, ContainerNameSupport, No, ToAssign,
    Yes,
};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::{
    shared_access_signature::SharedAccessSignature, ClientRequired, SharedAccessSignatureRequired,
    SharedAccessSignatureSupport,
};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct SignedUrlBuilder<'a, ContainerNameSet, BlobNameSet, SignatureSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_signature: PhantomData<SignatureSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    signature: Option<&'a SharedAccessSignature>,
}

impl<'a> SignedUrlBuilder<'a, No, No, No> {
    pub fn new(client: &'a Client) -> SignedUrlBuilder<'a, No, No, No> {
        SignedUrlBuilder {
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

impl<'a, ContainerNameSet, BlobNameSet, SignatureSet> ClientRequired<'a>
    for SignedUrlBuilder<'a, ContainerNameSet, BlobNameSet, SignatureSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SignatureSet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet, SignatureSet> ContainerNameRequired<'a>
    for SignedUrlBuilder<'a, Yes, BlobNameSet, SignatureSet>
where
    BlobNameSet: ToAssign,
    SignatureSet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, SignatureSet> BlobNameRequired<'a>
    for SignedUrlBuilder<'a, ContainerNameSet, Yes, SignatureSet>
where
    ContainerNameSet: ToAssign,
    SignatureSet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> SharedAccessSignatureRequired<'a>
    for SignedUrlBuilder<'a, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn shared_access_signature(&self) -> &'a SharedAccessSignature {
        self.signature.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SignatureSet> ContainerNameSupport<'a>
    for SignedUrlBuilder<'a, ContainerNameSet, BlobNameSet, SignatureSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SignatureSet: ToAssign,
{
    type O = SignedUrlBuilder<'a, Yes, BlobNameSet, SignatureSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, SignatureSet> BlobNameSupport<'a>
    for SignedUrlBuilder<'a, ContainerNameSet, BlobNameSet, SignatureSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SignatureSet: ToAssign,
{
    type O = SignedUrlBuilder<'a, ContainerNameSet, Yes, SignatureSet>;

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

impl<'a, ContainerNameSet, BlobNameSet, SignatureSet> SharedAccessSignatureSupport<'a>
    for SignedUrlBuilder<'a, ContainerNameSet, BlobNameSet, SignatureSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SignatureSet: ToAssign,
{
    type O = SignedUrlBuilder<'a, ContainerNameSet, BlobNameSet, Yes>;

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

impl<'a> SignedUrlBuilder<'a, Yes, Yes, Yes> {
    #[inline]
    pub fn finalize(self) -> String {
        generate_blob_uri(&self, Some(&self.signature.unwrap().token()))
    }
}
