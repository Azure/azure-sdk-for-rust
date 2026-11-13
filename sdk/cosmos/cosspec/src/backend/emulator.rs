// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! A running Azure Cosmos DB Emulator.

use super::{
    remote::{Credential, RemoteAccount},
    Backend, BackendAccount,
};
use crate::{config::EmulatorConfig, Capability, RegionMap, ServiceProfile, SpecResult};
use async_trait::async_trait;
use azure_core::credentials::Secret;
use azure_data_cosmos::{options::Region, RoutingStrategy};
use std::sync::Arc;

/// The emulator's well-known, publicly documented account key.
const EMULATOR_KEY: &str =
    "C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==";

/// The region the emulator reports for its one location.
const EMULATOR_REGION: Region = Region::SOUTH_CENTRAL_US;

/// Runs specs against a running Azure Cosmos DB Emulator (Windows or Linux).
///
/// The emulator is one region, so it provides only
/// [`SingleRegion`](ServiceProfile::SingleRegion), and no capabilities beyond it.
/// `PrimaryReadWrite` is the region the emulator reports, South Central US.
#[derive(Debug)]
pub struct EmulatorBackend {
    endpoint: String,
    profiles: Vec<ServiceProfile>,
}

impl EmulatorBackend {
    /// Creates a backend from its configuration.
    pub fn new(config: &EmulatorConfig) -> Self {
        Self {
            endpoint: config.endpoint().to_owned(),
            profiles: config.profiles(),
        }
    }
}

#[async_trait]
impl Backend for EmulatorBackend {
    fn kind(&self) -> &'static str {
        "emulator"
    }

    fn profiles(&self) -> Vec<ServiceProfile> {
        self.profiles.clone()
    }

    fn capabilities(&self, profile: ServiceProfile) -> Vec<Capability> {
        profile.capabilities().to_vec()
    }

    async fn open_account(&self, profile: ServiceProfile) -> SpecResult<Arc<dyn BackendAccount>> {
        if profile != ServiceProfile::SingleRegion {
            return Err(format!("the emulator cannot provide {profile}").into());
        }
        let regions = RegionMap::new(profile, |_| Some(EMULATOR_REGION))?;
        Ok(Arc::new(RemoteAccount {
            profile,
            regions,
            endpoint: self.endpoint.clone(),
            credential: Credential::Key(Secret::new(EMULATOR_KEY)),
            routing: RoutingStrategy::ProximityTo(EMULATOR_REGION),
            emulator_certificates: true,
        }))
    }
}
