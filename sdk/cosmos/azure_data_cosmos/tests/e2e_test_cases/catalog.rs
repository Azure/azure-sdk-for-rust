// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;
use serde_json::Value;

const DEFAULT_PROFILE: &str = "hostedEmulatorSmoke";
const SCENARIO_SCHEMA_REFERENCE: &str = "../../schema/scenario.v1.json";
const PROFILE_SCHEMA_REFERENCE: &str = "../schema/profile.v1.json";
const BACKENDS: [&str; 3] = [
    "azureLive",
    "hostedEmulatorGatewayV1",
    "hostedEmulatorGatewayV2",
];

const SCENARIOS: &[&str] = &[
    include_str!("../../../e2e_tests/scenarios/management/capabilities.json"),
    include_str!("../../../e2e_tests/scenarios/bootstrap/primary-success.json"),
    include_str!("../../../e2e_tests/scenarios/items/lifecycle.json"),
    include_str!("../../../e2e_tests/scenarios/items/upsert-create-update.json"),
    include_str!("../../../e2e_tests/scenarios/items/create-conflict.json"),
    include_str!("../../../e2e_tests/scenarios/items/not-found-wrong-partition-key.json"),
    include_str!("../../../e2e_tests/scenarios/items/optimistic-concurrency.json"),
    include_str!("../../../e2e_tests/scenarios/queries/parameterized-filter.json"),
    include_str!("../../../e2e_tests/scenarios/queries/invalid-syntax.json"),
    include_str!("../../../e2e_tests/scenarios/diagnostics/success-and-error.json"),
];

const PROFILES: &[&str] = &[
    include_str!("../../../e2e_tests/profiles/hostedEmulatorSmoke.json"),
    include_str!("../../../e2e_tests/profiles/targetDefault.json"),
    include_str!("../../../e2e_tests/profiles/legacyGatewayV1.json"),
    include_str!("../../../e2e_tests/profiles/lifecycleConsistencyMatrix.json"),
    include_str!("../../../e2e_tests/profiles/readConsistencyOverrideMatrix.json"),
];

const RUST_IMPLEMENTATIONS: &str = include_str!("../../../e2e_tests/implementations/rust.json");
const CONSISTENCY_MATRIX: &str = include_str!("../../../e2e-consistency-matrix.json");
const OVERRIDE_MATRIX: &str = include_str!("../../../e2e-read-consistency-override-matrix.json");
const SCENARIO_SCHEMA: &str = include_str!("../../../e2e_tests/schema/scenario.v1.json");
const PROFILE_SCHEMA: &str = include_str!("../../../e2e_tests/schema/profile.v1.json");

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Scenario {
    #[serde(rename = "$schema")]
    schema: String,
    spec_version: String,
    id: String,
    title: String,
    requirement: String,
    maturity: Maturity,
    precedents: Vec<Precedent>,
    profiles: Vec<String>,
    tags: Vec<String>,
    backends: BTreeMap<String, Backend>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Maturity {
    Candidate,
    Stable,
    Deprecated,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Precedent {
    sdk: ReferenceSdk,
    path: String,
    test: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ReferenceSdk {
    Service,
    Rust,
    Java,
    Dotnet,
    Python,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Backend {
    applicability: Applicability,
    fidelity: Fidelity,
    reason: Option<String>,
    #[serde(default)]
    requires: Vec<Capability>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
enum Capability {
    Capabilities,
    GatewayV2,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
enum Applicability {
    Required,
    Supported,
    Simulated,
    NotApplicable,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum Fidelity {
    Full,
    Partial,
    Simulated,
    None,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Profile {
    #[serde(rename = "$schema")]
    schema: String,
    spec_version: String,
    pub id: String,
    pub accounts: Vec<AccountDefinition>,
    pub runtimes: Vec<RuntimeDefinition>,
    pub clients: Vec<ClientDefinition>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AccountDefinition {
    pub id: String,
    write_mode: String,
    pub consistency: String,
    regions: Vec<RegionDefinition>,
    replication: ReplicationDefinition,
    per_partition_failover: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RegionDefinition {
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ReplicationDefinition {
    min_delay_ms: u64,
    max_delay_ms: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuntimeDefinition {
    pub id: String,
    pub gateway_v2: String,
    pub ppcb: String,
    pub default_read_consistency_strategy: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClientDefinition {
    pub id: String,
    pub binary_encoding: String,
    pub routing: String,
    pub default_read_consistency_strategy: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ImplementationMap {
    spec_version: String,
    sdk: String,
    test_target: String,
    scenarios: Vec<Implementation>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Implementation {
    id: String,
    test: String,
    status: ImplementationStatus,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
enum ImplementationStatus {
    Active,
    Planned,
    Unsupported,
}

impl Profile {
    pub fn account(&self, id: &str) -> &AccountDefinition {
        self.accounts
            .iter()
            .find(|definition| definition.id == id)
            .unwrap_or_else(|| panic!("profile '{}' has no account '{id}'", self.id))
    }

    pub fn runtime(&self, id: &str) -> &RuntimeDefinition {
        self.runtimes
            .iter()
            .find(|definition| definition.id == id)
            .unwrap_or_else(|| panic!("profile '{}' has no runtime '{id}'", self.id))
    }

    pub fn client(&self, id: &str) -> &ClientDefinition {
        self.clients
            .iter()
            .find(|definition| definition.id == id)
            .unwrap_or_else(|| panic!("profile '{}' has no client '{id}'", self.id))
    }
}

fn load_scenarios() -> Result<Vec<Scenario>, String> {
    SCENARIOS
        .iter()
        .map(|json| serde_json::from_str(json).map_err(|error| error.to_string()))
        .collect()
}

fn load_profiles() -> Result<Vec<Profile>, String> {
    PROFILES
        .iter()
        .map(|json| serde_json::from_str(json).map_err(|error| error.to_string()))
        .collect()
}

pub fn selected_profile_for(scenario_id: &str) -> Result<Option<Profile>, String> {
    let selected =
        std::env::var("AZURE_COSMOS_E2E_PROFILE").unwrap_or_else(|_| DEFAULT_PROFILE.to_owned());
    let scenarios = load_scenarios()?;
    let scenario = scenarios
        .iter()
        .find(|scenario| scenario.id == scenario_id)
        .ok_or_else(|| format!("E2E scenario '{scenario_id}' does not exist"))?;
    let profiles = load_profiles()?;
    if !profiles.iter().any(|profile| profile.id == selected) {
        return Err(format!("E2E profile '{selected}' does not exist"));
    }
    if !scenario.profiles.contains(&selected) {
        return Ok(None);
    }
    Ok(profiles.into_iter().find(|profile| profile.id == selected))
}

pub fn validate_catalog(implemented_tests: &[&str]) -> Result<(), String> {
    for (name, schema) in [("scenario", SCENARIO_SCHEMA), ("profile", PROFILE_SCHEMA)] {
        let schema: Value = serde_json::from_str(schema)
            .map_err(|error| format!("{name} schema is not JSON: {error}"))?;
        if schema.get("$schema").and_then(Value::as_str)
            != Some("https://json-schema.org/draft/2020-12/schema")
        {
            return Err(format!("{name} schema must use JSON Schema draft 2020-12"));
        }
    }

    let profiles = load_profiles()?;
    let mut profile_ids = BTreeSet::new();
    for profile in &profiles {
        if profile.schema != PROFILE_SCHEMA_REFERENCE
            || profile.spec_version != "1.0"
            || !valid_camel_id(&profile.id)
            || !profile_ids.insert(profile.id.as_str())
        {
            return Err(format!("invalid or duplicate profile '{}'", profile.id));
        }
        for (axis, ids) in [
            (
                "account",
                profile
                    .accounts
                    .iter()
                    .map(|definition| definition.id.as_str())
                    .collect::<Vec<_>>(),
            ),
            (
                "runtime",
                profile
                    .runtimes
                    .iter()
                    .map(|definition| definition.id.as_str())
                    .collect::<Vec<_>>(),
            ),
            (
                "client",
                profile
                    .clients
                    .iter()
                    .map(|definition| definition.id.as_str())
                    .collect::<Vec<_>>(),
            ),
        ] {
            let unique: BTreeSet<_> = ids.iter().copied().collect();
            if ids.is_empty() || unique.len() != ids.len() {
                return Err(format!(
                    "profile '{}' has an empty or duplicate {axis} axis",
                    profile.id
                ));
            }
        }
        for account in &profile.accounts {
            if account.regions.is_empty()
                || account.replication.min_delay_ms > account.replication.max_delay_ms
                || !matches!(account.write_mode.as_str(), "single" | "multi")
                || !matches!(
                    account.consistency.as_str(),
                    "strong" | "boundedStaleness" | "session" | "consistentPrefix" | "eventual"
                )
            {
                return Err(format!(
                    "profile '{}' account '{}' has invalid regions or replication delay",
                    profile.id, account.id
                ));
            }
        }
        if profile.runtimes.iter().any(|runtime| {
            !matches!(
                runtime.gateway_v2.as_str(),
                "enabled" | "disabled" | "backendDefault"
            ) || !matches!(runtime.ppcb.as_str(), "enabled" | "disabled" | "sdkDefault")
                || !valid_optional_read_strategy(
                    runtime.default_read_consistency_strategy.as_deref(),
                )
        }) || profile.clients.iter().any(|client| {
            !matches!(
                client.binary_encoding.as_str(),
                "enabled" | "disabled" | "sdkDefault"
            ) || !matches!(
                client.routing.as_str(),
                "proximity" | "preferredRegions" | "accountOrder"
            ) || !valid_optional_read_strategy(client.default_read_consistency_strategy.as_deref())
        }) {
            return Err(format!(
                "profile '{}' has invalid setup options",
                profile.id
            ));
        }
    }
    validate_pipeline_matrix(
        CONSISTENCY_MATRIX,
        profiles
            .iter()
            .find(|profile| profile.id == "lifecycleConsistencyMatrix")
            .expect("consistency profile must be registered"),
    )?;
    validate_pipeline_matrix(
        OVERRIDE_MATRIX,
        profiles
            .iter()
            .find(|profile| profile.id == "readConsistencyOverrideMatrix")
            .expect("override profile must be registered"),
    )?;

    let scenarios = load_scenarios()?;
    let mut scenario_ids = BTreeSet::new();
    for scenario in &scenarios {
        if scenario.schema != SCENARIO_SCHEMA_REFERENCE
            || scenario.spec_version != "1.0"
            || !valid_scenario_id(&scenario.id)
        {
            return Err(format!(
                "scenario '{}' has an unsupported version",
                scenario.id
            ));
        }
        if !scenario_ids.insert(scenario.id.as_str()) {
            return Err(format!("duplicate scenario id '{}'", scenario.id));
        }
        if scenario.title.is_empty()
            || scenario.requirement.is_empty()
            || scenario.precedents.is_empty()
            || scenario.profiles.is_empty()
            || scenario.tags.is_empty()
            || scenario
                .precedents
                .iter()
                .any(|precedent| precedent.path.is_empty() || precedent.test.is_empty())
        {
            return Err(format!(
                "scenario '{}' is missing required metadata",
                scenario.id
            ));
        }
        let selected_profiles: BTreeSet<_> = scenario.profiles.iter().map(String::as_str).collect();
        let tags: BTreeSet<_> = scenario.tags.iter().map(String::as_str).collect();
        if selected_profiles.len() != scenario.profiles.len()
            || !selected_profiles.is_subset(&profile_ids)
            || tags.len() != scenario.tags.len()
        {
            return Err(format!(
                "scenario '{}' references an unknown or duplicate profile",
                scenario.id
            ));
        }
        let backend_names: BTreeSet<_> = scenario.backends.keys().map(String::as_str).collect();
        if backend_names != BACKENDS.into_iter().collect() {
            return Err(format!(
                "scenario '{}' has incomplete backend applicability",
                scenario.id
            ));
        }
        for (backend_name, backend) in &scenario.backends {
            if backend.applicability == Applicability::NotApplicable && backend.reason.is_none() {
                return Err(format!(
                    "scenario '{}' must explain why '{backend_name}' is not applicable",
                    scenario.id
                ));
            }
            let requirements: BTreeSet<_> = backend.requires.iter().collect();
            if requirements.len() != backend.requires.len() {
                return Err(format!(
                    "scenario '{}' has duplicate requirements for '{backend_name}'",
                    scenario.id
                ));
            }
        }
    }
    let referenced_profiles: BTreeSet<_> = scenarios
        .iter()
        .flat_map(|scenario| scenario.profiles.iter().map(String::as_str))
        .collect();
    if let Ok(selected_profile) = std::env::var("AZURE_COSMOS_E2E_PROFILE") {
        if !profile_ids.contains(selected_profile.as_str()) {
            return Err(format!(
                "selected E2E profile '{selected_profile}' does not exist"
            ));
        }
        if !referenced_profiles.contains(selected_profile.as_str()) {
            return Err(format!(
                "selected E2E profile '{selected_profile}' has no active scenarios"
            ));
        }
    }

    let implementations: ImplementationMap =
        serde_json::from_str(RUST_IMPLEMENTATIONS).map_err(|error| error.to_string())?;
    if implementations.spec_version != "1.0"
        || implementations.sdk != "rust"
        || implementations.test_target != "e2e_tests"
    {
        return Err("invalid Rust implementation map header".to_owned());
    }
    let known_tests: BTreeSet<_> = implemented_tests.iter().copied().collect();
    let mut mapped_ids = BTreeSet::new();
    for implementation in &implementations.scenarios {
        if !scenario_ids.contains(implementation.id.as_str()) {
            return Err(format!(
                "implementation references unknown scenario '{}'",
                implementation.id
            ));
        }
        if !mapped_ids.insert(implementation.id.as_str()) {
            return Err(format!(
                "scenario '{}' is mapped more than once",
                implementation.id
            ));
        }
        if implementation.status == ImplementationStatus::Active
            && !known_tests.contains(implementation.test.as_str())
        {
            return Err(format!(
                "scenario '{}' references missing test '{}'",
                implementation.id, implementation.test
            ));
        }
    }
    if mapped_ids != scenario_ids {
        return Err("every scenario must have exactly one Rust implementation mapping".to_owned());
    }
    Ok(())
}

fn valid_optional_read_strategy(strategy: Option<&str>) -> bool {
    strategy.is_none_or(|strategy| {
        matches!(
            strategy,
            "Default" | "Eventual" | "Session" | "LatestCommitted" | "GlobalStrong"
        )
    })
}

fn validate_pipeline_matrix(json: &str, profile: &Profile) -> Result<(), String> {
    let document: Value = serde_json::from_str(json).map_err(|error| error.to_string())?;
    let matrix = document
        .get("matrix")
        .and_then(Value::as_object)
        .ok_or("pipeline matrix must contain an object named 'matrix'")?;
    let actual_profiles = matrix_axis(matrix, "AZURE_COSMOS_E2E_PROFILE")?;
    if actual_profiles != BTreeSet::from([profile.id.as_str()]) {
        return Err(format!(
            "pipeline matrix must select only profile '{}'",
            profile.id
        ));
    }
    for (axis, actual, expected) in [
        (
            "account",
            matrix_axis(matrix, "AZURE_COSMOS_E2E_ACCOUNT")?,
            profile
                .accounts
                .iter()
                .map(|definition| definition.id.as_str())
                .collect(),
        ),
        (
            "runtime",
            matrix_axis(matrix, "AZURE_COSMOS_E2E_RUNTIME")?,
            profile
                .runtimes
                .iter()
                .map(|definition| definition.id.as_str())
                .collect(),
        ),
        (
            "client",
            matrix_axis(matrix, "AZURE_COSMOS_E2E_CLIENT")?,
            profile
                .clients
                .iter()
                .map(|definition| definition.id.as_str())
                .collect(),
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "pipeline matrix for '{}' does not cover its {axis} axis: expected {expected:?}, got {actual:?}",
                profile.id
            ));
        }
    }
    let flavors = matrix_axis(matrix, "AZURE_COSMOS_EMULATOR_FLAVOR")?;
    if flavors != BTreeSet::from(["inmemory-v1", "inmemory-v2"]) {
        return Err(format!(
            "pipeline matrix for '{}' must cover Gateway V1 and Gateway V2",
            profile.id
        ));
    }
    Ok(())
}

fn matrix_axis<'a>(
    matrix: &'a serde_json::Map<String, Value>,
    name: &str,
) -> Result<BTreeSet<&'a str>, String> {
    matrix
        .get(name)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("pipeline matrix is missing '{name}'"))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .ok_or_else(|| format!("pipeline matrix axis '{name}' must contain strings"))
        })
        .collect()
}

fn valid_camel_id(value: &str) -> bool {
    value
        .chars()
        .next()
        .is_some_and(|first| first.is_ascii_lowercase())
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric())
}

fn valid_scenario_id(value: &str) -> bool {
    let mut segments = value.split('.');
    let first = segments.next();
    let rest: Vec<_> = segments.collect();
    first.is_some_and(valid_slug) && !rest.is_empty() && rest.into_iter().all(valid_slug)
}

fn valid_slug(value: &str) -> bool {
    value
        .chars()
        .next()
        .is_some_and(|first| first.is_ascii_lowercase())
        && value.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
}
