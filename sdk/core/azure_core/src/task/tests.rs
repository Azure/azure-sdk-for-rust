// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;
use futures::FutureExt;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[tokio::test]
async fn test_task_execution() {
    let spawner = new_task_spawner();
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
    handle
        .wait()
        .await
        .expect("Task should complete successfully");

    // Verify the task executed
    assert!(*result.lock().unwrap());
}

#[tokio::test]
async fn test_multiple_tasks() {
    let spawner = new_task_spawner();
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
        handle
            .wait()
            .await
            .expect("Task should complete successfully");
    }
    // Verify all tasks executed
    assert_eq!(*counter.lock().unwrap(), 5);
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn test_tokio_specific_handling() {
    let spawner = Arc::new(tokio_spawn::TokioSpawner);
    let task_completed = Arc::new(Mutex::new(false));
    let task_completed_clone = Arc::clone(&task_completed);

    let handle = spawner.spawn(
        async move {
            *task_completed_clone.lock().unwrap() = true;
        }
        .boxed(),
    );

    handle
        .wait()
        .await
        .expect("Task should complete successfully");
    assert!(*task_completed.lock().unwrap());
}

#[cfg(not(feature = "tokio"))]
#[tokio::test]
async fn std_specific_handling() {
    let spawner = Arc::new(standard_spawn::StdSpawner);
    let task_completed = Arc::new(Mutex::new(false));
    let task_completed_clone = Arc::clone(&task_completed);

    let handle = spawner.spawn(
        async move {
            *task_completed_clone.lock().unwrap() = true;
        }
        .boxed(),
    );

    // For std threads, we need to wait for the task to complete
    tokio::time::sleep(Duration::from_millis(100)).await;
    handle
        .wait()
        .await
        .expect("Task should complete successfully");
    assert!(*task_completed.lock().unwrap());
}

#[cfg(not(feature = "tokio"))]
#[tokio::test]
async fn std_multiple_tasks() {
    let spawner = Arc::new(standard_spawn::StdSpawner);
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
    // Wait for all tasks
    for handle in handles {
        handle
            .wait()
            .await
            .expect("Task should complete successfully");
    }
    // Verify all tasks executed
    assert_eq!(*counter.lock().unwrap(), 5);
}

#[cfg(not(feature = "tokio"))]
#[tokio::test]
async fn std_task_execution() {
    let spawner = Arc::new(standard_spawn::StdSpawner);
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
    handle
        .wait()
        .await
        .expect("Task should complete successfully");

    // Verify the task executed
    assert!(*result.lock().unwrap());
}
