// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::time::Duration;

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    clients::ContainerClient,
    options::{
        AvailabilityStrategy, ItemReadOptions, OperationOptions, ReadConsistencyStrategy, Region,
    },
    RoutingStrategy,
};

use crate::e2e_test_cases::{
    catalog::{AccountDefinition, ClientDefinition, Profile, RuntimeDefinition},
    fixture::{build_client_with_defaults, ClientSetup, E2eTestFixture, TestResult},
    support::{
        assert_critical_diagnostics, item, selected_scenario_profile, write_options_with_content,
        Item,
    },
};

const REPLICATION_TIMEOUT: Duration = Duration::from_secs(5);
const RETRY_DELAY: Duration = Duration::from_millis(50);

// The JSON profile selects the account, runtime, and client configuration. Rust then expands
// that setup into the operation-level cases below:
//
// * hostedEmulatorSmoke: one account-default read;
// * lifecycleConsistencyMatrix: Default, Eventual, Session, LatestCommitted, and GlobalStrong;
// * readConsistencyOverrideMatrix: inherit defaults, restore account default, and Eventual.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn crud_lifecycle() -> TestResult {
    let Some(profile) = selected_scenario_profile("item.lifecycle").await? else {
        return Ok(());
    };
    let setup = SelectedLifecycleSetup::from_profile(&profile)?;
    let read_cases = read_cases_for_selected_profile(&setup)?;

    for read_case in read_cases {
        run_lifecycle_case(&setup, &read_case).await?;
    }
    Ok(())
}

// This is the lifecycle contract. Keep the operation sequence and its assertions visible here;
// helpers below translate profiles, construct the varying read, and handle polling mechanics.
async fn run_lifecycle_case(
    setup: &SelectedLifecycleSetup<'_>,
    read_case: &PostCreateReadCase,
) -> TestResult {
    let execution = setup.execution_name(read_case.name);
    let client = setup.build_client().await?;

    E2eTestFixture::run_with_client(client, "/pk".into(), async |fixture| {
        let item_id = format!("lifecycle-{}", execution.replace('/', "-"));

        // Create an item and capture the session token used by explicit Session reads.
        let created = fixture
            .container
            .create_item("A", &item_id, item(&item_id, "A", 1), None)
            .await?;
        assert_eq!(created.status().status_code(), StatusCode::Created);
        assert_critical_diagnostics(&created.diagnostics(), "create_item", StatusCode::Created);
        let create_session_token = created
            .headers()
            .session_token()
            .map(|token| token.as_str().to_owned());

        // Read the created item using this case's operation-level consistency behavior.
        let read_outcome = read_created_item(
            &fixture.container,
            &item_id,
            create_session_token,
            read_case,
            &execution,
        )
        .await?;
        match read_outcome {
            PostCreateReadOutcome::Item(actual) => assert_eq!(
                actual,
                item(&item_id, "A", 1),
                "read returned the wrong item for '{execution}'"
            ),
            PostCreateReadOutcome::RejectedBeforeTransport => assert_eq!(
                read_case.expectation,
                ReadExpectation::RejectedBeforeTransport,
                "read was rejected unexpectedly for '{execution}'"
            ),
        }

        // Replace the item and verify the returned model.
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
        assert_critical_diagnostics(&replaced.diagnostics(), "replace_item", StatusCode::Ok);
        assert_eq!(replaced.into_model::<Item>()?, item(&item_id, "A", 2));

        // Delete the item, then use the delete session token to verify a plain 404/0.
        let deleted = fixture.container.delete_item("A", &item_id, None).await?;
        assert_eq!(deleted.status().status_code(), StatusCode::NoContent);
        assert_critical_diagnostics(&deleted.diagnostics(), "delete_item", StatusCode::NoContent);
        let delete_session_token = deleted
            .headers()
            .session_token()
            .map(|token| token.as_str().to_owned())
            .ok_or("delete response must carry a session token")?;
        assert_item_deleted(
            &fixture.container,
            &item_id,
            delete_session_token,
            &execution,
        )
        .await?;

        Ok(())
    })
    .await
}

// Operation cases -------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SessionTokenBehavior {
    SdkManaged,
    ExplicitCreateResponse,
    Omitted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ReadExpectation {
    SucceedsImmediately,
    EventuallySucceeds {
        allowed_transient_statuses: &'static [TransientReadStatus],
    },
    RejectedBeforeTransport,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TransientReadStatus {
    PlainNotFound,
    SessionNotAvailable,
}

impl TransientReadStatus {
    const fn http_status(self) -> ExpectedHttpStatus {
        match self {
            Self::PlainNotFound => PLAIN_NOT_FOUND,
            Self::SessionNotAvailable => SESSION_NOT_AVAILABLE,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PostCreateReadOutcome {
    Item(Item),
    RejectedBeforeTransport,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PostCreateReadCase {
    name: &'static str,
    consistency_override: Option<ReadConsistencyStrategy>,
    session_token: SessionTokenBehavior,
    expectation: ReadExpectation,
}

impl PostCreateReadCase {
    const fn new(
        name: &'static str,
        consistency_override: Option<ReadConsistencyStrategy>,
        session_token: SessionTokenBehavior,
        expectation: ReadExpectation,
    ) -> Self {
        Self {
            name,
            consistency_override,
            session_token,
            expectation,
        }
    }
}

fn read_cases_for_selected_profile(
    setup: &SelectedLifecycleSetup<'_>,
) -> TestResult<Vec<PostCreateReadCase>> {
    match setup.profile.id.as_str() {
        "hostedEmulatorSmoke" => Ok(vec![PostCreateReadCase::new(
            "default_strategy_reads_created_item",
            Some(ReadConsistencyStrategy::Default),
            SessionTokenBehavior::SdkManaged,
            ReadExpectation::SucceedsImmediately,
        )]),
        "lifecycleConsistencyMatrix" => Ok(read_cases_for_account_consistency(setup.account)),
        "readConsistencyOverrideMatrix" => Ok(read_cases_for_default_precedence(
            setup.account,
            setup.runtime.default_read_consistency_strategy.as_deref(),
            setup.client.default_read_consistency_strategy.as_deref(),
        )?),
        profile => Err(format!("item.lifecycle does not implement profile '{profile}'").into()),
    }
}

fn read_cases_for_account_consistency(account: &AccountDefinition) -> Vec<PostCreateReadCase> {
    let is_strong_account = account.consistency == "strong";

    let regional_read = if is_strong_account {
        ReadExpectation::SucceedsImmediately
    } else {
        eventually_succeeds_after(TransientReadStatus::PlainNotFound)
    };
    let session_read = if is_strong_account {
        ReadExpectation::SucceedsImmediately
    } else {
        eventually_succeeds_after(TransientReadStatus::SessionNotAvailable)
    };
    let account_default_read = match account.consistency.as_str() {
        "strong" => ReadExpectation::SucceedsImmediately,
        "session" => session_read,
        _ => regional_read,
    };
    let account_default_token = if account.consistency == "session" {
        SessionTokenBehavior::SdkManaged
    } else {
        SessionTokenBehavior::Omitted
    };
    let global_strong_read = if is_strong_account {
        ReadExpectation::SucceedsImmediately
    } else {
        ReadExpectation::RejectedBeforeTransport
    };

    vec![
        PostCreateReadCase::new(
            "default_strategy_uses_account_consistency",
            Some(ReadConsistencyStrategy::Default),
            account_default_token,
            account_default_read,
        ),
        PostCreateReadCase::new(
            "eventual_strategy_allows_replication_lag",
            Some(ReadConsistencyStrategy::Eventual),
            SessionTokenBehavior::Omitted,
            regional_read,
        ),
        PostCreateReadCase::new(
            "session_strategy_uses_create_token",
            Some(ReadConsistencyStrategy::Session),
            SessionTokenBehavior::ExplicitCreateResponse,
            session_read,
        ),
        PostCreateReadCase::new(
            "latest_committed_is_region_local",
            Some(ReadConsistencyStrategy::LatestCommitted),
            SessionTokenBehavior::Omitted,
            regional_read,
        ),
        PostCreateReadCase::new(
            "global_strong_requires_strong_account",
            Some(ReadConsistencyStrategy::GlobalStrong),
            SessionTokenBehavior::Omitted,
            global_strong_read,
        ),
    ]
}

fn read_cases_for_default_precedence(
    account: &AccountDefinition,
    runtime_default: Option<&str>,
    client_default: Option<&str>,
) -> TestResult<Vec<PostCreateReadCase>> {
    if account.consistency != "session" {
        return Err(format!(
            "readConsistencyOverrideMatrix requires a Session account, got '{}'",
            account.consistency
        )
        .into());
    }
    // Effective precedence is operation > client > runtime > account. These cases vary only the
    // operation value; the JSON profile supplies the selected client and runtime defaults.
    let inherited_uses_session = match client_default.or(runtime_default) {
        Some(strategy) => parse_read_consistency(strategy)? == ReadConsistencyStrategy::Session,
        None => account.consistency == "session",
    };
    let inherited_expectation = if inherited_uses_session {
        eventually_succeeds_after(TransientReadStatus::SessionNotAvailable)
    } else {
        eventually_succeeds_after(TransientReadStatus::PlainNotFound)
    };
    let inherited_token = if inherited_uses_session {
        SessionTokenBehavior::SdkManaged
    } else {
        SessionTokenBehavior::Omitted
    };

    Ok(vec![
        PostCreateReadCase::new(
            "inherits_client_then_runtime_then_account_default",
            None,
            inherited_token,
            inherited_expectation,
        ),
        PostCreateReadCase::new(
            "default_override_restores_account_consistency",
            Some(ReadConsistencyStrategy::Default),
            SessionTokenBehavior::ExplicitCreateResponse,
            eventually_succeeds_after(TransientReadStatus::SessionNotAvailable),
        ),
        PostCreateReadCase::new(
            "eventual_override_wins_over_all_defaults",
            Some(ReadConsistencyStrategy::Eventual),
            SessionTokenBehavior::Omitted,
            eventually_succeeds_after(TransientReadStatus::PlainNotFound),
        ),
    ])
}

fn eventually_succeeds_after(status: TransientReadStatus) -> ReadExpectation {
    ReadExpectation::EventuallySucceeds {
        allowed_transient_statuses: match status {
            TransientReadStatus::PlainNotFound => &[TransientReadStatus::PlainNotFound],
            TransientReadStatus::SessionNotAvailable => &[TransientReadStatus::SessionNotAvailable],
        },
    }
}

// Selected JSON setup ---------------------------------------------------------

struct SelectedLifecycleSetup<'a> {
    profile: &'a Profile,
    account: &'a AccountDefinition,
    runtime: &'a RuntimeDefinition,
    client: &'a ClientDefinition,
    routing: RoutingStrategy,
}

impl<'a> SelectedLifecycleSetup<'a> {
    fn from_profile(profile: &'a Profile) -> TestResult<Self> {
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

        let account = profile.account(selected_axis("AZURE_COSMOS_E2E_ACCOUNT", &account_ids)?);
        let runtime = profile.runtime(selected_axis("AZURE_COSMOS_E2E_RUNTIME", &runtime_ids)?);
        let client = profile.client(selected_axis("AZURE_COSMOS_E2E_CLIENT", &client_ids)?);
        let read_region = lifecycle_read_region(profile)?;
        let routing = lifecycle_routing(client, &read_region)?;

        Ok(Self {
            profile,
            account,
            runtime,
            client,
            routing,
        })
    }

    async fn build_client(&self) -> TestResult<azure_data_cosmos::CosmosClient> {
        build_client_with_defaults(ClientSetup::from_profile(
            self.runtime,
            self.client,
            self.routing.clone(),
        )?)
        .await
    }

    fn execution_name(&self, case_name: &str) -> String {
        format!(
            "{}/{}/{}/{}/{}",
            self.profile.id, self.account.id, self.runtime.id, self.client.id, case_name
        )
    }
}

// Retry mechanics -------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExpectedSubstatus {
    Any,
    ZeroOrMissing,
    Exact(u16),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ExpectedHttpStatus {
    status_code: StatusCode,
    substatus: ExpectedSubstatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ActualHttpStatus {
    status_code: StatusCode,
    substatus: Option<u16>,
}

impl ExpectedHttpStatus {
    fn matches(self, status_code: StatusCode, substatus: Option<u16>) -> bool {
        self.status_code == status_code
            && match self.substatus {
                ExpectedSubstatus::Any => true,
                ExpectedSubstatus::ZeroOrMissing => {
                    substatus.is_none() || matches!(substatus, Some(0))
                }
                ExpectedSubstatus::Exact(expected) => {
                    matches!(substatus, Some(actual) if actual == expected)
                }
            }
    }
}

const PLAIN_NOT_FOUND: ExpectedHttpStatus = ExpectedHttpStatus {
    status_code: StatusCode::NotFound,
    substatus: ExpectedSubstatus::ZeroOrMissing,
};
const SESSION_NOT_AVAILABLE: ExpectedHttpStatus = ExpectedHttpStatus {
    status_code: StatusCode::NotFound,
    substatus: ExpectedSubstatus::Exact(1002),
};
const READ_SUCCEEDED: ExpectedHttpStatus = ExpectedHttpStatus {
    status_code: StatusCode::Ok,
    substatus: ExpectedSubstatus::Any,
};
const CLIENT_REJECTED: ExpectedHttpStatus = ExpectedHttpStatus {
    status_code: StatusCode::BadRequest,
    substatus: ExpectedSubstatus::Any,
};

async fn read_created_item(
    container: &ContainerClient,
    item_id: &str,
    create_session_token: Option<String>,
    read_case: &PostCreateReadCase,
    execution: &str,
) -> TestResult<PostCreateReadOutcome> {
    let mut operation = OperationOptions::default();
    operation.read_consistency_strategy = read_case.consistency_override;
    operation.availability_strategy = Some(AvailabilityStrategy::Disabled);
    let mut options = ItemReadOptions::default().with_operation_options(operation);
    if read_case.session_token == SessionTokenBehavior::ExplicitCreateResponse {
        options = options.with_session_token(
            create_session_token.ok_or("create response must carry a session token")?,
        );
    }

    let deadline = tokio::time::Instant::now() + read_case.expectation.timeout();
    let mut observed_statuses = Vec::new();
    loop {
        match container
            .read_item("A", item_id, Some(options.clone()))
            .await
        {
            Ok(response) => {
                let status = ActualHttpStatus {
                    status_code: response.status().status_code(),
                    substatus: response.status().sub_status().map(|value| value.value()),
                };
                record_request_statuses(&response.diagnostics(), &mut observed_statuses);
                observed_statuses.push(status);
                if read_case
                    .expectation
                    .terminal_status()
                    .matches(status.status_code, status.substatus)
                {
                    assert_critical_diagnostics(
                        &response.diagnostics(),
                        "read_item",
                        StatusCode::Ok,
                    );
                    return Ok(PostCreateReadOutcome::Item(response.into_model::<Item>()?));
                }
                verify_transient_status(
                    read_case,
                    status,
                    deadline,
                    execution,
                    &observed_statuses,
                )?;
            }
            Err(error) => {
                let status = ActualHttpStatus {
                    status_code: error.status().status_code(),
                    substatus: error.status().sub_status().map(|value| value.value()),
                };
                if let Some(diagnostics) = error.diagnostics() {
                    record_request_statuses(&diagnostics, &mut observed_statuses);
                }
                observed_statuses.push(status);
                if read_case
                    .expectation
                    .terminal_status()
                    .matches(status.status_code, status.substatus)
                {
                    if read_case.expectation == ReadExpectation::RejectedBeforeTransport {
                        assert!(
                            error.response().is_none(),
                            "client validation for '{execution}' unexpectedly received a response"
                        );
                        let diagnostics = error
                            .diagnostics()
                            .expect("client rejection must carry diagnostics");
                        assert_eq!(
                            diagnostics.request_count(),
                            0,
                            "client validation must reject '{execution}' before transport"
                        );
                    }
                    return Ok(PostCreateReadOutcome::RejectedBeforeTransport);
                }
                verify_transient_status(
                    read_case,
                    status,
                    deadline,
                    execution,
                    &observed_statuses,
                )?;
            }
        }
        tokio::time::sleep(RETRY_DELAY).await;
    }
}

async fn assert_item_deleted(
    container: &ContainerClient,
    item_id: &str,
    delete_session_token: String,
    execution: &str,
) -> TestResult {
    let mut operation = OperationOptions::default();
    operation.read_consistency_strategy = Some(ReadConsistencyStrategy::Session);
    operation.availability_strategy = Some(AvailabilityStrategy::Disabled);
    let options = ItemReadOptions::default()
        .with_operation_options(operation)
        .with_session_token(delete_session_token);
    let deadline = tokio::time::Instant::now() + REPLICATION_TIMEOUT;

    loop {
        match container
            .read_item("A", item_id, Some(options.clone()))
            .await
        {
            Err(error)
                if PLAIN_NOT_FOUND.matches(
                    error.status().status_code(),
                    error.status().sub_status().map(|value| value.value()),
                ) =>
            {
                return Ok(());
            }
            Err(error)
                if SESSION_NOT_AVAILABLE.matches(
                    error.status().status_code(),
                    error.status().sub_status().map(|value| value.value()),
                ) && tokio::time::Instant::now() < deadline => {}
            Ok(_) if tokio::time::Instant::now() < deadline => {}
            Ok(_) => {
                return Err(format!(
                    "deleted item for '{execution}' remained visible after {REPLICATION_TIMEOUT:?}"
                )
                .into())
            }
            Err(error) => return Err(error.into()),
        }
        tokio::time::sleep(RETRY_DELAY).await;
    }
}

impl ReadExpectation {
    const fn terminal_status(self) -> ExpectedHttpStatus {
        match self {
            Self::SucceedsImmediately | Self::EventuallySucceeds { .. } => READ_SUCCEEDED,
            Self::RejectedBeforeTransport => CLIENT_REJECTED,
        }
    }

    const fn allowed_transient_statuses(self) -> &'static [TransientReadStatus] {
        match self {
            Self::EventuallySucceeds {
                allowed_transient_statuses,
            } => allowed_transient_statuses,
            Self::SucceedsImmediately | Self::RejectedBeforeTransport => &[],
        }
    }

    const fn timeout(self) -> Duration {
        match self {
            Self::EventuallySucceeds { .. } => REPLICATION_TIMEOUT,
            Self::SucceedsImmediately | Self::RejectedBeforeTransport => Duration::ZERO,
        }
    }
}

fn verify_transient_status(
    read_case: &PostCreateReadCase,
    actual: ActualHttpStatus,
    deadline: tokio::time::Instant,
    execution: &str,
    observed_statuses: &[ActualHttpStatus],
) -> TestResult {
    let allowed = read_case.expectation.allowed_transient_statuses();
    if !allowed.iter().any(|expected| {
        expected
            .http_status()
            .matches(actual.status_code, actual.substatus)
    }) {
        return Err(format!(
            "'{execution}' observed unexpected read status {actual:?}; expected transient {allowed:?} or terminal {:?}; observed {observed_statuses:?}",
            read_case.expectation.terminal_status()
        )
        .into());
    }
    if tokio::time::Instant::now() >= deadline {
        return Err(format!(
            "'{execution}' did not reach terminal status {:?} within {:?}; observed {observed_statuses:?}",
            read_case.expectation.terminal_status(),
            read_case.expectation.timeout()
        )
        .into());
    }
    Ok(())
}

fn record_request_statuses(
    diagnostics: &azure_data_cosmos::diagnostics::DiagnosticsContext,
    observed: &mut Vec<ActualHttpStatus>,
) {
    observed.extend(
        diagnostics
            .requests()
            .iter()
            .map(|request| ActualHttpStatus {
                status_code: request.status().status_code(),
                substatus: request.status().sub_status().map(|value| value.value()),
            }),
    );
}

// Profile value translation ---------------------------------------------------

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

fn parse_read_consistency(value: &str) -> TestResult<ReadConsistencyStrategy> {
    value.parse::<ReadConsistencyStrategy>().map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_consistency_cases_cover_every_account_and_strategy() {
        let profile = serde_json::from_str::<Profile>(include_str!(
            "../../../e2e_tests/profiles/lifecycleConsistencyMatrix.json"
        ))
        .expect("consistency profile must deserialize");
        let immediate = ReadExpectation::SucceedsImmediately;
        let plain_not_found = eventually_succeeds_after(TransientReadStatus::PlainNotFound);
        let session_not_available =
            eventually_succeeds_after(TransientReadStatus::SessionNotAvailable);
        let rejected = ReadExpectation::RejectedBeforeTransport;
        let expected_by_account = [
            ("strong", [immediate; 5]),
            (
                "boundedStaleness",
                [
                    plain_not_found,
                    plain_not_found,
                    session_not_available,
                    plain_not_found,
                    rejected,
                ],
            ),
            (
                "session",
                [
                    session_not_available,
                    plain_not_found,
                    session_not_available,
                    plain_not_found,
                    rejected,
                ],
            ),
            (
                "consistentPrefix",
                [
                    plain_not_found,
                    plain_not_found,
                    session_not_available,
                    plain_not_found,
                    rejected,
                ],
            ),
            (
                "eventual",
                [
                    plain_not_found,
                    plain_not_found,
                    session_not_available,
                    plain_not_found,
                    rejected,
                ],
            ),
        ];

        for (account, expected) in expected_by_account {
            let cases = read_cases_for_account_consistency(profile.account(account));
            assert_eq!(
                cases.iter().map(|case| case.name).collect::<Vec<_>>(),
                [
                    "default_strategy_uses_account_consistency",
                    "eventual_strategy_allows_replication_lag",
                    "session_strategy_uses_create_token",
                    "latest_committed_is_region_local",
                    "global_strong_requires_strong_account",
                ],
                "case names for {account}"
            );
            assert_eq!(
                cases
                    .iter()
                    .map(|case| case.consistency_override)
                    .collect::<Vec<_>>(),
                [
                    Some(ReadConsistencyStrategy::Default),
                    Some(ReadConsistencyStrategy::Eventual),
                    Some(ReadConsistencyStrategy::Session),
                    Some(ReadConsistencyStrategy::LatestCommitted),
                    Some(ReadConsistencyStrategy::GlobalStrong),
                ],
                "operation consistency overrides for {account}"
            );
            assert_eq!(
                cases
                    .iter()
                    .map(|case| case.expectation)
                    .collect::<Vec<_>>(),
                expected,
                "read expectations for {account}"
            );
            assert_eq!(
                cases
                    .iter()
                    .map(|case| case.session_token)
                    .collect::<Vec<_>>(),
                [
                    if account == "session" {
                        SessionTokenBehavior::SdkManaged
                    } else {
                        SessionTokenBehavior::Omitted
                    },
                    SessionTokenBehavior::Omitted,
                    SessionTokenBehavior::ExplicitCreateResponse,
                    SessionTokenBehavior::Omitted,
                    SessionTokenBehavior::Omitted,
                ],
                "session-token behavior for {account}"
            );
        }
    }

    #[test]
    fn override_cases_show_client_runtime_account_precedence() {
        let profile = serde_json::from_str::<Profile>(include_str!(
            "../../../e2e_tests/profiles/readConsistencyOverrideMatrix.json"
        ))
        .expect("override profile must deserialize");
        let account = profile.account("session");

        for runtime in &profile.runtimes {
            for client in &profile.clients {
                let inherited_uses_session = client
                    .default_read_consistency_strategy
                    .as_deref()
                    .or(runtime.default_read_consistency_strategy.as_deref())
                    .is_none_or(|strategy| strategy == "Session");
                let inherited_expectation = if inherited_uses_session {
                    eventually_succeeds_after(TransientReadStatus::SessionNotAvailable)
                } else {
                    eventually_succeeds_after(TransientReadStatus::PlainNotFound)
                };
                let inherited_token = if inherited_uses_session {
                    SessionTokenBehavior::SdkManaged
                } else {
                    SessionTokenBehavior::Omitted
                };

                let actual = read_cases_for_default_precedence(
                    account,
                    runtime.default_read_consistency_strategy.as_deref(),
                    client.default_read_consistency_strategy.as_deref(),
                )
                .expect("profile defaults must be valid");
                let expected = [
                    PostCreateReadCase::new(
                        "inherits_client_then_runtime_then_account_default",
                        None,
                        inherited_token,
                        inherited_expectation,
                    ),
                    PostCreateReadCase::new(
                        "default_override_restores_account_consistency",
                        Some(ReadConsistencyStrategy::Default),
                        SessionTokenBehavior::ExplicitCreateResponse,
                        eventually_succeeds_after(TransientReadStatus::SessionNotAvailable),
                    ),
                    PostCreateReadCase::new(
                        "eventual_override_wins_over_all_defaults",
                        Some(ReadConsistencyStrategy::Eventual),
                        SessionTokenBehavior::Omitted,
                        eventually_succeeds_after(TransientReadStatus::PlainNotFound),
                    ),
                ];
                assert_eq!(
                    actual, expected,
                    "runtime '{}' and client '{}'",
                    runtime.id, client.id
                );
            }
        }
    }

    #[test]
    fn status_matching_distinguishes_wildcard_zero_and_exact_substatus() {
        assert!(READ_SUCCEEDED.matches(StatusCode::Ok, Some(42)));
        assert!(PLAIN_NOT_FOUND.matches(StatusCode::NotFound, None));
        assert!(PLAIN_NOT_FOUND.matches(StatusCode::NotFound, Some(0)));
        assert!(!PLAIN_NOT_FOUND.matches(StatusCode::NotFound, Some(1002)));
        assert!(SESSION_NOT_AVAILABLE.matches(StatusCode::NotFound, Some(1002)));
    }
}
