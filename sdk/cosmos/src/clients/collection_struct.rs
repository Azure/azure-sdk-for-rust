use crate::clients::*;
use crate::requests;
use crate::{
    CollectionClient, CosmosClient, DatabaseClient, HasCosmosClient, HasDatabaseClient,
    HasHttpClient, IntoDocumentClient, IntoStoredProcedureClient, IntoTriggerClient,
    IntoUserDefinedFunctionClient, PartitionKeys, UserDefinedFunctionStruct, WithDocumentClient,
    WithStoredProcedureClient, WithTriggerClient, WithUserDefinedFunctionClient,
};
use azure_core::{HttpClient, No};
use std::borrow::Cow;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    database_client: Cow<'a, D>,
    collection_name: Cow<'a, str>,
    p_c: PhantomData<C>,
}

impl<'a, C, D> CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    #[inline]
    pub(crate) fn new(database_client: Cow<'a, D>, collection_name: Cow<'a, str>) -> Self {
        Self {
            database_client,
            collection_name,
            p_c: PhantomData {},
        }
    }
}

impl<'a, C, D> HasHttpClient for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    #[inline]
    fn http_client(&self) -> &dyn HttpClient {
        self.cosmos_client().http_client()
    }
}

impl<'a, C, D> HasCosmosClient<C> for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        self.database_client.cosmos_client()
    }
}

impl<'a, C, D> HasDatabaseClient<C, D> for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    #[inline]
    fn database_client(&self) -> &D {
        &self.database_client
    }
}

impl<'a, C, D> CollectionClient<C, D> for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    #[inline]
    fn collection_name(&self) -> &str {
        &self.collection_name
    }

    //fn get_collection(&self) -> requests::GetCollectionBuilder<'_, C, D> {
    //    requests::GetCollectionBuilder::new(self)
    //}

    //fn delete_collection(&self) -> requests::DeleteCollectionBuilder<'_, C, D> {
    //    requests::DeleteCollectionBuilder::new(self)
    //}

    //fn replace_collection(&self) -> requests::ReplaceCollectionBuilder<'_, '_, C, D, No, No> {
    //    requests::ReplaceCollectionBuilder::new(self)
    //}

    //fn list_documents(&self) -> requests::ListDocumentsBuilder<'_, '_, C, D> {
    //    requests::ListDocumentsBuilder::new(self)
    //}

    //fn create_document(&self) -> requests::CreateDocumentBuilder<'_, '_, C, D, No> {
    //    requests::CreateDocumentBuilder::new(self)
    //}

    //fn replace_document(&self) -> requests::ReplaceDocumentBuilder<'_, '_, C, D, No, No> {
    //    requests::ReplaceDocumentBuilder::new(self)
    //}

    //fn query_documents(&self) -> requests::QueryDocumentsBuilder<'_, '_, C, D, No> {
    //    requests::QueryDocumentsBuilder::new(self)
    //}

    //fn list_stored_procedures(&self) -> requests::ListStoredProceduresBuilder<'_, '_, C, D> {
    //    requests::ListStoredProceduresBuilder::new(self)
    //}

    //fn list_user_defined_functions(
    //    &self,
    //) -> requests::ListUserDefinedFunctionsBuilder<'_, '_, C, D> {
    //    requests::ListUserDefinedFunctionsBuilder::new(self)
    //}

    //fn list_triggers(&self) -> requests::ListTriggersBuilder<'_, '_, C, D> {
    //    requests::ListTriggersBuilder::new(self)
    //}

    //fn get_partition_key_ranges(&self) -> requests::GetPartitionKeyRangesBuilder<'_, '_, C, D> {
    //    requests::GetPartitionKeyRangesBuilder::new(self)
    //}
}

impl<'a, 'b, C, D> IntoDocumentClient<'b, C, D, Self, DocumentStruct<'a, 'b, C, D, Self>>
    for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn into_document_client<DocName>(
        self,
        document_name: DocName,
        partition_keys: PartitionKeys,
    ) -> DocumentStruct<'a, 'b, C, D, Self>
    where
        DocName: Into<Cow<'b, str>>,
    {
        DocumentStruct::new(Cow::Owned(self), document_name.into(), partition_keys)
    }
}

impl<'a, 'b, C, D> WithDocumentClient<'a, 'b, C, D, Self, DocumentStruct<'a, 'b, C, D, Self>>
    for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn with_document_client<DocName>(
        &'a self,
        document_name: DocName,
        partition_keys: PartitionKeys,
    ) -> DocumentStruct<'a, 'b, C, D, Self>
    where
        DocName: Into<Cow<'b, str>>,
    {
        DocumentStruct::new(Cow::Borrowed(self), document_name.into(), partition_keys)
    }
}

impl<'a, C, D> WithTriggerClient<'a, C, D, Self, TriggerStruct<'a, C, D, Self>>
    for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn with_trigger_client<IntoCowStr>(
        &'a self,
        trigger_name: IntoCowStr,
    ) -> TriggerStruct<'a, C, D, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        TriggerStruct::new(Cow::Borrowed(self), trigger_name.into())
    }
}

impl<'a, C, D> IntoTriggerClient<'a, C, D, Self, TriggerStruct<'a, C, D, Self>>
    for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn into_trigger_client<IntoCowStr>(
        self,
        trigger_name: IntoCowStr,
    ) -> TriggerStruct<'a, C, D, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        TriggerStruct::new(Cow::Owned(self), trigger_name.into())
    }
}

impl<'a, C, D>
    WithUserDefinedFunctionClient<'a, C, D, Self, UserDefinedFunctionStruct<'a, C, D, Self>>
    for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn with_user_defined_function_client<IntoCowStr>(
        &'a self,
        user_defined_function_name: IntoCowStr,
    ) -> UserDefinedFunctionStruct<'a, C, D, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        UserDefinedFunctionStruct::new(Cow::Borrowed(self), user_defined_function_name.into())
    }
}

impl<'a, C, D>
    IntoUserDefinedFunctionClient<'a, C, D, Self, UserDefinedFunctionStruct<'a, C, D, Self>>
    for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn into_user_defined_function_client<IntoCowStr>(
        self,
        user_defined_function_name: IntoCowStr,
    ) -> UserDefinedFunctionStruct<'a, C, D, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        UserDefinedFunctionStruct::new(Cow::Owned(self), user_defined_function_name.into())
    }
}

impl<'a, C, D> WithStoredProcedureClient<'a, C, D, Self, StoredProcedureStruct<'a, C, D, Self>>
    for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn with_stored_procedure_client<IntoCowStr>(
        &'a self,
        stored_procedure_name: IntoCowStr,
    ) -> StoredProcedureStruct<'a, C, D, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        StoredProcedureStruct::new(Cow::Borrowed(self), stored_procedure_name.into())
    }
}

impl<'a, C, D> IntoStoredProcedureClient<'a, C, D, Self, StoredProcedureStruct<'a, C, D, Self>>
    for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn into_stored_procedure_client<IntoCowStr>(
        self,
        stored_procedure_name: IntoCowStr,
    ) -> StoredProcedureStruct<'a, C, D, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        StoredProcedureStruct::new(Cow::Owned(self), stored_procedure_name.into())
    }
}
