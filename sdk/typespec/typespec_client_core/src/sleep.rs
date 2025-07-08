// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Sleep functions.

use crate::time::Duration;

#[cfg(not(target_family = "wasm"))]
use crate::async_runtime::get_async_runtime;

#[cfg(not(target_family = "wasm"))]
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
/// use typespec_client_core::{sleep, time::Duration};
///
/// #[tokio::main]
/// async fn main() {
///     // Sleep for 1 second
///     sleep(Duration::seconds(1)).await;
///     println!("Slept for 1 second");
/// }
/// ```
pub async fn sleep(duration: Duration) {
    get_async_runtime().sleep(duration).await
}

#[cfg(target_family = "wasm")]
pub async fn sleep(duration: Duration) {
    if let Ok(d) = duration.try_into() {
        gloo_timers::future::sleep(d).await;
    } else {
        // This means the duration is negative, don't sleep at all.
        return;
    }
}
