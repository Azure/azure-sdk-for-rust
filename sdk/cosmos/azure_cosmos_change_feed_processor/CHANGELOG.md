# Release History

## 0.1.0 (Unreleased)

### Features Added

- Added continuous typed start/stop processing over prepared contexts with shared callbacks, independent capacity/concurrency controls, state snapshots, and retained shutdown evidence.

- Added independently credential-bound feed/lease container configurations, an async builder with an overall preparation deadline, and configured lease/bootstrap factories that reuse resolved resources without implicit credential fallback.

- Exposed shared equal-count balancing and confirmed-session handoff APIs; balancing behavior remains in the CFP engine.
- Exposed shared-core workload bootstrap plan, initial lease, readiness, and store APIs without a typed Cosmos SDK dependency.
- Added typed processing under an acquired Cosmos lease session with renewal during application handling and separate processing, maintenance, and release results.

- Added typed latest-version change-feed page reads with account-key or token authentication, explicit continuation resumption, and response metadata.
- Added independent complete event models, AVAD/range-scoped reads, raw page context, and bounded single-lease handling with conditional checkpoint receipts and explicit stop/loss outcomes.

### Breaking Changes

- `ChangeFeedItem::current` returns `Option<&T>` to support delete events without a current document.

### Bugs Fixed

### Other Changes
