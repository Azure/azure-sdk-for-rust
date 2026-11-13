// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Run configuration: which backends to run, and which service profiles on each.
//!
//! A run reads its configuration from the TOML file passed with `--config <path>`.
//! Without one, it uses [`SpecConfig::default`]: every service profile on the
//! in-memory emulator.
//!
//! The backends are well known, so each has its own table under `[backends]`.
//! `enabled` turns a backend on or off; only `inmemory` is on by default. Each
//! backend's `profiles.<Profile>` tables hold that profile's settings, and a
//! profile's own `enabled` (default `true`) turns it off without removing it.
//! Enabled backends run in a fixed order: `inmemory`, `emulator`, then `live`.
//!
//! ```toml
//! [backends.inmemory.profiles.MultiRead]
//! enabled = false                    # every other profile still runs
//!
//! [backends.emulator]
//! enabled = true                     # SingleRegion only
//!
//! [backends.live]
//! enabled = true
//!
//! [backends.live.profiles.SingleRegion]
//! endpoint = "https://my-single-region.documents.azure.com:443/"
//! auth = { kind = "key", key_env = "COSSPEC_SINGLE_REGION_KEY" }
//! regions = { PrimaryReadWrite = "East US 2" }
//!
//! [backends.live.profiles.MultiWrite]
//! endpoint = "https://my-multi-write.documents.azure.com:443/"
//! auth = { kind = "entra" }
//! regions = { PrimaryReadWrite = "UK South", SecondaryReadWrite = "UK West" }
//! ```

use crate::{
    backend::{Backend, EmulatorBackend, InMemoryBackend, LiveBackend},
    RegionAlias, RegionMap, ServiceProfile, SpecResult,
};
use azure_core::credentials::Secret;
use azure_data_cosmos::options::Region;
use serde::Deserialize;
use std::{collections::BTreeMap, fmt, path::Path, sync::Arc};

/// The command-line option that names the configuration file.
pub const CONFIG_ARG: &str = "--config";

/// The default Azure Cosmos DB Emulator endpoint.
pub const EMULATOR_ENDPOINT: &str = "https://localhost:8081/";

fn enabled() -> bool {
    true
}

/// How to run the specs: the backends and their service profiles.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SpecConfig {
    /// One table per backend.
    #[serde(default)]
    pub backends: BackendsConfig,
}

impl SpecConfig {
    /// Reads and validates a TOML configuration file.
    pub fn load(path: &Path) -> SpecResult<Self> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("failed to read spec config {}: {e}", path.display()))?;
        Self::parse(&text)
            .map_err(|e| format!("invalid spec config {}: {e}", path.display()).into())
    }

    /// Parses and validates a TOML configuration.
    pub fn parse(text: &str) -> SpecResult<Self> {
        let config: SpecConfig = toml::from_str(text)?;
        config.validate()?;
        Ok(config)
    }

    /// Checks the rules TOML deserialization can't express.
    ///
    /// Disabled backends are validated too, so a mistake doesn't wait until
    /// someone enables them.
    pub fn validate(&self) -> SpecResult {
        let backends = &self.backends;
        backends.emulator.validate()?;
        backends.live.validate()?;
        if !(backends.inmemory.enabled || backends.emulator.enabled || backends.live.enabled) {
            return Err("every backend is disabled; set `enabled = true` on one".into());
        }
        Ok(())
    }

    /// Builds the enabled backends, in run order.
    pub fn backends(&self) -> SpecResult<Vec<Arc<dyn Backend>>> {
        let config = &self.backends;
        let mut backends: Vec<Arc<dyn Backend>> = Vec::new();
        if config.inmemory.enabled {
            backends.push(Arc::new(InMemoryBackend::new(config.inmemory.profiles())));
        }
        if config.emulator.enabled {
            backends.push(Arc::new(EmulatorBackend::new(&config.emulator)));
        }
        if config.live.enabled {
            backends.push(Arc::new(LiveBackend::new(&config.live)));
        }
        Ok(backends)
    }
}

/// The backend tables. Each backend is optional and has its own defaults.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BackendsConfig {
    /// The in-process in-memory emulator. Enabled by default.
    #[serde(default)]
    pub inmemory: InMemoryConfig,
    /// A running Azure Cosmos DB Emulator (Windows or Linux). Disabled by default.
    #[serde(default)]
    pub emulator: EmulatorConfig,
    /// Existing live Azure Cosmos DB accounts, one per service profile. Disabled by default.
    #[serde(default)]
    pub live: LiveConfig,
}

/// A service profile with no settings beyond whether it runs.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ProfileToggle {
    /// Whether the profile runs. Defaults to `true`.
    #[serde(default = "enabled")]
    pub enabled: bool,
}

/// Configuration for the in-memory emulator backend.
///
/// Every service profile runs unless its table sets `enabled = false`.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InMemoryConfig {
    /// Whether the backend runs. Defaults to `true`.
    #[serde(default = "enabled")]
    pub enabled: bool,
    /// Per-profile settings.
    #[serde(default)]
    pub profiles: BTreeMap<ServiceProfile, ProfileToggle>,
}

impl Default for InMemoryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            profiles: BTreeMap::new(),
        }
    }
}

impl InMemoryConfig {
    /// The service profiles to run.
    pub fn profiles(&self) -> Vec<ServiceProfile> {
        ServiceProfile::ALL
            .iter()
            .copied()
            .filter(|p| self.profiles.get(p).is_none_or(|t| t.enabled))
            .collect()
    }
}

/// Configuration for a running Azure Cosmos DB Emulator.
///
/// Everything but `enabled` is optional. The emulator only provides
/// [`SingleRegion`](ServiceProfile::SingleRegion); any other profile table is
/// an error.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EmulatorConfig {
    /// Whether the backend runs. Defaults to `false`.
    #[serde(default)]
    pub enabled: bool,
    /// The emulator endpoint; defaults to [`EMULATOR_ENDPOINT`].
    pub endpoint: Option<String>,
    /// Per-profile settings. Only `SingleRegion` is allowed.
    #[serde(default)]
    pub profiles: BTreeMap<ServiceProfile, ProfileToggle>,
}

impl EmulatorConfig {
    /// The emulator endpoint.
    pub fn endpoint(&self) -> &str {
        self.endpoint.as_deref().unwrap_or(EMULATOR_ENDPOINT)
    }

    /// The service profiles to run.
    pub fn profiles(&self) -> Vec<ServiceProfile> {
        let single = ServiceProfile::SingleRegion;
        if self.profiles.get(&single).is_none_or(|t| t.enabled) {
            vec![single]
        } else {
            Vec::new()
        }
    }

    fn validate(&self) -> SpecResult {
        if let Some(other) = self
            .profiles
            .keys()
            .find(|p| **p != ServiceProfile::SingleRegion)
        {
            return Err(format!(
                "the emulator backend only provides SingleRegion, but [backends.emulator.profiles.{other}] was configured"
            )
            .into());
        }
        if self
            .endpoint()
            .parse::<azure_data_cosmos::AccountEndpoint>()
            .is_err()
        {
            return Err(format!(
                "emulator endpoint '{}' is not a valid account endpoint",
                self.endpoint()
            )
            .into());
        }
        Ok(())
    }
}

/// Configuration for existing live accounts.
///
/// Each `profiles.<Profile>` table names an account shaped like that profile,
/// for example one deployed by `infra/main.bicep`. Only configured, enabled
/// profiles run.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveConfig {
    /// Whether the backend runs. Defaults to `false`.
    #[serde(default)]
    pub enabled: bool,
    /// The account for each service profile.
    #[serde(default)]
    pub profiles: BTreeMap<ServiceProfile, LiveAccountConfig>,
}

impl LiveConfig {
    /// The service profiles to run, with their accounts.
    pub fn accounts(&self) -> BTreeMap<ServiceProfile, LiveAccountConfig> {
        self.profiles
            .iter()
            .filter(|(_, a)| a.enabled)
            .map(|(p, a)| (*p, a.clone()))
            .collect()
    }

    fn validate(&self) -> SpecResult {
        for (profile, account) in &self.profiles {
            account
                .validate(*profile)
                .map_err(|e| format!("[backends.live.profiles.{profile}]: {e}"))?;
        }
        if self.enabled && self.accounts().is_empty() {
            return Err(
                "the live backend is enabled but has no enabled [backends.live.profiles.<Profile>] table"
                    .into(),
            );
        }
        Ok(())
    }
}

/// One live account.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveAccountConfig {
    /// Whether the profile runs. Defaults to `true`.
    #[serde(default = "enabled")]
    pub enabled: bool,
    /// The account endpoint, for example `https://my-account.documents.azure.com:443/`.
    pub endpoint: String,
    /// How to authenticate.
    pub auth: LiveAuth,
    /// The real region behind each of the profile's region aliases, for
    /// example `{ PrimaryReadWrite = "East US 2", SecondaryReadOnly = "West US 3" }`.
    pub regions: BTreeMap<RegionAlias, Region>,
}

impl LiveAccountConfig {
    fn validate(&self, profile: ServiceProfile) -> SpecResult {
        if let Some(extra) = self
            .regions
            .keys()
            .find(|alias| !profile.regions().contains(alias))
        {
            return Err(format!("{profile} has no {extra} region").into());
        }
        RegionMap::new(profile, |alias| self.regions.get(&alias).cloned())
            .map_err(|e| format!("regions: {e}"))?;
        if self
            .endpoint
            .parse::<azure_data_cosmos::AccountEndpoint>()
            .is_err()
        {
            return Err(format!("'{}' is not a valid account endpoint", self.endpoint).into());
        }
        if let LiveAuth::Key { key, key_env } = &self.auth {
            if key.is_some() == key_env.is_some() {
                return Err("key auth needs exactly one of `key` or `key_env`".into());
            }
        }
        Ok(())
    }
}

/// How to authenticate to a live account. The `kind` key selects the variant.
#[derive(Clone, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase", deny_unknown_fields)]
pub enum LiveAuth {
    /// An account key. Prefer `key_env`, so the key stays out of the file.
    Key {
        /// The key itself.
        key: Option<Secret>,
        /// The environment variable holding the key.
        key_env: Option<String>,
    },
    /// Microsoft Entra ID, using the developer's Azure CLI or Azure Developer CLI sign-in.
    Entra,
}

impl fmt::Debug for LiveAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiveAuth::Key { key_env, .. } => f
                .debug_struct("Key")
                .field("key_env", key_env)
                .finish_non_exhaustive(),
            LiveAuth::Entra => f.write_str("Entra"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LiveAuth, Region, RegionAlias, ServiceProfile, SpecConfig};

    fn kinds(config: &SpecConfig) -> Vec<&'static str> {
        config
            .backends()
            .unwrap()
            .iter()
            .map(|b| b.kind())
            .collect()
    }

    #[test]
    fn default_runs_every_profile_in_memory() {
        let config = SpecConfig::default();
        assert_eq!(kinds(&config), ["inmemory"]);
        assert_eq!(config.backends.inmemory.profiles(), ServiceProfile::ALL);
        assert_eq!(SpecConfig::parse("").unwrap(), config);
    }

    #[test]
    fn parses_every_backend() {
        let config = SpecConfig::parse(
            r#"
            [backends.inmemory.profiles.MultiRead]
            enabled = false

            [backends.emulator]
            enabled = true

            [backends.live]
            enabled = true
            [backends.live.profiles.SingleRegion]
            endpoint = "https://a.documents.azure.com:443/"
            auth = { kind = "key", key_env = "A_KEY" }
            regions = { PrimaryReadWrite = "East US" }
            [backends.live.profiles.MultiRead]
            endpoint = "https://b.documents.azure.com:443/"
            auth = { kind = "entra" }
            regions = { PrimaryReadWrite = "East US", SecondaryReadOnly = "West US" }
            [backends.live.profiles.MultiWrite]
            enabled = false
            endpoint = "https://c.documents.azure.com:443/"
            auth = { kind = "entra" }
            regions = { PrimaryReadWrite = "East US", SecondaryReadWrite = "West US" }
            "#,
        )
        .unwrap();
        assert_eq!(kinds(&config), ["inmemory", "emulator", "live"]);
        let backends = config.backends().unwrap();
        assert!(!backends[0].profiles().contains(&ServiceProfile::MultiRead));
        assert_eq!(backends[1].profiles(), [ServiceProfile::SingleRegion]);
        assert_eq!(
            backends[2].profiles(),
            [ServiceProfile::SingleRegion, ServiceProfile::MultiRead]
        );
        assert_eq!(
            config.backends.live.profiles[&ServiceProfile::MultiRead].auth,
            LiveAuth::Entra
        );
    }

    #[test]
    fn enabled_switches_backends() {
        let config = SpecConfig::parse(
            "[backends.inmemory]\nenabled = false\n[backends.emulator]\nenabled = true\n",
        )
        .unwrap();
        assert_eq!(kinds(&config), ["emulator"]);
        // Configured but not enabled: the live backend doesn't run.
        let config = SpecConfig::parse(
            r#"
            [backends.live.profiles.SingleRegion]
            endpoint = "https://a.documents.azure.com:443/"
            auth = { kind = "entra" }
            regions = { PrimaryReadWrite = "East US" }
            "#,
        )
        .unwrap();
        assert_eq!(kinds(&config), ["inmemory"]);
        let error = SpecConfig::parse("[backends.inmemory]\nenabled = false\n").unwrap_err();
        assert!(error.to_string().contains("every backend is disabled"));
    }

    #[test]
    fn emulator_rejects_profiles_other_than_single_region() {
        let error = SpecConfig::parse("[backends.emulator.profiles.MultiRead]\n").unwrap_err();
        assert!(error.to_string().contains("only provides SingleRegion"));
        let config = SpecConfig::parse(
            "[backends.emulator]\nenabled = true\n[backends.emulator.profiles.SingleRegion]\nenabled = false\n",
        )
        .unwrap();
        assert!(config.backends.emulator.profiles().is_empty());
    }

    #[test]
    fn rejects_unknown_backends_fields_and_profiles() {
        assert!(SpecConfig::parse("[backends.hosted]\nenabled = true\n").is_err());
        assert!(SpecConfig::parse("[backends.inmemory]\nenable = true\n").is_err());
        assert!(SpecConfig::parse("[backends.inmemory.profiles.Nope]\n").is_err());
        assert!(
            SpecConfig::parse("[backends.inmemory.profiles.SingleRegion]\nendpoint = \"x\"\n")
                .is_err()
        );
    }

    #[test]
    fn live_needs_profiles_and_one_key_source() {
        let error = SpecConfig::parse("[backends.live]\nenabled = true\n").unwrap_err();
        assert!(error.to_string().contains("no enabled"));
        let error = SpecConfig::parse(
            r#"
            [backends.live.profiles.SingleRegion]
            endpoint = "https://a.documents.azure.com:443/"
            auth = { kind = "key" }
            regions = { PrimaryReadWrite = "East US" }
            "#,
        )
        .unwrap_err();
        assert!(error.to_string().contains("exactly one"));
    }

    #[test]
    fn live_maps_exactly_the_profile_region_aliases() {
        let parse = |regions: &str| {
            SpecConfig::parse(&format!(
                r#"
                [backends.live.profiles.MultiRead]
                endpoint = "https://a.documents.azure.com:443/"
                auth = {{ kind = "entra" }}
                regions = {regions}
                "#
            ))
        };
        let config =
            parse(r#"{ PrimaryReadWrite = "North Europe", SecondaryReadOnly = "West Europe" }"#)
                .unwrap();
        assert_eq!(
            config.backends.live.profiles[&ServiceProfile::MultiRead].regions
                [&RegionAlias::SecondaryReadOnly],
            Region::WEST_EUROPE
        );
        let error = parse(r#"{ PrimaryReadWrite = "North Europe" }"#).unwrap_err();
        assert!(error
            .to_string()
            .contains("needs a region for SecondaryReadOnly"));
        let error = parse(r#"{ PrimaryReadWrite = "A", SecondaryReadWrite = "B" }"#).unwrap_err();
        assert!(error.to_string().contains("has no SecondaryReadWrite"));
        let error = parse(r#"{ PrimaryReadWrite = "A", SecondaryReadOnly = "a" }"#).unwrap_err();
        assert!(error.to_string().contains("both mapped"));
        assert!(parse(r#"{ Nowhere = "A" }"#).is_err());
    }

    #[test]
    fn example_configs_load() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("configs");
        let mut loaded = 0;
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|ext| ext == "toml") {
                SpecConfig::load(&path).unwrap();
                loaded += 1;
            }
        }
        assert!(loaded > 0);
    }

    #[test]
    fn debug_hides_inline_keys() {
        let config = SpecConfig::parse(
            r#"
            [backends.live.profiles.SingleRegion]
            endpoint = "https://a.documents.azure.com:443/"
            auth = { kind = "key", key = "super-secret" }
            regions = { PrimaryReadWrite = "East US" }
            "#,
        )
        .unwrap();
        assert!(!format!("{config:?}").contains("super-secret"));
    }
}
