// cspell:: words amqp servicebus sastoken

use super::error::AmqpManagementAttachError;
use super::session::Fe2o3AmqpSession;
use crate::{cbs::AmqpClaimsBasedSecurityTrait, fe2o3::error::AmqpManagementError};
use azure_core::error::Result;
use fe2o3_amqp_cbs::token::CbsToken;
use fe2o3_amqp_types::primitives::Timestamp;
use futures::{FutureExt, TryFutureExt};
use log::{debug, trace};
use std::borrow::BorrowMut;
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

#[derive(Debug)]
pub(crate) struct Fe2o3ClaimsBasedSecurity {
    cbs: OnceLock<Mutex<fe2o3_amqp_cbs::client::CbsClient>>,
    session: Arc<Mutex<fe2o3_amqp::session::SessionHandle<()>>>,
}

impl Fe2o3ClaimsBasedSecurity {
    pub(crate) fn new(session: Fe2o3AmqpSession) -> Self {
        Self {
            cbs: OnceLock::new(),
            session: session.get(),
        }
    }
}

unsafe impl Send for Fe2o3ClaimsBasedSecurity {}
unsafe impl Sync for Fe2o3ClaimsBasedSecurity {}

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
            .client_node_addr("rust_eventhubs_cbs")
            .attach(session.borrow_mut())
            .await
            .map_err(AmqpManagementAttachError::from)?;
        self.cbs.set(Mutex::new(cbs_client)).unwrap();
        Ok(())
    }

    fn authorize_path(
        &self,
        path: &String,
        secret: impl Into<String>,
        expires_at: time::OffsetDateTime,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
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
            .then(move |cbs| {
                //                let cbs = Arc::new(cbs);
                let mut cbs = cbs;
                let path = path.clone();
                let cbs_token = cbs_token.clone();
                async move { cbs.borrow_mut().put_token(path, cbs_token).await }
            })
            .map_err(|err| AmqpManagementError::from(err).into())
    }
}
