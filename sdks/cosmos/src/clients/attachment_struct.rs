use crate::requests;
use crate::traits::*;
use azure_core::No;
use std::borrow::Cow;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct AttachmentStruct<'a, C, D, COLL, DOC>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
    DOC: DocumentClient<C, D, COLL> + Clone,
{
    document_client: Cow<'a, DOC>,
    attachment_name: Cow<'a, str>,
    p_c: PhantomData<C>,
    p_d: PhantomData<D>,
    p_coll: PhantomData<COLL>,
}

impl<'a, C, D, COLL, DOC> AttachmentStruct<'a, C, D, COLL, DOC>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
    DOC: DocumentClient<C, D, COLL> + Clone,
{
    pub(crate) fn new(document_client: Cow<'a, DOC>, attachment_name: Cow<'a, str>) -> Self {
        Self {
            document_client,
            attachment_name,
            p_c: PhantomData {},
            p_d: PhantomData {},
            p_coll: PhantomData {},
        }
    }
}

impl<'a, C, D, COLL, DOC> HasHyperClient for AttachmentStruct<'a, C, D, COLL, DOC>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
    DOC: DocumentClient<C, D, COLL> + Clone,
{
    #[inline]
    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.document_client().hyper_client()
    }
}

impl<'a, C, D, COLL, DOC> HasCosmosClient<C> for AttachmentStruct<'a, C, D, COLL, DOC>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
    DOC: DocumentClient<C, D, COLL> + Clone,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        self.document_client().cosmos_client()
    }
}

impl<'a, C, D, COLL, DOC> HasDatabaseClient<C, D> for AttachmentStruct<'a, C, D, COLL, DOC>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
    DOC: DocumentClient<C, D, COLL> + Clone,
{
    #[inline]
    fn database_client(&self) -> &D {
        self.document_client().database_client()
    }
}

impl<'a, C, D, COLL, DOC> HasCollectionClient<C, D, COLL> for AttachmentStruct<'a, C, D, COLL, DOC>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
    DOC: DocumentClient<C, D, COLL> + Clone,
{
    #[inline]
    fn collection_client(&self) -> &COLL {
        self.document_client().collection_client()
    }
}

impl<'a, C, D, COLL, DOC> HasDocumentClient<C, D, COLL, DOC>
    for AttachmentStruct<'a, C, D, COLL, DOC>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
    DOC: DocumentClient<C, D, COLL> + Clone,
{
    #[inline]
    fn document_client(&self) -> &DOC {
        &self.document_client
    }
}

impl<'a, C, D, COLL, DOC> AttachmentClient<C, D, COLL, DOC>
    for AttachmentStruct<'a, C, D, COLL, DOC>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
    COLL: CollectionClient<C, D> + Clone,
    DOC: DocumentClient<C, D, COLL> + Clone,
{
    fn attachment_name(&self) -> &str {
        &self.attachment_name
    }

    fn create_slug(
        &self,
    ) -> requests::CreateSlugAttachmentBuilder<'_, '_, C, D, COLL, DOC, No, No> {
        requests::CreateSlugAttachmentBuilder::new(self)
    }

    fn replace_slug(
        &self,
    ) -> requests::ReplaceSlugAttachmentBuilder<'_, '_, C, D, COLL, DOC, No, No> {
        requests::ReplaceSlugAttachmentBuilder::new(self)
    }

    fn create_reference(
        &self,
    ) -> requests::CreateReferenceAttachmentBuilder<'_, '_, C, D, COLL, DOC, No, No> {
        requests::CreateReferenceAttachmentBuilder::new(self)
    }

    fn replace_reference(
        &self,
    ) -> requests::ReplaceReferenceAttachmentBuilder<'_, '_, C, D, COLL, DOC, No, No> {
        requests::ReplaceReferenceAttachmentBuilder::new(self)
    }

    fn delete(&self) -> requests::DeleteAttachmentBuilder<'_, '_, C, D, COLL, DOC> {
        requests::DeleteAttachmentBuilder::new(self)
    }

    fn get(&self) -> requests::GetAttachmentBuilder<'_, '_, C, D, COLL, DOC> {
        requests::GetAttachmentBuilder::new(self)
    }
}
