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

/// Builds a fully-configured [`CosmosClient`] with the observability handlers
/// registered.
///
/// The global OpenTelemetry providers must already be installed (see
/// [`crate::telemetry`]) so the metrics handler binds to a live meter at
/// construction.
pub async fn build_client(config: &Config) -> Result<CosmosClient, Box<dyn Error>> {
    let (endpoint_str, key) = config.resolve_endpoint_and_key()?;
    let endpoint: AccountEndpoint = endpoint_str.parse()?;
    let strategy = RoutingStrategy::ProximityTo(config.region.clone().into());

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
    {
        builder = apply_fault_injection(builder, config)?;
    }

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

    Ok(builder.build(account, strategy).await?)
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

/// Adds a single fault-injection rule derived from the CLI configuration, when a
/// non-zero probability is requested.
#[cfg(feature = "fault_injection")]
fn apply_fault_injection(
    builder: CosmosClientBuilder,
    config: &Config,
) -> Result<CosmosClientBuilder, Box<dyn Error>> {
    use std::time::Duration;

    use azure_data_cosmos::fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
        FaultInjectionRuleBuilder, FaultOperationType,
    };

    use crate::config::{FaultError, FaultOp};

    if config.fault_probability <= 0.0 {
        return Ok(builder);
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

    let rule = FaultInjectionRuleBuilder::new("soak-fault", result)
        .with_condition(condition)
        .build();

    tracing::warn!(
        probability = config.fault_probability,
        delay_ms = config.fault_delay_ms,
        "fault injection enabled"
    );

    Ok(builder.with_fault_injection_rules(vec![Arc::new(rule)])?)
}
