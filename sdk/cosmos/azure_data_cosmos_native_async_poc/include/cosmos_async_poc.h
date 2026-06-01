/* ========================================================================== *
 *  cosmos_async_poc.h
 *
 *  C ABI for the azure_data_cosmos_native_async_poc feasibility spike.
 *
 *  Purpose
 *  -------
 *  Validate that the planned `azure_data_cosmos_driver_native` C ABI
 *  (see PR #4461 / NATIVE_WRAPPER_SPEC.md) can be designed around an
 *  asynchronous, completion-queue-style model instead of the synchronous
 *  blocking-call model currently drafted.
 *
 *  Concurrency model
 *  -----------------
 *  Submission functions (e.g., cosmos_read_item) are NON-blocking. They
 *  enqueue work on the Rust-side tokio runtime and return an opaque
 *  operation handle immediately. The completion is delivered later, on a
 *  separate Rust worker thread, by pushing a record onto the per-consumer
 *  completion queue (cosmos_cq_t).
 *
 *  The host language (e.g., .NET) is expected to run ONE dedicated thread
 *  per cosmos_cq_t that loops on cosmos_cq_wait and dispatches completions
 *  to whatever native async primitive it prefers (TaskCompletionSource,
 *  CompletableFuture, channels, etc.). This is the same shape as the
 *  Microsoft.Azure.Cosmos.Direct (Rntbd2) receive loop already in production
 *  in the V3 .NET SDK.
 *
 *  user_data round-trip
 *  --------------------
 *  Every submission takes an opaque `void* user_data` pointer. Rust treats
 *  this value as completely opaque: it NEVER dereferences it, never inspects
 *  its bits beyond moving it. The same pointer value is returned via the
 *  out_user_data slot of cosmos_cq_wait when the corresponding completion
 *  is drained. The host uses this slot to find its per-op state
 *  (e.g., GCHandle.ToIntPtr(tcsCompleter) in .NET).
 *
 *  Ownership conventions used in this header
 *  -----------------------------------------
 *    OWNERSHIP: caller-frees -> caller must call the matching cosmos_*_free
 *    OWNERSHIP: callee-frees -> implementation owns the pointer; caller MUST NOT free
 *    OWNERSHIP: borrowed     -> pointer is valid only for the duration of the call
 *
 *  Threading conventions
 *  ---------------------
 *    THREAD: shared       -> may be called from any thread, concurrently
 *    THREAD: exclusive    -> not safe to call concurrently for the same handle
 *    THREAD: single-waiter -> at most ONE thread may be inside this call per CQ
 *
 *  Panic safety
 *  ------------
 *  Every extern "C" function in the Rust impl is wrapped in
 *  std::panic::catch_unwind. A panic across the FFI boundary is impossible
 *  by construction; panics surface to the host as COSMOS_ERROR with a
 *  message in cosmos_error_t.
 * ========================================================================== */

#ifndef COSMOS_ASYNC_POC_H
#define COSMOS_ASYNC_POC_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* -------------------------------------------------------------------------- *
 *  Opaque handles. Layout is intentionally unpublished and may change without
 *  notice; the host must treat these as black boxes.
 * -------------------------------------------------------------------------- */
typedef struct cosmos_runtime  cosmos_runtime_t;   /* tokio runtime              */
typedef struct cosmos_driver   cosmos_driver_t;    /* an authenticated client    */
typedef struct cosmos_cq       cosmos_cq_t;        /* completion queue           */
typedef struct cosmos_op       cosmos_op_t;        /* in-flight operation handle */
typedef struct cosmos_response cosmos_response_t;  /* a completed response       */

/* -------------------------------------------------------------------------- *
 *  Status codes returned by cosmos_cq_wait.
 *
 *  COSMOS_OK        : Submission completed successfully. *out_resp is non-null
 *                     and OWNED BY CALLER (must call cosmos_response_free).
 *  COSMOS_CANCELLED : Submission completed because someone called
 *                     cosmos_cancel. *out_resp is NULL. Caller MUST still
 *                     unpack *out_user_data and clean up its per-op state.
 *  COSMOS_TIMEOUT   : The timeout in cosmos_cq_wait elapsed with no completion.
 *                     *out_resp and *out_user_data are both NULL. NOT a terminal
 *                     state for any op.
 *  COSMOS_SHUTDOWN  : The CQ has been shut down (cosmos_cq_shutdown was called)
 *                     and is now drained. *out_resp and *out_user_data are NULL.
 *                     The host should exit its drain loop.
 *  COSMOS_ERROR     : Submission completed with an error. *out_resp is non-null
 *                     and OWNED BY CALLER; status/body accessors describe the
 *                     error. (We deliberately reuse cosmos_response_t for the
 *                     error path so the host's dispatcher does not need a
 *                     second free path.)
 * -------------------------------------------------------------------------- */
typedef enum cosmos_status {
    COSMOS_OK        = 0,
    COSMOS_CANCELLED = 1,
    COSMOS_TIMEOUT   = 2,
    COSMOS_SHUTDOWN  = 3,
    COSMOS_ERROR     = 4
} cosmos_status_t;

/* -------------------------------------------------------------------------- *
 *  Inline error returned by submission/setup APIs (where there is no response
 *  to attach the error to). The `message_utf8` string is valid only until the
 *  next call into the library on the SAME thread; if the host needs to keep
 *  it, it MUST copy it.
 *
 *  OWNERSHIP: borrowed (NOT caller-frees)
 * -------------------------------------------------------------------------- */
typedef struct cosmos_error {
    int32_t      code;          /* 0 on success; nonzero on failure       */
    const char*  message_utf8;  /* nul-terminated, borrowed (see above)   */
} cosmos_error_t;

/* ========================================================================== *
 *  Runtime lifecycle
 * ========================================================================== */

/*
 *  Construct a new tokio runtime (multi-thread, small worker pool).
 *
 *  OWNERSHIP: caller-frees via cosmos_runtime_free
 *  THREAD:    shared (call from any thread, but you almost never need >1)
 *  Returns NULL only on catastrophic allocation failure.
 */
cosmos_runtime_t* cosmos_runtime_new(void);

/*
 *  Free a runtime created by cosmos_runtime_new. Blocks briefly while tokio
 *  shuts down its worker threads. Passing NULL is a no-op (defensive).
 *
 *  THREAD: exclusive (do not call concurrently with anything else on this rt)
 */
void cosmos_runtime_free(cosmos_runtime_t* rt);

/* ========================================================================== *
 *  Driver lifecycle (authenticated client to a Cosmos account)
 * ========================================================================== */

/*
 *  Construct a driver bound to the given runtime, authenticated with a
 *  master key. The endpoint string is expected to be UTF-8 and nul-terminated;
 *  the master_key string likewise. Both strings are BORROWED for the duration
 *  of this call only.
 *
 *  On failure, returns NULL and populates *out_err (message_utf8 is borrowed,
 *  see cosmos_error_t docs above). The runtime remains valid; the caller may
 *  retry.
 *
 *  OWNERSHIP: caller-frees via cosmos_driver_free
 *  THREAD:    shared
 */
cosmos_driver_t* cosmos_driver_new(
    cosmos_runtime_t* rt,
    const char*       endpoint_utf8,
    const char*       master_key_utf8,
    cosmos_error_t*   out_err);

/*
 *  Free a driver. Outstanding operations submitted via this driver are
 *  guaranteed to have already produced a completion BEFORE this returns:
 *  callers should drain their cosmos_cq_t to empty before freeing the driver.
 *  (We choose not to enforce this in the POC; behavior of free-with-ops is
 *  documented as "undefined" so we can revisit if the spike surfaces it.)
 *
 *  THREAD: exclusive
 */
void cosmos_driver_free(cosmos_driver_t* drv);

/* ========================================================================== *
 *  Completion queue
 *
 *  ONE CQ per logical consumer. In the .NET spike, one CQ is owned by one
 *  CosmosNativeClient instance, drained by exactly one dedicated host thread.
 *  Multi-waiter CQs are NOT in scope for this feasibility spike.
 * ========================================================================== */

/*
 *  Create an empty completion queue.
 *
 *  OWNERSHIP: caller-frees via cosmos_cq_free
 *  THREAD:    shared
 */
cosmos_cq_t* cosmos_cq_new(void);

/*
 *  Free a completion queue. The caller MUST have already called
 *  cosmos_cq_shutdown and drained the queue until COSMOS_SHUTDOWN was
 *  returned. Behavior with pending completions in the queue is undefined
 *  in the POC.
 *
 *  THREAD: exclusive
 */
void cosmos_cq_free(cosmos_cq_t* cq);

/*
 *  Signal that no further submissions should be accepted against this CQ,
 *  wake any blocked waiters, and let the queue drain. After shutdown, new
 *  submissions that target this CQ will fail with COSMOS_ERROR. Already-
 *  in-flight operations still produce exactly one completion on the queue
 *  (Ok | Error | Cancelled). Once all in-flight ops have drained,
 *  cosmos_cq_wait returns COSMOS_SHUTDOWN.
 *
 *  Idempotent. Calling on an already-shut-down CQ is a no-op.
 *
 *  THREAD: shared (may be called from a thread other than the waiter)
 */
void cosmos_cq_shutdown(cosmos_cq_t* cq);

/*
 *  Drain one completion from the queue. Blocks up to timeout_ms milliseconds.
 *
 *  Return value:
 *    COSMOS_OK        : *out_resp is a caller-owned response (must be freed
 *                       with cosmos_response_free). *out_user_data is the
 *                       opaque value the host passed to the submission.
 *    COSMOS_CANCELLED : *out_resp is NULL. *out_user_data is the opaque value
 *                       the host passed to the submission. Host MUST still
 *                       look up its per-op state and release it.
 *    COSMOS_ERROR     : *out_resp is a caller-owned response carrying the
 *                       error details. *out_user_data is the opaque value.
 *    COSMOS_TIMEOUT   : *out_resp and *out_user_data are both NULL. Nothing
 *                       to do; loop back.
 *    COSMOS_SHUTDOWN  : *out_resp and *out_user_data are both NULL. Exit loop.
 *
 *  THREAD: single-waiter (at most ONE thread may be inside this call per CQ
 *          at any time; this is a POC simplification documented in §I4 of
 *          INVARIANTS.md).
 *
 *  timeout_ms semantics:
 *    > 0  : wait up to that many ms
 *    == 0 : poll (return TIMEOUT immediately if nothing is ready)
 *    < 0  : wait forever (the host should generally avoid this and use a
 *           short positive timeout so cosmos_cq_shutdown is observed promptly)
 */
cosmos_status_t cosmos_cq_wait(
    cosmos_cq_t*        cq,
    int32_t             timeout_ms,
    cosmos_response_t** out_resp,
    void**              out_user_data);

/* ========================================================================== *
 *  Submissions
 *
 *  These functions are non-blocking. They register the work with the runtime,
 *  obtain an opaque op handle, and return it immediately. The completion is
 *  delivered later via cosmos_cq_wait.
 * ========================================================================== */

/*
 *  Submit a read-item operation against (database, container, id, pk). The
 *  partition key is passed as a raw byte slice that the implementation will
 *  parse as a JSON value (POC convention: callers pass `"\"p\""` for a string
 *  PK named "p", as 4 bytes including the quotes).
 *
 *  user_data is BYTE-OPAQUE to the implementation: it is never dereferenced,
 *  never inspected, and is round-tripped to the matching completion via
 *  cosmos_cq_wait's out_user_data slot. The host owns the lifetime of
 *  whatever user_data refers to and is responsible for releasing it when
 *  the completion is observed (or when the op is cancelled).
 *
 *  Returns:
 *    non-NULL cosmos_op_t* : submission accepted; the op WILL produce
 *                            exactly one completion on `cq` (one of
 *                            OK / ERROR / CANCELLED). The handle must be
 *                            freed by calling cosmos_op_release (see below)
 *                            OR by passing it to cosmos_cancel; both options
 *                            release the host's ref to the handle.
 *    NULL                   : submission rejected synchronously (e.g., CQ has
 *                            been shut down, allocation failure, bad arg).
 *                            *out_err is populated. NO completion will be
 *                            delivered. The host MUST release any per-op
 *                            state it had stashed against user_data here,
 *                            before returning to its caller.
 *
 *  All input strings/byte buffers are BORROWED for the duration of this call.
 *  cosmos_error_t.message_utf8 (on failure) is borrowed per cosmos_error_t docs.
 *
 *  OWNERSHIP: returned op handle is caller-owned; release by cosmos_op_release
 *             or cosmos_cancel. The completion delivered on the CQ is a
 *             separate allocation (cosmos_response_t).
 *  THREAD:    shared
 */
cosmos_op_t* cosmos_read_item(
    cosmos_driver_t* drv,
    const char*      database_utf8,
    const char*      container_utf8,
    const char*      item_id_utf8,
    const uint8_t*   pk_bytes,
    size_t           pk_len,
    cosmos_cq_t*     cq,
    void*            user_data,
    cosmos_error_t*  out_err);

/* ========================================================================== *
 *  Op handle disposal
 *
 *  The host holds a ref to cosmos_op_t after submission solely so that it can
 *  call cosmos_cancel on the op. The implementation also holds a ref
 *  internally (so the tokio task can keep the op alive even after the host
 *  is done with it). When the host is finished with the op handle without
 *  wanting to cancel, it releases its ref via cosmos_op_release.
 *
 *  Net effect: it is ALWAYS safe to call exactly one of {cosmos_cancel,
 *  cosmos_op_release} on every op handle returned by a successful submission.
 *  It is a BUG to call neither (leaks the handle ref) or to call both.
 * ========================================================================== */

/*
 *  Release the host's reference to an op handle without cancelling. Safe to
 *  call after the completion has been observed.
 *
 *  THREAD: shared (but exclusive per-op: do not call concurrently with
 *          cosmos_cancel on the same op)
 */
void cosmos_op_release(cosmos_op_t* op);

/*
 *  Request cancellation of an in-flight op. Always causes EXACTLY ONE
 *  completion to be delivered for this op:
 *    - If the op had not yet produced a completion, the completion will be
 *      delivered with status COSMOS_CANCELLED, body NULL.
 *    - If the op had already produced a completion (race), this call has no
 *      effect on the queue contents; the op is just released.
 *
 *  cosmos_cancel ALSO releases the host's ref to the op handle (do NOT also
 *  call cosmos_op_release on this op after cancelling).
 *
 *  Passing NULL is a no-op (defensive).
 *
 *  THREAD: shared
 */
void cosmos_cancel(cosmos_op_t* op);

/* ========================================================================== *
 *  Response accessors + free
 *
 *  Response objects are returned from cosmos_cq_wait when status is OK or
 *  ERROR. Pointers returned by the accessors are valid only until
 *  cosmos_response_free is called on the parent response.
 * ========================================================================== */

/*
 *  HTTP status code attached to the response (e.g., 200, 404, 429, 500).
 *  Returns 0 if `resp` is NULL.
 *
 *  THREAD: shared (immutable after publication)
 */
int32_t cosmos_response_status(const cosmos_response_t* resp);

/*
 *  Body bytes of the response. The returned pointer is valid only until
 *  cosmos_response_free(resp); the caller MUST NOT free or modify it.
 *  Writes the byte length into *out_len.
 *
 *  Returns NULL with *out_len = 0 if `resp` is NULL or the body is empty.
 *
 *  OWNERSHIP: borrowed (callee retains; caller MUST NOT free)
 *  THREAD:    shared (immutable after publication)
 */
const uint8_t* cosmos_response_body(const cosmos_response_t* resp, size_t* out_len);

/*
 *  Free a response returned by cosmos_cq_wait. Passing NULL is a no-op.
 *
 *  THREAD: exclusive per-response (do not call concurrently with the
 *          accessors above on the same response)
 */
void cosmos_response_free(cosmos_response_t* resp);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* COSMOS_ASYNC_POC_H */
