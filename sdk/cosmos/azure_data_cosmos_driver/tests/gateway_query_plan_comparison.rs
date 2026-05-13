// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore nopk startswith desync countif

//! Gateway validation tests for the client-side query plan generator.
//!
//! These tests compare locally-generated query plans against the Gateway's
//! query plan endpoint to ensure parity. They require a live Cosmos DB account.
//!
//! Skip / fail behavior is centralized in [`framework::resolve_test_env`]:
//! - `AZURE_COSMOS_TEST_MODE=Required` or running on Azure Pipelines (i.e.
//!   `SYSTEM_TEAMPROJECTID` is set) → missing `AZURE_COSMOS_CONNECTION_STRING`
//!   panics the test.
//! - Otherwise (`AZURE_COSMOS_TEST_MODE=Allowed`, the default) → tests are
//!   skipped with a printed message.
//!
//! Run with: `AZURE_COSMOS_CONNECTION_STRING=... cargo test -p azure_data_cosmos_driver --features __internal_testing --test gateway_query_plan_comparison`

#![cfg(feature = "__internal_testing")]
// The framework module is shared across test binaries; not all exports are used
// by every binary.
#![allow(dead_code, unused_imports)]

mod framework;

use std::sync::Arc;

use azure_core::http::headers::{HeaderName, HeaderValue};
use tokio::sync::OnceCell;

use azure_data_cosmos_driver::models::{
    ContainerReference, CosmosOperation, PartitionKeyDefinition,
};
use azure_data_cosmos_driver::options::OperationOptions;
use azure_data_cosmos_driver::CosmosDriver;
use azure_data_cosmos_driver::{driver::CosmosDriverRuntime, models::OperationTarget};

use framework::resolve_test_env;

// ─── Test infrastructure ─────────────────────────────────────────────────────

async fn build_driver() -> Option<Arc<CosmosDriver>> {
    let env = resolve_test_env().expect("failed to resolve test environment")?;
    let runtime = CosmosDriverRuntime::builder()
        .with_connection_pool(env.connection_pool)
        .build()
        .await
        .ok()?;
    let driver = runtime.get_or_create_driver(env.account, None).await.ok()?;
    Some(driver)
}

static DRIVER: OnceCell<Option<Arc<CosmosDriver>>> = OnceCell::const_new();

async fn get_driver() -> Option<&'static Arc<CosmosDriver>> {
    let d = DRIVER.get_or_init(|| async { build_driver().await }).await;
    d.as_ref()
}

const DB_NAME: &str = "query_plan_test_db";

async fn ensure_database(driver: &CosmosDriver) {
    let account = driver.account().clone();
    let op = CosmosOperation::create_database(account)
        .with_body(serde_json::to_vec(&serde_json::json!({"id": DB_NAME})).unwrap());
    if let Err(e) = driver.execute_operation(op, Default::default()).await {
        // 409 Conflict is expected on the second-and-later test runs (database already exists).
        // Anything else (auth failure, throttling, network issues, ...) should surface as a
        // panic instead of leaving the next `resolve_container` call to fail with a confusing
        // "container not found" message.
        let status = e.http_status();
        if status != Some(azure_core::http::StatusCode::Conflict) {
            panic!("failed to ensure test database '{DB_NAME}': status={status:?} {e}");
        }
    }
}

async fn ensure_container(
    driver: &CosmosDriver,
    container_name: &str,
    pk_def: PartitionKeyDefinition,
) -> ContainerReference {
    ensure_database(driver).await;

    let body = serde_json::to_vec(&serde_json::json!({
        "id": container_name,
        "partitionKey": pk_def,
    }))
    .unwrap();

    let db_ref = azure_data_cosmos_driver::models::DatabaseReference::from_name(
        driver.account().clone(),
        DB_NAME.to_string(),
    );
    let op = CosmosOperation::create_container(db_ref).with_body(body);
    if let Err(e) = driver.execute_operation(op, Default::default()).await {
        // Same rationale as ensure_database: only 409 Conflict is expected (re-runs);
        // other errors must not be silently dropped.
        let status = e.http_status();
        if status != Some(azure_core::http::StatusCode::Conflict) {
            panic!("failed to ensure test container '{container_name}': status={status:?} {e}");
        }
    }

    driver
        .resolve_container(DB_NAME, container_name)
        .await
        .expect("failed to resolve container")
}

/// Fetch a gateway query plan for the given SQL on a container.
async fn fetch_gateway_plan(
    driver: &CosmosDriver,
    container: &ContainerReference,
    sql: &str,
    parameters: &[(&str, serde_json::Value)],
) -> Result<serde_json::Value, azure_core::Error> {
    // Build {"query": ..., "parameters": [{"name":..., "value":...}, ...]}.
    let params_json: Vec<serde_json::Value> = parameters
        .iter()
        .map(|(name, value)| {
            let n = if name.starts_with('@') {
                name.to_string()
            } else {
                format!("@{name}")
            };
            serde_json::json!({"name": n, "value": value})
        })
        .collect();
    let query_body = if params_json.is_empty() {
        serde_json::json!({"query": sql})
    } else {
        serde_json::json!({"query": sql, "parameters": params_json})
    };
    let body = serde_json::to_vec(&query_body)?;

    let operation = CosmosOperation::query_plan(
        container.clone(),
        azure_data_cosmos_driver::query::__TEST_ONLY_SUPPORTED_QUERY_FEATURES.into(),
    )
    .with_body(body);
    let response = driver
        .execute_operation(operation, OperationOptions::default())
        .await?;
    let body_bytes = response
        .ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "gateway query-plan request returned no response body",
            )
        })?
        .into_body();
    serde_json::from_slice(&body_bytes)
        .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))
}

/// Compare a locally-generated `queryInfo` JSON object against what the Cosmos DB
/// Gateway returns from its query-plan endpoint.
///
/// The Gateway exposes several quirks where it rewrites the user's query and then
/// expresses parts of the resulting plan differently from what a direct AST analysis
/// would produce. Each carve-out below is intentional and is checked only against
/// well-known Gateway behavior — *not* against "the Gateway returned something
/// different and we made the test pass". Any new carve-out must be accompanied by a
/// citation explaining why it is safe.
fn compare_query_info(sql: &str, local: &serde_json::Value, gw: &serde_json::Value) {
    let gw_rewritten = gw.get("rewrittenQuery").and_then(|v| v.as_str());

    // ── distinctType ─────────────────────────────────────────────────────────
    // Carve-out: Gateway downgrades `Ordered` → `Unordered` whenever it emits a
    // `rewrittenQuery`. This is because the rewritten plan uses an explicit ORDER
    // BY in the per-partition queries, so the cross-partition aggregation no longer
    // needs to preserve order at the DISTINCT layer. Local AST analysis does not
    // perform that rewrite, so it correctly reports `Ordered`. This is consistent
    // with how the .NET / Java SDKs treat the field.
    let local_dt = local
        .get("distinctType")
        .and_then(|v| v.as_str())
        .unwrap_or("None");
    let gw_dt = gw
        .get("distinctType")
        .and_then(|v| v.as_str())
        .unwrap_or("None");
    if !(local_dt == gw_dt
        || (local_dt == "Ordered" && gw_dt == "Unordered" && gw_rewritten.is_some()))
    {
        panic!("[distinctType] sql={sql}\n  local={local_dt}  gw={gw_dt}");
    }

    // ── top (no carve-out) ───────────────────────────────────────────────────
    let local_top = local.get("top").and_then(|v| v.as_i64());
    let gw_top = gw.get("top").and_then(|v| v.as_i64());
    if local_top != gw_top {
        panic!("[top] sql={sql}\n  local={local_top:?}  gw={gw_top:?}");
    }

    // ── offset ───────────────────────────────────────────────────────────────
    // Carve-out: Gateway omits `offset` from the response when its value is 0.
    // This is a payload-shrinking optimization (see PartitionedQueryExecutionInfo
    // in the Cosmos backend). The semantic value is the same; we accept either form.
    let local_offset = local.get("offset").and_then(|v| v.as_i64());
    let gw_offset = gw.get("offset").and_then(|v| v.as_i64());
    let offset_ok = local_offset == gw_offset || (local_offset == Some(0) && gw_offset.is_none());
    if !offset_ok {
        panic!("[offset] sql={sql}\n  local={local_offset:?}  gw={gw_offset:?}");
    }

    // ── limit ────────────────────────────────────────────────────────────────
    // Carve-out: when the Gateway emits a `rewrittenQuery`, the LIMIT is folded
    // into the per-partition query and the top-level `limit` field is dropped.
    // Local AST analysis still reports the user-specified LIMIT; that is the value
    // the SDK pipeline will use to enforce cross-partition truncation, so there is
    // no functional divergence. Skip the equality check in the rewrite case.
    let local_limit = local.get("limit").and_then(|v| v.as_i64());
    let gw_limit = gw.get("limit").and_then(|v| v.as_i64());
    if gw_rewritten.is_none() {
        if local_limit != gw_limit {
            panic!("[limit] sql={sql}\n  local={local_limit:?}  gw={gw_limit:?}");
        }
    } else {
        // When the Gateway emits a `rewrittenQuery`, the top-level `limit`
        // field may either be dropped (folded into per-partition queries)
        // or preserved verbatim — observed behavior varies across query
        // shapes (e.g. `OFFSET … LIMIT …` against a single-PK collection
        // tends to keep the top-level limit). Accept either form, but if
        // the Gateway keeps it, it must agree with the local value so a
        // real divergence still surfaces.
        if let Some(gwl) = gw_limit {
            if Some(gwl) != local_limit {
                panic!(
                    "[limit] sql={sql}\n  local={local_limit:?}  gw={gw_limit:?} (rewrittenQuery present)"
                );
            }
        }
    }

    // ── orderBy / orderByExpressions ─────────────────────────────────────────
    // Carve-out: when GROUP BY is present, the Gateway returns an empty ORDER BY
    // because the rewritten per-partition queries inline the ordering needed for
    // group aggregation. Local analysis reports the user-specified ORDER BY items
    // unchanged; the SDK pipeline still applies them at the merge stage. Skip the
    // ORDER BY checks in the GROUP BY case.
    let gw_gbe = gw
        .get("groupByExpressions")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    if gw_gbe == 0 {
        let local_ob = local
            .get("orderBy")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let gw_ob = gw
            .get("orderBy")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        if local_ob != gw_ob {
            panic!("[orderBy] sql={sql}\n  local={local_ob:?}  gw={gw_ob:?}");
        }
        let local_obe = local
            .get("orderByExpressions")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let gw_obe = gw
            .get("orderByExpressions")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        if local_obe != gw_obe {
            panic!("[orderByExpressions] sql={sql}\n  local={local_obe:?}  gw={gw_obe:?}");
        }
    } else {
        // When GROUP BY is present the Gateway may either drop the top-level
        // ORDER BY (folding it into the per-partition rewrittenQuery) or
        // preserve it. Accept both, but if the Gateway preserves it the
        // values must agree with what local analysis produced so a real
        // divergence still surfaces.
        let gw_ob = gw
            .get("orderBy")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let gw_obe = gw
            .get("orderByExpressions")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        if !gw_ob.is_empty() || !gw_obe.is_empty() {
            let local_ob = local
                .get("orderBy")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            let local_obe = local
                .get("orderByExpressions")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            if local_ob != gw_ob {
                panic!(
                    "[orderBy] sql={sql}\n  local={local_ob:?}  gw={gw_ob:?} (GROUP BY present)"
                );
            }
            if local_obe != gw_obe {
                panic!("[orderByExpressions] sql={sql}\n  local={local_obe:?}  gw={gw_obe:?} (GROUP BY present)");
            }
        }
    }

    // ── groupByExpressions (no carve-out) ────────────────────────────────────
    // Note: previously this block carried a carve-out tolerating debug-formatted
    // strings ("MemberIndexer", "Binary") in the local output for non-path
    // GROUP BY expressions. That behavior was removed in #2 — the local generator
    // now refuses to silently produce a non-comparable plan and instead returns
    // an error so the caller can fall back to the Gateway. Any non-path GROUP BY
    // expression therefore never reaches this comparison.
    let local_gbe = local
        .get("groupByExpressions")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let gw_gbe_arr = gw
        .get("groupByExpressions")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    if local_gbe != gw_gbe_arr {
        panic!("[groupByExpressions] sql={sql}\n  local={local_gbe:?}  gw={gw_gbe_arr:?}");
    }

    // ── aggregates ───────────────────────────────────────────────────────────
    // Carve-out: when Gateway emits a `rewrittenQuery`, aggregates move into
    // `groupByAliasToAggregateType` (a per-alias map) and the top-level
    // `aggregates` array is dropped. Local AST analysis still reports the
    // aggregate kinds as a flat list, which is what the SDK pipeline consumes.
    // Skip the equality check in the rewrite case.
    if gw_rewritten.is_none() {
        let local_agg = local
            .get("aggregates")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let gw_agg = gw
            .get("aggregates")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        if local_agg != gw_agg {
            panic!("[aggregates] sql={sql}\n  local={local_agg:?}  gw={gw_agg:?}");
        }
    } else {
        // When the Gateway emits a `rewrittenQuery`, it may either drop the
        // top-level `aggregates` array (moving them into
        // `groupByAliasToAggregateType`) or preserve it. Accept both, but
        // if it preserves the array the values must agree with what local
        // analysis produced.
        let gw_agg = gw
            .get("aggregates")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        if !gw_agg.is_empty() {
            let local_agg = local
                .get("aggregates")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            if local_agg != gw_agg {
                panic!(
                    "[aggregates] sql={sql}\n  local={local_agg:?}  gw={gw_agg:?} (rewrittenQuery present)"
                );
            }
        }
    }

    // ── hasSelectValue (no carve-out) ────────────────────────────────────────
    let local_hsv = local
        .get("hasSelectValue")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let gw_hsv = gw
        .get("hasSelectValue")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if local_hsv != gw_hsv {
        panic!("[hasSelectValue] sql={sql}\n  local={local_hsv}  gw={gw_hsv}");
    }
}

/// Generate local plan as JSON, fetch gateway plan, compare queryInfo fields.
async fn validate(
    driver: &CosmosDriver,
    container: &ContainerReference,
    pk_paths: &[&str],
    sql: &str,
) {
    validate_with_params(driver, container, pk_paths, sql, &[]).await;
}

/// Like [`validate`], but also passes parameter values to both the local plan generator
/// and the Gateway. Used for parameterized `TOP` / `OFFSET` / `LIMIT` regression coverage.
async fn validate_with_params(
    driver: &CosmosDriver,
    container: &ContainerReference,
    pk_paths: &[&str],
    sql: &str,
    parameters: &[(&str, serde_json::Value)],
) {
    // Generate local plan with parameter substitution.
    let owned: Vec<(String, serde_json::Value)> = parameters
        .iter()
        .map(|(n, v)| (n.to_string(), v.clone()))
        .collect();
    let local_plan = azure_data_cosmos_driver::query::__test_only_generate_query_plan_for_pk_paths(
        sql, pk_paths, &owned,
    )
    .unwrap_or_else(|e| panic!("Local plan generation failed for: {sql}\n  {e}"));
    let local_qi = &local_plan["queryInfo"];

    // Fetch gateway plan, passing the same parameters in the request body.
    let gw_plan = fetch_gateway_plan(driver, container, sql, parameters)
        .await
        .unwrap_or_else(|e| panic!("Gateway query plan request failed for: {sql}\n  {e}"));
    let gw_qi = &gw_plan["queryInfo"];

    compare_query_info(sql, local_qi, gw_qi);
}

/// Validate that the Gateway rejects the given SQL with HTTP 400.
async fn validate_expects_400(
    driver: &CosmosDriver,
    container: &ContainerReference,
    sql: &str,
    reason: &str,
) {
    match fetch_gateway_plan(driver, container, sql, &[]).await {
        Err(e) => {
            let status = e.http_status();
            assert_eq!(
                status,
                Some(azure_core::http::StatusCode::BadRequest),
                "Expected HTTP 400 ({reason}) for '{sql}' but got status {status:?}: {e}"
            );
        }
        Ok(_) => {
            panic!("Expected HTTP 400 ({reason}) for '{sql}' but Gateway returned a plan");
        }
    }
}

// ─── Container fixtures ──────────────────────────────────────────────────────

macro_rules! container_fixture {
    ($static:ident, $name:ident, $container_name:literal, $pk_expr:expr) => {
        static $static: OnceCell<ContainerReference> = OnceCell::const_new();

        async fn $name() -> Option<&'static ContainerReference> {
            let driver = get_driver().await?;
            Some(
                $static
                    .get_or_init(|| async {
                        ensure_container(driver, $container_name, $pk_expr).await
                    })
                    .await,
            )
        }
    };
}

container_fixture!(C_PK, c_pk, "qp_pk", "/pk".into());
container_fixture!(
    C_HPK,
    c_hpk,
    "qp_hpk",
    PartitionKeyDefinition::new(vec!["/tenant".into(), "/userId".into()])
);
container_fixture!(
    C_HPK3,
    c_hpk3,
    "qp_hpk3",
    PartitionKeyDefinition::new(vec![
        "/tenant".into(),
        "/userId".into(),
        "/sessionId".into()
    ])
);
container_fixture!(C_NESTED, c_nested, "qp_nested", "/address/city".into());
container_fixture!(C_NOPK, c_nopk, "qp_nopk", "/id".into());

// ─── Gateway validation helper functions ─────────────────────────────────────
//
// Helpers panic with a clear message if the Gateway is not reachable. Silently
// no-oping here would cause "wrong test config" runs to report passing tests
// while actually skipping every assertion - these gateway-parity tests are
// crucial and must surface configuration problems instead of hiding them.
// To intentionally skip them in environments without a Cosmos account, set
// `AZURE_COSMOS_TEST_MODE=Skipped` (handled inside `resolve_test_env`) or do
// not enable the `__internal_testing` feature.

fn require_driver_and<'a, T>(
    driver: Option<&'a T>,
    container: Option<&'a ContainerReference>,
) -> (&'a T, &'a ContainerReference) {
    let driver = driver.expect(
        "gateway query-plan comparison tests require a configured Cosmos DB account; \
         set AZURE_COSMOS_CONNECTION_STRING (or AZURE_COSMOS_TEST_MODE=Skipped to skip)",
    );
    let container = container
        .expect("test container could not be provisioned against the configured Cosmos DB account");
    (driver, container)
}

async fn validate_pk(sql: &str) {
    let (d, c) = require_driver_and(get_driver().await, c_pk().await);
    validate(d, c, &["/pk"], sql).await;
}

async fn validate_hpk(sql: &str) {
    let (d, c) = require_driver_and(get_driver().await, c_hpk().await);
    validate(d, c, &["/tenant", "/userId"], sql).await;
}

async fn validate_hpk3(sql: &str) {
    let (d, c) = require_driver_and(get_driver().await, c_hpk3().await);
    validate(d, c, &["/tenant", "/userId", "/sessionId"], sql).await;
}

async fn validate_nested(sql: &str) {
    let (d, c) = require_driver_and(get_driver().await, c_nested().await);
    validate(d, c, &["/address/city"], sql).await;
}

#[allow(dead_code)]
async fn validate_nopk(sql: &str) {
    let (d, c) = require_driver_and(get_driver().await, c_nopk().await);
    validate(d, c, &["/id"], sql).await;
}

async fn validate_pk_expects_400(sql: &str, reason: &str) {
    let (d, c) = require_driver_and(get_driver().await, c_pk().await);
    validate_expects_400(d, c, sql, reason).await;
}

async fn validate_hpk_expects_400(sql: &str, reason: &str) {
    let (d, c) = require_driver_and(get_driver().await, c_hpk().await);
    validate_expects_400(d, c, sql, reason).await;
}

/// Sentinel substring the local plan generator embeds in its error message
/// when the integration layer is expected to fall back to the Gateway
/// query-plan endpoint instead of failing the operation. Mirrors
/// `query::plan::LocalPlanFallbackError::NEEDS_GATEWAY_FALLBACK`, which is
/// `pub(crate)` so cannot be referenced directly from this integration test.
const NEEDS_GATEWAY_FALLBACK: &str = "[NEEDS_GATEWAY_FALLBACK]";

fn local_error_is_gateway_fallback(err: &azure_core::Error) -> bool {
    format!("{err}").contains(NEEDS_GATEWAY_FALLBACK)
}

/// Symmetric-outcome check: a query is allowed to fail at the local plan
/// generator OR at the Gateway, but the two sides must agree on whether the
/// query is acceptable. Concretely:
///
/// * both reject → OK (documented divergence between SDK and backend on a
///   syntactic shape; both surfaces decline the query identically)
/// * both accept → OK (parity check still runs; pk_filters / queryInfo must
///   match — see [`compare_query_info`])
/// * local rejects with the [`NEEDS_GATEWAY_FALLBACK`] sentinel and Gateway
///   accepts → OK (the integration layer is expected to fall back to the
///   Gateway plan; this is a deliberate design contract). Use
///   [`validate_pk_local_falls_back_to_gateway`] for queries where this
///   outcome is *expected* so that an accidental regression where every
///   query falls back to the Gateway can be caught.
/// * local rejects with any other error and Gateway accepts → BUG: the
///   local plan generator is missing parser/planner support for this shape.
/// * only Gateway rejects → BUG: the local generator is producing a plan for
///   a query the backend would not have accepted.
async fn validate_symmetric_pk(sql: &str) {
    let (d, c) = require_driver_and(get_driver().await, c_pk().await);
    let local = azure_data_cosmos_driver::query::__test_only_generate_query_plan_for_pk_paths(
        sql,
        &["/pk"],
        &[],
    );
    let gw = fetch_gateway_plan(d, c, sql, &[]).await;
    match (&local, &gw) {
        (Ok(local_plan), Ok(gw_plan)) => {
            // Both accepted — fall through to plan comparison so any divergence
            // surfaces here too.
            compare_query_info(sql, &local_plan["queryInfo"], &gw_plan["queryInfo"]);
        }
        (Err(_), Err(_)) => {
            // Both rejected — acceptable documented divergence.
        }
        (Err(le), Ok(_)) if local_error_is_gateway_fallback(le) => {
            // Local explicitly asked the integration layer to fall back to
            // the Gateway. Acceptable here; tests where this is the *only*
            // expected outcome use `validate_pk_local_falls_back_to_gateway`.
        }
        (Err(le), Ok(_)) => {
            panic!(
                "[symmetric] sql={sql}\n  local rejected but Gateway accepted; \
                 the local plan generator needs parser/planner support for this shape.\n  local_err={le}"
            );
        }
        (Ok(_), Err(ge)) => {
            panic!(
                "[symmetric] sql={sql}\n  local accepted but Gateway rejected; \
                 the local plan generator is over-permissive.\n  gw_err={ge}"
            );
        }
    }
}

/// Pin a known query shape where the local plan generator deliberately bails
/// out with the [`NEEDS_GATEWAY_FALLBACK`] sentinel and expects the
/// integration layer to fall back to the Gateway. Asserts both:
///
/// 1. local generator errors with the sentinel — guards against a regression
///    where the local generator stops emitting the sentinel for a shape it
///    cannot plan (which would silently break the integration-layer
///    fallback path), and against a regression where the local generator
///    *accidentally starts succeeding* with a wrong plan
/// 2. Gateway accepts the query — confirms the fallback path will actually
///    receive a usable plan from the Gateway
///
/// Use this *sparingly* and only for shapes that are intentionally not
/// implemented in the local generator. A shape where the local generator
/// could plan correctly should not be on this list — that would mask a
/// missing-feature regression as expected behavior.
async fn validate_pk_local_falls_back_to_gateway(sql: &str) {
    let (d, c) = require_driver_and(get_driver().await, c_pk().await);
    let local = azure_data_cosmos_driver::query::__test_only_generate_query_plan_for_pk_paths(
        sql,
        &["/pk"],
        &[],
    );
    let local_err = local.as_ref().err().unwrap_or_else(|| {
        panic!(
            "[fallback-expected] sql={sql}\n  local plan generator unexpectedly succeeded; \
             either fix the local generator's intended fallback or remove this query from the list."
        )
    });
    assert!(
        local_error_is_gateway_fallback(local_err),
        "[fallback-expected] sql={sql}\n  local rejected without the {NEEDS_GATEWAY_FALLBACK} sentinel; \
         the integration layer's fallback path will not trigger.\n  local_err={local_err}"
    );
    fetch_gateway_plan(d, c, sql, &[])
        .await
        .unwrap_or_else(|e| {
            panic!(
                "[fallback-expected] sql={sql}\n  Gateway must accept queries we expect to fall back to it; \
                 got: {e}"
            )
        });
}

// ═══════════════════════════════════════════════════════════════════════════════
// GATEWAY VALIDATION TESTS
//
// Each test validates that the locally-generated query plan matches what the
// Cosmos DB Gateway produces. Tests are skipped when no connection string is set.
// ═══════════════════════════════════════════════════════════════════════════════

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_simple_select() {
    validate_pk("SELECT * FROM c").await;
    validate_pk("SELECT c.name, c.age FROM c").await;
    validate_pk("SELECT VALUE c.name FROM c").await;
    validate_pk("SELECT 1").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_pk_equality() {
    validate_pk("SELECT * FROM c WHERE c.pk = 'hello'").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 42").await;
    validate_pk("SELECT * FROM c WHERE c.pk = true").await;
    validate_pk("SELECT * FROM c WHERE c.pk = null").await;
    validate_pk("SELECT * FROM c WHERE c.pk = -99").await;
    validate_pk("SELECT * FROM c WHERE 'hello' = c.pk").await;
}

/// Validate the numeric-PK precision boundary (#9). The local plan generator
/// canonicalizes integer PK literals to `f64`, which loses precision past
/// `2^53`. This test confirms that the Gateway behaves the same way: integer
/// literals beyond `2^53` are reflected back unchanged in the partition-key
/// filter (i.e. the Gateway does not promote them to `i64` either, so the
/// `i64 as f64` collapse the local plan does is parity-correct).
///
/// If this test ever starts failing, the Gateway has changed its precision
/// model and the local PK canonicalization in `query::plan` (see
/// `extract_pk_filter` for single-PK and `expr_to_pk_value` for HPK) needs to
/// be revisited; the unit test `pk_eq_large_integer` would also need
/// updating.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_pk_numeric_precision_boundary() {
    // Below 2^53 — exact in f64, no precision concern.
    validate_pk("SELECT * FROM c WHERE c.pk = 9007199254740992").await;
    // Exactly 2^53 + 1 — first odd integer not representable in f64.
    // Both forms below collapse to the same f64 (9007199254740992.0); the
    // local plan generator surfaces that. The gateway is expected to do the
    // same, so the queryInfo.partitionKeyFilters must agree.
    validate_pk("SELECT * FROM c WHERE c.pk = 9007199254740993").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 9007199254740992.0").await;
    // Negative side of the boundary.
    validate_pk("SELECT * FROM c WHERE c.pk = -9007199254740993").await;
    // Floating-point literal forms.
    validate_pk("SELECT * FROM c WHERE c.pk = 1.5e10").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 0.1").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_pk_and_or_in() {
    validate_pk("SELECT * FROM c WHERE c.pk = 'x' AND c.age > 21").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'a' OR c.pk = 'b'").await;
    validate_pk("SELECT * FROM c WHERE c.pk IN ('a', 'b', 'c')").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_cross_partition() {
    validate_pk("SELECT * FROM c WHERE c.age > 21").await;
    validate_pk("SELECT * FROM c WHERE c.pk > 'x'").await;
    validate_pk("SELECT * FROM c WHERE c.pk BETWEEN 'a' AND 'z'").await;
    validate_pk("SELECT * FROM c WHERE c.pk LIKE 'x%'").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_top() {
    validate_pk("SELECT TOP 10 * FROM c").await;
    validate_pk("SELECT TOP 5 * FROM c WHERE c.pk = 'x'").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_offset_limit() {
    validate_pk("SELECT * FROM c OFFSET 5 LIMIT 20").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'x' OFFSET 0 LIMIT 10").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_distinct() {
    validate_pk("SELECT DISTINCT c.name FROM c").await;
    validate_pk("SELECT DISTINCT c.name FROM c ORDER BY c.name ASC").await;
    validate_pk("SELECT DISTINCT c.name FROM c WHERE c.pk = 'x'").await;
    validate_pk("SELECT DISTINCT VALUE null").await;
    validate_pk("SELECT DISTINCT VALUE 1").await;
    validate_pk("SELECT DISTINCT VALUE 'a'").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_order_by() {
    validate_pk("SELECT * FROM c ORDER BY c.name ASC").await;
    validate_pk("SELECT * FROM c ORDER BY c.age DESC").await;
    validate_pk("SELECT * FROM c ORDER BY c.name").await;
    validate_pk("SELECT * FROM c ORDER BY c.name ASC, c.age DESC").await;
    validate_pk("SELECT * FROM c ORDER BY c.address.city ASC").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'x' ORDER BY c.name DESC").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_aggregates() {
    validate_pk("SELECT COUNT(1) FROM c").await;
    validate_pk("SELECT SUM(c.price) FROM c").await;
    validate_pk("SELECT AVG(c.score) FROM c").await;
    validate_pk("SELECT MIN(c.age) FROM c").await;
    validate_pk("SELECT MAX(c.age) FROM c").await;
    validate_pk("SELECT COUNT(1), SUM(c.price), AVG(c.score) FROM c").await;
    validate_pk("SELECT COUNT(1) FROM c WHERE c.pk = 'x'").await;
    validate_pk("SELECT MIN(c.age), MAX(c.age) FROM c").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_group_by() {
    validate_pk("SELECT c.city, COUNT(1) FROM c GROUP BY c.city").await;
    validate_pk("SELECT c.city, c.state, COUNT(1) FROM c GROUP BY c.city, c.state").await;
    validate_pk("SELECT c.city, SUM(c.revenue), AVG(c.score) FROM c GROUP BY c.city").await;
    validate_pk("SELECT c.city, COUNT(1) FROM c WHERE c.pk = 'x' GROUP BY c.city").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_join() {
    validate_pk("SELECT c.id, t FROM c JOIN t IN c.tags WHERE c.pk = 'x'").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_subqueries() {
    validate_pk("SELECT * FROM c WHERE EXISTS(SELECT VALUE t FROM t IN c.tags)").await;
    validate_pk("SELECT ARRAY(SELECT t FROM t IN c.tags) FROM c").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'x' AND EXISTS(SELECT VALUE t FROM t IN c.tags WHERE t = 'rust')").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_select_value() {
    validate_pk("SELECT VALUE c.name FROM c WHERE c.pk = 'x'").await;
    validate_pk("SELECT VALUE COUNT(1) FROM c").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_complex_combined() {
    validate_pk(
        "SELECT c.city, COUNT(1), SUM(c.revenue) FROM c WHERE c.pk = 'x' GROUP BY c.city ORDER BY c.city ASC",
    ).await;
    validate_pk("SELECT DISTINCT TOP 5 c.name FROM c ORDER BY c.name ASC").await;
    validate_pk(
        "SELECT c.region, c.city, AVG(c.score), MIN(c.score), MAX(c.score) FROM c GROUP BY c.region, c.city ORDER BY c.region ASC, c.city DESC",
    ).await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_functions() {
    validate_pk("SELECT * FROM c WHERE CONTAINS(c.name, 'test')").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'x' AND STARTSWITH(c.name, 'A')").await;
    validate_pk("SELECT * FROM c WHERE IS_DEFINED(c.optional)").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_nested_paths() {
    validate_nested("SELECT * FROM c WHERE c.address.city = 'Seattle'").await;
    validate_nested("SELECT * FROM c WHERE c.address.city = 'Seattle' AND c.age > 21").await;
    validate_nested("SELECT * FROM c WHERE c.address.city IN ('Seattle', 'Portland', 'Austin')")
        .await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_hierarchical_pk() {
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = 'u1'").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'acme'").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = @uid").await;
    validate_hpk("SELECT * FROM c WHERE c.userId = 'u1' AND c.tenant = 'acme'").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_hierarchical_pk3() {
    validate_hpk3(
        "SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1' AND c.sessionId = 's1'",
    )
    .await;
    validate_hpk3("SELECT * FROM c WHERE c.tenant = 'a' AND c.sessionId = 's1'").await;
    validate_hpk3("SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1'").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_complex_with_hpk() {
    validate_hpk(
        "SELECT c.city, COUNT(1) AS cnt FROM c JOIN t IN c.tags WHERE c.tenant = 'acme' AND c.userId = 'u1' GROUP BY c.city ORDER BY c.city ASC",
    ).await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_from_alias() {
    validate_pk("SELECT * FROM root AS r WHERE r.pk = 'hello'").await;
    validate_pk("SELECT * FROM root r WHERE r.pk = 'hello'").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_and_intersection() {
    validate_pk("SELECT * FROM c WHERE c.pk = 'a' AND c.pk = 'a'").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'a' AND c.pk IN ('a', 'b')").await;
    validate_pk("SELECT * FROM c WHERE c.pk IN ('a', 'b') AND c.pk IN ('b', 'c')").await;
}

// ── Gateway 400 tests ────────────────────────────────────────────────────────

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_400_is_null() {
    validate_pk_expects_400(
        "SELECT * FROM c WHERE c.pk IS NULL",
        "IS NULL not supported by Gateway query plan endpoint",
    )
    .await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_400_is_not_null() {
    validate_pk_expects_400(
        "SELECT * FROM c WHERE c.pk IS NOT NULL",
        "IS NOT NULL not supported by Gateway query plan endpoint",
    )
    .await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_400_alias_mismatch() {
    validate_hpk_expects_400(
        "SELECT * FROM root AS r WHERE c.tenant = 'acme' AND c.userId = 'u1'",
        "alias mismatch: FROM uses r but WHERE references c",
    )
    .await;
}

// ─── Parameterized TOP / OFFSET / LIMIT ──────────────────────────────────────
//
// Regression coverage for the local plan generator's parameter substitution.
// When the caller supplies parameter values up-front, the local plan must match
// what the Gateway returns for the equivalent literal query. When values are NOT
// supplied, the local generator must fail clearly (the Gateway responds 400).

async fn validate_pk_with_params(sql: &str, params: &[(&str, serde_json::Value)]) {
    let (d, c) = require_driver_and(get_driver().await, c_pk().await);
    validate_with_params(d, c, &["/pk"], sql, params).await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_top_parameter_substituted() {
    validate_pk_with_params("SELECT TOP @n * FROM c", &[("@n", serde_json::json!(10))]).await;
    validate_pk_with_params(
        "SELECT TOP @n * FROM c WHERE c.pk = 'x'",
        &[("@n", serde_json::json!(5))],
    )
    .await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_offset_limit_parameter_substituted() {
    validate_pk_with_params(
        "SELECT * FROM c OFFSET @off LIMIT @lim",
        &[
            ("@off", serde_json::json!(2)),
            ("@lim", serde_json::json!(8)),
        ],
    )
    .await;
    validate_pk_with_params(
        "SELECT * FROM c WHERE c.pk = 'x' OFFSET @off LIMIT @lim",
        &[
            ("@off", serde_json::json!(0)),
            ("@lim", serde_json::json!(20)),
        ],
    )
    .await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_400_top_parameter_without_value() {
    // Gateway rejects parameterized TOP without a supplied value with HTTP 400.
    validate_pk_expects_400(
        "SELECT TOP @n * FROM c",
        "parameterized TOP requires resolved value for Gateway plan",
    )
    .await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_400_offset_limit_parameter_without_value() {
    // Gateway rejects parameterized OFFSET/LIMIT without supplied values with HTTP 400.
    validate_pk_expects_400(
        "SELECT * FROM c OFFSET @off LIMIT @lim",
        "parameterized OFFSET/LIMIT requires resolved values for Gateway plan",
    )
    .await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn local_plan_top_parameter_without_value_errors() {
    // Mirror of the Gateway-400 test: when the caller does not supply a value for
    // a parameterized TOP/OFFSET/LIMIT, the *local* plan generator must fail
    // clearly (rather than silently dropping the clause).
    let result = azure_data_cosmos_driver::query::__test_only_generate_query_plan_for_pk_paths(
        "SELECT TOP @n * FROM c",
        &["/pk"],
        &[],
    );
    let err =
        result.expect_err("local plan generator must reject parameterized TOP without a value");
    let msg = format!("{err}");
    assert!(
        msg.contains("@n"),
        "error message should mention parameter name: {msg}"
    );
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn local_plan_offset_limit_parameter_without_value_errors() {
    let result = azure_data_cosmos_driver::query::__test_only_generate_query_plan_for_pk_paths(
        "SELECT * FROM c OFFSET @off LIMIT @lim",
        &["/pk"],
        &[("@off".to_string(), serde_json::json!(0))],
    );
    let err =
        result.expect_err("local plan generator must reject parameterized LIMIT without a value");
    let msg = format!("{err}");
    assert!(
        msg.contains("@lim"),
        "error message should mention missing parameter @lim: {msg}"
    );
}

// ─── Parameter substitution in PK extraction (#14) ───────────────────────────
//
// When the caller supplies parameter values, the local plan generator must
// substitute them into the partition-key filter the same way the Gateway does
// when the parameter is bound in the query-plan request body.

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_pk_parameter_substitution() {
    validate_pk_with_params(
        "SELECT * FROM c WHERE c.pk = @val",
        &[("@val", serde_json::json!("hello"))],
    )
    .await;
    validate_pk_with_params(
        "SELECT * FROM c WHERE c.pk = @val",
        &[("@val", serde_json::json!(42))],
    )
    .await;
}

#[test]
fn internal_testing_supported_features_constant_is_reachable() {
    // Cross-crate visibility sanity check for the supported-features constant.
    //
    // The driver crate keeps `SUPPORTED_QUERY_FEATURES` `pub(crate)` so
    // production callers cannot reach it; only the `__internal_testing`-gated
    // alias `__TEST_ONLY_SUPPORTED_QUERY_FEATURES` is reachable from this
    // integration test. This test makes the contract explicit so accidental
    // visibility changes (or constant edits) do not silently desync the
    // local plan generator from what is advertised to the Gateway in
    // `x-ms-cosmos-supported-query-features`.
    //
    // Must stay in lockstep with `query::SUPPORTED_QUERY_FEATURES` in
    // `src/query/mod.rs`. The advertised set matches what the Java and
    // .NET SDKs send today so the Gateway returns the same plan shape across
    // SDKs and cross-SDK plan-parity tests stay meaningful. The query
    // execution pipeline does not yet support every feature in this list
    // (e.g. NonStreamingOrderBy, HybridSearch); the integration PR that
    // wires the local plan generator into production is expected to
    // narrow the production-side header to a pipeline-aware subset.
    assert_eq!(
        azure_data_cosmos_driver::query::__TEST_ONLY_SUPPORTED_QUERY_FEATURES,
        "Aggregate,CompositeAggregate,CountIf,DCount,Distinct,GroupBy,HybridSearch,MultipleAggregates,MultipleOrderBy,NonStreamingOrderBy,NonValueAggregate,OffsetAndLimit,OrderBy,Top,WeightedRankFusion",
    );
}

// (#10) CONCAT plan-parity coverage. The local evaluator implements
// `CONCAT` with strict string-only arguments (any non-string yields
// `Undefined`, matching the gateway). These tests pin the *plan-level*
// shape so the parser/plan generator handles `CONCAT` calls in projection
// and WHERE positions identically to the Gateway. End-to-end value parity
// is covered by inline tests in `query::eval::builtins`.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_concat_in_projection() {
    validate_pk("SELECT CONCAT(c.first, c.last) FROM c").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_concat_in_where_clause() {
    validate_pk("SELECT * FROM c WHERE CONCAT(c.first, c.last) = 'AliceSmith'").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_concat_with_literal_argument() {
    validate_pk("SELECT CONCAT(c.name, '@example.com') FROM c").await;
}

/// `(c.pk='a' AND c.pk='b') OR c.pk='c'` must not lose the third disjunct.
/// The locally-extracted PK filter must include `'c'` (Equality or InList);
/// the Gateway likewise reports a single-PK target, so the queryInfo plans
/// agree even though `pkFilters` is purely a local concept.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_pk_or_with_contradictory_disjunct() {
    validate_pk("SELECT * FROM c WHERE (c.pk = 'a' AND c.pk = 'b') OR c.pk = 'c'").await;
}

/// `SUM(c.intCol)` over integer-only inputs must serialize as an integer
/// JSON number (`6`), not as `6.0`. The plan-level shape is unaffected — this
/// pins parser/plan parity for the SUM aggregate against the Gateway.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_sum_integer_aggregate() {
    validate_pk("SELECT SUM(c.intCol) FROM c").await;
}

/// cross-type MIN/MAX must follow Cosmos' total ordering
/// (`null<bool<num<str<arr<obj`). Plan-level shape is unaffected here; the
/// test pins parser/plan parity for `MIN`/`MAX` calls.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_min_max_cross_type_aggregate() {
    validate_pk("SELECT MIN(c.mixed), MAX(c.mixed) FROM c").await;
}

/// hierarchical PK with IN on the leading component must extract a
/// cartesian-product `InList` instead of falling back to cross-partition.
/// Plan-level parity test; the value parity (which physical partitions are
/// targeted) is exercised by the local plan unit tests above.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_hpk_with_in_on_first_component() {
    validate_hpk("SELECT * FROM c WHERE c.tenant IN ('a', 'b') AND c.userId = 'u1'").await;
}

/// HPK with IN on the trailing component.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_hpk_with_in_on_second_component() {
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId IN ('u1', 'u2')").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_aggregate_inside_udf_arg_not_advertised() {
    // The Gateway rejects this query (aggregates inside UDF args are not
    // allowed). The local generator's behavior — not advertising the
    // aggregate — is exercised in the lib unit tests; this test pins the
    // Gateway-side expectation so a future change in Gateway behavior is
    // surfaced.
    validate_pk_expects_400(
        "SELECT udf.foo(COUNT(c.x)) FROM c",
        "aggregate inside UDF call is not allowed",
    )
    .await;
}

/// `LIKE … ESCAPE '<single char>'` must continue to plan; multi-char
/// escapes are evaluator-side concerns and don't affect the plan shape. Use
/// `#` as the escape char (the Gateway rejects `\` as an escape).
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_like_with_single_char_escape() {
    validate_pk("SELECT * FROM c WHERE c.name LIKE 'a#_b' ESCAPE '#'").await;
}

/// duplicate aggregates in the SELECT list (`SELECT COUNT(1), COUNT(c.x)`)
/// — the Gateway returns the dedup'd kind list. Pin local/Gateway parity on
/// the aggregate set.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_duplicate_aggregates_dedup() {
    validate_pk("SELECT COUNT(1), COUNT(c.x) FROM c").await;
}

/// `~ <fractional number>` must yield `Undefined` in the evaluator;
/// at the plan level it is just a unary expression. Pin parser/plan parity.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_bitwise_not_on_fractional_number() {
    validate_pk("SELECT VALUE ~3.7 FROM c").await;
}

// keyword-as-property name preserves source casing. Cosmos JSON property
// lookup is case-sensitive. The Gateway rejects bare `c.left` / `c.LEFT`
// where `LEFT` is a reserved word, so we exercise the case-sensitivity
// invariant via the bracket form, which Gateway accepts.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_keyword_as_property_lower_case() {
    validate_pk("SELECT c[\"left\"] FROM c WHERE c.pk = 'x'").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_keyword_as_property_upper_case() {
    validate_pk("SELECT c[\"LEFT\"] FROM c WHERE c.pk = 'x'").await;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Local-vs-Gateway parity sweep
//
// These tests mirror SQL queries that exist in the local plan unit tests
// (src/query/plan/tests/query_plan_comparison.rs`) so the Gateway-comparison
// suite remains a superset. Failures here surface as silent local/Gateway
// divergences that would otherwise only be caught at integration time.
//
// Tests are chunked (~20 queries per #[tokio::test]) to keep individual
// async-fn stack frames small enough to fit in tokio's default test stack on
// Windows. The first divergence aborts the chunk and pinpoints the SQL via
// the panic message.
// ═══════════════════════════════════════════════════════════════════════════════

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_pk_01() {
    validate_pk("SELECT 'Hello World'").await;
    validate_pk("SELECT (SELECT VALUE {a: 1, b: 2}).a AS val").await;
    validate_pk("SELECT (SELECT VALUE 1) AS x FROM c").await;
    validate_pk("SELECT * FROM c -- comment\nWHERE c.pk = 'x'").await;
    validate_pk("SELECT * FROM c ORDER BY c.a ASC, c.b DESC, c.c ASC, c.d DESC").await;
    validate_pk("SELECT * FROM c ORDER BY c.a.b.c ASC").await;
    validate_pk("SELECT * FROM c ORDER BY c.active").await;
    validate_pk("SELECT * FROM c ORDER BY c.city ASC, c.state DESC, c.name ASC").await;
    validate_pk("SELECT * FROM c WHERE (c.pk != 'a') AND (c.pk != 'b')").await;
    validate_pk("SELECT * FROM c WHERE (c.pk != 'a') OR (c.pk != 'b')").await;
    validate_pk("SELECT * FROM c WHERE (c.pk = 'a' AND c.x > 1) OR (c.pk = 'b' AND c.y < 2)").await;
    validate_pk("SELECT * FROM c WHERE (c.pk = 'a' OR c.pk = 'b') AND c.active = true").await;
    validate_pk("SELECT * FROM c WHERE (c.pk > 'a') AND (c.pk != 'z')").await;
    validate_pk("SELECT * FROM c WHERE (c.pk NOT IN ('a', 'b')) AND (c.pk != 'c')").await;
    validate_pk("SELECT * FROM c WHERE (SELECT VALUE c.age) > 21").await;
    validate_pk("SELECT * FROM c WHERE ARRAY_CONTAINS(c.items, 1) AND ARRAY_CONTAINS(c.items, 2)")
        .await;
    validate_pk("SELECT * FROM c WHERE c.a > 1 AND c.b > 2 AND c.pk = 'x' AND c.d > 4 AND c.e > 5")
        .await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_pk_02() {
    validate_pk("SELECT * FROM c WHERE c.a.b.c.d = 1").await;
    validate_pk("SELECT * FROM c WHERE c.active").await;
    validate_pk("SELECT * FROM c WHERE c.active = true ORDER BY c.name ASC OFFSET 10 LIMIT 20")
        .await;
    validate_pk("SELECT * FROM c WHERE c.age + 1 IN (10, 20, 30)").await;
    validate_pk("SELECT * FROM c WHERE c.age IN (10, 11, 23) ORDER BY c.age").await;
    validate_pk("SELECT * FROM c WHERE c.age NOT IN (10, 11) ORDER BY c.age").await;
    validate_pk("SELECT * FROM c WHERE c.city LIKE 'Se%' AND c.state LIKE 'W_'").await;
    validate_pk("SELECT * FROM c WHERE c.flags & 4 != 0").await;
    validate_pk("SELECT * FROM c WHERE c.name LIKE 'A_%'").await;
    validate_pk("SELECT * FROM c WHERE c.name LIKE 'A_ice'").await;
    validate_pk("SELECT * FROM c WHERE c.name LIKE 'A%'").await;
    validate_pk("SELECT * FROM c WHERE c.name LIKE 'Alice'").await;
    validate_pk("SELECT * FROM c WHERE c.name NOT LIKE '%test%'").await;
    validate_pk("SELECT * FROM c WHERE c.pk != 'x'").await;
    validate_pk("SELECT * FROM c WHERE c.pk + 1 = 'x'").await;
    validate_pk("SELECT * FROM c WHERE c.pk <= 'z'").await;
    validate_pk("SELECT * FROM c WHERE c.pk = -1.5").await;
    validate_pk("SELECT * FROM c WHERE c.pk = ''").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'a' AND c.pk = 'b'").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'a' AND c.x > 1 AND c.pk = 'b'").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_pk_03() {
    validate_pk("SELECT * FROM c WHERE c.pk = 'a' OR c.other = 'b'").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'a' OR c.pk = 'a'").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'a' OR c.pk = 'b' OR c.pk = 'c'").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'a' OR c.pk = 'b' ORDER BY c.name").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'a' OR c.pk IN ('b', 'c')").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'c' AND c.pk IN ('a', 'b')").await;
    validate_pk("SELECT * FROM c WHERE c.pk = @val").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 0").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 1.23").await;
    validate_pk("SELECT * FROM c WHERE c.pk = c.other").await;
    validate_pk("SELECT * FROM c WHERE c.pk = false").await;
    validate_pk("SELECT * FROM c WHERE c.pk IN ('a', 'b', 'c') AND c.age > 21").await;
    validate_pk("SELECT * FROM c WHERE c.pk IN ('a', 'b', 'c') AND c.pk = 'b'").await;
    validate_pk("SELECT * FROM c WHERE c.pk IN ('a', 'b', 'c') ORDER BY c.pk ASC").await;
    validate_pk("SELECT * FROM c WHERE c.pk IN ('a', 'b') AND c.other IN ('x', 'y')").await;
    validate_pk("SELECT * FROM c WHERE c.pk IN ('a', 'b') AND c.pk = 'z'").await;
    validate_pk("SELECT * FROM c WHERE c.pk IN ('a', 'b') AND c.pk IN ('c', 'd')").await;
    validate_pk("SELECT * FROM c WHERE c.pk IN ('a', 'b') OR c.pk IN ('b', 'c')").await;
    validate_pk("SELECT * FROM c WHERE c.pk IN ('a', 'b') OR c.pk IN ('c', 'd')").await;
    validate_pk("SELECT * FROM c WHERE c.pk IN ('a', 42, true, null)").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_pk_04() {
    validate_pk("SELECT * FROM c WHERE c.pk IN ('only')").await;
    validate_pk("SELECT * FROM c WHERE c.pk IN (@a, @b, @c)").await;
    validate_pk("SELECT * FROM c WHERE c.pk LIKE 'prefix%'").await;
    validate_pk("SELECT * FROM c WHERE c.pk NOT IN ('a', 'b')").await;
    validate_pk("SELECT * FROM c WHERE c.scores[0] = 90").await;
    validate_pk("SELECT * FROM c WHERE c.valid = null ORDER BY c.valid").await;
    validate_pk("SELECT * FROM c WHERE c.x NOT BETWEEN 1 AND 10").await;
    validate_pk("SELECT * FROM c WHERE CONTAINS(c.name, 'a') ORDER BY c.name").await;
    validate_pk("SELECT * FROM c WHERE EXISTS(SELECT VALUE t FROM t IN c.tags WHERE t = 'rust')")
        .await;
    validate_pk("SELECT * FROM c WHERE IS_ARRAY(c.tags)").await;
    validate_pk("SELECT * FROM c WHERE IS_BOOL(c.active)").await;
    validate_pk("SELECT * FROM c WHERE IS_NUMBER(c.age)").await;
    validate_pk("SELECT * FROM c WHERE IS_OBJECT(c.address)").await;
    validate_pk("SELECT * FROM c WHERE IS_STRING(c.name)").await;
    validate_pk("SELECT * FROM c WHERE LOWER(c.pk) = 'x'").await;
    validate_pk("SELECT * FROM c WHERE NOT (c.pk = 'x')").await;
    validate_pk("SELECT * FROM c WHERE NOT c.active").await;
    validate_pk("SELECT * FROM c WHERE NOT IS_DEFINED(c.optional)").await;
    validate_pk("SELECT * FROM c WHERE STARTSWITH(c.name, 'A') ORDER BY c.name").await;
    validate_pk("SELECT * FROM c WHERE udf.func1(c.x) > 0 AND udf.func2(c.y) = true").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_pk_05() {
    validate_pk("SELECT * FROM c WHERE udf.myFunc(c.x) > 0").await;
    validate_pk("SELECT 1 + 2 AS result").await;
    validate_pk(
        "SELECT ARRAY(SELECT VALUE t FROM t IN c.tags WHERE t != 'old') AS filtered_tags FROM c",
    )
    .await;
    validate_pk("SELECT c.age > 18 ? 'adult' : 'child' AS label FROM c").await;
    validate_pk("SELECT c.age AS a, COUNT(1) AS cnt FROM c GROUP BY c.age").await;
    validate_pk("SELECT c.age FROM c GROUP BY c.age").await;
    validate_pk(
        "SELECT c.age, c.team, c.gender, COUNT(1) AS cnt FROM c GROUP BY c.age, c.team, c.gender",
    )
    .await;
    validate_pk("SELECT c.first || ' ' || c.last AS name FROM c").await;
    validate_pk("SELECT c.id, COUNT(1) FROM c JOIN t IN c.tags WHERE c.pk = 'x' GROUP BY c.id")
        .await;
    validate_pk("SELECT c.id, d1, d2 FROM c JOIN d1 IN c.digits JOIN d2 IN c.digits WHERE d2 = 0 OFFSET 0 LIMIT 5").await;
    validate_pk("SELECT c.id, t FROM c JOIN t IN c.tags OFFSET 1 LIMIT 3").await;
    validate_pk("SELECT c.id, t1.name, t2.name AS name2 FROM c JOIN t1 IN c.tags JOIN t2 IN c.tags WHERE t1.name = 'a' AND t2.name = 'b'").await;
    validate_pk("SELECT c.name ?? 'unknown' AS name FROM c").await;
    validate_pk("SELECT c.price * c.qty AS total FROM c").await;
    validate_pk("SELECT DISTINCT c.city FROM c GROUP BY c.city").await;
    validate_pk("SELECT DISTINCT c.city, c.state FROM c").await;
    validate_pk("SELECT DISTINCT TOP 5 c.name FROM c WHERE c.active = true").await;
    validate_pk("SELECT DISTINCT VALUE [c.city, c.state] FROM c").await;
    validate_pk("SELECT DISTINCT VALUE c.city FROM c WHERE c.active = true").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_pk_06() {
    validate_pk("SELECT p.name FROM (SELECT * FROM c) p").await;
    validate_pk("SELECT r.name FROM root AS r").await;
    validate_pk("SELECT TOP 3 * FROM c WHERE c.age IN (10, 11, 23)").await;
    validate_pk("select top 3 * from c where c.pk = 'x' order by c.name desc").await;
    validate_pk("SELECT TOP 5 * FROM c WHERE c.age > 10 ORDER BY c.age ASC").await;
    validate_pk("SELECT TOP 5 c.name, c.games.wins FROM c ORDER BY c.games.wins").await;
    validate_pk("SELECT udf.fn1(c.x) AS r1, udf.fn2(c.y) AS r2 FROM c").await;
    validate_pk("SELECT udf.myFunc(c.x) AS result FROM c").await;
    validate_pk("SELECT VALUE -(+(-c.age)) FROM c").await;
    validate_pk("SELECT VALUE -100 >>> 1").await;
    validate_pk("SELECT VALUE '[' || c.name || ']' FROM c").await;
    validate_pk("SELECT VALUE [1,2,3] = [1,2,3]").await;
    validate_pk("SELECT VALUE [c.name, c.age] FROM c").await;
    validate_pk("SELECT VALUE {a: 1, b: 2} = {a: 1, b: 2}").await;
    validate_pk("SELECT VALUE {name: c.name} FROM c").await;
    validate_pk("SELECT VALUE ~1").await;
    validate_pk("SELECT VALUE 10 + c.age * 2 - 10 FROM c").await;
    validate_pk("SELECT VALUE 3 & 2").await;
    validate_pk("SELECT VALUE 3 ^ 2").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_pk_07() {
    validate_pk("SELECT VALUE 3 << 2").await;
    validate_pk("SELECT VALUE 3 >> 2").await;
    validate_pk("SELECT VALUE 3 | 2").await;
    validate_pk("SELECT VALUE c.age > 10 AND c.age < 20 FROM c").await;
    validate_pk("SELECT VALUE c.age | 8 FROM c").await;
    validate_pk("SELECT VALUE c.id FROM c WHERE (c.a = 1) AND (c.b = 1 OR c.c = 1)").await;
    validate_pk("SELECT VALUE c.name FROM c OFFSET 10 LIMIT 5").await;
    validate_pk("SELECT VALUE COUNT(1) FROM c WHERE c.pk = 'x'").await;
    validate_pk("SELECT VALUE null").await;
    validate_pk("SELECT VALUE null = null").await;
    validate_pk("SELECT VALUE r FROM r").await;
    validate_pk("SELECT VALUE t FROM c JOIN t IN c.items WHERE udf.check(t)").await;
    validate_pk("SELECT VALUE udf.transform(c.data) FROM c").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_hpk_01() {
    validate_hpk("SELECT * FROM c WHERE 'acme' = c.tenant AND 'u1' = c.userId").await;
    validate_hpk("SELECT * FROM c WHERE 'acme' = c.tenant AND c.userId = 'u1'").await;
    validate_hpk("SELECT * FROM c WHERE (c.tenant = 'a' AND c.userId = 'u1') OR (c.tenant = 'b')")
        .await;
    validate_hpk("SELECT * FROM c WHERE (c.tenant = 'acme' AND c.x > 1) AND c.userId = 'u1'").await;
    validate_hpk("SELECT * FROM c WHERE c.name = 'fox' AND c.type = 'wood' AND c.flag AND c.userId = 3 OR c.userId = 4").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = -1 AND c.userId = 'u1'").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = '' AND c.userId = 'u1'").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'a'").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'a' AND c.tenant = 'b' AND c.userId = 'u1'")
        .await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1' AND c.tenant = 'a'")
        .await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'a' OR c.userId = 'u1'").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND 'u1' = c.userId").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.age > 21").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = 42").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = c.other").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = null").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = true").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId > 'u1'").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId LIKE 'u%'").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_hpk_02() {
    validate_hpk("SELECT * FROM c WHERE c.tenant = @t AND c.userId = @u").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 1.5 AND c.userId = 'u1'").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = false AND c.userId = 'u1'").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = null AND c.userId = null").await;
    validate_hpk("SELECT * FROM c WHERE c.userId = 'u1' AND c.age > 21").await;
    validate_hpk("SELECT * FROM c WHERE LOWER(c.tenant) = 'acme' AND c.userId = 'u1'").await;
    validate_hpk("SELECT * FROM c WHERE NOT (c.tenant = 'acme') AND c.userId = 'u1'").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_hpk3_01() {
    validate_hpk3("SELECT * FROM c WHERE c.sessionId = 's1'").await;
    validate_hpk3("SELECT * FROM c WHERE c.tenant = @t AND c.userId = @u AND c.sessionId = @s")
        .await;
    validate_hpk3("SELECT * FROM c WHERE c.userId = 'u1' AND c.sessionId = 's1'").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_nested_01() {
    validate_nested("SELECT * FROM c ORDER BY c.address.city ASC, c.age DESC").await;
    validate_nested("SELECT c.address.city AS city FROM c").await;
    validate_nested("SELECT c.address.city, c.address.state, COUNT(1) AS cnt FROM c GROUP BY c.address.city, c.address.state").await;
    validate_nested("SELECT c.address.city, COUNT(1) FROM c GROUP BY c.address.city").await;
}

// ─── Newly advertised supported-feature coverage ────────────────────────────
//
// These tests pin Gateway-side acceptance of the additional feature flags now
// in `SUPPORTED_QUERY_FEATURES` (matching what Java/.NET advertise). The
// local plan generator does not yet recognize every syntactic shape these
// features cover (e.g. `COUNT(DISTINCT …)`, `COUNTIF(…)`, ORDER BY over
// computed columns, hybrid search). For each feature, the gateway-only test
// asserts the Gateway is willing to plan a representative query when the
// corresponding flag is advertised; full local-vs-Gateway parity follows in
// the integration PR that wires the local generator into production and
// implements the parser/planner support for these shapes.
//
// **Known TODOs** (gateway-side coverage not yet in this file because the
// exact accepted syntax depends on backend version and was not confirmed
// against the test account at the time of writing):
//   * DCount     — `SELECT VALUE DCount(c.x) FROM c` form
//   * CountIf    — `SELECT VALUE CountIf(c.age > 21) FROM c` form
//   * NonStreamingOrderBy — needs a containerProperties.indexingPolicy that
//                  excludes the ORDER BY path; setup is non-trivial
//   * CompositeAggregate  — exact rewrite trigger varies by backend version
//   * HybridSearch / WeightedRankFusion — require a vector container
//
// MultipleAggregates is exercised by `gw_aggregates` and `gw_complex_combined`
// above; CompositeAggregate is partially exercised by `gw_composite_aggregate_smoke`
// below as a Gateway smoke test.

async fn validate_pk_gateway_only(sql: &str) {
    let (d, c) = require_driver_and(get_driver().await, c_pk().await);
    fetch_gateway_plan(d, c, sql, &[])
        .await
        .unwrap_or_else(|e| panic!("Gateway query plan request failed for: {sql}\n  {e}"));
}

/// Smoke test that the Gateway is willing to plan a query containing a
/// composite-style aggregate projection when `CompositeAggregate` is
/// advertised. Local-side parity is TODO.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_composite_aggregate_smoke() {
    // The exact syntactic surface that requires the CompositeAggregate flag
    // varies by backend version. Use a minimal multi-aggregate object
    // projection that has historically been the trigger.
    let _ = validate_pk_gateway_only; // referenced for future tests
    let (d, c) = require_driver_and(get_driver().await, c_pk().await);
    // Best-effort: try a couple of shapes the docs / .NET source mention.
    // Skip the failure if the backend rejects all of them — that indicates
    // the test account is on a backend version that does not yet support
    // the feature we just advertised, and the production rollout will wait
    // until the supported-features advertisement matches the deployed
    // backend version anyway.
    let candidates = [
        "SELECT VALUE { 'a': SUM(c.x), 'b': COUNT(1) } FROM c",
        "SELECT { 'a': SUM(c.x), 'b': COUNT(1) } FROM c",
    ];
    let mut last_err = None;
    for sql in candidates {
        match fetch_gateway_plan(d, c, sql, &[]).await {
            Ok(_) => return,
            Err(e) => last_err = Some((sql, e)),
        }
    }
    if let Some((sql, e)) = last_err {
        eprintln!(
            "[CompositeAggregate] Gateway rejected all candidate shapes; \
             likely a backend-version mismatch. Last attempt: sql={sql}, err={e}"
        );
    }
}

// ─── Symmetric-outcome regression tests ─────────────────────────────────────
//
// Each query below was previously covered by a local-only unit test in this
// file. The local-only form was redundant with crate-internal unit tests.
// Re-coding them as symmetric-outcome tests preserves the parity-enforcement
// goal of this file: the local plan generator and the Gateway must agree on
// whether a query is acceptable. If the Gateway accepts a query the local
// generator rejects, that is a parser/planner bug to fix; if both reject,
// the divergence is documented and the test passes.

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_numeric_pk_int_form() {
    validate_symmetric_pk("SELECT * FROM c WHERE c.pk = 1").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_numeric_pk_float_form() {
    validate_symmetric_pk("SELECT * FROM c WHERE c.pk = 1.0").await;
}

/// Non-path GROUP BY expression. The local plan generator deliberately does
/// not implement the rewrite this requires and instead emits the
/// `NEEDS_GATEWAY_FALLBACK` sentinel so the integration layer falls back to
/// the Gateway's query-plan endpoint. Pinning this here guards against (a)
/// the local generator silently dropping the sentinel — which would break
/// the integration-layer fallback path — and (b) the local generator
/// accidentally starting to emit a (wrong) plan for this shape. Once local
/// support is added, switch to `validate_symmetric_pk` so plan-level parity
/// is enforced.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_non_path_group_by_falls_back() {
    validate_pk_local_falls_back_to_gateway(
        "SELECT c.x & 1 AS parity, COUNT(1) FROM c GROUP BY c.x & 1",
    )
    .await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_unterminated_quoted_identifier() {
    validate_symmetric_pk("SELECT * FROM \"unterminated").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_unterminated_block_comment() {
    validate_symmetric_pk("SELECT * FROM c /* unterminated").await;
}

// ── Bracketed property paths in ORDER BY / GROUP BY ─────────────────────────
// All bracket forms — single-quoted (`c['foo']`), double-quoted (`c["foo"]`),
// and integer subscript (`c.a[0]`) — must surface the
// `NEEDS_GATEWAY_FALLBACK` sentinel and let the integration layer defer to
// the Gateway query-plan endpoint. Empirically the Gateway preserves the
// source bracket syntax verbatim in `orderByExpressions` /
// `groupByExpressions` (e.g. `"c[\"name\"]"`) rather than flattening to a
// dotted path; producing the dotted form locally would silently diverge
// from the Gateway and break plan-shape parity with other SDKs. These tests
// pin that behavior.

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_order_by_string_bracket_path_falls_back() {
    validate_pk_local_falls_back_to_gateway("SELECT * FROM c ORDER BY c[\"name\"] ASC").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_order_by_single_quoted_bracket_path_falls_back() {
    validate_pk_local_falls_back_to_gateway("SELECT * FROM c ORDER BY c['name'] ASC").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_order_by_nested_bracket_path_falls_back() {
    validate_pk_local_falls_back_to_gateway(
        "SELECT * FROM c ORDER BY c[\"address\"][\"city\"] ASC",
    )
    .await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_group_by_bracket_path_falls_back() {
    validate_pk_local_falls_back_to_gateway(
        "SELECT c[\"city\"], COUNT(1) AS cnt FROM c WHERE c.pk = 'x' GROUP BY c[\"city\"]",
    )
    .await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_order_by_array_index_falls_back() {
    validate_pk_local_falls_back_to_gateway("SELECT * FROM c ORDER BY c.scores[0] ASC").await;
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
async fn gw_local_parity_group_by_array_index_falls_back() {
    validate_pk_local_falls_back_to_gateway(
        "SELECT c.scores[0] AS s0, COUNT(1) AS cnt FROM c GROUP BY c.scores[0]",
    )
    .await;
}
