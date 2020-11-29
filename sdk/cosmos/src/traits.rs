use crate::clients::*;
use crate::resources::collection::*;
use crate::resources::{collection, database, stored_procedure, trigger, user};
use crate::{headers, ConsistencyLevel, IndexingDirective, Offer, PartitionKeys, Query};
use http::request::Builder;

pub trait CosmosClientRequired<'a> {
    fn cosmos_client(&'a self) -> &'a CosmosClient;
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
            headers::HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION,
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
    fn parameters(&self) -> Option<&'a stored_procedure::Parameters>;

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
    fn with_parameters(self, parameters: &'a stored_procedure::Parameters) -> Self::O;
}

pub trait ParallelizeCrossPartitionQueryOption {
    fn parallelize_cross_partition_query(&self) -> bool;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        builder.header(
            headers::HEADER_DOCUMENTDB_QUERY_PARALLELIZECROSSPARTITIONQUERY,
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
        builder.header(
            headers::HEADER_DOCUMENTDB_IS_UPSERT,
            self.is_upsert().to_string(),
        )
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
            builder.header(headers::HEADER_A_IM, "Incremental feed")
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
            headers::HEADER_ALLOW_MULTIPLE_WRITES,
            self.allow_tentative_writes().to_string(),
        )
    }
}

pub trait ConsistencyLevelSupport<'a> {
    type O;
    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O;
}

pub trait ConsistencyLevelOption<'a> {
    fn consistency_level(&self) -> Option<ConsistencyLevel>;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        if let Some(consistency_level) = self.consistency_level() {
            let builder = builder.header(
                headers::HEADER_CONSISTENCY_LEVEL,
                consistency_level.to_consistency_level_header(),
            );

            // if we have a Session consistency level we make sure to pass
            // the x-ms-session-token header too.
            if let ConsistencyLevel::Session(session_token) = consistency_level {
                builder.header(headers::HEADER_SESSION_TOKEN, session_token)
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
            builder.header(
                headers::HEADER_DOCUMENTDB_PARTITIONRANGEID,
                partition_range_id,
            )
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
            IndexingDirective::Exclude => {
                builder.header(headers::HEADER_INDEXING_DIRECTIVE, "Exclude")
            }
            IndexingDirective::Include => {
                builder.header(headers::HEADER_INDEXING_DIRECTIVE, "Include")
            }
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
            builder.header(headers::HEADER_MAX_ITEM_COUNT, -1)
        } else {
            builder.header(headers::HEADER_MAX_ITEM_COUNT, self.max_item_count())
        }
    }
}

pub trait PartitionKeySupport<'a> {
    type O;
    fn with_partition_key(self, partition_key: &'a collection::PartitionKey) -> Self::O;
}

pub trait PartitionKeyOption<'a> {
    fn partition_key(&self) -> Option<&'a collection::PartitionKey>;
}

pub trait PartitionKeyRequired<'a> {
    fn partition_key(&self) -> &'a collection::PartitionKey;
}

pub trait PartitionKeysSupport<'a> {
    type O;
    fn with_partition_keys(self, partition_keys: &'a PartitionKeys) -> Self::O;
}

pub trait TriggerOperationRequired {
    fn trigger_operation(&self) -> trigger::TriggerOperation;
}

pub trait TriggerOperationSupport {
    type O;
    fn with_trigger_operation(self, a: trigger::TriggerOperation) -> Self::O;
}

pub trait TriggerTypeRequired {
    fn trigger_type(&self) -> trigger::TriggerType;
}

pub trait TriggerTypeSupport {
    type O;
    fn with_trigger_type(self, a: trigger::TriggerType) -> Self::O;
}

pub trait PartitionKeysRequired<'a> {
    fn partition_keys(&self) -> &'a PartitionKeys;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        crate::add_partition_keys_header(self.partition_keys(), builder)
    }
}

pub trait PartitionKeysOption<'a> {
    fn partition_keys(&self) -> Option<&'a PartitionKeys>;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        if let Some(partition_keys) = self.partition_keys() {
            let serialized = partition_keys.to_json();
            builder.header(headers::HEADER_DOCUMENTDB_PARTITIONKEY, serialized)
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
    fn with_body(self, body: &'a str) -> Self::O;
}

pub trait UserDefinedFunctionBodyRequired<'a> {
    fn body(&self) -> &'a str;
}

pub trait UserDefinedFunctionBodySupport<'a> {
    type O;
    fn with_body(self, body: &'a str) -> Self::O;
}

pub trait TriggerBodyRequired<'a> {
    fn body(&self) -> &'a str;
}

pub trait TriggerBodySupport<'a> {
    type O;
    fn with_body(self, body: &'a str) -> Self::O;
}

pub trait ExpirySecondsOption {
    fn expiry_seconds(&self) -> u64;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        builder.header(
            headers::HEADER_DOCUMENTDB_EXPIRY_SECONDS,
            self.expiry_seconds(),
        )
    }
}

pub trait ExpirySecondsSupport {
    type O;
    fn with_expiry_seconds(self, expiry_seconds: u64) -> Self::O;
}

pub trait DatabaseClientRequired<'a> {
    fn database_client(&self) -> &'a DatabaseClient;
}

pub trait DatabaseSupport<'a> {
    type O;
    fn with_database(self, database: &'a str) -> Self::O;
}

pub trait CollectionClientRequired<'a> {
    fn collection_client(&self) -> &'a CollectionClient;
}

pub trait AttachmentClientRequired<'a> {
    fn attachment_client(&self) -> &'a AttachmentClient;
}

pub trait StoredProcedureClientRequired<'a> {
    fn stored_procedure_client(&self) -> &'a StoredProcedureClient;
}

pub trait UserDefinedFunctionClientRequired<'a> {
    fn user_defined_function_client(&self) -> &'a UserDefinedFunctionClient;
}

pub trait TriggerClientRequired<'a> {
    fn trigger_client(&'a self) -> &'a TriggerClient;
}

pub trait UserClientRequired<'a> {
    fn user_client(&'a self) -> &'a UserClient;
}

pub trait StoredProcedureNameRequired<'a> {
    fn stored_procedure_name(&self) -> &'a str;
}

pub trait StoredProcedureNameSupport<'a> {
    type O;
    fn with_stored_procedure_name(self, stored_procedure_name: &'a str) -> Self::O;
}

pub trait DocumentClientRequired<'a> {
    fn document_client(&'a self) -> &'a DocumentClient;
}

pub trait PermissionClientRequired<'a> {
    fn permission_client(&self) -> &'a PermissionClient;
}

pub trait OfferRequired {
    fn offer(&self) -> Offer;

    #[must_use]
    fn add_header(&self, builder: Builder) -> Builder {
        match self.offer() {
            Offer::Throughput(throughput) => {
                builder.header(headers::HEADER_OFFER_THROUGHPUT, throughput)
            }
            Offer::S1 => builder.header(headers::HEADER_OFFER_TYPE, "S1"),
            Offer::S2 => builder.header(headers::HEADER_OFFER_TYPE, "S2"),
            Offer::S3 => builder.header(headers::HEADER_OFFER_TYPE, "S3"),
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

pub trait DatabaseNameRequired<'a> {
    fn database_name(&'a self) -> &'a dyn database::DatabaseName;
}

pub trait DatabaseNameSupport<'a> {
    type O;
    fn with_database_name(self, database_name: &'a dyn database::DatabaseName) -> Self::O;
}

pub trait UserNameRequired<'a> {
    fn user_name(&self) -> &'a dyn user::UserName;
}

pub trait UserNameSupport<'a> {
    type O;
    fn with_user_name(self, user_name: &'a dyn user::UserName) -> Self::O;
}
