// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    driver::{
        dataflow::{
            intersect_feed_ranges, validate_exact_coverage, PipelineNodeState, RangedToken,
        },
        CosmosDriver,
    },
    error::{status_codes::CLIENT_BAD_REQUEST, CosmosError, Result},
    models::{
        effective_partition_key::EffectivePartitionKey, ChangeFeedStartFrom, ContainerReference,
        ContinuationToken, CosmosOperation, FeedRange, ResolvedToken,
    },
};

impl CosmosDriver {
    /// Derives independent child change-feed checkpoints from confirmed parent progress.
    ///
    /// `child_ranges` must be nonempty, disjoint, and cover `parent_range` exactly.
    /// Results correspond to the supplied order, not the current physical topology.
    /// Logical ranges can remain separate after a physical partition merge.
    ///
    /// Only driver-issued, account- and scope-bound EPK change-feed checkpoints are
    /// supported. The retained container RID, account endpoint, scope, and mode must
    /// match. `full_fidelity` selects AllVersionsAndDeletes rather than LatestVersion.
    /// The original start boundary and each saved range's server position are retained.
    /// No I/O is performed; subsequent reads still validate the retained source with
    /// the service and must not restart on a recreated container.
    ///
    /// # Errors
    ///
    /// Returns an error for legacy/unbound or raw server tokens, incompatible source,
    /// scope or mode, invalid child coverage, unsupported snapshot shapes, or `Now`
    /// checkpoints with ranges that have not yet recorded a concrete server position.
    /// Incomplete Now anchoring returns
    /// [`crate::error::status_codes::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH`];
    /// confirm more scoped polls before attempting handoff.
    ///
    /// # Durability
    ///
    /// The caller must confirm all pages represented by `checkpoint` before using
    /// these results and atomically fence the parent before publishing child leases.
    /// This method neither acknowledges pages nor proves checkpoint persistence.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use azure_data_cosmos_driver::{driver::CosmosDriver, models::{ContainerReference, ContinuationToken, FeedRange}};
    /// # fn derive(driver: &CosmosDriver, container: &ContainerReference, confirmed: &ContinuationToken,
    /// # parent: &FeedRange, children: &[FeedRange]) -> azure_data_cosmos_driver::error::Result<()> {
    /// let checkpoints = driver.derive_change_feed_checkpoints(
    ///     container, confirmed, parent, children, false,
    /// )?;
    /// assert_eq!(checkpoints.len(), children.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn derive_change_feed_checkpoints(
        &self,
        container: &ContainerReference,
        checkpoint: &ContinuationToken,
        parent_range: &FeedRange,
        child_ranges: &[FeedRange],
        full_fidelity: bool,
    ) -> Result<Vec<ContinuationToken>> {
        if self.account().endpoint() != container.account().endpoint() {
            return Err(invalid(
                "retained container account does not match the driver",
            ));
        }
        derive_checkpoints(
            container,
            checkpoint,
            parent_range,
            child_ranges,
            full_fidelity,
        )
    }
}

fn invalid(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(CLIENT_BAD_REQUEST)
        .with_message(message)
        .build()
}

fn operation(
    container: &ContainerReference,
    range: &FeedRange,
    full_fidelity: bool,
) -> CosmosOperation {
    if full_fidelity {
        CosmosOperation::change_feed_all_versions_and_deletes(
            container.clone(),
            Some(range.clone()),
        )
    } else {
        CosmosOperation::change_feed(container.clone(), Some(range.clone()))
    }
}

fn derive_checkpoints(
    container: &ContainerReference,
    checkpoint: &ContinuationToken,
    parent_range: &FeedRange,
    child_ranges: &[FeedRange],
    full_fidelity: bool,
) -> Result<Vec<ContinuationToken>> {
    let parent_operation = operation(container, parent_range, full_fidelity);
    let ResolvedToken::ClientV1(state) = checkpoint.resolve()? else {
        return Err(invalid(
            "child derivation requires a bound driver checkpoint, not a server token",
        ));
    };
    if !state.has_change_feed_binding() {
        return Err(invalid("legacy checkpoint has no account/scope binding; resume and confirm a new checkpoint first"));
    }
    state.is_valid_for_operation(&parent_operation)?;
    validate_children(parent_range, child_ranges)?;
    let PipelineNodeState::UnorderedMerge {
        active_tokens,
        start_from,
        next_epk,
    } = state.into_root_node_state()
    else {
        return Err(invalid(
            "child derivation supports only unbuffered EPK change-feed snapshots",
        ));
    };
    let mut saved = Vec::with_capacity(active_tokens.len());
    let mut last_max = parent_range.min_inclusive().clone();
    let mut complete = true;
    for token in active_tokens {
        let range = FeedRange::new(
            EffectivePartitionKey::try_from(token.min_epk.as_str())?,
            EffectivePartitionKey::try_from(token.max_epk.as_str())?,
        )?;
        if range.min_inclusive() >= range.max_exclusive()
            || range.min_inclusive() < &last_max
            || range.max_exclusive() > parent_range.max_exclusive()
            || token.server_continuation.is_empty()
        {
            return Err(invalid("checkpoint ranges must be ordered, disjoint, nonempty, and inside the parent scope"));
        }
        complete &= range.min_inclusive() == &last_max;
        last_max = range.max_exclusive().clone();
        saved.push((range, token.server_continuation));
    }
    complete &= &last_max == parent_range.max_exclusive();
    if (full_fidelity || start_from == Some(ChangeFeedStartFrom::Now)) && !complete {
        return Err(CosmosError::builder()
            .with_status(crate::error::status_codes::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH)
            .with_message("Now checkpoint is not anchored for every parent range")
            .build());
    }
    let next = next_epk
        .as_deref()
        .map(EffectivePartitionKey::try_from)
        .transpose()?;
    if next.as_ref().is_some_and(|next| {
        next < parent_range.min_inclusive() || next >= parent_range.max_exclusive()
    }) {
        return Err(invalid(
            "checkpoint poll cursor is outside the parent scope",
        ));
    }
    child_ranges
        .iter()
        .map(|child| {
            let active_tokens = saved
                .iter()
                .filter_map(|(range, continuation)| {
                    intersect_feed_ranges(range, child).map(|slice| RangedToken {
                        min_epk: slice.min_inclusive().to_hex(),
                        max_epk: slice.max_exclusive().to_hex(),
                        server_continuation: continuation.clone(),
                    })
                })
                .collect();
            let child_next = next
                .as_ref()
                .filter(|next| *next >= child.min_inclusive() && *next < child.max_exclusive())
                .unwrap_or(child.min_inclusive());
            ContinuationToken::encode_v1(
                &operation(container, child, full_fidelity),
                &PipelineNodeState::UnorderedMerge {
                    active_tokens,
                    start_from: start_from.clone(),
                    next_epk: Some(child_next.to_hex()),
                },
            )
        })
        .collect()
}

fn validate_children(parent: &FeedRange, children: &[FeedRange]) -> Result<()> {
    if parent.is_logical_partition()
        || parent.min_inclusive() >= parent.max_exclusive()
        || children.is_empty()
    {
        return Err(invalid(
            "parent and children must be nonempty explicit EPK ranges",
        ));
    }
    let mut sorted: Vec<_> = children.iter().collect();
    sorted.sort_by(|left, right| left.min_inclusive().cmp(right.min_inclusive()));
    if sorted
        .iter()
        .any(|child| child.is_logical_partition() || child.min_inclusive() >= child.max_exclusive())
    {
        return Err(invalid("children must be nonempty explicit EPK ranges"));
    }
    validate_exact_coverage(parent, sorted.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        diagnostics::DiagnosticsContextBuilder,
        driver::dataflow::{
            mocks::{MockTopologyProvider, NoopTopologyProvider},
            planner::build_unordered_merge,
            PartitionRoutingRefresh, PipelineContext, RequestExecutor, RequestTarget,
            ResolvedRange,
        },
        models::{
            AccountReference, ActivityId, ContainerProperties, CosmosResponse,
            CosmosResponseHeaders, CosmosStatus, SystemProperties,
        },
        options::DiagnosticsOptions,
    };
    use azure_core::http::{Etag, StatusCode};
    use futures::future::BoxFuture;
    use std::{borrow::Cow, sync::Arc};

    fn container(endpoint: &str, rid: &'static str) -> ContainerReference {
        ContainerReference::new(
            AccountReference::with_master_key(url::Url::parse(endpoint).unwrap(), "dGVzdA=="),
            "db",
            "db_rid",
            "coll",
            rid,
            &ContainerProperties {
                id: Cow::Borrowed("coll"),
                partition_key: serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap(),
                system_properties: SystemProperties::default(),
            },
        )
    }

    fn source() -> ContainerReference {
        container("https://test.documents.azure.com/", "coll_rid")
    }

    fn range(min: &str, max: &str) -> FeedRange {
        FeedRange::new(
            EffectivePartitionKey::try_from(min).unwrap(),
            EffectivePartitionKey::try_from(max).unwrap(),
        )
        .unwrap()
    }

    fn snapshot(start_from: Option<ChangeFeedStartFrom>) -> PipelineNodeState {
        PipelineNodeState::UnorderedMerge {
            active_tokens: vec![
                RangedToken {
                    min_epk: "".into(),
                    max_epk: "80".into(),
                    server_continuation: "left-confirmed".into(),
                },
                RangedToken {
                    min_epk: "80".into(),
                    max_epk: "FF".into(),
                    server_continuation: "right-confirmed".into(),
                },
            ],
            start_from,
            next_epk: Some("80".into()),
        }
    }

    fn checkpoint(state: &PipelineNodeState, full_fidelity: bool) -> ContinuationToken {
        ContinuationToken::encode_v1(
            &operation(&source(), &FeedRange::full(), full_fidelity),
            state,
        )
        .unwrap()
    }

    #[test]
    fn rejects_source_scope_mode_and_recreation_mismatch() {
        let token = checkpoint(&snapshot(Some(ChangeFeedStartFrom::Beginning)), false);
        for (container, scope, mode) in [
            (
                container("https://other.documents.azure.com/", "coll_rid"),
                FeedRange::full(),
                false,
            ),
            (
                container("https://test.documents.azure.com/", "recreated_rid"),
                FeedRange::full(),
                false,
            ),
            (source(), range("", "80"), false),
            (source(), FeedRange::full(), true),
        ] {
            assert!(derive_checkpoints(
                &container,
                &token,
                &scope,
                std::slice::from_ref(&scope),
                mode
            )
            .is_err());
        }
    }

    #[test]
    fn rejects_gaps_overlaps_empty_and_outside_children() {
        let token = checkpoint(&snapshot(None), false);
        for children in [
            vec![],
            vec![range("", "40"), range("80", "FF")],
            vec![range("", "80"), range("40", "FF")],
            vec![range("", "80")],
            vec![range("", ""), FeedRange::full()],
        ] {
            assert!(
                derive_checkpoints(&source(), &token, &FeedRange::full(), &children, false)
                    .is_err()
            );
        }
        assert!(derive_checkpoints(
            &source(),
            &token,
            &range("40", "FF"),
            &[FeedRange::full()],
            false
        )
        .is_err());
    }

    #[test]
    fn rejects_unbound_opaque_and_buffered_snapshots() {
        let mut state = snapshot(None);
        state = PipelineNodeState::SkipTake {
            remaining_skip: 0,
            remaining_take: Some(1),
            child: Box::new(state),
        };
        assert!(derive_checkpoints(
            &source(),
            &checkpoint(&state, false),
            &FeedRange::full(),
            &[FeedRange::full()],
            false
        )
        .is_err());
        let raw = ContinuationToken::from_string("server-etag".into());
        assert!(derive_checkpoints(
            &source(),
            &raw,
            &FeedRange::full(),
            &[FeedRange::full()],
            false
        )
        .is_err());
        let json = br#"{"op":"ChangeFeed","rid":"coll_rid","root":{"kind":"unordered_merge","active_tokens":[]}}"#;
        use base64::Engine;
        let legacy = ContinuationToken::from_string(format!(
            "c1.{}",
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(json)
        ));
        assert!(derive_checkpoints(
            &source(),
            &legacy,
            &FeedRange::full(),
            &[FeedRange::full()],
            false
        )
        .is_err());
    }

    #[test]
    fn now_requires_complete_positions_but_beginning_retains_unpolled_ranges() {
        let incomplete = PipelineNodeState::UnorderedMerge {
            active_tokens: vec![RangedToken {
                min_epk: "".into(),
                max_epk: "80".into(),
                server_continuation: "left-confirmed".into(),
            }],
            start_from: Some(ChangeFeedStartFrom::Now),
            next_epk: Some("80".into()),
        };
        let error = derive_checkpoints(
            &source(),
            &checkpoint(&incomplete, false),
            &FeedRange::full(),
            &[FeedRange::full()],
            false,
        )
        .unwrap_err();
        assert_eq!(
            error.status(),
            crate::error::status_codes::CLIENT_CONTINUATION_TOKEN_SHAPE_MISMATCH
        );
        for full_fidelity in [false, true] {
            let complete = checkpoint(&snapshot(Some(ChangeFeedStartFrom::Now)), full_fidelity);
            assert!(derive_checkpoints(
                &source(),
                &complete,
                &FeedRange::full(),
                &[range("", "40"), range("40", "FF")],
                full_fidelity
            )
            .is_ok());
        }
    }

    // The mock service owns event positions; the caller never interprets an ETag.
    struct EventLog;
    impl RequestExecutor for EventLog {
        fn execute_request<'a>(
            &'a mut self,
            operation: &'a CosmosOperation,
            target: RequestTarget,
            _refresh: PartitionRoutingRefresh,
            continuation: Option<String>,
        ) -> BoxFuture<'a, Result<CosmosResponse>> {
            Box::pin(async move {
                let RequestTarget::EffectivePartitionKeyRange {
                    range: slice,
                    partition_key_range,
                    ..
                } = target
                else {
                    panic!("EPK request required");
                };
                let scope = slice.unwrap_or(partition_key_range);
                let after = match continuation.as_deref() {
                    Some("left-confirmed") => 2,
                    Some("right-confirmed") => 1,
                    None => {
                        assert_eq!(
                            operation.change_feed_start(),
                            Some(&ChangeFeedStartFrom::Beginning)
                        );
                        0
                    }
                    other => panic!("unexpected server position {other:?}"),
                };
                let events = [
                    ("20", 1, "a"),
                    ("60", 2, "b"),
                    ("20", 3, "c"),
                    ("60", 4, "d"),
                    ("A0", 1, "e"),
                    ("A0", 2, "f"),
                ];
                let ids: Vec<_> = events
                    .iter()
                    .filter(|(epk, seq, _)| {
                        let epk = EffectivePartitionKey::try_from(*epk).unwrap();
                        epk >= *scope.min_inclusive()
                            && epk < *scope.max_exclusive()
                            && *seq > after
                    })
                    .map(|(_, _, id)| *id)
                    .collect();
                let mut headers = CosmosResponseHeaders::new();
                headers.etag = Some(Etag::from("next".to_owned()));
                let mut diagnostics = DiagnosticsContextBuilder::new(
                    ActivityId::new_uuid(),
                    Arc::new(DiagnosticsOptions::default()),
                );
                diagnostics.set_operation_status(StatusCode::Ok, None);
                Ok(CosmosResponse::new(
                    serde_json::to_vec(&ids).unwrap(),
                    headers,
                    CosmosStatus::new(StatusCode::Ok),
                    Arc::new(diagnostics.complete()),
                ))
            })
        }
    }

    async fn read_scope(token: &ContinuationToken, scope: &FeedRange, merged: bool) -> Vec<String> {
        let op = Arc::new(operation(&source(), scope, false));
        let ResolvedToken::ClientV1(state) = token.resolve().unwrap() else {
            panic!("client token");
        };
        state.is_valid_for_operation(&op).unwrap();
        let physical = if merged {
            vec![FeedRange::full()]
        } else {
            vec![range("", "40"), range("40", "80"), range("80", "FF")]
        };
        let mut topology = MockTopologyProvider::new(vec![Ok(physical
            .iter()
            .filter(|range| intersect_feed_ranges(range, scope).is_some())
            .enumerate()
            .map(|(id, range)| ResolvedRange {
                partition_key_range_id: id.to_string(),
                parents: vec![],
                range: range.clone(),
            })
            .collect())]);
        let mut plan = build_unordered_merge(
            scope,
            &mut topology,
            &op,
            Some(state.into_root_node_state()),
        )
        .await
        .unwrap();
        let width = plan.fan_out_width();
        let mut executor = EventLog;
        let mut topology = NoopTopologyProvider;
        let mut ids = vec![];
        for _ in 0..width {
            let page = plan
                .next_page(&mut PipelineContext::new(
                    &mut executor,
                    Some(&mut topology),
                ))
                .await
                .unwrap()
                .unwrap();
            ids.extend(serde_json::from_slice::<Vec<String>>(page.body_bytes()).unwrap());
        }
        ids
    }

    #[tokio::test]
    async fn progressed_split_children_union_has_no_gaps_or_duplicates_after_merge() {
        let token = checkpoint(&snapshot(Some(ChangeFeedStartFrom::Beginning)), false);
        // One child crosses a saved-token boundary; input order intentionally differs from EPK order.
        let children = vec![range("40", "FF"), range("", "40")];
        let tokens =
            derive_checkpoints(&source(), &token, &FeedRange::full(), &children, false).unwrap();
        for merged in [false, true] {
            let mut ids = vec![];
            for (scope, token) in children.iter().zip(&tokens) {
                ids.extend(read_scope(token, scope, merged).await);
            }
            ids.sort();
            assert_eq!(ids, ["c", "d", "f"]);
            let mut parent = read_scope(&token, &FeedRange::full(), merged).await;
            parent.sort();
            assert_eq!(ids, parent);
        }
    }

    #[tokio::test]
    async fn unpolled_child_reapplies_original_boundary_without_copying_other_progress() {
        let mut state = snapshot(Some(ChangeFeedStartFrom::Beginning));
        let PipelineNodeState::UnorderedMerge { active_tokens, .. } = &mut state else {
            unreachable!();
        };
        active_tokens.pop();
        let token = checkpoint(&state, false);
        let children = vec![range("", "80"), range("80", "FF")];
        let tokens =
            derive_checkpoints(&source(), &token, &FeedRange::full(), &children, false).unwrap();
        assert_eq!(read_scope(&tokens[1], &children[1], true).await, ["e", "f"]);
        let mut left = read_scope(&tokens[0], &children[0], false).await;
        left.sort();
        assert_eq!(left, ["c", "d"]);
    }

    #[tokio::test]
    async fn derives_after_confirmed_page_and_preserves_unpolled_pending_work() {
        use crate::driver::dataflow::mocks::{response_with_etag, MockRequestExecutor};
        let op = Arc::new(
            operation(&source(), &FeedRange::full(), false)
                .with_change_feed_start(ChangeFeedStartFrom::Beginning),
        );
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            ResolvedRange {
                partition_key_range_id: "0".into(),
                parents: vec![],
                range: range("", "80"),
            },
            ResolvedRange {
                partition_key_range_id: "1".into(),
                parents: vec![],
                range: range("80", "FF"),
            },
        ])]);
        let mut pipeline = build_unordered_merge(&FeedRange::full(), &mut topology, &op, None)
            .await
            .unwrap();
        let mut executor = MockRequestExecutor::new(vec![Ok(response_with_etag(
            br#"["a","b"]"#,
            "left-confirmed",
        ))]);
        let confirmed = pipeline
            .next_page(&mut PipelineContext::new(&mut executor, None))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(confirmed.body_bytes(), br#"["a","b"]"#);
        let token = ContinuationToken::encode_v1(&op, &pipeline.snapshot_state().unwrap()).unwrap();
        let children = [range("", "40"), range("40", "FF")];
        let checkpoints =
            derive_checkpoints(&source(), &token, &FeedRange::full(), &children, false).unwrap();
        let mut remaining = vec![];
        for (child, checkpoint) in children.iter().zip(&checkpoints) {
            remaining.extend(read_scope(checkpoint, child, false).await);
        }
        remaining.sort();
        assert_eq!(remaining, ["c", "d", "e", "f"]);
    }

    #[tokio::test]
    async fn now_post_poll_snapshot_becomes_durable_only_after_all_ranges_are_anchored() {
        use crate::driver::dataflow::mocks::{response_with_etag, MockRequestExecutor};
        let op = Arc::new(
            operation(&source(), &FeedRange::full(), false)
                .with_change_feed_start(ChangeFeedStartFrom::Now),
        );
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![
            ResolvedRange {
                partition_key_range_id: "0".into(),
                parents: vec![],
                range: range("", "80"),
            },
            ResolvedRange {
                partition_key_range_id: "1".into(),
                parents: vec![],
                range: range("80", "FF"),
            },
        ])]);
        let mut pipeline = build_unordered_merge(&FeedRange::full(), &mut topology, &op, None)
            .await
            .unwrap();
        let mut executor = MockRequestExecutor::new(vec![
            Ok(response_with_etag(b"", "left-confirmed")),
            Ok(response_with_etag(b"", "right-confirmed")),
        ]);
        for anchored in [false, true] {
            pipeline
                .next_page(&mut PipelineContext::new(&mut executor, None))
                .await
                .unwrap()
                .unwrap();
            let token =
                ContinuationToken::encode_v1(&op, &pipeline.snapshot_state().unwrap()).unwrap();
            assert_eq!(
                derive_checkpoints(
                    &source(),
                    &token,
                    &FeedRange::full(),
                    &[FeedRange::full()],
                    false
                )
                .is_ok(),
                anchored
            );
        }
    }
}
