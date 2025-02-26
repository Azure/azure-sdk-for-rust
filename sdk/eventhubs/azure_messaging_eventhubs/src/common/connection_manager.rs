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
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, trace};
use url::Url;

// The connection manager is responsible for managing the connection to the Event Hubs service.
// It also handles authorization and connection recovery.
pub(crate) struct ConnectionManager {
    application_id: Option<String>,
    custom_endpoint: Option<Url>,
    connections: Mutex<HashMap<Url, Arc<AmqpConnection>>>,
    authorization_scopes: Mutex<HashMap<Url, AccessToken>>,
}

impl ConnectionManager {
    pub fn new(application_id: Option<String>, custom_endpoint: Option<Url>) -> Self {
        Self {
            application_id,
            custom_endpoint,
            connections: Mutex::new(HashMap::new()),
            authorization_scopes: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) async fn ensure_connection(&self, url: &Url) -> Result<()> {
        let mut connections = self.connections.lock().await;
        if connections.contains_key(url) {
            trace!("Connection for {url} already exists.");
            return Ok(());
        }

        trace!("Creating connection for {url}.");
        let connection = Arc::new(AmqpConnection::new());
        connection
            .open(
                self.application_id
                    .clone()
                    .unwrap_or(uuid::Uuid::new_v4().to_string()),
                url.clone(),
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

        trace!("Connection for {url} created.");
        connections.insert(url.clone(), connection);
        Ok(())
    }

    pub(crate) async fn get_connection(&self, url: &Url) -> Result<Arc<AmqpConnection>> {
        let connections = self.connections.lock().await;

        let connection = connections.get(url).cloned();
        trace!("get_connection, found: {url}");
        let connection =
            connection.ok_or_else(|| EventHubsError::from(ErrorKind::MissingConnection))?;
        Ok(connection)
    }

    pub(crate) async fn close_connection(&self, url: &Url) -> Result<()> {
        let connections = self.connections.lock().await;
        let connection = connections
            .get(url)
            .ok_or_else(|| EventHubsError::from(ErrorKind::MissingConnection))?;

        connection.close().await?;
        Ok(())
    }

    pub(crate) async fn authorize_path(
        &self,
        connection: &Arc<AmqpConnection>,
        url: &Url,
        credential: Arc<dyn azure_core::credentials::TokenCredential>,
    ) -> Result<AccessToken> {
        debug!("Authorizing path: {url}");
        let mut scopes = self.authorization_scopes.lock().await;

        if !scopes.contains_key(url) {
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
                url.as_str().to_string(),
                None,
                token.token.secret().to_string(),
                expires_at,
            )
            .await?;

            // insert returns some if it *fails* to insert, None if it succeeded.
            let present = scopes.insert(url.clone(), token);
            if present.is_some() {
                return Err(EventHubsError::from(ErrorKind::UnableToAddAuthenticationToken).into());
            }
            trace!("Token added.");
        }
        Ok(scopes
            .get(url)
            .ok_or_else(|| EventHubsError::from(ErrorKind::UnableToAddAuthenticationToken))?
            .clone())
    }
}
