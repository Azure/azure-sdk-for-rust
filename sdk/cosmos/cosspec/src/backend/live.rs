// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Existing live Azure Cosmos DB accounts.

use super::{
    remote::{Credential, RemoteAccount},
    Backend, BackendAccount,
};
use crate::{
    config::{LiveAccountConfig, LiveAuth, LiveConfig},
    Capability, RegionMap, ServiceProfile, SpecResult,
};
use async_trait::async_trait;
use azure_core::credentials::{Secret, TokenCredential};
use azure_data_cosmos::RoutingStrategy;
use azure_identity::DeveloperToolsCredential;
use std::{collections::BTreeMap, sync::Arc};

/// Runs specs against existing live accounts, one per configured service profile.
///
/// Each account must already be shaped like its profile: one region per region
/// alias, in the same order, with the same write regions. The account's
/// `regions` table maps each alias to the real region it was deployed to.
/// `infra/main.bicep` deploys them. The
/// backend can't change topology, take regions offline, or create accounts, so it
/// provides only the capabilities of each profile itself.
#[derive(Debug)]
pub struct LiveBackend {
    accounts: BTreeMap<ServiceProfile, LiveAccountConfig>,
}

impl LiveBackend {
    /// Creates a backend from its configuration.
    pub fn new(config: &LiveConfig) -> Self {
        Self {
            accounts: config.accounts(),
        }
    }

    fn credential(auth: &LiveAuth) -> SpecResult<Credential> {
        Ok(match auth {
            LiveAuth::Key { key: Some(key), .. } => Credential::Key(key.clone()),
            LiveAuth::Key {
                key_env: Some(name),
                ..
            } => {
                let key = std::env::var(name)
                    .map_err(|_| format!("environment variable {name} is not set"))?;
                Credential::Key(Secret::new(key))
            }
            LiveAuth::Key { .. } => return Err("key auth needs `key` or `key_env`".into()),
            LiveAuth::Entra => {
                let credential: Arc<dyn TokenCredential> = DeveloperToolsCredential::new(None)?;
                Credential::Entra(credential)
            }
        })
    }
}

#[async_trait]
impl Backend for LiveBackend {
    fn kind(&self) -> &'static str {
        "live"
    }

    fn profiles(&self) -> Vec<ServiceProfile> {
        self.accounts.keys().copied().collect()
    }

    fn capabilities(&self, profile: ServiceProfile) -> Vec<Capability> {
        profile.capabilities().to_vec()
    }

    async fn open_account(&self, profile: ServiceProfile) -> SpecResult<Arc<dyn BackendAccount>> {
        let account = self
            .accounts
            .get(&profile)
            .ok_or_else(|| format!("no enabled [backends.live.profiles.{profile}] account"))?;
        let regions = RegionMap::new(profile, |alias| account.regions.get(&alias).cloned())
            .map_err(|e| format!("[backends.live.profiles.{profile}] regions: {e}"))?;
        Ok(Arc::new(RemoteAccount {
            profile,
            routing: RoutingStrategy::PreferredRegions(regions.preferred()),
            regions,
            endpoint: account.endpoint.clone(),
            credential: Self::credential(&account.auth)?,
            emulator_certificates: false,
        }))
    }
}
