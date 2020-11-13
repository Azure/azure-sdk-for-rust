use crate::requests;
use crate::{
    AttachmentStruct, CollectionClient, CosmosClient, DatabaseClient, DocumentClient,
    HasCollectionClient, HasCosmosClient, HasDatabaseClient, HasHttpClient, IntoAttachmentClient,
    PartitionKeys, WithAttachmentClient,
};
use azure_core::HttpClient;
use std::borrow::Cow;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct DocumentStruct<'a, 'b, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    collection_client: Cow<'a, COLL>,
    document_name: Cow<'b, str>,
    partition_keys: PartitionKeys,
    p_c: PhantomData<C>,
    p_d: PhantomData<D>,
}

impl<'a, 'b, C, D, COLL> DocumentStruct<'a, 'b, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    pub(crate) fn new(
        collection_client: Cow<'a, COLL>,
        document_name: Cow<'b, str>,
        partition_keys: PartitionKeys,
    ) -> Self {
        Self {
            collection_client,
            document_name,
            partition_keys,
            p_c: PhantomData {},
            p_d: PhantomData {},
        }
    }
}

impl<'a, 'b, C, D, COLL> HasHttpClient for DocumentStruct<'a, 'b, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    #[inline]
    fn http_client(&self) -> &dyn HttpClient {
        self.collection_client().http_client()
    }
}

impl<'a, 'b, C, D, COLL> HasCosmosClient<C> for DocumentStruct<'a, 'b, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        self.collection_client().cosmos_client()
    }
}

impl<'a, 'b, C, D, COLL> HasDatabaseClient<C, D> for DocumentStruct<'a, 'b, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    #[inline]
    fn database_client(&self) -> &D {
        self.collection_client().database_client()
    }
}

impl<'a, 'b, C, D, COLL> HasCollectionClient<C, D, COLL> for DocumentStruct<'a, 'b, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    #[inline]
    fn collection_client(&self) -> &COLL {
        &self.collection_client
    }
}

impl<'a, 'b, C, D, COLL> DocumentClient<C, D, COLL> for DocumentStruct<'a, 'b, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    fn document_name(&self) -> &str {
        &self.document_name
    }

    fn partition_keys(&self) -> &PartitionKeys {
        &self.partition_keys
    }

    fn get_document(&self) -> requests::GetDocumentBuilder<'_, '_, C, D, COLL> {
        requests::GetDocumentBuilder::new(self)
    }

    fn delete_document(&self) -> requests::DeleteDocumentBuilder<'_, C, D, COLL> {
        requests::DeleteDocumentBuilder::new(self)
    }

    fn list_attachments(&self) -> requests::ListAttachmentsBuilder<'_, '_, C, D, COLL> {
        requests::ListAttachmentsBuilder::new(self)
    }
}

impl<'a, 'b, C, D, COLL>
    WithAttachmentClient<'a, C, D, COLL, Self, AttachmentStruct<'a, C, D, COLL, Self>>
    for DocumentStruct<'a, 'b, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    fn with_attachment_client<IntoCowStr>(
        &'a self,
        attachment_name: IntoCowStr,
    ) -> AttachmentStruct<'a, C, D, COLL, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        AttachmentStruct::new(Cow::Borrowed(self), attachment_name.into())
    }
}

impl<'a, 'b, C, D, COLL>
    IntoAttachmentClient<'a, C, D, COLL, Self, AttachmentStruct<'a, C, D, COLL, Self>>
    for DocumentStruct<'a, 'b, C, D, COLL>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
{
    fn into_attachment_client<IntoCowStr>(
        self,
        attachment_name: IntoCowStr,
    ) -> AttachmentStruct<'a, C, D, COLL, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        AttachmentStruct::new(Cow::Owned(self), attachment_name.into())
    }
}
