// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Acknowledged change feed delivery without native-to-host callbacks.

#[cfg(test)]
mod tests;

use crate::{
    completion::{
        CosmosCompletion, CosmosOperationHandleState, CqOptions, OperationHandle, OperationInner,
        PendingCompletion,
    },
    container_ref::ContainerRefHandle,
    cursor::CosmosCursorBytes,
    delivery_queue::{DeliveryQueue, QueuedDelivery},
    driver::DriverHandle,
    error::{CosmosErrorCode, CosmosStatusCode, COSMOS_STATUS_SUCCESS},
    runtime::RuntimeContext,
    safety::MutexExt,
    string::{optional_text, required_text, CosmosStringView},
};
use azure_data_cosmos_driver::{
    change_feed_processor::{
        Batch, BatchReceipt, BootstrapOptions, ChangeFeedProcessorMode, Processor, ProcessorOptions,
    },
    models::{ChangeFeedStartFrom, ContinuationToken, MaxItemCountHint, ResponseBody},
    CosmosError,
};
use futures::FutureExt;
use std::{
    future::Future,
    num::NonZeroU32,
    panic::AssertUnwindSafe,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, Weak,
    },
    time::Duration,
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

/// Versioned processor configuration. Initialize with `cosmos_processor_request_init`.
#[repr(C)]
pub struct CosmosProcessorRequest {
    /// Full readable allocation size, including zero extension bytes.
    pub struct_size_bytes: u32,
    /// Must be 1.
    pub abi_version: u32,
    /// Workload name; copied during submission.
    pub processor_name: CosmosStringView,
    /// Worker identity; copied during submission.
    pub instance_name: CosmosStringView,
    /// 1: latest version; 2: all versions and deletes.
    pub mode: u32,
    /// 1: beginning; 2: now; 3: RFC 3339 time. Resume takes precedence.
    pub start_from: u32,
    /// Required only for a time-based start.
    pub start_time: CosmosStringView,
    /// Optional initial lease-scoped composite continuation.
    pub continuation: CosmosStringView,
    /// Positive item-count hint, or -1 for the service default.
    pub max_item_count: i32,
    /// Per-lease idle delay in milliseconds. Zero is permitted.
    pub poll_delay_ms: u64,
    /// Minimum target lease count.
    pub min_leases: u32,
    /// Maximum target lease count, or zero for uncapped.
    pub max_leases: u32,
    /// 0: fast bootstrap restart; 1: verify initial leases.
    pub verify_on_restart: u32,
    /// Must be zero.
    pub reserved: [u32; 4],
}

/// A format-isolated processor completion queue.
///
/// Freeing or shutting down this queue requests shutdown of its processors.
pub struct ProcessorQueue {
    inner: Arc<QueueInner>,
}

struct QueueInner {
    runtime: Arc<RuntimeContext>,
    deliveries: DeliveryQueue<Delivery>,
    processors: Mutex<Vec<Weak<ProcessorInner>>>,
}

impl QueueInner {
    fn stop_processors(&self) {
        for processor in self
            .processors
            .lock_recover()
            .iter()
            .filter_map(Weak::upgrade)
        {
            processor.driver.request_stop();
        }
    }
}

/// A running processor bound to its opening queue and runtime.
///
/// Synchronize handle free against calls that use the same raw pointer.
pub struct ProcessorHandle {
    inner: Arc<ProcessorInner>,
}

struct ProcessorInner {
    driver: Processor,
    queue: Weak<QueueInner>,
    next_pending: AtomicBool,
    stop_pending: AtomicBool,
}

impl ProcessorInner {
    fn pending(&self, stop: bool) -> &AtomicBool {
        if stop {
            &self.stop_pending
        } else {
            &self.next_pending
        }
    }
}

/// A one-use processing receipt. Free without acknowledgment on callback failure.
pub struct ProcessorReceipt {
    receipt: Mutex<Option<BatchReceipt>>,
    queue: Weak<QueueInner>,
}

/// Owned processor result. All borrowed views live until completion free.
#[repr(C)]
pub struct CosmosProcessorCompletion {
    /// Complete allocated record size.
    pub struct_size_bytes: u32,
    /// Currently 1.
    pub abi_version: u32,
    /// Common status, error, diagnostics, correlation, headers, and raw body.
    pub common: CosmosCompletion,
    /// 0: error; 1: started; 2: batch; 3: checkpoint saved; 4: stopped; 5: end.
    pub result_kind: u32,
    /// 0: no payload; 1: common raw bytes; 2: ordered item buffers.
    pub body_kind: u32,
    /// Complete item array, preserving envelopes and encoding.
    pub items: *const CosmosCursorBytes,
    /// Number of item buffers.
    pub items_len: usize,
    /// Workload-scoped lease identity for a batch.
    pub lease_token: CosmosStringView,
    /// Started processor; detach once before freeing this completion.
    pub processor: *mut ProcessorHandle,
    /// Batch receipt; detach once, then checkpoint after successful processing.
    pub receipt: *mut ProcessorReceipt,
    /// Private allocation owner.
    pub backing: *mut ProcessorCompletionBacking,
}

/// Opaque owner of borrowed completion views.
pub struct ProcessorCompletionBacking {
    _items: Vec<CosmosCursorBytes>,
    _lease: String,
}

enum ResultData {
    Started(Arc<ProcessorInner>),
    Batch(Box<Batch>),
    CheckpointSaved,
    Stopped,
    End,
}

struct Delivery {
    result: Result<ResultData, CosmosError>,
    op: Arc<OperationInner>,
    user_data: isize,
    queue: Weak<QueueInner>,
    admission: Option<(Arc<ProcessorInner>, bool)>,
}

impl QueuedDelivery for Delivery {
    fn published(&self) {
        let (state, status) = match &self.result {
            Ok(_) => (
                CosmosOperationHandleState::CosmosOperationHandleStateCompleted,
                COSMOS_STATUS_SUCCESS,
            ),
            Err(err) => (
                CosmosOperationHandleState::CosmosOperationHandleStateFailed,
                CosmosStatusCode::from_driver_error(err),
            ),
        };
        self.op.terminal_status.store(status.0, Ordering::Release);
        self.op.state.store(state as u8, Ordering::Release);
    }

    fn lost(&self) {
        self.op.terminal_status.store(
            CosmosErrorCode::CosmosErrorCodeDeliveryLost
                .as_status_code()
                .0,
            Ordering::Release,
        );
        self.op.state.store(
            CosmosOperationHandleState::CosmosOperationHandleStateFailed as u8,
            Ordering::Release,
        );
        if let Ok(ResultData::Started(processor)) = &self.result {
            processor.driver.request_stop();
        }
        if let Some((processor, stop)) = &self.admission {
            processor.pending(*stop).store(false, Ordering::Release);
            processor.driver.request_stop();
        }
    }
}

impl Delivery {
    fn transfer(self) -> *mut CosmosProcessorCompletion {
        let mut processor = std::ptr::null_mut();
        let mut receipt = std::ptr::null_mut();
        let mut lease = String::new();
        let mut items = Vec::new();
        let mut kind = 0;
        let mut body_kind = 0;
        let pending = match self.result {
            Ok(data) => {
                let mut response = None;
                match data {
                    ResultData::Started(inner) => {
                        kind = 1;
                        processor = Box::into_raw(Box::new(ProcessorHandle { inner }));
                    }
                    ResultData::Batch(batch) => {
                        kind = 2;
                        let (token, page, acknowledgment) = batch.into_parts();
                        lease = token;
                        receipt = Box::into_raw(Box::new(ProcessorReceipt {
                            receipt: Mutex::new(Some(acknowledgment)),
                            queue: self.queue,
                        }));
                        match page.body() {
                            ResponseBody::NoPayload => {}
                            ResponseBody::Bytes(_) => body_kind = 1,
                            ResponseBody::Items(buffers) => {
                                body_kind = 2;
                                items = buffers
                                    .iter()
                                    .map(|b| CosmosCursorBytes {
                                        data: b.as_ptr(),
                                        len: b.len(),
                                    })
                                    .collect();
                            }
                        }
                        response = Some(page);
                    }
                    ResultData::CheckpointSaved => kind = 3,
                    ResultData::Stopped => kind = 4,
                    ResultData::End => kind = 5,
                }
                PendingCompletion::ok_response(self.user_data, self.op, response, None)
            }
            Err(err) => PendingCompletion::error(self.user_data, self.op, err, true),
        };
        let result = CosmosProcessorCompletion {
            struct_size_bytes: std::mem::size_of::<CosmosProcessorCompletion>() as u32,
            abi_version: 1,
            common: pending.into_ffi(),
            result_kind: kind,
            body_kind,
            items: if items.is_empty() {
                std::ptr::null()
            } else {
                items.as_ptr()
            },
            items_len: items.len(),
            lease_token: CosmosStringView {
                data: lease.as_ptr(),
                len: lease.len(),
            },
            processor,
            receipt,
            backing: Box::into_raw(Box::new(ProcessorCompletionBacking {
                _items: items,
                _lease: lease,
            })),
        };
        if let Some((processor, stop)) = self.admission {
            processor.pending(stop).store(false, Ordering::Release);
        }
        Box::into_raw(Box::new(result))
    }
}

fn error(code: CosmosErrorCode) -> CosmosError {
    CosmosError::builder()
        .with_status(
            code.to_status()
                .unwrap_or_else(CosmosErrorCode::panic_status),
        )
        .with_message(format!("{code:?}"))
        .build()
}

fn pre_error(out: *mut CosmosStatusCode, code: CosmosStatusCode) {
    if !out.is_null() {
        // SAFETY: caller supplies writable error storage or NULL.
        unsafe { out.write(code) };
    }
}

fn launch(
    queue: Arc<QueueInner>,
    user_data: isize,
    admission: Option<(Arc<ProcessorInner>, bool)>,
    work: impl Future<Output = Result<ResultData, CosmosError>> + Send + 'static,
) -> *mut OperationHandle {
    let handle = OperationHandle::allocate();
    let op = OperationHandle::inner_arc(handle).expect("new operation handle");
    let runtime = queue.runtime.clone();
    runtime.tokio.spawn(async move {
        let result = AssertUnwindSafe(work)
            .catch_unwind()
            .await
            .unwrap_or_else(|_| Err(error(CosmosErrorCode::CosmosErrorCodeInternalError)));
        queue.deliveries.publish(Delivery {
            result,
            op,
            user_data,
            queue: Arc::downgrade(&queue),
            admission,
        });
    });
    handle
}

/// Initializes a full-size caller-owned request with default timing and page size.
#[no_mangle]
pub extern "C" fn cosmos_processor_request_init(out: *mut CosmosProcessorRequest) {
    if out.is_null() {
        return;
    }
    // SAFETY: caller provides writable storage for a complete request.
    unsafe {
        out.write(CosmosProcessorRequest {
            struct_size_bytes: std::mem::size_of::<CosmosProcessorRequest>() as u32,
            abi_version: 1,
            processor_name: CosmosStringView::default(),
            instance_name: CosmosStringView::default(),
            mode: 1,
            start_from: 2,
            start_time: CosmosStringView::default(),
            continuation: CosmosStringView::default(),
            max_item_count: 100,
            poll_delay_ms: 5000,
            min_leases: 0,
            max_leases: 0,
            verify_on_restart: 0,
            reserved: [0; 4],
        });
    }
}

unsafe fn options(
    input: *const CosmosProcessorRequest,
) -> Result<ProcessorOptions, CosmosErrorCode> {
    let invalid = CosmosErrorCode::CosmosErrorCodeInvalidOptionValue;
    if input.is_null() {
        return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
    }
    // SAFETY: callers provide a readable size/version prefix.
    let size = unsafe { std::ptr::addr_of!((*input).struct_size_bytes).read() } as usize;
    if size < 8 {
        return Err(invalid);
    }
    // SAFETY: size/version prefix is readable.
    let version = unsafe { std::ptr::addr_of!((*input).abi_version).read() };
    let known = std::mem::size_of::<CosmosProcessorRequest>();
    if version != 1 || size < known || size > isize::MAX as usize {
        return Err(invalid);
    }
    // SAFETY: caller guarantees the declared allocation and counted strings are readable.
    let (request, extension) = unsafe {
        (
            &*input,
            std::slice::from_raw_parts(input.cast::<u8>().add(known), size - known),
        )
    };
    if extension.iter().any(|b| *b != 0)
        || request.reserved != [0; 4]
        || request.verify_on_restart > 1
    {
        return Err(invalid);
    }
    // SAFETY: counted input views remain readable through submission.
    let (name, instance, timestamp, continuation) = unsafe {
        (
            required_text(request.processor_name, invalid)?,
            required_text(request.instance_name, invalid)?,
            optional_text(request.start_time, invalid)?,
            optional_text(request.continuation, invalid)?,
        )
    };
    let mode = match request.mode {
        1 => ChangeFeedProcessorMode::LatestVersion,
        2 => ChangeFeedProcessorMode::AllVersionsAndDeletes,
        _ => return Err(invalid),
    };
    let start = match (request.start_from, timestamp) {
        (1, None) => ChangeFeedStartFrom::Beginning,
        (2, None) => ChangeFeedStartFrom::Now,
        (3, Some(time)) => ChangeFeedStartFrom::PointInTime(
            OffsetDateTime::parse(&time, &Rfc3339).map_err(|_| invalid)?,
        ),
        _ => return Err(invalid),
    };
    let hint = match request.max_item_count {
        -1 => MaxItemCountHint::ServerDecides,
        value if value > 0 => {
            MaxItemCountHint::Limit(NonZeroU32::new(value as u32).ok_or(invalid)?)
        }
        _ => return Err(invalid),
    };
    let mut options = ProcessorOptions::new(
        BootstrapOptions::new(mode, name, instance)
            .with_lease_verification_enabled_on_restart(request.verify_on_restart == 1),
    )
    .with_start(start)
    .with_max_item_count(hint)
    .with_poll_delay(Duration::from_millis(request.poll_delay_ms))
    .with_lease_counts(request.min_leases, request.max_leases);
    if let Some(token) = continuation {
        options = options.with_continuation(ContinuationToken::from_string(token));
    }
    Ok(options)
}

/// Creates a queue. Capacity bounds admitted start/next results; zero is uncapped.
///
/// Acknowledgment and stop results use separate control admission so an outstanding
/// next request cannot block the checkpoint that would enable its next page.
#[no_mangle]
pub extern "C" fn cosmos_processor_queue_create(
    runtime: *const RuntimeContext,
    capacity: u32,
) -> *mut ProcessorQueue {
    let Some(runtime) = RuntimeContext::inner_arc(runtime) else {
        return std::ptr::null_mut();
    };
    Box::into_raw(Box::new(ProcessorQueue {
        inner: Arc::new(QueueInner {
            runtime,
            deliveries: DeliveryQueue::new(CqOptions {
                max_capacity: capacity,
                ..CqOptions::default()
            }),
            processors: Mutex::new(Vec::new()),
        }),
    }))
}

/// Starts a processor asynchronously. All input strings are copied during this call.
///
/// Handles and input allocations must be live for the call. The completion owns the
/// started handle until `cosmos_processor_completion_take_processor` detaches it.
#[no_mangle]
pub extern "C" fn cosmos_processor_start_submit(
    source_driver: *const DriverHandle,
    source: *const ContainerRefHandle,
    lease_driver: *const DriverHandle,
    leases: *const ContainerRefHandle,
    request: *const CosmosProcessorRequest,
    queue: *mut ProcessorQueue,
    user_data: isize,
    out_pre_error: *mut CosmosStatusCode,
) -> *mut OperationHandle {
    let prepare = || {
        // SAFETY: caller keeps handles and input readable for this call.
        let queue = unsafe { queue.as_ref() }
            .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?
            .inner
            .clone();
        let source_driver = DriverHandle::inner_arc(source_driver)
            .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?;
        let source = ContainerRefHandle::from_ptr(source)
            .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?
            .inner
            .clone();
        let lease_driver = DriverHandle::inner_arc(lease_driver)
            .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?;
        let leases = ContainerRefHandle::from_ptr(leases)
            .ok_or(CosmosErrorCode::CosmosErrorCodeInvalidArgument)?
            .inner
            .clone();
        // SAFETY: request follows the size-prefix and counted-string contracts.
        let options = unsafe { options(request)? };
        queue.deliveries.reserve()?;
        Ok::<_, CosmosErrorCode>((queue, source_driver, source, lease_driver, leases, options))
    };
    let prepared = AssertUnwindSafe(prepare);
    let (queue, source_driver, source, lease_driver, leases, options) =
        match std::panic::catch_unwind(prepared)
            .unwrap_or(Err(CosmosErrorCode::CosmosErrorCodeInternalError))
        {
            Ok(value) => value,
            Err(code) => {
                pre_error(out_pre_error, code.as_status_code());
                return std::ptr::null_mut();
            }
        };
    pre_error(out_pre_error, COSMOS_STATUS_SUCCESS);
    let weak = Arc::downgrade(&queue);
    launch(queue, user_data, None, async move {
        let driver = Processor::start(
            source_driver.inner.clone(),
            source,
            lease_driver.inner.clone(),
            leases,
            options,
        )
        .await?;
        let processor = Arc::new(ProcessorInner {
            driver,
            queue: weak.clone(),
            next_pending: AtomicBool::new(false),
            stop_pending: AtomicBool::new(false),
        });
        if let Some(queue) = weak.upgrade() {
            let mut processors = queue.processors.lock_recover();
            processors.retain(|entry| entry.strong_count() > 0);
            processors.push(Arc::downgrade(&processor));
            if queue.deliveries.state()
                != crate::completion::CosmosCompletionQueueState::CosmosCompletionQueueStateRunning
            {
                processor.driver.request_stop();
            }
        }
        Ok(ResultData::Started(processor))
    })
}

/// Requests the next batch. Only one undelivered next request is allowed per processor.
///
/// After transfer, request further batches to process other leases concurrently.
/// The driver still permits only one unacknowledged batch for each lease.
#[no_mangle]
pub extern "C" fn cosmos_processor_next_submit(
    processor: *const ProcessorHandle,
    user_data: isize,
    out_pre_error: *mut CosmosStatusCode,
) -> *mut OperationHandle {
    submit_processor(processor, user_data, out_pre_error, false)
}

/// Stops coordination and cancels outstanding receipts, then delivers a stopped result.
#[no_mangle]
pub extern "C" fn cosmos_processor_stop_submit(
    processor: *const ProcessorHandle,
    user_data: isize,
    out_pre_error: *mut CosmosStatusCode,
) -> *mut OperationHandle {
    submit_processor(processor, user_data, out_pre_error, true)
}

fn submit_processor(
    processor: *const ProcessorHandle,
    user_data: isize,
    out: *mut CosmosStatusCode,
    stop: bool,
) -> *mut OperationHandle {
    // SAFETY: caller keeps the processor handle live during this call.
    let Some(processor) = (unsafe { processor.as_ref() }) else {
        pre_error(
            out,
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code(),
        );
        return std::ptr::null_mut();
    };
    let processor = processor.inner.clone();
    let prepare = || {
        let queue = processor
            .queue
            .upgrade()
            .ok_or(CosmosErrorCode::CosmosErrorCodeQueueShutdown)?;
        let busy = processor.pending(stop);
        if busy.swap(true, Ordering::AcqRel) {
            return Err(CosmosErrorCode::CosmosErrorCodeCursorBusy);
        }
        let admitted = if stop {
            queue.deliveries.reserve_control()
        } else {
            queue.deliveries.reserve()
        };
        if let Err(err) = admitted {
            busy.store(false, Ordering::Release);
            return Err(err);
        }
        Ok(queue)
    };
    let queue = match prepare() {
        Ok(queue) => queue,
        Err(code) => {
            pre_error(out, code.as_status_code());
            return std::ptr::null_mut();
        }
    };
    pre_error(out, COSMOS_STATUS_SUCCESS);
    let admission = Some((processor.clone(), stop));
    launch(queue, user_data, admission, async move {
        if stop {
            processor.driver.stop().await;
            Ok(ResultData::Stopped)
        } else {
            Ok(processor
                .driver
                .next_batch()
                .await
                .map_or(ResultData::End, |batch| ResultData::Batch(Box::new(batch))))
        }
    })
}

/// Acknowledges successful callback completion and persists the batch checkpoint.
///
/// The receipt is consumed only after submission is accepted. Free its handle
/// after this call; the admitted operation owns the pending acknowledgment.
/// A successful result means persistence, not just receipt of this command.
#[no_mangle]
pub extern "C" fn cosmos_processor_checkpoint_submit(
    receipt: *const ProcessorReceipt,
    user_data: isize,
    out: *mut CosmosStatusCode,
) -> *mut OperationHandle {
    // SAFETY: caller keeps the receipt handle live during this call.
    let Some(receipt) = (unsafe { receipt.as_ref() }) else {
        pre_error(
            out,
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code(),
        );
        return std::ptr::null_mut();
    };
    let prepare = || {
        let queue = receipt
            .queue
            .upgrade()
            .ok_or(CosmosErrorCode::CosmosErrorCodeQueueShutdown)?;
        let mut value = receipt.receipt.lock_recover();
        if value.is_none() {
            return Err(CosmosErrorCode::CosmosErrorCodeInvalidArgument);
        }
        queue.deliveries.reserve_control()?;
        Ok((queue, value.take().expect("receipt checked under lock")))
    };
    let (queue, receipt) = match prepare() {
        Ok(value) => value,
        Err(code) => {
            pre_error(out, code.as_status_code());
            return std::ptr::null_mut();
        }
    };
    pre_error(out, COSMOS_STATUS_SUCCESS);
    launch(queue, user_data, None, async move {
        receipt.checkpoint().await?;
        Ok(ResultData::CheckpointSaved)
    })
}

/// Transfers up to `max` owned completion pointers. Timeout/empty is success with zero results.
///
/// Supply writable pointer slots and count; synchronize concurrent waits and queue free.
#[no_mangle]
pub extern "C" fn cosmos_processor_queue_wait(
    queue: *const ProcessorQueue,
    out: *mut *mut CosmosProcessorCompletion,
    max: usize,
    timeout_ms: u32,
    out_count: *mut usize,
) -> CosmosStatusCode {
    if out_count.is_null()
        || out.is_null()
        || max == 0
        || max > isize::MAX as usize / std::mem::size_of::<*mut CosmosProcessorCompletion>()
    {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    }
    // SAFETY: caller provides writable count and a live queue.
    unsafe {
        out_count.write(0);
    }
    let Some(queue) = (unsafe { queue.as_ref() }) else {
        return CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code();
    };
    let queue = queue.inner.clone();
    for index in 0..max {
        let Some(delivery) = queue
            .deliveries
            .wait(if index == 0 { timeout_ms } else { 0 })
        else {
            break;
        };
        // SAFETY: caller supplies max pointer slots and writable count.
        unsafe {
            out.add(index).write(delivery.transfer());
            out_count.write(index + 1);
        }
    }
    COSMOS_STATUS_SUCCESS
}

/// Detaches a started processor exactly once, or returns NULL.
#[no_mangle]
pub extern "C" fn cosmos_processor_completion_take_processor(
    completion: *mut CosmosProcessorCompletion,
) -> *mut ProcessorHandle {
    // SAFETY: caller supplies a live completion or NULL.
    unsafe { completion.as_mut() }.map_or(std::ptr::null_mut(), |c| {
        std::mem::replace(&mut c.processor, std::ptr::null_mut())
    })
}

/// Detaches a batch receipt exactly once, or returns NULL.
#[no_mangle]
pub extern "C" fn cosmos_processor_completion_take_receipt(
    completion: *mut CosmosProcessorCompletion,
) -> *mut ProcessorReceipt {
    // SAFETY: caller supplies a live completion or NULL.
    unsafe { completion.as_mut() }.map_or(std::ptr::null_mut(), |c| {
        std::mem::replace(&mut c.receipt, std::ptr::null_mut())
    })
}

/// Frees a receipt. An unacknowledged receipt abandons its batch without saving progress.
#[no_mangle]
pub extern "C" fn cosmos_processor_receipt_free(receipt: *mut ProcessorReceipt) {
    if !receipt.is_null() {
        // SAFETY: caller relinquishes a uniquely owned receipt allocation.
        unsafe {
            drop(Box::from_raw(receipt));
        }
    }
}

/// Requests shutdown without waiting and frees the caller's processor handle.
#[no_mangle]
pub extern "C" fn cosmos_processor_free(processor: *mut ProcessorHandle) {
    if !processor.is_null() {
        // SAFETY: caller relinquishes a uniquely owned processor handle.
        let processor = unsafe { Box::from_raw(processor) };
        processor.inner.driver.request_stop();
    }
}

/// Releases a completion, its payload views, and any undetached handles.
#[no_mangle]
pub extern "C" fn cosmos_processor_completion_free(completion: *mut CosmosProcessorCompletion) {
    if completion.is_null() {
        return;
    }
    // SAFETY: caller relinquishes a unique completion allocated by this API.
    unsafe {
        let mut completion = Box::from_raw(completion);
        completion.common.free_inner();
        cosmos_processor_free(completion.processor);
        cosmos_processor_receipt_free(completion.receipt);
        drop(Box::from_raw(completion.backing));
    }
}

/// Stops admission and requests processor shutdown. Previously admitted results can drain.
#[no_mangle]
pub extern "C" fn cosmos_processor_queue_shutdown(queue: *const ProcessorQueue) {
    // SAFETY: caller supplies a live queue or NULL.
    if let Some(queue) = unsafe { queue.as_ref() } {
        queue.inner.deliveries.shutdown();
        queue.inner.stop_processors();
    }
}

/// Frees the queue, abandons pending delivery, and requests processor shutdown.
#[no_mangle]
pub extern "C" fn cosmos_processor_queue_free(queue: *mut ProcessorQueue) {
    if queue.is_null() {
        return;
    }
    // SAFETY: caller relinquishes its unique queue handle.
    let queue = unsafe { Box::from_raw(queue) };
    queue.inner.deliveries.abandon();
    queue.inner.stop_processors();
}
