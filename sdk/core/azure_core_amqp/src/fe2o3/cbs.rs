// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell:: words amqp servicebus sastoken

use crate::{
    cbs::AmqpClaimsBasedSecurityApis,
    error::{AmqpErrorKind, AmqpManagementError},
    session::AmqpSession,
    AmqpError,
};
use azure_core::error::Result;
use fe2o3_amqp_cbs::token::CbsToken;
use fe2o3_amqp_types::primitives::Timestamp;
use std::borrow::BorrowMut;
use std::sync::OnceLock;
use tokio::sync::Mutex;
use tracing::{debug, trace};

pub(crate) struct Fe2o3ClaimsBasedSecurity<'a> {
    cbs: OnceLock<Mutex<fe2o3_amqp_cbs::client::CbsClient>>,
    session: &'a AmqpSession,
}

impl<'a> Fe2o3ClaimsBasedSecurity<'a> {
    pub fn new(session: &'a AmqpSession) -> Result<Self> {
        Ok(Self {
            cbs: OnceLock::new(),
            session,
        })
    }
}

impl Fe2o3ClaimsBasedSecurity<'_> {}

impl Drop for Fe2o3ClaimsBasedSecurity<'_> {
    fn drop(&mut self) {
        debug!("Dropping Fe2o3ClaimsBasedSecurity.");
    }
}

impl AmqpClaimsBasedSecurityApis for Fe2o3ClaimsBasedSecurity<'_> {
    async fn attach(&self) -> Result<()> {
        let session = self.session.implementation.get()?;
        let mut session = session.lock().await;
        let cbs_client = fe2o3_amqp_cbs::client::CbsClient::builder()
            .client_node_addr("rust_amqp_cbs")
            .attach(session.borrow_mut())
            .await
            .map_err(AmqpManagementError::from)?;
        self.cbs
            .set(Mutex::new(cbs_client))
            .map_err(|_| AmqpError::from(AmqpErrorKind::CbsAlreadyAttached))?;
        Ok(())
    }

    async fn detach(mut self) -> Result<()> {
        let cbs = self.cbs.take().ok_or(AmqpErrorKind::CbsNotSet)?;
        let cbs = cbs.into_inner();
        cbs.close().await.map_err(AmqpError::from)?;
        Ok(())
    }

    async fn authorize_path(
        &self,
        path: String,
        token_type: Option<String>,
        secret: String,
        expires_at: time::OffsetDateTime,
    ) -> Result<()> {
        trace!(
            "authorize_path: path: {:?}, expires_at: {:?}",
            path,
            expires_at
        );
        let cbs_token = CbsToken::new(
            secret,
            token_type.unwrap_or("jwt".to_string()),
            Some(Timestamp::from(
                expires_at
                    .to_offset(time::UtcOffset::UTC)
                    .unix_timestamp()
                    .checked_mul(1_000)
                    .ok_or_else(|| {
                        azure_core::Error::message(
                            azure_core::error::ErrorKind::Amqp,
                            "Unable to convert time to unix timestamp.",
                        )
                    })?,
            )),
        );
        self.cbs
            .get()
            .ok_or::<azure_core::Error>(AmqpErrorKind::CbsNotAttached.into())?
            .lock()
            .await
            .borrow_mut()
            .put_token(path, cbs_token)
            .await
            .map_err(|e| {
                let me = AmqpManagementError::try_from(e);
                if let Err(e) = me {
                    debug!("Failed to convert management error to azure error: {:?}", e);
                    return e;
                }
                AmqpErrorKind::ManagementError(me.unwrap()).into()
            })?;
        Ok(())
    }
}
