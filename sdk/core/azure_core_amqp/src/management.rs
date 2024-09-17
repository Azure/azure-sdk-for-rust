// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: words amqp sasl

use super::{
    session::AmqpSession,
    value::{AmqpOrderedMap, AmqpValue},
};
use azure_core::{authentication::AccessToken, error::Result};
use std::fmt::Debug;

#[cfg(all(feature = "fe2o3-amqp", not(target_arch = "wasm32")))]
type ManagementImplementation = super::fe2o3::management::Fe2o3AmqpManagement;

#[cfg(any(not(feature = "fe2o3-amqp"), target_arch = "wasm32"))]
type ManagementImplementation = super::noop::NoopAmqpManagement;

pub trait AmqpManagementApis {
    fn attach(&self) -> impl std::future::Future<Output = Result<()>>;

    #[allow(unused_variables)]
    fn call(
        &self,
        operation_type: impl Into<String>,
        application_properties: AmqpOrderedMap<String, AmqpValue>,
    ) -> impl std::future::Future<Output = Result<AmqpOrderedMap<String, AmqpValue>>>;
}

#[derive(Debug)]
pub struct AmqpManagement {
    implementation: ManagementImplementation,
}

impl AmqpManagementApis for AmqpManagement {
    async fn attach(&self) -> Result<()> {
        self.implementation.attach().await
    }
    async fn call(
        &self,
        operation_type: impl Into<String>,
        application_properties: AmqpOrderedMap<String, AmqpValue>,
    ) -> Result<AmqpOrderedMap<String, AmqpValue>> {
        self.implementation
            .call(operation_type, application_properties)
            .await
    }
}

impl AmqpManagement {
    pub fn new(
        session: AmqpSession,
        client_node_name: impl Into<String>,
        access_token: AccessToken,
    ) -> Self {
        Self {
            implementation: ManagementImplementation::new(session, client_node_name, access_token),
        }
    }
}
