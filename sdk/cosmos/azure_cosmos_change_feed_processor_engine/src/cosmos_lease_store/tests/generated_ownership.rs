// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::fixture;
use crate::{ChangeFeedReadOptions, LeaseSession};
use azure_data_cosmos_driver::models::{ChangeFeedStartFrom, ContinuationToken, FeedRange};
use rand::{rngs::StdRng, RngExt, SeedableRng};
use std::{error::Error, time::Duration};

#[derive(Default)]
struct Reference {
    owner: Option<&'static str>,
    generation: u64,
    progressed: bool,
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn generated_commands_follow_an_independent_owner_generation_and_progress_model(
) -> Result<(), Box<dyn Error>> {
    tokio::time::timeout(Duration::from_secs(30), async {
        for seed in [7, 19, 41, 97] {
            let f = fixture().await?;
            let start = f.store_a.observe().await?.checkpoint().to_owned();
            let mut reader = f
                .worker_a
                .open_reader(
                    ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning)
                        .with_continuation(ContinuationToken::from_string(start.clone())),
                )
                .await?;
            let advanced = reader.read_page().await?.continuation().clone();
            let stores = [&f.store_a, &f.store_b];
            let names = ["A", "B"];
            let mut sessions: [Option<LeaseSession>; 2] = [None, None];
            let mut reference = Reference::default();
            let mut random = StdRng::seed_from_u64(seed);
            for step in 0..24 {
                let actor = random.random_range(0..2);
                let command = random.random_range(0..6);
                match command {
                    0 => {
                        let acquired = stores[actor].try_acquire(names[actor]).await?;
                        assert_eq!(
                            acquired.is_some(),
                            reference.owner.is_none(),
                            "seed={seed} step={step} acquire"
                        );
                        if let Some(session) = acquired {
                            reference.owner = Some(names[actor]);
                            reference.generation += 1;
                            sessions[actor] = Some(session);
                        }
                    }
                    1..=3 => {
                        if let Some(session) = &sessions[actor] {
                            let expected = session.lease().await;
                            let authorized = reference.owner == Some(names[actor])
                                && reference.generation == expected.epoch().get()
                                && !session.control().is_lost();
                            let succeeded = match command {
                                1 => session.renew().await.is_ok(),
                                2 => session.checkpoint(&expected, &advanced).await.is_ok(),
                                _ => session.release().await.is_ok(),
                            };
                            assert_eq!(
                                succeeded, authorized,
                                "seed={seed} step={step} command={command}"
                            );
                            if succeeded && command == 2 {
                                reference.progressed = true;
                            }
                            if succeeded && command == 3 {
                                reference.owner = None;
                            }
                        }
                    }
                    4 => {
                        if reference.owner.is_some() && reference.owner != Some(names[actor]) {
                            let observation = stores[actor].observe().await?;
                            let session =
                                stores[actor].transfer(&observation, names[actor]).await?;
                            reference.owner = Some(names[actor]);
                            reference.generation += 1;
                            sessions[actor] = Some(session);
                        }
                    }
                    _ => {
                        sessions[actor] = None;
                    }
                }
                let durable = f.store_b.observe().await?;
                assert_eq!(durable.owner(), reference.owner, "seed={seed} step={step}");
                assert_eq!(
                    durable.generation(),
                    reference.generation,
                    "seed={seed} step={step}"
                );
                assert_eq!(
                    durable.checkpoint(),
                    if reference.progressed {
                        advanced.as_str()
                    } else {
                        &start
                    },
                    "seed={seed} step={step}"
                );
            }
        }
        Ok::<_, Box<dyn Error>>(())
    })
    .await??;
    Ok(())
}
