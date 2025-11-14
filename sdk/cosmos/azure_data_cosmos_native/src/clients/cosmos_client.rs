use std::os::raw::c_char;

use azure_core::credentials::Secret;
use azure_data_cosmos::clients::DatabaseClient;
use azure_data_cosmos::query::Query;
use azure_data_cosmos::CosmosClient;
use futures::TryStreamExt;

use crate::blocking::block_on;
use crate::error::{self, marshal_result, CosmosError, CosmosErrorCode};
use crate::string::{parse_cstr, safe_cstring_into_raw};

fn create_client_inner(
    endpoint_str: &str,
    key_str: &str,
) -> Result<Box<CosmosClient>, CosmosError> {
    let key_owned = key_str.to_string();
    let client = azure_data_cosmos::CosmosClient::with_key(
        endpoint_str,
        Secret::new(key_owned.clone()),
        None,
    )?;
    Ok(Box::new(client))
}

/// Creates a new CosmosClient and returns a pointer to it via the out parameter.
///
/// # Arguments
/// * `endpoint` - The Cosmos DB account endpoint, as a nul-terminated C string.
/// * `key` - The Cosmos DB account key, as a nul-terminated C string
/// * `out_client` - Output parameter that will receive a pointer to the created CosmosClient.
/// * `out_error` - Output parameter that will receive error information if the function fails.
///
/// # Returns
/// * Returns [`CosmosErrorCode::Success`] on success.
/// * Returns [`CosmosErrorCode::InvalidArgument`] if any input pointer is null or if the input strings are invalid.
#[no_mangle]
pub extern "C" fn cosmos_client_create_with_key(
    endpoint: *const c_char,
    key: *const c_char,
    out_client: *mut *mut CosmosClient,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if endpoint.is_null() || key.is_null() || out_client.is_null() || out_error.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }

    let endpoint_str = match parse_cstr(endpoint, error::CSTR_INVALID_ENDPOINT) {
        Ok(s) => s,
        Err(e) => {
            let code = e.code;
            unsafe {
                *out_error = e;
            }
            return code;
        }
    };

    let key_str = match parse_cstr(key, error::CSTR_INVALID_KEY) {
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
        create_client_inner(endpoint_str, key_str),
        out_error,
        |handle| unsafe {
            *out_client = Box::into_raw(handle);
        },
    )
}

/// Releases the memory associated with a [`CosmosClient`].
#[no_mangle]
pub extern "C" fn cosmos_client_free(client: *mut CosmosClient) {
    if !client.is_null() {
        unsafe { drop(Box::from_raw(client)) }
    }
}

fn database_client_inner(
    client: &CosmosClient,
    database_id_str: &str,
) -> Result<Box<DatabaseClient>, CosmosError> {
    let database_client = client.database_client(database_id_str);
    Ok(Box::new(database_client))
}

/// Gets a [`DatabaseClient`] from the given [`CosmosClient`] for the specified database ID.
///
/// # Arguments
/// * `client` - Pointer to the [`CosmosClient`].
/// * `database_id` - The database ID as a nul-terminated C string.
/// * `out_database` - Output parameter that will receive a pointer to the created [`DatabaseClient`].
/// * `out_error` - Output parameter that will receive error information if the function fails.
#[no_mangle]
pub extern "C" fn cosmos_client_database_client(
    client: *const CosmosClient,
    database_id: *const c_char,
    out_database: *mut *mut DatabaseClient,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if client.is_null() || database_id.is_null() || out_database.is_null() || out_error.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }

    let client_handle = unsafe { &*client };

    let database_id_str = match parse_cstr(database_id, error::CSTR_INVALID_DATABASE_ID) {
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
        database_client_inner(client_handle, database_id_str),
        out_error,
        |db_handle| unsafe {
            *out_database = Box::into_raw(db_handle);
        },
    )
}

fn query_databases_inner(client: &CosmosClient, query_str: &str) -> Result<String, CosmosError> {
    let cosmos_query = Query::from(query_str);
    let pager = client.query_databases(cosmos_query, None)?;

    // We don't expose the raw string in a FeedPage, so we need to collect and serialize.
    // We'll evaluate optimizing this later if needed.
    let results = block_on(pager.try_collect::<Vec<_>>())?;
    serde_json::to_string(&results).map_err(|_| {
        CosmosError::from_static_cstr(CosmosErrorCode::DataConversion, error::CSTR_INVALID_JSON)
    })
}

/// Queries the databases in the Cosmos DB account using the provided SQL query string.
///
/// # Arguments
/// * `client` - Pointer to the [`CosmosClient`].
/// * `query` - The SQL query string as a nul-terminated C string.
/// * `out_json` - Output parameter that will receive a pointer to the resulting JSON string
/// * `out_error` - Output parameter that will receive error information if the function fails.
#[no_mangle]
pub extern "C" fn cosmos_client_query_databases(
    client: *const CosmosClient,
    query: *const c_char,
    out_json: *mut *mut c_char,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if client.is_null() || query.is_null() || out_json.is_null() || out_error.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }

    let client_handle = unsafe { &*client };

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
        query_databases_inner(client_handle, query_str),
        out_error,
        |json_string| unsafe {
            let _ = safe_cstring_into_raw(&json_string, &mut *out_json, &mut *out_error);
        },
    )
}

fn create_database_inner(
    client: &CosmosClient,
    database_id_str: &str,
) -> Result<Box<DatabaseClient>, CosmosError> {
    block_on(client.create_database(database_id_str, None))?;

    let database_client = client.database_client(database_id_str);

    Ok(Box::new(database_client.into()))
}

/// Creates a new database in the Cosmos DB account with the specified database ID.
///
/// # Arguments
/// * `client` - Pointer to the [`CosmosClient`].
/// * `database_id` - The database ID as a nul-terminated C string.
/// * `out_database` - Output parameter that will receive a pointer to the created [`DatabaseClient`].
/// * `out_error` - Output parameter that will receive error information if the function fails.
#[no_mangle]
pub extern "C" fn cosmos_client_create_database(
    client: *const CosmosClient,
    database_id: *const c_char,
    out_database: *mut *mut DatabaseClient,
    out_error: *mut CosmosError,
) -> CosmosErrorCode {
    if client.is_null() || database_id.is_null() || out_database.is_null() {
        return CosmosErrorCode::InvalidArgument;
    }

    let client_handle = unsafe { &*client };

    let database_id_str = match parse_cstr(database_id, error::CSTR_INVALID_DATABASE_ID) {
        Ok(s) => s,
        Err(e) => {
            let code = e.code;
            unsafe {
                *out_error = e;
                *out_database = std::ptr::null_mut();
            }
            return code;
        }
    };

    marshal_result(
        create_database_inner(client_handle, database_id_str),
        out_error,
        |db_handle| unsafe {
            *out_database = Box::into_raw(db_handle);
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::cosmos_database_free;

    use super::*;
    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn test_cosmos_client_create_valid_params() {
        let endpoint = CString::new("https://test.documents.azure.com")
            .expect("test string should not contain NUL");
        let key = CString::new("test-key").expect("test string should not contain NUL");
        let mut client_ptr: *mut CosmosClient = ptr::null_mut();
        let mut error = CosmosError::success();

        let result = cosmos_client_create_with_key(
            endpoint.as_ptr(),
            key.as_ptr(),
            &mut client_ptr,
            &mut error,
        );

        assert_eq!(result, CosmosErrorCode::Success);
        assert!(!client_ptr.is_null());
        assert_eq!(error.code, CosmosErrorCode::Success);

        cosmos_client_free(client_ptr);
    }

    #[test]
    fn test_cosmos_client_create_null_params() {
        let mut client_ptr: *mut CosmosClient = ptr::null_mut();
        let mut error = CosmosError::success();

        let result =
            cosmos_client_create_with_key(ptr::null(), ptr::null(), &mut client_ptr, &mut error);

        assert_eq!(result, CosmosErrorCode::InvalidArgument);
        assert!(client_ptr.is_null());
    }

    #[test]
    fn test_cosmos_client_database_client() {
        let endpoint = CString::new("https://test.documents.azure.com")
            .expect("test string should not contain NUL");
        let key = CString::new("test-key").expect("test string should not contain NUL");
        let db_id = CString::new("test-db").expect("test string should not contain NUL");

        let mut client_ptr: *mut CosmosClient = ptr::null_mut();
        let mut db_ptr: *mut DatabaseClient = ptr::null_mut();
        let mut error = CosmosError::success();

        cosmos_client_create_with_key(
            endpoint.as_ptr(),
            key.as_ptr(),
            &raw mut client_ptr,
            &mut error,
        );
        assert!(!client_ptr.is_null());

        let result =
            cosmos_client_database_client(client_ptr, db_id.as_ptr(), &mut db_ptr, &mut error);

        assert_eq!(result, CosmosErrorCode::Success);
        assert!(!db_ptr.is_null());

        cosmos_database_free(db_ptr);
        cosmos_client_free(client_ptr);
    }

    #[test]
    fn test_cosmos_client_query_databases_null_params() {
        let mut json_ptr: *mut c_char = ptr::null_mut();
        let mut error = CosmosError::success();

        let result =
            cosmos_client_query_databases(ptr::null(), ptr::null(), &mut json_ptr, &mut error);

        assert_eq!(result, CosmosErrorCode::InvalidArgument);
        assert!(json_ptr.is_null());
    }

    #[test]
    fn test_cosmos_client_create_database_null_params() {
        let mut db_ptr: *mut DatabaseClient = ptr::null_mut();
        let mut error = CosmosError::success();

        // Test null client
        let result =
            cosmos_client_create_database(ptr::null(), ptr::null(), &mut db_ptr, &mut error);

        assert_eq!(result, CosmosErrorCode::InvalidArgument);
        assert!(db_ptr.is_null());

        // Reset for next test
        db_ptr = ptr::null_mut();
        error = CosmosError::success();

        // Test null database_id
        let result =
            cosmos_client_create_database(ptr::null(), ptr::null(), &mut db_ptr, &mut error);

        assert_eq!(result, CosmosErrorCode::InvalidArgument);
        assert!(db_ptr.is_null());
    }
}
