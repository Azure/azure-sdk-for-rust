// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    options::{
        AvailabilityStrategy, ItemReadOptions, OperationOptions, ReadConsistencyStrategy, Region,
    },
    RoutingStrategy,
};

use crate::e2e_test_cases::{
    catalog::{selected_profile_for, AccountDefinition, ClientDefinition, Profile},
    fixture::{build_client_with_defaults, E2eTestFixture, TestResult},
    support::{assert_critical_diagnostics, item, write_options_with_content, Item},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ExpectedStatus {
    status_code: u16,
    sub_status_code: Option<u16>,
}

impl ExpectedStatus {
    const fn new(status_code: u16, sub_status_code: Option<u16>) -> Self {
        Self {
            status_code,
            sub_status_code,
        }
    }

    fn matches(self, status_code: u16, sub_status_code: Option<u16>) -> bool {
        self.status_code == status_code
            && self
                .sub_status_code
                .is_none_or(|expected| expected == sub_status_code.unwrap_or(0))
    }
}

const PLAIN_NOT_FOUND: ExpectedStatus = ExpectedStatus::new(404, Some(0));
const SESSION_NOT_AVAILABLE: ExpectedStatus = ExpectedStatus::new(404, Some(1002));
const READ_SUCCEEDED: ExpectedStatus = ExpectedStatus::new(200, None);
const CLIENT_REJECTED: ExpectedStatus = ExpectedStatus::new(400, None);

#[derive(Debug)]
struct LifecycleReadCase {
    id: String,
    operation_strategy: Option<&'static str>,
    explicit_session_token: bool,
    acceptable_initial_statuses: &'static [ExpectedStatus],
    terminal_status: ExpectedStatus,
    max_wait_ms: Option<u64>,
}

impl LifecycleReadCase {
    fn succeeds(
        id: impl Into<String>,
        operation_strategy: Option<&'static str>,
        explicit_session_token: bool,
        acceptable_initial_statuses: &'static [ExpectedStatus],
    ) -> Self {
        Self {
            id: id.into(),
            operation_strategy,
            explicit_session_token,
            acceptable_initial_statuses,
            terminal_status: READ_SUCCEEDED,
            max_wait_ms: (!acceptable_initial_statuses.is_empty()).then_some(5_000),
        }
    }

    fn rejects(id: impl Into<String>, operation_strategy: &'static str) -> Self {
        Self {
            id: id.into(),
            operation_strategy: Some(operation_strategy),
            explicit_session_token: false,
            acceptable_initial_statuses: &[],
            terminal_status: CLIENT_REJECTED,
            max_wait_ms: None,
        }
    }

    fn is_terminal(&self, status_code: u16, sub_status_code: Option<u16>) -> bool {
        self.terminal_status.matches(status_code, sub_status_code)
    }

    fn is_acceptable_initial(&self, status_code: u16, sub_status_code: Option<u16>) -> bool {
        self.acceptable_initial_statuses
            .iter()
            .any(|expected| expected.matches(status_code, sub_status_code))
    }
}

fn lifecycle_cases(
    profile: &Profile,
    account: &AccountDefinition,
    runtime_default: Option<&str>,
    client_default: Option<&str>,
) -> TestResult<Vec<LifecycleReadCase>> {
    match profile.id.as_str() {
        "hostedEmulatorSmoke" => Ok(vec![LifecycleReadCase::succeeds(
            "hostedSmokeDefault",
            Some("Default"),
            false,
            &[],
        )]),
        "lifecycleConsistencyMatrix" => Ok(consistency_cases(account)),
        "readConsistencyOverrideMatrix" => Ok(override_cases(runtime_default, client_default)),
        profile => Err(format!("item.lifecycle does not implement profile '{profile}'").into()),
    }
}

fn consistency_cases(account: &AccountDefinition) -> Vec<LifecycleReadCase> {
    let strong = account.consistency == "strong";
    let account_session = account.consistency == "session";
    [
        ("Default", account_session, false),
        ("Eventual", false, false),
        ("Session", true, true),
        ("LatestCommitted", false, false),
        ("GlobalStrong", false, false),
    ]
    .into_iter()
    .map(|(strategy, session_read, explicit_session_token)| {
        let id = format!("{}/{}", account.id, strategy);
        if strategy == "GlobalStrong" && !strong {
            LifecycleReadCase::rejects(id, strategy)
        } else if strong {
            LifecycleReadCase::succeeds(id, Some(strategy), explicit_session_token, &[])
        } else if session_read {
            LifecycleReadCase::succeeds(
                id,
                Some(strategy),
                explicit_session_token,
                &[SESSION_NOT_AVAILABLE],
            )
        } else {
            LifecycleReadCase::succeeds(
                id,
                Some(strategy),
                explicit_session_token,
                &[PLAIN_NOT_FOUND],
            )
        }
    })
    .collect()
}

fn override_cases(
    runtime_default: Option<&str>,
    client_default: Option<&str>,
) -> Vec<LifecycleReadCase> {
    let inherited_session = client_default
        .or(runtime_default)
        .is_none_or(|strategy| strategy == "Session");
    let inherited_statuses: &'static [ExpectedStatus] = if inherited_session {
        &[SESSION_NOT_AVAILABLE]
    } else {
        &[PLAIN_NOT_FOUND]
    };
    vec![
        LifecycleReadCase::succeeds("override/Inherit", None, false, inherited_statuses),
        LifecycleReadCase::succeeds(
            "override/Default",
            Some("Default"),
            true,
            &[SESSION_NOT_AVAILABLE],
        ),
        LifecycleReadCase::succeeds(
            "override/Eventual",
            Some("Eventual"),
            false,
            &[PLAIN_NOT_FOUND],
        ),
    ]
}

fn lifecycle_read_region(profile: &Profile) -> TestResult<Region> {
    match profile.id.as_str() {
        "hostedEmulatorSmoke" => Ok(Region::EAST_US),
        "lifecycleConsistencyMatrix" | "readConsistencyOverrideMatrix" => Ok(Region::WEST_US),
        profile => {
            Err(format!("item.lifecycle does not define a read region for '{profile}'").into())
        }
    }
}

fn lifecycle_routing(
    client: &ClientDefinition,
    read_region: &Region,
) -> TestResult<RoutingStrategy> {
    match client.routing.as_str() {
        "proximity" => Ok(RoutingStrategy::ProximityTo(read_region.clone())),
        "preferredRegions" => {
            let mut regions = vec![read_region.clone()];
            if read_region != &Region::EAST_US {
                regions.push(Region::EAST_US);
            }
            Ok(RoutingStrategy::PreferredRegions(regions))
        }
        "accountOrder" => Ok(RoutingStrategy::PreferredRegions(Vec::new())),
        routing => Err(format!("unsupported lifecycle routing strategy '{routing}'").into()),
    }
}

fn selected_axis<'a>(environment_variable: &str, available: &'a [&str]) -> TestResult<&'a str> {
    match std::env::var(environment_variable) {
        Ok(selected) => available
            .iter()
            .copied()
            .find(|candidate| *candidate == selected)
            .ok_or_else(|| {
                format!("{environment_variable}='{selected}' is not one of {available:?}").into()
            }),
        Err(_) if available.len() == 1 => Ok(available[0]),
        Err(_) => Err(format!(
            "{environment_variable} is required because this profile defines {available:?}"
        )
        .into()),
    }
}

fn parse_optional_strategy(value: Option<&str>) -> TestResult<Option<ReadConsistencyStrategy>> {
    value
        .map(str::parse::<ReadConsistencyStrategy>)
        .transpose()
        .map_err(Into::into)
}

fn parse_setup_switch(value: &str, default: &str) -> TestResult<Option<bool>> {
    match value {
        "enabled" => Ok(Some(true)),
        "disabled" => Ok(Some(false)),
        value if value == default => Ok(None),
        value => Err(format!("unsupported setup switch '{value}'").into()),
    }
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn item_lifecycle() -> TestResult {
    let Some(profile) = selected_profile_for("item.lifecycle")? else {
        return Ok(());
    };
    run_lifecycle_consistency_matrix(&profile).await
}

async fn run_lifecycle_consistency_matrix(profile: &Profile) -> TestResult {
    let account_ids: Vec<_> = profile
        .accounts
        .iter()
        .map(|definition| definition.id.as_str())
        .collect();
    let runtime_ids: Vec<_> = profile
        .runtimes
        .iter()
        .map(|definition| definition.id.as_str())
        .collect();
    let client_ids: Vec<_> = profile
        .clients
        .iter()
        .map(|definition| definition.id.as_str())
        .collect();
    let account_id = selected_axis("AZURE_COSMOS_E2E_ACCOUNT", &account_ids)?;
    let runtime_id = selected_axis("AZURE_COSMOS_E2E_RUNTIME", &runtime_ids)?;
    let client_id = selected_axis("AZURE_COSMOS_E2E_CLIENT", &client_ids)?;
    let account = profile.account(account_id);
    let runtime = profile.runtime(runtime_id);
    let client_definition = profile.client(client_id);
    let runtime_strategy =
        parse_optional_strategy(runtime.default_read_consistency_strategy.as_deref())?;
    let client_strategy = parse_optional_strategy(
        client_definition
            .default_read_consistency_strategy
            .as_deref(),
    )?;
    let cases = lifecycle_cases(
        profile,
        account,
        runtime.default_read_consistency_strategy.as_deref(),
        client_definition
            .default_read_consistency_strategy
            .as_deref(),
    )?;
    let read_region = lifecycle_read_region(profile)?;
    let routing = lifecycle_routing(client_definition, &read_region)?;
    let gateway_v2_enabled = parse_setup_switch(&runtime.gateway_v2, "backendDefault")?;
    let ppcb_enabled = parse_setup_switch(&runtime.ppcb, "sdkDefault")?;
    let binary_encoding_enabled =
        parse_setup_switch(&client_definition.binary_encoding, "sdkDefault")?;

    for case in cases {
        let client = build_client_with_defaults(
            routing.clone(),
            runtime_strategy,
            client_strategy,
            gateway_v2_enabled,
            ppcb_enabled,
            binary_encoding_enabled,
        )
        .await?;

        E2eTestFixture::run_with_client(client, "/pk".into(), async |fixture| {
            let item_id = format!("lifecycle-{}", case.id.replace('/', "-"));
            let created = fixture
                .container
                .create_item("A", &item_id, item(&item_id, "A", 1), None)
                .await?;
            assert_eq!(created.status().status_code(), StatusCode::Created);
            assert_critical_diagnostics(
                &created.diagnostics(),
                "create_item",
                StatusCode::Created,
            );
            let create_token = created.headers().session_token().cloned();

            let mut operation = OperationOptions::default();
            operation.read_consistency_strategy = parse_optional_strategy(case.operation_strategy)?;
            operation.availability_strategy = Some(AvailabilityStrategy::Disabled);
            let mut read_options = ItemReadOptions::default().with_operation_options(operation);
            let mut terminal_item = None;
            if case.explicit_session_token {
                read_options = read_options.with_session_token(
                    create_token
                        .clone()
                        .ok_or("create response must carry a session token")?,
                );
            }

            let deadline = tokio::time::Instant::now()
                + std::time::Duration::from_millis(case.max_wait_ms.unwrap_or_default());
            let mut observed_statuses = Vec::new();
            loop {
                let (status_code, sub_status_code, terminal) = match fixture
                    .container
                    .read_item("A", &item_id, Some(read_options.clone()))
                    .await
                {
                    Ok(read) => {
                        let status_code = u16::from(read.status().status_code());
                        let sub_status_code =
                            read.status().sub_status().map(|value| value.value());
                        let terminal = case.is_terminal(status_code, sub_status_code);
                        for request in read.diagnostics().requests().iter() {
                            let request_status = u16::from(request.status().status_code());
                            let request_sub_status =
                                request.status().sub_status().map(|value| value.value());
                            if case.is_acceptable_initial(request_status, request_sub_status) {
                                observed_statuses
                                    .push((request_status, request_sub_status.unwrap_or(0)));
                            }
                        }
                        if terminal {
                            assert_critical_diagnostics(
                                &read.diagnostics(),
                                "read_item",
                                StatusCode::Ok,
                            );
                            terminal_item = Some(read.into_model::<Item>()?);
                        }
                        (status_code, sub_status_code, terminal)
                    }
                    Err(error) => {
                        let status_code = u16::from(error.status().status_code());
                        let sub_status_code =
                            error.status().sub_status().map(|value| value.value());
                        let terminal = case.is_terminal(status_code, sub_status_code);
                        if let Some(diagnostics) = error.diagnostics() {
                            if terminal && case.terminal_status == CLIENT_REJECTED {
                                assert_eq!(
                                    diagnostics.request_count(),
                                    0,
                                    "client validation must reject '{}' before transport",
                                    case.id
                                );
                            }
                            for request in diagnostics.requests().iter() {
                                let request_status = u16::from(request.status().status_code());
                                let request_sub_status =
                                    request.status().sub_status().map(|value| value.value());
                                if case.is_acceptable_initial(request_status, request_sub_status) {
                                    observed_statuses
                                        .push((request_status, request_sub_status.unwrap_or(0)));
                                }
                            }
                        }
                        (status_code, sub_status_code, terminal)
                    }
                };
                observed_statuses.push((status_code, sub_status_code.unwrap_or(0)));
                if terminal {
                    break;
                }
                if !case.is_acceptable_initial(status_code, sub_status_code) {
                    return Err(format!(
                        "execution '{}' observed unexpected read status {status_code}/{}; expected transient {:?} or terminal {:?}; observed {observed_statuses:?}",
                        case.id,
                        sub_status_code.unwrap_or(0),
                        case.acceptable_initial_statuses,
                        case.terminal_status,
                    )
                    .into());
                }
                if tokio::time::Instant::now() >= deadline {
                    return Err(format!(
                        "execution '{}' did not reach terminal status {:?} within {} ms; observed {observed_statuses:?}",
                        case.id,
                        case.terminal_status,
                        case.max_wait_ms.unwrap_or_default(),
                    )
                    .into());
                }
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }

            if case.terminal_status == READ_SUCCEEDED {
                assert_eq!(
                    terminal_item,
                    Some(item(&item_id, "A", 1)),
                    "terminal read for '{}' returned the wrong item",
                    case.id
                );
            } else {
                assert!(terminal_item.is_none());
            }
            let replaced = fixture
                .container
                .replace_item(
                    "A",
                    &item_id,
                    item(&item_id, "A", 2),
                    Some(write_options_with_content()),
                )
                .await?;
            assert_eq!(replaced.status().status_code(), StatusCode::Ok);
            assert_critical_diagnostics(
                &replaced.diagnostics(),
                "replace_item",
                StatusCode::Ok,
            );
            assert_eq!(replaced.into_model::<Item>()?, item(&item_id, "A", 2));
            let deleted = fixture.container.delete_item("A", &item_id, None).await?;
            assert_eq!(deleted.status().status_code(), StatusCode::NoContent);
            assert_critical_diagnostics(
                &deleted.diagnostics(),
                "delete_item",
                StatusCode::NoContent,
            );
            let delete_token = deleted
                .headers()
                .session_token()
                .cloned()
                .ok_or("delete response must carry a session token")?;
            let mut delete_read_operation = OperationOptions::default();
            delete_read_operation.read_consistency_strategy =
                Some(ReadConsistencyStrategy::Session);
            delete_read_operation.availability_strategy = Some(AvailabilityStrategy::Disabled);
            let delete_read_options = ItemReadOptions::default()
                .with_operation_options(delete_read_operation)
                .with_session_token(delete_token);

            let delete_deadline =
                tokio::time::Instant::now() + std::time::Duration::from_secs(5);
            loop {
                match fixture
                    .container
                    .read_item("A", &item_id, Some(delete_read_options.clone()))
                    .await
                {
                    Err(error)
                        if PLAIN_NOT_FOUND.matches(
                            u16::from(error.status().status_code()),
                            error.status().sub_status().map(|value| value.value()),
                        ) =>
                    {
                        break;
                    }
                    Err(error)
                        if SESSION_NOT_AVAILABLE.matches(
                            u16::from(error.status().status_code()),
                            error.status().sub_status().map(|value| value.value()),
                        ) && tokio::time::Instant::now() < delete_deadline =>
                    {
                        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    }
                    Ok(_) if tokio::time::Instant::now() < delete_deadline => {
                        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    }
                    Ok(_) => {
                        return Err(format!(
                            "deleted item for '{}' remained visible after 5 seconds",
                            case.id
                        )
                        .into())
                    }
                    Err(error) => return Err(error.into()),
                }
            }
            Ok(())
        })
        .await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        consistency_cases, override_cases, ExpectedStatus, CLIENT_REJECTED, PLAIN_NOT_FOUND,
        READ_SUCCEEDED, SESSION_NOT_AVAILABLE,
    };

    #[test]
    fn status_matching_normalizes_missing_substatus_to_zero() {
        assert!(PLAIN_NOT_FOUND.matches(404, None));
        assert!(PLAIN_NOT_FOUND.matches(404, Some(0)));
        assert!(!PLAIN_NOT_FOUND.matches(404, Some(1002)));
        assert!(READ_SUCCEEDED.matches(200, Some(0)));
        assert!(!ExpectedStatus::new(200, None).matches(404, None));
    }

    #[test]
    fn account_consistency_matrix_covers_all_operation_strategies() {
        let profile = serde_json::from_str::<super::Profile>(include_str!(
            "../../../e2e_tests/profiles/lifecycleConsistencyMatrix.json"
        ))
        .expect("consistency profile must deserialize");
        let expectations = [
            ("strong", [None, None, None, None, None]),
            (
                "boundedStaleness",
                [
                    Some(PLAIN_NOT_FOUND),
                    Some(PLAIN_NOT_FOUND),
                    Some(SESSION_NOT_AVAILABLE),
                    Some(PLAIN_NOT_FOUND),
                    None,
                ],
            ),
            (
                "session",
                [
                    Some(SESSION_NOT_AVAILABLE),
                    Some(PLAIN_NOT_FOUND),
                    Some(SESSION_NOT_AVAILABLE),
                    Some(PLAIN_NOT_FOUND),
                    None,
                ],
            ),
            (
                "consistentPrefix",
                [
                    Some(PLAIN_NOT_FOUND),
                    Some(PLAIN_NOT_FOUND),
                    Some(SESSION_NOT_AVAILABLE),
                    Some(PLAIN_NOT_FOUND),
                    None,
                ],
            ),
            (
                "eventual",
                [
                    Some(PLAIN_NOT_FOUND),
                    Some(PLAIN_NOT_FOUND),
                    Some(SESSION_NOT_AVAILABLE),
                    Some(PLAIN_NOT_FOUND),
                    None,
                ],
            ),
        ];

        for (account, transient_statuses) in expectations {
            let cases = consistency_cases(profile.account(account));
            assert_eq!(
                cases
                    .iter()
                    .map(|case| case.operation_strategy)
                    .collect::<Vec<_>>(),
                [
                    Some("Default"),
                    Some("Eventual"),
                    Some("Session"),
                    Some("LatestCommitted"),
                    Some("GlobalStrong"),
                ]
            );
            assert_eq!(
                cases
                    .iter()
                    .map(|case| case.explicit_session_token)
                    .collect::<Vec<_>>(),
                [false, false, true, false, false]
            );
            for (index, expected_transient) in transient_statuses.into_iter().enumerate() {
                assert_eq!(
                    cases[index].acceptable_initial_statuses,
                    expected_transient.as_slice()
                );
                let terminal = if index == 4 && account != "strong" {
                    CLIENT_REJECTED
                } else {
                    READ_SUCCEEDED
                };
                assert_eq!(cases[index].terminal_status, terminal);
                assert_eq!(cases[index].max_wait_ms, expected_transient.map(|_| 5_000));
            }
        }
    }

    #[test]
    fn override_matrix_keeps_operation_cases_in_source() {
        let profile = serde_json::from_str::<super::Profile>(include_str!(
            "../../../e2e_tests/profiles/readConsistencyOverrideMatrix.json"
        ))
        .expect("override profile must deserialize");

        for runtime in &profile.runtimes {
            for client in &profile.clients {
                let cases = override_cases(
                    runtime.default_read_consistency_strategy.as_deref(),
                    client.default_read_consistency_strategy.as_deref(),
                );
                assert_eq!(
                    cases
                        .iter()
                        .map(|case| case.operation_strategy)
                        .collect::<Vec<_>>(),
                    [None, Some("Default"), Some("Eventual")]
                );
                assert_eq!(
                    cases
                        .iter()
                        .map(|case| case.explicit_session_token)
                        .collect::<Vec<_>>(),
                    [false, true, false]
                );
                let inherited = if client
                    .default_read_consistency_strategy
                    .as_deref()
                    .or(runtime.default_read_consistency_strategy.as_deref())
                    .is_none_or(|strategy| strategy == "Session")
                {
                    SESSION_NOT_AVAILABLE
                } else {
                    PLAIN_NOT_FOUND
                };
                assert_eq!(cases[0].acceptable_initial_statuses, [inherited]);
                assert_eq!(
                    cases[1].acceptable_initial_statuses,
                    [SESSION_NOT_AVAILABLE]
                );
                assert_eq!(cases[2].acceptable_initial_statuses, [PLAIN_NOT_FOUND]);
                assert!(cases.iter().all(|case| {
                    case.terminal_status == READ_SUCCEEDED && case.max_wait_ms == Some(5_000)
                }));
            }
        }
    }
}
