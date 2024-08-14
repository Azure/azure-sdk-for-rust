// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Sleep functions.

#[cfg(not(feature = "tokio_sleep"))]
mod thread;

#[cfg(not(feature = "tokio_sleep"))]
pub use self::thread::{sleep, Sleep};

#[cfg(feature = "tokio_sleep")]
pub use tokio::time::{sleep, Sleep};

// Unit tests
#[cfg(test)]
mod tests {

    // Basic test that launches 10k futures and waits for them to complete:
    // it has a high chance of failing if there is a race condition in the sleep method;
    // otherwise, it runs quickly.
    #[cfg(not(feature = "tokio+sleep"))]
    #[tokio::test]
    async fn test_timeout() {
        use super::*;
        use std::time::Duration;
        use tokio::task::JoinSet;

        let mut join_set = JoinSet::default();
        let total = 10000;
        for _i in 0..total {
            join_set.spawn(async move {
                sleep(Duration::from_millis(10)).await;
            });
        }

        loop {
            let res =
                tokio::time::timeout(std::time::Duration::from_secs(10), join_set.join_next())
                    .await;
            assert!(res.is_ok());
            if let Ok(None) = res {
                break;
            }
        }
    }
}
