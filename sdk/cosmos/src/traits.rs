use crate::requests;
use crate::{PartitionKeys, ResourceType};
use azure_core::{HttpClient, No};
use http::request::Builder;
use std::borrow::Cow;
use std::fmt::Debug;

pub trait HasHttpClient: Debug + Send + Sync {
    fn http_client(&self) -> &dyn HttpClient;
}

pub trait CosmosClient: HasHttpClient + Send + Sync {
    //fn create_database(&self) -> requests::CreateDatabaseBuilder<'_, No>;
    //fn list_databases(&self) -> requests::ListDatabasesBuilder<'_>;

    fn prepare_request(
        &self,
        uri_path: &str,
        http_method: http::Method,
        resource_type: ResourceType,
    ) -> Builder;
}

pub trait HasCosmosClient<C>: HasHttpClient
where
    C: CosmosClient,
{
    fn cosmos_client(&self) -> &C;
}

pub trait DatabaseClient<C>: HasCosmosClient<C>
where
    C: CosmosClient,
{
    fn database_name(&self) -> &str;

    fn get_database(&self) -> requests::GetDatabaseBuilder<'_, '_, C>;
    //fn list_collections(&self) -> crate::requests::ListCollectionsBuilder<'_, C>;
    //fn create_collection(&self) -> requests::CreateCollectionBuilder<'_, C, No, No, No, No>;
    //fn delete_database(&self) -> requests::DeleteDatabaseBuilder<'_, C>;
    //fn list_users(&self) -> requests::ListUsersBuilder<'_, '_, C>;

    fn prepare_request(&self, method: http::Method) -> http::request::Builder {
        self.cosmos_client()
            .prepare_request("dbs", method, ResourceType::Databases)
    }
    fn prepare_request_with_database_name(&self, method: http::Method) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!("dbs/{}", self.database_name()),
            method,
            ResourceType::Databases,
        )
    }
}

pub trait HasDatabaseClient<C, D>: HasCosmosClient<C>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn database_client(&self) -> &D;
}

pub trait WithDatabaseClient<'a, C, D>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn with_database_client<IntoCowStr>(&'a self, database_name: IntoCowStr) -> D
    where
        IntoCowStr: Into<Cow<'a, str>>;
}

pub trait IntoDatabaseClient<'a, C, D>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn into_database_client<IntoCowStr>(self, database_name: IntoCowStr) -> D
    where
        IntoCowStr: Into<Cow<'a, str>>;
}

pub trait UserClient<C, D>: HasDatabaseClient<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn user_name(&self) -> &str;

    //fn create_user(&self) -> requests::CreateUserBuilder<'_, '_, C, D>;
    //fn delete_user(&self) -> requests::DeleteUserBuilder<'_, '_, C, D>;
    //fn get_user(&self) -> requests::GetUserBuilder<'_, '_, C, D>;
    //fn replace_user(&self) -> requests::ReplaceUserBuilder<'_, '_, C, D, No>;

    //fn list_permissions(&self) -> requests::ListPermissionsBuilder<'_, '_, C, D>;

    //fn prepare_request(&self, method: http::Method) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!("dbs/{}/users", self.database_client().database_name()),
    //        method,
    //        ResourceType::Users,
    //    )
    //}
    //fn prepare_request_with_user_name(&self, method: http::Method) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!(
    //            "dbs/{}/users/{}",
    //            self.database_client().database_name(),
    //            self.user_name()
    //        ),
    //        method,
    //        ResourceType::Users,
    //    )
    //}
}

pub trait HasUserClient<C, D, USER>: HasDatabaseClient<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    fn user_client(&self) -> &USER;
}

pub trait WithUserClient<'a, C, D, USER>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    fn with_user_client<IntoCowStr>(&'a self, user_name: IntoCowStr) -> USER
    where
        IntoCowStr: Into<Cow<'a, str>>;
}

pub trait IntoUserClient<'a, C, D, USER>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    fn into_user_client<IntoCowStr>(self, user_name: IntoCowStr) -> USER
    where
        IntoCowStr: Into<Cow<'a, str>>;
}

pub trait PermissionClient<C, D, USER>: HasUserClient<C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    fn permission_name(&self) -> &str;

    //fn create_permission(&self) -> requests::CreatePermissionBuilder<'_, '_, C, D, USER>;
    //fn get_permission(&self) -> requests::GetPermissionBuilder<'_, '_, C, D, USER>;
    //fn replace_permission(&self) -> requests::ReplacePermissionBuilder<'_, '_, C, D, USER>;
    //fn delete_permission(&self) -> requests::DeletePermissionsBuilder<'_, '_, C, D, USER>;

    //fn prepare_request(&self, method: http::Method) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!(
    //            "dbs/{}/users/{}/permissions",
    //            self.database_client().database_name(),
    //            self.user_client().user_name()
    //        ),
    //        method,
    //        ResourceType::Permissions,
    //    )
    //}
    //fn prepare_request_with_permission_name(&self, method: http::Method) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!(
    //            "dbs/{}/users/{}/permissions/{}",
    //            self.database_client().database_name(),
    //            self.user_client().user_name(),
    //            self.permission_name()
    //        ),
    //        method,
    //        ResourceType::Permissions,
    //    )
    //}
}

pub trait HasPermissionClient<C, D, USER, PERMISSION>: HasUserClient<C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
    PERMISSION: PermissionClient<C, D, USER>,
{
    fn permission_client(&self) -> &PERMISSION;
}

pub trait WithPermissionClient<'a, C, D, USER, PERMISSION>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
    PERMISSION: PermissionClient<C, D, USER>,
{
    fn with_permission_client<IntoCowStr>(&'a self, permission_name: IntoCowStr) -> PERMISSION
    where
        IntoCowStr: Into<Cow<'a, str>>;
}

pub trait IntoPermissionClient<'a, C, D, USER, PERMISSION>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
    PERMISSION: PermissionClient<C, D, USER>,
{
    fn into_permission_client<IntoCowStr>(self, permission_name: IntoCowStr) -> PERMISSION
    where
        IntoCowStr: Into<Cow<'a, str>>;
}

pub trait CollectionClient<C, D>: HasDatabaseClient<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn collection_name(&self) -> &str;

    //fn get_collection(&self) -> requests::GetCollectionBuilder<'_, C, D>;
    //fn delete_collection(&self) -> requests::DeleteCollectionBuilder<'_, C, D>;
    //fn replace_collection(&self) -> requests::ReplaceCollectionBuilder<'_, '_, C, D, No, No>;

    //fn list_triggers(&self) -> requests::ListTriggersBuilder<'_, '_, C, D>;
    //fn list_stored_procedures(&self) -> requests::ListStoredProceduresBuilder<'_, '_, C, D>;
    //fn list_user_defined_functions(
    //    &self,
    //) -> requests::ListUserDefinedFunctionsBuilder<'_, '_, C, D>;

    //fn create_document(&self) -> requests::CreateDocumentBuilder<'_, '_, C, D, No>;
    //fn replace_document(&self) -> requests::ReplaceDocumentBuilder<'_, '_, C, D, No, No>;
    //fn list_documents(&self) -> requests::ListDocumentsBuilder<'_, '_, C, D>;
    //fn query_documents(&self) -> requests::QueryDocumentsBuilder<'_, '_, C, D, No>;

    //fn get_partition_key_ranges(&self) -> requests::GetPartitionKeyRangesBuilder<'_, '_, C, D>;

    //fn prepare_request(&self, method: http::Method) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!("dbs/{}/colls", self.database_client().database_name()),
    //        method,
    //        ResourceType::Collections,
    //    )
    //}
    //fn prepare_request_with_collection_name(&self, method: http::Method) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!(
    //            "dbs/{}/colls/{}",
    //            self.database_client().database_name(),
    //            self.collection_name()
    //        ),
    //        method,
    //        ResourceType::Collections,
    //    )
    //}
}

pub trait HasCollectionClient<C, D, COLL>: HasDatabaseClient<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    fn collection_client(&self) -> &COLL;
}

pub trait WithCollectionClient<'a, C, D, COLL>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    fn with_collection_client<IntoCowStr>(&'a self, collection_name: IntoCowStr) -> COLL
    where
        IntoCowStr: Into<Cow<'a, str>>;
}

pub trait IntoCollectionClient<'a, C, D, COLL>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    fn into_collection_client<IntoCowStr>(self, collection_name: IntoCowStr) -> COLL
    where
        IntoCowStr: Into<Cow<'a, str>>;
}

pub trait UserDefinedFunctionClient<C, D, COLL>: HasCollectionClient<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    fn user_defined_function_name(&self) -> &str;

    //fn create_user_defined_function(
    //    &self,
    //) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, '_, C, D, COLL, No>;
    //fn replace_user_defined_function(
    //    &self,
    //) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, '_, C, D, COLL, No>;
    //fn delete_user_defined_function(
    //    &self,
    //) -> requests::DeleteUserDefinedFunctionBuilder<'_, '_, C, D, COLL>;

    //fn prepare_request(&self, method: http::Method) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!(
    //            "dbs/{}/colls/{}/udfs",
    //            self.database_client().database_name(),
    //            self.collection_client().collection_name(),
    //        ),
    //        method,
    //        ResourceType::UserDefinedFunctions,
    //    )
    //}
    //fn prepare_request_with_user_defined_function_name(
    //    &self,
    //    method: http::Method,
    //) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!(
    //            "dbs/{}/colls/{}/udfs/{}",
    //            self.database_client().database_name(),
    //            self.collection_client().collection_name(),
    //            self.user_defined_function_name()
    //        ),
    //        method,
    //        ResourceType::UserDefinedFunctions,
    //    )
    //}
}

pub trait HasUserDefinedFunctionClient<C, D, COLL, UDF>: HasCollectionClient<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    UDF: UserDefinedFunctionClient<C, D, COLL>,
{
    fn user_defined_function_client(&self) -> &UDF;
}

pub trait WithUserDefinedFunctionClient<'a, C, D, COLL, UDF>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    UDF: UserDefinedFunctionClient<C, D, COLL>,
{
    fn with_user_defined_function_client<IntoCowStr>(
        &'a self,
        user_defined_function_name: IntoCowStr,
    ) -> UDF
    where
        IntoCowStr: Into<Cow<'a, str>>;
}

pub trait IntoUserDefinedFunctionClient<'a, C, D, COLL, UDF>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    UDF: UserDefinedFunctionClient<C, D, COLL>,
{
    fn into_user_defined_function_client<IntoCowStr>(
        self,
        user_defined_function_name: IntoCowStr,
    ) -> UDF
    where
        IntoCowStr: Into<Cow<'a, str>>;
}

pub trait StoredProcedureClient<C, D, COLL>: HasCollectionClient<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    fn stored_procedure_name(&self) -> &str;

    //fn create_stored_procedure(
    //    &self,
    //) -> requests::CreateStoredProcedureBuilder<'_, '_, C, D, COLL, No>;
    //fn delete_stored_procedure(&self)
    //    -> requests::DeleteStoredProcedureBuilder<'_, '_, C, D, COLL>;
    //fn execute_stored_procedure(
    //    &self,
    //) -> requests::ExecuteStoredProcedureBuilder<'_, '_, C, D, COLL>;
    //fn replace_stored_procedure(
    //    &self,
    //) -> requests::ReplaceStoredProcedureBuilder<'_, '_, C, D, COLL, No>;

    //fn prepare_request(&self, method: http::Method) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!(
    //            "dbs/{}/colls/{}/sprocs",
    //            self.database_client().database_name(),
    //            self.collection_client().collection_name(),
    //        ),
    //        method,
    //        ResourceType::StoredProcedures,
    //    )
    //}
    //fn prepare_request_with_stored_procedure_name(
    //    &self,
    //    method: http::Method,
    //) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!(
    //            "dbs/{}/colls/{}/sprocs/{}",
    //            self.database_client().database_name(),
    //            self.collection_client().collection_name(),
    //            self.stored_procedure_name()
    //        ),
    //        method,
    //        ResourceType::StoredProcedures,
    //    )
    //}
}

pub trait HasStoredProcedureClient<C, D, COLL, SP>: HasCollectionClient<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    SP: StoredProcedureClient<C, D, COLL>,
{
    fn stored_procedure_client(&self) -> &SP;
}

pub trait WithStoredProcedureClient<'a, C, D, COLL, SP>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    SP: StoredProcedureClient<C, D, COLL>,
{
    fn with_stored_procedure_client<IntoCowStr>(&'a self, stored_procedure_name: IntoCowStr) -> SP
    where
        IntoCowStr: Into<Cow<'a, str>>;
}

pub trait IntoStoredProcedureClient<'a, C, D, COLL, SP>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    SP: StoredProcedureClient<C, D, COLL>,
{
    fn into_stored_procedure_client<IntoCowStr>(self, stored_procedure_name: IntoCowStr) -> SP
    where
        IntoCowStr: Into<Cow<'a, str>>;
}

pub trait TriggerClient<C, D, COLL>: HasCollectionClient<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    fn trigger_name(&self) -> &str;

    //fn create_trigger(&self)
    //    -> requests::CreateOrReplaceTriggerBuilder<'_, C, D, COLL, No, No, No>;
    //fn replace_trigger(
    //    &self,
    //) -> requests::CreateOrReplaceTriggerBuilder<'_, C, D, COLL, No, No, No>;
    //fn delete_trigger(&self) -> requests::DeleteTriggerBuilder<'_, '_, C, D, COLL>;

    //fn prepare_request(&self, method: http::Method) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!(
    //            "dbs/{}/colls/{}/triggers",
    //            self.database_client().database_name(),
    //            self.collection_client().collection_name(),
    //        ),
    //        method,
    //        ResourceType::Triggers,
    //    )
    //}
    //fn prepare_request_with_trigger_name(&self, method: http::Method) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!(
    //            "dbs/{}/colls/{}/triggers/{}",
    //            self.database_client().database_name(),
    //            self.collection_client().collection_name(),
    //            self.trigger_name()
    //        ),
    //        method,
    //        ResourceType::Triggers,
    //    )
    //}
}

pub trait HasTriggerClient<C, D, COLL, TRIGGER>: HasCollectionClient<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    TRIGGER: TriggerClient<C, D, COLL>,
{
    fn trigger_client(&self) -> &TRIGGER;
}

pub trait WithTriggerClient<'a, C, D, COLL, TRIGGER>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    TRIGGER: TriggerClient<C, D, COLL>,
{
    fn with_trigger_client<IntoCowStr>(&'a self, trigger_name: IntoCowStr) -> TRIGGER
    where
        IntoCowStr: Into<Cow<'a, str>>;
}

pub trait IntoTriggerClient<'a, C, D, COLL, TRIGGER>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    TRIGGER: TriggerClient<C, D, COLL>,
{
    fn into_trigger_client<IntoCowStr>(self, trigger_name: IntoCowStr) -> TRIGGER
    where
        IntoCowStr: Into<Cow<'a, str>>;
}

pub trait DocumentClient<C, D, COLL>: HasCollectionClient<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    fn document_name(&self) -> &str;
    fn partition_keys(&self) -> &PartitionKeys;

    //fn get_document(&self) -> requests::GetDocumentBuilder<'_, '_, C, D, COLL>;
    //fn delete_document(&self) -> requests::DeleteDocumentBuilder<'_, C, D, COLL>;
    //fn list_attachments(&self) -> requests::ListAttachmentsBuilder<'_, '_, C, D, COLL>;

    //fn prepare_request(&self, method: http::Method) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!(
    //            "dbs/{}/colls/{}/docs",
    //            self.database_client().database_name(),
    //            self.collection_client().collection_name()
    //        ),
    //        method,
    //        ResourceType::Documents,
    //    )
    //}
    //fn prepare_request_with_document_name(&self, method: http::Method) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!(
    //            "dbs/{}/colls/{}/docs/{}",
    //            self.database_client().database_name(),
    //            self.collection_client().collection_name(),
    //            self.document_name()
    //        ),
    //        method,
    //        ResourceType::Documents,
    //    )
    //}
}

pub trait HasDocumentClient<C, D, COLL, DOC>: HasCollectionClient<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    fn document_client(&self) -> &DOC;
}

pub trait WithDocumentClient<'a, 'b, C, D, COLL, DOC>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    fn with_document_client<DocName>(
        &'a self,
        document_name: DocName,
        partition_keys: PartitionKeys,
    ) -> DOC
    where
        DocName: Into<Cow<'b, str>>;
}

pub trait IntoDocumentClient<'b, C, D, COLL, DOC>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    fn into_document_client<DocName>(
        self,
        document_name: DocName,
        partition_keys: PartitionKeys,
    ) -> DOC
    where
        DocName: Into<Cow<'b, str>>;
}

pub trait AttachmentClient<C, D, COLL, DOC>: HasDocumentClient<C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    fn attachment_name(&self) -> &str;

    //fn create_slug(&self)
    //    -> requests::CreateSlugAttachmentBuilder<'_, '_, C, D, COLL, DOC, No, No>;
    //fn replace_slug(
    //    &self,
    //) -> requests::ReplaceSlugAttachmentBuilder<'_, '_, C, D, COLL, DOC, No, No>;
    //fn create_reference(
    //    &self,
    //) -> requests::CreateReferenceAttachmentBuilder<'_, '_, C, D, COLL, DOC, No, No>;
    //fn replace_reference(
    //    &self,
    //) -> requests::ReplaceReferenceAttachmentBuilder<'_, '_, C, D, COLL, DOC, No, No>;
    //fn delete(&self) -> requests::DeleteAttachmentBuilder<'_, '_, C, D, COLL, DOC>;
    //fn get(&self) -> requests::GetAttachmentBuilder<'_, '_, C, D, COLL, DOC>;

    //fn prepare_request(&self, method: http::Method) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!(
    //            "dbs/{}/colls/{}/docs/{}/attachments",
    //            self.database_client().database_name(),
    //            self.collection_client().collection_name(),
    //            self.document_client().document_name(),
    //        ),
    //        method,
    //        ResourceType::Attachments,
    //    )
    //}
    //fn prepare_request_with_attachment_name(&self, method: http::Method) -> http::request::Builder {
    //    self.cosmos_client().prepare_request(
    //        &format!(
    //            "dbs/{}/colls/{}/docs/{}/attachments/{}",
    //            self.database_client().database_name(),
    //            self.collection_client().collection_name(),
    //            self.document_client().document_name(),
    //            self.attachment_name()
    //        ),
    //        method,
    //        ResourceType::Attachments,
    //    )
    //}
}

pub trait HasAttachmentClient<C, D, COLL, DOC, ATT>: HasDocumentClient<C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
    ATT: AttachmentClient<C, D, COLL, DOC>,
{
    fn attachment_client(&self) -> &ATT;
}

pub trait WithAttachmentClient<'a, C, D, COLL, DOC, ATT>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
    ATT: AttachmentClient<C, D, COLL, DOC>,
{
    fn with_attachment_client<IntoCowStr>(&'a self, attachment_name: IntoCowStr) -> ATT
    where
        IntoCowStr: Into<Cow<'a, str>>;
}

pub trait IntoAttachmentClient<'a, C, D, COLL, DOC, ATT>: Debug + Send + Sync
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
    ATT: AttachmentClient<C, D, COLL, DOC>,
{
    fn into_attachment_client<IntoCowStr>(self, attachment_name: IntoCowStr) -> ATT
    where
        IntoCowStr: Into<Cow<'a, str>>;
}
