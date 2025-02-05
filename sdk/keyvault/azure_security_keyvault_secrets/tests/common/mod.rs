// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{ops::Mul, pin::Pin, time::Duration};
use tokio::time::{sleep, Sleep};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60 * 3);
const MAX_DURATION: Duration = Duration::from_secs(30);

/// Use in a retry loop.
#[derive(Debug)]
pub struct Retry(RetryImpl);

#[derive(Debug)]
enum RetryImpl {
    Immediate,
    Progressive {
        duration: Duration,
        timeout: Pin<Box<Sleep>>,
    },
}

impl Retry {
    /// Creates a retry that returns immediately.
    ///
    /// Useful when playing back recorded tests.
    pub fn immediate() -> Self {
        Self(RetryImpl::Immediate)
    }

    /// Creates a progressive retry that starts at 500 milliseconds and doubles up until 30 seconds.
    ///
    /// Useful when recording tests.
    ///
    /// # Arguments
    ///
    /// * `timeout` - How long until the retry times out. If `None`, the default of 3 minutes is used.
    ///   This only affects calls to [`Retry::next()`]. Actual timeout may be a little longer depending on your workload.
    pub fn progressive(timeout: Option<Duration>) -> Self {
        let timeout = Box::pin(sleep(timeout.unwrap_or(DEFAULT_TIMEOUT)));
        Self(RetryImpl::Progressive {
            duration: Duration::from_millis(500),
            timeout,
        })
    }

    /// Waits on the next retry interval or returns `None` if timed out.
    pub async fn next(&mut self) -> Option<()> {
        match &mut self.0 {
            RetryImpl::Immediate => Some(()),
            RetryImpl::Progressive { duration, timeout } => {
                tokio::select! {
                    _ = sleep(*duration) => {},
                    _ = timeout => {
                        return None;
                    },
                };

                *duration = duration.mul(2);
                if *duration > MAX_DURATION {
                    *duration = MAX_DURATION;
                }
                Some(())
            }
        }
    }

    /// Gets the current [`Duration`] for the next call to [`Retry::next()`].
    pub fn duration(&self) -> Option<Duration> {
        match &self.0 {
            RetryImpl::Immediate => None,
            RetryImpl::Progressive { duration, .. } => Some(*duration),
        }
    }
}

#[tokio::test]
async fn test_retry_immediate() {
    let mut i = 0;
    let mut retry = Retry::immediate();
    while (retry.next().await).is_some() {
        println!("Attempt {i}");
        i += 1;
        if i >= 5 {
            break;
        }
    }
}

#[ignore = "literal waste of time normally"]
#[tokio::test]
async fn test_retry_progressive() {
    let mut i = 0;
    let mut retry = Retry::progressive(Some(Duration::from_secs(2)));
    while (retry.next().await).is_some() {
        println!("Attempt {i}");
        i += 1;
        if i >= 5 {
            break;
        }
    }
}
