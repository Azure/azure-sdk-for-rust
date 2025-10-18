// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::constants;
use crate::models::ThroughputProperties;
use azure_core::http::headers::{AsHeaders, HeaderName, HeaderValue};
use azure_core::http::{headers, ClientMethodOptions, ClientOptions, Etag};
use std::convert::Infallible;
use std::fmt;
use std::fmt::Display;

/// Options used when creating a [`CosmosClient`](crate::CosmosClient).
#[derive(Clone, Default, Debug)]
pub struct CosmosClientOptions {
    pub client_options: ClientOptions,
}

/// Options to be passed to [`DatabaseClient::create_container()`](crate::clients::DatabaseClient::create_container()).
#[derive(Clone, Default)]
pub struct CreateContainerOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
    pub throughput: Option<ThroughputProperties>,
}

/// Options to be passed to [`ContainerClient::replace()`](crate::clients::ContainerClient::replace()).
#[derive(Clone, Default)]
pub struct ReplaceContainerOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`CosmosClient::create_database()`](crate::CosmosClient::create_database()).
#[derive(Clone, Default)]
pub struct CreateDatabaseOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
    pub throughput: Option<ThroughputProperties>,
}

/// Options to be passed to [`ContainerClient::delete()`](crate::clients::ContainerClient::delete()).
#[derive(Clone, Default)]
pub struct DeleteContainerOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`DatabaseClient::delete()`](crate::clients::DatabaseClient::delete()).
#[derive(Clone, Default)]
pub struct DeleteDatabaseOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

/// Specifies consistency levels that can be used when working with Cosmos APIs.
///
/// Learn more at [Consistency Levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
#[derive(Clone)]
pub enum ConsistencyLevel {
    ConsistentPrefix,
    Eventual,
    Session,
    BoundedStaleness,
    Strong,
}

impl Display for ConsistencyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ConsistencyLevel::ConsistentPrefix => "ConsistentPrefix",
            ConsistencyLevel::Eventual => "Eventual",
            ConsistencyLevel::Session => "Session",
            ConsistencyLevel::BoundedStaleness => "BoundedStaleness",
            ConsistencyLevel::Strong => "Strong",
        };
        write!(f, "{}", value)
    }
}

/// Specifies indexing directives that can be used when working with Cosmos APIs.
#[derive(Clone)]
pub enum IndexingDirective {
    Default,
    Include,
    Exclude,
}

impl Display for IndexingDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            IndexingDirective::Default => "Default",
            IndexingDirective::Include => "Include",
            IndexingDirective::Exclude => "Exclude",
        };
        write!(f, "{}", value)
    }
}

/// Options to be passed to APIs that manipulate items.
#[derive(Clone, Default)]
pub struct ItemOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
    /// Triggers executed before the operation.
    ///
    /// See [Triggers](https://learn.microsoft.com/rest/api/cosmos-db/triggers) for more.
    pub pre_triggers: Option<Vec<String>>,
    /// Triggers executed after the operation.
    ///
    /// See [Triggers](https://learn.microsoft.com/rest/api/cosmos-db/triggers) for more.
    pub post_triggers: Option<Vec<String>>,
    /// Applies when working with Session consistency.
    /// Each new write request to Azure Cosmos DB is assigned a new Session Token.
    /// The client instance will use this token internally with each read/query request to ensure that the set consistency level is maintained.
    ///
    /// See [Session Tokens](https://learn.microsoft.com/azure/cosmos-db/nosql/how-to-manage-consistency?tabs=portal%2Cdotnetv2%2Capi-async#utilize-session-tokens) for more.
    pub session_token: Option<String>,
    /// Used to specify the consistency level for the operation.
    ///
    /// The default value is the consistency level set on the Cosmos DB account.
    /// See [Consistency Levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
    pub consistency_level: Option<ConsistencyLevel>,
    /// Sets indexing directive for the operation.
    pub indexing_directive: Option<IndexingDirective>,
    /// If specified, the operation will only be performed if the item matches the provided Etag.
    ///
    /// See [Optimistic Concurrency Control](https://learn.microsoft.com/azure/cosmos-db/nosql/database-transactions-optimistic-concurrency#optimistic-concurrency-control) for more.
    pub if_match_etag: Option<Etag>,
    /// When this value is true, write operations will respond with the new value of the resource being written.
    ///
    /// The default for this is `false`, which reduces the network and CPU burden that comes from serializing and deserializing the response.
    pub enable_content_response_on_write: bool,
}

impl AsHeaders for ItemOptions<'_> {
    type Error = Infallible;
    type Iter = std::vec::IntoIter<(HeaderName, HeaderValue)>;

    fn as_headers(&self) -> Result<Self::Iter, Self::Error> {
        let mut headers = Vec::new();

        if let Some(pre_triggers) = &self.pre_triggers {
            headers.push((
                constants::PRE_TRIGGER_INCLUDE,
                pre_triggers.join(",").into(),
            ));
        }

        if let Some(post_triggers) = &self.post_triggers {
            headers.push((
                constants::POST_TRIGGER_INCLUDE,
                post_triggers.join(",").into(),
            ));
        }

        if let Some(session_token) = &self.session_token {
            headers.push((constants::SESSION_TOKEN, session_token.into()));
        }

        if let Some(consistency_level) = &self.consistency_level {
            headers.push((
                constants::CONSISTENCY_LEVEL,
                consistency_level.to_string().into(),
            ));
        }

        if let Some(indexing_directive) = &self.indexing_directive {
            headers.push((
                constants::INDEXING_DIRECTIVE,
                indexing_directive.to_string().into(),
            ));
        }

        if let Some(etag) = &self.if_match_etag {
            headers.push((headers::IF_MATCH, etag.to_string().into()));
        }

        if !self.enable_content_response_on_write {
            headers.push((headers::PREFER, constants::PREFER_MINIMAL));
        }

        Ok(headers.into_iter())
    }
}

/// Options to be passed to [`DatabaseClient::query_containers()`](crate::clients::DatabaseClient::query_containers())
#[derive(Clone, Default)]
pub struct QueryContainersOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`CosmosClient::query_databases()`](crate::CosmosClient::query_databases())
#[derive(Clone, Default)]
pub struct QueryDatabasesOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`ContainerClient::query_items()`](crate::clients::ContainerClient::query_items()).
#[derive(Clone, Default)]
pub struct QueryOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,

    /// An external query engine to use for executing the query.
    ///
    /// NOTE: This is an unstable feature and may change in the future.
    /// Specifically, the query engine may be built-in to the SDK in the future, and this option may be removed entirely.
    #[cfg(feature = "preview_query_engine")]
    pub query_engine: Option<crate::query::QueryEngineRef>,
}

impl QueryOptions<'_> {
    pub fn into_owned(self) -> QueryOptions<'static> {
        QueryOptions {
            method_options: ClientMethodOptions {
                context: self.method_options.context.into_owned(),
            },
            #[cfg(feature = "preview_query_engine")]
            query_engine: self.query_engine,
        }
    }
}

/// Options to be passed to [`ContainerClient::read()`](crate::clients::ContainerClient::read()).
#[derive(Clone, Default)]
pub struct ReadContainerOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`DatabaseClient::read()`](crate::clients::DatabaseClient::read()).
#[derive(Clone, Default)]
pub struct ReadDatabaseOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to operations related to Throughput offers.
#[derive(Clone, Default)]
pub struct ThroughputOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_options_as_headers() {
        let item_options = ItemOptions {
            pre_triggers: Some(vec!["PreTrigger1".to_string(), "PreTrigger2".to_string()]),
            post_triggers: Some(vec!["PostTrigger1".to_string(), "PostTrigger2".to_string()]),
            session_token: Some("SessionToken".to_string()),
            consistency_level: Some(ConsistencyLevel::Session),
            indexing_directive: Some(IndexingDirective::Include),
            if_match_etag: Some(Etag::from("etag_value")),
            enable_content_response_on_write: false,
            ..Default::default()
        };

        let headers_result: Vec<(HeaderName, HeaderValue)> =
            item_options.as_headers().unwrap().collect();

        let headers_expected: Vec<(HeaderName, HeaderValue)> = vec![
            (
                constants::PRE_TRIGGER_INCLUDE,
                "PreTrigger1,PreTrigger2".into(),
            ),
            (
                constants::POST_TRIGGER_INCLUDE,
                "PostTrigger1,PostTrigger2".into(),
            ),
            (constants::SESSION_TOKEN, "SessionToken".into()),
            (constants::CONSISTENCY_LEVEL, "Session".into()),
            (constants::INDEXING_DIRECTIVE, "Include".into()),
            (headers::IF_MATCH, "etag_value".into()),
            (headers::PREFER, constants::PREFER_MINIMAL),
        ];

        assert_eq!(headers_result, headers_expected);
    }

    #[test]
    fn item_options_empty_as_headers_with_content_response() {
        let item_options = ItemOptions::default();

        let headers_result: Vec<(HeaderName, HeaderValue)> =
            item_options.as_headers().unwrap().collect();

        let headers_expected: Vec<(HeaderName, HeaderValue)> =
            vec![(headers::PREFER, constants::PREFER_MINIMAL)];

        assert_eq!(headers_result, headers_expected);
    }

    #[test]
    fn item_options_empty_as_headers() {
        let item_options = ItemOptions {
            enable_content_response_on_write: true,
            ..Default::default()
        };

        let headers_result: Vec<(HeaderName, HeaderValue)> =
            item_options.as_headers().unwrap().collect();

        let headers_expected: Vec<(HeaderName, HeaderValue)> = vec![];

        assert_eq!(headers_result, headers_expected);
    }
}
