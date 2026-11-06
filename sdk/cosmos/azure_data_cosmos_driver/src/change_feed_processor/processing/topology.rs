// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{Context, Lease, Update};
use crate::{
    change_feed_processor::bootstrap::{
        error, lease_id, store::ChangeFeedProcessorBootstrapStoredRecord as Stored,
        validate_ranges, ChangeFeedProcessorBootstrapProcessingLease as ProcessingLease,
        ChangeFeedProcessorBootstrapRangePlan as RangePlan,
        ChangeFeedProcessorBootstrapRecord as Record,
    },
    change_feed_processor::operation_options,
    driver::dataflow::{
        intersect_feed_ranges, planner::project_change_feed_state, split_feed_envelope,
        validate_exact_coverage, PipelineNodeState,
    },
    models::{
        ChangeFeedFactoryMode, ContinuationToken, CosmosOperation, FeedRange, ResolvedToken,
        ResponseBody,
    },
    options::PlanOptions,
    Result,
};
use async_lock::Mutex;
use azure_core::http::StatusCode;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

pub(super) fn validate_children(parent: &FeedRange, children: &[FeedRange]) -> Result<()> {
    if children.len() < 2
        || children
            .iter()
            .any(|child| child.min_inclusive() >= child.max_exclusive())
    {
        return Err(error(
            StatusCode::Conflict,
            "A retired lease must have at least two nonempty children.",
        ));
    }
    validate_exact_coverage(parent, children.iter())
}

impl Context {
    pub(super) fn operation(&self, range: &FeedRange) -> CosmosOperation {
        let operation = match self.identity.mode {
            ChangeFeedFactoryMode::Incremental => {
                CosmosOperation::change_feed(self.source.clone(), Some(range.clone()))
            }
            ChangeFeedFactoryMode::FullFidelity => {
                CosmosOperation::change_feed_all_versions_and_deletes(
                    self.source.clone(),
                    Some(range.clone()),
                )
            }
        };
        operation.with_change_feed_start(self.options.start.clone())
    }

    pub(super) fn checkpoint(&self, lease: &Lease, range: &FeedRange) -> Result<ContinuationToken> {
        let operation = self.operation(&lease.base.range);
        let state = match lease
            .base
            .continuation
            .as_ref()
            .or(self.options.continuation.as_ref())
        {
            Some(token) => match token.resolve()? {
                ResolvedToken::Client(state) => {
                    state.is_valid_for_operation(&operation)?;
                    state.into_root_node_state()
                }
                ResolvedToken::ServerOpaque(_) => {
                    return Err(error(
                        StatusCode::BadRequest,
                        "Processor checkpoints must be composite change feed continuations.",
                    ))
                }
            },
            None => PipelineNodeState::UnorderedMerge {
                active_tokens: Vec::new(),
                start_from: Some(self.options.start.clone()),
                next_poll_min_epk: None,
            },
        };
        let projected = project_change_feed_state(
            state,
            &lease.base.range,
            range,
            operation.request_headers().full_fidelity_feed,
        )?;
        ContinuationToken::encode(&operation, &projected)
    }

    pub(super) async fn list(&self) -> Result<Vec<Lease>> {
        let operation = CosmosOperation::query_items(self.leases.clone(), Some(FeedRange::full()))
            .with_body(serde_json::to_vec(&serde_json::json!({
                "query": "SELECT * FROM c WHERE STARTSWITH(c.id, @prefix) AND IS_DEFINED(c.range) AND (NOT IS_DEFINED(c.children) OR NOT IS_DEFINED(c.split_complete) OR c.split_complete = false)",
                "parameters": [{"name": "@prefix", "value": format!("{}.lease.", self.namespace)}]
            }))?)
            .with_absolute_deadline(Some(Instant::now() + Duration::from_secs(25)));
        let mut plan = self
            .lease_driver
            .plan_operation(
                operation,
                &operation_options(),
                None,
                &PlanOptions::default(),
            )
            .await?;
        let mut leases = Vec::new();
        while let Some(response) = self
            .lease_driver
            .execute_plan(&mut plan, Some(self.leases.clone()), operation_options())
            .await?
        {
            let body = match response.into_body() {
                ResponseBody::Bytes(bytes) => {
                    ResponseBody::from_items(split_feed_envelope(&bytes)?)
                }
                body => body,
            };
            for lease in body.into_items::<Lease>()? {
                self.validate(&lease)?;
                leases.push(lease);
            }
        }
        leases.sort_by(|left, right| {
            left.base
                .range
                .min_inclusive()
                .cmp(right.base.range.min_inclusive())
        });
        Ok(leases)
    }

    pub(super) async fn verify_leases(&self) -> Result<Vec<Lease>> {
        let plan_id = format!("{}.plan", self.namespace);
        let plan = self
            .store()
            .read::<RangePlan>(&plan_id)
            .await?
            .ok_or_else(|| error(StatusCode::NotFound, "Bootstrap range plan is missing."))?;
        if plan.value.record.workload != self.identity
            || plan.value.record.id != plan_id
            || plan.value.record.ttl != -1
        {
            return Err(error(
                StatusCode::Conflict,
                "Incompatible bootstrap range plan.",
            ));
        }
        validate_ranges(&plan.value.ranges)?;
        for range in plan.value.ranges {
            let id = lease_id(&self.namespace, &range);
            let stored = match self.store().read::<Lease>(&id).await? {
                Some(stored) => stored,
                None => {
                    let lease = Lease {
                        base: ProcessingLease {
                            record: Record {
                                id: id.clone(),
                                workload: self.identity.clone(),
                                ttl: -1,
                            },
                            range: range.clone(),
                            owner: None,
                            continuation: None,
                        },
                        incarnation: None,
                        renewed_at: None,
                        children: None,
                        split_complete: false,
                        renewed: None,
                        pending: None,
                    };
                    self.store().create(&id, &lease).await?;
                    self.store().read::<Lease>(&id).await?.ok_or_else(|| {
                        error(
                            StatusCode::ServiceUnavailable,
                            "Created initial lease is not yet readable.",
                        )
                    })?
                }
            };
            self.validate(&stored.value)?;
            if stored.value.base.range != range {
                return Err(error(
                    StatusCode::Conflict,
                    "Processing lease range does not match its initial plan.",
                ));
            }
        }
        self.list().await
    }

    pub(super) async fn ensure_children(&self, parent: &mut Stored<Lease>) -> Result<()> {
        self.validate(&parent.value)?;
        let children = parent
            .value
            .children
            .as_ref()
            .ok_or_else(|| error(StatusCode::Conflict, "Processing lease is not retired."))?;
        if parent.value.split_complete {
            return Ok(());
        }
        for range in children {
            let id = lease_id(&self.namespace, range);
            let continuation = self.checkpoint(&parent.value, range)?;
            let stored = match self.store().read::<Lease>(&id).await? {
                Some(stored) => stored,
                None => {
                    let child = Lease {
                        base: ProcessingLease {
                            record: Record {
                                id: id.clone(),
                                workload: self.identity.clone(),
                                ttl: -1,
                            },
                            range: range.clone(),
                            owner: None,
                            continuation: Some(continuation),
                        },
                        incarnation: None,
                        renewed_at: None,
                        children: None,
                        renewed: None,
                        split_complete: false,
                        pending: None,
                    };
                    self.store().create(&id, &child).await?;
                    self.store().read::<Lease>(&id).await?.ok_or_else(|| {
                        error(
                            StatusCode::ServiceUnavailable,
                            "Created child lease is not yet readable.",
                        )
                    })?
                }
            };
            self.validate(&stored.value)?;
            if stored.value.base.range != *range {
                return Err(error(
                    StatusCode::Conflict,
                    "Child lease range does not match its parent.",
                ));
            }
        }
        self.update(parent, Update::CompleteSplit).await
    }

    pub(super) async fn recover(&self, lease: Arc<Mutex<Stored<Lease>>>) -> Result<bool> {
        let scope = lease.lock().await.value.base.range.clone();
        let topology = self
            .source_driver
            .resolve_partition_key_ranges_for_feed_range(
                &self.source,
                &scope,
                true,
                operation_options(),
            )
            .await?;
        let mut children = Vec::new();
        for physical in topology.iter() {
            if let Some(range) = intersect_feed_ranges(&scope, physical) {
                children.push(range);
            }
        }
        children.sort_by(|left, right| left.min_inclusive().cmp(right.min_inclusive()));
        validate_exact_coverage(&scope, children.iter())?;
        if children.len() == 1 {
            return Ok(false);
        }
        let mut lease = lease.lock().await;
        self.update(&mut lease, Update::Split(&children)).await?;
        self.ensure_children(&mut lease).await?;
        tracing::info!(lease = %lease.value.base.record.id, children = children.len(), "Created child leases for the retired split range");
        Ok(true)
    }
}
