// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell: words amqp mgmt

use super::error::AmqpBegin;
use crate::{
    connection::AmqpConnection,
    session::{AmqpSessionOptions, AmqpSessionTrait},
};
use async_std::sync::Mutex;
use azure_core::Result;
use std::{
    borrow::BorrowMut,
    sync::{Arc, OnceLock},
};
use tracing::debug;

#[derive(Debug, Clone, Default)]
pub(crate) struct Fe2o3AmqpSession {
    session: OnceLock<Arc<Mutex<fe2o3_amqp::session::SessionHandle<()>>>>,
}

impl Drop for Fe2o3AmqpSession {
    fn drop(&mut self) {
        debug!("Dropping Fe2o3AmqpSession.");
    }
}

impl Fe2o3AmqpSession {
    pub fn new() -> Self {
        Self {
            session: OnceLock::new(),
        }
    }

    /// Returns a reference to the session handle
    pub fn get(&self) -> Arc<Mutex<fe2o3_amqp::session::SessionHandle<()>>> {
        self.session.get().unwrap().clone()
    }
}

impl AmqpSessionTrait for Fe2o3AmqpSession {
    async fn begin(
        &self,
        connection: &AmqpConnection,
        options: Option<AmqpSessionOptions>,
    ) -> Result<()> {
        let mut connection = connection.0 .0.get().get().unwrap().lock().await;

        let mut session_builder = fe2o3_amqp::session::Session::builder();
        if options.is_some() {
            let options = options.unwrap();

            if let Some(incoming_window) = options.incoming_window {
                session_builder = session_builder.incoming_window(incoming_window);
            }
            if let Some(outgoing_window) = options.outgoing_window {
                session_builder = session_builder.outgoing_window(outgoing_window);
            }
            if let Some(handle_max) = options.handle_max {
                session_builder = session_builder.handle_max(handle_max);
            }
            if let Some(offered_capabilities) = options.offered_capabilities.clone() {
                for capability in offered_capabilities {
                    let capability: fe2o3_amqp_types::primitives::Symbol = capability.into();
                    session_builder = session_builder.add_offered_capabilities(capability);
                }
            }
            if let Some(desired_capabilities) = options.desired_capabilities.clone() {
                for capability in desired_capabilities {
                    let capability: fe2o3_amqp_types::primitives::Symbol = capability.into();
                    session_builder = session_builder.add_desired_capabilities(capability);
                }
            }
            if let Some(properties) = options.properties.clone() {
                let mut fields = fe2o3_amqp::types::definitions::Fields::new();
                for property in properties.iter() {
                    debug!("Property: {:?}, Value: {:?}", property.0, property.1);
                    let k: fe2o3_amqp_types::primitives::Symbol = property.0.into();
                    let v: fe2o3_amqp_types::primitives::Value = property.1.into();
                    debug!("Property: {:?}, Value: {:?}", k, v);

                    fields.insert(k, v);
                }
                session_builder = session_builder.properties(fields);
            }
            if let Some(buffer_size) = options.buffer_size {
                session_builder = session_builder.buffer_size(buffer_size);
            }
        }
        let session = session_builder
            .begin(connection.borrow_mut())
            .await
            .map_err(AmqpBegin::from)?;
        self.session.set(Arc::new(Mutex::new(session))).unwrap();
        Ok(())
    }

    async fn end(&self) -> Result<()> {
        todo!()
    }
}
