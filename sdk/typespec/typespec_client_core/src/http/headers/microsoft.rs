// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;

pub const ERROR_CODE: HeaderName = HeaderName::from_static("x-ms-error-code");
pub(crate) const RETRY_AFTER_MS: HeaderName = HeaderName::from_static("retry-after-ms");
pub const X_MS_RETRY_AFTER_MS: HeaderName = HeaderName::from_static("x-ms-retry-after-ms");
