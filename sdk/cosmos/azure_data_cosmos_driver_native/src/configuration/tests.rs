// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    cosmos_driver_options_build_v3, cosmos_driver_options_config_v3_init, cosmos_runtime_build_v2,
    cosmos_runtime_options_v2_init, record, runtime_builder, CosmosDriverOptionsConfigV3,
    CosmosDuration, CosmosOptionalU64, CosmosRuntimeOptionsV2, INVALID,
};
use crate::{
    account_ref::{cosmos_account_ref_free, cosmos_account_ref_with_master_key},
    driver_options::{cosmos_driver_options_free, DriverOptionsHandle},
    error::COSMOS_STATUS_SUCCESS,
    fault_injection::{
        cosmos_fault_injection_condition_default, cosmos_fault_injection_result_default,
        cosmos_fault_injection_rule_default,
    },
    op_request::cosmos_operation_options_default,
    runtime::{cosmos_runtime_free, RuntimeContext},
    string::view,
};
use async_trait::async_trait;
use azure_core::http::headers::Headers;
use azure_data_cosmos_driver::{
    models::{CosmosOperation, ItemReference, PartitionKey},
    options::{
        AvailabilityStrategy, DiagnosticsVerbosity, OperationOptions, PartitionTopologyCacheMode,
    },
    test::{
        ConnectionPoolOptions, HttpClientConfig, HttpClientFactory, HttpRequest, HttpResponse,
        TransportClient, TransportError,
    },
};
use std::{
    mem::MaybeUninit,
    ptr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

fn initialized<T>(init: extern "C" fn(*mut T)) -> T {
    let mut value = MaybeUninit::uninit();
    init(value.as_mut_ptr());
    // SAFETY: typed initializer wrote the complete record.
    unsafe { value.assume_init() }
}

fn duration(seconds: u64, nanoseconds: u32) -> CosmosDuration {
    CosmosDuration {
        is_set: 1,
        seconds,
        nanoseconds,
    }
}

fn integer(value: u64) -> CosmosOptionalU64 {
    CosmosOptionalU64 { is_set: 1, value }
}

#[test]
fn runtime_record_validates_version_size_and_extensions() {
    #[repr(C)]
    struct Extended {
        options: CosmosRuntimeOptionsV2,
        extension: u64,
    }

    let mut extended = Extended {
        options: initialized(cosmos_runtime_options_v2_init),
        extension: 0,
    };
    extended.options.struct_size_bytes = std::mem::size_of::<Extended>() as u32;
    // SAFETY: all pointers reference live, aligned, initialized records.
    unsafe {
        assert!(record(&extended.options).is_ok());
        extended.extension = 1;
        assert!(record(&extended.options).is_err());
        extended.extension = 0;
        extended.options.abi_version = 2;
        assert!(record(&extended.options).is_err());
        extended.options.abi_version = 1;
        extended.options.struct_size_bytes =
            std::mem::size_of::<CosmosRuntimeOptionsV2>() as u32 - 1;
        assert!(record(&extended.options).is_err());
        assert!(record::<CosmosRuntimeOptionsV2>(ptr::null()).is_err());
    }
}

#[test]
fn exact_durations_reject_invalid_payload_and_clock_overflow() {
    for (input, expected) in [
        (CosmosDuration::default(), Ok(None)),
        (duration(0, 0), Ok(Some(Duration::ZERO))),
        (duration(1, 1), Ok(Some(Duration::new(1, 1)))),
        (duration(0, 1_000_000_000), Err(INVALID.as_status_code())),
        (duration(u64::MAX, 0), Err(INVALID.as_status_code())),
        (
            CosmosDuration {
                is_set: 0,
                seconds: 1,
                nanoseconds: 0,
            },
            Err(INVALID.as_status_code()),
        ),
        (
            CosmosDuration {
                is_set: 2,
                seconds: 0,
                nanoseconds: 0,
            },
            Err(INVALID.as_status_code()),
        ),
    ] {
        assert_eq!(input.decode(), expected);
    }
}

#[test]
fn runtime_construction_reaches_pool_diagnostics_and_exact_defaults() {
    let mut operation = cosmos_operation_options_default();
    operation.max_failover_retry_count = 7;
    let mut options: CosmosRuntimeOptionsV2 = initialized(cosmos_runtime_options_v2_init);
    options.workload_id = 7;
    options.correlation_id = view(b"native-flat");
    options.user_agent_suffix = view(b"native-flat");
    options.wrapping_sdk_identifier = view(b"native-flat-wrapper");
    options.cpu_refresh_interval_ms = 5_000;
    options.operation_options = &operation;
    options.max_connect_timeout = duration(4, 17);
    options.max_http2_streams_per_client = integer(17);
    options.proxy_allowed = 1;
    options.gateway_v2_disabled = 2;
    options.local_address = view(b"127.0.0.2");
    options.diagnostics_verbosity = 2;
    options.max_request_diagnostics = integer(16);
    options.max_summary_size_bytes = integer(4096);
    options.endpoint_unavailability_ttl = duration(0, 17);
    options.max_throttle_retry_wait_time = duration(0, 23);
    options.hedge_threshold = duration(0, 31);
    let mut runtime = ptr::null_mut();
    assert_eq!(
        cosmos_runtime_build_v2(&options, &mut runtime, ptr::null_mut()),
        COSMOS_STATUS_SUCCESS
    );
    let runtime_owned = RuntimeContext::inner_arc(runtime).unwrap();
    cosmos_runtime_free(runtime);
    let runtime = &runtime_owned.driver;
    assert!(runtime.workload_id().is_some());
    assert!(runtime.correlation_id().is_some());
    assert!(runtime
        .user_agent()
        .as_str()
        .contains("native-flat-wrapper"));
    assert!(runtime.user_agent().as_str().ends_with(" native-flat"));
    assert_eq!(
        runtime.connection_pool().max_connect_timeout(),
        Duration::new(4, 17)
    );
    assert_eq!(runtime.connection_pool().max_http2_streams_per_client(), 17);
    assert!(!runtime.connection_pool().proxy_allowed());
    assert_eq!(
        runtime
            .connection_pool()
            .local_address()
            .unwrap()
            .to_string(),
        "127.0.0.2"
    );
    assert_eq!(
        runtime.diagnostics_options().default_verbosity(),
        DiagnosticsVerbosity::Detailed
    );
    assert_eq!(runtime.diagnostics_options().max_request_diagnostics(), 16);
    let defaults = runtime.default_operation_options();
    assert_eq!(defaults.max_failover_retry_count, Some(7));
    assert_eq!(
        defaults.endpoint_unavailability_ttl,
        Some(Duration::from_nanos(17))
    );
    assert_eq!(
        defaults
            .throttling_retry_options
            .as_ref()
            .unwrap()
            .max_retry_wait_time,
        Some(Duration::from_nanos(23))
    );
    assert!(
        matches!(defaults.availability_strategy, Some(AvailabilityStrategy::Hedging(strategy)) if strategy.threshold().get() == Duration::from_nanos(31))
    );
}

#[test]
fn runtime_rejects_invalid_enums_and_native_bounds() {
    let mut options: CosmosRuntimeOptionsV2 = initialized(cosmos_runtime_options_v2_init);
    for field in [0, 1, 2, 3] {
        match field {
            0 => options.proxy_allowed = 3,
            1 => {
                options.proxy_allowed = 0;
                options.tls_backend = 99;
            }
            2 => {
                options.tls_backend = 0;
                options.max_http2_streams_per_client = integer(u64::MAX);
            }
            _ => {
                options.max_http2_streams_per_client = CosmosOptionalU64::default();
                options.max_connect_timeout = duration(0, 1);
            }
        }
        let mut runtime = ptr::null_mut();
        assert_ne!(
            cosmos_runtime_build_v2(&options, &mut runtime, ptr::null_mut()),
            COSMOS_STATUS_SUCCESS
        );
        assert!(runtime.is_null());
    }
}

#[test]
fn client_configuration_copies_backups_preserves_identity_and_uses_native_topology() {
    let mut account = ptr::null_mut();
    assert_eq!(
        cosmos_account_ref_with_master_key(
            view(b"https://eastus.emulator.local"),
            view(b"ZW11bGF0b3Ita2V5"),
            &mut account,
            ptr::null_mut()
        ),
        COSMOS_STATUS_SUCCESS
    );
    let mut config: CosmosDriverOptionsConfigV3 = initialized(cosmos_driver_options_config_v3_init);
    config.topology_cache_mode = 2;
    config.read_failure_threshold = integer(2);
    config.circuit_breaker_enabled = 1;
    let mut overrides = Vec::new();
    for suffix in [Some("client-a"), Some("client-b"), None] {
        let endpoint_text = [
            "https://backup-one.invalid".to_string(),
            "https://backup-two.invalid".to_string(),
        ];
        let endpoints = endpoint_text.each_ref().map(|text| view(text.as_bytes()));
        config.backup_endpoints = endpoints.as_ptr();
        config.backup_endpoints_len = endpoints.len();
        config.user_agent_suffix =
            suffix.map_or(Default::default(), |suffix| view(suffix.as_bytes()));
        let mut options = ptr::null_mut();
        assert_eq!(
            cosmos_driver_options_build_v3(account, &config, &mut options),
            COSMOS_STATUS_SUCCESS
        );
        overrides.push(DriverOptionsHandle::inner_arc(options).unwrap());
        cosmos_driver_options_free(options);
    }
    cosmos_account_ref_free(account);
    assert_client_identity(overrides);
}

#[test]
fn client_exact_overrides_preserve_legacy_defaults_and_allow_explicit_backup_clearing() {
    let account = Box::into_raw(Box::new(crate::account_ref::AccountRefHandle {
        inner: azure_data_cosmos_driver::models::AccountReference::with_account_key(
            "https://primary.invalid".parse().unwrap(),
            "ZW11bGF0b3Ita2V5",
        )
        .with_backup_endpoints(vec!["https://backup.invalid".parse().unwrap()]),
    }));
    let mut operation = cosmos_operation_options_default();
    operation.max_failover_retry_count = 7;
    operation.max_throttle_retry_count = 19;
    let mut input: CosmosDriverOptionsConfigV3 = initialized(cosmos_driver_options_config_v3_init);
    input.operation_options = &operation;
    input.hedge_threshold = duration(0, 1);
    let empty = [view(b"unused")];
    for clear in [false, true] {
        input.backup_endpoints = if clear { empty.as_ptr() } else { ptr::null() };
        let mut handle = ptr::null_mut();
        assert_eq!(
            cosmos_driver_options_build_v3(account, &input, &mut handle),
            COSMOS_STATUS_SUCCESS
        );
        let copied = DriverOptionsHandle::inner_arc(handle).unwrap();
        cosmos_driver_options_free(handle);
        assert_eq!(
            copied.inner.account().backup_endpoints().len(),
            usize::from(!clear)
        );
        assert_eq!(
            copied.inner.operation_options().max_failover_retry_count,
            Some(7)
        );
        assert_eq!(
            copied
                .inner
                .operation_options()
                .throttling_retry_options
                .as_ref()
                .unwrap()
                .max_retry_count,
            Some(19)
        );
        assert!(
            matches!(copied.inner.operation_options().availability_strategy,
            Some(AvailabilityStrategy::Hedging(strategy)) if strategy.threshold().get() == Duration::from_nanos(1))
        );
    }
    cosmos_account_ref_free(account);
}

#[test]
fn flat_client_copies_regions_rules_and_operation_defaults() {
    let account = Box::into_raw(Box::new(crate::account_ref::AccountRefHandle {
        inner: azure_data_cosmos_driver::models::AccountReference::with_account_key(
            "https://primary.invalid".parse().unwrap(),
            "ZW11bGF0b3Ita2V5",
        ),
    }));
    let copied = {
        let region = String::from("East US");
        let regions = [view(region.as_bytes())];
        let id = String::from("flat-client-rule");
        let condition = cosmos_fault_injection_condition_default();
        let mut result = cosmos_fault_injection_result_default();
        result.custom_status_code = 429;
        let mut rule = cosmos_fault_injection_rule_default();
        rule.id = view(id.as_bytes());
        rule.condition = &condition;
        rule.result = &result;
        let mut operation = cosmos_operation_options_default();
        operation.max_failover_retry_count = 7;
        let mut input: CosmosDriverOptionsConfigV3 =
            initialized(cosmos_driver_options_config_v3_init);
        input.preferred_regions = regions.as_ptr();
        input.preferred_regions_len = regions.len();
        input.operation_options = &operation;
        input.fault_injection_rules = &rule;
        input.fault_injection_rules_len = 1;
        input.resource_name_validation_enabled = 1;
        let mut handle = ptr::null_mut();
        assert_eq!(
            cosmos_driver_options_build_v3(account, &input, &mut handle),
            COSMOS_STATUS_SUCCESS
        );
        let copied = DriverOptionsHandle::inner_arc(handle).unwrap();
        cosmos_driver_options_free(handle);
        copied
    };
    cosmos_account_ref_free(account);
    assert_eq!(copied.inner.preferred_regions()[0].as_str(), "eastus");
    assert_eq!(
        copied.inner.fault_injection_rules().unwrap()[0].id(),
        "flat-client-rule"
    );
    assert_eq!(
        copied.inner.operation_options().max_failover_retry_count,
        Some(7)
    );
    assert!(!copied.inner.resource_name_validation_enabled());
}

#[test]
fn unset_client_topology_preserves_native_environment_fallback() {
    const CHILD: &str = "COSMOS_NATIVE_TEST_INVALID_TOPOLOGY_ENV";
    if std::env::var_os(CHILD).is_none() {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "configuration::tests::unset_client_topology_preserves_native_environment_fallback",
            ])
            .env(CHILD, "1")
            .env("AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD", "0")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        return;
    }
    let account = Box::into_raw(Box::new(crate::account_ref::AccountRefHandle {
        inner: azure_data_cosmos_driver::models::AccountReference::with_account_key(
            "https://primary.invalid".parse().unwrap(),
            "ZW11bGF0b3Ita2V5",
        ),
    }));
    let mut input: CosmosDriverOptionsConfigV3 = initialized(cosmos_driver_options_config_v3_init);
    let mut handle = ptr::null_mut();
    assert_eq!(
        cosmos_driver_options_build_v3(account, &input, &mut handle),
        COSMOS_STATUS_SUCCESS
    );
    cosmos_driver_options_free(handle);
    input.topology_cache_mode = 2;
    assert_ne!(
        cosmos_driver_options_build_v3(account, &input, &mut handle),
        COSMOS_STATUS_SUCCESS
    );
    assert!(handle.is_null());
    cosmos_account_ref_free(account);
}

#[derive(Debug, Default)]
struct ConsumerTransport {
    bootstrap_hosts: Mutex<Vec<String>>,
    pool_builds: AtomicUsize,
    routing_reads: AtomicUsize,
    item_reads: AtomicUsize,
}

#[derive(Debug)]
struct ConsumerFactory(Arc<ConsumerTransport>);

impl HttpClientFactory for ConsumerFactory {
    fn build(
        &self,
        pool: &ConnectionPoolOptions,
        _: HttpClientConfig,
    ) -> azure_data_cosmos_driver::error::Result<Arc<dyn TransportClient>> {
        assert_eq!(pool.max_connect_timeout(), Duration::new(4, 17));
        assert!(!pool.is_http2_allowed());
        self.0.pool_builds.fetch_add(1, Ordering::SeqCst);
        Ok(self.0.clone())
    }
}

#[async_trait]
impl TransportClient for ConsumerTransport {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        let mut status = 200;
        let mut headers = Headers::new();
        let body = match request.url.path() {
            "/" => {
                let host = request.url.host_str().unwrap();
                self.bootstrap_hosts.lock().unwrap().push(host.to_owned());
                if host != "backup-two.invalid" {
                    status = 400;
                    serde_json::json!({"code":"BadRequest","message":"bootstrap unavailable"})
                } else {
                    serde_json::json!({
                        "id":"consumer-fixture","_rid":"fixture","_self":"",
                        "enableMultipleWriteLocations":false,
                        "writableLocations":[{"name":"East US","databaseAccountEndpoint":"https://backup-two.invalid/"}],
                        "readableLocations":[{"name":"East US","databaseAccountEndpoint":"https://backup-two.invalid/"}],
                        "userConsistencyPolicy":{"defaultConsistencyLevel":"Session"}
                    })
                }
            }
            "/dbs/testdb/colls/testcoll" => serde_json::json!({
                "id":"testcoll","_rid":"AQIDBAUGBwg=",
                "partitionKey":{"paths":["/pk"],"kind":"Hash","version":2}
            }),
            path if path.ends_with("/pkranges") => {
                self.routing_reads.fetch_add(1, Ordering::SeqCst);
                headers.insert("etag", "\"ranges\"");
                serde_json::json!({"PartitionKeyRanges":[
                        {"id":"0","minInclusive":"","maxExclusive":"FF"}
                    ],"_count":1})
            }
            path if path.ends_with("/docs/doc") => {
                let attempt = self.item_reads.fetch_add(1, Ordering::SeqCst);
                if attempt < 18 {
                    status = 429;
                    headers.insert("x-ms-retry-after-ms", "1");
                    serde_json::json!({"code":"TooManyRequests","message":"retry fixture"})
                } else {
                    serde_json::json!({"id":"doc","pk":"key"})
                }
            }
            path => panic!("unexpected consumer request {path}"),
        };
        Ok(HttpResponse {
            status,
            headers,
            body: serde_json::to_vec(&body).unwrap(),
        })
    }
}

#[tokio::test]
async fn configured_consumers_bootstrap_in_order_load_lazy_and_retain_diagnostics() {
    let mut legacy_operation = cosmos_operation_options_default();
    legacy_operation.binary_encoding_enabled = 1;
    legacy_operation.max_throttle_retry_count = 30;
    let mut runtime_options: CosmosRuntimeOptionsV2 = initialized(cosmos_runtime_options_v2_init);
    runtime_options.is_http2_allowed = 1;
    runtime_options.max_connect_timeout = duration(4, 17);
    runtime_options.diagnostics_verbosity = 2;
    runtime_options.max_request_diagnostics = integer(16);
    runtime_options.operation_options = &legacy_operation;
    runtime_options.max_throttle_retry_wait_time = duration(1, 17);
    runtime_options.endpoint_unavailability_ttl = duration(0, 23);
    let transport = Arc::new(ConsumerTransport::default());
    // SAFETY: all borrowed payloads remain readable throughout decoding.
    let runtime = unsafe { runtime_builder(&runtime_options) }
        .unwrap()
        .with_mock_http_client_factory(Arc::new(ConsumerFactory(transport.clone())))
        .build()
        .await
        .unwrap();
    let mut account = ptr::null_mut();
    assert_eq!(
        cosmos_account_ref_with_master_key(
            view(b"https://primary.invalid"),
            view(b"ZW11bGF0b3Ita2V5"),
            &mut account,
            ptr::null_mut()
        ),
        COSMOS_STATUS_SUCCESS
    );
    let endpoints = [
        view(b"https://backup-one.invalid"),
        view(b"https://backup-two.invalid"),
    ];
    let mut input: CosmosDriverOptionsConfigV3 = initialized(cosmos_driver_options_config_v3_init);
    input.backup_endpoints = endpoints.as_ptr();
    input.backup_endpoints_len = endpoints.len();
    input.topology_cache_mode = 2;
    let mut client = ptr::null_mut();
    assert_eq!(
        cosmos_driver_options_build_v3(account, &input, &mut client),
        COSMOS_STATUS_SUCCESS
    );
    let options = DriverOptionsHandle::inner_arc(client).unwrap();
    cosmos_driver_options_free(client);
    cosmos_account_ref_free(account);
    let driver = runtime
        .create_driver(options.inner.clone())
        .await
        .unwrap_or_else(|error| {
            panic!(
                "{error}; bootstrap calls: {:?}",
                transport.bootstrap_hosts.lock().unwrap()
            )
        });
    let hosts = transport.bootstrap_hosts.lock().unwrap().clone();
    assert_eq!(
        &hosts[..3],
        [
            "primary.invalid",
            "backup-one.invalid",
            "backup-two.invalid"
        ]
    );
    assert!(transport.pool_builds.load(Ordering::SeqCst) > 0);
    let defaults = OperationOptions::default();
    let effective = driver.operation_options_view(&defaults);
    assert_eq!(
        effective.endpoint_unavailability_ttl(),
        Some(&Duration::from_nanos(23))
    );
    let container = driver
        .resolve_container("testdb", "testcoll", defaults.clone())
        .await
        .unwrap();
    assert_eq!(transport.routing_reads.load(Ordering::SeqCst), 0);
    let response = driver
        .execute_singleton_operation(
            CosmosOperation::read_item(ItemReference::from_name(
                &container,
                PartitionKey::from("key"),
                "doc",
            )),
            defaults,
        )
        .await
        .unwrap();
    assert_eq!(transport.item_reads.load(Ordering::SeqCst), 19);
    let diagnostics = response.diagnostics();
    assert!(diagnostics.request_count() >= 19);
    assert!(diagnostics.retained_request_count() <= 16);
    assert!(diagnostics.retained_request_count() < diagnostics.request_count());
    assert_eq!(
        diagnostics.to_json_string(None),
        diagnostics.to_json_string(Some(DiagnosticsVerbosity::Detailed))
    );
    assert_ne!(
        diagnostics.to_json_string(None),
        diagnostics.to_json_string(Some(DiagnosticsVerbosity::Summary))
    );
    // Both budgets also permit the driver's single forced-final retry.
    for (nanoseconds, attempts) in [(999_999, 2), (1_000_001, 3)] {
        transport.item_reads.store(0, Ordering::SeqCst);
        let overrides = super::operation_options(
            OperationOptions::default(),
            CosmosDuration::default(),
            duration(0, nanoseconds),
            CosmosDuration::default(),
        )
        .unwrap();
        let error = driver
            .execute_singleton_operation(
                CosmosOperation::read_item(ItemReference::from_name(
                    &container,
                    PartitionKey::from("key"),
                    "doc",
                )),
                overrides,
            )
            .await
            .unwrap_err();
        assert_eq!(
            error.status().status_code(),
            azure_core::http::StatusCode::TooManyRequests
        );
        assert_eq!(transport.item_reads.load(Ordering::SeqCst), attempts);
    }
}
fn assert_client_identity(overrides: Vec<Arc<DriverOptionsHandle>>) {
    assert_eq!(
        overrides[0]
            .inner
            .account()
            .backup_endpoints()
            .iter()
            .map(|url| url.host_str().unwrap())
            .collect::<Vec<_>>(),
        ["backup-one.invalid", "backup-two.invalid"]
    );
    assert_eq!(
        overrides[0]
            .inner
            .partition_failover_options()
            .partition_topology_cache_mode(),
        PartitionTopologyCacheMode::Lazy
    );
    let tokio = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    tokio.block_on(async {
        let emulator = std::sync::Arc::new(
            azure_data_cosmos_driver::in_memory_emulator::InMemoryEmulatorHttpClient::new(
                azure_data_cosmos_driver::in_memory_emulator::VirtualAccountConfig::new(vec![
                    azure_data_cosmos_driver::in_memory_emulator::VirtualRegion::new(
                        "East US",
                        "https://eastus.emulator.local".parse().unwrap(),
                    ),
                ])
                .unwrap(),
            ),
        );
        let runtime = emulator
            .runtime_builder()
            .with_user_agent_suffix(
                azure_data_cosmos_driver::options::UserAgentSuffix::try_new("runtime").unwrap(),
            )
            .build()
            .await
            .unwrap();
        let original_identity = runtime.user_agent().as_str().to_string();
        let mut identities = Vec::new();
        for options in overrides {
            let driver = runtime.create_driver(options.inner.clone()).await.unwrap();
            identities.push(driver.user_agent().as_str().to_string());
        }
        assert!(identities[0].contains("client-a") && !identities[0].contains("client-b"));
        assert!(identities[1].contains("client-b") && !identities[1].contains("client-a"));
        assert!(identities[2].ends_with(" runtime"));
        assert!(!identities[2].contains("client-a") && !identities[2].contains("client-b"));
        assert_eq!(runtime.user_agent().as_str(), original_identity);
    });
}
