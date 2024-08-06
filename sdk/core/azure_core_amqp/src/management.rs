// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: words amqp sasl

use super::{
    session::AmqpSession,
    value::{AmqpOrderedMap, AmqpValue},
};
use azure_core::{auth::AccessToken, error::Result};
use std::fmt::Debug;

pub trait AmqpManagementTrait {
    fn attach(&self) -> impl std::future::Future<Output = Result<()>>;

    #[allow(unused_variables)]
    fn call(
        &self,
        operation_type: impl Into<String>,
        application_properties: AmqpOrderedMap<String, AmqpValue>,
    ) -> impl std::future::Future<Output = Result<AmqpOrderedMap<String, AmqpValue>>>;
}

#[derive(Debug)]
struct AmqpManagementImpl<T>(T);

impl<T> AmqpManagementImpl<T>
where
    T: AmqpManagementTrait,
{
    pub fn new(manager: T) -> Self {
        Self(manager)
    }
}

#[cfg(all(feature = "iron-oxide-amqp", not(target_arch = "wasm32")))]
type ManagementImplementation = super::fe2o3::management::Fe2o3AmqpManagement;

#[cfg(any(not(feature = "iron-oxide-amqp"), target_arch = "wasm32"))]
type ManagementImplementation = super::noop::NoopAmqpManagement;

#[derive(Debug)]
pub struct AmqpManagement(AmqpManagementImpl<ManagementImplementation>);

impl AmqpManagementTrait for AmqpManagement {
    async fn attach(&self) -> Result<()> {
        self.0 .0.attach().await
    }
    async fn call(
        &self,
        operation_type: impl Into<String>,
        application_properties: AmqpOrderedMap<String, AmqpValue>,
    ) -> Result<AmqpOrderedMap<String, AmqpValue>> {
        self.0 .0.call(operation_type, application_properties).await
    }
}

impl AmqpManagement {
    pub fn new(
        session: AmqpSession,
        client_node_name: impl Into<String>,
        access_token: AccessToken,
    ) -> Self {
        Self(AmqpManagementImpl::new(ManagementImplementation::new(
            session.0 .0,
            client_node_name,
            access_token,
        )))
    }
}
