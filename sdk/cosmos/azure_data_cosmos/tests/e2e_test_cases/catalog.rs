// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(dead_code)]

use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
};

use azure_data_cosmos::{
    diagnostics::DiagnosticsContext,
    models::{PartitionKeyDefinition, PartitionKeyKind, PartitionKeyValue, PartitionKeyVersion},
    PartitionKey,
};
use serde::Deserialize;
use serde_json::Value;

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

const VOCABULARY: &str = include_str!("../../../e2e_tests/vocabulary/v1.json");
const RUST_IMPLEMENTATIONS: &str = include_str!("../../../e2e_tests/implementations/rust.json");
const SCENARIO_SCHEMA: &str = include_str!("../../../e2e_tests/schema/scenario.v1.json");
const PROFILE_SCHEMA: &str = include_str!("../../../e2e_tests/schema/profile.v1.json");

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Scenario {
    #[serde(rename = "$schema")]
    schema: String,
    spec_version: String,
    pub id: String,
    title: String,
    requirement: String,
    maturity: String,
    precedents: Vec<Precedent>,
    profile: Option<String>,
    tags: Vec<String>,
    backends: BTreeMap<String, Backend>,
    pub fixtures: Vec<Fixture>,
    #[serde(default)]
    pub executions: Vec<Execution>,
    pub steps: Vec<Step>,
    cleanup: Cleanup,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Precedent {
    sdk: String,
    path: String,
    test: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Backend {
    applicability: String,
    fidelity: String,
    reason: Option<String>,
    #[serde(default)]
    requires: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fixture {
    pub id: String,
    container: ContainerSetup,
    pub items: Vec<FixtureItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ContainerSetup {
    partition_key: PartitionKeySetup,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PartitionKeySetup {
    paths: Vec<String>,
    kind: String,
    version: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FixtureItem {
    pub id: String,
    pub seed: bool,
    partition_key_values: Vec<Value>,
    pub document: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Execution {
    pub id: String,
    pub profile: String,
    pub account: String,
    pub runtime: String,
    pub client: String,
    pub read_consistency_strategy: String,
    pub read_region: String,
    pub session_token: String,
    pub expected_read: ExpectedRead,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExpectedRead {
    pub acceptable_initial_statuses: Vec<ExpectedStatus>,
    pub terminal_status: ExpectedStatus,
    pub max_wait_ms: Option<u64>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExpectedStatus {
    pub status_code: u16,
    pub sub_status_code: Option<u16>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Step {
    pub id: String,
    action: Action,
    pub expected: Expected,
    pub diagnostics: Option<DiagnosticsExpectation>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Action {
    kind: String,
    operation: String,
    input: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Expected {
    outcome: String,
    pub status: u16,
    pub sub_status: Option<u16>,
    error_category: Option<String>,
    state: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiagnosticsExpectation {
    operation_name: Option<Comparison>,
    activity_id: Option<Comparison>,
    effective_status: Option<Comparison>,
    request_count: Option<Comparison>,
    regions_contacted: Option<Comparison>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Comparison {
    comparator: String,
    value: Option<Value>,
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
    gateway_v2: String,
    ppcb: String,
    pub default_read_consistency_strategy: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClientDefinition {
    pub id: String,
    binary_encoding: String,
    routing: String,
    pub default_read_consistency_strategy: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Vocabulary {
    spec_version: String,
    backends: Vec<String>,
    applicability: Vec<String>,
    maturity: Vec<String>,
    operations: Vec<String>,
    error_categories: Vec<String>,
    diagnostic_comparators: Vec<String>,
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
    status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Cleanup {
    delete_database: bool,
}

pub fn load_scenarios() -> Result<Vec<Scenario>, String> {
    SCENARIOS
        .iter()
        .map(|json| serde_json::from_str(json).map_err(|error| error.to_string()))
        .collect()
}

pub fn scenario(id: &str) -> Scenario {
    load_scenarios()
        .expect("E2E scenario catalog must deserialize")
        .into_iter()
        .find(|scenario| scenario.id == id)
        .unwrap_or_else(|| panic!("E2E scenario '{id}' does not exist"))
}

pub fn profile(id: &str) -> Profile {
    PROFILES
        .iter()
        .map(|json| serde_json::from_str::<Profile>(json).expect("E2E profile must deserialize"))
        .find(|profile| profile.id == id)
        .unwrap_or_else(|| panic!("E2E profile '{id}' does not exist"))
}

impl Scenario {
    pub fn step(&self, id: &str) -> &Step {
        self.steps
            .iter()
            .find(|step| step.id == id)
            .unwrap_or_else(|| panic!("scenario '{}' has no step '{id}'", self.id))
    }

    pub fn fixture(&self, id: &str) -> &Fixture {
        self.fixtures
            .iter()
            .find(|fixture| fixture.id == id)
            .unwrap_or_else(|| panic!("scenario '{}' has no fixture '{id}'", self.id))
    }

    pub fn executions_for_configuration<'a>(
        &'a self,
        profile: &'a str,
        account: &'a str,
        runtime: &'a str,
        client: &'a str,
    ) -> impl Iterator<Item = &'a Execution> + 'a {
        self.executions.iter().filter(move |execution| {
            execution.profile == profile
                && execution.account == account
                && execution.runtime == runtime
                && execution.client == client
        })
    }
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

impl Fixture {
    pub fn item(&self, id: &str) -> &FixtureItem {
        self.items
            .iter()
            .find(|item| item.id == id)
            .unwrap_or_else(|| panic!("fixture '{}' has no item '{id}'", self.id))
    }

    pub fn partition_key_definition(&self) -> Result<PartitionKeyDefinition, String> {
        let kind = match self.container.partition_key.kind.as_str() {
            "Hash" => PartitionKeyKind::Hash,
            "MultiHash" => PartitionKeyKind::MultiHash,
            kind => {
                return Err(format!(
                    "fixture '{}' has unknown PK kind '{kind}'",
                    self.id
                ))
            }
        };
        let version = match self.container.partition_key.version {
            1 => PartitionKeyVersion::V1,
            2 => PartitionKeyVersion::V2,
            version => return Err(format!("fixture '{}' has PK version {version}", self.id)),
        };
        Ok(PartitionKeyDefinition::new(
            self.container
                .partition_key
                .paths
                .iter()
                .cloned()
                .map(Cow::Owned)
                .collect(),
        )
        .with_kind(kind)
        .with_version(version))
    }
}

impl FixtureItem {
    pub fn partition_key(&self) -> Result<PartitionKey, String> {
        let values = self
            .partition_key_values
            .iter()
            .map(partition_key_value)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(PartitionKey::from(values))
    }

    pub fn document_id(&self) -> Result<&str, String> {
        self.document
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("fixture item '{}' has no string document id", self.id))
    }
}

impl ExpectedStatus {
    pub fn matches(&self, status_code: u16, sub_status_code: Option<u16>) -> bool {
        self.status_code == status_code
            && self
                .sub_status_code
                .is_none_or(|expected| expected == sub_status_code.unwrap_or(0))
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.status_code == other.status_code
            && (self.sub_status_code.is_none()
                || other.sub_status_code.is_none()
                || self.sub_status_code == other.sub_status_code)
    }
}

impl ExpectedRead {
    pub fn is_terminal(&self, status_code: u16, sub_status_code: Option<u16>) -> bool {
        self.terminal_status.matches(status_code, sub_status_code)
    }

    pub fn is_acceptable_initial(&self, status_code: u16, sub_status_code: Option<u16>) -> bool {
        self.acceptable_initial_statuses
            .iter()
            .any(|expected| expected.matches(status_code, sub_status_code))
    }
}

fn partition_key_value(value: &Value) -> Result<PartitionKeyValue, String> {
    match value {
        Value::String(value) => Ok(value.clone().into()),
        Value::Number(value) => value
            .as_f64()
            .map(PartitionKeyValue::from)
            .ok_or_else(|| format!("partition key number '{value}' is not finite")),
        Value::Bool(value) => Ok((*value).into()),
        Value::Null => Ok(PartitionKey::NULL),
        value => Err(format!("unsupported partition key value '{value}'")),
    }
}

pub fn validate_catalog(implemented_tests: &[&str]) -> Result<(), String> {
    let vocabulary: Vocabulary =
        serde_json::from_str(VOCABULARY).map_err(|error| error.to_string())?;
    if vocabulary.spec_version != "1.0" {
        return Err("unsupported vocabulary version".to_owned());
    }

    for (name, schema) in [("scenario", SCENARIO_SCHEMA), ("profile", PROFILE_SCHEMA)] {
        let schema: Value = serde_json::from_str(schema)
            .map_err(|error| format!("{name} schema is not JSON: {error}"))?;
        if schema.get("$schema").and_then(Value::as_str)
            != Some("https://json-schema.org/draft/2020-12/schema")
        {
            return Err(format!("{name} schema must use JSON Schema draft 2020-12"));
        }
    }

    let profiles: Vec<Profile> = PROFILES
        .iter()
        .map(|json| serde_json::from_str(json).map_err(|error| error.to_string()))
        .collect::<Result<_, _>>()?;
    let mut profile_ids = BTreeSet::new();
    for profile in &profiles {
        if profile.spec_version != "1.0" || !profile_ids.insert(profile.id.as_str()) {
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
            {
                return Err(format!(
                    "profile '{}' account '{}' has invalid regions or replication delay",
                    profile.id, account.id
                ));
            }
        }
    }

    let scenarios = load_scenarios()?;
    let mut scenario_ids = BTreeSet::new();
    for scenario in &scenarios {
        if scenario.spec_version != "1.0" {
            return Err(format!(
                "scenario '{}' has an unsupported version",
                scenario.id
            ));
        }
        if !scenario_ids.insert(scenario.id.as_str()) {
            return Err(format!("duplicate scenario id '{}'", scenario.id));
        }
        if let Some(profile) = &scenario.profile {
            if !profile_ids.contains(profile.as_str()) {
                return Err(format!(
                    "scenario '{}' references unknown profile '{}'",
                    scenario.id, profile
                ));
            }
        }
        if scenario.profile.is_some() != scenario.executions.is_empty() {
            return Err(format!(
                "scenario '{}' must declare either one profile or an execution matrix",
                scenario.id
            ));
        }
        if !vocabulary.maturity.contains(&scenario.maturity) {
            return Err(format!("scenario '{}' has unknown maturity", scenario.id));
        }
        if scenario.precedents.is_empty()
            || scenario.tags.is_empty()
            || scenario.fixtures.is_empty()
            || scenario.steps.is_empty()
        {
            return Err(format!(
                "scenario '{}' is missing required evidence",
                scenario.id
            ));
        }
        let mut fixture_ids = BTreeSet::new();
        for fixture in &scenario.fixtures {
            if !fixture_ids.insert(fixture.id.as_str()) {
                return Err(format!(
                    "scenario '{}' has duplicate fixture '{}'",
                    scenario.id, fixture.id
                ));
            }
            let definition = fixture.partition_key_definition()?;
            let path_count = definition.paths().len();
            if path_count == 0
                || (definition.kind() == PartitionKeyKind::Hash && path_count != 1)
                || (definition.kind() == PartitionKeyKind::MultiHash
                    && (path_count < 2 || definition.version() != PartitionKeyVersion::V2))
            {
                return Err(format!(
                    "scenario '{}' fixture '{}' has an invalid partition key definition",
                    scenario.id, fixture.id
                ));
            }
            let mut item_ids = BTreeSet::new();
            for item in &fixture.items {
                if !item_ids.insert(item.id.as_str()) {
                    return Err(format!(
                        "scenario '{}' fixture '{}' has duplicate item '{}'",
                        scenario.id, fixture.id, item.id
                    ));
                }
                if item.partition_key_values.len() != path_count {
                    return Err(format!(
                        "scenario '{}' fixture '{}' item '{}' has {} partition-key values for {} paths",
                        scenario.id,
                        fixture.id,
                        item.id,
                        item.partition_key_values.len(),
                        path_count
                    ));
                }
                item.partition_key()?;
                item.document_id()?;
            }
        }
        let mut execution_ids = BTreeSet::new();
        for execution in &scenario.executions {
            if !execution_ids.insert(execution.id.as_str()) {
                return Err(format!(
                    "scenario '{}' has duplicate execution '{}'",
                    scenario.id, execution.id
                ));
            }
            if !profile_ids.contains(execution.profile.as_str()) {
                return Err(format!(
                    "scenario '{}' execution '{}' references unknown profile '{}'",
                    scenario.id, execution.id, execution.profile
                ));
            }
            let selected_profile = profiles
                .iter()
                .find(|profile| profile.id == execution.profile)
                .expect("profile existence checked above");
            if !selected_profile
                .accounts
                .iter()
                .any(|definition| definition.id == execution.account)
                || !selected_profile
                    .runtimes
                    .iter()
                    .any(|definition| definition.id == execution.runtime)
                || !selected_profile
                    .clients
                    .iter()
                    .any(|definition| definition.id == execution.client)
            {
                return Err(format!(
                    "scenario '{}' execution '{}' references a missing profile matrix cell",
                    scenario.id, execution.id
                ));
            }
            let expected = &execution.expected_read;
            let unique_initial_statuses: BTreeSet<_> =
                expected.acceptable_initial_statuses.iter().collect();
            if unique_initial_statuses.len() != expected.acceptable_initial_statuses.len()
                || expected
                    .acceptable_initial_statuses
                    .iter()
                    .enumerate()
                    .any(|(index, status)| {
                        expected.acceptable_initial_statuses[index + 1..]
                            .iter()
                            .any(|other| status.overlaps(other))
                    })
                || expected
                    .acceptable_initial_statuses
                    .iter()
                    .chain(std::iter::once(&expected.terminal_status))
                    .any(|status| !(100..=599).contains(&status.status_code))
                || expected
                    .acceptable_initial_statuses
                    .iter()
                    .any(|status| status.overlaps(&expected.terminal_status))
            {
                return Err(format!(
                    "scenario '{}' execution '{}' has invalid or overlapping status expectations",
                    scenario.id, execution.id
                ));
            }
            if expected.acceptable_initial_statuses.is_empty() {
                if expected.max_wait_ms.is_some() {
                    return Err(format!(
                        "scenario '{}' execution '{}' declares a wait without transient statuses",
                        scenario.id, execution.id
                    ));
                }
            } else if expected.max_wait_ms.is_none() {
                return Err(format!(
                    "scenario '{}' execution '{}' must bound retries for transient statuses",
                    scenario.id, execution.id
                ));
            }
            let selected_account = selected_profile
                .accounts
                .iter()
                .find(|definition| definition.id == execution.account)
                .expect("account existence checked above");
            if execution.read_consistency_strategy == "GlobalStrong"
                && selected_account.consistency != "strong"
                && (!expected.acceptable_initial_statuses.is_empty()
                    || !expected.is_terminal(400, None))
            {
                return Err(format!(
                    "scenario '{}' execution '{}' must reject GlobalStrong on a non-Strong profile",
                    scenario.id, execution.id
                ));
            }
        }
        if scenario.id == "item.lifecycle" {
            let required_strategies: BTreeSet<_> = [
                "Default",
                "Eventual",
                "Session",
                "LatestCommitted",
                "GlobalStrong",
            ]
            .into_iter()
            .collect();
            for account in [
                "strong",
                "boundedStaleness",
                "session",
                "consistentPrefix",
                "eventual",
            ] {
                let actual: BTreeSet<_> = scenario
                    .executions_for_configuration(
                        "lifecycleConsistencyMatrix",
                        account,
                        "unset",
                        "unset",
                    )
                    .map(|execution| execution.read_consistency_strategy.as_str())
                    .collect();
                if actual != required_strategies {
                    return Err(format!(
                        "item.lifecycle account '{account}' must cover every read consistency strategy; got {actual:?}"
                    ));
                }
            }
            let override_profile = profiles
                .iter()
                .find(|profile| profile.id == "readConsistencyOverrideMatrix")
                .expect("override profile must exist");
            for runtime in &override_profile.runtimes {
                for client in &override_profile.clients {
                    let actual: BTreeSet<_> = scenario
                        .executions_for_configuration(
                            "readConsistencyOverrideMatrix",
                            "session",
                            &runtime.id,
                            &client.id,
                        )
                        .map(|execution| execution.read_consistency_strategy.as_str())
                        .collect();
                    let required: BTreeSet<_> =
                        ["Inherit", "Default", "Eventual"].into_iter().collect();
                    if actual != required {
                        return Err(format!(
                            "item.lifecycle override cell runtime='{}' client='{}' has incomplete operation coverage: {actual:?}",
                            runtime.id, client.id
                        ));
                    }
                }
            }
        }
        if scenario.backends.len() != vocabulary.backends.len() {
            return Err(format!(
                "scenario '{}' has incomplete backend applicability",
                scenario.id
            ));
        }
        for backend_name in &vocabulary.backends {
            let backend = scenario.backends.get(backend_name).ok_or_else(|| {
                format!("scenario '{}' omits backend '{backend_name}'", scenario.id)
            })?;
            if !vocabulary.applicability.contains(&backend.applicability) {
                return Err(format!(
                    "scenario '{}' has unknown applicability",
                    scenario.id
                ));
            }
            if backend.applicability == "notApplicable" && backend.reason.is_none() {
                return Err(format!(
                    "scenario '{}' must explain why '{backend_name}' is not applicable",
                    scenario.id
                ));
            }
        }

        let mut step_ids = BTreeSet::new();
        for step in &scenario.steps {
            if !step_ids.insert(step.id.as_str()) {
                return Err(format!(
                    "scenario '{}' has duplicate step '{}'",
                    scenario.id, step.id
                ));
            }
            if !vocabulary.operations.contains(&step.action.operation) {
                return Err(format!(
                    "scenario '{}' uses unknown operation '{}'",
                    scenario.id, step.action.operation
                ));
            }
            for item_ref in [
                step.action
                    .input
                    .as_ref()
                    .and_then(|input| input.get("itemRef"))
                    .and_then(Value::as_str),
                step.expected
                    .state
                    .as_ref()
                    .and_then(|state| state.get("itemRef"))
                    .and_then(Value::as_str),
            ]
            .into_iter()
            .flatten()
            {
                for fixture in &scenario.fixtures {
                    if !fixture.items.iter().any(|item| item.id == item_ref) {
                        return Err(format!(
                            "scenario '{}' step '{}' references missing item '{}' in fixture '{}'",
                            scenario.id, step.id, item_ref, fixture.id
                        ));
                    }
                }
            }
            if step.expected.outcome == "error"
                && step
                    .expected
                    .error_category
                    .as_ref()
                    .is_some_and(|category| !vocabulary.error_categories.contains(category))
            {
                return Err(format!(
                    "scenario '{}' has unknown error category",
                    scenario.id
                ));
            }
            if let Some(diagnostics) = &step.diagnostics {
                for comparison in diagnostics.comparisons() {
                    if !vocabulary
                        .diagnostic_comparators
                        .contains(&comparison.comparator)
                    {
                        return Err(format!(
                            "scenario '{}' uses unknown diagnostics comparator '{}'",
                            scenario.id, comparison.comparator
                        ));
                    }
                    let needs_value =
                        !matches!(comparison.comparator.as_str(), "present" | "absent");
                    if needs_value != comparison.value.is_some() {
                        return Err(format!(
                            "scenario '{}' comparator '{}' has an invalid value",
                            scenario.id, comparison.comparator
                        ));
                    }
                }
            }
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
        if implementation.status == "active" && !known_tests.contains(implementation.test.as_str())
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

impl DiagnosticsExpectation {
    fn comparisons(&self) -> impl Iterator<Item = &Comparison> {
        [
            self.operation_name.as_ref(),
            self.activity_id.as_ref(),
            self.effective_status.as_ref(),
            self.request_count.as_ref(),
            self.regions_contacted.as_ref(),
        ]
        .into_iter()
        .flatten()
    }
}

pub fn assert_status(step: &Step, actual: u16, sub_status: Option<u16>) {
    assert_eq!(
        actual, step.expected.status,
        "status for step '{}'",
        step.id
    );
    if let Some(expected) = step.expected.sub_status {
        assert_eq!(
            sub_status.unwrap_or(0),
            expected,
            "substatus for step '{}'",
            step.id
        );
    }
}

pub fn assert_diagnostics(step: &Step, diagnostics: &DiagnosticsContext) {
    let Some(contract) = &step.diagnostics else {
        return;
    };
    if let Some(expected) = &contract.operation_name {
        assert_string(
            expected,
            diagnostics.operation_name(),
            "operationName",
            &step.id,
        );
    }
    if let Some(expected) = &contract.activity_id {
        let actual = diagnostics.activity_id().to_string();
        assert_string(expected, Some(actual.as_str()), "activityId", &step.id);
    }
    if let Some(expected) = &contract.effective_status {
        let actual = diagnostics
            .effective_status()
            .map(|status| u16::from(status.status_code()));
        assert_number(expected, actual.map(u64::from), "effectiveStatus", &step.id);
    }
    if let Some(expected) = &contract.request_count {
        assert_number(
            expected,
            Some(diagnostics.request_count() as u64),
            "requestCount",
            &step.id,
        );
    }
    if let Some(expected) = &contract.regions_contacted {
        let contacted_regions = diagnostics.regions_contacted();
        let regions: Vec<_> = contacted_regions
            .iter()
            .map(|region| region.as_str())
            .collect();
        let expected_regions = expected
            .value
            .as_ref()
            .and_then(Value::as_array)
            .expect("region comparison requires an array");
        if expected.comparator == "contains" {
            for region in expected_regions {
                let region = region.as_str().expect("region must be a string");
                assert!(
                    regions.contains(&region),
                    "diagnostics field regionsContacted for step '{}' did not contain '{region}': {regions:?}",
                    step.id
                );
            }
        }
    }
}

fn assert_string(expected: &Comparison, actual: Option<&str>, field: &str, step: &str) {
    match expected.comparator.as_str() {
        "present" => assert!(
            actual.is_some_and(|value| !value.is_empty()),
            "{field} for step '{step}' must be present"
        ),
        "absent" => assert!(actual.is_none(), "{field} for step '{step}' must be absent"),
        "exact" => assert_eq!(
            actual,
            expected.value.as_ref().and_then(Value::as_str),
            "diagnostics field {field} for step '{step}'"
        ),
        comparator => panic!("unsupported string comparator '{comparator}' for {field}"),
    }
}

fn assert_number(expected: &Comparison, actual: Option<u64>, field: &str, step: &str) {
    let value = expected.value.as_ref().and_then(Value::as_u64);
    match expected.comparator.as_str() {
        "present" => assert!(
            actual.is_some(),
            "{field} for step '{step}' must be present"
        ),
        "absent" => assert!(actual.is_none(), "{field} for step '{step}' must be absent"),
        "exact" => assert_eq!(actual, value, "diagnostics field {field} for step '{step}'"),
        "atLeast" => assert!(
            actual.zip(value).is_some_and(|(actual, expected)| actual >= expected),
            "diagnostics field {field} for step '{step}' must be at least {value:?}, got {actual:?}"
        ),
        "atMost" => {
            assert!(
            actual.zip(value).is_some_and(|(actual, expected)| actual <= expected),
            "diagnostics field {field} for step '{step}' must be at most {value:?}, got {actual:?}"
        )
        }
        comparator => panic!("unsupported numeric comparator '{comparator}' for {field}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{ExpectedRead, ExpectedStatus};

    #[test]
    fn status_without_expected_substatus_matches_any_substatus() {
        let status = ExpectedStatus {
            status_code: 200,
            sub_status_code: None,
        };

        assert!(status.matches(200, None));
        assert!(status.matches(200, Some(1002)));
        assert!(!status.matches(404, None));
    }

    #[test]
    fn explicit_zero_substatus_matches_missing_response_substatus() {
        let status = ExpectedStatus {
            status_code: 404,
            sub_status_code: Some(0),
        };

        assert!(status.matches(404, None));
        assert!(status.matches(404, Some(0)));
        assert!(!status.matches(404, Some(1002)));
    }

    #[test]
    fn status_overlap_accounts_for_wildcard_substatus() {
        let wildcard = ExpectedStatus {
            status_code: 404,
            sub_status_code: None,
        };
        let explicit = ExpectedStatus {
            status_code: 404,
            sub_status_code: Some(1002),
        };
        let other_status = ExpectedStatus {
            status_code: 200,
            sub_status_code: None,
        };

        assert!(wildcard.overlaps(&explicit));
        assert!(explicit.overlaps(&wildcard));
        assert!(!wildcard.overlaps(&other_status));
    }

    #[test]
    fn read_expectation_distinguishes_transient_and_terminal_statuses() {
        let expected = ExpectedRead {
            acceptable_initial_statuses: vec![ExpectedStatus {
                status_code: 404,
                sub_status_code: Some(1002),
            }],
            terminal_status: ExpectedStatus {
                status_code: 200,
                sub_status_code: None,
            },
            max_wait_ms: Some(5_000),
        };

        assert!(expected.is_acceptable_initial(404, Some(1002)));
        assert!(!expected.is_terminal(404, Some(1002)));
        assert!(expected.is_terminal(200, None));
        assert!(!expected.is_acceptable_initial(200, None));
    }
}
