// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Additive construction records. Legacy by-value records remain unchanged.

#[cfg(test)]
mod tests;

use crate::{
    account_ref::AccountRefHandle,
    driver_options::{
        cosmos_driver_options_build_v2, cosmos_driver_options_config_v2_default,
        cosmos_driver_options_free, DriverOptionsHandle,
    },
    error::{CosmosError, CosmosErrorCode, CosmosStatusCode, COSMOS_STATUS_SUCCESS},
    fault_injection::CosmosFaultInjectionRule,
    op_request::{decode_tristate_bool, CosmosOperationOptions},
    runtime::RuntimeContext,
    runtime_builder::{finish_runtime_build, CosmosRuntimeOptions},
    string::{optional_text, required_text, validate_array, CosmosStringView},
};
use azure_data_cosmos_driver::{
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

fn operation_options(
    mut options: OperationOptions,
    endpoint_unavailability_ttl: CosmosDuration,
    max_throttle_retry_wait_time: CosmosDuration,
    hedge_threshold: CosmosDuration,
) -> Result<OperationOptions, CosmosStatusCode> {
    if let Some(duration) = endpoint_unavailability_ttl.decode()? {
        options.endpoint_unavailability_ttl = Some(duration);
    }
    if let Some(duration) = max_throttle_retry_wait_time.decode()? {
        options
            .throttling_retry_options
            .get_or_insert_default()
            .max_retry_wait_time = Some(duration);
    }
    if let Some(duration) = hedge_threshold.decode()? {
        let threshold = HedgeThreshold::new(duration).ok_or(INVALID.as_status_code())?;
        options.availability_strategy = Some(AvailabilityStrategy::Hedging(HedgingStrategy::new(
            threshold,
        )));
    }
    Ok(options)
}

/// Self-contained runtime construction options. Initialize with [`cosmos_runtime_options_v2_init()`].
///
/// Boolean fields use 0 unset, 1 false, 2 true. Duration and numeric bounds are
/// validated by the native builders. All borrowed strings, arrays, and operation
/// options are copied during construction. Unset fields retain native defaults.
#[repr(C)]
pub struct CosmosRuntimeOptionsV2 {
    pub struct_size_bytes: u32,
    pub abi_version: u32,
    /// 0 inherits; otherwise a workload identifier from 1 through 50.
    pub workload_id: u8,
    pub correlation_id: CosmosStringView,
    pub user_agent_suffix: CosmosStringView,
    pub wrapping_sdk_identifier: CosmosStringView,
    /// 0 inherits; otherwise 1000 through 60000 milliseconds.
    pub cpu_refresh_interval_ms: u64,
    /// NULL inherits. Borrowed options and their inputs are copied during construction.
    pub operation_options: *const CosmosOperationOptions,
    pub endpoint_unavailability_ttl: CosmosDuration,
    pub max_throttle_retry_wait_time: CosmosDuration,
    /// A positive threshold enables hedging; explicit zero is rejected.
    pub hedge_threshold: CosmosDuration,
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
    /// 0 inherits; 1 selects summary; 2 selects detailed.
    pub diagnostics_verbosity: u32,
    pub max_summary_size_bytes: CosmosOptionalU64,
    pub max_request_diagnostics: CosmosOptionalU64,
}

impl CosmosRuntimeOptionsV2 {
    unsafe fn connection_pool(&self) -> Result<Option<ConnectionPoolOptions>, CosmosStatusCode> {
        let mut builder = ConnectionPoolOptions::builder();
        let mut configured = false;
        macro_rules! apply {
            ($decode:ident; $($field:ident => $setter:ident),+ $(,)?) => {$(
                if let Some(value) = self.$field.$decode()? {
                    configured = true;
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
            configured = true;
            builder = builder.with_proxy_allowed(value);
        }
        if let Some(value) =
            decode_tristate_bool(self.is_http2_allowed).map_err(|e| e.as_status_code())?
        {
            configured = true;
            builder = builder.with_is_http2_allowed(value);
        }
        if let Some(value) =
            decode_tristate_bool(self.gateway_v2_disabled).map_err(|e| e.as_status_code())?
        {
            configured = true;
            builder = builder.with_gateway_v2_disabled(value);
        }
        configured |= self.server_certificate_validation != 0 || self.tls_backend != 0;
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
            configured = true;
            builder =
                builder.with_local_address(address.parse().map_err(|_| INVALID.as_status_code())?);
        }
        if configured {
            builder
                .build()
                .map(Some)
                .map_err(|e| CosmosStatusCode::from_driver_error(&e))
        } else {
            Ok(None)
        }
    }
}

/// Self-contained client configuration. Initialize with [`cosmos_driver_options_config_v3_init()`].
///
/// Construction copies all strings, arrays, rules, and operation options. Unset
/// fields retain native defaults; no runtime or sibling client is mutated.
#[repr(C)]
pub struct CosmosDriverOptionsConfigV3 {
    pub struct_size_bytes: u32,
    pub abi_version: u32,
    pub preferred_regions: *const CosmosStringView,
    pub preferred_regions_len: usize,
    /// NULL inherits. Borrowed options and their inputs are copied during construction.
    pub operation_options: *const CosmosOperationOptions,
    pub fault_injection_rules: *const CosmosFaultInjectionRule,
    pub fault_injection_rules_len: usize,
    pub fault_injection_rule_stride: usize,
    /// 0 inherits, 1 disables validation, 2 enables validation.
    pub resource_name_validation_enabled: i8,
    /// NULL/0 inherits the account's backup endpoints; non-NULL/0 explicitly clears them.
    pub backup_endpoints: *const CosmosStringView,
    pub backup_endpoints_len: usize,
    /// NULL/0 inherits runtime identity; explicit empty is validated by UserAgentSuffix.
    pub user_agent_suffix: CosmosStringView,
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
    pub endpoint_unavailability_ttl: CosmosDuration,
    pub max_throttle_retry_wait_time: CosmosDuration,
    /// A positive threshold enables hedging; explicit zero is rejected.
    pub hedge_threshold: CosmosDuration,
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

/// Initializes client construction options to inherited defaults.
#[no_mangle]
pub extern "C" fn cosmos_driver_options_config_v3_init(out: *mut CosmosDriverOptionsConfigV3) {
    // SAFETY: caller provides full writable storage for this zero-valid record.
    unsafe {
        initialize(out);
        if let Some(out) = out.as_mut() {
            out.fault_injection_rule_stride = size_of::<CosmosFaultInjectionRule>();
        }
    }
}

unsafe fn runtime_builder(
    input: *const CosmosRuntimeOptionsV2,
) -> Result<CosmosDriverRuntimeBuilder, CosmosStatusCode> {
    // SAFETY: input and its borrowed payloads are readable until return.
    let input = unsafe { record(input)? };
    let legacy = CosmosRuntimeOptions {
        workload_id: input.workload_id,
        correlation_id: input.correlation_id,
        user_agent_suffix: input.user_agent_suffix,
        wrapping_sdk_identifier: input.wrapping_sdk_identifier,
        cpu_refresh_interval_ms: input.cpu_refresh_interval_ms,
        operation_options: std::ptr::null(),
    };
    // SAFETY: the input string views remain readable throughout construction.
    let mut builder = unsafe { legacy.apply_to(CosmosDriverRuntimeBuilder::new()) }
        .map_err(|e| e.as_status_code())?;
    let mut diagnostics = DiagnosticsOptions::builder();
    let mut configured = input.diagnostics_verbosity != 0;
    diagnostics = match input.diagnostics_verbosity {
        0 => diagnostics,
        1 => diagnostics.with_default_verbosity(DiagnosticsVerbosity::Summary),
        2 => diagnostics.with_default_verbosity(DiagnosticsVerbosity::Detailed),
        _ => return Err(INVALID.as_status_code()),
    };
    if let Some(value) = input.max_summary_size_bytes.decode()? {
        configured = true;
        diagnostics = diagnostics.with_max_summary_size_bytes(value);
    }
    if let Some(value) = input.max_request_diagnostics.decode()? {
        configured = true;
        diagnostics = diagnostics.with_max_request_diagnostics(value);
    }
    if configured {
        builder = builder.with_diagnostics_options(
            diagnostics
                .build()
                .map_err(|e| CosmosStatusCode::from_driver_error(&e))?,
        );
    }
    // SAFETY: the input's optional local-address view remains readable.
    if let Some(pool) = unsafe { input.connection_pool()? } {
        builder = builder.with_connection_pool(pool);
    }
    let operation = if input.operation_options.is_null() {
        OperationOptions::default()
    } else {
        // SAFETY: the operation record and its borrowed payloads remain readable.
        unsafe { (*input.operation_options).to_driver() }.map_err(|e| e.as_status_code())?
    };
    Ok(builder.with_default_operation_options(operation_options(
        operation,
        input.endpoint_unavailability_ttl,
        input.max_throttle_retry_wait_time,
        input.hedge_threshold,
    )?))
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
        // SAFETY: caller supplies a readable versioned record and borrowed payloads.
        let input = unsafe { record(input)? };
        let legacy = crate::driver_options::CosmosDriverOptionsConfigV2 {
            preferred_regions: input.preferred_regions,
            preferred_regions_len: input.preferred_regions_len,
            operation_options: input.operation_options,
            fault_injection_rules: input.fault_injection_rules,
            fault_injection_rules_len: input.fault_injection_rules_len,
            fault_injection_rule_stride: input.fault_injection_rule_stride,
            resource_name_validation_enabled: input.resource_name_validation_enabled,
            ..cosmos_driver_options_config_v2_default()
        };
        let mut base = std::ptr::null_mut();
        let status = cosmos_driver_options_build_v2(account, &legacy, &mut base);
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
        builder = builder.with_operation_options(operation_options(
            (**base.operation_options()).clone(),
            input.endpoint_unavailability_ttl,
            input.max_throttle_retry_wait_time,
            input.hedge_threshold,
        )?);
        {
            let config = input;
            let mut options = PartitionFailoverOptions::builder();
            let mut configured =
                config.topology_cache_mode != 0 || config.circuit_breaker_enabled != 0;
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
                    if let Some(value) = config.$field.decode()? {
                        configured = true;
                        options = options.$setter(value);
                    }
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
            if configured {
                builder = builder.with_partition_failover_options(
                    options
                        .build()
                        .map_err(|e| CosmosStatusCode::from_driver_error(&e))?,
                );
            }
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
