// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! An existing account reached over the network: the Azure Cosmos DB Emulator or
//! a live account.

use super::{BackendAccount, ClientOptions};
use crate::{
    provision::{ControlPlaneProvisioner, Provisioner},
    ServiceProfile, SpecResult,
};
use async_trait::async_trait;
use azure_core::credentials::{Secret, TokenCredential};
use azure_data_cosmos::{
    options::{ConnectionPoolOptions, ServerCertificateValidation},
    AccountEndpoint, AccountReference, CosmosClient, CosmosClientBuilder, CosmosRuntime,
    RoutingStrategy,
};
use std::{fmt, sync::Arc};

/// How a remote account authenticates.
#[derive(Clone)]
pub(super) enum Credential {
    Key(Secret),
    Entra(Arc<dyn TokenCredential>),
}

impl fmt::Debug for Credential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Credential::Key(_) => "Key",
            Credential::Entra(_) => "Entra",
        })
    }
}

/// An existing account. It's never created or deleted by the run; only the
/// databases and containers in it are.
#[derive(Debug)]
pub(super) struct RemoteAccount {
    pub profile: ServiceProfile,
    pub endpoint: String,
    pub credential: Credential,
    pub routing: RoutingStrategy,
    /// Accept the emulator's self-signed certificate.
    pub emulator_certificates: bool,
}

#[async_trait]
impl BackendAccount for RemoteAccount {
    fn profile(&self) -> ServiceProfile {
        self.profile
    }

    fn endpoint(&self) -> String {
        self.endpoint.clone()
    }

    async fn client(&self, options: ClientOptions) -> SpecResult<CosmosClient> {
        let mut builder = CosmosClientBuilder::new();
        if self.emulator_certificates {
            let pool = ConnectionPoolOptions::builder()
                .with_server_certificate_validation(
                    ServerCertificateValidation::RequiredUnlessEmulator,
                )
                .build()?;
            let runtime = CosmosRuntime::builder()
                .with_connection_pool(pool)
                .build()
                .await?;
            builder = builder.with_runtime(runtime);
        }
        if let Some(observer) = options.observer {
            builder = builder.with_transport_observer(observer);
        }
        if !options.faults.is_empty() {
            builder = builder.with_fault_injection_rules(options.faults)?;
        }
        let endpoint = self.endpoint.parse::<AccountEndpoint>()?;
        let account = match &self.credential {
            Credential::Key(key) => {
                AccountReference::with_authentication_key(endpoint, key.clone())
            }
            Credential::Entra(credential) => {
                AccountReference::with_credential(endpoint, credential.clone())
            }
        };
        Ok(Box::pin(builder.build(account, self.routing.clone())).await?)
    }

    fn provisioner(&self) -> &dyn Provisioner {
        &ControlPlaneProvisioner
    }
}
