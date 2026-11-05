// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Admission-time configuration and deadline snapshots for native hosts.

#[cfg(test)]
mod submit_tests;

use std::{
    future::Future,
    sync::Arc,
    time::{Duration, Instant},
};

use azure_core::http::StatusCode;
use azure_data_cosmos_driver::{
    driver::{CosmosDriver, CosmosDriverRuntime},
    error::{status_codes::substatus::CLIENT_OPERATION_TIMEOUT, CosmosError as DriverError},
    models::CosmosStatus,
    options::OperationOptions,
};

use crate::{
    error::{CosmosErrorCode, CosmosStatusCode},
    op_request::CosmosOperationOptions,
    runtime::RuntimeContext,
};

/// Opaque admission snapshot. Owned by the host until freed; submits clone it.
#[derive(Clone)]
pub struct OperationOptionsSnapshot {
    pub(crate) options: OperationOptions,
    runtime: Arc<CosmosDriverRuntime>,
    started: Instant,
    timeout: Option<Duration>,
}

impl OperationOptionsSnapshot {
    pub(crate) fn matches_driver(&self, driver: &CosmosDriver) -> bool {
        std::ptr::eq(self.runtime.as_ref(), driver.runtime())
    }

    pub(crate) async fn execute<T>(
        snapshot: Option<Self>,
        work: impl Future<Output = Result<T, DriverError>>,
    ) -> Result<T, DriverError> {
        let Some(snapshot) = snapshot else {
            return work.await;
        };
        let Some(timeout) = snapshot.timeout else {
            return work.await;
        };
        let remaining = timeout.saturating_sub(snapshot.started.elapsed());
        if !remaining.is_zero() {
            if let Ok(result) = tokio::time::timeout(remaining, work).await {
                return result;
            }
        }
        Err(DriverError::builder()
            .with_status(
                CosmosStatus::new(StatusCode::RequestTimeout)
                    .with_sub_status(CLIENT_OPERATION_TIMEOUT.value()),
            )
            .with_message("end-to-end operation timeout exceeded after native admission")
            .build())
    }
}

/// Atomically replaces runtime operation defaults. NULL resets all defaults.
///
/// All pointers must remain valid for this call. Input arrays and strings are
/// copied; invalid options leave the previous defaults unchanged.
///
/// # Safety
///
/// The runtime and any non-NULL options and borrowed arrays must be live.
#[no_mangle]
pub unsafe extern "C" fn cosmos_runtime_set_operation_options(
    runtime: *const RuntimeContext,
    options: *const CosmosOperationOptions,
) -> CosmosStatusCode {
    let Some(runtime) = RuntimeContext::from_ptr(runtime) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    };
    // SAFETY: the host keeps options and their borrowed inputs alive for this call.
    let options = match unsafe { decode_options(options) } {
        Ok(options) => options,
        Err(error) => return error.as_status_code(),
    };
    runtime.driver.set_default_operation_options(options);
    CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
}

/// Captures all configuration layers and starts the logical-operation budget.
///
/// `client_options` and `request_options` may be NULL to inherit. Outputs are
/// required. On success `out_timeout_ms` is the resolved Rust timeout (including
/// its minimum clamp), or -1 when unset. A snapshot keeps its runtime generation
/// through initialization, retries and patch stages; it must be submitted to a
/// driver from this runtime. Invalid inputs do not modify outputs.
///
/// # Safety
///
/// Handles, option structs and their arrays must be valid for this call; output
/// pointers must be writable. The host owns the returned snapshot.
#[no_mangle]
pub unsafe extern "C" fn cosmos_operation_options_snapshot_create(
    runtime: *const RuntimeContext,
    client_options: *const CosmosOperationOptions,
    request_options: *const CosmosOperationOptions,
    out_snapshot: *mut *mut OperationOptionsSnapshot,
    out_timeout_ms: *mut i64,
) -> CosmosStatusCode {
    let started = Instant::now();
    let Some(runtime) = RuntimeContext::from_ptr(runtime) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    };
    if out_snapshot.is_null() || out_timeout_ms.is_null() {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    // SAFETY: borrowed option inputs follow the documented host allocation contract.
    let decoded = unsafe {
        decode_options(client_options)
            .and_then(|client| decode_options(request_options).map(|request| (client, request)))
    };
    let (client, request) = match decoded {
        Ok(value) => value,
        Err(error) => return error.as_status_code(),
    };
    let options = request.with_resolution_snapshot(
        Arc::clone(runtime.driver.env_override_operation_options()),
        Arc::clone(runtime.driver.env_operation_options()),
        runtime.driver.default_operation_options(),
        Arc::new(client),
    );
    let timeout = options.resolution_snapshot_view().and_then(|view| {
        view.end_to_end_latency_policy()
            .map(|policy| policy.timeout())
    });
    let timeout_ms = match timeout {
        Some(timeout) => match i64::try_from(timeout.as_millis()) {
            Ok(timeout) => timeout,
            Err(_) => return CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_status_code(),
        },
        None => -1,
    };
    let snapshot = Box::new(OperationOptionsSnapshot {
        options,
        runtime: Arc::clone(&runtime.driver),
        started,
        timeout,
    });
    // SAFETY: non-NULL outputs are writable according to the host contract.
    unsafe {
        *out_snapshot = Box::into_raw(snapshot);
        *out_timeout_ms = timeout_ms;
    }
    CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
}

/// Frees a snapshot. NULL is a no-op; already-submitted operations retain a copy.
///
/// # Safety
///
/// The pointer must be NULL or a live snapshot returned by snapshot creation,
/// not concurrently borrowed by another call.
#[no_mangle]
pub unsafe extern "C" fn cosmos_operation_options_snapshot_free(
    snapshot: *mut OperationOptionsSnapshot,
) {
    if !snapshot.is_null() {
        // SAFETY: this call consumes the host's unique Box allocation.
        drop(unsafe { Box::from_raw(snapshot) });
    }
}

unsafe fn decode_options(
    options: *const CosmosOperationOptions,
) -> Result<OperationOptions, CosmosErrorCode> {
    // SAFETY: the caller guarantees validity of any non-NULL options and their inputs.
    match unsafe { options.as_ref() } {
        Some(options) => unsafe { options.to_driver() },
        None => Ok(OperationOptions::default()),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        cosmos_operation_options_snapshot_create, cosmos_operation_options_snapshot_free,
        cosmos_runtime_set_operation_options, OperationOptionsSnapshot,
    };
    use crate::{
        error::CosmosErrorCode,
        op_request::{cosmos_operation_options_default, CosmosOperationOptions},
        runtime::{__test_only_create_default_runtime, cosmos_runtime_free, RuntimeContext},
    };
    use std::{ptr, time::Duration};

    fn capture(
        runtime: *const RuntimeContext,
        client: &CosmosOperationOptions,
        request: &CosmosOperationOptions,
    ) -> (Box<OperationOptionsSnapshot>, i64) {
        let mut snapshot = ptr::null_mut();
        let mut timeout = -2;
        // SAFETY: all borrowed inputs/outputs live throughout capture; reclaim its unique allocation.
        unsafe {
            assert_eq!(
                cosmos_operation_options_snapshot_create(
                    runtime,
                    client,
                    request,
                    &mut snapshot,
                    &mut timeout
                ),
                CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
            );
            (Box::from_raw(snapshot), timeout)
        }
    }

    #[test]
    fn updates_preserve_admission_layers_and_nested_inheritance() {
        let runtime = __test_only_create_default_runtime();
        let mut defaults = cosmos_operation_options_default();
        defaults.throughput_bucket = i64::from(u32::MAX);
        defaults.priority_level = 1;
        defaults.max_throttle_retry_count = 9;
        defaults.max_throttle_retry_wait_time_ms = 1234;
        defaults.end_to_end_timeout_ms = 4000;
        defaults.endpoint_unavailability_ttl_ms = 555;
        defaults.binary_encoding_enabled = 1;
        // SAFETY: runtime and defaults are live.
        unsafe {
            assert_eq!(
                cosmos_runtime_set_operation_options(runtime, &defaults),
                CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
            );
        }
        let mut client = cosmos_operation_options_default();
        client.priority_level = 2;
        client.max_throttle_retry_count = 7;
        let mut request = cosmos_operation_options_default();
        request.max_throttle_retry_wait_time_ms = 0;
        request.binary_encoding_request_text_response = 2;
        let (old, timeout) = capture(runtime, &client, &request);
        assert_eq!(timeout, 4000);
        // SAFETY: replacing all defaults with an empty group is supported.
        unsafe {
            assert_eq!(
                cosmos_runtime_set_operation_options(runtime, ptr::null()),
                CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
            );
        }
        let (new, timeout) = capture(runtime, &client, &request);
        assert_eq!(timeout, -1);
        let old_view = old.options.resolution_snapshot_view().unwrap();
        let new_view = new.options.resolution_snapshot_view().unwrap();
        assert_eq!(
            old_view.throughput_control().throughput_bucket(),
            Some(&u32::MAX)
        );
        assert_eq!(new_view.throughput_control().throughput_bucket(), None);
        assert_eq!(
            old_view.throughput_control().priority_level(),
            new_view.throughput_control().priority_level()
        );
        assert_eq!(
            old_view.throttling_retry_options().max_retry_count(),
            Some(&7)
        );
        assert_eq!(
            old_view.throttling_retry_options().max_retry_wait_time(),
            Some(&Duration::ZERO)
        );
        assert_eq!(
            old_view.endpoint_unavailability_ttl(),
            Some(&Duration::from_millis(555))
        );
        assert_eq!(new_view.endpoint_unavailability_ttl(), None);
        // Binary flags replace the entire group: request's unspecified enabled uses its own default.
        assert!(old_view.binary_encoding().unwrap().enabled);
        assert!(old_view.binary_encoding().unwrap().request_text_response);
        cosmos_runtime_free(runtime);
    }

    #[test]
    fn invalid_update_is_atomic_and_sub_second_timeout_is_clamped() {
        let runtime = __test_only_create_default_runtime();
        let mut defaults = cosmos_operation_options_default();
        defaults.end_to_end_timeout_ms = 20;
        // SAFETY: runtime and input are live across both updates.
        unsafe {
            assert_eq!(
                cosmos_runtime_set_operation_options(runtime, &defaults),
                CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
            );
            defaults.end_to_end_timeout_ms = 9000;
            defaults.max_session_retry_count = i64::from(u32::MAX) + 1;
            assert_eq!(
                cosmos_runtime_set_operation_options(runtime, &defaults),
                CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_status_code()
            );
        }
        let (_, timeout) = capture(
            runtime,
            &cosmos_operation_options_default(),
            &cosmos_operation_options_default(),
        );
        assert_eq!(timeout, 1000);
        cosmos_runtime_free(runtime);
    }

    #[test]
    fn deadline_includes_time_before_submit_and_never_polls_expired_work() {
        let runtime = __test_only_create_default_runtime();
        let (mut snapshot, _) = capture(
            runtime,
            &cosmos_operation_options_default(),
            &cosmos_operation_options_default(),
        );
        snapshot.timeout = Some(Duration::from_secs(1));
        snapshot.started -= Duration::from_secs(2);
        let runtime_ref = RuntimeContext::from_ptr(runtime).unwrap();
        let error = runtime_ref
            .tokio
            .block_on(OperationOptionsSnapshot::execute(Some(*snapshot), async {
                panic!("expired work must not be polled");
                #[allow(unreachable_code)]
                Ok::<(), azure_data_cosmos_driver::error::CosmosError>(())
            }))
            .unwrap_err();
        assert!(error.status().is_timeout());
        cosmos_runtime_free(runtime);
    }

    #[test]
    fn snapshot_keeps_owned_inputs_after_host_free() {
        let runtime = __test_only_create_default_runtime();
        let (snapshot, _) = capture(
            runtime,
            &cosmos_operation_options_default(),
            &cosmos_operation_options_default(),
        );
        let submitted = (*snapshot).clone();
        // SAFETY: transfer the Box back to its FFI deallocator exactly once.
        unsafe {
            cosmos_operation_options_snapshot_free(Box::into_raw(snapshot));
        }
        cosmos_runtime_free(runtime);
        assert!(submitted.options.resolution_snapshot_view().is_some());
    }
}
