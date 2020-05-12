#![warn(unused_extern_crates)]
#![recursion_limit = "128"]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

pub mod attachment;
mod authorization_token;
pub mod clients;
pub mod collection;
mod consistency_level;
mod database;
mod document;
mod document_attributes;
mod errors;
pub(crate) mod from_headers;
mod headers;
mod indexing_directive;
pub mod offer;
mod partition_keys;
mod permission;
mod permission_resource;
mod permission_token;
pub mod prelude;
mod query;
mod requests;
mod resource;
mod resource_quota;
pub mod responses;
pub mod stored_procedure;
mod to_json_vector;
mod user;

pub use self::attachment::Attachment;
pub use self::authorization_token::*;
use self::collection::IndexingPolicy;
pub use self::consistency_level::ConsistencyLevel;
pub use self::database::{Database, DatabaseName};
pub use self::document::{Document, DocumentName};
pub use self::document_attributes::DocumentAttributes;
pub use self::indexing_directive::IndexingDirective;
pub use self::offer::Offer;
pub use self::permission::{Permission, PermissionMode, PermissionName};
pub use self::permission_resource::PermissionResource;
pub use self::permission_token::PermissionToken;
pub use self::query::{Param, ParamDef, Query};
pub use self::requests::*;
pub use self::resource::Resource;
pub use self::resource_quota::ResourceQuota;
use crate::clients::{
    AttachmentClient, Client, CollectionClient, CosmosUriBuilder, DatabaseClient, DocumentClient,
    PermissionClient, StoredProcedureClient, UserClient,
};
use crate::collection::Collection;
use crate::collection::CollectionName;
use crate::headers::*;
pub use crate::partition_keys::PartitionKeys;
use crate::stored_procedure::{Parameters, StoredProcedureName};
pub use crate::user::{User, UserName};
use attachment::AttachmentName;
use azure_sdk_core::No;
use http::request::Builder;
use serde::Serialize;

pub trait ClientRequired<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn client(&self) -> &'a Client<CUB>;
}

pub trait DatabaseRequired<'a> {
    fn database(&self) -> &'a str;
}

pub trait QueryCrossPartitionSupport {
    type O;
    fn with_query_cross_partition(self, query_cross_partition: bool) -> Self::O;
}

pub trait QueryCrossPartitionOption {
    fn query_cross_partition(&self) -> bool;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        builder.header(
            HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION,
            self.query_cross_partition().to_string(),
        )
    }
}

pub trait ParallelizeCrossPartitionQuerySupport {
    type O;
    fn with_parallelize_cross_partition_query(
        self,
        parallelize_cross_partition_query: bool,
    ) -> Self::O;
}

pub trait ParametersOption<'a> {
    fn parameters(&self) -> Option<&'a Parameters>;

    fn generate_body(&self) -> String {
        if let Some(parameters) = self.parameters() {
            parameters.to_json()
        } else {
            String::from("[]")
        }
    }
}

pub trait ParametersSupport<'a> {
    type O;
    fn with_parameters(self, parameters: &'a Parameters) -> Self::O;
}

pub trait ParallelizeCrossPartitionQueryOption {
    fn parallelize_cross_partition_query(&self) -> bool;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        builder.header(
            HEADER_DOCUMENTDB_QUERY_PARALLELIZECROSSPARTITIONQUERY,
            self.parallelize_cross_partition_query().to_string(),
        )
    }
}

pub trait IsUpsertSupport {
    type O;
    fn with_is_upsert(self, is_upsert: bool) -> Self::O;
}

pub trait IsUpsertOption {
    fn is_upsert(&self) -> bool;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        builder.header(HEADER_DOCUMENTDB_IS_UPSERT, self.is_upsert().to_string())
    }
}

pub trait AIMSupport {
    type O;
    fn with_a_im(self, a_im: bool) -> Self::O;
}

pub trait AIMOption {
    fn a_im(&self) -> bool;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        if self.a_im() {
            builder.header(HEADER_A_IM, "Incremental feed")
        } else {
            builder
        }
    }
}

pub trait AllowTentativeWritesSupport {
    type O;
    fn with_allow_tentative_writes(self, allow_tentative_writes: bool) -> Self::O;
}

pub trait AllowTentativeWritesOption {
    fn allow_tentative_writes(&self) -> bool;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        builder.header(
            HEADER_ALLOW_MULTIPLE_WRITES,
            self.allow_tentative_writes().to_string(),
        )
    }
}

pub trait ConsistencyLevelSupport<'a> {
    type O;
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O;
}

pub trait ConsistencyLevelOption<'a> {
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>>;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        if let Some(consistency_level) = self.consistency_level() {
            let builder = builder.header(
                HEADER_CONSISTENCY_LEVEL,
                consistency_level.to_consistency_level_header(),
            );

            // if we have a Session consistency level we make sure to pass
            // the x-ms-session-token header too.
            if let ConsistencyLevel::Session(session_token) = consistency_level {
                builder.header(HEADER_SESSION_TOKEN, session_token.as_ref())
            } else {
                builder
            }
        } else {
            builder
        }
    }
}

pub trait PartitionRangeIdSupport<'a> {
    type O;
    fn with_partition_range_id(self, partition_range_id: &'a str) -> Self::O;
}

pub trait PartitionRangeIdOption<'a> {
    fn partition_range_id(&self) -> Option<&'a str>;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        if let Some(partition_range_id) = self.partition_range_id() {
            builder.header(HEADER_DOCUMENTDB_PARTITIONRANGEID, partition_range_id)
        } else {
            builder
        }
    }
}

pub trait ContinuationSupport<'a> {
    type O;
    fn with_continuation(self, continuation: &'a str) -> Self::O;
}

pub trait ContinuationOption<'a> {
    fn continuation(&self) -> Option<&'a str>;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        if let Some(continuation) = self.continuation() {
            builder.header(HEADER_CONTINUATION, continuation)
        } else {
            builder
        }
    }
}

pub trait IndexingDirectiveSupport {
    type O;
    fn with_indexing_directive(self, indexing_directive: IndexingDirective) -> Self::O;
}

pub trait IndexingDirectiveOption {
    fn indexing_directive(&self) -> IndexingDirective;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        match self.indexing_directive() {
            IndexingDirective::Default => builder, // nothing to do
            IndexingDirective::Exclude => builder.header(HEADER_INDEXING_DIRECTIVE, "Exclude"),
            IndexingDirective::Include => builder.header(HEADER_INDEXING_DIRECTIVE, "Include"),
        }
    }
}

pub trait MaxItemCountSupport {
    type O;
    fn with_max_item_count(self, max_item_count: i32) -> Self::O;
}

pub trait MaxItemCountOption {
    fn max_item_count(&self) -> i32;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        if self.max_item_count() <= 0 {
            builder.header(HEADER_MAX_ITEM_COUNT, -1)
        } else {
            builder.header(HEADER_MAX_ITEM_COUNT, self.max_item_count())
        }
    }
}

pub trait PartitionKeySupport<'a> {
    type O;
    fn with_partition_key(self, partition_key: &'a self::collection::PartitionKey) -> Self::O;
}

pub trait PartitionKeyOption<'a> {
    fn partition_key(&self) -> Option<&'a self::collection::PartitionKey>;
}

pub trait PartitionKeyRequired<'a> {
    fn partition_key(&self) -> &'a self::collection::PartitionKey;
}

pub trait PartitionKeysSupport<'a> {
    type O;
    fn with_partition_keys(self, partition_keys: &'a PartitionKeys) -> Self::O;
}

pub(crate) fn add_partition_keys_header(
    partition_keys: &PartitionKeys,
    builder: Builder,
) -> Builder {
    let serialized = partition_keys.to_json();
    builder.header(HEADER_DOCUMENTDB_PARTITIONKEY, serialized)
}

pub trait PartitionKeysRequired<'a> {
    fn partition_keys(&self) -> &'a PartitionKeys;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        add_partition_keys_header(self.partition_keys(), builder)
    }
}

pub trait PartitionKeysOption<'a> {
    fn partition_keys(&self) -> Option<&'a PartitionKeys>;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        if let Some(partition_keys) = self.partition_keys() {
            let serialized = partition_keys.to_json();
            builder.header(HEADER_DOCUMENTDB_PARTITIONKEY, serialized)
        } else {
            builder
        }
    }
}

pub trait MediaRequired<'a> {
    fn media(&self) -> &'a str;
}

pub trait MediaSupport<'a> {
    type O;
    fn with_media(self, media: &'a str) -> Self::O;
}

pub trait StoredProcedureBodyRequired<'a> {
    fn body(&self) -> &'a str;
}

pub trait StoredProcedureBodySupport<'a> {
    type O;
    fn with_body(self, partition_keys: &'a str) -> Self::O;
}

pub trait ExpirySecondsOption {
    fn expiry_seconds(&self) -> u64;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        builder.header(HEADER_DOCUMENTDB_EXPIRY_SECONDS, self.expiry_seconds())
    }
}

pub trait ExpirySecondsSupport {
    type O;
    fn with_expiry_seconds(self, expiry_seconds: u64) -> Self::O;
}

pub trait DatabaseClientRequired<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_client(&self) -> &'a DatabaseClient<'a, CUB>;
}

pub trait DatabaseSupport<'a> {
    type O;
    fn with_database(self, database: &'a str) -> Self::O;
}

pub trait CollectionClientRequired<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn collection_client(&self) -> &'a CollectionClient<'a, CUB>;
}

//pub trait CollectionRequired<'a> {
//    fn collection(&self) -> &'a str;
//}

pub trait AttachmentClientRequired<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn attachment_client(&self) -> &'a AttachmentClient<'a, CUB>;
}

pub trait StoredProcedureClientRequired<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn stored_procedure_client(&self) -> &'a StoredProcedureClient<'a, CUB>;
}

pub trait UserClientRequired<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn user_client(&self) -> &'a UserClient<'a, CUB>;
}

pub trait StoredProcedureNameRequired<'a> {
    fn stored_procedure_name(&self) -> &'a str;
}

pub trait StoredProcedureNameSupport<'a> {
    type O;
    fn with_stored_procedure_name(self, stored_procedure_name: &'a str) -> Self::O;
}

pub trait DocumentClientRequired<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn document_client(&self) -> &'a DocumentClient<'a, CUB>;
}

pub trait PermissionClientRequired<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn permission_client(&self) -> &'a PermissionClient<'a, CUB>;
}

pub trait PermissionModeRequired<'a, R>
where
    R: PermissionResource,
{
    fn permission_mode(&self) -> &'a PermissionMode<R>;
}

pub trait PermissionModeSupport<'a, R>
where
    R: PermissionResource,
{
    type O;
    fn with_permission_mode(self, permission: &'a PermissionMode<R>) -> Self::O;
}

pub trait OfferRequired {
    fn offer(&self) -> Offer;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        match self.offer() {
            Offer::Throughput(throughput) => builder.header(HEADER_OFFER_THROUGHPUT, throughput),
            Offer::S1 => builder.header(HEADER_OFFER_TYPE, "S1"),
            Offer::S2 => builder.header(HEADER_OFFER_TYPE, "S2"),
            Offer::S3 => builder.header(HEADER_OFFER_TYPE, "S3"),
        }
    }
}

pub trait OfferSupport {
    type O;
    fn with_offer(self, offer: Offer) -> Self::O;
}

pub trait CollectionNameRequired<'a> {
    fn collection_name(&self) -> &'a dyn CollectionName;
}

pub trait CollectionNameSupport<'a> {
    type O;
    fn with_collection_name(self, collection_name: &'a dyn CollectionName) -> Self::O;
}

pub trait CollectionRequired<'a> {
    fn collection(&self) -> &'a Collection;
}

pub trait CollectionSupport<'a> {
    type O;
    fn with_collection(self, collection: &'a Collection) -> Self::O;
}

pub trait IndexingPolicyRequired<'a> {
    fn indexing_policy(&self) -> &'a IndexingPolicy;
}

pub trait IndexingPolicySupport<'a> {
    type O;
    fn with_indexing_policy(self, offer: &'a IndexingPolicy) -> Self::O;
}

pub trait DocumentRequired<'a, T>
where
    T: Serialize,
{
    fn document(&self) -> &'a Document<T>;
}

pub trait DocumentSupport<'a, T>
where
    T: Serialize,
{
    type O;
    fn with_document(self, document: &'a Document<T>) -> Self::O;
}

//pub trait CollectionSupport<'a> {
//    type O;
//    fn with_collection(self, collection: &'a str) -> Self::O;
//}

pub trait DocumentIdRequired<'a> {
    fn document_id(&self) -> &'a str;
}

pub trait DocumentIdSupport<'a> {
    type O;
    fn with_document_id(self, document_id: &'a str) -> Self::O;
}

pub trait QueryRequired<'a> {
    fn query(&self) -> &'a Query<'a>;
}

pub trait QuerySupport<'a> {
    type O;
    fn with_query(self, query: &'a Query<'a>) -> Self::O;
}

pub trait DatabaseNameRequired<'a, DB>
where
    DB: DatabaseName,
{
    fn database_name(&self) -> &'a DB;
}

pub trait DatabaseNameSupport<'a, DB>
where
    DB: DatabaseName,
{
    type O;
    fn with_database_name(self, database_name: &'a DB) -> Self::O;
}

pub trait UserNameRequired<'a> {
    fn user_name(&self) -> &'a dyn UserName;
}

pub trait UserNameSupport<'a> {
    type O;
    fn with_user_name(self, user_name: &'a dyn UserName) -> Self::O;
}

//// New implementation
pub trait CosmosTrait<CUB>
where
    CUB: CosmosUriBuilder,
{
    fn list_databases(&self) -> requests::ListDatabasesBuilder<'_, CUB>;
    fn with_database<'d>(&'d self, database_name: &'d dyn DatabaseName) -> DatabaseClient<'d, CUB>;
    fn create_database<DB>(&self) -> requests::CreateDatabaseBuilder<'_, CUB, DB, No>
    where
        DB: DatabaseName;
}

pub trait DatabaseTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    fn list_collections(&self) -> requests::ListCollectionsBuilder<'_, CUB>;
    fn get_database(&self) -> requests::GetDatabaseBuilder<'_, CUB>;
    fn delete_database(&self) -> requests::DeleteDatabaseBuilder<'_, CUB>;
    fn create_collection(&self) -> requests::CreateCollectionBuilder<'_, CUB, No, No, No, No>;
    fn with_collection<'c>(
        &'c self,
        collection_name: &'c dyn CollectionName,
    ) -> CollectionClient<'c, CUB>;
    fn with_user<'c>(&'c self, user_name: &'c dyn UserName) -> UserClient<'c, CUB>;
    fn list_users(&self) -> requests::ListUsersBuilder<'_, CUB>;
}

pub(crate) trait DatabaseBuilderTrait<'a, CUB>: DatabaseTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder;
}

pub trait CollectionTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    fn collection_name(&self) -> &'a dyn CollectionName;
    fn get_collection(&self) -> requests::GetCollectionBuilder<'_, CUB>;
    fn delete_collection(&self) -> requests::DeleteCollectionBuilder<'_, CUB>;
    fn replace_collection(&self) -> requests::ReplaceCollectionBuilder<'_, CUB, No, No>;
    fn list_documents(&self) -> requests::ListDocumentsBuilder<'_, '_, CUB>;
    fn create_document<T>(&self) -> requests::CreateDocumentBuilder<'_, '_, T, CUB, No, No>
    where
        T: Serialize;
    fn replace_document<T>(&self) -> requests::ReplaceDocumentBuilder<'_, '_, T, CUB, No, No>
    where
        T: Serialize;
    fn query_documents(&self) -> requests::QueryDocumentsBuilder<'_, '_, CUB, No>;
    fn with_stored_procedure<'c>(
        &'c self,
        stored_procedure_name: &'c dyn StoredProcedureName,
    ) -> StoredProcedureClient<'c, CUB>;
    fn list_stored_procedures(&self) -> requests::ListStoredProceduresBuilder<'_, CUB>;
    fn with_document<'c>(
        &'c self,
        document_name: &'c dyn DocumentName,
        partition_keys: &'c PartitionKeys,
    ) -> DocumentClient<'c, CUB>;
}

pub(crate) trait CollectionBuilderTrait<'a, CUB>: CollectionTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder;
}

pub trait DocumentTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    fn collection_name(&self) -> &'a dyn CollectionName;
    fn document_name(&self) -> &'a dyn DocumentName;
    fn partition_keys(&self) -> &'a PartitionKeys;
    fn get_document(&self) -> requests::GetDocumentBuilder<'_, '_, CUB>;
    fn delete_document(&self) -> requests::DeleteDocumentBuilder<'_, CUB>;
    fn list_attachments(&self) -> requests::ListAttachmentsBuilder<'_, '_, CUB>;
    fn with_attachment(
        &'a self,
        attachment_name: &'a dyn AttachmentName,
    ) -> AttachmentClient<'_, CUB>;
}

pub(crate) trait DocumentBuilderTrait<'a, CUB>: DocumentTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder;
}

pub trait StoredProcedureTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    fn collection_name(&self) -> &'a dyn CollectionName;
    fn stored_procedure_name(&self) -> &'a dyn StoredProcedureName;
    fn create_stored_procedure(&self) -> requests::CreateStoredProcedureBuilder<'_, CUB, No>;
    fn replace_stored_procedure(&self) -> requests::ReplaceStoredProcedureBuilder<'_, CUB, No>;
    fn execute_stored_procedure(&self) -> requests::ExecuteStoredProcedureBuilder<'_, '_, CUB>;
    fn delete_stored_procedure(&self) -> requests::DeleteStoredProcedureBuilder<'_, CUB>;
}

pub(crate) trait StoredProcedureBuilderTrait<'a, CUB>:
    StoredProcedureTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder;
}

pub trait AttachmentTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    fn collection_name(&self) -> &'a dyn CollectionName;
    fn document_name(&self) -> &'a dyn DocumentName;
    fn attachment_name(&self) -> &'a dyn AttachmentName;
    fn create_slug(&self) -> requests::CreateSlugAttachmentBuilder<'_, '_, CUB, No, No>;
    fn replace_slug(&self) -> requests::ReplaceSlugAttachmentBuilder<'_, '_, CUB, No, No>;
    fn create_reference(&self) -> requests::CreateReferenceAttachmentBuilder<'_, '_, CUB, No, No>;
    fn replace_reference(&self)
        -> requests::ReplaceReferenceAttachmentBuilder<'_, '_, CUB, No, No>;
    fn delete(&self) -> requests::DeleteAttachmentBuilder<'_, '_, CUB>;
    fn get(&self) -> requests::GetAttachmentBuilder<'_, '_, CUB>;
}

pub(crate) trait AttachmentBuilderTrait<'a, CUB>: AttachmentTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder;
}

pub trait UserTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    fn user_name(&self) -> &'a dyn UserName;
    fn create_user(&self) -> requests::CreateUserBuilder<'_, CUB>;
    fn get_user(&self) -> requests::GetUserBuilder<'_, CUB>;
    fn replace_user(&self) -> requests::ReplaceUserBuilder<'_, CUB, No>;
    fn delete_user(&self) -> requests::DeleteUserBuilder<'_, CUB>;
    fn with_permission<'c>(
        &'c self,
        permission_name: &'c dyn PermissionName,
    ) -> PermissionClient<'c, CUB>;
    fn list_permissions(&self) -> requests::ListPermissionsBuilder<'_, CUB>;
}

pub trait PermissionTrait<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName;
    fn user_name(&self) -> &'a dyn UserName;
    fn permission_name(&self) -> &'a dyn PermissionName;
    fn create_permission<R>(&self) -> requests::CreatePermissionBuilder<'_, CUB, R, No>
    where
        R: PermissionResource;
    fn replace_permission<R>(&self) -> requests::ReplacePermissionBuilder<'_, CUB, R, No>
    where
        R: PermissionResource;
    fn get_permission(&self) -> requests::GetPermissionBuilder<'_, CUB>;
    fn delete_permission(&self) -> requests::DeletePermissionsBuilder<'_, CUB>;
}
