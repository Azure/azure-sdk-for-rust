# Release History

## 1.0.0 (Unreleased)

### Features Added

- Stable release of features from 0.1.0

### Breaking Changes

- Removed the `UserDelegationKey` re-export; import it from `azure_storage_common::models` directly.

### Bugs Fixed

- Fixed SAS generation for blob and directory paths containing backslashes by normalizing them to forward slashes in the signed resource.

## 0.1.0 (2026-07-14)

### Features Added

- Initial supported release.
