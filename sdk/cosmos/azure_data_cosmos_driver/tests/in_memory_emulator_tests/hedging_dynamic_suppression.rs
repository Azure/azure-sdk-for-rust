// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end refresh coverage for the account-level hedging suppression signal.

use std::time::Duration;

use azure_data_cosmos_driver::in_memory_emulator::WriteMode;

use super::{
    hedging::{make_hedging_driver, read_item_hedge_diagnostics, seed_item},
    setup_multi_region,
};

const REFRESH_INTERVAL: Duration = Duration::from_secs(300);

#[tokio::test(start_paused = true)]
async fn background_refresh_reconciles_and_retains_hedging_suppression() {
    let ctx = setup_multi_region(WriteMode::Single).await;
    seed_item(&ctx, "refresh-item", "pk1").await;
    let (driver, options) = make_hedging_driver(&ctx, Duration::from_millis(500), Vec::new()).await;

    assert!(
        read_item_hedge_diagnostics(&driver, options.clone(), "refresh-item", "pk1")
            .await
            .is_some(),
        "initial explicit false must allow the configured strategy"
    );

    ctx.emulator
        .store()
        .config()
        .set_cross_region_hedging_disabled(Some(true));
    tokio::time::sleep(REFRESH_INTERVAL * 2).await;
    assert!(
        read_item_hedge_diagnostics(&driver, options.clone(), "refresh-item", "pk1")
            .await
            .is_none(),
        "explicit true must suppress the configured strategy"
    );

    ctx.emulator
        .store()
        .config()
        .set_cross_region_hedging_disabled(None);
    tokio::time::sleep(REFRESH_INTERVAL * 2).await;
    assert!(
        read_item_hedge_diagnostics(&driver, options.clone(), "refresh-item", "pk1")
            .await
            .is_none(),
        "an omitted property must preserve the last explicit signal"
    );

    ctx.emulator
        .store()
        .config()
        .set_cross_region_hedging_disabled(Some(false));
    tokio::time::sleep(REFRESH_INTERVAL * 2).await;
    assert!(
        read_item_hedge_diagnostics(&driver, options, "refresh-item", "pk1")
            .await
            .is_some(),
        "explicit false must resume the configured strategy"
    );
}
