// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    error::Result, session::AmqpSession, transaction::AmqpTransactionCoordinatorApis, AmqpError,
    TransactionId,
};
use fe2o3_amqp::transaction::{OwnedTransaction, TransactionDischarge, TransactionExt};
use std::{
    borrow::BorrowMut,
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
};
use tokio::sync::Mutex;
use tracing::debug;

pub(crate) struct Fe2o3TransactionCoordinator {
    attached: AtomicBool,
    active_transactions: Mutex<HashMap<TransactionId, OwnedTransaction>>,
    session: AmqpSession,
}

impl Fe2o3TransactionCoordinator {
    pub fn new(session: AmqpSession) -> Result<Self> {
        Ok(Self {
            attached: AtomicBool::new(false),
            active_transactions: Mutex::new(HashMap::new()),
            session,
        })
    }

    fn coordinator_not_attached() -> AmqpError {
        AmqpError::with_message("Transaction coordinator is not attached")
    }
}

impl Drop for Fe2o3TransactionCoordinator {
    fn drop(&mut self) {
        debug!("Dropping Fe2o3TransactionCoordinator.");
    }
}

#[async_trait::async_trait]
impl AmqpTransactionCoordinatorApis for Fe2o3TransactionCoordinator {
    async fn attach(&self) -> Result<()> {
        let session = self.session.implementation.get()?;
        let _guard = session.lock().await;
        self.attached.store(true, Ordering::SeqCst);
        Ok(())
    }

    async fn detach(mut self) -> Result<()> {
        self.attached.store(false, Ordering::SeqCst);
        let mut transactions = self.active_transactions.lock().await;
        for (txn_id, mut txn) in transactions.drain() {
            if let Err(e) = txn.discharge(true).await {
                tracing::warn!(?txn_id, error = ?e, "failed to roll back transaction during detach");
            }
        }
        Ok(())
    }

    async fn declare(&self) -> Result<TransactionId> {
        if !self.attached.load(Ordering::SeqCst) {
            return Err(Self::coordinator_not_attached());
        }

        let session = self.session.implementation.get()?;
        let mut session_guard = session.lock().await;

        let txn = OwnedTransaction::declare(session_guard.borrow_mut(), "coordinator-link", None)
            .await
            .map_err(AmqpError::from)?;

        let txn_id = txn.txn_id().as_slice().to_vec();
        self.active_transactions
            .lock()
            .await
            .insert(txn_id.clone(), txn);

        Ok(txn_id)
    }

    async fn discharge(&self, txn_id: TransactionId, fail: bool) -> Result<()> {
        if !self.attached.load(Ordering::SeqCst) {
            return Err(Self::coordinator_not_attached());
        }

        let mut txn = self
            .active_transactions
            .lock()
            .await
            .remove(&txn_id)
            .ok_or_else(|| {
                AmqpError::with_message("Transaction not found or already discharged")
            })?;

        txn.discharge(fail).await.map_err(AmqpError::from)?;
        Ok(())
    }
}

impl From<fe2o3_amqp::transaction::OwnedDeclareError> for AmqpError {
    fn from(e: fe2o3_amqp::transaction::OwnedDeclareError) -> Self {
        crate::error::AmqpErrorKind::TransportImplementationError(Box::new(e)).into()
    }
}

impl From<fe2o3_amqp::transaction::OwnedDischargeError> for AmqpError {
    fn from(e: fe2o3_amqp::transaction::OwnedDischargeError) -> Self {
        crate::error::AmqpErrorKind::TransportImplementationError(Box::new(e)).into()
    }
}
