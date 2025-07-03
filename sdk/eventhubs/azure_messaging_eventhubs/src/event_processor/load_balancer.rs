// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell::ignore rngs Seedable
use super::{
    models::Ownership,
    {CheckpointStore, ProcessorStrategy},
};
use crate::models::ConsumerClientDetails;
use azure_core::{error::ErrorKind as AzureErrorKind, time::Duration, Error, Result};
use rand::{seq::SliceRandom, Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex, MutexGuard},
    time::SystemTime,
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

    // Random number generator used for load balancing - this is a test hook to allow for deterministic test results.
    // In production, this should be a random number generator that is seeded with entropy.
    // The mutex ensures that we can access a mutable RNG even if self is immutable.
    rng: Mutex<Box<dyn RngCore + Send + Sync>>,
}

impl LoadBalancer {
    /// Creates a new LoadBalancer instance.
    pub fn new(
        checkpoint_store: Arc<dyn CheckpointStore>,
        consumer_client_details: ConsumerClientDetails,
        processor_strategy: ProcessorStrategy,
        duration: Duration,
        rng: Option<Box<dyn RngCore + Send + Sync>>,
    ) -> Self {
        LoadBalancer {
            checkpoint_store,
            processor_strategy,
            duration,
            consumer_client_details,
            rng: Mutex::new(rng.unwrap_or_else(|| Box::new(ChaCha20Rng::from_os_rng()))),
        }
    }

    fn rng(&self) -> Result<MutexGuard<'_, Box<dyn RngCore + Send + Sync>>> {
        self.rng
            .lock()
            .map_err(|_| Error::message(AzureErrorKind::Other, "Failed to lock RNG mutex"))
    }

    async fn get_available_partitions(&self, partition_ids: &[&str]) -> Result<LoadBalancerInfo> {
        trace!(
            "[{}]Get available partitions for {}/{}/{}",
            self.consumer_client_details.client_id,
            self.consumer_client_details.fully_qualified_namespace,
            self.consumer_client_details.eventhub_name,
            self.consumer_client_details.consumer_group
        );
        let ownerships = self
            .checkpoint_store
            .list_ownerships(
                &self.consumer_client_details.fully_qualified_namespace,
                &self.consumer_client_details.eventhub_name,
                &self.consumer_client_details.consumer_group,
            )
            .await?;

        trace!("Found {} Ownerships", ownerships.len());

        let mut unowned_or_expired: Vec<Ownership> = Vec::new();
        let mut partitions_already_added: HashSet<String> = HashSet::new();
        let mut grouped_by_owner: HashMap<String, Vec<Ownership>> = HashMap::new();
        grouped_by_owner.insert(self.consumer_client_details.client_id.clone(), Vec::new());

        for ownership in ownerships.iter() {
            partitions_already_added.insert(ownership.partition_id.clone());

            if let Some(last_modified_time) = ownership.last_modified_time {
                if last_modified_time + self.duration < SystemTime::now() {
                    unowned_or_expired.push(ownership.clone());
                    continue;
                }
            }
            // If the owner ID is empty, it means the partition is unowned.
            let Some(owner_id) = &ownership.owner_id else {
                unowned_or_expired.push(ownership.clone());
                continue;
            };

            // If we've not yet seen this owner, create an entry for the owner with an empty ownership vector.
            if !grouped_by_owner.contains_key(owner_id) {
                grouped_by_owner.insert(owner_id.clone(), Vec::new());
            }

            if let Some(val) = grouped_by_owner.get_mut(owner_id) {
                val.push(ownership.clone());
            };
        }

        let expired_count = unowned_or_expired.len();
        debug!("Number of expired partitions: {}", expired_count);

        // Add in any partitions that we haven't seen yet.
        for partition_id in partition_ids {
            if partitions_already_added.contains(*partition_id) {
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
                partition_id: partition_id.to_string(),
                owner_id: Some(self.consumer_client_details.client_id.clone()),
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

        // Sort the above_maximum partitions by partition ID to ensure that we are consistent in the order.
        // Note that we only need to do this when testing to ensure that the order of the partitions is consistent.
        #[cfg(test)]
        above_maximum.sort_by(|a, b| a.partition_id.cmp(&b.partition_id));

        let mut claim_more_partitions = true;
        let current: &[Ownership] =
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
                .get(&self.consumer_client_details.client_id)
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

    fn get_random_ownerships(
        &self,
        ownerships: &[Ownership],
        count: usize,
    ) -> Result<Vec<Ownership>> {
        debug!(
            "Getting random ownerships from {} with count {}",
            ownerships.len(),
            count
        );
        let limit = min(count, ownerships.len());
        if limit == 0 {
            return Ok(Vec::new());
        }

        let mut ownerships = ownerships.to_vec();

        ownerships.shuffle(self.rng()?.as_mut());
        ownerships.truncate(limit);

        debug!("Random ownerships: {:?}", ownerships);
        Ok(ownerships)
    }

    fn reset_ownership(&self, ownership: &mut Ownership) {
        ownership.owner_id = Some(self.consumer_client_details.client_id.clone());
    }

    fn balanced_load_balancer(
        &self,
        load_balancer_info: &LoadBalancerInfo,
    ) -> Result<Option<Ownership>> {
        if !load_balancer_info.unowned_or_expired.is_empty() {
            let index = self
                .rng()?
                .as_mut()
                .random_range(0..load_balancer_info.unowned_or_expired.len());
            let mut ownership = load_balancer_info.unowned_or_expired[index].clone();
            self.reset_ownership(&mut ownership);
            return Ok(Some(ownership));
        }

        if !load_balancer_info.above_max.is_empty() {
            let index = self
                .rng()?
                .as_mut()
                .random_range(0..load_balancer_info.above_max.len());
            let mut ownership = load_balancer_info.above_max[index].clone();
            self.reset_ownership(&mut ownership);
            return Ok(Some(ownership));
        }
        Ok(None)
    }

    fn greedy_load_balancer(
        &self,
        load_balancer_info: &LoadBalancerInfo,
    ) -> Result<Vec<Ownership>> {
        let mut ours = load_balancer_info.current_ownership.clone();
        debug!(
            "[{}]Greedy load balancer. ownership for client ID: {}",
            self.consumer_client_details.client_id,
            Self::partitions_for_ownership(&ours)
        );
        debug!(
            "UnownedOrExpired: {}",
            Self::partitions_for_ownership(&load_balancer_info.unowned_or_expired)
        );
        let mut random_ownerships = self.get_random_ownerships(
            &load_balancer_info.unowned_or_expired,
            load_balancer_info.max_allowed - ours.len(),
        )?;
        ours.append(&mut random_ownerships);

        if ours.len() < load_balancer_info.max_allowed {
            debug!("Not enough expired or unowned partitions, will need to steal from other processors. Stealing up to {} partitions.",
                load_balancer_info.max_allowed - ours.len());
            debug!("Stealing from {:?}", load_balancer_info.above_max);
            random_ownerships = self.get_random_ownerships(
                &load_balancer_info.above_max,
                load_balancer_info.max_allowed - ours.len(),
            )?;
            ours.append(&mut random_ownerships);
        }

        for ownership in ours.iter_mut() {
            self.reset_ownership(ownership);
        }
        Ok(ours)
    }

    fn partitions_for_ownership(ownerships: &[Ownership]) -> String {
        let mut s = "[".to_string();
        s += ownerships
            .iter()
            .map(|o| o.partition_id.clone())
            .collect::<Vec<_>>()
            .join(", ")
            .as_str();
        s += "]";
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
    pub async fn load_balance(&self, partition_ids: &[&str]) -> Result<Vec<Ownership>> {
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
                    let ownership = self.balanced_load_balancer(&load_balancer_info)?;
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
                    ownerships = self.greedy_load_balancer(&load_balancer_info)?;
                    debug!(
                        "[{}] Claiming ownership of {} partitions: {}",
                        self.consumer_client_details.client_id,
                        ownerships.len(),
                        Self::partitions_for_ownership(&ownerships)
                    );
                }
            }
        }

        let actual = self.checkpoint_store.claim_ownership(&ownerships).await?;
        Ok(actual)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::{
        event_processor::Ownership, in_memory_checkpoint_store::InMemoryCheckpointStore,
        models::ConsumerClientDetails, CheckpointStore,
    };
    use azure_core::Result;
    use tracing::info;

    pub fn test_setup() {
        crate::consumer::tests::setup();
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
        trace!("Asserting balanced ownership... {partition_count} partitions, {number_of_consumers} consumers");
        let ownerships = checkpoint_store
            .list_ownerships(TEST_EVENTHUB_FQDN, TEST_EVENTHUB_NAME, TEST_CONSUMER_GROUP)
            .await?;

        let minimum = partition_count / number_of_consumers;
        let mut maximum = minimum;
        if partition_count % number_of_consumers > 0 {
            info!("Adding one to maximum because of remainder");
            maximum += 1;
        }
        assert_eq!(ownerships.len(), partition_count);

        let ownership_map = group_by(ownerships.as_slice(), |o| {
            let Some(owner) = &o.owner_id else {
                return "".to_string();
            };
            owner.clone()
        });

        info!(
            "Ownership map: {}",
            ownership_map
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v.len()))
                .collect::<Vec<_>>()
                .join(", ")
        );
        assert_eq!(
            number_of_consumers,
            ownership_map.len(),
            "Expected {} consumers, but found {}",
            number_of_consumers,
            ownership_map.len()
        );

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
            .claim_ownership(&[
                new_test_ownership("0", "some-other-client"),
                new_test_ownership("3", "some-other-client"),
            ])
            .await?;

        let load_balancer = LoadBalancer::new(
            checkpoint_store.clone(),
            new_test_consumer_client_details("new-client"),
            ProcessorStrategy::Greedy,
            Duration::seconds(3600),
            None,
        );

        let ownerships = load_balancer.load_balance(&["0", "1", "2", "3"]).await?;

        let partition_ids = map_to_strings(&ownerships, |o| o.partition_id.clone());
        info!(
            "Claimed ownership of partitions: {}",
            partition_ids.join(", ")
        );

        assert_eq!(partition_ids.len(), 2);
        assert_eq!(partition_ids[0], "1");
        assert_eq!(partition_ids[1], "2");

        let final_ownerships = checkpoint_store
            .list_ownerships(TEST_EVENTHUB_FQDN, TEST_EVENTHUB_NAME, TEST_CONSUMER_GROUP)
            .await?;

        assert_eq!(final_ownerships.len(), 4);

        Ok(())
    }

    #[tokio::test]
    pub async fn processor_load_balancers_balanced_unowned_partitions() -> Result<()> {
        test_setup();

        let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());
        checkpoint_store
            .claim_ownership(&[
                new_test_ownership("0", "some-other-client"),
                new_test_ownership("3", "some-other-client"),
            ])
            .await?;

        let load_balancer = LoadBalancer::new(
            checkpoint_store.clone(),
            new_test_consumer_client_details("new-client"),
            ProcessorStrategy::Balanced,
            Duration::seconds(3600),
            None,
        );

        let ownerships = load_balancer.load_balance(&["0", "1", "2", "3"]).await?;

        assert_eq!(ownerships.len(), 1);

        let ownerships = load_balancer.load_balance(&["0", "1", "2", "3"]).await?;
        assert_eq!(ownerships.len(), 2);

        assert_balanced(checkpoint_store.clone(), 4, 2).await?;

        Ok(())
    }

    const TEST_EVENTHUB_FQDN: &str = "test-eventhub-fqdn";
    const TEST_EVENTHUB_NAME: &str = "test-eventhub-name";
    const TEST_CONSUMER_GROUP: &str = "test-consumer-group";

    fn new_test_ownership(partition_id: &str, client_id: &str) -> Ownership {
        Ownership {
            fully_qualified_namespace: TEST_EVENTHUB_FQDN.to_string(),
            event_hub_name: TEST_EVENTHUB_NAME.to_string(),
            consumer_group: TEST_CONSUMER_GROUP.to_string(),
            partition_id: partition_id.to_string(),
            owner_id: Some(client_id.to_string()),
            ..Default::default()
        }
    }

    fn new_test_consumer_client_details(client_id: &str) -> ConsumerClientDetails {
        ConsumerClientDetails {
            fully_qualified_namespace: TEST_EVENTHUB_FQDN.to_string(),
            eventhub_name: TEST_EVENTHUB_NAME.to_string(),
            consumer_group: TEST_CONSUMER_GROUP.to_string(),
            client_id: client_id.to_string(),
        }
    }

    fn find_common<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
        let mut common = vec![];
        for item in a.into_iter() {
            if b.contains(&item) {
                common.push(item);
            }
        }
        common
    }

    #[tokio::test]
    async fn processor_load_balancers_greedy_forced_to_steal() -> Result<()> {
        test_setup();

        let some_other_client_id = "some-other-client-id";
        let stealing_client_id = "stealing-client-id";

        let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());

        checkpoint_store
            .claim_ownership(&[
                new_test_ownership("0", some_other_client_id),
                new_test_ownership("1", some_other_client_id),
                new_test_ownership("2", some_other_client_id),
                new_test_ownership("3", some_other_client_id),
                new_test_ownership("4", some_other_client_id),
            ])
            .await?;

        let load_balancer = LoadBalancer::new(
            checkpoint_store.clone(),
            new_test_consumer_client_details(stealing_client_id),
            ProcessorStrategy::Greedy,
            Duration::seconds(3600),
            None,
        );

        let ownerships = load_balancer
            .load_balance(&["0", "1", "2", "3", "4"])
            .await?;

        let strings = map_to_strings(&ownerships, |o| o.partition_id.clone());
        info!("Claimed ownership of partitions: {}", strings.join(", "));
        assert_eq!(ownerships.len(), 2);

        let final_ownerships = checkpoint_store
            .list_ownerships(TEST_EVENTHUB_FQDN, TEST_EVENTHUB_NAME, TEST_CONSUMER_GROUP)
            .await?;

        let owners_map = group_by(final_ownerships.as_slice(), |o| {
            o.owner_id.clone().unwrap().clone()
        });

        let other_partitions = map_to_strings(
            &owners_map[&some_other_client_id.to_string()],
            |o: &Ownership| o.partition_id.clone(),
        );
        let stealing_partitions = map_to_strings(
            &owners_map[&stealing_client_id.to_string()],
            |o: &Ownership| o.partition_id.clone(),
        );

        let common_elements = find_common(other_partitions, stealing_partitions);
        assert_eq!(common_elements.len(), 0);

        info!(
            "Final ownerships: {}",
            owners_map
                .iter()
                .map(|(k, v)| format!(
                    "{}: {}",
                    k,
                    map_to_strings(v, |o| o.partition_id.clone()).join(",")
                ))
                .collect::<Vec<_>>()
                .join(", ")
        );

        assert_eq!(final_ownerships.len(), 5);

        Ok(())
    }

    async fn expire_ownership(
        checkpoint_store: Arc<dyn CheckpointStore>,
        ownership: &Ownership,
    ) -> Result<()> {
        let ownerships = checkpoint_store
            .list_ownerships(
                &ownership.fully_qualified_namespace,
                &ownership.event_hub_name,
                &ownership.consumer_group,
            )
            .await?;
        let etag = ownerships
            .iter()
            .find(|o| {
                o.partition_id == ownership.partition_id
                    && o.owner_id == ownership.owner_id
                    && o.consumer_group == ownership.consumer_group
            })
            .unwrap()
            .etag
            .clone();
        let mut ownership = ownership.clone();
        ownership.last_modified_time = Some(SystemTime::now() - Duration::seconds(3600));
        ownership.etag = etag;
        checkpoint_store.update_ownership(ownership).await?;
        Ok(())
    }

    async fn relinquish_ownership(
        checkpoint_store: Arc<dyn CheckpointStore>,
        ownership: &Ownership,
    ) -> Result<()> {
        let ownerships = checkpoint_store
            .list_ownerships(
                &ownership.fully_qualified_namespace,
                &ownership.event_hub_name,
                &ownership.consumer_group,
            )
            .await?;
        let etag = ownerships
            .iter()
            .find(|o| {
                o.partition_id == ownership.partition_id
                    && o.owner_id == ownership.owner_id
                    && o.consumer_group == ownership.consumer_group
            })
            .unwrap()
            .etag
            .clone();
        let mut ownership = ownership.clone();
        ownership.owner_id = None;
        ownership.etag = etag;
        checkpoint_store.update_ownership(ownership).await?;
        Ok(())
    }

    #[tokio::test]
    async fn processor_load_balancers_any_strategy_grab_expired_partition() -> Result<()> {
        test_setup();

        for strategy in [ProcessorStrategy::Greedy, ProcessorStrategy::Balanced] {
            const CLIENT_A: &str = "clientA";
            const CLIENT_B: &str = "clientB";
            const CLIENT_C_WITH_EXPIRED_PARTITION: &str = "clientC";

            let middle_ownership = new_test_ownership("2", CLIENT_C_WITH_EXPIRED_PARTITION);

            let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());
            checkpoint_store
                .claim_ownership(&[
                    new_test_ownership("0", CLIENT_A),
                    new_test_ownership("1", CLIENT_A),
                    middle_ownership.clone(),
                    new_test_ownership("3", CLIENT_B),
                    new_test_ownership("4", CLIENT_B),
                ])
                .await?;

            info!(
                "Expiring ownership for partition {}",
                middle_ownership.partition_id
            );
            expire_ownership(checkpoint_store.clone(), &middle_ownership).await?;

            info!("Load balancing with strategy: {:?}", strategy);
            let load_balancer = LoadBalancer::new(
                checkpoint_store.clone(),
                new_test_consumer_client_details(CLIENT_B),
                strategy,
                Duration::seconds(3600),
                None,
            );

            let ownerships = load_balancer
                .load_balance(&["0", "1", "2", "3", "4"])
                .await?;

            assert!(!map_to_strings(&ownerships, |o| o.partition_id.clone()).is_empty());
            assert_balanced(checkpoint_store, 5, 2).await?;
        }
        Ok(())
    }

    #[tokio::test]
    async fn processor_load_balancers_any_strategy_fully_balanced_odd() -> Result<()> {
        test_setup();

        for strategy in [ProcessorStrategy::Greedy, ProcessorStrategy::Balanced] {
            const CLIENT_A: &str = "clientA";
            const CLIENT_B: &str = "clientB";

            let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());
            checkpoint_store
                .claim_ownership(&[
                    new_test_ownership("0", CLIENT_A),
                    new_test_ownership("1", CLIENT_A),
                    new_test_ownership("2", CLIENT_A),
                    new_test_ownership("3", CLIENT_B),
                    new_test_ownership("4", CLIENT_B),
                ])
                .await?;

            info!("Load balancing with strategy: {:?}", strategy);
            {
                let load_balancer_b = LoadBalancer::new(
                    checkpoint_store.clone(),
                    new_test_consumer_client_details(CLIENT_B),
                    strategy,
                    Duration::seconds(3600),
                    None,
                );

                let ownerships = load_balancer_b
                    .load_balance(&["0", "1", "2", "3", "4"])
                    .await?;

                let mut owned_partitions = map_to_strings(&ownerships, |o| o.partition_id.clone());
                info!(
                    "Claimed ownership of partitions: {}",
                    owned_partitions.join(", ")
                );
                owned_partitions.sort();
                assert_eq!(ownerships.len(), 2);
                assert_eq!(owned_partitions[0], "3");
                assert_eq!(owned_partitions[1], "4");
                assert_balanced(checkpoint_store.clone(), 5, 2).await?;
            }
            {
                let load_balancer_a = LoadBalancer::new(
                    checkpoint_store.clone(),
                    new_test_consumer_client_details(CLIENT_A),
                    strategy,
                    Duration::seconds(3600),
                    None,
                );

                let ownerships = load_balancer_a
                    .load_balance(&["0", "1", "2", "3", "4"])
                    .await?;

                let mut owned_partitions = map_to_strings(&ownerships, |o| o.partition_id.clone());
                info!(
                    "Claimed ownership of partitions: {}",
                    owned_partitions.join(", ")
                );
                owned_partitions.sort();
                assert_eq!(ownerships.len(), 3);
                assert_eq!(owned_partitions[0], "0");
                assert_eq!(owned_partitions[1], "1");
                assert_eq!(owned_partitions[2], "2");
                assert_balanced(checkpoint_store, 5, 2).await?;
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn processor_load_balancers_any_strategy_fully_balanced_even() -> Result<()> {
        test_setup();

        for strategy in [ProcessorStrategy::Greedy, ProcessorStrategy::Balanced] {
            const CLIENT_A: &str = "clientA";
            const CLIENT_B: &str = "clientB";

            let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());
            checkpoint_store
                .claim_ownership(&[
                    new_test_ownership("0", CLIENT_A),
                    new_test_ownership("1", CLIENT_A),
                    new_test_ownership("2", CLIENT_B),
                    new_test_ownership("3", CLIENT_B),
                ])
                .await?;

            info!("Load balancing with strategy: {:?}", strategy);
            {
                let load_balancer_b = LoadBalancer::new(
                    checkpoint_store.clone(),
                    new_test_consumer_client_details(CLIENT_B),
                    strategy,
                    Duration::seconds(3600),
                    None,
                );

                let ownerships = load_balancer_b.load_balance(&["0", "1", "2", "3"]).await?;

                let mut owned_partitions = map_to_strings(&ownerships, |o| o.partition_id.clone());
                info!(
                    "Claimed ownership of partitions: {}",
                    owned_partitions.join(", ")
                );
                owned_partitions.sort();
                assert_eq!(ownerships.len(), 2);
                assert_eq!(owned_partitions[0], "2");
                assert_eq!(owned_partitions[1], "3");
                assert_balanced(checkpoint_store.clone(), 4, 2).await?;
            }
            {
                let load_balancer_a = LoadBalancer::new(
                    checkpoint_store.clone(),
                    new_test_consumer_client_details(CLIENT_A),
                    strategy,
                    Duration::seconds(3600),
                    None,
                );

                let ownerships = load_balancer_a.load_balance(&["0", "1", "2", "3"]).await?;

                let mut owned_partitions = map_to_strings(&ownerships, |o| o.partition_id.clone());
                info!(
                    "Claimed ownership of partitions: {}",
                    owned_partitions.join(", ")
                );
                owned_partitions.sort();
                assert_eq!(ownerships.len(), 2);
                assert_eq!(owned_partitions[0], "0");
                assert_eq!(owned_partitions[1], "1");
                assert_balanced(checkpoint_store, 4, 2).await?;
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn processor_load_balancers_any_strategy_grab_extra_partition_because_above_max(
    ) -> Result<()> {
        test_setup();

        for strategy in [ProcessorStrategy::Greedy, ProcessorStrategy::Balanced] {
            const CLIENT_A: &str = "clientA";
            const CLIENT_B: &str = "clientB";

            let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());
            checkpoint_store
                .claim_ownership(&[
                    new_test_ownership("0", CLIENT_A),
                    new_test_ownership("1", CLIENT_A),
                    // Nobody owns 2.
                    new_test_ownership("3", CLIENT_B),
                    new_test_ownership("4", CLIENT_B),
                ])
                .await?;

            info!("Load balancing with strategy: {:?}", strategy);

            let load_balancer = LoadBalancer::new(
                checkpoint_store.clone(),
                new_test_consumer_client_details(CLIENT_B),
                strategy,
                Duration::seconds(3600),
                None,
            );

            let ownerships = load_balancer
                .load_balance(&["0", "1", "2", "3", "4"])
                .await?;

            let owned_partitions = map_to_strings(&ownerships, |o| o.partition_id.clone());
            info!(
                "Claimed ownership of partitions: {}",
                owned_partitions.join(", ")
            );
            assert!(!owned_partitions.is_empty());
            assert_balanced(checkpoint_store.clone(), 5, 2).await?;
        }
        Ok(())
    }

    #[tokio::test]
    async fn processor_load_balancers_any_strategy_steals_to_balance() -> Result<()> {
        test_setup();

        for strategy in [ProcessorStrategy::Greedy, ProcessorStrategy::Balanced] {
            const LOTS_CLIENT_ID: &str = "has-too-many-client-id";
            const LITTLE_CLIENT_ID: &str = "has-too-few-id";

            let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());

            let all_partition_ids = vec!["0", "1", "2", "3"];

            checkpoint_store
                .claim_ownership(&[
                    new_test_ownership(all_partition_ids[0], LOTS_CLIENT_ID),
                    new_test_ownership(all_partition_ids[1], LOTS_CLIENT_ID),
                    new_test_ownership(all_partition_ids[2], LOTS_CLIENT_ID),
                    new_test_ownership(all_partition_ids[3], LITTLE_CLIENT_ID),
                ])
                .await?;

            info!("Load balancing with strategy: {:?}", strategy);

            {
                let too_many_load_balancer = LoadBalancer::new(
                    checkpoint_store.clone(),
                    new_test_consumer_client_details(LOTS_CLIENT_ID),
                    strategy,
                    Duration::seconds(3600),
                    None,
                );

                let ownerships = too_many_load_balancer
                    .load_balance(&all_partition_ids)
                    .await?;

                let mut owned_partitions = map_to_strings(&ownerships, |o| o.partition_id.clone());
                owned_partitions.sort();
                info!(
                    "Too many has claimed ownership of partitions: {}",
                    owned_partitions.join(", ")
                );
                assert_eq!("0, 1, 2", owned_partitions.join(", "));
            }
            {
                let too_few_load_balancer = LoadBalancer::new(
                    checkpoint_store.clone(),
                    new_test_consumer_client_details(LITTLE_CLIENT_ID),
                    strategy,
                    Duration::seconds(3600),
                    None,
                );

                let ownerships = too_few_load_balancer
                    .load_balance(&all_partition_ids)
                    .await?;

                info!("Too Few ownerships: {:?}", ownerships);
                let mut owned_partitions = map_to_strings(&ownerships, |o| o.partition_id.clone());
                owned_partitions.sort();
                info!(
                    "Claimed ownership of partitions: {}",
                    owned_partitions.join(", ")
                );

                assert_eq!(ownerships.len(), 2);
            }

            assert_balanced(checkpoint_store, all_partition_ids.len(), 2).await?;
        }
        Ok(())
    }

    #[tokio::test]
    async fn processor_load_balancers_any_strategy_grab_relinquished_partition() -> Result<()> {
        test_setup();

        for strategy in [ProcessorStrategy::Greedy, ProcessorStrategy::Balanced] {
            const CLIENT_A: &str = "clientA";
            const CLIENT_B: &str = "clientB";
            const CLIENT_C_WITH_EXPIRED_PARTITION: &str = "clientC";

            let middle_ownership = new_test_ownership("2", CLIENT_C_WITH_EXPIRED_PARTITION);

            let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());
            checkpoint_store
                .claim_ownership(&[
                    new_test_ownership("0", CLIENT_A),
                    new_test_ownership("1", CLIENT_A),
                    middle_ownership.clone(),
                    new_test_ownership("3", CLIENT_B),
                    new_test_ownership("4", CLIENT_B),
                ])
                .await?;

            info!(
                "Expiring ownership for partition {}",
                middle_ownership.partition_id
            );
            relinquish_ownership(checkpoint_store.clone(), &middle_ownership).await?;

            info!("Load balancing with strategy: {:?}", strategy);
            let load_balancer = LoadBalancer::new(
                checkpoint_store.clone(),
                new_test_consumer_client_details(CLIENT_B),
                strategy,
                Duration::seconds(3600),
                None,
            );

            let ownerships = load_balancer
                .load_balance(&["0", "1", "2", "3", "4"])
                .await?;

            assert!(!map_to_strings(&ownerships, |o| o.partition_id.clone()).is_empty());
            assert_balanced(checkpoint_store, 5, 2).await?;
        }
        Ok(())
    }

    #[tokio::test]
    async fn unit_test_load_balancer_balanced() -> Result<()> {
        test_setup();
        info!("Unit test for load balancer");

        //cspell: ignore abbc abbcc aaaabbb
        for state in ["abc", "abbc", "abbcc"] {
            for owner in ["a", "b", "c"] {
                info!("Layout {state} with owner {owner}");

                let (lb, parts) = create_load_balancer_for_unit_tests(state, owner).await?;
                let load_balancer_info = lb
                    .get_available_partitions(
                        &parts.iter().map(String::as_str).collect::<Vec<&str>>(),
                    )
                    .await?;

                assert!(!load_balancer_info.claim_more_partitions);
                assert!(load_balancer_info.above_max.is_empty());
            }
        }

        info!("Balanced with unequal ownership");
        let (lb, parts) = create_load_balancer_for_unit_tests("aaaabbb", "a").await?;
        let load_balancer_info = lb
            .get_available_partitions(&parts.iter().map(String::as_str).collect::<Vec<&str>>())
            .await?;
        assert!(!load_balancer_info.claim_more_partitions);
        assert!(load_balancer_info.above_max.is_empty());
        assert_eq!(4, load_balancer_info.max_allowed);
        assert!(load_balancer_info.above_max.is_empty());

        let (lb2, partitions2) = create_load_balancer_for_unit_tests("aaaabbb", "b").await?;
        let lbi2 = lb2
            .get_available_partitions(
                &partitions2
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<&str>>(),
            )
            .await?;
        assert!(!lbi2.claim_more_partitions);
        assert_eq!(4, lbi2.max_allowed);
        assert!(lbi2.above_max.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn unit_test_load_balancer_unbalanced() -> Result<()> {
        test_setup();
        info!("Unit test for load balancer (unbalanced)");

        //cspell: ignore aaaabb aaabbbcccd aaabbbccde aaaabbc

        {
            info!("A new owner enters the field.");
            let (load_balancer, partitions) =
                create_load_balancer_for_unit_tests("abb", "c").await?;
            let load_balancer_info = load_balancer
                .get_available_partitions(
                    &partitions.iter().map(String::as_str).collect::<Vec<&str>>(),
                )
                .await?;
            assert!(load_balancer_info.claim_more_partitions);
            assert_eq!(1, load_balancer_info.max_allowed);
            let lb_result =
                greedy_load_balance(&load_balancer, &load_balancer_info, partitions.len()).await?;
            assert_eq!(".c.", lb_result);
        }

        {
            info!("deficit, single partition");
            // Existing owner needs to steal a partition.
            let (load_balancer, partitions) =
                create_load_balancer_for_unit_tests("aaaabb", "b").await?;
            let load_balancer_info = load_balancer
                .get_available_partitions(
                    &partitions.iter().map(String::as_str).collect::<Vec<&str>>(),
                )
                .await?;
            assert!(load_balancer_info.claim_more_partitions);
            assert_eq!(3, load_balancer_info.max_allowed);
            let load_balance_result =
                greedy_load_balance(&load_balancer, &load_balancer_info, partitions.len()).await?;
            assert_eq!("..b.bb", load_balance_result);
        }
        {
            info!("deficit, multiple partitions");
            let (load_balancer, partitions) =
                create_load_balancer_for_unit_tests("aaabbbcccd", "d").await?;
            let load_balancer_info = load_balancer
                .get_available_partitions(
                    &partitions.iter().map(String::as_str).collect::<Vec<&str>>(),
                )
                .await?;
            assert!(load_balancer_info.claim_more_partitions);
            assert_eq!(2, load_balancer_info.max_allowed);
            let load_balance_result =
                greedy_load_balance(&load_balancer, &load_balancer_info, partitions.len()).await?;
            assert_eq!("........dd", load_balance_result);
        }

        {
            info!("deficit, multiple owners");
            let (load_balancer, partitions) =
                create_load_balancer_for_unit_tests("aaabbbccde", "d").await?;
            let load_balancer_info = load_balancer
                .get_available_partitions(
                    &partitions.iter().map(String::as_str).collect::<Vec<&str>>(),
                )
                .await?;
            assert!(load_balancer_info.claim_more_partitions);
            assert_eq!(2, load_balancer_info.max_allowed);
            let load_balance_result =
                greedy_load_balance(&load_balancer, &load_balancer_info, partitions.len()).await?;
            assert_eq!("..d.....d.", load_balance_result);
        }

        {
            info!("can grab an extra partition");
            let (load_balancer, partitions) =
                create_load_balancer_for_unit_tests("aaaabbc", "b").await?;
            let load_balancer_info = load_balancer
                .get_available_partitions(
                    &partitions.iter().map(String::as_str).collect::<Vec<&str>>(),
                )
                .await?;
            assert!(load_balancer_info.claim_more_partitions);
            assert_eq!(3, load_balancer_info.max_allowed);
            let load_balance_result =
                greedy_load_balance(&load_balancer, &load_balancer_info, partitions.len()).await?;
            assert_eq!("..b.bb.", load_balance_result);
        }

        Ok(())
    }

    fn ownerships_as_string(ownerships: &[Ownership], partition_count: usize) -> String {
        let mut bits = Vec::new();
        bits.resize(partition_count, '.');

        for o in ownerships {
            let partition_id = o.partition_id.parse::<usize>().unwrap();
            bits[partition_id] = if let Some(owner_id) = &o.owner_id {
                owner_id.chars().next().unwrap()
            } else {
                '.'
            };
        }

        bits.iter().collect::<String>()
    }

    async fn greedy_load_balance(
        load_balancer: &LoadBalancer,
        load_balancer_info: &LoadBalancerInfo,
        partition_count: usize,
    ) -> Result<String> {
        let ownerships = load_balancer.greedy_load_balancer(load_balancer_info)?;

        let mut above_max = load_balancer_info.above_max.clone();
        above_max.sort_by(|a, b| a.partition_id.cmp(&b.partition_id));

        let owned_as_string = ownerships_as_string(&ownerships, partition_count);

        info!("Claimed ownership of partitions: {}", owned_as_string);

        Ok(owned_as_string)
    }

    async fn create_load_balancer_for_unit_tests(
        state: &str,
        owner: &str,
    ) -> Result<(LoadBalancer, Vec<String>)> {
        let client_details = ConsumerClientDetails {
            fully_qualified_namespace: "fake_namespace.servicebus.windows.net".to_string(),
            eventhub_name: "fake_eventhub".to_string(),
            consumer_group: "$Default".to_string(),
            client_id: owner.to_string(),
        };
        // Create a list of partition IDs based on the indexes of the values in the state string
        let partition_map = state
            .chars()
            .enumerate()
            .map(|(i, val)| (i.to_string(), val.to_string()))
            .collect::<Vec<_>>();

        let mut partition_ids = Vec::new();
        let mut ownerships = Vec::new();
        for partition_info in partition_map.iter() {
            partition_ids.push(partition_info.0.clone());
            ownerships.push(Ownership {
                fully_qualified_namespace: client_details.fully_qualified_namespace.clone(),
                event_hub_name: client_details.eventhub_name.clone(),
                consumer_group: client_details.consumer_group.clone(),
                partition_id: partition_info.0.clone(),
                owner_id: Some(partition_info.1.clone()),
                ..Default::default()
            });
        }

        let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());
        let claimed = checkpoint_store.claim_ownership(&ownerships).await?;
        assert_eq!(claimed.len(), partition_ids.len());

        // A constant seed for the ChaCha20Rng to ensure deterministic results.
        // This is important for the tests to be repeatable.
        let seed = [
            1, 0, 52, 0, 0, 0, 0, 0, 1, 0, 10, 0, 22, 32, 0, 0, 2, 0, 55, 49, 0, 11, 0, 0, 3, 0, 0,
            0, 0, 0, 2, 92,
        ];

        let load_balancer = LoadBalancer::new(
            checkpoint_store.clone(),
            client_details,
            ProcessorStrategy::Balanced,                  // Ignored.
            Duration::seconds(3600), // No partitions are expired in these tests.
            Some(Box::new(ChaCha20Rng::from_seed(seed))), // For deterministic results.
        );
        Ok((load_balancer, partition_ids))
    }
}
