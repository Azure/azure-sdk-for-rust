// cspell: words amqp sasl

use super::{
    session::AmqpSession,
    value::{AmqpOrderedMap, AmqpValue},
};
use azure_core::{auth::AccessToken, error::Result};
use std::fmt::Debug;

pub(crate) trait AmqpManagementTrait {
    async fn attach(&self) -> Result<()> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn call(
        &self,
        operation_type: impl Into<String>,
        application_properties: AmqpOrderedMap<String, AmqpValue>,
    ) -> Result<AmqpOrderedMap<String, AmqpValue>> {
        unimplemented!()
    }
}

#[derive(Debug)]
struct AmqpManagementImpl<T>(T);

impl<T> AmqpManagementImpl<T>
where
    T: AmqpManagementTrait,
{
    pub(crate) fn new(manager: T) -> Self {
        Self(manager)
    }
}

#[cfg(any(feature = "enable-fe2o3-amqp"))]
type ManagementImplementation = super::fe2o3::management::Fe2o3AmqpManagement;

#[cfg(not(any(feature = "enable-fe2o3-amqp")))]
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
    pub(crate) fn new(
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
