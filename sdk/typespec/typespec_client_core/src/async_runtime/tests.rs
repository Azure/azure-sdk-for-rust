// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;
use futures::FutureExt;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[cfg(not(feature = "tokio"))]
#[test]
fn test_task_spawner_execution() {
    let runtime = get_async_runtime();
    let result = Arc::new(Mutex::new(false));
    let result_clone = Arc::clone(&result);

    let handle = runtime.spawn(
        async move {
            // Simulate some work
            crate::sleep::sleep(Duration::from_millis(50)).await;
            let mut value = result_clone.lock().unwrap();
            *value = true;
        }
        .boxed(),
    );

    futures::executor::block_on(handle).expect("Task should complete successfully");

    // Verify the task executed
    assert!(*result.lock().unwrap());
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn tokio_task_spawner_execution() {
    let async_runtime = get_async_runtime();
    let result = Arc::new(Mutex::new(false));
    let result_clone = Arc::clone(&result);

    let handle = async_runtime.spawn(
        async move {
            // Simulate some work
            crate::sleep::sleep(Duration::from_millis(50)).await;
            let mut value = result_clone.lock().unwrap();
            *value = true;
        }
        .boxed(),
    );

    handle.await.expect("Task should complete successfully");

    // Verify the task executed
    assert!(*result.lock().unwrap());
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn test_tokio_specific_handling() {
    let spawner = Arc::new(tokio_runtime::TokioRuntime);
    let task_completed = Arc::new(Mutex::new(false));
    let task_completed_clone = Arc::clone(&task_completed);

    let handle = spawner.spawn(
        async move {
            *task_completed_clone.lock().unwrap() = true;
        }
        .boxed(),
    );

    handle.await.expect("Task should complete successfully");
    assert!(*task_completed.lock().unwrap());
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn tokio_multiple_tasks() {
    let spawner = Arc::new(tokio_runtime::TokioRuntime);
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    // Spawn multiple tasks
    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = spawner.spawn(
            async move {
                let mut value = counter_clone.lock().unwrap();
                *value += 1;
            }
            .boxed(),
        );
        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        handle.await.expect("Task should complete successfully");
    }
    // Verify all tasks executed
    assert_eq!(*counter.lock().unwrap(), 5);
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn tokio_task_execution() {
    let spawner = Arc::new(tokio_runtime::TokioRuntime);
    let result = Arc::new(Mutex::new(false));
    let result_clone = Arc::clone(&result);

    let handle = spawner.spawn(
        async move {
            // Simulate some work
            crate::sleep::sleep(Duration::from_millis(50)).await;
            let mut value = result_clone.lock().unwrap();
            *value = true;
        }
        .boxed(),
    );

    // Wait for task completion
    handle.await.expect("Task should complete successfully");

    // Verify the task executed
    assert!(*result.lock().unwrap());
}

// When the "tokio" feature is enabled, the azure_core::sleep::sleep function uses tokio::time::sleep which requires a tokio runtime.
// When the "tokio" feature is not enabled, it uses std::thread::sleep which does not require a tokio runtime.
#[test]
fn std_specific_handling() {
    let spawner = Arc::new(standard_runtime::StdRuntime);
    let task_completed = Arc::new(Mutex::new(false));
    let task_completed_clone = Arc::clone(&task_completed);

    let handle = spawner.spawn(
        async move {
            *task_completed_clone.lock().unwrap() = true;
        }
        .boxed(),
    );

    // For std threads, we need to wait for the task to complete
    std::thread::sleep(Duration::from_millis(100));
    futures::executor::block_on(handle).expect("Task should complete successfully");
    assert!(*task_completed.lock().unwrap());
}

#[test]
fn std_multiple_tasks() {
    let spawner = Arc::new(standard_runtime::StdRuntime);
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    // Spawn multiple tasks
    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = spawner.spawn(
            async move {
                let mut value = counter_clone.lock().unwrap();
                *value += 1;
            }
            .boxed(),
        );
        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        futures::executor::block_on(handle).expect("Task should complete successfully");
    }
    // Verify all tasks executed
    assert_eq!(*counter.lock().unwrap(), 5);
}

// When the "tokio" feature is enabled, the azure_core::sleep::sleep function uses tokio::time::sleep which requires a tokio runtime.
// When the "tokio" feature is not enabled, it uses std::thread::sleep which does not require a tokio runtime.
#[cfg(not(feature = "tokio"))]
#[test]
fn std_task_execution() {
    let runtime = Arc::new(standard_runtime::StdRuntime);
    let result = Arc::new(Mutex::new(false));
    let result_clone = Arc::clone(&result);

    let handle = runtime.spawn(
        async move {
            // Simulate some work
            crate::sleep::sleep(Duration::from_millis(500)).await;
            let mut value = result_clone.lock().unwrap();
            *value = true;
        }
        .boxed(),
    );

    // Wait for task completion
    futures::executor::block_on(handle).expect("Task should complete successfully");

    // Verify the task executed
    assert!(*result.lock().unwrap());
}

// Basic test that launches 10k futures and waits for them to complete:
// it has a high chance of failing if there is a race condition in the sleep method;
// otherwise, it runs quickly.
#[cfg(not(feature = "tokio"))]
#[tokio::test]
async fn test_timeout() {
    use super::*;
    use std::time::Duration;
    use tokio::task::JoinSet;

    let async_runtime = get_async_runtime();
    let mut join_set = JoinSet::default();
    let total = 10000;
    for _i in 0..total {
        let runtime = async_runtime.clone();
        join_set.spawn(async move {
            runtime.sleep(Duration::from_millis(10)).await;
        });
    }

    loop {
        let res =
            tokio::time::timeout(std::time::Duration::from_secs(10), join_set.join_next()).await;
        assert!(res.is_ok());
        if let Ok(None) = res {
            break;
        }
    }
}

#[tokio::test]
async fn test_sleep() {
    let runtime = get_async_runtime();
    let start = std::time::Instant::now();
    runtime.sleep(Duration::from_millis(100)).await;
    let elapsed = start.elapsed();
    assert!(elapsed >= Duration::from_millis(100));
}

#[test]
fn test_get_runtime() {
    // Ensure that the runtime can be retrieved without panicking
    let _runtime = get_async_runtime();
}

struct DummyRuntime;

impl AsyncRuntime for DummyRuntime {
    fn spawn(&self, _f: TaskFuture) -> SpawnedTask {
        unimplemented!("DummyRuntime does not support spawning tasks");
    }

    fn sleep(
        &self,
        _duration: std::time::Duration,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
        unimplemented!("DummyRuntime does not support sleeping");
    }
}

// This test is ignored because by default, cargo test runs all tests in parallel, but
// this test sets the runtime, which will fail if run in parallel with other tests that
// get the runtime.
#[test]
#[ignore = "Skipping the runtime set test to avoid conflicts with parallel test execution"]
fn test_set_runtime() {
    let runtime = Arc::new(DummyRuntime);
    // Ensure that the runtime can be set without panicking
    set_async_runtime(runtime.clone()).unwrap();

    // Ensure that setting the runtime again fails
    set_async_runtime(runtime.clone()).unwrap_err();
}
