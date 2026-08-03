# Extend `AmqpReceiverApis` with Modified and annotated Rejected dispositions (#4935)

Designed by Claude Fable 5, revised once to add live tests. Implemented by Claude Opus 4.8.

## Understanding

`azure_core_amqp` can settle a received delivery with only three bare outcomes: accept, reject with no error, and release. Service Bus needs two more shapes. Abandon and defer need the AMQP `Modified` outcome with its three fields. Dead letter needs the `Rejected` outcome with a full described error (condition, description, and an info map). This task adds those shapes without breaking the crate's stable public API.

Done means issue #4943 can rewrite `abandon_message`, `defer_message`, and `dead_letter_message` in `azure_messaging_servicebus` with no further change to `azure_core_amqp`. The crate must be able to send:

- Abandon: `Modified { delivery_failed: false, undeliverable_here: false, message_annotations }`
- Defer: `Modified { delivery_failed: false, undeliverable_here: true, message_annotations }`
- Dead letter: `Rejected` with condition `com.microsoft:dead-letter` and an info map carrying `DeadLetterReason` and `DeadLetterErrorDescription`

Out of scope: rewriting the Service Bus settlement methods (that is #4943), any change to `azure_messaging_eventhubs`, playback-mode AMQP tests (the test proxy is HTTP-only, so every AMQP test in this repo is `#[recorded::test(live)]`), and removal of the existing three trait methods.

## Findings

- `AmqpReceiverApis` exposes only `accept_delivery`, `reject_delivery`, `release_delivery` (`sdk/core/azure_core_amqp/src/receiver.rs:104-110`). `reject_delivery` passes `None` for the error (`src/fe2o3/receiver.rs:157`).
- The crate is GA at 1.1.0, working version 1.2.0-beta.1. The semver CI gate is short-circuited (`eng/scripts/Test-Semver.ps1:14`, issue #4585), but the compatibility contract holds.
- Four implementors. Three in-crate: `AmqpReceiver`, `Fe2o3AmqpReceiver`, `NoopAmqpReceiver`. One in another published crate: `RecoverableReceiver` (`sdk/eventhubs/azure_messaging_eventhubs/src/common/recoverable/receiver.rs:52`), whose settlement methods are all `unimplemented!()`. A required new method would break it, so any new method needs a default body.
- fe2o3-amqp 0.14.0 (the pinned version) already offers `Receiver::reject(delivery, Option<Error>)` and `Receiver::modify(delivery, Modified)`.
- Building blocks exist: `AmqpDescribedError` with condition, description, info; `AmqpErrorCondition` is `#[non_exhaustive]`; a conversion from `AmqpOrderedMap` to fe2o3 `Fields`. The incoming error conversion exists; the outgoing direction did not.
- The issue text asks only for condition plus description on `Rejected`. That is not enough. .NET and Go both put the dead letter reason and description in the error's info map, so the info map is required or #4943 needs a second core change.

## Approach

One new public enum plus one defaulted trait method.

`AmqpDeliveryOutcome` has `Accepted`, `Rejected(Option<AmqpDescribedError>)`, `Released`, and `Modified { delivery_failed, undeliverable_here, message_annotations }`.

`settle_delivery(&self, delivery, outcome)` carries a default body that forwards the classic outcomes to the three existing methods and returns an error for the two new shapes. Existing external implementors keep compiling and get a loud failure rather than silent data loss. `Fe2o3AmqpReceiver` overrides it with the real frames. `AmqpErrorCondition` gains a `DeadLetter` variant, semver-minor on a `#[non_exhaustive]` enum.

Rejected alternatives: replacing the three methods with one outcome method (major break on a GA crate); required non-defaulted methods (breaks published `azure_messaging_eventhubs`); two narrow methods instead of the enum (the enum matches the AMQP model and mirrors `AmqpSendOutcome`); reusing `AmqpSendOutcome` (right shape, sender-specific name and docs).

Overturnable judgement calls: the enum is exhaustive because the AMQP spec closes the terminal outcome set; the default forwards rather than erroring on everything; the names `settle_delivery` and `AmqpDeliveryOutcome`; the `DeadLetter` variant is convenience, since `UnknownValue` already round-trips.

## Deviations found during implementation

Two things the design assumed turned out to be false, and both are recorded here because they change the plan.

1. **`AmqpDelivery` cannot be constructed in a unit test.** It is a newtype over the transport delivery with only a `pub(crate)` constructor, so the planned recording-implementor test could not compile. The routing decision was extracted into a pure function, `AmqpDeliveryOutcome::default_settlement_route`, returning a crate-private `DefaultSettlementRoute`. The default trait body matches on it, and the unit tests assert the routing directly. The semver-safety property stays covered without a live delivery.

2. **The live tests cannot live in `azure_messaging_servicebus`.** The workspace consumes `azure_core` and `azure_core_amqp` from crates.io (`Cargo.toml:59-67`, no `[patch]` section), while the local core crates path-depend on each other. Pointing the Service Bus crate at the local `azure_core_amqp` pulls in a second `azure_core`, and the two `azure_core::Error` types do not bridge, so the Service Bus library itself stops compiling. Making it work needs a cascading path override of `azure_core`, `azure_core_amqp`, `azure_core_test`, and `azure_identity` in the consumer manifest. That is out of proportion to this change, so the live tests are deferred. See the open question below.

## Risks

- `async_trait` default methods add `Self: Sync` bounds. Every workspace implementor is `Sync`.
- The fe2o3 conversion routes conditions through `ErrorCondition::Custom(Symbol)`, whose wire encoding matches the named variants. fe2o3 flags `Custom` as possibly removed later.
- The in-repo dependents build against the published `azure_core_amqp`, so compiling them does not actually exercise the trait change. The no-break claim rests on the defaulted-method design, not on a compile.
- No wire-level validation happened. The frames are unproven against a real broker until the live tests land.

## Steps

Steps 1 through 4 and 6 are complete. Step 5 is blocked.

1. **Done.** `AmqpDeliveryOutcome`, `DefaultSettlementRoute`, the defaulted `settle_delivery`, the `AmqpReceiver` delegation, the crate-root re-export, and two routing unit tests. Files: `src/receiver.rs`, `src/lib.rs`.
2. **Done.** `settle_delivery` for `Fe2o3AmqpReceiver` and `From<AmqpDescribedError> for fe2o3 Error`, plus two conversion unit tests. Files: `src/fe2o3/receiver.rs`, `src/fe2o3/error.rs`.
3. **Done.** The noop override. File: `src/noop.rs`.
4. **Done.** The `DeadLetter` condition across all four match blocks, with round-trip assertions. Files: `src/error/error_condition.rs`, `src/error/tests.rs`.
5. **Blocked.** Four `#[recorded::test(live)]` disposition tests. A complete draft exists and covers Modified redelivery with an incremented delivery count, Released without an increment, `undeliverable_here` stopping redelivery, and Rejected dead-lettering with the reason and description intact. It cannot compile anywhere until the dependency question above is settled.
6. **Done.** Changelog entries under 1.2.0-beta.1. File: `CHANGELOG.md`.

## Validation

Run and passing: `cargo fmt --all`; `cargo check -p azure_core_amqp`; `cargo check -p azure_core_amqp --no-default-features` (the only build that compiles `noop.rs`); `cargo clippy -p azure_core_amqp --all-targets --all-features` with zero warnings; `cargo test -p azure_core_amqp` with 114 unit tests and 3 doc tests green; `cargo check -p azure_messaging_eventhubs -p azure_messaging_servicebus`.

Not run: any live test, and therefore any wire-level proof.

Note for whoever lands step 5: `#[recorded::test]` bakes `CARGO_MANIFEST_DIR` at compile time, so these tests cannot run from inside a git worktree. The authoritative run needs the main checkout, a provisioned namespace, and `az login`.

## Open question

Where do the live disposition tests go, given they cannot go in `azure_messaging_servicebus`?

- Put them in `sdk/core/azure_core_amqp/tests/`, sending and receiving over raw AMQP with no Service Bus client. This needs a `tests/` directory, dev-dependencies on `azure_core_test` and `azure_identity`, and a decision about which service directory provisions the Service Bus namespace, because `sdk/core/` provisions no live Azure resources today.
- Or leave them for #4943, where the Service Bus crate will consume a released `azure_core_amqp` 1.2 and the dependency problem disappears.

## Docs to update

`sdk/core/azure_core_amqp/CHANGELOG.md` is updated. Doc comments on the new enum, its variants, and the new trait method are in the code. The crate README does not document individual receiver methods, so it needs no change.
