// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{driver::DriverHandle, runtime::RuntimeContext};
use azure_data_cosmos_driver::{
    error::Result,
    in_memory_emulator::{
        ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
    },
    models::{
        AccountReference, CosmosOperation, ItemReference, PartitionKey, PartitionKeyDefinition,
    },
    options::{DriverOptions, OperationOptions},
};
use std::sync::Arc;
use url::Url;

mod transport;
pub(super) use transport::Gate;

pub(super) fn scripted(current_thread: bool) -> (*mut RuntimeContext, *mut DriverHandle) {
    scripted_gated(current_thread, None)
}

pub(super) fn scripted_gated(
    current_thread: bool,
    gate: Option<Arc<Gate>>,
) -> (*mut RuntimeContext, *mut DriverHandle) {
    let tokio = if current_thread {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
    } else {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap()
    };
    let runtime = tokio
        .block_on(
            azure_data_cosmos_driver::CosmosDriverRuntimeBuilder::new()
                .with_mock_http_client_factory(Arc::new(transport::Factory(gate)))
                .build(),
        )
        .unwrap();
    let driver = tokio
        .block_on(
            runtime.create_driver(
                DriverOptions::builder(AccountReference::with_master_key(
                    Url::parse("https://eastus.emulator.local").unwrap(),
                    "ZW11bGF0b3Ita2V5",
                ))
                .build(),
            ),
        )
        .unwrap();
    (
        Arc::into_raw(Arc::new(RuntimeContext {
            tokio: std::mem::ManuallyDrop::new(tokio),
            driver: runtime,
        })) as *mut RuntimeContext,
        DriverHandle::from_arc_into_raw(Arc::new(DriverHandle { inner: driver })),
    )
}

pub(super) fn create(current_thread: bool) -> Result<(*mut RuntimeContext, *mut DriverHandle)> {
    let endpoint = Url::parse("https://eastus.emulator.local").unwrap();
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new("East US", endpoint.clone())])?;
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    emulator.store().create_database("testdb");
    emulator.store().create_container_with_config(
        "testdb",
        "testcoll",
        PartitionKeyDefinition::new(vec!["/pk".into()]),
        ContainerConfig::new().with_partition_count(2).build()?,
    );
    let tokio = if current_thread {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
    } else {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap()
    };
    let (runtime, driver) = tokio.block_on(async {
        let runtime = emulator.runtime_builder().build().await?;
        let account = AccountReference::with_master_key(endpoint, "ZW11bGF0b3Ita2V5");
        let driver = runtime
            .create_driver(DriverOptions::builder(account).build())
            .await?;
        let container = driver
            .resolve_container("testdb", "testcoll", OperationOptions::default())
            .await?;
        for rank in [5, 1, 4, 2, 3, 0] {
            let id = format!("d{rank}");
            let pk = format!("pk-{rank}");
            let body = serde_json::json!({"id":id,"pk":pk,"rank":rank,"group":rank % 3});
            driver
                .execute_singleton_operation(
                    CosmosOperation::create_item(ItemReference::from_name(
                        &container,
                        PartitionKey::from(pk),
                        id,
                    ))
                    .with_body(serde_json::to_vec(&body).unwrap()),
                    OperationOptions::default(),
                )
                .await?;
        }
        Ok::<_, azure_data_cosmos_driver::error::CosmosError>((runtime, driver))
    })?;
    Ok((
        Arc::into_raw(Arc::new(RuntimeContext {
            tokio: std::mem::ManuallyDrop::new(tokio),
            driver: runtime,
        })) as *mut RuntimeContext,
        DriverHandle::from_arc_into_raw(Arc::new(DriverHandle { inner: driver })),
    ))
}

/// Test-only populated two-range emulator; absent from production builds.
// cbindgen:ignore
#[cfg(feature = "test-abi")]
#[no_mangle]
pub extern "C" fn cosmos_test_cursor_fixture(
    scripted_transport: u32,
    out_runtime: *mut *mut RuntimeContext,
    out_driver: *mut *mut DriverHandle,
) -> crate::error::CosmosStatusCode {
    use crate::error::{CosmosErrorCode, CosmosStatusCode, COSMOS_STATUS_SUCCESS};
    if out_runtime.is_null() || out_driver.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    crate::safety::ffi_guard(
        CosmosErrorCode::CosmosErrorCodeInternalError.as_status_code(),
        || {
            match if scripted_transport == 0 {
                create(false)
            } else {
                Ok(scripted(false))
            } {
                Ok((runtime, driver)) => {
                    // SAFETY: caller supplies two writable output slots.
                    unsafe {
                        out_runtime.write(runtime);
                        out_driver.write(driver);
                    }
                    COSMOS_STATUS_SUCCESS
                }
                Err(error) => CosmosStatusCode::from_driver_error(&error),
            }
        },
    )
}
