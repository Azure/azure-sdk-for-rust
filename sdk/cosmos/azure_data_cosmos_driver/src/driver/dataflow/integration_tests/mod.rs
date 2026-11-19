// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end integration tests that compose multiple `dataflow` layers
//! (planner + drain + request + snapshot + continuation-token) against the
//! mock executor and topology provider. Unit tests live next to the layer
//! they cover; cross-layer scenarios live here.

mod change_feed_resume;
mod order_by_resume;
mod query_resume;

use super::{
    PartitionRoutingRefresh, Pipeline, PipelineContext, PipelineResponse, RequestExecutor,
    RequestTarget,
};
use crate::{error::Result, models::CosmosOperation};
use futures::{channel::oneshot, future::BoxFuture};
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

/// Hold each backend page pending until the test releases it. Counting calls
/// before constructing the future also catches eager submission of siblings.
async fn assert_serial_leaf_requests(pipeline: &mut Pipeline, pages: [PipelineResponse; 2]) {
    struct GatedExecutor {
        calls: Arc<AtomicUsize>,
        replies: VecDeque<oneshot::Receiver<PipelineResponse>>,
        ranges: Vec<String>,
    }

    impl RequestExecutor for GatedExecutor {
        fn execute_request<'a>(
            &'a mut self,
            _: &'a CosmosOperation,
            target: RequestTarget,
            _: PartitionRoutingRefresh,
            _: Option<String>,
        ) -> BoxFuture<'a, Result<PipelineResponse>> {
            let RequestTarget::EffectivePartitionKeyRange {
                partition_key_range_id,
                ..
            } = target
            else {
                panic!("expected a physical query range")
            };
            self.ranges.push(partition_key_range_id);
            self.calls.fetch_add(1, Ordering::SeqCst);
            let reply = self.replies.pop_front().expect("unexpected leaf request");
            Box::pin(async move { Ok(reply.await.expect("test dropped reply")) })
        }
    }

    let calls = Arc::new(AtomicUsize::new(0));
    let (releases, replies): (Vec<_>, VecDeque<_>) = (0..2).map(|_| oneshot::channel()).unzip();
    let mut executor = GatedExecutor {
        calls: Arc::clone(&calls),
        replies,
        ranges: Vec::new(),
    };
    let mut drain = Box::pin(async {
        let mut context = PipelineContext::new(&mut executor, None);
        while pipeline.next_page(&mut context).await.unwrap().is_some() {}
    });
    for (index, (release, page)) in releases.into_iter().zip(pages).enumerate() {
        assert!(futures::poll!(&mut drain).is_pending());
        assert_eq!(
            calls.load(Ordering::SeqCst),
            index + 1,
            "a sibling started before the active leaf returned"
        );
        release.send(page).expect("leaf cancelled while pending");
    }
    drain.await;
    assert_eq!(executor.ranges, ["pk-left", "pk-right"]);
}
