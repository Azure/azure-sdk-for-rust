// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`PipelineNode`] trait and [`PageResult`] returned from each pull.

use async_trait::async_trait;

use crate::models::{CosmosResponse, FeedRange};

use super::{context::PipelineContext, snapshot::PipelineNodeState};

/// Result of a single `next_page` call on a pipeline node.
///
/// The `Page` variant contains a large `CosmosResponse` inline, but boxing it
/// would add a heap allocation on every page fetch — the hot path. The `SplitRequired`
/// variant is rare (only on partition splits), so the size difference is acceptable.
#[must_use = "a PageResult carries the next page, drain signal, or a split request that the caller must act on"]
#[allow(clippy::large_enum_variant)]
pub(crate) enum PageResult {
    /// A page of results was produced.
    ///
    /// `is_terminal` is `true` when this node has no more pages to emit
    /// after this one — set by leaf nodes when the server returned no
    /// continuation token, and propagated by intermediate nodes when their
    /// last child has emitted its terminal page. Parents use this to evict
    /// drained children eagerly so that snapshots of the pipeline do not
    /// include children that are already done.
    Page {
        response: CosmosResponse,
        is_terminal: bool,
    },
    /// This node has no more pages to emit.
    Drained,
    /// This node's EPK range has split and needs to be replaced by new child nodes.
    ///
    /// It is the parent intermediate node's responsibility to splice
    /// `replacements` into its children list (in place of the child that
    /// emitted this result) and re-attempt draining from the first replacement.
    /// If a node returns `SplitRequired` to a parent that does not handle
    /// splits (e.g. the pipeline root), the operation fails.
    SplitRequired {
        /// New child nodes covering the sub-ranges of the split partition.
        replacements: SplitReplacements,
    },
}

impl std::fmt::Debug for PageResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageResult::Page { is_terminal, .. } => {
                write!(f, "Page(terminal={is_terminal})")
            }
            PageResult::Drained => f.write_str("Drained"),
            PageResult::SplitRequired { replacements, .. } => {
                write!(f, "SplitRequired({} nodes)", replacements.len())
            }
        }
    }
}

/// Replacement nodes for a split partition, proven at construction to exactly
/// tile the range of the node they replace.
///
/// Exact tiling is a contract requirement of every `SplitRequired` result: a
/// gap silently drops rows and an overlap silently duplicates them, neither of
/// which surfaces as an error downstream. Encoding it here means consumers
/// ([`SequentialDrain`](super::drain::SequentialDrain),
/// [`UnorderedMerge`](super::unordered_merge::UnorderedMerge),
/// [`StreamingOrderedMerge`](super::streaming_ordered_merge::StreamingOrderedMerge))
/// can splice replacements without re-checking, and no production path can
/// construct an unvalidated set — [`Self::untiled`] is `#[cfg(test)]`-only.
///
/// The payload is a private enum so the only ways to obtain a value crate-wide
/// are [`Self::try_tiling`] and the test-only [`Self::untiled`]; the invariant
/// cannot be bypassed by constructing or mutating a variant directly.
pub(crate) struct SplitReplacements(Repr);

enum Repr {
    /// Validated by [`SplitReplacements::try_tiling`], stored in ascending
    /// `min_inclusive` order.
    Tiled(Vec<(FeedRange, Box<dyn PipelineNode>)>),
    /// Range-less test stubs, for consumers that only splice nodes.
    #[cfg(test)]
    Untiled(Vec<Box<dyn PipelineNode>>),
}

impl SplitReplacements {
    /// Accepts `nodes` only if each carries a
    /// [`feed_range`](PipelineNode::feed_range) and, sorted ascending, they
    /// exactly tile `scope` with no gaps or overlaps.
    pub(crate) fn try_tiling(
        scope: &FeedRange,
        nodes: Vec<Box<dyn PipelineNode>>,
    ) -> crate::error::Result<Self> {
        let mut ranged: Vec<(FeedRange, Box<dyn PipelineNode>)> = Vec::with_capacity(nodes.len());
        for node in nodes {
            let range = node
                .feed_range()
                .ok_or_else(|| {
                    split_replacement_invalid("split replacement node has no feed_range")
                })?
                .clone();
            ranged.push((range, node));
        }
        ranged.sort_by(|a, b| a.0.min_inclusive().cmp(b.0.min_inclusive()));
        validate_exact_coverage(scope, ranged.iter().map(|(range, _)| range))?;
        Ok(Self(Repr::Tiled(ranged)))
    }

    /// Test-only escape hatch for mock nodes that carry no feed range and so
    /// have no tiling invariant to uphold.
    #[cfg(test)]
    pub(crate) fn untiled(nodes: Vec<Box<dyn PipelineNode>>) -> Self {
        Self(Repr::Untiled(nodes))
    }

    pub(crate) fn len(&self) -> usize {
        match &self.0 {
            Repr::Tiled(ranged) => ranged.len(),
            #[cfg(test)]
            Repr::Untiled(nodes) => nodes.len(),
        }
    }

    /// Consumes the set, yielding nodes in ascending range order.
    pub(crate) fn into_nodes(self) -> Vec<Box<dyn PipelineNode>> {
        match self.0 {
            Repr::Tiled(ranged) => ranged.into_iter().map(|(_, node)| node).collect(),
            #[cfg(test)]
            Repr::Untiled(nodes) => nodes,
        }
    }

    /// Consumes the set, yielding `(range, node)` pairs in ascending range
    /// order. Errors only for [`Self::Untiled`] stubs lacking a feed range;
    /// a [`Self::Tiled`] set already proved every range present.
    pub(crate) fn into_ranged(
        self,
    ) -> crate::error::Result<Vec<(FeedRange, Box<dyn PipelineNode>)>> {
        match self.0 {
            Repr::Tiled(ranged) => Ok(ranged),
            #[cfg(test)]
            Repr::Untiled(nodes) => nodes
                .into_iter()
                .map(|node| {
                    let range = node
                        .feed_range()
                        .ok_or_else(|| {
                            split_replacement_invalid("split replacement node has no feed_range")
                        })?
                        .clone();
                    Ok((range, node))
                })
                .collect(),
        }
    }
}

/// Returns `Ok(())` if `ranges` (yielded in ascending `min_inclusive` order,
/// each already clipped to `scope`) exactly tiles `scope` end-to-end with no
/// gaps or overlaps. Backs [`SplitReplacements::try_tiling`] and the streaming
/// ORDER BY planner's split/merge resume path.
pub(crate) fn validate_exact_coverage<'a>(
    scope: &FeedRange,
    ranges: impl Iterator<Item = &'a FeedRange>,
) -> crate::error::Result<()> {
    let mut cursor = scope.min_inclusive().clone();
    let mut saw_any = false;
    for owned in ranges {
        saw_any = true;
        if owned.min_inclusive() != &cursor {
            return Err(split_replacement_invalid(format!(
                "replacement range [{}, {}) does not start at the expected cursor {}",
                owned.min_inclusive().to_hex(),
                owned.max_exclusive().to_hex(),
                cursor.to_hex(),
            )));
        }
        cursor = owned.max_exclusive().clone();
    }
    if !saw_any {
        return Err(split_replacement_invalid(
            "topology resolution produced no replacement ranges",
        ));
    }
    if &cursor != scope.max_exclusive() {
        return Err(split_replacement_invalid(format!(
            "replacement ranges cover up to {} but the prior range extended to {}",
            cursor.to_hex(),
            scope.max_exclusive().to_hex(),
        )));
    }
    Ok(())
}

pub(crate) fn split_replacement_invalid(
    message: impl Into<std::borrow::Cow<'static, str>>,
) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID)
        .with_message(message)
        .build()
}

/// A dataflow node that emits pages and may own child nodes.
///
/// Each `next_page` call boxes a future via `async_trait`; the per-page
/// allocation is negligible compared to the multi-millisecond network I/O
/// of a Cosmos DB request.
#[async_trait]
pub(crate) trait PipelineNode: Send + std::any::Any {
    /// Emits the next page of results, signals drain completion, or requests a split.
    async fn next_page(
        &mut self,
        context: &mut PipelineContext<'_>,
    ) -> crate::error::Result<PageResult>;

    /// Consumes this node and returns its children as a `Vec`.
    ///
    /// Used by tests to inspect the dataflow tree's shape after planning.
    #[cfg(test)]
    fn into_children(self) -> Vec<Box<dyn PipelineNode>>;

    /// Snapshots this node's state for continuation-token serialization.
    ///
    /// Returns an error if a dataflow invariant is violated (e.g. an
    /// intermediate node observes a child without a `feed_range`). Such
    /// errors should be impossible in production code paths; surfacing
    /// them as `Err` rather than encoding them into the payload prevents
    /// a malformed snapshot from being serialized and later mis-parsed.
    fn snapshot_state(&self) -> crate::error::Result<PipelineNodeState>;

    /// Returns `true` if it's possible for this node to require a topology change (split or merge) in the future.
    ///
    /// A node where `topology_can_change()` is true cannot be the root of the pipeline because there is no parent to perform the necessary split/merge replacement if a topology change occurs.
    fn topology_can_change(&self) -> bool;

    /// Returns the EPK range this node currently targets, if known.
    ///
    /// Used by intermediate nodes (e.g. [`super::SequentialDrain`]) to record
    /// the current cursor position when snapshotting, without needing to know
    /// the concrete type of their children. Defaults to `None`.
    ///
    /// # Invariant
    ///
    /// Every node in the dataflow tree is responsible for some contiguous EPK
    /// sub-range of the container key space. Intermediate nodes that drain
    /// children in EPK order (such as [`super::SequentialDrain`]) may use the
    /// front child's `feed_range()` as their own cursor; intermediates that
    /// combine results across ranges (e.g. a future k-way merge for streaming
    /// `ORDER BY`) are responsible for snapshotting whatever cursor
    /// representation makes sense for their ordering semantics.
    fn feed_range(&self) -> Option<&FeedRange> {
        None
    }

    /// Returns the number of leaf request nodes this node fans out to.
    ///
    /// Leaf nodes return `1` (or `0` for a no-op leaf that issues no request).
    /// Intermediate ("parent") nodes return the sum of their children's widths,
    /// so a pipeline of any shape reports its total leaf fan-out by recursion.
    /// The planner uses this to enforce a maximum fan-out on fresh
    /// cross-partition plans; because every node contributes its own accounting,
    /// the check scales to any future pipeline shape.
    ///
    /// This is a required method (rather than defaulted) so a newly added parent
    /// node type cannot silently inherit an incorrect leaf-count of `1` — every
    /// implementer must state its fan-out explicitly.
    fn fan_out_width(&self) -> usize;
}

#[cfg(test)]
impl dyn PipelineNode {
    /// Downcasts this node to a concrete type.
    pub(crate) fn downcast_ref<T: PipelineNode>(&self) -> Option<&T> {
        (self as &dyn std::any::Any).downcast_ref::<T>()
    }

    /// Downcasts this node to a concrete type.
    pub(crate) fn downcast<T: PipelineNode>(self: Box<Self>) -> Option<Box<T>> {
        (self as Box<dyn std::any::Any>).downcast::<T>().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        driver::dataflow::mocks::MockLeaf, models::effective_partition_key::EffectivePartitionKey,
    };

    fn range(min: &str, max: &str) -> FeedRange {
        FeedRange::new(
            EffectivePartitionKey::from(min),
            EffectivePartitionKey::from(max),
        )
        .unwrap()
    }

    fn leaf(min: &str, max: &str) -> Box<dyn PipelineNode> {
        Box::new(MockLeaf::with_pages(vec![]).with_feed_range(range(min, max)))
    }

    /// The invariant `SplitReplacements` exists to uphold: replacements that
    /// exactly tile the split scope are accepted.
    #[test]
    fn try_tiling_accepts_exact_coverage() {
        let replacements =
            SplitReplacements::try_tiling(&range("", "80"), vec![leaf("", "40"), leaf("40", "80")])
                .expect("exactly tiling replacements are accepted");
        assert_eq!(replacements.len(), 2);
    }

    /// Input order is not part of the contract — the type sorts before
    /// validating so an out-of-order producer still yields ascending nodes.
    #[test]
    fn try_tiling_sorts_unordered_input() {
        let replacements =
            SplitReplacements::try_tiling(&range("", "80"), vec![leaf("40", "80"), leaf("", "40")])
                .expect("out-of-order replacements are sorted, then accepted");
        let ranged = replacements.into_ranged().unwrap();
        assert_eq!(
            ranged
                .iter()
                .map(|(r, _)| r.min_inclusive().to_hex())
                .collect::<Vec<_>>(),
            vec!["".to_owned(), "40".to_owned()],
        );
    }

    /// A gap would silently drop every row in the uncovered sub-range.
    #[test]
    fn try_tiling_rejects_gap() {
        let err =
            SplitReplacements::try_tiling(&range("", "80"), vec![leaf("", "20"), leaf("40", "80")])
                .map(|_| ())
                .expect_err("a gap between replacements is rejected");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID),
        );
    }

    /// An overlap would silently emit the overlapping rows twice.
    #[test]
    fn try_tiling_rejects_overlap() {
        let err =
            SplitReplacements::try_tiling(&range("", "80"), vec![leaf("", "60"), leaf("40", "80")])
                .map(|_| ())
                .expect_err("overlapping replacements are rejected");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID),
        );
    }

    /// Coverage that stops short of the scope's upper bound drops the tail.
    #[test]
    fn try_tiling_rejects_short_coverage() {
        let err =
            SplitReplacements::try_tiling(&range("", "80"), vec![leaf("", "40"), leaf("40", "60")])
                .map(|_| ())
                .expect_err("replacements that stop short of the scope are rejected");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID),
        );
    }

    #[test]
    fn try_tiling_rejects_empty_set() {
        let err = SplitReplacements::try_tiling(&range("", "80"), vec![])
            .map(|_| ())
            .expect_err("an empty replacement set covers nothing and is rejected");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID),
        );
    }

    /// The service may report a boundary padded to the partition key
    /// definition's full width while the scope carries its trimmed equivalent
    /// (or vice versa). `Ord` treats those as the same boundary, so coverage
    /// must accept them — comparing raw bytes would reject a valid tiling.
    #[test]
    fn validate_exact_coverage_accepts_zero_padded_bounds() {
        validate_exact_coverage(
            &range("", "FF"),
            [range("", "8000"), range("80", "FF")].iter(),
        )
        .expect("`8000` and `80` name the same boundary");

        validate_exact_coverage(&range("0000", "FF"), [range("", "FF")].iter())
            .expect("a padded scope start matches an unpadded range start");
    }

    /// Mirrors .NET's `isRoutingMapFullySpecified` parameterization
    /// (azure-cosmos-dotnet-v3#5260): the backend may report a tiling at mixed
    /// widths or fully padded, and both must resolve identically.
    #[test]
    fn validate_exact_coverage_is_width_agnostic() {
        let scope = range("", "FF");
        let mixed = [range("", "3F00"), range("3F", "7F"), range("7F0000", "FF")];
        let padded = [
            range("", "3F000000"),
            range("3F000000", "7F00"),
            range("7F", "FF"),
        ];
        validate_exact_coverage(&scope, mixed.iter()).expect("mixed-width tiling covers the scope");
        validate_exact_coverage(&scope, padded.iter())
            .expect("the same tiling, padded, must resolve identically");

        // A genuine gap is still caught at either width.
        validate_exact_coverage(&scope, [range("", "3F"), range("40", "FF")].iter())
            .expect_err("a real gap between 3F and 40 must still be rejected");
    }

    /// Coverage is unverifiable without a range, so a range-less node is
    /// rejected rather than assumed to fit.
    #[test]
    fn try_tiling_rejects_node_without_feed_range() {
        let err = SplitReplacements::try_tiling(
            &range("", "80"),
            vec![Box::new(MockLeaf::with_pages(vec![]))],
        )
        .map(|_| ())
        .expect_err("a replacement without a feed_range is rejected");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_STREAMING_MERGE_SPLIT_REPLACEMENT_INVALID),
        );
    }
}
