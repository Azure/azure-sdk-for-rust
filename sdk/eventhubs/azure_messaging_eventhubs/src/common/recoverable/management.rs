// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::RecoverableConnection;
use crate::{
    common::{recover_azure_operation, retry::ErrorRecoveryAction},
    RetryOptions,
};
use azure_core::{error::ErrorKind as AzureErrorKind, http::Url};
use azure_core_amqp::{
    error::Result, AmqpError, AmqpManagement, AmqpManagementApis, AmqpOrderedMap, AmqpSession,
    AmqpSessionApis, AmqpSimpleValue, AmqpValue,
};
use std::sync::{Arc, Weak};
use tracing::trace;

pub(crate) struct RecoverableManagementClient {
    recoverable_connection: Weak<RecoverableConnection>,
}

impl RecoverableManagementClient {
    /// Creates a new RecoverableManagementClient.
    ///
    /// # Arguments
    ///
    /// * `recoverable_connection` - The recoverable connection to use for management operations.
    pub(super) fn new(recoverable_connection: Weak<RecoverableConnection>) -> Self {
        Self {
            recoverable_connection,
        }
    }
    fn should_retry_management_response(e: &AmqpError) -> ErrorRecoveryAction {
        RecoverableConnection::should_retry_amqp_error(e)
    }

    pub(super) async fn create_management_client(
        connection: Arc<RecoverableConnection>,
        retry_options: &RetryOptions,
    ) -> Result<Arc<AmqpManagement>> {
        // Clients must call ensure_connection before calling ensure_management_client.

        trace!("Create management session.");
        recover_azure_operation(
            || async {
                let amqp_connection = connection.ensure_connection().await.map_err(|e| {
                    AmqpError::from(azure_core::Error::with_error(
                        AzureErrorKind::Other,
                        e,
                        "Error ensuring connection",
                    ))
                })?;

                let session = AmqpSession::new();
                session.begin(amqp_connection.as_ref(), None).await?;
                trace!("Session created.");

                let management_path = connection.url.to_string() + "/$management";
                let management_path =
                    Url::parse(&management_path).map_err(azure_core::Error::from)?;
                let access_token = connection
                    .authorizer
                    .authorize_path(&connection, &management_path)
                    .await
                    .map_err(|e| {
                        AmqpError::from(azure_core::Error::with_error(
                            AzureErrorKind::Other,
                            e,
                            "Error ensuring connection",
                        ))
                    })?;

                trace!("Create management client.");
                let management = Arc::new(AmqpManagement::new(
                    session,
                    "eventhubs_management".to_string(),
                    access_token,
                )?);
                management.attach().await?;

                Ok(management)
            },
            retry_options,
            Self::should_retry_management_response,
            None,
            None::<()>,
        )
        .await
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpManagementApis for RecoverableManagementClient {
    async fn call(
        &self,
        operation_type: String,
        application_properties: AmqpOrderedMap<String, AmqpSimpleValue>,
    ) -> Result<AmqpOrderedMap<String, AmqpValue>> {
        let result = recover_azure_operation(
            || {
                let operation_type = operation_type.clone();
                let application_properties = application_properties.clone();

                async move {
                    let connection = self
                        .recoverable_connection
                        .upgrade()
                        .ok_or_else(|| AmqpError::with_message("Missing Connection"))?;

                    #[cfg(test)]
                    connection.get_forced_error()?;

                    let result = connection
                        .ensure_amqp_management()
                        .await?
                        .call(operation_type, application_properties)
                        .await;
                    if let Err(ref e) = result {
                        trace!("Management call error: {:?}", e);
                    }
                    result
                    //                    Ok(result)
                }
            },
            &self
                .recoverable_connection
                .upgrade()
                .ok_or_else(|| AmqpError::with_message("Missing Connection"))?
                .retry_options,
            Self::should_retry_management_response,
            Some(|connection, reason| {
                Box::pin(RecoverableConnection::recover_from_error(
                    connection, reason,
                ))
            }),
            Some(self.recoverable_connection.clone()),
        )
        .await?;
        Ok(result)
    }

    async fn attach(&self) -> Result<()> {
        unimplemented!("AmqpManagementClient does not support attach operation");
    }

    async fn detach(self) -> Result<()> {
        unimplemented!("AmqpManagementClient does not support detach operation");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core_amqp::error::AmqpErrorCondition;
    use azure_core_test::{recorded, TestContext};

    #[recorded::test]
    async fn should_retry_management_response(_ctx: TestContext) -> Result<()> {
        {
            let error = AmqpError::new_management_error(
                azure_core::http::StatusCode::TooManyRequests,
                Some("Too many requests!".into()),
            );

            assert_eq!(
                RecoverableManagementClient::should_retry_management_response(&error),
                ErrorRecoveryAction::RetryAction
            );
        }
        {
            let error = AmqpError::new_management_error(
                azure_core::http::StatusCode::SwitchingProtocols,
                Some("Switcheroo".into()),
            );
            assert_eq!(
                RecoverableManagementClient::should_retry_management_response(&error),
                ErrorRecoveryAction::ReturnError
            );
        }

        {
            let error = AmqpError::new_management_error(
                azure_core::http::StatusCode::BadGateway,
                Some("Bad Gateway".into()),
            );
            assert_eq!(
                RecoverableManagementClient::should_retry_management_response(&error),
                ErrorRecoveryAction::RetryAction
            );
        }
        {
            let error = AmqpError::new_management_error(
                azure_core::http::StatusCode::RequestTimeout,
                Some("Request Timeout".into()),
            );
            assert_eq!(
                RecoverableManagementClient::should_retry_management_response(&error),
                ErrorRecoveryAction::RetryAction
            );
        }
        {
            let error = AmqpError::new_management_error(
                azure_core::http::StatusCode::RequestTimeout,
                Some("Request Timeout".into()),
            );
            assert_eq!(
                RecoverableManagementClient::should_retry_management_response(&error),
                ErrorRecoveryAction::RetryAction
            );
        }
        {
            let error = AmqpError::new_management_error(
                azure_core::http::StatusCode::InternalServerError,
                Some("Internal Server Error".into()),
            );
            assert_eq!(
                RecoverableManagementClient::should_retry_management_response(&error),
                ErrorRecoveryAction::RetryAction
            );
        }

        {
            let error = AmqpError::new_described_error(
                AmqpErrorCondition::ResourceLimitExceeded,
                Some("Resource Limit Exceeded".into()),
                Default::default(),
            );

            assert_eq!(
                RecoverableManagementClient::should_retry_management_response(&error),
                ErrorRecoveryAction::RetryAction
            );
        }
        {
            let error = AmqpError::new_described_error(
                AmqpErrorCondition::IllegalState,
                Some("Illegal State".into()),
                Default::default(),
            );

            assert_eq!(
                RecoverableManagementClient::should_retry_management_response(&error),
                ErrorRecoveryAction::ReturnError
            );
        }
        Ok(())
    }
}
