// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::{
    session::AmqpSession,
    value::{AmqpOrderedMap, AmqpValue},
};
use azure_core::{credentials::AccessToken, error::Result};

#[cfg(all(feature = "fe2o3-amqp", not(target_arch = "wasm32")))]
type ManagementImplementation = super::fe2o3::management::Fe2o3AmqpManagement;

#[cfg(any(not(feature = "fe2o3-amqp"), target_arch = "wasm32"))]
type ManagementImplementation = super::noop::NoopAmqpManagement;

pub trait AmqpManagementApis {
    fn attach(&self) -> impl std::future::Future<Output = Result<()>>;
    fn detach(self) -> impl std::future::Future<Output = Result<()>>;

    #[allow(unused_variables)]
    fn call(
        &self,
        operation_type: String,
        application_properties: AmqpOrderedMap<String, AmqpValue>,
    ) -> impl std::future::Future<Output = Result<AmqpOrderedMap<String, AmqpValue>>>;
}
pub(crate) mod error {
    use std::{error::Error, fmt::Debug};

    pub enum AmqpManagementError {
        AmqpManagementAlreadyAttached,
        AmqpManagementNotAttached,

        // mapped low level errors.
        InvalidManagementResponse(String),
        SendError(Box<dyn std::error::Error + Sync + Send>),
        ReceiveError(Box<dyn std::error::Error + Sync + Send>),
        DecodingError,
        NotAccepted,
        Disposition,
        HttpStatusCode(azure_core::StatusCode, Option<String>),
    }

    impl std::fmt::Display for AmqpManagementError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self {
                AmqpManagementError::AmqpManagementAlreadyAttached => {
                    f.write_str("AMQP Management is already attached")
                }
                AmqpManagementError::AmqpManagementNotAttached => {
                    f.write_str("AMQP Management is not attached")
                }
                AmqpManagementError::InvalidManagementResponse(s) => {
                    if let Some(e) = self.source() {
                        f.write_fmt(format_args!("Invalid Management Response: {}: {}", s, e))
                    } else {
                        f.write_fmt(format_args!("Invalid Management Response: {}", s))
                    }
                }
                AmqpManagementError::SendError(s) => {
                    f.write_fmt(format_args!("Error sending management request: {s}"))
                }
                AmqpManagementError::ReceiveError(r) => {
                    f.write_fmt(format_args!("Error receiving request: {r}"))
                }
                AmqpManagementError::DecodingError => f.write_str("Error decoding response."),
                AmqpManagementError::NotAccepted => f.write_str("Management request not accepted."),
                AmqpManagementError::Disposition => {
                    f.write_str("Management disposition not accepted.")
                }
                AmqpManagementError::HttpStatusCode(status_code, d) => {
                    if let Some(d) = d {
                        f.write_fmt(format_args!(
                            "Management HTTP Status code: {} ({})",
                            status_code, d
                        ))
                    } else {
                        f.write_fmt(format_args!("Management HTTP Status code: {}", status_code,))
                    }
                }
            }
        }
    }

    impl Debug for AmqpManagementError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::AmqpManagementAlreadyAttached => write!(f, "AmqpManagementAlreadyAttached"),
                Self::AmqpManagementNotAttached => write!(f, "AmqpManagementNotAttached"),
                Self::InvalidManagementResponse(arg0) => f
                    .debug_tuple("InvalidManagementResponse")
                    .field(arg0)
                    .finish(),
                Self::SendError(arg0) => f.debug_tuple("SendError").field(arg0).finish(),
                Self::ReceiveError(arg0) => f.debug_tuple("ReceiveError").field(arg0).finish(),
                Self::DecodingError => write!(f, "DecodingError"),
                Self::NotAccepted => write!(f, "NotAccepted"),
                Self::Disposition => write!(f, "Disposition"),
                Self::HttpStatusCode(arg0, arg1) => f
                    .debug_tuple("HttpStatusCode")
                    .field(arg0)
                    .field(arg1)
                    .finish(),
            }
        }
    }

    impl std::error::Error for AmqpManagementError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                AmqpManagementError::AmqpManagementAlreadyAttached => None,
                AmqpManagementError::AmqpManagementNotAttached => None,
                AmqpManagementError::InvalidManagementResponse(_) => None,
                AmqpManagementError::SendError(error) => Some(error.as_ref()),
                AmqpManagementError::ReceiveError(error) => Some(error.as_ref()),
                AmqpManagementError::DecodingError => None,
                AmqpManagementError::NotAccepted => None,
                AmqpManagementError::Disposition => None,
                AmqpManagementError::HttpStatusCode(_, _) => None,
            }
        }
    }
}

pub struct AmqpManagement {
    implementation: ManagementImplementation,
}

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
        client_node_name: String,
        access_token: AccessToken,
    ) -> Result<Self> {
        Ok(Self {
            implementation: ManagementImplementation::new(session, client_node_name, access_token)?,
        })
    }
}
