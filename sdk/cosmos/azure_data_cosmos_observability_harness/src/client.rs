// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos client construction: registers the built-in diagnostics handlers and
//! (optionally) fault-injection rules, and wires up an emulator-friendly runtime.

use std::error::Error;
use std::sync::Arc;

use azure_core::credentials::Secret;
use azure_data_cosmos::diagnostics::SamplingLogHandler;
use azure_data_cosmos::options::{ConnectionPoolOptions, ServerCertificateValidation};
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, CosmosClientBuilder, CosmosRuntime,
    RoutingStrategy,
};

use crate::config::{AuthMethod, Config};

/// Known loopback hosts the SDK already recognizes as emulator endpoints without
/// any extra configuration.
const EMULATOR_LOCALHOST_HOSTS: &[&str] = &["localhost", "127.0.0.1", "[::1]", "[0:0:0:0:0:0:0:1]"];

/// Builds a fully-configured [`CosmosClient`] with the observability handlers
/// registered, plus a [`FaultActivation`] the caller arms once the load loop
/// starts (keeping setup and seeding fault-free).
///
/// The global OpenTelemetry providers must already be installed (see
/// [`crate::telemetry`]) so the metrics handler binds to a live meter at
/// construction.
pub async fn build_client(
    config: &Config,
) -> Result<(CosmosClient, FaultActivation), Box<dyn Error>> {
    let (endpoint_str, key) = config.resolve_endpoint_and_key()?;
    let endpoint: AccountEndpoint = endpoint_str.parse()?;
    let strategy = RoutingStrategy::ProximityTo(config.region.clone().into());

    // Honor `--emulator` for a *custom* (non-localhost) emulator host: the SDK's
    // `RequiredUnlessEmulator` policy only relaxes TLS for hosts it recognizes
    // (localhost variants or an exact `AZURE_COSMOS_EMULATOR_HOST` match), so
    // point that variable at the requested endpoint. `--emulator` + `--endpoint`
    // explicitly select this host, so overwrite any stale pre-existing value.
    if config.emulator {
        if let Some(host) = endpoint.url().host_str() {
            let is_localhost = EMULATOR_LOCALHOST_HOSTS
                .iter()
                .any(|h| host.eq_ignore_ascii_case(h));
            if !is_localhost {
                std::env::set_var("AZURE_COSMOS_EMULATOR_HOST", host);
            }
        }
    }

    let mut builder = CosmosClientBuilder::new();

    // The emulator serves a self-signed certificate, so route through a runtime
    // that skips certificate validation for emulator hosts.
    if config.is_emulator(&endpoint_str) {
        let runtime = CosmosRuntime::builder()
            .with_connection_pool(
                ConnectionPoolOptions::builder()
                    .with_server_certificate_validation(
                        ServerCertificateValidation::RequiredUnlessEmulator,
                    )
                    .build()?,
            )
            .build()
            .await?;
        builder = builder.with_runtime(runtime);
    }

    builder = register_handlers(builder, config);

    #[cfg(feature = "fault_injection")]
    let fault_activation = {
        let (updated, rule) = apply_fault_injection(builder, config)?;
        builder = updated;
        FaultActivation {
            schedule: rule.map(|rule| FaultSchedule {
                rule,
                start_secs: config.fault_start_secs,
                duration_secs: config.fault_duration_secs,
            }),
        }
    };
    #[cfg(not(feature = "fault_injection"))]
    let fault_activation = FaultActivation::default();

    let account = match config.auth {
        AuthMethod::Key => {
            let key = key.ok_or("key authentication requires an account key")?;
            AccountReference::with_authentication_key(endpoint, Secret::from(key))
        }
        AuthMethod::Aad => {
            let credential: Arc<dyn azure_core::credentials::TokenCredential> =
                azure_identity::DeveloperToolsCredential::new(None)?;
            AccountReference::with_credential(endpoint, credential)
        }
    };

    Ok((builder.build(account, strategy).await?, fault_activation))
}

/// Schedules activation of the (optional) fault-injection rule relative to the
/// start of the load loop, so database/container setup and seeding always run
/// fault-free and `--fault-start-secs` is measured from when real load begins.
#[derive(Default)]
pub struct FaultActivation {
    #[cfg(feature = "fault_injection")]
    schedule: Option<FaultSchedule>,
}

#[cfg(feature = "fault_injection")]
struct FaultSchedule {
    rule: Arc<azure_data_cosmos::fault_injection::FaultInjectionRule>,
    start_secs: u64,
    duration_secs: u64,
}

impl FaultActivation {
    /// Arms the fault window relative to *now*. Call at the start of the load
    /// loop (after seeding): the rule is enabled after `--fault-start-secs` and,
    /// when `--fault-duration-secs` is set, disabled again after the window. A
    /// no-op when fault injection is disabled or no rule was configured.
    pub fn arm(self) {
        #[cfg(feature = "fault_injection")]
        if let Some(schedule) = self.schedule {
            use std::time::Duration;

            let FaultSchedule {
                rule,
                start_secs,
                duration_secs,
            } = schedule;

            // With no start delay, enable synchronously here so the very first
            // load operations are covered — `tokio::spawn` would not run before
            // the workers start issuing requests.
            if start_secs == 0 {
                rule.enable();
                tracing::warn!(start_secs, "fault window opened");
                if duration_secs > 0 {
                    tokio::spawn(async move {
                        tokio::time::sleep(Duration::from_secs(duration_secs)).await;
                        rule.disable();
                        tracing::warn!(duration_secs, "fault window closed");
                    });
                }
            } else {
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_secs(start_secs)).await;
                    rule.enable();
                    tracing::warn!(start_secs, "fault window opened");
                    if duration_secs > 0 {
                        tokio::time::sleep(Duration::from_secs(duration_secs)).await;
                        rule.disable();
                        tracing::warn!(duration_secs, "fault window closed");
                    }
                });
            }
        }
    }
}

/// Registers the built-in diagnostics handlers. Handlers run in registration
/// order: metrics, then distributed tracing (both feature-gated), then the
/// always-on sampled log handler.
#[cfg_attr(not(feature = "metrics"), allow(unused_variables))]
fn register_handlers(builder: CosmosClientBuilder, config: &Config) -> CosmosClientBuilder {
    // Each handler is appended via cfg-gated shadowing so the chain builds
    // cleanly regardless of which handler features are enabled.
    #[cfg(feature = "metrics")]
    let builder = {
        use azure_data_cosmos::diagnostics::{CosmosMetricsHandler, MetricsOptions};

        let options = if config.extended_metrics {
            MetricsOptions::default()
                .with_request_charge_metric(true)
                .with_returned_rows_metric(true)
                .with_extended_attributes(true)
        } else {
            MetricsOptions::default()
        };
        builder.with_diagnostics_handler(Arc::new(CosmosMetricsHandler::with_options(options)))
    };

    #[cfg(feature = "distributed_tracing")]
    let builder = {
        use azure_data_cosmos::diagnostics::CosmosTracingHandler;

        builder.with_diagnostics_handler(Arc::new(CosmosTracingHandler::new()))
    };

    // The sampled log handler is not feature-gated in the SDK; always register it
    // so failures/threshold breaches surface on the `tracing` pipeline.
    builder.with_diagnostics_handler(Arc::new(SamplingLogHandler::new()))
}

/// Builds the single fault-injection rule from the CLI configuration and
/// registers it, returning a handle so the caller can arm it on a schedule. The
/// rule is built **disabled** so that database/container setup and seeding run
/// fault-free; [`FaultActivation::arm`] enables it (and later disables it) once
/// the load loop starts. Returns `None` when no fault probability is requested.
#[cfg(feature = "fault_injection")]
#[allow(clippy::type_complexity)]
fn apply_fault_injection(
    builder: CosmosClientBuilder,
    config: &Config,
) -> Result<
    (
        CosmosClientBuilder,
        Option<Arc<azure_data_cosmos::fault_injection::FaultInjectionRule>>,
    ),
    Box<dyn Error>,
> {
    use std::time::Duration;

    use azure_data_cosmos::fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
        FaultInjectionRuleBuilder, FaultOperationType,
    };

    use crate::config::{FaultError, FaultOp};

    if config.fault_probability <= 0.0 {
        return Ok((builder, None));
    }

    let error_type = match config.fault_error {
        FaultError::ServiceUnavailable => FaultInjectionErrorType::ServiceUnavailable,
        FaultError::TooManyRequests => FaultInjectionErrorType::TooManyRequests,
        FaultError::InternalServerError => FaultInjectionErrorType::InternalServerError,
        FaultError::Timeout => FaultInjectionErrorType::Timeout,
        FaultError::RetryWith => FaultInjectionErrorType::RetryWith,
        FaultError::ConnectionError => FaultInjectionErrorType::ConnectionError,
    };

    let mut result = FaultInjectionResultBuilder::new()
        .with_error(error_type)
        .with_probability(config.fault_probability as f32);
    if config.fault_delay_ms > 0 {
        result = result.with_delay(Duration::from_millis(config.fault_delay_ms));
    }
    let result = result.build();

    let condition = {
        let mut condition = FaultInjectionConditionBuilder::new();
        let op_type = match config.fault_operation {
            FaultOp::All => None,
            FaultOp::Read => Some(FaultOperationType::ReadItem),
            FaultOp::Write => Some(FaultOperationType::UpsertItem),
            FaultOp::Query => Some(FaultOperationType::QueryItem),
        };
        if let Some(op_type) = op_type {
            condition = condition.with_operation_type(op_type);
        }
        condition.build()
    };

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("soak-fault", result)
            .with_condition(condition)
            .build(),
    );
    // Start disabled so setup/seeding are never faulted; the window is armed
    // (relative to the load loop) by `FaultActivation::arm`.
    rule.disable();

    tracing::warn!(
        probability = config.fault_probability,
        delay_ms = config.fault_delay_ms,
        start_secs = config.fault_start_secs,
        duration_secs = config.fault_duration_secs,
        "fault injection configured (armed at load start)"
    );

    Ok((
        builder.with_fault_injection_rules(vec![Arc::clone(&rule)])?,
        Some(rule),
    ))
}
