# Release History

## 0.2.0 (Unreleased)

### Features Added

- Added initial support for an EventHubs processor.

  Note that as currently implemented, the processor is not very functional, since it requires that the customer provide an instance of a checkpoint store.

  For people who wish to play with the checkpoint store, there is an `InMemoryCheckpointStore` created (under the "test" feature) which can be used to experiment with the EventHubs processor.

- Removed the requirement that streaming messages from the `stream_events` method on the `EventReceiver` use `pin_mut!()` on the provided stream.
- Removed direct dependencies on `tokio` package.
- Added `partition_id` option to `SendMessageOptions`.

### Breaking Changes

- The stream returned by the `stream_events` API needs to be declared as mutable.

### Bugs Fixed

- If you call `send_event` or `send_message` with a specific target partition, the call now respects the desired target partition.

### Other Changes

## 0.1.0 (2025-02-18)

### Features Added

- Initial supported release.
