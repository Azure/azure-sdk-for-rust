// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    models::DefaultConsistencyLevel,
    options::{
        AvailabilityStrategy, ConnectionPoolOptions, ContentResponseOnWrite, DriverOptions,
        OperationOptionsView, PartitionFailoverOptions, PartitionTopologyCacheMode, PriorityLevel,
        ReadConsistencyStrategy, Region, ServerCertificateValidation, Utf8DecodingMode,
    },
};
use serde::Serialize;
use std::sync::Arc;

// Keep names and cfgs together; the manifest-parity test also checks disabled features.
macro_rules! feature_inventory {
    ($($name:literal),* $(,)?) => {
        const FEATURES: &[(&str, bool)] = &[$(($name, cfg!(feature = $name))),*];
    };
}

feature_inventory!(
    "__internal_backtrace_bench",
    "__internal_in_memory_emulator",
    "__internal_mocking",
    "__internal_native_query_plan",
    "__internal_test_diagnostics_construction",
    "__internal_testing",
    "__tls",
    "default",
    "fault_injection",
    "native_tls",
    "reqwest",
    "rustls",
    "tokio",
    "unstable_dtx",
    "unstable_patch",
);

fn enabled_features() -> impl Iterator<Item = &'static str> {
    FEATURES
        .iter()
        .filter_map(|(name, enabled)| enabled.then_some(*name))
}

fn serialize_features<S: serde::Serializer>(_: &(), serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(enabled_features())
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ClientConfiguration {
    connection: Arc<ConnectionConfiguration>,
    driver: Option<DriverConfiguration>,
}

impl Serialize for ClientConfiguration {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Output<'a> {
            driver_version: &'static str,
            #[serde(serialize_with = "serialize_features")]
            features: (),
            connection: &'a ConnectionConfiguration,
            #[serde(flatten)]
            driver: Option<&'a DriverConfiguration>,
        }
        Output {
            driver_version: env!("CARGO_PKG_VERSION"),
            features: (),
            connection: &self.connection,
            driver: self.driver.as_ref(),
        }
        .serialize(serializer)
    }
}

impl ClientConfiguration {
    pub(crate) fn for_runtime(pool: &ConnectionPoolOptions) -> Self {
        Self {
            connection: Arc::new(ConnectionConfiguration::from(pool)),
            driver: None,
        }
    }

    pub(crate) fn for_driver(&self, options: &DriverOptions) -> Self {
        Self {
            connection: Arc::clone(&self.connection),
            driver: Some(DriverConfiguration {
                preferred_regions: options.preferred_regions().to_vec(),
                partition_failover: PartitionFailoverConfiguration::from(
                    options.partition_failover_options(),
                ),
                hedging: HedgeLimits {
                    max_concurrent_metadata_attempts: options
                        .hedging_options()
                        .max_concurrent_metadata_attempts(),
                },
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct DriverConfiguration {
    preferred_regions: Vec<Region>,
    partition_failover: PartitionFailoverConfiguration,
    hedging: HedgeLimits,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct HedgeLimits {
    max_concurrent_metadata_attempts: usize,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct PartitionFailoverConfiguration {
    topology_cache_mode: &'static str,
    circuit_breaker_enabled: bool,
    read_failure_threshold: u32,
    write_failure_threshold: u32,
    counter_reset_window_ms: u128,
    partition_unavailability_duration_ms: u128,
    failback_sweep_interval_ms: u128,
    consecutive_hedge_win_threshold: u32,
}

impl From<&PartitionFailoverOptions> for PartitionFailoverConfiguration {
    fn from(options: &PartitionFailoverOptions) -> Self {
        Self {
            topology_cache_mode: match options.partition_topology_cache_mode() {
                PartitionTopologyCacheMode::Eager => "eager",
                PartitionTopologyCacheMode::Lazy => "lazy",
            },
            circuit_breaker_enabled: options.circuit_breaker_enabled(),
            read_failure_threshold: options.read_failure_threshold(),
            write_failure_threshold: options.write_failure_threshold(),
            counter_reset_window_ms: options.counter_reset_window().as_millis(),
            partition_unavailability_duration_ms: options
                .partition_unavailability_duration()
                .as_millis(),
            failback_sweep_interval_ms: options.failback_sweep_interval().as_millis(),
            consecutive_hedge_win_threshold: options.consecutive_hedge_win_threshold(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct ConnectionConfiguration {
    proxy_allowed: bool,
    min_connect_timeout_ms: u128,
    max_connect_timeout_ms: u128,
    min_dataplane_request_timeout_ms: u128,
    max_dataplane_request_timeout_ms: u128,
    min_metadata_request_timeout_ms: u128,
    max_metadata_request_timeout_ms: u128,
    max_idle_connections_per_endpoint: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_connection_timeout_ms: Option<u128>,
    max_http2_streams_per_client: u32,
    http2_fan_out_threshold_percent: u8,
    max_http2_connections_per_endpoint: usize,
    min_http2_connections_per_endpoint: usize,
    idle_http2_client_timeout_ms: u128,
    http2_health_check_interval_ms: u128,
    http2_consecutive_failure_threshold: u32,
    http2_eviction_grace_period_ms: u128,
    http2_keep_alive_interval_ms: u128,
    http2_keep_alive_timeout_ms: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_keepalive_time_ms: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_keepalive_interval_ms: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tcp_keepalive_retries: Option<u32>,
    http2_allowed: bool,
    gateway_v2_disabled: bool,
    server_certificate_validation: &'static str,
    #[cfg(feature = "rustls")]
    tls_backend: &'static str,
}

impl From<&ConnectionPoolOptions> for ConnectionConfiguration {
    fn from(pool: &ConnectionPoolOptions) -> Self {
        Self {
            proxy_allowed: pool.proxy_allowed(),
            min_connect_timeout_ms: pool.min_connect_timeout().as_millis(),
            max_connect_timeout_ms: pool.max_connect_timeout().as_millis(),
            min_dataplane_request_timeout_ms: pool.min_dataplane_request_timeout().as_millis(),
            max_dataplane_request_timeout_ms: pool.max_dataplane_request_timeout().as_millis(),
            min_metadata_request_timeout_ms: pool.min_metadata_request_timeout().as_millis(),
            max_metadata_request_timeout_ms: pool.max_metadata_request_timeout().as_millis(),
            max_idle_connections_per_endpoint: pool.max_idle_connections_per_endpoint(),
            idle_connection_timeout_ms: pool.idle_connection_timeout().map(|d| d.as_millis()),
            max_http2_streams_per_client: pool.max_http2_streams_per_client(),
            http2_fan_out_threshold_percent: pool.http2_fan_out_threshold_percent(),
            max_http2_connections_per_endpoint: pool.max_http2_connections_per_endpoint(),
            min_http2_connections_per_endpoint: pool.min_http2_connections_per_endpoint(),
            idle_http2_client_timeout_ms: pool.idle_http2_client_timeout().as_millis(),
            http2_health_check_interval_ms: pool.http2_health_check_interval().as_millis(),
            http2_consecutive_failure_threshold: pool.http2_consecutive_failure_threshold(),
            http2_eviction_grace_period_ms: pool.http2_eviction_grace_period().as_millis(),
            http2_keep_alive_interval_ms: pool.http2_keep_alive_interval().as_millis(),
            http2_keep_alive_timeout_ms: pool.http2_keep_alive_timeout().as_millis(),
            tcp_keepalive_time_ms: pool.tcp_keepalive_time().map(|d| d.as_millis()),
            tcp_keepalive_interval_ms: pool.tcp_keepalive_interval().map(|d| d.as_millis()),
            tcp_keepalive_retries: pool.tcp_keepalive_retries(),
            http2_allowed: pool.is_http2_allowed(),
            gateway_v2_disabled: pool.gateway_v2_disabled(),
            server_certificate_validation: match pool.server_certificate_validation() {
                ServerCertificateValidation::Required => "required",
                ServerCertificateValidation::RequiredUnlessEmulator => "required_unless_emulator",
            },
            #[cfg(feature = "rustls")]
            tls_backend: match pool.tls_backend() {
                crate::options::TlsBackend::Rustls => "rustls",
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct OperationConfiguration {
    pub(crate) scope: &'static str,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub(crate) varied: bool,
    read_consistency_strategy: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) account_consistency: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) effective_consistency: Option<&'static str>,
    excluded_regions: Vec<Region>,
    content_response_on_write: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput_control: Option<ThroughputConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_to_end_timeout_ms: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_failover_retry_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) endpoint_unavailability_ttl_ms: Option<u128>,
    session_capturing_disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_session_retry_count: Option<u32>,
    pub(crate) throttling_retry_options: ThrottlingConfiguration,
    #[serde(skip_serializing_if = "Option::is_none")]
    hedging_enabled: Option<bool>,
    availability_strategy: AvailabilityConfiguration,
    custom_headers_configured_count: usize,
    binary_encoding: BinaryConfiguration,
    utf8_decoding_mode: &'static str,
    #[cfg(feature = "unstable_patch")]
    patch_strategy: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ThroughputConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput_bucket: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority_level: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct ThrottlingConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_retry_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_retry_wait_time_ms: Option<u128>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct AvailabilityConfiguration {
    policy: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold_ms: Option<u128>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct BinaryConfiguration {
    enabled: bool,
    request_text_response: bool,
}

impl From<&OperationOptionsView<'_>> for OperationConfiguration {
    fn from(view: &OperationOptionsView<'_>) -> Self {
        let throughput = view.throughput_control();
        let throughput_bucket = throughput.throughput_bucket().copied();
        let priority_level = throughput.priority_level().map(|p| match p {
            PriorityLevel::High => "high",
            PriorityLevel::Low => "low",
        });
        let throttling = view.throttling_retry_options();
        let binary = view.binary_encoding().cloned().unwrap_or_default();
        Self {
            scope: "configured",
            varied: false,
            read_consistency_strategy: match view.read_consistency_strategy() {
                None | Some(ReadConsistencyStrategy::Default) => "default",
                Some(ReadConsistencyStrategy::Eventual) => "eventual",
                Some(ReadConsistencyStrategy::Session) => "session",
                Some(ReadConsistencyStrategy::LatestCommitted) => "latest_committed",
                Some(ReadConsistencyStrategy::GlobalStrong) => "global_strong",
            },
            account_consistency: None,
            effective_consistency: None,
            excluded_regions: view
                .excluded_regions()
                .map(|r| r.iter().cloned().collect())
                .unwrap_or_default(),
            content_response_on_write: match view
                .content_response_on_write()
                .copied()
                .unwrap_or_default()
            {
                ContentResponseOnWrite::Enabled => "enabled",
                ContentResponseOnWrite::Disabled => "disabled",
            },
            throughput_control: (throughput_bucket.is_some() || priority_level.is_some())
                .then_some(ThroughputConfiguration {
                    throughput_bucket,
                    priority_level,
                }),
            end_to_end_timeout_ms: view
                .end_to_end_latency_policy()
                .map(|p| p.timeout().as_millis()),
            max_failover_retry_count: view.max_failover_retry_count().copied(),
            endpoint_unavailability_ttl_ms: view
                .endpoint_unavailability_ttl()
                .map(|d| d.as_millis()),
            session_capturing_disabled: view.session_capturing_disabled().copied().unwrap_or(false),
            max_session_retry_count: view.max_session_retry_count().copied(),
            throttling_retry_options: ThrottlingConfiguration {
                max_retry_count: throttling.max_retry_count().copied(),
                max_retry_wait_time_ms: throttling.max_retry_wait_time().map(|d| d.as_millis()),
            },
            hedging_enabled: view.hedging_enabled().copied(),
            availability_strategy: match view.availability_strategy() {
                Some(AvailabilityStrategy::Disabled) => AvailabilityConfiguration {
                    policy: "disabled",
                    threshold_ms: None,
                },
                Some(AvailabilityStrategy::Hedging(strategy)) => AvailabilityConfiguration {
                    policy: "hedging",
                    threshold_ms: Some(strategy.threshold().get().as_millis()),
                },
                None => AvailabilityConfiguration {
                    policy: "default",
                    threshold_ms: None,
                },
            },
            custom_headers_configured_count: view.custom_headers().map_or(0, |h| h.len()),
            binary_encoding: BinaryConfiguration {
                enabled: binary.enabled,
                request_text_response: binary.request_text_response,
            },
            utf8_decoding_mode: match view.utf8_decoding_mode().copied().unwrap_or_default() {
                Utf8DecodingMode::Strict => "strict",
                Utf8DecodingMode::Lossy => "lossy",
            },
            #[cfg(feature = "unstable_patch")]
            patch_strategy: match view.patch_strategy().copied().unwrap_or_default() {
                crate::options::PatchStrategy::Auto => "auto",
                crate::options::PatchStrategy::ClientSide => "client_side",
                crate::options::PatchStrategy::ServerSide => "server_side",
            },
        }
    }
}

impl OperationConfiguration {
    pub(crate) fn same_settings(&self, other: &Self) -> bool {
        let mut settings = self.clone();
        settings.scope = other.scope;
        settings.varied = other.varied;
        settings == *other
    }

    pub(crate) fn set_consistency(
        &mut self,
        account: DefaultConsistencyLevel,
        effective: DefaultConsistencyLevel,
    ) {
        fn name(level: DefaultConsistencyLevel) -> &'static str {
            match level {
                DefaultConsistencyLevel::Strong => "strong",
                DefaultConsistencyLevel::BoundedStaleness => "bounded_staleness",
                DefaultConsistencyLevel::Session => "session",
                DefaultConsistencyLevel::ConsistentPrefix => "consistent_prefix",
                DefaultConsistencyLevel::Eventual => "eventual",
            }
        }
        self.account_consistency = Some(name(account));
        self.effective_consistency = Some(name(effective));
    }
}

#[cfg(test)]
mod tests {
    use super::{enabled_features, ClientConfiguration, OperationConfiguration, FEATURES};
    use crate::options::{OperationOptions, OperationOptionsView, ThroughputControlOptions};
    use std::{collections::BTreeSet, sync::Arc};

    #[test]
    fn feature_inventory_matches_manifest() {
        let manifest: toml::Value = toml::from_str(include_str!("../../Cargo.toml")).unwrap();
        let declared: BTreeSet<_> = manifest["features"]
            .as_table()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        let inventory: BTreeSet<_> = FEATURES.iter().map(|(name, _)| *name).collect();
        assert_eq!(inventory, declared);
        assert!(FEATURES.windows(2).all(|pair| pair[0].0 < pair[1].0));
        let client =
            ClientConfiguration::for_runtime(&crate::options::ConnectionPoolOptions::default());
        let json = serde_json::to_value(client).unwrap();
        assert_eq!(
            json["features"],
            serde_json::json!(enabled_features().collect::<Vec<_>>())
        );
        eprintln!("Active driver features: {}", json["features"]);
    }

    #[test]
    fn projection_resolves_nested_options_and_excludes_header_contents() {
        let runtime = Arc::new(OperationOptions {
            max_session_retry_count: Some(5),
            throughput_control: Some(ThroughputControlOptions {
                throughput_bucket: Some(4),
                priority_level: Some(crate::options::PriorityLevel::Low),
            }),
            ..Default::default()
        });
        let request = OperationOptions {
            max_session_retry_count: Some(0),
            session_capturing_disabled: Some(false),
            throughput_control: Some(ThroughputControlOptions {
                throughput_bucket: Some(0),
                ..Default::default()
            }),
            custom_headers: Some(
                [(
                    azure_core::http::headers::HeaderName::from_static("x-secret"),
                    azure_core::http::headers::HeaderValue::from("secret-value"),
                )]
                .into(),
            ),
            ..Default::default()
        };
        let view =
            OperationOptionsView::new(None, Some(Arc::clone(&runtime)), None, Some(&request));
        let snapshot = OperationConfiguration::from(&view);
        drop(view);
        let json = serde_json::to_value(snapshot).unwrap();
        assert_eq!(json["max_session_retry_count"], 0);
        assert_eq!(
            json["throughput_control"],
            serde_json::json!({"throughput_bucket": 0, "priority_level": "low"})
        );
        assert_eq!(json["custom_headers_configured_count"], 1);
        assert!(!json.to_string().contains("secret"));
        assert_eq!(json["read_consistency_strategy"], "default");
        assert!(json.get("end_to_end_timeout_ms").is_none());
    }

    #[test]
    fn projection_honors_pinned_layers_and_kill_switches() {
        let pinned = OperationOptions {
            hedging_enabled: Some(true),
            excluded_regions: Some(crate::options::ExcludedRegions::new()),
            ..Default::default()
        }
        .with_resolution_snapshot(
            Arc::new(OperationOptions::from_env_override_vars(|key| {
                if key == "AZURE_COSMOS_HEDGING_ENABLED_OVERRIDE" {
                    Ok("false".into())
                } else {
                    Err(std::env::VarError::NotPresent)
                }
            })),
            Arc::new(OperationOptions::default()),
            Arc::new(OperationOptions {
                max_session_retry_count: Some(4),
                ..Default::default()
            }),
            Arc::new(OperationOptions {
                max_session_retry_count: Some(8),
                excluded_regions: Some(
                    crate::options::ExcludedRegions::new().with_region("westus"),
                ),
                ..Default::default()
            }),
        );
        let snapshot = OperationConfiguration::from(&pinned.resolution_snapshot_view().unwrap());
        let json = serde_json::to_value(snapshot).unwrap();
        assert_eq!(json["hedging_enabled"], false);
        assert_eq!(json["max_session_retry_count"], 8);
        assert_eq!(json["excluded_regions"], serde_json::json!([]));
    }
}
