// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell:: words amqp servicebus sastoken

use crate::{cbs::AmqpClaimsBasedSecurityApis, error::Result, session::AmqpSession, AmqpError};
use azure_core::{credentials::Secret, time::OffsetDateTime};
use fe2o3_amqp_cbs::token::CbsToken;
use fe2o3_amqp_types::primitives::Timestamp;
use std::borrow::BorrowMut;
use std::sync::OnceLock;
use tokio::sync::Mutex;
use tracing::{debug, trace};

pub(crate) struct Fe2o3ClaimsBasedSecurity {
    cbs: OnceLock<Mutex<fe2o3_amqp_cbs::client::CbsClient>>,
    session: AmqpSession,
}

impl Fe2o3ClaimsBasedSecurity {
    pub fn new(session: AmqpSession) -> Result<Self> {
        Ok(Self {
            cbs: OnceLock::new(),
            session,
        })
    }

    fn cbs_already_attached() -> AmqpError {
        AmqpError::with_message("Claims Based Security is already attached")
    }
    fn cbs_not_set() -> AmqpError {
        AmqpError::with_message("Claims Based Security is not set")
    }
    fn cbs_not_attached() -> AmqpError {
        AmqpError::with_message("Claims Based Security is not attached")
    }
}

impl Fe2o3ClaimsBasedSecurity {}

impl Drop for Fe2o3ClaimsBasedSecurity {
    fn drop(&mut self) {
        debug!("Dropping Fe2o3ClaimsBasedSecurity.");
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpClaimsBasedSecurityApis for Fe2o3ClaimsBasedSecurity {
    async fn attach(&self) -> Result<()> {
        let session = self.session.implementation.get()?;
        let mut session = session.lock().await;
        let cbs_client = fe2o3_amqp_cbs::client::CbsClient::builder()
            .client_node_addr("rust_amqp_cbs")
            .attach(session.borrow_mut())
            .await
            .map_err(AmqpError::from)?;
        self.cbs
            .set(Mutex::new(cbs_client))
            .map_err(|_| Self::cbs_already_attached())?;
        Ok(())
    }

    async fn detach(mut self) -> Result<()> {
        let cbs = self.cbs.take().ok_or_else(Self::cbs_not_set)?;
        let cbs = cbs.into_inner();
        cbs.close().await.map_err(AmqpError::from)?;
        Ok(())
    }

    async fn authorize_path(
        &self,
        path: String,
        token_type: Option<String>,
        secret: &Secret,
        expires_at: OffsetDateTime,
    ) -> Result<()> {
        trace!(
            "authorize_path: path: {:?}, expires_at: {:?}",
            path,
            expires_at
        );
        let cbs_token = CbsToken::new(
            secret.secret(),
            token_type.unwrap_or("jwt".to_string()),
            Some(Timestamp::from(
                expires_at
                    .to_utc()
                    .unix_timestamp()
                    .checked_mul(1_000)
                    .ok_or_else(|| {
                        AmqpError::with_message("Unable to convert time to unix timestamp.")
                    })?,
            )),
        );
        self.cbs
            .get()
            .ok_or_else(Self::cbs_not_attached)?
            .lock()
            .await
            .borrow_mut()
            .put_token(path, cbs_token)
            .await
            .map_err(AmqpError::from)?;
        Ok(())
    }
}
