// Copyright (c) Microsoft Corp. All Rights Reserved.
// cspell: words amqp

use super::{
    cbs::AmqpClaimsBasedSecurityTrait,
    connection::{AmqpConnection, AmqpConnectionTrait},
    management::AmqpManagementTrait,
    messaging::{AmqpMessage, AmqpSource, AmqpTarget},
    receiver::{AmqpReceiverOptions, AmqpReceiverTrait},
    sender::{AmqpSendOptions, AmqpSenderOptions, AmqpSenderTrait},
    session::{AmqpSession, AmqpSessionOptions, AmqpSessionTrait},
    value::{AmqpOrderedMap, AmqpValue},
};
use azure_core::{auth::AccessToken, error::Result};

#[derive(Debug, Default)]
pub(crate) struct NoopAmqpConnection {}

#[derive(Debug, Default)]
pub(crate) struct NoopAmqpManagement {}

#[derive(Debug, Default)]
pub(crate) struct NoopAmqpSender {}

#[derive(Debug, Default)]
pub(crate) struct NoopAmqpReceiver {}

#[derive(Debug, Default, Clone)]
pub(crate) struct NoopAmqpSession {}

#[derive(Debug, Default)]
pub(crate) struct NoopAmqpClaimsBasedSecurity {}

impl NoopAmqpConnection {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
impl AmqpConnectionTrait for NoopAmqpConnection {
    async fn close(&self) -> Result<()> {
        unimplemented!()
    }
}

impl NoopAmqpSession {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl AmqpSessionTrait for NoopAmqpSession {
    #[allow(unused_variables)]
    async fn begin(
        &self,
        connection: &AmqpConnection,
        options: Option<AmqpSessionOptions>,
    ) -> Result<()> {
        unimplemented!()
    }

    async fn end(&self) -> Result<()> {
        unimplemented!()
    }
}

impl NoopAmqpClaimsBasedSecurity {
    #[allow(unused_variables)]
    pub(crate) fn new(session: NoopAmqpSession) -> Self {
        Self {}
    }
}

impl AmqpClaimsBasedSecurityTrait for NoopAmqpClaimsBasedSecurity {
    async fn attach(&self) -> Result<()> {
        unimplemented!();
    }
}

impl NoopAmqpManagement {
    #[allow(unused_variables)]
    pub(crate) fn new(
        session: NoopAmqpSession,
        name: impl Into<String>,
        access_token: AccessToken,
    ) -> Self {
        Self {}
    }
}
impl AmqpManagementTrait for NoopAmqpManagement {
    async fn attach(&self) -> Result<()> {
        unimplemented!();
    }

    #[allow(unused_variables)]
    async fn call(
        &self,
        operation_type: impl Into<String>,
        application_properties: AmqpOrderedMap<String, AmqpValue>,
    ) -> Result<AmqpOrderedMap<String, AmqpValue>> {
        unimplemented!();
    }
}

impl NoopAmqpSender {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl AmqpSenderTrait for NoopAmqpSender {
    #[allow(unused_variables)]
    async fn attach(
        &self,
        session: &AmqpSession,
        name: impl Into<String>,
        target: impl Into<AmqpTarget>,
        options: Option<AmqpSenderOptions>,
    ) -> Result<()> {
        unimplemented!();
    }

    async fn max_message_size(&self) -> Option<u64> {
        unimplemented!();
    }

    #[allow(unused_variables)]
    async fn send(&self, message: AmqpMessage, options: Option<AmqpSendOptions>) -> Result<()> {
        unimplemented!();
    }
}

impl NoopAmqpReceiver {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl AmqpReceiverTrait for NoopAmqpReceiver {
    #[allow(unused_variables)]
    async fn attach(
        &self,
        session: &AmqpSession,
        source: impl Into<AmqpSource>,
        options: Option<AmqpReceiverOptions>,
    ) -> Result<()> {
        unimplemented!();
    }

    async fn max_message_size(&self) -> Option<u64> {
        unimplemented!();
    }

    #[allow(unused_variables)]
    async fn receive(&self) -> Result<AmqpMessage> {
        unimplemented!();
    }
}
