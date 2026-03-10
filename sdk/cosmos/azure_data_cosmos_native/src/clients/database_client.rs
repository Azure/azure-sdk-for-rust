// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::ffi::CString;
use std::os::raw::c_char;

use azure_data_cosmos::clients::{ContainerClient, DatabaseClient};
use azure_data_cosmos::models::ContainerProperties;
use azure_data_cosmos::query::Query;
use futures::TryStreamExt;

use crate::context::CallContext;
use crate::error::{self, CosmosErrorCode, Error};
use crate::options::{
    CreateContainerOptions, DeleteDatabaseOptions, QueryOptions, ReadDatabaseOptions,
};
use crate::string::parse_cstr;
use crate::unwrap_required_ptr;

/// Releases the memory associated with a [`DatabaseClient`].
#[no_mangle]
#[tracing::instrument(level = "debug")]
pub extern "C" fn cosmos_database_free(database: *mut DatabaseClient) {
    if !database.is_null() {
        tracing::trace!(?database, "freeing database client");
        unsafe { drop(Box::from_raw(database)) }
    }
}

/// Retrieves a pointer to a [`ContainerClient`] for the specified container ID within the given database.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `database` - Pointer to the [`DatabaseClient`].
/// * `container_id` - The container ID as a nul-terminated C string.
/// * `out_container` - Output parameter that will receive a pointer to the [`ContainerClient`].
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, database = ?database))]
pub extern "C" fn cosmos_database_container_client(
    ctx: *mut CallContext,
    database: *const DatabaseClient,
    container_id: *const c_char,
    out_container: *mut *mut ContainerClient,
) -> CosmosErrorCode {
    context!(ctx).run_async_with_output(out_container, async {
        let database = unwrap_required_ptr(database, error::messages::INVALID_DATABASE_POINTER)?;
        let container_id = parse_cstr(container_id, error::messages::INVALID_CONTAINER_ID)?;
        let container_client = database.container_client(container_id).await;
        Ok(Box::new(container_client))
    })
}

/// Reads the properties of the specified database and returns them as a JSON string.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `database` - Pointer to the [`DatabaseClient`].
/// * `options` - Pointer to [`ReadDatabaseOptions`] for read configuration, may be null.
/// * `out_json` - Output parameter that will receive a pointer to the JSON string.
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, database = ?database))]
pub extern "C" fn cosmos_database_read(
    ctx: *mut CallContext,
    database: *const DatabaseClient,
    #[allow(
        unused_variables,
        reason = "options parameter is reserved for future use, and prefixing with '_' appears in docs"
    )]
    options: *const ReadDatabaseOptions,
    out_json: *mut *const c_char,
) -> CosmosErrorCode {
    context!(ctx).run_async_with_output(out_json, async {
        let database = unwrap_required_ptr(database, error::messages::INVALID_DATABASE_POINTER)?;
        let response = database.read(None).await?;
        let json = response.into_body().into_string()?;
        Ok(CString::new(json)?)
    })
}

/// Deletes the specified database.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `database` - Pointer to the [`DatabaseClient`].
/// * `options` - Pointer to [`DeleteDatabaseOptions`] for delete configuration, may be null.
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, database = ?database))]
pub extern "C" fn cosmos_database_delete(
    ctx: *mut CallContext,
    database: *const DatabaseClient,
    #[allow(
        unused_variables,
        reason = "options parameter is reserved for future use, and prefixing with '_' appears in docs"
    )]
    options: *const DeleteDatabaseOptions,
) -> CosmosErrorCode {
    context!(ctx).run_async(async {
        let database = unwrap_required_ptr(database, error::messages::INVALID_DATABASE_POINTER)?;
        database.delete(None).await?;
        Ok(())
    })
}

/// Creates a new container within the specified database.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `database` - Pointer to the [`DatabaseClient`].
/// * `container_id` - The container ID as a nul-terminated C string.
/// * `partition_key_path` - The partition key path as a nul-terminated C string.
/// * `options` - Pointer to [`CreateContainerOptions`] for create container configuration, may be null.
/// * `out_container` - Output parameter that will receive a pointer to the newly created [`ContainerClient`].
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, database = ?database))]
pub extern "C" fn cosmos_database_create_container(
    ctx: *mut CallContext,
    database: *const DatabaseClient,
    container_id: *const c_char,
    partition_key_path: *const c_char,
    #[allow(
        unused_variables,
        reason = "options parameter is reserved for future use, and prefixing with '_' appears in docs"
    )]
    options: *const CreateContainerOptions,
    out_container: *mut *mut ContainerClient,
) -> CosmosErrorCode {
    context!(ctx).run_async_with_output(out_container, async {
        let database = unwrap_required_ptr(database, error::messages::INVALID_DATABASE_POINTER)?;

        let container_id =
            parse_cstr(container_id, error::messages::INVALID_CONTAINER_ID)?.to_string();
        let partition_key_path =
            parse_cstr(partition_key_path, error::messages::INVALID_PARTITION_KEY)?.to_string();
        let properties =
            ContainerProperties::new(container_id.clone(), partition_key_path.clone().into());

        database.create_container(properties, None).await?;

        let container_client = database.container_client(&container_id).await;

        Ok(Box::new(container_client))
    })
}

/// Queries the containers within the specified database and returns the results as a JSON string.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `database` - Pointer to the [`DatabaseClient`].
/// * `query` - The query string as a nul-terminated C string.
/// * `options` - Pointer to [`QueryOptions`] for query configuration, may be null.
/// * `out_json` - Output parameter that will receive a pointer to the JSON string.
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, database = ?database))]
pub extern "C" fn cosmos_database_query_containers(
    ctx: *mut CallContext,
    database: *const DatabaseClient,
    query: *const c_char,
    #[allow(
        unused_variables,
        reason = "options parameter is reserved for future use, and prefixing with '_' appears in docs"
    )]
    options: *const QueryOptions,
    out_json: *mut *const c_char,
) -> CosmosErrorCode {
    context!(ctx).run_async_with_output(out_json, async {
        let database = unwrap_required_ptr(database, error::messages::INVALID_DATABASE_POINTER)?;

        let query = parse_cstr(query, error::messages::INVALID_QUERY)?;
        let cosmos_query = Query::from(query);
        let pager = database.query_containers(cosmos_query, None)?;

        // We don't expose the raw string in a FeedPage, so we need to collect and serialize.
        // We'll evaluate optimizing this later if needed.
        let results = pager.try_collect::<Vec<_>>().await?;
        let s = serde_json::to_string(&results).map_err(|_| {
            Error::new(
                CosmosErrorCode::DataConversion,
                error::messages::INVALID_JSON,
            )
        })?;
        Ok(CString::new(s)?)
    })
}
