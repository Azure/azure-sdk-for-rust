// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(feature = "__internal_native_query_plan")]
use crate::models::AccountEndpoint;
use crate::{
    driver::{
        dataflow::query_plan::{QueryPlan, RawQueryPlan},
        pipeline::operation_pipeline::OperationOverrides,
    },
    models::{ContainerReference, CosmosOperation},
    options::{OperationOptions, QueryPlanMode},
};

use super::CosmosDriver;

/// Result of resolving a query plan from any provider.
pub(super) enum ResolvedQueryPlan {
    /// A plan ready for topology/pipeline construction.
    Plan(Box<QueryPlan>),
    /// Contradictory filters — provably empty result set.
    Empty,
}

impl From<crate::query::local_plan_adapter::ProviderResolution> for ResolvedQueryPlan {
    fn from(resolution: crate::query::local_plan_adapter::ProviderResolution) -> Self {
        match resolution {
            crate::query::local_plan_adapter::ProviderResolution::Plan(plan) => Self::Plan(plan),
            crate::query::local_plan_adapter::ProviderResolution::Empty => Self::Empty,
        }
    }
}

/// Returns an empty resolution when local planning proves topology is unnecessary.
pub(super) fn try_resolve_without_topology(
    driver: &CosmosDriver,
    container: &ContainerReference,
    operation: &CosmosOperation,
    options: &OperationOptions,
) -> Option<ResolvedQueryPlan> {
    if driver.effective_query_plan_mode(options) == QueryPlanMode::GatewayOnly {
        return None;
    }

    matches!(
        crate::query::local_plan_adapter::try_local_plan(
            operation.body(),
            container.partition_key_definition(),
        ),
        Ok(crate::query::local_plan_adapter::ProviderResolution::Empty)
    )
    .then_some(ResolvedQueryPlan::Empty)
}

/// Resolves a query plan through the enabled providers in precedence order.
pub(super) async fn resolve_query_plan(
    driver: &CosmosDriver,
    container: &ContainerReference,
    operation: &CosmosOperation,
    options: &OperationOptions,
) -> crate::error::Result<ResolvedQueryPlan> {
    let mode = driver.effective_query_plan_mode(options);
    if mode != QueryPlanMode::GatewayOnly {
        if let Some(plan) =
            try_plan_query_using_native_planner(driver, container, operation, mode).await
        {
            return Ok(ResolvedQueryPlan::Plan(Box::new(plan)));
        }
        if let Some(resolution) = try_plan_query_using_local_planner(container, operation, mode) {
            return Ok(resolution);
        }
    }

    let plan = Box::pin(gateway_query_plan(driver, container, operation, options)).await?;
    tracing::debug!(provider = "gateway", ?mode, "using Gateway query plan");
    Ok(ResolvedQueryPlan::Plan(Box::new(plan)))
}

#[cfg(feature = "__internal_native_query_plan")]
async fn try_plan_query_using_native_planner(
    driver: &CosmosDriver,
    container: &ContainerReference,
    operation: &CosmosOperation,
    mode: QueryPlanMode,
) -> Option<QueryPlan> {
    let account_endpoint = AccountEndpoint::from(operation.resource_reference().account());
    let query_engine_config = driver
        .runtime
        .account_metadata_cache()
        .get(&account_endpoint)
        .await
        .map(|properties| properties.query_engine_configuration.clone())
        .unwrap_or_default();

    let result = operation
        .body()
        .and_then(|body| std::str::from_utf8(body).ok())
        .map(|query_spec| {
            let partition_key = container.partition_key_definition();
            let paths: Vec<&str> = partition_key
                .paths()
                .iter()
                .map(|path| path.as_ref())
                .collect();
            driver.native_query_plan_provider.get_query_plan(
                query_spec,
                &paths,
                partition_key.kind(),
                &query_engine_config,
            )
        });

    match result {
        Some(Ok(plan)) => {
            tracing::debug!(
                provider = "native_ffi",
                ?mode,
                "using native FFI query plan"
            );
            Some(plan)
        }
        Some(Err(crate::query_plan_native::error::QueryPlanError::LibraryNotAvailable {
            ..
        })) => {
            tracing::debug!(
                provider = "native_ffi",
                fallback_reason = "library_not_available",
                "native query plan library not available, trying local Rust planner"
            );
            None
        }
        Some(Err(error)) => {
            tracing::warn!(
                provider = "native_ffi",
                fallback_reason = error.diagnostic_code(),
                hresult = ?error.hresult(),
                "native query plan generation failed, trying local Rust planner"
            );
            None
        }
        None => {
            tracing::debug!(
                provider = "native_ffi",
                fallback_reason = "no_body",
                "no body for native plan, trying local Rust planner"
            );
            None
        }
    }
}

#[cfg(not(feature = "__internal_native_query_plan"))]
async fn try_plan_query_using_native_planner(
    _driver: &CosmosDriver,
    _container: &ContainerReference,
    _operation: &CosmosOperation,
    _mode: QueryPlanMode,
) -> Option<QueryPlan> {
    None
}

fn try_plan_query_using_local_planner(
    container: &ContainerReference,
    operation: &CosmosOperation,
    mode: QueryPlanMode,
) -> Option<ResolvedQueryPlan> {
    match crate::query::local_plan_adapter::try_local_plan(
        operation.body(),
        container.partition_key_definition(),
    ) {
        Ok(resolution) => {
            tracing::debug!(
                provider = "local_rust",
                outcome = match &resolution {
                    crate::query::local_plan_adapter::ProviderResolution::Plan(_) => "plan",
                    crate::query::local_plan_adapter::ProviderResolution::Empty => "empty",
                },
                ?mode,
                "using local Rust query plan"
            );
            Some(resolution.into())
        }
        Err(reason) => {
            tracing::debug!(
                provider = "local_rust",
                fallback_reason = %reason,
                "local plan ineligible, falling back to gateway"
            );
            None
        }
    }
}

async fn gateway_query_plan(
    driver: &CosmosDriver,
    container: &ContainerReference,
    operation: &CosmosOperation,
    options: &OperationOptions,
) -> crate::error::Result<QueryPlan> {
    let query_plan_operation = CosmosOperation::query_plan(
        container.clone(),
        std::borrow::Cow::Borrowed(crate::query::SUPPORTED_QUERY_FEATURES),
    )
    .with_body(operation.body().unwrap_or_default().to_vec());

    let response = driver
        .execute_operation_direct(
            &query_plan_operation,
            OperationOverrides::default(),
            options,
        )
        .await?;

    let query_plan_body = match response.body() {
        crate::models::ResponseBody::Bytes(body) => body.clone(),
        _ => {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("query plan response did not contain a body")
                .with_source(std::io::Error::other("missing body"))
                .build());
        }
    };
    let raw_plan: RawQueryPlan = serde_json::from_slice(&query_plan_body).map_err(|error| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
            .with_message("failed to parse query plan response")
            .with_source(error)
            .build()
    })?;
    raw_plan.resolve(container.partition_key_definition())
}
