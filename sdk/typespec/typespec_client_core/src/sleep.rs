// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Sleep functions.

use crate::async_runtime::get_async_runtime;

/// Sleeps for the specified duration using the configured async runtime.
///
/// # Arguments
/// * `duration` - The duration to sleep for.
///
/// # Returns
/// A future that resolves when the sleep duration has elapsed.
///
/// # Example
/// ```
/// use typespec_client_core::sleep;
/// use std::time::Duration;
///
/// #[tokio::main]
/// async fn main() {
///     // Sleep for 1 second
///     sleep(Duration::from_secs(1)).await;
///     println!("Slept for 1 second");
/// }
/// ```
pub async fn sleep(duration: std::time::Duration) {
    get_async_runtime().sleep(duration).await
}
