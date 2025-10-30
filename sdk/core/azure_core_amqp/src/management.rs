// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::{
    error::Result,
    session::AmqpSession,
    simple_value::AmqpSimpleValue,
    value::{AmqpOrderedMap, AmqpValue},
};
use azure_core::credentials::AccessToken;

#[cfg(all(feature = "fe2o3_amqp", not(target_arch = "wasm32")))]
type ManagementImplementation = super::fe2o3::management::Fe2o3AmqpManagement;

#[cfg(any(not(feature = "fe2o3_amqp"), target_arch = "wasm32"))]
type ManagementImplementation = super::noop::NoopAmqpManagement;

/// Trait defining the asynchronous APIs for AMQP management operations.
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait AmqpManagementApis {
    /// Attaches the management node to the AMQP session.
    async fn attach(&self) -> Result<()>;
    /// Detaches from the management node from the AMQP session.
    async fn detach(self) -> Result<()>;

    /// Calls a management operation with the specified type and application properties.
    ///
    /// # Arguments
    /// - `operation_type`: A string representing the type of management operation to perform.
    /// - `application_properties`: An ordered map of application properties to include in the management request.
    ///
    /// # Returns
    /// A result containing an ordered map of application properties from the management response, or an error if the operation fails.
    async fn call(
        &self,
        operation_type: String,
        application_properties: AmqpOrderedMap<String, AmqpSimpleValue>,
    ) -> Result<AmqpOrderedMap<String, AmqpValue>>;
}

/// Struct representing the AMQP management functionality.
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
    /// Creates a new instance of `AmqpManagement`.
    ///
    /// # Arguments
    /// - `session`: An `AmqpSession` to use for the management operations.
    /// - `client_node_name`: A string representing the client node name.
    /// - `access_token`: An `AccessToken` for authentication.
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
