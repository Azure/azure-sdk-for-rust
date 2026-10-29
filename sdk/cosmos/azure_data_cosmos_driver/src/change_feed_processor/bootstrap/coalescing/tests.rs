// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    ChangeFeedProcessorBootstrapCoalescer, ChangeFeedProcessorBootstrapCoalescingCleanup,
    ChangeFeedProcessorBootstrapCoalescingEntry, ChangeFeedProcessorBootstrapCoalescingKey,
};
use crate::{
    change_feed_processor::bootstrap::{error, retry_bootstrap},
    models::ChangeFeedFactoryMode,
    Result,
};
use azure_core::http::StatusCode;
use futures::{
    channel::oneshot,
    future::{join_all, pending},
    poll, FutureExt,
};
use std::{
    mem::discriminant,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

fn key() -> ChangeFeedProcessorBootstrapCoalescingKey {
    ChangeFeedProcessorBootstrapCoalescingKey {
        namespace: "source-and-processor".into(),
        lease_container: "leases".into(),
        lease_driver: 1,
        mode: discriminant(&ChangeFeedFactoryMode::Incremental),
    }
}

fn attempt_deadline() -> Instant {
    Instant::now() + Duration::from_secs(30)
}

struct FactoryDropProbe {
    coalescer: Arc<ChangeFeedProcessorBootstrapCoalescer>,
    lock_available: Arc<AtomicBool>,
}

impl Drop for FactoryDropProbe {
    fn drop(&mut self) {
        self.lock_available
            .store(self.coalescer.entries.try_lock().is_ok(), Ordering::SeqCst);
    }
}

#[tokio::test]
async fn overlapping_waiters_share_one_execution() -> Result<()> {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let executions = Arc::new(AtomicUsize::new(0));
    let waiters: Vec<_> = (0..4)
        .map(|_| {
            let executions = Arc::clone(&executions);
            coalescer.start_or_join(key(), attempt_deadline(), move || {
                async move {
                    executions.fetch_add(1, Ordering::SeqCst);
                    Ok(true)
                }
                .boxed()
            })
        })
        .collect();
    assert_eq!(coalescer.active_count(), 1);
    for result in join_all(waiters).await {
        result?;
    }
    assert_eq!(executions.load(Ordering::SeqCst), 1);
    assert_eq!(coalescer.active_count(), 0);
    Ok(())
}

#[tokio::test]
async fn later_waiter_survives_initiating_caller_cancellation() -> Result<()> {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let (started_tx, started_rx) = oneshot::channel();
    let (release_tx, release_rx) = oneshot::channel();
    let first = coalescer.start_or_join(key(), attempt_deadline(), move || {
        async move {
            started_tx.send(()).unwrap();
            release_rx.await.unwrap();
            Ok(true)
        }
        .boxed()
    });
    let initiating_caller = tokio::spawn(first);
    started_rx.await.unwrap();
    tokio::time::sleep(Duration::from_millis(10)).await;
    let later = coalescer.start_or_join(key(), attempt_deadline(), move || {
        async { panic!("a later waiter must not start another bootstrap") }.boxed()
    });
    initiating_caller.abort();
    assert!(initiating_caller.await.unwrap_err().is_cancelled());
    assert_eq!(coalescer.active_count(), 1);
    release_tx.send(()).unwrap();
    later.await?;
    assert_eq!(coalescer.active_count(), 0);
    Ok(())
}

#[tokio::test]
async fn follower_cancellation_does_not_cancel_initiator() -> Result<()> {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let (started_tx, started_rx) = oneshot::channel();
    let (release_tx, release_rx) = oneshot::channel();
    let first = coalescer.start_or_join(key(), attempt_deadline(), move || {
        async move {
            started_tx.send(()).unwrap();
            release_rx.await.unwrap();
            Ok(true)
        }
        .boxed()
    });
    let initiating_caller = tokio::spawn(first);
    started_rx.await.unwrap();
    let follower = coalescer.start_or_join(key(), attempt_deadline(), move || {
        async { panic!("a follower must not start another bootstrap") }.boxed()
    });
    assert_eq!(coalescer.active_count(), 1);
    drop(follower);
    release_tx.send(()).unwrap();
    initiating_caller.await.unwrap()?;
    assert_eq!(coalescer.active_count(), 0);
    Ok(())
}

#[tokio::test]
async fn last_waiter_cancellation_releases_unpolled_work() {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let retained = Arc::new(());
    let weak = Arc::downgrade(&retained);
    let first = coalescer.start_or_join(key(), attempt_deadline(), move || {
        async move {
            let _retained = retained;
            pending::<Result<bool>>().await
        }
        .boxed()
    });
    let follower = coalescer.start_or_join(key(), attempt_deadline(), move || {
        async { panic!("a follower must not start another bootstrap") }.boxed()
    });
    drop(first);
    assert!(weak.upgrade().is_some());
    drop(follower);
    assert!(weak.upgrade().is_none());
    assert_eq!(coalescer.active_count(), 0);
}

#[tokio::test]
async fn errors_are_shared_and_later_calls_retry() -> Result<()> {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let first = coalescer.start_or_join(key(), attempt_deadline(), move || {
        async { Err(error(StatusCode::Conflict, "incompatible workload")) }.boxed()
    });
    let follower = coalescer.start_or_join(key(), attempt_deadline(), move || {
        async { panic!("a follower must observe the original error") }.boxed()
    });
    let (first, follower) = futures::join!(first, follower);
    let first = first.unwrap_err();
    let follower = follower.unwrap_err();
    assert_eq!(first.status().status_code(), StatusCode::Conflict);
    assert_eq!(follower.status(), first.status());
    assert_eq!(follower.to_string(), first.to_string());
    assert_eq!(coalescer.active_count(), 0);
    coalescer
        .start_or_join(key(), attempt_deadline(), move || {
            async { Ok(true) }.boxed()
        })
        .await?;
    assert_eq!(coalescer.active_count(), 0);
    Ok(())
}

#[tokio::test]
async fn successes_are_not_permanently_cached() -> Result<()> {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    coalescer
        .start_or_join(key(), attempt_deadline(), move || {
            async { Ok(true) }.boxed()
        })
        .await?;
    let error = coalescer
        .start_or_join(key(), attempt_deadline(), move || {
            async { Err(error(StatusCode::ServiceUnavailable, "new attempt")) }.boxed()
        })
        .await
        .unwrap_err();
    assert_eq!(error.status().status_code(), StatusCode::ServiceUnavailable);
    assert_eq!(coalescer.active_count(), 0);
    Ok(())
}

#[tokio::test]
async fn incompatible_keys_do_not_coalesce() -> Result<()> {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let executions = Arc::new(AtomicUsize::new(0));
    let original = key();
    let mut keys = vec![original.clone()];
    let mut other = original.clone();
    other.namespace = "another-source-or-processor".into();
    keys.push(other);
    let mut other = original.clone();
    other.lease_container = "another-lease-container".into();
    keys.push(other);
    let mut other = original.clone();
    other.lease_driver = 2;
    keys.push(other);
    let mut other = original;
    other.mode = discriminant(&ChangeFeedFactoryMode::FullFidelity);
    keys.push(other);
    let waiters: Vec<_> = keys
        .into_iter()
        .map(|key| {
            let executions = Arc::clone(&executions);
            coalescer.start_or_join(key, attempt_deadline(), move || {
                async move {
                    executions.fetch_add(1, Ordering::SeqCst);
                    Ok(true)
                }
                .boxed()
            })
        })
        .collect();
    assert_eq!(coalescer.active_count(), 5);
    for result in join_all(waiters).await {
        result?;
    }
    assert_eq!(executions.load(Ordering::SeqCst), 5);
    assert_eq!(coalescer.active_count(), 0);
    Ok(())
}

#[tokio::test]
async fn joining_does_not_extend_the_original_deadline() {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let first =
        coalescer.start_or_join(key(), Instant::now(), || pending::<Result<bool>>().boxed());
    let follower = coalescer.start_or_join(key(), attempt_deadline(), || {
        panic!("a joiner must not construct an attempt")
    });
    let results = tokio::time::timeout(Duration::from_secs(2), join_all([first, follower]))
        .await
        .expect("joining must not replace the original deadline");
    for result in results {
        assert_eq!(
            result.unwrap_err().status().status_code(),
            StatusCode::RequestTimeout
        );
    }
    assert_eq!(coalescer.active_count(), 0);
}

#[tokio::test]
async fn panics_release_waiters_and_allow_a_new_attempt() -> Result<()> {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let first = coalescer.start_or_join(key(), attempt_deadline(), move || {
        async { panic!("simulated bootstrap panic") }.boxed()
    });
    let follower = coalescer.start_or_join(key(), attempt_deadline(), move || {
        async { panic!("a follower must not run") }.boxed()
    });
    for result in join_all([first, follower]).await {
        assert_eq!(
            result.unwrap_err().status().status_code(),
            StatusCode::InternalServerError
        );
    }
    assert_eq!(coalescer.active_count(), 0);
    coalescer
        .start_or_join(key(), attempt_deadline(), move || {
            async { Ok(true) }.boxed()
        })
        .await?;
    Ok(())
}

#[test]
fn stale_cleanup_preserves_a_replacement_flight() {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let key = key();
    let old_cleanup = ChangeFeedProcessorBootstrapCoalescingCleanup {
        coalescer: Arc::clone(&coalescer),
        key: key.clone(),
        generation: Arc::new(()),
    };
    let replacement = async { Ok(true) }.boxed().shared();
    let generation = Arc::new(());
    coalescer.entries().insert(
        key.clone(),
        ChangeFeedProcessorBootstrapCoalescingEntry {
            generation: Arc::clone(&generation),
            future: replacement.downgrade().unwrap(),
        },
    );
    drop(old_cleanup);
    assert!(Arc::ptr_eq(
        &coalescer.entries().get(&key).unwrap().generation,
        &generation,
    ));
}

#[tokio::test]
async fn retryable_failure_re_registers_with_a_small_remaining_budget() -> Result<()> {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let executions = Arc::new(AtomicUsize::new(0));
    retry_bootstrap(
        || {
            let executions = executions.clone();
            coalescer.start_or_join(key(), attempt_deadline(), move || {
                async move {
                    if executions.fetch_add(1, Ordering::SeqCst) == 0 {
                        Err(error(StatusCode::ServiceUnavailable, "temporary failure"))
                    } else {
                        Ok(true)
                    }
                }
                .boxed()
            })
        },
        Instant::now() + Duration::from_secs(2),
        Duration::from_millis(1),
    )
    .await?;
    assert_eq!(executions.load(Ordering::SeqCst), 2);
    assert_eq!(coalescer.active_count(), 0);
    Ok(())
}

#[tokio::test]
async fn panic_re_registers_automatically() -> Result<()> {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let executions = Arc::new(AtomicUsize::new(0));
    retry_bootstrap(
        || {
            let executions = executions.clone();
            coalescer.start_or_join(key(), attempt_deadline(), move || {
                assert_ne!(
                    executions.fetch_add(1, Ordering::SeqCst),
                    0,
                    "first factory invocation panics"
                );
                async { Ok(true) }.boxed()
            })
        },
        Instant::now() + Duration::from_secs(2),
        Duration::from_millis(1),
    )
    .await?;
    assert_eq!(executions.load(Ordering::SeqCst), 2);
    assert_eq!(coalescer.active_count(), 0);
    Ok(())
}

#[tokio::test]
async fn permanent_failure_does_not_re_register() {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let executions = Arc::new(AtomicUsize::new(0));
    let result = retry_bootstrap(
        || {
            let executions = executions.clone();
            coalescer.start_or_join(key(), attempt_deadline(), move || {
                async move {
                    executions.fetch_add(1, Ordering::SeqCst);
                    Err(error(StatusCode::Conflict, "incompatible workload"))
                }
                .boxed()
            })
        },
        Instant::now() + Duration::from_secs(2),
        Duration::from_millis(1),
    )
    .await;
    assert_eq!(
        result.unwrap_err().status().status_code(),
        StatusCode::Conflict
    );
    assert_eq!(executions.load(Ordering::SeqCst), 1);
    assert_eq!(coalescer.active_count(), 0);
}

#[tokio::test]
async fn repeated_failures_do_not_reset_the_caller_deadline() {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let executions = Arc::new(AtomicUsize::new(0));
    let deadline = Instant::now() + Duration::from_millis(100);
    let result = tokio::time::timeout(
        Duration::from_secs(2),
        retry_bootstrap(
            || {
                let executions = executions.clone();
                coalescer.start_or_join(key(), attempt_deadline(), move || {
                    async move {
                        executions.fetch_add(1, Ordering::SeqCst);
                        Err(error(StatusCode::ServiceUnavailable, "persistent failure"))
                    }
                    .boxed()
                })
            },
            deadline,
            Duration::from_millis(5),
        ),
    )
    .await
    .expect("re-registration must not extend the original caller budget");
    assert_eq!(
        result.unwrap_err().status().status_code(),
        StatusCode::RequestTimeout
    );
    assert!(executions.load(Ordering::SeqCst) > 0);
    assert_eq!(coalescer.active_count(), 0);
}

#[tokio::test]
async fn expired_caller_never_registers_an_attempt() {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let mut registrations = 0;
    let result = retry_bootstrap(
        || {
            registrations += 1;
            coalescer.start_or_join(key(), attempt_deadline(), move || {
                async { Ok(true) }.boxed()
            })
        },
        Instant::now() - Duration::from_secs(1),
        Duration::from_millis(1),
    )
    .await;
    assert_eq!(
        result.unwrap_err().status().status_code(),
        StatusCode::RequestTimeout
    );
    assert_eq!(registrations, 0);
    assert_eq!(coalescer.active_count(), 0);
}

#[tokio::test]
async fn later_caller_re_registers_after_originating_deadline() -> Result<()> {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let first_deadline = Instant::now() + Duration::from_millis(100);
    let mut first = Box::pin(retry_bootstrap(
        || coalescer.start_or_join(key(), first_deadline, || pending::<Result<bool>>().boxed()),
        first_deadline,
        Duration::from_millis(1),
    ));
    assert!(poll!(first.as_mut()).is_pending());
    let executions = Arc::new(AtomicUsize::new(0));
    let mut later = Box::pin(retry_bootstrap(
        || {
            let executions = executions.clone();
            coalescer.start_or_join(key(), attempt_deadline(), move || {
                async move {
                    executions.fetch_add(1, Ordering::SeqCst);
                    Ok(true)
                }
                .boxed()
            })
        },
        Instant::now() + Duration::from_secs(2),
        Duration::from_millis(1),
    ));
    assert!(poll!(later.as_mut()).is_pending());
    assert_eq!(coalescer.active_count(), 1);
    let (first, later) = futures::join!(first, later);
    assert_eq!(
        first.unwrap_err().status().status_code(),
        StatusCode::RequestTimeout
    );
    later?;
    assert_eq!(executions.load(Ordering::SeqCst), 1);
    assert_eq!(coalescer.active_count(), 0);
    Ok(())
}

#[tokio::test]
async fn caller_timeout_does_not_cancel_another_waiter() -> Result<()> {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let (release_tx, release_rx) = oneshot::channel();
    let mut receiver = Some(release_rx);
    let mut survivor = Box::pin(retry_bootstrap(
        || {
            let receiver = receiver.take().expect("only one attempt should execute");
            coalescer.start_or_join(key(), attempt_deadline(), move || {
                async move {
                    receiver.await.unwrap();
                    Ok(true)
                }
                .boxed()
            })
        },
        Instant::now() + Duration::from_secs(2),
        Duration::from_millis(1),
    ));
    assert!(poll!(survivor.as_mut()).is_pending());
    let result = retry_bootstrap(
        || {
            coalescer.start_or_join(key(), attempt_deadline(), move || {
                async { panic!("a short-lived caller must join the running attempt") }.boxed()
            })
        },
        Instant::now() + Duration::from_millis(20),
        Duration::from_millis(1),
    )
    .await;
    assert_eq!(
        result.unwrap_err().status().status_code(),
        StatusCode::RequestTimeout
    );
    assert_eq!(coalescer.active_count(), 1);
    release_tx.send(()).unwrap();
    survivor.await?;
    assert_eq!(coalescer.active_count(), 0);
    Ok(())
}

#[tokio::test]
async fn service_internal_error_is_not_treated_as_a_caught_panic() {
    let mut registrations = 0;
    let result = retry_bootstrap(
        || {
            registrations += 1;
            async { Err(error(StatusCode::InternalServerError, "service failure")) }
        },
        Instant::now() + Duration::from_secs(2),
        Duration::from_millis(1),
    )
    .await;
    assert_eq!(
        result.unwrap_err().status().status_code(),
        StatusCode::InternalServerError
    );
    assert_eq!(registrations, 1);
}

#[tokio::test]
async fn selected_factory_is_lazy_and_runs_outside_the_registry_lock() -> Result<()> {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let constructions = Arc::new(AtomicUsize::new(0));
    let lock_available = Arc::new(AtomicBool::new(false));
    let retained = coalescer.clone();
    let observed_constructions = constructions.clone();
    let observed_lock = lock_available.clone();
    let first = coalescer.start_or_join(key(), attempt_deadline(), move || {
        observed_constructions.fetch_add(1, Ordering::SeqCst);
        observed_lock.store(retained.entries.try_lock().is_ok(), Ordering::SeqCst);
        async { Ok(true) }.boxed()
    });
    let follower = coalescer.start_or_join(key(), attempt_deadline(), || {
        panic!("joining must not invoke another attempt factory")
    });
    assert_eq!(constructions.load(Ordering::SeqCst), 0);
    assert_eq!(coalescer.active_count(), 1);
    for result in join_all([first, follower]).await {
        assert!(result?);
    }
    assert_eq!(constructions.load(Ordering::SeqCst), 1);
    assert!(lock_available.load(Ordering::SeqCst));
    assert_eq!(coalescer.active_count(), 0);
    Ok(())
}

#[tokio::test]
async fn constructor_panic_is_shared_without_poisoning_registration() -> Result<()> {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let constructions = Arc::new(AtomicUsize::new(0));
    let observed = constructions.clone();
    let first = coalescer.start_or_join(key(), attempt_deadline(), move || {
        observed.fetch_add(1, Ordering::SeqCst);
        panic!("attempt constructor panicked")
    });
    let follower = coalescer.start_or_join(key(), attempt_deadline(), || {
        panic!("a follower must not construct an attempt")
    });
    assert_eq!(constructions.load(Ordering::SeqCst), 0);
    for result in join_all([first, follower]).await {
        assert_eq!(
            result.unwrap_err().status().status_code(),
            StatusCode::InternalServerError
        );
    }
    assert_eq!(constructions.load(Ordering::SeqCst), 1);
    assert_eq!(coalescer.active_count(), 0);
    assert!(
        coalescer
            .start_or_join(key(), attempt_deadline(), || async { Ok(true) }.boxed())
            .await?
    );
    Ok(())
}

#[tokio::test]
async fn attempt_expiring_before_poll_does_not_invoke_its_factory() {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let constructions = Arc::new(AtomicUsize::new(0));
    let observed = constructions.clone();
    let attempt = coalescer.start_or_join(
        key(),
        Instant::now() + Duration::from_millis(10),
        move || {
            observed.fetch_add(1, Ordering::SeqCst);
            async { Ok(true) }.boxed()
        },
    );
    assert_eq!(constructions.load(Ordering::SeqCst), 0);
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert_eq!(
        attempt.await.unwrap_err().status().status_code(),
        StatusCode::RequestTimeout
    );
    assert_eq!(constructions.load(Ordering::SeqCst), 0);
    assert_eq!(coalescer.active_count(), 0);
}

#[tokio::test]
async fn unused_factory_is_dropped_without_the_registry_lock() {
    let coalescer = Arc::new(ChangeFeedProcessorBootstrapCoalescer::default());
    let first = coalescer.start_or_join(key(), attempt_deadline(), || {
        pending::<Result<bool>>().boxed()
    });
    let lock_available = Arc::new(AtomicBool::new(false));
    let probe = FactoryDropProbe {
        coalescer: coalescer.clone(),
        lock_available: lock_available.clone(),
    };
    let follower = coalescer.start_or_join(key(), attempt_deadline(), move || {
        drop(probe);
        panic!("an unused factory must be dropped, not invoked")
    });
    assert!(lock_available.load(Ordering::SeqCst));
    drop(first);
    drop(follower);
    assert_eq!(coalescer.active_count(), 0);
}
