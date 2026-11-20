// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Additive construction records. Legacy by-value records remain unchanged.

#[cfg(test)]
mod tests;

use crate::{
    account_ref::AccountRefHandle,
    driver_options::{
        cosmos_driver_options_build_v2, cosmos_driver_options_free, CosmosDriverOptionsConfigV2,
        DriverOptionsHandle,
    },
    error::{CosmosError, CosmosErrorCode, CosmosStatusCode, COSMOS_STATUS_SUCCESS},
    op_request::{decode_tristate_bool, CosmosOperationOptions},
    runtime::RuntimeContext,
    runtime_builder::{finish_runtime_build, CosmosRuntimeOptions},
    string::{optional_text, required_text, validate_array, CosmosStringView},
};
use azure_data_cosmos_driver::{
    models::{
        AutoscaleAutoUpgradePolicy, AutoscaleThroughputPolicy, CosmosRequestHeaders,
        OfferAutoscaleSettings,
    },
    options::{
        AvailabilityStrategy, ConnectionPoolOptions, DiagnosticsOptions, DiagnosticsVerbosity,
        DriverOptions, HedgeThreshold, HedgingStrategy, OperationOptions, PartitionFailoverOptions,
        PartitionTopologyCacheMode, ServerCertificateValidation, TlsBackend, UserAgentSuffix,
    },
    CosmosDriverRuntimeBuilder,
};
use std::{
    mem::size_of,
    time::{Duration, Instant},
};
use url::Url;

const INVALID: CosmosErrorCode = CosmosErrorCode::CosmosErrorCodeInvalidOptionValue;

/// Typed throughput headers for database and container creation.
///
/// Initialize or reset with [`cosmos_management_options_init()`]. Unset leaves
/// inherited custom headers unchanged. Set values take precedence over conflicting
/// custom headers without replacing their map. All inputs are copied during submit.
#[repr(C)]
pub struct CosmosManagementOptions {
    pub struct_size_bytes: u32,
    pub abi_version: u32,
    /// 0 unset, 1 manual, 2 autoscale. Inactive payloads must be zero.
    pub throughput_kind: u32,
    /// Positive RU/s when `throughput_kind` is 1; otherwise zero.
    pub manual_throughput: u64,
    /// Positive maximum RU/s when `throughput_kind` is 2; otherwise zero.
    pub autoscale_max_throughput: u64,
    /// 0 absent, 1 present. Only valid for autoscale.
    pub has_auto_upgrade_policy: u8,
    /// 0 absent, 1 present. Requires `has_auto_upgrade_policy` to be 1.
    pub has_throughput_policy: u8,
    /// Full unsigned percentage; zero when the throughput policy is absent.
    pub increment_percent: u32,
}

pub(crate) unsafe fn management_headers(
    input: *const CosmosManagementOptions,
) -> Result<Option<CosmosRequestHeaders>, CosmosStatusCode> {
    if input.is_null() {
        return Ok(None);
    }
    // SAFETY: caller supplies readable versioned storage for the duration of submit.
    let input = unsafe { record(input)? };
    if input.has_auto_upgrade_policy > 1
        || input.has_throughput_policy > input.has_auto_upgrade_policy
        || (input.has_throughput_policy == 0 && input.increment_percent != 0)
    {
        return Err(INVALID.as_status_code());
    }
    let no_policy = input.has_auto_upgrade_policy == 0;
    let mut headers = CosmosRequestHeaders::default();
    match input.throughput_kind {
        0 if input.manual_throughput == 0 && input.autoscale_max_throughput == 0 && no_policy => {
            return Ok(None);
        }
        1 if input.manual_throughput > 0 && input.autoscale_max_throughput == 0 && no_policy => {
            headers.offer_throughput = Some(input.manual_throughput);
        }
        2 if input.manual_throughput == 0 && input.autoscale_max_throughput > 0 => {
            let mut settings = OfferAutoscaleSettings::new(input.autoscale_max_throughput);
            if input.has_auto_upgrade_policy == 1 {
                let mut policy = AutoscaleAutoUpgradePolicy::default();
                if input.has_throughput_policy == 1 {
                    let mut throughput = AutoscaleThroughputPolicy::default();
                    throughput.increment_percent = input.increment_percent;
                    policy.throughput_policy = Some(throughput);
                }
                settings.auto_upgrade_policy = Some(policy);
            }
            headers.offer_autopilot_settings = Some(settings);
        }
        _ => return Err(INVALID.as_status_code()),
    }
    Ok(Some(headers))
}

/// Initializes or resets typed management overrides to unset.
///
/// Resetting removes only these typed overrides, not inherited custom headers.
/// `out` must be NULL or point to complete, aligned, writable record storage.
#[no_mangle]
pub extern "C" fn cosmos_management_options_init(out: *mut CosmosManagementOptions) {
    // SAFETY: caller provides full writable storage for this zero-valid record.
    unsafe { initialize(out) };
}

/// Optional integer. `is_set` is 0 or 1; an unset value must be zero.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CosmosOptionalU64 {
    pub is_set: u8,
    pub value: u64,
}

impl CosmosOptionalU64 {
    fn decode<T: TryFrom<u64>>(self) -> Result<Option<T>, CosmosStatusCode> {
        match (self.is_set, self.value) {
            (0, 0) => Ok(None),
            (1, value) => T::try_from(value)
                .map(Some)
                .map_err(|_| INVALID.as_status_code()),
            _ => Err(INVALID.as_status_code()),
        }
    }
}

/// Optional exact Rust duration: seconds plus nanoseconds less than 1,000,000,000.
///
/// `is_set` is 0 or 1. Unset payloads must be zero. The full Rust Duration domain
/// is represented, but durations that overflow the platform monotonic clock are
/// rejected. Each consuming driver option also applies its own bounds.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CosmosDuration {
    pub is_set: u8,
    pub seconds: u64,
    pub nanoseconds: u32,
}

impl CosmosDuration {
    pub(crate) fn decode(self) -> Result<Option<Duration>, CosmosStatusCode> {
        match (self.is_set, self.seconds, self.nanoseconds) {
            (0, 0, 0) => Ok(None),
            (1, seconds, nanos) if nanos < 1_000_000_000 => {
                let duration = Duration::new(seconds, nanos);
                if Instant::now().checked_add(duration).is_none() {
                    return Err(INVALID.as_status_code());
                }
                Ok(Some(duration))
            }
            _ => Err(INVALID.as_status_code()),
        }
    }
}

/// Exact-duration overrides for operation defaults at runtime or client construction.
///
/// NULL `legacy` preserves the enclosing runtime or client base. All nested inputs are copied.
#[repr(C)]
pub struct CosmosOperationOptionsV2 {
    pub struct_size_bytes: u32,
    pub abi_version: u32,
    pub legacy: *const CosmosOperationOptions,
    pub endpoint_unavailability_ttl: CosmosDuration,
    pub max_throttle_retry_wait_time: CosmosDuration,
    /// A positive threshold enables hedging; zero is rejected.
    pub hedge_threshold: CosmosDuration,
}

pub(crate) unsafe fn operation_options(
    input: *const CosmosOperationOptionsV2,
    inherited: OperationOptions,
) -> Result<OperationOptions, CosmosStatusCode> {
    // SAFETY: caller supplies readable versioned record and nested inputs.
    let input = unsafe { record(input)? };
    let mut options = if input.legacy.is_null() {
        inherited
    } else {
        // SAFETY: legacy and nested borrowed arrays remain live for this call.
        unsafe { (*input.legacy).to_driver() }.map_err(|e| e.as_status_code())?
    };
    if let Some(duration) = input.endpoint_unavailability_ttl.decode()? {
        options.endpoint_unavailability_ttl = Some(duration);
    }
    if let Some(duration) = input.max_throttle_retry_wait_time.decode()? {
        options
            .throttling_retry_options
            .get_or_insert_default()
            .max_retry_wait_time = Some(duration);
    }
    if let Some(duration) = input.hedge_threshold.decode()? {
        let threshold = HedgeThreshold::new(duration).ok_or(INVALID.as_status_code())?;
        options.availability_strategy = Some(AvailabilityStrategy::Hedging(HedgingStrategy::new(
            threshold,
        )));
    }
    Ok(options)
}

/// Typed connection-pool overrides. Unset fields preserve native environment/default resolution.
///
/// Boolean fields use 0 unset, 1 false, 2 true. Duration and numeric bounds are
/// validated by the native pool builder; no host policy engine is required.
#[repr(C)]
pub struct CosmosConnectionPoolOptions {
    pub struct_size_bytes: u32,
    pub abi_version: u32,
    pub proxy_allowed: i8,
    pub is_http2_allowed: i8,
    pub gateway_v2_disabled: i8,
    /// 0 inherits, 1 requires validation, 2 requires validation except for the emulator.
    pub server_certificate_validation: u32,
    /// 0 inherits; 1 selects Rustls. Other values are rejected.
    pub tls_backend: u32,
    /// Optional IP address, copied before construction returns.
    pub local_address: CosmosStringView,
    pub min_connect_timeout: CosmosDuration,
    pub max_connect_timeout: CosmosDuration,
    pub min_dataplane_request_timeout: CosmosDuration,
    pub max_dataplane_request_timeout: CosmosDuration,
    pub min_metadata_request_timeout: CosmosDuration,
    pub max_metadata_request_timeout: CosmosDuration,
    pub idle_connection_timeout: CosmosDuration,
    pub idle_http2_client_timeout: CosmosDuration,
    pub http2_health_check_interval: CosmosDuration,
    pub http2_eviction_grace_period: CosmosDuration,
    pub http2_keep_alive_interval: CosmosDuration,
    pub http2_keep_alive_timeout: CosmosDuration,
    pub tcp_keepalive_time: CosmosDuration,
    pub tcp_keepalive_interval: CosmosDuration,
    pub max_idle_connections_per_endpoint: CosmosOptionalU64,
    pub max_http2_streams_per_client: CosmosOptionalU64,
    pub http2_fan_out_threshold_percent: CosmosOptionalU64,
    pub max_http2_connections_per_endpoint: CosmosOptionalU64,
    pub min_http2_connections_per_endpoint: CosmosOptionalU64,
    pub http2_consecutive_failure_threshold: CosmosOptionalU64,
    pub tcp_keepalive_retries: CosmosOptionalU64,
}

impl CosmosConnectionPoolOptions {
    unsafe fn build(&self) -> Result<ConnectionPoolOptions, CosmosStatusCode> {
        let mut builder = ConnectionPoolOptions::builder();
        macro_rules! apply {
            ($decode:ident; $($field:ident => $setter:ident),+ $(,)?) => {$(
                if let Some(value) = self.$field.$decode()? {
                    builder = builder.$setter(value);
                }
            )+};
        }
        apply!(decode;
            min_connect_timeout => with_min_connect_timeout,
            max_connect_timeout => with_max_connect_timeout,
            min_dataplane_request_timeout => with_min_dataplane_request_timeout,
            max_dataplane_request_timeout => with_max_dataplane_request_timeout,
            min_metadata_request_timeout => with_min_metadata_request_timeout,
            max_metadata_request_timeout => with_max_metadata_request_timeout,
            idle_connection_timeout => with_idle_connection_timeout,
            idle_http2_client_timeout => with_idle_http2_client_timeout,
            http2_health_check_interval => with_http2_health_check_interval,
            http2_eviction_grace_period => with_http2_eviction_grace_period,
            http2_keep_alive_interval => with_http2_keep_alive_interval,
            http2_keep_alive_timeout => with_http2_keep_alive_timeout,
            tcp_keepalive_time => with_tcp_keepalive_time,
            tcp_keepalive_interval => with_tcp_keepalive_interval,
            max_idle_connections_per_endpoint => with_max_idle_connections_per_endpoint,
            max_http2_streams_per_client => with_max_http2_streams_per_client,
            http2_fan_out_threshold_percent => with_http2_fan_out_threshold_percent,
            max_http2_connections_per_endpoint => with_max_http2_connections_per_endpoint,
            min_http2_connections_per_endpoint => with_min_http2_connections_per_endpoint,
            http2_consecutive_failure_threshold => with_http2_consecutive_failure_threshold,
            tcp_keepalive_retries => with_tcp_keepalive_retries,
        );
        if let Some(value) =
            decode_tristate_bool(self.proxy_allowed).map_err(|e| e.as_status_code())?
        {
            builder = builder.with_proxy_allowed(value);
        }
        if let Some(value) =
            decode_tristate_bool(self.is_http2_allowed).map_err(|e| e.as_status_code())?
        {
            builder = builder.with_is_http2_allowed(value);
        }
        if let Some(value) =
            decode_tristate_bool(self.gateway_v2_disabled).map_err(|e| e.as_status_code())?
        {
            builder = builder.with_gateway_v2_disabled(value);
        }
        builder = match self.server_certificate_validation {
            0 => builder,
            1 => builder.with_server_certificate_validation(ServerCertificateValidation::Required),
            2 => builder.with_server_certificate_validation(
                ServerCertificateValidation::RequiredUnlessEmulator,
            ),
            _ => return Err(INVALID.as_status_code()),
        };
        builder = match self.tls_backend {
            0 => builder,
            1 => builder.with_tls_backend(TlsBackend::Rustls),
            _ => return Err(INVALID.as_status_code()),
        };
        // SAFETY: input view remains live throughout construction.
        if let Some(address) =
            unsafe { optional_text(self.local_address, INVALID) }.map_err(|e| e.as_status_code())?
        {
            builder =
                builder.with_local_address(address.parse().map_err(|_| INVALID.as_status_code())?);
        }
        builder
            .build()
            .map_err(|e| CosmosStatusCode::from_driver_error(&e))
    }
}

/// Runtime diagnostics defaults and retention limits, applied at construction only.
#[repr(C)]
pub struct CosmosDiagnosticsOptions {
    pub struct_size_bytes: u32,
    pub abi_version: u32,
    /// 0 inherits; 1 selects summary; 2 selects detailed.
    pub default_verbosity: u32,
    pub max_summary_size_bytes: CosmosOptionalU64,
    pub max_request_diagnostics: CosmosOptionalU64,
}

/// Additive runtime construction. NULL nested records inherit defaults.
#[repr(C)]
pub struct CosmosRuntimeOptionsV2 {
    pub struct_size_bytes: u32,
    pub abi_version: u32,
    pub legacy: *const CosmosRuntimeOptions,
    pub connection_pool: *const CosmosConnectionPoolOptions,
    pub diagnostics: *const CosmosDiagnosticsOptions,
    pub operation_options: *const CosmosOperationOptionsV2,
}

/// Client-scoped topology and partition-failover overrides.
#[repr(C)]
pub struct CosmosPartitionFailoverOptions {
    pub struct_size_bytes: u32,
    pub abi_version: u32,
    /// 0 inherits; 1 eager; 2 lazy.
    pub topology_cache_mode: u32,
    /// 0 inherits, 1 disables, 2 enables; native environment kill switches retain precedence.
    pub circuit_breaker_enabled: i8,
    pub read_failure_threshold: CosmosOptionalU64,
    pub write_failure_threshold: CosmosOptionalU64,
    pub consecutive_hedge_win_threshold: CosmosOptionalU64,
    pub counter_reset_window: CosmosDuration,
    pub partition_unavailability_duration: CosmosDuration,
    pub failback_sweep_interval: CosmosDuration,
}

/// Additive client construction, copying all nested records, strings, and ordered endpoints.
#[repr(C)]
pub struct CosmosDriverOptionsConfigV3 {
    pub struct_size_bytes: u32,
    pub abi_version: u32,
    /// Required v2 base, preserving fault-injection and existing option behavior.
    pub legacy: *const CosmosDriverOptionsConfigV2,
    /// NULL/0 inherits the account's backup endpoints; non-NULL/0 explicitly clears them.
    pub backup_endpoints: *const CosmosStringView,
    pub backup_endpoints_len: usize,
    /// NULL/0 inherits runtime identity; explicit empty is validated by UserAgentSuffix.
    pub user_agent_suffix: CosmosStringView,
    pub partition_failover: *const CosmosPartitionFailoverOptions,
    pub operation_options: *const CosmosOperationOptionsV2,
}

unsafe fn record<'a, T>(input: *const T) -> Result<&'a T, CosmosStatusCode> {
    if input.is_null() || !input.is_aligned() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code());
    }
    // SAFETY: all versioned records begin with two readable u32 fields.
    let size = unsafe { input.cast::<u32>().read() } as usize;
    if size < size_of::<T>() || size > isize::MAX as usize {
        return Err(INVALID.as_status_code());
    }
    // SAFETY: declared size covers the prefix, known record, and extension.
    unsafe {
        if input.cast::<u32>().add(1).read() != 1
            || std::slice::from_raw_parts(
                input.cast::<u8>().add(size_of::<T>()),
                size - size_of::<T>(),
            )
            .iter()
            .any(|byte| *byte != 0)
        {
            return Err(INVALID.as_status_code());
        }
        Ok(&*input)
    }
}

unsafe fn initialize<T>(out: *mut T) {
    if !out.is_null() {
        // SAFETY: internal callers use zero-valid records with a two-u32 prefix.
        unsafe {
            out.write(std::mem::zeroed());
            out.cast::<u32>().write(size_of::<T>() as u32);
            out.cast::<u32>().add(1).write(1);
        }
    }
}

/// Initializes runtime construction options to inherited defaults.
#[no_mangle]
pub extern "C" fn cosmos_runtime_options_v2_init(out: *mut CosmosRuntimeOptionsV2) {
    // SAFETY: caller provides full writable storage for this zero-valid record.
    unsafe { initialize(out) };
}

/// Initializes transport configuration to inherited defaults.
#[no_mangle]
pub extern "C" fn cosmos_connection_pool_options_init(out: *mut CosmosConnectionPoolOptions) {
    // SAFETY: caller provides full writable storage for this zero-valid record.
    unsafe { initialize(out) };
}

/// Initializes diagnostics configuration to inherited defaults.
#[no_mangle]
pub extern "C" fn cosmos_diagnostics_options_init(out: *mut CosmosDiagnosticsOptions) {
    // SAFETY: caller provides full writable storage for this zero-valid record.
    unsafe { initialize(out) };
}

/// Initializes partition-failover configuration to inherited defaults.
#[no_mangle]
pub extern "C" fn cosmos_partition_failover_options_init(out: *mut CosmosPartitionFailoverOptions) {
    // SAFETY: caller provides full writable storage for this zero-valid record.
    unsafe { initialize(out) };
}

/// Initializes client construction options to inherited defaults.
#[no_mangle]
pub extern "C" fn cosmos_driver_options_config_v3_init(out: *mut CosmosDriverOptionsConfigV3) {
    // SAFETY: caller provides full writable storage for this zero-valid record.
    unsafe { initialize(out) };
}

/// Initializes exact-duration operation defaults to inherited settings.
#[no_mangle]
pub extern "C" fn cosmos_operation_options_v2_init(out: *mut CosmosOperationOptionsV2) {
    // SAFETY: caller provides full writable storage for this zero-valid record.
    unsafe { initialize(out) };
}

unsafe fn runtime_builder(
    input: *const CosmosRuntimeOptionsV2,
) -> Result<CosmosDriverRuntimeBuilder, CosmosStatusCode> {
    // SAFETY: input and all nested records are readable until return.
    let input = unsafe { record(input)? };
    let mut builder = CosmosDriverRuntimeBuilder::new();
    if !input.legacy.is_null() {
        // SAFETY: legacy runtime options and their inputs remain live.
        builder = unsafe { (*input.legacy).apply_to(builder) }.map_err(|e| e.as_status_code())?;
    }
    if !input.connection_pool.is_null() {
        // SAFETY: caller guarantees the nested record's allocation.
        builder = builder.with_connection_pool(unsafe { record(input.connection_pool)?.build()? });
    }
    if !input.diagnostics.is_null() {
        // SAFETY: caller guarantees the nested record's allocation.
        let diagnostics = unsafe { record(input.diagnostics)? };
        let mut options = DiagnosticsOptions::builder();
        options = match diagnostics.default_verbosity {
            0 => options,
            1 => options.with_default_verbosity(DiagnosticsVerbosity::Summary),
            2 => options.with_default_verbosity(DiagnosticsVerbosity::Detailed),
            _ => return Err(INVALID.as_status_code()),
        };
        if let Some(value) = diagnostics.max_summary_size_bytes.decode()? {
            options = options.with_max_summary_size_bytes(value);
        }
        if let Some(value) = diagnostics.max_request_diagnostics.decode()? {
            options = options.with_max_request_diagnostics(value);
        }
        builder = builder.with_diagnostics_options(
            options
                .build()
                .map_err(|e| CosmosStatusCode::from_driver_error(&e))?,
        );
    }
    if !input.operation_options.is_null() {
        let mut inherited = OperationOptions::default();
        if !input.legacy.is_null() {
            // SAFETY: the legacy base and nested options remain readable.
            let legacy = unsafe { &*input.legacy };
            if !legacy.operation_options.is_null() {
                // SAFETY: the legacy operation record is readable until return.
                inherited = unsafe { (*legacy.operation_options).to_driver() }
                    .map_err(|e| e.as_status_code())?;
            }
        }
        // SAFETY: caller guarantees nested operation inputs.
        builder = builder.with_default_operation_options(unsafe {
            operation_options(input.operation_options, inherited)?
        });
    }
    Ok(builder)
}

/// Builds a runtime with typed transport, diagnostics, and precision-preserving defaults.
///
/// No runtime or sibling client is mutated. All nested inputs are copied before return.
#[no_mangle]
pub extern "C" fn cosmos_runtime_build_v2(
    input: *const CosmosRuntimeOptionsV2,
    out: *mut *mut RuntimeContext,
    error: *mut *mut CosmosError,
) -> CosmosStatusCode {
    if out.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    // SAFETY: caller supplies writable output slots.
    unsafe {
        *out = std::ptr::null_mut();
        if !error.is_null() {
            *error = std::ptr::null_mut();
        }
    }
    // SAFETY: input and all nested records are readable until return.
    match unsafe { runtime_builder(input) } {
        Ok(builder) => finish_runtime_build(RuntimeContext::new_with_builder(builder), out, error),
        Err(status) => status,
    }
}

/// Builds independent client options with ordered backups, identity, and topology overrides.
#[no_mangle]
pub extern "C" fn cosmos_driver_options_build_v3(
    account: *const AccountRefHandle,
    input: *const CosmosDriverOptionsConfigV3,
    out: *mut *mut DriverOptionsHandle,
) -> CosmosStatusCode {
    if out.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    // SAFETY: caller supplies a writable output slot.
    unsafe { *out = std::ptr::null_mut() };
    let build = || {
        // SAFETY: caller supplies readable versioned and nested records.
        let input = unsafe { record(input)? };
        let mut base = std::ptr::null_mut();
        let status = cosmos_driver_options_build_v2(account, input.legacy, &mut base);
        if status != COSMOS_STATUS_SUCCESS {
            return Err(status);
        }
        let base_owned = DriverOptionsHandle::inner_arc(base)
            .ok_or(CosmosErrorCode::CosmosErrorCodeInternalError.as_status_code())?;
        cosmos_driver_options_free(base);
        let base = &base_owned.inner;
        let mut account = base.account().clone();
        validate_array(input.backup_endpoints, input.backup_endpoints_len)
            .map_err(|e| e.as_status_code())?;
        if !input.backup_endpoints.is_null() {
            let mut endpoints = Vec::with_capacity(input.backup_endpoints_len);
            for index in 0..input.backup_endpoints_len {
                // SAFETY: validated array metadata; caller guarantees readable string views.
                let text = unsafe { required_text(*input.backup_endpoints.add(index), INVALID) }
                    .map_err(|e| e.as_status_code())?;
                let endpoint = Url::parse(&text).map_err(|_| INVALID.as_status_code())?;
                if !matches!(endpoint.scheme(), "http" | "https") || endpoint.host_str().is_none() {
                    return Err(INVALID.as_status_code());
                }
                endpoints.push(endpoint);
            }
            account = account.with_backup_endpoints(endpoints);
        }
        let mut builder = DriverOptions::builder(account)
            .with_operation_options((**base.operation_options()).clone())
            .with_preferred_regions(base.preferred_regions().to_vec())
            .with_partition_failover_options(base.partition_failover_options().clone())
            .with_hedging_options(base.hedging_options().clone())
            .with_resource_name_validation(base.resource_name_validation_enabled());
        if let Some(rules) = base.fault_injection_rules() {
            builder = builder
                .with_fault_injection_rules(rules.to_vec())
                .map_err(|e| CosmosStatusCode::from_driver_error(&e))?;
        }
        // SAFETY: counted identity view remains live until return.
        if let Some(suffix) = unsafe { optional_text(input.user_agent_suffix, INVALID) }
            .map_err(|e| e.as_status_code())?
        {
            builder = builder.with_user_agent_suffix(
                UserAgentSuffix::try_new(suffix).ok_or(INVALID.as_status_code())?,
            );
        }
        if !input.operation_options.is_null() {
            // SAFETY: nested inputs remain live until return.
            builder = builder.with_operation_options(unsafe {
                operation_options(
                    input.operation_options,
                    (**base.operation_options()).clone(),
                )?
            });
        }
        if !input.partition_failover.is_null() {
            // SAFETY: nested record and prefix remain readable until return.
            let config = unsafe { record(input.partition_failover)? };
            let mut options = PartitionFailoverOptions::builder();
            options = match config.topology_cache_mode {
                0 => options,
                1 => options.with_partition_topology_cache_mode(PartitionTopologyCacheMode::Eager),
                2 => options.with_partition_topology_cache_mode(PartitionTopologyCacheMode::Lazy),
                _ => return Err(INVALID.as_status_code()),
            };
            if let Some(value) = decode_tristate_bool(config.circuit_breaker_enabled)
                .map_err(|e| e.as_status_code())?
            {
                options = options.with_circuit_breaker_enabled(value);
            }
            macro_rules! apply {
                ($($field:ident => $setter:ident),+ $(,)?) => {$(
                    if let Some(value) = config.$field.decode()? { options = options.$setter(value); }
                )+};
            }
            apply!(
                read_failure_threshold => with_read_failure_threshold,
                write_failure_threshold => with_write_failure_threshold,
                consecutive_hedge_win_threshold => with_consecutive_hedge_win_threshold,
                counter_reset_window => with_counter_reset_window,
                partition_unavailability_duration => with_partition_unavailability_duration,
                failback_sweep_interval => with_failback_sweep_interval,
            );
            builder = builder.with_partition_failover_options(
                options
                    .build()
                    .map_err(|e| CosmosStatusCode::from_driver_error(&e))?,
            );
        }
        Ok(builder.build())
    };
    match build() {
        Ok(options) => {
            // SAFETY: caller supplies a writable output slot.
            unsafe { *out = DriverOptionsHandle::into_raw(options) };
            COSMOS_STATUS_SUCCESS
        }
        Err(status) => status,
    }
}
