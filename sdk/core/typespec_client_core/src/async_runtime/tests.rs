// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;
use crate::time::Duration;
use std::sync::{Arc, Mutex};

#[cfg(not(feature = "tokio"))]
#[test]
fn test_task_spawner_execution() {
    let runtime = get_async_runtime();
    let result = Arc::new(Mutex::new(false));
    let result_clone = Arc::clone(&result);

    let handle = runtime.spawn(Box::pin(async move {
        // Simulate some work
        crate::sleep::sleep(Duration::milliseconds(50)).await;
        let mut value = result_clone.lock().unwrap();
        *value = true;
    }));

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

    let handle = async_runtime.spawn(Box::pin(async move {
        // Simulate some work
        crate::sleep::sleep(Duration::milliseconds(50)).await;
        let mut value = result_clone.lock().unwrap();
        *value = true;
    }));

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

    let handle = spawner.spawn(Box::pin(async move {
        *task_completed_clone.lock().unwrap() = true;
    }));

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
        let handle = spawner.spawn(Box::pin(async move {
            let mut value = counter_clone.lock().unwrap();
            *value += 1;
        }));
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

    let handle = spawner.spawn(Box::pin(async move {
        // Simulate some work
        crate::sleep::sleep(Duration::milliseconds(50)).await;
        let mut value = result_clone.lock().unwrap();
        *value = true;
    }));

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

    let handle = spawner.spawn(Box::pin(async move {
        *task_completed_clone.lock().unwrap() = true;
    }));

    // For std threads, we need to wait for the task to complete
    std::thread::sleep(Duration::milliseconds(100).try_into().unwrap());
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
        let handle = spawner.spawn(Box::pin(async move {
            let mut value = counter_clone.lock().unwrap();
            *value += 1;
        }));
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

    let handle = runtime.spawn(Box::pin(async move {
        // Simulate some work
        crate::sleep::sleep(Duration::milliseconds(500)).await;
        let mut value = result_clone.lock().unwrap();
        *value = true;
    }));

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
    use crate::time::Duration;
    use tokio::task::JoinSet;

    let async_runtime = get_async_runtime();
    let mut join_set = JoinSet::default();
    let total = 10000;
    for _i in 0..total {
        let runtime = async_runtime.clone();
        join_set.spawn(async move {
            runtime.sleep(Duration::milliseconds(10)).await;
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
    runtime.sleep(Duration::milliseconds(100)).await;
    let elapsed = start.elapsed();
    assert!(elapsed >= Duration::milliseconds(100));
}

#[test]
fn test_get_runtime() {
    // Ensure that the runtime can be retrieved without panicking
    let _runtime = get_async_runtime();
}

struct TestRuntime;

impl AsyncRuntime for TestRuntime {
    fn spawn(&self, _f: TaskFuture) -> SpawnedTask {
        unimplemented!("TestRuntime does not support spawning tasks");
    }

    fn sleep(&self, _duration: Duration) -> TaskFuture {
        unimplemented!("TestRuntime does not support sleeping");
    }

    fn yield_now(&self) -> TaskFuture {
        unimplemented!("TestRuntime does not support yielding");
    }
}

// This test is ignored because by default, cargo test runs all tests in parallel, but
// this test sets the runtime, which will fail if run in parallel with other tests that
// get the runtime.
#[test]
#[ignore = "Skipping the runtime set test to avoid conflicts with parallel test execution"]
fn test_set_runtime() {
    let runtime = Arc::new(TestRuntime);
    // Ensure that the runtime can be set without panicking
    set_async_runtime(runtime.clone()).unwrap();

    // Ensure that setting the runtime again fails
    set_async_runtime(runtime.clone()).unwrap_err();
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn tokio_abort_cancels_task() {
    let spawner = Arc::new(tokio_runtime::TokioRuntime);
    let started = Arc::new(Mutex::new(false));
    let completed = Arc::new(Mutex::new(false));
    let started_clone = Arc::clone(&started);
    let completed_clone = Arc::clone(&completed);

    let handle = spawner.spawn(Box::pin(async move {
        *started_clone.lock().unwrap() = true;
        // Sleep long enough that abort will fire before completion
        crate::sleep::sleep(Duration::seconds(10)).await;
        *completed_clone.lock().unwrap() = true;
    }));

    // Give the task a moment to start
    crate::sleep::sleep(Duration::milliseconds(50)).await;
    assert!(*started.lock().unwrap(), "task should have started");

    handle.abort();

    // Awaiting the aborted task should yield an error (cancellation), not a successful completion.
    let result = handle.await;
    assert!(
        result.is_err(),
        "aborted task should return an error when awaited"
    );

    // The task should not have completed its work after being aborted
    assert!(
        !*completed.lock().unwrap(),
        "task should not have completed after abort"
    );
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn tokio_abort_then_await() {
    let spawner = Arc::new(tokio_runtime::TokioRuntime);

    let handle = spawner.spawn(Box::pin(async {
        crate::sleep::sleep(Duration::seconds(10)).await;
    }));

    // Give the task a moment to start
    crate::sleep::sleep(Duration::milliseconds(50)).await;

    handle.abort();

    // Awaiting an aborted task should resolve with a cancellation error (not hang)
    let result = handle.await;
    assert!(
        result.is_err(),
        "aborted task should return an error result"
    );
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn tokio_abort_already_completed_task() {
    let spawner = Arc::new(tokio_runtime::TokioRuntime);
    let completed = Arc::new(Mutex::new(false));
    let completed_clone = Arc::clone(&completed);

    let handle = spawner.spawn(Box::pin(async move {
        *completed_clone.lock().unwrap() = true;
    }));

    // Wait for the task to complete
    crate::sleep::sleep(Duration::milliseconds(50)).await;
    assert!(*completed.lock().unwrap());

    // Aborting an already-completed task should not panic
    handle.abort();
}

#[test]
fn std_abort_prevents_blocking() {
    let spawner = Arc::new(standard_runtime::StdRuntime);
    let completed = Arc::new(Mutex::new(false));
    let completed_clone = Arc::clone(&completed);

    let handle = spawner.spawn(Box::pin(async move {
        // Sleep long enough that abort will fire before completion
        std::thread::sleep(std::time::Duration::from_secs(10));
        *completed_clone.lock().unwrap() = true;
    }));

    // Give the task a moment to start on its thread
    std::thread::sleep(std::time::Duration::from_millis(50));

    // Abort should not panic and should mark the task as finished
    handle.abort();

    // Awaiting the aborted task should resolve immediately (not block for 10s)
    let result = futures::executor::block_on(handle);
    assert!(result.is_ok());

    // The task's long sleep may still be running on its thread, but the future resolved
    // without waiting for completion.
}

#[test]
fn std_abort_already_completed_task() {
    let spawner = Arc::new(standard_runtime::StdRuntime);
    let completed = Arc::new(Mutex::new(false));
    let completed_clone = Arc::clone(&completed);

    let handle = spawner.spawn(Box::pin(async move {
        *completed_clone.lock().unwrap() = true;
    }));

    // Wait for the task to complete
    std::thread::sleep(std::time::Duration::from_millis(100));
    assert!(*completed.lock().unwrap());

    // Aborting an already-completed task should not panic
    handle.abort();

    let result = futures::executor::block_on(handle);
    assert!(result.is_ok());
}

#[test]
fn std_abort_multiple_tasks() {
    let spawner = Arc::new(standard_runtime::StdRuntime);
    let mut handles = Vec::new();

    for _ in 0..5 {
        let handle = spawner.spawn(Box::pin(async {
            std::thread::sleep(std::time::Duration::from_secs(10));
        }));
        handles.push(handle);
    }

    // Give tasks a moment to start
    std::thread::sleep(std::time::Duration::from_millis(50));

    // Abort all tasks
    for handle in &handles {
        handle.abort();
    }

    // All aborted tasks should resolve without blocking
    for handle in handles {
        let result = futures::executor::block_on(handle);
        assert!(result.is_ok());
    }
}
