// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{report_errors, ErrorHandler, ErrorReporter, ERROR_QUEUE_CAPACITY};
use crate::{CosmosError, CosmosStatus};
use azure_core::http::StatusCode;
use futures::{FutureExt, StreamExt};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

fn failure() -> CosmosError {
    CosmosError::builder()
        .with_status(CosmosStatus::new(StatusCode::ServiceUnavailable))
        .with_message("Processor failure.")
        .build()
}

#[test]
fn error_notifications_are_bounded_and_close_drains_accepted_reports() {
    let (reporter, mut received, _canceled) = ErrorReporter::new(true);
    for _ in 0..ERROR_QUEUE_CAPACITY + 2 {
        reporter.report(Some("lease".into()), failure());
    }
    reporter.close();
    let mut count = 0;
    while let Some((context, error)) = received.next().now_or_never().unwrap() {
        assert_eq!(context.unwrap().lease_token(), "lease");
        assert_eq!(error.status().status_code(), StatusCode::ServiceUnavailable);
        count += 1;
    }
    assert_eq!(count, ERROR_QUEUE_CAPACITY);
}

#[tokio::test]
async fn error_hook_panics_do_not_stop_subsequent_notifications() {
    let (reporter, received, canceled) = ErrorReporter::new(true);
    let calls = Arc::new(AtomicUsize::new(0));
    let completed = Arc::new(AtomicUsize::new(0));
    let handler: Arc<ErrorHandler> = {
        let calls = calls.clone();
        let completed = completed.clone();
        Arc::new(move |_, _| {
            let index = calls.fetch_add(1, Ordering::SeqCst);
            assert_ne!(index, 0, "observer construction panic");
            let completed = completed.clone();
            Box::pin(async move {
                assert_ne!(index, 1, "observer future panic");
                completed.fetch_add(1, Ordering::SeqCst);
            })
        })
    };
    for _ in 0..3 {
        reporter.report(None, failure());
    }
    reporter.close();
    report_errors(Some(handler), received, canceled).await;
    assert_eq!(calls.load(Ordering::SeqCst), 3);
    assert_eq!(completed.load(Ordering::SeqCst), 1);
}
