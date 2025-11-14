use std::os::raw::c_char;

use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::query::Query;
use futures::TryStreamExt;
use serde_json::value::RawValue;

use crate::blocking::block_on;
use crate::error::{self, marshal_result, CosmosError, CosmosErrorCode};
use crate::string::{parse_cstr, safe_cstring_into_raw};

/// Releases the memory associated with a [`ContainerClient`].
#[no_mangle]
pub extern "C" fn cosmos_container_free(container: *mut ContainerClient) {
    if !container.is_null() {
        unsafe { drop(Box::from_raw(container)) }
    }
}

fn create_item_inner(
    container: &ContainerClient,
    partition_key: &str,
    json_str: &str,
) -> Result<(), CosmosError> {
    let raw_value = RawValue::from_string(json_str.to_string())?;

    // Clone for async - Azure SDK needs owned String for async block
    let pk = partition_key.to_string();
    block_on(container.create_item(pk, raw_value, None))?;
    Ok(())
}

/// Creates a new item in the specified container.
///
/// # Arguments
/// * `container` - Pointer to the `ContainerClient`.
/// * `partition_key` - The partition key value as a nul-terminated C string.
/// * `json_data` - The item data as a raw JSON nul-terminated C string.
/// * `out_error` - Output parameter that will receive error information if the function fails.
#[no_mangle]
pub extern "C" fn cosmos_container_create_item(
    container: *const ContainerClient,
    partition_key: *const c_char,
    json_data: *const c_char,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if container.is_null() || partition_key.is_null() || json_data.is_null() || out_error.is_null()
    {
        return CosmosErrorCode::InvalidArgument;
    }

    let container_handle = unsafe { &*container };

    let partition_key_str = match parse_cstr(partition_key, error::CSTR_INVALID_PARTITION_KEY) {
        Ok(s) => s,
        Err(e) => {
            let code = e.code;
            unsafe {
                *out_error = e;
            }
            return code;
        }
    };

    let json_str = match parse_cstr(json_data, error::CSTR_INVALID_JSON_DATA) {
        Ok(s) => s,
        Err(e) => {
            let code = e.code;
            unsafe {
                *out_error = e;
            }
            return code;
        }
    };

    marshal_result(
        create_item_inner(container_handle, partition_key_str, json_str),
        out_error,
        |_| {},
    )
}

fn upsert_item_inner(
    container: &ContainerClient,
    partition_key: &str,
    json_str: &str,
) -> Result<(), CosmosError> {
    let raw_value: Box<RawValue> = serde_json::from_str(json_str)?;
    let pk = partition_key.to_string();
    block_on(container.upsert_item(pk, raw_value, None))?;
    Ok(())
}

/// Upserts an item in the specified container.
///
/// # Arguments
/// * `container` - Pointer to the `ContainerClient`.
/// * `partition_key` - The partition key value as a nul-terminated C string.
/// * `json_data` - The item data as a raw JSON nul-terminated C string.
/// * `out_error` - Output parameter that will receive error information if the function fails.
#[no_mangle]
pub extern "C" fn cosmos_container_upsert_item(
    container: *const ContainerClient,
    partition_key: *const c_char,
    json_data: *const c_char,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if container.is_null() || partition_key.is_null() || json_data.is_null() || out_error.is_null()
    {
        return CosmosErrorCode::InvalidArgument;
    }

    let container_handle = unsafe { &*container };

    let partition_key_str = match parse_cstr(partition_key, error::CSTR_INVALID_PARTITION_KEY) {
        Ok(s) => s,
        Err(e) => {
            let code = e.code;
            unsafe {
                *out_error = e;
            }
            return code;
        }
    };

    let json_str = match parse_cstr(json_data, error::CSTR_INVALID_JSON_DATA) {
        Ok(s) => s,
        Err(e) => {
            let code = e.code;
            unsafe {
                *out_error = e;
            }
            return code;
        }
    };

    marshal_result(
        upsert_item_inner(container_handle, partition_key_str, json_str),
        out_error,
        |_| {},
    )
}

// Inner function: Returns JSON string
fn read_item_inner(
    container: &ContainerClient,
    partition_key: &str,
    item_id: &str,
) -> Result<String, CosmosError> {
    let pk = partition_key.to_string();

    // The type we read into doesn't matter, because we'll extract the raw string instead of deserializing.
    let response = block_on(container.read_item::<()>(pk, item_id, None))?;
    Ok(response.into_body().into_string()?)
}

/// Reads an item from the specified container.
///
/// # Arguments
/// * `container` - Pointer to the `ContainerClient`.
/// * `partition_key` - The partition key value as a nul-terminated C string.
/// * `item_id` - The ID of the item to read as a nul-terminated C string.
/// * `out_json` - Output parameter that will receive the item data as a raw JSON nul-terminated C string.
/// * `out_error` - Output parameter that will receive error information if the function fails.
#[no_mangle]
pub extern "C" fn cosmos_container_read_item(
    container: *const ContainerClient,
    partition_key: *const c_char,
    item_id: *const c_char,
    out_json: *mut *mut c_char,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if container.is_null()
        || partition_key.is_null()
        || item_id.is_null()
        || out_json.is_null()
        || out_error.is_null()
    {
        return CosmosErrorCode::InvalidArgument;
    }

    let container_handle = unsafe { &*container };

    let partition_key_str = match parse_cstr(partition_key, error::CSTR_INVALID_PARTITION_KEY) {
        Ok(s) => s,
        Err(e) => {
            let code = e.code;
            unsafe {
                *out_error = e;
            }
            return code;
        }
    };

    let item_id_str = match parse_cstr(item_id, error::CSTR_INVALID_ITEM_ID) {
        Ok(s) => s,
        Err(e) => {
            let code = e.code;
            unsafe {
                *out_error = e;
            }
            return code;
        }
    };

    marshal_result(
        read_item_inner(container_handle, partition_key_str, item_id_str),
        out_error,
        |json_string| unsafe {
            let _ = safe_cstring_into_raw(&json_string, &mut *out_json, &mut *out_error);
        },
    )
}

fn replace_item_inner(
    container: &ContainerClient,
    partition_key: &str,
    item_id: &str,
    json_str: &str,
) -> Result<(), CosmosError> {
    let raw_value = RawValue::from_string(json_str.to_string())?;
    let pk = partition_key.to_string();
    block_on(container.replace_item(pk, item_id, raw_value, None))?;
    Ok(())
}

/// Replaces an existing item in the specified container.
///
/// # Arguments
/// * `container` - Pointer to the `ContainerClient`.
/// * `partition_key` - The partition key value as a nul-terminated C string.
/// * `item_id` - The ID of the item to replace as a nul-terminated C string.
/// * `json_data` - The new item data as a raw JSON nul-terminated C string.
/// * `out_error` - Output parameter that will receive error information if the function fails
#[no_mangle]
pub extern "C" fn cosmos_container_replace_item(
    container: *const ContainerClient,
    partition_key: *const c_char,
    item_id: *const c_char,
    json_data: *const c_char,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if container.is_null()
        || partition_key.is_null()
        || item_id.is_null()
        || json_data.is_null()
        || out_error.is_null()
    {
        return CosmosErrorCode::InvalidArgument;
    }

    let container_handle = unsafe { &*container };

    let partition_key_str = match parse_cstr(partition_key, error::CSTR_INVALID_PARTITION_KEY) {
        Ok(s) => s,
        Err(e) => {
            let code = e.code;
            unsafe {
                *out_error = e;
            }
            return code;
        }
    };

    let item_id_str = match parse_cstr(item_id, error::CSTR_INVALID_ITEM_ID) {
        Ok(s) => s,
        Err(e) => {
            let code = e.code;
            unsafe {
                *out_error = e;
            }
            return code;
        }
    };

    let json_str = match parse_cstr(json_data, error::CSTR_INVALID_JSON_DATA) {
        Ok(s) => s,
        Err(e) => {
            let code = e.code;
            unsafe {
                *out_error = e;
            }
            return code;
        }
    };

    marshal_result(
        replace_item_inner(container_handle, partition_key_str, item_id_str, json_str),
        out_error,
        |_| {},
    )
}

fn delete_item_inner(
    container: &ContainerClient,
    partition_key: &str,
    item_id: &str,
) -> Result<(), CosmosError> {
    let pk = partition_key.to_string();
    block_on(container.delete_item(pk, item_id, None))?;
    Ok(())
}

/// Deletes an item from the specified container.
///
/// # Arguments
/// * `container` - Pointer to the `ContainerClient`.
/// * `partition_key` - The partition key value as a nul-terminated C string.
/// * `item_id` - The ID of the item to delete as a nul-terminated C string.
/// * `out_error` - Output parameter that will receive error information if the function fails
#[no_mangle]
pub extern "C" fn cosmos_container_delete_item(
    container: *const ContainerClient,
    partition_key: *const c_char,
    item_id: *const c_char,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if container.is_null() || partition_key.is_null() || item_id.is_null() || out_error.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }

    let container_handle = unsafe { &*container };

    let partition_key_str = match parse_cstr(partition_key, error::CSTR_INVALID_PARTITION_KEY) {
        Ok(s) => s,
        Err(e) => {
            let code = e.code;
            unsafe {
                *out_error = e;
            }
            return code;
        }
    };

    let item_id_str = match parse_cstr(item_id, error::CSTR_INVALID_ITEM_ID) {
        Ok(s) => s,
        Err(e) => {
            let code = e.code;
            unsafe {
                *out_error = e;
            }
            return code;
        }
    };

    marshal_result(
        delete_item_inner(container_handle, partition_key_str, item_id_str),
        out_error,
        |_| {},
    )
}

// TODO: Patch

fn read_container_inner(container: &ContainerClient) -> Result<String, CosmosError> {
    let response = block_on(container.read(None))?;
    Ok(response.into_body().into_string()?)
}

/// Reads the properties of the specified container.
///
/// # Arguments
/// * `container` - Pointer to the `ContainerClient`.
/// * `out_json` - Output parameter that will receive the container properties as a raw JSON nul-terminated C string.
/// * `out_error` - Output parameter that will receive error information if the function fails.
#[no_mangle]
pub extern "C" fn cosmos_container_read(
    container: *const ContainerClient,
    out_json: *mut *mut c_char,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if container.is_null() || out_json.is_null() || out_error.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }

    let container_handle = unsafe { &*container };

    marshal_result(
        read_container_inner(container_handle),
        out_error,
        |json_string| unsafe {
            let _ = safe_cstring_into_raw(&json_string, &mut *out_json, &mut *out_error);
        },
    )
}

fn query_items_inner(
    container: &ContainerClient,
    query_str: &str,
    partition_key_opt: Option<&str>,
) -> Result<String, CosmosError> {
    let cosmos_query = Query::from(query_str);
    let pk_owned = partition_key_opt.map(|s| s.to_string());

    let pager = if let Some(pk) = pk_owned {
        container.query_items::<Box<RawValue>>(cosmos_query, pk, None)?
    } else {
        container.query_items::<Box<RawValue>>(cosmos_query, (), None)?
    };

    // We don't expose the raw string in a FeedPage, so we need to collect and serialize.
    // We'll evaluate optimizing this later if needed.
    let results = block_on(pager.try_collect::<Vec<_>>())?;
    serde_json::to_string(&results).map_err(|_| {
        CosmosError::from_static_cstr(CosmosErrorCode::DataConversion, error::CSTR_INVALID_JSON)
    })
}

/// Queries items in the specified container.
///
/// # Arguments
/// * `container` - Pointer to the `ContainerClient`.
/// * `query` - The query to execute as a nul-terminated C string.
/// * `partition_key` - Optional partition key value as a nul-terminated C string. Specify a null pointer for a cross-partition query.
/// * `out_json` - Output parameter that will receive the query results as a raw JSON nul-terminated C string.
/// * `out_error` - Output parameter that will receive error information if the function fails.
#[no_mangle]
pub extern "C" fn cosmos_container_query_items(
    container: *const ContainerClient,
    query: *const c_char,
    partition_key: *const c_char,
    out_json: *mut *mut c_char,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if container.is_null() || query.is_null() || out_json.is_null() || out_error.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }

    let container_handle = unsafe { &*container };

    let query_str = match parse_cstr(query, error::CSTR_INVALID_QUERY) {
        Ok(s) => s,
        Err(e) => {
            let code = e.code;
            unsafe {
                *out_error = e;
            }
            return code;
        }
    };

    let partition_key_opt = if partition_key.is_null() {
        None
    } else {
        match parse_cstr(partition_key, error::CSTR_INVALID_PARTITION_KEY) {
            Ok("") => None,
            Ok(s) => Some(s),
            Err(e) => {
                let code = e.code;
                unsafe {
                    *out_error = e;
                }
                return code;
            }
        }
    };

    marshal_result(
        query_items_inner(container_handle, query_str, partition_key_opt),
        out_error,
        |json_string| unsafe {
            let _ = safe_cstring_into_raw(&json_string, &mut *out_json, &mut *out_error);
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_container_crud_operations_null_validation() {
        assert_eq!(
            cosmos_container_create_item(ptr::null(), ptr::null(), ptr::null(), ptr::null_mut()),
            CosmosErrorCode::InvalidArgument
        );

        assert_eq!(
            cosmos_container_read_item(
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null_mut(),
                ptr::null_mut()
            ),
            CosmosErrorCode::InvalidArgument
        );

        assert_eq!(
            cosmos_container_replace_item(
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null_mut()
            ),
            CosmosErrorCode::InvalidArgument
        );

        assert_eq!(
            cosmos_container_delete_item(ptr::null(), ptr::null(), ptr::null(), ptr::null_mut()),
            CosmosErrorCode::InvalidArgument
        );

        assert_eq!(
            cosmos_container_query_items(
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null_mut(),
                ptr::null_mut()
            ),
            CosmosErrorCode::InvalidArgument
        );

        assert_eq!(
            cosmos_container_read(ptr::null(), ptr::null_mut(), ptr::null_mut()),
            CosmosErrorCode::InvalidArgument
        );
    }
}
