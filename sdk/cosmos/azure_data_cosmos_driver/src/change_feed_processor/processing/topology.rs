// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{Context, Lease, Update};
use crate::{
    change_feed_processor::{
        bootstrap::{
            error, lease_id, store::Stored, validate_ranges, ProcessingLease, RangePlan, Record,
        },
        ChangeFeedProcessorMode,
    },
    driver::dataflow::{
        intersect_feed_ranges, planner::project_change_feed_state, validate_exact_coverage,
        PipelineNodeState,
    },
    models::{ContinuationToken, CosmosOperation, FeedRange, ResolvedToken},
    Result,
};
use async_lock::Mutex;
use azure_core::http::StatusCode;
use std::{
    collections::{HashSet, VecDeque},
    sync::Arc,
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
            ChangeFeedProcessorMode::LatestVersion => {
                CosmosOperation::change_feed(self.source.clone(), Some(range.clone()))
            }
            ChangeFeedProcessorMode::AllVersionsAndDeletes => {
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
                ResolvedToken::ClientV1(state) => {
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
            },
        };
        let projected = project_change_feed_state(
            state,
            &lease.base.range,
            range,
            operation.request_headers().full_fidelity_feed,
        )?;
        ContinuationToken::encode_v1(&operation, &projected)
    }

    pub(super) async fn list(&self) -> Result<Vec<Lease>> {
        let plan = self
            .store()
            .read::<RangePlan>(&format!("{}.plan", self.namespace))
            .await?
            .ok_or_else(|| error(StatusCode::NotFound, "Bootstrap range plan is missing."))?;
        if plan.value.record.workload != self.identity || plan.value.record.ttl != -1 {
            return Err(error(
                StatusCode::Conflict,
                "Incompatible bootstrap range plan.",
            ));
        }
        validate_ranges(&plan.value.ranges)?;
        let mut pending: VecDeque<_> = plan
            .value
            .ranges
            .into_iter()
            .map(|range| (range, None))
            .collect();
        let mut visited = HashSet::new();
        let mut leaves = Vec::new();
        while let Some((range, seed)) = pending.pop_front() {
            let id = lease_id(&self.namespace, &range);
            if !visited.insert(id.clone()) {
                return Err(error(
                    StatusCode::Conflict,
                    "Processing lease lineage overlaps.",
                ));
            }
            let stored = match self.store().read::<Lease>(&id).await? {
                Some(stored) => stored,
                None => {
                    let continuation = seed.ok_or_else(|| {
                        error(StatusCode::NotFound, "Initial processing lease is missing.")
                    })?;
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
            if stored.value.base.range != range {
                return Err(error(
                    StatusCode::Conflict,
                    "Processing lease range does not match its lineage.",
                ));
            }
            if let Some(children) = &stored.value.children {
                for child in children {
                    pending
                        .push_back((child.clone(), Some(self.checkpoint(&stored.value, child)?)));
                }
            } else {
                leaves.push(stored.value);
            }
        }
        leaves.sort_by(|left, right| {
            left.base
                .range
                .min_inclusive()
                .cmp(right.base.range.min_inclusive())
        });
        validate_ranges(
            &leaves
                .iter()
                .map(|lease| lease.base.range.clone())
                .collect::<Vec<_>>(),
        )?;
        Ok(leaves)
    }

    pub(super) async fn recover(&self, lease: Arc<Mutex<Stored<Lease>>>) -> Result<bool> {
        let scope = lease.lock().await.value.base.range.clone();
        let topology = self
            .source_driver
            .resolve_all_partition_key_ranges(&self.source, true)
            .await?
            .ok_or_else(|| {
                error(
                    StatusCode::ServiceUnavailable,
                    "Source partition topology is unavailable.",
                )
            })?;
        let mut children = Vec::new();
        for physical in topology.iter() {
            if let Some(range) = intersect_feed_ranges(&scope, &FeedRange::try_from(physical)?) {
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
        tracing::info!(lease = %lease.value.base.record.id, children = children.len(), "Retired split lease; child leases are available for recovery and balancing");
        Ok(true)
    }
}
