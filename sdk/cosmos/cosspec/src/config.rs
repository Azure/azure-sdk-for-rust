// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Run configuration: which backends to run, and which service profiles on each.
//!
//! A run reads its configuration from the TOML file passed with `--config <path>`.
//! Without one, it uses [`SpecConfig::default`]: every service profile on the
//! in-memory emulator.
//!
//! ```toml
//! [[backends]]
//! kind = "inmemory"
//!
//! [[backends]]
//! kind = "emulator"
//!
//! [[backends]]
//! kind = "live"
//!
//! [backends.profiles.SingleRegion]
//! endpoint = "https://my-single-region.documents.azure.com:443/"
//! auth = { kind = "key", key_env = "COSSPEC_SINGLE_REGION_KEY" }
//!
//! [backends.profiles.MultiWrite]
//! endpoint = "https://my-multi-write.documents.azure.com:443/"
//! auth = { kind = "entra" }
//! ```

use crate::{
    backend::{Backend, EmulatorBackend, InMemoryBackend, LiveBackend},
    ServiceProfile, SpecResult,
};
use azure_core::credentials::Secret;
use serde::Deserialize;
use std::{collections::BTreeMap, fmt, path::Path, sync::Arc};

/// The command-line option that names the configuration file.
pub const CONFIG_ARG: &str = "--config";

/// The default Azure Cosmos DB Emulator endpoint.
pub const EMULATOR_ENDPOINT: &str = "https://localhost:8081/";

/// How to run the specs: the backends, in order, each with its service profiles.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SpecConfig {
    /// The backends to run, in order.
    pub backends: Vec<BackendConfig>,
}

impl Default for SpecConfig {
    /// Every service profile on the in-memory emulator.
    fn default() -> Self {
        Self {
            backends: vec![BackendConfig::InMemory(InMemoryConfig::default())],
        }
    }
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
    pub fn validate(&self) -> SpecResult {
        if self.backends.is_empty() {
            return Err("at least one backend is required".into());
        }
        let mut names = Vec::new();
        for backend in &self.backends {
            let name = backend.name();
            if name.is_empty()
                || !name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
            {
                return Err(format!(
                    "backend name '{name}' must be lowercase letters, digits, '_', or '-'"
                )
                .into());
            }
            if names.contains(&name) {
                return Err(format!(
                    "backend name '{name}' is used twice; give one of them a distinct `name`"
                )
                .into());
            }
            names.push(name);
            backend.validate()?;
        }
        Ok(())
    }

    /// Builds the configured backends, in order.
    pub fn backends(&self) -> SpecResult<Vec<Arc<dyn Backend>>> {
        self.backends.iter().map(BackendConfig::build).collect()
    }
}

/// One backend to run. The `kind` key selects the variant.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum BackendConfig {
    /// The in-process in-memory emulator.
    #[serde(rename = "inmemory")]
    InMemory(InMemoryConfig),
    /// A running Azure Cosmos DB Emulator (Windows or Linux).
    Emulator(EmulatorConfig),
    /// Existing live Azure Cosmos DB accounts, one per service profile.
    Live(LiveConfig),
}

impl BackendConfig {
    /// The backend kind, as written in `kind`.
    pub fn kind(&self) -> &'static str {
        match self {
            BackendConfig::InMemory(_) => "inmemory",
            BackendConfig::Emulator(_) => "emulator",
            BackendConfig::Live(_) => "live",
        }
    }

    /// The backend name used in test names: `name` if set, else the kind.
    pub fn name(&self) -> &str {
        let name = match self {
            BackendConfig::InMemory(c) => c.name.as_deref(),
            BackendConfig::Emulator(c) => c.name.as_deref(),
            BackendConfig::Live(c) => c.name.as_deref(),
        };
        name.unwrap_or(self.kind())
    }

    fn validate(&self) -> SpecResult {
        match self {
            BackendConfig::InMemory(_) => Ok(()),
            BackendConfig::Emulator(c) => c.validate(),
            BackendConfig::Live(c) => c.validate(self.name()),
        }
    }

    fn build(&self) -> SpecResult<Arc<dyn Backend>> {
        let name = self.name().to_owned();
        Ok(match self {
            BackendConfig::InMemory(c) => Arc::new(InMemoryBackend::new(name, c.profiles())),
            BackendConfig::Emulator(c) => Arc::new(EmulatorBackend::new(name, c)?),
            BackendConfig::Live(c) => Arc::new(LiveBackend::new(name, c)?),
        })
    }
}

/// Configuration for the in-memory emulator backend.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct InMemoryConfig {
    /// The backend name; defaults to `inmemory`.
    pub name: Option<String>,
    /// The service profiles to run; defaults to all of them.
    pub profiles: Option<Vec<ServiceProfile>>,
}

impl InMemoryConfig {
    /// The service profiles to run.
    pub fn profiles(&self) -> Vec<ServiceProfile> {
        self.profiles
            .clone()
            .unwrap_or_else(|| ServiceProfile::ALL.to_vec())
    }
}

/// Configuration for a running Azure Cosmos DB Emulator.
///
/// Everything is optional. The emulator only provides
/// [`SingleRegion`](ServiceProfile::SingleRegion).
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EmulatorConfig {
    /// The backend name; defaults to `emulator`.
    pub name: Option<String>,
    /// The emulator endpoint; defaults to [`EMULATOR_ENDPOINT`].
    pub endpoint: Option<String>,
    /// The service profiles to run. Only `SingleRegion` is allowed, and it's the default.
    pub profiles: Option<Vec<ServiceProfile>>,
}

impl EmulatorConfig {
    /// The emulator endpoint.
    pub fn endpoint(&self) -> &str {
        self.endpoint.as_deref().unwrap_or(EMULATOR_ENDPOINT)
    }

    /// The service profiles to run.
    pub fn profiles(&self) -> Vec<ServiceProfile> {
        self.profiles
            .clone()
            .unwrap_or_else(|| vec![ServiceProfile::SingleRegion])
    }

    fn validate(&self) -> SpecResult {
        if let Some(other) = self
            .profiles()
            .into_iter()
            .find(|p| *p != ServiceProfile::SingleRegion)
        {
            return Err(format!(
                "the emulator backend only provides SingleRegion, but {other} was configured"
            )
            .into());
        }
        Ok(())
    }
}

/// Configuration for existing live accounts.
///
/// Each configured service profile names an account shaped like that profile,
/// for example one deployed by `infra/main.bicep`. Only configured profiles run.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveConfig {
    /// The backend name; defaults to `live`.
    pub name: Option<String>,
    /// The account for each service profile to run.
    #[serde(default)]
    pub profiles: BTreeMap<ServiceProfile, LiveAccountConfig>,
}

impl LiveConfig {
    fn validate(&self, name: &str) -> SpecResult {
        if self.profiles.is_empty() {
            return Err(format!(
                "live backend '{name}' needs at least one [backends.profiles.<Profile>] table"
            )
            .into());
        }
        for (profile, account) in &self.profiles {
            account
                .validate()
                .map_err(|e| format!("live backend '{name}', profile {profile}: {e}"))?;
        }
        Ok(())
    }
}

/// One live account.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LiveAccountConfig {
    /// The account endpoint, for example `https://my-account.documents.azure.com:443/`.
    pub endpoint: String,
    /// How to authenticate.
    pub auth: LiveAuth,
}

impl LiveAccountConfig {
    fn validate(&self) -> SpecResult {
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
    use super::{BackendConfig, LiveAuth, ServiceProfile, SpecConfig};

    #[test]
    fn default_runs_every_profile_in_memory() {
        let config = SpecConfig::default();
        assert_eq!(config.backends.len(), 1);
        let BackendConfig::InMemory(in_memory) = &config.backends[0] else {
            panic!("expected the in-memory backend");
        };
        assert_eq!(in_memory.profiles(), ServiceProfile::ALL);
        assert_eq!(config.backends[0].name(), "inmemory");
    }

    #[test]
    fn parses_every_backend_kind() {
        let config = SpecConfig::parse(
            r#"
            [[backends]]
            kind = "inmemory"
            profiles = ["SingleRegion", "MultiWrite"]

            [[backends]]
            kind = "emulator"

            [[backends]]
            kind = "live"
            [backends.profiles.SingleRegion]
            endpoint = "https://a.documents.azure.com:443/"
            auth = { kind = "key", key_env = "A_KEY" }
            [backends.profiles.MultiRead]
            endpoint = "https://b.documents.azure.com:443/"
            auth = { kind = "entra" }
            "#,
        )
        .unwrap();
        let names: Vec<_> = config.backends.iter().map(BackendConfig::name).collect();
        assert_eq!(names, ["inmemory", "emulator", "live"]);
        let BackendConfig::Live(live) = &config.backends[2] else {
            panic!("expected the live backend");
        };
        assert_eq!(
            live.profiles.keys().copied().collect::<Vec<_>>(),
            [ServiceProfile::SingleRegion, ServiceProfile::MultiRead]
        );
        assert_eq!(
            live.profiles[&ServiceProfile::MultiRead].auth,
            LiveAuth::Entra
        );
    }

    #[test]
    fn emulator_rejects_profiles_other_than_single_region() {
        let error = SpecConfig::parse(
            r#"
            [[backends]]
            kind = "emulator"
            profiles = ["SingleRegion", "MultiRead"]
            "#,
        )
        .unwrap_err();
        assert!(error.to_string().contains("only provides SingleRegion"));
    }

    #[test]
    fn rejects_unknown_fields_and_profiles() {
        assert!(SpecConfig::parse("[[backends]]\nkind = \"inmemory\"\nprofile = []\n").is_err());
        assert!(
            SpecConfig::parse("[[backends]]\nkind = \"inmemory\"\nprofiles = [\"Nope\"]\n")
                .is_err()
        );
        assert!(SpecConfig::parse("[[backends]]\nkind = \"hosted\"\n").is_err());
    }

    #[test]
    fn rejects_duplicate_backend_names() {
        let error = SpecConfig::parse(
            "[[backends]]\nkind = \"inmemory\"\n[[backends]]\nkind = \"inmemory\"\n",
        )
        .unwrap_err();
        assert!(error.to_string().contains("used twice"));
        SpecConfig::parse(
            "[[backends]]\nkind = \"inmemory\"\n[[backends]]\nkind = \"inmemory\"\nname = \"inmemory-2\"\n",
        )
        .unwrap();
    }

    #[test]
    fn live_needs_profiles_and_one_key_source() {
        assert!(SpecConfig::parse("[[backends]]\nkind = \"live\"\n").is_err());
        let error = SpecConfig::parse(
            r#"
            [[backends]]
            kind = "live"
            [backends.profiles.SingleRegion]
            endpoint = "https://a.documents.azure.com:443/"
            auth = { kind = "key" }
            "#,
        )
        .unwrap_err();
        assert!(error.to_string().contains("exactly one"));
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
            [[backends]]
            kind = "live"
            [backends.profiles.SingleRegion]
            endpoint = "https://a.documents.azure.com:443/"
            auth = { kind = "key", key = "super-secret" }
            "#,
        )
        .unwrap();
        assert!(!format!("{config:?}").contains("super-secret"));
    }
}
