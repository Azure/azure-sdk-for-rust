// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell:: words amqp servicebus sastoken

use super::{
    error::{AmqpManagement, AmqpManagementAttach},
    session::Fe2o3AmqpSession,
};
use crate::cbs::AmqpClaimsBasedSecurityTrait;
use async_std::sync::Mutex;
use azure_core::error::Result;
use fe2o3_amqp_cbs::token::CbsToken;
use fe2o3_amqp_types::primitives::Timestamp;
use std::borrow::BorrowMut;
use std::{
    fmt::Debug,
    sync::{Arc, OnceLock},
};
use tracing::{debug, trace};

#[derive(Debug)]
pub(crate) struct Fe2o3ClaimsBasedSecurity {
    cbs: OnceLock<Mutex<fe2o3_amqp_cbs::client::CbsClient>>,
    session: Arc<Mutex<fe2o3_amqp::session::SessionHandle<()>>>,
}

impl Fe2o3ClaimsBasedSecurity {
    pub fn new(session: Fe2o3AmqpSession) -> Self {
        Self {
            cbs: OnceLock::new(),
            session: session.get(),
        }
    }
}

impl Fe2o3ClaimsBasedSecurity {}

impl Drop for Fe2o3ClaimsBasedSecurity {
    fn drop(&mut self) {
        debug!("Dropping Fe2o3ClaimsBasedSecurity.");
    }
}

impl AmqpClaimsBasedSecurityTrait for Fe2o3ClaimsBasedSecurity {
    async fn attach(&self) -> Result<()> {
        let mut session = self.session.lock().await;
        let cbs_client = fe2o3_amqp_cbs::client::CbsClient::builder()
            .client_node_addr("rust_amqp_cbs")
            .attach(session.borrow_mut())
            .await
            .map_err(AmqpManagementAttach::from)?;
        self.cbs.set(Mutex::new(cbs_client)).unwrap();
        Ok(())
    }

    async fn authorize_path(
        &self,
        path: impl Into<String> + Debug,
        secret: impl Into<String>,
        expires_at: time::OffsetDateTime,
    ) -> Result<()> {
        trace!(
            "authorize_path: path: {:?}, expires_at: {:?}",
            path,
            expires_at
        );
        let cbs_token = CbsToken::new(
            secret.into(),
            "jwt",
            Some(Timestamp::from(
                expires_at
                    .to_offset(time::UtcOffset::UTC)
                    .unix_timestamp()
                    .checked_mul(1_000)
                    .unwrap(),
            )),
        );
        self.cbs
            .get()
            .unwrap()
            .lock()
            .await
            .borrow_mut()
            .put_token(path.into(), cbs_token)
            .await
            .map_err(AmqpManagement::from)?;
        Ok(())
    }
}
