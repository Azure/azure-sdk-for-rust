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
    fixture::{build_client_with_defaults, ClientSetup, E2eTest, TestResult},
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
// * smokeTests: one account-default read;
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
    run_selected_lifecycle_cases(&setup).await
}

// This is the lifecycle contract. Keep the operation sequence and its assertions visible here;
// helpers below translate profiles, construct the varying read, and handle polling mechanics.
async fn run_lifecycle_case(
    setup: &SelectedLifecycleSetup<'_>,
    read_case: &PostCreateReadCase,
) -> TestResult {
    let execution = setup.execution_name(read_case.name);
    let client = setup.build_client().await?;

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
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
                &setup.read_region,
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
            assert_critical_diagnostics(
                &deleted.diagnostics(),
                "delete_item",
                StatusCode::NoContent,
            );
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

async fn run_selected_lifecycle_cases(setup: &SelectedLifecycleSetup<'_>) -> TestResult {
    match setup.profile.id.as_str() {
        "smokeTests" => default_smoke_read(setup).await,
        "lifecycleConsistencyMatrix" => {
            default_strategy_uses_account_consistency(setup).await?;
            eventual_strategy_allows_replication_lag(setup).await?;
            session_strategy_uses_create_token(setup).await?;
            latest_committed_is_region_local(setup).await?;
            global_strong_requires_strong_account(setup).await
        }
        "readConsistencyOverrideMatrix" => {
            inherited_defaults_follow_precedence(setup).await?;
            default_override_restores_account_consistency(setup).await?;
            eventual_override_wins_over_defaults(setup).await
        }
        profile => Err(format!("item.lifecycle does not implement profile '{profile}'").into()),
    }
}

async fn default_smoke_read(setup: &SelectedLifecycleSetup<'_>) -> TestResult {
    run_lifecycle_case(
        setup,
        &PostCreateReadCase::new(
            "default_strategy_reads_created_item",
            Some(ReadConsistencyStrategy::Default),
            SessionTokenBehavior::SdkManaged,
            ReadExpectation::SucceedsImmediately,
        ),
    )
    .await
}

async fn default_strategy_uses_account_consistency(
    setup: &SelectedLifecycleSetup<'_>,
) -> TestResult {
    let (session_token, expectation) = match setup.account.consistency.as_str() {
        "strong" => (
            SessionTokenBehavior::Omitted,
            ReadExpectation::SucceedsImmediately,
        ),
        "session" => (
            SessionTokenBehavior::SdkManaged,
            eventually_succeeds_after(TransientReadStatus::SessionNotAvailable),
        ),
        _ => (
            SessionTokenBehavior::Omitted,
            eventually_succeeds_after(TransientReadStatus::PlainNotFound),
        ),
    };
    run_lifecycle_case(
        setup,
        &PostCreateReadCase::new(
            "default_strategy_uses_account_consistency",
            Some(ReadConsistencyStrategy::Default),
            session_token,
            expectation,
        ),
    )
    .await
}

async fn eventual_strategy_allows_replication_lag(
    setup: &SelectedLifecycleSetup<'_>,
) -> TestResult {
    run_lifecycle_case(
        setup,
        &PostCreateReadCase::new(
            "eventual_strategy_allows_replication_lag",
            Some(ReadConsistencyStrategy::Eventual),
            SessionTokenBehavior::Omitted,
            regional_read_expectation(setup.account),
        ),
    )
    .await
}

async fn session_strategy_uses_create_token(setup: &SelectedLifecycleSetup<'_>) -> TestResult {
    run_lifecycle_case(
        setup,
        &PostCreateReadCase::new(
            "session_strategy_uses_create_token",
            Some(ReadConsistencyStrategy::Session),
            SessionTokenBehavior::ExplicitCreateResponse,
            session_read_expectation(setup.account),
        ),
    )
    .await
}

async fn latest_committed_is_region_local(setup: &SelectedLifecycleSetup<'_>) -> TestResult {
    run_lifecycle_case(
        setup,
        &PostCreateReadCase::new(
            "latest_committed_is_region_local",
            Some(ReadConsistencyStrategy::LatestCommitted),
            SessionTokenBehavior::Omitted,
            regional_read_expectation(setup.account),
        ),
    )
    .await
}

async fn global_strong_requires_strong_account(setup: &SelectedLifecycleSetup<'_>) -> TestResult {
    let expectation = if setup.account.consistency == "strong" {
        ReadExpectation::SucceedsImmediately
    } else {
        ReadExpectation::RejectedBeforeTransport
    };
    run_lifecycle_case(
        setup,
        &PostCreateReadCase::new(
            "global_strong_requires_strong_account",
            Some(ReadConsistencyStrategy::GlobalStrong),
            SessionTokenBehavior::Omitted,
            expectation,
        ),
    )
    .await
}

async fn inherited_defaults_follow_precedence(setup: &SelectedLifecycleSetup<'_>) -> TestResult {
    require_session_account(setup.account)?;
    let inherited_strategy = setup
        .client
        .default_read_consistency_strategy
        .as_deref()
        .or(setup.runtime.default_read_consistency_strategy.as_deref());
    let inherited_uses_session = inherited_strategy
        .map(parse_read_consistency)
        .transpose()?
        .is_none_or(|strategy| strategy == ReadConsistencyStrategy::Session);
    let (session_token, expectation) = if inherited_uses_session {
        (
            SessionTokenBehavior::SdkManaged,
            eventually_succeeds_after(TransientReadStatus::SessionNotAvailable),
        )
    } else {
        (
            SessionTokenBehavior::Omitted,
            eventually_succeeds_after(TransientReadStatus::PlainNotFound),
        )
    };
    run_lifecycle_case(
        setup,
        &PostCreateReadCase::new(
            "inherits_client_then_runtime_then_account_default",
            None,
            session_token,
            expectation,
        ),
    )
    .await
}

async fn default_override_restores_account_consistency(
    setup: &SelectedLifecycleSetup<'_>,
) -> TestResult {
    require_session_account(setup.account)?;
    run_lifecycle_case(
        setup,
        &PostCreateReadCase::new(
            "default_override_restores_account_consistency",
            Some(ReadConsistencyStrategy::Default),
            SessionTokenBehavior::ExplicitCreateResponse,
            eventually_succeeds_after(TransientReadStatus::SessionNotAvailable),
        ),
    )
    .await
}

async fn eventual_override_wins_over_defaults(setup: &SelectedLifecycleSetup<'_>) -> TestResult {
    require_session_account(setup.account)?;
    run_lifecycle_case(
        setup,
        &PostCreateReadCase::new(
            "eventual_override_wins_over_all_defaults",
            Some(ReadConsistencyStrategy::Eventual),
            SessionTokenBehavior::Omitted,
            eventually_succeeds_after(TransientReadStatus::PlainNotFound),
        ),
    )
    .await
}

fn regional_read_expectation(account: &AccountDefinition) -> ReadExpectation {
    if account.consistency == "strong" {
        ReadExpectation::SucceedsImmediately
    } else {
        eventually_succeeds_after(TransientReadStatus::PlainNotFound)
    }
}

fn session_read_expectation(account: &AccountDefinition) -> ReadExpectation {
    if account.consistency == "strong" {
        ReadExpectation::SucceedsImmediately
    } else {
        eventually_succeeds_after(TransientReadStatus::SessionNotAvailable)
    }
}

fn require_session_account(account: &AccountDefinition) -> TestResult {
    if account.consistency == "session" {
        Ok(())
    } else {
        Err(format!(
            "readConsistencyOverrideMatrix requires a Session account, got '{}'",
            account.consistency
        )
        .into())
    }
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
    read_region: Region,
    routing: RoutingStrategy,
}

impl<'a> SelectedLifecycleSetup<'a> {
    fn from_profile(profile: &'a Profile) -> TestResult<Self> {
        let account = profile.selected_account()?;
        let runtime = profile.selected_runtime()?;
        let client = profile.selected_client()?;
        let read_region = lifecycle_read_region(profile)?;
        let routing = lifecycle_routing(client, &read_region)?;

        Ok(Self {
            profile,
            account,
            runtime,
            client,
            read_region,
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
    expected_region: &Region,
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
                verify_observed_statuses(read_case, execution, &observed_statuses)?;
                assert_initial_read_region(&response.diagnostics(), expected_region, execution);
                if read_case
                    .expectation
                    .terminal_status()
                    .matches(status.status_code, status.substatus)
                {
                    verify_required_transient_observed(read_case, execution, &observed_statuses)?;
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
                    if diagnostics.request_count() > 0 {
                        assert_initial_read_region(&diagnostics, expected_region, execution);
                    }
                }
                observed_statuses.push(status);
                verify_observed_statuses(read_case, execution, &observed_statuses)?;
                if read_case
                    .expectation
                    .terminal_status()
                    .matches(status.status_code, status.substatus)
                {
                    verify_required_transient_observed(read_case, execution, &observed_statuses)?;
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
        let result = container
            .read_item("A", item_id, Some(options.clone()))
            .await;
        let status = match &result {
            Ok(response) => ActualHttpStatus {
                status_code: response.status().status_code(),
                substatus: response.status().sub_status().map(|value| value.value()),
            },
            Err(error) => ActualHttpStatus {
                status_code: error.status().status_code(),
                substatus: error.status().sub_status().map(|value| value.value()),
            },
        };
        match deleted_read_action(status, tokio::time::Instant::now() < deadline) {
            DeletedReadAction::Deleted => return Ok(()),
            DeletedReadAction::Retry => {}
            DeletedReadAction::SessionViolation => {
                return Err(format!(
                    "deleted item for '{execution}' was served despite its explicit Session token"
                )
                .into())
            }
            DeletedReadAction::TimedOut => {
                return Err(format!(
                    "read for '{execution}' remained at 404/1002 after {REPLICATION_TIMEOUT:?}"
                )
                .into())
            }
            DeletedReadAction::Unexpected => match result {
                Err(error) => return Err(error.into()),
                Ok(_) => unreachable!("successful reads are session violations"),
            },
        }
        tokio::time::sleep(RETRY_DELAY).await;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DeletedReadAction {
    Deleted,
    Retry,
    SessionViolation,
    TimedOut,
    Unexpected,
}

fn deleted_read_action(status: ActualHttpStatus, before_deadline: bool) -> DeletedReadAction {
    if PLAIN_NOT_FOUND.matches(status.status_code, status.substatus) {
        DeletedReadAction::Deleted
    } else if SESSION_NOT_AVAILABLE.matches(status.status_code, status.substatus) {
        if before_deadline {
            DeletedReadAction::Retry
        } else {
            DeletedReadAction::TimedOut
        }
    } else if READ_SUCCEEDED.matches(status.status_code, status.substatus) {
        DeletedReadAction::SessionViolation
    } else {
        DeletedReadAction::Unexpected
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

fn verify_observed_statuses(
    read_case: &PostCreateReadCase,
    execution: &str,
    observed_statuses: &[ActualHttpStatus],
) -> TestResult {
    let terminal = read_case.expectation.terminal_status();
    let allowed = read_case.expectation.allowed_transient_statuses();
    if let Some(unexpected) = observed_statuses.iter().find(|actual| {
        !terminal.matches(actual.status_code, actual.substatus)
            && !allowed.iter().any(|expected| {
                expected
                    .http_status()
                    .matches(actual.status_code, actual.substatus)
            })
    }) {
        return Err(format!(
            "'{execution}' observed unexpected internal read status {unexpected:?}; expected transient {allowed:?} or terminal {terminal:?}; observed {observed_statuses:?}"
        )
        .into());
    }
    Ok(())
}

fn verify_required_transient_observed(
    read_case: &PostCreateReadCase,
    execution: &str,
    observed_statuses: &[ActualHttpStatus],
) -> TestResult {
    let allowed = read_case.expectation.allowed_transient_statuses();
    if !allowed.is_empty()
        && !observed_statuses.iter().any(|actual| {
            allowed.iter().any(|expected| {
                expected
                    .http_status()
                    .matches(actual.status_code, actual.substatus)
            })
        })
    {
        return Err(format!(
            "'{execution}' reached terminal status without observing required transient {allowed:?}; observed {observed_statuses:?}"
        )
        .into());
    }
    Ok(())
}

fn assert_initial_read_region(
    diagnostics: &azure_data_cosmos::diagnostics::DiagnosticsContext,
    expected_region: &Region,
    execution: &str,
) {
    assert_eq!(
        diagnostics
            .requests()
            .first()
            .and_then(|request| request.region()),
        Some(expected_region),
        "'{execution}' must begin its read in the selected region"
    );
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
        "smokeTests" => Ok(Region::EAST_US),
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

fn parse_read_consistency(value: &str) -> TestResult<ReadConsistencyStrategy> {
    value.parse::<ReadConsistencyStrategy>().map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_matching_distinguishes_wildcard_zero_and_exact_substatus() {
        assert!(READ_SUCCEEDED.matches(StatusCode::Ok, Some(42)));
        assert!(PLAIN_NOT_FOUND.matches(StatusCode::NotFound, None));
        assert!(PLAIN_NOT_FOUND.matches(StatusCode::NotFound, Some(0)));
        assert!(!PLAIN_NOT_FOUND.matches(StatusCode::NotFound, Some(1002)));
        assert!(SESSION_NOT_AVAILABLE.matches(StatusCode::NotFound, Some(1002)));
    }

    #[test]
    fn successful_read_after_delete_is_a_session_violation() {
        let success = ActualHttpStatus {
            status_code: StatusCode::Ok,
            substatus: None,
        };
        assert_eq!(
            deleted_read_action(success, true),
            DeletedReadAction::SessionViolation
        );
    }

    #[test]
    fn internal_attempts_must_match_the_case_allowlist() {
        let case = PostCreateReadCase::new(
            "plain-not-found-only",
            Some(ReadConsistencyStrategy::Eventual),
            SessionTokenBehavior::Omitted,
            eventually_succeeds_after(TransientReadStatus::PlainNotFound),
        );
        let allowed = [
            ActualHttpStatus {
                status_code: StatusCode::NotFound,
                substatus: Some(0),
            },
            ActualHttpStatus {
                status_code: StatusCode::Ok,
                substatus: None,
            },
        ];
        assert!(verify_observed_statuses(&case, "allowed", &allowed).is_ok());

        let disallowed = [
            ActualHttpStatus {
                status_code: StatusCode::NotFound,
                substatus: Some(1002),
            },
            allowed[1],
        ];
        assert!(verify_observed_statuses(&case, "disallowed", &disallowed).is_err());
    }

    #[test]
    fn eventual_success_requires_an_expected_transient() {
        let case = PostCreateReadCase::new(
            "session-not-available-first",
            Some(ReadConsistencyStrategy::Session),
            SessionTokenBehavior::ExplicitCreateResponse,
            eventually_succeeds_after(TransientReadStatus::SessionNotAvailable),
        );
        let success = ActualHttpStatus {
            status_code: StatusCode::Ok,
            substatus: None,
        };
        assert!(verify_required_transient_observed(&case, "missing", &[success]).is_err());

        let observed = [
            ActualHttpStatus {
                status_code: StatusCode::NotFound,
                substatus: Some(1002),
            },
            success,
        ];
        assert!(verify_required_transient_observed(&case, "observed", &observed).is_ok());
    }
}
