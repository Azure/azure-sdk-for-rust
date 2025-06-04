// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Sleep functions.

use crate::get_async_runtime;

pub async fn sleep(duration: std::time::Duration) {
    get_async_runtime().sleep(duration).await
}
