// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use azure_core_test::recorded;
use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::ProducerClient;
use std::{
    alloc::{GlobalAlloc, Layout, System},
    env,
    error::Error,
    sync::atomic::{AtomicUsize, Ordering},
};

struct LiveByteAllocator;

static LIVE_BYTES: AtomicUsize = AtomicUsize::new(0);

#[global_allocator]
static ALLOCATOR: LiveByteAllocator = LiveByteAllocator;

unsafe impl GlobalAlloc for LiveByteAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            LIVE_BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) };
        LIVE_BYTES.fetch_sub(layout.size(), Ordering::Relaxed);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            LIVE_BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        }
        pointer
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_pointer = unsafe { System.realloc(pointer, layout, new_size) };
        if !new_pointer.is_null() {
            match new_size.cmp(&layout.size()) {
                std::cmp::Ordering::Greater => {
                    LIVE_BYTES.fetch_add(new_size - layout.size(), Ordering::Relaxed);
                }
                std::cmp::Ordering::Less => {
                    LIVE_BYTES.fetch_sub(layout.size() - new_size, Ordering::Relaxed);
                }
                std::cmp::Ordering::Equal => {}
            }
        }
        new_pointer
    }
}

#[recorded::test(live)]
async fn producer_lifecycle_send_close_has_no_sustained_heap_trend() -> Result<(), Box<dyn Error>> {
    let host = match env::var("EVENTHUBS_HOST") {
        Ok(host) if !host.is_empty() => host,
        _ => return Ok(()),
    };
    let eventhub = match env::var("EVENT_HUB_NAME") {
        Ok(eventhub) if !eventhub.is_empty() => eventhub,
        _ => match env::var("EVENTHUB_NAME") {
            Ok(eventhub) if !eventhub.is_empty() => eventhub,
            _ => return Ok(()),
        },
    };
    let credential = match DeveloperToolsCredential::new(None) {
        Ok(credential) => credential,
        Err(_) => return Ok(()),
    };

    const WARMUP_CYCLES: usize = 5;
    const MEASURED_CYCLES: usize = 95;
    const BLOCK_SIZE: usize = 5;
    const BLOCK_COUNT: usize = MEASURED_CYCLES / BLOCK_SIZE;
    const TOTAL_CYCLES: usize = WARMUP_CYCLES + MEASURED_CYCLES;

    let mut samples = [0usize; MEASURED_CYCLES];
    for cycle in 0..TOTAL_CYCLES {
        let client = match ProducerClient::builder()
            .with_application_id(
                "producer_lifecycle_send_close_has_no_sustained_heap_trend".to_string(),
            )
            .open(host.as_str(), eventhub.as_str(), credential.clone())
            .await
        {
            Ok(client) => client,
            Err(error) => panic!("producer open failed during cycle {cycle}: {error}"),
        };

        let send_result = client.send_event("lifecycle memory test", None).await;
        assert!(
            send_result.is_ok(),
            "producer send failed during cycle {cycle}: {send_result:?}"
        );

        let close_result = client.close().await;
        assert!(
            close_result.is_ok(),
            "producer close failed during cycle {cycle}: {close_result:?}"
        );

        if cycle >= WARMUP_CYCLES {
            samples[cycle - WARMUP_CYCLES] = LIVE_BYTES.load(Ordering::Relaxed);
        }
    }

    let mut medians = [0.0; BLOCK_COUNT];
    for (block_index, median) in medians.iter_mut().enumerate() {
        let start = block_index * BLOCK_SIZE;
        let mut block = [0usize; BLOCK_SIZE];
        block.copy_from_slice(&samples[start..start + BLOCK_SIZE]);
        block.sort_unstable();
        *median = block[BLOCK_SIZE / 2] as f64;
    }

    let x_mean = (BLOCK_COUNT - 1) as f64 / 2.0;
    let y_mean = medians.iter().sum::<f64>() / BLOCK_COUNT as f64;
    let sum_squared_x_deviations = (0..BLOCK_COUNT)
        .map(|index| {
            let deviation = index as f64 - x_mean;
            deviation * deviation
        })
        .sum::<f64>();
    let slope = medians
        .iter()
        .enumerate()
        .map(|(index, median)| (index as f64 - x_mean) * (median - y_mean))
        .sum::<f64>()
        / sum_squared_x_deviations;
    let intercept = y_mean - slope * x_mean;
    let residual_sum_of_squares = medians
        .iter()
        .enumerate()
        .map(|(index, median)| {
            let residual = median - (intercept + slope * index as f64);
            residual * residual
        })
        .sum::<f64>();
    let noise = (residual_sum_of_squares / (BLOCK_COUNT - 2) as f64).sqrt();
    let slope_standard_error = noise / sum_squared_x_deviations.sqrt();
    let lower_bound = slope - 3.0 * slope_standard_error;

    assert!(
        lower_bound <= 0.0,
        "lower_bound <= 0: slope={slope:.3} bytes/block, noise={noise:.3}, lower_bound={lower_bound:.3}, block medians={medians:?}"
    );

    Ok(())
}
