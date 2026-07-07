// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Cancellation contract harness — reference for host-language bindings
// (e.g. the .NET P/Invoke layer bridging `CancellationToken`).
//
// The native wrapper models cancellation as a *handle-based, cooperative*
// operation, NOT a field inside options:
//
//   * `cosmos_operation_handle_cancel(handle)` sets a cancel-requested
//     latch and wakes the submit task's `tokio::select!{ biased; ... }`
//     cancel branch (via a stored `Notify` permit, so a cancel that races
//     *ahead* of the task is still observed on its first poll).
//   * On cancel the task drops the in-flight driver future (best-effort
//     cooperative cancellation) and posts exactly one `CANCELLED`
//     completion, so the host continuation is always released — never
//     left `IN_FLIGHT`.
//   * The completion's `was_cancel_requested` field lets the host
//     distinguish "cancel won the race" from "cancel lost" (the op
//     completed first but a cancel was also requested).
//
// This harness covers the deterministic contracts a binding must rely on:
//
//   1. NULL-safety: cancel / state / free tolerate NULL.
//   2. Idempotency: multiple cancels on the same handle are safe.
//   3. End-to-end "cancel wins": submit a bootstrap (`get_or_create`)
//      against an unroutable black-hole endpoint so the driver future
//      can never complete on its own, cancel it, and drain a single
//      `CANCELLED` completion with `was_cancel_requested == true` and the
//      handle in a terminal `CANCELLED` state. This is the exact flow a
//      .NET `CancellationToken.Register(() => cosmos_operation_handle_cancel(h))`
//      bridge relies on.
//
// The "cancel loses" path (op completes before the cancel is observed,
// yielding OK/ERROR with `was_cancel_requested == true`) requires a real
// fast-completing operation and is exercised against the emulator in CI,
// not here — there is no network-free way to make a real op win
// deterministically.

#include "test_common.h"

// RFC 5737 TEST-NET-1. Documentation/black-hole range: routers drop these
// packets rather than answering or resetting, so a TCP connect to it stays
// pending until the OS connect timeout (tens of seconds). That guarantees
// the bootstrap future is still in flight when we cancel, making the
// "cancel wins" race deterministic without a real endpoint.
static const char *kBlackholeEndpoint = "https://192.0.2.1/";

// Well-known Cosmos emulator key — valid base64 so account-ref construction
// succeeds. The signature is never validated here because no request ever
// completes.
static const char *kEmulatorKey =
    "C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==";

// User-data cookie round-tripped through the (now `intptr_t`) ABI.
static const intptr_t kUserDataCookie = (intptr_t)0x00C0FFEE;

// ─────────────────────────────────────────────────────────────────────
// Section 1 — NULL-safety
// ─────────────────────────────────────────────────────────────────────

static int test_cancel_and_state_null_safe(void)
{
    int result = TEST_PASS;

    // Must not crash.
    cosmos_operation_handle_cancel(NULL);
    cosmos_operation_handle_free(NULL);
    ASSERT(1, "cancel(NULL) / free(NULL) are no-ops");

    // A NULL handle reports IN_FLIGHT (the conservative default).
    ASSERT(cosmos_operation_handle_state(NULL) ==
               COSMOS_OPERATION_HANDLE_STATE_IN_FLIGHT,
           "state(NULL) == IN_FLIGHT");

    // Freeing a zero-length / NULL completion run is a no-op.
    cosmos_completion_queue_free_completions(NULL, 0);
    return result;
}

// ─────────────────────────────────────────────────────────────────────
// Section 2 — end-to-end cancel-wins (and idempotency)
// ─────────────────────────────────────────────────────────────────────

// Builds a usable runtime + completion queue. Returns 0 on success and
// fills the out-params; on failure returns non-zero and the out-params are
// left NULL (caller skips the test).
static int make_runtime_and_cq(cosmos_runtime_t **out_runtime,
                               cosmos_completion_queue_t **out_cq)
{
    *out_runtime = NULL;
    *out_cq = NULL;

    // Best-effort identifier; failure here is non-fatal to the test intent.
    cosmos_runtime_options_t opts = cosmos_runtime_options_default();
    opts.user_agent_suffix = "cancel-c-tests";

    cosmos_runtime_t *runtime = NULL;
    cosmos_error_t *err = NULL;
    int32_t rc = cosmos_runtime_build(&opts, &runtime, &err);
    if (rc != COSMOS_ERROR_CODE_SUCCESS || runtime == NULL) {
        cosmos_error_free(err);
        return 1;
    }

    cosmos_completion_queue_t *cq = cosmos_completion_queue_create(runtime, NULL);
    if (cq == NULL) {
        cosmos_runtime_free(runtime);
        return 1;
    }

    *out_runtime = runtime;
    *out_cq = cq;
    return 0;
}

static int test_cancel_before_drain_yields_cancelled_completion(void)
{
    int result = TEST_PASS;
    cosmos_runtime_t *runtime = NULL;
    cosmos_completion_queue_t *cq = NULL;
    cosmos_account_ref_t *account = NULL;
    cosmos_operation_handle_t *handle = NULL;
    cosmos_completion_t completion;
    size_t drained = 0;

    if (make_runtime_and_cq(&runtime, &cq) != 0) {
        printf("    SKIP: could not build runtime/cq in this environment\n");
        return TEST_SKIP;
    }

    cosmos_error_t *acct_err = NULL;
    int32_t rc = cosmos_account_ref_with_master_key(
        kBlackholeEndpoint, kEmulatorKey, &account, &acct_err);
    REQUIRE(rc == COSMOS_ERROR_CODE_SUCCESS && account != NULL,
            "account_ref built for black-hole endpoint (rc=%d)", rc);
    cosmos_error_free(acct_err);

    // Submit a bootstrap. The driver future starts trying to fetch account
    // metadata against the unroutable endpoint and will never complete on
    // its own within the test window.
    cosmos_error_code_t pre = COSMOS_ERROR_CODE_SUCCESS;
    handle = cosmos_driver_get_or_create_submit(
        runtime, account, NULL, cq, kUserDataCookie, &pre);
    REQUIRE(handle != NULL && pre == COSMOS_ERROR_CODE_SUCCESS,
            "get_or_create_submit returned a handle (pre=%d)", pre);

    // Request cancellation. Idempotent — call it twice to prove that.
    cosmos_operation_handle_cancel(handle);
    cosmos_operation_handle_cancel(handle);

    // Drain exactly one completion. The biased cancel branch wins, so this
    // resolves promptly (the dropped connect future does not block).
    drained = cosmos_completion_queue_wait(cq, &completion, 1, 10000 /* ms */);
    REQUIRE(drained == 1,
            "a completion was delivered after cancel (no hang/leak)");

    // The contract: cancel won the race.
    ASSERT(completion.outcome == COSMOS_COMPLETION_OUTCOME_CANCELLED,
           "completion outcome == CANCELLED (=%d)", completion.outcome);
    ASSERT(completion.status == COSMOS_ERROR_CODE_OPERATION_CANCELLED,
           "completion status == OPERATION_CANCELLED (=%d)", completion.status);
    ASSERT(completion.was_cancel_requested == 1,
           "was_cancel_requested == true");

    // The opaque cookie round-trips verbatim (intptr_t contract).
    ASSERT(completion.user_data == kUserDataCookie,
           "user_data cookie round-trips (got 0x%llx)",
           (unsigned long long)completion.user_data);

    // The producing handle is in a terminal CANCELLED state — never left
    // IN_FLIGHT.
    ASSERT(cosmos_operation_handle_state(handle) ==
               COSMOS_OPERATION_HANDLE_STATE_CANCELLED,
           "handle state == CANCELLED (=%d)",
           cosmos_operation_handle_state(handle));

    // Exactly one completion: a second non-blocking drain finds nothing.
    cosmos_completion_t extra;
    size_t extra_n = cosmos_completion_queue_wait(cq, &extra, 1, 0);
    ASSERT(extra_n == 0, "exactly one completion delivered for the submit");
    cosmos_completion_queue_free_completions(&extra, extra_n);

cleanup:
    cosmos_completion_queue_free_completions(&completion, drained);
    cosmos_operation_handle_free(handle);
    cosmos_account_ref_free(account);
    cosmos_completion_queue_free(cq);
    cosmos_runtime_free(runtime);
    return result;
}

TEST_SUITE_BEGIN("Cancellation — handle-based cooperative cancel contract")
TEST_REGISTER(cancel_and_state_null_safe)
TEST_REGISTER(cancel_before_drain_yields_cancelled_completion)
TEST_SUITE_END("Cancellation — handle-based cooperative cancel contract")
