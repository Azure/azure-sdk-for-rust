// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    invalid, record_join, BootstrapStore, LeaseBalancer, LeaseControl, LeaseWorker,
    ManagedLeaseState, ManagedProcessorOptions, ProcessorEngine, Shared, Shutdown,
};
use crate::{InitialLease, LeaseReleaseOutcome, LeaseRunOutcome};
use azure_data_cosmos_driver::{models::FeedRange, CosmosError, Result};
use std::{collections::BTreeMap, sync::Arc};
use tokio::task::JoinSet;

struct HandoffAttempt {
    shutdown: Shutdown,
    completed: bool,
}
impl Drop for HandoffAttempt {
    fn drop(&mut self) {
        if !self.completed {
            self.shutdown
                .lock()
                .expect("shutdown report lock poisoned")
                .errors
                .push(std::sync::Arc::new(invalid(
                    "topology write interrupted; saved journal requires reconciliation",
                )));
        }
    }
}

#[allow(clippy::too_many_arguments)] // Borrowed coordinator resources, not a public construction API.
pub(super) async fn reconcile(
    engine: &ProcessorEngine,
    workload: &BootstrapStore,
    balancer: &mut LeaseBalancer,
    sessions: &mut BTreeMap<String, LeaseWorker>,
    tasks: &mut JoinSet<super::ManagedLeaseShutdown>,
    options: &ManagedProcessorOptions,
    control: &LeaseControl,
    shared: &Shared,
    shutdown: &Shutdown,
) -> Result<()> {
    workload.resume_topology_transitions().await?;
    let physical = engine.physical_ranges().await?;
    for candidate in workload.topology_candidates().await? {
        if control.is_lost() {
            return Ok(());
        }
        let mut children = Vec::new();
        for range in &physical {
            if range.overlaps(&candidate.range) {
                let min = range
                    .min_inclusive()
                    .max(candidate.range.min_inclusive())
                    .clone();
                let max = range
                    .max_exclusive()
                    .min(candidate.range.max_exclusive())
                    .clone();
                children.push(FeedRange::new(min, max)?);
            }
        }
        if children.len() < 2 {
            continue;
        }
        if !workload.handoff_fits(children.len()).await? {
            tracing::warn!(lease_id=%candidate.id, "subdivision deferred at the 32-record bound; retaining the logical parent");
            continue;
        }
        if candidate.owner.is_some() && !sessions.contains_key(&candidate.id) {
            continue;
        }
        balancer.exclude_lease(candidate.id.clone())?;
        let mut quiesce_error = None;
        if let Some(parent) = sessions.get(&candidate.id) {
            parent.stop.stop();
            let mut joined_parent = false;
            while let Some(joined) = tasks.join_next().await {
                let mut safe_parent = false;
                if let Ok(result) = &joined {
                    let id = result.id().to_owned();
                    sessions.remove(&id);
                    if id == candidate.id {
                        joined_parent = true;
                        safe_parent = result.outcome() == LeaseRunOutcome::StoppedDrained
                            && matches!(result.release(), LeaseReleaseOutcome::Released)
                            && result.error().is_none();
                        if !safe_parent {
                            quiesce_error = Some(match (result.error(), result.release()) {
                                (Some(error), _) => error.clone(),
                                (_, LeaseReleaseOutcome::Failed(error)) => Arc::new(error.clone()),
                                _ => Arc::new(invalid(
                                    "parent delivery was not cleanly joined and released",
                                )),
                            });
                        }
                    }
                }
                let failed = joined.is_err();
                record_join(shutdown, joined);
                if failed {
                    return Err(invalid("a task failed during topology quiescence"));
                }
                if joined_parent {
                    if !safe_parent {
                        balancer.include_lease(&candidate.id);
                    }
                    break;
                }
            }
            if let Some(error) = quiesce_error {
                record_failure(shared, shutdown, &candidate.id, error);
                continue;
            }
            if !joined_parent {
                return Err(invalid("parent task was not present in the owned task set"));
            }
        }
        let current = workload
            .topology_candidates()
            .await?
            .into_iter()
            .find(|lease| lease.id == candidate.id);
        let Some(current) = current else {
            shutdown
                .lock()
                .expect("shutdown report lock poisoned")
                .clear_topology_error(&candidate.id);
            balancer.include_lease(&candidate.id);
            continue;
        };
        if current.owner.is_some() {
            balancer.include_lease(&candidate.id);
            continue;
        }
        let tokens = match engine.driver().derive_change_feed_checkpoints(
            engine.container(),
            &current.checkpoint,
            &current.range,
            &children,
            options.mode == crate::ChangeFeedMode::AllVersionsAndDeletes,
        ) {
            Ok(tokens) => tokens,
            Err(error) => {
                record_failure(shared, shutdown, &candidate.id, Arc::new(error));
                balancer.include_lease(&candidate.id);
                continue;
            }
        };
        let seeds = children
            .into_iter()
            .zip(tokens)
            .map(|(range, token)| InitialLease::new(range, token))
            .collect::<Result<Vec<_>>>()?;
        let mut attempt = HandoffAttempt {
            shutdown: shutdown.clone(),
            completed: false,
        };
        let result = workload
            .begin_handoff(&current.id, &current.checkpoint, seeds)
            .await;
        attempt.completed = true;
        match result {
            Ok(()) => {
                shutdown
                    .lock()
                    .expect("shutdown report lock poisoned")
                    .clear_topology_error(&current.id);
                if let Some(snapshot) = shared
                    .lock()
                    .expect("managed state lock poisoned")
                    .leases
                    .get_mut(&current.id)
                {
                    snapshot.state = ManagedLeaseState::OwnershipLost;
                }
            }
            Err(error)
                if error.status().status_code()
                    == azure_core::http::StatusCode::PreconditionFailed =>
            {
                tracing::info!(lease_id=%current.id, error=%error, "handoff lost an exact-revision race; retrying after fresh inventory");
            }
            Err(error) => {
                record_failure(shared, shutdown, &candidate.id, Arc::new(error));
            }
        }
        balancer.include_lease(&candidate.id);
    }
    Ok(())
}

fn record_failure(shared: &Shared, shutdown: &Shutdown, id: &str, error: Arc<CosmosError>) {
    tracing::warn!(lease_id=%id, error=%error, "durable subdivision failed; retaining the logical parent");
    shutdown
        .lock()
        .expect("shutdown report lock poisoned")
        .record_topology_error(id, error.clone());
    if let Some(snapshot) = shared
        .lock()
        .expect("managed state lock poisoned")
        .leases
        .get_mut(id)
    {
        snapshot.state = ManagedLeaseState::Quarantined;
        snapshot.last_error = Some(error);
    }
}
