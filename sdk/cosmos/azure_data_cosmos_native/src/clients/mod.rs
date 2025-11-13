#![allow(
    clippy::missing_safety_doc,
    reason = "We're operating on raw pointers received from FFI."
)]

pub mod container_client;
pub mod cosmos_client;
pub mod database_client;

pub use container_client::*;
pub use cosmos_client::*;
pub use database_client::*;

// Below are opaque handle types for FFI.
// These types are used as the type names for pointers passed across the FFI boundary, but they are zero-sized in Rust.
// The actual data is stored in the corresponding Azure SDK for Rust types and we provide functions to transmute pointers from these handle types to the actual types.
// This pattern allows cbindgen to see the types for generating headers, while keeping the actual implementation details hidden from the C side.
//
// You might be tempted to use a macro to generate them, but cbindgen does not handle macros well, so we define them manually.

pub struct CosmosClientHandle;

impl CosmosClientHandle {
    pub unsafe fn unwrap_ptr<'a>(
        ptr: *const CosmosClientHandle,
    ) -> &'a azure_data_cosmos::CosmosClient {
        (ptr as *const azure_data_cosmos::CosmosClient)
            .as_ref()
            .unwrap()
    }

    pub unsafe fn wrap_ptr(value: Box<azure_data_cosmos::CosmosClient>) -> *mut CosmosClientHandle {
        Box::into_raw(value) as *mut CosmosClientHandle
    }

    pub unsafe fn free_ptr(ptr: *mut CosmosClientHandle) {
        if !ptr.is_null() {
            drop(Box::from_raw(ptr as *mut azure_data_cosmos::CosmosClient));
        }
    }
}

pub struct DatabaseClientHandle;

impl DatabaseClientHandle {
    pub unsafe fn unwrap_ptr<'a>(
        ptr: *const DatabaseClientHandle,
    ) -> &'a azure_data_cosmos::clients::DatabaseClient {
        (ptr as *const azure_data_cosmos::clients::DatabaseClient)
            .as_ref()
            .unwrap()
    }

    pub unsafe fn wrap_ptr(
        value: Box<azure_data_cosmos::clients::DatabaseClient>,
    ) -> *mut DatabaseClientHandle {
        Box::into_raw(value) as *mut DatabaseClientHandle
    }

    pub unsafe fn free_ptr(ptr: *mut DatabaseClientHandle) {
        if !ptr.is_null() {
            drop(Box::from_raw(
                ptr as *mut azure_data_cosmos::clients::DatabaseClient,
            ));
        }
    }
}

pub struct ContainerClientHandle;

impl ContainerClientHandle {
    pub unsafe fn unwrap_ptr<'a>(
        ptr: *const ContainerClientHandle,
    ) -> &'a azure_data_cosmos::clients::ContainerClient {
        (ptr as *const azure_data_cosmos::clients::ContainerClient)
            .as_ref()
            .unwrap()
    }

    pub unsafe fn wrap_ptr(
        value: Box<azure_data_cosmos::clients::ContainerClient>,
    ) -> *mut ContainerClientHandle {
        Box::into_raw(value) as *mut ContainerClientHandle
    }

    pub unsafe fn free_ptr(ptr: *mut ContainerClientHandle) {
        if !ptr.is_null() {
            drop(Box::from_raw(
                ptr as *mut azure_data_cosmos::clients::ContainerClient,
            ));
        }
    }
}
