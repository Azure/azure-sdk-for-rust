// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    batch_revision, coverage, invalid, BootstrapPhase, BootstrapPlan, BootstrapStore, InitialLease,
    LeaseRecord, StoredMode, INITIALIZATION_GENERATION, MAX_INITIAL_LEASES,
};
use crate::cosmos_lease_store::{CosmosLeaseStore, LeaseIdentity};
use azure_core::http::StatusCode;
use azure_data_cosmos_driver::{
    models::{ContinuationToken, CosmosOperation, ItemReference},
    CosmosError, CosmosStatus, Result,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub(crate) struct TopologyCandidate {
    pub id: String,
    pub range: azure_data_cosmos_driver::models::FeedRange,
    pub checkpoint: ContinuationToken,
    pub owner: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Handoff {
    protocol: u32,
    source: String,
    mode: StoredMode,
    checkpoint: String,
    children: Vec<InitialLease>,
}

#[cfg(test)]
mod tests;

fn conflict(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(CosmosStatus::new(StatusCode::PreconditionFailed))
        .with_message(message)
        .build()
}

fn point(workload: &BootstrapStore, id: &str) -> CosmosLeaseStore {
    let mut store = workload.store.clone();
    store.id = id.to_owned();
    store.item =
        ItemReference::from_name(&store.container, store.partition_key.clone(), id.to_owned());
    store
}

fn revision(record: &LeaseRecord) -> Result<&str> {
    record
        .extra
        .get("_etag")
        .and_then(Value::as_str)
        .filter(|etag| !etag.is_empty())
        .ok_or_else(|| invalid("handoff record has no ETag"))
}

fn next_layout(root: &mut LeaseRecord) -> Result<()> {
    let generation = match root.extra.get("topologyGeneration") {
        None => 0,
        Some(value) => value
            .as_u64()
            .ok_or_else(|| invalid("invalid topology generation"))?,
    };
    root.extra.insert(
        "topologyGeneration".into(),
        json!(generation
            .checked_add(1)
            .ok_or_else(|| invalid("topology generation exhausted"))?),
    );
    Ok(())
}

fn child_id(seed: &InitialLease) -> String {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    format!(
        "cfp.range.{}",
        URL_SAFE_NO_PAD.encode(format!(
            "{}|{}",
            seed.range.min_inclusive(),
            seed.range.max_exclusive()
        ),)
    )
}

fn journal(workload: &BootstrapStore, parent: &LeaseRecord) -> Result<Handoff> {
    let handoff: Handoff = serde_json::from_value(
        parent
            .extra
            .get("handoff")
            .ok_or_else(|| invalid("transitioning parent has no recovery journal"))?
            .clone(),
    )?;
    if handoff.protocol != 1
        || handoff.source != workload.plan.source
        || handoff.mode != workload.plan.mode
        || handoff.checkpoint != parent.checkpoint
    {
        return Err(invalid(
            "handoff journal does not match the workload or confirmed parent progress",
        ));
    }
    BootstrapPlan::new(
        handoff.source.clone(),
        workload.plan.group.clone(),
        workload.plan.mode(),
        workload.plan.start.clone(),
        parent.range.clone(),
        handoff.children.clone(),
    )?;
    Ok(handoff)
}

impl BootstrapStore {
    pub(crate) async fn handoff_fits(&self, child_count: usize) -> Result<bool> {
        Ok(self
            .discover()
            .await?
            .len()
            .checked_add(child_count)
            .is_some_and(|count| count <= MAX_INITIAL_LEASES))
    }
    pub(crate) async fn topology_candidates(&self) -> Result<Vec<TopologyCandidate>> {
        let mut candidates = Vec::new();
        for record in self.discover().await? {
            if super::processing_eligible(&record)? {
                candidates.push(TopologyCandidate {
                    id: record.id,
                    range: record.range,
                    checkpoint: ContinuationToken::from_string(record.checkpoint),
                    owner: record.ownership.owner,
                });
            }
        }
        Ok(candidates)
    }
    /// Stages a quiesced parent and all pending children in one conditional transaction.
    ///
    /// The caller derives children through the driver's supported checkpoint
    /// contract after joining/releasing parent delivery. No pending child is eligible.
    pub(crate) async fn begin_handoff(
        &self,
        parent_id: &str,
        confirmed: &ContinuationToken,
        children: Vec<InitialLease>,
    ) -> Result<()> {
        let root = self.store.observe().await?;
        if self.metadata(&root.record)?.phase != BootstrapPhase::Ready {
            return Err(conflict("handoff requires a Ready workload"));
        }
        let parent = point(self, parent_id).observe().await?;
        if parent.record.checkpoint != confirmed.as_str() {
            return Err(conflict("parent progress changed before handoff"));
        }
        if parent
            .record
            .extra
            .get("leaseState")
            .and_then(Value::as_str)
            == Some("Transitioning")
        {
            let saved = journal(self, &parent.record)?;
            if saved.children != children {
                return Err(invalid("existing handoff has a different child plan"));
            }
            return self.complete_handoff(&parent.record).await;
        }
        if parent
            .record
            .extra
            .get("leaseState")
            .and_then(Value::as_str)
            == Some("Retired")
        {
            if journal(self, &parent.record)?.children != children {
                return Err(invalid("retired parent has a different committed handoff"));
            }
            return Ok(());
        }
        if parent.owner().is_some() || !super::processing_eligible(&parent.record)? {
            return Err(conflict(
                "parent delivery must be joined and released before handoff",
            ));
        }
        if children.len() < 2 {
            return Err(invalid("subdivision requires at least two children"));
        }
        BootstrapPlan::new(
            self.plan.source.clone(),
            self.plan.group.clone(),
            self.plan.mode(),
            self.plan.start.clone(),
            parent.record.range.clone(),
            children.clone(),
        )?;
        let records = self.discover().await?;
        if !coverage(&self.plan, &records, &self.store.id)?.is_empty() {
            return Err(invalid(
                "handoff requires complete existing logical coverage",
            ));
        }
        if records
            .len()
            .checked_add(children.len())
            .is_none_or(|count| count > MAX_INITIAL_LEASES)
        {
            return Err(invalid(
                "handoff exceeds the 32-record workload bound including retained evidence",
            ));
        }
        let handoff = Handoff {
            protocol: 1,
            source: self.plan.source.clone(),
            mode: self.plan.mode,
            checkpoint: confirmed.as_str().to_owned(),
            children,
        };
        let mut staged = parent.record.clone();
        staged.ownership.generation = staged
            .ownership
            .generation
            .checked_add(1)
            .ok_or_else(|| invalid("parent generation exhausted"))?;
        staged
            .extra
            .insert("leaseState".into(), json!("Transitioning"));
        staged
            .extra
            .insert("handoff".into(), serde_json::to_value(&handoff)?);
        let root_revision = root.revision().to_owned();
        let mut layout = root.record;
        next_layout(&mut layout)?;
        let mut operations = vec![
            json!({"operationType":"Replace","id":layout.id,"ifMatch":root_revision,"resourceBody":layout}),
            json!({"operationType":"Replace","id":staged.id,"ifMatch":parent.revision(),"resourceBody":staged}),
        ];
        for seed in &handoff.children {
            let id = child_id(seed);
            if records.iter().any(|record| record.id == id) {
                return Err(invalid(
                    "handoff child identity already exists outside this transition",
                ));
            }
            let child = LeaseRecord {
                id,
                version: 1,
                ownership: LeaseIdentity {
                    owner: None,
                    generation: 0,
                },
                range: seed.range.clone(),
                checkpoint: seed.checkpoint.clone(),
                lease_duration_ms: self.store.options.duration_millis(),
                extra: BTreeMap::from([
                    ("workload".into(), json!(self.store.id)),
                    ("recordKind".into(), json!("WorkLease")),
                    (
                        "initializationGeneration".into(),
                        json!(INITIALIZATION_GENERATION),
                    ),
                    ("leaseState".into(), json!("Pending")),
                    ("handoffParent".into(), json!(parent_id)),
                ]),
            };
            operations.push(json!({"operationType":"Create","resourceBody":child}));
        }
        let response = self
            .store
            .execute(
                CosmosOperation::batch(
                    self.store.container.clone(),
                    self.store.partition_key.clone(),
                )
                .with_body(serde_json::to_vec(&operations)?),
            )
            .await?;
        batch_revision(response, operations.len(), 0)?;
        self.complete_handoff(&point(self, parent_id).observe().await?.record)
            .await
    }

    /// Resumes saved transitions without selecting new positions or deleting recovery evidence.
    pub(crate) async fn resume_topology_transitions(&self) -> Result<u32> {
        let records = self.discover().await?;
        let mut completed = 0;
        for parent in records.iter().filter(|record| {
            record.extra.get("leaseState").and_then(Value::as_str) == Some("Transitioning")
        }) {
            self.complete_handoff(parent).await?;
            completed += 1;
        }
        Ok(completed)
    }

    async fn complete_handoff(&self, parent: &LeaseRecord) -> Result<()> {
        let handoff = journal(self, parent)?;
        if parent.ownership.owner.is_some() {
            return Err(invalid(
                "transitioning parent must not retain processing ownership",
            ));
        }
        let root = self.store.observe().await?;
        if self.metadata(&root.record)?.phase != BootstrapPhase::Ready {
            return Err(conflict("workload readiness changed during handoff"));
        }
        let records = self.discover().await?;
        let current = records
            .iter()
            .find(|record| record.id == parent.id)
            .ok_or_else(|| invalid("handoff parent recovery evidence is missing"))?;
        if current.extra.get("leaseState").and_then(Value::as_str) == Some("Retired") {
            journal(self, current)?;
            return Ok(());
        }
        if revision(current)? != revision(parent)? {
            return Err(conflict("handoff parent revision changed"));
        }
        let mut active = Vec::new();
        for seed in &handoff.children {
            let id = child_id(seed);
            let child = records
                .iter()
                .find(|record| record.id == id)
                .ok_or_else(|| invalid("pending child is missing from the saved handoff"))?;
            if child.extra.get("leaseState").and_then(Value::as_str) != Some("Pending")
                || child.extra.get("handoffParent") != Some(&json!(parent.id))
                || child.range != seed.range
                || child.checkpoint != seed.checkpoint
                || child.ownership.owner.is_some()
                || child.ownership.generation != 0
            {
                return Err(invalid(
                    "pending child does not match the saved recovery plan",
                ));
            }
            let mut activated = child.clone();
            activated.extra.insert("leaseState".into(), json!("Active"));
            active.push(activated);
        }
        let mut future: Vec<_> = records
            .iter()
            .filter(|record| {
                record.id != parent.id && !active.iter().any(|child| child.id == record.id)
            })
            .cloned()
            .collect();
        future.extend(active.iter().cloned());
        if !coverage(&self.plan, &future, &self.store.id)?.is_empty() {
            return Err(invalid("handoff would publish incomplete coverage"));
        }
        let root_revision = root.revision().to_owned();
        let mut layout = root.record;
        next_layout(&mut layout)?;
        let mut retired = current.clone();
        retired.extra.insert("leaseState".into(), json!("Retired"));
        let mut operations = vec![
            json!({"operationType":"Replace","id":layout.id,"ifMatch":root_revision,"resourceBody":layout}),
            json!({"operationType":"Replace","id":retired.id,"ifMatch":revision(current)?,"resourceBody":retired}),
        ];
        for child in active {
            operations.push(json!({"operationType":"Replace","id":child.id,"ifMatch":revision(&child)?,"resourceBody":child}));
        }
        let response = self
            .store
            .execute(
                CosmosOperation::batch(
                    self.store.container.clone(),
                    self.store.partition_key.clone(),
                )
                .with_body(serde_json::to_vec(&operations)?),
            )
            .await?;
        batch_revision(response, operations.len(), 0)?;
        Ok(())
    }
}
