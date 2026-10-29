// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    change_feed_processor::{
        bootstrap::error,
        processing::{lost, BatchReceipt, ChangeFeedBatch},
    },
    models::CosmosResponse,
    Result,
};
use azure_core::http::StatusCode;
use futures::{future::poll_fn, task::AtomicWaker};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex, MutexGuard},
    task::Poll,
};

#[derive(Clone, PartialEq, Eq, Hash)]
struct SessionKey {
    lease_token: String,
    incarnation: String,
}

#[derive(Default)]
struct Mailbox {
    batch: Mutex<Option<ChangeFeedBatch>>,
}

impl Mailbox {
    fn batch(&self) -> MutexGuard<'_, Option<ChangeFeedBatch>> {
        self.batch.lock().unwrap_or_else(|err| {
            tracing::error!("Recovering poisoned lease mailbox");
            err.into_inner()
        })
    }
}

#[derive(Default)]
struct Registry {
    sessions: HashMap<SessionKey, Arc<Mailbox>>,
    ready: VecDeque<SessionKey>,
    closed: bool,
}

pub(super) struct BatchDistributor {
    registry: Mutex<Registry>,
    waker: AtomicWaker,
}

impl Default for BatchDistributor {
    fn default() -> Self {
        Self {
            registry: Mutex::new(Registry::default()),
            waker: AtomicWaker::new(),
        }
    }
}

impl BatchDistributor {
    fn registry(&self) -> MutexGuard<'_, Registry> {
        self.registry.lock().unwrap_or_else(|err| {
            tracing::error!("Recovering poisoned batch distributor");
            err.into_inner()
        })
    }

    pub(super) fn register(
        self: &Arc<Self>,
        lease_token: String,
        incarnation: Option<String>,
    ) -> Result<LeaseSession> {
        let key = SessionKey {
            lease_token,
            incarnation: incarnation.ok_or_else(lost)?,
        };
        let mut registry = self.registry();
        if registry.closed {
            return Err(lost());
        }
        if registry.sessions.contains_key(&key) {
            return Err(error(
                StatusCode::Conflict,
                "Lease delivery session is already registered.",
            ));
        }
        let mailbox = Arc::new(Mailbox::default());
        registry.sessions.insert(key.clone(), mailbox.clone());
        Ok(LeaseSession {
            key,
            mailbox,
            distributor: self.clone(),
        })
    }

    pub(super) fn is_running(&self) -> bool {
        !self.registry().closed
    }

    pub(super) async fn next_batch(&self) -> Option<ChangeFeedBatch> {
        poll_fn(|cx| {
            self.waker.register(cx.waker());
            loop {
                let batch = {
                    let mut registry = self.registry();
                    let Some(key) = registry.ready.pop_front() else {
                        return if registry.closed {
                            Poll::Ready(None)
                        } else {
                            Poll::Pending
                        };
                    };
                    let Some(mailbox) = registry.sessions.get(&key) else {
                        continue;
                    };
                    let batch = mailbox.batch().take();
                    batch
                };
                if let Some(batch) = batch {
                    if !batch.receipt.sender.is_canceled() {
                        return Poll::Ready(Some(batch));
                    }
                }
            }
        })
        .await
    }

    fn retire(&self, key: &SessionKey, mailbox: &Arc<Mailbox>) {
        {
            let mut registry = self.registry();
            if !registry
                .sessions
                .get(key)
                .is_some_and(|registered| Arc::ptr_eq(registered, mailbox))
            {
                return;
            }
            registry.sessions.remove(key);
            registry.ready.retain(|ready| ready != key);
        }
        let pending = mailbox.batch().take();
        drop(pending);
        self.waker.wake();
    }

    pub(super) fn close(&self) {
        let sessions = {
            let mut registry = self.registry();
            registry.closed = true;
            registry.ready.clear();
            std::mem::take(&mut registry.sessions)
        };
        for mailbox in sessions.into_values() {
            let pending = mailbox.batch().take();
            drop(pending);
        }
        self.waker.wake();
    }
}

/// A producer's acquisition-scoped mailbox, retired when its supervisor exits.
pub(super) struct LeaseSession {
    key: SessionKey,
    mailbox: Arc<Mailbox>,
    distributor: Arc<BatchDistributor>,
}

impl LeaseSession {
    pub(super) fn deliver(&self, response: CosmosResponse, receipt: BatchReceipt) -> Result<()> {
        let mut registry = self.distributor.registry();
        if registry.closed
            || !registry
                .sessions
                .get(&self.key)
                .is_some_and(|registered| Arc::ptr_eq(registered, &self.mailbox))
        {
            return Err(lost());
        }
        let mut pending = self.mailbox.batch();
        if pending.is_some() {
            return Err(error(
                StatusCode::Conflict,
                "Lease session already has a queued batch.",
            ));
        }
        *pending = Some(ChangeFeedBatch {
            lease_token: self.key.lease_token.clone(),
            response,
            receipt,
        });
        // A nonempty mailbox cannot publish again, so each session is queued once.
        registry.ready.push_back(self.key.clone());
        drop(pending);
        drop(registry);
        self.distributor.waker.wake();
        Ok(())
    }
}

impl Drop for LeaseSession {
    fn drop(&mut self) {
        self.distributor.retire(&self.key, &self.mailbox);
    }
}

#[cfg(test)]
mod tests;
