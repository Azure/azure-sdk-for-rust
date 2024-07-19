// Copyright (c) Microsoft Corp. All Rights Reserved.
// cspell: words amqp

use super::sender::AmqpSenderOptions;
use super::session::AmqpSessionOptions;
use crate::amqp_client::messaging::{AmqpMessage, AmqpTarget};
use crate::amqp_client::value::{AmqpOrderedMap, AmqpValue};
use azure_core::error::Result;

#[derive(Debug)]
pub(crate) struct NoopAmqpConnection {}

#[derive(Debug)]
pub(crate) struct NoopAmqpManagement {}

#[derive(Debug)]
pub(crate) struct NoopAmqpSender {}

#[derive(Debug)]
pub(crate) struct NoopAmqpSession {}

#[derive(Debug)]
pub(crate) struct NoopAmqpClaimsBasedSecurity {}

impl NoopAmqpConnection {
    pub(crate) fn new() -> Self {
        Self {}
    }
    pub(crate) async fn close(&self) -> Result<()> {
        todo!()
    }
    pub(crate) async fn create_session(
        &self,
        options: AmqpSessionOptions,
    ) -> Result<NoopAmqpSession> {
        Ok(NoopAmqpSession {})
    }

    pub(crate) async fn create_claims_based_security(&self) -> Result<NoopAmqpClaimsBasedSecurity> {
        Ok(NoopAmqpClaimsBasedSecurity {})
    }
}

impl NoopAmqpSession {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) async fn create_sender(
        &self,
        target: AmqpTarget,
        options: Option<AmqpSenderOptions>,
    ) -> Result<NoopAmqpSender> {
        Ok(NoopAmqpSender {})
    }

    pub(crate) async fn create_management(
        &self,
        _client_node_name: &str,
    ) -> Result<NoopAmqpManagement> {
        Ok(NoopAmqpManagement {})
    }

    pub(crate) async fn end(&self) -> Result<()> {
        todo!()
    }
}

impl NoopAmqpClaimsBasedSecurity {
    pub(crate) fn new() -> Self {
        Self {}
    }
    pub(crate) async fn authorize_path(
        &self,
        _path: &str,
        _secret: &str,
        _expires_on: i64,
    ) -> Result<()> {
        todo!()
    }
}

impl NoopAmqpManagement {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) async fn call(
        &self,
        _operation_type: String,
        _entity: String,
        _application_properties: Option<AmqpOrderedMap<String, AmqpValue>>,
    ) -> Result<AmqpOrderedMap<String, AmqpValue>> {
        todo!()
    }
}

impl NoopAmqpSender {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn max_message_size(&self) -> Option<u64> {
        todo!()
    }

    pub(crate) async fn send(&self, _message: AmqpMessage) -> Result<()> {
        todo!()
    }
}
