// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::{
    session::AmqpSession,
    simple_value::AmqpSimpleValue,
    value::{AmqpOrderedMap, AmqpValue},
};
use azure_core::{credentials::AccessToken, error::Result};

#[cfg(all(feature = "fe2o3_amqp", not(target_arch = "wasm32")))]
type ManagementImplementation = super::fe2o3::management::Fe2o3AmqpManagement;

#[cfg(any(not(feature = "fe2o3_amqp"), target_arch = "wasm32"))]
type ManagementImplementation = super::noop::NoopAmqpManagement;

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait AmqpManagementApis {
    async fn attach(&self) -> Result<()>;
    async fn detach(self) -> Result<()>;

    #[allow(unused_variables)]
    async fn call(
        &self,
        operation_type: String,
        application_properties: AmqpOrderedMap<String, AmqpSimpleValue>,
    ) -> Result<AmqpOrderedMap<String, AmqpValue>>;
}

pub struct AmqpManagement {
    implementation: ManagementImplementation,
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpManagementApis for AmqpManagement {
    async fn attach(&self) -> Result<()> {
        self.implementation.attach().await
    }
    async fn detach(self) -> Result<()> {
        self.implementation.detach().await
    }
    async fn call(
        &self,
        operation_type: String,
        application_properties: AmqpOrderedMap<String, AmqpSimpleValue>,
    ) -> Result<AmqpOrderedMap<String, AmqpValue>> {
        self.implementation
            .call(operation_type, application_properties)
            .await
    }
}

impl AmqpManagement {
    pub fn new(
        session: AmqpSession,
        client_node_name: String,
        access_token: AccessToken,
    ) -> Result<Self> {
        Ok(Self {
            implementation: ManagementImplementation::new(session, client_node_name, access_token)?,
        })
    }
}
