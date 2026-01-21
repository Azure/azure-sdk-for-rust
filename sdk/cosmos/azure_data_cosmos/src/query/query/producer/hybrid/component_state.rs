// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::query::{
    producer::{hybrid::models::HybridRequestId, state::PaginationState},
    DataRequest, QueryInfo,
};

#[derive(Debug)]
pub struct ComponentQueryState {
    pub query_index: u32,
    pub query_info: QueryInfo,
    pub weight: f64,
    partition_states: Vec<(String, PaginationState)>,
    remaining_partitions: usize,
}

impl ComponentQueryState {
    pub fn new(
        query_index: u32,
        query_info: QueryInfo,
        weight: f64,
        pkrange_ids: &[String],
    ) -> Self {
        tracing::trace!(
            query_index,
            weight,
            ?pkrange_ids,
            "creating component query state"
        );
        Self {
            query_index,
            query_info,
            weight,
            partition_states: pkrange_ids
                .iter()
                .map(|pkrange_id| (pkrange_id.clone(), PaginationState::Initial))
                .collect(),
            remaining_partitions: pkrange_ids.len(),
        }
    }

    pub fn requests(&self) -> Vec<DataRequest> {
        let mut requests = Vec::new();
        for (pkrange_id, pagination_state) in &self.partition_states {
            let req = match pagination_state {
                PaginationState::Initial => Some(DataRequest::with_query(
                    HybridRequestId::for_component_query(self.query_index, 0).into(),
                    pkrange_id.clone(),
                    None,
                    self.query_info.rewritten_query.clone(),
                    true,
                )),
                PaginationState::Continuing {
                    next_page_index,
                    token,
                } => Some(DataRequest::with_query(
                    HybridRequestId::for_component_query(self.query_index, *next_page_index as u32)
                        .into(),
                    pkrange_id.clone(),
                    Some(token.clone()),
                    self.query_info.rewritten_query.clone(),
                    true,
                )),
                PaginationState::Done => None,
            };
            if let Some(request) = req {
                requests.push(request);
            }
        }
        requests
    }

    pub fn complete(&self) -> bool {
        let result = self.remaining_partitions == 0;
        debug_assert!(
            !result
                || self
                    .partition_states
                    .iter()
                    .all(|(_, state)| matches!(state, PaginationState::Done))
        );
        result
    }

    pub fn update_partition_state(
        &mut self,
        pkrange_id: &str,
        continuation: Option<String>,
    ) -> crate::Result<()> {
        let state = self
            .partition_states
            .iter_mut()
            .find(|(id, _)| id == pkrange_id)
            .ok_or_else(|| {
                crate::ErrorKind::InvalidGatewayResponse.with_message(format!(
                    "received response for unknown partition key range ID: {}",
                    pkrange_id
                ))
            })?;
        if matches!(state.1, PaginationState::Done) {
            Ok(())
        } else {
            state.1.update(continuation);
            if matches!(state.1, PaginationState::Done) {
                self.remaining_partitions -= 1;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::QueryInfo;

    fn create_test_query_info(query: &str) -> QueryInfo {
        QueryInfo {
            rewritten_query: query.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_completion_and_remaining_partitions() {
        let pkrange_ids = vec!["p1".to_string(), "p2".to_string(), "p3".to_string()];
        let mut state = ComponentQueryState::new(
            0,
            create_test_query_info("SELECT * FROM c"),
            1.0,
            &pkrange_ids,
        );

        assert!(!state.complete());
        assert_eq!(state.remaining_partitions, 3);

        // Continuation tokens should not affect completion or remaining count
        state
            .update_partition_state("p1", Some("token1".to_string()))
            .unwrap();
        assert!(!state.complete());
        assert_eq!(state.remaining_partitions, 3);

        state
            .update_partition_state("p2", Some("token2".to_string()))
            .unwrap();
        assert!(!state.complete());
        assert_eq!(state.remaining_partitions, 3);

        // Multiple continuations on same partition should not change count
        state
            .update_partition_state("p1", Some("token1_page2".to_string()))
            .unwrap();
        assert!(!state.complete());
        assert_eq!(state.remaining_partitions, 3);

        // Complete partitions one at a time
        state.update_partition_state("p1", None).unwrap();
        assert!(!state.complete());
        assert_eq!(state.remaining_partitions, 2);

        state.update_partition_state("p2", None).unwrap();
        assert!(!state.complete());
        assert_eq!(state.remaining_partitions, 1);

        state.update_partition_state("p3", None).unwrap();
        assert!(state.complete());
        assert_eq!(state.remaining_partitions, 0);
    }

    #[test]
    fn test_request_generation_lifecycle() {
        let pkrange_ids = vec!["p1".to_string(), "p2".to_string()];
        let mut state = ComponentQueryState::new(
            2,
            create_test_query_info("SELECT c.value FROM c ORDER BY c.timestamp"),
            0.7,
            &pkrange_ids,
        );

        let query = "SELECT c.value FROM c ORDER BY c.timestamp";
        let requests = state.requests();
        let expected_requests = vec![
            DataRequest::with_query(
                HybridRequestId::for_component_query(2, 0).into(),
                "p1",
                None,
                query,
                true,
            ),
            DataRequest::with_query(
                HybridRequestId::for_component_query(2, 0).into(),
                "p2",
                None,
                query,
                true,
            ),
        ];
        assert_eq!(requests, expected_requests);

        state
            .update_partition_state("p1", Some("continuation_token".to_string()))
            .unwrap();
        let requests = state.requests();
        let expected_requests = vec![
            DataRequest::with_query(
                HybridRequestId::for_component_query(2, 1).into(),
                "p1",
                Some("continuation_token".to_string()),
                query,
                true,
            ),
            DataRequest::with_query(
                HybridRequestId::for_component_query(2, 0).into(),
                "p2",
                None,
                query,
                true,
            ),
        ];
        assert_eq!(requests, expected_requests);

        state.update_partition_state("p1", None).unwrap();
        let requests = state.requests();
        let expected_requests = vec![DataRequest::with_query(
            HybridRequestId::for_component_query(2, 0).into(),
            "p2",
            None,
            query,
            true,
        )];
        assert_eq!(requests, expected_requests);

        state.update_partition_state("p2", None).unwrap();
        let requests = state.requests();
        let expected_requests: Vec<DataRequest> = vec![];
        assert_eq!(requests, expected_requests);
    }

    #[test]
    fn test_request_id_generation() {
        let pkrange_ids = vec!["partition1".to_string()];
        let mut state = ComponentQueryState::new(
            5,
            create_test_query_info("SELECT * FROM c"),
            1.0,
            &pkrange_ids,
        );

        let requests = state.requests();
        let expected_id_page0: u64 = HybridRequestId::for_component_query(5, 0).into();
        assert_eq!(requests[0].id, expected_id_page0);

        state
            .update_partition_state("partition1", Some("token1".to_string()))
            .unwrap();
        let requests = state.requests();
        let expected_id_page1: u64 = HybridRequestId::for_component_query(5, 1).into();
        assert_eq!(requests[0].id, expected_id_page1);

        state
            .update_partition_state("partition1", Some("token2".to_string()))
            .unwrap();
        let requests = state.requests();
        let expected_id_page2: u64 = HybridRequestId::for_component_query(5, 2).into();
        assert_eq!(requests[0].id, expected_id_page2);
    }

    #[test]
    fn test_mixed_partition_states() {
        let pkrange_ids = vec!["p1".to_string(), "p2".to_string(), "p3".to_string()];
        let mut state = ComponentQueryState::new(
            0,
            create_test_query_info("SELECT c.data FROM c"),
            1.0,
            &pkrange_ids,
        );

        state
            .update_partition_state("p2", Some("token_p2".to_string()))
            .unwrap();
        state.update_partition_state("p3", None).unwrap();

        let requests = state.requests();
        let expected_requests = vec![
            DataRequest::with_query(
                HybridRequestId::for_component_query(0, 0).into(),
                "p1",
                None,
                "SELECT c.data FROM c",
                true,
            ),
            DataRequest::with_query(
                HybridRequestId::for_component_query(0, 1).into(),
                "p2",
                Some("token_p2".to_string()),
                "SELECT c.data FROM c",
                true,
            ),
        ];
        assert_eq!(requests, expected_requests);
        assert!(!state.complete());
        assert_eq!(state.remaining_partitions, 2);

        state.update_partition_state("p1", None).unwrap();
        let requests = state.requests();
        let expected_requests = vec![DataRequest::with_query(
            HybridRequestId::for_component_query(0, 1).into(),
            "p2",
            Some("token_p2".to_string()),
            "SELECT c.data FROM c",
            true,
        )];
        assert_eq!(requests, expected_requests);
        assert_eq!(state.remaining_partitions, 1);

        state.update_partition_state("p2", None).unwrap();
        let requests = state.requests();
        let expected_requests: Vec<DataRequest> = vec![];
        assert_eq!(requests, expected_requests);
        assert!(state.complete());
        assert_eq!(state.remaining_partitions, 0);
    }
}
