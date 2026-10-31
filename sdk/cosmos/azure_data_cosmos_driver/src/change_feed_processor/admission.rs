// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use futures::task::AtomicWaker;
use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex, MutexGuard},
    task::{Context, Poll},
};

#[derive(Debug, Default)]
struct State {
    maximum: u32,
    outstanding: u32,
    waiting: VecDeque<Arc<AtomicWaker>>,
}

impl State {
    fn available(&self) -> bool {
        self.outstanding < self.maximum
    }

    fn next(&self) -> Option<Arc<AtomicWaker>> {
        self.available()
            .then(|| self.waiting.front().cloned())
            .flatten()
    }
}

#[derive(Debug, Default)]
pub(crate) struct BatchAdmission {
    state: Mutex<State>,
}

impl BatchAdmission {
    #[cfg(test)]
    pub(super) fn counts(&self) -> (u32, usize) {
        let state = self.state();
        (state.outstanding, state.waiting.len())
    }

    fn state(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|err| {
            tracing::error!("Recovering poisoned batch admission");
            err.into_inner()
        })
    }

    pub(super) fn observe_limit(&self, limit: u32) {
        let next = {
            let mut state = self.state();
            state.maximum = state.maximum.max(limit);
            state.next()
        };
        wake(next);
    }

    pub(super) fn acquire(self: &Arc<Self>) -> AdmissionWait {
        let waiter = Arc::new(AtomicWaker::new());
        self.state().waiting.push_back(waiter.clone());
        AdmissionWait {
            admission: self.clone(),
            waiter,
            acquired: false,
        }
    }
}

fn wake(next: Option<Arc<AtomicWaker>>) {
    if let Some(next) = next {
        next.wake();
    }
}

pub(super) struct AdmissionWait {
    admission: Arc<BatchAdmission>,
    waiter: Arc<AtomicWaker>,
    acquired: bool,
}

impl Future for AdmissionWait {
    type Output = BatchPermit;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.waiter.register(cx.waker());
        let next = {
            let mut state = self.admission.state();
            if !state.available()
                || !state
                    .waiting
                    .front()
                    .is_some_and(|first| Arc::ptr_eq(first, &self.waiter))
            {
                return Poll::Pending;
            }
            state.waiting.pop_front();
            state.outstanding += 1;
            state.next()
        };
        self.acquired = true;
        wake(next);
        Poll::Ready(BatchPermit {
            admission: self.admission.clone(),
        })
    }
}

impl Drop for AdmissionWait {
    fn drop(&mut self) {
        if !self.acquired {
            let next = {
                let mut state = self.admission.state();
                state
                    .waiting
                    .retain(|entry| !Arc::ptr_eq(entry, &self.waiter));
                state.next()
            };
            wake(next);
        }
    }
}

pub(super) struct BatchPermit {
    admission: Arc<BatchAdmission>,
}

impl Drop for BatchPermit {
    fn drop(&mut self) {
        let next = {
            let mut state = self.admission.state();
            state.outstanding -= 1;
            state.next()
        };
        wake(next);
    }
}

#[cfg(test)]
mod tests;
