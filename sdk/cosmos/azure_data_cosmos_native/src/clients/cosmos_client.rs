// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::ffi::CString;
use std::os::raw::c_char;

use azure_core::credentials::Secret;
use azure_data_cosmos::{
    clients::DatabaseClient, query::Query, ConnectionString, CosmosAccountEndpoint,
    CosmosAccountReference, CosmosClient, CosmosClientBuilder, QueryOptions,
};
use futures::TryStreamExt;

use crate::context::CallContext;
use crate::error::{self, CosmosErrorCode, Error};
use crate::options::{ClientOptions, CreateDatabaseOptions};
use crate::string::parse_cstr;
use crate::unwrap_required_ptr;

/// Creates a new CosmosClient and returns a pointer to it via the out parameter.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `endpoint` - The Cosmos DB account endpoint, as a nul-terminated C string.
/// * `key` - The Cosmos DB account key, as a nul-terminated C string
/// * `options` - Pointer to [`ClientOptions`] for client configuration, may be null.
/// * `out_client` - Output parameter that will receive a pointer to the created CosmosClient.
///
/// # Returns
/// * Returns [`CosmosErrorCode::Success`] on success.
/// * Returns [`CosmosErrorCode::InvalidArgument`] if any input pointer is null or if the input strings are invalid.
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx))]
pub extern "C" fn cosmos_client_create_with_key(
    ctx: *mut CallContext,
    endpoint: *const c_char,
    key: *const c_char,
    options: *const ClientOptions,
    out_client: *mut *mut CosmosClient,
) -> CosmosErrorCode {
    context!(ctx).run_async_with_output(out_client, async {
        let endpoint: CosmosAccountEndpoint =
            parse_cstr(endpoint, error::messages::INVALID_ENDPOINT)?.parse()?;
        let key = parse_cstr(key, error::messages::INVALID_KEY)?.to_string();

        let account = CosmosAccountReference::with_master_key(endpoint, Secret::new(key));
        let mut builder = CosmosClientBuilder::new();

        // Apply options from C options if provided
        if !options.is_null() {
            let c_options = unsafe { &*options };
            if c_options.allow_invalid_certificates() {
                builder = builder.with_allow_emulator_invalid_certificates(true);
            }
        }

        let client = builder.build(account).await?;

        Ok(Box::new(client))
    })
}

/// Creates a new CosmosClient using a connection string and returns a pointer to it via the out parameter.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `connection_string` - The Cosmos DB connection string, as a nul-terminated C string.
///                         Can be "emulator" to use the well-known emulator endpoint and key,
///                         or a full connection string in the format:
///                         `AccountEndpoint=https://...;AccountKey=...;`
/// * `options` - Pointer to [`ClientOptions`] for client configuration, may be null.
/// * `out_client` - Output parameter that will receive a pointer to the created CosmosClient.
///
/// # Returns
/// * Returns [`CosmosErrorCode::Success`] on success.
/// * Returns [`CosmosErrorCode::InvalidArgument`] if any input pointer is null or if the input string is invalid.
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx))]
pub extern "C" fn cosmos_client_create_with_connection_string(
    ctx: *mut CallContext,
    connection_string: *const c_char,
    options: *const ClientOptions,
    out_client: *mut *mut CosmosClient,
) -> CosmosErrorCode {
    context!(ctx).run_async_with_output(out_client, async {
        let connection_string_str = parse_cstr(
            connection_string,
            error::messages::INVALID_CONNECTION_STRING,
        )?;

        let conn: ConnectionString = connection_string_str.parse()?;
        let endpoint: CosmosAccountEndpoint = conn.account_endpoint.parse()?;
        let account = CosmosAccountReference::with_master_key(endpoint, conn.account_key);
        let mut builder = CosmosClientBuilder::new();

        // Apply options from C options if provided
        if !options.is_null() {
            let c_options = unsafe { &*options };
            if c_options.allow_invalid_certificates() {
                builder = builder.with_allow_emulator_invalid_certificates(true);
            }
        }

        let client = builder.build(account).await?;

        Ok(Box::new(client))
    })
}

/// Releases the memory associated with a [`CosmosClient`].
#[no_mangle]
#[tracing::instrument(level = "debug")]
pub extern "C" fn cosmos_client_free(client: *mut CosmosClient) {
    if !client.is_null() {
        tracing::trace!(?client, "freeing cosmos client");
        unsafe { drop(Box::from_raw(client)) }
    }
}

/// Gets a [`DatabaseClient`] from the given [`CosmosClient`] for the specified database ID.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `client` - Pointer to the [`CosmosClient`].
/// * `database_id` - The database ID as a nul-terminated C string.
/// * `out_database` - Output parameter that will receive a pointer to the created [`DatabaseClient`].
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, client = ?client))]
pub extern "C" fn cosmos_client_database_client(
    ctx: *mut CallContext,
    client: *const CosmosClient,
    database_id: *const c_char,
    out_database: *mut *mut DatabaseClient,
) -> CosmosErrorCode {
    context!(ctx).run_sync_with_output(out_database, || {
        let client = unwrap_required_ptr(client, error::messages::INVALID_CLIENT_POINTER)?;
        let database_id = parse_cstr(database_id, error::messages::INVALID_DATABASE_ID)?;
        let database_client = client.database_client(database_id);
        Ok(Box::new(database_client))
    })
}

/// Queries the databases in the Cosmos DB account using the provided SQL query string.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `client` - Pointer to the [`CosmosClient`].
/// * `query` - The SQL query string as a nul-terminated C string.
/// * `options` - Pointer to [`QueryOptions`] for query configuration, may be null.
/// * `out_json` - Output parameter that will receive a pointer to the resulting JSON string
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, client = ?client))]
pub extern "C" fn cosmos_client_query_databases(
    ctx: *mut CallContext,
    client: *const CosmosClient,
    query: *const c_char,
    #[allow(
        unused_variables,
        reason = "options parameter is reserved for future use, and prefixing with '_' appears in docs"
    )]
    options: *const QueryOptions,
    out_json: *mut *const c_char,
) -> CosmosErrorCode {
    context!(ctx).run_async_with_output(out_json, async {
        let client = unwrap_required_ptr(client, error::messages::INVALID_CLIENT_POINTER)?;
        let query_str = parse_cstr(query, error::messages::INVALID_QUERY)?;

        let cosmos_query = Query::from(query_str);
        let pager = client.query_databases(cosmos_query, None)?;

        // We don't expose the raw string in a FeedPage, so we need to collect and serialize.
        // We'll evaluate optimizing this later if needed.
        let results = pager.try_collect::<Vec<_>>().await?;
        let json = serde_json::to_string(&results).map_err(|_| {
            Error::new(
                CosmosErrorCode::DataConversion,
                error::messages::INVALID_JSON,
            )
        })?;
        let json = CString::new(json)?;
        Ok(json)
    })
}

/// Creates a new database in the Cosmos DB account with the specified database ID, and returns a pointer to the created [`DatabaseClient`].
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `client` - Pointer to the [`CosmosClient`].
/// * `options` - Pointer to [`CreateDatabaseOptions`] for create database configuration, may be null.
/// * `database_id` - The database ID as a nul-terminated C string.
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, client = ?client))]
pub extern "C" fn cosmos_client_create_database(
    ctx: *mut CallContext,
    client: *const CosmosClient,
    database_id: *const c_char,
    #[allow(
        unused_variables,
        reason = "options parameter is reserved for future use, and prefixing with '_' appears in docs"
    )]
    options: *const CreateDatabaseOptions,
    out_database: *mut *mut DatabaseClient,
) -> CosmosErrorCode {
    context!(ctx).run_async_with_output(out_database, async {
        let client = unwrap_required_ptr(client, error::messages::INVALID_CLIENT_POINTER)?;

        let database_id = parse_cstr(database_id, error::messages::INVALID_DATABASE_ID)?;
        client.create_database(database_id, None).await?;
        Ok(Box::new(client.database_client(database_id)))
    })
}
