// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: words amqp sasl

use azure_core::error::Result;
use std::fmt::Debug;

use super::session::AmqpSession;

#[cfg(all(feature = "fe2o3-amqp", not(target_arch = "wasm32")))]
type CbsImplementation = super::fe2o3::cbs::Fe2o3ClaimsBasedSecurity;

#[cfg(any(not(any(feature = "fe2o3-amqp")), target_arch = "wasm32"))]
type CbsImplementation = super::noop::NoopAmqpClaimsBasedSecurity;

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
    fn attach(&self) -> impl std::future::Future<Output = Result<()>>;

    /// Asynchronously authorizes an AMQP path using the provided secret.
    ///
    /// The authorization is valid until the specified `expires_on` time. The path is typically a URI that represents an AMQP resource. The secret is typically a SAS token. The `expires_on` time is the time at which the authorization expires.
    ///
    /// # Parameters
    ///
    /// - `path`: A `String` reference representing the AMQP path to be authorized.
    /// - `secret`: An implementor of `Into<String>` representing the secret used for authorization. This is typically a JSON Web token.
    /// - `expires_on`: A `time::OffsetDateTime` representing the expiration time of the authorization.
    ///
    /// # Returns
    ///
    /// A `Result` which is:
    /// - `Ok(())` on successful authorization of the AMQP path.
    /// - `Err(e)` where `e` is an error from the `azure_core::error::Result` indicating the failure reason.
    ///
    fn authorize_path(
        &self,
        path: impl Into<String> + Debug,
        secret: impl Into<String>,
        expires_on: time::OffsetDateTime,
    ) -> impl std::future::Future<Output = Result<()>>;
}

#[derive(Debug)]
pub struct AmqpClaimsBasedSecurity {
    implementation: CbsImplementation,
}

impl AmqpClaimsBasedSecurityApis for AmqpClaimsBasedSecurity {
    async fn authorize_path(
        &self,
        path: impl Into<String> + Debug,
        secret: impl Into<String>,
        expires_on: time::OffsetDateTime,
    ) -> Result<()> {
        self.implementation
            .authorize_path(path, secret, expires_on)
            .await
    }

    async fn attach(&self) -> Result<()> {
        self.implementation.attach().await
    }
}

impl AmqpClaimsBasedSecurity {
    pub fn new(session: AmqpSession) -> Result<Self> {
        Ok(Self {
            implementation: CbsImplementation::new(session)?,
        })
    }
}
