// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation targeting and per-request overrides.

use std::sync::Arc;

use azure_core::http::headers::{AsHeaders, HeaderName, HeaderValue};

use crate::{
    driver::cache::ContainerRoutingMap, models::request_header_names,
    pipeline::operation::recovery::recreation::ContainerRecreationRecoveryTracker,
};

/// Internal, non-customer-overridable hedging and routing pin carried on
/// [`OperationOverrides::region_pin`](crate::pipeline::operation::types::OperationOverrides::region_pin).
///
/// Attached to a read whose continuation token is region-affine — today the
/// PartitionKeyRange change feed, whose ETag is only meaningful to the region
/// that issued it. Its presence alone suppresses hedging; the optional
/// `endpoint` additionally forces the attempt onto a specific region.
///
/// This deliberately does **not** reuse [`AvailabilityStrategy::Disabled`]:
/// that is a customer-facing option, and `resolve_availability_strategy` lets
/// the `AZURE_COSMOS_HEDGING_ENABLED=true` environment switch override it. A
/// correctness constraint must not be something configuration can turn off, so
/// it lives here instead.
///
/// [`AvailabilityStrategy::Disabled`]: crate::options::AvailabilityStrategy::Disabled
#[derive(Debug, Clone, Default)]
pub(crate) struct RegionPin {
    /// Region this attempt must be routed to, bypassing the normal region
    /// selection in [`resolve_endpoint`](crate::pipeline::operation::routing::resolve).
    ///
    /// Normally always populated: the region that served the cold page is
    /// recorded and every later page in the chain is pinned to it, so a
    /// `FailoverRetry`/`SessionRetry` cannot carry the region-affine
    /// continuation into a region that never issued it.
    ///
    /// `None` is the degraded fallback for the rare case where the serving
    /// region could not be identified. Normal region selection then applies,
    /// but hedging stays suppressed — racing a second region would send the
    /// token somewhere it means nothing.
    pub endpoint: Option<crate::pipeline::operation::routing::CosmosEndpoint>,
}

/// Per-request overrides that take precedence over values from [`CosmosOperation`](crate::models::CosmosOperation).
///
/// Used by the dataflow pipeline to inject routing and pagination state that
/// varies per physical partition or per page, without mutating the shared
/// `CosmosOperation`. Most fields, when `Some`, emit the corresponding request
/// header in [`OperationOverrides::apply_headers`]; `region_pin` instead
/// constrains hedging and the STAGE 2 routing decision.
#[derive(Debug, Clone, Default)]
pub(crate) struct OperationOverrides {
    /// Feed range to constrain the request to (emits `x-ms-start-epk` / `x-ms-end-epk`).
    pub feed_range: Option<crate::models::FeedRange>,

    /// Physical pkrange bounds (full EPK span of the resolved partition). Carried
    /// to the GW_V2 dispatcher via internal `x-ms-thinclient-pkrange-min`/`-max`
    /// headers so it can derive `StartEpkHash`/`EndEpkHash` RNTBD tokens for the
    /// full-pkrange XPK case without emitting the public EPK headers (which the
    /// legacy gateway rejects for queries when paired with `partitionkeyrangeid`).
    pub pkrange_bounds: Option<crate::models::FeedRange>,

    /// Physical partition key range ID (emits `x-ms-documentdb-partitionkeyrangeid`).
    pub partition_key_range_id: Option<String>,

    /// Physical partition identity resolved for a logical-partition request.
    ///
    /// This seeds PPCB/PPAF and session-token scoping but is never
    /// emitted as a request header; the logical partition key remains the
    /// authoritative wire-routing target.
    pub resolved_partition_key_range_id: Option<String>,

    /// Ancestors whose session vectors remain valid for the resolved child.
    pub resolved_partition_key_range_parents: Vec<String>,

    /// Snapshot used to validate automatic classic Gateway session scoping.
    pub session_token_routing_map: Option<Arc<ContainerRoutingMap>>,

    /// Logical partition key (emits `x-ms-documentdb-partitionkey`).
    pub partition_key: Option<crate::models::PartitionKey>,

    /// Whether this request routes by one logical partition key.
    ///
    /// Used to suppress invalid composite session-token fallback on Gateway 2.0
    /// when optional physical identity resolution is unavailable.
    pub logical_partition_key_target: bool,

    /// Continuation token for pagination (emits `x-ms-continuation`).
    pub continuation: Option<String>,

    /// Constrains this attempt to a single region and forbids hedging.
    ///
    /// Unlike the other fields this is NOT a request header — it is consumed by
    /// STAGE 2 (routing), STAGE 2b (pre-attempt hedge dispatch), and STAGE 5b
    /// (post-attempt hedge upgrade) of the operation pipeline. It keeps a paged
    /// read on one region: the PartitionKeyRange change feed carries its
    /// continuation as an ETag that is only meaningful to the region that
    /// issued it, so every page after the first must neither race another
    /// region nor be failed over into one.
    ///
    /// See [`RegionPin`] for why this is an internal signal rather than
    /// `AvailabilityStrategy::Disabled`. Boxed so the field stays pointer-sized
    /// — this struct is captured by the operation future, which is already
    /// close to the `clippy::large_futures` budget.
    pub region_pin: Option<Box<RegionPin>>,

    /// Prevents an outer logical-operation coordinator from starting a second
    /// container-recreation recovery after it already consumed that budget.
    pub container_recreation_recovery_disabled: bool,

    /// Reports whether this request consumed recreation recovery and whether
    /// its physical routing requires the owning plan to be rebuilt.
    pub container_recreation_recovery_tracker: Option<Arc<ContainerRecreationRecoveryTracker>>,
}

impl OperationOverrides {
    /// Returns the physical partition identity available to pipeline consumers.
    pub(crate) fn effective_partition_key_range_id(&self) -> Option<&str> {
        self.partition_key_range_id
            .as_deref()
            .or(self.resolved_partition_key_range_id.as_deref())
    }

    /// Parent range IDs for the effective internal logical identity.
    pub(crate) fn effective_partition_key_range_parents(
        &self,
        partition_key_range_id: &str,
    ) -> &[String] {
        if self.effective_partition_key_range_id() == Some(partition_key_range_id) {
            &self.resolved_partition_key_range_parents
        } else {
            &[]
        }
    }

    /// Clears generation-specific logical identity while preserving wire routing.
    pub(crate) fn clear_resolved_partition_key_range(&mut self) {
        self.resolved_partition_key_range_id = None;
        self.resolved_partition_key_range_parents.clear();
        self.session_token_routing_map = None;
    }

    /// Whether this request carries wire-routing constraints tied to one
    /// container generation and therefore needs its dataflow plan rebuilt after
    /// recreation.
    ///
    /// The internal logical-partition identity is intentionally excluded: it is
    /// not emitted on the wire and can be cleared for an in-place retry.
    pub(crate) fn requires_plan_rebuild_after_container_recreation(&self) -> bool {
        self.partition_key_range_id.is_some()
            || self.feed_range.is_some()
            || self.pkrange_bounds.is_some()
    }

    /// The endpoint this attempt is pinned to, if any.
    ///
    /// `None` either because there is no pin at all, or because the pin only
    /// suppresses hedging and leaves region selection alone.
    pub fn pinned_endpoint(&self) -> Option<&crate::pipeline::operation::routing::CosmosEndpoint> {
        self.region_pin.as_ref().and_then(|p| p.endpoint.as_ref())
    }

    /// Whether hedging must not fire for this attempt.
    ///
    /// Distinct from the customer-facing `AvailabilityStrategy::Disabled`,
    /// which `AZURE_COSMOS_HEDGING_ENABLED=true` deliberately overrides.
    pub fn hedging_suppressed(&self) -> bool {
        self.region_pin.is_some()
    }

    /// Applies the override headers to the given header map.
    ///
    /// Headers set here take precedence over any previously-set values for
    /// the same header name (they overwrite on conflict).
    ///
    /// When `continuation_as_if_none_match` is `true` (change feed reads), the
    /// continuation token is emitted as the `If-None-Match` header instead of
    /// `x-ms-continuation`, since the change feed carries its continuation via
    /// the ETag/`If-None-Match` mechanism.
    pub fn apply_headers(
        &self,
        headers: &mut azure_core::http::headers::Headers,
        continuation_as_if_none_match: bool,
    ) -> crate::error::Result<()> {
        if let Some(feed_range) = &self.feed_range {
            // Narrowed slices need explicit filters in addition to physical routing.
            headers.insert(
                HeaderName::from_static(request_header_names::START_EPK),
                HeaderValue::from(feed_range.min_inclusive().to_hex()),
            );
            headers.insert(
                HeaderName::from_static(request_header_names::END_EPK),
                HeaderValue::from(feed_range.max_exclusive().to_hex()),
            );
            // `x-ms-start-epk`/`x-ms-end-epk` describe an effective-partition-key
            // *range*, so the key type must be `EffectivePartitionKeyRange`. The
            // point value `EffectivePartitionKey` is rejected by the gateway with
            // `400 "One of the input values is invalid"` for range-scoped requests
            // (issues #4680 and #4681).
            headers.insert(
                HeaderName::from_static(request_header_names::READ_FEED_KEY_TYPE),
                HeaderValue::from_static(request_header_names::READ_FEED_KEY_TYPE_EPK_RANGE),
            );
        }

        if let Some(bounds) = &self.pkrange_bounds {
            // Internal-only headers consumed by the GW_V2 dispatcher to
            // synthesize `StartEpkHash`/`EndEpkHash` RNTBD tokens for the
            // full-pkrange XPK case. Legacy gateway ignores unknown headers.
            headers.insert(
                HeaderName::from_static(request_header_names::THINCLIENT_PKRANGE_MIN),
                HeaderValue::from(bounds.min_inclusive().to_hex()),
            );
            headers.insert(
                HeaderName::from_static(request_header_names::THINCLIENT_PKRANGE_MAX),
                HeaderValue::from(bounds.max_exclusive().to_hex()),
            );
        }

        if let Some(pk_range_id) = &self.partition_key_range_id {
            headers.insert(
                HeaderName::from_static(request_header_names::PARTITION_KEY_RANGE_ID),
                HeaderValue::from(pk_range_id.clone()),
            );
        }

        if let Some(pk) = &self.partition_key {
            let pk_headers = pk.as_headers()?;
            for (name, value) in pk_headers {
                headers.insert(name, value);
            }
        }

        if let Some(continuation) = &self.continuation {
            let header_name = if continuation_as_if_none_match {
                request_header_names::IF_NONE_MATCH
            } else {
                request_header_names::CONTINUATION
            };
            headers.insert(
                HeaderName::from_static(header_name),
                HeaderValue::from(continuation.clone()),
            );
        }

        Ok(())
    }
}
