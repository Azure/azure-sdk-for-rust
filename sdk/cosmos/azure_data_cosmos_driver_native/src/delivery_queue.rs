// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    completion::{CosmosCompletionQueueState, CqOptions},
    error::CosmosErrorCode,
    safety::MutexExt,
};
use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
    time::{Duration, Instant},
};

pub(crate) trait QueuedDelivery {
    fn published(&self);
    fn lost(&self);
}

pub(crate) struct DeliveryQueue<T> {
    inner: Mutex<State<T>>,
    available: Condvar,
    pub(crate) options: CqOptions,
}

struct State<T> {
    deliveries: VecDeque<T>,
    reserved: u32,
    shutdown: bool,
    abandoned: bool,
}

impl<T: QueuedDelivery> DeliveryQueue<T> {
    pub(crate) fn new(options: CqOptions) -> Self {
        Self {
            inner: Mutex::new(State {
                deliveries: VecDeque::new(),
                reserved: 0,
                shutdown: false,
                abandoned: false,
            }),
            available: Condvar::new(),
            options,
        }
    }

    pub(crate) fn reserve(&self) -> Result<(), CosmosErrorCode> {
        self.reserve_inner(false, || Ok(()), |()| ())
    }

    pub(crate) fn reserve_control(&self) -> Result<(), CosmosErrorCode> {
        self.reserve_inner(true, || Ok(()), |()| ())
    }

    pub(crate) fn reserve_with<V, R>(
        &self,
        validate: impl FnOnce() -> Result<V, CosmosErrorCode>,
        commit: impl FnOnce(V) -> R,
    ) -> Result<R, CosmosErrorCode> {
        self.reserve_inner(false, validate, commit)
    }

    fn reserve_inner<V, R>(
        &self,
        control: bool,
        validate: impl FnOnce() -> Result<V, CosmosErrorCode>,
        commit: impl FnOnce(V) -> R,
    ) -> Result<R, CosmosErrorCode> {
        let mut state = self.inner.lock_recover();
        if state.shutdown || state.abandoned {
            return Err(CosmosErrorCode::CosmosErrorCodeQueueShutdown);
        }
        let validated = validate()?;
        if state.reserved == u32::MAX
            || (!control
                && self.options.max_capacity > 0
                && state.reserved >= self.options.max_capacity)
        {
            return Err(CosmosErrorCode::CosmosErrorCodeQueueFull);
        }
        state.reserved += 1;
        Ok(commit(validated))
    }

    pub(crate) fn publish(&self, delivery: T) {
        let mut state = self.inner.lock_recover();
        if state.abandoned {
            state.reserved -= 1;
            delivery.lost();
        } else {
            delivery.published();
            state.deliveries.push_back(delivery);
        }
        self.available.notify_all();
    }

    pub(crate) fn wait(&self, timeout_ms: u32) -> Option<T> {
        let deadline = (timeout_ms != u32::MAX)
            .then(|| Instant::now() + Duration::from_millis(u64::from(timeout_ms)));
        let mut state = self.inner.lock_recover();
        loop {
            if let Some(delivery) = state.deliveries.pop_front() {
                state.reserved -= 1;
                return Some(delivery);
            }
            if state.abandoned || (state.shutdown && state.reserved == 0) {
                return None;
            }
            state = if let Some(deadline) = deadline {
                let remaining = deadline.saturating_duration_since(Instant::now());
                if remaining.is_zero() {
                    return None;
                }
                self.available
                    .wait_timeout(state, remaining)
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .0
            } else {
                self.available
                    .wait(state)
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
            };
        }
    }

    pub(crate) fn shutdown(&self) {
        self.inner.lock_recover().shutdown = true;
        self.available.notify_all();
    }

    pub(crate) fn is_abandoned(&self) -> bool {
        self.inner.lock_recover().abandoned
    }

    #[cfg(test)]
    pub(crate) fn with_front<R>(&self, inspect: impl FnOnce(&T) -> R) -> R {
        inspect(
            self.inner
                .lock_recover()
                .deliveries
                .front()
                .expect("queued result"),
        )
    }

    pub(crate) fn abandon(&self) {
        let mut state = self.inner.lock_recover();
        state.abandoned = true;
        let deliveries = std::mem::take(&mut state.deliveries);
        state.reserved -= deliveries.len() as u32;
        for delivery in deliveries {
            delivery.lost();
        }
        self.available.notify_all();
    }

    pub(crate) fn state(&self) -> CosmosCompletionQueueState {
        let state = self.inner.lock_recover();
        if state.abandoned || (state.shutdown && state.reserved == 0) {
            CosmosCompletionQueueState::CosmosCompletionQueueStateDrained
        } else if state.shutdown {
            CosmosCompletionQueueState::CosmosCompletionQueueStateShutdown
        } else {
            CosmosCompletionQueueState::CosmosCompletionQueueStateRunning
        }
    }
}
