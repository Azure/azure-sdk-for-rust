// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{AdmissionWait, BatchAdmission, BatchPermit};
use futures::{
    task::{waker_ref, ArcWake},
    FutureExt,
};
use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    task::{Context, Poll},
};

#[derive(Default)]
struct Wakes(AtomicUsize);

impl ArcWake for Wakes {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.0.fetch_add(1, Ordering::SeqCst);
    }
}

fn take(wait: AdmissionWait) -> BatchPermit {
    wait.now_or_never().expect("next FIFO turn")
}

#[test]
fn ten_thousand_leases_take_fifo_turns_under_a_small_shared_bound() {
    let admission = Arc::new(BatchAdmission::default());
    admission.observe_limit(3);
    let mut waiting: VecDeque<_> = (0..10_000).map(|id| (id, admission.acquire())).collect();
    let mut active = VecDeque::new();
    for _ in 0..3 {
        let (id, wait) = waiting.pop_front().unwrap();
        active.push_back((id, take(wait)));
    }
    for expected in 0..20_000 {
        assert_eq!(admission.state().outstanding, 3);
        assert_eq!(admission.state().waiting.len(), 9_997);
        let (id, permit) = active.pop_front().unwrap();
        assert_eq!(id, expected % 10_000);
        drop(permit);
        assert_eq!(admission.state().outstanding, 2);
        waiting.push_back((id, admission.acquire()));
        let (id, wait) = waiting.pop_front().unwrap();
        active.push_back((id, take(wait)));
    }
    drop(waiting);
    drop(active);
    assert_eq!(admission.state().outstanding, 0);
    assert!(admission.state().waiting.is_empty());
}

#[test]
fn lower_and_repeated_worker_limits_preserve_the_highest_capacity() {
    let admission = Arc::new(BatchAdmission::default());
    admission.observe_limit(1);
    admission.observe_limit(3);
    admission.observe_limit(3);
    let mut permits: Vec<_> = (0..3).map(|_| take(admission.acquire())).collect();
    let mut waiting = Box::pin(admission.acquire());
    let wakes = Arc::new(Wakes::default());
    let waker = waker_ref(&wakes);
    let mut cx = Context::from_waker(&waker);
    assert!(waiting.as_mut().poll(&mut cx).is_pending());
    admission.observe_limit(1);
    assert!(waiting.as_mut().poll(&mut cx).is_pending());
    drop(permits.pop());
    assert!(wakes.0.load(Ordering::SeqCst) > 0);
    let Poll::Ready(permit) = waiting.as_mut().poll(&mut cx) else {
        panic!("the highest capacity should remain available");
    };
    drop(permit);
    drop(permits);
    assert_eq!(admission.state().outstanding, 0);
}

#[test]
fn cancellation_removes_front_and_tail_and_wakes_the_next_turn() {
    let admission = Arc::new(BatchAdmission::default());
    admission.observe_limit(1);
    let front = admission.acquire();
    let mut next = admission.acquire();
    let tail = admission.acquire();
    let wakes = Arc::new(Wakes::default());
    let waker = waker_ref(&wakes);
    let mut cx = Context::from_waker(&waker);
    assert!(Pin::new(&mut next).poll(&mut cx).is_pending());
    drop(tail);
    drop(front);
    assert!(wakes.0.load(Ordering::SeqCst) > 0);
    let permit = take(next);
    assert!(admission.state().waiting.is_empty());
    drop(permit);
    for _ in 0..10_000 {
        drop(admission.acquire());
    }
    assert!(admission.state().waiting.is_empty());
    assert_eq!(admission.state().outstanding, 0);
}

#[test]
fn increasing_capacity_wakes_waiters_and_other_drivers_are_independent() {
    let first = Arc::new(BatchAdmission::default());
    let second = Arc::new(BatchAdmission::default());
    first.observe_limit(1);
    second.observe_limit(1);
    let first_permit = take(first.acquire());
    let other_permit = take(second.acquire());
    let mut waiting = first.acquire();
    let wakes = Arc::new(Wakes::default());
    let waker = waker_ref(&wakes);
    let mut cx = Context::from_waker(&waker);
    assert!(Pin::new(&mut waiting).poll(&mut cx).is_pending());
    first.observe_limit(2);
    assert!(wakes.0.load(Ordering::SeqCst) > 0);
    let second_permit = take(waiting);
    drop((first_permit, second_permit, other_permit));
    assert_eq!(first.state().outstanding, 0);
    let permits = [take(first.acquire()), take(first.acquire())];
    assert!(first.acquire().now_or_never().is_none());
    drop(permits);
    assert_eq!(second.state().outstanding, 0);
}
