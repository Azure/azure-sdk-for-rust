// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
#![allow(unused_variables)]

use super::{
    cbs::AmqpClaimsBasedSecurityApis,
    connection::{AmqpConnection, AmqpConnectionApis, AmqpConnectionOptions},
    error::Result,
    management::AmqpManagementApis,
    messaging::{
        AmqpDelivery, AmqpDeliveryApis, AmqpMessage, AmqpSource, AmqpTarget, DeliveryNumber,
        DeliveryTag,
    },
    receiver::{AmqpReceiverApis, AmqpReceiverOptions, ReceiverCreditMode},
    sender::{AmqpSendOptions, AmqpSendOutcome, AmqpSenderApis, AmqpSenderOptions},
    session::{AmqpSession, AmqpSessionApis, AmqpSessionOptions},
    simple_value::AmqpSimpleValue,
    value::{AmqpOrderedMap, AmqpSymbol, AmqpValue},
};
use azure_core::{
    credentials::{AccessToken, Secret},
    time::OffsetDateTime,
};

#[derive(Default)]
pub(crate) struct NoopAmqpConnection {}

#[derive(Default)]
pub(crate) struct NoopAmqpManagement {}

#[derive(Default)]
pub(crate) struct NoopAmqpSender {}

#[derive(Default)]
pub(crate) struct NoopAmqpReceiver {}

#[derive(Default, Clone)]
pub(crate) struct NoopAmqpSession {}

#[derive(Debug)]
pub(crate) struct NoopAmqpDelivery {}

#[derive(Default)]
pub(crate) struct NoopAmqpClaimsBasedSecurity {}

impl NoopAmqpConnection {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpConnectionApis for NoopAmqpConnection {
    async fn open(
        &self,
        name: String,
        url: azure_core::http::Url,
        options: Option<AmqpConnectionOptions>,
    ) -> Result<()> {
        unimplemented!()
    }
    async fn close(&self) -> Result<()> {
        unimplemented!()
    }

    async fn close_with_error(
        &self,
        condition: AmqpSymbol,
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

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
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
    pub fn new(session: AmqpSession) -> Result<Self> {
        Ok(Self {})
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpClaimsBasedSecurityApis for NoopAmqpClaimsBasedSecurity {
    async fn attach(&self) -> Result<()> {
        unimplemented!();
    }
    async fn detach(self) -> Result<()> {
        unimplemented!();
    }
    async fn authorize_path(
        &self,
        path: String,
        token_type: Option<String>,
        secret: &Secret,
        expires_on: OffsetDateTime,
    ) -> Result<()> {
        unimplemented!()
    }
}

impl NoopAmqpManagement {
    pub fn new(session: AmqpSession, name: String, access_token: AccessToken) -> Result<Self> {
        Ok(Self {})
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpManagementApis for NoopAmqpManagement {
    async fn attach(&self) -> Result<()> {
        unimplemented!();
    }

    async fn detach(self) -> Result<()> {
        unimplemented!();
    }

    async fn call(
        &self,
        operation_type: String,
        application_properties: AmqpOrderedMap<String, AmqpSimpleValue>,
    ) -> Result<AmqpOrderedMap<String, AmqpValue>> {
        unimplemented!();
    }
}

impl NoopAmqpSender {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpSenderApis for NoopAmqpSender {
    async fn attach(
        &self,
        session: &AmqpSession,
        name: String,
        target: impl Into<AmqpTarget> + Send,
        options: Option<AmqpSenderOptions>,
    ) -> Result<()> {
        unimplemented!();
    }
    async fn detach(self) -> Result<()> {
        unimplemented!();
    }
    async fn max_message_size(&self) -> Result<Option<u64>> {
        unimplemented!();
    }

    async fn send<M>(&self, message: M, options: Option<AmqpSendOptions>) -> Result<AmqpSendOutcome>
    where
        M: Into<AmqpMessage> + std::fmt::Debug + Send,
    {
        unimplemented!();
    }

    async fn send_ref<M>(
        &self,
        message: M,
        options: Option<AmqpSendOptions>,
    ) -> Result<AmqpSendOutcome>
    where
        M: AsRef<AmqpMessage> + std::fmt::Debug + Send,
    {
        unimplemented!();
    }
}

impl NoopAmqpReceiver {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpReceiverApis for NoopAmqpReceiver {
    async fn attach(
        &self,
        session: &AmqpSession,
        source: impl Into<AmqpSource> + Send,
        options: Option<AmqpReceiverOptions>,
    ) -> Result<()> {
        unimplemented!();
    }

    async fn detach(self) -> Result<()> {
        unimplemented!();
    }

    #[allow(unused_variables)]
    async fn set_credit_mode(&self, credit_mode: ReceiverCreditMode) -> Result<()> {
        unimplemented!();
    }

    async fn credit_mode(&self) -> Result<ReceiverCreditMode> {
        unimplemented!();
    }

    #[allow(unused_variables)]
    async fn receive_delivery(&self) -> Result<AmqpDelivery> {
        unimplemented!();
    }

    async fn accept_delivery(&self, delivery: &AmqpDelivery) -> Result<()> {
        unimplemented!();
    }
    async fn reject_delivery(&self, delivery: &AmqpDelivery) -> Result<()> {
        unimplemented!();
    }
    async fn release_delivery(&self, delivery: &AmqpDelivery) -> Result<()> {
        unimplemented!();
    }
}

impl AmqpDeliveryApis for NoopAmqpDelivery {
    fn message(&self) -> &AmqpMessage {
        unimplemented!();
    }
    fn delivery_id(&self) -> DeliveryNumber {
        unimplemented!();
    }
    fn delivery_tag(&self) -> &DeliveryTag {
        unimplemented!();
    }

    fn message_format(&self) -> &Option<u32> {
        unimplemented!();
    }
    fn into_message(self) -> AmqpMessage {
        unimplemented!();
    }
}
