# Change log

## 0.1.2-alpha

- Fixed a bug where `EventStream` is not `Send` because `ClosingBoxedFuture` misses `Send` in its
  trait bounds
- Changed visibility of `EventStream` to public

## 0.1.1

- Fixed wrong crate name in the example

## 0.1.0

- Initial release
