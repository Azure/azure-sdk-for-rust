// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Helpers for merging and managing session tokens across feed ranges.

use crate::feed_range::FeedRange;
use azure_core::error::ErrorKind;
use azure_data_cosmos_driver::models::{SessionToken, SessionTokenSegment};

/// Returns `true` if the session token string contains multiple comma-separated segments.
fn is_compound(token: &str) -> bool {
    token.contains(',')
}

/// Merges two non-compound session token strings that cover the same feed range.
///
/// When the tokens have different partition key range IDs, keeps the ID from
/// the token with the higher global LSN (the more recent topology).
fn merge_tokens_same_range(token1: &str, token2: &str) -> azure_core::Result<String> {
    let mut seg1: SessionTokenSegment = token1.parse()?;
    let seg2: SessionTokenSegment = token2.parse()?;

    if seg1.pk_range_id() != seg2.pk_range_id()
        && seg2.is_as_recent_as(&seg1)
        && !seg1.is_as_recent_as(&seg2)
    {
        seg1.set_pk_range_id(seg2.pk_range_id());
    }

    seg1.merge_value(&seg2);
    Ok(seg1.to_string())
}

/// Phase 1: merge session tokens that share the exact same feed range (non-compound only).
fn merge_same_ranges(overlapping: &mut Vec<(FeedRange, String)>) -> azure_core::Result<()> {
    let mut i = 0;
    while i < overlapping.len() {
        let mut j = i + 1;
        while j < overlapping.len() {
            if !is_compound(&overlapping[i].1)
                && !is_compound(&overlapping[j].1)
                && overlapping[i].0 == overlapping[j].0
            {
                let token1 = overlapping[i].1.clone();
                let token2 = overlapping[j].1.clone();
                let merged = merge_tokens_same_range(&token1, &token2)?;
                let feed_range = overlapping[i].0.clone();
                overlapping.remove(j);
                overlapping.remove(i);
                overlapping.push((feed_range, merged));
                i = 0;
                j = 1;
            } else {
                j += 1;
                if j >= overlapping.len() {
                    i += 1;
                    j = i + 1;
                }
            }
        }
        if j >= overlapping.len() {
            i += 1;
        }
    }
    Ok(())
}

/// Describes the action to take after analyzing parent/child subset relationships.
enum MergeAction {
    /// No parent/child match found; do nothing.
    None,
    /// Parent is newer — remove children, keep parent at index 0.
    KeepParent { children_indices: Vec<usize> },
    /// Children are newer (split) — remove children and parent, add compound.
    ReplaceParent {
        children_indices: Vec<usize>,
        compound: (FeedRange, String),
    },
    /// Mixed — remove children, keep parent, add compound of all tokens.
    AddCompound {
        children_indices: Vec<usize>,
        compound: (FeedRange, String),
    },
}

/// Phase 2: detect partition split/merge scenarios by analyzing parent/child feed range
/// relationships.
///
/// - **Split**: children's combined range equals parent and children have higher LSN →
///   parent is replaced by compound of children.
/// - **Merge**: parent has higher LSN than children → children are removed.
/// - **Mixed**: some children are newer, some aren't → all tokens are kept as compound.
///
/// The input is sorted by range size descending so that parents are always processed
/// before their children, regardless of the caller's input order.
fn merge_ranges_with_subsets(
    mut overlapping: Vec<(FeedRange, String)>,
) -> azure_core::Result<Vec<(FeedRange, String)>> {
    // Sort by range size descending: larger ranges (parents) first.
    // Primary: max_exclusive descending, secondary: min_inclusive ascending.
    overlapping.sort_by(|(a, _), (b, _)| {
        b.max_exclusive
            .cmp(&a.max_exclusive)
            .then(a.min_inclusive.cmp(&b.min_inclusive))
    });

    let mut processed = Vec::new();

    while !overlapping.is_empty() {
        let range_cmp = overlapping[0].0.clone();
        let token_cmp = overlapping[0].1.clone();

        if !is_compound(&token_cmp) {
            let seg_cmp: SessionTokenSegment = token_cmp.parse()?;

            // Find non-compound subsets of the current range
            let subsets: Vec<(usize, FeedRange, String)> = overlapping
                .iter()
                .enumerate()
                .skip(1)
                .filter(|(_, (fr, tok))| !is_compound(tok) && fr.is_subset_of(&range_cmp))
                .map(|(i, (fr, tok))| (i, fr.clone(), tok.clone()))
                .collect();

            if subsets.len() == 1 {
                // Single subset: only remove child if parent has strictly higher LSN
                let child_seg: SessionTokenSegment = subsets[0].2.parse()?;
                if seg_cmp.is_as_recent_as(&child_seg) && !child_seg.is_as_recent_as(&seg_cmp) {
                    overlapping.remove(subsets[0].0);
                }
            } else if subsets.len() > 1 {
                let action = analyze_subsets(&range_cmp, &seg_cmp, &token_cmp, &subsets)?;

                // Apply the action — caller manages all mutations consistently
                match action {
                    MergeAction::None => {}
                    MergeAction::KeepParent { children_indices } => {
                        remove_indices(&mut overlapping, &children_indices);
                    }
                    MergeAction::ReplaceParent {
                        children_indices,
                        compound,
                    } => {
                        remove_indices(&mut overlapping, &children_indices);
                        overlapping.remove(0); // remove parent
                        overlapping.push(compound);
                        // Restart: the compound is now at the end and will be processed later
                        continue;
                    }
                    MergeAction::AddCompound {
                        children_indices,
                        compound,
                    } => {
                        remove_indices(&mut overlapping, &children_indices);
                        overlapping.push(compound);
                        // Parent stays at index 0, falls through to processed.push below
                    }
                }
            }
        }

        processed.push(overlapping.remove(0));
    }

    Ok(processed)
}

/// Removes elements at the given indices from a vec, handling index shifts correctly.
fn remove_indices<T>(vec: &mut Vec<T>, indices: &[usize]) {
    let mut sorted = indices.to_vec();
    sorted.sort_unstable();
    sorted.reverse();
    for idx in sorted {
        vec.remove(idx);
    }
}

/// Analyzes multiple child subsets to determine if they combine to equal the parent range,
/// and returns the appropriate merge action.
///
/// Subsets are sorted by `min_inclusive` ascending before the merge loop to ensure
/// adjacent children are processed in order, regardless of the caller's input ordering.
fn analyze_subsets(
    parent_range: &FeedRange,
    parent_seg: &SessionTokenSegment,
    parent_token: &str,
    subsets: &[(usize, FeedRange, String)],
) -> azure_core::Result<MergeAction> {
    // Sort subsets by min_inclusive so adjacent children are always in order
    let mut sorted_subsets = subsets.to_vec();
    sorted_subsets.sort_by(|a, b| a.1.min_inclusive.cmp(&b.1.min_inclusive));

    for start_idx in 0..sorted_subsets.len() {
        let mut merged_range = sorted_subsets[start_idx].1.clone();
        let mut tokens = vec![sorted_subsets[start_idx].2.clone()];
        let mut indices = vec![sorted_subsets[start_idx].0];

        for (k, (idx, fr, tok)) in sorted_subsets.iter().enumerate() {
            if k == start_idx {
                continue;
            }
            if merged_range.can_merge(fr) {
                merged_range = merged_range.merge_with(fr);
                tokens.push(tok.clone());
                indices.push(*idx);
            }
            if *parent_range == merged_range {
                // Children combine to equal the parent — determine action
                let mut children_more_updated = true;
                let mut parent_more_updated = true;
                for t in &tokens {
                    let child_seg: SessionTokenSegment = t.parse()?;
                    if parent_seg.is_as_recent_as(&child_seg)
                        && !child_seg.is_as_recent_as(parent_seg)
                    {
                        children_more_updated = false;
                    } else {
                        parent_more_updated = false;
                    }
                }

                if children_more_updated {
                    // Split: children are newer
                    let compound = tokens.join(",");
                    return Ok(MergeAction::ReplaceParent {
                        children_indices: indices,
                        compound: (merged_range, compound),
                    });
                } else if parent_more_updated {
                    // Merge: parent is newer
                    return Ok(MergeAction::KeepParent {
                        children_indices: indices,
                    });
                } else {
                    // Mixed: keep all as compound
                    tokens.push(parent_token.to_owned());
                    let compound = tokens.join(",");
                    return Ok(MergeAction::AddCompound {
                        children_indices: indices,
                        compound: (merged_range, compound),
                    });
                }
            }
        }
    }

    Ok(MergeAction::None)
}

/// Phase 3: split compound session tokens into individual segments.
///
/// Feed range context is intentionally dropped here — from this point on,
/// merging is done purely by partition key range ID (Phase 4).
fn split_compound_tokens(ranges_and_tokens: &[(FeedRange, String)]) -> Vec<String> {
    let mut result = Vec::new();
    for (_, token) in ranges_and_tokens {
        for segment in token.split(',') {
            let trimmed = segment.trim();
            if !trimmed.is_empty() {
                result.push(trimmed.to_owned());
            }
        }
    }
    result
}

/// Phase 4: merge session token segments that share the same partition key range ID.
///
/// Delegates to `SessionToken::merge()` on the driver side so that token format
/// details stay encapsulated.
fn merge_tokens_by_partition(tokens: Vec<String>) -> azure_core::Result<SessionToken> {
    let mut result = SessionToken::new(tokens[0].clone());
    for t in &tokens[1..] {
        result = result.merge(&SessionToken::new(t.clone()))?;
    }
    Ok(result)
}

/// Gets the most up-to-date session token from a list of feed range and session token pairs
/// for a specific target feed range.
///
/// This function merges session tokens from feed ranges that overlap with the target,
/// handling partition split and merge scenarios automatically. It is useful when
/// maintaining your own session token cache across multiple clients.
///
/// Session tokens and feed ranges are scoped to a single container. Only pass session
/// tokens and feed ranges obtained from the same container.
///
/// # Arguments
///
/// * `feed_ranges_to_session_tokens` - Pairs of feed ranges and their associated session tokens.
/// * `target_feed_range` - The feed range to get the most up-to-date session token for.
///
/// # Errors
///
/// Returns an error if no input feed ranges overlap with the target feed range,
/// or if any session token string is malformed.
///
/// # Examples
///
/// ```rust,no_run
/// # use azure_data_cosmos::{clients::ContainerClient, FeedRange, SessionToken};
/// # async fn example(container: ContainerClient) -> azure_core::Result<()> {
/// // After read/write operations, capture session tokens from response headers.
/// // When using multiple clients against the same container, merge their tokens
/// // to get the most up-to-date session state.
/// let feed_range = FeedRange::full();
/// let token_a: SessionToken = "0:1#100#3=50".into();
/// let token_b: SessionToken = "0:1#200#3=60".into();
///
/// let latest = container.get_latest_session_token(
///     &[(feed_range.clone(), token_a), (feed_range, token_b)],
///     &FeedRange::full(),
/// )?;
/// // latest == "0:1#200#3=60" (higher LSN values win)
/// # Ok(())
/// # }
/// ```
pub(crate) fn get_latest_session_token(
    feed_ranges_to_session_tokens: &[(FeedRange, SessionToken)],
    target_feed_range: &FeedRange,
) -> azure_core::Result<SessionToken> {
    // Step 1: Filter to overlapping feed ranges
    let mut overlapping: Vec<(FeedRange, String)> = feed_ranges_to_session_tokens
        .iter()
        .filter(|(fr, _)| target_feed_range.overlaps(fr))
        .map(|(fr, st)| (fr.clone(), st.as_str().to_owned()))
        .collect();

    if overlapping.is_empty() {
        return Err(azure_core::Error::with_message(
            ErrorKind::Other,
            "no overlapping feed ranges with the target feed range",
        ));
    }

    // Step 2: Merge session tokens for identical feed ranges
    merge_same_ranges(&mut overlapping)?;

    // Step 3: Handle partition split/merge via subset detection
    let processed = merge_ranges_with_subsets(overlapping)?;

    // Step 4: Split compound tokens into individual segments
    let remaining = split_compound_tokens(&processed);

    if remaining.len() == 1 {
        return Ok(SessionToken::new(remaining.into_iter().next().unwrap()));
    }

    // Step 5: Merge segments with same partition key range ID
    merge_tokens_by_partition(remaining)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::EffectivePartitionKey;

    fn fr(min: &str, max: &str) -> FeedRange {
        FeedRange {
            min_inclusive: EffectivePartitionKey::from(min),
            max_exclusive: EffectivePartitionKey::from(max),
        }
    }

    fn st(s: &str) -> SessionToken {
        SessionToken::new(s.to_owned())
    }

    // === Normal merge scenarios ===

    #[test]
    fn normal_case_same_range_merge() {
        let result = get_latest_session_token(
            &[
                (fr("AA", "BB"), st("0:1#54#3=50")),
                (fr("AA", "BB"), st("0:1#51#3=52")),
            ],
            &fr("AA", "BB"),
        )
        .unwrap();
        assert_eq!(result.as_str(), "0:1#54#3=52");
    }

    #[test]
    fn split_with_both_children() {
        let result = get_latest_session_token(
            &[
                (fr("AA", "DD"), st("0:1#51#3=52")),
                (fr("AA", "BB"), st("1:1#55#3=52")),
                (fr("BB", "DD"), st("2:1#54#3=52")),
            ],
            &fr("AA", "DD"),
        )
        .unwrap();
        // Children are newer → split detected
        let parts: Vec<&str> = result.as_str().split(',').collect();
        assert_eq!(parts.len(), 2);
        assert!(parts.contains(&"1:1#55#3=52"));
        assert!(parts.contains(&"2:1#54#3=52"));
    }

    #[test]
    fn split_with_one_child() {
        let result = get_latest_session_token(
            &[
                (fr("AA", "DD"), st("0:1#51#3=52")),
                (fr("AA", "BB"), st("1:1#55#3=52")),
            ],
            &fr("AA", "DD"),
        )
        .unwrap();
        // Single child can't cover parent → both survive
        let parts: Vec<&str> = result.as_str().split(',').collect();
        assert_eq!(parts.len(), 2);
        assert!(parts.contains(&"0:1#51#3=52"));
        assert!(parts.contains(&"1:1#55#3=52"));
    }

    #[test]
    fn merge_parent_newer() {
        let result = get_latest_session_token(
            &[
                (fr("AA", "DD"), st("0:1#55#3=52")),
                (fr("AA", "BB"), st("1:1#51#3=52")),
            ],
            &fr("AA", "DD"),
        )
        .unwrap();
        // Parent is newer → child removed (merge scenario)
        assert_eq!(result.as_str(), "0:1#55#3=52");
    }

    #[test]
    fn compound_token_passthrough() {
        let result = get_latest_session_token(
            &[
                (fr("AA", "DD"), st("2:1#54#3=52,1:1#55#3=52")),
                (fr("AA", "BB"), st("0:1#51#3=52")),
            ],
            &fr("AA", "BB"),
        )
        .unwrap();
        // Compound passes through, all tokens preserved
        let parts: Vec<&str> = result.as_str().split(',').collect();
        assert_eq!(parts.len(), 3);
        assert!(parts.contains(&"2:1#54#3=52"));
        assert!(parts.contains(&"1:1#55#3=52"));
        assert!(parts.contains(&"0:1#51#3=52"));
    }

    #[test]
    fn several_compound_tokens() {
        let result = get_latest_session_token(
            &[
                (fr("AA", "DD"), st("2:1#57#3=52,1:1#57#3=52")),
                (fr("AA", "DD"), st("2:1#56#3=52,1:1#58#3=52")),
            ],
            &fr("AA", "DD"),
        )
        .unwrap();
        // Compound tokens split and merged by pk range id
        let parts: Vec<&str> = result.as_str().split(',').collect();
        assert_eq!(parts.len(), 2);
        assert!(parts.contains(&"2:1#57#3=52"));
        assert!(parts.contains(&"1:1#58#3=52"));
    }

    #[test]
    fn overlapping_ranges() {
        let result = get_latest_session_token(
            &[
                (fr("AA", "CC"), st("0:1#54#3=52")),
                (fr("BB", "FF"), st("2:1#51#3=52")),
            ],
            &fr("AA", "EE"),
        )
        .unwrap();
        // Both overlap with target, different pk range ids
        let parts: Vec<&str> = result.as_str().split(',').collect();
        assert_eq!(parts.len(), 2);
        assert!(parts.contains(&"0:1#54#3=52"));
        assert!(parts.contains(&"2:1#51#3=52"));
    }

    #[test]
    fn no_relevant_feed_ranges() {
        let result = get_latest_session_token(
            &[
                (fr("CC", "DD"), st("0:1#54#3=52")),
                (fr("EE", "FF"), st("0:1#51")),
            ],
            &fr("AA", "BB"),
        );
        assert!(result.is_err());
    }

    // === Additional edge cases ===

    #[test]
    fn same_range_different_pk_range_ids() {
        // When same feed range has different pk range ids, keep the one with higher LSN
        let result = get_latest_session_token(
            &[
                (fr("AA", "BB"), st("0:1#100#3=50")),
                (fr("AA", "BB"), st("1:1#200#3=60")),
            ],
            &fr("AA", "BB"),
        )
        .unwrap();
        // pk range id 1 has higher global LSN (200 > 100)
        assert!(result.as_str().starts_with("1:"));
        assert!(result.as_str().contains("#200#"));
    }

    #[test]
    fn v1_tokens_merge() {
        let result = get_latest_session_token(
            &[(fr("AA", "BB"), st("0:100")), (fr("AA", "BB"), st("0:200"))],
            &fr("AA", "BB"),
        )
        .unwrap();
        assert_eq!(result.as_str(), "0:200");
    }

    #[test]
    fn single_input() {
        let result =
            get_latest_session_token(&[(fr("AA", "FF"), st("0:1#100#1=10"))], &fr("AA", "FF"))
                .unwrap();
        assert_eq!(result.as_str(), "0:1#100#1=10");
    }

    #[test]
    fn full_range_target() {
        let result = get_latest_session_token(
            &[(FeedRange::full(), st("0:1#100#1=10"))],
            &FeedRange::full(),
        )
        .unwrap();
        assert_eq!(result.as_str(), "0:1#100#1=10");
    }

    #[test]
    fn mixed_split_scenario() {
        // Parent LSN is between the two children
        let result = get_latest_session_token(
            &[
                (fr("AA", "DD"), st("0:1#53#3=52")),
                (fr("AA", "BB"), st("1:1#55#3=52")),
                (fr("BB", "DD"), st("2:1#51#3=52")),
            ],
            &fr("AA", "DD"),
        )
        .unwrap();
        // Mixed: child 1 newer (55 > 53), child 2 older (51 < 53) → keep all
        let parts: Vec<&str> = result.as_str().split(',').collect();
        assert_eq!(parts.len(), 3);
        assert!(parts.contains(&"0:1#53#3=52"));
        assert!(parts.contains(&"1:1#55#3=52"));
        assert!(parts.contains(&"2:1#51#3=52"));
    }

    // === Input ordering tests (Findings 2, 3, 4) ===

    #[test]
    fn three_way_split_sorted() {
        // Parent with 3 children in sorted order, all children newer
        let result = get_latest_session_token(
            &[
                (fr("AA", "FF"), st("0:1#50#3=50")),
                (fr("AA", "BB"), st("1:1#55#3=52")),
                (fr("BB", "DD"), st("2:1#56#3=52")),
                (fr("DD", "FF"), st("3:1#57#3=52")),
            ],
            &fr("AA", "FF"),
        )
        .unwrap();
        // All children newer → split detected, parent replaced
        let parts: Vec<&str> = result.as_str().split(',').collect();
        assert_eq!(parts.len(), 3);
        assert!(parts.contains(&"1:1#55#3=52"));
        assert!(parts.contains(&"2:1#56#3=52"));
        assert!(parts.contains(&"3:1#57#3=52"));
        // Parent token should NOT be present
        assert!(!parts.contains(&"0:1#50#3=50"));
    }

    #[test]
    fn three_way_split_shuffled() {
        // Same as above but children in non-sorted order (Finding 3)
        let result = get_latest_session_token(
            &[
                (fr("AA", "FF"), st("0:1#50#3=50")),
                (fr("DD", "FF"), st("3:1#57#3=52")),
                (fr("AA", "BB"), st("1:1#55#3=52")),
                (fr("BB", "DD"), st("2:1#56#3=52")),
            ],
            &fr("AA", "FF"),
        )
        .unwrap();
        // Should produce same result regardless of child order
        let parts: Vec<&str> = result.as_str().split(',').collect();
        assert_eq!(parts.len(), 3);
        assert!(parts.contains(&"1:1#55#3=52"));
        assert!(parts.contains(&"2:1#56#3=52"));
        assert!(parts.contains(&"3:1#57#3=52"));
        assert!(!parts.contains(&"0:1#50#3=50"));
    }

    #[test]
    fn children_before_parent_in_input() {
        // Children appear before parent in the input array (Finding 4)
        let result = get_latest_session_token(
            &[
                (fr("AA", "BB"), st("1:1#55#3=52")),
                (fr("BB", "DD"), st("2:1#54#3=52")),
                (fr("AA", "DD"), st("0:1#51#3=52")),
            ],
            &fr("AA", "DD"),
        )
        .unwrap();
        // Should still detect the split — same as split_with_both_children
        let parts: Vec<&str> = result.as_str().split(',').collect();
        assert_eq!(parts.len(), 2);
        assert!(parts.contains(&"1:1#55#3=52"));
        assert!(parts.contains(&"2:1#54#3=52"));
    }

    #[test]
    fn unrelated_between_parent_and_children() {
        // Unrelated feed range sits between parent and children in input (Finding 2)
        let result = get_latest_session_token(
            &[
                (fr("AA", "DD"), st("0:1#51#3=52")),
                (fr("EE", "FF"), st("9:1#99#3=99")),
                (fr("AA", "BB"), st("1:1#55#3=52")),
                (fr("BB", "DD"), st("2:1#54#3=52")),
            ],
            &fr("AA", "FF"),
        )
        .unwrap();
        // Split should still be detected for [AA,DD), and unrelated [EE,FF) preserved
        let parts: Vec<&str> = result.as_str().split(',').collect();
        assert_eq!(parts.len(), 3);
        assert!(parts.contains(&"1:1#55#3=52"));
        assert!(parts.contains(&"2:1#54#3=52"));
        assert!(parts.contains(&"9:1#99#3=99"));
        // Parent should have been replaced by children
        assert!(!parts.contains(&"0:1#51#3=52"));
    }

    // === FeedRange helper tests ===

    #[test]
    fn can_merge_adjacent() {
        let a = fr("AA", "BB");
        let b = fr("BB", "DD");
        assert!(a.can_merge(&b));
        assert!(b.can_merge(&a));
    }

    #[test]
    fn can_merge_overlapping() {
        let a = fr("AA", "CC");
        let b = fr("BB", "DD");
        assert!(a.can_merge(&b));
    }

    #[test]
    fn cannot_merge_disjoint() {
        let a = fr("AA", "BB");
        let b = fr("CC", "DD");
        assert!(!a.can_merge(&b));
    }

    #[test]
    fn merge_with_produces_bounding_range() {
        let a = fr("AA", "BB");
        let b = fr("BB", "DD");
        let merged = a.merge_with(&b);
        assert_eq!(merged, fr("AA", "DD"));
    }
}
