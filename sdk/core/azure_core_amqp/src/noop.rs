// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: words amqp
#![allow(unused_variables)]

use super::{
    cbs::AmqpClaimsBasedSecurityApis,
    connection::{AmqpConnection, AmqpConnectionApis, AmqpConnectionOptions},
    management::AmqpManagementApis,
    messaging::{AmqpMessage, AmqpSource, AmqpTarget},
    receiver::{AmqpReceiverApis, AmqpReceiverOptions},
    sender::{AmqpSendOptions, AmqpSenderApis, AmqpSenderOptions},
    session::{AmqpSession, AmqpSessionApis, AmqpSessionOptions},
    value::{AmqpOrderedMap, AmqpSymbol, AmqpValue},
};
use azure_core::{authentication::AccessToken, error::Result};

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
    pub fn new() -> Self {
        Self {}
    }
}
impl AmqpConnectionApis for NoopAmqpConnection {
    async fn open(
        &self,
        name: impl Into<String>,
        url: azure_core::Url,
        options: Option<AmqpConnectionOptions>,
    ) -> Result<()> {
        unimplemented!()
    }
    async fn close(&self) -> Result<()> {
        unimplemented!()
    }

    async fn close_with_error(
        &self,
        condition: impl Into<AmqpSymbol>,
        description: Option<String>,
        info: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    ) -> Result<()> {
        unimplemented!()
    }
}

impl NoopAmqpSession {
    pub fn new() -> Self {
        Self {}
    }
}

impl AmqpSessionApis for NoopAmqpSession {
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
    pub fn new(session: AmqpSession) -> Self {
        Self {}
    }
}

impl AmqpClaimsBasedSecurityApis for NoopAmqpClaimsBasedSecurity {
    async fn attach(&self) -> Result<()> {
        unimplemented!();
    }
    async fn authorize_path(
        &self,
        path: impl Into<String> + std::fmt::Debug,
        secret: impl Into<String>,
        expires_on: time::OffsetDateTime,
    ) -> Result<()> {
        unimplemented!()
    }
}

impl NoopAmqpManagement {
    pub fn new(session: AmqpSession, name: impl Into<String>, access_token: AccessToken) -> Self {
        Self {}
    }
}
impl AmqpManagementApis for NoopAmqpManagement {
    async fn attach(&self) -> Result<()> {
        unimplemented!();
    }

    async fn call(
        &self,
        operation_type: impl Into<String>,
        application_properties: AmqpOrderedMap<String, AmqpValue>,
    ) -> Result<AmqpOrderedMap<String, AmqpValue>> {
        unimplemented!();
    }
}

impl NoopAmqpSender {
    pub fn new() -> Self {
        Self {}
    }
}

impl AmqpSenderApis for NoopAmqpSender {
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

    async fn send(
        &self,
        message: impl Into<AmqpMessage>,
        options: Option<AmqpSendOptions>,
    ) -> Result<()> {
        unimplemented!();
    }
}

impl NoopAmqpReceiver {
    pub fn new() -> Self {
        Self {}
    }
}

impl AmqpReceiverApis for NoopAmqpReceiver {
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
