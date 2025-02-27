// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::common::user_agent::get_package_version;
use crate::{
    common::user_agent::{get_package_name, get_platform_info, get_user_agent},
    error::{ErrorKind, EventHubsError},
    models::AmqpValue,
};
use azure_core::credentials::AccessToken;
use azure_core::Result;
use azure_core_amqp::{
    AmqpClaimsBasedSecurity, AmqpClaimsBasedSecurityApis as _, AmqpConnection,
    AmqpConnectionApis as _, AmqpConnectionOptions, AmqpSession, AmqpSessionApis as _, AmqpSymbol,
};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;
use tracing::{debug, trace};
use url::Url;

/// The connection manager is responsible for managing the connection to the Event Hubs service.
/// It also handles authorization and connection recovery.
///
/// Currently the connection manager only handles a *single* connection, eventually it will manage
/// a pool of connections to the service.
pub(crate) struct ConnectionManager {
    url: Url,
    application_id: Option<String>,
    custom_endpoint: Option<Url>,
    connections: Mutex<OnceLock<Arc<AmqpConnection>>>,
    authorization_scopes: Mutex<HashMap<Url, AccessToken>>,
}

impl ConnectionManager {
    pub fn new(url: Url, application_id: Option<String>, custom_endpoint: Option<Url>) -> Self {
        Self {
            url,
            application_id,
            custom_endpoint,
            connections: Mutex::new(OnceLock::new()),
            authorization_scopes: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) async fn ensure_connection(&self) -> Result<()> {
        let connections = self.connections.lock().await;
        if connections.get().is_some() {
            return Ok(());
        }

        trace!("Creating connection for {}.", self.url);
        let connection = Arc::new(AmqpConnection::new());
        connection
            .open(
                self.application_id
                    .clone()
                    .unwrap_or(uuid::Uuid::new_v4().to_string()),
                self.url.clone(),
                Some(AmqpConnectionOptions {
                    properties: Some(
                        vec![
                            ("user-agent", get_user_agent(&self.application_id)),
                            ("version", get_package_version()),
                            ("platform", get_platform_info()),
                            ("product", get_package_name()),
                        ]
                        .into_iter()
                        .map(|(k, v)| (AmqpSymbol::from(k), AmqpValue::from(v)))
                        .collect(),
                    ),
                    custom_endpoint: self.custom_endpoint.clone(),
                    ..Default::default()
                }),
            )
            .await?;

        trace!("Connection for {} created.", self.url);
        connections
            .set(connection)
            .map_err(|_| EventHubsError::from(ErrorKind::UnableToAddConnection))?;
        Ok(())
    }

    pub(crate) async fn get_connection(&self) -> Result<Arc<AmqpConnection>> {
        let connections = self.connections.lock().await;

        let connection = connections
            .get()
            .cloned()
            .ok_or(EventHubsError::from(ErrorKind::MissingConnection))?;
        Ok(connection)
    }

    pub(crate) async fn close_connection(&self) -> Result<()> {
        let connections = self.connections.lock().await;
        let connection = connections
            .get()
            .ok_or(EventHubsError::from(ErrorKind::MissingConnection))?;

        connection.close().await?;
        Ok(())
    }

    pub(crate) async fn authorize_path(
        &self,
        connection: &Arc<AmqpConnection>,
        path: &Url,
        credential: Arc<dyn azure_core::credentials::TokenCredential>,
    ) -> Result<AccessToken> {
        debug!("Authorizing path: {path}");
        let mut scopes = self.authorization_scopes.lock().await;

        if !scopes.contains_key(path) {
            // Create an ephemeral session to host the authentication.
            let session = AmqpSession::new();
            session.begin(connection.as_ref(), None).await?;

            let cbs = AmqpClaimsBasedSecurity::new(&session)?;
            cbs.attach().await?;

            debug!("Get Token.");
            let token = credential
                .get_token(&["https://eventhubs.azure.net/.default"])
                .await?;
            debug!("Got token: {:?}", token.token.secret());
            let expires_at = token.expires_on;
            cbs.authorize_path(
                path.as_str().to_string(),
                None,
                token.token.secret().to_string(),
                expires_at,
            )
            .await?;

            // insert returns some if it *fails* to insert, None if it succeeded.
            let present = scopes.insert(path.clone(), token);
            if present.is_some() {
                return Err(EventHubsError::from(ErrorKind::UnableToAddAuthenticationToken).into());
            }
            trace!("Token added.");
        }
        Ok(scopes
            .get(path)
            .ok_or(EventHubsError::from(
                ErrorKind::UnableToAddAuthenticationToken,
            ))?
            .clone())
    }
}
