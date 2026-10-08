# Release History

## 0.1.0 (Unreleased)

### Features Added

- Added managed continuous execution, saved-plan-first range discovery, source-compatible Now anchoring, lease-local recovery, and completion-bearing task ownership and shutdown.
- Added conditional same-generation checkpoint reconciliation that fences late writes before authorizing progress.
- Added periodic fenced logical-lease subdivision using driver-derived child checkpoints, atomic staging/activation, and restart recovery with retained parent evidence.

- Added local engine/lease/bootstrap constructors for prepared driver/container references with account/credential validation and RID-based source identity.

- Added decentralized equal-count logical-lease selection, complete paginated inventory, bounded jittered pickup cycles, and readiness/ETag/generation-fenced active transfer without checkpoint reset.
- Added bounded explicit-plan workload bootstrap with transactional initializer fencing, complete-coverage restart verification, preserved seed/checkpoint recovery, and readiness-gated work-lease acquisition.
- Added a Cosmos-backed pre-created lease store with ETag acquisition/takeover, coordinated renewal/checkpoint/release, an independent ownership-safety watchdog, and explicit maintenance/release outcomes.

- Added schema-agnostic latest-version change-feed page reads through the Cosmos driver, returning raw responses and continuation tokens.
- Added `ProcessorEngine` as the main engine type, retaining `ChangeFeedProcessorEngine` as a compatibility name.
- Added retained mode/range-scoped readers and a schema-agnostic single-lease coordinator with confirmed progress, checkpoint retries, ownership-loss signals, and bounded draining.
- Added refreshed topology planning that retains logical lease ranges and distinct durable checkpoints across physical splits and merges without unsafe token conversion.

### Breaking Changes

### Bugs Fixed

- Signal ownership loss immediately when an in-flight lease/bootstrap write is cancelled.
- Preserve matching local sessions during inventory scans spanning independent renewals.
- Validate renewal headroom for the preceding write, renewal delay, and following request.
- Classify sent checkpoint failures, including HTTP 412 without attempt provenance, as ambiguous.
- Return after the final confirmed idle batch without consuming the idle-poll delay.

- Surface unwinding application-callback panics as lease processing failures with recovery positions and diagnostics, without checkpointing or reading the next batch.

### Other Changes
