// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
use super::{
    models::Ownership,
    processor::{CheckpointStore, ProcessorStrategy},
};
use crate::models::ConsumerClientDetails;
use azure_core::{error::ErrorKind as AzureErrorKind, Error, Result};
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::{Duration, SystemTime},
};
use tracing::trace;

pub struct LoadBalancerInfo {
    current_ownership: Vec<Ownership>,
    unowned_or_expired: Vec<Ownership>,
    above_max: Vec<Ownership>,
    max_allowed: usize,
    extra_partition_possible: bool,
    //    raw: Vec<Ownership>,
}

pub struct LoadBalancer {
    checkpoint_store: Arc<dyn CheckpointStore>,
    processor_strategy: ProcessorStrategy,
    duration: Duration,
    consumer_client_details: ConsumerClientDetails,
}

impl LoadBalancer {
    pub fn new(
        checkpoint_store: Arc<dyn CheckpointStore>,
        consumer_client_details: ConsumerClientDetails,
        processor_strategy: ProcessorStrategy,
        duration: Duration,
    ) -> Self {
        LoadBalancer {
            checkpoint_store,
            processor_strategy,
            duration,
            consumer_client_details,
        }
    }
    async fn get_available_partitions(&self, partition_ids: &[String]) -> Result<LoadBalancerInfo> {
        trace!(
            "[{}]Get available partitions for {}/{}/{}",
            self.consumer_client_details.client_id,
            self.consumer_client_details.fully_qualified_namespace,
            self.consumer_client_details.eventhub_name,
            self.consumer_client_details.consumer_group
        );
        trace!("Partitions: {}", partition_ids.join(", "));
        let ownerships = self
            .checkpoint_store
            .list_ownerships(
                self.consumer_client_details
                    .fully_qualified_namespace
                    .as_str(),
                self.consumer_client_details.eventhub_name.as_str(),
                self.consumer_client_details.consumer_group.as_str(),
                None,
            )
            .await?;

        trace!("Found {} Ownerships", ownerships.len());

        let mut unowned_or_expired: Vec<Ownership> = Vec::new();
        let mut already_processed: HashSet<String> = HashSet::new();
        let mut grouped_by_owner: HashMap<String, Vec<Ownership>> = HashMap::new();
        grouped_by_owner.insert(self.consumer_client_details.client_id.clone(), Vec::new());

        for ownership in ownerships.iter() {
            if already_processed.contains(&ownership.partition_id) {
                continue;
            }
            already_processed.insert(ownership.partition_id.clone());

            if let Some(last_modified_time) = ownership.last_modified_time {
                if last_modified_time + self.duration < SystemTime::now() {
                    unowned_or_expired.push(ownership.clone());
                    continue;
                }
            }
            if !grouped_by_owner.contains_key(&ownership.owner_id) {
                grouped_by_owner.insert(ownership.owner_id.clone(), Vec::new());
            }

            grouped_by_owner
                .get_mut(&ownership.owner_id)
                .ok_or_else(|| {
                    Error::message(
                        AzureErrorKind::Other,
                        format!(
                            "Ownership not found for partition {} - This should not happen",
                            ownership.partition_id
                        ),
                    )
                })?
                .push(ownership.clone());
        }

        trace!("Number of expired partitions: {}", unowned_or_expired.len());

        for partition_id in partition_ids.iter() {
            if already_processed.contains(partition_id) {
                trace!("Already processed: {}, ignoring.", partition_id);
                continue;
            }

            let new_ownership = Ownership {
                consumer_group: self.consumer_client_details.consumer_group.clone(),
                event_hub_name: self.consumer_client_details.eventhub_name.clone(),
                fully_qualified_namespace: self
                    .consumer_client_details
                    .fully_qualified_namespace
                    .clone(),
                partition_id: partition_id.clone(),
                owner_id: self.consumer_client_details.client_id.clone(),
                ..Default::default()
            };
            trace!("Adding new ownership: {:?}", new_ownership);
            unowned_or_expired.push(new_ownership);
        }
        trace!("Number of unowned partitions: {}", unowned_or_expired.len());

        let mut max_allowed = partition_ids.len() / grouped_by_owner.len();
        let has_remainder = partition_ids.len() % grouped_by_owner.len() > 0;
        if has_remainder {
            trace!(
                "Partitions {} cannot be evenly distributed among {} owners",
                partition_ids.len(),
                grouped_by_owner.len()
            );
            max_allowed += 1;
        }

        let above_max: Vec<Ownership> = grouped_by_owner
            .iter()
            .filter_map(|(owner_id, ownerships)| {
                if owner_id == &self.consumer_client_details.client_id {
                    return None;
                }
                if ownerships.len() > max_allowed {
                    Some(ownerships.clone())
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        Ok(LoadBalancerInfo {
            current_ownership: grouped_by_owner
                .get(self.consumer_client_details.client_id.as_str())
                .unwrap_or(&vec![])
                .clone(),
            unowned_or_expired,
            above_max,
            extra_partition_possible: has_remainder,
            //            raw: ownerships,
            max_allowed,
        })
    }

    fn get_random_ownerships(&self, ownerships: &[Ownership], count: usize) -> Vec<Ownership> {
        let mut random_ownerships = Vec::new();
        let mut ownerships = ownerships.to_vec();

        ownerships.shuffle(&mut thread_rng());

        for (ownerships_count, ownership) in ownerships.iter().enumerate() {
            if ownerships_count >= count {
                break;
            }
            random_ownerships.push(ownership.clone());
        }

        random_ownerships
    }

    fn reset_ownership(&self, ownership: &Ownership) -> Ownership {
        let mut ownership = ownership.clone();
        ownership.owner_id = self.consumer_client_details.client_id.clone();
        ownership
    }

    fn balanced_load_balancer(&self, load_balancer_info: &LoadBalancerInfo) -> Vec<Ownership> {
        let mut ours = Vec::new();
        if !load_balancer_info.unowned_or_expired.is_empty() {
            let index = thread_rng().gen_range(0..load_balancer_info.unowned_or_expired.len());
            let ownership = self.reset_ownership(&load_balancer_info.unowned_or_expired[index]);
            ours.push(ownership);
        }

        if !load_balancer_info.above_max.is_empty() {
            let index = thread_rng().gen_range(0..load_balancer_info.above_max.len());
            let ownership = self.reset_ownership(&load_balancer_info.above_max[index]);
            ours.push(ownership);
        }
        ours
    }

    fn greedy_load_balancer(&self, load_balancer_info: &LoadBalancerInfo) -> Vec<Ownership> {
        let mut ours = load_balancer_info.current_ownership.clone();
        let mut random_ownerships = self.get_random_ownerships(
            &load_balancer_info.unowned_or_expired,
            load_balancer_info.max_allowed - ours.len(),
        );
        ours.append(&mut random_ownerships);
        for ownership in ours.iter_mut() {
            *ownership = self.reset_ownership(ownership);
        }
        ours
    }

    fn partitions_for_ownership(ownerships: &[Ownership]) -> String {
        let mut s = String::from("[");
        let mut first = true;
        for ownership in ownerships.iter() {
            if first {
                first = false;
            } else {
                s.push_str(", ");
            }
            s.push_str(ownership.partition_id.as_str());
        }
        s.push(']');
        s
    }

    pub async fn load_balance(&self, partition_ids: &[String]) -> Result<Vec<Ownership>> {
        let load_balancer_info = self.get_available_partitions(partition_ids).await?;
        let mut claim_more = true;

        if load_balancer_info.current_ownership.len() > load_balancer_info.max_allowed {
            claim_more = false;

            trace!(
                "Owns {} of {} partitions. Max allowed is {}",
                load_balancer_info.current_ownership.len(),
                partition_ids.len(),
                load_balancer_info.max_allowed
            );
        } else if load_balancer_info.extra_partition_possible
            && load_balancer_info.current_ownership.len() == load_balancer_info.max_allowed - 1
        {
            claim_more = !load_balancer_info.unowned_or_expired.is_empty()
                || !load_balancer_info.above_max.is_empty();
            trace!(
                "Unowned/expired: {} Above max: {}. Need to claim more: {}",
                load_balancer_info.unowned_or_expired.len(),
                load_balancer_info.above_max.len(),
                claim_more
            );
        }
        let mut ownerships = load_balancer_info.current_ownership.clone();
        if claim_more {
            match self.processor_strategy {
                ProcessorStrategy::Balanced => {
                    let mut ours = self.balanced_load_balancer(&load_balancer_info);
                    trace!(
                        "[{}]Claiming ownership of {} partitions: {}",
                        self.consumer_client_details.client_id,
                        ours.len(),
                        Self::partitions_for_ownership(&ours)
                    );
                    ownerships.append(&mut ours);
                }
                ProcessorStrategy::Greedy => {
                    let mut ours = self.greedy_load_balancer(&load_balancer_info);
                    trace!(
                        "[{}]Claiming ownership of {} partitions: {}",
                        self.consumer_client_details.client_id,
                        ours.len(),
                        Self::partitions_for_ownership(&ours)
                    );
                    ownerships.append(&mut ours);
                }
            }
        }
        trace!(
            "[{}] Asked for {}",
            self.consumer_client_details.client_id,
            Self::partitions_for_ownership(&ownerships),
        );
        let actual = self
            .checkpoint_store
            .claim_ownership(ownerships, None)
            .await?;
        trace!(
            "[{}] Got {}",
            self.consumer_client_details.client_id,
            Self::partitions_for_ownership(&actual)
        );

        Ok(actual)
    }
}
