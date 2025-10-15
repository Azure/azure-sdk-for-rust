// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    AccountProperties, DocumentServiceRequest, OperationType,
    ReadOnlyLocationMap, ReadOnlyUrlCollection, ResourceType,
};
use azure_core::Result as AzureResult;
use url::Url;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use async_trait::async_trait;
use crate::CosmosClientOptions;
use crate::routing::location_cache::{DatabaseAccountLocationsInfo, LocationCache, RequestOperation};

#[derive(Clone)]
pub struct Uri(pub String);

#[derive(Debug, Default)]
pub struct GlobalEndpointManager {
    client_options: CosmosClientOptions,
    preferred_locations: Vec<String>,
    location_cache: Mutex<LocationCache>,
    min_time_between_account_refresh: Duration,
    background_refresh_location_time_interval: Duration,
    is_account_refresh_in_progress: AtomicBool,
    is_background_account_refresh_active: AtomicBool,
}

#[allow(non_snake_case)]
impl GlobalEndpointManager {
    pub fn new(client_options: CosmosClientOptions) -> Self {
        Self {
            client_options,
            preferred_locations: Vec::new(),
            location_cache: Mutex::new(LocationCache::new("empty".to_string(), Vec::new())),
            min_time_between_account_refresh: Duration::from_secs(5 * 60),
            background_refresh_location_time_interval: Duration::from_secs(5 * 60),
            is_account_refresh_in_progress: AtomicBool::new(false),
            is_background_account_refresh_active: AtomicBool::new(false),
        }
    }

    fn ReadEndpoints(&self) -> Vec<String> { self.location_cache.lock().unwrap().read_endpoints() }

    fn AccountReadEndpoints(&self) -> Vec<String> { self.location_cache.lock().unwrap().read_endpoints() }

    fn WriteEndpoints(&self) -> Vec<String> { self.location_cache.lock().unwrap().write_endpoints() }

    fn ThinClientReadEndpoints(&self) -> Vec<String> { unimplemented!("ThinClientReadEndpoints not implemented yet") }

    fn ThinClientWriteEndpoints(&self) -> Vec<String> { unimplemented!("ThinClientWriteEndpoints not implemented yet") }

    fn PreferredLocationCount(&self) -> i32 { unimplemented!("PreferredLocationCount not implemented yet") }

    fn ResolveServiceEndpoint(&self, request: &DocumentServiceRequest) -> String { self.location_cache.lock().unwrap().resolve_service_endpoint(0, RequestOperation::Read) }

    fn GetLocation(&self, _endpoint: &Url) -> Option<String> { unimplemented!("GetLocation not implemented yet") }

    fn MarkEndpointUnavailableForRead(&self, endpoint: &str) { self.location_cache.lock().unwrap().mark_endpoint_unavailable(endpoint, RequestOperation::Read) }

    fn MarkEndpointUnavailableForWrite(&self, endpoint: &str) { self.location_cache.lock().unwrap().mark_endpoint_unavailable(endpoint, RequestOperation::Write) }

    fn CanUseMultipleWriteLocations(&self, _request: &DocumentServiceRequest) -> bool { self.location_cache.lock().unwrap().can_use_multiple_write_locations() }

    fn InitializeAccountPropertiesAndStartBackgroundRefresh(&self, _account: AccountProperties) { unimplemented!("InitializeAccountPropertiesAndStartBackgroundRefresh not implemented yet") }

    fn RefreshLocationAsync<'a>(&'a self, _forceRefresh: bool) -> core::pin::Pin<Box<dyn core::future::Future<Output = AzureResult<()>> + Send + 'a>> {
        Box::pin(async move { unimplemented!("RefreshLocationAsync not implemented yet") })
    }

    fn GetAvailableWriteEndpointsByLocation(&self) -> ReadOnlyLocationMap { unimplemented!("GetAvailableWriteEndpointsByLocation not implemented yet") }

    fn GetAvailableReadEndpointsByLocation(&self) -> ReadOnlyLocationMap { unimplemented!("GetAvailableReadEndpointsByLocation not implemented yet") }

    fn CanSupportMultipleWriteLocations(&self, _resource_type: ResourceType, _operation_type: OperationType) -> bool { unimplemented!("CanSupportMultipleWriteLocations not implemented yet") }
}
