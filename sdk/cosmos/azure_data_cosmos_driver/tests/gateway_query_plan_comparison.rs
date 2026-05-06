// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore nopk startswith

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

use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
use azure_data_cosmos_driver::models::{
    ContainerReference, CosmosOperation, PartitionKeyDefinition,
};
use azure_data_cosmos_driver::options::OperationOptions;
use azure_data_cosmos_driver::CosmosDriver;

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

    // Headers required for a query-plan request are folded in by
    // `CosmosOperation::query_plan` (see #12). We pre-populate the
    // cross-partition toggle (specific to gateway-comparison tests) and let
    // the factory merge the four mandatory query-plan headers on top.
    let mut custom_headers = std::collections::HashMap::new();
    custom_headers.insert(
        HeaderName::from("x-ms-documentdb-query-enablecrosspartition"),
        HeaderValue::from("True"),
    );
    let caller_options = OperationOptions::default().with_custom_headers(custom_headers);
    let (operation, op_options) = CosmosOperation::query_plan(container.clone(), caller_options);
    let operation = operation.with_body(body);

    let response = driver.execute_operation(operation, op_options).await?;
    let body_bytes = response.into_body();
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
        // F22 direction-check: when the Gateway rewrites the query, the
        // top-level `limit` must be dropped (folded into the per-partition
        // rewrittenQuery). If it ever stops doing so, this carve-out's
        // premise is wrong and the silent skip would mask a real divergence.
        assert!(
            gw_limit.is_none(),
            "[limit carve-out premise broken] sql={sql}\n  Gateway emitted rewrittenQuery but did not drop top-level limit; gw_limit={gw_limit:?}"
        );
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
        // F22 direction-check: when GROUP BY is present, the Gateway folds
        // the ORDER BY into the per-partition rewritten queries and returns
        // empty `orderBy` / `orderByExpressions` arrays. If it ever starts
        // returning a non-empty top-level orderBy in this case, the carve-out
        // is wrong and the silent skip would mask a divergence.
        let gw_ob = gw
            .get("orderBy")
            .and_then(|v| v.as_array())
            .map(|a| a.len())
            .unwrap_or(0);
        let gw_obe = gw
            .get("orderByExpressions")
            .and_then(|v| v.as_array())
            .map(|a| a.len())
            .unwrap_or(0);
        assert!(
            gw_ob == 0 && gw_obe == 0,
            "[orderBy carve-out premise broken] sql={sql}\n  Gateway returned non-empty orderBy/orderByExpressions despite GROUP BY presence; gw_orderBy_len={gw_ob}  gw_orderByExpressions_len={gw_obe}"
        );
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
        // F22 direction-check: when rewrittenQuery is present, the Gateway
        // moves aggregates into `groupByAliasToAggregateType` and returns an
        // empty top-level `aggregates`. If it ever stops doing so, the silent
        // skip above would mask a divergence in aggregate handling.
        let gw_agg_len = gw
            .get("aggregates")
            .and_then(|v| v.as_array())
            .map(|a| a.len())
            .unwrap_or(0);
        assert!(
            gw_agg_len == 0,
            "[aggregates carve-out premise broken] sql={sql}\n  Gateway emitted rewrittenQuery but did not drop top-level aggregates; gw_aggregates_len={gw_agg_len}"
        );
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

// ═══════════════════════════════════════════════════════════════════════════════
// GATEWAY VALIDATION TESTS
//
// Each test validates that the locally-generated query plan matches what the
// Cosmos DB Gateway produces. Tests are skipped when no connection string is set.
// ═══════════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn gw_simple_select() {
    validate_pk("SELECT * FROM c").await;
    validate_pk("SELECT c.name, c.age FROM c").await;
    validate_pk("SELECT VALUE c.name FROM c").await;
    validate_pk("SELECT 1").await;
}

#[tokio::test]
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
async fn gw_pk_and_or_in() {
    validate_pk("SELECT * FROM c WHERE c.pk = 'x' AND c.age > 21").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'a' OR c.pk = 'b'").await;
    validate_pk("SELECT * FROM c WHERE c.pk IN ('a', 'b', 'c')").await;
}

#[tokio::test]
async fn gw_cross_partition() {
    validate_pk("SELECT * FROM c WHERE c.age > 21").await;
    validate_pk("SELECT * FROM c WHERE c.pk > 'x'").await;
    validate_pk("SELECT * FROM c WHERE c.pk BETWEEN 'a' AND 'z'").await;
    validate_pk("SELECT * FROM c WHERE c.pk LIKE 'x%'").await;
}

#[tokio::test]
async fn gw_top() {
    validate_pk("SELECT TOP 10 * FROM c").await;
    validate_pk("SELECT TOP 5 * FROM c WHERE c.pk = 'x'").await;
}

#[tokio::test]
async fn gw_offset_limit() {
    validate_pk("SELECT * FROM c OFFSET 5 LIMIT 20").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'x' OFFSET 0 LIMIT 10").await;
}

#[tokio::test]
async fn gw_distinct() {
    validate_pk("SELECT DISTINCT c.name FROM c").await;
    validate_pk("SELECT DISTINCT c.name FROM c ORDER BY c.name ASC").await;
    validate_pk("SELECT DISTINCT c.name FROM c WHERE c.pk = 'x'").await;
    validate_pk("SELECT DISTINCT VALUE null").await;
    validate_pk("SELECT DISTINCT VALUE 1").await;
    validate_pk("SELECT DISTINCT VALUE 'a'").await;
}

#[tokio::test]
async fn gw_order_by() {
    validate_pk("SELECT * FROM c ORDER BY c.name ASC").await;
    validate_pk("SELECT * FROM c ORDER BY c.age DESC").await;
    validate_pk("SELECT * FROM c ORDER BY c.name").await;
    validate_pk("SELECT * FROM c ORDER BY c.name ASC, c.age DESC").await;
    validate_pk("SELECT * FROM c ORDER BY c.address.city ASC").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'x' ORDER BY c.name DESC").await;
}

#[tokio::test]
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
async fn gw_group_by() {
    validate_pk("SELECT c.city, COUNT(1) FROM c GROUP BY c.city").await;
    validate_pk("SELECT c.city, c.state, COUNT(1) FROM c GROUP BY c.city, c.state").await;
    validate_pk("SELECT c.city, SUM(c.revenue), AVG(c.score) FROM c GROUP BY c.city").await;
    validate_pk("SELECT c.city, COUNT(1) FROM c WHERE c.pk = 'x' GROUP BY c.city").await;
}

#[tokio::test]
async fn gw_join() {
    validate_pk("SELECT c.id, t FROM c JOIN t IN c.tags WHERE c.pk = 'x'").await;
}

#[tokio::test]
async fn gw_subqueries() {
    validate_pk("SELECT * FROM c WHERE EXISTS(SELECT VALUE t FROM t IN c.tags)").await;
    validate_pk("SELECT ARRAY(SELECT t FROM t IN c.tags) FROM c").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'x' AND EXISTS(SELECT VALUE t FROM t IN c.tags WHERE t = 'rust')").await;
}

#[tokio::test]
async fn gw_select_value() {
    validate_pk("SELECT VALUE c.name FROM c WHERE c.pk = 'x'").await;
    validate_pk("SELECT VALUE COUNT(1) FROM c").await;
}

#[tokio::test]
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
async fn gw_functions() {
    validate_pk("SELECT * FROM c WHERE CONTAINS(c.name, 'test')").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'x' AND STARTSWITH(c.name, 'A')").await;
    validate_pk("SELECT * FROM c WHERE IS_DEFINED(c.optional)").await;
}

#[tokio::test]
async fn gw_nested_paths() {
    validate_nested("SELECT * FROM c WHERE c.address.city = 'Seattle'").await;
    validate_nested("SELECT * FROM c WHERE c.address.city = 'Seattle' AND c.age > 21").await;
    validate_nested("SELECT * FROM c WHERE c.address.city IN ('Seattle', 'Portland', 'Austin')")
        .await;
}

#[tokio::test]
async fn gw_hierarchical_pk() {
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = 'u1'").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'acme'").await;
    validate_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = @uid").await;
    validate_hpk("SELECT * FROM c WHERE c.userId = 'u1' AND c.tenant = 'acme'").await;
}

#[tokio::test]
async fn gw_hierarchical_pk3() {
    validate_hpk3(
        "SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1' AND c.sessionId = 's1'",
    )
    .await;
    validate_hpk3("SELECT * FROM c WHERE c.tenant = 'a' AND c.sessionId = 's1'").await;
    validate_hpk3("SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1'").await;
}

#[tokio::test]
async fn gw_complex_with_hpk() {
    validate_hpk(
        "SELECT c.city, COUNT(1) AS cnt FROM c JOIN t IN c.tags WHERE c.tenant = 'acme' AND c.userId = 'u1' GROUP BY c.city ORDER BY c.city ASC",
    ).await;
}

#[tokio::test]
async fn gw_from_alias() {
    validate_pk("SELECT * FROM root AS r WHERE r.pk = 'hello'").await;
    validate_pk("SELECT * FROM root r WHERE r.pk = 'hello'").await;
}

#[tokio::test]
async fn gw_and_intersection() {
    validate_pk("SELECT * FROM c WHERE c.pk = 'a' AND c.pk = 'a'").await;
    validate_pk("SELECT * FROM c WHERE c.pk = 'a' AND c.pk IN ('a', 'b')").await;
    validate_pk("SELECT * FROM c WHERE c.pk IN ('a', 'b') AND c.pk IN ('b', 'c')").await;
}

// ── Gateway 400 tests ────────────────────────────────────────────────────────

#[tokio::test]
async fn gw_400_is_null() {
    validate_pk_expects_400(
        "SELECT * FROM c WHERE c.pk IS NULL",
        "IS NULL not supported by Gateway query plan endpoint",
    )
    .await;
}

#[tokio::test]
async fn gw_400_is_not_null() {
    validate_pk_expects_400(
        "SELECT * FROM c WHERE c.pk IS NOT NULL",
        "IS NOT NULL not supported by Gateway query plan endpoint",
    )
    .await;
}

#[tokio::test]
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
async fn gw_top_parameter_substituted() {
    validate_pk_with_params("SELECT TOP @n * FROM c", &[("@n", serde_json::json!(10))]).await;
    validate_pk_with_params(
        "SELECT TOP @n * FROM c WHERE c.pk = 'x'",
        &[("@n", serde_json::json!(5))],
    )
    .await;
}

#[tokio::test]
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
async fn gw_400_top_parameter_without_value() {
    // Gateway rejects parameterized TOP without a supplied value with HTTP 400.
    validate_pk_expects_400(
        "SELECT TOP @n * FROM c",
        "parameterized TOP requires resolved value for Gateway plan",
    )
    .await;
}

#[tokio::test]
async fn gw_400_offset_limit_parameter_without_value() {
    // Gateway rejects parameterized OFFSET/LIMIT without supplied values with HTTP 400.
    validate_pk_expects_400(
        "SELECT * FROM c OFFSET @off LIMIT @lim",
        "parameterized OFFSET/LIMIT requires resolved values for Gateway plan",
    )
    .await;
}

#[tokio::test]
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

// ─── Numeric PK canonicalization (#3) ────────────────────────────────────────
//
// `c.pk = 1` and `c.pk = 1.0` must hash to the same effective partition key, so
// the locally-extracted PK filter must canonicalize both literal forms to the
// same `Number(f64)` representation. The Gateway's plan response itself does not
// expose the PK filter, so we validate this indirectly: the *queryInfo* plans
// produced for the two queries must be identical.

#[tokio::test]
async fn local_plan_numeric_pk_canonicalization() {
    let int_form = azure_data_cosmos_driver::query::__test_only_generate_query_plan_for_pk_paths(
        "SELECT * FROM c WHERE c.pk = 1",
        &["/pk"],
        &[],
    )
    .unwrap();
    let float_form = azure_data_cosmos_driver::query::__test_only_generate_query_plan_for_pk_paths(
        "SELECT * FROM c WHERE c.pk = 1.0",
        &["/pk"],
        &[],
    )
    .unwrap();
    // The pkFilters block should be identical (canonical numeric form).
    assert_eq!(int_form["pkFilters"], float_form["pkFilters"],);
    // queryInfo must also be identical (both queries are structurally the same).
    assert_eq!(int_form["queryInfo"], float_form["queryInfo"]);
}

// ─── Fail-fast on non-path GROUP BY expressions (#2) ─────────────────────────
//
// The Gateway accepts non-path GROUP BY expressions like `c.x & 1` and rewrites
// the query. The local plan generator cannot reproduce the rewrite faithfully,
// so it now refuses to silently emit a non-comparable plan and instead returns
// an error so the caller can fall back to the Gateway query-plan endpoint.

#[tokio::test]
async fn local_plan_non_path_group_by_errors() {
    let result = azure_data_cosmos_driver::query::__test_only_generate_query_plan_for_pk_paths(
        "SELECT c.x & 1 AS parity, COUNT(1) FROM c GROUP BY c.x & 1",
        &["/pk"],
        &[],
    );
    let err = result.expect_err(
        "non-path GROUP BY expression must surface an error so the caller falls back to Gateway",
    );
    assert!(
        format!("{err}").contains("GROUP BY / ORDER BY"),
        "unexpected error message: {err}"
    );
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
    // `src/query/mod.rs`. `MultipleAggregates` and `CompositeAggregate` are
    // intentionally NOT advertised today (see the doc comment on the constant
    // for the rationale - the local pipeline cannot execute the rewritten
    // queries the Gateway hands back when those features are enabled).
    assert_eq!(
        azure_data_cosmos_driver::query::__TEST_ONLY_SUPPORTED_QUERY_FEATURES,
        "NonValueAggregate,Aggregate,Distinct,MultipleOrderBy,OffsetAndLimit,OrderBy,Top,GroupBy",
    );
}

// (#10) CONCAT plan-parity coverage. The local evaluator implements
// `CONCAT` with strict string-only arguments (any non-string yields
// `Undefined`, matching the gateway). These tests pin the *plan-level*
// shape so the parser/plan generator handles `CONCAT` calls in projection
// and WHERE positions identically to the Gateway. End-to-end value parity
// is covered by inline tests in `query::eval::builtins`.
#[tokio::test]
async fn gw_concat_in_projection() {
    validate_pk("SELECT CONCAT(c.first, c.last) FROM c").await;
}

#[tokio::test]
async fn gw_concat_in_where_clause() {
    validate_pk("SELECT * FROM c WHERE CONCAT(c.first, c.last) = 'AliceSmith'").await;
}

#[tokio::test]
async fn gw_concat_with_literal_argument() {
    validate_pk("SELECT CONCAT(c.name, '@example.com') FROM c").await;
}
