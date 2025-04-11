// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use async_trait::async_trait;
use azure_core::error::Result;

use super::session::AmqpSession;

#[cfg(all(feature = "fe2o3_amqp", not(target_arch = "wasm32")))]
type CbsImplementation<'a> = super::fe2o3::cbs::Fe2o3ClaimsBasedSecurity<'a>;

#[cfg(any(not(any(feature = "fe2o3_amqp")), target_arch = "wasm32"))]
type CbsImplementation<'a> = super::noop::NoopAmqpClaimsBasedSecurity<'a>;

#[async_trait]
pub trait AmqpClaimsBasedSecurityApis {
    /// Asynchronously attaches the Claims-Based Security (CBS) node to the AMQP session.
    ///
    /// This method is responsible for setting up the necessary AMQP links for CBS operations.
    /// It must be called before attempting to authorize any AMQP paths using the `authorize_path` method.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// - `Ok(())` on successful attachment of the CBS node.
    /// - `Err(e)` where `e` is an error from the `azure_core::error::Result` indicating the failure reason.
    ///
    async fn attach(&self) -> Result<()>;

    /// Asynchronously detaches the Claims-Based Security (CBS) node from the AMQP session.
    /// This method is responsible for cleaning up the AMQP links used for CBS operations.
    async fn detach(self) -> Result<()>;

    /// Asynchronously authorizes an AMQP path using the provided secret.
    ///
    /// The authorization is valid until the specified `expires_on` time. The path is typically a URI that represents an AMQP resource. The secret is typically a SAS token. The `expires_on` time is the time at which the authorization expires.
    ///
    /// # Parameters
    ///
    /// - `path`: A string representing the AMQP path to be authorized.
    /// - `token_type`: An optional string representing the type of token used for authorization. This is either "servicebus.windows.net:sastoken" or "jwt". If it is not supplied, "jwt" is assumed.
    /// - `secret`: A string representing the secret used for authorization. This is typically a JSON Web token.
    /// - `expires_on`: The expiration time of the authorization.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// - `Ok(())` on successful authorization of the AMQP path.
    /// - `Err(e)` where `e` is an error from the `azure_core::error::Result` indicating the failure reason.
    ///
    async fn authorize_path(
        &self,
        path: String,
        token_type: Option<String>,
        secret: String,
        expires_on: time::OffsetDateTime,
    ) -> Result<()>;
}

pub struct AmqpClaimsBasedSecurity<'a> {
    implementation: CbsImplementation<'a>,
}

impl<'a> AmqpClaimsBasedSecurity<'a> {
    pub fn new(session: &'a AmqpSession) -> Result<Self> {
        Ok(Self {
            implementation: CbsImplementation::new(session)?,
        })
    }
}

#[async_trait]
impl AmqpClaimsBasedSecurityApis for AmqpClaimsBasedSecurity<'_> {
    async fn authorize_path(
        &self,
        path: String,
        token_type: Option<String>,
        secret: String,
        expires_on: time::OffsetDateTime,
    ) -> Result<()> {
        self.implementation
            .authorize_path(path, token_type, secret, expires_on)
            .await
    }
    async fn attach(&self) -> Result<()> {
        self.implementation.attach().await
    }
    async fn detach(self) -> Result<()> {
        self.implementation.detach().await
    }
}
