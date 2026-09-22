// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pull-based retained plans with delivery-reserved, format-isolated queues.

use crate::{
    completion::{
        CompletionQueue, CompletionQueueInner, CosmosCompletion, CosmosCompletionQueueState,
        CosmosOperationHandleState, CqOptions, OperationHandle, OperationInner, PendingCompletion,
    },
    cursor_request::{build_cursor_request, CosmosCursorRequest},
    driver::DriverHandle,
    error::{CosmosErrorCode, CosmosStatusCode, COSMOS_STATUS_SUCCESS},
    runtime::RuntimeContext,
    safety::MutexExt,
    string::CosmosStringView,
};
use azure_data_cosmos_driver::{
    error::{CosmosError, SubStatusCode},
    models::{ContainerReference, CosmosResponse, ResponseBody},
    options::OperationOptions,
    CosmosDriver, OperationPlan,
};
use futures::FutureExt;
use std::{
    borrow::Cow,
    collections::VecDeque,
    sync::{atomic::Ordering, Arc, Condvar, Mutex, Weak},
    time::{Duration, Instant},
};

/// Borrowed item bytes; zero-length members retain their position.
#[repr(C)]
pub struct CosmosCursorBytes {
    /// Readable bytes, valid until the owning completion is freed.
    pub data: *const u8,
    /// Byte count.
    pub len: usize,
}

/// Allocated V2 completion. All views survive cursor advancement and cursor free.
#[repr(C)]
pub struct CosmosCursorCompletion {
    /// Full allocated record size.
    pub struct_size_bytes: u32,
    /// Currently 1.
    pub abi_version: u32,
    /// Frozen common outcome, status, correlation, headers and error fields.
    pub common: CosmosCompletion,
    /// 0: no successful result; 1: opened; 2: page; 3: checkpoint; 4: end.
    pub result_kind: u32,
    /// 0: no payload; 1: raw bytes in `common.body`; 2: ordered items.
    pub body_kind: u32,
    /// All driver item buffers, not just the first; NULL when empty.
    pub items: *const CosmosCursorBytes,
    /// Number of item buffers.
    pub items_len: usize,
    /// Counted checkpoint, valid until completion free.
    pub checkpoint: CosmosStringView,
    /// Owned opened cursor; detach exactly once with `cosmos_cursor_completion_take_cursor`.
    pub cursor: *mut CursorHandle,
    /// Private allocation owner.
    pub backing: *mut CursorCompletionBacking,
}

/// Opaque owner of the item view array.
pub struct CursorCompletionBacking {
    _items: Vec<CosmosCursorBytes>,
}

/// Opaque retained plan. Synchronize handle free against all calls using that handle.
pub struct CursorHandle {
    inner: Arc<CursorInner>,
}

struct CursorInner {
    queue: Weak<CompletionQueueInner>,
    driver: Arc<CosmosDriver>,
    container: Option<ContainerReference>,
    options: OperationOptions,
    state: Mutex<CursorState>,
}

struct CursorState {
    plan: Option<OperationPlan>,
    busy: bool,
    exhausted: bool,
    terminal: CosmosStatusCode,
}

impl CursorInner {
    fn fail(&self, status: CosmosStatusCode) {
        let mut state = self.state.lock_recover();
        state.terminal = status;
        state.plan = None;
    }
}

enum ResultData {
    Opened(Arc<CursorInner>),
    Page(Box<CosmosResponse>),
    Checkpoint(String),
    End,
}

struct Delivery {
    result: Result<ResultData, CosmosError>,
    cancelled: bool,
    cursor: Option<Arc<CursorInner>>,
    op: Arc<OperationInner>,
    user_data: isize,
    cancel_requested: bool,
}

impl Delivery {
    fn lost(&self) {
        let status = CosmosErrorCode::CosmosErrorCodeDeliveryLost.as_status_code();
        self.op.terminal_status.store(status.0, Ordering::Release);
        self.op.state.store(
            CosmosOperationHandleState::CosmosOperationHandleStateFailed as u8,
            Ordering::Release,
        );
        if let Some(cursor) = &self.cursor {
            cursor.fail(status);
        }
        if let Ok(ResultData::Opened(cursor)) = &self.result {
            cursor.fail(status);
        }
    }

    fn transfer(self, include_details: bool) -> *mut CosmosCursorCompletion {
        let cancel_requested =
            self.cancel_requested || self.op.cancel_requested.load(Ordering::Acquire);
        let mut kind = 0;
        let mut body_kind = 0;
        let mut items = Vec::new();
        let mut opened = std::ptr::null_mut();
        let mut token_len = 0;
        let pending = match self.result {
            Ok(data) => {
                let mut response = None;
                let mut token = None;
                match data {
                    ResultData::Opened(cursor) => {
                        kind = 1;
                        opened = Box::into_raw(Box::new(CursorHandle { inner: cursor }));
                    }
                    ResultData::Page(page) => {
                        kind = 2;
                        match page.body() {
                            ResponseBody::NoPayload => {}
                            ResponseBody::Bytes(_) => body_kind = 1,
                            ResponseBody::Items(buffers) => {
                                body_kind = 2;
                                items = buffers
                                    .iter()
                                    .map(|bytes| CosmosCursorBytes {
                                        data: bytes.as_ptr(),
                                        len: bytes.len(),
                                    })
                                    .collect();
                            }
                        }
                        response = Some(*page);
                    }
                    ResultData::Checkpoint(text) => {
                        kind = 3;
                        token_len = text.len();
                        token = Some(text);
                    }
                    ResultData::End => kind = 4,
                }
                PendingCompletion::ok_response(self.user_data, self.op, response, token)
            }
            Err(error) if self.cancelled => {
                drop(error);
                PendingCompletion::cancelled(self.user_data, self.op)
            }
            Err(error) => PendingCompletion::error(self.user_data, self.op, error, include_details),
        };
        let mut common = pending.into_ffi();
        common.was_cancel_requested = u8::from(cancel_requested);
        let checkpoint = CosmosStringView {
            data: common.next_continuation.cast(),
            len: token_len,
        };
        let items_ptr = if items.is_empty() {
            std::ptr::null()
        } else {
            items.as_ptr()
        };
        let items_len = items.len();
        let completion = Box::new(CosmosCursorCompletion {
            struct_size_bytes: std::mem::size_of::<CosmosCursorCompletion>() as u32,
            abi_version: 1,
            common,
            result_kind: kind,
            body_kind,
            items: items_ptr,
            items_len,
            checkpoint,
            cursor: opened,
            backing: Box::into_raw(Box::new(CursorCompletionBacking { _items: items })),
        });
        // Transfer, not publication, releases the cursor. Old page owners remain independent.
        if let Some(cursor) = self.cursor {
            cursor.state.lock_recover().busy = false;
        }
        Box::into_raw(completion)
    }
}

pub(crate) struct CursorQueue {
    inner: Mutex<CursorQueueState>,
    available: Condvar,
    options: CqOptions,
}

struct CursorQueueState {
    deliveries: VecDeque<Delivery>,
    reserved: u32,
    shutdown: bool,
    abandoned: bool,
}

impl CursorQueue {
    pub(crate) fn new(options: CqOptions) -> Self {
        Self {
            inner: Mutex::new(CursorQueueState {
                deliveries: VecDeque::new(),
                reserved: 0,
                shutdown: false,
                abandoned: false,
            }),
            available: Condvar::new(),
            options,
        }
    }

    fn admit(
        &self,
        cursor: Option<&CursorInner>,
    ) -> Result<Option<OperationPlan>, CosmosErrorCode> {
        let mut queue = self.inner.lock_recover();
        if queue.shutdown || queue.abandoned {
            return Err(CosmosErrorCode::CosmosErrorCodeQueueShutdown);
        }
        let plan = if let Some(cursor) = cursor {
            let mut state = cursor.state.lock_recover();
            if state.busy {
                return Err(CosmosErrorCode::CosmosErrorCodeCursorBusy);
            }
            if state.terminal != COSMOS_STATUS_SUCCESS {
                return Err(CosmosErrorCode::CosmosErrorCodeCursorClosed);
            }
            if self.options.max_capacity > 0 && queue.reserved >= self.options.max_capacity {
                return Err(CosmosErrorCode::CosmosErrorCodeQueueFull);
            }
            state.busy = true;
            state.plan.take()
        } else {
            if self.options.max_capacity > 0 && queue.reserved >= self.options.max_capacity {
                return Err(CosmosErrorCode::CosmosErrorCodeQueueFull);
            }
            None
        };
        queue.reserved += 1;
        Ok(plan)
    }

    fn publish(&self, mut delivery: Delivery) {
        let mut queue = self.inner.lock_recover();
        let op = Arc::clone(&delivery.op);
        let _publication = op.publication.lock_recover();
        delivery.cancel_requested = op.cancel_requested.load(Ordering::Acquire);
        // Order cancellation against publication, including the gap after future completion.
        if delivery.cancel_requested && !delivery.cancelled {
            delivery.result = Err(error(CosmosErrorCode::CosmosErrorCodeOperationCancelled));
            delivery.cancelled = true;
            if let Some(cursor) = &delivery.cursor {
                cursor.fail(CosmosErrorCode::CosmosErrorCodeOperationCancelled.as_status_code());
            }
        }
        if queue.abandoned {
            queue.reserved -= 1;
            delivery.lost();
            return;
        }
        let (state, status) = match &delivery.result {
            Ok(_) => (
                CosmosOperationHandleState::CosmosOperationHandleStateCompleted,
                COSMOS_STATUS_SUCCESS,
            ),
            Err(err) => (
                if delivery.cancelled {
                    CosmosOperationHandleState::CosmosOperationHandleStateCancelled
                } else {
                    CosmosOperationHandleState::CosmosOperationHandleStateFailed
                },
                CosmosStatusCode::from_driver_error(err),
            ),
        };
        op.terminal_status.store(status.0, Ordering::Release);
        op.state.store(state as u8, Ordering::Release);
        queue.deliveries.push_back(delivery);
        self.available.notify_all();
    }

    fn wait(&self, timeout_ms: u32) -> Option<Delivery> {
        let deadline = (timeout_ms != u32::MAX)
            .then(|| Instant::now() + Duration::from_millis(u64::from(timeout_ms)));
        let mut queue = self.inner.lock_recover();
        loop {
            if let Some(delivery) = queue.deliveries.pop_front() {
                queue.reserved -= 1;
                return Some(delivery);
            }
            if queue.abandoned || (queue.shutdown && queue.reserved == 0) {
                return None;
            }
            queue = if let Some(deadline) = deadline {
                let remaining = deadline.saturating_duration_since(Instant::now());
                if remaining.is_zero() {
                    return None;
                }
                self.available
                    .wait_timeout(queue, remaining)
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .0
            } else {
                self.available
                    .wait(queue)
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
            };
        }
    }

    pub(crate) fn shutdown(&self) {
        self.inner.lock_recover().shutdown = true;
        self.available.notify_all();
    }

    pub(crate) fn abandon(&self) {
        let mut queue = self.inner.lock_recover();
        queue.abandoned = true;
        let pending = std::mem::take(&mut queue.deliveries);
        queue.reserved -= pending.len() as u32;
        for delivery in pending {
            delivery.lost();
        }
        self.available.notify_all();
    }

    pub(crate) fn state(&self) -> CosmosCompletionQueueState {
        let queue = self.inner.lock_recover();
        if queue.abandoned || (queue.shutdown && queue.reserved == 0) {
            CosmosCompletionQueueState::CosmosCompletionQueueStateDrained
        } else if queue.shutdown {
            CosmosCompletionQueueState::CosmosCompletionQueueStateShutdown
        } else {
            CosmosCompletionQueueState::CosmosCompletionQueueStateRunning
        }
    }
}

fn error(code: CosmosErrorCode) -> CosmosError {
    let message: Cow<'static, str> = match code {
        CosmosErrorCode::CosmosErrorCodeRepresentationUnsupported => {
            "The legacy response cannot represent all item buffers; use the retained cursor interface for feed results".into()
        }
        CosmosErrorCode::CosmosErrorCodeOperationCancelled => "The cursor operation was cancelled".into(),
        CosmosErrorCode::CosmosErrorCodeCursorClosed => "The cursor is no longer usable".into(),
        CosmosErrorCode::CosmosErrorCodeDeliveryLost => {
            "The completion was not delivered; cursor progress can no longer be used safely".into()
        }
        CosmosErrorCode::CosmosErrorCodeInternalError => {
            "The cursor operation failed internally".into()
        }
        _ => format!("{code:?}").into(),
    };
    CosmosError::builder()
        .with_status(
            code.to_status()
                .unwrap_or_else(CosmosErrorCode::panic_status),
        )
        .with_message(message)
        .build()
}

pub(crate) fn legacy_representation_error() -> CosmosError {
    error(CosmosErrorCode::CosmosErrorCodeRepresentationUnsupported)
}

fn write_error(out: *mut CosmosStatusCode, status: CosmosStatusCode) {
    if !out.is_null() {
        // SAFETY: caller supplies a writable status slot.
        unsafe { out.write(status) };
    }
}

/// Create an isolated cursor queue. Capacity 0 is unbounded; every admitted result reserves a slot.
#[no_mangle]
pub extern "C" fn cosmos_cursor_queue_create(
    runtime: *const RuntimeContext,
    max_capacity: u32,
) -> *mut CompletionQueue {
    let Some(runtime) = RuntimeContext::inner_arc(runtime) else {
        return std::ptr::null_mut();
    };
    CompletionQueue::new_with_mode(
        runtime,
        CqOptions {
            max_capacity,
            ..CqOptions::default()
        },
        true,
    )
}

/// Plan without consuming a data page. Input pointers need only survive this call.
#[no_mangle]
pub extern "C" fn cosmos_cursor_open_submit(
    driver: *const DriverHandle,
    request: *const CosmosCursorRequest,
    queue: *mut CompletionQueue,
    user_data: isize,
    out_pre_error: *mut CosmosStatusCode,
) -> *mut OperationHandle {
    let prepare = || {
        let queue = CompletionQueue::inner_arc(queue)
            .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?;
        let cursor_queue = queue
            .cursor
            .as_ref()
            .ok_or(CosmosErrorCode::CosmosErrorCodeQueueFormat)?;
        let driver = DriverHandle::inner_arc(driver)
            .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?;
        // SAFETY: request follows the versioned prefix and counted input contracts.
        let built = unsafe { build_cursor_request(request)? };
        cursor_queue.admit(None)?;
        Ok::<_, CosmosErrorCode>((queue, driver, built))
    };
    let prepared = std::panic::catch_unwind(std::panic::AssertUnwindSafe(prepare))
        .unwrap_or(Err(CosmosErrorCode::CosmosErrorCodeInternalError));
    let (queue, driver, built) = match prepared {
        Ok(values) => values,
        Err(code) => {
            write_error(out_pre_error, code.as_status_code());
            return std::ptr::null_mut();
        }
    };
    write_error(out_pre_error, COSMOS_STATUS_SUCCESS);
    let handle = OperationHandle::allocate();
    // SAFETY: handle was just allocated.
    let op = unsafe { Arc::clone(&(*handle).inner) };
    let runtime = Arc::clone(queue.runtime());
    runtime.tokio.spawn(async move {
        let work = async {
            let container = built.operation.container().cloned();
            let plan = driver
                .inner
                .plan_operation(
                    built.operation,
                    &built.options,
                    built.continuation.as_ref(),
                    &built.plan_options,
                )
                .await?;
            Ok(ResultData::Opened(Arc::new(CursorInner {
                queue: Arc::downgrade(&queue),
                driver: Arc::clone(&driver.inner),
                container,
                options: built.options,
                state: Mutex::new(CursorState {
                    plan: Some(plan),
                    busy: false,
                    exhausted: false,
                    terminal: COSMOS_STATUS_SUCCESS,
                }),
            })))
        };
        let (result, cancelled) = run(work, &op).await;
        if let Some(cursor_queue) = &queue.cursor {
            cursor_queue.publish(Delivery {
                result,
                cancelled,
                cursor: None,
                op,
                user_data,
                cancel_requested: false,
            });
        }
    });
    handle
}

async fn run(
    work: impl std::future::Future<Output = Result<ResultData, CosmosError>>,
    op: &OperationInner,
) -> (Result<ResultData, CosmosError>, bool) {
    let work = std::panic::AssertUnwindSafe(work).catch_unwind();
    tokio::pin!(work);
    tokio::select! {
        biased;
        _ = op.cancel_notify.notified() => (Err(error(CosmosErrorCode::CosmosErrorCodeOperationCancelled)), true),
        result = &mut work => (result.unwrap_or_else(|_| Err(error(CosmosErrorCode::CosmosErrorCodeInternalError))), false),
    }
}

fn submit_cursor(
    cursor: *const CursorHandle,
    checkpoint: bool,
    user_data: isize,
    out_pre_error: *mut CosmosStatusCode,
) -> *mut OperationHandle {
    // SAFETY: a non-NULL handle must remain live throughout this call.
    let Some(cursor) = (unsafe { cursor.as_ref() }) else {
        write_error(
            out_pre_error,
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code(),
        );
        return std::ptr::null_mut();
    };
    let cursor = Arc::clone(&cursor.inner);
    let Some(queue) = cursor.queue.upgrade() else {
        write_error(
            out_pre_error,
            CosmosErrorCode::CosmosErrorCodeQueueShutdown.as_status_code(),
        );
        return std::ptr::null_mut();
    };
    let Some(cursor_queue) = &queue.cursor else {
        write_error(
            out_pre_error,
            CosmosErrorCode::CosmosErrorCodeQueueFormat.as_status_code(),
        );
        return std::ptr::null_mut();
    };
    let mut plan = match cursor_queue.admit(Some(&cursor)) {
        Ok(plan) => plan,
        Err(code) => {
            write_error(out_pre_error, code.as_status_code());
            return std::ptr::null_mut();
        }
    };
    write_error(out_pre_error, COSMOS_STATUS_SUCCESS);
    let handle = OperationHandle::allocate();
    // SAFETY: handle was just allocated.
    let op = unsafe { Arc::clone(&(*handle).inner) };
    let runtime = Arc::clone(queue.runtime());
    runtime.tokio.spawn(async move {
        let work = async {
            let plan = plan
                .as_mut()
                .ok_or_else(|| error(CosmosErrorCode::CosmosErrorCodeCursorClosed))?;
            if checkpoint {
                return plan
                    .to_continuation_token()
                    .map(|token| ResultData::Checkpoint(token.as_str().to_owned()));
            }
            if cursor.state.lock_recover().exhausted {
                return Ok(ResultData::End);
            }
            cursor
                .driver
                .execute_plan(plan, cursor.container.clone(), cursor.options.clone())
                .await
                .map(|page| page.map_or(ResultData::End, |page| ResultData::Page(Box::new(page))))
        };
        let (result, cancelled) = run(work, &op).await;
        {
            let mut state = cursor.state.lock_recover();
            let unsupported = checkpoint
                && result.as_ref().err().is_some_and(|err| {
                    matches!(
                        err.status().sub_status(),
                        Some(
                            SubStatusCode::CLIENT_BUFFERED_QUERY_CONTINUATION_UNSUPPORTED
                                | SubStatusCode::CLIENT_CONTINUATION_TOKEN_NON_QUERY_OPERATION
                        )
                    )
                });
            if result.is_ok() || unsupported {
                state.exhausted |= matches!(result, Ok(ResultData::End));
                state.plan = plan;
            } else if let Err(err) = &result {
                state.terminal = CosmosStatusCode::from_driver_error(err);
            }
        }
        if let Some(cursor_queue) = &queue.cursor {
            cursor_queue.publish(Delivery {
                result,
                cancelled,
                cursor: Some(cursor),
                op,
                user_data,
                cancel_requested: false,
            });
        }
    });
    handle
}

/// Request one page. Busy lasts until transfer, not until the prior page is freed.
#[no_mangle]
pub extern "C" fn cosmos_cursor_next_submit(
    cursor: *const CursorHandle,
    user_data: isize,
    out_pre_error: *mut CosmosStatusCode,
) -> *mut OperationHandle {
    submit_cursor(cursor, false, user_data, out_pre_error)
}

/// Snapshot delivered-to-wrapper progress without advancing. Unsupported snapshots remain pageable.
#[no_mangle]
pub extern "C" fn cosmos_cursor_checkpoint_submit(
    cursor: *const CursorHandle,
    user_data: isize,
    out_pre_error: *mut CosmosStatusCode,
) -> *mut OperationHandle {
    submit_cursor(cursor, true, user_data, out_pre_error)
}

/// Non-blocking handle release; admitted work retains ownership and still completes.
#[no_mangle]
pub extern "C" fn cosmos_cursor_free(cursor: *mut CursorHandle) {
    if !cursor.is_null() {
        // SAFETY: caller relinquishes this unique, live handle.
        unsafe {
            drop(Box::from_raw(cursor));
        }
    }
}

/// Inspect terminal cursor failure, including queue destruction without result delivery.
#[no_mangle]
pub extern "C" fn cosmos_cursor_status(cursor: *const CursorHandle) -> CosmosStatusCode {
    // SAFETY: caller supplies a live handle or NULL.
    let Some(cursor) = (unsafe { cursor.as_ref() }) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    };
    let Some(queue) = cursor.inner.queue.upgrade() else {
        return CosmosErrorCode::CosmosErrorCodeDeliveryLost.as_status_code();
    };
    if queue
        .cursor
        .as_ref()
        .is_some_and(|q| q.inner.lock_recover().abandoned)
    {
        return CosmosErrorCode::CosmosErrorCodeDeliveryLost.as_status_code();
    }
    cursor.inner.state.lock_recover().terminal
}

/// Transfer allocated completion pointers. Timeout/empty is success with count zero.
/// Wrong format and invalid arguments return an error without consuming any result.
#[no_mangle]
pub extern "C" fn cosmos_cursor_queue_wait(
    queue: *mut CompletionQueue,
    out: *mut *mut CosmosCursorCompletion,
    max: usize,
    timeout_ms: u32,
    out_count: *mut usize,
) -> CosmosStatusCode {
    if out_count.is_null()
        || out.is_null()
        || max == 0
        || max > isize::MAX as usize / std::mem::size_of::<*mut CosmosCursorCompletion>()
    {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    // SAFETY: caller supplies a writable count.
    unsafe { out_count.write(0) };
    let Some(queue) = CompletionQueue::inner_arc(queue) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    };
    let Some(cursor_queue) = &queue.cursor else {
        return CosmosErrorCode::CosmosErrorCodeQueueFormat.as_status_code();
    };
    for index in 0..max {
        let Some(delivery) = cursor_queue.wait(if index == 0 { timeout_ms } else { 0 }) else {
            break;
        };
        let completion = delivery.transfer(cursor_queue.options.include_error_details);
        // SAFETY: caller supplies `max` pointer slots and a writable count.
        unsafe {
            out.add(index).write(completion);
            out_count.write(index + 1);
        }
    }
    COSMOS_STATUS_SUCCESS
}

/// Detach an opened cursor exactly once; NULL if absent or previously taken.
#[no_mangle]
pub extern "C" fn cosmos_cursor_completion_take_cursor(
    completion: *mut CosmosCursorCompletion,
) -> *mut CursorHandle {
    // SAFETY: caller supplies a live V2 completion or NULL.
    let Some(completion) = (unsafe { completion.as_mut() }) else {
        return std::ptr::null_mut();
    };
    std::mem::replace(&mut completion.cursor, std::ptr::null_mut())
}

/// Free one V2 completion and all its borrowed views, plus any undetached cursor.
#[no_mangle]
pub extern "C" fn cosmos_cursor_completion_free(completion: *mut CosmosCursorCompletion) {
    if completion.is_null() {
        return;
    }

    // SAFETY: caller relinquishes a unique V2 completion allocated by this library.
    unsafe {
        let mut completion = Box::from_raw(completion);
        completion.common.free_inner();
        cosmos_cursor_free(completion.cursor);
        drop(Box::from_raw(completion.backing));
    }
}

#[cfg(any(test, feature = "test-abi"))]
mod fixture;

#[cfg(test)]
mod tests;
