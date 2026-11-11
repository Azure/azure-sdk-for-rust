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
            "A split must have at least two nonempty children.",
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
        self.list_overlapping(None).await
    }

    async fn list_overlapping(&self, range: Option<&FeedRange>) -> Result<Vec<Lease>> {
        let mut query =
            "SELECT * FROM c WHERE STARTSWITH(c.id, @prefix) AND IS_DEFINED(c.range)".to_owned();
        let mut parameters = vec![
            serde_json::json!({"name":"@prefix", "value":format!("{}.lease.", self.namespace)}),
        ];
        if let Some(range) = range {
            query.push_str(" AND c.range.Range.min < @max AND c.range.Range.max > @min");
            parameters
                .push(serde_json::json!({"name":"@min", "value":range.min_inclusive().to_hex()}));
            parameters
                .push(serde_json::json!({"name":"@max", "value":range.max_exclusive().to_hex()}));
        }
        let operation = CosmosOperation::query_items(self.leases.clone(), Some(FeedRange::full()))
            .with_body(serde_json::to_vec(&serde_json::json!({
                "query": query,
                "parameters": parameters
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

    async fn has_replacements(&self, range: &FeedRange) -> Result<bool> {
        let mut replacements = Vec::new();
        for lease in self.list_overlapping(Some(range)).await? {
            let candidate = &lease.base.range;
            if candidate == range {
                continue;
            }
            if candidate.min_inclusive() <= range.min_inclusive()
                && candidate.max_exclusive() >= range.max_exclusive()
            {
                continue;
            }
            if candidate.min_inclusive() < range.min_inclusive()
                || candidate.max_exclusive() > range.max_exclusive()
            {
                return Err(error(
                    StatusCode::Conflict,
                    "Replacement leases cross the missing lease range.",
                ));
            }
            replacements.push(lease);
        }
        if replacements.is_empty() {
            return Ok(false);
        }
        replacements.sort_by(|left, right| {
            left.base
                .range
                .min_inclusive()
                .cmp(right.base.range.min_inclusive())
                .then_with(|| {
                    right
                        .base
                        .range
                        .max_exclusive()
                        .cmp(left.base.range.max_exclusive())
                })
        });
        let mut cursor = range.min_inclusive();
        let mut covering: Option<&Lease> = None;
        for lease in &replacements {
            let candidate = &lease.base.range;
            if candidate.min_inclusive() < cursor {
                // A split parent can coexist with its children until deletion succeeds.
                if covering.is_some_and(|parent| {
                    candidate.max_exclusive() <= parent.base.range.max_exclusive()
                }) {
                    continue;
                }
                return Err(error(StatusCode::Conflict, "Replacement leases overlap."));
            }
            if candidate.min_inclusive() != cursor || lease.base.continuation.is_none() {
                return Err(error(
                    StatusCode::Conflict,
                    "Replacement lease coverage or progress is incomplete.",
                ));
            }
            cursor = candidate.max_exclusive();
            covering = Some(lease);
        }
        if cursor != range.max_exclusive() {
            return Err(error(
                StatusCode::Conflict,
                "Replacement lease coverage is incomplete.",
            ));
        }
        Ok(true)
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
                    if self.has_replacements(&range).await? {
                        continue;
                    }
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
                        last_processed_at: None,
                        processing_deadline: None,
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

    pub(super) async fn split(
        &self,
        parent: &mut Stored<Lease>,
        children: &[FeedRange],
    ) -> Result<()> {
        self.validate(&parent.value)?;
        validate_children(&parent.value.base.range, children)?;
        if parent.value.pending.is_some() {
            return Err(error(
                StatusCode::Conflict,
                "Split requires durable progress.",
            ));
        }
        self.update(parent, Update::Renew).await?;
        let id = parent.value.base.record.id.clone();
        for range in children {
            let id = lease_id(&self.namespace, range);
            let continuation = self.checkpoint(&parent.value, range)?;
            let stored = match self.store().read::<Lease>(&id).await? {
                Some(stored) => stored,
                None => {
                    if self.has_replacements(range).await? {
                        continue;
                    }
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
                        last_processed_at: None,
                        processing_deadline: None,
                        renewed: None,
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
            if stored.value.base.range != *range || stored.value.base.continuation.is_none() {
                return Err(error(
                    StatusCode::Conflict,
                    "Child lease range or continuation does not match its parent.",
                ));
            }
        }
        self.store().delete(&id, parent.etag.clone()).await
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
        self.split(&mut lease, &children).await?;
        tracing::info!(lease = %lease.value.base.record.id, children = children.len(), "Created child leases and deleted the split parent");
        Ok(true)
    }
}
