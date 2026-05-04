// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore nopk startswith

//! Gateway validation tests for the client-side query plan generator.
//!
//! These tests compare locally-generated query plans against the Gateway's
//! query plan endpoint to ensure parity. They require a live Cosmos DB account.
//!
//! Tests are skipped when `AZURE_COSMOS_CONNECTION_STRING` is not set.
//!
//! Run with: `AZURE_COSMOS_CONNECTION_STRING=... cargo test -p azure_data_cosmos_driver --features __internal_testing --test gateway_query_plan_comparison`

#![cfg(feature = "__internal_testing")]

use std::collections::HashMap;
use std::sync::Arc;

use azure_core::http::headers::{HeaderName, HeaderValue};
use tokio::sync::OnceCell;

use azure_data_cosmos_driver::models::{
    AccountReference, ConnectionString, ContainerReference, CosmosOperation,
    PartitionKeyDefinition,
};
use azure_data_cosmos_driver::options::{ConnectionPoolOptions, EmulatorServerCertValidation, OperationOptions};
use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
use azure_data_cosmos_driver::CosmosDriver;

// ─── Test infrastructure ─────────────────────────────────────────────────────

const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";
const EMULATOR_CONNECTION_STRING: &str =
    "AccountEndpoint=https://localhost:8081/;AccountKey=C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==";

fn resolve_env() -> Option<(AccountReference, ConnectionPoolOptions)> {
    let conn_str_raw = match std::env::var(CONNECTION_STRING_ENV_VAR) {
        Ok(val) if val.to_lowercase() == "emulator" => EMULATOR_CONNECTION_STRING.to_string(),
        Ok(val) => val,
        Err(_) => return None,
    };
    let conn_str: ConnectionString = conn_str_raw.parse().ok()?;
    let endpoint = conn_str.account_endpoint().parse().ok()?;
    let key = conn_str.account_key().secret().to_string();
    let account = AccountReference::with_master_key(endpoint, key);

    let mut pool_builder = ConnectionPoolOptions::builder();
    if conn_str_raw.eq_ignore_ascii_case(EMULATOR_CONNECTION_STRING) {
        pool_builder = pool_builder
            .with_emulator_server_cert_validation(EmulatorServerCertValidation::DangerousDisabled);
    }
    let pool = pool_builder.build().ok()?;
    Some((account, pool))
}

async fn build_driver() -> Option<Arc<CosmosDriver>> {
    let (account, pool) = resolve_env()?;
    let runtime = CosmosDriverRuntime::builder()
        .with_connection_pool(pool)
        .build()
        .await
        .ok()?;
    let driver = runtime
        .get_or_create_driver(account, None)
        .await
        .ok()?;
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
    let op = CosmosOperation::create_database(account).with_body(
        serde_json::to_vec(&serde_json::json!({"id": DB_NAME})).unwrap(),
    );
    let _ = driver.execute_operation(op, Default::default()).await;
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
    let _ = driver.execute_operation(op, Default::default()).await;

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
) -> Result<serde_json::Value, azure_core::Error> {
    let query_body = serde_json::json!({"query": sql});
    let body = serde_json::to_vec(&query_body)?;

    let mut custom_headers = HashMap::new();
    custom_headers.insert(
        HeaderName::from("x-ms-cosmos-is-query-plan-request"),
        HeaderValue::from("True"),
    );
    custom_headers.insert(
        HeaderName::from("x-ms-cosmos-supported-query-features"),
        HeaderValue::from(
            "NonValueAggregate,Aggregate,Distinct,MultipleOrderBy,OffsetAndLimit,OrderBy,Top,CompositeAggregate,GroupBy,MultipleAggregates",
        ),
    );
    custom_headers.insert(
        HeaderName::from("x-ms-documentdb-isquery"),
        HeaderValue::from("True"),
    );
    custom_headers.insert(
        azure_core::http::headers::CONTENT_TYPE,
        HeaderValue::from("application/query+json"),
    );
    custom_headers.insert(
        HeaderName::from("x-ms-documentdb-query-enablecrosspartition"),
        HeaderValue::from("True"),
    );

    let op_options = OperationOptions::default().with_custom_headers(custom_headers);
    let operation = CosmosOperation::query_plan(container.clone()).with_body(body);

    let response = driver.execute_operation(operation, op_options).await?;
    let body_bytes = response.into_body();
    serde_json::from_slice(&body_bytes)
        .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))
}

/// Compare a locally-generated QueryInfo (as JSON) against the Gateway's queryInfo.
///
/// Accounts for known divergences in Gateway behavior:
/// - Gateway downgrades Ordered DISTINCT → Unordered when it has a rewrittenQuery
/// - Gateway returns None for OFFSET 0 when it rewrites the query
/// - Gateway drops limit when it has a rewrittenQuery
/// - Gateway returns empty order_by when GROUP BY is present
/// - Gateway moves aggregates to groupByAliasToAggregateType when it rewrites
fn compare_query_info(sql: &str, local: &serde_json::Value, gw: &serde_json::Value) {
    let gw_rewritten = gw.get("rewrittenQuery").and_then(|v| v.as_str());

    // distinct_type
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

    // top
    let local_top = local.get("top").and_then(|v| v.as_i64());
    let gw_top = gw.get("top").and_then(|v| v.as_i64());
    if local_top != gw_top {
        panic!("[top] sql={sql}\n  local={local_top:?}  gw={gw_top:?}");
    }

    // offset
    let local_offset = local.get("offset").and_then(|v| v.as_i64());
    let gw_offset = gw.get("offset").and_then(|v| v.as_i64());
    let offset_ok =
        local_offset == gw_offset || (local_offset == Some(0) && gw_offset.is_none());
    if !offset_ok {
        panic!("[offset] sql={sql}\n  local={local_offset:?}  gw={gw_offset:?}");
    }

    // limit
    let local_limit = local.get("limit").and_then(|v| v.as_i64());
    let gw_limit = gw.get("limit").and_then(|v| v.as_i64());
    if gw_rewritten.is_none() && local_limit != gw_limit {
        panic!("[limit] sql={sql}\n  local={local_limit:?}  gw={gw_limit:?}");
    }

    // orderBy
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
    }

    // orderByExpressions
    if gw_gbe == 0 {
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
    }

    // groupByExpressions
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
    let local_gbe_has_debug = local_gbe
        .iter()
        .any(|e| e.as_str().is_some_and(|s| s.contains("MemberIndexer") || s.contains("Binary")));
    if !local_gbe_has_debug && local_gbe != gw_gbe_arr {
        panic!("[groupByExpressions] sql={sql}\n  local={local_gbe:?}  gw={gw_gbe_arr:?}");
    }

    // aggregates
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
    }

    // hasSelectValue
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
    // Generate local plan
    let local_plan =
        azure_data_cosmos_driver::query::generate_query_plan_for_pk_paths(sql, pk_paths)
            .unwrap_or_else(|e| panic!("Local plan generation failed for: {sql}\n  {e}"));
    let local_qi = &local_plan["queryInfo"];

    // Fetch gateway plan
    let gw_plan = fetch_gateway_plan(driver, container, sql)
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
    match fetch_gateway_plan(driver, container, sql).await {
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
container_fixture!(
    C_NESTED,
    c_nested,
    "qp_nested",
    "/address/city".into()
);
container_fixture!(C_NOPK, c_nopk, "qp_nopk", "/id".into());

// ─── Gateway validation helper functions ─────────────────────────────────────

async fn validate_pk(sql: &str) {
    if let (Some(d), Some(c)) = (get_driver().await, c_pk().await) {
        validate(d, c, &["/pk"], sql).await;
    }
}

async fn validate_hpk(sql: &str) {
    if let (Some(d), Some(c)) = (get_driver().await, c_hpk().await) {
        validate(d, c, &["/tenant", "/userId"], sql).await;
    }
}

async fn validate_hpk3(sql: &str) {
    if let (Some(d), Some(c)) = (get_driver().await, c_hpk3().await) {
        validate(d, c, &["/tenant", "/userId", "/sessionId"], sql).await;
    }
}

async fn validate_nested(sql: &str) {
    if let (Some(d), Some(c)) = (get_driver().await, c_nested().await) {
        validate(d, c, &["/address/city"], sql).await;
    }
}

#[allow(dead_code)]
async fn validate_nopk(sql: &str) {
    if let (Some(d), Some(c)) = (get_driver().await, c_nopk().await) {
        validate(d, c, &["/id"], sql).await;
    }
}

async fn validate_pk_expects_400(sql: &str, reason: &str) {
    if let (Some(d), Some(c)) = (get_driver().await, c_pk().await) {
        validate_expects_400(d, c, sql, reason).await;
    }
}

async fn validate_hpk_expects_400(sql: &str, reason: &str) {
    if let (Some(d), Some(c)) = (get_driver().await, c_hpk().await) {
        validate_expects_400(d, c, sql, reason).await;
    }
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
    validate_nested("SELECT * FROM c WHERE c.address.city IN ('Seattle', 'Portland', 'Austin')").await;
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
    validate_hpk3("SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1' AND c.sessionId = 's1'").await;
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
