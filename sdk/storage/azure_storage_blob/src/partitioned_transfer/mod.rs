use std::{cmp::max, future::Future, num::NonZero, pin::Pin};

use futures::{
    future::{self},
    Stream, TryStreamExt,
};

type AzureResult<T> = azure_core::Result<T>;

/// Executes async operations from a queue with a concurrency limit.
///
/// This function consumes a stream (`ops_queue`) of async operation factories (closures returning futures),
/// and runs up to `parallel` operations concurrently. As operations complete, new ones are started from the queue,
/// maintaining the concurrency limit. If any operation or queue item returns an error, the function returns early
/// with that error. When all operations and queue items are complete, returns `Ok(())`.
///
/// # Parameters
/// - `ops_queue`: A stream yielding `Result<FnOnce() -> TFut, TErr>`. Each item is either a closure producing a future,
///   or an error. The stream must be `Unpin`.
/// - `parallel`: The maximum number of operations to run concurrently. Must be non-zero.
///
/// # Behavior
/// - Operations are scheduled as soon as possible, up to the concurrency limit.
/// - If an error is encountered in the queue or in any operation, the function returns that error immediately.
/// - When the queue is exhausted, waits for all running operations to complete before returning.
///
/// # Example
/// ```rust
/// use futures::{stream, StreamExt};
/// use std::num::NonZeroUsize;
///
/// async fn example() {
///     let ops = vec![
///         Ok(|| async { Ok(()) }),
///         Ok(|| async { Ok(()) }),
///     ];
///     let ops_stream = stream::iter(ops);
///     run_all_with_concurrency_limit(ops_stream, NonZeroUsize::new(2).unwrap()).await.unwrap();
/// }
/// ```
///
/// # Errors
/// Returns the first error encountered from the queue or any operation.
///
/// # Type Parameters
/// - `TFut`: Future type returned by each operation.
/// - `TErr`: Error type for queue or operation failures.
async fn run_all_with_concurrency_limit<Fut, Err>(
    mut ops_queue: impl Stream<Item = Result<impl FnOnce() -> Fut, Err>> + Unpin,
    parallel: NonZero<usize>,
) -> Result<(), Err>
where
    Fut: Future<Output = Result<(), Err>>,
{
    let parallel = parallel.get();

    // if no real parallelism, take the simple option of executing ops sequentially.
    // The "true" implementation can't handle parallel < 2.
    if parallel == 1 {
        while let Some(op) = ops_queue.try_next().await? {
            op().await?;
        }
        return Ok(());
    }

    let first_op = match ops_queue.try_next().await? {
        Some(item) => item,
        None => return Ok(()),
    };

    let mut get_next_completed_op_future = future::select_all(vec![Box::pin(first_op())]);
    let mut get_next_queue_op_future = ops_queue.try_next();
    loop {
        // while max parallel running ops, focus on just running ops
        get_next_completed_op_future = run_down(get_next_completed_op_future, parallel - 1).await?;

        match future::select(get_next_queue_op_future, get_next_completed_op_future).await {
            future::Either::Left((Err(e), _)) => return Err(e),
            future::Either::Right(((Err(e), _, _), _)) => return Err(e),

            // Next op in the queue arrived first. Add it to existing running ops.
            future::Either::Left((Ok(next_op_in_queue), running_ops_fut)) => {
                get_next_queue_op_future = ops_queue.try_next();
                get_next_completed_op_future = running_ops_fut;

                match next_op_in_queue {
                    Some(op) => {
                        get_next_completed_op_future =
                            combine_select_all(get_next_completed_op_future, Box::pin(op()));
                    }
                    // queue was finished, race is over
                    None => break,
                }
            }
            // A running op completed first. Start another select_all with remaining running ops.
            future::Either::Right(((Ok(_), _, remaining_running_ops), next_op_fut)) => {
                // remaining_running_ops could be empty now.
                // select panics on empty iter, so we can't race in this case.
                // forcibly wait for next op in queue and handle it before continuing.
                if remaining_running_ops.is_empty() {
                    let next_op = match next_op_fut.await? {
                        Some(item) => item,
                        None => return Ok(()),
                    };
                    get_next_queue_op_future = ops_queue.try_next();
                    get_next_completed_op_future = future::select_all(vec![Box::pin(next_op())]);
                } else {
                    get_next_queue_op_future = next_op_fut;
                    get_next_completed_op_future = future::select_all(remaining_running_ops);
                }
            }
        }
    }

    let _ = future::try_join_all(get_next_completed_op_future.into_inner()).await?;
    Ok(())
}

/// Loops `future::select_all()` with the existing `SelectAll`` until the target remaining
/// inner futures is reached. Will always leave at least one inner future remaining, for
/// type simplicity (select_all panics on len == 0);
async fn run_down<Fut, Err>(
    select_fut: future::SelectAll<Pin<Box<Fut>>>,
    target_remaining: usize,
) -> Result<future::SelectAll<Pin<Box<Fut>>>, Err>
where
    Fut: Future<Output = Result<(), Err>>,
{
    let target_remaining = max(target_remaining, 1);
    let mut select_vec = select_fut.into_inner();
    while select_vec.len() > target_remaining {
        let result;
        (result, _, select_vec) = future::select_all(select_vec).await;
        result?;
    }
    Ok(future::select_all(select_vec))
}

/// Adds a pin-boxed future to an existing SelectAll of pin-boxed futures.
fn combine_select_all<Fut>(
    select_fut: future::SelectAll<Pin<Box<Fut>>>,
    new_fut: Pin<Box<Fut>>,
) -> future::SelectAll<Pin<Box<Fut>>>
where
    Fut: Future,
{
    let mut futures = select_fut.into_inner();
    futures.push(new_fut);
    future::select_all(futures)
}

#[cfg(test)]
mod tests {
    use futures::{ready, FutureExt};

    use super::*;
    use std::{pin::Pin, sync::mpsc::channel, task::Poll, time::Duration};

    #[tokio::test]
    async fn enforce_concurrency_limit() -> AzureResult<()> {
        let parallel = 4usize;
        let num_ops = parallel + 1;
        let wait_time_millis = 10u64;
        let op_time_millis = wait_time_millis + 50;

        let (sender, receiver) = channel();

        // setup a series of operations that send a unique number to a channel
        // we can then assert the expected numbers made it to the channel at expected times
        let ops = (0..num_ops).map(|i| {
            let s = sender.clone();
            Ok(async move || {
                s.send(i).unwrap();
                tokio::time::sleep(Duration::from_millis(op_time_millis)).await;
                AzureResult::<()>::Ok(())
            })
        });

        let race = future::select(
            Box::pin(run_all_with_concurrency_limit(
                futures::stream::iter(ops),
                NonZero::new(parallel).unwrap(),
            )),
            Box::pin(tokio::time::sleep(Duration::from_millis(wait_time_millis))),
        )
        .await;
        match race {
            future::Either::Left(_) => panic!("Wrong future won the race."),
            future::Either::Right((_, run_all_fut)) => {
                let mut items: Vec<_> = receiver.try_iter().collect();
                items.sort();
                assert_eq!(items, (0..parallel).collect::<Vec<_>>());

                run_all_fut.await?;
                assert_eq!(receiver.try_iter().collect::<Vec<_>>().len(), 1);
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn handles_slow_stream() -> AzureResult<()> {
        let parallel = 10;
        let num_ops = 5;
        let op_time_millis = 10;
        let stream_time_millis = op_time_millis + 10;
        // setup a series of operations that send a unique number to a channel
        // we can then assert the expected numbers made it to the channel at expected times
        let ops = (0..num_ops).map(|_| {
            Ok(async move || {
                tokio::time::sleep(Duration::from_millis(op_time_millis)).await;
                AzureResult::<()>::Ok(())
            })
        });

        run_all_with_concurrency_limit(
            SlowStream::new(ops, Duration::from_millis(stream_time_millis)),
            NonZero::new(parallel).unwrap(),
        )
        .await
    }

    #[tokio::test]
    async fn success_when_no_ops() -> AzureResult<()> {
        let parallel = 4usize;

        // not possible to manually type what we need
        // make a vec with a concrete element and then remove it to get the desired typing
        let op = || future::ready::<Result<(), azure_core::Error>>(Ok(()));
        let mut ops = vec![Ok(op)];
        ops.pop();

        run_all_with_concurrency_limit(futures::stream::iter(ops), NonZero::new(parallel).unwrap())
            .await
    }

    struct SlowStream<Iter> {
        sleep: Pin<Box<tokio::time::Sleep>>,
        interval: Duration,
        iter: Iter,
    }
    impl<Iter> SlowStream<Iter> {
        fn new(iter: Iter, interval: Duration) -> Self {
            Self {
                sleep: Box::pin(tokio::time::sleep(interval)),
                interval,
                iter,
            }
        }
    }
    impl<Iter: Iterator + Unpin> Stream for SlowStream<Iter> {
        type Item = Iter::Item;

        fn poll_next(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Option<Self::Item>> {
            let this = self.get_mut();
            ready!(this.sleep.poll_unpin(cx));
            this.sleep = Box::pin(tokio::time::sleep(this.interval));
            Poll::Ready(this.iter.next())
        }
    }
}
