// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::session::AmqpSession;
use crate::{error::Result, TransactionId};

#[cfg(feature = "fe2o3_amqp")]
type TransactionCoordinatorImplementation = crate::fe2o3::transaction::Fe2o3TransactionCoordinator;

#[cfg(not(feature = "fe2o3_amqp"))]
type TransactionCoordinatorImplementation = crate::noop::NoopAmqpTransactionCoordinator;

/// Trait defining the asynchronous APIs for AMQP Transaction Coordinator operations.
#[async_trait::async_trait]
pub trait AmqpTransactionCoordinatorApis {
    /// Asynchronously initializes the transaction coordinator on the AMQP session and verifies readiness.
    async fn attach(&self) -> Result<()>;

    /// Asynchronously detaches the transaction coordinator from the AMQP session, explicitly rolling back any remaining active transactions.
    async fn detach(self) -> Result<()>;

    /// Asynchronously declares a new transaction.
    async fn declare(&self) -> Result<TransactionId>;

    /// Asynchronously discharges (commits or rolls back) a transaction.
    ///
    /// # Parameters
    /// - `txn_id`: The ID of the transaction to discharge.
    /// - `fail`: `true` to roll back/abort, `false` to commit.
    async fn discharge(&self, txn_id: TransactionId, fail: bool) -> Result<()>;
}

/// Struct representing an AMQP Transaction Coordinator link.
pub struct AmqpTransactionCoordinator {
    implementation: TransactionCoordinatorImplementation,
}

impl AmqpTransactionCoordinator {
    /// Creates a new instance of `AmqpTransactionCoordinator` using the provided AMQP session.
    pub fn new(session: AmqpSession) -> Result<Self> {
        Ok(Self {
            implementation: TransactionCoordinatorImplementation::new(session)?,
        })
    }
}

#[async_trait::async_trait]
impl AmqpTransactionCoordinatorApis for AmqpTransactionCoordinator {
    async fn attach(&self) -> Result<()> {
        self.implementation.attach().await
    }

    async fn detach(self) -> Result<()> {
        self.implementation.detach().await
    }

    async fn declare(&self) -> Result<TransactionId> {
        self.implementation.declare().await
    }

    async fn discharge(&self, txn_id: TransactionId, fail: bool) -> Result<()> {
        self.implementation.discharge(txn_id, fail).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn unattached_coordinator_declare_and_discharge_fail() {
        let session = AmqpSession::new();
        let coordinator = AmqpTransactionCoordinator::new(session).unwrap();

        assert!(coordinator.declare().await.is_err());
        assert!(coordinator.discharge(vec![1, 2, 3], false).await.is_err());
    }

    #[tokio::test]
    async fn unattached_session_attach_fails() {
        let session = AmqpSession::new();
        let coordinator = AmqpTransactionCoordinator::new(session).unwrap();

        assert!(coordinator.attach().await.is_err());
    }
}
