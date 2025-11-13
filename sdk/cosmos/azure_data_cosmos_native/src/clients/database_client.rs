use std::os::raw::c_char;

use azure_data_cosmos::clients::{ContainerClient, DatabaseClient};
use azure_data_cosmos::models::ContainerProperties;
use azure_data_cosmos::query::Query;
use futures::TryStreamExt;

use crate::blocking::block_on;
use crate::error::{self, marshal_result, CosmosError, CosmosErrorCode};
use crate::string::{parse_cstr, safe_cstring_into_raw};
use crate::{ContainerClientHandle, DatabaseClientHandle};

/// Releases the memory associated with a [`DatabaseClient`].
#[no_mangle]
pub extern "C" fn cosmos_database_free(database: *mut DatabaseClientHandle) {
    if !database.is_null() {
        unsafe { DatabaseClientHandle::free_ptr(database) }
    }
}

fn container_client_inner(
    database: &DatabaseClient,
    container_id_str: &str,
) -> Result<Box<ContainerClient>, CosmosError> {
    let container_client = database.container_client(container_id_str);
    Ok(Box::new(container_client.into()))
}

/// Retrieves a pointer to a [`ContainerClient`] for the specified container ID within the given database.
///
/// # Arguments
/// * `database` - Pointer to the [`DatabaseClient`].
/// * `container_id` - The container ID as a nul-terminated C string.
/// * `out_container` - Output parameter that will receive a pointer to the [`ContainerClient`].
/// * `out_error` - Output parameter that will receive error information if the function fails.
#[no_mangle]
pub extern "C" fn cosmos_database_container_client(
    database: *const DatabaseClientHandle,
    container_id: *const c_char,
    out_container: *mut *mut ContainerClientHandle,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if database.is_null()
        || container_id.is_null()
        || out_container.is_null()
        || out_error.is_null()
    {
        return CosmosErrorCode::InvalidArgument;
    }

    let database_handle = unsafe { DatabaseClientHandle::unwrap_ptr(database) };

    let container_id_str = match parse_cstr(container_id, error::CSTR_INVALID_CONTAINER_ID) {
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
        container_client_inner(database_handle, container_id_str),
        out_error,
        |container_handle| unsafe {
            *out_container = ContainerClientHandle::wrap_ptr(container_handle);
        },
    )
}

fn read_database_inner(database: &DatabaseClient) -> Result<String, CosmosError> {
    let response = block_on(database.read(None))?;
    Ok(response.into_body().into_string()?)
}

/// Reads the properties of the specified database and returns them as a JSON string.
///
/// # Arguments
/// * `database` - Pointer to the [`DatabaseClient`].
/// * `out_json` - Output parameter that will receive a pointer to the JSON string.
/// * `out_error` - Output parameter that will receive error information if the function fails.
#[no_mangle]
pub extern "C" fn cosmos_database_read(
    database: *const DatabaseClient,
    out_json: *mut *mut c_char,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if database.is_null() || out_json.is_null() || out_error.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }

    let database_handle = unsafe { &*database };

    marshal_result(
        read_database_inner(database_handle),
        out_error,
        |json_string| unsafe {
            let _ = safe_cstring_into_raw(&json_string, &mut *out_json, &mut *out_error);
        },
    )
}

fn delete_database_inner(database: &DatabaseClient) -> Result<(), CosmosError> {
    block_on(database.delete(None))?;
    Ok(())
}

/// Deletes the specified database.
///
/// # Arguments
/// * `database` - Pointer to the [`DatabaseClient`].
/// * `out_error` - Output parameter that will receive error information if the function fails.
#[no_mangle]
pub extern "C" fn cosmos_database_delete(
    database: *const DatabaseClient,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if database.is_null() || out_error.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }

    let database_handle = unsafe { &*database };

    marshal_result(delete_database_inner(database_handle), out_error, |_| {})
}

fn create_container_inner(
    database: &DatabaseClient,
    container_id_str: &str,
    partition_key_path_str: &str,
) -> Result<Box<ContainerClient>, CosmosError> {
    let container_id_owned = container_id_str.to_string();
    let partition_key_owned = partition_key_path_str.to_string();

    let properties = ContainerProperties {
        id: container_id_owned.clone().into(),
        partition_key: partition_key_owned.clone().into(),
        ..Default::default()
    };

    block_on(database.create_container(properties, None))?;

    let container_client = database.container_client(&container_id_owned);

    Ok(Box::new(container_client.into()))
}

/// Creates a new container within the specified database.
///
/// # Arguments
/// * `database` - Pointer to the [`DatabaseClient`].
/// * `container_id` - The container ID as a nul-terminated C string.
/// * `partition_key_path` - The partition key path as a nul-terminated C string.
/// * `out_container` - Output parameter that will receive a pointer to the newly created [`ContainerClient`].
/// * `out_error` - Output parameter that will receive error information if the function fails.
#[no_mangle]
pub extern "C" fn cosmos_database_create_container(
    database: *const DatabaseClient,
    container_id: *const c_char,
    partition_key_path: *const c_char,
    out_container: *mut *mut ContainerClient,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if database.is_null()
        || container_id.is_null()
        || partition_key_path.is_null()
        || out_container.is_null()
        || out_error.is_null()
    {
        return CosmosErrorCode::InvalidArgument;
    }

    let database_handle = unsafe { &*database };

    let container_id_str = match parse_cstr(container_id, error::CSTR_INVALID_CONTAINER_ID) {
        Ok(s) => s,
        Err(e) => {
            let code = e.code;
            unsafe {
                *out_error = e;
                *out_container = std::ptr::null_mut();
            }
            return code;
        }
    };

    let partition_key_path_str =
        match parse_cstr(partition_key_path, error::CSTR_INVALID_PARTITION_KEY) {
            Ok(s) => s,
            Err(e) => {
                let code = e.code;
                unsafe {
                    *out_error = e;
                    *out_container = std::ptr::null_mut();
                }
                return code;
            }
        };

    marshal_result(
        create_container_inner(database_handle, container_id_str, partition_key_path_str),
        out_error,
        |container_handle| unsafe {
            *out_container = Box::into_raw(container_handle);
        },
    )
}

fn query_containers_inner(
    database: &DatabaseClient,
    query_str: &str,
) -> Result<String, CosmosError> {
    let cosmos_query = Query::from(query_str);
    let pager = database.query_containers(cosmos_query, None)?;

    // We don't expose the raw string in a FeedPage, so we need to collect and serialize.
    // We'll evaluate optimizing this later if needed.
    let results = block_on(pager.try_collect::<Vec<_>>())?;
    serde_json::to_string(&results).map_err(|_| {
        CosmosError::from_static_cstr(CosmosErrorCode::DataConversion, error::CSTR_INVALID_JSON)
    })
}

/// Queries the containers within the specified database and returns the results as a JSON string.
///
/// # Arguments
/// * `database` - Pointer to the [`DatabaseClient`].
/// * `query` - The query string as a nul-terminated C string.
/// * `out_json` - Output parameter that will receive a pointer to the JSON string.
/// * `out_error` - Output parameter that will receive error information if the function fails.
#[no_mangle]
pub extern "C" fn cosmos_database_query_containers(
    database: *const DatabaseClient,
    query: *const c_char,
    out_json: *mut *mut c_char,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if database.is_null() || query.is_null() || out_json.is_null() || out_error.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }

    let database_handle = unsafe { &*database };

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

    marshal_result(
        query_containers_inner(database_handle, query_str),
        out_error,
        |json_string| unsafe {
            let _ = safe_cstring_into_raw(&json_string, &mut *out_json, &mut *out_error);
        },
    )
}

// TODO: Add more database operations following Azure SDK pattern:
// - Additional advanced database operations if needed

#[cfg(test)]
mod tests {
    use crate::{
        cosmos_client_create, cosmos_client_database_client, cosmos_client_free,
        cosmos_container_free, ContainerClientHandle, CosmosClientHandle,
    };

    use super::*;
    use std::{ffi::CString, ptr};

    #[test]
    fn test_database_container_client_null_params() {
        let mut container_ptr: *mut ContainerClientHandle = ptr::null_mut();
        let mut error = CosmosError::success();

        let result = cosmos_database_container_client(
            ptr::null(),
            ptr::null(),
            &mut container_ptr,
            &mut error,
        );

        assert_eq!(result, CosmosErrorCode::InvalidArgument);
        assert!(container_ptr.is_null());
    }

    #[test]
    fn test_database_read_null_params() {
        let mut json_ptr: *mut c_char = ptr::null_mut();
        let mut error = CosmosError::success();

        let result = cosmos_database_read(ptr::null(), &mut json_ptr, &mut error);

        assert_eq!(result, CosmosErrorCode::InvalidArgument);
        assert!(json_ptr.is_null());
    }

    #[test]
    fn test_database_delete_null_params() {
        let mut error = CosmosError::success();

        let result = cosmos_database_delete(ptr::null(), &mut error);

        assert_eq!(result, CosmosErrorCode::InvalidArgument);
    }

    #[test]
    fn test_database_create_container_null_params() {
        let mut container_ptr: *mut ContainerClient = ptr::null_mut();
        let mut error = CosmosError::success();

        // Test null database
        let result = cosmos_database_create_container(
            ptr::null(),
            ptr::null(),
            ptr::null(),
            &mut container_ptr,
            &mut error,
        );

        assert_eq!(result, CosmosErrorCode::InvalidArgument);
        assert!(container_ptr.is_null());

        // Reset for next test
        container_ptr = ptr::null_mut();
        error = CosmosError::success();

        // Test with valid database but null container_id
        // Note: We can't create a real DatabaseClientHandle without Azure SDK setup,
        // so we test the null parameter validation only
        let result = cosmos_database_create_container(
            ptr::null(),
            ptr::null(),
            c"/test".as_ptr() as *const c_char,
            &mut container_ptr,
            &mut error,
        );

        assert_eq!(result, CosmosErrorCode::InvalidArgument);
        assert!(container_ptr.is_null());
    }

    #[test]
    fn test_database_query_containers_null_params() {
        let mut json_ptr: *mut c_char = ptr::null_mut();
        let mut error = CosmosError::success();

        let result =
            cosmos_database_query_containers(ptr::null(), ptr::null(), &mut json_ptr, &mut error);

        assert_eq!(result, CosmosErrorCode::InvalidArgument);
        assert!(json_ptr.is_null());
    }

    #[test]
    fn test_database_container_client() {
        let endpoint = CString::new("https://test.documents.azure.com")
            .expect("test string should not contain NUL");
        let key = CString::new("test-key").expect("test string should not contain NUL");
        let db_id = CString::new("test-db").expect("test string should not contain NUL");

        let mut client_ptr: *mut CosmosClientHandle = ptr::null_mut();
        let mut db_ptr: *mut DatabaseClientHandle = ptr::null_mut();
        let mut container_ptr: *mut ContainerClientHandle = ptr::null_mut();
        let mut error = CosmosError::success();

        cosmos_client_create(endpoint.as_ptr(), key.as_ptr(), &mut client_ptr, &mut error);
        assert!(!client_ptr.is_null());

        cosmos_client_database_client(client_ptr, db_id.as_ptr(), &mut db_ptr, &mut error);
        assert!(!db_ptr.is_null());

        let result = cosmos_database_container_client(
            db_ptr,
            c"test-container".as_ptr() as *const c_char,
            &mut container_ptr,
            &mut error,
        );
        assert_eq!(result, CosmosErrorCode::Success);
        assert!(!container_ptr.is_null());

        cosmos_container_free(container_ptr);
        cosmos_database_free(db_ptr);
        cosmos_client_free(client_ptr);
    }
}
