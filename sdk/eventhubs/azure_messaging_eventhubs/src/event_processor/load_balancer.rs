// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
use super::{
    models::Ownership,
    {CheckpointStore, ProcessorStrategy},
};
use crate::models::ConsumerClientDetails;
use azure_core::Result;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    sync::Arc,
    time::{Duration, SystemTime},
};
use tracing::{debug, trace};

/// LoadBalancerInfo contains information about the current ownership of partitions
/// and the partitions that are unowned or expired.
/// It also contains information about the maximum number of partitions that can be owned
/// and whether there are extra partitions that can be claimed.
/// It is used by the LoadBalancer to determine which partitions to claim ownership of.
#[derive(Debug)]
pub struct LoadBalancerInfo {
    current_ownership: Vec<Ownership>,
    unowned_or_expired: Vec<Ownership>,
    claim_more_partitions: bool,
    above_max: Vec<Ownership>,
    max_allowed: usize,
    //    raw: Vec<Ownership>,
}

/// LoadBalancer is responsible for managing the ownership of partitions in an Event Hub.
/// It uses a checkpoint store to keep track of the ownership of partitions.
/// It provides methods to claim ownership of partitions and to load balance the ownership
/// among multiple consumers.
pub struct LoadBalancer {
    checkpoint_store: Arc<dyn CheckpointStore>,
    processor_strategy: ProcessorStrategy,
    duration: Duration,
    consumer_client_details: ConsumerClientDetails,
}

impl LoadBalancer {
    /// Creates a new LoadBalancer instance.
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
            )
            .await?;

        trace!("Found {} Ownerships", ownerships.len());

        let mut unowned_or_expired: Vec<Ownership> = Vec::new();
        let mut already_added: HashSet<String> = HashSet::new();
        let mut grouped_by_owner: HashMap<String, Vec<Ownership>> = HashMap::new();
        grouped_by_owner.insert(self.consumer_client_details.client_id.clone(), Vec::new());

        for ownership in ownerships.iter() {
            already_added.insert(ownership.partition_id.clone());

            if let Some(last_modified_time) = ownership.last_modified_time {
                if last_modified_time + self.duration < SystemTime::now() {
                    unowned_or_expired.push(ownership.clone());
                    continue;
                }
            }
            // If the owner ID is empty, it means the partition is unowned.
            if ownership.owner_id.is_empty() {
                unowned_or_expired.push(ownership.clone());
                continue;
            }

            // If we've not yet seen this partition, add it to the list of unowned or expired with an empty vector.
            if !grouped_by_owner.contains_key(&ownership.owner_id) {
                grouped_by_owner.insert(ownership.owner_id.clone(), Vec::new());
            }

            if let Some(val) = grouped_by_owner.get_mut(&ownership.owner_id) {
                val.push(ownership.clone());
            };
        }

        let expired_count = unowned_or_expired.len();
        debug!("Number of expired partitions: {}", expired_count);

        // Add in any partitions that we haven't seen yet.
        for partition_id in partition_ids {
            if already_added.contains(partition_id) {
                trace!("Already processed: {}, ignoring.", partition_id);
                continue;
            }

            trace!("Adding new ownership for partition {}", partition_id);
            unowned_or_expired.push(Ownership {
                consumer_group: self.consumer_client_details.consumer_group.clone(),
                event_hub_name: self.consumer_client_details.eventhub_name.clone(),
                fully_qualified_namespace: self
                    .consumer_client_details
                    .fully_qualified_namespace
                    .clone(),
                partition_id: partition_id.clone(),
                owner_id: self.consumer_client_details.client_id.clone(),
                ..Default::default()
            });
        }
        debug!("Number of unowned partitions: {}", unowned_or_expired.len());

        let minimum_required = partition_ids.len() / grouped_by_owner.len();
        let mut maximum_allowed = minimum_required;
        let allow_extra_partition = partition_ids.len() % grouped_by_owner.len() > 0;
        if allow_extra_partition
            && grouped_by_owner[&self.consumer_client_details.client_id].len() >= minimum_required
        {
            maximum_allowed += 1;
        }

        let mut above_maximum: Vec<Ownership> = Vec::new();
        for (id, ownerships) in grouped_by_owner.iter() {
            if id == &self.consumer_client_details.client_id {
                continue;
            }

            if ownerships.len() > maximum_allowed {
                above_maximum.append(ownerships.clone().as_mut());
            }
        }

        let mut claim_more_partitions = true;
        let current: &Vec<Ownership> =
            grouped_by_owner[&self.consumer_client_details.client_id].as_ref();

        if current.len() >= maximum_allowed {
            claim_more_partitions = false;
        } else if allow_extra_partition && current.len() == maximum_allowed - 1 {
            claim_more_partitions = !unowned_or_expired.is_empty() || !above_maximum.is_empty();
        }

        debug!("[{}] claimMorePartitions: {}, owners: {}, current: {}, unowned: {}, expired: {}, above: {}",
            self.consumer_client_details.client_id,
            claim_more_partitions,
            grouped_by_owner.len(),
            current.len(),
            unowned_or_expired.len() - expired_count,
            expired_count,
            above_maximum.len());

        let rv = LoadBalancerInfo {
            current_ownership: grouped_by_owner
                .get(self.consumer_client_details.client_id.as_str())
                .unwrap_or(&vec![])
                .clone(),
            unowned_or_expired,
            above_max: above_maximum,
            max_allowed: maximum_allowed,
            claim_more_partitions,
        };
        debug!("Get available partitions returns: {:?}", rv);
        Ok(rv)
    }

    fn get_random_ownerships(&self, ownerships: &[Ownership], count: usize) -> Vec<Ownership> {
        let limit = min(count, ownerships.len());
        if limit == 0 {
            return Vec::new();
        }

        let mut ownerships = ownerships.to_vec();

        ownerships.shuffle(&mut thread_rng());
        ownerships
    }

    fn reset_ownership(&self, ownership: &mut Ownership) {
        ownership.owner_id = self.consumer_client_details.client_id.clone();
    }

    fn balanced_load_balancer(&self, load_balancer_info: &LoadBalancerInfo) -> Option<Ownership> {
        if !load_balancer_info.unowned_or_expired.is_empty() {
            let index = thread_rng().gen_range(0..load_balancer_info.unowned_or_expired.len());
            let mut ownership = load_balancer_info.unowned_or_expired[index].clone();
            self.reset_ownership(&mut ownership);
            return Some(ownership);
        }

        if !load_balancer_info.above_max.is_empty() {
            let index = thread_rng().gen_range(0..load_balancer_info.above_max.len());
            let mut ownership = load_balancer_info.above_max[index].clone();
            self.reset_ownership(&mut ownership);
            return Some(ownership);
        }
        None
    }

    fn greedy_load_balancer(&self, load_balancer_info: &LoadBalancerInfo) -> Vec<Ownership> {
        let mut ours = load_balancer_info.current_ownership.clone();
        let mut random_ownerships = self.get_random_ownerships(
            &load_balancer_info.unowned_or_expired,
            load_balancer_info.max_allowed - ours.len(),
        );
        ours.append(&mut random_ownerships);

        if ours.len() < load_balancer_info.max_allowed {
            debug!("Not enough expired or unowned partitions, will need to steal from other processors.");
            random_ownerships = self.get_random_ownerships(
                &load_balancer_info.above_max,
                load_balancer_info.max_allowed - ours.len(),
            );
            ours.append(&mut random_ownerships);
        }

        for ownership in ours.iter_mut() {
            self.reset_ownership(ownership);
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

    /// Load balance the ownership of partitions.
    /// This method checks the current ownership of partitions and determines whether to claim
    /// more ownership based on the current load and the maximum allowed ownership.
    /// It uses the `ProcessorStrategy` to determine the load balancing strategy.
    /// It returns a vector of `Ownership` objects representing the claimed ownership of partitions.
    /// It also logs the load balancing process and the claimed ownership.
    ///
    /// # Arguments
    /// * `partition_ids` - A slice of partition IDs to load balance.
    ///
    /// # Returns
    /// A result containing a vector of `Ownership` objects representing the newly claimed ownership of partitions.
    ///
    /// # Errors
    /// Returns an error if the load balancing process fails.
    pub async fn load_balance(&self, partition_ids: &[String]) -> Result<Vec<Ownership>> {
        debug!("Load balance for partitions: {}", partition_ids.join(", "));
        let load_balancer_info = self.get_available_partitions(partition_ids).await?;
        trace!(
            "[{}] Load balancer info: {:?}",
            self.consumer_client_details.client_id,
            load_balancer_info
        );

        let mut ownerships = load_balancer_info.current_ownership.clone();

        if load_balancer_info.claim_more_partitions {
            match self.processor_strategy {
                ProcessorStrategy::Balanced => {
                    debug!(
                        "[{}] Using balanced load balancer",
                        self.consumer_client_details.client_id
                    );
                    let ownership = self.balanced_load_balancer(&load_balancer_info);
                    if let Some(ownership) = ownership {
                        debug!(
                            "[{}] Claiming ownership of partition: {}",
                            self.consumer_client_details.client_id, ownership.partition_id
                        );
                        ownerships.push(ownership);
                    }
                }
                ProcessorStrategy::Greedy => {
                    debug!(
                        "[{}] Using greedy load balancer",
                        self.consumer_client_details.client_id
                    );
                    ownerships = self.greedy_load_balancer(&load_balancer_info);
                    debug!(
                        "[{}] Claiming ownership of {} partitions: {}",
                        self.consumer_client_details.client_id,
                        ownerships.len(),
                        Self::partitions_for_ownership(&ownerships)
                    );
                }
            }
        }

        let actual = self.checkpoint_store.claim_ownership(ownerships).await?;
        Ok(actual)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        event_processor::Ownership, in_memory_checkpoint_store::InMemoryCheckpointStore,
        models::ConsumerClientDetails, CheckpointStore,
    };
    use azure_core::Result;
    use tracing::info;

    static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

    pub fn test_setup() {
        INIT_LOGGING.call_once(|| {
            println!("Setting up test logger...");

            tracing_subscriber::fmt::init();
        });
    }

    fn map_to_strings<T, U>(source: &[T], mapper: U) -> Vec<String>
    where
        U: Fn(&T) -> String,
    {
        let mut result: Vec<String> = source.iter().map(mapper).collect();
        result.sort();
        result
    }

    fn group_by<T, U>(source: &[T], mapper: U) -> HashMap<String, Vec<T>>
    where
        T: Clone,
        U: Fn(&T) -> String,
    {
        let mut result: HashMap<String, Vec<T>> = HashMap::new();
        for item in source.iter() {
            let key = mapper(item);
            result.entry(key).or_default().push(item.clone());
        }
        result
    }

    async fn assert_balanced(
        checkpoint_store: Arc<dyn CheckpointStore>,
        partition_count: usize,
        number_of_consumers: usize,
    ) -> Result<()> {
        let ownerships = checkpoint_store
            .list_ownerships("fqdn1", "eventHub", "$Default")
            .await?;

        let minimum = partition_count / number_of_consumers;
        let mut maximum = minimum;
        if partition_count % number_of_consumers > 0 {
            maximum += 1;
        }
        assert_eq!(ownerships.len(), partition_count);

        let ownership_map = group_by(ownerships.as_slice(), |o| o.owner_id.clone());
        assert_eq!(number_of_consumers, ownership_map.len());

        for (owner_id, partitions) in ownership_map.iter() {
            let partition_count = partitions.len();
            info!(
                "Owner ID: {}, Partitions: {}",
                owner_id,
                map_to_strings(partitions, |o| o.partition_id.clone()).join(", ")
            );
            assert!(partition_count == minimum || partition_count == maximum);
        }

        let mut partition_ids = vec![];
        for ownership in ownerships.iter() {
            partition_ids.push(ownership.partition_id.clone());
        }

        assert_eq!(partition_ids.len(), partition_count);

        Ok(())
    }

    #[tokio::test]
    pub async fn processor_load_balancers_greedy_enough_unowned_partitions() -> Result<()> {
        test_setup();

        let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());
        checkpoint_store
            .claim_ownership(vec![
                Ownership {
                    fully_qualified_namespace: "fqdn1".to_string(),
                    event_hub_name: "eventHub".to_string(),
                    consumer_group: "$Default".to_string(),
                    partition_id: "0".to_string(),
                    owner_id: "some-other-client".to_string(),
                    ..Default::default()
                },
                Ownership {
                    fully_qualified_namespace: "fqdn1".to_string(),
                    event_hub_name: "eventHub".to_string(),
                    consumer_group: "$Default".to_string(),
                    partition_id: "3".to_string(),
                    owner_id: "some-other-client".to_string(),
                    ..Default::default()
                },
            ])
            .await?;

        let load_balancer = LoadBalancer::new(
            checkpoint_store.clone(),
            ConsumerClientDetails {
                fully_qualified_namespace: "fqdn1".to_string(),
                eventhub_name: "eventHub".to_string(),
                consumer_group: "$Default".to_string(),
                client_id: "new-client".to_string(),
            },
            ProcessorStrategy::Greedy,
            Duration::from_secs(3600),
        );

        let ownerships = load_balancer
            .load_balance(&[
                "0".to_string(),
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
            ])
            .await?;

        let partition_ids = map_to_strings(&ownerships, |o| o.partition_id.clone());
        info!(
            "Claimed ownership of partitions: {}",
            partition_ids.join(", ")
        );

        assert_eq!(partition_ids.len(), 2);
        assert_eq!(partition_ids[0], "1");
        assert_eq!(partition_ids[1], "2");

        let final_ownerships = checkpoint_store
            .list_ownerships("fqdn1", "eventHub", "$Default")
            .await?;

        assert_eq!(final_ownerships.len(), 4);

        Ok(())
    }

    #[tokio::test]
    pub async fn processor_load_balancers_balanced_unowned_partitions() -> Result<()> {
        test_setup();

        let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());
        checkpoint_store
            .claim_ownership(vec![
                Ownership {
                    fully_qualified_namespace: "fqdn1".to_string(),
                    event_hub_name: "eventHub".to_string(),
                    consumer_group: "$Default".to_string(),
                    partition_id: "0".to_string(),
                    owner_id: "some-other-client".to_string(),
                    ..Default::default()
                },
                Ownership {
                    fully_qualified_namespace: "fqdn1".to_string(),
                    event_hub_name: "eventHub".to_string(),
                    consumer_group: "$Default".to_string(),
                    partition_id: "3".to_string(),
                    owner_id: "some-other-client".to_string(),
                    ..Default::default()
                },
            ])
            .await?;

        let load_balancer = LoadBalancer::new(
            checkpoint_store.clone(),
            ConsumerClientDetails {
                fully_qualified_namespace: "fqdn1".to_string(),
                eventhub_name: "eventHub".to_string(),
                consumer_group: "$Default".to_string(),
                client_id: "new-client".to_string(),
            },
            ProcessorStrategy::Balanced,
            Duration::from_secs(3600),
        );

        let ownerships = load_balancer
            .load_balance(&[
                "0".to_string(),
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
            ])
            .await?;

        assert_eq!(ownerships.len(), 1);

        let ownerships = load_balancer
            .load_balance(&[
                "0".to_string(),
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
            ])
            .await?;
        assert_eq!(ownerships.len(), 2);

        assert_balanced(checkpoint_store.clone(), 4, 2).await?;

        Ok(())
    }
}
