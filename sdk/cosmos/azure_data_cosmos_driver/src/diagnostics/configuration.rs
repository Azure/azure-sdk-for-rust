// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    models::DefaultConsistencyLevel,
    options::{
        ConnectionPoolOptions, DriverOptions, HedgingOptions, OperationOptions,
        OperationOptionsView, PartitionFailoverOptions, ReadConsistencyStrategy, Region,
        ThrottlingRetryOptions, ThroughputControlOptions,
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
    connection: Arc<ConnectionPoolOptions>,
    driver: Option<DriverConfiguration>,
}

impl Serialize for ClientConfiguration {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Output<'a> {
            driver_version: &'static str,
            #[serde(serialize_with = "serialize_features")]
            features: (),
            connection: &'a ConnectionPoolOptions,
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
            connection: Arc::new(pool.clone()),
            driver: None,
        }
    }

    pub(crate) fn for_driver(&self, options: &DriverOptions) -> Self {
        Self {
            connection: Arc::clone(&self.connection),
            driver: Some(DriverConfiguration {
                preferred_regions: options.preferred_regions().to_vec(),
                partition_failover: options.partition_failover_options().clone(),
                hedging: options.hedging_options().clone(),
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct DriverConfiguration {
    preferred_regions: Vec<Region>,
    partition_failover: PartitionFailoverOptions,
    hedging: HedgingOptions,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct OperationConfiguration {
    pub(crate) scope: &'static str,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub(crate) varied: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) account_consistency: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) effective_consistency: Option<&'static str>,
    custom_headers_configured_count: usize,
    #[serde(flatten)]
    pub(crate) options: OperationOptions,
}

impl From<&OperationOptionsView<'_>> for OperationConfiguration {
    fn from(view: &OperationOptionsView<'_>) -> Self {
        let throughput = view.throughput_control();
        let throughput_bucket = throughput.throughput_bucket().copied();
        let priority_level = throughput.priority_level().copied();
        let throttling = view.throttling_retry_options();
        Self {
            scope: "configured",
            varied: false,
            account_consistency: None,
            effective_consistency: None,
            custom_headers_configured_count: view.custom_headers().map_or(0, |h| h.len()),
            // Exhaustive initializers make new options require a diagnostics decision.
            options: OperationOptions {
                resolution_snapshot: None,
                custom_headers: None,
                #[cfg(feature = "unstable_patch")]
                patch_strategy: Some(view.patch_strategy().copied().unwrap_or_default()),
                read_consistency_strategy: Some(
                    view.read_consistency_strategy()
                        .copied()
                        .unwrap_or(ReadConsistencyStrategy::Default),
                ),
                excluded_regions: Some(view.excluded_regions().cloned().unwrap_or_default()),
                content_response_on_write: Some(
                    view.content_response_on_write()
                        .copied()
                        .unwrap_or_default(),
                ),
                throughput_control: (throughput_bucket.is_some() || priority_level.is_some())
                    .then_some(ThroughputControlOptions {
                        throughput_bucket,
                        priority_level,
                    }),
                end_to_end_latency_policy: view.end_to_end_latency_policy().cloned(),
                max_failover_retry_count: view.max_failover_retry_count().copied(),
                endpoint_unavailability_ttl: view.endpoint_unavailability_ttl().copied(),
                session_capturing_disabled: Some(
                    view.session_capturing_disabled().copied().unwrap_or(false),
                ),
                max_session_retry_count: view.max_session_retry_count().copied(),
                throttling_retry_options: Some(ThrottlingRetryOptions {
                    max_retry_count: throttling.max_retry_count().copied(),
                    max_retry_wait_time: throttling.max_retry_wait_time().copied(),
                }),
                hedging_enabled: view.hedging_enabled().copied(),
                availability_strategy: view.availability_strategy().copied(),
                binary_encoding: Some(view.binary_encoding().cloned().unwrap_or_default()),
                utf8_decoding_mode: Some(view.utf8_decoding_mode().copied().unwrap_or_default()),
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
    use crate::{
        models::AccountReference,
        options::{
            AvailabilityStrategy, BinaryEncodingOptions, ConnectionPoolOptions,
            ContentResponseOnWrite, DriverOptions, EndToEndOperationLatencyPolicy, ExcludedRegions,
            HedgeThreshold, HedgingOptions, HedgingStrategy, OperationOptions,
            OperationOptionsView, PartitionFailoverOptions, PriorityLevel, ReadConsistencyStrategy,
            ThrottlingRetryOptions, ThroughputControlOptions, Utf8DecodingMode,
        },
    };
    use std::{collections::BTreeSet, sync::Arc, time::Duration};

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
        let client = ClientConfiguration::for_runtime(&ConnectionPoolOptions::default());
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
                priority_level: Some(PriorityLevel::Low),
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
        assert!(snapshot.options.custom_headers.is_none());
        assert!(snapshot.options.resolution_snapshot.is_none());
        assert!(!serde_json::to_string(&request).unwrap().contains("secret"));
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
    fn canonical_options_preserve_diagnostics_units_and_policy_shape() {
        let request = OperationOptions {
            read_consistency_strategy: Some(ReadConsistencyStrategy::LatestCommitted),
            excluded_regions: Some(ExcludedRegions::new().with_region("westus")),
            content_response_on_write: Some(ContentResponseOnWrite::Enabled),
            throughput_control: Some(ThroughputControlOptions {
                throughput_bucket: Some(0),
                priority_level: Some(PriorityLevel::Low),
            }),
            end_to_end_latency_policy: Some(EndToEndOperationLatencyPolicy::new(
                Duration::from_millis(1_234),
            )),
            max_failover_retry_count: Some(0),
            endpoint_unavailability_ttl: Some(Duration::from_micros(1_234)),
            session_capturing_disabled: Some(false),
            max_session_retry_count: Some(0),
            throttling_retry_options: Some(ThrottlingRetryOptions {
                max_retry_count: Some(0),
                max_retry_wait_time: Some(Duration::ZERO),
            }),
            hedging_enabled: Some(false),
            availability_strategy: Some(AvailabilityStrategy::Hedging(HedgingStrategy::new(
                HedgeThreshold::new(Duration::from_millis(333)).unwrap(),
            ))),
            binary_encoding: Some(
                BinaryEncodingOptions::new()
                    .with_enabled(false)
                    .with_request_text_response(true),
            ),
            utf8_decoding_mode: Some(Utf8DecodingMode::Lossy),
            ..Default::default()
        };
        let snapshot = OperationConfiguration::from(&OperationOptionsView::new(
            None,
            None,
            None,
            Some(&request),
        ));
        let json = serde_json::to_value(snapshot).unwrap();
        let mut expected = serde_json::json!({
            "scope": "configured",
            "custom_headers_configured_count": 0,
            "read_consistency_strategy": "latest_committed",
            "excluded_regions": ["westus"],
            "content_response_on_write": "enabled",
            "throughput_control": {"throughput_bucket": 0, "priority_level": "low"},
            "end_to_end_timeout_ms": 1234,
            "max_failover_retry_count": 0,
            "endpoint_unavailability_ttl_ms": 1,
            "session_capturing_disabled": false,
            "max_session_retry_count": 0,
            "throttling_retry_options": {"max_retry_count": 0, "max_retry_wait_time_ms": 0},
            "hedging_enabled": false,
            "availability_strategy": {"policy": "hedging", "threshold_ms": 333},
            "binary_encoding": {"enabled": false, "request_text_response": true},
            "utf8_decoding_mode": "lossy"
        });
        if cfg!(feature = "unstable_patch") {
            expected["patch_strategy"] = serde_json::json!("auto");
        }
        assert_eq!(json, expected);

        let disabled = OperationOptions {
            availability_strategy: Some(AvailabilityStrategy::Disabled),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(disabled).unwrap()["availability_strategy"],
            serde_json::json!({"policy": "disabled"})
        );
    }

    #[test]
    fn client_uses_canonical_options_without_exposing_local_address() {
        let pool = ConnectionPoolOptions::builder()
            .with_local_address("192.0.2.1".parse().unwrap())
            .build()
            .unwrap();
        let partition_failover = PartitionFailoverOptions::builder()
            .with_circuit_breaker_enabled_override(false)
            .build_from_env(&|_| None)
            .unwrap();
        let hedging = HedgingOptions::builder()
            .with_max_concurrent_metadata_attempts(0)
            .build();
        let driver = DriverOptions::builder(AccountReference::with_account_key(
            "https://example.documents.azure.com/".parse().unwrap(),
            "secret-account-key",
        ))
        .with_partition_failover_options(partition_failover.clone())
        .with_hedging_options(hedging.clone())
        .build();
        let runtime = ClientConfiguration::for_runtime(&pool);
        let json = serde_json::to_value(runtime.for_driver(&driver)).unwrap();
        assert_eq!(json["connection"], serde_json::to_value(&pool).unwrap());
        assert_eq!(
            json["connection"]["target_http2_streams_per_client"],
            pool.target_http2_streams_per_client()
        );
        assert_eq!(
            json["partition_failover"],
            serde_json::to_value(partition_failover).unwrap()
        );
        assert_eq!(
            json["partition_failover"]["circuit_breaker_enabled_override"],
            false
        );
        assert_eq!(json["hedging"], serde_json::to_value(hedging).unwrap());
        let serialized = json.to_string();
        assert!(!serialized.contains("192.0.2.1"));
        assert!(!serialized.contains("local_address"));
        assert!(!serialized.contains("secret-account-key"));
    }

    #[test]
    fn projection_honors_pinned_layers_and_kill_switches() {
        let pinned = OperationOptions {
            hedging_enabled: Some(true),
            excluded_regions: Some(ExcludedRegions::new()),
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
                excluded_regions: Some(ExcludedRegions::new().with_region("westus")),
                ..Default::default()
            }),
        );
        let pinned_json = serde_json::to_value(&pinned).unwrap();
        assert!(pinned_json.get("resolution_snapshot").is_none());
        assert!(pinned_json.get("max_session_retry_count").is_none());
        let snapshot = OperationConfiguration::from(&pinned.resolution_snapshot_view().unwrap());
        assert!(snapshot.options.resolution_snapshot.is_none());
        let json = serde_json::to_value(snapshot).unwrap();
        assert_eq!(json["hedging_enabled"], false);
        assert_eq!(json["max_session_retry_count"], 8);
        assert_eq!(json["excluded_regions"], serde_json::json!([]));
    }
}
