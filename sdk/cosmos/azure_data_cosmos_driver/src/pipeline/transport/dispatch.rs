// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Dispatch and cancellation of one prepared HTTP request.

use super::{
    adaptive_transport::AdaptiveTransport, cosmos_transport_client::HttpRequest,
    sharded_transport::TransportDispatch, EndpointKey,
};
use crate::timeout;
use std::time::Duration;

pub(super) enum DispatchOutcome {
    Completed(TransportDispatch),
    TimedOut { shard_hint: Option<u64> },
}

/// Sends one request, returning its dispatch result or a timeout with a best-effort shard hint.
///
/// A timeout cancels the send by dropping its future; no shard is preselected without a deadline.
pub(super) async fn dispatch_http_request(
    request: &HttpRequest,
    transport: &AdaptiveTransport,
    timeout: Option<Duration>,
    excluded_shard_id: Option<u64>,
    endpoint_key: &EndpointKey,
) -> DispatchOutcome {
    if let Some(duration) = timeout {
        let shard_hint = transport.pre_select_shard(excluded_shard_id, endpoint_key);
        let transport_future =
            transport.send_with_dispatch(request, excluded_shard_id, endpoint_key, shard_hint);
        return match timeout::with_timeout(duration, transport_future).await {
            Ok(dispatch) => DispatchOutcome::Completed(dispatch),
            Err(timeout::TimedOut) => DispatchOutcome::TimedOut { shard_hint },
        };
    }
    DispatchOutcome::Completed(
        transport
            .send_with_dispatch(request, excluded_shard_id, endpoint_key, None)
            .await,
    )
}
