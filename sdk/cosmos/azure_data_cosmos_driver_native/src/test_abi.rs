// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Test-only helpers used by the linked-C ABI suite.

use std::{
    ptr,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use azure_core::http::headers::Headers;
use azure_data_cosmos_driver::{
    driver::CosmosDriverRuntime,
    options::OperationOptions,
    test::{
        ConnectionPoolOptions, HttpClientConfig, HttpClientFactory, HttpRequest, HttpResponse,
        TransportClient, TransportError,
    },
};

use crate::{
    container_ref::ContainerRefHandle,
    driver::DriverHandle,
    driver_options::DriverOptionsHandle,
    error::{CosmosErrorCode, CosmosStatusCode},
    runtime::RuntimeContext,
};

#[derive(Debug, Default)]
struct AbiTransport {
    paths: Mutex<Vec<String>>,
}

#[async_trait]
impl TransportClient for AbiTransport {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        let path = request.url.path();
        self.paths.lock().unwrap().push(path.to_owned());
        let body = match path {
            "/" => serde_json::json!({
                "_self": "",
                "id": "native-fault-injection",
                "_rid": "native-fault-injection",
                "writableLocations": [{
                    "name": "West US 2",
                    "databaseAccountEndpoint": "https://native-fault.invalid/"
                }],
                "readableLocations": [{
                    "name": "West US 2",
                    "databaseAccountEndpoint": "https://native-fault.invalid/"
                }],
                "enableMultipleWriteLocations": false,
                "userConsistencyPolicy": {"defaultConsistencyLevel": "Session"}
            }),
            "/dbs/db/colls/items" => serde_json::json!({
                "id": "items",
                "_rid": "AQIDBAUGBwg=",
                "partitionKey": {"paths": ["/pk"], "kind": "Hash", "version": 2}
            }),
            _ => serde_json::json!({"id": "item", "pk": "tenant"}),
        };
        Ok(HttpResponse {
            status: 200,
            headers: Headers::new(),
            body: serde_json::to_vec(&body).unwrap(),
        })
    }
}

#[derive(Debug)]
struct AbiFactory(Arc<AbiTransport>);

impl HttpClientFactory for AbiFactory {
    fn build(
        &self,
        _: &ConnectionPoolOptions,
        _: HttpClientConfig,
    ) -> azure_data_cosmos_driver::error::Result<Arc<dyn TransportClient>> {
        Ok(self.0.clone())
    }
}

/// Creates a fully initialized mock-backed runtime, driver, and container.
///
/// This symbol is compiled only by the `test-abi` feature and excluded from
/// the public header. The linked-C suite forward-declares it locally.
#[no_mangle]
pub extern "C" fn __test_only_create_fault_injection_fixture(
    options: *const DriverOptionsHandle,
    out_runtime: *mut *mut RuntimeContext,
    out_driver: *mut *mut DriverHandle,
    out_container: *mut *mut ContainerRefHandle,
) -> CosmosStatusCode {
    if out_runtime.is_null() || out_driver.is_null() || out_container.is_null() || options.is_null()
    {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    // SAFETY: output slots were checked above and are initialized on success only.
    unsafe {
        *out_runtime = ptr::null_mut();
        *out_driver = ptr::null_mut();
        *out_container = ptr::null_mut();
    }
    let Some(options) = DriverOptionsHandle::inner_arc(options) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    };
    let tokio = match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
    {
        Ok(tokio) => tokio,
        Err(_) => return CosmosErrorCode::CosmosErrorCodeInternalError.as_status_code(),
    };
    let transport = Arc::new(AbiTransport::default());
    let runtime = match tokio.block_on(
        CosmosDriverRuntime::builder()
            .with_mock_http_client_factory(Arc::new(AbiFactory(transport)))
            .build(),
    ) {
        Ok(runtime) => runtime,
        Err(error) => return CosmosStatusCode::from_driver_error(&error),
    };
    let driver = match tokio.block_on(runtime.create_driver(options.inner.clone())) {
        Ok(driver) => driver,
        Err(error) => return CosmosStatusCode::from_driver_error(&error),
    };
    let container = match tokio.block_on(driver.resolve_container_by_name(
        "db",
        "items",
        OperationOptions::default(),
    )) {
        Ok(container) => container,
        Err(error) => return CosmosStatusCode::from_driver_error(&error),
    };

    let runtime = Arc::into_raw(Arc::new(RuntimeContext {
        tokio,
        driver: runtime,
    })) as *mut RuntimeContext;
    let driver = DriverHandle::from_arc_into_raw(Arc::new(DriverHandle { inner: driver }));
    let container = ContainerRefHandle::into_raw(container);
    // SAFETY: all output slots are writable by contract and now receive ownership.
    unsafe {
        *out_runtime = runtime;
        *out_driver = driver;
        *out_container = container;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
}
