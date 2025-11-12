use std::{future::Future, num::NonZero};

use futures::{
    future::{self},
    Stream, TryStreamExt,
};

type AzureResult<T> = azure_core::Result<T>;

async fn run_all_with_concurrency_limit<TFut, TErr>(
    mut ops_queue: impl Stream<Item = Result<impl FnOnce() -> TFut, TErr>> + Unpin,
    parallel: NonZero<usize>,
) -> Result<(), TErr>
where
    TFut: Future<Output = Result<(), TErr>>,
{
    let parallel = parallel.get();

    let first_op = ops_queue.try_next().await?.ok_or_else(|| todo!())?;

    let mut get_next_completed_op_future = future::select_all(vec![Box::pin(first_op())]);
    let mut get_next_queue_op_future = ops_queue.try_next();
    loop {
        // while max parallel running ops, focus on just running ops
        let mut running_ops = get_next_completed_op_future.into_inner();
        while running_ops.len() >= parallel {
            let result;
            (result, _, running_ops) = future::select_all(running_ops).await;
            result?
        }
        get_next_completed_op_future = future::select_all(running_ops);

        match future::select(get_next_queue_op_future, get_next_completed_op_future).await {
            future::Either::Left((Err(e), _)) => return Err(e),
            future::Either::Right(((Err(e), _, _), _)) => return Err(e),

            // next op in the queue arrived first
            future::Either::Left((Ok(next_op_in_queue), running_ops_fut)) => {
                get_next_queue_op_future = ops_queue.try_next();
                get_next_completed_op_future = running_ops_fut;

                match next_op_in_queue {
                    Some(op) => {
                        running_ops = get_next_completed_op_future.into_inner();
                        running_ops.push(Box::pin(op()));
                        get_next_completed_op_future = future::select_all(running_ops);
                    }
                    // queue was finished, race is over
                    None => break,
                }
            }
            // a running op completed first
            future::Either::Right(((Ok(_res), _, remaining_running_ops), next_op_fut)) => {
                get_next_queue_op_future = next_op_fut;
                get_next_completed_op_future = if remaining_running_ops.is_empty() {
                    todo!("handle no remaining ops but op queue hasn't completed")
                } else {
                    future::select_all(remaining_running_ops)
                };
            }
        }
    }

    let mut running_ops = get_next_completed_op_future.into_inner();
    while !running_ops.is_empty() {
        let result;
        (result, _, running_ops) = future::select_all(running_ops).await;
        result?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::mpsc::channel, time::Duration};

    #[tokio::test]
    async fn limit_ops() -> AzureResult<()> {
        let parallel = 4usize;
        let num_ops = parallel + 1;
        let wait_time_millis = 10u64;
        let op_time_millis = wait_time_millis + 50;

        let (sender, receiver) = channel();

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
                let mut nums: Vec<_> = receiver.try_iter().collect();
                nums.sort();
                assert_eq!(nums, (0..parallel).collect::<Vec<_>>());

                run_all_fut.await?;
                assert_eq!(receiver.try_iter().collect::<Vec<_>>().len(), 1);
            }
        }

        Ok(())
    }
}
