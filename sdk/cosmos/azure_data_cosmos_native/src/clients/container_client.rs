// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::ffi::CString;
use std::os::raw::c_char;

use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::query::Query;
use futures::TryStreamExt;
use serde_json::value::RawValue;

use crate::context::CallContext;
use crate::error::{self, CosmosErrorCode, Error};
use crate::options::{DeleteContainerOptions, ItemOptions, QueryOptions, ReadContainerOptions};
use crate::string::parse_cstr;
use crate::unwrap_required_ptr;

/// Releases the memory associated with a [`ContainerClient`].
#[no_mangle]
#[tracing::instrument(level = "debug")]
pub extern "C" fn cosmos_container_free(container: *mut ContainerClient) {
    if !container.is_null() {
        tracing::trace!(?container, "freeing container client");
        unsafe { drop(Box::from_raw(container)) }
    }
}

/// Creates a new item in the specified container.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `container` - Pointer to the `ContainerClient`.
/// * `partition_key` - The partition key value as a nul-terminated C string.
/// * `json_data` - The item data as a raw JSON nul-terminated C string.
/// * `options` - Pointer to [`ItemOptions`] for item creation configuration, may be null.
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, container = ?container))]
pub extern "C" fn cosmos_container_create_item(
    ctx: *mut CallContext,
    container: *const ContainerClient,
    partition_key: *const c_char,
    json_data: *const c_char,
    #[allow(
        unused_variables,
        reason = "options parameter is reserved for future use, and prefixing with '_' appears in docs"
    )]
    options: *const ItemOptions,
) -> CosmosErrorCode {
    context!(ctx).run_async(async {
        let container = unwrap_required_ptr(container, error::messages::INVALID_CONTAINER_POINTER)?;
        let partition_key =
            parse_cstr(partition_key, error::messages::INVALID_PARTITION_KEY)?.to_string();
        let json = parse_cstr(json_data, error::messages::INVALID_JSON)?.to_string();
        let raw_value = RawValue::from_string(json)?;
        container
            .create_item(partition_key, raw_value, None)
            .await?;
        Ok(())
    })
}

/// Upserts an item in the specified container.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `container` - Pointer to the `ContainerClient`.
/// * `partition_key` - The partition key value as a nul-terminated C string.
/// * `json_data` - The item data as a raw JSON nul-terminated C string.
/// * `options` - Pointer to [`ItemOptions`] for item upsert configuration, may be null.
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, container = ?container))]
pub extern "C" fn cosmos_container_upsert_item(
    ctx: *mut CallContext,
    container: *const ContainerClient,
    partition_key: *const c_char,
    json_data: *const c_char,
    #[allow(
        unused_variables,
        reason = "options parameter is reserved for future use, and prefixing with '_' appears in docs"
    )]
    options: *const ItemOptions,
) -> CosmosErrorCode {
    context!(ctx).run_async(async {
        let container = unwrap_required_ptr(container, error::messages::INVALID_CONTAINER_POINTER)?;
        let partition_key =
            parse_cstr(partition_key, error::messages::INVALID_PARTITION_KEY)?.to_string();
        let json = parse_cstr(json_data, error::messages::INVALID_JSON)?.to_string();
        let raw_value = RawValue::from_string(json)?;
        container
            .upsert_item(partition_key, raw_value, None)
            .await?;
        Ok(())
    })
}

/// Reads an item from the specified container.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `container` - Pointer to the `ContainerClient`.
/// * `partition_key` - The partition key value as a nul-terminated C string.
/// * `item_id` - The ID of the item to read as a nul-terminated C string.
/// * `options` - Pointer to [`ItemOptions`] for item read configuration, may be null.
/// * `out_json` - Output parameter that will receive the item data as a raw JSON nul-terminated C string.
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, container = ?container))]
pub extern "C" fn cosmos_container_read_item(
    ctx: *mut CallContext,
    container: *const ContainerClient,
    partition_key: *const c_char,
    item_id: *const c_char,
    #[allow(
        unused_variables,
        reason = "options parameter is reserved for future use, and prefixing with '_' appears in docs"
    )]
    options: *const ItemOptions,
    out_json: *mut *const c_char,
) -> CosmosErrorCode {
    context!(ctx).run_async_with_output(out_json, async {
        let container = unwrap_required_ptr(container, error::messages::INVALID_CONTAINER_POINTER)?;
        let partition_key =
            parse_cstr(partition_key, error::messages::INVALID_PARTITION_KEY)?.to_string();
        let item_id = parse_cstr(item_id, error::messages::INVALID_ITEM_ID)?;

        // We can specify '()' as the type parameter because we only want the raw JSON string.
        let response = container
            .read_item::<()>(partition_key, item_id, None)
            .await?;
        let body = response.into_body().into_string()?;

        Ok(CString::new(body)?)
    })
}

/// Replaces an existing item in the specified container.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `container` - Pointer to the `ContainerClient`.
/// * `partition_key` - The partition key value as a nul-terminated C string.
/// * `item_id` - The ID of the item to replace as a nul-terminated C string.
/// * `json_data` - The new item data as a raw JSON nul-terminated C string.
/// * `options` - Pointer to [`ItemOptions`] for item replacement configuration, may be null.
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, container = ?container))]
pub extern "C" fn cosmos_container_replace_item(
    ctx: *mut CallContext,
    container: *const ContainerClient,
    partition_key: *const c_char,
    item_id: *const c_char,
    json_data: *const c_char,
    #[allow(
        unused_variables,
        reason = "options parameter is reserved for future use, and prefixing with '_' appears in docs"
    )]
    options: *const ItemOptions,
) -> CosmosErrorCode {
    context!(ctx).run_async(async {
        let container = unwrap_required_ptr(container, error::messages::INVALID_CONTAINER_POINTER)?;
        let partition_key =
            parse_cstr(partition_key, error::messages::INVALID_PARTITION_KEY)?.to_string();
        let item_id = parse_cstr(item_id, error::messages::INVALID_ITEM_ID)?;
        let json = parse_cstr(json_data, error::messages::INVALID_JSON)?.to_string();

        let raw_value = RawValue::from_string(json)?;
        let pk = partition_key.to_string();
        container.replace_item(pk, item_id, raw_value, None).await?;
        Ok(())
    })
}

/// Deletes an item from the specified container.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `container` - Pointer to the `ContainerClient`.
/// * `partition_key` - The partition key value as a nul-terminated C string.
/// * `item_id` - The ID of the item to delete as a nul-terminated C string.
/// * `options` - Pointer to [`ItemOptions`] for item deletion configuration, may be null.
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, container = ?container))]
pub extern "C" fn cosmos_container_delete_item(
    ctx: *mut CallContext,
    container: *const ContainerClient,
    partition_key: *const c_char,
    item_id: *const c_char,
    #[allow(
        unused_variables,
        reason = "options parameter is reserved for future use, and prefixing with '_' appears in docs"
    )]
    options: *const ItemOptions,
) -> CosmosErrorCode {
    context!(ctx).run_async(async {
        let container = unwrap_required_ptr(container, error::messages::INVALID_CONTAINER_POINTER)?;
        let partition_key =
            parse_cstr(partition_key, error::messages::INVALID_PARTITION_KEY)?.to_string();
        let item_id = parse_cstr(item_id, error::messages::INVALID_ITEM_ID)?;
        container.delete_item(partition_key, item_id, None).await?;
        Ok(())
    })
}

// TODO: Patch

/// Reads the properties of the specified container.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `container` - Pointer to the `ContainerClient`.
/// * `options` - Pointer to [`ReadContainerOptions`] for read container configuration, may be null.
/// * `out_json` - Output parameter that will receive the container properties as a raw JSON nul-terminated C string.
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, container = ?container))]
pub extern "C" fn cosmos_container_read(
    ctx: *mut CallContext,
    container: *const ContainerClient,
    #[allow(
        unused_variables,
        reason = "options parameter is reserved for future use, and prefixing with '_' appears in docs"
    )]
    options: *const ReadContainerOptions,
    out_json: *mut *const c_char,
) -> CosmosErrorCode {
    context!(ctx).run_async_with_output(out_json, async {
        let container = unwrap_required_ptr(container, error::messages::INVALID_CONTAINER_POINTER)?;
        let response = container.read(None).await?;
        let body = response.into_body().into_string()?;
        Ok(CString::new(body)?)
    })
}

/// Deletes the specified container.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `container` - Pointer to the [`ContainerClient`].
/// * `options` - Pointer to [`DeleteContainerOptions`] for delete container configuration, may be null.
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, container = ?container))]
pub extern "C" fn cosmos_container_delete(
    ctx: *mut CallContext,
    container: *const ContainerClient,
    #[allow(
        unused_variables,
        reason = "options parameter is reserved for future use, and prefixing with '_' appears in docs"
    )]
    options: *const DeleteContainerOptions,
) -> CosmosErrorCode {
    context!(ctx).run_async(async {
        let container = unwrap_required_ptr(container, error::messages::INVALID_CONTAINER_POINTER)?;
        container.delete(None).await?;
        Ok(())
    })
}

/// Queries items in the specified container.
///
/// # Arguments
/// * `ctx` - Pointer to a [`CallContext`] to use for this call.
/// * `container` - Pointer to the `ContainerClient`.
/// * `query` - The query to execute as a nul-terminated C string.
/// * `partition_key` - Optional partition key value as a nul-terminated C string. Specify a null pointer for a cross-partition query.
/// * `options` - Pointer to [`QueryOptions`] for query configuration, may be null.
/// * `out_json` - Output parameter that will receive the query results as a raw JSON nul-terminated C string.
#[no_mangle]
#[tracing::instrument(level = "debug", skip_all, fields(ctx = ?ctx, container = ?container))]
pub extern "C" fn cosmos_container_query_items(
    ctx: *mut CallContext,
    container: *const ContainerClient,
    query: *const c_char,
    partition_key: *const c_char,
    #[allow(
        unused_variables,
        reason = "options parameter is reserved for future use, and prefixing with '_' appears in docs"
    )]
    options: *const QueryOptions,
    out_json: *mut *const c_char,
) -> CosmosErrorCode {
    context!(ctx).run_async_with_output(out_json, async {
        let container = unwrap_required_ptr(container, error::messages::INVALID_CONTAINER_POINTER)?;
        let query = Query::from(parse_cstr(query, error::messages::INVALID_QUERY)?);

        let partition_key = if partition_key.is_null() {
            None
        } else {
            Some(parse_cstr(partition_key, error::messages::INVALID_PARTITION_KEY)?.to_string())
        };

        let pager = if let Some(pk) = partition_key {
            container.query_items::<Box<RawValue>>(query, pk, None)?
        } else {
            container.query_items::<Box<RawValue>>(query, (), None)?
        };

        // We don't expose the raw string in a FeedPage, so we need to collect and serialize.
        // We'll evaluate optimizing this later if needed.
        let results = pager.try_collect::<Vec<_>>().await?;
        let json = serde_json::to_string(&results).map_err(|_| {
            Error::new(
                CosmosErrorCode::DataConversion,
                error::messages::INVALID_JSON,
            )
        })?;
        Ok(CString::new(json)?)
    })
}
